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
    // S419 : un fond de bande existe-t-il (C7c-3) ?
    floors: f32, r0: f32, r1: f32, r2: f32,
    // S420 — le critère de bascule (`ColumnsSwitch`) : instant (µs, 32 bits), maintien (µs), dilatation, fond en mailles et son
    // hystérésis, prédiction du corps, fond demandé ; pente, pente de relâche (négative : aucune), marge du corps, horizon.
    s_now: u32, s_hold: u32, s_dil: u32, s_fcells: u32,
    s_fhyst: u32, s_pred: u32, s_has_fcells: u32, s_pad: u32,
    s_slope: f32, s_release: f32, s_margin: f32, s_horizon: f32,
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
// S417 — **les volumes en entiers** : le volume de chaque colonne [0, 2C), les débits des faces `x` [2C, 2C + 2Fx) puis `y`,
// en quanta `q = dx³/8 · 2⁻²⁴` (un volume de particule vaut 2²⁴ quanta), sur deux mots (bas, haut), complément à deux.
// Chaque débit s'arrondit une fois au quantum et s'ajoute d'un côté, se retranche de l'autre : la conservation est exacte.
@group(0) @binding(17) var<storage, read_write> ivol: array<u32>;
// S418 — les soldes des faces-mailles de frontière, en quanta sur deux mots : faces `u` [0, nu), puis `v`.
@group(0) @binding(18) var<storage, read_write> isolde: array<u32>;
// S418 — `n` résident et les compteurs de l'échange : n, n au début de l'échange, longueur de la liste des absorbées,
// absorbées, retirées, posées, refusées.
@group(0) @binding(19) var<storage, read_write> pcount: array<atomic<u32>>;
// S418 — la liste des absorbées, puis l'ordre de visite (C7c-2).
@group(0) @binding(20) var<storage, read_write> plist: array<u32>;
// S418 — le compactage : vivantes par groupe de 256 (préfixe), et un tampon de passage (x, v, trois lignes de C).
@group(0) @binding(21) var<storage, read_write> pblk: array<u32>;
@group(0) @binding(22) var<storage, read_write> pscratch: array<vec4<f32>>;
// S420 — l'état du critère, par colonne : instant requis [0, C), besoin, dilatation, gardée, hauteur (bits), demande, fond demandé
// (bits) — sept tranches de C.
@group(0) @binding(23) var<storage, read_write> swb: array<u32>;
// S421 — la liste ordonnée des faces-mailles actives de l'échange.
@group(0) @binding(24) var<storage, read_write> flist: array<u32>;

