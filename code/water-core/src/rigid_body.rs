//! **Le corps rigide du jeu** — S331, lot 4 ([ADR-178](../../../docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md)
//! D7, [ADR-189](../../../docs/adr/ADR-189-la-v1-d-abord.md)).
//!
//! # Ce que ce module est
//!
//! Un corps à six degrés de liberté — position du centre de masse, quaternion d'orientation, vitesses
//! linéaire et angulaire —, poussé par un **proxy de flottabilité** : un jeu de points attachés au corps,
//! chacun représentant un volume, dont l'immersion passe de 0 à 1 sur son épaisseur
//! ([ADR-008](../../../docs/adr/ADR-008-flottabilite-et-autorite.md) §2, la fonction `sat`). Pour un point
//! tenu à plat sous une ligne d'eau plane, cette rampe est exacte : le volume immergé d'un pavé droit est
//! celui de la géométrie, à l'arrondi près. La masse ajoutée est une matrice diagonale, dans le repère du
//! monde ; l'intégrateur est symplectique — vitesses d'abord, positions ensuite —, en `f64`, déterministe.
//!
//! # D'où vient l'eau — I-04
//!
//! **Toute force capable de changer une issue de jeu vient de B + W, jamais de δ** (ADR-008 §1). Le corps
//! n'interroge l'eau que par [`WaterQuery`] : une eau calme pour C10, la mer B + W pour la porte D. δ, lui,
//! reçoit le corps comme une paroi mobile (`Volume3::set_solid`) et ne lui rend qu'un décalage visuel
//! borné ; rien de δ n'entre ici.
//!
//! # Ce que ce module n'est pas encore
//!
//! Ni contact, ni collision, ni masse ajoutée en rotation, ni amortissement par rayonnement — seulement
//! une traînée quadratique facultative par point immergé.

use crate::body::{Milieu, G};

/// Ce que le corps demande à l'eau : **B + W**, jamais δ (I-04).
pub trait WaterQuery {
    /// Altitude de la surface libre au point horizontal `(x, y)`, m.
    fn surface(&self, x: f64, y: f64) -> f64;
    /// Vitesse de l'eau en un point, m/s — orbitale sous la houle, nulle en eau calme.
    fn velocity(&self, p: [f64; 3]) -> [f64; 3];
}

/// Une eau calme, à l'altitude donnée.
#[derive(Clone, Copy, Debug)]
pub struct CalmWater {
    pub level: f64,
}

impl WaterQuery for CalmWater {
    fn surface(&self, _: f64, _: f64) -> f64 {
        self.level
    }
    fn velocity(&self, _: [f64; 3]) -> [f64; 3] {
        [0.; 3]
    }
}

/// Un point du proxy de flottabilité, dans le repère du corps : position par rapport au centre de masse,
/// volume représenté, épaisseur sur laquelle son immersion passe de 0 à 1, aire qu'il oppose à la traînée.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProxyPoint {
    pub body: [f64; 3],
    pub volume: f64,
    pub thickness: f64,
    pub area: f64,
}

/// Forces et moment que l'eau et la pesanteur exercent sur le corps, repère du monde, et le volume immergé.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Forces {
    pub force: [f64; 3],
    pub torque: [f64; 3],
    pub immersed_volume: f64,
}

/// Le corps rigide du jeu.
#[derive(Clone, Debug)]
pub struct RigidBody {
    /// Masse, kg.
    pub mass: f64,
    /// Moments principaux d'inertie, repère du corps, kg·m².
    pub inertia: [f64; 3],
    /// Masse ajoutée, diagonale, repère du monde, kg — l'eau que le corps entraîne en accélérant.
    pub added_mass: [f64; 3],
    /// Centre de masse, repère du monde, m.
    pub position: [f64; 3],
    /// Quaternion unitaire `(w, x, y, z)`, du corps vers le monde.
    pub orientation: [f64; 4],
    /// Vitesse du centre de masse, m/s.
    pub velocity: [f64; 3],
    /// Vitesse angulaire, repère du monde, rad/s.
    pub angular_velocity: [f64; 3],
    /// Le proxy de flottabilité — alloué une fois, parcouru à chaque pas.
    pub proxy: Vec<ProxyPoint>,
    /// Coefficient de traînée quadratique par point immergé ; 0 : aucune.
    pub drag: f64,
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

/// `q·v·q*` : un vecteur du repère du corps vers celui du monde.
pub fn rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let u = [q[1], q[2], q[3]];
    let t = scale(cross(u, v), 2.);
    add(add(v, scale(t, q[0])), cross(u, t))
}

/// `q*·v·q` : un vecteur du monde vers le repère du corps.
pub fn unrotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    rotate([q[0], -q[1], -q[2], -q[3]], v)
}

