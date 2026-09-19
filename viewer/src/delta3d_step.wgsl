// S301 / ADR-175 D1 — **le pas couplé résident sur la carte**, étages propres au pas.
//
// Ce module ne réécrit ni le fond (S300, `delta3d_background.wgsl`) ni la projection (S299,
// `delta3d_cg.wgsl`) : les trois sources vivent sur le même device et lisent les mêmes tampons.
// Ici vivent les étages qui manquaient — prédiction, divergence, correction, extrapolation,
// transport, éponge, surface publiée — chacun porté de `delta3d_mobile.rs` et
// `delta3d_coupling.rs` dans l'ordre d'opérations du cœur. Aucune identité au bit n'est promise
// (ADR-175 D4) ; ce qui est promis, c'est la même formule.
//
// Rien ici ne revient au CPU pendant le pas. Aucun état n'est sérialisé (I-17).

struct Step {
    nx: u32,
    ny: u32,
    nz: u32,
    faces: u32,
    dx: f32,
    dt: f32,
    rho: f32,
    g_eff: f32,
    rest: f32,
    k1: f32,
    transport: f32,
    theta_min: f32,
    sponge_x: f32,
    sponge_y: f32,
    sponge_rate: f32,
    _pad: f32,
};

// Vitesses aux faces : [u | v | w] courantes, puis [u | v | w] prédites, même rangement que le
// cœur. L'indice d'une face est aussi celui de son échantillon de fond dans `faces`.
@group(0) @binding(0) var<storage, read_write> vel: array<f32>;
// Échantillons de fond aux faces, 26 flottants chacun, écrits par `sample_faces` (S300).
@group(0) @binding(1) var<storage, read> faces: array<f32>;
// [eta (colonnes) | divergence (mailles) | eta_roundoff (colonnes)] : l'entrée du couplage S300.
@group(0) @binding(2) var<storage, read_write> cells_in: array<f32>;
// [surface totale (colonnes) | fantôme du haut (colonnes) | second membre | préconditionneur].
@group(0) @binding(3) var<storage, read> cells_out: array<f32>;
// État du gradient conjugué ; la tranche 0 est la pression.
@group(0) @binding(4) var<storage, read> state: array<f32>;
// [flux_x | bande_x | flux_y | bande_y | surface publiée].
@group(0) @binding(5) var<storage, read_write> work: array<f32>;
@group(0) @binding(6) var<uniform> s: Step;

const FIELDS: u32 = 26u;

fn n_u() -> u32 { return (s.nx + 1u) * s.ny * s.nz; }
fn n_v() -> u32 { return s.nx * (s.ny + 1u) * s.nz; }
fn fu(i: u32, j: u32, k: u32) -> u32 { return (k * s.ny + j) * (s.nx + 1u) + i; }
fn fv(i: u32, j: u32, k: u32) -> u32 { return n_u() + (k * (s.ny + 1u) + j) * s.nx + i; }
fn fw(i: u32, j: u32, k: u32) -> u32 { return n_u() + n_v() + (k * s.ny + j) * s.nx + i; }

fn unit(a: u32) -> vec3<u32> { return vec3<u32>(u32(a == 0u), u32(a == 1u), u32(a == 2u)); }
fn component(v: vec3<u32>, a: u32) -> u32 {
    if (a == 0u) { return v.x; }
    if (a == 1u) { return v.y; }
    return v.z;
}
fn dims(axis: u32) -> vec3<u32> {
    return vec3<u32>(s.nx + u32(axis == 0u), s.ny + u32(axis == 1u), s.nz + u32(axis == 2u));
}
fn face(axis: u32, p: vec3<u32>) -> u32 {
    if (axis == 0u) { return fu(p.x, p.y, p.z); }
    if (axis == 1u) { return fv(p.x, p.y, p.z); }
    return fw(p.x, p.y, p.z);
}
fn cur(axis: u32, p: vec3<u32>) -> f32 { return vel[face(axis, p)]; }
fn bg(f: u32, field: u32) -> f32 { return faces[f * FIELDS + field]; }

/// Emplacement de face → (i, j, k, axe), par soustractions successives comme `sample_faces`.
fn decode(slot: u32) -> vec4<u32> {
    var axis = 0u;
    var rest = slot;
    loop {
        let d = dims(axis);
        let size = d.x * d.y * d.z;
        if (rest < size || axis == 2u) { break; }
        rest = rest - size;
        axis = axis + 1u;
    }
    let d = dims(axis);
    return vec4<u32>(rest % d.x, (rest / d.x) % d.y, rest / (d.x * d.y), axis);
}

