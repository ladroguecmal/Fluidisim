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

varying vec2 plan;

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
    ALBEDO = albedo_sable * variation * gain_eau * (0.6 + 0.4 * max(dot(n_b, normalize(SOLEIL_B)), 0.0));
}
