//! S390 / C3 — **la multigrille de la référence sur la carte** ([ADR-207](../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
//! conception S384 §5).
//!
//! Le cycle en V de S385 (`water_core::delta3d_multigrid`) comme préconditionneur du gradient conjugué résident de `Step3`,
//! à travail fixe : le nombre de dispatchs ne dépend que du profil et de la forme, jamais de la donnée (ADR-175 D2).
//! Ce module réserve la hiérarchie et ses uniformes (I-06 : à la configuration, par `Step3::enable_multigrid`), et
//! encode les séquences ; les noyaux sont dans `delta3d_mg.wgsl`, compilé à la suite de `delta3d_cg.wgsl`.
use crate::delta3d::{buffer, GROUP};
use water_core::delta3d::Domain3;

/// Amortissement du lissage, **dérivé** en S385 (`SMOOTH_DAMPING_3D` du cœur, privé) : ω = 6/7.
pub const OMEGA: f32 = 6. / 7.;
/// Lissages avant et après, et au plus grossier : ceux du cœur (S385).
pub const PRE_SWEEPS: usize = 2;
pub const POST_SWEEPS: usize = 2;
pub const COARSE_SWEEPS: usize = 8;

/// Entrées du module combiné, dans l'ordre de ce tableau.
const NAMES: [&str; 23] = [
    "bnorm_fold", "finish_bnorm", "mg_kind", "mg_init_warm", "mg_fine_qz", "mg_fine_zq", "mg_fine_qz_fold0",
    "mg_fine_qz_fold1", "finish_rz", "mg_direction_first", "apply_fold", "finish_dq", "mg_update_xr", "mg_restrict",
    "mg_first", "mg_sweep_tx", "mg_sweep_xt", "mg_prolong", "finish_beta", "direction", "residual_fold", "finish_residual",
    "mg_fine_first",
];
const BNORM_FOLD: usize = 0;
const FINISH_BNORM: usize = 1;
const KIND: usize = 2;
const INIT_WARM: usize = 3;
const FINE_QZ: usize = 4;
const FINE_ZQ: usize = 5;
const FINE_QZ_FOLD0: usize = 6;
const FINE_QZ_FOLD1: usize = 7;
const FINISH_RZ: usize = 8;
const DIRECTION_FIRST: usize = 9;
const APPLY_FOLD: usize = 10;
const FINISH_DQ: usize = 11;
const UPDATE_XR: usize = 12;
const RESTRICT: usize = 13;
const FIRST: usize = 14;
const SWEEP_TX: usize = 15;
const SWEEP_XT: usize = 16;
const PROLONG: usize = 17;
const FINISH_BETA: usize = 18;
const DIRECTION: usize = 19;
const RESIDUAL_FOLD: usize = 20;
const FINISH_RESIDUAL: usize = 21;
const FINE_FIRST: usize = 22;

/// Taille de l'uniforme `Level` : deux `Grid` de douze mots et quatre mots.
const LEVEL_UNIFORM: u64 = 112;

/// La règle de division du cœur (`coarsens3`) : trois dimensions paires et d'au moins quatre.
pub fn coarsens(nx: usize, ny: usize, nz: usize) -> bool {
    nx % 2 == 0 && ny % 2 == 0 && nz % 2 == 0 && nx >= 4 && ny >= 4 && nz >= 4
}

/// Les niveaux grossiers d'une forme, du plus fin au plus grossier.
pub fn levels_of(d: Domain3) -> Vec<[usize; 3]> {
    let (mut x, mut y, mut z) = (d.nx, d.ny, d.nz);
    let mut out = Vec::new();
    while coarsens(x, y, z) {
        x /= 2;
        y /= 2;
        z /= 2;
        out.push([x, y, z]);
    }
    out
}