// ── Prédiction : advection MAC centrée, couplage au fond, éponge ─────────────────────────────
//
// `advect_mobile3` puis `predict_coupled3` du cœur, fusionnés par face : les deux ne lisent que
// les vitesses **courantes** et n'écrivent que la vitesse prédite de leur face.

/// `advect_mobile3`, face par face. Les faces que le cœur n'advecte pas — bords en x et y,
/// fond et sommet en z — gardent leur valeur.
fn advect(axis: u32, i: u32, j: u32, k: u32) -> f32 {
    let h = 0.5 / s.dx;
    if (axis == 0u) {
        let uc = vel[fu(i, j, k)];
        if (i == 0u || i >= s.nx) { return uc; }
        let ux = (vel[fu(i + 1u, j, k)] - vel[fu(i - 1u, j, k)]) * h;
        var up = uc; if (k + 1u < s.nz) { up = vel[fu(i, j, k + 1u)]; }
        var dn = uc; if (k > 0u) { dn = vel[fu(i, j, k - 1u)]; }
        let uz = (up - dn) * h;
        let wc = 0.25 * (vel[fw(i - 1u, j, k)] + vel[fw(i, j, k)] + vel[fw(i - 1u, j, k + 1u)] + vel[fw(i, j, k + 1u)]);
        var rt = uc; if (j + 1u < s.ny) { rt = vel[fu(i, j + 1u, k)]; }
        var lf = uc; if (j > 0u) { lf = vel[fu(i, j - 1u, k)]; }
        let uy = (rt - lf) * h;
        let vc = 0.25 * (vel[fv(i - 1u, j, k)] + vel[fv(i, j, k)] + vel[fv(i - 1u, j + 1u, k)] + vel[fv(i, j + 1u, k)]);
        return uc - s.dt * (uc * ux + wc * uz + vc * uy);
    }
    if (axis == 1u) {
        let uc = vel[fv(i, j, k)];
        if (j == 0u || j >= s.ny) { return uc; }
        let ux = (vel[fv(i, j + 1u, k)] - vel[fv(i, j - 1u, k)]) * h;
        var up = uc; if (k + 1u < s.nz) { up = vel[fv(i, j, k + 1u)]; }
        var dn = uc; if (k > 0u) { dn = vel[fv(i, j, k - 1u)]; }
        let uz = (up - dn) * h;
        let wc = 0.25 * (vel[fw(i, j - 1u, k)] + vel[fw(i, j, k)] + vel[fw(i, j - 1u, k + 1u)] + vel[fw(i, j, k + 1u)]);
        var rt = uc; if (i + 1u < s.nx) { rt = vel[fv(i + 1u, j, k)]; }
        var lf = uc; if (i > 0u) { lf = vel[fv(i - 1u, j, k)]; }
        let uy = (rt - lf) * h;
        let vc = 0.25 * (vel[fu(i, j - 1u, k)] + vel[fu(i, j, k)] + vel[fu(i + 1u, j - 1u, k)] + vel[fu(i + 1u, j, k)]);
        return uc - s.dt * (uc * ux + wc * uz + vc * uy);
    }
    let wc = vel[fw(i, j, k)];
    if (k == 0u || k >= s.nz) { return wc; }
    var rt = wc; if (i + 1u < s.nx) { rt = vel[fw(i + 1u, j, k)]; }
    var lf = wc; if (i > 0u) { lf = vel[fw(i - 1u, j, k)]; }
    let wx = (rt - lf) * h;
    let wz = (vel[fw(i, j, k + 1u)] - vel[fw(i, j, k - 1u)]) * h;
    let uc = 0.25 * (vel[fu(i, j, k - 1u)] + vel[fu(i + 1u, j, k - 1u)] + vel[fu(i, j, k)] + vel[fu(i + 1u, j, k)]);
    var bk = wc; if (j + 1u < s.ny) { bk = vel[fw(i, j + 1u, k)]; }
    var fr = wc; if (j > 0u) { fr = vel[fw(i, j - 1u, k)]; }
    let wy = (bk - fr) * h;
    let vc = 0.25 * (vel[fv(i, j, k - 1u)] + vel[fv(i, j + 1u, k - 1u)] + vel[fv(i, j, k)] + vel[fv(i, j + 1u, k)]);
    return wc - s.dt * (uc * wx + wc * wz + vc * wy);
}

