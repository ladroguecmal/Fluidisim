// S467 — LE JOUEUR (`saut.gd`), éclairé comme le corps vu à travers l'eau (`couleur_corps`, `saut_optique.gdshaderinc`) et comme
// les parois de la piscine (S374) : `albédo · E/π · eclairage(n)`, et non par Godot — au-dessus de la surface, le maillage ; au-dessous,
// le lancer de rayons de l'eau : les deux dans les mêmes unités, de la même formule, du même albédo (`ALBEDO_JOUEUR` de `saut.gd`).
// Sous le ciel couvert, `eclairage` suit (`couvert`, `ciel.gdshaderinc`).
shader_type spatial;
render_mode unshaded;

#include "res://ciel.gdshaderinc"

uniform vec3 albedo = vec3(0.22, 0.22, 0.24);

void fragment() {
    vec3 n = normalize((INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
    // Godot (y en haut) vers B (z en haut) : l'inverse de `b_vers_godot`.
    vec3 nb = vec3(n.x, -n.z, n.y);
    ALBEDO = albedo * gain_eau * eclairage(nb, normalize(SOLEIL_B), 1.0);
}
