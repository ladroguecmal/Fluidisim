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
