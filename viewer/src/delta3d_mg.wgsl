// S390 / C3 — **la multigrille de la référence sur la carte** (ADR-207, conception S384 §5).
//
// Le cycle en V de S385 (`code/water-core/src/delta3d_multigrid.rs`) porté tel quel, comme préconditionneur du gradient
// conjugué de `delta3d_cg.wgsl`, à travail fixe. Ce fichier se compile **à la suite** de `delta3d_cg.wgsl` — il en
// emploie `params`, `state`, `stencil3`, `fold_at` — et ajoute deux liaisons : la hiérarchie `mg` et l'uniforme du niveau.
//
// La recette, et pourquoi (le gradient conjugué n'est valide qu'avec un préconditionneur symétrique défini positif) :
// Jacobi amorti ω = 6/7 (dérivé en S385), deux lissages avant et deux après, huit au plus grossier ; restriction par
// la moyenne des huit filles, prolongation par injection — adjointes à un facteur 8 près ; niveaux grossiers
// rediscrétisés depuis la surface à chaque projection. Le niveau fin est l'opérateur exact `stencil3`, sa diagonale
// inverse la tranche `M`. La carte n'a pas de solide : toutes les faces intérieures sont ouvertes, une maille grossière
// est active si une fille est mouillée, d'air sinon ; au-dessus du domaine, de l'air ; murs et fond, rien.
//
// Tampons de travail du niveau fin : `Q` (libre entre la mise à jour et le produit `A·d` suivant) et `Z`. Les lissages
// vont de l'un à l'autre ; le résultat finit dans `Z`.

struct Grid {
    nx: u32,
    ny: u32,
    nz: u32,
    cells: u32,
    kind: u32,
    x: u32,
    r: u32,
    t: u32,
    inv: f32,
    top: u32,
    _a: u32,
    _b: u32,
};

/// `level` : le niveau que le noyau écrit ; `fine` : le niveau plus fin (`top` = 1 : la grille de `state`).
struct Level {
    level: Grid,
    fine: Grid,
    omega: f32,
    _c: f32,
    _d: f32,
    _e: f32,
};

@group(0) @binding(5) var<storage, read_write> mg: array<f32>;
@group(0) @binding(6) var<uniform> lv: Level;

/// `(L·p)_c` sur un niveau grossier, et sa diagonale ; `p` lu à partir de `src`. Voisin actif : différence ; voisin
/// d'air : Dirichlet à demi-maille (`2·p`) ; au-dessus du domaine : de l'air ; mur, fond : rien. Zéro hors maille active.
/// Même ordre de faces que `row_sums` du cœur : x−, x+, y−, y+, z−, z+.
fn coarse_row(g: Grid, src: u32, c: u32) -> vec2<f32> {
    if (mg[g.kind + c] <= 0.0) {
        return vec2<f32>(0.0, 0.0);
    }
    let plane = g.nx * g.ny;
    let i = c % g.nx;
    let j = (c / g.nx) % g.ny;
    let k = c / plane;
    let pc = mg[src + c];
    var acc = 0.0;
    var dg = 0.0;
    var n: array<u32, 6>;
    var inside: array<bool, 6>;
    inside[0] = i > 0u;
    n[0] = c - select(0u, 1u, i > 0u);
    inside[1] = i + 1u < g.nx;
    n[1] = c + 1u;
    inside[2] = j > 0u;
    n[2] = c - select(0u, g.nx, j > 0u);
    inside[3] = j + 1u < g.ny;
    n[3] = c + g.nx;
    inside[4] = k > 0u;
    n[4] = c - select(0u, plane, k > 0u);
    inside[5] = k + 1u < g.nz;
    n[5] = c + plane;
    for (var f = 0u; f < 6u; f++) {
        if (inside[f]) {
            let km = mg[g.kind + n[f]];
            if (km > 0.0) {
                acc = acc + (pc - mg[src + n[f]]);
                dg = dg + 1.0;
            } else if (km < 0.0) {
                acc = acc + 2.0 * pc;
                dg = dg + 2.0;
            }
        } else if (f == 5u) {
            acc = acc + 2.0 * pc;
            dg = dg + 2.0;
        }
    }
    return vec2<f32>(acc * g.inv, dg * g.inv);
}

