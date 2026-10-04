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
const H_ANY: u32 = 9u;        // S482 : racines enfermées comptées par l'aplatissement (0 : rien d'enfermé, les parcours sautent)

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
        atomicStore(&pko[pk_h() + H_ANY], 0u);
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
    let r = uf_find(c + 1u);
    atomicStore(&pko[c], r);
    if r == c + 1u {
        atomicAdd(&pko[pk_h() + H_ANY], 1u);
    }
}

// 4. La numérotation, par un groupe : les racines enfermées (`pko[c] == c + 1`) dans l'ordre des mailles. Chaque fil compte celles
// de sa tranche contiguë, un préfixe donne le rang de départ de chaque tranche, puis chaque fil écrit le rang de ses racines dans
// `[3C, 4C)`. Au-delà de MAXP, le rang est NONE : la poche reste libre, comptée (la référence fait de même).
var<workgroup> pk_scan: array<u32, 256>;

fn is_root(c: u32) -> bool {
    return label[c] == AIR && atomicLoad(&pko[c]) == c + 1u;
}

var<workgroup> pk_any: u32;

// S482 : rien d'enfermé (le compte de l'aplatissement), les parcours par un groupe sortent aussitôt.
fn nothing_enclosed(t: u32) -> bool {
    if t == 0u {
        pk_any = atomicLoad(&pko[pk_h() + H_ANY]);
    }
    return workgroupUniformLoad(&pk_any) == 0u;
}

