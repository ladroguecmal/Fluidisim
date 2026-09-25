// S359 — LE FOND DE LA SCÈNE CÔTIÈRE : du sable, procédural, sans texture téléchargée.
//
// Ce fond n'existe que pour voir l'eau : la lumière qu'elle absorbe et diffuse selon la profondeur (liste 8.5). Il
// n'agit pas sur les vagues — B ne connaît pas la bathymétrie (liste 2.7, absente). Albédo moyen 0,29 : sable
// quartzeux mouillé, *à calibrer* sur une référence. Rides de sable et variation d'albédo : habillage, sans provenance
// physique autre que leur ordre de grandeur (rides de houle, 0,5 à 1 m).
//
// **Éclairé comme le corps d'eau** (S359 P6) — `gain·(0,6 + 0,4·max(n·soleil, 0))`, le modèle de l'afficheur — et non
// par Godot : le mélange de Maritorena demande le fond et l'eau sous le même éclairement. Il n'est vu qu'à travers
// l'eau.
shader_type spatial;
render_mode unshaded;

#include "res://ciel.gdshaderinc"

uniform vec3 albedo_sable = vec3(0.34, 0.30, 0.24);
uniform float gain_eau = 2.0;

// S361 — **les caustiques** : le soleil réfracté par la surface éclaire le fond selon la projection
// `X_f = X_s + (H + η)·p(∇η(X_s))`, `p` la pente horizontale du rayon réfracté (Snell exact) ; l'éclairement direct est
// multiplié par `1/|det J|`, `J = I + p⊗∇η + (H + η)·(∂p/∂s)·Hess η`, au point de départ `X_s`, retrouvé par point fixe
// depuis le point du fond (quatre itérations). Surface : la bande (analytique, mêmes composantes que l'eau) et la cascade
// de 32 m de la surface fine (λ ≥ 0,5 m) ; la cascade de 4 m est exclue — sa focale est de l'ordre du mètre, bien
// au-dessus de ce fond. `det_min` plafonne la concentration, *à calibrer* : le disque du soleil l'arrondit en vrai.
uniform vec4 bande[64];
uniform int n_bande = 0;
uniform sampler2D detail_a0 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform sampler2D detail_c0 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform float detail_cote0 = 32.0;
uniform bool caustiques = false;
uniform bool caustiques_detail = true;
uniform float det_min = 0.05;
uniform float indice = 1.34;
// Contrôle (S361) : 0 rendu ; 1 l'éclairement focalisé C en sortie directe — rouge C/4, vert fract(10·C), qui le
// précise au-delà des 8 bits ; 2 la même sur la scène, rouge C/20, vert fract(C) ; `profondeur_controle` > 0 impose H,
// `soleil_controle` non nul impose le soleil (coordonnées de B).
uniform int controle = 0;
uniform float profondeur_controle = 0.0;
uniform vec3 soleil_controle = vec3(0.0);

varying vec2 plan;
varying float profondeur_fond;

float hachage(vec2 q) {
    return fract(sin(dot(q, vec2(127.1, 311.7))) * 43758.5453);
}

float bruit(vec2 q) {
    vec2 i = floor(q);
    vec2 f = fract(q);
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hachage(i), hachage(i + vec2(1.0, 0.0)), u.x), mix(hachage(i + vec2(0.0, 1.0)), hachage(i + vec2(1.0, 1.0)), u.x), u.y);
}

void vertex() {
    vec3 monde = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
    plan = vec2(monde.x, -monde.z);
    profondeur_fond = -monde.y;
}

