// S356, ADR-191 — **LES CRÊTES DE B : L'ÉCUME ET LA LUMIÈRE QUI LES TRAVERSE** (liste 8.4 ; RENDU-ECART-S307 §6).
//
// Module séparable du rendu de l'eau, concaténé avant `water.wgsl`, qui l'appelle. Ses entrées sont explicites : la
// variable normalisée `s = (J − 1)/σ` du jacobien `J` du déplacement CWM (`σ`, l'écart-type de sa partie linéaire au
// filtrage du pixel), sa variation dans le pixel, l'empreinte `h`, la normale, le rayon et la couleur déjà calculée.
// Ses paramètres viennent de l'hôte (`rendu_cretes.rs`) : `p.ecume_seuils` et `p.cretes`. Options éteintes, la
// couleur rendue est celle d'avant, au bit.
//
// **L'écume** : la couverture de Monahan & O'Muircheartaigh (1980) au vent de la mer — 0,42 % à `U₁₀` = 7,8 m/s —
// tirée d'un seuil sur `s`, **un par empreinte** (le filtrage alourdit la queue basse de `s` : un seuil unique
// donnait jusqu'à deux fois la couverture au loin) ; réflectance effective de Koepke (1984), 0,22, sous l'éclairement
// du corps d'eau d'ADR-177 — le gain `p.cut.z` y tient lieu de `E/π`.
//
// **La lumière des crêtes** (Sea of Thieves, SIGGRAPH 2018) : là où la surface se comprime (`s < 0`), le soleil
// traverse la crête et en ressort vers qui regarde du côté du soleil. Sa teinte est la **transmission de l'eau pure**
// sur l'épaisseur d'une crête, `exp(−a·L)` : `a` de Pope & Fry (1997), les absorptions mêmes de la couleur d'ADR-177
// (0,342 ; 0,0565 ; 0,00923 m⁻¹ à 650, 550, 450 nm), `L` = `Hs` de la mer de vent, 1,5 m — un cyan clair. Sa force
// (la part de la lumière incidente qui ressort de la crête vers l'œil) et l'exposant de sa diffusion vers l'avant
// sont **à calibrer par la revue R19** (I-14).

/// Transmission de l'eau pure sur 1,5 m, `exp(−a·L)` aux trois longueurs d'onde de la couleur d'ADR-177.
const TRANSMISSION_CRETE = vec3<f32>(0.5987, 0.9187, 0.9863);

/// Le seuil de `s` à l'empreinte d'indice `i`, `h_i = 2^(i−7)` m, quatre par vecteur.
fn ecume_table(i: u32) -> f32 {
    let v = p.ecume_seuils[i / 4u];
    let r = i % 4u;
    if (r == 0u) { return v.x; }
    if (r == 1u) { return v.y; }
    if (r == 2u) { return v.z; }
    return v.w;
}

/// Le seuil à l'empreinte `h`, interpolé en `log₂ h` entre 7,8 mm et 64 m, comme `seuil_a` de l'hôte.
fn ecume_seuil(h: f32) -> f32 {
    let x = clamp(log2(max(h, 0.0078125)) + 7.0, 0.0, 13.0);
    let i = min(u32(floor(x)), 12u);
    let f = x - f32(i);
    return ecume_table(i) * (1.0 - f) + ecume_table(i + 1u) * f;
}

/// Les crêtes sur la couleur déjà rendue. `largeur` est `fwidth(s)`, calculé par l'appelant en flot uniforme : la
/// couverture du pixel est la part de sa variation de `s` qui passe sous le seuil — un bord antialiasé sans paramètre.
fn cretes_couleur(color: vec3<f32>, s: f32, largeur: f32, h: f32, n: vec3<f32>, ray: vec3<f32>) -> vec3<f32> {
    var c = color;
    let sun = normalize(vec3<f32>(-0.4, 0.3, 0.8));
    let eclairage = p.cut.z * (0.6 + 0.4 * max(dot(n, sun), 0.0));
    if (p.cretes.y > 0.0) {
        // Comprimée de deux écarts-types, la crête est pleinement translucide ; alignée sur le soleil en azimut, elle
        // diffuse vers l'œil.
        let masque = clamp(-0.5 * s, 0.0, 1.0);
        let r = ray.xy / max(length(ray.xy), 1e-6);
        let avant = pow(max(dot(r, normalize(sun.xy)), 0.0), p.cretes.z);
        c += p.cretes.y * masque * avant * TRANSMISSION_CRETE * eclairage;
    }
    if (p.cretes.x > 0.0) {
        let couverture = clamp((ecume_seuil(h) - s) / max(largeur, 1e-4) + 0.5, 0.0, 1.0);
        c = mix(c, vec3<f32>(p.cretes.x * eclairage), couverture);
    }
    return c;
}
