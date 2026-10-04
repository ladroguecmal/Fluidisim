// S481 — K2-2 : **l'air enfermé sur la carte** (docs/registres/CAMPAGNE-K2-S478.md, ADR-220 D1). La référence est
// `water_core::apic3d` (`apic3d_poches.rs`, S479) ; ce module la reproduit à côté du pas de `apic3d_carte.wgsl`, sans le toucher :
// mêmes liaisons (la structure `Params` est prise au texte du nuanceur principal à la compilation), deux tampons de plus.
//
// - **La détection** (`pockets_detect`) : une union-find sans verrou sur les mailles d'air. L'identifiant d'une maille `c` est
//   `c + 1` ; l'air libre est la racine 0, celle des mailles d'air de la rangée du haut. L'accrochage se fait toujours de la plus
//   grande racine vers la plus petite (`atomicMin`) : la racine finale d'une composante enfermée est sa plus petite maille, et les
//   poches se numérotent dans l'ordre de leur plus petite maille — l'ordre où la référence les remplit. Le résultat ne dépend pas
//   de l'ordre des fils.
// - Aucun atomique flottant : les sommes par poche se font dans un ordre fixe, un groupe par poche (S481 P3).

// --- Les deux tampons des poches ----------------------------------------------------------------------------------------
// `pko` (entiers) : [0, C) l'union-find ; [C, 2C) la poche de chaque maille (0 : aucune, b + 1) ; [2C, 3C) celle du pas d'avant ;
// [3C, 4C) le rang des racines ; puis l'en-tête (`PK_H`) et les listes.
@group(0) @binding(27) var<storage, read_write> pko: array<atomic<u32>>;
// `pkf` (flottants) : les grandeurs de chaque poche, `PF_*` tranches de `MAXP`.
@group(0) @binding(28) var<storage, read_write> pkf: array<f32>;

const AIR: u32 = 0u;
const WATER: u32 = 1u;
const SOLID: u32 = 2u;
@group(0) @binding(12) var<storage, read_write> label: array<u32>;
@group(0) @binding(11) var<storage, read_write> cellf: array<f32>;

const MAXP: u32 = 64u;
const NONE: u32 = 0xffffffffu;
// L'en-tête de `pko`, après les quatre tranches de mailles : compteurs.
const H_COUNT: u32 = 0u;      // poches détectées (avant la résorption), au plus MAXP
const H_OLD_COUNT: u32 = 1u;  // celles du pas d'avant
const H_OVERFLOW: u32 = 2u;   // poches laissées libres au-delà de MAXP, cumulées
const H_ROOTS: u32 = 3u;      // racines enfermées trouvées ce pas (toutes, même au-delà de MAXP)

fn pk_h() -> u32 {
    return 4u * P.cells;
}

fn uf_at(c: u32) -> u32 {
    return atomicLoad(&pko[c]);
}

fn cell_ijk(c: u32) -> vec3<u32> {
    return vec3<u32>(c % P.nx, (c / P.nx) % P.ny, c / (P.nx * P.ny));
}

// La racine de l'identifiant `v` (0 : l'air libre).
fn uf_find(v0: u32) -> u32 {
    var v = v0;
    loop {
        if v == 0u {
            return 0u;
        }
        let p = atomicLoad(&pko[v - 1u]);
        if p == v {
            return v;
        }
        v = p;
    }
    // Jamais atteint (naga demande un retour après la boucle).
    return 0u;
}

// Réunit les composantes de `a` et `b` : la plus grande racine s'accroche à la plus petite.
fn uf_union(a0: u32, b0: u32) {
    var a = a0;
    var b = b0;
    loop {
        a = uf_find(a);
        b = uf_find(b);
        if a == b {
            return;
        }
        if a > b {
            let t = a;
            a = b;
            b = t;
        }
        // `b` ≥ 1 est une racine : `pko[b − 1] == b` sauf si un autre fil vient de l'accrocher.
        let old = atomicMin(&pko[b - 1u], a);
        if old == b {
            return;
        }
        b = old;
    }
}

// 1. L'état d'avant gardé ; l'union-find initialisée : l'air de la rangée du haut est libre (0), toute autre maille d'air est sa
// propre racine, le reste n'en a pas.
@compute @workgroup_size(128)
fn pk_init(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c == 0u {
        atomicStore(&pko[pk_h() + H_OLD_COUNT], atomicLoad(&pko[pk_h() + H_COUNT]));
        atomicStore(&pko[pk_h() + 8u], 0u);
    }
    // Les recouvrements du pas (`pk_overlap`) repartent de zéro.
    if c < MAXP * MAXP {
        atomicStore(&pko[pk_h() + 128u + c], 0u);
    }
    if c >= P.cells {
        return;
    }
    atomicStore(&pko[2u * P.cells + c], atomicLoad(&pko[P.cells + c]));
    var v = NONE;
    if label[c] == AIR {
        if c / (P.nx * P.ny) == P.nz - 1u {
            v = 0u;
        } else {
            v = c + 1u;
        }
    }
    atomicStore(&pko[c], v);
}

