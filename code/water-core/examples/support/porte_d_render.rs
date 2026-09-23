//! S333 — rendu local de la scène de la porte D : une vue en perspective de la surface ombrée et de la coque,
//! et une carte de δ vue de dessus. Rastérisation CPU, sans dépendance ; PPM seulement (ADR-124).
//!
//! **Habillage de banc**, qui ne relève pas du système d'eau : ciel en dégradé, soleil, couleur de l'eau,
//! Fresnel de Schlick, coque grise sans texture. Ni écume, ni réfraction, ni sous-surface.
use std::{fs::File, io::{BufWriter, Write}};

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn normalize(a: [f32; 3]) -> [f32; 3] {
    let l = dot(a, a).sqrt();
    a.map(|v| v / l)
}

/// Direction de la lumière, de côté, à droite de la caméra et à 25° : aucun reflet solaire dans le champ, et
/// les pentes de l'eau se lisent comme un relief — un choix de banc pour rendre δ lisible, pas un rendu d'eau.
const SOLEIL: [f32; 3] = [0.77, -0.64, 0.47];

fn ciel(dz: f32) -> [f32; 3] {
    // Racine : le ciel change vite près de l'horizon, là où une eau presque plane le réfléchit.
    let t = dz.clamp(0., 1.).sqrt();
    [150. - 90. * t, 185. - 70. * t, 215. - 40. * t]
}

/// Une image RGB avec son tampon de profondeur.
pub struct Image {
    pub w: usize,
    pub h: usize,
    rgb: Vec<u8>,
    depth: Vec<f32>,
}

impl Image {
    /// Fond de ciel en dégradé.
    pub fn new(w: usize, h: usize) -> Image {
        let mut rgb = vec![0u8; w * h * 3];
        for y in 0..h {
            let c = ciel(1. - y as f32 / h as f32);
            for x in 0..w {
                rgb[(y * w + x) * 3..(y * w + x) * 3 + 3].copy_from_slice(&c.map(|v| v as u8));
            }
        }
        Image { w, h, rgb, depth: vec![f32::INFINITY; w * h] }
    }

    fn put(&mut self, x: usize, y: usize, c: [u8; 3]) {
        let q = (y * self.w + x) * 3;
        self.rgb[q..q + 3].copy_from_slice(&c);
    }

    /// Écrit le PPM et rend son empreinte FNV-1a, en-tête compris.
    pub fn save(&self, path: &str) -> std::io::Result<u64> {
        let header = format!("P6\n{} {}\n255\n", self.w, self.h);
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for b in header.bytes().chain(self.rgb.iter().copied()) {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
        let mut f = BufWriter::new(File::create(path)?);
        f.write_all(header.as_bytes())?;
        f.write_all(&self.rgb)?;
        Ok(hash)
    }
}

/// Caméra perspective sur un panneau `[x0, x0 + w[` de l'image.
pub struct Camera {
    eye: [f32; 3],
    fwd: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    focal: f32,
    x0: usize,
    w: usize,
    h: usize,
}

impl Camera {
    pub fn look(eye: [f32; 3], target: [f32; 3], fov_deg: f32, x0: usize, w: usize, h: usize) -> Camera {
        let fwd = normalize(sub(target, eye));
        let right = normalize(cross(fwd, [0., 0., 1.]));
        let up = cross(right, fwd);
        let focal = 0.5 * h as f32 / (0.5 * fov_deg.to_radians()).tan();
        Camera { eye, fwd, right, up, focal, x0, w, h }
    }

    /// Point d'écran `(x, y, profondeur)` ; `None` derrière la caméra.
    fn project(&self, p: [f32; 3]) -> Option<[f32; 3]> {
        let d = sub(p, self.eye);
        let z = dot(d, self.fwd);
        if z <= 0.1 {
            return None;
        }
        Some([self.x0 as f32 + 0.5 * self.w as f32 + self.focal * dot(d, self.right) / z,
            0.5 * self.h as f32 - self.focal * dot(d, self.up) / z, z])
    }

