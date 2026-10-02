// **S452 — la surface continue, en direct sur la carte** (ADR-211 D2 ; R37 reçu). Deux étapes, sur le device de la carte de la
// bande, sans retour au CPU :
//
// 1. `fondu` — le `champ_rendu` de S451 porté : sur les colonnes à moins de deux mailles d'une frontière bande | colonnes, une moyenne
//    horizontale 3 × 3 de `φ`, maille par maille (réflexion aux plans x = 0 et y = 0, la dernière au bord lointain) ; ailleurs, `φ`
//    tel quel. Deux passes, en va-et-vient (`cellf` → `tmp` → `champ`). L'ordre des sommes est celui du CPU.
// 2. `vs` / `fs` — un rayon par pixel dans le champ fondu (le quart reflété en entier, trilinéaire aux centres des mailles) : le
//    premier changement de signe par pas d'une demi-maille, puis bissection ; la normale, le gradient de `φ` aux points de la grille
//    interpolé (comme les normales lissées de S451) ; l'ombrage de R37 ; la sphère en gris (intersection analytique).

struct Rendu {
    dims: vec4<u32>,   // nx, ny, nz, —
    img: vec4<u32>,    // largeur, hauteur, —, —
    s: vec4<f32>,      // dx, f (1 / tan(fov / 2)), aspect, —
    oeil: vec4<f32>,
    avant: vec4<f32>,
    droite: vec4<f32>,
    haut: vec4<f32>,
    soleil: vec4<f32>,
    sphere: vec4<f32>, // centre, rayon (0 : pas de corps)
};

@group(0) @binding(0) var<uniform> R: Rendu;
@group(0) @binding(1) var<storage, read> phi_in: array<f32>;
@group(0) @binding(2) var<storage, read> cmask: array<u32>;
@group(0) @binding(3) var<storage, read_write> phi_out: array<f32>;

fn colonne(i: u32, j: u32) -> bool {
    return cmask[j * R.dims.x + i] != 0u;
}

@compute @workgroup_size(64)
fn fondu(@builtin(global_invocation_id) id: vec3<u32>) {
    let nx = R.dims.x;
    let ny = R.dims.y;
    let c = id.x;
    if c >= nx * ny * R.dims.z {
        return;
    }
    let i = c % nx;
    let j = (c / nx) % ny;
    let k = c / (nx * ny);
    let moi = colonne(i, j);
    var zone = false;
    for (var y = max(i32(j) - 2, 0); y <= min(i32(j) + 2, i32(ny) - 1); y++) {
        for (var x = max(i32(i) - 2, 0); x <= min(i32(i) + 2, i32(nx) - 1); x++) {
            if colonne(u32(x), u32(y)) != moi {
                zone = true;
            }
        }
    }
    if !zone {
        phi_out[c] = phi_in[c];
        return;
    }
    var somme = 0.0;
    for (var dj = -1; dj <= 1; dj++) {
        for (var di = -1; di <= 1; di++) {
            let x = u32(max(min(i32(i) + di, i32(nx) - 1), 0));
            let y = u32(max(min(i32(j) + dj, i32(ny) - 1), 0));
            somme += phi_in[(k * ny + y) * nx + x];
        }
    }
    phi_out[c] = somme / 9.0;
}

// Le rendu : le champ fondu, en lecture seule (une liaison à part : chaque étape a sa disposition).
@group(0) @binding(4) var<storage, read> champ: array<f32>;

// Un point de la grille du quart : `i`, `j` de −1 (l'image de 0) au dernier, `k` borné.
fn grille(i: i32, j: i32, k: i32) -> f32 {
    let x = u32(clamp(i, 0, i32(R.dims.x) - 1));
    let y = u32(clamp(j, 0, i32(R.dims.y) - 1));
    let z = u32(clamp(k, 0, i32(R.dims.z) - 1));
    return champ[(z * R.dims.y + y) * R.dims.x + x];
}

// `φ` au point `p` (le domaine entier, reflété dans le quart), trilinéaire aux centres des mailles.
fn phi(p: vec3<f32>) -> f32 {
    let g = vec3<f32>(abs(p.x), abs(p.y), p.z) / R.s.x - 0.5;
    let b = floor(g);
    let f = g - b;
    let i = vec3<i32>(b);
    let c00 = mix(grille(i.x, i.y, i.z), grille(i.x + 1, i.y, i.z), f.x);
    let c10 = mix(grille(i.x, i.y + 1, i.z), grille(i.x + 1, i.y + 1, i.z), f.x);
    let c01 = mix(grille(i.x, i.y, i.z + 1), grille(i.x + 1, i.y, i.z + 1), f.x);
    let c11 = mix(grille(i.x, i.y + 1, i.z + 1), grille(i.x + 1, i.y + 1, i.z + 1), f.x);
    return mix(mix(c00, c10, f.y), mix(c01, c11, f.y), f.z);
}

// Le gradient de `φ` à un point de la grille (différences centrées, décentrées au bord : S451).
fn gradient_grille(i: i32, j: i32, k: i32) -> vec3<f32> {
    let xl = max(i - 1, -1);
    let xh = min(i + 1, i32(R.dims.x) - 1);
    let yl = max(j - 1, -1);
    let yh = min(j + 1, i32(R.dims.y) - 1);
    let zl = max(k - 1, 0);
    let zh = min(k + 1, i32(R.dims.z) - 1);
    return vec3<f32>(
        (grille(xh, j, k) - grille(xl, j, k)) / f32(max(xh - xl, 1)),
        (grille(i, yh, k) - grille(i, yl, k)) / f32(max(yh - yl, 1)),
        (grille(i, j, zh) - grille(i, j, zl)) / f32(max(zh - zl, 1)),
    );
}