/// Dispatchs d'un cycle en V, pour `levels` niveaux grossiers : un lissage fin avant (le premier est dans la mise à
/// jour), restriction, premier lissage et lissages de chaque niveau, prolongations et lissages après, deux lissages fins
/// après (le dernier replie `r·z`).
pub fn vcycle_dispatches(levels: usize) -> u32 {
    if levels == 0 {
        return 3;
    }
    let down: usize = (1..=levels).map(|l| 2 + if l == levels { COARSE_SWEEPS - 1 } else { PRE_SWEEPS - 1 }).sum();
    let up = (levels - 1) * (1 + POST_SWEEPS) + 1;
    (1 + down + up + 2) as u32
}

/// Dispatchs de la projection multigrille : ‖b‖ (2), géométrie des niveaux, départ (1 + cycle + 2), `cycles` cycles
/// (produit, α, mise à jour, cycle, β, direction), vrai résidu (2).
pub fn projection_dispatches(cycles: u32, levels: usize) -> u32 {
    let v = vcycle_dispatches(levels);
    2 + levels as u32 + (1 + v + 2) + cycles * (3 + v + 2) + 2
}

pub struct Multigrid {
    pipelines: Vec<wgpu::ComputePipeline>,
    /// Un groupe de liaison par niveau grossier, `binds[l − 1]` pour le niveau `l`.
    binds: Vec<wgpu::BindGroup>,
    uniform: wgpu::Buffer,
    stride: u64,
    /// Mailles réservées par niveau, et décalages `[kind, x, r, t]` dans `buffer`.
    reserved: Vec<usize>,
    offsets: Vec<[usize; 4]>,
    pub buffer: wgpu::Buffer,
    /// Flottants réservés, comptés (I-06).
    pub floats: usize,
    /// Niveaux de la forme courante.
    levels: std::cell::Cell<usize>,
    dims: std::cell::RefCell<Vec<[usize; 3]>>,
    /// Lissages après la prolongation, niveaux grossiers intermédiaires : `POST_SWEEPS`, sauf banc (pair : le
    /// résultat reste dans `x`).
    coarse_post: std::cell::Cell<usize>,
}

