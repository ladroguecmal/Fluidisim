// S358 — **le pas linéaire de la porte D sur la carte** (`Volume3::step_surface_linear`).
//
// Le domaine δ d'une coque est un domaine **linéaire** : surface linéarisée sous un couvercle à `z₀`, faces coupées
// par un fond et un solide, reçu par la porte D sur la référence CPU (S324–S338). Ce module en porte le pas, étage par
// étage, dans l'ordre du cœur : prédit = courant, éponge sur les vitesses prédites, second membre pondéré par les
// ouvertures plus le terme du couvercle, gradient conjugué de Jacobi à cycles fixes repartant du pas précédent,
// correction entre mailles fluides, flux de colonne pondérés, hauteur compensée, rappel de l'éponge. Aucune identité
// au bit n'est promise (ADR-175 D4) ; ce qui est promis, c'est la même formule.
//
// Les ouvertures et les fractions sont **des données**, réservées à la création (I-06) : toutes ouvertes, ou celles
// que le cœur a découpées. Le pas ne relit rien (SPEC-004 §8.4) ; rien n'est sérialisé (I-17), rien ne sort vers le
// jeu (I-04).

struct Params {
    nx: u32,
    ny: u32,
    nz: u32,
    cells: u32,
    faces: u32,
    groups: u32,
    _p0: u32,
    _p1: u32,
    dx: f32,
    inv_dx2: f32,
    rho_g: f32,
    z0: f32,
    // `−ρ/dt`, `dt/ρ`, `dt/dx`, `dt` : les coefficients temporels, construits en f64 et arrondis (I-08).
    scale: f32,
    k1: f32,
    transport: f32,
    dt: f32,
    sponge_x: f32,
    sponge_y: f32,
    sponge_rate: f32,
    // S493 : le plancher d'ouverture du couvercle en partie couvert (`lid_floor` du cœur : `dt²·g/dx` borné à [0,1 ; 1]).
    lid_floor: f32,
};

// Vitesses aux faces : [u | v | w] courantes, puis [u | v | w] prédites, rangées comme le cœur.
@group(0) @binding(0) var<storage, read_write> vel: array<f32>;
// [ouvertures des faces (faces) | fraction fluide des mailles (mailles)].
@group(0) @binding(1) var<storage, read> geo: array<f32>;
// [η | reste de la somme compensée | flux x ((nx+1)·ny) | flux y (nx·(ny+1)) | surface publiée] — colonnes.
@group(0) @binding(2) var<storage, read_write> cols: array<f32>;
// Gradient conjugué, sept tranches de mailles ; la tranche X est la pression.
@group(0) @binding(3) var<storage, read_write> state: array<f32>;
@group(0) @binding(4) var<storage, read_write> partial: array<f32>;
@group(0) @binding(5) var<storage, read_write> scalar: array<f32>;
@group(0) @binding(6) var<uniform> s: Params;
/// **S503 — le mouvement d'un solide** (`Linear3::set_motion`) : le terme de paroi de chaque maille (`cells`), la vitesse imposée
/// des faces qui se ferment ou s'ouvrent (`faces` ; `f32::MAX` : garder), l'eau déposée sur chaque colonne (`columns`, m) ;
/// **S504** : les poids du transfert de S334 (`5 × columns` : rapport de fermeture, parts gauche, droite, avant, arrière).
@group(0) @binding(7) var<storage, read> motion: array<f32>;

const X: u32 = 0u;   // pression
const R: u32 = 1u;   // résidu
const Z: u32 = 2u;   // M⁻¹r
const D: u32 = 3u;   // direction
const Q: u32 = 4u;   // A·d
const M: u32 = 5u;   // Jacobi
const B: u32 = 6u;   // second membre

const RZ: u32 = 0u;
const DQ: u32 = 1u;
const ALPHA: u32 = 2u;
const BETA: u32 = 3u;
const BNORM: u32 = 4u;
const RESIDUAL: u32 = 5u;

var<workgroup> scratch: array<f32, 64>;

