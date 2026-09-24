// S349 / porte A — **le décalage d'un tableau du domaine δ**, de mailles entières.
//
// Un domaine qui se déplace de `(di, dj)` mailles garde son état dans le recouvrement et fait naître au repos ce qui
// entre (I-12). Chaque tableau — vitesses d'une famille de faces, surface, reste compensé, pression de départ, surface
// publiée — est d'abord recopié dans un tampon de travail, puis réécrit en place : l'élément `(i, j, k)` prend la
// valeur de `(i + di, j + dj, k)` si elle existe, la valeur de repos sinon. Aucune arithmétique : un déplacement
// de données, au bit.

struct Shift {
    nx: u32,
    ny: u32,
    nz: u32,
    count: u32,
    di: i32,
    dj: i32,
    offset: u32,
    fill: f32,
};

@group(0) @binding(0) var<storage, read> scratch: array<f32>;
@group(0) @binding(1) var<storage, read_write> target_: array<f32>;
@group(0) @binding(2) var<uniform> sh: Shift;

@compute @workgroup_size(64)
fn shift(@builtin(global_invocation_id) id: vec3<u32>) {
    let n = id.x;
    if (n >= sh.count) { return; }
    let i = i32(n % sh.nx);
    let j = i32((n / sh.nx) % sh.ny);
    let k = n / (sh.nx * sh.ny);
    let si = i + sh.di;
    let sj = j + sh.dj;
    var v = sh.fill;
    if (si >= 0 && si < i32(sh.nx) && sj >= 0 && sj < i32(sh.ny)) {
        v = scratch[(k * sh.ny + u32(sj)) * sh.nx + u32(si)];
    }
    target_[sh.offset + n] = v;
}
