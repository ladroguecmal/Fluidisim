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

// --- S187 : réseaux à pas par axe, et réseaux à indices quelconques -------------------
//
// Rien de ce qui précède n'est modifié. S184 mesure son coût avec `scatter`, S186 son
// erreur, et S186 a publié deux empreintes qui en dépendent : les formes générales
// s'écrivent **à côté**, et la réception 2 de S187 exige qu'elles reproduisent la forme
// uniforme en bits. Une implémentation générale qui ne retrouve pas son cas particulier
// est fausse quelque part, et l'écart se lirait ensuite comme un effet de la graduation.

/// Indices de mailles où se posent les nœuds d'un réseau uniforme de pas `r`, dans la
/// convention de `lattice_points` : `1 + a·r`, et le dernier nœud peut déborder des
/// mailles intérieures — c'est ce débordement que `base` prend en compte.
pub fn axis_indices(n: usize, r: usize) -> Vec<usize> {
    (0..nodes_per_axis(n, r)).map(|a| 1 + a * r).collect()
}

/// Poids d'interpolation d'un axe : pour chaque maille intérieure, le nœud de gauche et la
/// fraction vers le nœud de droite. Quand la maille tombe sur le dernier nœud ou au-delà,
/// la fraction vaut zéro et la valeur du nœud passe telle quelle.
fn axis_weights(n: usize, idx: &[usize]) -> Vec<(usize, usize, f32)> {
    (1..n - 1)
        .map(|i| {
            let mut j = 0;
            while j + 1 < idx.len() && idx[j + 1] <= i {
                j += 1;
            }
            if j + 1 < idx.len() {
                let span = idx[j + 1] - idx[j];
                (j, j + 1, (i - idx[j]) as f32 / span as f32)
            } else {
                (j, j, 0.0)
            }
        })
        .collect()
}

/// Points d'un réseau donné par ses indices de mailles sur chaque axe, dans l'ordre
/// `(a, b, c)` — celui qu'attendent `scatter_indexed` et `scatter_axes`.
pub fn lattice_points_indexed(n: usize, idx: &[Vec<usize>; 3]) -> Vec<WorldPos> {
    let mut v = Vec::with_capacity(idx[0].len() * idx[1].len() * idx[2].len());
    for &a in &idx[0] {
        for &b in &idx[1] {
            for &c in &idx[2] {
                v.push(point(cell_local(n, a, b, c)));
            }
        }
    }
    v
}