fn at(section: u32, c: u32) -> u32 { return section * s.cells + c; }
fn columns() -> u32 { return s.nx * s.ny; }
fn n_u() -> u32 { return (s.nx + 1u) * s.ny * s.nz; }
fn n_v() -> u32 { return s.nx * (s.ny + 1u) * s.nz; }
fn fu(i: u32, j: u32, k: u32) -> u32 { return (k * s.ny + j) * (s.nx + 1u) + i; }
fn fv(i: u32, j: u32, k: u32) -> u32 { return n_u() + (k * (s.ny + 1u) + j) * s.nx + i; }
fn fw(i: u32, j: u32, k: u32) -> u32 { return n_u() + n_v() + (k * s.ny + j) * s.nx + i; }
fn open(f: u32) -> f32 { return geo[f]; }
fn frac(c: u32) -> f32 { return geo[s.faces + c]; }
fn x_faces() -> u32 { return (s.nx + 1u) * s.ny; }
fn y_faces() -> u32 { return s.nx * (s.ny + 1u); }
// Tranches de `cols`.
fn roundoff_at(c: u32) -> u32 { return columns() + c; }
fn flux_x_at(f: u32) -> u32 { return 2u * columns() + f; }
fn flux_y_at(f: u32) -> u32 { return 2u * columns() + x_faces() + f; }
fn published_at(c: u32) -> u32 { return 2u * columns() + x_faces() + y_faces() + c; }
/// S504 : ce qu'une colonne reçoit du transfert, entre le rassemblement et l'application.
fn transfer_at(c: u32) -> u32 { return 3u * columns() + x_faces() + y_faces() + c; }
/// S504 : le dépôt du pas et les poids du transfert, dans le tampon de mouvement.
fn depot(c: u32) -> f32 { return motion[s.cells + s.faces + c]; }
fn transfer_w(c: u32, q: u32) -> f32 { return motion[s.cells + s.faces + columns() + 5u * c + q]; }

fn ramp(x: f32, n: u32, width: f32) -> f32 {
    if (width == 0.0) { return 0.0; }
    return max(1.0 - min(x, f32(n) * s.dx - x) / width, 0.0);
}

/// `Sponge3::factor` d'ADR-164, en f32 ; le cœur le calcule en f64. Hors des bandes, 1 exactement.
fn sponge(x: f32, y: f32) -> f32 {
    if (s.sponge_rate == 0.0) { return 1.0; }
    let rx = ramp(x, s.nx, s.sponge_x);
    let ry = ramp(y, s.ny, s.sponge_y);
    let r2 = rx * rx + ry * ry;
    if (r2 == 0.0) { return 1.0; }
    return exp(-s.sponge_rate * s.dt * r2);
}

/// La perturbation compensée d'une colonne, `(η − z₀) − reste`. **S358 : `η − z₀` en entiers.** Écrite en flottant,
/// l'expression est réassociée par le compilateur en `η − (z₀ + reste)`, où `z₀ + reste` s'arrondit à l'ulp de `z₀`
/// et le reste disparaît : mesuré, la somme publiée dérivait de 3,4·10⁻⁵ m dès le premier pas quand la somme vraie
/// tenait à 10⁻⁹ près. La différence exacte ne laisse rien à réassocier (L345).
fn perturbation(col: u32) -> f32 {
    return difference(cols[col], s.z0) - cols[roundoff_at(col)];
}

/// Pression dynamique imposée au couvercle de la colonne : `ρg·((η − z₀) − reste)`. **S493 — le couvercle partiel d'un
/// décor fixe qui le perce** (S334, `Volume3::lid`) : l'excès d'eau d'une colonne en partie couverte se tient dans sa part
/// libre `a`, où la pression vaut `1/max(a, plancher)` fois celle de la hauteur de remplissage. Un décor fixe ne dépose rien
/// (le dépôt d'une coque qui bouge n'est pas porté). Couvercle plein ou fermé : la valeur d'avant, au bit.
fn lid(col: u32) -> f32 {
    return s.rho_g * perturbation(col);
}

