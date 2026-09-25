// S357, ADR-192 D2 — LA MER DE B DANS GODOT : le nuanceur d'eau du prototype.
//
// Port de `viewer/src/water.wgsl` (bande CWM d'ADR-157, second ordre de Tayfun d'ADR-176, queue en pentes filtrées
// d'ADR-155 et ADR-161, modulation d'ADR-158) et de `viewer/src/water_cretes.wgsl` (S356 : l'écume à la couverture de
// Monahan, un seuil par empreinte ; la lumière des crêtes). Axes de B : x à l'est, y au nord, z en haut ; Godot :
// x, y en haut, z au sud — un point (x, y) de B est (x, −z) dans Godot. Ce que ce nuanceur ne fait plus lui-même,
// Godot le fait : ciel, soleil, reflets (GGX), tonalité, brume. La pente que la maille ne résout pas devient rugosité.
shader_type spatial;
render_mode cull_disabled, specular_schlick_ggx, diffuse_lambert;

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
// La lumière des crêtes (S356) : transmission de l'eau pure sur Hs ; force à calibrer (R19).
uniform vec3 transmission_crete = vec3(0.5987, 0.9187, 0.9863);
uniform float force_cretes = 0.15;
// Couleur du corps d'eau : R(0⁻) de Pope & Fry et Morel (ADR-177), en réflectance diffuse (t²/n²·π/Q ≈ 0,54, Q = π).
uniform vec3 R0 = vec3(0.00068, 0.00826, 0.08960);
// Empreinte angulaire d'un pixel, `2·tan(fov/2)/hauteur`, et pas radial relatif de la grille polaire.
uniform float angle_pixel = 0.0013;
uniform float pas_radial = 0.03;
// S359 — témoin du diagnostic de l'horizon : la rugosité confiée à Godot, bornée (1 : sans borne).
uniform float rugosite_max = 1.0;

varying vec2 lag;
varying vec3 g_bande;
varying vec2 pente_bande;
varying float eps;
varying float sigma2_bande;

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
}

void fragment() {
    float h = max(length(dFdx(lag)), length(dFdy(lag)));
    // La queue filtrée : pentes, gradient du déplacement, variance résolue, et variance que la maille ne résout pas.
    vec2 s_q = vec2(0.0);
    vec3 g_q = vec3(0.0);
    float var_q = 0.0, var_non = 0.0;
    for (int i = 0; i < n_queue; i++) {
        vec4 c = queue[i];
        float k = length(c.yz);
        float w = poids(k, h) * exp(-0.5 * k * k * h * h);
        var_non += 0.5 * (c.x * k) * (c.x * k) * (1.0 - w * w);
        if (w > 0.0) {
            float phase = dot(c.yz, lag) + c.w;
            vec2 u = c.yz / k;
            s_q += w * c.x * cos(phase) * c.yz;
            g_q -= w * c.x * k * sin(phase) * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
            var_q += 0.5 * (w * c.x * k) * (w * c.x * k);
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
    NORMAL = normalize((VIEW_MATRIX * vec4(n_monde, 0.0)).xyz);
    // L'écume (S356) : couverture antialiasée par la variation de `s` dans le pixel.
    float sigma = sqrt(max(sigma2_bande + energie * var_q, 0.0));
    float s = (det - 1.0) / max(sigma, 1e-6);
    float couverture = clamp((seuil_ecume(h) - s) / max(fwidth(s), 1e-4) + 0.5, 0.0, 1.0);
    // Corps d'eau, écume ; la pente non résolue en rugosité GGX : α = √mss, rugosité de Godot = √α.
    ALBEDO = mix(0.54 * R0, vec3(reflectance_ecume), couverture);
    float rugosite = clamp(pow(max(energie * var_non, 1e-8), 0.25), 0.02, 1.0);
    ROUGHNESS = mix(min(rugosite, rugosite_max), 0.9, couverture);
    SPECULAR = mix(0.25, 0.1, couverture);
    METALLIC = 0.0;
    // La lumière des crêtes : transmise par ce qui se comprime, éclairée par Godot depuis l'arrière de la crête.
    BACKLIGHT = force_cretes * clamp(-0.5 * s, 0.0, 1.0) * transmission_crete * (1.0 - couverture);
}
