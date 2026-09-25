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
// [flux_x | bande_x | flux_y | bande_y].
@group(0) @binding(5) var<storage, read_write> work: array<f32>;
@group(0) @binding(6) var<uniform> s: Step;
// Surface **publiée** (ADR-175 D7) : la perturbation de hauteur par colonne, compensée. C'est un
// tampon à part pour que le rendu ne lie jamais un tampon interne de δ (I-13).
@group(0) @binding(7) var<storage, read_write> published: array<f32>;

// S343 : la disposition compacte du fond — dix champs par face, selon son axe `a` : `η`, `u` (3),
// `du/dt[a]`, la ligne `a` de `grad u` (3), `p`, `grad p[a]`. Le pas ne lit que ceux-là, et toujours ceux de
// l'axe de la face : `slot` rend leur rang depuis l'indice de `BackgroundSample`.
const FIELDS: u32 = 10u;
fn slot(field: u32) -> u32 {
    if (field == 0u) { return 0u; }
    if (field <= 6u) { return field - 3u; }
    if (field <= 9u) { return 4u; }
    if (field <= 18u) { return 5u + (field - 10u) % 3u; }
    if (field == 19u) { return 8u; }
    return 9u;
}

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
fn bg(f: u32, field: u32) -> f32 { return faces[f * FIELDS + slot(field)]; }

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
    let r2 = rx * rx + ry * ry;
    // Hors des bandes le cœur rend 1 exactement et saute la relaxation ; on ne confie pas
    // cette exactitude à l'`exp` de la carte.
    if (r2 == 0.0) { return 1.0; }
    return exp(-s.sponge_rate * s.dt * r2);
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

// ── Divergence des vitesses prédites, entrée du second membre couplé (S300) ─────────────────

fn cells() -> u32 { return s.nx * s.ny * s.nz; }
fn columns() -> u32 { return s.nx * s.ny; }

@compute @workgroup_size(64)
fn divergence(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= cells()) { return; }
    let plane = columns();
    let i = c % s.nx;
    let j = (c / s.nx) % s.ny;
    let k = c / plane;
    let o = s.faces;
    let fl = vel[o + fu(i, j, k)];
    let fr = vel[o + fu(i + 1u, j, k)];
    let ff = vel[o + fv(i, j, k)];
    let fk = vel[o + fv(i, j + 1u, k)];
    let fb = vel[o + fw(i, j, k)];
    let ft = vel[o + fw(i, j, k + 1u)];
    cells_in[plane + c] = ((fr - fl + ft - fb) + (fk - ff)) / s.dx;
}

// ── Correction aux faces et extrapolation verticale ──────────────────────────────────────────
//
// `correct_mobile3` et `extrapolate_mobile3` du cœur, avec la surface **totale** de début de pas
// (`cells_out`, écrite par `couple_columns`) : c'est elle qui classe les mailles, comme
// `height3` quand le couplage est armé. Les fantômes portent le fond : latéraux par l'échantillon
// de la face, celui du haut par `ghost_up` de S300.

fn height(i: u32, j: u32) -> f32 { return cells_out[j * s.nx + i]; }
fn wet(i: u32, j: u32, k: u32) -> bool { return (f32(k) + 0.5) * s.dx < height(i, j); }
fn pressure(i: u32, j: u32, k: u32) -> f32 { return state[(k * s.ny + j) * s.nx + i]; }

/// `ghost_side3` vu depuis la maille mouillée `(wi, wj)` vers la sèche `(di, dj)`, avec le fond de
/// la face `f` (`prepare_background3`) ; `sign` vaut +1 quand la mouillée est du côté bas.
fn ghost_side(wi: u32, wj: u32, di: u32, dj: u32, k: u32, f: u32, axis: u32, sign: f32) -> vec2<f32> {
    let zc = (f32(k) + 0.5) * s.dx;
    let h = height(wi, wj);
    let theta = max((h - zc) / (h - height(di, dj)), s.theta_min);
    let background = -(bg(f, 19u) + sign * (theta - 0.5) * s.dx * bg(f, 20u + axis));
    return vec2<f32>(1.0 / theta, s.rho * s.g_eff * (zc - s.rest) + background);
}