@compute @workgroup_size(256)
fn pk_number(@builtin(local_invocation_id) l: vec3<u32>) {
    let t = l.x;
    if nothing_enclosed(t) {
        if t == 0u {
            atomicStore(&pko[pk_h() + H_ROOTS], 0u);
            atomicStore(&pko[pk_h() + H_COUNT], 0u);
        }
        return;
    }
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
// `POCHE_MAILLES_MIN` de la référence (S481).
const POCHE_MAILLES_MIN: f32 = 8.0;

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
    if nothing_enclosed(t) {
        if t == 0u {
            atomicStore(&pko[pk_h() + H_LA], 0u);
            atomicStore(&pko[pk_h() + H_LW], 0u);
        }
        return;
    }
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
        if geo < cell_volume || pkf[pf(PF_CELLS, b)] < POCHE_MAILLES_MIN {
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

// =====================================================================================================================
// **La projection avec poches** (S481 P4 ; `project_with_pockets`). Le gradient conjugué diagonal de la carte, plus une inconnue
// par poche : sa pression relative `x_b` (Pa). Les lignes des mailles d'eau ne changent pas (`assemble` : la poche voisine compte
// `1/θ` dans la diagonale) ; seul `A·d` y voit `d_b` au lieu de zéro. La ligne d'une poche :
// `(s + Σ 1/θ)·x_b − Σ d_c/θ = s·(P_b − P_atm) + (scale/dx)·Σ u*_sortant`, `s = ρ·V/(γ·P·dt²·dx)` — symétrique avec celles de
// l'eau : le système reste défini positif. `A·d` des poches : une réduction par poche sur LF, les faces eau | poche, dans un ordre
// fixe. Les noyaux d'`apic3d_carte.wgsl` qui ne voient pas les poches (`assemble`, `cg_init_reduce`, `cg_update`, `cg_direction`)
// sont repris tels quels ; ceux-ci les remplacent dans la séquence.

@group(0) @binding(9) var<storage, read_write> faces: array<f32>;
@group(0) @binding(13) var<storage, read_write> partials: array<f32>;
@group(0) @binding(14) var<storage, read_write> scalars: array<f32>;

const F_RHS: u32 = 2u;
const F_R: u32 = 3u;
const F_Z: u32 = 4u;
const F_D: u32 = 5u;
const F_Q: u32 = 6u;
const F_DIAG: u32 = 7u;
const S_B2: u32 = 0u;
const S_RZ: u32 = 1u;
const S_RR: u32 = 2u;
const S_DQ: u32 = 3u;
const S_ALPHA: u32 = 4u;
const S_BETA: u32 = 5u;
const S_DONE: u32 = 6u;
const S_IT: u32 = 7u;
// Grandeurs des poches pour le gradient conjugué.
const PF_DIAG: u32 = 13u;
const PF_RHS: u32 = 14u;
const PF_X: u32 = 15u;
const PF_R: u32 = 16u;
const PF_Z: u32 = 17u;
const PF_D: u32 = 18u;
const PF_Q: u32 = 19u;
const PF_DV: u32 = 22u;
// `PF_SCAL` : [1] Σ r·z des poches, [2] Σ r·r, au dernier α.
const PK_LF_PAYLOAD: u32 = 2048u; // 32·MAXP : `1/θ` et le flux sortant de chaque face de LF, deux mots

fn cf(field: u32, c: u32) -> u32 {
    return field * P.cells + c;
}

// `theta` d'`apic3d_carte.wgsl`.
fn theta(c: u32, a: u32) -> f32 {
    let fc = cellf[cf(F_PHI, c)];
    let fa = cellf[cf(F_PHI, a)];
    return max(fc / (fc - fa), P.theta_min);
}

// La face de la voisine `m` (`neighbour` d'`apic3d_carte.wgsl`), même au bord.
fn face_of_nb(c: u32, m: u32) -> u32 {
    let q = cell_ijk(c);
    let fu = (q.z * P.ny + q.y) * (P.nx + 1u) + q.x;
    let fv = P.nu + (q.z * (P.ny + 1u) + q.y) * P.nx + q.x;
    let fw = P.nu + P.nv + (q.z * P.ny + q.y) * P.nx + q.x;
    switch m {
        case 0u: { return fu; }
        case 1u: { return fu + 1u; }
        case 2u: { return fv; }
        case 3u: { return fv + P.nx; }
        case 4u: { return fw; }
        default: { return fw + P.nx * P.ny; }
    }
}

// La poche de la voisine `m` d'une maille d'eau (0 : aucune).
fn pocket_nb(c: u32, m: u32) -> u32 {
    let a = nb_cell(c, m);
    if a == NONE || label[a] != AIR {
        return 0u;
    }
    return of_at(a);
}

// LF, les faces eau | poche, dans l'ordre des mailles puis des voisines : (maille, poche + 8·voisine) ; `1/θ` et le flux sortant
// de la poche par cette face (`−signe·u*`) dans `pkf`.
@compute @workgroup_size(256)
fn pk_faces(@builtin(local_invocation_id) l: vec3<u32>) {
    let t = l.x;
    // S482 : sans poche gardée, aucune face.
    if t == 0u {
        pk_any = atomicLoad(&pko[pk_h() + H_COUNT]);
    }
    if workgroupUniformLoad(&pk_any) == 0u {
        if t == 0u {
            atomicStore(&pko[pk_h() + H_LF], 0u);
        }
        return;
    }
    let chunk = (P.cells + 255u) / 256u;
    let lo = min(t * chunk, P.cells);
    let hi = min(lo + chunk, P.cells);
    var n = 0u;
    for (var c = lo; c < hi; c = c + 1u) {
        if label[c] != WATER {
            continue;
        }
        for (var m = 0u; m < 6u; m = m + 1u) {
            if pocket_nb(c, m) != 0u {
                n = n + 1u;
            }
        }
    }
    pk_scan[t] = n;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var v = 0u;
        if t >= s {
            v = pk_scan[t - s];
        }
        workgroupBarrier();
        pk_scan[t] = pk_scan[t] + v;
        workgroupBarrier();
    }
    var at = pk_scan[t] - n;
    var ovf = false;
    for (var c = lo; c < hi; c = c + 1u) {
        if label[c] != WATER {
            continue;
        }
        for (var m = 0u; m < 6u; m = m + 1u) {
            let b = pocket_nb(c, m);
            if b == 0u {
                continue;
            }
            if at < P.cells {
                atomicStore(&pko[lf_base() + 2u * at], c);
                atomicStore(&pko[lf_base() + 2u * at + 1u], b + 8u * m);
                let sign = select(1.0, -1.0, m % 2u == 0u);
                pkf[PK_LF_PAYLOAD + 2u * at] = 1.0 / theta(c, nb_cell(c, m));
                pkf[PK_LF_PAYLOAD + 2u * at + 1u] = -sign * faces[face_of_nb(c, m)];
            } else {
                ovf = true;
            }
            at = at + 1u;
        }
    }
    if ovf {
        atomicStore(&pko[pk_h() + H_LIST_OVF], 1u);
    }
    if t == 255u {
        atomicStore(&pko[pk_h() + H_LF], min(pk_scan[255], P.cells));
    }
}

fn lf_pocket(e: u32) -> u32 {
    return atomicLoad(&pko[lf_base() + 2u * e + 1u]) % 8u;
}

// Une somme par poche sur LF, un groupe par poche : `Σ 1/θ` et `Σ flux` (`quoi` = 0), ou `Σ d_c/θ` (`quoi` = 1).
var<workgroup> pk_red2: array<vec2<f32>, 256>;

fn lf_sum(t: u32, b: u32, quoi: u32) -> vec2<f32> {
    var s = vec2<f32>(0.0, 0.0);
    let n = atomicLoad(&pko[pk_h() + H_LF]);
    for (var e = t; e < n; e = e + 256u) {
        if lf_pocket(e) != b + 1u {
            continue;
        }
        let it = pkf[PK_LF_PAYLOAD + 2u * e];
        if quoi == 0u {
            s = s + vec2<f32>(it, pkf[PK_LF_PAYLOAD + 2u * e + 1u]);
        } else {
            s.x = s.x + it * cellf[cf(F_D, atomicLoad(&pko[lf_base() + 2u * e]))];
        }
    }
    pk_red2[t] = s;
    workgroupBarrier();
    for (var h = 128u; h > 0u; h = h / 2u) {
        if t < h {
            pk_red2[t] = pk_red2[t] + pk_red2[t + h];
        }
        workgroupBarrier();
    }
    return pk_red2[0];
}

// Les lignes des poches, et leur départ : `x = 0`, `r = rhs`, `z = d = r/diag`.
@compute @workgroup_size(256)
fn pk_rows(@builtin(local_invocation_id) l: vec3<u32>, @builtin(workgroup_id) w: vec3<u32>) {
    let t = l.x;
    let b = w.x;
    if t == 0u {
        pk_n = atomicLoad(&pko[pk_h() + H_COUNT]);
    }
    if b >= workgroupUniformLoad(&pk_n) {
        return;
    }
    let sums = lf_sum(t, b, 0u);
    if t == 0u {
        let vol = pkf[pf(PF_VOL, b)];
        let pres = pkf[pf(PF_P, b)];
        let s = P.rho * vol / (GAMMA_AIR * pres * P.dt * P.dt * P.dx);
        let scale = -P.rho * P.dx * P.dx / P.dt;
        let diag = s + sums.x;
        let rhs = s * (pres - P_ATM) + (scale / P.dx) * sums.y;
        pkf[pf(PF_DIAG, b)] = diag;
        pkf[pf(PF_RHS, b)] = rhs;
        pkf[pf(PF_X, b)] = 0.0;
        pkf[pf(PF_R, b)] = rhs;
        pkf[pf(PF_Z, b)] = rhs / diag;
        pkf[pf(PF_D, b)] = rhs / diag;
    }
}

// Les sommes du gradient conjugué (`reduce_pair`, `gather_pair`, `done_uniform` d'`apic3d_carte.wgsl`).
var<workgroup> red_a: array<f32, 256>;
var<workgroup> red_b: array<f32, 256>;
var<workgroup> wg_done: f32;

fn reduce_pair(l: u32, w: u32, a: f32, b: f32) {
    red_a[l] = a;
    red_b[l] = b;
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if l < s {
            red_a[l] = red_a[l] + red_a[l + s];
            red_b[l] = red_b[l] + red_b[l + s];
        }
        workgroupBarrier();
    }
    if l == 0u {
        partials[2u * w] = red_a[0];
        partials[2u * w + 1u] = red_b[0];
    }
}

