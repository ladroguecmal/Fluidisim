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
//! celui de la géométrie, à l'arrondi près. **S333 :** la force est celle de la pression que le proxy
//! suppose, `ρg(η(x) − z)`, sur le volume plongé — `ρg(−∇η, 1)` : sa part horizontale entraîne le corps
//! avec la houle. La masse ajoutée est une matrice diagonale, dans le repère du monde ; l'intégrateur est
//! symplectique — vitesses d'abord, positions ensuite —, en `f64`, déterministe.
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
//! Ni contact, ni collision, ni masse ajoutée en rotation — seulement une traînée quadratique facultative par
//! point immergé et, depuis S336, un **amortissement de rayonnement** linéaire : une constante de l'archétype,
//! que δ mesure hors ligne comme la masse ajoutée, jamais une force de δ au pas. Depuis S336, la masse
//! ajoutée agit sur l'accélération **relative** à l'eau.

use crate::body::{Milieu, G};

/// Ce que le corps demande à l'eau : **B + W**, jamais δ (I-04).
pub trait WaterQuery {
    /// Altitude de la surface libre au point horizontal `(x, y)`, m.
    fn surface(&self, x: f64, y: f64) -> f64;
    /// **S333 :** pente de la surface libre, `(∂η/∂x, ∂η/∂y)` — nulle en eau calme.
    fn slope(&self, x: f64, y: f64) -> [f64; 2];
    /// Vitesse de l'eau en un point, m/s — orbitale sous la houle, nulle en eau calme.
    fn velocity(&self, p: [f64; 3]) -> [f64; 3];
    /// **S336 :** accélération de l'eau en un point, m/s² — ce que la masse ajoutée voit ; nulle en eau calme.
    fn acceleration(&self, p: [f64; 3]) -> [f64; 3];
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
    fn slope(&self, _: f64, _: f64) -> [f64; 2] {
        [0.; 2]
    }
    fn velocity(&self, _: [f64; 3]) -> [f64; 3] {
        [0.; 3]
    }
    fn acceleration(&self, _: [f64; 3]) -> [f64; 3] {
        [0.; 3]
    }
}

/// **S333 : l'eau de B derrière la requête du corps** — la houle analytique à l'instant `time`, interrogée
/// dans le repère local de son ancre, où vit le corps : le cœur ne convertit jamais de mètres en position du
/// monde à l'exécution. La vitesse est la vitesse orbitale de surface de B, sans atténuation en profondeur :
/// elle ne sert qu'à la traînée. Hors du rayon de référentiel, une eau calme au niveau zéro.
pub struct BackgroundWater<'a> {
    pub background: &'a crate::background::Background,
    pub time: crate::types::SimTime,
}

impl WaterQuery for BackgroundWater<'_> {
    fn surface(&self, x: f64, y: f64) -> f64 {
        self.background.eval_local([x as f32, y as f32, 0.], self.time).map_or(0., |s| s.eta as f64)
    }
    fn slope(&self, x: f64, y: f64) -> [f64; 2] {
        self.background.eval_local([x as f32, y as f32, 0.], self.time)
            .map_or([0.; 2], |s| [-(s.normal[0] / s.normal[2]) as f64, -(s.normal[1] / s.normal[2]) as f64])
    }
    fn velocity(&self, p: [f64; 3]) -> [f64; 3] {
        self.background.eval_local([p[0] as f32, p[1] as f32, 0.], self.time).map_or([0.; 3], |s| s.u_total.map(|v| v as f64))
    }
    fn acceleration(&self, p: [f64; 3]) -> [f64; 3] {
        self.background.acceleration_local([p[0] as f32, p[1] as f32, 0.], self.time).map_or([0.; 3], |a| a.map(|v| v as f64))
    }
}

