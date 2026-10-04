// S357, ADR-192 D2 — LA MER DE B DANS GODOT : le nuanceur d'eau du prototype.
//
// Port de `viewer/src/water.wgsl` (bande CWM d'ADR-157, second ordre de Tayfun d'ADR-176, queue en pentes filtrées
// d'ADR-155 et ADR-161, modulation d'ADR-158) et de `viewer/src/water_cretes.wgsl` (S356 : l'écume à la couverture de
// Monahan, un seuil par empreinte ; la lumière des crêtes). Axes de B : x à l'est, y au nord, z en haut ; Godot :
// x, y en haut, z au sud — un point (x, y) de B est (x, −z) dans Godot.
//
// **S359 — la lumière de l'eau est calculée ici, plus par l'éclairage de Godot.** S357 confiait à Godot les reflets
// (GGX) et convertissait la pente que la maille ne résout pas en rugosité : son approximation de l'environnement
// éteint alors la réflexion rasante, et la mer ne renvoyait que 0,15 du ciel sous l'horizon, l'afficheur 0,71
// (REVUE-VISUELLE §24). Ici, comme l'afficheur : le ciel clair (`ciel.gdshaderinc`, le même que celui de la scène),
// Fresnel, et la pente non résolue **intégrée** par quadrature de Gauss-Hermite d'ordre 3 sur sa covariance transportée
// par le jacobien (ADR-161) ; le soleil en éclat. Sous la surface : la colonne d'eau (liste 8.5). Godot garde la brume,
// la tonalité, le halo et tout le reste de la scène.
// S373 : `shader_type` et `render_mode` dans `eau.gdshader` et `eau_demi.gdshader`, qui incluent ce corps.

#include "res://ciel.gdshaderinc"
// S365 : R0, `gain_eau`, `kd`, l'atténuation du faisceau et le mode immergé — une source pour l'eau, le fond et le ciel.
#include "res://optique_eau.gdshaderinc"
// S371 : la bande de B (`bande`, `n_bande`, `split`, `k_moyens`, déclarés là depuis) et le milieu du pixel — la caméra à
// demi immergée.
#include "res://surface_b.gdshaderinc"
// S379 : les rides de la pluie, factices (ADR-202 D3) — les mêmes que sur le bassin.
#include "res://pluie.gdshaderinc"
// L'horloge des rides, repliée par le script (`mer.gd`, sur l'heure).
uniform float temps_pluie = 0.0;
// S383 — **les gerbes au loin** : hors de la fenêtre des particules (`gerbes_fenetre`, plan de B, coordonnées de Lagrange ;
// fondu sur son dernier mètre, où les particules s'effacent d'autant), la part du pixel couverte par les gerbes, en moyenne,
// `M/tan ε` (`pluie.gd`, `moment_gerbes`), ε l'élévation du rayon au-dessus de l'eau ; leur radiance, celle des gouttes.
uniform float gerbes_moment = 0.0;
uniform vec4 gerbes_fenetre = vec4(0.0);