/// `ghost_up3` : couvercle de la colonne, perturbation compensée plus le fond.
fn ghost_up(i: u32, j: u32, k: u32) -> vec2<f32> {
    let col = j * s.nx + i;
    let theta = max((height(i, j) - (f32(k) + 0.5) * s.dx) / s.dx, s.theta_min);
    let roundoff = cells_in[columns() + cells() + col];
    let value = s.rho * s.g_eff * (difference(cells_in[col], s.rest) - roundoff) + cells_out[columns() + col];
    return vec2<f32>(1.0 / theta, value);
}

@compute @workgroup_size(64)
fn correct(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= s.faces) { return; }
    let q = decode(slot);
    let axis = q.w;
    let i = q.x;
    let j = q.y;
    let k = q.z;
    var value = vel[s.faces + slot];
    if (axis < 2u) {
        let n = select(s.ny, s.nx, axis == 0u);
        let pa = select(j, i, axis == 0u);
        if (pa > 0u && pa < n) {
            var xi = i;
            var yj = j;
            if (axis == 0u) { xi = i - 1u; } else { yj = j - 1u; }
            let lw = wet(xi, yj, k);
            let rw = wet(i, j, k);
            if (lw && rw) {
                value = value - s.k1 * (pressure(i, j, k) - pressure(xi, yj, k)) / s.dx;
            } else if (lw) {
                let g = ghost_side(xi, yj, i, j, k, slot, axis, 1.0);
                value = value - s.k1 * (g.y - pressure(xi, yj, k)) * g.x / s.dx;
            } else if (rw) {
                let g = ghost_side(i, j, xi, yj, k, slot, axis, -1.0);
                value = value - s.k1 * (pressure(i, j, k) - g.y) * g.x / s.dx;
            }
        }
    } else if (k >= 1u && wet(i, j, k - 1u)) {
        let below = pressure(i, j, k - 1u);
        if (k < s.nz && wet(i, j, k)) {
            value = value - s.k1 * (pressure(i, j, k) - below) / s.dx;
        } else {
            let g = ghost_up(i, j, k - 1u);
            value = value - s.k1 * (g.y - below) * g.x / s.dx;
        }
    }
    vel[slot] = value;
}

/// Une invocation par colonne : elle seule écrit ses faces u(i), v(j) et w, donc aucune course.
@compute @workgroup_size(64)
fn extrapolate(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let i = c % s.nx;
    let j = c / s.nx;
    if (i > 0u) {
        var has = false;
        var last = 0.0;
        for (var k = 0u; k < s.nz; k = k + 1u) {
            let f = fu(i, j, k);
            if (wet(i - 1u, j, k) || wet(i, j, k)) { last = vel[f]; has = true; }
            else if (has) { vel[f] = last; }
        }
    }
    if (j > 0u) {
        var has = false;
        var last = 0.0;
        for (var k = 0u; k < s.nz; k = k + 1u) {
            let f = fv(i, j, k);
            if (wet(i, j - 1u, k) || wet(i, j, k)) { last = vel[f]; has = true; }
            else if (has) { vel[f] = last; }
        }
    }
    var has = false;
    var last = 0.0;
    for (var k = 1u; k <= s.nz; k = k + 1u) {
        let f = fw(i, j, k);
        if (wet(i, j, k - 1u)) { last = vel[f]; has = true; }
        else if (has) { vel[f] = last; }
    }
}

// ── Transport de la surface, bandes de couplage, éponge, surface publiée ─────────────────────
//
// `transport_coupled3` puis `relax_coupled3`. Deux dispatchs : tous les débits lisent `eta^n`
// avant qu'aucune hauteur ne soit écrite, comme le cœur. La compensation de la somme (S233) ne se
// porte **pas** telle quelle : écrite en flottant, le compilateur l'annule (voir
// `exact_difference`). Le reste entre dans le fantôme du haut et dans la surface publiée.

