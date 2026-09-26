// S379 — LES RIDES DE LA PLUIE (ADR-202 D3, ADR-203 D7) : un effet de rendu, sans simulation, dont les nombres ont une
// provenance. Les gouttes qui laissent un anneau net (D ≥ 1,5 mm) arrivent au taux de Marshall et Palmer × Atlas
// (`pluie.gd`, 447 m⁻²·s⁻¹ à 10 mm/h). Chaque anneau est un paquet d'ondes capillaires-gravité, en **deux trains** : en
// tête, les crêtes de 1,71 cm, là où la vitesse de phase est minimale (0,231 m/s) ; derrière, celles de 4,4 cm, là où la
// vitesse de groupe est minimale (0,178 m/s) — les crêtes d'espacement croissant des photographies, et le train qu'on
// résout encore 2,6 fois plus loin. L'anneau s'éteint en 0,6 s ; une petite bosse au centre le premier dixième de
// seconde. Les photographies de S379 (notes, P2) : un paquet de 2 à 5 crêtes, visible par le seul reflet — la ride
// perturbe la **normale**, jamais la couleur.
//
// Tirage : des couches de mailles carrées (`PLUIE_MAILLE`), chacune décalée au hasard à chaque période de `PLUIE_VIE` ;
// une goutte par maille et par période, née à un instant uniforme de la période, placée près du centre de la maille à
// `PLUIE_JEU` près — l'anneau ne sort jamais de sa maille, un pixel ne lit donc que sa maille, dans la période en cours
// et la précédente. Chaque couche porte `1/(maille²·vie)` gouttes par m² et par s ; `taux·maille²·vie` couches, la
// dernière partielle (une goutte n'y est présente qu'avec la probabilité de la fraction) : le taux est exact.
//
// Niveau de détail, par l'empreinte du pixel sur l'eau **dans la direction du rayon de chaque anneau** (`|Mᵀ·dir|`, `M` les
// dérivées d'écran de la position) : en vue rasante, l'empreinte s'allonge vers l'œil, et seuls les flancs d'un anneau
// gardent leurs crêtes — ce que montrent les photographies. Par train : les crêtes tant que cette empreinte est sous λ/4,
// fondues jusqu'à λ/2 (la limite de Nyquist) en **variance de pente le long du rayon** — l'anneau devient une bande
// rugueuse, anisotrope comme la pente qu'elle remplace ; les bandes, fondues de `w` à `2w` (la demi-largeur du paquet)
// dans la variance uniforme, dont la part est la moyenne de ce fondu sur quatre directions ; plus aucune boucle quand la
// plus petite empreinte dépasse `2w` du train long (5 cm). Les petites gouttes (0,5 à 1,5 mm) ne sont que variance,
// partout. La covariance rendue est celle de la pente, (xx, xy, yy).

// x : taux des anneaux (m⁻²·s⁻¹) ; y : Λ (mm⁻¹) ; z, w : moments `Σ flux·f²` des petites gouttes et des anneaux
// (`pluie.gd`, `uniformes`). Zéro : temps sec.
uniform vec4 pluie = vec4(0.0);
// Contrôle du taux (critère 2 de S379) : la sortie ne dit plus que « un cœur d'anneau de moins de 30 ms est ici ».
uniform bool controle_pluie = false;

// Les deux trains : vitesse (m/s), longueur d'onde (m), demi-largeur du paquet `w` (m, support ±2w), pente de crête d'un
// anneau neuf pour une goutte de 2 mm (**réglée sur les photographies**, pas mesurée).
const float PLUIE_C1 = 0.231;
const float PLUIE_L1 = 0.0171;
const float PLUIE_W1 = 0.01;
const float PLUIE_S1 = 0.12;
const float PLUIE_C2 = 0.178;
const float PLUIE_L2 = 0.044;
const float PLUIE_W2 = 0.025;
const float PLUIE_S2 = 0.08;
const float PLUIE_VIE = 0.6;
const float PLUIE_JEU = 0.03;
// 2·(max(c·vie + 2·w) + jeu) = 2·(0,159 + 0,03) = 0,378 m, arrondi au-dessus.
const float PLUIE_MAILLE = 0.38;
const float PLUIE_BOSSE = 0.5;
const float PLUIE_BOSSE_R = 0.006;
const float PLUIE_BOSSE_VIE = 0.1;
const int PLUIE_COUCHES_MAX = 256;
// Énergie de pente d'un train, anneau de facteur 1, intégrée sur son aire et sa vie (m²·s) :
// ½·pente²·2π·c·(∫g² = 1,6254·w)·(∫t·(1 − t/vie)⁴ dt = vie²/30).
const float PLUIE_E1 = 0.5 * PLUIE_S1 * PLUIE_S1 * 6.2831853 * PLUIE_C1 * 1.6254 * PLUIE_W1 * PLUIE_VIE * PLUIE_VIE / 30.0;
const float PLUIE_E2 = 0.5 * PLUIE_S2 * PLUIE_S2 * 6.2831853 * PLUIE_C2 * 1.6254 * PLUIE_W2 * PLUIE_VIE * PLUIE_VIE / 30.0;