impl Multigrid {
    /// Réserve la hiérarchie pour la **capacité** : à chaque niveau, les dimensions arrondies par excès, tant que `nz`
    /// se divise — une forme plus petite (S350) tient donc toujours dans la réserve. **À la configuration seulement.**
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &wgpu::Device,
        capacity: Domain3,
        heights: &wgpu::Buffer,
        state: &wgpu::Buffer,
        partial: &wgpu::Buffer,
        scalar: &wgpu::Buffer,
        cg_uniform: &wgpu::Buffer,
    ) -> Self {
        let mut reserved = Vec::new();
        let (mut x, mut y, mut z) = (capacity.nx, capacity.ny, capacity.nz);
        while z % 2 == 0 && z >= 4 && x >= 4 && y >= 4 {
            x = x.div_ceil(2);
            y = y.div_ceil(2);
            z /= 2;
            reserved.push(x * y * z);
        }
        let mut offsets = Vec::with_capacity(reserved.len());
        let mut floats = 0;
        for &cells in &reserved {
            offsets.push([floats, floats + cells, floats + 2 * cells, floats + 3 * cells]);
            floats += 4 * cells;
        }
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let mg_buffer = buffer(device, (floats.max(1) * 4) as u64, storage);
        let alignment = device.limits().min_uniform_buffer_offset_alignment as u64;
        let stride = LEVEL_UNIFORM.div_ceil(alignment) * alignment;
        let slots = reserved.len().max(1) as u64;
        let uniform = buffer(device, slots * stride, wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST);
        let layout = crate::delta3d_step::layout(device, &['r', 'w', 'w', 'w', 'u', 'w', 'u']);
        let binds = (0..slots)
            .map(|l| {
                let plain = [heights, state, partial, scalar, cg_uniform, &mg_buffer];
                let mut entries: Vec<_> = plain
                    .iter()
                    .enumerate()
                    .map(|(i, b)| wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() })
                    .collect();
                entries.push(wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform,
                        offset: l * stride,
                        size: wgpu::BufferSize::new(LEVEL_UNIFORM),
                    }),
                });
                device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &layout, entries: &entries })
            })
            .collect();
        let source = format!("{}\n{}", include_str!("delta3d_cg.wgsl"), include_str!("delta3d_mg.wgsl"));
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("multigrille delta 3d"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipelines = crate::delta3d_step::pipelines(device, &layout, &module, &NAMES);
        Multigrid {
            pipelines,
            binds,
            uniform,
            stride,
            reserved,
            offsets,
            buffer: mg_buffer,
            floats,
            levels: std::cell::Cell::new(0),
            dims: std::cell::RefCell::new(Vec::new()),
            coarse_post: std::cell::Cell::new(POST_SWEEPS),
        }
    }

    /// Réécrit les uniformes pour la forme courante ; rend le nombre de niveaux employés.
    pub fn write_shape(&self, queue: &wgpu::Queue, shape: Domain3) -> usize {
        let dims: Vec<[usize; 3]> = levels_of(shape)
            .into_iter()
            .zip(&self.reserved)
            .take_while(|(d, &r)| d[0] * d[1] * d[2] <= r)
            .map(|(d, _)| d)
            .collect();
        let grid = |d: [usize; 3], o: [usize; 4], h: f32, top: bool| -> Vec<u8> {
            let mut b = Vec::with_capacity(48);
            for v in [d[0] as u32, d[1] as u32, d[2] as u32, (d[0] * d[1] * d[2]) as u32] {
                b.extend_from_slice(&v.to_le_bytes());
            }
            for v in o {
                b.extend_from_slice(&(v as u32).to_le_bytes());
            }
            b.extend_from_slice(&(1. / (h * h)).to_le_bytes());
            for v in [top as u32, 0, 0] {
                b.extend_from_slice(&v.to_le_bytes());
            }
            b
        };
        let mut h = shape.dx;
        for (l, &d) in dims.iter().enumerate() {
            let fine = if l == 0 {
                grid([shape.nx, shape.ny, shape.nz], [0; 4], h, true)
            } else {
                grid(dims[l - 1], self.offsets[l - 1], h, false)
            };
            h *= 2.;
            let mut bytes = grid(d, self.offsets[l], h, false);
            bytes.extend_from_slice(&fine);
            for v in [OMEGA, 0., 0., 0.] {
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            queue.write_buffer(&self.uniform, l as u64 * self.stride, &bytes);
        }
        if dims.is_empty() {
            // Sans niveau grossier, les noyaux fins lisent quand même `omega` dans le premier emplacement.
            let mut bytes = grid([shape.nx, shape.ny, shape.nz], [0; 4], h, true);
            bytes.extend_from_slice(&grid([shape.nx, shape.ny, shape.nz], [0; 4], h, true));
            for v in [OMEGA, 0., 0., 0.] {
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            queue.write_buffer(&self.uniform, 0, &bytes);
        }
        self.levels.set(dims.len());
        *self.dims.borrow_mut() = dims;
        self.levels.get()
    }

    pub fn levels(&self) -> usize {
        self.levels.get()
    }

    /// **Banc** : voir `Step3::mg_coarse_post_for_bench`.
    pub fn set_coarse_post(&self, sweeps: usize) {
        self.coarse_post.set(sweeps);
    }

    /// Mailles d'un niveau grossier de la forme courante (`l` ≥ 1).
    fn cells(&self, l: usize) -> u32 {
        let d = self.dims.borrow()[l - 1];
        (d[0] * d[1] * d[2]) as u32
    }

    fn run(&self, pass: &mut wgpu::ComputePass<'_>, index: usize, slot: usize, groups: u32) {
        pass.set_bind_group(0, &self.binds[slot.saturating_sub(1)], &[]);
        pass.set_pipeline(&self.pipelines[index]);
        pass.dispatch_workgroups(groups, 1, 1);
    }

    /// `z = M⁻¹·r` : le cycle en V, `q` portant déjà le premier lissage ; replie `r·z` dans la tranche `fold`.
    fn vcycle(&self, pass: &mut wgpu::ComputePass<'_>, fine: u32, fold: u32) {
        let levels = self.levels();
        self.run(pass, FINE_QZ, 1, fine);
        if levels > 0 {
            for l in 1..=levels {
                let g = self.cells(l).div_ceil(GROUP);
                self.run(pass, RESTRICT, l, g);
                self.run(pass, FIRST, l, g);
                let sweeps = if l == levels { COARSE_SWEEPS } else { PRE_SWEEPS };
                for s in 1..sweeps {
                    self.run(pass, if s % 2 == 1 { SWEEP_TX } else { SWEEP_XT }, l, g);
                }
            }
            for l in (1..levels).rev() {
                let g = self.cells(l).div_ceil(GROUP);
                self.run(pass, PROLONG, l + 1, g);
                for s in 0..self.coarse_post.get() {
                    self.run(pass, if s % 2 == 0 { SWEEP_XT } else { SWEEP_TX }, l, g);
                }
            }
            self.run(pass, PROLONG, 1, fine);
        }
        self.run(pass, FINE_ZQ, 1, fine);
        self.run(pass, if fold == 0 { FINE_QZ_FOLD0 } else { FINE_QZ_FOLD1 }, 1, fine);
    }

    /// `‖b‖`, géométrie des niveaux, départ chaud préconditionné, première direction.
    pub fn encode_start(&self, pass: &mut wgpu::ComputePass<'_>, fine: u32) {
        self.run(pass, BNORM_FOLD, 1, fine);
        self.run(pass, FINISH_BNORM, 1, 1);
        for l in 1..=self.levels() {
            self.run(pass, KIND, l, self.cells(l).div_ceil(GROUP));
        }
        self.run(pass, INIT_WARM, 1, fine);
        self.vcycle(pass, fine, 0);
        self.run(pass, FINISH_RZ, 1, 1);
        self.run(pass, DIRECTION_FIRST, 1, fine);
    }

    /// Un cycle du gradient conjugué préconditionné par le cycle en V.
    pub fn encode_cycle(&self, pass: &mut wgpu::ComputePass<'_>, fine: u32) {
        self.run(pass, APPLY_FOLD, 1, fine);
        self.run(pass, FINISH_DQ, 1, 1);
        self.run(pass, UPDATE_XR, 1, fine);
        self.vcycle(pass, fine, 1);
        self.run(pass, FINISH_BETA, 1, 1);
        self.run(pass, DIRECTION, 1, fine);
    }

    /// Le vrai résidu, comme le chemin de Jacobi.
    pub fn encode_residual(&self, pass: &mut wgpu::ComputePass<'_>, fine: u32) {
        self.run(pass, RESIDUAL_FOLD, 1, fine);
        self.run(pass, FINISH_RESIDUAL, 1, 1);
    }

    /// **Banc** : le cycle en V seul, sur le `r` déjà écrit dans la tranche `R` ; rend `z` dans `Z`. La géométrie des
    /// niveaux est recalculée d'abord ; `q` reçoit le premier lissage depuis `z = 0`, comme la mise à jour.
    pub fn encode_vcycle_only(&self, pass: &mut wgpu::ComputePass<'_>, fine: u32) {
        for l in 1..=self.levels() {
            self.run(pass, KIND, l, self.cells(l).div_ceil(GROUP));
        }
        self.run(pass, FINE_FIRST, 1, fine);
        self.vcycle(pass, fine, 1);
    }
}

// ── L'instrument (S390, critère 2) ─────────────────────────────────────────────────────────────────────────────────

/// **Réplique CPU en `f64`** du cycle en V de la carte, écrite depuis le cœur (S385) et `stencil3`, pas depuis le WGSL :
/// même géométrie (hauteurs par colonne, fantôme de surface, murs), même diagonale fine (la tranche `M` lue sur la
/// carte), mêmes lissages et transferts. Ce qu'elle juge : que la carte calcule le cycle qu'on croit.
pub struct Replica {
    d: Domain3,
    heights: Vec<f64>,
    m: Vec<f64>,
    theta: f64,
}

/// Un niveau grossier de la réplique : nature des mailles (1 active, −1 air), `1/h²`.
struct RLevel {
    n: [usize; 3],
    kind: Vec<f64>,
    inv: f64,
}

impl Replica {
    pub fn new(d: Domain3, heights: &[f32], m: &[f32]) -> Self {
        Replica {
            d,
            heights: heights.iter().map(|&v| v as f64).collect(),
            m: m.iter().map(|&v| v as f64).collect(),
            theta: water_core::delta_projection::SURFACE_THETA_MIN as f64,
        }
    }

    fn h(&self, i: usize, j: usize) -> f64 {
        self.heights[j * self.d.nx + i]
    }

    fn zc(&self, k: usize) -> f64 {
        (k as f64 + 0.5) * self.d.dx as f64
    }

    pub fn wet(&self, c: usize) -> bool {
        let (nx, ny) = (self.d.nx, self.d.ny);
        self.zc(c / (nx * ny)) < self.h(c % nx, (c / nx) % ny)
    }

    /// `(A·v)_c` et la diagonale, en `f64` : l'opérateur du pas mobile tel que `stencil3` l'écrit.
    pub fn apply(&self, v: &[f64], c: usize) -> (f64, f64) {
        let Domain3 { nx, ny, nz, dx } = self.d;
        let plane = nx * ny;
        let (i, j, k) = (c % nx, (c / nx) % ny, c / plane);
        let h = self.h(i, j);
        let zc = self.zc(k);
        if zc >= h {
            return (0., 0.);
        }
        let dx = dx as f64;
        let vc = v[c];
        let (mut acc, mut diag) = (0., 0.);
        let mut side = |inside: bool, n: usize, o: f64| {
            if !inside {
                return;
            }
            if zc < o {
                acc += vc - v[n];
                diag += 1.;
            } else {
                let a = 1. / ((h - zc) / (h - o)).max(self.theta);
                acc += vc * a;
                diag += a;
            }
        };
        side(i > 0, c.wrapping_sub(1), if i > 0 { self.h(i - 1, j) } else { 0. });
        side(i + 1 < nx, c + 1, if i + 1 < nx { self.h(i + 1, j) } else { 0. });
        side(j > 0, c.wrapping_sub(nx), if j > 0 { self.h(i, j - 1) } else { 0. });
        side(j + 1 < ny, c + nx, if j + 1 < ny { self.h(i, j + 1) } else { 0. });
        if k > 0 {
            acc += vc - v[c - plane];
            diag += 1.;
        }
        if k + 1 < nz && self.zc(k + 1) < h {
            acc += vc - v[c + plane];
            diag += 1.;
        } else {
            let a = 1. / ((h - zc) / dx).max(self.theta);
            acc += vc * a;
            diag += a;
        }
        let inv = 1. / (dx * dx);
        (acc * inv, diag * inv)
    }

    fn levels(&self, coarse: usize) -> Vec<RLevel> {
        let mut out: Vec<RLevel> = Vec::new();
        let mut h = self.d.dx as f64;
        for (l, n) in levels_of(self.d).into_iter().take(coarse).enumerate() {
            h *= 2.;
            let mut kind = vec![0.; n[0] * n[1] * n[2]];
            for (cc, kc) in kind.iter_mut().enumerate() {
                let (i, j, k) = (cc % n[0], (cc / n[0]) % n[1], cc / (n[0] * n[1]));
                let (mut active, mut air) = (false, false);
                for dd in 0..8 {
                    let (di, dj, dk) = (dd & 1, (dd >> 1) & 1, (dd >> 2) & 1);
                    let v = if l == 0 {
                        let fc = ((2 * k + dk) * self.d.ny + 2 * j + dj) * self.d.nx + 2 * i + di;
                        if self.wet(fc) { 1. } else { -1. }
                    } else {
                        let f = &out[l - 1];
                        f.kind[((2 * k + dk) * f.n[1] + 2 * j + dj) * f.n[0] + 2 * i + di]
                    };
                    if v > 0. {
                        active = true;
                    } else if v < 0. {
                        air = true;
                    }
                }
                *kc = if active { 1. } else if air { -1. } else { 0. };
            }
            out.push(RLevel { n, kind, inv: 1. / (h * h) });
        }
        out
    }

    fn row(g: &RLevel, p: &[f64], c: usize) -> (f64, f64) {
        if g.kind[c] <= 0. {
            return (0., 0.);
        }
        let [nx, ny, nz] = g.n;
        let plane = nx * ny;
        let (i, j, k) = (c % nx, (c / nx) % ny, c / plane);
        let pc = p[c];
        let (mut acc, mut dg) = (0., 0.);
        let faces = [
            (i > 0, c.wrapping_sub(1), false),
            (i + 1 < nx, c + 1, false),
            (j > 0, c.wrapping_sub(nx), false),
            (j + 1 < ny, c + nx, false),
            (k > 0, c.wrapping_sub(plane), false),
            (k + 1 < nz, c + plane, true),
        ];
        for (inside, n, top) in faces {
            if inside {
                if g.kind[n] > 0. {
                    acc += pc - p[n];
                    dg += 1.;
                } else if g.kind[n] < 0. {
                    acc += 2. * pc;
                    dg += 2.;
                }
            } else if top {
                acc += 2. * pc;
                dg += 2.;
            }
        }
        (acc * g.inv, dg * g.inv)
    }

    fn restrict(fine_n: [usize; 3], t: &[f64], g: &RLevel) -> Vec<f64> {
        let [nx, ny, _] = fine_n;
        let mut r = vec![0.; g.kind.len()];
        for (cc, rc) in r.iter_mut().enumerate() {
            let (i, j, k) = (cc % g.n[0], (cc / g.n[0]) % g.n[1], cc / (g.n[0] * g.n[1]));
            let mut s = 0.;
            for dd in 0..8 {
                let (di, dj, dk) = (dd & 1, (dd >> 1) & 1, (dd >> 2) & 1);
                s += t[((2 * k + dk) * ny + 2 * j + dj) * nx + 2 * i + di];
            }
            *rc = 0.125 * s;
        }
        r
    }

    fn prolong_add(fine_n: [usize; 3], fine: &mut [f64], g: &RLevel, x: &[f64]) {
        let [nx, ny, nz] = fine_n;
        for (fc, f) in fine.iter_mut().enumerate().take(nx * ny * nz) {
            let (i, j, k) = (fc % nx, (fc / nx) % ny, fc / (nx * ny));
            *f += x[((k / 2) * g.n[1] + j / 2) * g.n[0] + i / 2];
        }
    }

    /// Jacobi amorti, `sweeps` fois, sur un niveau grossier, depuis la valeur de `x`.
    fn smooth(g: &RLevel, r: &[f64], x: &mut Vec<f64>, sweeps: usize) {
        let omega = OMEGA as f64;
        for _ in 0..sweeps {
            let next: Vec<f64> = (0..x.len())
                .map(|c| {
                    let (a, dg) = Self::row(g, x, c);
                    if dg > 0. { x[c] + omega * (r[c] - a) / dg } else { x[c] }
                })
                .collect();
            *x = next;
        }
    }

    fn fine_sweep(&self, r: &[f64], z: &[f64]) -> Vec<f64> {
        let omega = OMEGA as f64;
        (0..z.len())
            .map(|c| if self.m[c] > 0. { z[c] + omega * self.m[c] * (r[c] - self.apply(z, c).0) } else { z[c] })
            .collect()
    }

    /// `z = M⁻¹·r` : le cycle en V de la carte, `coarse_post` lissages après sur les niveaux intermédiaires.
    pub fn vcycle(&self, r: &[f64], coarse: usize, coarse_post: usize) -> Vec<f64> {
        let fine_n = [self.d.nx, self.d.ny, self.d.nz];
        let mut z = vec![0.; r.len()];
        for _ in 0..PRE_SWEEPS {
            z = self.fine_sweep(r, &z);
        }
        let levels = self.levels(coarse);
        if !levels.is_empty() {
            let t: Vec<f64> = (0..z.len()).map(|c| r[c] - self.apply(&z, c).0).collect();
            let last = levels.len() - 1;
            let mut rs = vec![Self::restrict(fine_n, &t, &levels[0])];
            let mut xs: Vec<Vec<f64>> = Vec::new();
            for l in 0..=last {
                let mut x = vec![0.; levels[l].kind.len()];
                Self::smooth(&levels[l], &rs[l], &mut x, if l == last { COARSE_SWEEPS } else { PRE_SWEEPS });
                if l < last {
                    let t: Vec<f64> = (0..x.len()).map(|c| rs[l][c] - Self::row(&levels[l], &x, c).0).collect();
                    rs.push(Self::restrict(levels[l].n, &t, &levels[l + 1]));
                }
                xs.push(x);
            }
            for l in (0..last).rev() {
                let coarse_x = xs[l + 1].clone();
                Self::prolong_add(levels[l].n, &mut xs[l], &levels[l + 1], &coarse_x);
                let mut x = std::mem::take(&mut xs[l]);
                Self::smooth(&levels[l], &rs[l], &mut x, coarse_post);
                xs[l] = x;
            }
            Self::prolong_add(fine_n, &mut z, &levels[0], &xs[0]);
            for (zc, &mc) in z.iter_mut().zip(&self.m) {
                if mc == 0. {
                    *zc = 0.;
                }
            }
        }
        for _ in 0..POST_SWEEPS {
            z = self.fine_sweep(r, &z);
        }
        z
    }
}

/// Générateur reproductible, sans dépendance : `xorshift64*`, valeurs dans `[−1, 1]`.
fn aleas(n: usize, graine: u64) -> Vec<f32> {
    let mut s = graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..n)
        .map(|_| {
            s ^= s >> 12;
            s ^= s << 25;
            s ^= s >> 27;
            let u = (s.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64;
            (2. * u - 1.) as f32
        })
        .collect()
}

/// **S390, critère 2 — l'instrument.** La scène de la porte B (`Config::review`) après `CHAUFFE=` (30) pas de Jacobi :
/// le cycle en V de la carte contre la réplique `f64`, sur deux résidus aléatoires et sur le second membre du pas ;
/// puis la symétrie `⟨u, M⁻¹v⟩ = ⟨M⁻¹u, v⟩` et la positivité, sur la carte. `ASYMETRIQUE=1` retire les lissages après
/// des niveaux intermédiaires : l'essai doit alors échouer. Lignes `MG_CYCLE_S390`.
pub fn recevoir_cycle() -> Result<(), String> {
    use water_core::SimTime;
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = crate::delta3d_scene::Config::review();
        let (u, v, w, eta) = config.initial_state();
        let mut carte = crate::delta3d_step::Step3::new(
            background,
            config.domain,
            config.origin,
            crate::delta3d_scene::RHO,
            crate::delta3d_scene::G,
        )
        .await?;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        carte.set_state(&u, &v, &w, &eta)?;
        let chauffe: u64 = std::env::var("CHAUFFE").ok().and_then(|v| v.parse().ok()).unwrap_or(30);
        for n in 0..chauffe {
            carte.publish_time(background, SimTime(n * config.step_us))?;
            carte.run_for_bench(config.cycles, crate::delta3d_step::Upto::Full)?;
        }
        let levels = carte.enable_multigrid();
        let asym = std::env::var("ASYMETRIQUE").is_ok_and(|v| v == "1");
        let post = if asym { 0 } else { POST_SWEEPS };
        carte.mg_coarse_post_for_bench(post)?;
        let d = config.domain;
        let n = d.cells();
        println!(
            "MG_CYCLE_S390 carte={:?} domaine={}x{}x{} niveaux={levels} dims={:?} chauffe={chauffe} lissages_apres_intermediaires={post} flottants_reserves={}",
            carte.adapter, d.nx, d.ny, d.nz, levels_of(d), carte.multigrid().map_or(0, |m| m.1)
        );
        // La diagonale lue sur la carte, contre celle de l'opérateur répliqué.
        let (_, h, m) = carte.mg_vcycle_for_bench(&vec![0.; n])?;
        let replica = Replica::new(d, &h, &m);
        let zero = vec![0f64; n];
        let (mut diag_pire, mut mouillees, mut incoherentes) = (0f64, 0usize, 0usize);
        for c in 0..n {
            let (_, dg) = replica.apply(&zero, c);
            let wet = replica.wet(c);
            if wet != (m[c] > 0.) {
                incoherentes += 1;
            }
            if wet && dg > 0. {
                mouillees += 1;
                diag_pire = diag_pire.max((m[c] as f64 * dg - 1.).abs());
            }
        }
        println!("MG_CYCLE_S390 diagonale mouillees={mouillees} ecart_relatif_M_contre_1_sur_diag={diag_pire:.3e} mouillage_incoherent={incoherentes}");
        // Trois résidus : deux aléatoires sur les mailles mouillées, et le second membre du pas (tranche B).
        let b = carte.state_slice_for_bench(6)?;
        let masque = |x: Vec<f32>| -> Vec<f32> { x.iter().zip(&m).map(|(&x, &mm)| if mm > 0. { x } else { 0. }).collect() };
        let entrees: Vec<(&str, Vec<f32>)> =
            vec![("aleatoire_1", masque(aleas(n, 1))), ("aleatoire_2", masque(aleas(n, 2))), ("second_membre", b)];
        let mut pire = 0f64;
        let mut sorties: Vec<Vec<f64>> = Vec::new();
        for (nom, r) in &entrees {
            let (z, _, _) = carte.mg_vcycle_for_bench(r)?;
            let r64: Vec<f64> = r.iter().map(|&x| x as f64).collect();
            let zr = replica.vcycle(&r64, levels, post);
            let echelle = zr.iter().fold(0f64, |a, &x| a.max(x.abs()));
            let ecart = z.iter().zip(&zr).fold(0f64, |a, (&g, &c)| a.max((g as f64 - c).abs()));
            let rel = if echelle > 0. { ecart / echelle } else { ecart };
            pire = pire.max(rel);
            println!("MG_CYCLE_S390 entree={nom} max_z={echelle:.4e} ecart_carte_replique={ecart:.3e} relatif={rel:.3e}");
            sorties.push(z.iter().map(|&x| x as f64).collect());
        }
        // Symétrie et positivité, sur la carte : u, v les deux résidus aléatoires.
        let dot = |a: &[f64], b: &[f32]| a.iter().zip(b).map(|(x, &y)| x * y as f64).sum::<f64>();
        let (u1, v1) = (&entrees[0].1, &entrees[1].1);
        let (mu, mv) = (&sorties[0], &sorties[1]);
        let (s1, s2) = (dot(mv, u1), dot(mu, v1));
        let sym = (s1 - s2).abs() / s1.abs().max(s2.abs());
        let (pos_u, pos_v) = (dot(mu, u1), dot(mv, v1));
        // La réplique elle-même, en f64 : symétrique au rendu près si la recette l'est.
        let u64v: Vec<f64> = u1.iter().map(|&x| x as f64).collect();
        let v64v: Vec<f64> = v1.iter().map(|&x| x as f64).collect();
        let (ru, rv) = (replica.vcycle(&u64v, levels, post), replica.vcycle(&v64v, levels, post));
        let t1: f64 = rv.iter().zip(&u64v).map(|(a, b)| a * b).sum();
        let t2: f64 = ru.iter().zip(&v64v).map(|(a, b)| a * b).sum();
        let sym_rep = (t1 - t2).abs() / t1.abs().max(t2.abs());
        println!(
            "MG_CYCLE_S390 symetrie_carte={sym:.3e} (<u,Mv>={s1:.6e} <Mu,v>={s2:.6e}) symetrie_replique_f64={sym_rep:.3e} positivite <u,Mu>={pos_u:.6e} <v,Mv>={pos_v:.6e}"
        );
        let tenu = pire <= 1e-5 && sym <= 1e-5 && pos_u > 0. && pos_v > 0. && diag_pire <= 1e-5 && incoherentes == 0;
        println!(
            "MG_CYCLE_S390 bilan ecart_max_relatif={pire:.3e} symetrie={sym:.3e} critere_2={}",
            if tenu { "tenu" } else { "manque" }
        );
        Ok(())
    })
}