/// La surface en `x` (B) : pente, η, hessienne (xx, yy, xy) — la bande, puis la cascade de 32 m.
void surface(vec2 x, out vec2 pente, out float eta, out vec3 hess) {
    pente = vec2(0.0);
    eta = 0.0;
    hess = vec3(0.0);
    for (int i = 0; i < n_bande; i++) {
        vec4 c = bande[i];
        float phase = dot(c.yz, x) + c.w;
        float sn = sin(phase);
        eta += c.x * sn;
        pente += c.x * cos(phase) * c.yz;
        hess -= c.x * sn * vec3(c.y * c.y, c.z * c.z, c.y * c.z);
    }
    if (caustiques_detail) {
        vec2 uv = x / detail_cote0;
        vec4 a = texture(detail_a0, uv);
        vec4 h = texture(detail_c0, uv);
        pente += a.xy;
        eta += h.w;
        hess += h.xyz;
    }
}

/// La pente horizontale du rayon de soleil réfracté par une facette de pente `s` : `r.xy/(−r.z)`.
vec2 rayon(vec2 s, vec3 soleil) {
    vec3 n = normalize(vec3(-s, 1.0));
    vec3 r = refract(-soleil, n, 1.0 / indice);
    return r.xy / max(-r.z, 1e-4);
}

/// L'éclairement direct focalisé au point du fond `xf`, à la profondeur `h` sous le plan moyen.
float focalisation(vec2 xf, float h, vec3 soleil) {
    vec2 xs = xf - h * rayon(vec2(0.0), soleil);
    vec2 s;
    float eta;
    vec3 hess;
    for (int i = 0; i < 4; i++) {
        surface(xs, s, eta, hess);
        xs = xf - (h + eta) * rayon(s, soleil);
    }
    surface(xs, s, eta, hess);
    vec2 p = rayon(s, soleil);
    float d = 1e-3;
    vec2 m0 = (rayon(s + vec2(d, 0.0), soleil) - rayon(s - vec2(d, 0.0), soleil)) / (2.0 * d);
    vec2 m1 = (rayon(s + vec2(0.0, d), soleil) - rayon(s - vec2(0.0, d), soleil)) / (2.0 * d);
    mat2 m = mat2(m0, m1);
    mat2 hs = mat2(vec2(hess.x, hess.z), vec2(hess.z, hess.y));
    mat2 j = mat2(1.0) + outerProduct(p, s) + (h + eta) * (m * hs);
    return 1.0 / max(abs(determinant(j)), det_min);
}

void fragment() {
    // Rides de 0,7 m, crêtes sinueuses ; leur pente perturbe la normale.
    float sinuosite = 1.5 * bruit(plan * 0.15);
    float phase = TAU * (dot(plan, vec2(0.28, 0.96)) / 0.7 + sinuosite);
    float amplitude = 0.06 * (0.5 + bruit(plan * 0.4));
    vec2 pente = amplitude * TAU / 0.7 * cos(phase) * vec2(0.28, 0.96);
    // La normale du fond : celle de la grille (`NORMAL`, vue) et la pente des rides, en coordonnées de B.
    vec3 n_grille = (INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
    vec3 n_b = normalize(vec3(n_grille.x, -n_grille.z, n_grille.y) + vec3(-pente, 0.0));
    float variation = 0.85 + 0.3 * (0.6 * bruit(plan * 0.9) + 0.4 * bruit(plan * 3.7));
    vec3 soleil = dot(soleil_controle, soleil_controle) > 0.0 ? normalize(soleil_controle) : normalize(SOLEIL_B);
    float h = profondeur_controle > 0.0 ? profondeur_controle : max(profondeur_fond, 0.0);
    float c = caustiques ? focalisation(plan, h, soleil) : 1.0;
    ALBEDO = albedo_sable * variation * gain_eau * (0.6 + 0.4 * max(dot(n_b, soleil), 0.0) * c);
    if (controle == 1) {
        ALBEDO = vec3(c / 4.0, fract(10.0 * c), 0.0);
    } else if (controle == 2) {
        // La scène : C jusqu'au plafond 1/det_min ; rouge C/20, vert fract(C).
        ALBEDO = vec3(c / 20.0, fract(c), 0.0);
    }
}
