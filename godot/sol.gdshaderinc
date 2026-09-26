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
//
// **S361 — les caustiques** : la part directe `0,4·n·soleil` est multipliée par la carte de caustiques
// (`caustiques.gdshader`, méthode directe, énergie conservée), qui couvre `carte_cote` mètres autour de `carte_origine`.
// Hors de la carte, et quand le pixel couvre plus de deux texels, la valeur tend vers 1 — sa moyenne : c'est un niveau de
// détail, pas un réglage. La méthode à rebours de S361 P3 est retirée (commit `c5faf558` pour la rejouer).
// S373 : `shader_type` et `render_mode` dans `sol.gdshader` et `sol_demi.gdshader`, qui incluent ce corps.

#include "res://ciel.gdshaderinc"
// S365 : `gain_eau`, `Kd`, l'atténuation du faisceau et le mode immergé (`optique_eau.gdshaderinc`).
#include "res://optique_eau.gdshaderinc"
// S371 : la bande de B et le milieu du pixel — la caméra à demi immergée (`surface_b.gdshaderinc`).
#include "res://surface_b.gdshaderinc"

uniform vec3 albedo_sable = vec3(0.34, 0.30, 0.24);

uniform bool caustiques = false;
uniform sampler2D carte_caustiques : filter_linear, repeat_disable;
uniform vec2 carte_origine = vec2(0.0);
uniform float carte_cote = 64.0;
uniform float carte_texels = 1024.0;
// La carte est écrite divisée par `carte_echelle`, pour tenir dans son format.
uniform float carte_echelle = 32.0;
// Sens de la carte en y (le repère d'écran de la vue orthographique), éprouvé par le contrôle à onde selon y.
uniform bool carte_retournee = false;
// Contrôle (S361) : 0 rendu ; 1 l'éclairement focalisé C en sortie directe — rouge C/4, vert fract(10·C), qui le
// précise au-delà des 8 bits ; 2 la même sur la scène, rouge C/20, vert fract(C). `soleil_controle` non nul impose le
// soleil (coordonnées de B). S365 : 3, la caméra dans l'eau, la transmission de la ligne de visée `exp(−c·d)`.
uniform int controle = 0;
uniform vec3 soleil_controle = vec3(0.0);

varying vec2 plan;
varying float profondeur_fond;

// S363 : le hachage entier du ciel (`ciel.gdshaderinc`), sans la sensibilité à l'arrondi de `fract(sin(x)·43758)`.
float hachage(vec2 q) {
    return hachage_ciel(q);
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

/// **Le disque du soleil** (S361) : 0,53° de diamètre apparent, soit un demi-angle de 4,65·10⁻³ rad dans l'air et
/// 4,65·10⁻³/1,34 = 3,47·10⁻³ dans l'eau ; au fond, l'image d'un point de la surface est un disque de rayon `H·3,47·10⁻³`
/// — ce qui borne la concentration d'un pli. Moyenne sur huit points du disque et son centre (énergie conservée).
const float DEMI_ANGLE_SOLEIL_EAU = 3.47e-3;

float lire_carte(vec2 uv) {
    return carte_echelle * texture(carte_caustiques, vec2(uv.x, carte_retournee ? 1.0 - uv.y : uv.y)).r;
}

/// L'éclairement direct focalisé lu dans la carte, moyenné sur l'image du disque solaire ; 1 hors de la carte, et vers 1
/// quand le pixel couvre plus de deux texels (fondu jusqu'à six) ou près du bord (le dernier dixième).
float focalisation() {
    vec2 uv = (plan - carte_origine) / carte_cote + 0.5;
    if (uv.x <= 0.0 || uv.x >= 1.0 || uv.y <= 0.0 || uv.y >= 1.0) { return 1.0; }
    float rayon_disque = max(profondeur_fond, 0.0) * DEMI_ANGLE_SOLEIL_EAU / carte_cote;
    float c = lire_carte(uv) / 9.0;
    for (int i = 0; i < 8; i++) {
        float a = TAU * (float(i) + 0.5) / 8.0;
        float r = rayon_disque * (i % 2 == 0 ? 0.55 : 0.95);
        c += lire_carte(uv + r * vec2(cos(a), sin(a))) / 9.0;
    }
    float texel = carte_cote / carte_texels;
    float empreinte = max(length(dFdx(plan)), length(dFdy(plan)));
    float bord = min(min(uv.x, 1.0 - uv.x), min(uv.y, 1.0 - uv.y));
    float fondu = max(clamp((empreinte - 2.0 * texel) / (4.0 * texel), 0.0, 1.0), 1.0 - clamp(bord / 0.1, 0.0, 1.0));
    return mix(c, 1.0, fondu);
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
    float c = caustiques ? focalisation() : 1.0;
    ALBEDO = albedo_sable * variation * gain_eau * (0.6 + 0.4 * max(dot(n_b, soleil), 0.0) * c);
    // S365 — **vu de l'eau** (liste 8.6) : l'éclairement descendu jusqu'au fond, `exp(−Kd·H)` — vu d'en haut, c'est
    // `eau.gdshader` qui l'applique —, puis les `d` mètres d'eau jusqu'à l'œil (`a_travers_l_eau`).
    // S371 : le milieu du pixel, décidé à l'objectif (`surface_b.gdshaderinc`) ; dans la transition, les deux vues.
    vec3 monde = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
    float d = length(monde - CAMERA_POSITION_WORLD);
    vec3 milieu = milieu_du_pixel(CAMERA_POSITION_WORLD, normalize(monde - CAMERA_POSITION_WORLD));
    if (milieu.x > 0.0) {
        vec3 vue = normalize(monde - CAMERA_POSITION_WORLD);
        vec3 omega = vec3(vue.x, -vue.z, vue.y);
        vec3 dessous = a_travers_l_eau(ALBEDO * exp(-kd * max(profondeur_fond, 0.0)), d, omega, milieu.y);
        ALBEDO = milieu.x >= 1.0 ? dessous : mix(ALBEDO, dessous, milieu.x);
    }
#ifdef BRUME_PAR_PIXEL
    // S373 : la perspective aérienne, sur la part d'air du pixel seulement — variante `sol_demi.gdshader`.
    vec3 vue_air = normalize(monde - CAMERA_POSITION_WORLD);
    FOG = (controle == 0 && !controle_milieu)
        ? brume_air(vec3(vue_air.x, -vue_air.z, vue_air.y), length(VERTEX), 1.0 - milieu.x) : vec4(0.0);
#endif
    if (controle_milieu) {
        ALBEDO = vec3(milieu.x, 0.0, 0.0);
    } else if (controle == 3) {
        ALBEDO = exp(-attenuation_c * d);
    } else if (controle == 1) {
        ALBEDO = vec3(c / 4.0, fract(10.0 * c), 0.0);
    } else if (controle == 2) {
        ALBEDO = vec3(c / 20.0, fract(c), 0.0);
    }
}
