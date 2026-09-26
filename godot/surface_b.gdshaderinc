// S371 — LA SURFACE DE B À L'APLOMB D'UN POINT, ET LE MILIEU DU PIXEL : la caméra à demi immergée (ADR-019 §6, liste
// 8.6). Une seule source pour l'eau (`eau.gdshader`), le fond (`sol.gdshader`) et le ciel (`ciel.gdshader`) ; à inclure
// après `optique_eau.gdshaderinc` (`sous_eau`, `profondeur_camera`).
//
// **Le milieu se décide par pixel, à l'objectif.** Le point où le rayon du pixel traverse le plan proche de la caméra est
// sous la surface de B à son aplomb, ou au-dessus : le pixel est vu de l'eau, ou de l'air. Rien avant le plan proche n'est
// rendu ; la ligne d'eau sur l'objectif est l'intersection de la surface et de ce plan. Calculée ici, analytiquement,
// elle ne scintille pas comme une ligne qui émergerait du maillage (ADR-019 §6). Hors de la zone des vagues
// (`demi_immergee` faux, posé par `mer.gd`), le milieu est celui de l'œil, d'un bloc, comme depuis S365.
//
// Hublot en **dôme** (déclaré, S371 P2) : pas de grossissement de la moitié immergée ; la transition, d'environ 1 % de la
// hauteur d'image sur les photographies de référence, est celle de la surface floue qui touche le hublot.

// Les composantes de la bande `[a, kx, ky, φ(t)]` (phases repliées par `mer.gd` en double précision, I-08) et le second
// ordre de Tayfun par système (ADR-176 D1) — déclarés dans `eau.gdshader` jusqu'à S370.
uniform vec4 bande[64];
uniform int n_bande = 0;
uniform int split = 64;
uniform vec2 k_moyens = vec2(0.0);

uniform bool demi_immergee = false;
// La caméra : position, et sa base dans le monde de Godot (x, y en haut, z au sud) ; son plan proche, m ; la taille d'un
// pixel sur ce plan, `2·near·tan(fov/2)/hauteur`, m.
uniform vec3 camera_position = vec3(0.0);
uniform vec3 camera_avant = vec3(0.0, 0.0, -1.0);
uniform vec3 camera_droite = vec3(1.0, 0.0, 0.0);
uniform vec3 camera_haut = vec3(0.0, 1.0, 0.0);
uniform float plan_proche = 0.1;
uniform float pixel_proche = 1.3e-4;
// La largeur de la transition, pixels.
uniform float transition_px = 4.0;

/// La bande au point lagrangien `q` (B), à empreinte nulle — toutes les composantes entières, comme `vertex()` d'`eau.gdshader`
/// près de l'œil : le déplacement horizontal `d`, son gradient `g` = (g_xx, g_xy, g_yy), la hauteur `eta` avec le second
/// ordre, et sa pente lagrangienne `s`.
void bande_au_point(vec2 q, out vec2 d, out vec3 g, out float eta, out vec2 s) {
    d = vec2(0.0);
    g = vec3(0.0);
    eta = 0.0;
    s = vec2(0.0);
    vec2 e2 = vec2(0.0), qd = vec2(0.0);
    vec4 sa = vec4(0.0), sq = vec4(0.0);
    for (int i = 0; i < n_bande; i++) {
        vec4 c = bande[i];
        float k = length(c.yz);
        if (k == 0.0) { continue; }
        float a = c.x;
        float phase = dot(c.yz, q) + c.w;
        vec2 u = c.yz / k;
        float cs = cos(phase);
        float sn = sin(phase);
        d += a * cs * u;
        eta += a * sn;
        s += a * cs * c.yz;
        g -= a * k * sn * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
        if (i < split) {
            e2.x += a * sn; qd.x += a * cs;
            sa.xy += a * cs * c.yz; sq.xy -= a * sn * c.yz;
        } else {
            e2.y += a * sn; qd.y += a * cs;
            sa.zw += a * cs * c.yz; sq.zw -= a * sn * c.yz;
        }
    }
    if (k_moyens.x > 0.0) {
        eta += 0.5 * k_moyens.x * (e2.x * e2.x - qd.x * qd.x) + 0.5 * k_moyens.y * (e2.y * e2.y - qd.y * qd.y);
        s += k_moyens.x * (e2.x * sa.xy - qd.x * sq.xy) + k_moyens.y * (e2.y * sa.zw - qd.y * sq.zw);
    }
}

/// La surface de B à l'aplomb du point horizontal `x` (B) : (hauteur, pente eulérienne). Le point lagrangien `q` tel que
/// `q + d(q) = x`, par Newton (`J = I + g`, symétrique), quatre évaluations — repli en point fixe si `det J < 0,1`, comme
/// la pente d'`eau.gdshader` ; puis le résidu `f` de la dernière, corrigé au premier ordre : `η(x) ≈ η(q) − ∇η·f`.
vec3 surface_a_l_aplomb(vec2 x) {
    vec2 q = x;
    vec2 d;
    vec3 g;
    float eta;
    vec2 s;
    vec2 f = vec2(0.0);
    float jxx = 1.0, jxy = 0.0, jyy = 1.0, det = 1.0;
    for (int it = 0; it < 4; it++) {
        bande_au_point(q, d, g, eta, s);
        f = q + d - x;
        jxx = 1.0 + g.x;
        jxy = g.y;
        jyy = 1.0 + g.z;
        det = jxx * jyy - jxy * jxy;
        if (it < 3) {
            q -= det >= 0.1 ? vec2(jyy * f.x - jxy * f.y, -jxy * f.x + jxx * f.y) / det : f;
        }
    }
    vec2 pente = det >= 0.1 ? vec2((jyy * s.x - jxy * s.y) / det, (-jxy * s.x + jxx * s.y) / det) : s;
    return vec3(eta - dot(pente, f), pente);
}

/// Le milieu du pixel dont le rayon part de `oeil` dans la direction `dir` (monde de Godot, unitaire) : x, la part d'eau,
/// de 0 (air) à 1 (eau) ; y, la profondeur de l'objectif sous la surface, m (≥ 0) ; z, la distance signée de l'objectif à
/// la ligne d'eau, en pixels, positive dans l'air. La distance est `s/|∇s|`, `s` la hauteur de l'objectif au-dessus de la
/// surface et `∇s` pris d'un pixel au suivant sur le plan proche — exact au premier ordre.
vec3 milieu_du_pixel(vec3 oeil, vec3 dir) {
    if (!demi_immergee) {
        return vec3(sous_eau ? 1.0 : 0.0, profondeur_camera, sous_eau ? -1e9 : 1e9);
    }
    vec3 p = oeil + dir * (plan_proche / max(dot(dir, camera_avant), 1e-4));
    vec3 surf = surface_a_l_aplomb(vec2(p.x, -p.z));
    float s = p.y - surf.x;
    // ∇s dans le monde de Godot : un point (x, y) de B est (x, −z) ; s = y − η(x, −z).
    vec3 grad = vec3(-surf.y, 1.0, surf.z);
    float par_pixel = pixel_proche * length(vec2(dot(grad, camera_droite), dot(grad, camera_haut)));
    float dist = s / max(par_pixel, 1e-12);
    float t = max(transition_px, 1e-3);
    float w = 1.0 - smoothstep(-0.5 * t, 0.5 * t, dist);
    return vec3(w, max(-s, 0.0), dist);
}