fn gather_pair(l: u32) -> vec2<f32> {
    let parts = (P.cells + 255u) / 256u;
    var a = 0.0;
    var b = 0.0;
    for (var w = l; w < parts; w = w + 256u) {
        a = a + partials[2u * w];
        b = b + partials[2u * w + 1u];
    }
    red_a[l] = a;
    red_b[l] = b;
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if l < s {
            red_a[l] = red_a[l] + red_a[l + s];
            red_b[l] = red_b[l] + red_b[l + s];
        }
        workgroupBarrier();
    }
    return vec2<f32>(red_a[0], red_b[0]);
}

fn done_uniform(l: u32) -> bool {
    if l == 0u {
        wg_done = scalars[S_DONE];
    }
    return workgroupUniformLoad(&wg_done) != 0.0;
}

// `cg_init_finish`, poches comprises.
@compute @workgroup_size(256)
fn pk_cg_init_finish(@builtin(local_invocation_id) l: vec3<u32>) {
    let s = gather_pair(l.x);
    if l.x == 0u {
        var b2 = s.x;
        var rz = s.y;
        let n = atomicLoad(&pko[pk_h() + H_COUNT]);
        for (var b = 0u; b < n; b = b + 1u) {
            let r = pkf[pf(PF_R, b)];
            b2 = b2 + r * r;
            rz = rz + r * pkf[pf(PF_Z, b)];
        }
        scalars[S_B2] = b2;
        scalars[S_RZ] = rz;
        scalars[S_RR] = b2;
        scalars[S_IT] = 0.0;
        scalars[S_DONE] = select(0.0, 1.0, !(b2 > 0.0));
    }
}

