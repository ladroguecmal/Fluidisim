// S383 — LA BANDE DE B, seule : ses composantes et leur somme en un point de Lagrange. Sortie de `surface_b.gdshaderinc`
// (S371), qui l'inclut, pour que les gerbes de la pluie (`gerbe.gdshader`, un nuanceur de particules) se posent sur la
// même surface que la mer, sans rien lire d'autre. Une seule source.

// Les composantes de la bande `[a, kx, ky, φ(t)]` (phases repliées par `mer.gd` en double précision, I-08) et le second
// ordre de Tayfun par système (ADR-176 D1) — déclarés dans `eau.gdshader` jusqu'à S370.
uniform vec4 bande[64];
uniform int n_bande = 0;
uniform int split = 64;
uniform vec2 k_moyens = vec2(0.0);

/// La bande au point lagrangien `q` (B), à empreinte nulle — toutes les composantes entières, comme `vertex()` d'`eau.gdshader`
/// près de l'œil : le déplacement horizontal `d`, son gradient `g` = (g_xx, g_xy, g_yy), la hauteur `eta` avec le second
/// ordre, et sa pente lagrangienne `s`.
void bande_au_point(vec2 q, out vec2 d, out vec3 g, out float eta, out vec2 s) {
    d = vec2(0.0);
    g = vec3(0.0);
    eta = 0.0;
    s = vec2(0.0);
    vec2 e2 = vec2(0.0), qd = vec2(0.0);
    vec4 sa = vec4(0.0), sq = vec4(0.0);
    for (int i = 0; i < n_bande; i++) {
        vec4 c = bande[i];
        float k = length(c.yz);
        if (k == 0.0) { continue; }
        float a = c.x;
        float phase = dot(c.yz, q) + c.w;
        vec2 u = c.yz / k;
        float cs = cos(phase);
        float sn = sin(phase);
        d += a * cs * u;
        eta += a * sn;
        s += a * cs * c.yz;
        g -= a * k * sn * vec3(u.x * u.x, u.x * u.y, u.y * u.y);
        if (i < split) {
            e2.x += a * sn; qd.x += a * cs;
            sa.xy += a * cs * c.yz; sq.xy -= a * sn * c.yz;
        } else {
            e2.y += a * sn; qd.y += a * cs;
            sa.zw += a * cs * c.yz; sq.zw -= a * sn * c.yz;
        }
    }
    if (k_moyens.x > 0.0) {
        eta += 0.5 * k_moyens.x * (e2.x * e2.x - qd.x * qd.x) + 0.5 * k_moyens.y * (e2.y * e2.y - qd.y * qd.y);
        s += k_moyens.x * (e2.x * sa.xy - qd.x * sq.xy) + k_moyens.y * (e2.y * sa.zw - qd.y * sq.zw);
    }
}
