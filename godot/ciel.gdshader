// S359 — le ciel de la scène : le ciel clair de l'afficheur (`ciel.gdshaderinc`), celui que l'eau reflète.
shader_type sky;

#include "res://ciel.gdshaderinc"

void sky() {
    COLOR = ciel_b(vec3(EYEDIR.x, -EYEDIR.z, EYEDIR.y), 4);
}
