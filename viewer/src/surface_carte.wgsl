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
    dims: vec4<u32>,   // nx, ny, nz, quart (S454 : 1, le quart reflété ; 0, un domaine entier)
    img: vec4<u32>,    // largeur, hauteur, —, —
    s: vec4<f32>,      // dx, f (1 / tan(fov / 2)), aspect, —
    oeil: vec4<f32>,
    avant: vec4<f32>,
    droite: vec4<f32>,
    haut: vec4<f32>,
    soleil: vec4<f32>,
    sphere: vec4<f32>, // centre, rayon (0 : pas de corps)
    // S457 — la houle B (`a`, `k`, `ω`, `φ`) et la mer : niveau moyen, instant, lumière reçue (1) ou ombrage de R37 (0).
    houle: vec4<f32>,
    mer: vec4<f32>,
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

// Le point `p` dans la grille : reflété dans le quart (S452), ou tel quel sur un domaine entier (S454).
fn dans_grille(p: vec3<f32>) -> vec3<f32> {
    if R.dims.w != 0u {
        return vec3<f32>(abs(p.x), abs(p.y), p.z) / R.s.x - 0.5;
    }
    return p / R.s.x - 0.5;
}

// `φ` au point `p`, trilinéaire aux centres des mailles.
fn phi(p: vec3<f32>) -> f32 {
    let g = dans_grille(p);
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
    let g = dans_grille(p);
    let b = floor(g);
    let f = g - b;
    let i = vec3<i32>(b);
    let c00 = mix(gradient_grille(i.x, i.y, i.z), gradient_grille(i.x + 1, i.y, i.z), f.x);
    let c10 = mix(gradient_grille(i.x, i.y + 1, i.z), gradient_grille(i.x + 1, i.y + 1, i.z), f.x);
    let c01 = mix(gradient_grille(i.x, i.y, i.z + 1), gradient_grille(i.x + 1, i.y, i.z + 1), f.x);
    let c11 = mix(gradient_grille(i.x, i.y + 1, i.z + 1), gradient_grille(i.x + 1, i.y + 1, i.z + 1), f.x);
    let n = mix(mix(c00, c10, f.y), mix(c01, c11, f.y), f.z);
    if R.dims.w == 0u {
        return n;
    }
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

// ---------------------------------------------------------------------------------------------------------------------
// S457 — **LA LUMIÈRE DE L'EAU REÇUE** (R14, R20, R24 ; ADR-177, ADR-194), portée de `godot/ciel.gdshaderinc`,
// `godot/optique_eau.gdshaderinc` et `godot/eau.gdshaderinc` : le ciel calé sur la photographie de référence (S308, S363), le
// corps d'eau `R(0⁻)` de Pope & Fry et Morel sous l'éclairement `E/π = 2`, Fresnel exact (indice 1,34), l'éclat du soleil, et
// la colonne d'eau de Maritorena, Morel et Gentili (1994) sur le trajet oblique : `fond·T + corps·(1 − T)`, `T = e^(−Kd·(H + L))`
// — le fond de sable (et le corps) vus par réfraction de Snell. Hors du domaine simulé, la mer continue : la surface de B,
// analytique (S457, ADR-215 D2) — les zones de relaxation y ramènent la surface simulée.

const SOLEIL_B = vec3<f32>(-0.4, 0.3, 0.8);
const CIEL_H = vec3<f32>(0.311, 0.554, 0.795);
const CIEL_F = vec3<f32>(0.139, 0.327, 0.722);
const R0_EAU = vec3<f32>(0.00068, 0.00826, 0.08960);
const GAIN_EAU = 2.0;
const KD = vec3<f32>(0.4259, 0.0724, 0.0158);
const INDICE = 1.34;
const SABLE = vec3<f32>(0.34, 0.30, 0.24);

fn pcg(v: u32) -> u32 {
    let etat = v * 747796405u + 2891336453u;
    let mot = ((etat >> ((etat >> 28u) + 4u)) ^ etat) * 277803737u;
    return (mot >> 22u) ^ mot;
}

fn hachage_ciel(q: vec2<f32>) -> f32 {
    let i = vec2<i32>(floor(q + 0.5));
    return f32(pcg(bitcast<u32>(i.x) + pcg(bitcast<u32>(i.y)))) * (1.0 / 4294967295.0);
}

fn bruit_ciel(q: vec2<f32>) -> f32 {
    let i = floor(q);
    let f = fract(q);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hachage_ciel(i);
    let b = hachage_ciel(i + vec2<f32>(1.0, 0.0));
    let c = hachage_ciel(i + vec2<f32>(0.0, 1.0));
    let d = hachage_ciel(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn nuages(rayon: vec3<f32>, octaves: u32) -> f32 {
    if rayon.z <= 0.0 {
        return 0.0;
    }
    let fondu = smoothstep(0.05, 0.12, rayon.z) * (1.0 - smoothstep(0.30, 0.55, rayon.z));
    if fondu <= 0.0 {
        return 0.0;
    }
    let q = rayon.xy / rayon.z * 1.3;
    var n = 0.50 * bruit_ciel(q) + 0.25 * bruit_ciel(q * 2.03 + 17.0);
    if octaves > 2u {
        n += 0.15 * bruit_ciel(q * 4.11 + 41.0) + 0.10 * bruit_ciel(q * 8.17 + 83.0);
    } else {
        n += 0.25 * 0.5;
    }
    return smoothstep(0.52, 0.80, n) * fondu;
}

// Le ciel calé sur la photographie : `H·e^(−K·sin h)`, `K = −ln F / sin 25°` ; nuages ; le soleil.
fn ciel_b(rayon: vec3<f32>, octaves: u32) -> vec3<f32> {
    let soleil = normalize(SOLEIL_B);
    let k = -log(CIEL_F) / sin(radians(25.0));
    var c = CIEL_H * exp(-k * clamp(rayon.z, 0.0, 1.0));
    c = mix(c, vec3<f32>(0.92, 0.93, 0.95), nuages(rayon, octaves));
    let d = max(dot(rayon, soleil), 0.0);
    return c + vec3<f32>(1.0, 0.95, 0.85) * (pow(d, 1024.0) * 4.0 + pow(d, 32.0) * 0.15);
}

fn eclairage(n: vec3<f32>) -> f32 {
    return 0.6 + 0.4 * max(dot(n, normalize(SOLEIL_B)), 0.0);
}

fn fresnel_exact(c: f32, n: f32) -> f32 {
    let st2 = (1.0 - c * c) / (n * n);
    let ct = sqrt(max(1.0 - st2, 0.0));
    let rs = (c - n * ct) / (c + n * ct);
    let rp = (n * c - ct) / (n * c + ct);
    return 0.5 * (rs * rs + rp * rp);
}

// La distance au corps le long de `o + t·d` (1e30 : manqué).
fn touche_corps(o: vec3<f32>, d: vec3<f32>) -> f32 {
    if R.sphere.w <= 0.0 {
        return 1e30;
    }
    let oc = o - R.sphere.xyz;
    let b = dot(oc, d);
    let c = dot(oc, oc) - R.sphere.w * R.sphere.w;
    let disc = b * b - c;
    if disc < 0.0 {
        return 1e30;
    }
    let t = -b - sqrt(disc);
    return select(1e30, t, t > 1e-4);
}

// Le corps (le joueur), gris, sous l'éclairement de la scène.
fn couleur_corps(p: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(0.22, 0.22, 0.24) * GAIN_EAU * eclairage(normalize(p - R.sphere.xyz));
}

// Le fond de sable au point `p` (z = 0).
fn couleur_sable(p: vec3<f32>) -> vec3<f32> {
    let variation = 0.85 + 0.3 * (0.6 * bruit_ciel(p.xy * 0.9) + 0.4 * bruit_ciel(p.xy * 3.7));
    return SABLE * variation * GAIN_EAU * eclairage(vec3<f32>(0.0, 0.0, 1.0));
}

// La surface de l'eau au point `p`, normale `n0` (vers l'air), vue dans la direction `d`.
fn couleur_eau(p: vec3<f32>, n0: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    var n = normalize(n0);
    if dot(n, d) > 0.0 {
        n = -n;
    }
    let soleil = normalize(SOLEIL_B);
    let f = fresnel_exact(clamp(dot(-d, n), 0.0, 1.0), INDICE);
    let reflet = reflect(d, n);
    let eclat = pow(max(dot(reflet, soleil), 0.0), 180.0);
    // Sous la surface : le rayon réfracté jusqu'au corps ou au fond ; la colonne d'eau.
    let r = refract(d, n, 1.0 / INDICE);
    var vu = vec3<f32>(0.0);
    var t_col = vec3<f32>(0.0);
    if r.z < -1e-3 {
        let t_fond = p.z / -r.z;
        let t_c = touche_corps(p, r);
        var q = p + r * t_fond;
        var c = couleur_sable(q);
        if t_c < t_fond {
            q = p + r * t_c;
            c = couleur_corps(q);
        }
        t_col = exp(-KD * (max(p.z - q.z, 0.0) + length(q - p)));
        vu = c * t_col;
    }
    let corps = R0_EAU * GAIN_EAU * eclairage(n);
    let sous = corps * (1.0 - t_col) + vu;
    return mix(sous, ciel_b(reflet, 2u), f) + vec3<f32>(1.0, 0.95, 0.85) * eclat * 1.2;
}

// La surface de B, analytique : `h + a·cos(k·x − ω·t + φ)`, et sa pente en `x`.
fn hauteur_b(x: f32) -> vec2<f32> {
    let theta = R.houle.y * x - R.houle.z * R.mer.y + R.houle.w;
    return vec2<f32>(R.mer.x + R.houle.x * cos(theta), -R.houle.x * R.houle.y * sin(theta));
}

// La première rencontre du rayon avec la surface de B (Newton depuis le plan moyen) ; 1e30 : aucune.
fn touche_b(o: vec3<f32>, d: vec3<f32>) -> f32 {
    if d.z >= -1e-4 {
        return 1e30;
    }
    var t = (R.mer.x - o.z) / d.z;
    for (var i = 0; i < 6; i++) {
        let hb = hauteur_b(o.x + t * d.x);
        let g = o.z + t * d.z - hb.x;
        let dg = d.z - hb.y * d.x;
        t = t - g / dg;
    }
    let hb = hauteur_b(o.x + t * d.x);
    if t <= 0.0 || abs(o.z + t * d.z - hb.x) > 1e-3 {
        return 1e30;
    }
    return t;
}

@fragment
fn fs(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // La direction du rayon, comme au CPU (le coin du pixel).
    let px = floor(pos.xy);
    let ndc = vec2<f32>(px.x / f32(R.img.x) * 2.0 - 1.0, 1.0 - 2.0 * px.y / f32(R.img.y));
    let dir = normalize(R.avant.xyz + R.droite.xyz * ndc.x * R.s.z / R.s.y + R.haut.xyz * ndc.y / R.s.y);
    let o = R.oeil.xyz;
    let lumiere = R.mer.z != 0.0;
    var couleur = ciel(dir);
    if lumiere {
        couleur = ciel_b(dir, 4u);
    }
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
    // La boîte échantillonnée, des centres extrêmes des mailles : le quart reflété, ou le domaine entier.
    let dx = R.s.x;
    let hi = vec3<f32>((f32(R.dims.x) - 0.5) * dx, (f32(R.dims.y) - 0.5) * dx, (f32(R.dims.z) - 0.5) * dx);
    var lo = vec3<f32>(0.5 * dx);
    if R.dims.w != 0u {
        lo = vec3<f32>(-hi.x, -hi.y, 0.5 * dx);
    }
    let inv = 1.0 / dir;
    let t0 = (lo - o) * inv;
    let t1 = (hi - o) * inv;
    let entree = max(max(min(t0.x, t1.x), min(t0.y, t1.y)), max(min(t0.z, t1.z), 0.0));
    let sortie = min(min(max(t0.x, t1.x), max(t0.y, t1.y)), min(max(t0.z, t1.z), t_corps));
    var touche = false;
    var t_eau = 1e30;
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
                if lumiere {
                    couleur = couleur_eau(p, normale(p), dir);
                } else {
                    couleur = ombre(normale(p), p, true);
                }
                touche = true;
                t_eau = 0.5 * (a + b);
                break;
            }
            if tb >= sortie {
                break;
            }
            ta = tb;
            fa = fb;
        }
    }
    // S457 : hors du domaine simulé, la mer de B (un domaine entier seulement).
    if lumiere && R.dims.w == 0u {
        let tb = touche_b(o, dir);
        let pb = o + dir * tb;
        // Dehors : hors de la boîte marchée (les centres extrêmes des mailles) — sans bande morte au raccord.
        let dehors = pb.x < lo.x || pb.y < lo.y || pb.x > hi.x || pb.y > hi.y;
        if tb < 1e29 && dehors && tb < t_eau && tb < t_corps {
            // La pente de B, éteinte quand une longueur d'onde tient dans moins de quelques pixels (l'empreinte du pixel à
            // `tb`) : au loin, la houle n'est plus résolue et scintillerait.
            let empreinte = tb * 2.0 / (R.s.y * f32(R.img.y));
            let resolue = exp(-pow(empreinte * R.houle.y * 0.5, 2.0));
            let hb = hauteur_b(pb.x);
            couleur = couleur_eau(pb, vec3<f32>(-hb.y * resolue, 0.0, 1.0), dir);
            // La perspective aérienne (S262 : 6 km d'air clair).
            couleur = mix(couleur, CIEL_H, 1.0 - exp(-tb / 6000.0));
            touche = true;
        }
    }
    if !touche && t_corps < 1e29 {
        let p = o + dir * t_corps;
        if lumiere {
            couleur = couleur_corps(p);
        } else {
            couleur = ombre(p - R.sphere.xyz, p, false);
        }
    }
    return vec4<f32>(pow(clamp(couleur, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2)), 1.0);
}