// La queue `[a, kx, ky, φ(t)]`, phases repliées par le script en double précision (I-08).
uniform vec4 queue[60];
uniform int n_queue = 0;
// Modulation de la queue par la bande (ADR-158), retard d'ADR-176 en tours ; les asymétries (k̄ des deux systèmes) et la
// coupure entre eux : `surface_b.gdshaderinc`.
uniform float modulation_M = 0.0;
uniform float retard_tours = 0.0;
// L'écume (S356) : quatorze seuils de `s = (J − 1)/σ` aux empreintes 2^(i−7) m ; sa réflectance.
uniform float ecume_seuils[14];
uniform float reflectance_ecume = 0.55;
// S360 — l'écume au déferlement : la bande seule (les vagues dominantes), à une empreinte d'au moins
// `ecume_empreinte_min` ; ses seuils, calculés par l'afficheur comme les autres, pour la même couverture de Monahan.
uniform float ecume_seuils_deferlement[14];
uniform float ecume_empreinte_min = 1.0;
// S368 — **le champ d'écume** (`ecume.gd`, la référence de S367 sur la carte) : (actif, résiduel) sur `ecume_cote` mètres
// à partir du coin `ecume_origine` (axes de B). Sans lui (`ECUME=ancienne`), l'écume instantanée de S360.
uniform sampler2D ecume_champ : filter_linear, repeat_disable;
uniform vec2 ecume_origine = vec2(0.0);
uniform float ecume_cote = 256.0;
uniform bool ecume_champ_actif = false;
// **2026-09-26, décision de l'utilisateur** : *« Oublie l'écume sauf si tu trouve des photos qui informe de la forme et
// couleur et position dans la topologie »* — l'écume est éteinte par défaut (`mer.gd`, `ECUME=champ` ou `ancienne`).
uniform bool ecume_visible = true;
// La direction dominante des vagues (B, unitaire) : la dentelle s'y étire, comme les traînées de S367.
uniform vec2 ecume_direction = vec2(0.0, 1.0);
// La lumière des crêtes (S356) : transmission de l'eau pure sur Hs ; force et exposant de sa diffusion vers l'avant,
// ceux de l'afficheur, à calibrer.
uniform vec3 transmission_crete = vec3(0.5987, 0.9187, 0.9863);
uniform float force_cretes = 0.15;
uniform float exposant_cretes = 4.0;
// Couleur du corps d'eau : R(0⁻) de Pope & Fry et Morel (ADR-177) ; `gain_eau` tient lieu de `E/π` (ADR-177 D2), 2 comme
// l'afficheur de R19 — déclarés dans `optique_eau.gdshaderinc` depuis S365. Le fond est éclairé par le même modèle
// (`sol.gdshader`) : fond et corps d'eau sont dans les mêmes unités, ce que demande le mélange de Maritorena.
// Empreinte angulaire d'un pixel, `2·tan(fov/2)/hauteur`, et pas radial relatif de la grille polaire.
uniform float angle_pixel = 0.0013;
uniform float pas_radial = 0.03;
// S359 — la colonne d'eau : ce que Godot a rendu derrière la surface (fond, objets) et sa profondeur. La mer, qui les
// lit, est dessinée après eux et n'entre pas dans le tampon de profondeur.
uniform sampler2D ecran : hint_screen_texture, filter_linear_mipmap;
uniform sampler2D profondeur_ecran : hint_depth_texture, filter_nearest;
// S360 — la surface fine : la queue de B réalisée par FFT (`detail.gd`), deux cascades de 32 m et 4 m ; par cascade,
// (s_x, s_y, s_x², s_y²) et (s_x·s_y, g_xx, g_yy, g_xy), avec leurs niveaux de détail — la moyenne de l'empreinte, et la
// variance qu'elle ne résout pas (LEAN, Olano et Baker 2010). Sans elle (`detail_actif` faux), la queue de 60 composantes.
uniform sampler2D detail_a0 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform sampler2D detail_b0 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform sampler2D detail_a1 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform sampler2D detail_b1 : filter_linear_mipmap_anisotropic, repeat_enable;
uniform vec2 detail_cotes = vec2(32.0, 4.0);
uniform bool detail_actif = false;
// S371 — texels par côté des cascades (`detail.gd`, N) : au-delà d'un grossissement de 32 (texel vu sur plus de 32 pixels),
// la lecture passe en bicubique (`lire_detail`).
uniform float detail_texels = 256.0;
// L'atténuation diffuse `Kd = (a + b_b)/μ̄_d` des trois bandes (650, 550, 450 nm) : `a` de Pope & Fry, `b_b` de Morel,
// ceux d'ADR-177 ; `μ̄_d` = 0,8 *à calibrer*. Calculée et donnée par `mer.gd` (`optique_eau.gdshaderinc`, qui porte aussi
// l'indice de l'eau de mer depuis S366).
// Mode contrôle (S359) : 0 rendu ; 1 profondeur verticale du fond vu sans réfraction, `H/echelle_controle`, en sortie
// directe — `mer.gd --controle-fond` la relit contre la bathymétrie ; 2 la transmission `T` de la colonne, par canal ;
// 3 (S360) la couverture d'écume, relue par `mer.gd --controle-ecume` et `outils/ecume_taches.py`.
uniform int controle = 0;
uniform float echelle_controle = 50.0;

varying vec2 lag;
varying vec3 g_bande;
varying vec2 pente_bande;
varying float eps;
varying float sigma2_bande;
varying vec3 g_ecume;
varying float sigma2_ecume;

float poids(float k, float h) {
    float t = clamp(2.0 * k * h / PI - 1.0, 0.0, 1.0);
    return 1.0 - t * t * (3.0 - 2.0 * t);
}

float seuil_ecume(float h) {
    float x = clamp(log2(max(h, 0.0078125)) + 7.0, 0.0, 13.0);
    int i = min(int(floor(x)), 12);
    float f = x - float(i);
    return ecume_seuils[i] * (1.0 - f) + ecume_seuils[i + 1] * f;
}

float seuil_deferlement(float h) {
    float x = clamp(log2(max(h, 0.0078125)) + 7.0, 0.0, 13.0);
    int i = min(int(floor(x)), 12);
    float f = x - float(i);
    return ecume_seuils_deferlement[i] * (1.0 - f) + ecume_seuils_deferlement[i + 1] * f;
}

