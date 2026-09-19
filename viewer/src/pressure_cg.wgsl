// S289 / ADR-172 — gradient conjugué **résident** : opérateur, réductions et scalaires restent
// sur le GPU, aucun retour CPU entre itérations. Six champs par maille vivent dans un seul
// tampon, par tranches de `cells` : pression, résidu, résidu préconditionné, direction,
// `A·d`, et l'inverse de la diagonale. L'opérateur est celui d'ADR-172, figé, exporté par le
// cœur ; rien ici n'est un état publié ni une sauvegarde de δ.
struct Row { weights: vec4<f32>, ghosts: vec4<f32> }
struct Params { nx: u32, cells: u32, inv: f32, groups: u32 }
@group(0) @binding(0) var<storage, read> rows: array<Row>;
@group(0) @binding(1) var<storage, read> rhs: array<f32>;
@group(0) @binding(2) var<storage, read_write> state: array<f32>;
@group(0) @binding(3) var<storage, read_write> partial: array<f32>;
@group(0) @binding(4) var<storage, read_write> scalar: array<f32>;
@group(0) @binding(5) var<uniform> params: Params;

const P: u32 = 0u;   // pression
const R: u32 = 1u;   // résidu
const Z: u32 = 2u;   // M⁻¹r
const D: u32 = 3u;   // direction
const Q: u32 = 4u;   // A·d
const M: u32 = 5u;   // inverse de la diagonale

const RZ: u32 = 0u;
const DQ: u32 = 1u;
const ALPHA: u32 = 2u;
const BETA: u32 = 3u;

const HUGE: f32 = 3.0e38;

var<workgroup> scratch: array<f32, 64>;

fn at(section: u32, c: u32) -> u32 { return section * params.cells + c; }

/// `(A·x)_c` et la diagonale de la ligne, pour la tranche `section`. Même parcours de faces
/// que `apply_mobile` du cœur : fantôme de surface quand `ghost > 0`, différence sinon.
fn stencil(section: u32, c: u32) -> vec2<f32> {
    var acc = 0.0;
    var diag = 0.0;
    let base = section * params.cells;
    for (var f = 0u; f < 4u; f++) {
        let a = rows[c].weights[f];
        let g = rows[c].ghosts[f];
        if a == 0.0 { continue; }
        if g > 0.0 {
            acc += a * state[base + c] * g;
            diag += a * g;
        } else {
            var j = c;
            switch f {
                case 0u: { j = c - 1u; }
                case 1u: { j = c + 1u; }
                case 2u: { j = c - params.nx; }
                default: { j = c + params.nx; }
            }
            acc += a * (state[base + c] - state[base + j]);
            diag += a;
        }
    }
    return vec2<f32>(acc * params.inv, diag * params.inv);
}

/// Somme d'arbre dans le groupe, puis une valeur par groupe. Ordre fixé par la structure :
/// reproductible sur une même carte, sans prétention d'identité inter-GPU (ADR-172).
fn fold(value: f32, lid: u32, group: u32) { fold_at(0u, value, lid, group); }

fn fold_at(base: u32, value: f32, lid: u32, group: u32) {
    scratch[lid] = value;
    workgroupBarrier();
    for (var s = 32u; s > 0u; s >>= 1u) {
        if lid < s { scratch[lid] += scratch[lid + s]; }
        workgroupBarrier();
    }
    if lid == 0u { partial[base * params.groups + group] = scratch[0]; }
    workgroupBarrier();
}

fn product(a: u32, b: u32, c: u32) -> f32 {
    if c >= params.cells { return 0.0; }
    return state[at(a, c)] * state[at(b, c)];
}

/// Somme des valeurs par groupe, dans un seul groupe. `groups` ≤ 65 536 par construction.
fn gather(lid: u32) -> f32 { return gather_at(0u, lid); }

/// Même somme, sur la tranche de `partial` qui commence à `base * groups` (S290).
fn gather_at(base: u32, lid: u32) -> f32 {
    var v = 0.0;
    var i = lid;
    let origin = base * params.groups;
    loop {
        if i >= params.groups { break; }
        v += partial[origin + i];
        i += 64u;
    }
    scratch[lid] = v;
    workgroupBarrier();
    for (var s = 32u; s > 0u; s >>= 1u) {
        if lid < s { scratch[lid] += scratch[lid + s]; }
        workgroupBarrier();
    }
    // S290 : `scratch` sert à plusieurs phases dans un même noyau. Sans cette barrière, un fil
    // pourrait l'écraser avant qu'un autre n'ait lu le résultat.
    let out = scratch[0];
    workgroupBarrier();
    return out;
}

/// Amorçage : diagonale, vrai résidu `b − A·p` du départ fourni, direction préconditionnée.
/// Une ligne vide (maille d'air ou solide) rend zéro partout, comme le cœur.
@compute @workgroup_size(64)
fn init(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if c >= params.cells { return; }
    let op = stencil(P, c);
    var m = 0.0;
    if op.y > 0.0 { m = 1.0 / op.y; }
    state[at(M, c)] = m;
    let r = rhs[c] - op.x;
    state[at(R, c)] = r;
    let z = m * r;
    state[at(Z, c)] = z;
    state[at(D, c)] = z;
}

