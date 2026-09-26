// L'eau de la mer de b, variante de la caméra à demi immergée (S373) : la brume est la nôtre (`brume_air`), réglée par pixel
// sur la part d'air — la brume du moteur ne se règle pas par pixel (S371 P2). Le corps : `eau.gdshaderinc`.
shader_type spatial;
render_mode cull_disabled, unshaded;

#define BRUME_PAR_PIXEL
#include "res://eau.gdshaderinc"