/// **S494 : B + W derrière la requête du corps** — la houle de B et les impacts confirmés de W, composés par la composition
/// autoritaire ([`crate::prepared_water::Prepared::sample_local`], ADR-077), dans le repère local de l'ancre de B. Un point que
/// la composition refuse — hors du domaine d'un impact, au-delà de sa validité, pente au-delà de `max_slope` — rend B seul, et
/// le refus est **compté** (`refusals`) : l'hôte le lit, rien ne le masque. L'accélération de W est une différence centrée de
/// 1 ms sur sa vitesse de surface (B garde la sienne, analytique) : la masse ajoutée seule la lit.
///
/// **S495 : la pression** — le sillage d'un objet en marche, l'autre part de W —, publiée par son contrôleur à l'instant `time`
/// (`pressure`), entre dans la même composition (`prepared_water::mixed::sample_local`, ADR-077 : B, impacts puis pressions, une
/// seule normalisation). Une pression publiée à un autre instant est refusée (comptée). L'accélération ne porte que celle des
/// impacts : la pression n'est publiée qu'à `time`.
pub struct MixedWater<'a, 'j, const N: usize = 64> {
    pub bound: &'a crate::prepared_water::BoundBackground<'a>,
    pub impacts: &'a crate::prepared_water::Prepared<'a, 'j, N>,
    pub pressure: Option<&'a crate::bound_pressure::Prepared<'a>>,
    pub time: crate::types::SimTime,
    pub max_slope: f32,
    pub refusals: core::cell::Cell<u64>,
}

impl<const N: usize> MixedWater<'_, '_, N> {
    fn refus(&self, x: f64, y: f64, time: crate::types::SimTime) -> Option<crate::types::WaterSample> {
        self.refusals.set(self.refusals.get() + 1);
        self.bound.binding().0.eval_local([x as f32, y as f32, 0.], time)
    }
    fn sample(&self, x: f64, y: f64, time: crate::types::SimTime) -> Option<crate::types::WaterSample> {
        crate::prepared_water::mixed::sample_local(self.bound, self.impacts, self.pressure, time, [x as f32, y as f32], self.max_slope)
            .ok()
            .or_else(|| self.refus(x, y, time))
    }
    /// La vitesse de surface des impacts seuls : B et les impacts composés, moins B, au même point et au même instant.
    fn w_velocity(&self, x: f64, y: f64, time: crate::types::SimTime) -> [f64; 3] {
        let b = self.bound.binding().0.eval_local([x as f32, y as f32, 0.], time).map_or([0.; 3], |s| s.u_total.map(|v| v as f64));
        let m = self.impacts.sample_local(self.bound, [x as f32, y as f32], time, self.max_slope).ok().or_else(|| self.refus(x, y, time));
        let m = m.map_or([0.; 3], |s| s.u_total.map(|v| v as f64));
        [m[0] - b[0], m[1] - b[1], m[2] - b[2]]
    }
}

