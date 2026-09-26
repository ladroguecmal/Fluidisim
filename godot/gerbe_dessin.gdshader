// S383 — LE DESSIN D'UNE GERBE (ADR-205, pièce 4), placée par `gerbe.gdshader`. La silhouette suit la gerbe mesurée par
// Murphy et al. (2015) — goutte de 4,1 mm à 7,2 m/s ; relevée sur la figure 3 de Wang et al. (2023), S383 P2 — ramenée à
// la goutte par l'échelle `s` (longueurs) et `√s` (temps) :
// - jusqu'à 10 ms, la **couronne** : une coupe de hauteur `H(t)`, large de `W(t)` à sa base, un peu moins en haut ; ses
//   parois, vues par la tranche, plus opaques que ses faces ;
// - ensuite le **dôme** (la couronne refermée, `B(t)` de haut) et le **jet** central jusqu'à `H(t)`, sa tête ronde ;
// - elle retombe entre 52 et 80 ms (au-delà des mesures : hypothèse dite).
// La caméra pose 1/60 s : la silhouette est moyennée sur quatre instants de la pose (la couronne, qui ne dure que 10 ms, ne
// se voit que floue, comme sur une photographie). Radiance : celle de l'eau d'une goutte (`goutte.gdshader`, Garg et Nayar :
// ≈ 0,94 de la moyenne de l'environnement) ; opacités des parois, du dôme et du jet **réglées, pas mesurées**. Gerbe plus
// petite que deux pixels : dessinée agrandie, l'opacité réduite de l'aire (`1/agrandi²`).
shader_type spatial;
render_mode unshaded, blend_mix, depth_draw_never, cull_disabled, shadows_disabled, fog_disabled;

#include "res://ciel.gdshaderinc"

uniform bool controle = false;
varying vec4 donnees;

const float T_G[9] = {0.0, 1.0, 3.0, 7.0, 12.0, 18.0, 41.0, 52.0, 80.0};
const float H_G[9] = {0.0, 5.9, 11.2, 17.1, 19.4, 24.7, 23.5, 21.8, 0.0};
const float W_G[9] = {4.1, 14.0, 16.0, 21.0, 25.0, 26.0, 29.0, 29.0, 20.0};
const float B_G[9] = {0.0, 5.9, 11.2, 17.1, 11.8, 8.8, 7.6, 8.2, 0.0};
const float POSE_MS = 1000.0 / 60.0;

void vertex() {
    donnees = INSTANCE_CUSTOM;
}

/// Une grandeur de la gerbe de référence (mm) au temps `t` (ms), interpolée entre les relevés.
float releve(float t, int quoi) {
    float v0 = 0.0;
    for (int i = 0; i < 8; i++) {
        if (t >= T_G[i] && t < T_G[i + 1]) {
            float f = (t - T_G[i]) / (T_G[i + 1] - T_G[i]);
            float a = quoi == 0 ? H_G[i] : (quoi == 1 ? W_G[i] : B_G[i]);
            float b = quoi == 0 ? H_G[i + 1] : (quoi == 1 ? W_G[i + 1] : B_G[i + 1]);
            v0 = mix(a, b, f);
        }
    }
    return v0;
}

/// L'opacité de la gerbe de référence au point (x, y) mm (x horizontal depuis l'axe, y au-dessus de l'eau), au temps `t` ms.
float silhouette(float x, float y, float t) {
    if (t <= 0.0 || t >= T_G[8] || y < 0.0) {
        return 0.0;
    }
    float h = releve(t, 0);
    float w = 0.5 * releve(t, 1);
    float ax = abs(x);
    if (t < 10.0) {
        if (y > h) {
            return 0.0;
        }
        float r = w * (1.0 - 0.25 * y / max(h, 1e-3));
        if (ax > r) {
            return 0.0;
        }
        float bord = ax > r - 0.15 * w || y > h - 1.0 ? 0.6 : 0.25;
        return bord;
    }
    float b = releve(t, 2);
    float a = 0.0;
    float e = (x / w) * (x / w) + (y / max(b, 1e-3)) * (y / max(b, 1e-3));
    if (e <= 1.0) {
        a = e > 0.75 ? 0.55 : 0.3;
    }
    if (ax <= 2.0 && y >= 0.8 * b && y <= h - 2.5) {
        a = 0.8;
    }
    vec2 tete = vec2(x, y - (h - 2.5));
    if (dot(tete, tete) <= 2.5 * 2.5 && h > 2.5) {
        a = 0.8;
    }
    return a;
}

/// Une direction de Godot (x, y en haut, z au sud) en coordonnées de B pour le ciel.
vec3 vers_b(vec3 d) {
    return vec3(d.x, -d.z, d.y);
}

void fragment() {
    if (controle) {
        ALBEDO = vec3(0.0, 1.0, 0.0);
        ALPHA = 1.0;
    } else {
        float s = max(donnees.z, 1e-3);
        float agrandi = max(donnees.w, 1.0);
        // Le carré couvre la gerbe de référence : x ∈ [−14,5 ; 14,5] mm, y ∈ [0 ; 24,7] mm.
        float x = (UV.x - 0.5) * 29.0;
        float y = (1.0 - UV.y) * 24.7;
        float t = 1000.0 * donnees.x / sqrt(s);
        float pose = POSE_MS / sqrt(s);
        float a = 0.0;
        for (int i = 0; i < 4; i++) {
            a += 0.25 * silhouette(x, y, t - (float(i) + 0.5) * 0.25 * pose);
        }
        if (a <= 0.0) {
            discard;
        }
        vec3 monde = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
        vec3 vue = normalize(monde - CAMERA_POSITION_WORLD);
        vec3 horizontale = normalize(vec3(vue.x, 0.0, vue.z) + vec3(1e-6, 0.0, 0.0));
        vec3 haut = vec3(0.0, 1.0, 0.0);
        vec3 ciel = (ciel_b(vers_b(normalize(horizontale + 0.577 * haut)), 2)
            + ciel_b(vers_b(normalize(horizontale + 1.732 * haut)), 2) + ciel_b(vec3(0.0, 0.0, 1.0), 2)) / 3.0;
        vec3 sol = 0.2 * ciel_b(vec3(0.0, 0.0, 1.0), 2);
        ALBEDO = 0.94 * 0.5 * (ciel + sol);
        ALPHA = clamp(a / (agrandi * agrandi), 0.0, 1.0);
    }
}
