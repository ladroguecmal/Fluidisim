// S380 — LES GOUTTES DANS L'AIR (ADR-205, pièce 1) : chaque goutte d'une classe de taille, placée, dimensionnée et
// animée **par hachage de son indice** — aucune émission, aucun état : une boîte fixe dans le monde, repliée autour d'un
// centre qui suit la caméra (chaque goutte garde sa place dans le monde, la boîte ne fait que la ramener quand elle en
// sort). Le nombre de gouttes est celui de Marshall et Palmer dans le volume de la boîte (`pluie_air.gd`) ; le diamètre
// est tiré de leur loi exponentielle tronquée à la classe ; la vitesse est celle d'Atlas. La goutte s'arrête au sol et à
// l'eau (`plancher`, `nappes`). La traînée est le trajet pendant `PLUIE_TAU` (Garg et Nayar 2007), sa largeur au moins un
// pixel ; son opacité, calculée ici, passe au dessin (`goutte.gdshader`) par CUSTOM.
shader_type particles;
render_mode disable_force, disable_velocity;

// Le centre de la boîte (monde de Godot), ses demi-côtés horizontaux et sa hauteur (x, z, y), la caméra.
uniform vec3 centre = vec3(0.0);
uniform vec3 boite = vec3(4.0, 6.0, 4.0);
uniform vec3 camera = vec3(0.0);
// Loi des tailles : Λ (mm⁻¹) et la classe [d0, d1) (mm).
uniform float lambda_mp = 2.5;
uniform float d0 = 1.0;
uniform float d1 = 2.0;
uniform float temps = 0.0;
// L'angle d'un pixel (rad) : la largeur minimale de la traînée, et l'opacité de Garg et Nayar.
uniform float angle_pixel = 0.0012;
// Sous `plancher`, rien ; dans chaque rectangle de `nappes` (x0, z0, x1, z1), rien sous `niveaux[i]` (l'eau des bacs).
uniform float plancher = 0.0;
uniform vec4 nappes[4];
uniform float niveaux[4];
uniform int n_nappes = 0;
// Contrôle (critère 2 de S380) : seules les gouttes de la tranche [y0, y1] ; largeur de deux pixels, opacité pleine.
uniform bool controle = false;
uniform vec2 tranche = vec2(0.0, 0.5);

const float PLUIE_TAU = 1.0 / 60.0;

uvec4 pcg4d(uvec4 v) {
    v = v * 1664525u + 1013904223u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    v ^= v >> 16u;
    v.x += v.y * v.w; v.y += v.z * v.x; v.z += v.x * v.y; v.w += v.y * v.z;
    return v;
}

float u01(uint h) {
    return (float(h >> 8u) + 0.5) * (1.0 / 16777216.0);
}

void process() {
    uvec4 h = pcg4d(uvec4(INDEX, uint(d0 * 1000.0), 0x7a3b19c5u, 0x2f6e2b1du));
    // Le diamètre : exponentielle de paramètre Λ tronquée à [d0, d1), par inversion.
    float masse = 1.0 - exp(-lambda_mp * (d1 - d0));
    float d = d0 - log(1.0 - u01(h.w) * masse) / lambda_mp;
    float v = 9.65 - 10.3 * exp(-0.6 * d);
    vec3 taille = vec3(2.0 * boite.x, boite.y, 2.0 * boite.z);
    vec3 p = vec3(u01(h.x), u01(h.y), u01(h.z)) * taille;
    p.y -= v * temps;
    // Le repli : la copie de la goutte la plus proche du centre, par période de la boîte.
    p += taille * round((centre - p) / taille);
    float sol = plancher;
    for (int i = 0; i < n_nappes; i++) {
        vec4 r = nappes[i];
        if (p.x > r.x && p.x < r.z && p.z > r.y && p.z < r.w) {
            sol = max(sol, niveaux[i]);
        }
    }
    float distance = length(camera - p);
    float s = max(distance * angle_pixel, 1e-5);
    float longueur = v * PLUIE_TAU + d * 1e-3;
    float largeur = max(d * 1e-3, s);
    // Garg et Nayar : part du pixel couverte en largeur × part du temps d'exposition où la goutte y passe.
    float opacite = min(1.0, d * 1e-3 / s) * min(1.0, (d * 1e-3 + s) / (v * PLUIE_TAU));
    bool visible = p.y > sol;
    if (controle) {
        visible = visible && p.y >= tranche.x && p.y < tranche.y;
        longueur = 2.0 * s;
        largeur = 2.0 * s;
        opacite = 1.0;
    }
    vec3 vers_camera = camera - p;
    vec3 face = normalize(vec3(vers_camera.x, 0.0, vers_camera.z) + vec3(1e-6, 0.0, 0.0));
    vec3 haut = vec3(0.0, 1.0, 0.0);
    if (controle) {
        // Vue d'aplomb : le carré posé à plat, face au ciel.
        face = vec3(0.0, 1.0, 0.0);
        haut = vec3(0.0, 0.0, -1.0);
    }
    vec3 droite = normalize(cross(haut, face));
    // La tête de la goutte est en `p` ; la traînée monte au-dessus d'elle (le trajet déjà fait pendant l'exposition).
    TRANSFORM = mat4(vec4(droite * largeur, 0.0), vec4(haut * longueur, 0.0), vec4(face, 0.0),
        vec4(p + (controle ? vec3(0.0) : 0.5 * longueur * haut), 1.0));
    if (!visible) {
        // Une goutte sous le sol ou l'eau reste vivante (elle reviendra en haut de la boîte) mais sans surface.
        TRANSFORM[0] = vec4(0.0);
        TRANSFORM[1] = vec4(0.0);
    }
    CUSTOM = vec4(d, v, opacite, 0.0);
    ACTIVE = true;
}