/// S371 — **une cascade lue en B-spline bicubique** (Sigg et Hadwiger 2005, *GPU Gems 2* ch. 20 : quatre lectures
/// bilinéaires) : pentes et moments continus jusqu'à la dérivée seconde. Vue à quelques centimètres — caméra au ras de
/// l'eau —, un texel de la cascade de 4 m (1,6 cm) couvre des dizaines de pixels, et la lecture bilinéaire, continue mais
/// anguleuse, dessinait le bord de la fenêtre de Snell en escalier. Le champ n'a rien sous deux texels : le lisser est
/// ce qu'il sait.
vec4 bicubique(sampler2D t, vec2 uv) {
    vec2 st = uv * detail_texels - 0.5;
    vec2 i = floor(st);
    vec2 f = st - i;
    vec2 f2 = f * f;
    vec2 f3 = f2 * f;
    vec2 w0 = (-f3 + 3.0 * f2 - 3.0 * f + 1.0) / 6.0;
    vec2 w1 = (3.0 * f3 - 6.0 * f2 + 4.0) / 6.0;
    vec2 w2 = (-3.0 * f3 + 3.0 * f2 + 3.0 * f + 1.0) / 6.0;
    vec2 w3 = f3 / 6.0;
    vec2 g0 = w0 + w1;
    vec2 g1 = w2 + w3;
    vec2 p0 = (i - 1.0 + w1 / g0 + 0.5) / detail_texels;
    vec2 p1 = (i + 1.0 + w3 / g1 + 0.5) / detail_texels;
    return g0.y * (g0.x * textureLod(t, vec2(p0.x, p0.y), 0.0) + g1.x * textureLod(t, vec2(p1.x, p0.y), 0.0))
        + g1.y * (g0.x * textureLod(t, vec2(p0.x, p1.y), 0.0) + g1.x * textureLod(t, vec2(p1.x, p1.y), 0.0));
}

/// Une cascade au point `uv` (en côtés) pour l'empreinte `h` (m) d'un pixel et le côté `cote` : la lecture filtrée de
/// S360, et, au-delà d'un grossissement de 32, vers le bicubique (entier à 64) — la surface à moins de ≈ 3 m de l'œil pour
/// la cascade de 32 m, ≈ 0,4 m pour celle de 4 m : caméra au ras de l'eau. Un seuil de 4 changeait les poses validées
/// (R21), où la cascade de 32 m est déjà grossie de 5 à 15 fois ; ce régime-là reste en bilinéaire, à examiner.
vec4 lire_detail(sampler2D t, vec2 uv, float h, float cote) {
    vec4 v = texture(t, uv);
    float grossi = smoothstep(32.0, 64.0, cote / detail_texels / max(h, 1e-9));
    return grossi > 0.0 ? mix(v, bicubique(t, uv), grossi) : v;
}

/// Le point de la scène vu au pixel `uv`, en coordonnées du monde (Godot 4.3+, profondeur inversée, NDC z dans [0, 1]).
vec3 scene_monde(vec2 uv, mat4 inv_proj, mat4 inv_vue) {
    float d = texture(profondeur_ecran, uv).r;
    vec4 v = inv_proj * vec4(uv * 2.0 - 1.0, d, 1.0);
    v.xyz /= v.w;
    return (inv_vue * vec4(v.xyz, 1.0)).xyz;
}

/// Réflectance de Fresnel exacte, air → eau, lumière non polarisée ; `c` cosinus d'incidence.
float fresnel(float c, float n) {
    float st2 = (1.0 - c * c) / (n * n);
    float ct = sqrt(max(1.0 - st2, 0.0));
    float rs = (c - n * ct) / (c + n * ct);
    float rp = (n * c - ct) / (n * c + ct);
    return 0.5 * (rs * rs + rp * rp);
}

/// ADR-161 : la covariance des pentes non résolues, transportée par le jacobien `J = I + g` du déplacement.
vec3 transport_covariance(vec3 c, vec3 g) {
    float det = (1.0 + g.x) * (1.0 + g.z) - g.y * g.y;
    if (det < 0.1) { return c; }
    float a = (1.0 + g.z) / det;
    float b = -g.y / det;
    float d = (1.0 + g.x) / det;
    return vec3(a * a * c.x + 2.0 * a * b * c.y + b * b * c.z,
        a * b * c.x + (a * d + b * b) * c.y + b * d * c.z, b * b * c.x + 2.0 * b * d * c.y + d * d * c.z);
}