// `cg_apply`, la poche voisine lue dans `d_b`.
@compute @workgroup_size(256)
fn pk_cg_apply(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
               @builtin(workgroup_id) w: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let c = g.x;
    var dq = 0.0;
    if c < P.cells {
        var s = 0.0;
        if label[c] == WATER {
            let dc = cellf[cf(F_D, c)];
            for (var m = 0u; m < 6u; m = m + 1u) {
                let n = nb_cell(c, m);
                if n == NONE {
                    continue;
                }
                let lb = label[n];
                if lb == WATER {
                    s = s + (dc - cellf[cf(F_D, n)]);
                } else if lb == AIR {
                    let b = of_at(n);
                    var db = 0.0;
                    if b != 0u {
                        db = pkf[pf(PF_D, b - 1u)];
                    }
                    s = s + (dc - db) / theta(c, n);
                }
            }
        }
        cellf[cf(F_Q, c)] = s;
        dq = cellf[cf(F_D, c)] * s;
    }
    reduce_pair(l.x, w.x, dq, 0.0);
}

// `A·d` des poches : `q_b = diag_b·d_b − Σ d_c/θ`.
@compute @workgroup_size(256)
fn pk_cg_rows(@builtin(local_invocation_id) l: vec3<u32>, @builtin(workgroup_id) w: vec3<u32>) {
    let t = l.x;
    let b = w.x;
    if t == 0u {
        pk_n = atomicLoad(&pko[pk_h() + H_COUNT]);
    }
    let n = workgroupUniformLoad(&pk_n);
    if done_uniform(t) || b >= n {
        return;
    }
    let sums = lf_sum(t, b, 1u);
    if t == 0u {
        pkf[pf(PF_Q, b)] = pkf[pf(PF_DIAG, b)] * pkf[pf(PF_D, b)] - sums.x;
    }
}

// `cg_alpha`, poches comprises ; puis la mise à jour des poches (`x`, `r`, `z`) et leurs sommes `r·z`, `r·r`.
@compute @workgroup_size(256)
fn pk_cg_alpha(@builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    if l.x == 0u {
        let n = atomicLoad(&pko[pk_h() + H_COUNT]);
        var dq = s.x;
        for (var b = 0u; b < n; b = b + 1u) {
            dq = dq + pkf[pf(PF_D, b)] * pkf[pf(PF_Q, b)];
        }
        scalars[S_DQ] = dq;
        if !(dq > 0.0) {
            scalars[S_DONE] = 1.0;
            return;
        }
        let alpha = scalars[S_RZ] / dq;
        scalars[S_ALPHA] = alpha;
        var rz = 0.0;
        var rr = 0.0;
        for (var b = 0u; b < n; b = b + 1u) {
            pkf[pf(PF_X, b)] = pkf[pf(PF_X, b)] + alpha * pkf[pf(PF_D, b)];
            let r = pkf[pf(PF_R, b)] - alpha * pkf[pf(PF_Q, b)];
            pkf[pf(PF_R, b)] = r;
            let z = r / pkf[pf(PF_DIAG, b)];
            pkf[pf(PF_Z, b)] = z;
            rz = rz + r * z;
            rr = rr + r * r;
        }
        pkf[pf(PF_SCAL, 1u)] = rz;
        pkf[pf(PF_SCAL, 2u)] = rr;
    }
}