impl<const N: usize> WaterQuery for MixedWater<'_, '_, N> {
    fn surface(&self, x: f64, y: f64) -> f64 {
        self.sample(x, y, self.time).map_or(0., |s| s.eta as f64)
    }
    fn slope(&self, x: f64, y: f64) -> [f64; 2] {
        self.sample(x, y, self.time).map_or([0.; 2], |s| [-(s.normal[0] / s.normal[2]) as f64, -(s.normal[1] / s.normal[2]) as f64])
    }
    fn velocity(&self, p: [f64; 3]) -> [f64; 3] {
        self.sample(p[0], p[1], self.time).map_or([0.; 3], |s| s.u_total.map(|v| v as f64))
    }
    fn acceleration(&self, p: [f64; 3]) -> [f64; 3] {
        let b = self.bound.binding().0;
        let mut a = b.acceleration_local([p[0] as f32, p[1] as f32, 0.], self.time).map_or([0.; 3], |a| a.map(|v| v as f64));
        let (avant, apres) = (crate::types::SimTime(self.time.0.saturating_sub(1000)), crate::types::SimTime(self.time.0 + 1000));
        let (u0, u1) = (self.w_velocity(p[0], p[1], avant), self.w_velocity(p[0], p[1], apres));
        let h = (apres.0 - avant.0) as f64 * 1e-6;
        for k in 0..3 {
            a[k] += (u1[k] - u0[k]) / h;
        }
        a
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
    /// **S336 : amortissement de rayonnement**, diagonal, repère du monde, N·s/m — linéaire en la vitesse du
    /// centre de masse relative à l'eau qui le porte. Constante de l'archétype (ADR-008 §2), mesurée par δ ; 0 :
    /// aucun.
    pub radiation_damping: [f64; 3],
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
            radiation_damping: [0.; 3],
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
            // S333 : la force de la pression que le proxy suppose, `p = ρg(η(x) − z)`, sur le volume plongé,
            // est `ρg(−∇η, 1)` — sa part horizontale entraîne le corps avec la houle (ADR-008 §2). En eau
            // calme, rien ne change au bit.
            let poussee = milieu.rho * G * immerse;
            let mut f = [0., 0., poussee];
            let pente = water.slope(x[0], x[1]);
            if pente != [0.; 2] {
                f[0] = -poussee * pente[0];
                f[1] = -poussee * pente[1];
            }
            if self.drag > 0. {
                let rel = sub(add(self.velocity, cross(self.angular_velocity, r)), water.velocity(x));
                let k = -0.5 * milieu.rho * self.drag * p.area * frac * norm(rel);
                f = add(f, scale(rel, k));
            }
            force = add(force, f);
            // S500 (ADR-227) : la poussée d'un point en partie immergé s'applique au centre de sa part immergée — son milieu abaissé
            // de `(1 − f)·e/2` le long de l'axe du corps —, pas en son milieu : la hauteur du centre de carène exacte à une couche
            // (B6, S499). La force ne change pas ; un point noyé, ni son moment.
            let bras = if frac < 1. { add(r, rotate(self.orientation, [0., 0., -(1. - frac) * 0.5 * p.thickness])) } else { r };
            torque = add(torque, cross(bras, f));
        }
        // S336 : l'eau que la coque met en mouvement emporte son énergie en ondes — un amortissement linéaire en la
        // vitesse relative à l'eau au centre de masse. Nul par défaut : rien ne change.
        if self.radiation_damping != [0.; 3] {
            let [x, y, _] = self.position;
            let u = water.velocity([x, y, water.surface(x, y)]);
            for a in 0..3 {
                force[a] -= self.radiation_damping[a] * (self.velocity[a] - u[a]);
            }
        }
        // S336 : la masse ajoutée agit sur l'accélération relative à l'eau — `(m + A)·v̇ = F + A·a_eau` : l'eau
        // accélérée pousse la coque de `A·a_eau`. Sans ce terme, sur la houle, la coque surréagit ; en eau calme,
        // il est nul.
        if self.added_mass != [0.; 3] {
            let [x, y, _] = self.position;
            let a = water.acceleration([x, y, water.surface(x, y)]);
            if a != [0.; 3] {
                for k in 0..3 {
                    force[k] += self.added_mass[k] * a[k];
                }
            }
        }
        Forces { force, torque, immersed_volume }
    }

    /// **S497 — le tronçon de sillage d'un corps en marche** (liste 6.3) : le tronçon suivant de l'émetteur de sillage (ADR-104),
    /// visé du `cursor` — où la source en est — vers la position **prédite** du corps à la fin du tronçon, `x + v·Δ`, sous la charge
    /// que porte sa coque, son poids `m·g` (ADR-103 : une charge prescrite par l'hôte ; c'est elle). Le chemin de la source reste
    /// continu, comme l'émetteur l'exige, et se recale sur le corps à chaque tronçon : l'écart en fin de tronçon est celui de la
    /// prédiction, `½·|a|·Δ²` pour une accélération `a` constante, et ne s'accumule pas.
    pub fn wake_leg(&self, cursor: crate::wake_source::Cursor, duration_us: u64) -> crate::wake_source::Leg {
        let d = duration_us as f64 * 1e-6;
        let vise = |k: usize| ((self.position[k] + self.velocity[k] * d - cursor.position[k] as f64) / d) as f32;
        crate::wake_source::Leg { duration_us, velocity: [vise(0), vise(1)], downward_force_n: (self.mass * G) as f32 }
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

/// **S498 — le régime d'intégration d'un corps flottant** (ADR-008 §3), selon `ω·dt` : normal jusqu'à 0,3 ; sous-cyclé (2 à 4 sous-pas)
/// jusqu'à 1 ; au-delà, **contraint** — projeté sur la surface, sans force de flottabilité.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regime {
    Normal,
    Subcycled(u32),
    Constrained,
}

/// **S498 — ce qu'un corps flottant calcule à sa création** (ADR-008 §3) : la pulsation de pilonnement `ω = √(k/(m + m_a))` en eau
/// calme à l'équilibre, le régime au pas `dt` de l'hôte, et l'altitude d'équilibre de son centre au-dessus de la surface, `c`, que le
/// mode contraint lui impose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Floating {
    pub omega: f64,
    pub regime: Regime,
    pub offset: f64,
}