/// `sample_light` de l'afficheur pour la pente `pente` (B) et le rayon `rayon` (B) : sous la surface, le corps d'eau et
/// le fond vu ; au-dessus, le ciel réfléchi ; Fresnel exact entre les deux ; l'éclat du soleil.
vec3 lumiere(Optique op, vec2 pente, vec3 rayon, vec3 fond_vu, vec3 t_colonne) {
    vec3 n = normalize(vec3(-pente, 1.0));
    vec3 soleil = normalize(SOLEIL_B);
    float f = fresnel(clamp(dot(-rayon, n), 0.0, 1.0), indice);
    vec3 reflet = reflect(rayon, n);
    float eclat = pow(max(dot(reflet, soleil), 0.0), 180.0);
    // S381 : sous le ciel couvert, ni éclat ni lumière directe (`ciel.gdshaderinc`).
    if (couvert > 0.0) {
        eclat *= soleil_direct();
    }
    vec3 corps = op.R0 * gain_eau * eclairage(n, soleil, 1.0);
    vec3 sous = corps * (1.0 - t_colonne) + fond_vu;
    return mix(sous, ciel_b(reflet, 2), f) + vec3(1.0, 0.95, 0.85) * eclat * 1.2;
}

/// S368 — **les bords de cellules** (Worley 1996) : `F2 − F1` des distances aux deux points les plus proches d'un réseau
/// jeté au hasard (un point par cellule, hachage entier du ciel) ; nul sur les frontières. L'écume résiduelle s'organise en
/// réseau polygonal ; le bruit de valeurs, lui, trace des crêtes qui suivent sa grille (vu en S368 : un labyrinthe).
float bords_cellules(vec2 q) {
    vec2 c = floor(q);
    vec2 f = q - c;
    float d1 = 8.0;
    float d2 = 8.0;
    for (int j = -1; j <= 1; j++) {
        for (int i = -1; i <= 1; i++) {
            vec2 o = vec2(float(i), float(j));
            vec2 pt = o + vec2(hachage_ciel(c + o), hachage_ciel(c + o + vec2(311.0, 113.0)));
            float d = length(pt - f);
            if (d < d1) {
                d2 = d1;
                d1 = d;
            } else if (d < d2) {
                d2 = d;
            }
        }
    }
    return d2 - d1;
}

/// S365 — **la surface vue d'en dessous** (liste 8.6, ADR-019 §2). `rayon` monte vers la surface (B). Fresnel exact eau →
/// air ; dans la **fenêtre de Snell**, le ciel réfracté, radiance multipliée par n² (`L/n²` se conserve à travers
/// l'interface) ; au-delà de l'angle critique `arcsin(1/n)`, **réflexion totale** : l'eau elle-même, vue dans la direction
/// réfléchie. Non portés : le fond ou un objet vus dans ce miroir, la face inférieure de l'écume.
vec3 lumiere_dessous(Optique op, vec2 pente, vec3 rayon) {
    vec3 n = normalize(vec3(-pente, 1.0));
    float c = dot(rayon, n);
    // Incidence rasante, ou facette vue par-derrière : réflexion totale. (Sans ce garde, Fresnel y vaut 0/0 : S365 P3,
    // des pixels noirs vers l'horizon de la surface vue d'en dessous.)
    float f = c > 1e-4 ? fresnel(min(c, 1.0), 1.0 / indice) : 1.0;
    vec3 eau = eau_infinie_o(op, reflect(rayon, n), 0.0);
    if (f >= 1.0) { return eau; }
    return (1.0 - f) * indice * indice * ciel_b(refract(rayon, -n, indice), 2) + f * eau;
}

/// La lumière intégrée sur la pente non résolue (ADR-161) : Cholesky de sa covariance `cv`, 3 × 3 nœuds de Gauss-Hermite ;
/// vue d'en dessous ou d'au-dessus.
vec3 integree(Optique op, vec2 pente, vec3 rayon, vec3 cv, bool dessous, vec3 fond_vu, vec3 t_colonne) {
    if (cv.x + cv.z <= 0.0) {
        return dessous ? lumiere_dessous(op, pente, rayon) : lumiere(op, pente, rayon, fond_vu, t_colonne);
    }
    float l00 = sqrt(max(cv.x, 0.0));
    float l10 = l00 > 0.0 ? cv.y / l00 : 0.0;
    float l11 = sqrt(max(cv.z - l10 * l10, 0.0));
    vec3 noeuds = vec3(-1.73205080757, 0.0, 1.73205080757);
    vec3 poids_gh = vec3(0.166666666667, 0.666666666667, 0.166666666667);
    vec3 couleur = vec3(0.0);
    for (int j = 0; j < 3; j++) {
        for (int i = 0; i < 3; i++) {
            vec2 p = pente + vec2(l00 * noeuds[i], l10 * noeuds[i] + l11 * noeuds[j]);
            couleur += poids_gh[i] * poids_gh[j]
                * (dessous ? lumiere_dessous(op, p, rayon) : lumiere(op, p, rayon, fond_vu, t_colonne));
        }
    }
    return couleur;
}

