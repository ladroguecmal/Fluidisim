// S359 — LE CIEL CLAIR DE L'AFFICHEUR, une seule source pour le ciel de Godot et pour les reflets de l'eau.
//
// Port de `viewer/src/water.wgsl` (`sky_detail`, `clouds`, `value_noise`, `hash2`), chemin « ciel clair » de
// `--meilleur` : horizon et zénith relevés sur la photographie de référence de l'utilisateur (S261, R14), en linéaire ;
// nuages sur un plan à 1,2 km, quatre octaves pour le ciel, deux dans les reflets (S262) ; soleil. Coordonnées de B :
// x à l'est, y au nord, z en haut — un rayon de Godot (x, y, z) est (x, −z, y) dans B.

const vec3 CIEL_HORIZON = vec3(0.694, 0.838, 0.930);
const vec3 CIEL_ZENITH = vec3(0.015, 0.150, 0.600);
const vec3 SOLEIL_B = vec3(-0.4, 0.3, 0.8);

// S363 — **le ciel calé sur la photographie de référence de l'utilisateur** (S308, `ciel_mesure` de l'afficheur,
// jamais passé dans `--meilleur`) : `ciel(s) = H·exp(−K·s)`, `s = sin(élévation)`, `K = −ln F / sin(haut du cadre)` — une
// extinction par canal, la signature de Rayleigh : le rouge s'effondre vers le zénith, le bleu à peine. `H` et `F` sont
// les valeurs linéaires mesurées de la photographie, de l'horizon au haut du cadre ; l'élévation du haut du cadre est
// inconnue — 25°, un cadrage ordinaire, hypothèse déclarée. Le « ciel clair » d'avant est plat (sommet à 0,81 de
// l'horizon, la photographie à 0,36) ; `ciel_mesure = false` le rend, en témoin.
const vec3 CIEL_MESURE_HORIZON = vec3(0.311, 0.554, 0.795);
const vec3 CIEL_MESURE_F = vec3(0.139, 0.327, 0.722);
uniform bool ciel_mesure = true;
uniform float ciel_mesure_haut_deg = 25.0;

/// S363 — **un hachage entier** : PCG (O'Neill 2014), sous la forme de Jarzynski et Olano (2020, *Hash Functions for
/// GPU Rendering*, JCGT 9(3)). L'ancien `fract(sin(x)·43758)`, avec `x` de l'ordre de 10⁴, changeait du tout au tout pour
/// un ulp de `x` : Godot compilant `i + (1, 0)` autrement de part et d'autre de la colonne centrale, le ciel y avait une
/// couture — mesurée, un saut 10,7 fois la médiane de ses voisins (`outils/couture_ciel.py`). En entiers, rien ne dépend
/// de l'arrondi : `q` est déjà un nœud entier du réseau.
uint pcg_hachage(uint v) {
    uint etat = v * 747796405u + 2891336453u;
    uint mot = ((etat >> ((etat >> 28u) + 4u)) ^ etat) * 277803737u;
    return (mot >> 22u) ^ mot;
}

float hachage_ciel(vec2 q) {
    ivec2 i = ivec2(floor(q + 0.5));
    return float(pcg_hachage(uint(i.x) + pcg_hachage(uint(i.y)))) * (1.0 / 4294967295.0);
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
    vec3 c;
    if (ciel_mesure) {
        vec3 k = -log(CIEL_MESURE_F) / max(sin(radians(ciel_mesure_haut_deg)), 1e-3);
        c = CIEL_MESURE_HORIZON * exp(-k * clamp(rayon.z, 0.0, 1.0));
    } else {
        float t = pow(clamp(rayon.z, 0.0, 1.0), 0.35);
        c = mix(CIEL_HORIZON, CIEL_ZENITH, t);
    }
    c = mix(c, vec3(0.92, 0.93, 0.95), nuages(rayon, octaves));
    float d = max(dot(rayon, soleil), 0.0);
    return c + vec3(1.0, 0.95, 0.85) * (pow(d, 1024.0) * 4.0 + pow(d, 32.0) * 0.15);
}