// 2. Les réunions : chaque maille d'air avec ses voisines d'air en +x, +y, +z.
@compute @workgroup_size(128)
fn pk_merge(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells || label[c] != AIR {
        return;
    }
    let q = cell_ijk(c);
    if q.x + 1u < P.nx && label[c + 1u] == AIR {
        uf_union(c + 1u, c + 2u);
    }
    if q.y + 1u < P.ny && label[c + P.nx] == AIR {
        uf_union(c + 1u, c + P.nx + 1u);
    }
    if q.z + 1u < P.nz && label[c + P.nx * P.ny] == AIR {
        uf_union(c + 1u, c + P.nx * P.ny + 1u);
    }
}

// 3. L'aplatissement : chaque maille d'air porte sa racine.
@compute @workgroup_size(128)
fn pk_flatten(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells || label[c] != AIR {
        return;
    }
    atomicStore(&pko[c], uf_find(c + 1u));
}

// 4. La numérotation, par un groupe : les racines enfermées (`pko[c] == c + 1`) dans l'ordre des mailles. Chaque fil compte celles
// de sa tranche contiguë, un préfixe donne le rang de départ de chaque tranche, puis chaque fil écrit le rang de ses racines dans
// `[3C, 4C)`. Au-delà de MAXP, le rang est NONE : la poche reste libre, comptée (la référence fait de même).
var<workgroup> pk_scan: array<u32, 256>;

fn is_root(c: u32) -> bool {
    return label[c] == AIR && atomicLoad(&pko[c]) == c + 1u;
}

@compute @workgroup_size(256)
fn pk_number(@builtin(local_invocation_id) l: vec3<u32>) {
    let t = l.x;
    let chunk = (P.cells + 255u) / 256u;
    let lo = min(t * chunk, P.cells);
    let hi = min(lo + chunk, P.cells);
    var n = 0u;
    for (var c = lo; c < hi; c = c + 1u) {
        if is_root(c) {
            n = n + 1u;
        }
    }
    pk_scan[t] = n;
    workgroupBarrier();
    // Préfixe inclusif (Hillis–Steele).
    for (var s = 1u; s < 256u; s = s * 2u) {
        var v = 0u;
        if t >= s {
            v = pk_scan[t - s];
        }
        workgroupBarrier();
        pk_scan[t] = pk_scan[t] + v;
        workgroupBarrier();
    }
    var rank = pk_scan[t] - n;
    for (var c = lo; c < hi; c = c + 1u) {
        if is_root(c) {
            atomicStore(&pko[3u * P.cells + c], select(NONE, rank, rank < MAXP));
            rank = rank + 1u;
        }
    }
    if t == 255u {
        let total = pk_scan[255];
        atomicStore(&pko[pk_h() + H_ROOTS], total);
        atomicStore(&pko[pk_h() + H_COUNT], min(total, MAXP));
        if total > MAXP {
            atomicAdd(&pko[pk_h() + H_OVERFLOW], total - MAXP);
        }
    }
}

// 5. La poche de chaque maille : celle du rang de sa racine (0 pour l'air libre, l'eau, le solide, et au-delà de MAXP).
@compute @workgroup_size(128)
fn pk_assign(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    var b = 0u;
    if label[c] == AIR {
        let r = atomicLoad(&pko[c]);
        if r != 0u {
            let rank = atomicLoad(&pko[3u * P.cells + r - 1u]);
            if rank != NONE {
                b = rank + 1u;
            }
        }
    }
    atomicStore(&pko[P.cells + c], b);
}

// =====================================================================================================================
// **Le bilan par poche** (S481 P3 ; `pockets_detect`, étapes 3 à 6 de la référence).
//
// Deux listes dans l'ordre des mailles : LA, les mailles d'air de chaque poche (maille, poche) ; LW, les mailles d'eau qui bordent
// une poche, une entrée par poche distincte (la référence compte une maille d'eau une fois par poche). Puis un groupe par poche
// somme, dans un ordre fixe, son volume (la fraction d'air `clamp(0,5 + φ/dx, 0, 1)` de ses mailles et des mailles d'eau qui la
// bordent), son centre, ses mailles, et la pression de l'eau qui la borde (sa naissance). Les recouvrements avec les poches d'avant
// sont des compteurs entiers. Un fil fait ensuite l'héritage, la naissance, le rappel du volume et la résorption.