// `cg_beta`, poches comprises ; puis la direction des poches.
@compute @workgroup_size(256)
fn pk_cg_beta(@builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    if l.x == 0u {
        let rz = s.x + pkf[pf(PF_SCAL, 1u)];
        let rr = s.y + pkf[pf(PF_SCAL, 2u)];
        let beta = rz / scalars[S_RZ];
        scalars[S_BETA] = beta;
        scalars[S_RZ] = rz;
        scalars[S_RR] = rr;
        let it = scalars[S_IT] + 1.0;
        scalars[S_IT] = it;
        if rr <= P.tol2 * scalars[S_B2] || it >= f32(P.max_it) {
            scalars[S_DONE] = 1.0;
        }
        let n = atomicLoad(&pko[pk_h() + H_COUNT]);
        for (var b = 0u; b < n; b = b + 1u) {
            pkf[pf(PF_D, b)] = pkf[pf(PF_Z, b)] + beta * pkf[pf(PF_D, b)];
        }
    }
}

// `correct`, la pression de la poche du côté de l'air (zéro pour l'air libre).
fn on_wall_pk(axis: u32, idx: vec3<u32>) -> bool {
    if axis == 0u {
        return idx.x == 0u || idx.x == P.nx;
    }
    if axis == 1u {
        return idx.y == 0u || idx.y == P.ny;
    }
    return idx.z == 0u || idx.z == P.nz;
}

fn air_pressure(a: u32) -> f32 {
    let b = of_at(a);
    if b == 0u {
        return 0.0;
    }
    return pkf[pf(PF_X, b - 1u)];
}

@compute @workgroup_size(128)
fn pk_correct(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    // La face `f` : son axe et son indice (`face_of` d'`apic3d_carte.wgsl`).
    var axis = 0u;
    var local = f;
    var dims = vec3<u32>(P.nx + 1u, P.ny, P.nz);
    if f >= P.nu + P.nv {
        axis = 2u;
        local = f - P.nu - P.nv;
        dims = vec3<u32>(P.nx, P.ny, P.nz + 1u);
    } else if f >= P.nu {
        axis = 1u;
        local = f - P.nu;
        dims = vec3<u32>(P.nx, P.ny + 1u, P.nz);
    }
    let idx = vec3<u32>(local % dims.x, (local / dims.x) % dims.y, local / (dims.x * dims.y));
    if on_wall_pk(axis, idx) {
        return;
    }
    let along = vec3<u32>(select(0u, 1u, axis == 0u), select(0u, 1u, axis == 1u), select(0u, 1u, axis == 2u));
    let cm = idx - along;
    let c = (cm.z * P.ny + cm.y) * P.nx + cm.x;
    let n = (idx.z * P.ny + idx.y) * P.nx + idx.x;
    if label[c] == SOLID || label[n] == SOLID {
        return;
    }
    let wc = label[c] == WATER;
    let wn = label[n] == WATER;
    var grad = 0.0;
    if wc && wn {
        grad = cellf[cf(F_P, n)] - cellf[cf(F_P, c)];
    } else if wc {
        grad = (air_pressure(n) - cellf[cf(F_P, c)]) / theta(c, n);
    } else if wn {
        grad = (cellf[cf(F_P, n)] - air_pressure(c)) / theta(n, c);
    } else {
        return;
    }
    faces[f] = faces[f] - P.dt / (P.rho * P.dx) * grad;
}

// Au bout du pas : la variation de volume que la loi linéarisée prévoit, la pression, le volume suivi, le dernier pas.
@compute @workgroup_size(1)
fn pk_post() {
    let n = atomicLoad(&pko[pk_h() + H_COUNT]);
    for (var b = 0u; b < n; b = b + 1u) {
        let vol = pkf[pf(PF_VOL, b)];
        let pres = pkf[pf(PF_P, b)];
        let x = pkf[pf(PF_X, b)];
        let dv = -(x - (pres - P_ATM)) * vol / (GAMMA_AIR * pres);
        pkf[pf(PF_DV, b)] = dv;
        pkf[pf(PF_P, b)] = P_ATM + x;
        pkf[pf(PF_VFLUX, b)] = vol + dv;
    }
    pkf[pf(PF_SCAL, 0u)] = P.dt;
}

// =====================================================================================================================
// **S482 — les poches dans la multigrille** (K2-2b). Le préconditionneur par blocs : le cycle en V d'`apic3d_carte.wgsl` sur les
// mailles (l'air des poches y reste une condition nulle, comme l'air libre), la diagonale sur les poches. `M` reste symétrique défini
// positif, le gradient conjugué reste valide ; il ne voit pas le couplage maille | poche, que l'opérateur, lui, porte (`pk_cg_apply`,
// `pk_cg_rows`). Les noyaux fusionnés de S424 (`mg_cg_update_alpha`, `mg_cg_beta_direction`) ont ici leur variante : chaque groupe
// replie les mêmes produits — mailles, puis poches dans l'ordre des poches — et calcule le même `α`, le même `β`, au bit ; le
// groupe 0 met à jour les poches. `r·z` en double tampon selon la parité (`CG_PAR`), comme les originaux.