/// Indice de la fille `(di, dj, dk)` de la maille grossière `(i, j, k)`, dans une grille `nx × ny`.
fn child(nx: u32, ny: u32, i: u32, j: u32, k: u32, d: u32) -> u32 {
    let di = d & 1u;
    let dj = (d >> 1u) & 1u;
    let dk = (d >> 2u) & 1u;
    return ((2u * k + dk) * ny + 2u * j + dj) * nx + 2u * i + di;
}

/// Un lissage de Jacobi amorti au niveau fin : `dst = src + ω·M·(r − A·src)` ; maille sèche : `dst = src`.
fn fine_sweep(src: u32, dst: u32, c: u32) {
    let s = state[at(src, c)];
    let m = state[at(M, c)];
    if (m > 0.0) {
        state[at(dst, c)] = s + lv.omega * m * (state[at(R, c)] - stencil3(src, c).x);
    } else {
        state[at(dst, c)] = s;
    }
}

/// Un lissage de Jacobi amorti sur `lv.level`, de `src` vers `dst`.
fn coarse_sweep(src: u32, dst: u32, c: u32) {
    let g = lv.level;
    let a = coarse_row(g, src, c);
    if (a.y > 0.0) {
        mg[dst + c] = mg[src + c] + lv.omega * (mg[g.r + c] - a.x) / a.y;
    } else {
        mg[dst + c] = mg[src + c];
    }
}

// ── Géométrie, une fois par projection ────────────────────────────────────────────────────────────────────────────

/// Nature des mailles de `lv.level`, depuis le niveau plus fin : active si une fille l'est, d'air sinon.
@compute @workgroup_size(64)
fn mg_kind(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = lv.level;
    let cc = id.x;
    if (cc >= g.cells) { return; }
    let i = cc % g.nx;
    let j = (cc / g.nx) % g.ny;
    let k = cc / (g.nx * g.ny);
    var any_active = false;
    var air = false;
    for (var d = 0u; d < 8u; d++) {
        var v = 0.0;
        if (lv.fine.top == 1u) {
            let di = d & 1u;
            let dj = (d >> 1u) & 1u;
            let dk = (d >> 2u) & 1u;
            let zc = (f32(2u * k + dk) + 0.5) * params.dx;
            v = select(-1.0, 1.0, zc < height(2u * i + di, 2u * j + dj));
        } else {
            v = mg[lv.fine.kind + child(lv.fine.nx, lv.fine.ny, i, j, k, d)];
        }
        if (v > 0.0) { any_active = true; } else if (v < 0.0) { air = true; }
    }
    mg[g.kind + cc] = select(select(0.0, -1.0, air), 1.0, any_active);
}

// ── Gradient conjugué préconditionné par le cycle ─────────────────────────────────────────────────────────────────

/// Départ chaud : `r = b − A·x`, comme `init_warm` ; puis le premier lissage depuis `z = 0` : `q = ω·M·r`. Rien n'est
/// replié : le produit `r·z` suit le cycle.
@compute @workgroup_size(64)
fn mg_init_warm(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    let plane = params.nx * params.ny;
    let i = c % params.nx;
    let j = (c / params.nx) % params.ny;
    let k = c / plane;
    if ((f32(k) + 0.5) * params.dx >= height(i, j)) {
        state[at(X, c)] = 0.0;
        state[at(R, c)] = 0.0;
        state[at(Z, c)] = 0.0;
        state[at(D, c)] = 0.0;
        state[at(Q, c)] = 0.0;
    } else {
        let r = state[at(B, c)] - stencil3(X, c).x;
        state[at(R, c)] = r;
        let m = state[at(M, c)];
        state[at(Q, c)] = select(0.0, lv.omega * m * r, m > 0.0);
    }
}

/// `x += α d`, `r −= α q`, puis le premier lissage depuis `z = 0` dans `q`, que le cycle n'emploie plus.
@compute @workgroup_size(64)
fn mg_update_xr(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    let alpha = scalar[ALPHA];
    let r = state[at(R, c)] - alpha * state[at(Q, c)];
    state[at(X, c)] = state[at(X, c)] + alpha * state[at(D, c)];
    state[at(R, c)] = r;
    let m = state[at(M, c)];
    state[at(Q, c)] = select(0.0, lv.omega * m * r, m > 0.0);
}

/// **Banc** : le premier lissage seul, depuis `z = 0` et le `r` écrit : `q = ω·M·r`.
@compute @workgroup_size(64)
fn mg_fine_first(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    let m = state[at(M, c)];
    state[at(Q, c)] = select(0.0, lv.omega * m * state[at(R, c)], m > 0.0);
}

