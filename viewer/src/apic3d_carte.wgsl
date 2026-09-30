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
    tol2: f32, p0: f32, p1: f32, p2: f32,
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
    let r = P.reach;
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
    var phi = P.dx;
    if sw > 0.0 {
        let m = sx / sw - q;
        phi = sqrt(m.x * m.x + m.y * m.y + m.z * m.z) - P.radius;
    }
    cellf[c] = phi;
    label[c] = select(AIR, WATER, phi < 0.0);
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