// Le nombre de particules, résident.
fn np() -> u32 {
    return atomicLoad(&pcount[0]);
}

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
    if k >= np() || px[k].w != 0.0 {
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
    if k >= np() || px[k].w != 0.0 {
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
    // S413 : sous le fond de la bande, l'eau est à la grille — `φ = z − fond` (`columns_label`).
    let fl = floor_of(j * P.nx + i);
    if q.z < fl {
        cellf[c] = q.z - fl;
        var lab_f = WATER;
        if P.has_body != 0.0 && eq.x * eq.x + eq.y * eq.y + eq.z * eq.z < P.br * P.br {
            lab_f = SOLID;
        }
        label[c] = lab_f;
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
    if k >= np() {
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
    if k >= np() {
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
    if a >= np() {
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
    if k >= np() {
        return;
    }
    let p = px[k].xyz;
    var q = clamp_domain(p + shift[k].xyz);
    // S400 : une particule que la séparation pousserait dans une colonne de la zone garde sa position horizontale.
    if P.has_columns != 0.0 {
        let a = cell_of(q);
        let b = cell_of(p);
        if cmask[a.y * P.nx + a.x] != 0u && cmask[b.y * P.nx + b.x] == 0u {
            q = vec3<f32>(p.x, p.y, q.z);
        }
    }
    px[k] = vec4<f32>(q, px[k].w);
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
    if k >= np() || P.has_body == 0.0 {
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
    if atomicLoad(&pcount[COUNT_BAND]) == 0u {
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
            // S413 : une colonne de la bande à fond compte ses virtuelles jusqu'à son fond — la sienne comprise.
            var eta = 0.0;
            if cmask[col] != 0u {
                eta = max(cols[col], 0.0);
            } else if floor_of(col) > 0.0 {
                eta = floor_of(col);
            } else {
                continue;
            }
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

// `columns_begin` : la vitesse de la grille au début du pas, gardée dans la copie `faces[2F..3F)` jusqu'à l'advection.
@compute @workgroup_size(128)
fn columns_begin(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces || P.has_columns == 0.0 {
        return;
    }
    faces[2u * P.faces + f] = faces[f];
}

// `sample` : la vitesse du pas précédent, trilinéaire bornée, au point `p`.
fn sample_prev(p: vec3<f32>) -> vec3<f32> {
    var out = vec3<f32>(0.0);
    for (var axis = 0u; axis < 3u; axis = axis + 1u) {
        let l = lerp_of(p, axis);
        var value = 0.0;
        for (var m = 0u; m < 8u; m = m + 1u) {
            let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
            let idx = select(l.base, l.next, sel);
            let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
            value = value + wa.x * wa.y * wa.z * faces[2u * P.faces + face_global(axis, idx)];
        }
        if axis == 0u {
            out.x = value;
        } else if axis == 1u {
            out.y = value;
        } else {
            out.z = value;
        }
    }
    return out;
}

fn in_zone(i: i32, j: i32) -> bool {
    return i >= 0 && j >= 0 && i < i32(P.nx) && j < i32(P.ny) && cmask[u32(j) * P.nx + u32(i)] != 0u;
}

// `columns_advect` : les faces de la zone prennent la vitesse du pas précédent au pied de la caractéristique,
// `u(x_f) ← u⁻(x_f − dt·u⁻(x_f))`. Une face `u` ou `v` en est si l'une de ses deux colonnes en est (S406 : la face de frontière
// appartient à la zone) ; une face `w`, si sa colonne en est.
@compute @workgroup_size(128)
fn columns_advect(@builtin(global_invocation_id) g: vec3<u32>) {
    let f = g.x;
    if f >= P.faces || P.has_columns == 0.0 {
        return;
    }
    let fc = face_of(f);
    let i = i32(fc.idx.x);
    let j = i32(fc.idx.y);
    let k = fc.idx.z;
    var owned = false;
    if fc.axis == 0u {
        owned = grid_at(i - 1, j, k) || grid_at(i, j, k);
    } else if fc.axis == 1u {
        owned = grid_at(i, j - 1, k) || grid_at(i, j, k);
    } else {
        // S413 : la face au-dessus d'une maille sous le fond lui appartient — la dernière comprise.
        owned = in_zone(i, j) || below_floor(i, j, max(k, 1u) - 1u);
    }
    if !owned {
        return;
    }
    let x = (vec3<f32>(fc.idx) + origin_of(fc.axis)) * P.dx;
    let v = sample_prev(x);
    let foot = x - P.dt * v;
    let a = sample_prev(foot);
    faces[f] = select(select(a.z, a.y, fc.axis == 1u), a.x, fc.axis == 0u);
}

// ---------------------------------------------------------------------------------------------------------------------
// `columns_transport` (S398, S408) : `η` transporté par les débits mouillés entre colonnes de la zone — hauteur de face moyenne
// des deux colonnes, la colonne seule à une frontière — en entiers (ci-dessus).

// Le quantum rapporté à une maille de hauteur : `q / dx² = dx · 2⁻²⁷`.
fn quantum_height() -> f32 {
    return P.dx * 7.450580596923828e-9;
}

// Un flottant de quanta (|x| < 2⁵⁵), arrondi à l'entier le plus proche, en complément à deux sur deux mots.
fn to_i64(x: f32) -> vec2<u32> {
    let a = abs(x);
    let hi = floor(a / 4294967296.0);
    let lo = floor(a - hi * 4294967296.0 + 0.5);
    var r = vec2<u32>(u32(min(lo, 4294967295.0)), u32(hi));
    if x < 0.0 {
        r = add_i64(vec2<u32>(~r.x, ~r.y), vec2<u32>(1u, 0u));
    }
    return r;
}

fn add_i64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    let carry = select(0u, 1u, lo < a.x);
    return vec2<u32>(lo, a.y + b.y + carry);
}

fn neg_i64(a: vec2<u32>) -> vec2<u32> {
    return add_i64(vec2<u32>(~a.x, ~a.y), vec2<u32>(1u, 0u));
}

// L'entier, en flottant.
fn i64_to_f32(a: vec2<u32>) -> f32 {
    return f32(bitcast<i32>(a.y)) * 4294967296.0 + f32(a.x);
}

fn ivol_get(k: u32) -> vec2<u32> {
    return vec2<u32>(ivol[2u * k], ivol[2u * k + 1u]);
}

fn ivol_set(k: u32, v: vec2<u32>) {
    ivol[2u * k] = v.x;
    ivol[2u * k + 1u] = v.y;
}

// Le débit d'une face de colonnes : `e` d'abord les faces `x` (`(nx + 1)·ny`), puis `y` (`nx·(ny + 1)`) ; une face de mur, ou
// entre deux colonnes de particules, ne porte rien.
@compute @workgroup_size(128)
fn columns_flux(@builtin(global_invocation_id) g: vec3<u32>) {
    let fx = (P.nx + 1u) * P.ny;
    let fy = P.nx * (P.ny + 1u);
    let e = g.x;
    if e >= fx + fy || P.has_columns == 0.0 {
        return;
    }
    let ncol = P.nx * P.ny;
    var axis = 0u;
    var i = 0u;
    var j = 0u;
    if e < fx {
        i = e % (P.nx + 1u);
        j = e / (P.nx + 1u);
    } else {
        axis = 1u;
        i = (e - fx) % P.nx;
        j = (e - fx) / P.nx;
    }
    var q = vec2<u32>(0u, 0u);
    // S419 : les contributions de la face aux soldes verticaux de sa colonne basse et de sa colonne haute.
    var to_low = vec2<u32>(0u, 0u);
    var to_high = vec2<u32>(0u, 0u);
    let wall = (axis == 0u && (i == 0u || i == P.nx)) || (axis == 1u && (j == 0u || j == P.ny));
    if !wall {
        var low_col = 0u;
        if axis == 0u {
            low_col = j * P.nx + i - 1u;
        } else {
            low_col = (j - 1u) * P.nx + i;
        }
        let high_col = j * P.nx + i;
        let low = cmask[low_col] != 0u;
        let high = cmask[high_col] != 0u;
        if low || high {
            var surface = 0.0;
            if low && high {
                surface = 0.5 * (cols[low_col] + cols[high_col]);
            } else if low {
                surface = cols[low_col];
            } else {
                surface = cols[high_col];
            }
            let per_quantum = 1.0 / (P.dx * P.dx * P.dx * 0.125 * 5.960464477539063e-8);
            for (var k = 0u; k < P.nz; k = k + 1u) {
                let wet = clamp((surface - f32(k) * P.dx) / P.dx, 0.0, 1.0);
                if wet == 0.0 {
                    break;
                }
                var face = 0u;
                if axis == 0u {
                    face = (k * P.ny + j) * (P.nx + 1u) + i;
                } else {
                    face = P.nu + (k * (P.ny + 1u) + j) * P.nx + i;
                }
                // Volume vers les `+`, m³, en quanta.
                let volume = faces[face] * P.dx * wet * P.dx * P.dt;
                let vq = to_i64(volume * per_quantum);
                q = add_i64(q, vq);
                // S399 : à une frontière, le volume qui passe va au solde de la face-maille, dû par la bande ou à elle —
                // `solde −= vers la zone`.
                if low != high {
                    var into_zone = vq;
                    if low {
                        into_zone = neg_i64(vq);
                    }
                    // S413 : une rangée sous le fond de la bande va de contenant à contenant — au solde vertical de la colonne
                    // de la bande, et non à la frontière latérale.
                    var band_col = low_col;
                    if low {
                        band_col = high_col;
                    }
                    if P.floors != 0.0 && (f32(k) + 0.5) * P.dx < floor_of(band_col) {
                        if low {
                            to_high = add_i64(to_high, neg_i64(into_zone));
                        } else {
                            to_low = add_i64(to_low, neg_i64(into_zone));
                        }
                    } else {
                        solde_add(face, neg_i64(into_zone));
                    }
                }
            }
        } else if P.floors != 0.0 && (floor_of(low_col) > 0.0 || floor_of(high_col) > 0.0) {
            // S413 : entre deux colonnes de la bande, une rangée sous le fond des deux va de contenant à contenant ; sous le fond
            // d'une seule, d'un contenant aux particules de l'autre (son solde vertical et le solde latéral). Mailles pleines.
            let per_quantum = 1.0 / (P.dx * P.dx * P.dx * 0.125 * 5.960464477539063e-8);
            for (var k = 0u; k < P.nz; k = k + 1u) {
                let z = (f32(k) + 0.5) * P.dx;
                let gl = z < floor_of(low_col);
                let gh = z < floor_of(high_col);
                if !gl && !gh {
                    break;
                }
                var face = 0u;
                if axis == 0u {
                    face = (k * P.ny + j) * (P.nx + 1u) + i;
                } else {
                    face = P.nu + (k * (P.ny + 1u) + j) * P.nx + i;
                }
                let vq = to_i64(faces[face] * P.dx * P.dx * P.dt * per_quantum);
                if gl {
                    to_low = add_i64(to_low, neg_i64(vq));
                }
                if gh {
                    to_high = add_i64(to_high, vq);
                }
                if gl != gh {
                    if gl {
                        solde_add(face, vq);
                    } else {
                        solde_add(face, neg_i64(vq));
                    }
                }
            }
        }
    }
    ivol_set(ncol + e, q);
    let fx_fy = (P.nx + 1u) * P.ny + P.nx * (P.ny + 1u);
    ivol_set(ncol + fx_fy + 2u * e, to_low);
    ivol_set(ncol + fx_fy + 2u * e + 1u, to_high);
}

// `η ← η − (Σ débits sortants)/dx²`, en entiers, puis lu en flottant pour la surface.
@compute @workgroup_size(128)
fn columns_update(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    let ncol = P.nx * P.ny;
    if col >= ncol || P.has_columns == 0.0 || cmask[col] == 0u {
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    let fx = (P.nx + 1u) * P.ny;
    let xl = ncol + j * (P.nx + 1u) + i;
    let yl = ncol + fx + j * P.nx + i;
    var v = ivol_get(col);
    v = add_i64(v, ivol_get(xl));
    v = add_i64(v, neg_i64(ivol_get(xl + 1u)));
    v = add_i64(v, ivol_get(yl));
    v = add_i64(v, neg_i64(ivol_get(yl + P.nx)));
    ivol_set(col, v);
    cols[col] = i64_to_f32(v) * quantum_height();
}

fn solde_get(face: u32) -> vec2<u32> {
    return vec2<u32>(isolde[2u * face], isolde[2u * face + 1u]);
}

fn solde_add(face: u32, v: vec2<u32>) {
    let r = add_i64(solde_get(face), v);
    isolde[2u * face] = r.x;
    isolde[2u * face + 1u] = r.y;
}

// ---------------------------------------------------------------------------------------------------------------------
// **Le compactage stable** (S418) : les particules marquées (`x.w ≠ 0`) sortent, les vivantes gardent leur ordre. Compte par
// groupe de 256, préfixe des groupes sur un fil, rangement dans le tampon de passage, recopie, nouveau `n`.

var<workgroup> alive_mem: array<u32, 256>;

fn is_alive(k: u32) -> bool {
    return k < np() && px[k].w == 0.0;
}

@compute @workgroup_size(256)
fn compact_count(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                 @builtin(workgroup_id) w: vec3<u32>) {
    alive_mem[l.x] = select(0u, 1u, is_alive(g.x));
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if l.x < s {
            alive_mem[l.x] = alive_mem[l.x] + alive_mem[l.x + s];
        }
        workgroupBarrier();
    }
    if l.x == 0u {
        pblk[w.x] = alive_mem[0];
    }
}

// Le préfixe des groupes ; le total va à `pblk[groupes]`. `groups` : la capacité sur 256, passée par `P.q2`.
@compute @workgroup_size(1)
fn compact_scan() {
    let groups = u32(P.q2);
    var s = 0u;
    for (var b = 0u; b < groups; b = b + 1u) {
        let v = pblk[b];
        pblk[b] = s;
        s = s + v;
    }
    pblk[groups] = s;
}

@compute @workgroup_size(256)
fn compact_scatter(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                   @builtin(workgroup_id) w: vec3<u32>) {
    let k = g.x;
    let own = select(0u, 1u, is_alive(k));
    alive_mem[l.x] = own;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var add = 0u;
        if l.x >= s {
            add = alive_mem[l.x - s];
        }
        workgroupBarrier();
        alive_mem[l.x] = alive_mem[l.x] + add;
        workgroupBarrier();
    }
    if own == 1u {
        let dst = pblk[w.x] + alive_mem[l.x] - 1u;
        pscratch[5u * dst] = vec4<f32>(px[k].xyz, 0.0);
        pscratch[5u * dst + 1u] = pv[k];
        pscratch[5u * dst + 2u] = pc[3u * k];
        pscratch[5u * dst + 3u] = pc[3u * k + 1u];
        pscratch[5u * dst + 4u] = pc[3u * k + 2u];
    }
}

@compute @workgroup_size(128)
fn compact_copy(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    let groups = u32(P.q2);
    if k >= pblk[groups] {
        return;
    }
    px[k] = pscratch[5u * k];
    pv[k] = pscratch[5u * k + 1u];
    pc[3u * k] = pscratch[5u * k + 2u];
    pc[3u * k + 1u] = pscratch[5u * k + 3u];
    pc[3u * k + 2u] = pscratch[5u * k + 4u];
}

@compute @workgroup_size(1)
fn compact_finish() {
    atomicStore(&pcount[0], pblk[u32(P.q2)]);
}

// ---------------------------------------------------------------------------------------------------------------------
// **L'échange à la frontière** (`columns_exchange`, S399–S407 ; C7c-2). La référence est séquentielle : elle visite les
// particules en montant et retire par **échange avec la dernière** ; le mélange des vitesses aux faces dépend de l'ordre. La carte
// reproduit ses tableaux **indice pour indice** : les marques se posent en parallèle, les gestes (rares) se font sur un fil, dans
// l'ordre de la référence.

const COUNT_N: u32 = 0u;
const COUNT_START: u32 = 1u;
const COUNT_LIST: u32 = 2u;
const COUNT_ABSORBED: u32 = 3u;
const COUNT_REMOVED: u32 = 4u;
const COUNT_POSED: u32 = 5u;
const COUNT_REFUSED: u32 = 6u;
// S420 : la bascule refusée (capacité), une bande existe-t-elle (le masque change avec la bascule).
const COUNT_SWITCH_REFUSED: u32 = 7u;
const COUNT_BAND: u32 = 8u;
// S420 : le prédicat de la liste ordonnée — 0 : absorbée (échange) ; 1 : dans une colonne convertie (bascule).
const COUNT_LIST_MODE: u32 = 9u;
// Un volume de particule, en quanta : 2²⁴.
const VP_QUANTA: u32 = 16777216u;

@compute @workgroup_size(1)
fn exchange_begin() {
    atomicStore(&pcount[COUNT_START], np());
    atomicStore(&pcount[COUNT_LIST], 0u);
}

// (1) Les particules entrées dans une colonne de la zone, listées (dans le désordre des atomiques ; le fil les trie).
@compute @workgroup_size(128)
fn absorb_mark(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= np() || P.has_columns == 0.0 {
        return;
    }
    let m = cell_of(px[k].xyz);
    let col = m.y * P.nx + m.x;
    // S413 : une particule passée sous le fond de sa colonne de la bande est absorbée aussi.
    if cmask[col] != 0u || px[k].z < floor_of(col) {
        let slot = atomicAdd(&pcount[COUNT_LIST], 1u);
        plist[slot] = k;
    }
}

// `zone_face` : une face de la zone — ses deux colonnes dans la zone (sa seule, au bord ; une face `w`, sa colonne).
fn zone_face(axis: u32, idx: vec3<u32>) -> bool {
    let i = i32(idx.x);
    let j = i32(idx.y);
    if axis == 0u {
        let a = in_zone(i - 1, j);
        let b = in_zone(i, j);
        return (a || idx.x == 0u) && (b || idx.x == P.nx) && (a || b);
    }
    if axis == 1u {
        let a = in_zone(i, j - 1);
        let b = in_zone(i, j);
        return (a || idx.y == 0u) && (b || idx.y == P.ny) && (a || b);
    }
    return in_zone(i, j);
}

fn copy_particle(src: u32, dst: u32) {
    px[dst] = px[src];
    pv[dst] = pv[src];
    pc[3u * dst] = pc[3u * src];
    pc[3u * dst + 1u] = pc[3u * src + 1u];
    pc[3u * dst + 2u] = pc[3u * src + 2u];
}

// Une particule absorbée : sa quantité de mouvement rendue aux faces de la zone (S406), son volume au solde de la face-maille
// de frontière la plus proche — ou à `η`, loin de toute bande.
fn absorb_one(k: u32) {
    let p = px[k].xyz;
    let v = pv[k].xyz;
    let cm = cell_of(p);
    let col_p = cm.y * P.nx + cm.x;
    if cmask[col_p] == 0u {
        // S413 : sous le fond, la particule entre dans le contenant plein ; sa quantité de mouvement va aux faces à la grille qui
        // l'entourent (S406), son volume au solde vertical de sa colonne.
        for (var axis = 0u; axis < 3u; axis = axis + 1u) {
            let l = lerp_of(p, axis);
            let va = select(select(v.z, v.y, axis == 1u), v.x, axis == 0u);
            for (var m = 0u; m < 8u; m = m + 1u) {
                let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
                let idx = select(l.base, l.next, sel);
                let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
                let wt = wa.x * wa.y * wa.z;
                if wt == 0.0 || !floor_face(axis, idx) {
                    continue;
                }
                let f = face_global(axis, idx);
                faces[f] = faces[f] + wt * (va - faces[f]) / 8.0;
            }
        }
        solde_add(solde_w_index(col_p), vec2<u32>(VP_QUANTA, 0u));
        atomicAdd(&pcount[COUNT_ABSORBED], 1u);
        return;
    }
    for (var axis = 0u; axis < 3u; axis = axis + 1u) {
        let l = lerp_of(p, axis);
        let va = select(select(v.z, v.y, axis == 1u), v.x, axis == 0u);
        for (var m = 0u; m < 8u; m = m + 1u) {
            let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
            let idx = select(l.base, l.next, sel);
            let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
            let wt = wa.x * wa.y * wa.z;
            if wt == 0.0 || !zone_face(axis, idx) {
                continue;
            }
            let f = face_global(axis, idx);
            faces[f] = faces[f] + wt * (va - faces[f]) / 8.0;
        }
    }
    let c = cell_of(p);
    var best_d = 0.0;
    var best_face = 0xffffffffu;
    for (var dir = 0u; dir < 4u; dir = dir + 1u) {
        var di = 0;
        var dj = 0;
        if dir == 0u { di = -1; } else if dir == 1u { di = 1; } else if dir == 2u { dj = -1; } else { dj = 1; }
        let a = i32(c.x) + di;
        let b = i32(c.y) + dj;
        if a < 0 || b < 0 || a >= i32(P.nx) || b >= i32(P.ny) || in_zone(a, b) {
            continue;
        }
        let fi = c.x + select(0u, 1u, di > 0);
        let fj = c.y + select(0u, 1u, dj > 0);
        var d = 0.0;
        var face = 0u;
        if dir < 2u {
            d = abs(p.x - f32(fi) * P.dx);
            face = (c.z * P.ny + c.y) * (P.nx + 1u) + fi;
        } else {
            d = abs(p.y - f32(fj) * P.dx);
            face = P.nu + (c.z * (P.ny + 1u) + fj) * P.nx + c.x;
        }
        if best_face == 0xffffffffu || d < best_d {
            best_d = d;
            best_face = face;
        }
    }
    if best_face != 0xffffffffu {
        solde_add(best_face, vec2<u32>(VP_QUANTA, 0u));
    } else {
        let col = c.y * P.nx + c.x;
        let vol = add_i64(ivol_get(col), vec2<u32>(VP_QUANTA, 0u));
        ivol_set(col, vol);
        cols[col] = i64_to_f32(vol) * quantum_height();
    }
    atomicAdd(&pcount[COUNT_ABSORBED], 1u);
}

// (1) sur un fil : la liste triée, puis la visite de la référence — en montant ; une absorbée est remplacée par la dernière,
// qui est examinée aussitôt.
@compute @workgroup_size(1)
fn absorb_serial() {
    visit_remove(0u);
}

// La liste triée, puis la visite de la référence — en montant ; une particule listée est remplacée par la dernière, examinée
// aussitôt. `mode` 0 : absorbée (`absorb_one`) ; 1 : retirée par la bascule (comptée). Rend le nombre de retirées.
fn visit_remove(mode: u32) -> u32 {
    var removed = 0u;
    // S420 : la liste arrive triée (`list_count`, `list_scatter`) — un tri par insertion sur un fil dépassait le délai de garde du
    // pilote pour la bascule initiale (120 000 particules).
    let len = atomicLoad(&pcount[COUNT_LIST]);
    var n_cur = np();
    var front = 0u;
    var back = i32(len) - 1;
    loop {
        if front >= len || plist[front] >= n_cur || i32(front) > back {
            break;
        }
        let a = plist[front];
        front = front + 1u;
        if mode == 0u {
            absorb_one(a);
        } else if mode == 2u {
            raise_one(a);
        }
        removed = removed + 1u;
        loop {
            let last = n_cur - 1u;
            n_cur = last;
            if last == a {
                break;
            }
            copy_particle(last, a);
            if back >= i32(front) && plist[u32(back)] == last {
                back = back - 1;
                if mode == 0u {
                    absorb_one(a);
                } else if mode == 2u {
                    raise_one(a);
                }
                removed = removed + 1u;
                continue;
            }
            break;
        }
    }
    atomicStore(&pcount[COUNT_N], n_cur);
    return removed;
}

fn solde_le_minus_vp(face: u32) -> bool {
    // solde ≤ −vp  ⇔  −solde ≥ vp : sur deux mots signés.
    let s = solde_get(face);
    let hi = bitcast<i32>(s.y);
    if hi >= 0 {
        return false;
    }
    let m = neg_i64(s);
    return m.y > 0u || m.x >= VP_QUANTA;
}

fn solde_ge_vp(face: u32) -> bool {
    let s = solde_get(face);
    let hi = bitcast<i32>(s.y);
    return hi > 0 || (hi == 0 && s.x >= VP_QUANTA);
}

// La vitesse et `C` que la grille donne en un point (`grid_affine`).
fn grid_affine_at(p: vec3<f32>, k: u32) {
    let a = interp(0u, p);
    let b = interp(1u, p);
    let c = interp(2u, p);
    pv[k] = vec4<f32>(a.x, b.x, c.x, 0.0);
    pc[3u * k] = vec4<f32>(a.yzw, 0.0);
    pc[3u * k + 1u] = vec4<f32>(b.yzw, 0.0);
    pc[3u * k + 2u] = vec4<f32>(c.yzw, 0.0);
}

// (2) et (3) sur un fil : chaque face-maille de frontière règle son solde, dans l'ordre de la référence — retrait de la
// particule de la bande la plus proche de la face (à cette profondeur d'abord), pose contre la face au sous-réseau le plus libre ;
// puis les marquées retirées, du plus grand indice au plus petit, par échange avec la dernière.
@compute @workgroup_size(1)
fn exchange_serial() {
    if P.has_columns == 0.0 {
        return;
    }
    var n = np();
    let first_new = n;
    var marked = 0u;
    let vp_depth = P.dx / 16.0;
    // S421 : seules les faces-mailles **actives** (frontière, solde au-delà d'une particule), dans l'ordre de la référence — un
    // solde ne change que par ses propres gestes : la liste, faite avant, est exacte.
    let n_active = atomicLoad(&pcount[COUNT_FLIST]);
    for (var e_k = 0u; e_k < n_active; e_k = e_k + 1u) {
        let e = flist[e_k];
        let fc = face_cell_of(e);
        let axis = fc.x;
        let l = fc.y;
        let fj = fc.z;
        let fi = fc.w;
        var lo = vec2<u32>(0u, 0u);
        let hi = vec2<u32>(fi, fj);
        if axis == 0u {
            lo = vec2<u32>(fi - 1u, fj);
        } else {
            lo = vec2<u32>(fi, fj - 1u);
        }
        let zl = grid_at(i32(lo.x), i32(lo.y), l);
        var face = e;
        var plane = 0.0;
        if axis == 0u {
            plane = f32(fi) * P.dx;
        } else {
            plane = f32(fj) * P.dx;
        }
                    let band = select(lo, hi, zl);
                    let side = select(-1.0, 1.0, zl);
                    // (2) Retirer ce qui est dû.
                    loop {
                        if !solde_le_minus_vp(face) {
                            break;
                        }
                        var found = false;
                        var pick = 0u;
                        var pick_d = 0.0;
                        for (var dk = 0u; dk < P.nz; dk = dk + 1u) {
                            for (var side_k = 0u; side_k < 2u; side_k = side_k + 1u) {
                                if dk == 0u && side_k == 1u {
                                    continue;
                                }
                                var k = i32(l) - i32(dk);
                                if side_k == 1u {
                                    k = i32(l) + i32(dk);
                                }
                                if k < 0 || k >= i32(P.nz) {
                                    continue;
                                }
                                let cell = cell_index(band.x, band.y, u32(k));
                                for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
                                    let m = order[s];
                                    if px[m].w != 0.0 {
                                        continue;
                                    }
                                    let p = px[m].xyz;
                                    let d = abs(select(p.y, p.x, axis == 0u) - plane);
                                    // (dk, d) lexicographique : dk ne décroît pas dans la boucle, seul `d` départage.
                                    if !found || d < pick_d {
                                        found = true;
                                        pick = m;
                                        pick_d = d;
                                    }
                                }
                            }
                            if found {
                                break;
                            }
                        }
                        if !found {
                            break;
                        }
                        px[pick].w = 1.0;
                        plist[marked] = pick;
                        marked = marked + 1u;
                        atomicAdd(&pcount[COUNT_REMOVED], 1u);
                        solde_add(face, vec2<u32>(VP_QUANTA, 0u));
                    }
                    // (3) Poser ce qui est reçu, à la face (S407).
                    loop {
                        if !solde_ge_vp(face) {
                            break;
                        }
                        if n >= arrayLength(&plist) {
                            atomicAdd(&pcount[COUNT_REFUSED], 1u);
                            break;
                        }
                        let offset = plane + side * vp_depth;
                        let cell = cell_index(band.x, band.y, l);
                        // S421 : les quatre emplacements en un seul parcours (le minimum ne dépend pas de l'ordre).
                        var c0 = vec3<f32>(offset, (f32(band.y) + 0.25) * P.dx, (f32(l) + 0.25) * P.dx);
                        var c1 = vec3<f32>(offset, (f32(band.y) + 0.75) * P.dx, (f32(l) + 0.25) * P.dx);
                        var c2 = vec3<f32>(offset, (f32(band.y) + 0.25) * P.dx, (f32(l) + 0.75) * P.dx);
                        var c3 = vec3<f32>(offset, (f32(band.y) + 0.75) * P.dx, (f32(l) + 0.75) * P.dx);
                        if axis == 1u {
                            c0 = vec3<f32>((f32(band.x) + 0.25) * P.dx, offset, (f32(l) + 0.25) * P.dx);
                            c1 = vec3<f32>((f32(band.x) + 0.75) * P.dx, offset, (f32(l) + 0.25) * P.dx);
                            c2 = vec3<f32>((f32(band.x) + 0.25) * P.dx, offset, (f32(l) + 0.75) * P.dx);
                            c3 = vec3<f32>((f32(band.x) + 0.75) * P.dx, offset, (f32(l) + 0.75) * P.dx);
                        }
                        let best_p = most_free(c0, c1, c2, c3, cell, first_new, n);
                        px[n] = vec4<f32>(best_p, 0.0);
                        grid_affine_at(best_p, n);
                        n = n + 1u;
                        atomicAdd(&pcount[COUNT_POSED], 1u);
                        solde_add(face, neg_i64(vec2<u32>(VP_QUANTA, 0u)));
                    }
    }
    // (4) S413 — le solde vertical de chaque colonne à fond : dû, la particule la plus basse au-dessus du fond est retirée ;
    // reçu, une particule est posée à la face du fond (`dx/16`), au sous-réseau le plus libre.
    if P.floors != 0.0 {
        for (var j = 0u; j < P.ny; j = j + 1u) {
            for (var i = 0u; i < P.nx; i = i + 1u) {
                let col = j * P.nx + i;
                let fond = floor_of(col);
                if fond <= 0.0 {
                    continue;
                }
                let kf = min(u32(floor(fond / P.dx + 0.5)), P.nz - 1u);
                let sw = solde_w_index(col);
                loop {
                    if !solde_le_minus_vp(sw) {
                        break;
                    }
                    var found = false;
                    var pick = 0u;
                    var pick_z = 0.0;
                    for (var l = kf; l < P.nz; l = l + 1u) {
                        let cell = cell_index(i, j, l);
                        for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
                            let m = order[s];
                            if px[m].w == 0.0 && (!found || px[m].z < pick_z) {
                                found = true;
                                pick = m;
                                pick_z = px[m].z;
                            }
                        }
                        if found {
                            break;
                        }
                    }
                    if !found {
                        break;
                    }
                    px[pick].w = 1.0;
                    plist[marked] = pick;
                    marked = marked + 1u;
                    atomicAdd(&pcount[COUNT_REMOVED], 1u);
                    solde_add(sw, vec2<u32>(VP_QUANTA, 0u));
                }
                loop {
                    if !solde_ge_vp(sw) {
                        break;
                    }
                    if n >= arrayLength(&plist) {
                        atomicAdd(&pcount[COUNT_REFUSED], 1u);
                        break;
                    }
                    let z = fond + P.dx / 16.0;
                    let cell = cell_index(i, j, kf);
                    let best_q = most_free(
                        vec3<f32>((f32(i) + 0.25) * P.dx, (f32(j) + 0.25) * P.dx, z),
                        vec3<f32>((f32(i) + 0.75) * P.dx, (f32(j) + 0.25) * P.dx, z),
                        vec3<f32>((f32(i) + 0.25) * P.dx, (f32(j) + 0.75) * P.dx, z),
                        vec3<f32>((f32(i) + 0.75) * P.dx, (f32(j) + 0.75) * P.dx, z),
                        cell, first_new, n);
                    px[n] = vec4<f32>(best_q, 0.0);
                    grid_affine_at(best_q, n);
                    n = n + 1u;
                    atomicAdd(&pcount[COUNT_POSED], 1u);
                    solde_add(sw, neg_i64(vec2<u32>(VP_QUANTA, 0u)));
                }
            }
        }
    }
    // Les marquées, du plus grand indice au plus petit (tri décroissant de la liste), par échange avec la dernière.
    for (var s = 1u; s < marked; s = s + 1u) {
        let v = plist[s];
        var t = s;
        loop {
            if t == 0u || plist[t - 1u] >= v {
                break;
            }
            plist[t] = plist[t - 1u];
            t = t - 1u;
        }
        plist[t] = v;
    }
    for (var s = 0u; s < marked; s = s + 1u) {
        let m = plist[s];
        let last = n - 1u;
        copy_particle(last, m);
        n = last;
    }
    atomicStore(&pcount[COUNT_N], n);
}

// ---------------------------------------------------------------------------------------------------------------------
// **Le fond de la bande** (S413, C7c-3) : par colonne de particules, `cols[2C + 32 + col]`, sur une face de maille ; zéro dans la zone.

fn floor_of(col: u32) -> f32 {
    if P.floors == 0.0 || cmask[col] != 0u {
        return 0.0;
    }
    return cols[2u * P.nx * P.ny + 32u + col];
}

// Une maille sous le fond d'une colonne de la bande (son centre).
fn below_floor(i: i32, j: i32, k: u32) -> bool {
    if P.floors == 0.0 || i < 0 || j < 0 || i >= i32(P.nx) || j >= i32(P.ny) {
        return false;
    }
    let col = u32(j) * P.nx + u32(i);
    return cmask[col] == 0u && (f32(k) + 0.5) * P.dx < floor_of(col);
}

// `grid_cell` : une maille à la grille — dans la zone, ou sous le fond.
fn grid_at(i: i32, j: i32, k: u32) -> bool {
    return in_zone(i, j) || below_floor(i, j, k);
}

fn solde_w_index(col: u32) -> u32 {
    return P.nu + P.nv + col;
}

// S419 — le solde vertical de chaque colonne à fond : la somme des contributions de ses quatre faces (haute de la face à sa
// gauche, basse de celle à sa droite, de même en `y`).
@compute @workgroup_size(128)
fn floor_update(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    let ncol = P.nx * P.ny;
    if col >= ncol || P.floors == 0.0 || cmask[col] != 0u {
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    let fx = (P.nx + 1u) * P.ny;
    let fx_fy = fx + P.nx * (P.ny + 1u);
    let base = ncol + fx_fy;
    let xl = j * (P.nx + 1u) + i;
    let yl = fx + j * P.nx + i;
    var v = vec2<u32>(0u, 0u);
    v = add_i64(v, ivol_get(base + 2u * xl + 1u));
    v = add_i64(v, ivol_get(base + 2u * (xl + 1u)));
    v = add_i64(v, ivol_get(base + 2u * yl + 1u));
    v = add_i64(v, ivol_get(base + 2u * (yl + P.nx)));
    solde_add(solde_w_index(col), v);
}

// `floor_face` (S413) : une face à la grille autour d'une particule absorbée sous le fond — une face `u` ou `v` dont l'une des
// deux mailles est à la grille, une face `w` au-dessus d'une maille à la grille (ou dans une colonne de la zone).
fn floor_face(axis: u32, idx: vec3<u32>) -> bool {
    let i = i32(idx.x);
    let j = i32(idx.y);
    let k = idx.z;
    if axis == 0u {
        return grid_at(i - 1, j, k) || grid_at(i, j, k);
    }
    if axis == 1u {
        return grid_at(i, j - 1, k) || grid_at(i, j, k);
    }
    return in_zone(i, j) || (k >= 1u && grid_at(i, j, k - 1u));
}

// S419 — `|d|²` évalué comme la référence, `(x² + y²) + z²` avec **trois produits arrondis** : un choix par le maximum (le
// sous-réseau le plus libre) se joue sur des distances presque égales, et un `mad` fusionné par FXC (Rust n'en fait pas) le
// tranche autrement. Le passage par les bits empêche la contraction.
fn square_sum(d: vec3<f32>) -> f32 {
    let x = bitcast<f32>(bitcast<u32>(d.x * d.x));
    let y = bitcast<f32>(bitcast<u32>(d.y * d.y));
    let z = bitcast<f32>(bitcast<u32>(d.z * d.z));
    return (x + y) + z;
}

// ---------------------------------------------------------------------------------------------------------------------
// **La décision de la bascule** (`ColumnsSwitch::decide`, S408–S410 ; C7c-4), sur la surface rafraîchie (tri et `reconstruct`).

const SW_AT: u32 = 0u;
const SW_NEED: u32 = 1u;
const SW_SPREAD: u32 = 2u;
const SW_KEEP: u32 = 3u;
const SW_HEIGHT: u32 = 4u;
const SW_REQUEST: u32 = 5u;
const SW_FLOOR: u32 = 6u;
const NEVER: u32 = 0xffffffffu;

fn sw(which: u32, col: u32) -> u32 {
    return which * P.nx * P.ny + col;
}

fn nan_f32() -> f32 {
    return bitcast<f32>(0x7fc00000u);
}

fn is_finite_f32(x: f32) -> bool {
    return (bitcast<u32>(x) & 0x7f800000u) != 0x7f800000u;
}

// `convertible_height` : un seul segment d'eau posé sur le fond, aucune maille solide, mailles occupées d'un seul tenant depuis le
// fond ; la hauteur à l'iso-zéro. NaN sinon.
fn convertible_height(i: u32, j: u32) -> f32 {
    var filled = 0u;
    var run = true;
    var top_count = 0u;
    var neg_run = true;
    for (var k = 0u; k < P.nz; k = k + 1u) {
        let c = cell_index(i, j, k);
        if label[c] == SOLID {
            return nan_f32();
        }
        let occupied = start[c + 1u] > start[c] || grid_at(i32(i), i32(j), k);
        if run && occupied {
            filled = filled + 1u;
        } else {
            run = false;
            if occupied {
                return nan_f32();
            }
        }
        let neg = cellf[field(F_PHI, c)] < 0.0;
        if neg_run && neg {
            top_count = top_count + 1u;
        } else {
            neg_run = false;
            if neg {
                return nan_f32();
            }
        }
    }
    if !(cellf[field(F_PHI, cell_index(i, j, 0u))] < 0.0) || filled == 0u || top_count >= P.nz {
        return nan_f32();
    }
    let top = top_count - 1u;
    let a = cellf[field(F_PHI, cell_index(i, j, top))];
    let b = cellf[field(F_PHI, cell_index(i, j, top + 1u))];
    return (f32(top) + 0.5) * P.dx + P.dx * a / (a - b);
}

// (1) les hauteurs et les colonnes non convertibles ; (2) le corps : le segment de l'horizon, élargi de la marge.
@compute @workgroup_size(128)
fn switch_need(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    if col >= P.nx * P.ny {
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    swb[sw(SW_KEEP, col)] = 0u;
    var need = 0u;
    var h = 0.0;
    if cmask[col] != 0u {
        h = cols[col];
    } else {
        h = convertible_height(i, j);
        if !is_finite_f32(h) {
            need = 1u;
        }
    }
    if P.has_body != 0.0 {
        let d = vec3<f32>(P.bvx, P.bvy, P.bvz) * P.s_horizon;
        let reach = P.br + P.s_margin;
        let low = min(P.bcz, P.bcz + d.z) - P.br;
        let len2 = d.x * d.x + d.y * d.y;
        let x = (f32(i) + 0.5) * P.dx - P.bcx;
        let y = (f32(j) + 0.5) * P.dx - P.bcy;
        var s = 0.0;
        if len2 > 0.0 {
            s = clamp((x * d.x + y * d.y) / len2, 0.0, 1.0);
        }
        let ex = x - s * d.x;
        let ey = y - s * d.y;
        var surface = f32(P.nz) * P.dx;
        if is_finite_f32(h) {
            surface = h;
        }
        if ex * ex + ey * ey <= reach * reach && low <= surface + P.s_margin {
            need = 1u;
        }
    }
    swb[sw(SW_NEED, col)] = need;
    swb[sw(SW_HEIGHT, col)] = bitcast<u32>(h);
}

fn height_at(x: i32, y: i32) -> f32 {
    if x < 0 || y < 0 || x >= i32(P.nx) || y >= i32(P.ny) {
        return nan_f32();
    }
    return bitcast<f32>(swb[sw(SW_HEIGHT, u32(y) * P.nx + u32(x))]);
}

fn slope_of(here: f32, lo: f32, hi: f32) -> f32 {
    let l = is_finite_f32(lo);
    let h = is_finite_f32(hi);
    if l && h {
        return (hi - lo) / (2.0 * P.dx);
    }
    if l {
        return (here - lo) / P.dx;
    }
    if h {
        return (hi - here) / P.dx;
    }
    return 0.0;
}

// (3) la pente : différences centrées sur les hauteurs connues, décentrées à côté d'une inconnue.
@compute @workgroup_size(128)
fn switch_slope(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    if col >= P.nx * P.ny {
        return;
    }
    let i = i32(col % P.nx);
    let j = i32(col / P.nx);
    let here = bitcast<f32>(swb[sw(SW_HEIGHT, col)]);
    if swb[sw(SW_NEED, col)] != 0u || !is_finite_f32(here) {
        return;
    }
    let sx = slope_of(here, height_at(i - 1, j), height_at(i + 1, j));
    let sy = slope_of(here, height_at(i, j - 1), height_at(i, j + 1));
    let s2 = sx * sx + sy * sy;
    if s2 > P.s_slope * P.s_slope {
        swb[sw(SW_NEED, col)] = 1u;
    } else if cmask[col] == 0u && P.s_release >= 0.0 && s2 > P.s_release * P.s_release {
        swb[sw(SW_KEEP, col)] = 1u;
    }
}

// (4) la dilatation de Chebyshev, séparable : d'abord en `x`.
@compute @workgroup_size(128)
fn switch_spread(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    if col >= P.nx * P.ny {
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    let r = P.s_dil;
    let lo = select(0u, i - r, i >= r);
    let hi = min(i + r + 1u, P.nx);
    var any = 0u;
    for (var a = lo; a < hi; a = a + 1u) {
        if swb[sw(SW_NEED, j * P.nx + a)] != 0u {
            any = 1u;
        }
    }
    swb[sw(SW_SPREAD, col)] = any;
}

// puis en `y`, et le maintien : une colonne repasse aux colonnes après `hold` sans être requise.
@compute @workgroup_size(128)
fn switch_request(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    if col >= P.nx * P.ny {
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    let r = P.s_dil;
    let lo = select(0u, j - r, j >= r);
    let hi = min(j + r + 1u, P.ny);
    var required = swb[sw(SW_KEEP, col)] != 0u;
    for (var b = lo; b < hi; b = b + 1u) {
        if swb[sw(SW_SPREAD, b * P.nx + i)] != 0u {
            required = true;
        }
    }
    if required {
        swb[sw(SW_AT, col)] = P.s_now;
    }
    let at = swb[sw(SW_AT, col)];
    let band = required || (at != NEVER && P.s_now - at < P.s_hold);
    swb[sw(SW_REQUEST, col)] = select(1u, 0u, band);
}

// ---------------------------------------------------------------------------------------------------------------------
// **La bascule à masse exacte** (`apply_columns_mask`, S408, S414 ; C7c-4), en quanta, sur un fil dans l'ordre de la référence.

fn reserve_index() -> u32 {
    return P.nu + P.nv + P.nx * P.ny;
}

fn converted(col: u32) -> bool {
    return cmask[col] == 0u && swb[sw(SW_REQUEST, col)] != 0u && is_finite_f32(bitcast<f32>(swb[sw(SW_HEIGHT, col)]));
}

@compute @workgroup_size(1)
fn switch_begin() {
    atomicStore(&pcount[COUNT_LIST], 0u);
    atomicStore(&pcount[COUNT_SWITCH_REFUSED], 0u);
}

// Les particules des colonnes converties, listées.
@compute @workgroup_size(128)
fn convert_mark(@builtin(global_invocation_id) g: vec3<u32>) {
    let k = g.x;
    if k >= np() {
        return;
    }
    let m = cell_of(px[k].xyz);
    if converted(m.y * P.nx + m.x) {
        let slot = atomicAdd(&pcount[COUNT_LIST], 1u);
        plist[slot] = k;
    }
}

// Ce qu'ensemence une colonne de hauteur `h` : sous-couches pleines, puis la dernière au plus près (0 à 4).
fn plan_full(h: f32) -> u32 {
    let half = 0.5 * P.dx;
    return min(u32(max(floor(h / half), 0.0)), 2u * P.nz);
}

fn plan_last(h: f32, full: u32) -> u32 {
    if full >= 2u * P.nz {
        return 0u;
    }
    let half = 0.5 * P.dx;
    let rest = h - f32(full) * half;
    return u32(clamp(floor(rest / half * 4.0 + 0.5), 0.0, 4.0));
}

fn column_height(col: u32) -> f32 {
    return i64_to_f32(ivol_get(col)) * quantum_height();
}

fn i64_is_zero(a: vec2<u32>) -> bool {
    return a.x == 0u && a.y == 0u;
}

@compute @workgroup_size(1)
fn switch_apply() {
    let ncol = P.nx * P.ny;
    // (0) La capacité : l'ensemencement des colonnes qui repassent aux particules.
    var needed = 0u;
    for (var col = 0u; col < ncol; col = col + 1u) {
        if cmask[col] != 0u && swb[sw(SW_REQUEST, col)] == 0u {
            let h = column_height(col);
            let full = plan_full(h);
            needed = needed + 4u * full + plan_last(h, full);
        }
    }
    if np() + needed > arrayLength(&plist) {
        atomicStore(&pcount[COUNT_SWITCH_REFUSED], 1u);
        return;
    }
    // (1) Particules → colonnes : les particules des converties retirées (l'ordre de la référence), leur volume compté.
    let removed = visit_remove(1u);
    var total = vec2<u32>(0u, 0u);
    for (var r = 0u; r < removed; r = r + 1u) {
        total = add_i64(total, vec2<u32>(VP_QUANTA, 0u));
    }
    var count = 0u;
    var geo_sum = vec2<u32>(0u, 0u);
    for (var col = 0u; col < ncol; col = col + 1u) {
        if !converted(col) {
            continue;
        }
        // S414 : l'eau sous le fond et le solde vertical comptent avec les particules ; le fond s'efface.
        let fond = floor_of(col);
        if fond > 0.0 {
            let cells = u32(floor(fond / P.dx + 0.5));
            for (var c = 0u; c < 8u * cells; c = c + 1u) {
                total = add_i64(total, vec2<u32>(VP_QUANTA, 0u));
            }
            total = add_i64(total, solde_get(solde_w_index(col)));
            cols[2u * ncol + 32u + col] = 0.0;
            let z = solde_w_index(col);
            isolde[2u * z] = 0u;
            isolde[2u * z + 1u] = 0u;
        }
        let h = bitcast<f32>(swb[sw(SW_HEIGHT, col)]);
        let gq = to_i64(h / quantum_height());
        ivol_set(col, gq);
        geo_sum = add_i64(geo_sum, gq);
        count = count + 1u;
    }
    if count > 0u {
        // Le niveau par la masse, à un quart de maille au plus (2²⁵ quanta) ; le reste à la réserve, exactement.
        let wanted = i64_to_f32(add_i64(total, neg_i64(geo_sum))) / f32(count);
        let shift = to_i64(clamp(wanted, -33554432.0, 33554432.0));
        var given = vec2<u32>(0u, 0u);
        for (var col = 0u; col < ncol; col = col + 1u) {
            if converted(col) {
                let v = add_i64(ivol_get(col), shift);
                ivol_set(col, v);
                cols[col] = i64_to_f32(v) * quantum_height();
                given = add_i64(given, v);
            }
        }
        solde_add(reserve_index(), add_i64(total, neg_i64(given)));
    }
    // (2) Colonnes → particules : ensemencées sous leur hauteur, en ordre de colonnes.
    var n = np();
    let offsets = array<vec2<f32>, 4>(vec2<f32>(0.25, 0.25), vec2<f32>(0.75, 0.75), vec2<f32>(0.75, 0.25), vec2<f32>(0.25, 0.75));
    for (var col = 0u; col < ncol; col = col + 1u) {
        if !(cmask[col] != 0u && swb[sw(SW_REQUEST, col)] == 0u) {
            continue;
        }
        let i = col % P.nx;
        let j = col / P.nx;
        let v0 = ivol_get(col);
        let h = i64_to_f32(v0) * quantum_height();
        let full = plan_full(h);
        let last = plan_last(h, full);
        var seeded = 0u;
        var subs = full;
        if last > 0u {
            subs = full + 1u;
        }
        for (var sub = 0u; sub < subs; sub = sub + 1u) {
            let z = (f32(sub) + 0.5) * (0.5 * P.dx);
            var cnt = 4u;
            if sub >= full {
                cnt = last;
            }
            for (var a = 0u; a < cnt; a = a + 1u) {
                let o = offsets[a];
                let p = vec3<f32>((f32(i) + o.x) * P.dx, (f32(j) + o.y) * P.dx, z);
                px[n] = vec4<f32>(p, 0.0);
                grid_affine_at(p, n);
                n = n + 1u;
                seeded = seeded + 1u;
            }
        }
        var rest = v0;
        for (var s = 0u; s < seeded; s = s + 1u) {
            rest = add_i64(rest, neg_i64(vec2<u32>(VP_QUANTA, 0u)));
        }
        solde_add(reserve_index(), rest);
        ivol_set(col, vec2<u32>(0u, 0u));
        cols[col] = 0.0;
    }
    atomicStore(&pcount[COUNT_N], n);
    // (3) Les soldes des faces qui cessent d'être frontière : à la réserve. Puis le masque.
    for (var axis = 0u; axis < 2u; axis = axis + 1u) {
        var fx = P.nx + 1u;
        var fy = P.ny;
        if axis == 1u {
            fx = P.nx;
            fy = P.ny + 1u;
        }
        for (var fj = 0u; fj < fy; fj = fj + 1u) {
            for (var fi = 0u; fi < fx; fi = fi + 1u) {
                var lo = 0u;
                var hi = 0u;
                if axis == 0u {
                    if fi == 0u || fi == P.nx {
                        continue;
                    }
                    lo = fj * P.nx + fi - 1u;
                    hi = fj * P.nx + fi;
                } else {
                    if fj == 0u || fj == P.ny {
                        continue;
                    }
                    lo = (fj - 1u) * P.nx + fi;
                    hi = fj * P.nx + fi;
                }
                let before = (cmask[lo] != 0u) != (cmask[hi] != 0u);
                let after = (new_mask(lo) != 0u) != (new_mask(hi) != 0u);
                if before && !after {
                    for (var l = 0u; l < P.nz; l = l + 1u) {
                        var face = 0u;
                        if axis == 0u {
                            face = (l * P.ny + fj) * (P.nx + 1u) + fi;
                        } else {
                            face = P.nu + (l * (P.ny + 1u) + fj) * P.nx + fi;
                        }
                        solde_add(reserve_index(), solde_get(face));
                        isolde[2u * face] = 0u;
                        isolde[2u * face + 1u] = 0u;
                    }
                }
            }
        }
    }
    var band = 0u;
    for (var col = 0u; col < ncol; col = col + 1u) {
        let m = new_mask(col);
        swb[sw(SW_SPREAD, col)] = m;
        if m == 0u {
            band = 1u;
        }
    }
    for (var col = 0u; col < ncol; col = col + 1u) {
        cmask[col] = swb[sw(SW_SPREAD, col)];
    }
    atomicStore(&pcount[COUNT_BAND], band);
}

// Le masque après la bascule : une convertie devient colonne ; une colonne demandée en particules le devient ; sinon l'ancien.
fn new_mask(col: u32) -> u32 {
    if converted(col) {
        return 1u;
    }
    if cmask[col] != 0u && swb[sw(SW_REQUEST, col)] == 0u {
        return 0u;
    }
    return cmask[col];
}

// **La réserve réglée** (`columns_settle_reserve`, S408) : à parts égales sur les faces-mailles de frontière mouillées.
fn settle_reserve() {
    let r = solde_get(reserve_index());
    if i64_is_zero(r) {
        return;
    }
    var count = 0u;
    for (var round_k = 0u; round_k < 2u; round_k = round_k + 1u) {
        var share = vec2<u32>(0u, 0u);
        if round_k == 1u {
            if count == 0u {
                return;
            }
            share = to_i64(i64_to_f32(r) / f32(count));
        }
        for (var axis = 0u; axis < 2u; axis = axis + 1u) {
            var fx = P.nx + 1u;
            var fy = P.ny;
            if axis == 1u {
                fx = P.nx;
                fy = P.ny + 1u;
            }
            for (var l = 0u; l < P.nz; l = l + 1u) {
                for (var fj = 0u; fj < fy; fj = fj + 1u) {
                    for (var fi = 0u; fi < fx; fi = fi + 1u) {
                        var lo = 0u;
                        var hi = 0u;
                        if axis == 0u {
                            if fi == 0u || fi == P.nx {
                                continue;
                            }
                            lo = fj * P.nx + fi - 1u;
                            hi = fj * P.nx + fi;
                        } else {
                            if fj == 0u || fj == P.ny {
                                continue;
                            }
                            lo = (fj - 1u) * P.nx + fi;
                            hi = fj * P.nx + fi;
                        }
                        let zl = cmask[lo] != 0u;
                        let zh = cmask[hi] != 0u;
                        if zl == zh {
                            continue;
                        }
                        let zone = select(hi, lo, zl);
                        if !((f32(l) + 0.5) * P.dx < cols[zone]) {
                            continue;
                        }
                        if round_k == 0u {
                            count = count + 1u;
                        } else {
                            var face = 0u;
                            if axis == 0u {
                                face = (l * P.ny + fj) * (P.nx + 1u) + fi;
                            } else {
                                face = P.nu + (l * (P.ny + 1u) + fj) * P.nx + fi;
                            }
                            solde_add(face, share);
                            solde_add(reserve_index(), neg_i64(share));
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// **La liste ordonnée** (S420) : les particules qui vérifient le prédicat du mode, par indice croissant — compte par groupe, préfixe
// des groupes (`compact_scan`), rangement par préfixe dans le groupe.

fn listed(k: u32) -> bool {
    if k >= np() {
        return false;
    }
    let m = cell_of(px[k].xyz);
    let col = m.y * P.nx + m.x;
    let mode = atomicLoad(&pcount[COUNT_LIST_MODE]);
    if mode == 0u {
        return P.has_columns != 0.0 && (cmask[col] != 0u || px[k].z < floor_of(col));
    }
    if mode == 2u {
        // S414 : sous le nouveau fond d'une colonne qui monte.
        let to = floor_cells_target(col);
        return cmask[col] == 0u && to > floor_cells_now(col) && px[k].z < f32(to) * P.dx;
    }
    return converted(col);
}

@compute @workgroup_size(1)
fn list_mode_absorb() {
    atomicStore(&pcount[COUNT_LIST_MODE], 0u);
}

@compute @workgroup_size(1)
fn list_mode_convert() {
    atomicStore(&pcount[COUNT_LIST_MODE], 1u);
}

@compute @workgroup_size(256)
fn list_count(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
              @builtin(workgroup_id) w: vec3<u32>) {
    alive_mem[l.x] = select(0u, 1u, listed(g.x));
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if l.x < s {
            alive_mem[l.x] = alive_mem[l.x] + alive_mem[l.x + s];
        }
        workgroupBarrier();
    }
    if l.x == 0u {
        pblk[w.x] = alive_mem[0];
    }
}

@compute @workgroup_size(256)
fn list_scatter(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                @builtin(workgroup_id) w: vec3<u32>) {
    let k = g.x;
    let own = select(0u, 1u, listed(k));
    alive_mem[l.x] = own;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var add = 0u;
        if l.x >= s {
            add = alive_mem[l.x - s];
        }
        workgroupBarrier();
        alive_mem[l.x] = alive_mem[l.x] + add;
        workgroupBarrier();
    }
    if own == 1u {
        plist[pblk[w.x] + alive_mem[l.x] - 1u] = k;
    }
}

@compute @workgroup_size(1)
fn list_finish() {
    atomicStore(&pcount[COUNT_LIST], pblk[u32(P.q2)]);
}

// ---------------------------------------------------------------------------------------------------------------------
// **Le fond placé et déplacé** (`place_floor`, `move_band_floor`, S414 ; C7c-4).

fn floor_cells_now(col: u32) -> u32 {
    return min(u32(floor(cols[2u * P.nx * P.ny + 32u + col] / P.dx + 0.5)), P.nz);
}

fn floor_cells_target(col: u32) -> u32 {
    return min(u32(floor(bitcast<f32>(swb[sw(SW_FLOOR, col)]) / P.dx + 0.5)), P.nz);
}

fn sat_sub(a: u32, b: u32) -> u32 {
    return select(0u, a - b, a >= b);
}

// La cible de chaque colonne de la bande : `k` sous la première maille non-eau depuis le bas (étiquettes de la surface
// rafraîchie) ; avec la prédiction, sous le point le plus bas que le corps atteindra ; l'hystérésis.
@compute @workgroup_size(128)
fn floor_place(@builtin(global_invocation_id) g: vec3<u32>) {
    let col = g.x;
    if col >= P.nx * P.ny || P.s_has_fcells == 0u {
        return;
    }
    if cmask[col] != 0u {
        swb[sw(SW_FLOOR, col)] = bitcast<u32>(0.0);
        return;
    }
    let i = col % P.nx;
    let j = col / P.nx;
    var low = P.nz;
    for (var l = 0u; l < P.nz; l = l + 1u) {
        if label[cell_index(i, j, l)] != WATER {
            low = l;
            break;
        }
    }
    var aim = sat_sub(low, P.s_fcells);
    if P.s_pred != 0u && P.has_body != 0.0 {
        let d = vec3<f32>(P.bvx, P.bvy, P.bvz) * P.s_horizon;
        let lowest = min(P.bcz, P.bcz + d.z) - P.br;
        let reach = P.br + P.s_margin;
        let x = (f32(i) + 0.5) * P.dx - P.bcx;
        let y = (f32(j) + 0.5) * P.dx - P.bcy;
        let len2 = d.x * d.x + d.y * d.y;
        var s = 0.0;
        if len2 > 0.0 {
            s = clamp((x * d.x + y * d.y) / len2, 0.0, 1.0);
        }
        let ex = x - s * d.x;
        let ey = y - s * d.y;
        if ex * ex + ey * ey <= reach * reach {
            aim = min(aim, sat_sub(u32(max(floor(lowest / P.dx), 0.0)), P.s_fcells));
        }
    }
    let now = floor_cells_now(col);
    var next = now;
    if aim < now || aim > now + P.s_fhyst {
        next = aim;
    }
    swb[sw(SW_FLOOR, col)] = bitcast<u32>(f32(next) * P.dx);
}

@compute @workgroup_size(1)
fn list_mode_raise() {
    atomicStore(&pcount[COUNT_LIST_MODE], 2u);
}

// Une particule sous le nouveau fond : absorbée, elle paie le solde vertical de sa colonne.
fn raise_one(k: u32) {
    let m = cell_of(px[k].xyz);
    solde_add(solde_w_index(m.y * P.nx + m.x), vec2<u32>(VP_QUANTA, 0u));
}

@compute @workgroup_size(1)
fn floor_move() {
    if P.s_has_fcells == 0u || atomicLoad(&pcount[COUNT_SWITCH_REFUSED]) != 0u {
        return;
    }
    let ncol = P.nx * P.ny;
    var needed = 0u;
    for (var col = 0u; col < ncol; col = col + 1u) {
        if cmask[col] == 0u {
            needed = needed + 8u * sat_sub(floor_cells_now(col), floor_cells_target(col));
        }
    }
    if np() + needed > arrayLength(&plist) {
        atomicStore(&pcount[COUNT_SWITCH_REFUSED], 2u);
        return;
    }
    // Remonter : les particules sous le nouveau fond, dans l'ordre de la référence.
    visit_remove(2u);
    var n = np();
    for (var col = 0u; col < ncol; col = col + 1u) {
        if cmask[col] != 0u {
            continue;
        }
        let i = col % P.nx;
        let j = col / P.nx;
        let was = floor_cells_now(col);
        let to = floor_cells_target(col);
        if to > was {
            // Les mailles prises au fond : pleines — l'écart au volume absorbé, au solde vertical.
            for (var c = 0u; c < 8u * (to - was); c = c + 1u) {
                solde_add(solde_w_index(col), neg_i64(vec2<u32>(VP_QUANTA, 0u)));
            }
        } else if to < was {
            // Descendre : les mailles libérées, ensemencées au réseau nominal.
            for (var l = to; l < was; l = l + 1u) {
                for (var a = 0u; a < 8u; a = a + 1u) {
                    let ax = f32(a % 2u);
                    let ay = f32((a / 2u) % 2u);
                    let az = f32(a / 4u);
                    let q = vec3<f32>((f32(i) + (ax + 0.5) * 0.5) * P.dx, (f32(j) + (ay + 0.5) * 0.5) * P.dx,
                        (f32(l) + (az + 0.5) * 0.5) * P.dx);
                    px[n] = vec4<f32>(q, 0.0);
                    grid_affine_at(q, n);
                    n = n + 1u;
                }
            }
        }
        cols[2u * ncol + 32u + col] = f32(to) * P.dx;
    }
    atomicStore(&pcount[COUNT_N], n);
}

// ---------------------------------------------------------------------------------------------------------------------
// **S421 — l'échange sans parcours séquentiel des faces-mailles** (C7e).

const COUNT_FLIST: u32 = 10u;
const COUNT_WET: u32 = 11u;
const COUNT_SHARE_LO: u32 = 12u;
const COUNT_SHARE_HI: u32 = 13u;

// Une face-maille par son indice de solde (faces `u`, puis `v` : l'ordre de la boucle de la référence) : (axe, rangée, fj, fi).
fn face_cell_of(e: u32) -> vec4<u32> {
    if e < P.nu {
        return vec4<u32>(0u, e / ((P.nx + 1u) * P.ny), (e / (P.nx + 1u)) % P.ny, e % (P.nx + 1u));
    }
    let f = e - P.nu;
    return vec4<u32>(1u, f / (P.nx * (P.ny + 1u)), (f / P.nx) % (P.ny + 1u), f % P.nx);
}

// La face-maille `e` est-elle intérieure, ses deux côtés (lo, hi) ?
fn face_sides(e: u32) -> vec4<u32> {
    let fc = face_cell_of(e);
    if fc.x == 0u {
        if fc.w == 0u || fc.w == P.nx {
            return vec4<u32>(0u, 0u, 0u, 0u);
        }
        return vec4<u32>(1u, fc.z * P.nx + fc.w - 1u, fc.z * P.nx + fc.w, fc.y);
    }
    if fc.z == 0u || fc.z == P.ny {
        return vec4<u32>(0u, 0u, 0u, 0u);
    }
    return vec4<u32>(1u, (fc.z - 1u) * P.nx + fc.w, fc.z * P.nx + fc.w, fc.y);
}

// Active pour l'échange : frontière lue maille par maille (la zone, ou sous le fond), et un geste à faire.
fn face_active(e: u32) -> bool {
    if e >= P.nu + P.nv {
        return false;
    }
    let sd = face_sides(e);
    if sd.x == 0u {
        return false;
    }
    let zl = grid_at(i32(sd.y % P.nx), i32(sd.y / P.nx), sd.w);
    let zh = grid_at(i32(sd.z % P.nx), i32(sd.z / P.nx), sd.w);
    return zl != zh && (solde_le_minus_vp(e) || solde_ge_vp(e));
}

// Mouillée pour la réserve (`columns_settle_reserve`) : frontière par le masque, l'eau de la colonne de la zone à cette rangée.
fn face_wet(e: u32) -> bool {
    if e >= P.nu + P.nv {
        return false;
    }
    let sd = face_sides(e);
    if sd.x == 0u {
        return false;
    }
    let zl = cmask[sd.y] != 0u;
    let zh = cmask[sd.z] != 0u;
    if zl == zh {
        return false;
    }
    let zone = select(sd.z, sd.y, zl);
    return (f32(sd.w) + 0.5) * P.dx < cols[zone];
}

@compute @workgroup_size(256)
fn flist_count(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
               @builtin(workgroup_id) w: vec3<u32>) {
    alive_mem[l.x] = select(0u, 1u, face_active(g.x));
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s / 2u) {
        if l.x < s {
            alive_mem[l.x] = alive_mem[l.x] + alive_mem[l.x + s];
        }
        workgroupBarrier();
    }
    if l.x == 0u {
        pblk[w.x] = alive_mem[0];
    }
}

@compute @workgroup_size(256)
fn flist_scatter(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_id) l: vec3<u32>,
                 @builtin(workgroup_id) w: vec3<u32>) {
    let e = g.x;
    let own = select(0u, 1u, face_active(e));
    alive_mem[l.x] = own;
    workgroupBarrier();
    for (var s = 1u; s < 256u; s = s * 2u) {
        var add = 0u;
        if l.x >= s {
            add = alive_mem[l.x - s];
        }
        workgroupBarrier();
        alive_mem[l.x] = alive_mem[l.x] + add;
        workgroupBarrier();
    }
    if own == 1u {
        flist[pblk[w.x] + alive_mem[l.x] - 1u] = e;
    }
}

@compute @workgroup_size(1)
fn flist_finish() {
    atomicStore(&pcount[COUNT_FLIST], pblk[u32(P.q2)]);
}

// La réserve : les faces-mailles mouillées comptées, la part (comme la référence : `réserve / compte`), puis ajoutée à chacune.
@compute @workgroup_size(128)
fn settle_count(@builtin(global_invocation_id) g: vec3<u32>) {
    if face_wet(g.x) && !i64_is_zero(solde_get(reserve_index())) {
        atomicAdd(&pcount[COUNT_WET], 1u);
    }
}

@compute @workgroup_size(1)
fn settle_reset() {
    atomicStore(&pcount[COUNT_WET], 0u);
    atomicStore(&pcount[COUNT_SHARE_LO], 0u);
    atomicStore(&pcount[COUNT_SHARE_HI], 0u);
}

fn mul_i64_u32(a: vec2<u32>, c: u32) -> vec2<u32> {
    // (hi·2³² + lo)·c, modulo 2⁶⁴ : `lo·c` sur 64 bits par moitiés de 16 bits.
    let lo0 = a.x & 0xffffu;
    let lo1 = a.x >> 16u;
    let c0 = c & 0xffffu;
    let c1 = c >> 16u;
    let p00 = lo0 * c0;
    let p01 = lo0 * c1;
    let p10 = lo1 * c0;
    let p11 = lo1 * c1;
    let mid = (p00 >> 16u) + (p01 & 0xffffu) + (p10 & 0xffffu);
    let low = (p00 & 0xffffu) | (mid << 16u);
    let high = p11 + (p01 >> 16u) + (p10 >> 16u) + (mid >> 16u);
    return vec2<u32>(low, high + a.y * c);
}

@compute @workgroup_size(1)
fn settle_share() {
    let r = solde_get(reserve_index());
    let count = atomicLoad(&pcount[COUNT_WET]);
    if i64_is_zero(r) || count == 0u {
        return;
    }
    let share = to_i64(i64_to_f32(r) / f32(count));
    atomicStore(&pcount[COUNT_SHARE_LO], share.x);
    atomicStore(&pcount[COUNT_SHARE_HI], share.y);
    solde_add(reserve_index(), neg_i64(mul_i64_u32(share, count)));
}

@compute @workgroup_size(128)
fn settle_add(@builtin(global_invocation_id) g: vec3<u32>) {
    let share = vec2<u32>(atomicLoad(&pcount[COUNT_SHARE_LO]), atomicLoad(&pcount[COUNT_SHARE_HI]));
    if i64_is_zero(share) || !face_wet(g.x) {
        return;
    }
    solde_add(g.x, share);
}

// S421 — **le sous-réseau le plus libre** des quatre emplacements : pour chacun, la distance carrée à la plus proche des particules
// vivantes de la maille et des particules posées depuis le début de l'échange ; le plus grand l'emporte, le premier en cas
// d'égalité (la règle de la référence). Un seul parcours pour les quatre : chaque particule lue une fois au lieu de quatre.
fn most_free(c0: vec3<f32>, c1: vec3<f32>, c2: vec3<f32>, c3: vec3<f32>, cell: u32, first_new: u32, n: u32) -> vec3<f32> {
    var near = vec4<f32>(3.4028234663852886e38);
    for (var s = start[cell]; s < start[cell + 1u]; s = s + 1u) {
        let m = order[s];
        if px[m].w == 0.0 {
            let p = px[m].xyz;
            near = min(near, vec4<f32>(square_sum(p - c0), square_sum(p - c1), square_sum(p - c2), square_sum(p - c3)));
        }
    }
    for (var q = first_new; q < n; q = q + 1u) {
        let p = px[q].xyz;
        near = min(near, vec4<f32>(square_sum(p - c0), square_sum(p - c1), square_sum(p - c2), square_sum(p - c3)));
    }
    var best = c0;
    var best_near = near.x;
    if near.y > best_near {
        best = c1;
        best_near = near.y;
    }
    if near.z > best_near {
        best = c2;
        best_near = near.z;
    }
    if near.w > best_near {
        best = c3;
    }
    return best;
}

// ---------------------------------------------------------------------------------------------------------------------
// **S421 — l'absorption en groupe** : la visite de la référence reste séquentielle (le fil 0), mais les vingt-quatre faces qu'une
// absorbée met à jour sont **distinctes** — huit nœuds par axe, trois familles — : vingt-quatre fils les traitent ensemble, sans
// changer le résultat. Le fil 0 fait le reste (solde, échange avec la dernière) et choisit la suivante.

const NONE: u32 = 0xffffffffu;
var<workgroup> ab_slot: u32;
var<workgroup> ab_front: u32;
var<workgroup> ab_back: i32;
var<workgroup> ab_n: u32;

// La particule suivante de la visite, par le front ; `NONE` à la fin.
fn ab_next_front(len: u32) -> u32 {
    if ab_front >= len || plist[ab_front] >= ab_n || i32(ab_front) > ab_back {
        return NONE;
    }
    let a = plist[ab_front];
    ab_front = ab_front + 1u;
    return a;
}

// Le mélange d'un nœud (`t` : axe `t / 8`, emplacement `t % 8`) pour la particule `k`.
fn absorb_blend_slot(k: u32, t: u32, below: bool) {
    let p = px[k].xyz;
    let v = pv[k].xyz;
    let axis = t / 8u;
    let m = t % 8u;
    let l = lerp_of(p, axis);
    let va = select(select(v.z, v.y, axis == 1u), v.x, axis == 0u);
    let sel = vec3<bool>((m & 1u) == 1u, ((m >> 1u) & 1u) == 1u, ((m >> 2u) & 1u) == 1u);
    let idx = select(l.base, l.next, sel);
    let wa = select(vec3<f32>(1.0) - l.frac, l.frac, sel);
    let wt = wa.x * wa.y * wa.z;
    if wt == 0.0 {
        return;
    }
    if below {
        if !floor_face(axis, idx) {
            return;
        }
    } else if !zone_face(axis, idx) {
        return;
    }
    let f = face_global(axis, idx);
    faces[f] = faces[f] + wt * (va - faces[f]) / 8.0;
}

// Ce que fait `absorb_one` hors du mélange : le solde (vertical sous le fond ; de la face de frontière la plus proche, ou le volume
// de la colonne, dans la zone) et le compte.
fn absorb_rest(k: u32) {
    let p = px[k].xyz;
    let c = cell_of(p);
    let col = c.y * P.nx + c.x;
    if cmask[col] == 0u {
        solde_add(solde_w_index(col), vec2<u32>(VP_QUANTA, 0u));
        atomicAdd(&pcount[COUNT_ABSORBED], 1u);
        return;
    }
    var best_d = 0.0;
    var best_face = NONE;
    for (var dir = 0u; dir < 4u; dir = dir + 1u) {
        var di = 0;
        var dj = 0;
        if dir == 0u { di = -1; } else if dir == 1u { di = 1; } else if dir == 2u { dj = -1; } else { dj = 1; }
        let a = i32(c.x) + di;
        let b = i32(c.y) + dj;
        if a < 0 || b < 0 || a >= i32(P.nx) || b >= i32(P.ny) || in_zone(a, b) {
            continue;
        }
        let fi = c.x + select(0u, 1u, di > 0);
        let fj = c.y + select(0u, 1u, dj > 0);
        var d = 0.0;
        var face = 0u;
        if dir < 2u {
            d = abs(p.x - f32(fi) * P.dx);
            face = (c.z * P.ny + c.y) * (P.nx + 1u) + fi;
        } else {
            d = abs(p.y - f32(fj) * P.dx);
            face = P.nu + (c.z * (P.ny + 1u) + fj) * P.nx + c.x;
        }
        if best_face == NONE || d < best_d {
            best_d = d;
            best_face = face;
        }
    }
    if best_face != NONE {
        solde_add(best_face, vec2<u32>(VP_QUANTA, 0u));
    } else {
        let vol = add_i64(ivol_get(col), vec2<u32>(VP_QUANTA, 0u));
        ivol_set(col, vol);
        cols[col] = i64_to_f32(vol) * quantum_height();
    }
    atomicAdd(&pcount[COUNT_ABSORBED], 1u);
}

@compute @workgroup_size(32)
fn absorb_group(@builtin(local_invocation_id) lid: vec3<u32>) {
    let t = lid.x;
    let len = atomicLoad(&pcount[COUNT_LIST]);
    if t == 0u {
        ab_front = 0u;
        ab_back = i32(len) - 1;
        ab_n = np();
        ab_slot = ab_next_front(len);
    }
    loop {
        let a = workgroupUniformLoad(&ab_slot);
        if a == NONE {
            break;
        }
        // Le mélange, vingt-quatre fils sur vingt-quatre faces distinctes.
        let c = cell_of(px[a].xyz);
        let below = cmask[c.y * P.nx + c.x] == 0u;
        if t < 24u {
            absorb_blend_slot(a, t, below);
        }
        storageBarrier();
        workgroupBarrier();
        if t == 0u {
            absorb_rest(a);
            // La référence : la dernière prend la place de l'absorbée ; absorbée elle-même, elle est traitée aussitôt.
            let last = ab_n - 1u;
            ab_n = last;
            var next = NONE;
            if last != a {
                copy_particle(last, a);
                if ab_back >= i32(ab_front) && plist[u32(ab_back)] == last {
                    ab_back = ab_back - 1;
                    next = a;
                } else {
                    next = ab_next_front(len);
                }
            } else {
                next = ab_next_front(len);
            }
            ab_slot = next;
        }
        storageBarrier();
        workgroupBarrier();
    }
    if t == 0u {
        atomicStore(&pcount[COUNT_N], ab_n);
    }
}
