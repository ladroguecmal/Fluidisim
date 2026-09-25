// S359 — le ciel de la scène : le ciel clair de l'afficheur (`ciel.gdshaderinc`), celui que l'eau reflète.
shader_type sky;

#include "res://ciel.gdshaderinc"
// S365 : l'œil dans l'eau ne voit pas le ciel au-delà de ce que la scène couvre, mais l'eau elle-même, sur une ligne
// infinie (`optique_eau.gdshaderinc`).
#include "res://optique_eau.gdshaderinc"

void sky() {
    vec3 dir = vec3(EYEDIR.x, -EYEDIR.z, EYEDIR.y);
    // Ligne infinie : `a_travers_l_eau` à très grande distance, la pente de la ligne bornée à l'horizontale.
    COLOR = sous_eau ? a_travers_l_eau(vec3(0.0), 1e6, vec3(dir.xy, min(dir.z, 0.0)), profondeur_camera) : ciel_b(dir, 4);
}
