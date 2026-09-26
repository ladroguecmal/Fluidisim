// S374 — LES PAROIS DE LA PISCINE (`piscine.gd`) : béton, carrelage, sol. Éclairées comme le fond de la scène côtière
// (`sol.gdshaderinc`, S359) et le corps d'eau — `albédo · E/π · (0,6 + 0,4·max(n·soleil, 0))` —, et non par Godot : vues
// à travers l'eau (`bassin.gdshader`), elles doivent être dans les mêmes unités que la lumière de l'eau. Albédos
// d'habillage, sans provenance autre que leur ordre de grandeur (béton clair ≈ 0,4, carrelage de piscine bleu clair).
// Le carrelage : joints tous les `joint` mètres, un peu plus sombres.
shader_type spatial;
render_mode unshaded;

#include "res://ciel.gdshaderinc"
#include "res://optique_eau.gdshaderinc"

uniform vec3 albedo = vec3(0.4);
uniform float joint = 0.0;

varying vec3 monde;

void vertex() {
    monde = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
}

void fragment() {
    vec3 n = normalize((INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
    vec3 nb = vec3(n.x, -n.z, n.y);
    float motif = 1.0;
    if (joint > 0.0) {
        // Les deux coordonnées du plan de la face : celles qui ne sont pas sa normale.
        vec3 a = abs(n);
        vec2 q = a.y > 0.5 ? monde.xz : (a.x > 0.5 ? monde.zy : monde.xy);
        vec2 f = abs(fract(q / joint + 0.5) - 0.5) * joint;
        float largeur = 0.004;
        motif = 1.0 - 0.35 * (1.0 - smoothstep(largeur, largeur + max(fwidth(q.x), 1e-4), min(f.x, f.y)));
    }
    ALBEDO = albedo * motif * gain_eau * (0.6 + 0.4 * max(dot(nb, normalize(SOLEIL_B)), 0.0));
}