/// S493 — la pression d'un couvercle **en partie couvert** (`0 < a < 1`), appelée seulement là : un couvercle plein garde
/// l'expression d'avant aux deux endroits où elle sert, au bit (le compilateur réordonnait sinon le chemin plein, L345).
fn lid_partial(col: u32, a: f32) -> f32 {
    // S504 : l'eau que la coque vient de déposer dans la colonne n'est pas à la surface — le flux de sa paroi la retire pendant
    // ce pas (`Volume3::lid`). Zéro sans coque qui bouge.
    return s.rho_g * (perturbation(col) - depot(col)) / max(a, s.lid_floor);
}

/// Emplacement de face → (i, j, k, axe).
fn decode(slot: u32) -> vec4<u32> {
    if (slot < n_u()) {
        let n = s.nx + 1u;
        return vec4<u32>(slot % n, (slot / n) % s.ny, slot / (n * s.ny), 0u);
    }
    if (slot < n_u() + n_v()) {
        let r = slot - n_u();
        return vec4<u32>(r % s.nx, (r / s.nx) % (s.ny + 1u), r / (s.nx * (s.ny + 1u)), 1u);
    }
    let r = slot - n_u() - n_v();
    return vec4<u32>(r % s.nx, (r / s.nx) % s.ny, r / (s.nx * s.ny), 2u);
}

// ── Prédiction : le modèle est linéaire, le champ prédit est le courant, amorti par l'éponge ────────────────────

@compute @workgroup_size(64)
fn predict(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= s.faces) { return; }
    let q = decode(slot);
    let x = (f32(q.x) + select(0.5, 0.0, q.w == 0u)) * s.dx;
    let y = (f32(q.y) + select(0.5, 0.0, q.w == 1u)) * s.dx;
    vel[s.faces + slot] = vel[slot] * sponge(x, y);
}

// ── Opérateur pondéré (`apply_cut`) ───────────────────────────────────────────────────────────────────────────────

/// `(A·v)_c` sur la tranche `section` et la diagonale de la ligne, `1/dx²` compris. Faces gauche, droite, avant,
/// arrière, bas, haut ; une face d'ouverture nulle ne porte rien ; un voisin solide non plus ; le couvercle à une
/// demi-maille, `2a`. Zéro sur une maille solide.
fn stencil(section: u32, c: u32) -> vec2<f32> {
    if (frac(c) == 0.0) { return vec2<f32>(0.0, 0.0); }
    let plane = columns();
    let i = c % s.nx;
    let j = (c / s.nx) % s.ny;
    let k = c / plane;
    let base = section * s.cells;
    let vc = state[base + c];
    var acc = 0.0;
    var diag = 0.0;
    var a = open(fu(i, j, k));
    if (a != 0.0 && i > 0u && frac(c - 1u) > 0.0) { acc += a * (vc - state[base + c - 1u]); diag += a; }
    a = open(fu(i + 1u, j, k));
    if (a != 0.0 && i + 1u < s.nx && frac(c + 1u) > 0.0) { acc += a * (vc - state[base + c + 1u]); diag += a; }
    a = open(fv(i, j, k));
    if (a != 0.0 && j > 0u && frac(c - s.nx) > 0.0) { acc += a * (vc - state[base + c - s.nx]); diag += a; }
    a = open(fv(i, j + 1u, k));
    if (a != 0.0 && j + 1u < s.ny && frac(c + s.nx) > 0.0) { acc += a * (vc - state[base + c + s.nx]); diag += a; }
    a = open(fw(i, j, k));
    if (a != 0.0 && k > 0u && frac(c - plane) > 0.0) { acc += a * (vc - state[base + c - plane]); diag += a; }
    a = open(fw(i, j, k + 1u));
    if (a != 0.0) {
        if (k + 1u < s.nz) {
            if (frac(c + plane) > 0.0) { acc += a * (vc - state[base + c + plane]); diag += a; }
        } else {
            acc += 2.0 * a * vc;
            diag += 2.0 * a;
        }
    }
    return vec2<f32>(acc * s.inv_dx2, diag * s.inv_dx2);
}

// ── Réductions : arbre dans le groupe, puis un groupe qui rassemble ───────────────────────────────────────────────

