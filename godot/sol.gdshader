// S359 — LE FOND DE LA SCÈNE CÔTIÈRE : du sable, procédural, sans texture téléchargée.
//
// Ce fond n'existe que pour voir l'eau : la lumière qu'elle absorbe et diffuse selon la profondeur (liste 8.5). Il
// n'agit pas sur les vagues — B ne connaît pas la bathymétrie (liste 2.7, absente). Albédo moyen 0,29 : sable
// quartzeux mouillé, *à calibrer* sur une référence. Rides de sable et variation d'albédo : habillage, sans provenance
// physique autre que leur ordre de grandeur (rides de houle, 0,5 à 1 m).
shader_type spatial;
render_mode diffuse_lambert, specular_schlick_ggx;

uniform vec3 albedo_sable = vec3(0.34, 0.30, 0.24);

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
    vec3 n_monde = normalize(vec3(-pente.x, 1.0, pente.y));
    NORMAL = normalize((VIEW_MATRIX * vec4(n_monde, 0.0)).xyz);
    float variation = 0.85 + 0.3 * (0.6 * bruit(plan * 0.9) + 0.4 * bruit(plan * 3.7));
    ALBEDO = albedo_sable * variation;
    ROUGHNESS = 0.95;
    SPECULAR = 0.2;
}
