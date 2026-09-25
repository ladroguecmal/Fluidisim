// S359 — LE CIEL CLAIR DE L'AFFICHEUR, une seule source pour le ciel de Godot et pour les reflets de l'eau.
//
// Port de `viewer/src/water.wgsl` (`sky_detail`, `clouds`, `value_noise`, `hash2`), chemin « ciel clair » de
// `--meilleur` : horizon et zénith relevés sur la photographie de référence de l'utilisateur (S261, R14), en linéaire ;
// nuages sur un plan à 1,2 km, quatre octaves pour le ciel, deux dans les reflets (S262) ; soleil. Coordonnées de B :
// x à l'est, y au nord, z en haut — un rayon de Godot (x, y, z) est (x, −z, y) dans B.

const vec3 CIEL_HORIZON = vec3(0.694, 0.838, 0.930);
const vec3 CIEL_ZENITH = vec3(0.015, 0.150, 0.600);
const vec3 SOLEIL_B = vec3(-0.4, 0.3, 0.8);

float hachage_ciel(vec2 q) {
    float h = dot(q, vec2(127.1, 311.7));
    return fract(sin(h) * 43758.5453);
}

float bruit_ciel(vec2 q) {
    vec2 i = floor(q);
    vec2 f = fract(q);
    vec2 u = f * f * (3.0 - 2.0 * f);
    float a = hachage_ciel(i);
    float b = hachage_ciel(i + vec2(1.0, 0.0));
    float c = hachage_ciel(i + vec2(0.0, 1.0));
    float d = hachage_ciel(i + vec2(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

float nuages(vec3 rayon, int octaves) {
    if (rayon.z <= 0.0) { return 0.0; }
    float fondu = smoothstep(0.05, 0.12, rayon.z) * (1.0 - smoothstep(0.30, 0.55, rayon.z));
    if (fondu <= 0.0) { return 0.0; }
    vec2 q = rayon.xy / rayon.z * 1.3;
    float n = 0.50 * bruit_ciel(q) + 0.25 * bruit_ciel(q * 2.03 + 17.0);
    if (octaves > 2) {
        n += 0.15 * bruit_ciel(q * 4.11 + 41.0) + 0.10 * bruit_ciel(q * 8.17 + 83.0);
    } else {
        n += 0.25 * 0.5;
    }
    return smoothstep(0.52, 0.80, n) * fondu;
}

/// La radiance du ciel dans la direction `rayon` (B, normé), linéaire.
vec3 ciel_b(vec3 rayon, int octaves) {
    vec3 soleil = normalize(SOLEIL_B);
    float t = pow(clamp(rayon.z, 0.0, 1.0), 0.35);
    vec3 c = mix(CIEL_HORIZON, CIEL_ZENITH, t);
    c = mix(c, vec3(0.92, 0.93, 0.95), nuages(rayon, octaves));
    float d = max(dot(rayon, soleil), 0.0);
    return c + vec3(1.0, 0.95, 0.85) * (pow(d, 1024.0) * 4.0 + pow(d, 32.0) * 0.15);
}
