// S379 — LES RIDES DE LA PLUIE (ADR-202 D3, ADR-203 D7) : un effet de rendu, sans simulation, dont les nombres ont une
// provenance. Les gouttes qui laissent un anneau net (D ≥ 1,5 mm) arrivent au taux de Marshall et Palmer × Atlas
// (`pluie.gd`, 448 m⁻²·s⁻¹ à 10 mm/h) ; chaque anneau est un paquet de crêtes de 1,71 cm — la longueur d'onde où les
// ondes capillaires-gravité ont leur vitesse de phase minimale, 0,231 m/s — qui s'éloigne à cette vitesse et s'éteint en
// 0,6 s ; une petite bosse au centre le premier dixième de seconde. Les photographies de S379 (notes, P2) : un paquet de
// 2 à 5 crêtes, visible par le seul reflet — la ride perturbe la **normale**, jamais la couleur.
//
// Tirage : des couches de mailles carrées (`PLUIE_MAILLE`), chacune décalée au hasard à chaque période de `PLUIE_VIE` ;
// une goutte par maille et par période, née à un instant uniforme de la période, placée près du centre de la maille à
// `PLUIE_JEU` près — l'anneau ne sort jamais de sa maille, un pixel ne lit donc que sa maille, dans la période en cours
// et la précédente. Chaque couche porte `1/(maille²·vie)` gouttes par m² et par s ; `taux·maille²·vie` couches, la
// dernière partielle (une goutte n'y est présente qu'avec la probabilité de la fraction) : le taux est exact.
//
// Niveau de détail, par l'empreinte du pixel sur l'eau **dans la direction du rayon de chaque anneau** (`|Mᵀ·dir|`, `M` les
// dérivées d'écran de la position) : en vue rasante, l'empreinte s'allonge vers l'œil, et seuls les flancs d'un anneau
// gardent leurs crêtes — ce que montrent les photographies. Les crêtes tant que cette empreinte est sous λ/4, fondues en
// variance de pente locale (l'anneau devient une bande rugueuse) jusqu'à λ/2, la limite de Nyquist ; les bandes, fondues de 1 à 2 cm dans la
// variance uniforme, dont la part est la moyenne de ce fondu sur quatre directions ; plus aucune boucle quand la plus
// petite empreinte dépasse 2 cm. Les petites gouttes (0,5 à 1,5 mm) ne sont que variance, partout. La variance rendue est
// la **somme des deux composantes** de la pente, E[|s|²] : chaque composante en porte la moitié.

// x : taux des anneaux (m⁻²·s⁻¹) ; y : Λ (mm⁻¹) ; z, w : moments `Σ flux·f²` des petites gouttes et des anneaux
// (`pluie.gd`, `uniformes`). Zéro : temps sec.
uniform vec4 pluie = vec4(0.0);
// Contrôle du taux (critère 2 de S379) : la sortie ne dit plus que « un cœur d'anneau de moins de 30 ms est ici ».
uniform bool controle_pluie = false;

const float PLUIE_C = 0.231;
const float PLUIE_LAMBDA = 0.0171;
const float PLUIE_VIE = 0.6;
const float PLUIE_W = 0.01;
const float PLUIE_JEU = 0.03;
// 2·(c·vie + 2·w + jeu) = 0,377 m, arrondi au-dessus.
const float PLUIE_MAILLE = 0.38;
// Pente de crête d'un anneau neuf, goutte de 2 mm ; celle de la bosse centrale. **Réglées sur les photographies.**
const float PLUIE_PENTE = 0.12;
const float PLUIE_BOSSE = 0.5;
const float PLUIE_BOSSE_R = 0.006;
const float PLUIE_BOSSE_VIE = 0.1;
const int PLUIE_COUCHES_MAX = 256;
// Énergie de pente d'un anneau de facteur 1, intégrée sur son aire et sa vie (m²·s) :
// ½·pente²·2π·c·(∫g² = 1,6254·w)·(∫t·(1 − t/vie)⁴ dt = vie²/30).
const float PLUIE_E_UNITE = 0.5 * PLUIE_PENTE * PLUIE_PENTE * 6.2831853 * PLUIE_C * 1.6254 * PLUIE_W * PLUIE_VIE * PLUIE_VIE / 30.0;

uvec4 pluie_pcg4d(uvec4 v) {
    v = v * 1664525u + 1013904223u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    v ^= v >> 16u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    return v;
}

float pluie_u01(uint h) {
    return float(h >> 8u) * (1.0 / 16777216.0);
}

/// L'empreinte du pixel dans la direction `d` (unitaire) : `|Mᵀ·d|`, `M = [dpdx | dpdy]`.
float pluie_empreinte(vec2 dpdx, vec2 dpdy, vec2 d) {
    return length(vec2(dot(dpdx, d), dot(dpdy, d)));
}