/// `s − a`, **exacte et calculée en entiers** (S301). Pour deux flottants normaux positifs à moins
/// d'un facteur deux l'un de l'autre — une hauteur de colonne avant et après un pas —, la
/// différence est représentable (Sterbenz) ; le cœur l'obtient donc exactement en flottant.
/// Mesuré S301 sur cette carte : le compilateur réécrit `(a + b) − a` en `b`, les 165 colonnes
/// d'un pas rendant un reste nul là où le CPU en trouvait 165 non nuls. En entiers sur les bits
/// IEEE, il n'y a rien à simplifier. Même remède que la fraction de phase de S300 (L345).
fn exact_difference(s: f32, a: f32) -> f32 {
    let bs = bitcast<u32>(s);
    let ba = bitcast<u32>(a);
    let es = i32((bs >> 23u) & 0xffu);
    let ea = i32((ba >> 23u) & 0xffu);
    let ms = i32((bs & 0x7fffffu) | 0x800000u);
    let ma = i32((ba & 0x7fffffu) | 0x800000u);
    let e = min(es, ea);
    let d = (ms << u32(es - e)) - (ma << u32(ea - e));
    // d · 2^(e − 150), la puissance de deux construite sur ses bits : aucune `exp2` approchée.
    return f32(d) * bitcast<f32>(u32(e - 150 + 127) << 23u);
}

/// S358 — `x − a` exacte quand Sterbenz la garantit (deux flottants normaux positifs dont les exposants diffèrent d'un
/// au plus, ce que tient une hauteur de colonne près de son repos), flottante sinon. `(η − repos) − reste` écrit en
/// flottant est réassocié par le compilateur en `η − (repos + reste)`, où le reste se perd dans l'ulp du repos (L345) :
/// mesuré en S358 sur le pas linéaire, la somme publiée dérivait de 3,4·10⁻⁵ m dès le premier pas.
fn difference(x: f32, a: f32) -> f32 {
    let ex = i32((bitcast<u32>(x) >> 23u) & 0xffu);
    let ea = i32((bitcast<u32>(a) >> 23u) & 0xffu);
    if (x > 0.0 && a > 0.0 && ex > 0 && ea > 0 && ex < 255 && ea < 255 && abs(ex - ea) <= 1) {
        return exact_difference(x, a);
    }
    return x - a;
}

fn x_faces() -> u32 { return (s.nx + 1u) * s.ny; }
fn y_faces() -> u32 { return s.nx * (s.ny + 1u); }

/// `band3` : débit de fond entre le repos et la surface, dans la couche `k`.
fn band(f: u32, axis: u32, k: u32, surface: f32) -> f32 {
    let lower = f32(k) * s.dx;
    let upper = f32(k + 1u) * s.dx;
    let start = clamp(s.rest, lower, upper);
    let end = clamp(surface, lower, upper);
    return (end - start) * (bg(f, 4u + axis) + bg(f, 10u + 3u * axis + 2u) * (0.5 * (end + start) - (f32(k) + 0.5) * s.dx));
}

@compute @workgroup_size(64)
fn fluxes(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= x_faces() + y_faces()) { return; }
    var axis = 0u;
    var index = slot;
    var a = 0u;
    var b = 0u;
    var n = s.nx;
    if (slot < x_faces()) {
        a = slot % (s.nx + 1u);
        b = slot / (s.nx + 1u);
    } else {
        axis = 1u;
        index = slot - x_faces();
        a = index / s.nx;
        b = index % s.nx;
        n = s.ny;
    }
    // Colonne d'indice `a` le long de l'axe, `b` en travers ; face `a` de la couche `k`.
    var lo = 0u;
    var hi = 0u;
    if (a > 0u) { lo = select(b * s.nx + (a - 1u), (a - 1u) * s.nx + b, axis == 1u); }
    if (a < n) { hi = select(b * s.nx + a, a * s.nx + b, axis == 1u); }
    let edge_face = select(fu(a, b, 0u), fv(b, a, 0u), axis == 1u);
    var surface = 0.0;
    if (a == 0u) {
        surface = cells_in[hi] + bg(edge_face, 0u);
    } else if (a == n) {
        surface = cells_in[lo] + bg(edge_face, 0u);
    } else {
        surface = 0.5 * (cells_out[lo] + cells_out[hi]);
    }
    var flux = 0.0;
    var total_band = 0.0;
    for (var k = 0u; k < s.nz; k = k + 1u) {
        let f = select(fu(a, b, k), fv(b, a, k), axis == 1u);
        if (a > 0u && a < n) {
            let wet_part = clamp((surface - f32(k) * s.dx) / s.dx, 0.0, 1.0);
            if (wet_part > 0.0) { flux = flux + vel[f] * s.dx * wet_part; }
        }
        total_band = total_band + band(f, axis, k, surface);
    }
    let base = select(0u, 2u * x_faces(), axis == 1u);
    let span = select(x_faces(), y_faces(), axis == 1u);
    work[base + index] = flux;
    work[base + span + index] = total_band;
}

