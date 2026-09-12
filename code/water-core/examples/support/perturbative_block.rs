//! S185 : le bloc perturbatif et sa géométrie, partagés par `perturbative_step` (S184) et
//! `cadence_error` (S185). **Un seul pas, écrit une fois.** Deux véhicules qui évolueraient
//! chacun leur copie du même pas divergeraient, et la comparaison entre les deux sessions ne
//! voudrait plus rien dire — c'est le mécanisme de L137, transposé au code d'essai.
use water_core::WorldPos;

pub const DX: f64 = 0.25;
/// Viscosité cinématique de banc, m²/s — ADR-114 n'en fixe aucune par défaut.
pub const NU: f32 = 1.0e-6;
/// Pas de réseau éprouvés ; la géométrie du bloc en dépend, parce que le nœud supérieur
/// d'un réseau grossier déborde du bloc et doit rester dans le domaine.
pub const RATIOS: [usize; 4] = [1, 2, 4, 8];

/// Nombre de nœuds par axe pour un pas de réseau `r`, couvrant les mailles intérieures.
pub fn nodes_per_axis(n: usize, r: usize) -> usize {
    let span = n - 3; // indices intérieurs 1..=n-2, donc `span+1` positions
    span / r + if span % r == 0 { 1 } else { 2 }
}
/// Le plus grand indice de maille qu'un réseau atteigne.
pub fn reach(n: usize) -> usize {
    RATIOS
        .iter()
        .map(|r| 1 + (nodes_per_axis(n, *r) - 1) * r)
        .max()
        .unwrap()
}
/// Coin du bloc en local, m. Placé pour que **tout** nœud de **tout** réseau vérifie
/// `z <= 0` (exigence de `differential_local`) et reste dans la boîte de pression et le
/// rayon d'impact. Le bloc est donc d'autant plus profond qu'il est grand.
pub fn base(n: usize) -> [f64; 3] {
    let span = reach(n) as f64 * DX;
    [1.0 - 0.5 * span, 1.0 - 0.5 * span, -0.05 - span]
}
pub fn point(local: [f64; 3]) -> WorldPos {
    WorldPos::from_metres(1e9 + local[0], -1e9 + local[1], local[2])
}
pub fn cell_local(n: usize, i: usize, j: usize, k: usize) -> [f64; 3] {
    let b = base(n);
    [
        b[0] + i as f64 * DX,
        b[1] + j as f64 * DX,
        b[2] + k as f64 * DX,
    ]
}
pub fn lattice_points(n: usize, r: usize) -> Vec<WorldPos> {
    let m = nodes_per_axis(n, r);
    let mut v = Vec::with_capacity(m * m * m);
    for a in 0..m {
        for b in 0..m {
            for c in 0..m {
                v.push(point(cell_local(n, 1 + a * r, 1 + b * r, 1 + c * r)));
            }
        }
    }
    v
}
pub fn interior_points(n: usize) -> Vec<WorldPos> {
    let mut v = Vec::with_capacity((n - 2).pow(3));
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                v.push(point(cell_local(n, i, j, k)));
            }
        }
    }
    v
}

