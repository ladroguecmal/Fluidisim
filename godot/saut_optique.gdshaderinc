// S461 — C11 : L'OPTIQUE DE LA SCÈNE DU SAUT, commune à l'eau du domaine (`saut_eau.gdshader`) et à la mer de B au-delà
// (`saut_mer.gdshader`). Celle de `surface_carte.wgsl` (S457), elle-même portée d'ici : le ciel et l'éclairement de la scène
// (`ciel.gdshaderinc`), `R(0⁻)`, `E/π` et `Kd` (`optique_eau.gdshaderinc`), Fresnel exact, l'éclat du soleil, la colonne d'eau de
// Maritorena jusqu'au fond de sable — ou au corps — par réfraction. Coordonnées de B (x à l'est, y au nord, z en haut) ; Godot :
// un point (x, y, z) de B est (x, z, −y).

#include "res://ciel.gdshaderinc"
#include "res://optique_eau.gdshaderinc"
// S465 : les rides de la pluie (ADR-202 D3, S379), leur horloge à part.
#include "res://pluie.gdshaderinc"
uniform float temps_pluie = 0.0;

// Le corps (le joueur) : centre (B) et rayon ; rayon nul : pas de corps. S466 : la demi-longueur de son axe vertical (une
// capsule ; 0 : la sphère).
uniform vec4 corps = vec4(0.0);
uniform float corps_demi_longueur = 0.0;
// S467 : son albédo — celui du maillage au-dessus de l'eau (`joueur.gdshader`, `ALBEDO_JOUEUR` de `saut.gd`).
uniform vec3 corps_albedo = vec3(0.22, 0.22, 0.24);
// La houle B (`a`, `k`, `ω`, `φ`), le niveau moyen et l'instant.
uniform vec4 houle = vec4(0.0);
uniform float niveau = 1.4;
uniform float temps = 0.0;
// Le domaine simulé : son étendue en x et y (m).
uniform vec2 etendue = vec2(4.0, 4.0);

const vec3 SABLE = vec3(0.34, 0.30, 0.24);

// S463 — **la surface fine** de S360 : les pentes des deux cascades FFT de la queue de B (`detail.gd`, 32 m et 4 m), lues au niveau
// de détail de l'empreinte du pixel, échelonnées par `force_detail` (la queue exportée est celle d'un vent établi ; la scène est
// calme — 0,5 par défaut, tranché S463) et ajoutées à la normale des faces tournées vers le haut.
uniform sampler2D detail_a0 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform sampler2D detail_a1 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform vec2 detail_cotes = vec2(32.0, 4.0);
uniform float detail_texels = 256.0;
uniform float force_detail = 0.5;
uniform bool detail_actif = false;

vec2 pente_fine(vec2 p, float empreinte) {
    if (!detail_actif) {
        return vec2(0.0);
    }
    float l0 = log2(max(empreinte * detail_texels / detail_cotes.x, 1.0));
    float l1 = log2(max(empreinte * detail_texels / detail_cotes.y, 1.0));
    vec2 s = textureLod(detail_a0, p / detail_cotes.x, l0).xy + textureLod(detail_a1, p / detail_cotes.y, l1).xy;
    return force_detail * s;
}

// S462 — **les caustiques** : la focalisation de la lumière par la surface simulée (l'afficheur, `saut_caustiques.bin` : `C/8` sur
// 8 bits, aux coordonnées du fond, pour une lumière verticale), et `domaine` son étendue (m) ; au-delà, celle de B, analytique.
uniform sampler2D caustiques_tex : filter_linear, repeat_disable;
uniform vec2 domaine = vec2(4.0, 4.0);
uniform bool caustiques = true;

// La focalisation au point `q` du fond (B) : le motif d'une lumière verticale, décalé de l'obliquité du soleil réfracté.
float focalisation(vec3 q) {
    // S465 : sous le ciel couvert, le soleil direct est éteint, les caustiques avec lui.
    if (!caustiques || couvert >= 1.0) {
        return 1.0;
    }
    vec3 r = refract(-normalize(SOLEIL_B), vec3(0.0, 0.0, 1.0), 1.0 / indice);
    vec2 p = q.xy - r.xy * (niveau / -r.z);
    if (p.x > 0.0 && p.y > 0.0 && p.x < domaine.x && p.y < domaine.y) {
        return texture(caustiques_tex, p / domaine).r * 8.0;
    }
    // B : `1/|1 + D·η_xx|`, `D = h·(1 − 1/n)`.
    float theta = houle.y * p.x - houle.z * temps + houle.w;
    float exx = -houle.x * houle.y * houle.y * cos(theta);
    return min(1.0 / max(abs(1.0 + niveau * (1.0 - 1.0 / indice) * exx), 0.125), 8.0);
}

vec3 b_vers_godot(vec3 p) {
    return vec3(p.x, p.z, -p.y);
}

vec3 godot_vers_b(vec3 p) {
    return vec3(p.x, -p.z, p.y);
}

float fresnel_exact(float c, float n) {
    float st2 = (1.0 - c * c) / (n * n);
    float ct = sqrt(max(1.0 - st2, 0.0));
    float rs = (c - n * ct) / (c + n * ct);
    float rp = (n * c - ct) / (n * c + ct);
    return 0.5 * (rs * rs + rp * rp);
}

// S466 — le point de l'axe du corps le plus proche de `p` (une capsule verticale ; le centre si la demi-longueur est nulle).
vec3 axe_corps(vec3 p) {
    float l = corps_demi_longueur;
    return vec3(corps.xy, clamp(p.z, corps.z - l, corps.z + l));
}