override CG_PAR: u32 = 0u;
const S_RZ2: u32 = 8u;
const MG_OMEGA: f32 = 0.85714287;

fn rz_cur() -> u32 {
    return select(S_RZ, S_RZ2, CG_PAR == 1u);
}

fn rz_next() -> u32 {
    return select(S_RZ2, S_RZ, CG_PAR == 1u);
}

fn pocket_count() -> u32 {
    return atomicLoad(&pko[pk_h() + H_COUNT]);
}

// Le départ (`mg_cg_init_finish`), poches comprises : `r·z` et `b·b` des mailles (le dernier noyau du cycle) plus ceux des poches.
@compute @workgroup_size(256)
fn pk_mg_init_finish(@builtin(local_invocation_id) l: vec3<u32>) {
    let s = gather_pair(l.x);
    if l.x == 0u {
        var rz = s.x;
        var b2 = s.y;
        let n = pocket_count();
        for (var b = 0u; b < n; b = b + 1u) {
            let r = pkf[pf(PF_R, b)];
            rz = rz + r * pkf[pf(PF_Z, b)];
            b2 = b2 + r * r;
        }
        scalars[S_B2] = b2;
        scalars[S_RZ] = rz;
        scalars[S_RR] = b2;
        scalars[S_IT] = 0.0;
        scalars[S_DONE] = select(0.0, 1.0, !(b2 > 0.0));
    }
}

// `α = (r·z)/(d·q)`, poches comprises ; `p += α·d`, `r −= α·q`, `q = ω·r/diag` sur les mailles ; au groupe 0, les poches.
@compute @workgroup_size(256)
fn pk_mg_update_alpha(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    let n = pocket_count();
    var dq = s.x;
    for (var b = 0u; b < n; b = b + 1u) {
        dq = dq + pkf[pf(PF_D, b)] * pkf[pf(PF_Q, b)];
    }
    if !(dq > 0.0) {
        if g.x == 0u {
            scalars[S_DQ] = dq;
            scalars[S_DONE] = 1.0;
        }
        return;
    }
    let alpha = scalars[rz_cur()] / dq;
    if g.x == 0u {
        scalars[S_DQ] = dq;
        scalars[S_ALPHA] = alpha;
        var rz = 0.0;
        var rr = 0.0;
        for (var b = 0u; b < n; b = b + 1u) {
            pkf[pf(PF_X, b)] = pkf[pf(PF_X, b)] + alpha * pkf[pf(PF_D, b)];
            let r = pkf[pf(PF_R, b)] - alpha * pkf[pf(PF_Q, b)];
            pkf[pf(PF_R, b)] = r;
            let z = r / pkf[pf(PF_DIAG, b)];
            pkf[pf(PF_Z, b)] = z;
            rz = rz + r * z;
            rr = rr + r * r;
        }
        pkf[pf(PF_SCAL, 1u)] = rz;
        pkf[pf(PF_SCAL, 2u)] = rr;
    }
    let c = g.x;
    if c < P.cells {
        cellf[cf(F_P, c)] = cellf[cf(F_P, c)] + alpha * cellf[cf(F_D, c)];
        let r = cellf[cf(F_R, c)] - alpha * cellf[cf(F_Q, c)];
        cellf[cf(F_R, c)] = r;
        let diag = cellf[cf(F_DIAG, c)];
        if label[c] == WATER && diag > 0.0 {
            cellf[cf(F_Q, c)] = MG_OMEGA * r / diag;
        } else {
            cellf[cf(F_Q, c)] = 0.0;
        }
    }
}

// `β`, l'arrêt (au groupe 0), `d = z + β·d` sur les mailles ; au groupe 0, la direction des poches.
@compute @workgroup_size(256)
fn pk_mg_beta_direction(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    let rz = s.x + pkf[pf(PF_SCAL, 1u)];
    let rr = s.y + pkf[pf(PF_SCAL, 2u)];
    let beta = rz / scalars[rz_cur()];
    if g.x == 0u {
        scalars[S_BETA] = beta;
        scalars[rz_next()] = rz;
        scalars[S_RR] = rr;
        let it = scalars[S_IT] + 1.0;
        scalars[S_IT] = it;
        if rr <= P.tol2 * scalars[S_B2] || it >= f32(P.max_it) {
            scalars[S_DONE] = 1.0;
        }
        let n = pocket_count();
        for (var b = 0u; b < n; b = b + 1u) {
            pkf[pf(PF_D, b)] = pkf[pf(PF_Z, b)] + beta * pkf[pf(PF_D, b)];
        }
    }
    let c = g.x;
    if c < P.cells {
        cellf[cf(F_D, c)] = cellf[cf(F_Z, c)] + beta * cellf[cf(F_D, c)];
    }
}

