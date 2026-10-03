// S461 — C11 : L'EAU DU DOMAINE SIMULÉ, rejouée. Le champ fondu `φ` de l'afficheur (`--v1-banc EXPORT_GODOT`, 8 bits, ±2 mailles,
// la fenêtre `k0..k1`) dans une texture 3D ; un rayon par pixel, comme `surface_carte.wgsl` : le premier changement de signe par
// pas d'une demi-maille, huit bissections, la normale au gradient, l'optique de `saut_optique.gdshaderinc`. Dessiné sur les faces
// arrière d'une boîte (le rayon part de l'œil, dedans ou dehors) ; la profondeur écrite au point touché.
shader_type spatial;
render_mode unshaded, cull_front, depth_draw_always;

#include "res://saut_optique.gdshaderinc"

uniform sampler3D phi_tex : filter_linear, repeat_disable;
// nx, ny, nz ; k0, k1 ; dx.
uniform vec3 dims = vec3(80.0, 80.0, 58.0);
uniform vec2 fenetre = vec2(11.0, 48.0);
uniform float dx = 0.05;
// L'empreinte angulaire d'un pixel, `2·tan(fov/2)/hauteur`.
uniform float angle_pixel = 0.0013;

float phi(vec3 p) {
    float zk = p.z / dx;
    if (zk < fenetre.x + 0.5) {
        return -2.0 * dx;
    }
    if (zk > fenetre.y - 0.5) {
        return 2.0 * dx;
    }
    vec3 uvw = vec3(p.x / (dims.x * dx), p.y / (dims.y * dx), (zk - fenetre.x) / (fenetre.y - fenetre.x));
    return (texture(phi_tex, uvw).r - 0.5) * 4.0 * dx;
}

vec3 gradient(vec3 p) {
    float h = 0.5 * dx;
    return vec3(phi(p + vec3(h, 0.0, 0.0)) - phi(p - vec3(h, 0.0, 0.0)),
            phi(p + vec3(0.0, h, 0.0)) - phi(p - vec3(0.0, h, 0.0)),
            phi(p + vec3(0.0, 0.0, h)) - phi(p - vec3(0.0, 0.0, h)));
}

void fragment() {
    vec3 o = godot_vers_b(CAMERA_POSITION_WORLD);
    vec3 cible = godot_vers_b((INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz);
    vec3 d = normalize(cible - o);
    vec3 hi = vec3((dims.x - 0.5) * dx, (dims.y - 0.5) * dx, (fenetre.y - 0.5) * dx);
    vec3 lo = vec3(0.5 * dx, 0.5 * dx, (fenetre.x + 0.5) * dx);
    vec3 inv = 1.0 / d;
    vec3 t0 = (lo - o) * inv;
    vec3 t1 = (hi - o) * inv;
    float entree = max(max(min(t0.x, t1.x), min(t0.y, t1.y)), max(min(t0.z, t1.z), 0.0));
    float t_corps = touche_corps(o, d);
    float sortie = min(min(max(t0.x, t1.x), max(t0.y, t1.y)), min(max(t0.z, t1.z), t_corps));
    bool touche = false;
    vec3 p = vec3(0.0);
    if (entree < sortie) {
        float pas = 0.5 * dx;
        float ta = entree;
        float fa = phi(o + d * ta);
        for (int garde = 0; garde < 4096; garde++) {
            float tb = min(ta + pas, sortie);
            float fb = phi(o + d * tb);
            if ((fa < 0.0) != (fb < 0.0)) {
                float a = ta;
                float b = tb;
                for (int q = 0; q < 8; q++) {
                    float m = 0.5 * (a + b);
                    if ((phi(o + d * m) < 0.0) == (fa < 0.0)) {
                        a = m;
                    } else {
                        b = m;
                    }
                }
                p = o + d * (0.5 * (a + b));
                touche = true;
                break;
            }
            if (tb >= sortie) {
                break;
            }
            ta = tb;
            fa = fb;
        }
    }
    if (!touche) {
        discard;
    }
    ALBEDO = couleur_eau(p, gradient(p), d, length(p - o) * angle_pixel);
    DEPTH = profondeur_ecran(p, VIEW_MATRIX, PROJECTION_MATRIX);
}
