// S299 / ADR-175 D2 — **projection résidente à travail borné**, gradient conjugué préconditionné.
//
// Le cycle entier vit sur la carte : opérateur, produits scalaires, mise à jour des champs *et*
// des scalaires. Entre deux itérations il n'y a aucun retour CPU — ni lecture, ni décision, ni
// scalaire rapatrié. Le nombre de cycles est **fixé par l'appelant** : jamais une boucle jusqu'à
// convergence, jamais une longueur qui dépend de la donnée (ADR-175 D2, I-05).
//
// L'opérateur est celui de `delta3d.wgsl`, réécrit ici pour lire une tranche de `state` : même
// règle de géométrie, même ordre de faces x−, x+, y−, y+, z−, z+.

struct Params {
    nx: u32,
    ny: u32,
    nz: u32,
    cells: u32,
    dx: f32,
    inv_dx2: f32,
    theta_min: f32,
    rest: f32,
    rho_g: f32,
    scale: f32,
    groups: f32,
    _pad: f32,
};

@group(0) @binding(0) var<storage, read> heights: array<f32>;
@group(0) @binding(1) var<storage, read_write> state: array<f32>;
@group(0) @binding(2) var<storage, read_write> partial: array<f32>;
@group(0) @binding(3) var<storage, read_write> scalar: array<f32>;
@group(0) @binding(4) var<uniform> params: Params;

const X: u32 = 0u;   // pression cherchée
const R: u32 = 1u;   // résidu
const Z: u32 = 2u;   // M⁻¹r
const D: u32 = 3u;   // direction
const Q: u32 = 4u;   // A·d
const M: u32 = 5u;   // préconditionneur de Jacobi
const B: u32 = 6u;   // second membre (divergence à l'entrée)

const RZ: u32 = 0u;
const DQ: u32 = 1u;
const ALPHA: u32 = 2u;
const BETA: u32 = 3u;
const BNORM: u32 = 4u;

var<workgroup> scratch: array<f32, 64>;

fn at(section: u32, c: u32) -> u32 { return section * params.cells + c; }

fn height(i: u32, j: u32) -> f32 { return heights[j * params.nx + i]; }

fn groups_count() -> u32 { return u32(params.groups); }

/// `(A·v)_c` pour la tranche `section`, et la diagonale de la ligne. Zéro sur maille sèche.
fn stencil3(section: u32, c: u32) -> vec2<f32> {
    let plane = params.nx * params.ny;
    let i = c % params.nx;
    let j = (c / params.nx) % params.ny;
    let k = c / plane;
    let h = height(i, j);
    let zc = (f32(k) + 0.5) * params.dx;
    if (zc >= h) {
        return vec2<f32>(0.0, 0.0);
    }
    let base = section * params.cells;
    let vc = state[base + c];
    var acc = 0.0;
    var diag = 0.0;

    if (i > 0u) {
        let o = height(i - 1u, j);
        if (zc < o) { acc = acc + (vc - state[base + c - 1u]); diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); acc = acc + vc * a; diag = diag + a; }
    }
    if (i + 1u < params.nx) {
        let o = height(i + 1u, j);
        if (zc < o) { acc = acc + (vc - state[base + c + 1u]); diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); acc = acc + vc * a; diag = diag + a; }
    }
    if (j > 0u) {
        let o = height(i, j - 1u);
        if (zc < o) { acc = acc + (vc - state[base + c - params.nx]); diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); acc = acc + vc * a; diag = diag + a; }
    }
    if (j + 1u < params.ny) {
        let o = height(i, j + 1u);
        if (zc < o) { acc = acc + (vc - state[base + c + params.nx]); diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); acc = acc + vc * a; diag = diag + a; }
    }
    if (k > 0u) {
        acc = acc + (vc - state[base + c - plane]);
        diag = diag + 1.0;
    }
    if (k + 1u < params.nz && (f32(k + 1u) + 0.5) * params.dx < h) {
        acc = acc + (vc - state[base + c + plane]);
        diag = diag + 1.0;
    } else {
        let a = 1.0 / max((h - zc) / params.dx, params.theta_min);
        acc = acc + vc * a;
        diag = diag + a;
    }
    return vec2<f32>(acc * params.inv_dx2, diag * params.inv_dx2);
}