const F_PHI: u32 = 0u;
const F_P: u32 = 1u;
const H_LA: u32 = 4u;        // longueur de LA
const H_LW: u32 = 5u;        // longueur de LW
const H_LF: u32 = 6u;        // longueur de LF (la projection, P4)
const H_LIST_OVF: u32 = 8u;  // une liste a débordé (drapeau)
const H_REMAP: u32 = 16u;    // la poche gardée de chaque poche détectée (0 : résorbée), MAXP mots
const H_OVERLAP: u32 = 128u; // recouvrements, (nouvelle × MAXP + ancienne)
const HEADER_WORDS: u32 = 4224u; // 128 + MAXP·MAXP

// Les grandeurs de chaque poche dans `pkf`, une tranche de MAXP chacune.
const PF_AIR: u32 = 0u;
const PF_VFLUX: u32 = 2u;
const PF_VOL: u32 = 4u;
const PF_GEO: u32 = 5u;
const PF_P: u32 = 6u;
const PF_CX: u32 = 7u;
const PF_CELLS: u32 = 10u;
const PF_SP: u32 = 11u;
const PF_NP: u32 = 12u;
const PF_SCAL: u32 = 31u;    // [0] le dernier pas, s

const P_ATM: f32 = 101325.0;
const GAMMA_AIR: f32 = 1.4;
const RAPPEL_VOLUME_S: f32 = 0.1;

fn pf(field: u32, b: u32) -> u32 {
    return field * MAXP + b;
}

fn la_base() -> u32 {
    return pk_h() + HEADER_WORDS;
}

fn lw_base() -> u32 {
    return la_base() + 2u * P.cells;
}

fn lf_base() -> u32 {
    return lw_base() + 2u * P.cells;
}

fn of_at(c: u32) -> u32 {
    return atomicLoad(&pko[P.cells + c]);
}

// La voisine `m` (0 : −x, 1 : +x, 2 : −y, 3 : +y, 4 : −z, 5 : +z) ; NONE hors du domaine.
fn nb_cell(c: u32, m: u32) -> u32 {
    let q = cell_ijk(c);
    switch m {
        case 0u: { return select(NONE, c - 1u, q.x > 0u); }
        case 1u: { return select(NONE, c + 1u, q.x + 1u < P.nx); }
        case 2u: { return select(NONE, c - P.nx, q.y > 0u); }
        case 3u: { return select(NONE, c + P.nx, q.y + 1u < P.ny); }
        case 4u: { return select(NONE, c - P.nx * P.ny, q.z > 0u); }
        default: { return select(NONE, c + P.nx * P.ny, q.z + 1u < P.nz); }
    }
}

// La poche de la voisine `m` de la maille d'eau `c` (0 : aucune — pas d'eau, pas d'air, ou de l'air libre).
fn bord_b(c: u32, m: u32) -> u32 {
    let a = nb_cell(c, m);
    if a == NONE || label[a] != AIR {
        return 0u;
    }
    return of_at(a);
}

// La voisine `m` apporte-t-elle une poche que les précédentes n'ont pas ? Les poches distinctes que borde une maille d'eau, dans
// l'ordre des voisines, sans tableau local (FXC refuse l'écriture indexée dans un tableau de structure).
fn bord_new(c: u32, m: u32) -> bool {
    let b = bord_b(c, m);
    if b == 0u {
        return false;
    }
    for (var k = 0u; k < m; k = k + 1u) {
        if bord_b(c, k) == b {
            return false;
        }
    }
    return true;
}

fn bord_n(c: u32) -> u32 {
    if label[c] != WATER {
        return 0u;
    }
    var n = 0u;
    for (var m = 0u; m < 6u; m = m + 1u) {
        if bord_new(c, m) {
            n = n + 1u;
        }
    }
    return n;
}

// Les deux listes, par un groupe : compte par tranche, préfixe, écriture — l'ordre des mailles.
var<workgroup> pk_scan2: array<vec2<u32>, 256>;