    /// Un triangle du monde, couleur plate, dans le tampon de profondeur, borné au panneau.
    pub fn triangle(&self, img: &mut Image, p: [[f32; 3]; 3], color: [u8; 3]) {
        let c = color.map(|v| v as f32);
        self.shaded(img, p, [c, c, c]);
    }

    /// Un triangle du monde, couleurs aux sommets interpolées (Gouraud), dans le tampon de profondeur.
    fn shaded(&self, img: &mut Image, p: [[f32; 3]; 3], colors: [[f32; 3]; 3]) {
        let (Some(a), Some(b), Some(c)) = (self.project(p[0]), self.project(p[1]), self.project(p[2])) else { return };
        let q = [a, b, c];
        let edge = |a: [f32; 3], b: [f32; 3], x: f32, y: f32| (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0]);
        let area = edge(q[0], q[1], q[2][0], q[2][1]);
        if area.abs() < 1e-9 {
            return;
        }
        let xmin = q.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min).floor().max(self.x0 as f32) as usize;
        let xmax = q.iter().map(|p| p[0]).fold(f32::NEG_INFINITY, f32::max).ceil().min((self.x0 + self.w - 1) as f32);
        let ymin = q.iter().map(|p| p[1]).fold(f32::INFINITY, f32::min).floor().max(0.) as usize;
        let ymax = q.iter().map(|p| p[1]).fold(f32::NEG_INFINITY, f32::max).ceil().min((self.h - 1) as f32);
        if xmax < 0. || ymax < 0. {
            return;
        }
        for y in ymin..=ymax as usize {
            for x in xmin..=xmax as usize {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let wa = edge(q[1], q[2], px, py) / area;
                let wb = edge(q[2], q[0], px, py) / area;
                let wc = 1. - wa - wb;
                if wa < 0. || wb < 0. || wc < 0. {
                    continue;
                }
                let z = 1. / (wa / q[0][2] + wb / q[1][2] + wc / q[2][2]);
                let i = y * img.w + x;
                if z < img.depth[i] {
                    img.depth[i] = z;
                    let c = [0, 1, 2].map(|k| (wa * colors[0][k] + wb * colors[1][k] + wc * colors[2][k]).clamp(0., 255.) as u8);
                    img.put(x, y, c);
                }
            }
        }
    }

    /// La couleur de l'eau en un point de normale `n` vu depuis l'œil : fond bleu-vert, ciel réfléchi par
    /// Fresnel de Schlick.
    fn water(&self, p: [f32; 3], n: [f32; 3]) -> [f32; 3] {
        let v = normalize(sub(self.eye, p));
        let cos = dot(n, v).max(0.);
        let fresnel = 0.02 + 0.98 * (1. - cos).powi(5);
        let r = sub(n.map(|x| 2. * dot(n, v) * x), v);
        let sky = ciel(r[2].max(0.));
        let eclaire = 0.2 + 2. * dot(n, normalize(SOLEIL)).max(0.);
        let deep = [8. * eclaire, 46. * eclaire, 62. * eclaire];
        [0, 1, 2].map(|c| deep[c] * (1. - fresnel) + 1.1 * sky[c] * fresnel)
    }

    /// La surface `z = height(i, j)` aux nœuds `(x0 + i·step, y0 + j·step)`, `n × n` nœuds ; normales des
    /// différences centrées, couleur aux nœuds, interpolée.
    pub fn surface(&self, img: &mut Image, n: usize, origin: [f32; 2], step: f32, height: &dyn Fn(usize, usize) -> f32) {
        let z: Vec<f32> = (0..n * n).map(|q| height(q % n, q / n)).collect();
        let p = |i: usize, j: usize| [origin[0] + i as f32 * step, origin[1] + j as f32 * step, z[j * n + i]];
        let couleur: Vec<[f32; 3]> = (0..n * n)
            .map(|q| {
                let (i, j) = (q % n, q / n);
                let (a, b) = (i.saturating_sub(1), (i + 1).min(n - 1));
                let (c, d) = (j.saturating_sub(1), (j + 1).min(n - 1));
                let gx = (z[j * n + b] - z[j * n + a]) / ((b - a) as f32 * step);
                let gy = (z[d * n + i] - z[c * n + i]) / ((d - c) as f32 * step);
                self.water(p(i, j), normalize([-gx, -gy, 1.]))
            })
            .collect();
        for j in 0..n - 1 {
            for i in 0..n - 1 {
                for t in [[(i, j), (i + 1, j), (i + 1, j + 1)], [(i, j), (i + 1, j + 1), (i, j + 1)]] {
                    self.shaded(img, t.map(|(i, j)| p(i, j)), t.map(|(i, j)| couleur[j * n + i]));
                }
            }
        }
    }

    /// Un pavé de centre `c`, de demi-côtés `h`, tourné par le quaternion `q` (du corps vers le monde) : coque
    /// grise, pont plus sombre, éclairage de Lambert.
    pub fn hull(&self, img: &mut Image, c: [f32; 3], q: [f32; 4], h: [f32; 3]) {
        let rot = |v: [f32; 3]| {
            let u = [q[1], q[2], q[3]];
            let t = cross(u, v).map(|x| 2. * x);
            let r = cross(u, t);
            [v[0] + q[0] * t[0] + r[0], v[1] + q[0] * t[1] + r[1], v[2] + q[0] * t[2] + r[2]]
        };
        let corner = |s: [f32; 3]| {
            let l = rot([s[0] * h[0], s[1] * h[1], s[2] * h[2]]);
            [c[0] + l[0], c[1] + l[1], c[2] + l[2]]
        };
        // Six faces : axe, signe ; leurs quatre coins parcourus dans l'ordre.
        for (axis, sign) in [(0, 1.), (0, -1.), (1, 1.), (1, -1.), (2, 1.), (2, -1.)] {
            let (a, b) = ((axis + 1) % 3, (axis + 2) % 3);
            let mut s = [[0f32; 3]; 4];
            for (k, (da, db)) in [(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)].iter().enumerate() {
                s[k][axis] = sign;
                s[k][a] = *da;
                s[k][b] = *db;
            }
            let w = s.map(corner);
            let mut normal = [0f32; 3];
            normal[axis] = sign;
            let n = rot(normal);
            let light = 0.35 + 0.65 * dot(n, normalize(SOLEIL)).max(0.);
            let base = if axis == 2 && sign > 0. { [120., 112., 100.] } else { [214., 214., 206.] };
            let color = base.map(|v| (v * light).clamp(0., 255.) as u8);
            self.triangle(img, [w[0], w[1], w[2]], color);
            self.triangle(img, [w[0], w[2], w[3]], color);
        }
    }
}

