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