@compute @workgroup_size(256)
fn pk_lists(@builtin(local_invocation_id) l: vec3<u32>) {
    let t = l.x;
    let chunk = (P.cells + 255u) / 256u;
    let lo = min(t * chunk, P.cells);
    let hi = min(lo + chunk, P.cells);
    var n = vec2<u32>(0u, 0u);
    for (var c = lo; c < hi; c = c + 1u) {
        if of_at(c) != 0u {
            n.x = n.x + 1u;
        }
        n.y = n.y + bord_n(c);
    }
    pk_scan2[t] = n;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var v = vec2<u32>(0u, 0u);
        if t >= s {
            v = pk_scan2[t - s];
        }
        workgroupBarrier();
        pk_scan2[t] = pk_scan2[t] + v;
        workgroupBarrier();
    }
    var at = pk_scan2[t] - n;
    var ovf = false;
    for (var c = lo; c < hi; c = c + 1u) {
        let b = of_at(c);
        if b != 0u {
            atomicStore(&pko[la_base() + 2u * at.x], c);
            atomicStore(&pko[la_base() + 2u * at.x + 1u], b);
            at.x = at.x + 1u;
        }
        if label[c] == WATER {
            for (var m = 0u; m < 6u; m = m + 1u) {
                if !bord_new(c, m) {
                    continue;
                }
                if at.y < P.cells {
                    atomicStore(&pko[lw_base() + 2u * at.y], c);
                    atomicStore(&pko[lw_base() + 2u * at.y + 1u], bord_b(c, m));
                } else {
                    ovf = true;
                }
                at.y = at.y + 1u;
            }
        }
    }
    if ovf {
        atomicStore(&pko[pk_h() + H_LIST_OVF], 1u);
    }
    if t == 255u {
        atomicStore(&pko[pk_h() + H_LA], pk_scan2[255].x);
        atomicStore(&pko[pk_h() + H_LW], min(pk_scan2[255].y, P.cells));
    }
}

// Les recouvrements avec les poches d'avant (compteurs entiers ; remis à zéro par `pk_init`).
@compute @workgroup_size(128)
fn pk_overlap(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    let b = of_at(c);
    let o = atomicLoad(&pko[2u * P.cells + c]);
    if b != 0u && o != 0u {
        atomicAdd(&pko[pk_h() + H_OVERLAP + (b - 1u) * MAXP + o - 1u], 1u);
    }
}

fn frac(c: u32) -> f32 {
    return clamp(0.5 + cellf[F_PHI * P.cells + c] / P.dx, 0.0, 1.0);
}

// La somme par poche : un groupe par poche, chaque fil une tranche fixe des listes, puis l'arbre — un ordre fixe.
var<workgroup> pk_red: array<array<f32, 7>, 256>;
var<workgroup> pk_n: u32;

@compute @workgroup_size(256)
fn pk_reduce(@builtin(local_invocation_id) l: vec3<u32>, @builtin(workgroup_id) w: vec3<u32>) {
    let t = l.x;
    let b = w.x;
    if t == 0u {
        pk_n = atomicLoad(&pko[pk_h() + H_COUNT]);
    }
    if b >= workgroupUniformLoad(&pk_n) {
        return;
    }
    let cell_volume = P.dx * P.dx * P.dx;
    var s = array<f32, 7>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let na = atomicLoad(&pko[pk_h() + H_LA]);
    for (var e = t; e < na; e = e + 256u) {
        if atomicLoad(&pko[la_base() + 2u * e + 1u]) != b + 1u {
            continue;
        }
        let c = atomicLoad(&pko[la_base() + 2u * e]);
        let f = frac(c) * cell_volume;
        let q = (vec3<f32>(cell_ijk(c)) + vec3<f32>(0.5)) * P.dx;
        s[0] = s[0] + f;
        s[1] = s[1] + f * q.x;
        s[2] = s[2] + f * q.y;
        s[3] = s[3] + f * q.z;
        s[4] = s[4] + 1.0;
        // La pression de l'eau qui la borde, au pas d'avant (la naissance).
        for (var m = 0u; m < 6u; m = m + 1u) {
            let n = nb_cell(c, m);
            if n != NONE && label[n] == WATER {
                s[5] = s[5] + cellf[F_P * P.cells + n];
                s[6] = s[6] + 1.0;
            }
        }
    }
    let nw = atomicLoad(&pko[pk_h() + H_LW]);
    for (var e = t; e < nw; e = e + 256u) {
        if atomicLoad(&pko[lw_base() + 2u * e + 1u]) == b + 1u {
            s[0] = s[0] + frac(atomicLoad(&pko[lw_base() + 2u * e])) * cell_volume;
        }
    }
    pk_red[t] = s;
    workgroupBarrier();
    for (var h = 128u; h > 0u; h = h / 2u) {
        if t < h {
            for (var k = 0u; k < 7u; k = k + 1u) {
                pk_red[t][k] = pk_red[t][k] + pk_red[t + h][k];
            }
        }
        workgroupBarrier();
    }
    if t == 0u {
        let r = pk_red[0];
        pkf[pf(PF_GEO, b)] = r[0];
        let inv = select(0.0, 1.0 / r[0], r[0] > 0.0);
        pkf[pf(PF_CX, b)] = r[1] * inv;
        pkf[pf(PF_CX + 1u, b)] = r[2] * inv;
        pkf[pf(PF_CX + 2u, b)] = r[3] * inv;
        pkf[pf(PF_CELLS, b)] = r[4];
        pkf[pf(PF_SP, b)] = r[5];
        pkf[pf(PF_NP, b)] = r[6];
    }
}