/// Somme d'arbre dans le groupe, une valeur par groupe. Ordre fixé par la structure :
/// reproductible sur une même carte, sans prétention d'identité entre cartes (ADR-175 D4).
fn fold_at(base: u32, value: f32, lid: u32, group: u32) {
    scratch[lid] = value;
    workgroupBarrier();
    for (var s = 32u; s > 0u; s >>= 1u) {
        if (lid < s) { scratch[lid] += scratch[lid + s]; }
        workgroupBarrier();
    }
    if (lid == 0u) { partial[base * groups_count() + group] = scratch[0]; }
    workgroupBarrier();
}

fn gather_at(base: u32, lid: u32) -> f32 {
    var v = 0.0;
    var i = lid;
    let origin = base * groups_count();
    loop {
        if (i >= groups_count()) { break; }
        v += partial[origin + i];
        i += 64u;
    }
    scratch[lid] = v;
    workgroupBarrier();
    for (var s = 32u; s > 0u; s >>= 1u) {
        if (lid < s) { scratch[lid] += scratch[lid + s]; }
        workgroupBarrier();
    }
    let out = scratch[0];
    workgroupBarrier();
    return out;
}

/// Second membre et préconditionneur, depuis la géométrie. `B` porte la divergence à l'entrée
/// et le second membre à la sortie — chaque fil ne touche que sa maille.
@compute @workgroup_size(64)
fn assemble(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    let plane = params.nx * params.ny;
    let i = c % params.nx;
    let j = (c / params.nx) % params.ny;
    let k = c / plane;
    let h = height(i, j);
    let zc = (f32(k) + 0.5) * params.dx;
    if (zc >= h) {
        state[at(B, c)] = 0.0;
        state[at(M, c)] = 0.0;
        return;
    }
    var b = params.scale * state[at(B, c)];
    var diag = 0.0;
    let side_value = params.rho_g * (zc - params.rest);
    if (i > 0u) {
        let o = height(i - 1u, j);
        if (zc < o) { diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); diag = diag + a; b = b + side_value * a * params.inv_dx2; }
    }
    if (i + 1u < params.nx) {
        let o = height(i + 1u, j);
        if (zc < o) { diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); diag = diag + a; b = b + side_value * a * params.inv_dx2; }
    }
    if (j > 0u) {
        let o = height(i, j - 1u);
        if (zc < o) { diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); diag = diag + a; b = b + side_value * a * params.inv_dx2; }
    }
    if (j + 1u < params.ny) {
        let o = height(i, j + 1u);
        if (zc < o) { diag = diag + 1.0; }
        else { let a = 1.0 / max((h - zc) / (h - o), params.theta_min); diag = diag + a; b = b + side_value * a * params.inv_dx2; }
    }
    if (k > 0u) { diag = diag + 1.0; }
    if (k + 1u < params.nz && (f32(k + 1u) + 0.5) * params.dx < h) {
        diag = diag + 1.0;
    } else {
        let a = 1.0 / max((h - zc) / params.dx, params.theta_min);
        diag = diag + a;
        b = b + params.rho_g * (h - params.rest) * a * params.inv_dx2;
    }
    state[at(B, c)] = b;
    if (diag > 0.0) { state[at(M, c)] = 1.0 / (diag * params.inv_dx2); } else { state[at(M, c)] = 0.0; }
}

/// `‖b‖²` replié dans la tranche 2, juste après l'assemblage : c'est l'échelle contre laquelle
/// le résidu se lit. Sans elle, un résidu « relatif » serait rapporté à la mauvaise grandeur.
@compute @workgroup_size(64)
fn bnorm_fold(@builtin(global_invocation_id) id: vec3<u32>,
              @builtin(local_invocation_index) lid: u32,
              @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        let b = state[at(B, c)];
        contribution = b * b;
    }
    fold_at(2u, contribution, lid, wid.x);
}

