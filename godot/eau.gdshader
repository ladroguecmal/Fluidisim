// L'eau de la mer de b — le corps du nuanceur est `eau.gdshaderinc` (S373).
// Cette variante laisse la brume au moteur (Godot, `fog_process`) : caméra hors de la zone des vagues.
shader_type spatial;
render_mode cull_disabled, unshaded;

#include "res://eau.gdshaderinc"