impl RigidBody {
    /// **S498 : la raideur de flottaison**, `ρg·Σ V/e` sur les points du proxy dans leur rampe (ni secs ni noyés) : pour un pavé droit à
    /// une couche partielle par colonne, `ρg·A` exactement.
    pub fn heave_stiffness(&self, water: &dyn WaterQuery, milieu: Milieu) -> f64 {
        let mut k = 0.;
        for p in &self.proxy {
            let x = add(self.position, rotate(self.orientation, p.body));
            let frac = (water.surface(x[0], x[1]) - (x[2] - 0.5 * p.thickness)) / p.thickness;
            if frac > 0. && frac < 1. {
                k += milieu.rho * G * p.volume / p.thickness;
            }
        }
        k
    }

    /// **S498 : l'altitude d'équilibre du centre au-dessus d'une surface plane**, le corps droit : le volume immergé égal à `m/ρ`, par
    /// dichotomie sur l'étendue du proxy (soixante itérations : au bit de la précision double).
    pub fn equilibrium_offset(&self, milieu: Milieu) -> f64 {
        let mut droit = self.clone();
        droit.orientation = [1., 0., 0., 0.];
        let etendue = self.proxy.iter().map(|p| p.body[2].abs() + p.thickness).fold(0., f64::max);
        let (mut bas, mut haut) = (-etendue, etendue);
        for _ in 0..60 {
            let c = 0.5 * (bas + haut);
            droit.position = [0., 0., c];
            let v = droit.forces(&CalmWater { level: 0. }, milieu).immersed_volume;
            if v * milieu.rho > self.mass {
                bas = c;
            } else {
                haut = c;
            }
        }
        0.5 * (bas + haut)
    }

    /// **S498 : la création d'un corps flottant** (ADR-008 §3) : `ω` en eau calme à l'équilibre, le régime au pas `dt`.
    pub fn floating(&self, dt: f64, milieu: Milieu) -> Floating {
        let offset = self.equilibrium_offset(milieu);
        let mut droit = self.clone();
        droit.orientation = [1., 0., 0., 0.];
        droit.position = [0., 0., offset];
        let omega = (droit.heave_stiffness(&CalmWater { level: 0. }, milieu) / (self.mass + self.added_mass[2])).sqrt();
        let x = omega * dt;
        let regime = if x <= 0.3 {
            Regime::Normal
        } else if x <= 1. {
            Regime::Subcycled(((x / 0.3).ceil() as u32).clamp(2, 4))
        } else {
            Regime::Constrained
        };
        Floating { omega, regime, offset }
    }

    /// **S498 : un pas selon le régime** (ADR-008 §3). Normal : [`RigidBody::step`] sous `water`, l'eau au début du pas. Sous-cyclé :
    /// `n` pas de `dt/n` sous la même eau. **Contraint** : aucune force de flottabilité ; la vitesse horizontale relaxée vers celle de
    /// l'eau au taux `ω`, la position avancée, puis **projetée** sur la surface de `next` — l'eau à la fin du pas — : `z = η + c`,
    /// l'axe du corps sur la normale, sans rotation propre. Exactement stable, sans coût.
    pub fn step_floating(&mut self, f: &Floating, dt: f64, water: &dyn WaterQuery, next: &dyn WaterQuery, milieu: Milieu) {
        match f.regime {
            Regime::Normal => {
                self.step(dt, water, milieu);
            }
            Regime::Subcycled(n) => {
                for _ in 0..n {
                    self.step(dt / n as f64, water, milieu);
                }
            }
            Regime::Constrained => {
                let [x, y, z] = self.position;
                let u = next.velocity([x, y, next.surface(x, y)]);
                let r = (-f.omega * dt).exp();
                for k in 0..2 {
                    self.velocity[k] = u[k] + (self.velocity[k] - u[k]) * r;
                    self.position[k] += dt * self.velocity[k];
                }
                let [x, y, _] = self.position;
                self.position[2] = next.surface(x, y) + f.offset;
                self.velocity[2] = (self.position[2] - z) / dt;
                self.orientation = surface_tilt(next.slope(x, y));
                self.angular_velocity = [0.; 3];
            }
        }
    }
}

