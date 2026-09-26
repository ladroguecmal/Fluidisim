// S382 P6c — **LA CUISSON DE LA PART DU CIEL VUE** (ADR-206). Évaluée à chaque image dans `vertex()`, elle coûtait 2,3 à
// 2,5 ms (≈ 110 000 sommets, deux passes) ; elle ne change que si un occultant ou une surface bouge. Une passe de rendu
// unique la calcule donc pour tous les sommets : chaque sommet de la scène est un **point** (maillage en coordonnées du
// monde, position et normale), placé au texel de son rang (`VERTEX_ID`) dans une texture flottante de `largeur` texels de
// large ; il y écrit (part du ciel uniforme, part du ciel couvert de la CIE). Les surfaces la relisent à leur sommet
// (`ciel_vu_cuit`, `occultation.gdshaderinc`). Refaite quand un occultant change (`piscine.gd`, `cuire_ciel_vu`).
shader_type spatial;
render_mode unshaded, cull_disabled, depth_test_disabled, depth_draw_never;

#include "res://ciel.gdshaderinc"
#include "res://occultation.gdshaderinc"

uniform int largeur = 1024;
uniform int hauteur = 1;

varying vec2 vu;

void vertex() {
    vu = ciel_vu(VERTEX, normalize(NORMAL));
    int i = VERTEX_ID;
    float x = (float(i % largeur) + 0.5) / float(largeur);
    float y = (float(i / largeur) + 0.5) / float(hauteur);
    POSITION = vec4(2.0 * x - 1.0, 2.0 * y - 1.0, 0.5, 1.0);
    POINT_SIZE = 1.0;
}

void fragment() {
    ALBEDO = vec3(vu, 0.0);
}