/// Hauteur d'une colonne, puis relaxation d'éponge, puis publication. Une invocation par colonne.
@compute @workgroup_size(64)
fn advance(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let i = c % s.nx;
    let j = c / s.nx;
    let fx = x_faces();
    let fy = y_faces();
    let l = j * (s.nx + 1u) + i;
    let f = j * s.nx + i;
    let xs = (work[l + 1u] - work[l]) + (work[fx + l + 1u] - work[fx + l]);
    let ys = (work[2u * fx + f + s.nx] - work[2u * fx + f]) + (work[2u * fx + fy + f + s.nx] - work[2u * fx + fy + f]);
    let r = columns() + cells() + c;
    var eta = cells_in[c];
    var roundoff = cells_in[r];
    var increment = -s.transport * (xs + ys) - roundoff;
    var height = eta + increment;
    roundoff = exact_difference(height, eta) - increment;
    eta = height;
    let factor = sponge((f32(i) + 0.5) * s.dx, (f32(j) + 0.5) * s.dx);
    if (factor != 1.0) {
        increment = (factor - 1.0) * (eta - s.rest) - factor * roundoff;
        height = eta + increment;
        roundoff = exact_difference(height, eta) - increment;
        eta = height;
    }
    cells_in[c] = eta;
    cells_in[r] = roundoff;
    published[c] = difference(eta, s.rest) - roundoff;
}

// ── Diagnostics D3 : qualité mesurée sur la carte, relue en différé ──────────────────────────
//
// ADR-175 D3 : chaque pas mesure la divergence projetée des lignes franches — la grandeur
// d'ADR-144, `max|div u|·dx/max|u|` sur les mailles mouillées sans fantôme — et la masse de la
// perturbation. Rien ici ne commande le pas : le résultat part dans un anneau de relecture et
// l'hôte le lit quand la carte l'a rendu, avec son âge. Au-dessus de 10⁻⁵, le pas est déclaré
// dégradé ; il n'est ni refusé ni refait.
//
// Partiels par groupe puis résultat final dans `work`, après les débits : les deux usages ne se
// chevauchent pas dans le temps, mais leurs tranches restent distinctes pour la lisibilité.

var<workgroup> reduce_max: array<vec3<f32>, 64>;
var<workgroup> reduce_sum: array<vec2<f32>, 64>;

fn diag_base() -> u32 { return 2u * x_faces() + 2u * y_faces(); }
fn diag_groups() -> u32 { return (max(cells(), s.faces) + 63u) / 64u; }
fn diag_result() -> u32 { return diag_base() + 5u * diag_groups(); }

/// Ligne franche au sens du cœur : mouillée, et aucune de ses faces n'est un fantôme — voisins
/// latéraux existants tous mouillés, voisin du dessus existant et mouillé. Un mur n'est pas un
/// fantôme.
fn plain(i: u32, j: u32, k: u32) -> bool {
    if (i > 0u && !wet(i - 1u, j, k)) { return false; }
    if (i + 1u < s.nx && !wet(i + 1u, j, k)) { return false; }
    if (j > 0u && !wet(i, j - 1u, k)) { return false; }
    if (j + 1u < s.ny && !wet(i, j + 1u, k)) { return false; }
    return k + 1u < s.nz && wet(i, j, k + 1u);
}

