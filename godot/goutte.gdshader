// S380 — LE DESSIN D'UNE GOUTTE QUI TOMBE (ADR-205, pièce 1) : la traînée que placent `pluie_air.gdshader` et
// `pluie_air.gd`. Garg et Nayar (2007, « Vision and Rain ») : une goutte est une lentille grand-angle — elle renvoie vers
// l'œil la lumière d'un cône d'environ 165° autour de la direction de visée, et sa radiance vaut ≈ 0,94 de la moyenne de
// l'environnement sur ce cône ; elle ne couvre qu'une part du pixel et n'y passe qu'une part du temps d'exposition :
// l'opacité, calculée par la particule. Ici l'environnement est approché par sa moitié haute — le ciel de la scène
// (`ciel.gdshaderinc`) à 30°, 60° et au zénith — et sa moitié basse, sol ou eau, prise à 0,2 fois le ciel au zénith.
// **Approché, pas mesuré.** Contrôle (critère 2 de S380) : une couleur pure par classe de taille.
shader_type spatial;
render_mode unshaded, blend_mix, depth_draw_never, cull_disabled, shadows_disabled, fog_disabled;

#include "res://ciel.gdshaderinc"

uniform bool controle = false;
varying vec4 donnees;

void vertex() {
    donnees = INSTANCE_CUSTOM;
}

/// Une direction de Godot (x, y en haut, z au sud) en coordonnées de B pour le ciel.
vec3 vers_b(vec3 d) {
    return vec3(d.x, -d.z, d.y);
}

void fragment() {
    if (controle) {
        float d = donnees.x;
        ALBEDO = d < 1.5 ? vec3(1.0, 0.0, 0.0) : (d < 2.0 ? vec3(0.0, 1.0, 0.0) : (d < 3.0 ? vec3(0.0, 0.0, 1.0) : vec3(1.0)));
        ALPHA = 1.0;
    } else {
        vec3 monde = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
        vec3 vue = normalize(monde - CAMERA_POSITION_WORLD);
        vec3 horizontale = normalize(vec3(vue.x, 0.0, vue.z) + vec3(1e-6, 0.0, 0.0));
        vec3 haut = vec3(0.0, 1.0, 0.0);
        vec3 ciel = (ciel_b(vers_b(normalize(horizontale + 0.577 * haut)), 2)
            + ciel_b(vers_b(normalize(horizontale + 1.732 * haut)), 2) + ciel_b(vec3(0.0, 0.0, 1.0), 2)) / 3.0;
        vec3 sol = 0.2 * ciel_b(vec3(0.0, 0.0, 1.0), 2);
        ALBEDO = 0.94 * 0.5 * (ciel + sol);
        ALPHA = clamp(donnees.z, 0.0, 1.0);
    }
}
