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

// S381 — **l'éclairement de la scène**, `E/π` (ADR-177 D2 : il tient lieu de l'éclairement de la surface ; 2 par défaut),
// déclaré ici depuis S381 (auparavant dans `optique_eau.gdshaderinc`) : l'eau, le fond, les parois l'emploient, et le ciel
// couvert en tire sa luminance.
uniform float gain_eau = 2.0;

// S381 — **LE CIEL DE PLUIE** (ADR-205, pièce 3). `couvert`, de 0 (ciel clair) à 1 (ciel couvert, nimbostratus) ; la
// pluie le met à 1, la météo le commandera. Sous un ciel couvert d'épaisseur optique de quelques dizaines, le soleil direct
// est nul (`e^(−τ/μ)`) : ni disque, ni éclat, ni caustiques, ni ombres. La luminance est celle du **ciel couvert normalisé
// de la CIE** (Moon et Spencer 1942), `L(h) = Lz·(1 + 2·sin h)/3`, **neutre** (la lumière du jour D65, le blanc de sRGB ;
// mesuré presque neutre sur les photographies de S381). L'éclairement horizontal qu'il donne, `(7π/9)·Lz`, est tenu égal
// à celui du ciel clair de la scène — **l'œil s'adapte**, hypothèse déclarée : `Lz = (9/7)·gain·(0,6 + 0,4·sin h_soleil)`.
uniform float couvert = 0.0;
// L'albédo du sol qui renvoie la lumière du ciel couvert vers les faces tournées vers le bas (herbe, béton : ≈ 0,2).
const float ALBEDO_SOL_COUVERT = 0.2;

/// L'éclairement horizontal du ciel clair de la scène, rapporté à `gain` : `0,6 + 0,4·sin h_soleil` (0,939).
float eclairement_horizontal_clair() {
    return 0.6 + 0.4 * normalize(SOLEIL_B).z;
}

/// La luminance du ciel couvert de la CIE dans la direction `rayon` (B) ; sous l'horizon, celle de l'horizon.
vec3 ciel_couvert(vec3 rayon) {
    float lz = (9.0 / 7.0) * gain_eau * eclairement_horizontal_clair();
    return vec3(lz * (1.0 + 2.0 * clamp(rayon.z, 0.0, 1.0)) / 3.0);
}

/// S382 — **Le ciel vu d'une face inclinée**, formes closes (`cos β = n_z`, ciel au-dessus de l'horizon seulement) : le
/// ciel uniforme, `U(β) = ∫ max(n·ω, 0) dω = π(1 + cos β)/2` ; le ciel couvert de la CIE, en unités de `Lz`,
/// `E(β) = [U(β) + (4/3)·((π − β)·cos β + sin β)]/3` — le second terme est `2·∫ sin h·max(n·ω, 0) dω`, intégré sur le
/// fuseau que découpent l'horizon et le plan de la face. `E(0) = 7π/9`. Contrôlée contre l'intégrale numérique : 4·10⁻⁸.
float ciel_uniforme_incline(float nz) {
    return PI * 0.5 * (1.0 + clamp(nz, -1.0, 1.0));
}
float ciel_couvert_incline(float nz) {
    float c = clamp(nz, -1.0, 1.0);
    return (ciel_uniforme_incline(c) + (4.0 / 3.0) * ((PI - acos(c)) * c + sqrt(max(1.0 - c * c, 0.0)))) / 3.0;
}

/// **Le facteur d'éclairement** d'une surface de normale `n` (B, unitaire) : sous le ciel clair, le modèle de l'afficheur,
/// `0,6 + 0,4·max(n·soleil, 0)·direct` (`direct` : les caustiques, ou 1) ; sous le ciel couvert, l'éclairement horizontal
/// conservé, réparti par la CIE — une face inclinée en reçoit `E(β)/E(0)` (0,3962 à la verticale ; S382, forme close, au
/// lieu de l'interpolation de S381, fausse de 4,3 % au pire) —, plus la lumière renvoyée par le sol, `albédo·(1 − n_z)/2`.
/// Mêlés par `couvert` ; ciel clair : l'expression d'avant, au bit.
float eclairage(vec3 n, vec3 soleil, float direct) {
    float clair = 0.6 + 0.4 * max(dot(n, soleil), 0.0) * direct;
    if (couvert <= 0.0) {
        return clair;
    }
    float ciel = ciel_couvert_incline(n.z) / (7.0 * PI / 9.0);
    float sol = ALBEDO_SOL_COUVERT * 0.5 * (1.0 - n.z);
    return mix(clair, eclairement_horizontal_clair() * (ciel + sol), couvert);
}

/// La part du soleil direct qui passe : 1 sous le ciel clair, 0 sous le ciel couvert.
float soleil_direct() {
    return 1.0 - couvert;
}

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
    vec3 clair = c + vec3(1.0, 0.95, 0.85) * (pow(d, 1024.0) * 4.0 + pow(d, 32.0) * 0.15);
    // S381 : le ciel de pluie ; ciel clair, la valeur d'avant.
    if (couvert > 0.0) {
        return mix(clair, ciel_couvert(rayon), couvert);
    }
    return clair;
}

/// S373 — **la perspective aérienne**, réécrite ici depuis celle de Godot (`fog_process`, `scene_forward_clustered.glsl`,
/// 4.4-stable) pour se régler **par pixel** : quantité `1 − exp(−ρ·d)` sur la distance `d` de l'œil au fragment ; couleur,
/// la radiance du ciel dans la direction de visée (`rayon`, B) — le ciel dont Godot tire le cube de radiance qu'il lit
/// là (`ciel.gdshader`, quatre octaves), sans le flou de ses niveaux. `part_air` : 1 vu de l'air, 0 vu de l'eau
/// (`milieu_du_pixel`) ; S366 et S371 l'éteignaient d'un bloc. Rendue par la sortie `FOG` du nuanceur : la brume du moteur
/// ne touche plus ce matériau. `brume_densite` : celle de l'environnement (`mer.gd`), 0 dans les contrôles.
uniform float brume_densite = 0.00012;

vec4 brume_air(vec3 rayon, float distance, float part_air) {
    float quantite = 1.0 - exp(min(0.0, -distance * brume_densite));
    return vec4(ciel_b(rayon, 4), quantite * part_air);
}
