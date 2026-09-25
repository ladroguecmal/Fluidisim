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
shader_type spatial;
render_mode cull_disabled, unshaded;

#include "res://ciel.gdshaderinc"
// S365 : R0, `gain_eau`, `kd`, l'atténuation du faisceau et le mode immergé — une source pour l'eau, le fond et le ciel.
#include "res://optique_eau.gdshaderinc"

// Composantes `[a, kx, ky, φ(t)]`, phases repliées par le script en double précision (I-08).
uniform vec4 bande[64];
uniform int n_bande = 0;
uniform vec4 queue[60];
uniform int n_queue = 0;
// Modulation de la queue par la bande (ADR-158), retard d'ADR-176 en tours, asymétries (k̄ des deux systèmes).
uniform float modulation_M = 0.0;
uniform float retard_tours = 0.0;
uniform int split = 64;
uniform vec2 k_moyens = vec2(0.0);
// L'écume (S356) : quatorze seuils de `s = (J − 1)/σ` aux empreintes 2^(i−7) m ; sa réflectance.
uniform float ecume_seuils[14];
uniform float reflectance_ecume = 0.55;
// S360 — l'écume au déferlement : la bande seule (les vagues dominantes), à une empreinte d'au moins
// `ecume_empreinte_min` ; ses seuils, calculés par l'afficheur comme les autres, pour la même couverture de Monahan.
uniform float ecume_seuils_deferlement[14];
uniform float ecume_empreinte_min = 1.0;
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
// L'atténuation diffuse `Kd = (a + b_b)/μ̄_d` des trois bandes (650, 550, 450 nm) : `a` de Pope & Fry, `b_b` de Morel,
// ceux d'ADR-177 ; `μ̄_d` = 0,8 *à calibrer*. Calculée et donnée par `mer.gd` (`optique_eau.gdshaderinc`). Indice de
// l'eau de mer.
uniform float indice = 1.34;
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
vec3 lumiere(vec2 pente, vec3 rayon, vec3 fond_vu, vec3 t_colonne) {
    vec3 n = normalize(vec3(-pente, 1.0));
    vec3 soleil = normalize(SOLEIL_B);
    float f = fresnel(clamp(dot(-rayon, n), 0.0, 1.0), indice);
    vec3 reflet = reflect(rayon, n);
    float eclat = pow(max(dot(reflet, soleil), 0.0), 180.0);
    vec3 corps = R0 * gain_eau * (0.6 + 0.4 * max(dot(n, soleil), 0.0));
    vec3 sous = corps * (1.0 - t_colonne) + fond_vu;
    return mix(sous, ciel_b(reflet, 2), f) + vec3(1.0, 0.95, 0.85) * eclat * 1.2;
}

/// S365 — **la surface vue d'en dessous** (liste 8.6, ADR-019 §2). `rayon` monte vers la surface (B). Fresnel exact eau →
/// air ; dans la **fenêtre de Snell**, le ciel réfracté, radiance multipliée par n² (`L/n²` se conserve à travers
/// l'interface) ; au-delà de l'angle critique `arcsin(1/n)`, **réflexion totale** : l'eau elle-même, vue dans la direction
/// réfléchie. Non portés : le fond ou un objet vus dans ce miroir, la face inférieure de l'écume.
vec3 lumiere_dessous(vec2 pente, vec3 rayon) {
    vec3 n = normalize(vec3(-pente, 1.0));
    float f = fresnel(clamp(dot(rayon, n), 0.0, 1.0), 1.0 / indice);
    vec3 eau = eau_infinie(reflect(rayon, n), 0.0);
    if (f >= 1.0) { return eau; }
    return (1.0 - f) * indice * indice * ciel_b(refract(rayon, -n, indice), 2) + f * eau;
}