fn fold_at(base: u32, value: f32, lid_: u32, group: u32) {
    scratch[lid_] = value;
    workgroupBarrier();
    for (var w = 32u; w > 0u; w >>= 1u) {
        if (lid_ < w) { scratch[lid_] += scratch[lid_ + w]; }
        workgroupBarrier();
    }
    if (lid_ == 0u) { partial[base * s.groups + group] = scratch[0]; }
    workgroupBarrier();
}

fn gather_at(base: u32, lid_: u32) -> f32 {
    var v = 0.0;
    var g = lid_;
    loop {
        if (g >= s.groups) { break; }
        v += partial[base * s.groups + g];
        g += 64u;
    }
    scratch[lid_] = v;
    workgroupBarrier();
    for (var w = 32u; w > 0u; w >>= 1u) {
        if (lid_ < w) { scratch[lid_] += scratch[lid_ + w]; }
        workgroupBarrier();
    }
    let out = scratch[0];
    workgroupBarrier();
    return out;
}

// ── Second membre (`project`) : `−ρ/dt · div(u*)` pondéré, plus le couvercle ; Jacobi ; ‖b‖² replié ───────────────

@compute @workgroup_size(64)
fn rhs(@builtin(global_invocation_id) id: vec3<u32>,
       @builtin(local_invocation_index) lid_: u32,
       @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < s.cells) {
        var b = 0.0;
        var m = 0.0;
        if (frac(c) > 0.0) {
            let plane = columns();
            let i = c % s.nx;
            let j = (c / s.nx) % s.ny;
            let k = c / plane;
            let o = s.faces;
            let fl = open(fu(i, j, k)) * vel[o + fu(i, j, k)];
            let fr = open(fu(i + 1u, j, k)) * vel[o + fu(i + 1u, j, k)];
            let ff = open(fv(i, j, k)) * vel[o + fv(i, j, k)];
            let fk = open(fv(i, j + 1u, k)) * vel[o + fv(i, j + 1u, k)];
            let fb = open(fw(i, j, k)) * vel[o + fw(i, j, k)];
            let ft = open(fw(i, j, k + 1u)) * vel[o + fw(i, j, k + 1u)];
            let paroi = motion[c];
            if (paroi == 0.0) {
                b = s.scale * (((fr - fl + ft - fb) + (fk - ff)) / s.dx);
            } else {
                // S503 : la part des faces que le solide mobile couvre avance à sa vitesse (`Volume3::wall_term`).
                b = s.scale * ((((fr - fl + ft - fb) + (fk - ff)) / s.dx) + paroi);
            }
            if (k + 1u == s.nz) {
                let a = open(fw(i, j, s.nz));
                if (a >= 1.0) { b += 2.0 * a * lid(j * s.nx + i) * s.inv_dx2; }
                else if (a > 0.0) { b += 2.0 * a * lid_partial(j * s.nx + i, a) * s.inv_dx2; }
            }
            let diag = stencil(X, c).y;
            if (diag > 0.0) { m = 1.0 / diag; }
        }
        state[at(B, c)] = b;
        state[at(M, c)] = m;
        contribution = b * b;
    }
    fold_at(2u, contribution, lid_, wid.x);
}

@compute @workgroup_size(64)
fn finish_bnorm(@builtin(local_invocation_index) lid_: u32) {
    let v = gather_at(2u, lid_);
    if (lid_ == 0u) { scalar[BNORM] = v; }
}

// ── Gradient conjugué de Jacobi, départ chaud (la pression du pas précédent) ─────────────────────────────────────

@compute @workgroup_size(64)
fn init_warm(@builtin(global_invocation_id) id: vec3<u32>,
             @builtin(local_invocation_index) lid_: u32,
             @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < s.cells) {
        if (frac(c) == 0.0) {
            state[at(X, c)] = 0.0;
            state[at(R, c)] = 0.0;
            state[at(Z, c)] = 0.0;
            state[at(D, c)] = 0.0;
        } else {
            let r = state[at(B, c)] - stencil(X, c).x;
            let z = state[at(M, c)] * r;
            state[at(R, c)] = r;
            state[at(Z, c)] = z;
            state[at(D, c)] = z;
            contribution = r * z;
        }
        state[at(Q, c)] = 0.0;
    }
    fold_at(0u, contribution, lid_, wid.x);
}