/// **S332 : la distance signée d'un pavé orienté** — centre `c`, quaternion `q`, demi-côtés `h` — au point
/// `p` du monde, négative dedans : la coque d'un corps, telle que δ la reçoit aux nœuds de sa grille.
pub fn oriented_box_distance(c: [f64; 3], q: [f64; 4], h: [f64; 3], p: [f64; 3]) -> f64 {
    let l = unrotate(q, sub(p, c));
    let d = [l[0].abs() - h[0], l[1].abs() - h[1], l[2].abs() - h[2]];
    let dehors = (d[0].max(0.).powi(2) + d[1].max(0.).powi(2) + d[2].max(0.).powi(2)).sqrt();
    dehors + d[0].max(d[1]).max(d[2]).min(0.)
}

/// Produit de quaternions `(w, x, y, z)`.
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

/// **S333 : l'inclinaison de la surface** de pente `(∂η/∂x, ∂η/∂y)` — le quaternion qui porte la verticale sur
/// la normale, `(1 + n_z, ẑ × n)` normalisé. Une surface plane rend l'identité, exactement.
pub fn surface_tilt(slope: [f64; 2]) -> [f64; 4] {
    let n = (1. + slope[0] * slope[0] + slope[1] * slope[1]).sqrt();
    let q = [n + 1., slope[1], -slope[0], 0.];
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
    [q[0] / len, q[1] / len, q[2] / len, 0.]
}

/// Ce que δ reçoit d'une coque à un pas : sa pose dans la grille, et la vitesse de sa paroi — translation, et
/// rotation autour du centre —, tout en f32.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallMotion {
    pub center: [f32; 3],
    pub orientation: [f32; 4],
    pub velocity: [f32; 3],
    pub angular: [f32; 3],
}

/// **S333 : la coque dans δ, relative à l'eau qui la porte.** δ porte la perturbation — rayonnement et
/// diffraction —, pas la houle, qui est à B + W : la coque y entre à sa pose **relative** à l'eau qui la
/// porte, et sa paroi avance à la vitesse relative. Théorie linéaire d'une coque courte devant la longueur
/// d'onde : l'eau est lue au centre de la coque — élévation, inclinaison, vitesse horizontale. Hauteur et
/// inclinaison se lisent directement ; la position horizontale relative s'**intègre**, `Σ dt·(V − u)`, parce
/// que l'excursion de la particule ne se lit, en eulérien, qu'au second ordre près. La pose est arrondie en
/// f32, comme δ la reçoit, et les vitesses de paroi en sont la **différence finie** : découpe et paroi
/// restent cohérentes au bit, et une coque qui suit l'eau garde exactement la même pose. En eau calme, c'est
/// la pose absolue de S332, translatée dans la grille.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HullInDelta {
    /// Position horizontale relative du centre, dans la grille de δ, m.
    horizontal: [f64; 2],
    /// Altitude, dans la grille de δ, du couvercle où se tient la surface de l'eau qui porte la coque.
    lid: f64,
    /// La dernière pose rendue à δ.
    pub center: [f32; 3],
    pub orientation: [f32; 4],
}

impl HullInDelta {
    /// La coque au centre `body.position`, posée dans une grille dont le coin horizontal est au point du monde
    /// `origin` et le couvercle à l'altitude `lid`. `water` est l'eau à l'instant de `body`.
    pub fn new(body: &RigidBody, water: &dyn WaterQuery, origin: [f64; 2], lid: f64) -> HullInDelta {
        let mut h = HullInDelta {
            horizontal: [body.position[0] - origin[0], body.position[1] - origin[1]],
            lid,
            center: [0.; 3],
            orientation: [1., 0., 0., 0.],
        };
        (h.center, h.orientation) = h.pose(body, water);
        h
    }

    /// Pose relative : hauteur au-dessus de la surface, orientation dans le repère incliné de la surface.
    fn pose(&self, body: &RigidBody, water: &dyn WaterQuery) -> ([f32; 3], [f32; 4]) {
        let [x, y, z] = body.position;
        let t = surface_tilt(water.slope(x, y));
        let q = quat_mul([t[0], -t[1], -t[2], -t[3]], body.orientation);
        (
            [self.horizontal[0] as f32, self.horizontal[1] as f32, (self.lid + (z - water.surface(x, y))) as f32],
            q.map(|v| v as f32),
        )
    }

