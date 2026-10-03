// S461 — C11 : LA MER DE B AU-DELÀ DU DOMAINE SIMULÉ — la surface analytique `h + a·cos(k·x − ω·t + φ)`, rencontrée par Newton
// depuis le plan moyen, comme `surface_carte.wgsl` (S457) ; dans l'empreinte du domaine, rien (l'eau simulée y est). Sa pente est
// éteinte quand une longueur d'onde tient dans quelques pixels. Dessinée sur un grand plan au niveau moyen.
shader_type spatial;
render_mode unshaded, cull_disabled, depth_draw_always;

#include "res://saut_optique.gdshaderinc"

// L'empreinte angulaire d'un pixel, `2·tan(fov/2)/hauteur`.
uniform float angle_pixel = 0.0013;
// La boîte marchée par `saut_eau.gdshader` : des centres extrêmes des mailles, `[debut, etendue]` (sans bande morte, S457).
uniform float debut = 0.025;

void fragment() {
    vec3 o = godot_vers_b(CAMERA_POSITION_WORLD);
    vec3 d = normalize(godot_vers_b((INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz) - o);
    if (d.z >= -1e-4) {
        discard;
    }
    float t = (niveau - o.z) / d.z;
    for (int i = 0; i < 6; i++) {
        vec2 hb = hauteur_b(o.x + t * d.x);
        t -= (o.z + t * d.z - hb.x) / (d.z - hb.y * d.x);
    }
    vec3 p = o + d * t;
    if (t <= 0.0 || (p.x > debut && p.x < etendue.x && p.y > debut && p.y < etendue.y)) {
        discard;
    }
    float resolue = exp(-pow(t * angle_pixel * houle.y * 0.5, 2.0));
    vec2 hb = hauteur_b(p.x);
    ALBEDO = couleur_eau(p, vec3(-hb.y * resolue, 0.0, 1.0), d);
    DEPTH = profondeur_ecran(p, VIEW_MATRIX, PROJECTION_MATRIX);
}