@compute @workgroup_size(64)
fn finish_bnorm(@builtin(local_invocation_index) lid: u32) {
    let v = gather_at(2u, lid);
    if (lid == 0u) { scalar[BNORM] = v; }
}

/// Départ froid : `x = 0`, donc `r = b`. Direction préconditionnée, et `⟨r, M r⟩` replié.
@compute @workgroup_size(64)
fn init(@builtin(global_invocation_id) id: vec3<u32>,
        @builtin(local_invocation_index) lid: u32,
        @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        let b = state[at(B, c)];
        let m = state[at(M, c)];
        state[at(X, c)] = 0.0;
        state[at(R, c)] = b;
        let z = m * b;
        state[at(Z, c)] = z;
        state[at(D, c)] = z;
        state[at(Q, c)] = 0.0;
        contribution = b * z;
    }
    fold_at(0u, contribution, lid, wid.x);
}

@compute @workgroup_size(64)
fn finish_rz(@builtin(local_invocation_index) lid: u32) {
    let v = gather_at(0u, lid);
    if (lid == 0u) { scalar[RZ] = v; }
}

/// `q = A·d`, puis `⟨d, q⟩` replié dans la même passe.
@compute @workgroup_size(64)
fn apply_fold(@builtin(global_invocation_id) id: vec3<u32>,
              @builtin(local_invocation_index) lid: u32,
              @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        let q = stencil3(D, c).x;
        state[at(Q, c)] = q;
        contribution = state[at(D, c)] * q;
    }
    fold_at(0u, contribution, lid, wid.x);
}

/// `alpha = ⟨r, M r⟩ / ⟨d, q⟩`. Un dénominateur non strictement positif rend `alpha = 0` :
/// le cycle devient neutre au lieu de produire un infini. Le pas reste borné, jamais refait.
@compute @workgroup_size(64)
fn finish_dq(@builtin(local_invocation_index) lid: u32) {
    let v = gather_at(0u, lid);
    if (lid == 0u) {
        scalar[DQ] = v;
        if (v > 0.0) { scalar[ALPHA] = scalar[RZ] / v; } else { scalar[ALPHA] = 0.0; }
    }
}

/// `x += α d`, `r −= α q`, `z = M r`, et `⟨r, M r⟩` neuf replié dans la tranche 1.
@compute @workgroup_size(64)
fn update(@builtin(global_invocation_id) id: vec3<u32>,
          @builtin(local_invocation_index) lid: u32,
          @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        let alpha = scalar[ALPHA];
        let r = state[at(R, c)] - alpha * state[at(Q, c)];
        state[at(X, c)] = state[at(X, c)] + alpha * state[at(D, c)];
        state[at(R, c)] = r;
        let z = state[at(M, c)] * r;
        state[at(Z, c)] = z;
        contribution = r * z;
    }
    fold_at(1u, contribution, lid, wid.x);
}

@compute @workgroup_size(64)
fn finish_beta(@builtin(local_invocation_index) lid: u32) {
    let v = gather_at(1u, lid);
    if (lid == 0u) {
        let previous = scalar[RZ];
        if (previous > 0.0) { scalar[BETA] = v / previous; } else { scalar[BETA] = 0.0; }
        scalar[RZ] = v;
    }
}

@compute @workgroup_size(64)
fn direction(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= params.cells) { return; }
    state[at(D, c)] = state[at(Z, c)] + scalar[BETA] * state[at(D, c)];
}

/// Diagnostic de fin : vrai résidu `b − A·x`, jamais la récurrence. Replié dans la tranche 2 ;
/// la relecture est **différée**, elle ne commande rien dans le pas.
@compute @workgroup_size(64)
fn residual_fold(@builtin(global_invocation_id) id: vec3<u32>,
                 @builtin(local_invocation_index) lid: u32,
                 @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < params.cells) {
        let r = state[at(B, c)] - stencil3(X, c).x;
        contribution = r * r;
    }
    fold_at(2u, contribution, lid, wid.x);
}

@compute @workgroup_size(64)
fn finish_residual(@builtin(local_invocation_index) lid: u32) {
    let v = gather_at(2u, lid);
    if (lid == 0u) { scalar[DQ] = v; }
}