// =====================================================================================================================
// **S482 — les compactions à plusieurs groupes** (K2-2b). `pk_number`, `pk_lists` et `pk_faces` parcouraient toutes les mailles avec
// un seul groupe (≈ 2 ms à 60 000 mailles, ≈ 10 à 371 000). Ici, la même compaction en trois passes — le compte de chaque bloc de 256
// mailles, le préfixe des blocs (un groupe, sur `cells/256` entrées), l'écriture (le préfixe dans le bloc) — **dans le même ordre des
// mailles**, donc les mêmes listes, au mot près. Quatre sortes (`KIND`, constante de pipeline) : 0 les racines enfermées (leur rang),
// 1 les mailles d'air des poches (LA), 2 les mailles d'eau qui les bordent (LW), 3 les faces eau | poche (LF).

override KIND: u32 = 0u;
const H_TOT: u32 = 100u; // totaux des quatre sortes

fn nblocks() -> u32 {
    return (P.cells + 255u) / 256u;
}

fn blk_base() -> u32 {
    return lf_base() + 4u * P.cells;
}

// Les entrées que la maille `c` apporte à la sorte `KIND`.
fn cell_count(c: u32) -> u32 {
    // Des `if` et un seul retour : FXC refuse un `switch` dont les branches retournent.
    var n = 0u;
    if KIND == 0u {
        n = select(0u, 1u, is_root(c));
    } else if KIND == 1u {
        n = select(0u, 1u, of_at(c) != 0u);
    } else if KIND == 2u {
        n = bord_n(c);
    } else if label[c] == WATER {
        for (var m = 0u; m < 6u; m = m + 1u) {
            if pocket_nb(c, m) != 0u {
                n = n + 1u;
            }
        }
    }
    return n;
}

// Les écrit à partir du rang `at` ; rend vrai si une liste a débordé.
fn cell_write(c: u32, at0: u32) -> bool {
    var at = at0;
    var ovf = false;
    switch KIND {
        case 0u: {
            if is_root(c) {
                atomicStore(&pko[3u * P.cells + c], select(NONE, at, at < MAXP));
            }
        }
        case 1u: {
            let b = of_at(c);
            if b != 0u {
                atomicStore(&pko[la_base() + 2u * at], c);
                atomicStore(&pko[la_base() + 2u * at + 1u], b);
            }
        }
        case 2u: {
            if label[c] == WATER {
                for (var m = 0u; m < 6u; m = m + 1u) {
                    if !bord_new(c, m) {
                        continue;
                    }
                    if at < P.cells {
                        atomicStore(&pko[lw_base() + 2u * at], c);
                        atomicStore(&pko[lw_base() + 2u * at + 1u], bord_b(c, m));
                    } else {
                        ovf = true;
                    }
                    at = at + 1u;
                }
            }
        }
        default: {
            if label[c] == WATER {
                for (var m = 0u; m < 6u; m = m + 1u) {
                    let b = pocket_nb(c, m);
                    if b == 0u {
                        continue;
                    }
                    if at < P.cells {
                        atomicStore(&pko[lf_base() + 2u * at], c);
                        atomicStore(&pko[lf_base() + 2u * at + 1u], b + 8u * m);
                        let sign = select(1.0, -1.0, m % 2u == 0u);
                        pkf[PK_LF_PAYLOAD + 2u * at] = 1.0 / theta(c, nb_cell(c, m));
                        pkf[PK_LF_PAYLOAD + 2u * at + 1u] = -sign * faces[face_of_nb(c, m)];
                    } else {
                        ovf = true;
                    }
                    at = at + 1u;
                }
            }
        }
    }
    return ovf;
}

