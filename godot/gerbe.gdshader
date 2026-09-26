// S383 — LES GERBES DE LA PLUIE (ADR-205, pièce 4) : à chaque impact qui laisse un anneau, **le même** que celui des rides
// — le tirage de `pluie.gdshaderinc` (couche, période, maille → instant, centre, diamètre) —, une gerbe au-dessus de l'eau.
// Une particule par couche `j`, par période (la courante et la précédente, `q`) et par maille d'une fenêtre qui couvre les
// eaux ; rien n'est émis ni gardé : l'indice dit tout. La particule vit tant que la gerbe dure (`GERBE_VIE`, à l'échelle
// de la goutte) et que l'impact tombe dans une des eaux (`nappes`, `niveaux`) ; elle place un carré tourné vers la caméra,
// base au niveau de l'eau, à la taille de la gerbe ; le dessin (`gerbe_dessin.gdshader`) en fait la silhouette.
//
// L'échelle (S383, P2) : la gerbe mesurée par Murphy et al. (2015 ; Wang et al. 2023), goutte de 4,1 mm à 7,2 m/s, Fr 1 322,
// portée en longueur par `s = (D/4,1 mm)·(Fr/1 322)^0,26` (exposants de Watson et al. 2024), en temps par `√s` (hypothèse
// dite : effondrement gouverné par la gravité), `Fr = v²/(g·D)`, `v` d'Atlas.
shader_type particles;
render_mode disable_force, disable_velocity;

#include "res://pluie.gdshaderinc"
// S383 P4b : la mer — la bande de B (`bande`, `n_bande`, `split`, `k_moyens`) et `bande_au_point`.
#include "res://bande_b.gdshaderinc"

uniform float temps = 0.0;
uniform vec3 camera = vec3(0.0);
uniform float angle_pixel = 0.0012;
// La fenêtre (x0, z0, x1, z1, monde de Godot) et son nombre de mailles par axe.
uniform vec4 fenetre = vec4(0.0);
uniform ivec2 n_mailles = ivec2(1, 1);
uniform vec4 nappes[4];
uniform float niveaux[4];
uniform int n_nappes = 0;
// S383 P4b — **sur la mer** : la fenêtre est dans le plan de B, en coordonnées de Lagrange (celles des rides de la mer) ; la
// gerbe se pose au point déplacé `q + d(q)`, à la hauteur `η(q)` de la bande (`bande_b.gdshaderinc`).
uniform bool sur_la_mer = false;
// Contrôle (critères 3 et 4) : un carré de `taille_controle` posé à plat au centre de chaque gerbe de moins de
// `vie_controle` secondes.
uniform bool controle = false;
uniform float taille_controle = 0.02;
uniform float vie_controle = 0.05;

// La gerbe de référence (s = 1) : hauteur et largeur maximales (m), durée (s).
const float GERBE_H = 0.0247;
const float GERBE_L = 0.029;
const float GERBE_VIE = 0.08;
const float PLUIE_POSE = 1.0 / 60.0;

float gerbe_echelle(float d_mm) {
    float v = 9.65 - 10.3 * exp(-0.6 * d_mm);
    float fr = v * v / (9.81 * d_mm * 1e-3);
    return (d_mm / 4.1) * pow(fr / 1322.0, 0.26);
}

/// La gerbe de la particule `index` : faux s'il n'y en a pas maintenant ; sinon sa transformation et ses données.
bool gerbe_placer(uint index, out mat4 xf, out vec4 donnees) {
    xf = mat4(vec4(0.0), vec4(0.0), vec4(0.0, 0.0, 1.0, 0.0), vec4(0.0, -1000.0, 0.0, 1.0));
    donnees = vec4(0.0);
    if (pluie.x <= 0.0) {
        return false;
    }
    int nx = n_mailles.x;
    int nz = n_mailles.y;
    int i = int(index);
    int ix = i % nx;
    int iz = (i / nx) % nz;
    int q = (i / (nx * nz)) % 2;
    int j = i / (nx * nz * 2);
    float couches = pluie.x * PLUIE_MAILLE * PLUIE_MAILLE * PLUIE_VIE;
    if (float(j) >= couches) {
        return false;
    }
    float presence = clamp(couches - float(j), 0.0, 1.0);
    float phase = pluie_phase(j);
    int k = int(floor((temps + phase) / PLUIE_VIE)) - q;
    uint hk = pluie_graine(j, k);
    vec2 decal = pluie_decalage(hk);
    vec2 m = floor((fenetre.xy + decal) / PLUIE_MAILLE) + vec2(float(ix), float(iz));
    float naissance;
    vec2 centre;
    float taille;
    if (!pluie_impact(j, k, m, hk, decal, presence, phase, naissance, centre, taille)) {
        return false;
    }
    float age = temps - naissance;
    float s = gerbe_echelle(taille);
    float vie = controle ? vie_controle : GERBE_VIE * sqrt(s);
    // La pose de la caméra : une gerbe née pendant la dernière pose se voit déjà ; morte avant, plus.
    if (age < 0.0 || age >= vie + (controle ? 0.0 : PLUIE_POSE)) {
        return false;
    }
    vec3 base;
    if (sur_la_mer) {
        vec2 d;
        vec3 g;
        float eta;
        vec2 pente;
        bande_au_point(centre, d, g, eta, pente);
        base = vec3(centre.x + d.x, eta, -(centre.y + d.y));
    } else {
        // Sur quelle eau, à quel niveau ; hors de l'eau, pas de gerbe (le sol : P7).
        float niveau = -1e9;
        for (int n = 0; n < n_nappes; n++) {
            vec4 r = nappes[n];
            if (centre.x > r.x && centre.x < r.z && centre.y > r.y && centre.y < r.w) {
                niveau = niveaux[n];
            }
        }
        if (niveau < -1e8) {
            return false;
        }
        base = vec3(centre.x, niveau, centre.y);
    }
    if (controle) {
        xf = mat4(vec4(taille_controle, 0.0, 0.0, 0.0), vec4(0.0, 0.0, -taille_controle, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0), vec4(base + vec3(0.0, 0.002, 0.0), 1.0));
        donnees = vec4(age, taille, s, 1.0);
        return true;
    }
    // Le carré : la gerbe entière (largeur et hauteur maximales à son échelle), agrandi à deux pixels au moins — le dessin
    // réduit alors son opacité de la part du pixel qu'elle couvre vraiment.
    float largeur = GERBE_L * s;
    float hauteur = GERBE_H * s;
    float pixel = max(length(camera - base) * angle_pixel, 1e-5);
    float agrandi = max(1.0, 2.0 * pixel / hauteur);
    vec3 vers_camera = camera - base;
    vec3 face = normalize(vec3(vers_camera.x, 0.0, vers_camera.z) + vec3(1e-6, 0.0, 0.0));
    vec3 haut = vec3(0.0, 1.0, 0.0);
    vec3 droite = normalize(cross(haut, face));
    xf = mat4(vec4(droite * largeur * agrandi, 0.0), vec4(haut * hauteur * agrandi, 0.0), vec4(face, 0.0),
        vec4(base + 0.5 * hauteur * agrandi * haut, 1.0));
    // x : âge (s) ; y : diamètre (mm) ; z : échelle s ; w : l'agrandissement (le dessin en tire la part couverte).
    donnees = vec4(age, taille, s, agrandi);
    return true;
}

void process() {
    mat4 xf;
    vec4 donnees;
    gerbe_placer(INDEX, xf, donnees);
    TRANSFORM = xf;
    CUSTOM = donnees;
    ACTIVE = true;
}