    /// Après un pas `dt` du corps, `water` à l'instant nouveau : la pose relative avance, et la paroi reçoit la
    /// différence finie des deux poses — vitesse de translation, et rotation `2·vec(q₁·q̄₀)/dt`.
    pub fn advance(&mut self, body: &RigidBody, water: &dyn WaterQuery, dt: f64) -> WallMotion {
        let [x, y, _] = body.position;
        let u = water.velocity([x, y, water.surface(x, y)]);
        self.horizontal[0] += dt * (body.velocity[0] - u[0]);
        self.horizontal[1] += dt * (body.velocity[1] - u[1]);
        let (c0, q0) = (self.center, self.orientation);
        (self.center, self.orientation) = self.pose(body, water);
        let pas = dt as f32;
        let velocity = [0, 1, 2].map(|a| (self.center[a] - c0[a]) / pas);
        let mut d = quat_mul(self.orientation.map(|v| v as f64), [q0[0] as f64, -q0[1] as f64, -q0[2] as f64, -q0[3] as f64]);
        if d[0] < 0. {
            d = d.map(|v| -v);
        }
        let angular = [1, 2, 3].map(|a| (2. * d[a] / dt) as f32);
        WallMotion { center: self.center, orientation: self.orientation, velocity, angular }
    }

    /// La distance signée de la coque — un pavé de demi-côtés `half` — aux nœuds d'une grille de `cells`
    /// mailles de côté `dx`, `x` le plus rapide : ce que `Volume3` reçoit.
    pub fn box_nodes(&self, half: [f64; 3], cells: [usize; 3], dx: f64, out: &mut Vec<f32>) {
        let c = self.center.map(|v| v as f64);
        let q = self.orientation.map(|v| v as f64);
        out.clear();
        for k in 0..=cells[2] {
            for j in 0..=cells[1] {
                for i in 0..=cells[0] {
                    out.push(oriented_box_distance(c, q, half, [i as f64 * dx, j as f64 * dx, k as f64 * dx]) as f32);
                }
            }
        }
    }
}

/// **S332 : le décalage visuel que δ rend au corps** — ADR-008 §1 : un ressort borné entre la pose physique
/// et la pose affichée. La force de δ l'excite ; **la trajectoire de jeu ne la voit jamais** (I-04). Un
/// client qui ne simule pas δ montre le même corps au même endroit, avec un peu moins de vie. Translation
/// seule : la part en rotation, ≤ 3°, manque encore.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderOffset {
    pub offset: [f64; 3],
    pub velocity: [f64; 3],
    /// Pulsation propre du ressort, rad/s, et son amortissement réduit.
    pub omega: f64,
    pub zeta: f64,
    /// Borne du décalage, m — 8 cm (ADR-008 §1).
    pub limit: f64,
}

impl RenderOffset {
    pub fn new(omega: f64, zeta: f64) -> RenderOffset {
        RenderOffset { offset: [0.; 3], velocity: [0.; 3], omega, zeta, limit: 0.08 }
    }

    /// Un pas du ressort sous la force de δ, rapportée à la masse du corps ; la butée retire la vitesse qui
    /// l'enfoncerait. Rend le décalage.
    pub fn step(&mut self, dt: f64, force: [f64; 3], mass: f64) -> [f64; 3] {
        for a in 0..3 {
            let acc = force[a] / mass - self.omega * self.omega * self.offset[a] - 2. * self.zeta * self.omega * self.velocity[a];
            self.velocity[a] += dt * acc;
            self.offset[a] += dt * self.velocity[a];
        }
        let n = norm(self.offset);
        if n > self.limit {
            self.offset = scale(self.offset, self.limit / n);
            let r = scale(self.offset, 1. / self.limit);
            let vr = self.velocity[0] * r[0] + self.velocity[1] * r[1] + self.velocity[2] * r[2];
            if vr > 0. {
                self.velocity = sub(self.velocity, scale(r, vr));
            }
        }
        self.offset
    }
}

#[cfg(test)]
#[path = "tests_rigid_body.rs"]
mod tests;