@compute @workgroup_size(64)
fn finish_rz(@builtin(local_invocation_index) lid_: u32) {
    let v = gather_at(0u, lid_);
    if (lid_ == 0u) { scalar[RZ] = v; }
}

@compute @workgroup_size(64)
fn apply_fold(@builtin(global_invocation_id) id: vec3<u32>,
              @builtin(local_invocation_index) lid_: u32,
              @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < s.cells) {
        let q = stencil(D, c).x;
        state[at(Q, c)] = q;
        contribution = state[at(D, c)] * q;
    }
    fold_at(0u, contribution, lid_, wid.x);
}

/// Un dénominateur non strictement positif rend `alpha = 0` : le cycle devient neutre, jamais infini.
@compute @workgroup_size(64)
fn finish_dq(@builtin(local_invocation_index) lid_: u32) {
    let v = gather_at(0u, lid_);
    if (lid_ == 0u) {
        scalar[DQ] = v;
        if (v > 0.0) { scalar[ALPHA] = scalar[RZ] / v; } else { scalar[ALPHA] = 0.0; }
    }
}

@compute @workgroup_size(64)
fn update(@builtin(global_invocation_id) id: vec3<u32>,
          @builtin(local_invocation_index) lid_: u32,
          @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < s.cells) {
        let alpha = scalar[ALPHA];
        let r = state[at(R, c)] - alpha * state[at(Q, c)];
        state[at(X, c)] = state[at(X, c)] + alpha * state[at(D, c)];
        state[at(R, c)] = r;
        let z = state[at(M, c)] * r;
        state[at(Z, c)] = z;
        contribution = r * z;
    }
    fold_at(1u, contribution, lid_, wid.x);
}

@compute @workgroup_size(64)
fn finish_beta(@builtin(local_invocation_index) lid_: u32) {
    let v = gather_at(1u, lid_);
    if (lid_ == 0u) {
        let previous = scalar[RZ];
        if (previous > 0.0) { scalar[BETA] = v / previous; } else { scalar[BETA] = 0.0; }
        scalar[RZ] = v;
    }
}

@compute @workgroup_size(64)
fn direction(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= s.cells) { return; }
    state[at(D, c)] = state[at(Z, c)] + scalar[BETA] * state[at(D, c)];
}

/// Vrai résidu `b − A·x`, jamais la récurrence ; diagnostic, relu en différé ou par les bancs.
@compute @workgroup_size(64)
fn residual_fold(@builtin(global_invocation_id) id: vec3<u32>,
                 @builtin(local_invocation_index) lid_: u32,
                 @builtin(workgroup_id) wid: vec3<u32>) {
    let c = id.x;
    var contribution = 0.0;
    if (c < s.cells) {
        let r = state[at(B, c)] - stencil(X, c).x;
        contribution = r * r;
    }
    fold_at(2u, contribution, lid_, wid.x);
}

@compute @workgroup_size(64)
fn finish_residual(@builtin(local_invocation_index) lid_: u32) {
    let v = gather_at(2u, lid_);
    if (lid_ == 0u) { scalar[RESIDUAL] = v; }
}

// ── Correction (`correct_cut`) : entre deux mailles fluides, le couvercle à sa demi-maille ──────────────────────

fn pressure(c: u32) -> f32 { return state[c]; }