uvec4 pluie_pcg4d(uvec4 v) {
    v = v * 1664525u + 1013904223u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    v ^= v >> 16u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    return v;
}

/// Hachage scalaire court (« lowbias32 », C. Wellons) : pour ce qui ne dépend que de la couche et de la période.
uint pluie_hash(uint x) {
    x ^= x >> 16u;
    x *= 0x7feb352du;
    x ^= x >> 15u;
    x *= 0x846ca68bu;
    x ^= x >> 16u;
    return x;
}

float pluie_u01(uint h) {
    return float(h >> 8u) * (1.0 / 16777216.0);
}

/// L'empreinte du pixel dans la direction `d` (unitaire) : `|Mᵀ·d|`, `M = [dpdx | dpdy]`.
float pluie_empreinte(vec2 dpdx, vec2 dpdy, vec2 d) {
    return length(vec2(dot(dpdx, d), dot(dpdy, d)));
}

/// La part d'un train rendue en variance uniforme : son fondu de bande, moyen sur quatre directions.
float pluie_part_uniforme(vec2 dpdx, vec2 dpdy, float w) {
    return 0.25 * (smoothstep(w, 2.0 * w, pluie_empreinte(dpdx, dpdy, vec2(1.0, 0.0)))
        + smoothstep(w, 2.0 * w, pluie_empreinte(dpdx, dpdy, vec2(0.0, 1.0)))
        + smoothstep(w, 2.0 * w, pluie_empreinte(dpdx, dpdy, vec2(0.7071068, 0.7071068)))
        + smoothstep(w, 2.0 * w, pluie_empreinte(dpdx, dpdy, vec2(0.7071068, -0.7071068))));
}

/// Un train d'un anneau en un point : ajoute sa pente résolue à `s`, sa variance de bande (le long de `dir`) à `cv`.
void pluie_train(float x, float w, float lambda, float pente, float fr, vec2 dir, inout vec2 s, inout vec3 cv) {
    if (abs(x) >= 2.0 * w) {
        return;
    }
    float u = x / (2.0 * w);
    float amp = pente * (1.0 - u * u) * (1.0 - u * u);
    float a = 1.0 - smoothstep(0.25 * lambda, 0.5 * lambda, fr);
    s += dir * (a * amp * sin(6.2831853 / lambda * x));
    float rugueux = (1.0 - a) * (1.0 - smoothstep(w, 2.0 * w, fr)) * 0.5 * amp * amp;
    cv += rugueux * vec3(dir.x * dir.x, dir.x * dir.y, dir.y * dir.y);
}

