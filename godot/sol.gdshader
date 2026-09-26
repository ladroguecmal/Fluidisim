// Le fond de la scène côtière — le corps du nuanceur est `sol.gdshaderinc` (S373).
// Cette variante laisse la brume au moteur (Godot, `fog_process`) : caméra hors de la zone des vagues.
shader_type spatial;
render_mode unshaded;

#include "res://sol.gdshaderinc"