@compute @workgroup_size(64)
fn correct(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= s.faces) { return; }
    let a = open(slot);
    // Une face fermée n'a pas de vitesse — le cœur la garde nulle depuis `close_walls`.
    if (a == 0.0) {
        vel[slot] = 0.0;
        return;
    }
    let q = decode(slot);
    let i = q.x;
    let j = q.y;
    let k = q.z;
    let plane = columns();
    var value = vel[s.faces + slot];
    if (q.w == 0u) {
        if (i > 0u && i < s.nx) {
            let l = (k * s.ny + j) * s.nx + i - 1u;
            if (frac(l) > 0.0 && frac(l + 1u) > 0.0) { value -= s.k1 * (pressure(l + 1u) - pressure(l)) / s.dx; }
        }
    } else if (q.w == 1u) {
        if (j > 0u && j < s.ny) {
            let l = (k * s.ny + j - 1u) * s.nx + i;
            if (frac(l) > 0.0 && frac(l + s.nx) > 0.0) { value -= s.k1 * (pressure(l + s.nx) - pressure(l)) / s.dx; }
        }
    } else if (k >= 1u) {
        let below = ((k - 1u) * s.ny + j) * s.nx + i;
        if (frac(below) > 0.0) {
            if (k < s.nz) {
                if (frac(below + plane) > 0.0) { value -= s.k1 * (pressure(below + plane) - pressure(below)) / s.dx; }
            } else {
                let a_lid = open(slot);
                if (a_lid >= 1.0 || a_lid == 0.0) {
                    value -= s.k1 * (lid(j * s.nx + i) - pressure(below)) / (0.5 * s.dx);
                } else {
                    value -= s.k1 * (lid_partial(j * s.nx + i, a_lid) - pressure(below)) / (0.5 * s.dx);
                }
            }
        }
    }
    vel[slot] = value;
}

// ── Flux de colonne, puis hauteur compensée et éponge ───────────────────────────────────────────────────────────

/// `s − a`, **exacte et calculée en entiers** (S301) : le compilateur réécrit `(a + b) − a` en `b` (L345). Pour
/// deux flottants normaux positifs à moins d'un facteur deux l'un de l'autre, la différence est représentable.
fn exact_difference(x: f32, a: f32) -> f32 {
    let bs = bitcast<u32>(x);
    let ba = bitcast<u32>(a);
    let es = i32((bs >> 23u) & 0xffu);
    let ea = i32((ba >> 23u) & 0xffu);
    let ms = i32((bs & 0x7fffffu) | 0x800000u);
    let ma = i32((ba & 0x7fffffu) | 0x800000u);
    let e = min(es, ea);
    let d = (ms << u32(es - e)) - (ma << u32(ea - e));
    return f32(d) * bitcast<f32>(u32(e - 150 + 127) << 23u);
}

/// `x − a` exacte quand Sterbenz la garantit — deux flottants normaux positifs dont les exposants diffèrent d'un au
/// plus, ce que tient une hauteur de colonne près de son repos —, flottante sinon.
fn difference(x: f32, a: f32) -> f32 {
    let ex = i32((bitcast<u32>(x) >> 23u) & 0xffu);
    let ea = i32((bitcast<u32>(a) >> 23u) & 0xffu);
    if (x > 0.0 && a > 0.0 && ex > 0 && ea > 0 && ex < 255 && ea < 255 && abs(ex - ea) <= 1) {
        return exact_difference(x, a);
    }
    return x - a;
}

/// `Σ_k ouverture·u·dx` du fond vers le couvercle, à chaque face latérale de colonne.
@compute @workgroup_size(64)
fn fluxes(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= x_faces() + y_faces()) { return; }
    var q = 0.0;
    if (slot < x_faces()) {
        let i = slot % (s.nx + 1u);
        let j = slot / (s.nx + 1u);
        for (var k = 0u; k < s.nz; k = k + 1u) {
            let f = fu(i, j, k);
            q += open(f) * vel[f] * s.dx;
        }
        cols[flux_x_at(slot)] = q;
    } else {
        let r = slot - x_faces();
        let i = r % s.nx;
        let j = r / s.nx;
        for (var k = 0u; k < s.nz; k = k + 1u) {
            let f = fv(i, j, k);
            q += open(f) * vel[f] * s.dx;
        }
        cols[flux_y_at(r)] = q;
    }
}

