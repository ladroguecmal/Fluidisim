// S299 / ADR-175 §4.2 — opérateur de pression 3D **assemblé sur la carte**.
//
// Le cœur n'exporte aucune ligne en 3D (ADR-172 est resté en 2D) : la règle de `mobile_row` est
// portée ici, pas lue. Une maille sèche rend 0 ; une face latérale vers une colonne sèche et la
// face du haut portent un coefficient fantôme `1/θ`, θ borné par `theta_min` (conditionnement,
// pas seuil physique). Les six faces sont accumulées dans l'ordre x−, x+, y−, y+, z−, z+ :
// l'addition f32 n'est pas associative, et c'est l'ordre du cœur.

struct Params {
    nx: u32,
    ny: u32,
    nz: u32,
    cells: u32,
    dx: f32,
    inv_dx2: f32,
    theta_min: f32,
    _pad: f32,
};

@group(0) @binding(0) var<storage, read> heights: array<f32>;
@group(0) @binding(1) var<storage, read> p: array<f32>;
@group(0) @binding(2) var<storage, read_write> out: array<f32>;
@group(0) @binding(3) var<uniform> params: Params;

fn height(i: u32, j: u32) -> f32 {
    return heights[j * params.nx + i];
}

fn wet(i: u32, j: u32, k: u32) -> bool {
    return (f32(k) + 0.5) * params.dx < height(i, j);
}

fn cell(i: u32, j: u32, k: u32) -> u32 {
    return (k * params.ny + j) * params.nx + i;
}

// Face latérale vers une colonne sèche : θ = (h − z_c) / (h − h_voisin), borné.
fn side_coefficient(h: f32, zc: f32, h_other: f32) -> f32 {
    return 1.0 / max((h - zc) / (h - h_other), params.theta_min);
}

@compute @workgroup_size(64)
fn apply_operator(@builtin(global_invocation_id) gid: vec3<u32>) {
    let c = gid.x;
    if (c >= params.cells) {
        return;
    }
    let plane = params.nx * params.ny;
    let i = c % params.nx;
    let j = (c / params.nx) % params.ny;
    let k = c / plane;

    let h = height(i, j);
    let zc = (f32(k) + 0.5) * params.dx;
    if (zc >= h) {
        out[c] = 0.0;
        return;
    }

    let pc = p[c];
    var acc = 0.0;

    // x− puis x+ : hors domaine est un mur, qui ne contribue pas.
    if (i > 0u) {
        let o = height(i - 1u, j);
        if (zc < o) { acc = acc + (pc - p[c - 1u]); }
        else { acc = acc + pc * side_coefficient(h, zc, o); }
    }
    if (i + 1u < params.nx) {
        let o = height(i + 1u, j);
        if (zc < o) { acc = acc + (pc - p[c + 1u]); }
        else { acc = acc + pc * side_coefficient(h, zc, o); }
    }
    // y− puis y+.
    if (j > 0u) {
        let o = height(i, j - 1u);
        if (zc < o) { acc = acc + (pc - p[c - params.nx]); }
        else { acc = acc + pc * side_coefficient(h, zc, o); }
    }
    if (j + 1u < params.ny) {
        let o = height(i, j + 1u);
        if (zc < o) { acc = acc + (pc - p[c + params.nx]); }
        else { acc = acc + pc * side_coefficient(h, zc, o); }
    }
    // z− : une maille sous une maille mouillée l'est toujours ; le fond est un mur.
    if (k > 0u) {
        acc = acc + (pc - p[c - plane]);
    }
    // z+ : fluide au-dessus, sinon le fantôme de surface, θ = (h − z_c)/dx.
    if (k + 1u < params.nz && (f32(k + 1u) + 0.5) * params.dx < h) {
        acc = acc + (pc - p[c + plane]);
    } else {
        acc = acc + pc * (1.0 / max((h - zc) / params.dx, params.theta_min));
    }

    out[c] = acc * params.inv_dx2;
}