// Rien à compacter : rien d'enfermé (sortes 0 à 2), aucune poche gardée (sorte 3). Uniforme dans le groupe.
fn blk_skip(t: u32) -> bool {
    if t == 0u {
        if KIND == 3u {
            pk_any = atomicLoad(&pko[pk_h() + H_COUNT]);
        } else {
            pk_any = atomicLoad(&pko[pk_h() + H_ANY]);
        }
    }
    return workgroupUniformLoad(&pk_any) == 0u;
}

// 1. Le compte de chaque bloc.
@compute @workgroup_size(256)
fn pk_blk_count(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                @builtin(workgroup_id) w: vec3<u32>) {
    let t = l.x;
    if blk_skip(t) {
        return;
    }
    var n = 0u;
    if g.x < P.cells {
        n = cell_count(g.x);
    }
    pk_scan[t] = n;
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if t < s {
            pk_scan[t] = pk_scan[t] + pk_scan[t + s];
        }
        workgroupBarrier();
    }
    if t == 0u {
        atomicStore(&pko[blk_base() + KIND * nblocks() + w.x], pk_scan[0]);
    }
}

// 2. Le préfixe exclusif des blocs, par un groupe (chaque fil une tranche contiguë) ; le total, et ce qu'il commande.
@compute @workgroup_size(256)
fn pk_blk_scan(@builtin(local_invocation_id) l: vec3<u32>) {
    let t = l.x;
    let h = pk_h();
    if blk_skip(t) {
        if t == 0u {
            atomicStore(&pko[h + H_TOT + KIND], 0u);
            switch KIND {
                case 0u: {
                    atomicStore(&pko[h + H_ROOTS], 0u);
                    atomicStore(&pko[h + H_COUNT], 0u);
                }
                case 1u: { atomicStore(&pko[h + H_LA], 0u); }
                case 2u: { atomicStore(&pko[h + H_LW], 0u); }
                default: { atomicStore(&pko[h + H_LF], 0u); }
            }
        }
        return;
    }
    let nb = nblocks();
    let base = blk_base() + KIND * nb;
    let chunk = (nb + 255u) / 256u;
    let lo = min(t * chunk, nb);
    let hi = min(lo + chunk, nb);
    var n = 0u;
    for (var k = lo; k < hi; k = k + 1u) {
        n = n + atomicLoad(&pko[base + k]);
    }
    pk_scan[t] = n;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var v = 0u;
        if t >= s {
            v = pk_scan[t - s];
        }
        workgroupBarrier();
        pk_scan[t] = pk_scan[t] + v;
        workgroupBarrier();
    }
    var at = pk_scan[t] - n;
    for (var k = lo; k < hi; k = k + 1u) {
        let m = atomicLoad(&pko[base + k]);
        atomicStore(&pko[base + k], at);
        at = at + m;
    }
    if t == 255u {
        let total = pk_scan[255];
        atomicStore(&pko[h + H_TOT + KIND], total);
        switch KIND {
            case 0u: {
                atomicStore(&pko[h + H_ROOTS], total);
                atomicStore(&pko[h + H_COUNT], min(total, MAXP));
                if total > MAXP {
                    atomicAdd(&pko[h + H_OVERFLOW], total - MAXP);
                }
            }
            case 1u: { atomicStore(&pko[h + H_LA], total); }
            case 2u: {
                atomicStore(&pko[h + H_LW], min(total, P.cells));
                if total > P.cells {
                    atomicStore(&pko[h + H_LIST_OVF], 1u);
                }
            }
            default: {
                atomicStore(&pko[h + H_LF], min(total, P.cells));
                if total > P.cells {
                    atomicStore(&pko[h + H_LIST_OVF], 1u);
                }
            }
        }
    }
}

// 3. L'écriture : le préfixe dans le bloc, plus celui du bloc.
@compute @workgroup_size(256)
fn pk_blk_write(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                @builtin(workgroup_id) w: vec3<u32>) {
    let t = l.x;
    if blk_skip(t) {
        return;
    }
    var n = 0u;
    if g.x < P.cells {
        n = cell_count(g.x);
    }
    pk_scan[t] = n;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var v = 0u;
        if t >= s {
            v = pk_scan[t - s];
        }
        workgroupBarrier();
        pk_scan[t] = pk_scan[t] + v;
        workgroupBarrier();
    }
    if g.x < P.cells && n > 0u {
        let at = atomicLoad(&pko[blk_base() + KIND * nblocks() + w.x]) + pk_scan[t] - n;
        if cell_write(g.x, at) {
            atomicStore(&pko[pk_h() + H_LIST_OVF], 1u);
        }
    }
}