/// Carte de δ vue de dessus : bleu dessous, rouge dessus, saturés à `± echelle` ; colonnes couvertes par la coque
/// en gris ; un trait tous les 2 m.
pub fn delta_map(nx: usize, ny: usize, dx: f32, delta: &[f32], covered: &[bool], echelle: f32, pixels: usize) -> Image {
    let mut img = Image::new(pixels, pixels);
    let s = pixels as f32 / (nx.max(ny) as f32);
    for y in 0..pixels {
        for x in 0..pixels {
            let (i, j) = ((x as f32 / s) as usize, ny - 1 - ((y as f32 / s) as usize).min(ny - 1));
            if i >= nx {
                continue;
            }
            let c = j * nx + i;
            let color = if covered[c] {
                [150, 150, 150]
            } else {
                let f = (delta[c] / echelle).clamp(-1., 1.);
                if f >= 0. { [255, (255. * (1. - f)) as u8, (255. * (1. - f)) as u8] } else { [(255. * (1. + f)) as u8, (255. * (1. + f)) as u8, 255] }
            };
            let metre_x = ((x as f32 / s) * dx) % 2.;
            let metre_y = ((y as f32 / s) * dx) % 2.;
            let trait_ = metre_x < dx / s * 1.5 || metre_y < dx / s * 1.5;
            img.put(x, y, if trait_ { color.map(|v| (v as f32 * 0.8) as u8) } else { color });
        }
    }
    img
}