@compute @workgroup_size(64)
fn advance(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let i = c % s.nx;
    let j = c / s.nx;
    let l = j * (s.nx + 1u) + i;
    let left = cols[flux_x_at(l)];
    let right = cols[flux_x_at(l + 1u)];
    let front = cols[flux_y_at(c)];
    let back = cols[flux_y_at(c + s.nx)];
    var eta = cols[c];
    var roundoff = cols[roundoff_at(c)];
    var increment = -s.transport * ((right - left) + (back - front)) - roundoff;
    var height = eta + increment;
    roundoff = exact_difference(height, eta) - increment;
    eta = height;
    let factor = sponge((f32(i) + 0.5) * s.dx, (f32(j) + 0.5) * s.dx);
    if (factor != 1.0) {
        increment = (factor - 1.0) * (eta - s.z0) - factor * roundoff;
        height = eta + increment;
        roundoff = exact_difference(height, eta) - increment;
        eta = height;
    }
    cols[c] = eta;
    cols[roundoff_at(c)] = roundoff;
    cols[published_at(c)] = difference(eta, s.z0) - roundoff;
}

// ── S503 : le mouvement d'un solide, appliqué au début du pas qui suit `set_motion` ─────────────────────────────────────────

/// Les faces que le solide vient de fermer (vitesse nulle) ou d'ouvrir (la vitesse normale de sa paroi).
@compute @workgroup_size(64)
fn motion_faces(@builtin(global_invocation_id) id: vec3<u32>) {
    let f = id.x;
    if (f >= s.faces) { return; }
    let o = motion[s.cells + f];
    if (o < 3.0e38) { vel[f] = o; }
}

/// L'eau que le solide déplace, déposée sur la surface de sa colonne, en somme compensée comme le transport.
@compute @workgroup_size(64)
fn motion_deposit(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let depot = motion[s.cells + s.faces + c];
    if (depot == 0.0) { return; }
    let eta = cols[c];
    let roundoff = cols[roundoff_at(c)];
    let increment = depot - roundoff;
    let height = eta + increment;
    cols[c] = height;
    cols[roundoff_at(c)] = exact_difference(height, eta) - increment;
    cols[published_at(c)] = difference(height, s.z0) - cols[roundoff_at(c)];
}

// ── S504 : le transfert de S334, l'eau qu'une paroi qui glisse pousse vers les voisines ──────────────────────────────────────

/// L'eau poussée hors d'une colonne dont le couvercle se referme : sa surface, hors le dépôt du pas, fois le rapport de fermeture.
fn pousse(c: u32) -> f32 {
    let r = transfer_w(c, 0u);
    if (r == 0.0) { return 0.0; }
    return (perturbation(c) - depot(c)) * r;
}

/// Ce que la colonne reçoit de ses quatre voisines, moins ce qu'elle pousse : un rassemblement, sans écriture concurrente.
@compute @workgroup_size(64)
fn motion_gather(@builtin(global_invocation_id) id: vec3<u32>) {
    let n = id.x;
    if (n >= columns()) { return; }
    let i = n % s.nx;
    let j = n / s.nx;
    var recu = 0.0;
    // La voisine de droite pousse vers sa gauche (part 1), celle de gauche vers sa droite (part 2), etc.
    if (i + 1u < s.nx) { recu += pousse(n + 1u) * transfer_w(n + 1u, 1u); }
    if (i > 0u) { recu += pousse(n - 1u) * transfer_w(n - 1u, 2u); }
    if (j + 1u < s.ny) { recu += pousse(n + s.nx) * transfer_w(n + s.nx, 3u); }
    if (j > 0u) { recu += pousse(n - s.nx) * transfer_w(n - s.nx, 4u); }
    cols[transfer_at(n)] = recu - pousse(n);
}

/// Le transfert appliqué, en somme compensée.
@compute @workgroup_size(64)
fn motion_apply(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let t = cols[transfer_at(c)];
    if (t == 0.0) { return; }
    let eta = cols[c];
    let roundoff = cols[roundoff_at(c)];
    let increment = t - roundoff;
    let height = eta + increment;
    cols[c] = height;
    cols[roundoff_at(c)] = exact_difference(height, eta) - increment;
    cols[published_at(c)] = difference(height, s.z0) - cols[roundoff_at(c)];
}