// La distance au corps le long de `o + t·d` (1e30 : manqué) : une capsule verticale (Quílez), la sphère si `L = 0` — celle de
// `surface_carte.wgsl`.
float touche_corps(vec3 o, vec3 d) {
    float r = corps.w;
    if (r <= 0.0) {
        return 1e30;
    }
    vec3 pa = corps.xyz - vec3(0.0, 0.0, corps_demi_longueur);
    vec3 pb = corps.xyz + vec3(0.0, 0.0, corps_demi_longueur);
    vec3 ba = pb - pa;
    vec3 oa = o - pa;
    float baba = dot(ba, ba);
    float bard = dot(ba, d);
    float baoa = dot(ba, oa);
    float rdoa = dot(d, oa);
    float oaoa = dot(oa, oa);
    float a = baba - bard * bard;
    float b = baba * rdoa - baoa * bard;
    float c = baba * oaoa - baoa * baoa - r * r * baba;
    float h = b * b - a * c;
    if (baba > 0.0 && h >= 0.0) {
        float t = (-b - sqrt(h)) / a;
        float y = baoa + t * bard;
        if (y > 0.0 && y < baba && t > 1e-4) {
            return t;
        }
    }
    // Les calottes : la sphère de l'extrémité la plus proche du rayon.
    vec3 oc = oa;
    if (baba > 0.0 && baoa + (-b / max(a, 1e-12)) * bard > 0.5 * baba) {
        oc = o - pb;
    }
    float bb = dot(oc, d);
    float cc = dot(oc, oc) - r * r;
    float hh = bb * bb - cc;
    if (hh < 0.0) {
        return 1e30;
    }
    float t = -bb - sqrt(hh);
    return t > 1e-4 ? t : 1e30;
}

vec3 couleur_corps(vec3 p) {
    return corps_albedo * gain_eau * eclairage(normalize(p - axe_corps(p)), normalize(SOLEIL_B), 1.0);
}

float bruit_sable(vec2 q) {
    vec2 i = floor(q);
    vec2 f = fract(q);
    vec2 u = f * f * (3.0 - 2.0 * f);
    float a = hachage_ciel(i);
    float b = hachage_ciel(i + vec2(1.0, 0.0));
    float c = hachage_ciel(i + vec2(0.0, 1.0));
    float d = hachage_ciel(i + vec2(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

vec3 couleur_sable(vec3 p) {
    float variation = 0.85 + 0.3 * (0.6 * bruit_sable(p.xy * 0.9) + 0.4 * bruit_sable(p.xy * 3.7));
    // S462 : la part directe de l'éclairement focalisée (comme `sol.gdshader`, S361).
    return SABLE * variation * gain_eau * eclairage(vec3(0.0, 0.0, 1.0), normalize(SOLEIL_B), focalisation(p));
}

// La surface de l'eau au point `p`, normale `n0` (vers l'air), vue dans la direction `d` (B) ; `empreinte` : la taille du pixel
// à cette distance, m.
vec3 couleur_eau(vec3 p, vec3 n0, vec3 d, float empreinte) {
    vec3 n = normalize(n0);
    // S463 : la surface fine, sur ce qui regarde vers le haut ; S465 : les rides de la pluie (leur variance non rendue).
    vec2 s_fine = pente_fine(p.xy, empreinte);
    if (pluie.x > 0.0) {
        vec3 cv_pluie;
        float coeur_pluie;
        s_fine += pluie_rides(p.xy, temps_pluie, vec2(empreinte, 0.0), vec2(0.0, empreinte), cv_pluie, coeur_pluie);
    }
    n = normalize(n + vec3(-s_fine, 0.0) * max(n.z, 0.0));
    if (dot(n, d) > 0.0) {
        n = -n;
    }
    vec3 soleil = normalize(SOLEIL_B);
    float f = fresnel_exact(clamp(dot(-d, n), 0.0, 1.0), indice);
    vec3 reflet = reflect(d, n);
    float eclat = pow(max(dot(reflet, soleil), 0.0), 180.0);
    vec3 r = refract(d, n, 1.0 / indice);
    vec3 vu = vec3(0.0);
    vec3 t_col = vec3(0.0);
    if (r.z < -1e-3) {
        float t_fond = p.z / -r.z;
        float t_c = touche_corps(p, r);
        vec3 q = p + r * t_fond;
        vec3 c = couleur_sable(q);
        if (t_c < t_fond) {
            q = p + r * t_c;
            c = couleur_corps(q);
        }
        t_col = exp(-kd * (max(p.z - q.z, 0.0) + length(q - p)));
        vu = c * t_col;
    }
    vec3 corps_eau = R0 * gain_eau * eclairage(n, soleil, 1.0);
    vec3 sous = corps_eau * (1.0 - t_col) + vu;
    // S465 : sous le ciel couvert, plus d'éclat (`soleil_direct`).
    return mix(sous, ciel_b(reflet, 2), f) + vec3(1.0, 0.95, 0.85) * eclat * 1.2 * soleil_direct();
}

// La surface de B : `h + a·cos(k·x − ω·t + φ)` et sa pente en x.
vec2 hauteur_b(float x) {
    float theta = houle.y * x - houle.z * temps + houle.w;
    return vec2(niveau + houle.x * cos(theta), -houle.x * houle.y * sin(theta));
}

// La profondeur d'un point B, à écrire dans DEPTH.
float profondeur_ecran(vec3 p_b, mat4 vue, mat4 projection) {
    vec4 clip = projection * vue * vec4(b_vers_godot(p_b), 1.0);
    return clip.z / clip.w;
}
