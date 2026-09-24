// S349 / porte A — **le décalage d'un tableau du domaine δ**, de mailles entières. S350 : **et son redimensionnement**.
//
// Un domaine qui se déplace de `(di, dj)` mailles, ou change de forme, garde son état dans le recouvrement et fait
// naître au repos ce qui entre (I-12). Tout l'ancien état est d'abord recopié dans un tampon de travail, dans l'ancienne
// disposition ; chaque tableau — vitesses d'une famille de faces, surface, reste compensé, pression de départ, surface
// publiée — est ensuite réécrit dans la nouvelle : l'élément `(i, j, k)` prend la valeur de l'ancien
// `(i + di, j + dj, k)` s'il existe, la valeur de repos sinon. Aucune arithmétique : un déplacement de données, au bit.
//
// S350 : **les murs**. Les faces normales du bord — `u` en `i = 0` et `i = nx`, `v` en `j = 0` et `j = ny` — ne sont
// jamais écrites par le pas : ce sont les murs de δ, nuls. `wall` nomme la famille dont les deux bords sont des murs
// (1 : `u`, en x ; 2 : `v`, en y ; 0 : aucune) ; ses faces de bord reçoivent la valeur de repos.

struct Shift {
    // Le tableau cible : dimensions dans la nouvelle forme, nombre d'éléments, rang dans son tampon.
    nx: u32,
    ny: u32,
    nz: u32,
    count: u32,
    di: i32,
    dj: i32,
    offset: u32,
    fill: f32,
    wall: u32,
    // Le tableau source, dans le tampon de travail : dimensions dans l'ancienne forme, rang.
    snx: u32,
    sny: u32,
    source: u32,
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
    let mur = (sh.wall == 1u && (i == 0 || i == i32(sh.nx) - 1)) || (sh.wall == 2u && (j == 0 || j == i32(sh.ny) - 1));
    var v = sh.fill;
    if (!mur && si >= 0 && si < i32(sh.snx) && sj >= 0 && sj < i32(sh.sny)) {
        v = scratch[sh.source + (k * sh.sny + u32(sj)) * sh.snx + u32(si)];
    }
    target_[sh.offset + n] = v;
}
