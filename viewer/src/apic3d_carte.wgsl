// S416 — C7a : le pas d'APIC 3D **nu** sur la carte (docs/validation/APIC-CARTE-S416.md).
// La référence est `water_core::apic3d` ; chaque noyau renvoie à la fonction qu'il reproduit. Même disposition que la
// référence : mailles `(k·ny + j)·nx + i`, faces `u`, `v`, `w` à la suite dans un seul tampon. Aucun atomique flottant :
// les transferts sont des **collectes** sur les particules triées par maille.

struct Params {
    nx: u32, ny: u32, nz: u32, cells: u32,
    n: u32, nu: u32, nv: u32, nw: u32,
    faces: u32, nblocks: u32, reach: u32, max_it: u32,
    dx: f32, radius: f32, kr: f32, dt: f32,
    gdt: f32, rho: f32, theta_min: f32, dmin: f32,
    margin: f32, lx: f32, ly: f32, lz: f32,
    tol2: f32, has_body: f32, br: f32, p2: f32,
    // S417 : le corps cinématique — centre au début du pas, vitesse, centre avancé (`move_body`).
    bcx: f32, bcy: f32, bcz: f32, bvx: f32,
    bvy: f32, bvz: f32, bmx: f32, bmy: f32,
    bmz: f32, has_columns: f32, band: f32, q2: f32,
}

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var<storage, read_write> px: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> pv: array<vec4<f32>>;
// C par particule : trois lignes `C[axe]`, à la suite.
@group(0) @binding(3) var<storage, read_write> pc: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> shift: array<vec4<f32>>;
@group(0) @binding(5) var<storage, read_write> count: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> start: array<u32>;
@group(0) @binding(7) var<storage, read_write> order: array<u32>;
@group(0) @binding(8) var<storage, read_write> blocks: array<u32>;
// Faces : vitesse [0, F), poids [F, 2F), copie [2F, 3F).
@group(0) @binding(9) var<storage, read_write> faces: array<f32>;
// Drapeaux : valide [0, F), copie [F, 2F).
@group(0) @binding(10) var<storage, read_write> fflags: array<u32>;
// Mailles : φ, p, rhs, r, z, d, q, diag — huit champs de `cells`.
@group(0) @binding(11) var<storage, read_write> cellf: array<f32>;
@group(0) @binding(12) var<storage, read_write> label: array<u32>;
@group(0) @binding(13) var<storage, read_write> partials: array<f32>;
// Scalaires du gradient conjugué : b2, rz, rr, dq, alpha, beta, fini, itérations.
@group(0) @binding(14) var<storage, read_write> scalars: array<f32>;
// S417 — la zone des colonnes (C7c) : `η` [0, C), son reste [C, 2C), la table de lecture de S400 [2C, 2C + 32), puis les
// débits en double flottant (C7c-1, P7).
@group(0) @binding(15) var<storage, read_write> cols: array<f32>;
@group(0) @binding(16) var<storage, read_write> cmask: array<u32>;

const AIR: u32 = 0u;
const WATER: u32 = 1u;
const SOLID: u32 = 2u;
const WG: u32 = 128u;
const SCAN: u32 = 256u;

fn cell_index(i: u32, j: u32, k: u32) -> u32 {
    return (k * P.ny + j) * P.nx + i;
}

// `Apic3::cell_of` : la maille qui contient un point, bornée au domaine.
fn cell_of(p: vec3<f32>) -> vec3<u32> {
    let f = vec3<u32>(max(p / P.dx, vec3<f32>(0.0)));
    return min(f, vec3<u32>(P.nx - 1u, P.ny - 1u, P.nz - 1u));
}

// ---------------------------------------------------------------------------------------------------------------------
// Tri par maille (`Apic3::bin`) : compte, préfixe exclusif, rangement, puis chaque maille trie sa tranche par indice —
// l'ordre de la référence, et un pas déterministe malgré l'ordre des atomiques.

@compute @workgroup_size(128)
fn bin_clear(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c < P.cells {
        atomicStore(&count[c], 0u);
    }
}

@compute @workgroup_size(128)
fn bin_count(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n {
        return;
    }
    let m = cell_of(px[k].xyz);
    atomicAdd(&count[cell_index(m.x, m.y, m.z)], 1u);
}

var<workgroup> scan_mem: array<u32, 256>;

// Préfixe exclusif dans chaque groupe de 256 mailles ; la somme du groupe va à `blocks`.
@compute @workgroup_size(256)
fn scan_local(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
              @builtin(workgroup_id) w: vec3<u32>) {
    let c = g.x;
    var own = 0u;
    if c < P.cells {
        own = atomicLoad(&count[c]);
    }
    scan_mem[l.x] = own;
    workgroupBarrier();
    for (var s = 1u; s < SCAN; s = s * 2u) {
        var add = 0u;
        if l.x >= s {
            add = scan_mem[l.x - s];
        }
        workgroupBarrier();
        scan_mem[l.x] = scan_mem[l.x] + add;
        workgroupBarrier();
    }
    if c < P.cells {
        start[c] = scan_mem[l.x] - own;
    }
    if l.x == SCAN - 1u {
        blocks[w.x] = scan_mem[l.x];
    }
}

