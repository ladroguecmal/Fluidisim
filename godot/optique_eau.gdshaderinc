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

// S477 — **LE TYPE D'EAU DANS L'ESPACE** (ADR-217 D1). Les propriétés optiques d'un point — `R0`, `kd`, `c` — dans une structure,
// que les fonctions de la lumière de l'eau reçoivent (Godot n'a pas de variable globale modifiable). `optique_en(xy)` la lit dans la
// **carte des constituants** — par texel, Chl (mg/m³), a_g(440) (m⁻¹), MES (g/m³), sur le rectangle `type_eau_zone` de B — et la
// calcule par le modèle de `outils/type_eau.py` (ses sources, son en-tête) ; hors de la carte, ou sans elle, les uniformes de la scène,
// au bit. Les constituants s'interpolent linéairement entre texels — une approximation : `Chl^e` n'est pas linéaire.
struct Optique {
    vec3 R0;
    vec3 kd;
    vec3 c;
};
uniform bool type_eau_carte_active = false;
uniform sampler2D type_eau_carte : filter_linear, repeat_disable;
uniform vec4 type_eau_zone = vec4(0.0, 0.0, 1.0, 1.0);

/// Les uniformes de la scène (l'eau pure d'ADR-177, ou `TYPE_EAU`).
Optique optique_scene() {
    return Optique(R0, kd, attenuation_c);
}

/// Le modèle de `type_eau.py` : l'eau pure d'ADR-177 plus le phytoplancton (Morel et Maritorena 2001), la matière dissoute et les
/// particules minérales (Babin et al. 2003), aux bandes 650, 550, 450 nm.
Optique optique_constituants(vec3 k) {
    const vec3 A_EAU = vec3(0.340, 0.0565, 0.00922);
    const vec3 BB_EAU = vec3(0.00071, 0.00145, 0.00344);
    const vec3 LAMBDA = vec3(650.0, 550.0, 450.0);
    const vec3 CHI = vec3(0.04500, 0.04111, 0.10165);
    const vec3 EXPO = vec3(0.67200, 0.64927, 0.67692);
    const float MU_D = 0.8;
    float chl = max(k.x, 0.0);
    float ag = max(k.y, 0.0);
    float mes = max(k.z, 0.0);
    vec3 a = A_EAU;
    vec3 bb = BB_EAU;
    vec3 b = 2.0 * BB_EAU;
    if (chl > 0.0) {
        float l10 = log(chl) / log(10.0);
        float bp550 = 0.416 * pow(chl, 0.766);
        float nu = chl <= 2.0 ? 0.5 * (clamp(l10, log(0.02) / log(10.0), log(2.0) / log(10.0)) - 0.3) : 0.0;
        vec3 bbp = (0.002 + 0.01 * (0.5 - 0.25 * l10) * pow(LAMBDA / 550.0, vec3(nu))) * bp550;
        a += max(MU_D * CHI * pow(vec3(chl), EXPO) - bbp, vec3(0.0));
        bb += bbp;
        b += bp550 * 550.0 / LAMBDA;
    }
    a += ag * exp(-0.0176 * (LAMBDA - 440.0));
    if (mes > 0.0) {
        a += 0.04 * mes * exp(-0.0123 * (LAMBDA - 443.0));
        vec3 bpm = 0.5 * mes * 555.0 / LAMBDA;
        b += bpm;
        bb += 0.018 * bpm;
    }
    return Optique(0.33 * bb / (a + bb), (a + bb) / MU_D, a + b);
}

/// Les propriétés optiques au point `xy` de B : la carte, ou la scène.
Optique optique_en(vec2 xy) {
    if (!type_eau_carte_active) {
        return optique_scene();
    }
    vec2 uv = (xy - type_eau_zone.xy) / (type_eau_zone.zw - type_eau_zone.xy);
    if (uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0) {
        return optique_scene();
    }
    return optique_constituants(textureLod(type_eau_carte, uv, 0.0).rgb);
}

/// La radiance d'une ligne de visée infinie dans l'eau, direction `omega` (B, unitaire), à la profondeur `z` (m).
vec3 eau_infinie_o(Optique op, vec3 omega, float z) {
    vec3 o = normalize(vec3(omega.xy + vec2(1e-5, 0.0), min(omega.z, 0.0)));
    vec3 s = soleil_dans_l_eau();
    float f = 1.0 - lobe.y * henyey_greenstein(lobe.z, -s.z) + lobe.x * (1.0 + o.z)
        + lobe.y * henyey_greenstein(lobe.z, dot(o, s));
    return op.R0 * gain_eau * exp(-op.kd * max(z, 0.0)) * f;
}

vec3 eau_infinie(vec3 omega, float z) {
    return eau_infinie_o(optique_scene(), omega, z);
}

/// Ce qu'on voit à travers `d` mètres d'eau depuis l'œil, à la profondeur `z0`, dans la direction `omega` (B). La lumière
/// que la ligne diffuse vers l'œil suit l'éclairement local, `exp(−Kd·z)`, et la profondeur varie le long de la ligne,
/// `z = z0 + g·s`, `g = −ω_z` : `∫₀^d L∞(z0)·e^(−Kd·g·s)·c·e^(−c·s) ds = L∞(z0)·c·d·φ((c + Kd·g)·d)`, `φ(x) = (1 − e^(−x))/x`
/// — exact pour ce modèle. S365 P3 : la profondeur **moyenne** du trajet traçait une ligne entre le fond lointain et le
/// fond du ciel ; la lumière diffusée vient du premier `1/c` mètres, pas du milieu du trajet.
vec3 a_travers_l_eau_o(Optique op, vec3 couleur, float d, vec3 omega, float z0) {
    d = max(d, 0.0);
    vec3 x = (op.c + op.kd * (-omega.z)) * d;
    vec3 phi = vec3(0.0);
    for (int i = 0; i < 3; i++) {
        phi[i] = abs(x[i]) < 1e-4 ? 1.0 - 0.5 * x[i] : (1.0 - exp(-x[i])) / x[i];
    }
    return couleur * exp(-op.c * d) + eau_infinie_o(op, omega, z0) * op.c * d * phi;
}

vec3 a_travers_l_eau(vec3 couleur, float d, vec3 omega, float z0) {
    return a_travers_l_eau_o(optique_scene(), couleur, d, omega, z0);
}