/// Interpolation trilinéaire depuis un réseau à indices quelconques vers les centres de
/// mailles intérieures. Forme générale ; `scatter` en est le cas uniforme.
pub fn scatter_indexed(n: usize, idx: &[Vec<usize>; 3], nodes: &[[f32; 3]], block: &mut Block) {
    let (my, mz) = (idx[1].len(), idx[2].len());
    let at = |a: usize, b: usize, c: usize| (a * my + b) * mz + c;
    let wx = axis_weights(n, &idx[0]);
    let wy = axis_weights(n, &idx[1]);
    let wz = axis_weights(n, &idx[2]);
    for i in 1..n - 1 {
        let (ai, ai1, ti) = wx[i - 1];
        for j in 1..n - 1 {
            let (aj, aj1, tj) = wy[j - 1];
            for k in 1..n - 1 {
                let (ak, ak1, tk) = wz[k - 1];
                let c = (i * n + j) * n + k;
                for a in 0..3 {
                    let g = |x: usize, y: usize, z: usize| nodes[at(x, y, z)][a];
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

/// Réseau à pas indépendant par axe. Sert l'attribution par axe de S187 : décimer
/// verticalement seul, ou horizontalement seul, n'est pas décimer isotropiquement.
pub fn axes_indices(n: usize, r: [usize; 3]) -> [Vec<usize>; 3] {
    [
        axis_indices(n, r[0]),
        axis_indices(n, r[1]),
        axis_indices(n, r[2]),
    ]
}

/// Nombre de nœuds d'un réseau donné par ses indices.
pub fn indexed_count(idx: &[Vec<usize>; 3]) -> usize {
    idx[0].len() * idx[1].len() * idx[2].len()
}

/// S188 : indices de mailles d'un réseau **ancré** — `want` nœuds couvrant `[1, n-2]`,
/// arrondi au plus proche, doublons fusionnés. C'est la règle 1 d'ADR-118 : le premier et
/// le dernier nœud se posent **sur** les mailles extrêmes du domaine, au lieu de déborder
/// comme `axis_indices`. S187 mesurait cet écart à un facteur six ; S188 rejoue la
/// composition dessus. Une seule définition pour les deux sessions (L137).
pub fn anchored_indices(n: usize, want: usize) -> Vec<usize> {
    let span = (n - 3) as f64;
    let mut out: Vec<usize> = (0..want)
        .map(|a| 1 + (a as f64 * span / (want - 1) as f64).round() as usize)
        .collect();
    out.dedup();
    out
}

/// Indice compact d'une maille intérieure, dans l'ordre de `interior_points`.
pub fn compact_index(n: usize, i: usize, j: usize, k: usize) -> usize {
    let m = n - 2;
    ((i - 1) * m + (j - 1)) * m + (k - 1)
}

/// S189 : dérivée seconde **verticale** de la source, par tranche, en maximum sur la
/// tranche — le profil dont la graduation d'ADR-118 a besoin. Sorti de `graded_lattice`
/// (S187) pour être partagé avec `graded_composition` (S189) : deux copies du même profil
/// poseraient deux réseaux différents, et les deux sessions ne se compareraient plus (L137).
///
/// Les deux tranches de bord empruntent leur valeur à leur voisine : la maille fantôme
/// existe dans le bloc mais pas dans le réseau plein, et recopier la voisine vaut mieux
/// qu'inventer une valeur. Le fait est déclaré plutôt que caché.
pub fn d2z_profile(n: usize, field: &[[f32; 3]]) -> Vec<f64> {
    let inv = 1.0 / (DX * DX);
    let mut p = vec![0.0f64; n - 2];
    for k in 2..n - 2 {
        let mut m = 0.0f64;
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                for x in 0..3 {
                    let v = field[compact_index(n, i, j, k + 1)][x] as f64
                        - 2.0 * field[compact_index(n, i, j, k)][x] as f64
                        + field[compact_index(n, i, j, k - 1)][x] as f64;
                    m = m.max((v * inv).abs());
                }
            }
        }
        p[k - 1] = m;
    }
    p[0] = p[1];
    let last = p.len() - 1;
    p[last] = p[last - 1];
    p
}

/// Indices verticaux **équidistribués** : `h·√|∂²_z S|` constant, donc des nœuds posés à
/// incréments égaux de `Φ = ∫ √|∂²_z S| dz` (ADR-118, règle 2). Accrochés aux indices de
/// mailles, doublons fusionnés, extrémités forcées.
pub fn graded_indices(n: usize, profile: &[f64], want: usize) -> Vec<usize> {
    let m = profile.len();
    let mut phi = vec![0.0f64; m];
    for k in 1..m {
        phi[k] = phi[k - 1] + 0.5 * (profile[k - 1].sqrt() + profile[k].sqrt()) * DX;
    }
    let total = phi[m - 1];
    let mut out = Vec::with_capacity(want);
    for a in 0..want {
        let target = a as f64 * total / (want - 1) as f64;
        let mut best = (f64::INFINITY, 0usize);
        for k in 0..m {
            let d = (phi[k] - target).abs();
            if d < best.0 {
                best = (d, k);
            }
        }
        let cell = 1 + best.1;
        if out.last() != Some(&cell) {
            out.push(cell);
        }
    }
    if out[0] != 1 {
        out.insert(0, 1);
    }
    if *out.last().unwrap() != n - 2 {
        out.push(n - 2);
    }
    out
}