impl RigidBody {
    /// **Un pavé droit homogène**, côtés `size` (x, y, z), masse volumique `density`, centre en
    /// `position`, droit ; proxy de `layers` points par axe, aux centres d'une grille régulière.
    pub fn cuboid(size: [f64; 3], density: f64, position: [f64; 3], layers: [usize; 3]) -> RigidBody {
        let [a, b, c] = size;
        let mass = density * a * b * c;
        let inertia = [mass * (b * b + c * c) / 12., mass * (a * a + c * c) / 12., mass * (a * a + b * b) / 12.];
        let [nx, ny, nz] = layers;
        let (hx, hy, hz) = (a / nx as f64, b / ny as f64, c / nz as f64);
        let mut proxy = Vec::with_capacity(nx * ny * nz);
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    proxy.push(ProxyPoint {
                        body: [(i as f64 + 0.5) * hx - 0.5 * a, (j as f64 + 0.5) * hy - 0.5 * b, (k as f64 + 0.5) * hz - 0.5 * c],
                        volume: hx * hy * hz,
                        thickness: hz,
                        area: hx * hy,
                    });
                }
            }
        }
        RigidBody {
            mass,
            inertia,
            added_mass: [0.; 3],
            position,
            orientation: [1., 0., 0., 0.],
            velocity: [0.; 3],
            angular_velocity: [0.; 3],
            proxy,
            drag: 0.,
        }
    }

    /// Forces de pesanteur, de poussée et de traînée, et leur moment au centre de masse. Aucune
    /// allocation.
    pub fn forces(&self, water: &dyn WaterQuery, milieu: Milieu) -> Forces {
        let mut force = [0., 0., -self.mass * G];
        let mut torque = [0.; 3];
        let mut immersed_volume = 0.;
        for p in &self.proxy {
            let r = rotate(self.orientation, p.body);
            let x = add(self.position, r);
            let eta = water.surface(x[0], x[1]);
            let frac = ((eta - (x[2] - 0.5 * p.thickness)) / p.thickness).clamp(0., 1.);
            if frac == 0. {
                continue;
            }
            let immerse = p.volume * frac;
            immersed_volume += immerse;
            let mut f = [0., 0., milieu.rho * G * immerse];
            if self.drag > 0. {
                let rel = sub(add(self.velocity, cross(self.angular_velocity, r)), water.velocity(x));
                let k = -0.5 * milieu.rho * self.drag * p.area * frac * norm(rel);
                f = add(f, scale(rel, k));
            }
            force = add(force, f);
            torque = add(torque, cross(r, f));
        }
        Forces { force, torque, immersed_volume }
    }

    /// **Un pas symplectique** : vitesses d'abord, sous les forces de l'état présent ; positions et
    /// orientation ensuite, avec les vitesses nouvelles. Rend les forces employées.
    pub fn step(&mut self, dt: f64, water: &dyn WaterQuery, milieu: Milieu) -> Forces {
        let fr = self.forces(water, milieu);
        for a in 0..3 {
            self.velocity[a] += dt * fr.force[a] / (self.mass + self.added_mass[a]);
        }
        // Rotation : `I ω̇ = τ − ω × Iω`, écrite dans le repère du corps.
        let q = self.orientation;
        let wb = unrotate(q, self.angular_velocity);
        let tb = unrotate(q, fr.torque);
        let iw = [self.inertia[0] * wb[0], self.inertia[1] * wb[1], self.inertia[2] * wb[2]];
        let rhs = sub(tb, cross(wb, iw));
        let dwb = [rhs[0] / self.inertia[0], rhs[1] / self.inertia[1], rhs[2] / self.inertia[2]];
        self.angular_velocity = add(self.angular_velocity, scale(rotate(q, dwb), dt));
        for a in 0..3 {
            self.position[a] += dt * self.velocity[a];
        }
        let w = self.angular_velocity;
        // `q̇ = ½ (0, ω) ⊗ q`, puis renormalisation.
        let dq = [
            -0.5 * (w[0] * q[1] + w[1] * q[2] + w[2] * q[3]),
            0.5 * (w[0] * q[0] + w[1] * q[3] - w[2] * q[2]),
            0.5 * (w[1] * q[0] + w[2] * q[1] - w[0] * q[3]),
            0.5 * (w[2] * q[0] + w[0] * q[2] - w[1] * q[1]),
        ];
        let mut n = [q[0] + dt * dq[0], q[1] + dt * dq[1], q[2] + dt * dq[2], q[3] + dt * dq[3]];
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2] + n[3] * n[3]).sqrt();
        for x in &mut n {
            *x /= len;
        }
        self.orientation = n;
        fr
    }
}

#[cfg(test)]
#[path = "tests_rigid_body.rs"]
mod tests;