/// Les rides au point `p` (m, dans le plan de l'eau, repère quelconque : la pente rendue est dans le même), au temps
/// `t` (s) ; `dpdx`, `dpdy` : les dérivées d'écran de `p`. xy : la pente résolue ; z : la variance de pente non résolue,
/// E[|s|²]. `coeur` : 1 si un cœur d'anneau de moins de 30 ms (rayon 1 cm) couvre le point — le contrôle du taux.
vec3 pluie_rides(vec2 p, float t, vec2 dpdx, vec2 dpdy, out float coeur) {
    coeur = 0.0;
    float v = PLUIE_E_UNITE * pluie.z;
    float b = 0.25 * (smoothstep(0.01, 0.02, pluie_empreinte(dpdx, dpdy, vec2(1.0, 0.0)))
        + smoothstep(0.01, 0.02, pluie_empreinte(dpdx, dpdy, vec2(0.0, 1.0)))
        + smoothstep(0.01, 0.02, pluie_empreinte(dpdx, dpdy, vec2(0.7071068, 0.7071068)))
        + smoothstep(0.01, 0.02, pluie_empreinte(dpdx, dpdy, vec2(0.7071068, -0.7071068))));
    v += b * PLUIE_E_UNITE * pluie.w;
    // La plus petite valeur singulière de `M` : la plus petite empreinte, toutes directions.
    float somme = dot(dpdx, dpdx) + dot(dpdy, dpdy);
    float det = abs(dpdx.x * dpdy.y - dpdx.y * dpdy.x);
    float s_max = sqrt(0.5 * (somme + sqrt(max(somme * somme - 4.0 * det * det, 0.0))));
    if (det / max(s_max, 1e-12) >= 0.02 && !controle_pluie) {
        return vec3(0.0, 0.0, v);
    }
    float couches = pluie.x * PLUIE_MAILLE * PLUIE_MAILLE * PLUIE_VIE;
    int nc = min(int(ceil(couches)), PLUIE_COUCHES_MAX);
    vec2 s = vec2(0.0);
    float vl = 0.0;
    float k_onde = 6.2831853 / PLUIE_LAMBDA;
    for (int j = 0; j < nc; j++) {
        float presence = clamp(couches - float(j), 0.0, 1.0);
        float phase = pluie_u01(pluie_pcg4d(uvec4(uint(j), 0x2545f491u, 0x9e3779b9u, 7u)).x) * PLUIE_VIE;
        float kf = floor((t + phase) / PLUIE_VIE);
        for (int q = 0; q < 2; q++) {
            int k = int(kf) - q;
            uvec4 hk = pluie_pcg4d(uvec4(uint(j), uint(k), 0x51ed2701u, 11u));
            vec2 decal = vec2(pluie_u01(hk.x), pluie_u01(hk.y)) * PLUIE_MAILLE;
            vec2 m = floor((p + decal) / PLUIE_MAILLE);
            uvec4 h = pluie_pcg4d(uvec4(uvec2(ivec2(m)), hk.z, hk.w));
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
            float x = dist - PLUIE_C * age;
            bool dans_paquet = abs(x) < 2.0 * PLUIE_W;
            bool dans_bosse = age < PLUIE_BOSSE_VIE && dist < 3.0 * PLUIE_BOSSE_R;
            if (!dans_paquet && !dans_bosse) {
                continue;
            }
            float taille = min(1.5 - log(1.0 - pluie_u01(h.w)) / max(pluie.y, 1e-3), 6.0);
            float f = pow(taille / 2.0, 1.5);
            vec2 dir = d / max(dist, 1e-5);
            float fr = pluie_empreinte(dpdx, dpdy, dir);
            float a = 1.0 - smoothstep(PLUIE_LAMBDA / 4.0, PLUIE_LAMBDA / 2.0, fr);
            float br = smoothstep(0.01, 0.02, fr);
            if (dans_paquet) {
                float u = x / (2.0 * PLUIE_W);
                float g = (1.0 - u * u) * (1.0 - u * u);
                float e = 1.0 - age / PLUIE_VIE;
                float amp = PLUIE_PENTE * f * e * e * g;
                s += dir * (a * amp * sin(k_onde * x));
                vl += (1.0 - a) * (1.0 - br) * 0.5 * amp * amp;
            }
            if (dans_bosse) {
                // Une bosse de Gauss de rayon `PLUIE_BOSSE_R` : ∇η pointe vers le centre ; pente de crête `PLUIE_BOSSE·f`.
                float e = 1.0 - age / PLUIE_BOSSE_VIE;
                float r = dist / PLUIE_BOSSE_R;
                s -= dir * (a * PLUIE_BOSSE * f * e * e * 2.3316 * r * exp(-r * r));
            }
        }
    }
    return vec3(s, v + vl);
}
