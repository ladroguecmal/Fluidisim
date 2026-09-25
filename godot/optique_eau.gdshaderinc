// S365 — L'OPTIQUE DE L'EAU, une seule source pour la surface (`eau.gdshader`), le fond (`sol.gdshader`) et le fond du
// ciel (`ciel.gdshader`) : ce que l'eau renvoie et ce qu'elle éteint. Valeurs données par `mer.gd`, qui porte les
// constantes (Pope & Fry 1997, Morel 1974 : celles d'ADR-177 et de S359) ; les défauts ci-dessous en sont la copie.
//
// **Vue de l'intérieur** (liste 8.6, ADR-019 §3). Sur la ligne de visée, Beer–Lambert par canal : `T = exp(−c·d)`, `c =
// a + b` l'atténuation du faisceau (Duntley 1963 ; Mobley 1994, *Light and Water*, §3.2), `b = 2·b_b` pour la diffusion
// moléculaire, symétrique. Ce que la ligne ajoute : la radiance de l'eau elle-même, `L∞·(1 − T)`, avec, à la profondeur
// `z`, `L∞ = R0·E·exp(−Kd·z)·f(ω)` — `R0·E` ce que l'eau renvoie vers le haut sous la surface (ADR-177), `exp(−Kd·z)`
// l'éclairement qui descend, `f(ω)` = 1 au nadir, 3 à l'horizontale et au-dessus : en eau claire la radiance horizontale
// vaut quelques fois la montante (Mobley 1994, ch. 10) — **à calibrer** sur le banc B11.

// Réflectance sous la surface R(0⁻) et `E/π` (ADR-177) ; atténuation diffuse `Kd = (a + b_b)/μ̄_d` ; atténuation du
// faisceau `c = a + 2·b_b`.
uniform vec3 R0 = vec3(0.00068, 0.00826, 0.08960);
uniform float gain_eau = 2.0;
uniform vec3 kd = vec3(0.4259, 0.0724, 0.0158);
uniform vec3 attenuation_c = vec3(0.3414, 0.0594, 0.0161);
// La caméra est-elle dans l'eau, et à quelle profondeur (m) : `mer.gd`, à chaque image, depuis la bande de B.
uniform bool sous_eau = false;
uniform float profondeur_camera = 0.0;

/// La radiance d'une ligne de visée infinie dans l'eau, direction `omega` (B, unitaire), à la profondeur `z` (m).
vec3 eau_infinie(vec3 omega, float z) {
    float f = 1.0 + 2.0 * clamp(1.0 + omega.z, 0.0, 1.0);
    return R0 * gain_eau * exp(-kd * max(z, 0.0)) * f;
}

/// Ce qu'on voit à travers `d` mètres d'eau depuis l'œil, à la profondeur `z0`, dans la direction `omega` (B). La lumière
/// que la ligne diffuse vers l'œil suit l'éclairement local, `exp(−Kd·z)`, et la profondeur varie le long de la ligne,
/// `z = z0 + g·s`, `g = −ω_z` : `∫₀^d L∞(z0)·e^(−Kd·g·s)·c·e^(−c·s) ds = L∞(z0)·c·d·φ((c + Kd·g)·d)`, `φ(x) = (1 − e^(−x))/x`
/// — exact pour ce modèle. S365 P3 : la profondeur **moyenne** du trajet traçait une ligne entre le fond lointain et le
/// fond du ciel ; la lumière diffusée vient du premier `1/c` mètres, pas du milieu du trajet.
vec3 a_travers_l_eau(vec3 couleur, float d, vec3 omega, float z0) {
    d = max(d, 0.0);
    vec3 x = (attenuation_c + kd * (-omega.z)) * d;
    vec3 phi = vec3(0.0);
    for (int i = 0; i < 3; i++) {
        phi[i] = abs(x[i]) < 1e-4 ? 1.0 - 0.5 * x[i] : (1.0 - exp(-x[i])) / x[i];
    }
    return couleur * exp(-attenuation_c * d) + eau_infinie(omega, z0) * attenuation_c * d * phi;
}