/// Bloc de mailles. Les mailles intérieures sont `1..n-1` sur chaque axe ; la couche
/// extérieure est fantôme et n'est jamais évoluée.
pub struct Block {
    pub n: usize,
    pub u: Vec<[f32; 3]>,
    pub next: Vec<[f32; 3]>,
    /// Source par maille intérieure, rangée dans le même indexage que `u`.
    pub s: Vec<[f32; 3]>,
}
impl Block {
    /// État de départ non nul et déterministe : un pas sur zéro ne mesurerait ni
    /// l'advection ni le laplacien.
    pub fn new(n: usize) -> Self {
        let total = n * n * n;
        let u = (0..total)
            .map(|t| {
                let f = t as f32 * 0.001;
                [
                    0.01 * (f).sin(),
                    0.01 * (1.7 * f).cos(),
                    0.005 * (0.3 * f).sin(),
                ]
            })
            .collect();
        Self {
            n,
            u,
            next: vec![[0.0; 3]; total],
            s: vec![[0.0; 3]; total],
        }
    }
    /// État de départ nul : `u'(T)` est alors **entièrement** ce que la source a produit,
    /// et une erreur relative a un dénominateur qui veut dire quelque chose.
    pub fn at_rest(n: usize) -> Self {
        let total = n * n * n;
        Self {
            n,
            u: vec![[0.0; 3]; total],
            next: vec![[0.0; 3]; total],
            s: vec![[0.0; 3]; total],
        }
    }
    #[inline]
    pub fn at(&self, i: usize, j: usize, k: usize) -> usize {
        (i * self.n + j) * self.n + k
    }
    pub fn interior(&self) -> usize {
        (self.n - 2).pow(3)
    }
    /// Un pas explicite. `with_source` décide si le terme `−S` est appliqué ; c'est la
    /// **seule** différence entre les deux chemins comparés en S184.
    pub fn step(&mut self, dt: f32, with_source: bool) {
        self.advance(dt, with_source, NU)
    }
    /// Variante à viscosité choisie ; `nu = 0` sert la réception 1 de S184.
    pub fn advance(&mut self, dt: f32, with_source: bool, nu: f32) {
        let n = self.n;
        let inv2 = 0.5 / DX as f32;
        let invsq = 1.0 / (DX * DX) as f32;
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                for k in 1..n - 1 {
                    let c = self.at(i, j, k);
                    let u = self.u[c];
                    let px = self.u[self.at(i + 1, j, k)];
                    let mx = self.u[self.at(i - 1, j, k)];
                    let py = self.u[self.at(i, j + 1, k)];
                    let my = self.u[self.at(i, j - 1, k)];
                    let pz = self.u[self.at(i, j, k + 1)];
                    let mz = self.u[self.at(i, j, k - 1)];
                    let mut out = [0.0f32; 3];
                    for a in 0..3 {
                        let dx = (px[a] - mx[a]) * inv2;
                        let dy = (py[a] - my[a]) * inv2;
                        let dz = (pz[a] - mz[a]) * inv2;
                        let adv = u[0] * dx + u[1] * dy + u[2] * dz;
                        let lap = (px[a] + mx[a] + py[a] + my[a] + pz[a] + mz[a] - 6.0 * u[a])
                            * invsq;
                        let src = if with_source { self.s[c][a] } else { 0.0 };
                        out[a] = u[a] + dt * (-adv + nu * lap - src);
                    }
                    self.next[c] = out;
                }
            }
        }
        core::mem::swap(&mut self.u, &mut self.next);
    }
}

/// Interpolation trilinéaire des nœuds vers les centres de mailles intérieures.
/// Le coin supérieur est borné : à `r = 1` le poids vaut zéro et la valeur du nœud passe
/// telle quelle, ce qui rend la réception 3 de S184 exacte sans cas particulier.
pub fn scatter(n: usize, r: usize, nodes: &[[f32; 3]], block: &mut Block) {
    let m = nodes_per_axis(n, r);
    let idx = |a: usize, b: usize, c: usize| (a * m + b) * m + c;
    let inv = 1.0 / r as f32;
    for i in 1..n - 1 {
        let (ai, ti) = ((i - 1) / r, ((i - 1) % r) as f32 * inv);
        let ai1 = (ai + 1).min(m - 1);
        for j in 1..n - 1 {
            let (aj, tj) = ((j - 1) / r, ((j - 1) % r) as f32 * inv);
            let aj1 = (aj + 1).min(m - 1);
            for k in 1..n - 1 {
                let (ak, tk) = ((k - 1) / r, ((k - 1) % r) as f32 * inv);
                let ak1 = (ak + 1).min(m - 1);
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    let g = |x: usize, y: usize, z: usize| nodes[idx(x, y, z)][a];
                    let l = |p: f32, q: f32, t: f32| p + t * (q - p);
                    let x00 = l(g(ai, aj, ak), g(ai1, aj, ak), ti);
                    let x10 = l(g(ai, aj1, ak), g(ai1, aj1, ak), ti);
                    let x01 = l(g(ai, aj, ak1), g(ai1, aj, ak1), ti);
                    let x11 = l(g(ai, aj1, ak1), g(ai1, aj1, ak1), ti);
                    let y0 = l(x00, x10, tj);
                    let y1 = l(x01, x11, tj);
                    block.s[c][a] = l(y0, y1, tk);
                }
            }
        }
    }
}
pub fn load_direct(n: usize, values: &[[f32; 3]], block: &mut Block) {
    let mut t = 0;
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for k in 1..n - 1 {
                block.s[(i * n + j) * n + k] = values[t];
                t += 1;
            }
        }
    }
}
