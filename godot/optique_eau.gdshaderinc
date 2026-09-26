// S365 — L'OPTIQUE DE L'EAU, une seule source pour la surface (`eau.gdshader`), le fond (`sol.gdshader`) et le fond du
// ciel (`ciel.gdshader`) : ce que l'eau renvoie et ce qu'elle éteint. Valeurs données par `mer.gd`, qui porte les
// constantes (Pope & Fry 1997, Morel 1974 : celles d'ADR-177 et de S359) ; les défauts ci-dessous en sont la copie.
//
// **Vue de l'intérieur** (liste 8.6, ADR-019 §3). Sur la ligne de visée, Beer–Lambert par canal : `T = exp(−c·d)`, `c =
// a + b` l'atténuation du faisceau (Duntley 1963 ; Mobley 1994, *Light and Water*, §3.2), `b = 2·b_b` pour la diffusion
// moléculaire, symétrique. Ce que la ligne ajoute : la radiance de l'eau elle-même, `L∞·(1 − T)`, avec, à la profondeur
// `z`, `L∞ = R0·E·exp(−Kd·z)·f(ω)` — `R0·E` ce que l'eau renvoie vers le haut sous la surface (ADR-177), `exp(−Kd·z)`
// l'éclairement qui descend, `f(ω)` la forme de la distribution de radiance : **S366, calée sur la mesure** de Tyler
// (1960, lac Pend Oreille, 4,2 m ; `outils/tyler_radiance.py`) — un lobe avant de Henyey-Greenstein autour du soleil
// réfracté, `f = a0 + β·(1 − cos θ) + K·HG(g, ω·s)`, `f(nadir) = 1` ; résidu logarithmique 0,12 contre 0,36 pour la forme
// sans azimut de S365. Eau de lac, où les particules diffusent vers l'avant ; notre eau est pure : lobe emprunté, dit.
// Domaine mesuré : l'hémisphère bas et l'horizontale ; au-dessus, la valeur de l'horizontale au même azimut.

// Réflectance sous la surface R(0⁻) et `E/π` (ADR-177) ; atténuation diffuse `Kd = (a + b_b)/μ̄_d` ; atténuation du
// faisceau `c = a + 2·b_b`.
uniform vec3 R0 = vec3(0.00068, 0.00826, 0.08960);
// `gain_eau`, `E/π` : S381, déclaré dans `ciel.gdshaderinc` (l'éclairement de la scène ; le ciel couvert en tire sa
// luminance) — toujours inclus avant ce fichier.
uniform vec3 kd = vec3(0.4259, 0.0724, 0.0158);
uniform vec3 attenuation_c = vec3(0.3414, 0.0594, 0.0161);
// La caméra est-elle dans l'eau, et à quelle profondeur (m) : `mer.gd`, à chaque image, depuis la bande de B.
uniform bool sous_eau = false;
uniform float profondeur_camera = 0.0;
// Indice de l'eau de mer (S365 : ici, pour la surface et pour le soleil réfracté). Le lobe (β, K, g) : `mer.gd`.
uniform float indice = 1.34;
uniform vec3 lobe = vec3(1.0031, 213.349, 0.855);

/// Henyey-Greenstein, par stéradian.
float henyey_greenstein(float g, float mu) {
    return (1.0 - g * g) / (4.0 * PI * pow(1.0 + g * g - 2.0 * g * mu, 1.5));
}

/// Le soleil vu de l'eau (B, unitaire, vers le soleil) : `SOLEIL_B` réfracté par une surface plane (Snell).
vec3 soleil_dans_l_eau() {
    vec3 s = normalize(SOLEIL_B);
    float h = length(s.xy);
    float st = h / indice;
    return vec3(s.xy / max(h, 1e-6) * st, sqrt(max(1.0 - st * st, 0.0)));
}

/// La radiance d'une ligne de visée infinie dans l'eau, direction `omega` (B, unitaire), à la profondeur `z` (m).
vec3 eau_infinie(vec3 omega, float z) {
    vec3 o = normalize(vec3(omega.xy + vec2(1e-5, 0.0), min(omega.z, 0.0)));
    vec3 s = soleil_dans_l_eau();
    float f = 1.0 - lobe.y * henyey_greenstein(lobe.z, -s.z) + lobe.x * (1.0 + o.z)
        + lobe.y * henyey_greenstein(lobe.z, dot(o, s));
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
