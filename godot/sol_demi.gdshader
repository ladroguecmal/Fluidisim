// Le fond de la scène côtière, variante de la caméra à demi immergée (S373) : la brume est la nôtre (`brume_air`), réglée par pixel
// sur la part d'air — la brume du moteur ne se règle pas par pixel (S371 P2). Le corps : `sol.gdshaderinc`.
shader_type spatial;
render_mode unshaded;

#define BRUME_PAR_PIXEL
#include "res://sol.gdshaderinc"