@compute @workgroup_size(64)
fn apply_dir(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if c >= params.cells { return; }
    state[at(Q, c)] = stencil(D, c).x;
}

@compute @workgroup_size(64)
fn reduce_rz(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    fold(product(R, Z, id.x), lid, wid.x);
}

@compute @workgroup_size(64)
fn reduce_dq(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    fold(product(D, Q, id.x), lid, wid.x);
}

@compute @workgroup_size(64)
fn reduce_rr(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    fold(product(R, R, id.x), lid, wid.x);
}

/// `α = ⟨r,z⟩/⟨d,q⟩`. Un dénominateur nul, négatif ou non fini donne `α = 0` : le cycle
/// n'avance plus, et c'est le cœur qui constate — jamais le GPU qui publie.
@compute @workgroup_size(64)
fn finish_dq(@builtin(local_invocation_index) lid: u32) {
    let dq = gather(lid);
    if lid == 0u {
        scalar[DQ] = dq;
        let rz = scalar[RZ];
        var a = 0.0;
        if dq > 0.0 && dq < HUGE && rz > 0.0 && rz < HUGE { a = rz / dq; }
        scalar[ALPHA] = a;
    }
}

/// `β = ⟨r,z⟩_{n+1}/⟨r,z⟩_n`, puis `⟨r,z⟩` est remplacé. Au premier appel `⟨r,z⟩_n = 0`
/// et `β = 0` : la direction reste celle de l'amorçage.
@compute @workgroup_size(64)
fn finish_rz(@builtin(local_invocation_index) lid: u32) {
    let rzn = gather(lid);
    if lid == 0u {
        let rz = scalar[RZ];
        var b = 0.0;
        if rz > 0.0 && rz < HUGE && rzn >= 0.0 && rzn < HUGE { b = rzn / rz; }
        scalar[BETA] = b;
        scalar[RZ] = rzn;
    }
}

/// `‖r‖²` du cycle, pour le diagnostic de l'hôte. N'ouvre aucune porte d'acceptation.
@compute @workgroup_size(64)
fn finish_rr(@builtin(local_invocation_index) lid: u32) {
    let rr = gather(lid);
    if lid == 0u { scalar[DQ] = rr; }
}

@compute @workgroup_size(64)
fn update_pr(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if c >= params.cells { return; }
    let a = scalar[ALPHA];
    let p = state[at(P, c)] + a * state[at(D, c)];
    let r = state[at(R, c)] - a * state[at(Q, c)];
    state[at(P, c)] = p;
    state[at(R, c)] = r;
    state[at(Z, c)] = state[at(M, c)] * r;
}

@compute @workgroup_size(64)
fn update_dir(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if c >= params.cells { return; }
    state[at(D, c)] = state[at(Z, c)] + scalar[BETA] * state[at(D, c)];
}

// ─── S290 : noyaux fusionnés — chaque réduction est repliée dans le noyau qui produit ses
// valeurs. La valeur repliée est celle que le même fil vient de calculer et d'écrire, donc
// **la mathématique est inchangée** et le découpage des groupes aussi : l'égalité au bit avec
// les noyaux séparés est exigible, et le banc la vérifie. Le repli se fait hors de toute
// sortie anticipée, parce que `fold` porte des barrières de groupe.

@compute @workgroup_size(64)
fn init_fold(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let op = stencil(P, c);
        var m = 0.0;
        if op.y > 0.0 { m = 1.0 / op.y; }
        state[at(M, c)] = m;
        let r = rhs[c] - op.x;
        state[at(R, c)] = r;
        let z = m * r;
        state[at(Z, c)] = z;
        state[at(D, c)] = z;
        v = r * z;
    }
    fold(v, lid, wid.x);
}

@compute @workgroup_size(64)
fn apply_fold(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let q = stencil(D, c).x;
        state[at(Q, c)] = q;
        v = state[at(D, c)] * q;
    }
    fold(v, lid, wid.x);
}

@compute @workgroup_size(64)
fn update_fold(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let a = scalar[ALPHA];
        let p = state[at(P, c)] + a * state[at(D, c)];
        let r = state[at(R, c)] - a * state[at(Q, c)];
        state[at(P, c)] = p;
        state[at(R, c)] = r;
        let z = state[at(M, c)] * r;
        state[at(Z, c)] = z;
        v = r * z;
    }
    fold(v, lid, wid.x);
}