/// `collocated3` : composante `a` au centre d'une face d'axe `axis`, quatre voisins dans l'ordre
/// du cœur ; au sommet `w`, deux voisins de la dernière couche.
fn collocated(axis: u32, a: u32, p: vec3<u32>) -> f32 {
    if (a == axis) { return cur(a, p); }
    let lo = p - unit(axis);
    let hi = lo + unit(a);
    if (axis == 2u && p.z == s.nz) { return 0.5 * (cur(a, lo) + cur(a, hi)); }
    let q = lo + unit(axis);
    let r = hi + unit(axis);
    if (a < axis) { return 0.25 * (cur(a, lo) + cur(a, hi) + cur(a, q) + cur(a, r)); }
    return 0.25 * (cur(a, lo) + cur(a, q) + cur(a, hi) + cur(a, r));
}

/// Dérivée centrée de la composante `axis` selon `a`, repliée aux bords ; au sommet `w`, décentrée.
fn derivative(axis: u32, a: u32, p: vec3<u32>, end: vec3<u32>) -> f32 {
    let center = cur(axis, p);
    let pa = component(p, a);
    var below = center; if (pa > 0u) { below = cur(axis, p - unit(a)); }
    var above = center; if (pa + 1u < component(end, a)) { above = cur(axis, p + unit(a)); }
    if (axis == 2u && a == 2u && p.z == s.nz) { return (center - below) / s.dx; }
    return (above - below) * (0.5 / s.dx);
}

/// `extra3` : advection croisée fond/perturbation et résidu de quantité de mouvement du fond
/// (`momentum_residual(rho, 0)`), dans l'ordre de sommation du cœur. Ligne `axis` de `grad_u`.
fn extra(f: u32, axis: u32, v0: f32, v1: f32, v2: f32, d0: f32, d1: f32, d2: f32) -> f32 {
    let g = 10u + 3u * axis;
    let advection = (bg(f, 4u) * bg(f, g) + bg(f, 5u) * bg(f, g + 1u)) + bg(f, 6u) * bg(f, g + 2u);
    let residual = (bg(f, 7u + axis) + bg(f, 20u + axis) / s.rho) + advection;
    return bg(f, 4u) * d0 + bg(f, 6u) * d2 + v0 * bg(f, g) + v2 * bg(f, g + 2u) + residual
        + bg(f, 5u) * d1 + v1 * bg(f, g + 1u);
}

fn ramp(x: f32, n: u32, width: f32) -> f32 {
    if (width == 0.0) { return 0.0; }
    return max(1.0 - min(x, f32(n) * s.dx - x) / width, 0.0);
}

/// Facteur d'éponge d'ADR-164, taux quadratique. Le cœur le calcule en f64 ; ici en f32.
fn sponge(x: f32, y: f32) -> f32 {
    if (s.sponge_rate == 0.0) { return 1.0; }
    let rx = ramp(x, s.nx, s.sponge_x);
    let ry = ramp(y, s.ny, s.sponge_y);
    return exp(-s.sponge_rate * s.dt * (rx * rx + ry * ry));
}

@compute @workgroup_size(64)
fn predict(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= s.faces) { return; }
    let q = decode(slot);
    let axis = q.w;
    let p = q.xyz;
    var value = advect(axis, p.x, p.y, p.z);
    let end = dims(axis);
    let pa = component(p, axis);
    // Le cœur saute la face 0 et, en x et y, la face `n` — pas `n + 1`, qui n'existe pas.
    if (pa != 0u && (axis == 2u || pa + 1u < component(end, axis))) {
        let v0 = collocated(axis, 0u, p);
        let v1 = collocated(axis, 1u, p);
        let v2 = collocated(axis, 2u, p);
        let d0 = derivative(axis, 0u, p, end);
        let d1 = derivative(axis, 1u, p, end);
        let d2 = derivative(axis, 2u, p, end);
        let add = s.dt * extra(slot, axis, v0, v1, v2, d0, d1, d2);
        let x = (f32(p.x) + select(0.5, 0.0, axis == 0u)) * s.dx;
        let y = (f32(p.y) + select(0.5, 0.0, axis == 1u)) * s.dx;
        value = (value - add) * sponge(x, y);
    }
    vel[s.faces + slot] = value;
}