void vertex() {
    vec3 monde = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
    vec2 q = vec2(monde.x, -monde.z);
    // Empreinte du sommet : le pas de la grille, ou le pixel s'il est plus grand.
    float h = max(pas_radial * length(VERTEX.xz), length(monde - CAMERA_POSITION_WORLD) * angle_pixel);
    vec2 d = vec2(0.0);
    vec2 s = vec2(0.0);
    vec3 g = vec3(0.0);
    float eta = 0.0, e = 0.0, eq = 0.0, v = 0.0;
    vec2 e2 = vec2(0.0), qd = vec2(0.0);
    vec4 sa = vec4(0.0), sq = vec4(0.0);
    vec3 ge = vec3(0.0);
    float ve = 0.0;
    float h_ecume = max(h, ecume_empreinte_min);
    for (int i = 0; i < n_bande; i++) {
        vec4 c = bande[i];
        float k = length(c.yz);
        if (k == 0.0) { continue; }
        float a = c.x * poids(k, h);
        float phase = dot(c.yz, q) + c.w;
        vec2 u = c.yz / k;
        float cs = cos(phase);
        float sn = sin(phase);
        d += a * cs * u;
        eta += a * sn;
        e += a * k * sn;
        eq += a * k * cs;
        s += a * cs * c.yz;
        g -= a * k * sn * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
        v += 0.5 * (a * k) * (a * k);
        float a_e = c.x * poids(k, h_ecume);
        ge -= a_e * k * sn * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
        ve += 0.5 * (a_e * k) * (a_e * k);
        if (i < split) {
            e2.x += a * sn; qd.x += a * cs;
            sa.xy += a * cs * c.yz; sq.xy -= a * sn * c.yz;
        } else {
            e2.y += a * sn; qd.y += a * cs;
            sa.zw += a * cs * c.yz; sq.zw -= a * sn * c.yz;
        }
    }
    // Second ordre de Tayfun par système (ADR-176 D1).
    if (k_moyens.x > 0.0) {
        eta += 0.5 * k_moyens.x * (e2.x * e2.x - qd.x * qd.x) + 0.5 * k_moyens.y * (e2.y * e2.y - qd.y * qd.y);
        s += k_moyens.x * (e2.x * sa.xy - qd.x * sq.xy) + k_moyens.y * (e2.y * sa.zw - qd.y * sq.zw);
    }
    float angle = retard_tours * TAU;
    VERTEX += vec3(d.x, eta, -d.y);
    lag = q;
    g_bande = g;
    pente_bande = s;
    eps = cos(angle) * e + sin(angle) * eq;
    sigma2_bande = v;
    g_ecume = ge;
    sigma2_ecume = ve;
}