/// Les rides au point `p` (m, dans le plan de l'eau, repère quelconque : la pente rendue est dans le même), au temps
/// `t` (s) ; `dpdx`, `dpdy` : les dérivées d'écran de `p`. Rend la pente résolue ; `cv` : la covariance de pente non
/// résolue (xx, xy, yy) ; `coeur` : 1 si un cœur d'anneau de moins de 30 ms (rayon 1 cm) couvre le point — le contrôle
/// du taux.
vec2 pluie_rides(vec2 p, float t, vec2 dpdx, vec2 dpdy, out vec3 cv, out float coeur) {
    coeur = 0.0;
    float uniforme = (PLUIE_E1 + PLUIE_E2) * pluie.z
        + (PLUIE_E1 * pluie_part_uniforme(dpdx, dpdy, PLUIE_W1) + PLUIE_E2 * pluie_part_uniforme(dpdx, dpdy, PLUIE_W2)) * pluie.w;
    cv = vec3(0.5 * uniforme, 0.0, 0.5 * uniforme);
    // La plus petite valeur singulière de `M` : la plus petite empreinte, toutes directions.
    float somme = dot(dpdx, dpdx) + dot(dpdy, dpdy);
    float det = abs(dpdx.x * dpdy.y - dpdx.y * dpdy.x);
    float s_max = sqrt(0.5 * (somme + sqrt(max(somme * somme - 4.0 * det * det, 0.0))));
    if (det / max(s_max, 1e-12) >= 2.0 * PLUIE_W2 && !controle_pluie) {
        return vec2(0.0);
    }
    float couches = pluie.x * PLUIE_MAILLE * PLUIE_MAILLE * PLUIE_VIE;
    int nc = min(int(ceil(couches)), PLUIE_COUCHES_MAX);
    vec2 s = vec2(0.0);
    for (int j = 0; j < nc; j++) {
        float presence = clamp(couches - float(j), 0.0, 1.0);
        float phase = pluie_u01(pluie_hash(uint(j) * 0x9e3779b9u + 0x2545f491u)) * PLUIE_VIE;
        float kf = floor((t + phase) / PLUIE_VIE);
        for (int q = 0; q < 2; q++) {
            int k = int(kf) - q;
            // Le décalage de la couche pour cette période : 16 bits par axe d'un hachage court ; la graine de la maille.
            uint hk = pluie_hash(uint(j) * 0x85ebca6bu ^ uint(k) * 0xc2b2ae35u ^ 0x51ed2701u);
            vec2 decal = vec2(float(hk >> 16u), float(hk & 0xffffu)) * (PLUIE_MAILLE / 65536.0);
            vec2 m = floor((p + decal) / PLUIE_MAILLE);
            uvec4 h = pluie_pcg4d(uvec4(uvec2(ivec2(m)), hk, uint(j)));
            if (float(h.w & 0xffffu) * (1.0 / 65536.0) >= presence) {
                continue;
            }
            float age = t - (float(k) * PLUIE_VIE - phase + pluie_u01(h.z) * PLUIE_VIE);
            if (age < 0.0 || age >= PLUIE_VIE) {
                continue;
            }
            vec2 centre = (m + 0.5) * PLUIE_MAILLE - decal + (vec2(pluie_u01(h.x), pluie_u01(h.y)) * 2.0 - 1.0) * PLUIE_JEU;
            vec2 d = p - centre;
            float dist = length(d);
            if (age < 0.03 && dist < 0.01) {
                coeur = 1.0;
            }
            float x1 = dist - PLUIE_C1 * age;
            float x2 = dist - PLUIE_C2 * age;
            bool dans_bosse = age < PLUIE_BOSSE_VIE && dist < 3.0 * PLUIE_BOSSE_R;
            if (abs(x1) >= 2.0 * PLUIE_W1 && abs(x2) >= 2.0 * PLUIE_W2 && !dans_bosse) {
                continue;
            }
            float taille = min(1.5 - log(1.0 - pluie_u01(h.w)) / max(pluie.y, 1e-3), 6.0);
            float e = 1.0 - age / PLUIE_VIE;
            float f = pow(taille / 2.0, 1.5) * e * e;
            vec2 dir = d / max(dist, 1e-5);
            float fr = pluie_empreinte(dpdx, dpdy, dir);
            pluie_train(x1, PLUIE_W1, PLUIE_L1, PLUIE_S1 * f, fr, dir, s, cv);
            pluie_train(x2, PLUIE_W2, PLUIE_L2, PLUIE_S2 * f, fr, dir, s, cv);
            if (dans_bosse) {
                // Une bosse de Gauss de rayon `PLUIE_BOSSE_R` : ∇η pointe vers le centre ; pente de crête `PLUIE_BOSSE·f`,
                // résolue comme le train court.
                float eb = 1.0 - age / PLUIE_BOSSE_VIE;
                float r = dist / PLUIE_BOSSE_R;
                float a = 1.0 - smoothstep(0.25 * PLUIE_L1, 0.5 * PLUIE_L1, fr);
                s -= dir * (a * PLUIE_BOSSE * pow(taille / 2.0, 1.5) * eb * eb * 2.3316 * r * exp(-r * r));
            }
        }
    }
    return s;
}
