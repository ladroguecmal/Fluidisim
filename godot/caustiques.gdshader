// S361 — LA CARTE DE CAUSTIQUES, MÉTHODE DIRECTE (Wyman, « Interactive image-space techniques for approximating
// caustics », 2006 ; le principe est l'optique géométrique de la conservation de l'énergie).
//
// Une grille dense de la surface — bande analytique et cascade de 32 m de la surface fine — ; chaque sommet `X_s` est
// envoyé au point du fond où arrive son rayon de soleil réfracté, `X_f = X_s + (H(X_f) + η)·p(∇η)` (Snell exact, deux
// passes pour la profondeur au point d'arrivée) ; la vue est orthographique sur le fond, `cote_carte` mètres autour de
// `origine`. Chaque triangle y dépose, en mélange **additif**, le rapport de son aire de surface à son aire au fond :
// `|det ∂X_s/∂écran| / (aire d'un texel)`. Les replis — plusieurs points de surface pour un point du fond — s'ajoutent
// d'eux-mêmes, et l'énergie est conservée par construction : là où toute la surface qui y tombe est dessinée, la carte
// vaut 1 en moyenne. La méthode à rebours de S361 P3 ne suivait qu'un antécédent (énergie 2 à 5,6 sur la scène).
shader_type spatial;
render_mode unshaded, blend_add, cull_disabled, depth_draw_never, depth_test_disabled, fog_disabled, shadows_disabled;

#include "res://ciel.gdshaderinc"

uniform vec4 bande[64];
uniform int n_bande = 0;
uniform sampler2D detail_a0 : filter_linear, repeat_enable;
uniform sampler2D detail_c0 : filter_linear, repeat_enable;
uniform float detail_cote0 = 32.0;
uniform bool caustiques_detail = true;
uniform float indice = 1.34;
// La carte : son centre (B) et son côté, m ; sa résolution, texels.
uniform vec2 origine = vec2(0.0);
uniform float cote_carte = 64.0;
uniform float texels = 1024.0;
// La bathymétrie de `mer.gd::profondeur`, échantillonnée sur la grille même du fond (`fond_origine`, `fond_pas`).
uniform sampler2D bathymetrie : filter_linear, repeat_disable;
uniform vec2 fond_origine = vec2(-1200.0, -200.0);
uniform vec2 fond_taille = vec2(2400.0, 1100.0);
// La carte est écrite divisée par `carte_echelle` (le fond la multiplie), pour tenir dans son format.
uniform float carte_echelle = 32.0;
// Contrôle : profondeur imposée (> 0), soleil imposé (non nul).
uniform float profondeur_controle = 0.0;
uniform vec3 soleil_controle = vec3(0.0);

varying vec2 source;

float profondeur(vec2 q) {
    if (profondeur_controle > 0.0) { return profondeur_controle; }
    vec2 uv = (q - fond_origine) / fond_taille;
    return textureLod(bathymetrie, uv, 0.0).r;
}

vec2 rayon(vec2 s, vec3 soleil) {
    vec3 n = normalize(vec3(-s, 1.0));
    vec3 r = refract(-soleil, n, 1.0 / indice);
    return r.xy / max(-r.z, 1e-4);
}

void vertex() {
    vec3 monde = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
    vec2 xs = vec2(monde.x, -monde.z);
    vec2 s = vec2(0.0);
    float eta = 0.0;
    for (int i = 0; i < n_bande; i++) {
        vec4 c = bande[i];
        float phase = dot(c.yz, xs) + c.w;
        eta += c.x * sin(phase);
        s += c.x * cos(phase) * c.yz;
    }
    if (caustiques_detail) {
        vec2 uv = xs / detail_cote0;
        s += textureLod(detail_a0, uv, 0.0).xy;
        eta += textureLod(detail_c0, uv, 0.0).w;
    }
    vec3 soleil = dot(soleil_controle, soleil_controle) > 0.0 ? normalize(soleil_controle) : normalize(SOLEIL_B);
    vec2 p = rayon(s, soleil);
    vec2 xf = xs + (profondeur(xs) + eta) * p;
    xf = xs + (profondeur(xf) + eta) * p;
    xf = xs + (profondeur(xf) + eta) * p;
    source = xs;
    vec2 uv = (xf - origine) / cote_carte;
    POSITION = vec4(2.0 * uv.x, 2.0 * uv.y, 0.5, 1.0);
}

void fragment() {
    vec2 dx = dFdx(source);
    vec2 dy = dFdy(source);
    float aire_surface = abs(dx.x * dy.y - dx.y * dy.x);
    float texel = cote_carte / texels;
    ALBEDO = vec3(aire_surface / (texel * texel) / carte_echelle);
}