// ─── S290, chemin « scalaire recalculé » : 3 dispatchs par itération ───────────────────────
//
// Les deux dispatchs à un seul groupe de travail — ceux qui sommaient les valeurs par groupe
// pour produire `α` puis `β` — coûtaient exactement le même enregistrement que n'importe quel
// autre dispatch, 1,86 µs, pour un travail négligeable. Ici **chaque groupe refait la somme
// lui-même**, au début du noyau qui a besoin du scalaire. Les valeurs sommées ont été écrites
// par le **dispatch précédent** : leur visibilité est celle d'une frontière de dispatch, que
// WebGPU garantit. Aucune atomique, aucune synchronisation inter-groupes, aucun pari sur la
// spécification — et le scalaire n'est plus stocké, donc plus écrit en concurrence.
//
// La somme est l'**même** arbre sur les **mêmes** valeurs : l'égalité au bit avec le chemin
// précédent est exigible, et le banc la vérifie.
//
// `partial` porte trois tranches de `groups` valeurs : 0 pour ⟨d,q⟩, 1 et 2 pour ⟨r,z⟩ en
// alternance — la tranche ancienne doit survivre pendant que la nouvelle s'écrit.
const PART_DQ: u32 = 0u;
const PART_RZ0: u32 = 1u;
const PART_RZ1: u32 = 2u;

fn guard(num: f32, den: f32) -> f32 {
    if den > 0.0 && den < HUGE && num > 0.0 && num < HUGE { return num / den; }
    return 0.0;
}

/// `p += α d ; r -= α q ; z = M⁻¹r`, puis ⟨r,z⟩ dans la tranche neuve. `α` est refait ici.
fn advance(old: u32, fresh: u32, id: vec3<u32>, lid: u32, group: u32) {
    let dq = gather_at(PART_DQ, lid);
    let rz = gather_at(old, lid);
    let a = guard(rz, dq);
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let p = state[at(P, c)] + a * state[at(D, c)];
        let r = state[at(R, c)] - a * state[at(Q, c)];
        state[at(P, c)] = p;
        state[at(R, c)] = r;
        let z = state[at(M, c)] * r;
        state[at(Z, c)] = z;
        v = r * z;
    }
    fold_at(fresh, v, lid, group);
}

/// `d = z + β d`, `β` refait depuis les deux tranches de ⟨r,z⟩.
fn direct(old: u32, fresh: u32, id: vec3<u32>, lid: u32) {
    let rzn = gather_at(fresh, lid);
    let rz = gather_at(old, lid);
    let b = guard(rzn, rz);
    let c = id.x;
    if c < params.cells { state[at(D, c)] = state[at(Z, c)] + b * state[at(D, c)]; }
}

@compute @workgroup_size(64)
fn advance_even(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    advance(PART_RZ0, PART_RZ1, id, lid, wid.x);
}
@compute @workgroup_size(64)
fn advance_odd(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    advance(PART_RZ1, PART_RZ0, id, lid, wid.x);
}
@compute @workgroup_size(64)
fn direct_even(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32) {
    direct(PART_RZ0, PART_RZ1, id, lid);
}
@compute @workgroup_size(64)
fn direct_odd(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32) {
    direct(PART_RZ1, PART_RZ0, id, lid);
}

/// Amorçage de ce chemin : ⟨r,z⟩₀ va dans la tranche 1, que la première itération lit.
@compute @workgroup_size(64)
fn init_fold0(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let op = stencil(P, c);
        var m = 0.0;
        if op.y > 0.0 { m = 1.0 / op.y; }
        state[at(M, c)] = m;
        let r = rhs[c] - op.x;
        state[at(R, c)] = r;
        let z = m * r;
        state[at(Z, c)] = z;
        state[at(D, c)] = z;
        v = r * z;
    }
    fold_at(PART_RZ0, v, lid, wid.x);
}

/// `q = A·d` avec ⟨d,q⟩ dans la tranche 0. Identique à `apply_fold`, autre tranche.
@compute @workgroup_size(64)
fn apply_fold0(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells {
        let q = stencil(D, c).x;
        state[at(Q, c)] = q;
        v = state[at(D, c)] * q;
    }
    fold_at(PART_DQ, v, lid, wid.x);
}

/// Diagnostics finaux en un seul dispatch : `‖r‖²` replié puis sommé par le premier groupe,
/// et ⟨r,z⟩ relu de la tranche vive. Un seul groupe écrit, donc pas de concurrence.
fn close(fresh: u32, id: vec3<u32>, lid: u32, group: u32) {
    var v = 0.0;
    let c = id.x;
    if c < params.cells { v = state[at(R, c)] * state[at(R, c)]; }
    fold_at(PART_DQ, v, lid, group);
    let rz = gather_at(fresh, lid);
    if group == 0u && lid == 0u { scalar[RZ] = rz; }
}
@compute @workgroup_size(64)
fn close_even(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    close(PART_RZ0, id, lid, wid.x);
}
@compute @workgroup_size(64)
fn close_odd(@builtin(global_invocation_id) id: vec3<u32>,
    @builtin(local_invocation_index) lid: u32, @builtin(workgroup_id) wid: vec3<u32>) {
    close(PART_RZ1, id, lid, wid.x);
}
/// Somme des `‖r‖²` par groupe : un seul groupe, une seule fois par appel.
@compute @workgroup_size(64)
fn close_sum(@builtin(local_invocation_index) lid: u32) {
    let rr = gather_at(PART_DQ, lid);
    if lid == 0u { scalar[DQ] = rr; }
}