/// Pas de retour anticipé : les barrières doivent être atteintes par tout le groupe.
@compute @workgroup_size(64)
fn diagnose(@builtin(global_invocation_id) gid: vec3<u32>,
            @builtin(local_invocation_index) lid: u32,
            @builtin(workgroup_id) wid: vec3<u32>) {
    let n = gid.x;
    var whole = 0.0;
    var franche = 0.0;
    var speed = 0.0;
    var mass = 0.0;
    var outside = 0.0;
    if (n < cells()) {
        let i = n % s.nx;
        let j = (n / s.nx) % s.ny;
        let k = n / columns();
        if (wet(i, j, k)) {
            let d = abs(((vel[fu(i + 1u, j, k)] - vel[fu(i, j, k)] + vel[fw(i, j, k + 1u)] - vel[fw(i, j, k)])
                + (vel[fv(i, j + 1u, k)] - vel[fv(i, j, k)])) / s.dx);
            whole = d;
            if (plain(i, j, k)) { franche = d; }
        }
    }
    if (n < s.faces) { speed = abs(vel[n]); }
    if (n < columns()) {
        mass = published[n];
        // Bornes du cœur (`mobile_in_bounds`) sur la surface totale de fin de pas.
        let total = cells_in[n] + bg(fw(n % s.nx, n / s.nx, 0u), 0u);
        if (!(total >= 2.0 * s.dx && total <= f32(s.nz - 1u) * s.dx)) { outside = 1.0; }
    }
    reduce_max[lid] = vec3<f32>(whole, franche, speed);
    reduce_sum[lid] = vec2<f32>(mass, outside);
    workgroupBarrier();
    for (var half_width = 32u; half_width > 0u; half_width >>= 1u) {
        if (lid < half_width) {
            reduce_max[lid] = max(reduce_max[lid], reduce_max[lid + half_width]);
            reduce_sum[lid] = reduce_sum[lid] + reduce_sum[lid + half_width];
        }
        workgroupBarrier();
    }
    if (lid == 0u) {
        let base = diag_base() + 5u * wid.x;
        work[base] = reduce_max[0].x;
        work[base + 1u] = reduce_max[0].y;
        work[base + 2u] = reduce_max[0].z;
        work[base + 3u] = reduce_sum[0].x;
        work[base + 4u] = reduce_sum[0].y;
    }
}

/// Un seul groupe : rassemble les partiels. Résultat : divergence de toutes les lignes et des
/// lignes franches, déjà rapportées à `max|u|/dx` comme ADR-144 ; `max|u|` ; volume de la
/// perturbation (m³) ; colonnes hors bornes.
@compute @workgroup_size(64)
fn diagnose_finish(@builtin(local_invocation_index) lid: u32) {
    var m = vec3<f32>(0.0);
    var t = vec2<f32>(0.0);
    var g = lid;
    loop {
        if (g >= diag_groups()) { break; }
        let base = diag_base() + 5u * g;
        m = max(m, vec3<f32>(work[base], work[base + 1u], work[base + 2u]));
        t = t + vec2<f32>(work[base + 3u], work[base + 4u]);
        g = g + 64u;
    }
    reduce_max[lid] = m;
    reduce_sum[lid] = t;
    workgroupBarrier();
    for (var half_width = 32u; half_width > 0u; half_width >>= 1u) {
        if (lid < half_width) {
            reduce_max[lid] = max(reduce_max[lid], reduce_max[lid + half_width]);
            reduce_sum[lid] = reduce_sum[lid] + reduce_sum[lid + half_width];
        }
        workgroupBarrier();
    }
    if (lid == 0u) {
        let r = diag_result();
        let speed = reduce_max[0].z;
        var ratio = 0.0;
        if (speed > 0.0) { ratio = s.dx / speed; }
        work[r] = reduce_max[0].x * ratio;
        work[r + 1u] = reduce_max[0].y * ratio;
        work[r + 2u] = speed;
        work[r + 3u] = reduce_sum[0].x * s.dx * s.dx;
        work[r + 4u] = reduce_sum[0].y;
    }
}