void fragment() {
    float h = max(length(dFdx(lag)), length(dFdy(lag)));
    // La queue filtrée : pentes, gradient du déplacement, variance résolue, et covariance que la maille ne résout pas.
    vec2 s_q = vec2(0.0);
    vec3 g_q = vec3(0.0);
    vec3 cov_q = vec3(0.0);
    if (detail_actif) {
        // S360 : la queue dense, lue à l'empreinte du pixel ; la covariance non résolue, E[s·sᵀ] − E[s]·E[s]ᵀ.
        vec4 a0 = lire_detail(detail_a0, lag / detail_cotes.x, h, detail_cotes.x);
        vec4 b0 = lire_detail(detail_b0, lag / detail_cotes.x, h, detail_cotes.x);
        vec4 a1 = lire_detail(detail_a1, lag / detail_cotes.y, h, detail_cotes.y);
        vec4 b1 = lire_detail(detail_b1, lag / detail_cotes.y, h, detail_cotes.y);
        s_q = a0.xy + a1.xy;
        g_q = vec3(b0.y + b1.y, b0.w + b1.w, b0.z + b1.z);
        cov_q = vec3(max(a0.z - a0.x * a0.x, 0.0), b0.x - a0.x * a0.y, max(a0.w - a0.y * a0.y, 0.0))
            + vec3(max(a1.z - a1.x * a1.x, 0.0), b1.x - a1.x * a1.y, max(a1.w - a1.y * a1.y, 0.0));
    }
    for (int i = 0; i < (detail_actif ? 0 : n_queue); i++) {
        vec4 c = queue[i];
        float k = length(c.yz);
        float w = poids(k, h) * exp(-0.5 * k * k * h * h);
        cov_q += (0.5 * c.x * c.x * (1.0 - w * w)) * vec3(c.y * c.y, c.y * c.z, c.z * c.z);
        if (w > 0.0) {
            float phase = dot(c.yz, lag) + c.w;
            vec2 u = c.yz / k;
            s_q += w * c.x * cos(phase) * c.yz;
            g_q -= w * c.x * k * sin(phase) * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
        }
    }
    float energie = max(0.0, 1.0 + modulation_M * eps);
    vec3 g = g_bande + sqrt(energie) * g_q;
    vec2 p_lag = pente_bande + sqrt(energie) * s_q;
    vec3 cov_t = energie * cov_q;
    // S379 — **les rides de la pluie** (`pluie.gdshaderinc`), en coordonnées de Lagrange : elles suivent l'eau dans son
    // mouvement orbital, comme la queue ; leur pente s'ajoute à celle de Lagrange avant le passage en Euler, leur covariance
    // à la covariance non résolue avant son transport. Temps sec : rien, au bit.
    float variance_pluie = 0.0;
    if (pluie.x > 0.0) {
        float coeur_pluie;
        vec3 cov_pluie;
        p_lag += pluie_rides(lag, temps_pluie, dFdx(lag), dFdy(lag), cov_pluie, coeur_pluie);
        variance_pluie = cov_pluie.x + cov_pluie.z;
        cov_t += cov_pluie;
    }
    // Pente eulérienne J⁻ᵀ·∇η ; repli (det < 0,1) : pente de Lagrange, comme l'afficheur.
    float jxx = 1.0 + g.x, jxy = g.y, jyy = 1.0 + g.z;
    float det = jxx * jyy - jxy * jxy;
    vec2 pente = det >= 0.1 ? vec2((jyy * p_lag.x - jxy * p_lag.y) / det, (-jxy * p_lag.x + jxx * p_lag.y) / det) : p_lag;
    vec3 n_monde = normalize(vec3(-pente.x, 1.0, pente.y));
    // S360 — la variable des crêtes, `s = (J − 1)/σ` (S356), sur la **bande seule**, à l'empreinte d'au moins un mètre :
    // l'écume au déferlement des vagues dominantes, et la lumière qui traverse leurs crêtes ; couverture antialiasée par
    // sa variation dans le pixel.
    float det_e = (1.0 + g_ecume.x) * (1.0 + g_ecume.z) - g_ecume.y * g_ecume.y;
    float s_e = (det_e - 1.0) / max(sqrt(max(sigma2_ecume, 0.0)), 1e-6);
    float couverture = clamp((seuil_deferlement(max(h, ecume_empreinte_min)) - s_e) / max(fwidth(s_e), 1e-4) + 0.5, 0.0, 1.0);
    // S359 — la colonne d'eau (liste 8.5). Le fond vu à travers la surface, par réfraction de Snell : le point du fond
    // où le rayon réfracté descend jusqu'à la profondeur vue sans réfraction, reprojeté à l'écran puis relu ; un objet
    // hors de l'eau à cet endroit garde le pixel lui-même. Puis Maritorena, Morel et Gentili (1994), généralisé au
    // trajet oblique : fond × T + corps d'eau × (1 − T), T = exp(−Kd·(H + L)) — la lumière descend H jusqu'au fond et
    // remonte L jusqu'à la surface. Fond et corps d'eau dans les mêmes unités ; la sortie par la surface, (1 − F), est
    // celle de `lumiere`. Non portés : la réduction de l'éclairement du fond par (1 − F) du soleil (≈ 2 %), la
    // focalisation par les vagues (caustiques).
    vec3 surface = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
    // S477 : les propriétés optiques du point de surface (la carte du type d'eau, ou la scène).
    Optique op = optique_en(vec2(surface.x, -surface.z));
    // S371 — **le milieu du pixel** (`surface_b.gdshaderinc`) : décidé à l'objectif, là où le rayon traverse le plan
    // proche — part d'eau `milieu.x`, profondeur de l'objectif `milieu.y`. Hors de la zone des vagues, celui de l'œil,
    // d'un bloc (S365). Dans la transition, les deux vues se mélangent.
    vec3 milieu = milieu_du_pixel(CAMERA_POSITION_WORLD, normalize(surface - CAMERA_POSITION_WORLD));
    vec3 vu_dessous = vec3(0.0);
    vec3 vu_dessus = vec3(0.0);
    // S365 — vue d'en dessous : la surface, puis les `d` mètres d'eau jusqu'à l'œil (`a_travers_l_eau`). Pas
    // `FRONT_FACING` : la grille polaire présente sa face avant par en dessous.
    if (milieu.x > 0.0) {
        vec3 vue_d = normalize(surface - CAMERA_POSITION_WORLD);
        vec3 rayon_d = vec3(vue_d.x, -vue_d.z, vue_d.y);
        vec3 lum = integree(op, pente, rayon_d, transport_covariance(cov_t, g), true, vec3(0.0), vec3(0.0));
        float d = length(surface - CAMERA_POSITION_WORLD);
        vu_dessous = a_travers_l_eau_o(op, lum, d, rayon_d, milieu.y);
        if (controle == 4) {
            // Contrôle S365 : le coefficient de Fresnel eau → air, sans la ligne de visée.
            float c4 = dot(rayon_d, normalize(vec3(-pente, 1.0)));
            vu_dessous = vec3(c4 > 1e-4 ? fresnel(min(c4, 1.0), 1.0 / indice) : 1.0);
        }
    }
    if (milieu.x < 1.0) {
    vec3 fond0 = scene_monde(SCREEN_UV, INV_PROJECTION_MATRIX, INV_VIEW_MATRIX);
    float h0 = surface.y - fond0.y;
    vec3 vue = normalize(surface - CAMERA_POSITION_WORLD);
    vec3 r = refract(vue, n_monde, 1.0 / indice);
    vec2 uv_r = SCREEN_UV;
    vec3 fond = fond0;
    if (h0 > 0.0 && r.y < -0.05) {
        vec3 point = surface + r * (h0 / -r.y);
        vec4 clip = PROJECTION_MATRIX * (VIEW_MATRIX * vec4(point, 1.0));
        vec2 uv = clip.xy / clip.w * 0.5 + 0.5;
        // S371 : à demi immergée, le fond relu là doit avoir été vu de l'air — sinon il porte déjà la colonne d'eau vue de
        // l'objectif (`sol.gdshader`), et on garde le fond non réfracté de ce pixel-ci.
        bool relu_dans_l_air = !demi_immergee
            || milieu_du_pixel(CAMERA_POSITION_WORLD, normalize(point - CAMERA_POSITION_WORLD)).x <= 0.0;
        if (relu_dans_l_air && uv.x > 0.0 && uv.x < 1.0 && uv.y > 0.0 && uv.y < 1.0) {
            vec3 f1 = scene_monde(uv, INV_PROJECTION_MATRIX, INV_VIEW_MATRIX);
            if (f1.y < surface.y) {
                uv_r = uv;
                fond = f1;
            }
        }
    }
    float h_fond = max(surface.y - fond.y, 0.0);
    vec3 t_colonne = exp(-op.kd * (h_fond + length(fond - surface)));
    // S379 : la variance de la pluie étale le fond vu, comme au bassin (`bassin.gdshader`).
    float lod_pluie = 0.0;
    if (variance_pluie > 0.0) {
        float etal = (1.0 - 1.0 / indice) * sqrt(0.5 * variance_pluie) * length(fond - surface);
        lod_pluie = log2(1.0 + etal / max(h, 1e-5));
    }
    vec3 fond_vu = textureLod(ecran, uv_r, lod_pluie).rgb * t_colonne;
    // La lumière, pente non résolue intégrée (ADR-161) : Cholesky de sa covariance, 3 × 3 nœuds de Gauss-Hermite.
    vec3 rayon = vec3(vue.x, -vue.z, vue.y);
    vec3 couleur = integree(op, pente, rayon, transport_covariance(cov_t, g), false, fond_vu, t_colonne);
    // Les crêtes (S356, `cretes_couleur`) : la lumière qui traverse ce qui se comprime, vue du côté du soleil ; l'écume.
    vec3 soleil = normalize(SOLEIL_B);
    float eclaire = gain_eau * eclairage(normalize(vec3(-pente, 1.0)), soleil, 1.0);
    vec2 horizontal = rayon.xy / max(length(rayon.xy), 1e-6);
    float avant = pow(max(dot(horizontal, normalize(soleil.xy)), 0.0), exposant_cretes);
    // S381 : cette lumière est celle du soleil direct, pondérée vers son azimut ; sous le ciel couvert, elle s'éteint comme
    // l'éclat (vue sur les images de R30 : des taches claires, aux places qu'elles ont par ciel clair). La lumière diffuse du
    // ciel qui traverse les crêtes n'est pas portée.
    if (couvert > 0.0) {
        avant *= soleil_direct();
    }
    couleur += force_cretes * clamp(-0.5 * s_e, 0.0, 1.0) * avant * transmission_crete * eclaire;
    if (ecume_champ_actif) {
        // S368 — l'écume du champ, là où l'eau est dans le monde. **L'actif** : blanc, au bord irrégulier — le seuil ½ du
        // champ bruité à l'échelle des bulles et des flocons (0,3 à 1,2 m). **Le résiduel** : une dentelle — les bords de
        // cellules de 1,4 et 0,55 m, déformés par un bruit (±0,3 m), étirés 2,2 fois le long des vagues comme les
        // traînées de S367, rompus par un masque de 3,5 m, et visibles là où le film est dense (montée de 0,3 à 0,9) ;
        // translucides (0,3). Quand le pixel couvre le motif, il tend vers sa moyenne. Rendu à juger (R26).
        vec2 qm = vec2(surface.x, -surface.z);
        vec2 uv_e = (qm - ecume_origine) / ecume_cote;
        float bord = clamp(min(min(uv_e.x, 1.0 - uv_e.x), min(uv_e.y, 1.0 - uv_e.y)) / 0.08, 0.0, 1.0);
        vec2 champ = (uv_e.x > 0.0 && uv_e.x < 1.0 && uv_e.y > 0.0 && uv_e.y < 1.0)
            ? texture(ecume_champ, uv_e).rg * bord : vec2(0.0);
        float empreinte = max(h, 1e-3);
        float fondu_bulles = clamp((empreinte - 0.15) / 0.45, 0.0, 1.0);
        float bruit = 0.6 * bruit_ciel(qm / 0.3) + 0.4 * bruit_ciel(qm / 1.2 + 37.0) - 0.5;
        // Une montée large (0,15 à 0,75) : le mouton pâlit en se trouant à mesure que l'actif décroît, au lieu de
        // s'éteindre d'un coup au seuil ½ (vu en S368 sur la séquence) ; la mi-opacité reste au seuil ½ du champ.
        float actif = smoothstep(0.15, 0.75, champ.x + 0.5 * bruit * (1.0 - fondu_bulles));
        actif = mix(actif, smoothstep(0.15, 0.75, champ.x), fondu_bulles);
        vec2 qw = qm + 0.6 * vec2(bruit_ciel(qm / 2.3 + 5.0), bruit_ciel(qm / 2.3 + 91.0)) - 0.3;
        vec2 qa = vec2(dot(qw, vec2(-ecume_direction.y, ecume_direction.x)), 0.45 * dot(qw, ecume_direction));
        float l1 = 1.0 - smoothstep(0.0, 0.06, bords_cellules(qa / 1.4));
        float l2 = 1.0 - smoothstep(0.0, 0.08, bords_cellules(qa / 0.55 + 53.0));
        float trous = smoothstep(0.35, 0.65, bruit_ciel(qm / 3.5 + 17.0));
        float dentelle = max(l1, 0.6 * l2) * trous;
        float fondu_dentelle = clamp((empreinte - 0.05) / 0.3, 0.0, 1.0);
        dentelle = mix(dentelle, 0.08, fondu_dentelle);
        float residuel = smoothstep(0.3, 0.9, champ.y) * dentelle;
        couverture = max(actif, 0.3 * residuel);
    }
    if (!ecume_visible) {
        couverture = 0.0;
    }
    couleur = mix(couleur, vec3(reflectance_ecume * eclaire), couverture);
    vu_dessus = couleur;
    if (gerbes_moment > 0.0) {
        vec2 dedans = min(lag - gerbes_fenetre.xy, gerbes_fenetre.zw - lag);
        float particules = smoothstep(0.0, 1.0, min(dedans.x, dedans.y));
        float tan_e = max(-vue.y, 1e-3) / max(length(vue.xz), 1e-3);
        float part = min(1.0, gerbes_moment / tan_e) * (1.0 - particules);
        if (part > 0.0) {
            vec3 horizontale = normalize(vec3(vue.x, 0.0, vue.z) + vec3(1e-6, 0.0, 0.0));
            vec3 h_b = vec3(horizontale.x, -horizontale.z, 0.0);
            vec3 env = (ciel_b(normalize(h_b + vec3(0.0, 0.0, 0.577)), 2) + ciel_b(normalize(h_b + vec3(0.0, 0.0, 1.732)), 2)
                + ciel_b(vec3(0.0, 0.0, 1.0), 2)) / 3.0;
            vu_dessus = mix(vu_dessus, 0.94 * 0.5 * (env + 0.2 * ciel_b(vec3(0.0, 0.0, 1.0), 2)), part);
        }
    }
    if (controle > 0 && controle < 4) {
        vu_dessus = controle == 1 ? vec3(clamp(h0 / echelle_controle, 0.0, 1.0)) : (controle == 2 ? t_colonne : vec3(couverture));
    }
    }
    ALBEDO = milieu.x >= 1.0 ? vu_dessous : (milieu.x <= 0.0 ? vu_dessus : mix(vu_dessus, vu_dessous, milieu.x));
#ifdef BRUME_PAR_PIXEL
    // S373 : la perspective aérienne, sur la part d'air du pixel seulement (`brume_air`, `ciel.gdshaderinc`) — variante
    // `eau_demi.gdshader`, caméra à demi immergée. Partout ailleurs, la brume du moteur (critère 1 de S373 manqué par la
    // brume réécrite : 11 niveaux au pire, DEMI-IMMERGEE-S371 §10).
    vec3 vue_b = normalize(surface - CAMERA_POSITION_WORLD);
    FOG = (controle == 0 && !controle_milieu)
        ? brume_air(vec3(vue_b.x, -vue_b.z, vue_b.y), length(VERTEX), 1.0 - milieu.x) : vec4(0.0);
#endif
    // S371 P6 : le ménisque sur le hublot (`surface_b.gdshaderinc`), côté air ; pas sur le fond, qui n'y est vu qu'à travers
    // cette surface.
    ALBEDO = menisque(ALBEDO, milieu.z);
    if (controle_milieu) {
        ALBEDO = vec3(milieu.x, 0.0, 0.0);
    }
}