// L'héritage, la naissance, le rappel, la pression et la résorption, par un fil (MAXP poches).
@compute @workgroup_size(1)
fn pk_scalars() {
    let h = pk_h();
    let n = atomicLoad(&pko[h + H_COUNT]);
    let on = atomicLoad(&pko[h + H_OLD_COUNT]);
    var old_air: array<f32, 64>;
    var old_vf: array<f32, 64>;
    for (var o = 0u; o < on; o = o + 1u) {
        old_air[o] = pkf[pf(PF_AIR, o)];
        old_vf[o] = pkf[pf(PF_VFLUX, o)];
    }
    var air: array<f32, 64>;
    var vf: array<f32, 64>;
    for (var b = 0u; b < n; b = b + 1u) {
        air[b] = 0.0;
        vf[b] = 0.0;
    }
    // 4. L'air hérité : l'air de chaque poche d'avant partagé au prorata des mailles recouvertes.
    for (var o = 0u; o < on; o = o + 1u) {
        var total = 0u;
        for (var b = 0u; b < n; b = b + 1u) {
            total = total + atomicLoad(&pko[h + H_OVERLAP + b * MAXP + o]);
        }
        if total == 0u {
            continue;
        }
        for (var b = 0u; b < n; b = b + 1u) {
            let wgt = atomicLoad(&pko[h + H_OVERLAP + b * MAXP + o]);
            if wgt > 0u {
                let part = f32(wgt) / f32(total);
                air[b] = air[b] + old_air[o] * part;
                vf[b] = vf[b] + old_vf[o] * part;
            }
        }
    }
    let rappel = min(pkf[pf(PF_SCAL, 0u)] / RAPPEL_VOLUME_S, 1.0);
    let cell_volume = P.dx * P.dx * P.dx;
    var kept = 0u;
    for (var b = 0u; b < n; b = b + 1u) {
        let geo = pkf[pf(PF_GEO, b)];
        // 5. La naissance : la pression de l'eau qui la borde, plus l'atmosphère.
        if air[b] <= 0.0 {
            let np_ = pkf[pf(PF_NP, b)];
            var p0 = P_ATM;
            if np_ > 0.0 {
                p0 = p0 + pkf[pf(PF_SP, b)] / np_;
            }
            p0 = max(p0, 0.1 * P_ATM);
            air[b] = geo * pow(p0, 1.0 / GAMMA_AIR);
            vf[b] = geo;
        }
        let vol = vf[b] + (geo - vf[b]) * rappel;
        // 6. La résorption des poches de moins d'une maille ; les gardées se rangent dans l'ordre.
        if geo < cell_volume {
            atomicStore(&pko[h + H_REMAP + b], 0u);
            continue;
        }
        atomicStore(&pko[h + H_REMAP + b], kept + 1u);
        pkf[pf(PF_VOL, kept)] = vol;
        pkf[pf(PF_GEO, kept)] = geo;
        pkf[pf(PF_CELLS, kept)] = pkf[pf(PF_CELLS, b)];
        for (var a = 0u; a < 3u; a = a + 1u) {
            pkf[pf(PF_CX + a, kept)] = pkf[pf(PF_CX + a, b)];
        }
        pkf[pf(PF_AIR, kept)] = air[b];
        pkf[pf(PF_VFLUX, kept)] = vf[b];
        pkf[pf(PF_P, kept)] = pow(air[b] / vol, GAMMA_AIR);
        kept = kept + 1u;
    }
    atomicStore(&pko[h + H_COUNT], kept);
}

// La poche gardée de chaque maille (0 : résorbée, l'air y redevient libre).
@compute @workgroup_size(128)
fn pk_remap(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    let b = of_at(c);
    if b != 0u {
        atomicStore(&pko[P.cells + c], atomicLoad(&pko[pk_h() + H_REMAP + b - 1u]));
    }
}