// Préfixe exclusif des sommes de groupe, par un seul fil : `nblocks` est petit (une maille sur 256).
@compute @workgroup_size(1)
fn scan_blocks() {
    var s = 0u;
    for (var b = 0u; b < P.nblocks; b = b + 1u) {
        let v = blocks[b];
        blocks[b] = s;
        s = s + v;
    }
    start[P.cells] = s;
}

// Ajoute le préfixe du groupe ; remet le compte à zéro pour qu'il serve de curseur au rangement.
@compute @workgroup_size(256)
fn scan_add(@builtin(global_invocation_id) g: vec3<u32>, @builtin(workgroup_id) w: vec3<u32>) {
    let c = g.x;
    if c < P.cells {
        start[c] = start[c] + blocks[w.x];
        atomicStore(&count[c], 0u);
    }
}

@compute @workgroup_size(128)
fn bin_scatter(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n {
        return;
    }
    let m = cell_of(px[k].xyz);
    let c = cell_index(m.x, m.y, m.z);
    let slot = start[c] + atomicAdd(&count[c], 1u);
    order[slot] = k;
}

// Chaque maille trie sa tranche par indice croissant (insertion : une dizaine de particules).
@compute @workgroup_size(128)
fn bin_sort(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    let a = start[c];
    let b = start[c + 1u];
    for (var s = a + 1u; s < b; s = s + 1u) {
        let v = order[s];
        var t = s;
        loop {
            if t <= a || order[t - 1u] <= v {
                break;
            }
            order[t] = order[t - 1u];
            t = t - 1u;
        }
        order[t] = v;
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Grilles décalées (`staggered`) : l'axe d'une face, ses indices, l'origine de son nœud (0, 0, 0) en mailles, ses dimensions.

struct Face {
    axis: u32,
    idx: vec3<u32>,
    local: u32,
}

fn dims_of(axis: u32) -> vec3<u32> {
    if axis == 0u {
        return vec3<u32>(P.nx + 1u, P.ny, P.nz);
    }
    if axis == 1u {
        return vec3<u32>(P.nx, P.ny + 1u, P.nz);
    }
    return vec3<u32>(P.nx, P.ny, P.nz + 1u);
}

fn origin_of(axis: u32) -> vec3<f32> {
    if axis == 0u {
        return vec3<f32>(0.0, 0.5, 0.5);
    }
    if axis == 1u {
        return vec3<f32>(0.5, 0.0, 0.5);
    }
    return vec3<f32>(0.5, 0.5, 0.0);
}

// Premier indice global de la famille `axis` dans les tampons de faces.
fn face_base(axis: u32) -> u32 {
    if axis == 0u {
        return 0u;
    }
    if axis == 1u {
        return P.nu;
    }
    return P.nu + P.nv;
}

fn face_of(f: u32) -> Face {
    var axis = 2u;
    var local = f - P.nu - P.nv;
    if f < P.nu {
        axis = 0u;
        local = f;
    } else if f < P.nu + P.nv {
        axis = 1u;
        local = f - P.nu;
    }
    let d = dims_of(axis);
    return Face(axis, vec3<u32>(local % d.x, (local / d.x) % d.y, local / (d.x * d.y)), local);
}

// Les poids trilinéaires bornés d'un point sur la grille `axis` (`weights`) : base, fraction et nœud suivant par composante.
struct Lerp {
    base: vec3<u32>,
    frac: vec3<f32>,
    next: vec3<u32>,
}

fn lerp_of(p: vec3<f32>, axis: u32) -> Lerp {
    let d = dims_of(axis);
    let top = vec3<f32>(d - vec3<u32>(1u));
    let f = clamp(p / P.dx - origin_of(axis), vec3<f32>(0.0), max(top - vec3<f32>(1e-4), vec3<f32>(0.0)));
    let base = vec3<u32>(floor(f));
    return Lerp(base, f - vec3<f32>(base), min(base + vec3<u32>(1u), d - vec3<u32>(1u)));
}

// Le poids du nœud `node` pour ce point : la somme des emplacements de `weights` qui le désignent.
fn node_weight(l: Lerp, node: vec3<u32>) -> f32 {
    let wa = select(vec3<f32>(0.0), vec3<f32>(1.0) - l.frac, node == l.base) + select(vec3<f32>(0.0), l.frac, node == l.next);
    return wa.x * wa.y * wa.z;
}

// **Particules → grille**, APIC (`particles_to_grid`) : `u_f = Σ w·(v_a + C_a·(x_f − x_p)) / Σ w`, en collecte sur les mailles
// qui peuvent atteindre la face — deux le long de son axe, trois dans les autres.
@compute @workgroup_size(128)
fn p2g(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    let fc = face_of(f);
    let xf = (vec3<f32>(fc.idx) + origin_of(fc.axis)) * P.dx;
    let along = vec3<bool>(fc.axis == 0u, fc.axis == 1u, fc.axis == 2u);
    let lo = max(vec3<i32>(fc.idx) - vec3<i32>(1), vec3<i32>(0));
    let hi = min(select(vec3<i32>(fc.idx) + vec3<i32>(1), vec3<i32>(fc.idx), along),
        vec3<i32>(i32(P.nx) - 1, i32(P.ny) - 1, i32(P.nz) - 1));
    var sum = 0.0;
    var wsum = 0.0;
    for (var c = lo.z; c <= hi.z; c = c + 1) {
        for (var b = lo.y; b <= hi.y; b = b + 1) {
            for (var a = lo.x; a <= hi.x; a = a + 1) {
                let cell = cell_index(u32(a), u32(b), u32(c));
                for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
                    let k = order[s];
                    let p = px[k].xyz;
                    let wt = node_weight(lerp_of(p, fc.axis), fc.idx);
                    if wt == 0.0 {
                        continue;
                    }
                    let row = pc[3u * k + fc.axis].xyz;
                    let e = xf - p;
                    let affine = row.x * e.x + row.y * e.y + row.z * e.z;
                    sum = sum + wt * (pv[k][fc.axis] + affine);
                    wsum = wsum + wt;
                }
            }
        }
    }
    if wsum > 0.0 {
        faces[f] = sum / wsum;
    } else {
        faces[f] = 0.0;
    }
    faces[P.faces + f] = wsum;
}

// ---------------------------------------------------------------------------------------------------------------------
// **La surface reconstruite** (`reconstruct`, Zhu et Bridson 2005) : aux centres des mailles, `φ = |q − x̄| − r`, `x̄` la
// moyenne des particules à moins de `R = kernel·dx`, pondérée par `(1 − s²/R²)³` ; `φ = dx` sans voisine. Les parois
// reflètent les particules (S389) ; le couvercle non. Puis les étiquettes : eau où `φ < 0`.

fn smooth_kernel(s2: f32) -> f32 {
    if s2 < 1.0 {
        let t = 1.0 - s2;
        return t * t * t;
    }
    return 0.0;
}

// Image : 1 — la particule ; −1 — reflétée par la paroi basse ; 2 — par la paroi haute.
fn mirror(v: f32, m: f32, l: f32) -> f32 {
    if m == 1.0 {
        return v;
    }
    if m == -1.0 {
        return -v;
    }
    return 2.0 * l - v;
}

// Les images cherchées : l'image `m` (0, 1, 2) d'un axe est prise si `m = 0`, ou si le centre est près de sa paroi.
fn image_on(m: u32, near_low: bool, near_high: bool) -> bool {
    return m == 0u || (m == 1u && near_low) || (m == 2u && near_high);
}

@compute @workgroup_size(128)
fn reconstruct(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    let i = c % P.nx;
    let j = (c / P.nx) % P.ny;
    let k = c / (P.nx * P.ny);
    let q = (vec3<f32>(f32(i), f32(j), f32(k)) + vec3<f32>(0.5)) * P.dx;
    let radius = P.kr;
    let inv_r2 = 1.0 / (radius * radius);
    let near_x0 = q.x < radius;
    let near_x1 = q.x > P.lx - radius;
    let near_y0 = q.y < radius;
    let near_y1 = q.y > P.ly - radius;
    let near_z0 = q.z < radius;
    let images = vec3<f32>(1.0, -1.0, 2.0);
    // S393 : le corps reflète aussi, image radiale `c + (2R − d)·n`, cherchée seulement près de lui.
    let bc = vec3<f32>(P.bcx, P.bcy, P.bcz);
    let eq = q - bc;
    let use_body = P.has_body != 0.0 && sqrt(eq.x * eq.x + eq.y * eq.y + eq.z * eq.z) < P.br + radius;
    let r = P.reach;
    // S398 : une maille de la zone des colonnes prend `φ = z − η` (`columns_label`) ; rien à reconstruct.
    if P.has_columns != 0.0 && cmask[j * P.nx + i] != 0u {
        let phi_c = q.z - columns_read(cols[j * P.nx + i]);
        cellf[c] = phi_c;
        var lab_c = select(AIR, WATER, phi_c < 0.0);
        if P.has_body != 0.0 && eq.x * eq.x + eq.y * eq.y + eq.z * eq.z < P.br * P.br {
            lab_c = SOLID;
        }
        label[c] = lab_c;
        return;
    }
    let lo = vec3<u32>(select(0u, i - r, i >= r), select(0u, j - r, j >= r), select(0u, k - r, k >= r));
    let hi = vec3<u32>(min(i + r + 1u, P.nx), min(j + r + 1u, P.ny), min(k + r + 1u, P.nz));
    var sw = 0.0;
    var sx = vec3<f32>(0.0);
    for (var cz = lo.z; cz < hi.z; cz = cz + 1u) {
        for (var cy = lo.y; cy < hi.y; cy = cy + 1u) {
            for (var cx = lo.x; cx < hi.x; cx = cx + 1u) {
                let cell = cell_index(cx, cy, cz);
                for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
                    let p0 = px[order[s]].xyz;
                    for (var ax = 0u; ax < 3u; ax = ax + 1u) {
                        if image_on(ax, near_x0, near_x1) {
                            for (var ay = 0u; ay < 3u; ay = ay + 1u) {
                                if image_on(ay, near_y0, near_y1) {
                                    for (var az = 0u; az < 2u; az = az + 1u) {
                                        if image_on(az, near_z0, false) {
                                            let p = vec3<f32>(mirror(p0.x, images[ax], P.lx), mirror(p0.y, images[ay], P.ly),
                                                mirror(p0.z, images[az], 0.0));
                                            let d = p - q;
                                            let wt = smooth_kernel((d.x * d.x + d.y * d.y + d.z * d.z) * inv_r2);
                                            if wt > 0.0 {
                                                sw = sw + wt;
                                                sx = sx + wt * p;
                                            }
                                            if use_body {
                                                let e = p - bc;
                                                let de = sqrt(e.x * e.x + e.y * e.y + e.z * e.z);
                                                if de > 0.0 && de < 2.0 * P.br {
                                                    let f = (2.0 * P.br - de) / de;
                                                    let pi = bc + f * e;
                                                    let di = pi - q;
                                                    let wi = smooth_kernel((di.x * di.x + di.y * di.y + di.z * di.z) * inv_r2);
                                                    if wi > 0.0 {
                                                        sw = sw + wi;
                                                        sx = sx + wi * pi;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // S399 : près de la zone, la reconstruction compte aussi les particules **virtuelles** des colonnes.
    if P.has_columns != 0.0 {
        let v = virtual_column_sums(q, i, j, radius, inv_r2, near_x0, near_x1, near_y0, near_y1, near_z0);
        sw = sw + v.w;
        sx = sx + v.xyz;
    }
    var phi = P.dx;
    if sw > 0.0 {
        let m = sx / sw - q;
        phi = sqrt(m.x * m.x + m.y * m.y + m.z * m.z) - P.radius;
    }
    cellf[c] = phi;
    var lab = select(AIR, WATER, phi < 0.0);
    // `label_body` : une maille dont le centre est dans la sphère est solide.
    if P.has_body != 0.0 && eq.x * eq.x + eq.y * eq.y + eq.z * eq.z < P.br * P.br {
        lab = SOLID;
    }
    label[c] = lab;
}

// ---------------------------------------------------------------------------------------------------------------------
// Gravité et parois (`step`, `walls`) : `w −= g·dt` partout, puis vitesse normale nulle sur le bord du domaine.

fn on_wall(fc: Face) -> bool {
    let d = dims_of(fc.axis);
    let along = select(select(fc.idx.z, fc.idx.y, fc.axis == 1u), fc.idx.x, fc.axis == 0u);
    let top = select(select(d.z, d.y, fc.axis == 1u), d.x, fc.axis == 0u) - 1u;
    return along == 0u || along == top;
}

@compute @workgroup_size(128)
fn gravity_walls(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    let fc = face_of(f);
    if fc.axis == 2u {
        faces[f] = faces[f] - P.gdt;
    }
    if on_wall(fc) {
        faces[f] = 0.0;
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// **La projection** (`project`) : `A·p = −(ρ·dx²/dt)·div u*` sur l'eau, gradient conjugué préconditionné par la diagonale,
// scalaires gardés sur la carte ; puis `u −= (dt/(ρ·dx))·∇p` sur les faces intérieures qui touchent l'eau, la pression
// d'air nulle à `θ·dx`. Champs de `cellf` : φ 0, p 1, rhs 2, r 3, z 4, d 5, q 6, diag 7.

const F_PHI: u32 = 0u;
const F_P: u32 = 1u;
const F_RHS: u32 = 2u;
const F_R: u32 = 3u;
const F_Z: u32 = 4u;
const F_D: u32 = 5u;
const F_Q: u32 = 6u;
const F_DIAG: u32 = 7u;
// Scalaires.
const S_B2: u32 = 0u;
const S_RZ: u32 = 1u;
const S_RR: u32 = 2u;
const S_DQ: u32 = 3u;
const S_ALPHA: u32 = 4u;
const S_BETA: u32 = 5u;
const S_DONE: u32 = 6u;
const S_IT: u32 = 7u;

fn field(which: u32, c: u32) -> u32 {
    return which * P.cells + c;
}

// La fraction fantôme de la maille d'eau `c` vers sa voisine d'air `a` (`theta`).
fn theta(c: u32, a: u32) -> f32 {
    let fc = cellf[field(F_PHI, c)];
    let fa = cellf[field(F_PHI, a)];
    return max(fc / (fc - fa), P.theta_min);
}

// Voisine `m` (0 : −x, 1 : +x, 2 : −y, 3 : +y, 4 : −z, 5 : +z) de la maille `(i, j, k)`, et la face qui les sépare ;
// `valid` faux hors du domaine (`neighbours`).
struct Nb {
    valid: bool,
    cell: u32,
    face: u32,
}

fn neighbour(i: u32, j: u32, k: u32, m: u32) -> Nb {
    let fu = (k * P.ny + j) * (P.nx + 1u) + i;
    let fv = P.nu + (k * (P.ny + 1u) + j) * P.nx + i;
    let fw = P.nu + P.nv + (k * P.ny + j) * P.nx + i;
    switch m {
        case 0u: { return Nb(i > 0u, cell_index(max(i, 1u) - 1u, j, k), fu); }
        case 1u: { return Nb(i + 1u < P.nx, cell_index(min(i + 1u, P.nx - 1u), j, k), fu + 1u); }
        case 2u: { return Nb(j > 0u, cell_index(i, max(j, 1u) - 1u, k), fv); }
        case 3u: { return Nb(j + 1u < P.ny, cell_index(i, min(j + 1u, P.ny - 1u), k), fv + P.nx); }
        case 4u: { return Nb(k > 0u, cell_index(i, j, max(k, 1u) - 1u), fw); }
        default: { return Nb(k + 1u < P.nz, cell_index(i, j, min(k + 1u, P.nz - 1u)), fw + P.nx * P.ny); }
    }
}

// La face `m` d'une maille, même hors du domaine (une paroi), pour la divergence.
fn face_value(i: u32, j: u32, k: u32, m: u32) -> f32 {
    return faces[neighbour(i, j, k, m).face];
}

@compute @workgroup_size(256)
fn assemble(@builtin(global_invocation_id) g: vec3<u32>) {
    let c = g.x;
    if c >= P.cells {
        return;
    }
    cellf[field(F_P, c)] = 0.0;
    if label[c] != WATER {
        cellf[field(F_RHS, c)] = 0.0;
        cellf[field(F_DIAG, c)] = 0.0;
        cellf[field(F_R, c)] = 0.0;
        cellf[field(F_Z, c)] = 0.0;
        cellf[field(F_D, c)] = 0.0;
        return;
    }
    let i = c % P.nx;
    let j = (c / P.nx) % P.ny;
    let k = c / (P.nx * P.ny);
    let scale = -P.rho * P.dx * P.dx / P.dt;
    var div = 0.0;
    var diag = 0.0;
    for (var m = 0u; m < 6u; m = m + 1u) {
        let sign = select(1.0, -1.0, m % 2u == 0u);
        let nb = neighbour(i, j, k, m);
        div = div + sign * faces[nb.face];
        if nb.valid {
            let l = label[nb.cell];
            if l == WATER {
                diag = diag + 1.0;
            } else if l == AIR {
                diag = diag + 1.0 / theta(c, nb.cell);
            }
        }
    }
    let rhs = scale * div / P.dx;
    cellf[field(F_RHS, c)] = rhs;
    cellf[field(F_DIAG, c)] = diag;
    cellf[field(F_R, c)] = rhs;
    var z = 0.0;
    if diag > 0.0 {
        z = rhs / diag;
    }
    cellf[field(F_Z, c)] = z;
    cellf[field(F_D, c)] = z;
}

var<workgroup> red_a: array<f32, 256>;
var<workgroup> red_b: array<f32, 256>;
var<workgroup> wg_done: f32;

// Somme des deux produits du groupe dans `partials` (deux par groupe).
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

// Somme des `partials` de tous les groupes, par un seul groupe : rend les deux sommes au fil 0.
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

// Le drapeau de fin, lu uniformément dans le groupe.
fn done_uniform(l: u32) -> bool {
    if l == 0u {
        wg_done = scalars[S_DONE];
    }
    return workgroupUniformLoad(&wg_done) != 0.0;
}

@compute @workgroup_size(256)
fn cg_init_reduce(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                  @builtin(workgroup_id) w: vec3<u32>) {
    let c = g.x;
    var a = 0.0;
    var b = 0.0;
    if c < P.cells {
        let rhs = cellf[field(F_RHS, c)];
        a = rhs * rhs;
        b = cellf[field(F_R, c)] * cellf[field(F_Z, c)];
    }
    reduce_pair(l.x, w.x, a, b);
}

@compute @workgroup_size(256)
fn cg_init_finish(@builtin(local_invocation_id) l: vec3<u32>) {
    let s = gather_pair(l.x);
    if l.x == 0u {
        scalars[S_B2] = s.x;
        scalars[S_RZ] = s.y;
        scalars[S_RR] = s.x;
        scalars[S_IT] = 0.0;
        scalars[S_DONE] = select(0.0, 1.0, !(s.x > 0.0));
    }
}

// `q = A·d` et le produit `d·q` : `Σ (d_c − d_n)` vers l'eau, `d_c/θ` vers l'air, rien vers une paroi (`apply`).
@compute @workgroup_size(256)
fn cg_apply(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
            @builtin(workgroup_id) w: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let c = g.x;
    var dq = 0.0;
    if c < P.cells {
        var s = 0.0;
        if label[c] == WATER {
            let i = c % P.nx;
            let j = (c / P.nx) % P.ny;
            let k = c / (P.nx * P.ny);
            let dc = cellf[field(F_D, c)];
            for (var m = 0u; m < 6u; m = m + 1u) {
                let nb = neighbour(i, j, k, m);
                if nb.valid {
                    let lb = label[nb.cell];
                    if lb == WATER {
                        s = s + (dc - cellf[field(F_D, nb.cell)]);
                    } else if lb == AIR {
                        s = s + dc / theta(c, nb.cell);
                    }
                }
            }
        }
        cellf[field(F_Q, c)] = s;
        dq = cellf[field(F_D, c)] * s;
    }
    reduce_pair(l.x, w.x, dq, 0.0);
}

@compute @workgroup_size(256)
fn cg_alpha(@builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    if l.x == 0u {
        scalars[S_DQ] = s.x;
        if !(s.x > 0.0) {
            scalars[S_DONE] = 1.0;
        } else {
            scalars[S_ALPHA] = scalars[S_RZ] / s.x;
        }
    }
}

@compute @workgroup_size(256)
fn cg_update(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
             @builtin(workgroup_id) w: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let c = g.x;
    var rz = 0.0;
    var rr = 0.0;
    if c < P.cells {
        let alpha = scalars[S_ALPHA];
        cellf[field(F_P, c)] = cellf[field(F_P, c)] + alpha * cellf[field(F_D, c)];
        let r = cellf[field(F_R, c)] - alpha * cellf[field(F_Q, c)];
        cellf[field(F_R, c)] = r;
        let diag = cellf[field(F_DIAG, c)];
        var z = 0.0;
        if diag > 0.0 {
            z = r / diag;
        }
        cellf[field(F_Z, c)] = z;
        rz = r * z;
        rr = r * r;
    }
    reduce_pair(l.x, w.x, rz, rr);
}

@compute @workgroup_size(256)
fn cg_beta(@builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let s = gather_pair(l.x);
    if l.x == 0u {
        scalars[S_BETA] = s.x / scalars[S_RZ];
        scalars[S_RZ] = s.x;
        scalars[S_RR] = s.y;
        let it = scalars[S_IT] + 1.0;
        scalars[S_IT] = it;
        if s.y <= P.tol2 * scalars[S_B2] || it >= f32(P.max_it) {
            scalars[S_DONE] = 1.0;
        }
    }
}

@compute @workgroup_size(256)
fn cg_direction(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>) {
    if done_uniform(l.x) {
        return;
    }
    let c = g.x;
    if c < P.cells {
        cellf[field(F_D, c)] = cellf[field(F_Z, c)] + scalars[S_BETA] * cellf[field(F_D, c)];
    }
}

// La correction : chaque face intérieure entre `c` (côté négatif) et `n` ; une paroi n'est jamais corrigée.
@compute @workgroup_size(128)
fn correct(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    let fc = face_of(f);
    if on_wall(fc) {
        return;
    }
    let along = vec3<u32>(select(0u, 1u, fc.axis == 0u), select(0u, 1u, fc.axis == 1u), select(0u, 1u, fc.axis == 2u));
    let cm = fc.idx - along;
    let c = cell_index(cm.x, cm.y, cm.z);
    let n = cell_index(fc.idx.x, fc.idx.y, fc.idx.z);
    if label[c] == SOLID || label[n] == SOLID {
        return;
    }
    let wc = label[c] == WATER;
    let wn = label[n] == WATER;
    var grad = 0.0;
    if wc && wn {
        grad = cellf[field(F_P, n)] - cellf[field(F_P, c)];
    } else if wc {
        grad = -cellf[field(F_P, c)] / theta(c, n);
    } else if wn {
        grad = cellf[field(F_P, n)] / theta(n, c);
    } else {
        return;
    }
    let k1 = P.dt / (P.rho * P.dx);
    faces[f] = faces[f] - k1 * grad;
}

// ---------------------------------------------------------------------------------------------------------------------
// **L'extrapolation** (`extrapolate`) : valide, une face qui touche une maille d'eau ; trois couches, chaque face invalide
// prend la moyenne de ses six voisines valides de la couche précédente ; au-delà, zéro — sauf les faces qu'une particule a
// alimentées ; puis les parois. Copies dans `faces[2F..3F)` et `fflags[F..2F)`.

fn face_global(axis: u32, idx: vec3<u32>) -> u32 {
    let d = dims_of(axis);
    return face_base(axis) + (idx.z * d.y + idx.y) * d.x + idx.x;
}

fn water_at(a: i32, b: i32, c: i32) -> bool {
    if a < 0 || b < 0 || c < 0 || a >= i32(P.nx) || b >= i32(P.ny) || c >= i32(P.nz) {
        return false;
    }
    return label[cell_index(u32(a), u32(b), u32(c))] == WATER;
}

@compute @workgroup_size(128)
fn extrap_valid(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    let fc = face_of(f);
    let q = vec3<i32>(fc.idx);
    var lower = q;
    if fc.axis == 0u {
        lower = q - vec3<i32>(1, 0, 0);
    } else if fc.axis == 1u {
        lower = q - vec3<i32>(0, 1, 0);
    } else {
        lower = q - vec3<i32>(0, 0, 1);
    }
    let ok = water_at(lower.x, lower.y, lower.z) || water_at(q.x, q.y, q.z);
    fflags[f] = select(0u, 1u, ok);
}

@compute @workgroup_size(128)
fn extrap_copy(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    faces[2u * P.faces + f] = faces[f];
    fflags[P.faces + f] = fflags[f];
}

@compute @workgroup_size(128)
fn extrap_layer(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    if fflags[P.faces + f] != 0u {
        return;
    }
    let fc = face_of(f);
    let d = vec3<i32>(dims_of(fc.axis));
    let q = vec3<i32>(fc.idx);
    var s = 0.0;
    var n = 0u;
    for (var m = 0u; m < 6u; m = m + 1u) {
        var o = vec3<i32>(0);
        let sign = select(1, -1, m % 2u == 0u);
        if m < 2u {
            o = vec3<i32>(sign, 0, 0);
        } else if m < 4u {
            o = vec3<i32>(0, sign, 0);
        } else {
            o = vec3<i32>(0, 0, sign);
        }
        let t = q + o;
        if all(t >= vec3<i32>(0)) && all(t < d) {
            let h = face_global(fc.axis, vec3<u32>(t));
            if fflags[P.faces + h] != 0u {
                s = s + faces[2u * P.faces + h];
                n = n + 1u;
            }
        }
    }
    if n > 0u {
        faces[f] = s / f32(n);
        fflags[f] = 1u;
    }
}

@compute @workgroup_size(128)
fn extrap_zero(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces {
        return;
    }
    if fflags[f] == 0u && faces[P.faces + f] == 0.0 {
        faces[f] = 0.0;
    }
    if on_wall(face_of(f)) {
        faces[f] = 0.0;
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// **Grille → particules** (`grid_to_particles`) et **advection** RK2 (`advect`, `grid_velocity`) : les huit emplacements
// de `weights`, dans son ordre.

// La valeur interpolée sur la grille `axis` au point `p`, et `Σ ∂w·u` (la ligne de `C`).
fn interp(axis: u32, p: vec3<f32>) -> vec4<f32> {
    let l = lerp_of(p, axis);
    let inv = 1.0 / P.dx;
    var value = 0.0;
    var grad = vec3<f32>(0.0);
    for (var m = 0u; m < 8u; m = m + 1u) {
        let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
        let idx = select(l.base, l.next, sel);
        let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
        let ga = select(vec3<f32>(-inv), vec3<f32>(inv), sel);
        let wt = wa.x * wa.y * wa.z;
        let gw = vec3<f32>(ga.x * wa.y * wa.z, wa.x * ga.y * wa.z, wa.x * wa.y * ga.z);
        let f = faces[face_global(axis, idx)];
        value = value + wt * f;
        grad = grad + gw * f;
    }
    return vec4<f32>(value, grad);
}

fn interp_value(axis: u32, p: vec3<f32>) -> f32 {
    let l = lerp_of(p, axis);
    var value = 0.0;
    for (var m = 0u; m < 8u; m = m + 1u) {
        let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
        let idx = select(l.base, l.next, sel);
        let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
        value = value + wa.x * wa.y * wa.z * faces[face_global(axis, idx)];
    }
    return value;
}

fn grid_velocity(p: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(interp_value(0u, p), interp_value(1u, p), interp_value(2u, p));
}

@compute @workgroup_size(128)
fn g2p(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n {
        return;
    }
    let p = px[k].xyz;
    let a = interp(0u, p);
    let b = interp(1u, p);
    let c = interp(2u, p);
    pv[k] = vec4<f32>(a.x, b.x, c.x, 0.0);
    pc[3u * k] = vec4<f32>(a.yzw, 0.0);
    pc[3u * k + 1u] = vec4<f32>(b.yzw, 0.0);
    pc[3u * k + 2u] = vec4<f32>(c.yzw, 0.0);
}

fn clamp_domain(p: vec3<f32>) -> vec3<f32> {
    return clamp(p, vec3<f32>(P.margin), vec3<f32>(P.lx, P.ly, P.lz) - vec3<f32>(P.margin));
}

@compute @workgroup_size(128)
fn advect(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n {
        return;
    }
    let p = px[k].xyz;
    let v1 = grid_velocity(p);
    let mid = p + 0.5 * P.dt * v1;
    let v2 = grid_velocity(mid);
    px[k] = vec4<f32>(clamp_domain(p + P.dt * v2), 0.0);
}

// ---------------------------------------------------------------------------------------------------------------------
// **La séparation** (`separate`) : deux particules plus proches que 0,4 maille s'écartent chacune du quart de leur
// recouvrement ; en collecte, chaque particule somme les poussées de ses voisines — la somme par paires de la référence.

@compute @workgroup_size(128)
fn separate_shift(@builtin(global_invocation_id) g: vec3<u32>) {
    let a = g.x;
    if a >= P.n {
        return;
    }
    let xa = px[a].xyz;
    let m = cell_of(xa);
    let lo = vec3<u32>(select(0u, m.x - 1u, m.x > 0u), select(0u, m.y - 1u, m.y > 0u), select(0u, m.z - 1u, m.z > 0u));
    let hi = min(m + vec3<u32>(2u), vec3<u32>(P.nx, P.ny, P.nz));
    var sh = vec3<f32>(0.0);
    for (var cz = lo.z; cz < hi.z; cz = cz + 1u) {
        for (var cy = lo.y; cy < hi.y; cy = cy + 1u) {
            for (var cx = lo.x; cx < hi.x; cx = cx + 1u) {
                let cell = cell_index(cx, cy, cz);
                for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
                    let b = order[s];
                    if b == a {
                        continue;
                    }
                    let e = px[b].xyz - xa;
                    let d = sqrt(e.x * e.x + e.y * e.y + e.z * e.z);
                    if d < P.dmin && d > 1e-6 * P.dx {
                        let w = 0.25 * (P.dmin - d) / d;
                        sh = sh - w * e;
                    }
                }
            }
        }
    }
    shift[a] = vec4<f32>(sh, 0.0);
}

@compute @workgroup_size(128)
fn separate_apply(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n {
        return;
    }
    px[k] = vec4<f32>(clamp_domain(px[k].xyz + shift[k].xyz), 0.0);
}

// ---------------------------------------------------------------------------------------------------------------------
// **Le corps** (S393) : `impose_body` — toute face qui touche une maille solide prend la vitesse du corps, une face de bord
// reste une paroi ; `move_body` — les particules atteintes par le corps avancé sont repoussées à sa surface (plus 0,05 maille),
// leur vitesse normale au moins égale à la sienne.

@compute @workgroup_size(128)
fn impose_body(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces || P.has_body == 0.0 {
        return;
    }
    let fc = face_of(f);
    let d = dims_of(fc.axis);
    let along = vec3<u32>(select(0u, 1u, fc.axis == 0u), select(0u, 1u, fc.axis == 1u), select(0u, 1u, fc.axis == 2u));
    let a = dot(fc.idx, along);
    let top = dot(d, along) - 1u;
    var touches = false;
    if a > 0u {
        let m = fc.idx - along;
        touches = touches || label[cell_index(m.x, m.y, m.z)] == SOLID;
    }
    if a < top {
        touches = touches || label[cell_index(fc.idx.x, fc.idx.y, fc.idx.z)] == SOLID;
    }
    if !touches {
        return;
    }
    let vb = select(select(P.bvz, P.bvy, fc.axis == 1u), P.bvx, fc.axis == 0u);
    faces[f] = select(vb, 0.0, on_wall(fc));
}

@compute @workgroup_size(128)
fn move_body(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= P.n || P.has_body == 0.0 {
        return;
    }
    let c = vec3<f32>(P.bmx, P.bmy, P.bmz);
    let reach = P.br + 0.05 * P.dx;
    let p = px[k].xyz;
    let e = p - c;
    let d = sqrt(e.x * e.x + e.y * e.y + e.z * e.z);
    if d >= reach {
        return;
    }
    var n = vec3<f32>(0.0, 0.0, 1.0);
    if d > 0.0 {
        n = e / d;
    }
    px[k] = vec4<f32>(clamp_domain(c + n * reach), 0.0);
    var v = pv[k].xyz;
    let bv = vec3<f32>(P.bvx, P.bvy, P.bvz);
    let vn = v.x * n.x + v.y * n.y + v.z * n.z;
    let wn = bv.x * n.x + bv.y * n.y + bv.z * n.z;
    if vn < wn {
        v = v + (wn - vn) * n;
    }
    pv[k] = vec4<f32>(v, 0.0);
}

// ---------------------------------------------------------------------------------------------------------------------
// **La zone des colonnes** (S398–S400, C7c-1).

// `columns_read` (S400) : la hauteur que la pression voit, `η + e(η)`, `e` l'erreur que la bande ferait en lisant la même
// surface (table périodique sur une maille) ; `η` sans bande.
fn columns_read(eta: f32) -> f32 {
    if P.band == 0.0 {
        return eta;
    }
    let x = eta / P.dx - 0.5;
    let o = (x - floor(x)) * 32.0;
    let m = min(u32(floor(o)), 31u);
    let t = o - f32(m);
    let base = 2u * P.nx * P.ny;
    let a = cols[base + m];
    let b = cols[base + (m + 1u) % 32u];
    return eta + a + t * (b - a);
}

// `virtual_column_sums` (S399) : chaque colonne de la zone à portée du noyau compte comme `2 × 2` particules par rangée,
// `round(2η/dx)` rangées étirées sur `[0, η]` ; images aux parois comme les particules. Rend `(Σw·p, Σw)`.
fn virtual_column_sums(q: vec3<f32>, i: u32, j: u32, radius: f32, inv_r2: f32, near_x0: bool, near_x1: bool,
                       near_y0: bool, near_y1: bool, near_z0: bool) -> vec4<f32> {
    let r = P.reach;
    let images = vec3<f32>(1.0, -1.0, 2.0);
    var sw = 0.0;
    var sx = vec3<f32>(0.0);
    let bl = select(0u, j - r, j >= r);
    let bh = min(j + r + 1u, P.ny);
    let al = select(0u, i - r, i >= r);
    let ah = min(i + r + 1u, P.nx);
    for (var b = bl; b < bh; b = b + 1u) {
        for (var a = al; a < ah; a = a + 1u) {
            let col = b * P.nx + a;
            if cmask[col] == 0u {
                continue;
            }
            let eta = max(cols[col], 0.0);
            let rows = max(u32(floor(2.0 * eta / P.dx + 0.5)), 1u);
            let pitch = eta / f32(rows);
            let lo = u32(max(floor(max(q.z - radius, 0.0) / pitch - 0.5), 0.0));
            let hi = min(u32(max(ceil((q.z + radius) / pitch - 0.5), 0.0)), rows - 1u);
            for (var row = lo; row <= hi; row = row + 1u) {
                let z = (f32(row) + 0.5) * pitch;
                for (var o = 0u; o < 4u; o = o + 1u) {
                    let ox = select(0.25, 0.75, (o & 1u) == 1u);
                    let oy = select(0.25, 0.75, (o & 2u) == 2u);
                    let p0 = vec3<f32>((f32(a) + ox) * P.dx, (f32(b) + oy) * P.dx, z);
                    for (var mx = 0u; mx < 3u; mx = mx + 1u) {
                        if image_on(mx, near_x0, near_x1) {
                            for (var my = 0u; my < 3u; my = my + 1u) {
                                if image_on(my, near_y0, near_y1) {
                                    for (var mz = 0u; mz < 2u; mz = mz + 1u) {
                                        if image_on(mz, near_z0, false) {
                                            let p = vec3<f32>(mirror(p0.x, images[mx], P.lx), mirror(p0.y, images[my], P.ly),
                                                mirror(p0.z, images[mz], 0.0));
                                            let d = p - q;
                                            let wt = smooth_kernel((d.x * d.x + d.y * d.y + d.z * d.z) * inv_r2);
                                            if wt > 0.0 {
                                                sw = sw + wt;
                                                sx = sx + wt * p;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return vec4<f32>(sx, sw);
}