/// Lissage fin, `q → z`.
@compute @workgroup_size(64)
fn mg_fine_qz(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    fine_sweep(Q, Z, c);
}

/// Lissage fin, `z → q`.
@compute @workgroup_size(64)
fn mg_fine_zq(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    fine_sweep(Z, Q, c);
}

/// Dernier lissage fin, `q → z`, et `⟨r, z⟩` replié dans la tranche 1 (le `β` du cycle).
@compute @workgroup_size(64)
fn mg_fine_qz_fold1(@builtin(global_invocation_id) id: vec3<u32>,
                    @builtin(local_invocation_index) lid: u32,
                    @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        fine_sweep(Q, Z, c);
        contribution = state[at(R, c)] * state[at(Z, c)];
    }
    fold_at(1u, contribution, lid, wid.x);
}

/// Le même, replié dans la tranche 0 : le départ (`finish_rz`).
@compute @workgroup_size(64)
fn mg_fine_qz_fold0(@builtin(global_invocation_id) id: vec3<u32>,
                    @builtin(local_invocation_index) lid: u32,
                    @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        fine_sweep(Q, Z, c);
        contribution = state[at(R, c)] * state[at(Z, c)];
    }
    fold_at(0u, contribution, lid, wid.x);
}

/// Première direction : `d = z`.
@compute @workgroup_size(64)
fn mg_direction_first(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    state[at(D, c)] = state[at(Z, c)];
}

// ── Le cycle en V ─────────────────────────────────────────────────────────────────────────────────────────────────

/// **Restriction** vers `lv.level` : chaque maille reçoit la moyenne des résidus de ses huit filles, dans l'ordre de
/// `CHILDREN` du cœur. Niveau fin : `r − A·z` ; niveau grossier : `r − L·x`.
@compute @workgroup_size(64)
fn mg_restrict(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = lv.level;
    let cc = id.x;
    if (cc >= g.cells) { return; }
    let i = cc % g.nx;
    let j = (cc / g.nx) % g.ny;
    let k = cc / (g.nx * g.ny);
    var s = 0.0;
    let f = lv.fine;
    for (var d = 0u; d < 8u; d++) {
        let fc = child(f.nx, f.ny, i, j, k, d);
        if (f.top == 1u) {
            s += state[at(R, fc)] - stencil3(Z, fc).x;
        } else {
            s += mg[f.r + fc] - coarse_row(f, f.x, fc).x;
        }
    }
    mg[g.r + cc] = 0.125 * s;
}

/// Premier lissage d'un niveau grossier, depuis `x = 0` : `t = ω·r/diag`.
@compute @workgroup_size(64)
fn mg_first(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = lv.level;
    let c = id.x;
    if (c >= g.cells) { return; }
    let a = coarse_row(g, g.x, c);
    if (a.y > 0.0) {
        mg[g.t + c] = lv.omega * mg[g.r + c] / a.y;
    } else {
        mg[g.t + c] = 0.0;
    }
}

/// Lissage grossier, `t → x`.
@compute @workgroup_size(64)
fn mg_sweep_tx(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= lv.level.cells) { return; }
    coarse_sweep(lv.level.t, lv.level.x, c);
}

/// Lissage grossier, `x → t`.
@compute @workgroup_size(64)
fn mg_sweep_xt(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= lv.level.cells) { return; }
    coarse_sweep(lv.level.x, lv.level.t, c);
}

/// **Prolongation** depuis `lv.level` vers le niveau plus fin : chaque fille ajoute la valeur de sa mère. Niveau fin :
/// dans `z`, puis une maille sèche revient à zéro, comme le cœur.
@compute @workgroup_size(64)
fn mg_prolong(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = lv.level;
    let f = lv.fine;
    let fc = id.x;
    if (fc >= f.cells) { return; }
    let i = fc % f.nx;
    let j = (fc / f.nx) % f.ny;
    let k = fc / (f.nx * f.ny);
    let parent = ((k / 2u) * g.ny + j / 2u) * g.nx + i / 2u;
    let v = mg[g.x + parent];
    if (f.top == 1u) {
        if (state[at(M, fc)] == 0.0) {
            state[at(Z, fc)] = 0.0;
        } else {
            state[at(Z, fc)] = state[at(Z, fc)] + v;
        }
    } else {
        mg[f.x + fc] = mg[f.x + fc] + v;
    }
}