// La normale lissée au point `p` : le gradient aux huit points voisins, interpolé ; le signe rendu au quart d'origine.
fn normale(p: vec3<f32>) -> vec3<f32> {
    let g = vec3<f32>(abs(p.x), abs(p.y), p.z) / R.s.x - 0.5;
    let b = floor(g);
    let f = g - b;
    let i = vec3<i32>(b);
    let c00 = mix(gradient_grille(i.x, i.y, i.z), gradient_grille(i.x + 1, i.y, i.z), f.x);
    let c10 = mix(gradient_grille(i.x, i.y + 1, i.z), gradient_grille(i.x + 1, i.y + 1, i.z), f.x);
    let c01 = mix(gradient_grille(i.x, i.y, i.z + 1), gradient_grille(i.x + 1, i.y, i.z + 1), f.x);
    let c11 = mix(gradient_grille(i.x, i.y + 1, i.z + 1), gradient_grille(i.x + 1, i.y + 1, i.z + 1), f.x);
    let n = mix(mix(c00, c10, f.y), mix(c01, c11, f.y), f.z);
    return vec3<f32>(select(n.x, -n.x, p.x < 0.0), select(n.y, -n.y, p.y < 0.0), n.z);
}

fn ciel(dir: vec3<f32>) -> vec3<f32> {
    let t = pow(0.5 + 0.5 * clamp(dir.z, -1.0, 1.0), 0.8);
    return vec3<f32>(0.55 + 0.25 * t, 0.68 + 0.2 * t, 0.82 + 0.15 * t);
}

// L'ombrage de R37 (`ombre`, S451) : l'eau — Fresnel de Schlick, reflet du ciel, Lambert, reflet du soleil ; le corps en gris.
fn ombre(n0: vec3<f32>, p: vec3<f32>, eau: bool) -> vec3<f32> {
    let vue = normalize(R.oeil.xyz - p);
    var n = normalize(n0);
    if dot(n, vue) < 0.0 {
        n = -n;
    }
    let soleil = R.soleil.xyz;
    if eau {
        let cs = clamp(dot(n, vue), 0.0, 1.0);
        let fresnel = 0.02 + 0.98 * pow(1.0 - cs, 5.0);
        let refl = normalize(2.0 * dot(n, vue) * n - vue);
        let lambert = max(dot(n, soleil), 0.0);
        let fond = vec3<f32>(0.02 + 0.10 * lambert, 0.16 + 0.18 * lambert, 0.24 + 0.20 * lambert);
        let spec = pow(max(dot(refl, soleil), 0.0), 60.0) * 0.8;
        return fond * (1.0 - fresnel) + ciel(refl) * fresnel + vec3<f32>(spec);
    }
    let l = 0.25 + 0.65 * max(dot(n, soleil), 0.0);
    return vec3<f32>(0.55 * l, 0.55 * l, 0.58 * l);
}

@vertex
fn vs(@builtin(vertex_index) v: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((v << 1u) & 2u), f32(v & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // La direction du rayon, comme au CPU (le coin du pixel).
    let px = floor(pos.xy);
    let ndc = vec2<f32>(px.x / f32(R.img.x) * 2.0 - 1.0, 1.0 - 2.0 * px.y / f32(R.img.y));
    let dir = normalize(R.avant.xyz + R.droite.xyz * ndc.x * R.s.z / R.s.y + R.haut.xyz * ndc.y / R.s.y);
    let o = R.oeil.xyz;
    var couleur = ciel(dir);
    var t_corps = 1e30;
    // La sphère.
    if R.sphere.w > 0.0 {
        let oc = o - R.sphere.xyz;
        let b = dot(oc, dir);
        let c = dot(oc, oc) - R.sphere.w * R.sphere.w;
        let disc = b * b - c;
        if disc >= 0.0 {
            let t = -b - sqrt(disc);
            if t > 0.0 {
                t_corps = t;
            }
        }
    }
    // La boîte échantillonnée : le quart reflété, des centres extrêmes des mailles.
    let dx = R.s.x;
    let hi = vec3<f32>((f32(R.dims.x) - 0.5) * dx, (f32(R.dims.y) - 0.5) * dx, (f32(R.dims.z) - 0.5) * dx);
    let lo = vec3<f32>(-hi.x, -hi.y, 0.5 * dx);
    let inv = 1.0 / dir;
    let t0 = (lo - o) * inv;
    let t1 = (hi - o) * inv;
    let entree = max(max(min(t0.x, t1.x), min(t0.y, t1.y)), max(min(t0.z, t1.z), 0.0));
    let sortie = min(min(max(t0.x, t1.x), max(t0.y, t1.y)), min(max(t0.z, t1.z), t_corps));
    var touche = false;
    if entree < sortie {
        let pas = 0.5 * dx;
        var ta = entree;
        var fa = phi(o + dir * ta);
        loop {
            let tb = min(ta + pas, sortie);
            let fb = phi(o + dir * tb);
            if (fa < 0.0) != (fb < 0.0) {
                // La bissection.
                var a = ta;
                var b = tb;
                for (var q = 0; q < 8; q++) {
                    let m = 0.5 * (a + b);
                    if (phi(o + dir * m) < 0.0) == (fa < 0.0) {
                        a = m;
                    } else {
                        b = m;
                    }
                }
                let p = o + dir * (0.5 * (a + b));
                couleur = ombre(normale(p), p, true);
                touche = true;
                break;
            }
            if tb >= sortie {
                break;
            }
            ta = tb;
            fa = fb;
        }
    }
    if !touche && t_corps < 1e29 {
        let p = o + dir * t_corps;
        couleur = ombre(p - R.sphere.xyz, p, false);
    }
    return vec4<f32>(pow(clamp(couleur, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2)), 1.0);
}
