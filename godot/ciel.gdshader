// S359 — le ciel de la scène : le ciel clair de l'afficheur (`ciel.gdshaderinc`), celui que l'eau reflète.
shader_type sky;

#include "res://ciel.gdshaderinc"
// S365 : l'œil dans l'eau ne voit pas le ciel au-delà de ce que la scène couvre, mais l'eau elle-même, sur une ligne
// infinie (`optique_eau.gdshaderinc`).
#include "res://optique_eau.gdshaderinc"
// S371 : la bande de B et le milieu du pixel — la caméra à demi immergée (`surface_b.gdshaderinc`).
#include "res://surface_b.gdshaderinc"

void sky() {
    vec3 dir = vec3(EYEDIR.x, -EYEDIR.z, EYEDIR.y);
    // S371 : le milieu du pixel, décidé à l'objectif (`surface_b.gdshaderinc`) ; la position de la caméra est un uniforme —
    // lire `POSITION` ferait passer le ciel en mise à jour continue, et changerait le cube de radiance. Le cube lui-même
    // garde le milieu de l'œil.
    vec3 milieu = AT_CUBEMAP_PASS ? vec3(sous_eau ? 1.0 : 0.0, profondeur_camera, 0.0) : milieu_du_pixel(camera_position, EYEDIR);
    vec3 dessous = vec3(0.0);
    vec3 dessus = vec3(0.0);
    // Ligne infinie : `a_travers_l_eau` à très grande distance, la pente de la ligne bornée à l'horizontale.
    if (milieu.x > 0.0) {
        dessous = a_travers_l_eau(vec3(0.0), 1e6, vec3(dir.xy, min(dir.z, 0.0)), milieu.y);
    }
    if (milieu.x < 1.0) {
        dessus = ciel_b(dir, 4);
    }
    COLOR = milieu.x >= 1.0 ? dessous : (milieu.x <= 0.0 ? dessus : mix(dessus, dessous, milieu.x));
}