/// La lumière intégrée sur la pente non résolue (ADR-161) : Cholesky de sa covariance `cv`, 3 × 3 nœuds de Gauss-Hermite ;
/// vue d'en dessous ou d'au-dessus.
vec3 integree(vec2 pente, vec3 rayon, vec3 cv, bool dessous, vec3 fond_vu, vec3 t_colonne) {
    if (cv.x + cv.z <= 0.0) {
        return dessous ? lumiere_dessous(pente, rayon) : lumiere(pente, rayon, fond_vu, t_colonne);
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
                * (dessous ? lumiere_dessous(p, rayon) : lumiere(p, rayon, fond_vu, t_colonne));
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
        vec4 a0 = texture(detail_a0, lag / detail_cotes.x);
        vec4 b0 = texture(detail_b0, lag / detail_cotes.x);
        vec4 a1 = texture(detail_a1, lag / detail_cotes.y);
        vec4 b1 = texture(detail_b1, lag / detail_cotes.y);
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
    // S365 — vue d'en dessous, quand l'œil est dans l'eau (`sous_eau`, posé par `mer.gd`) : la surface, puis les `d`
    // mètres d'eau jusqu'à l'œil, à leur profondeur moyenne. Pas `FRONT_FACING` : la grille polaire présente sa face
    // avant par en dessous, et la bascule reste d'un bloc tant que la caméra à demi immergée n'est pas traitée.
    if (sous_eau) {
        vec3 vue_d = normalize(surface - CAMERA_POSITION_WORLD);
        vec3 rayon_d = vec3(vue_d.x, -vue_d.z, vue_d.y);
        vec3 lum = integree(pente, rayon_d, transport_covariance(energie * cov_q, g), true, vec3(0.0), vec3(0.0));
        float d = length(surface - CAMERA_POSITION_WORLD);
        ALBEDO = a_travers_l_eau(lum, d, rayon_d, max(-0.5 * (surface.y + CAMERA_POSITION_WORLD.y), 0.0));
        if (controle == 4) {
            // Contrôle S365 : le coefficient de Fresnel eau → air, sans la ligne de visée.
            ALBEDO = vec3(fresnel(clamp(dot(rayon_d, normalize(vec3(-pente, 1.0))), 0.0, 1.0), 1.0 / indice));
        }
    } else {
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
        if (uv.x > 0.0 && uv.x < 1.0 && uv.y > 0.0 && uv.y < 1.0) {
            vec3 f1 = scene_monde(uv, INV_PROJECTION_MATRIX, INV_VIEW_MATRIX);
            if (f1.y < surface.y) {
                uv_r = uv;
                fond = f1;
            }
        }
    }
    float h_fond = max(surface.y - fond.y, 0.0);
    vec3 t_colonne = exp(-kd * (h_fond + length(fond - surface)));
    vec3 fond_vu = textureLod(ecran, uv_r, 0.0).rgb * t_colonne;
    // La lumière, pente non résolue intégrée (ADR-161) : Cholesky de sa covariance, 3 × 3 nœuds de Gauss-Hermite.
    vec3 rayon = vec3(vue.x, -vue.z, vue.y);
    vec3 couleur = integree(pente, rayon, transport_covariance(energie * cov_q, g), false, fond_vu, t_colonne);
    // Les crêtes (S356, `cretes_couleur`) : la lumière qui traverse ce qui se comprime, vue du côté du soleil ; l'écume.
    vec3 soleil = normalize(SOLEIL_B);
    float eclairage = gain_eau * (0.6 + 0.4 * max(dot(normalize(vec3(-pente, 1.0)), soleil), 0.0));
    vec2 horizontal = rayon.xy / max(length(rayon.xy), 1e-6);
    float avant = pow(max(dot(horizontal, normalize(soleil.xy)), 0.0), exposant_cretes);
    couleur += force_cretes * clamp(-0.5 * s_e, 0.0, 1.0) * avant * transmission_crete * eclairage;
    couleur = mix(couleur, vec3(reflectance_ecume * eclairage), couverture);
    ALBEDO = couleur;
    if (controle > 0 && controle < 4) {
        ALBEDO = controle == 1 ? vec3(clamp(h0 / echelle_controle, 0.0, 1.0)) : (controle == 2 ? t_colonne : vec3(couverture));
    }
    }
}
