// S374 — L'EAU D'UN BAC DE V (la piscine, `piscine.gd`) : une surface plane à la cote que V publie, vue d'au-dessus.
//
// L'optique est celle de la mer (`eau.gdshaderinc`, S359) ramenée au bac : Fresnel exact, le ciel de la scène réfléchi
// (`ciel.gdshaderinc`), l'éclat du soleil ; le fond et les parois vus par réfraction de Snell en espace écran, la
// colonne d'eau pure selon Maritorena (`optique_eau.gdshaderinc` : R0, Kd, E/π). La surface n'a pas de vagues de B :
// une agitation d'habillage — quelques trains de rides — dont l'amplitude monte là où l'eau tombe (le jet de la pompe,
// la lame du déversoir). Aucune de ces rides n'est calculée par V ; seule la cote l'est.
//
// S379 — **les rides de la pluie** (`pluie.gdshaderinc`) s'ajoutent à la pente, sur la surface plane comme sur celle de δ
// (ADR-203 : les impacts restent factices, même près du joueur) ; leur pente non résolue est intégrée comme la mer le fait
// de la sienne (ADR-161) : le ciel par 3 × 3 nœuds de Gauss-Hermite, l'éclat du soleil élargi en forme close. Temps sec :
// ni l'une ni l'autre, l'image d'avant au bit.
shader_type spatial;
render_mode cull_disabled, unshaded;

#include "res://ciel.gdshaderinc"
#include "res://optique_eau.gdshaderinc"
#include "res://pluie.gdshaderinc"

uniform sampler2D ecran : hint_screen_texture, filter_linear_mipmap;
uniform sampler2D profondeur_ecran : hint_depth_texture, filter_nearest;
// Agitation de fond (pente quadratique moyenne des rides, sans dimension) et foyer d'agitation (un point du monde de
// Godot, x et z ; rayon, m ; pente ajoutée au centre) — l'impact du jet, la chute de la lame.
uniform float agitation = 0.01;
uniform vec3 foyer = vec3(0.0);
uniform float foyer_rayon = 0.5;
uniform float foyer_pente = 0.0;
uniform float temps = 0.0;
// S375 — **la surface de δ 3D** (ADR-200) : la hauteur de chaque colonne, au-dessus du repos, dans deux images (celle
// d'avant et celle d'après, lues au plus proche aux centres des colonnes, UV du maillage), mélangées par `melange` ; la
// pente, par différences centrées. Sans elle, la surface plane de V et ses rides d'habillage (S374).
uniform bool champ_actif = false;
uniform sampler2D hauteur_a : filter_nearest, repeat_disable;
uniform sampler2D hauteur_b : filter_nearest, repeat_disable;
uniform float melange = 0.0;
uniform vec2 champ_texel = vec2(0.0125, 0.025);
uniform float champ_dx = 0.1;
varying vec2 pente_champ;

float hauteur_champ(vec2 uv) {
    return mix(texture(hauteur_a, uv).r, texture(hauteur_b, uv).r, melange);
}

void vertex() {
    pente_champ = vec2(0.0);
    if (champ_actif) {
        VERTEX.y += hauteur_champ(UV);
        vec2 du = vec2(champ_texel.x, 0.0);
        vec2 dv = vec2(0.0, champ_texel.y);
        // UV.y croît vers le nord (−z de Godot) : la pente en z de Godot est l'opposée de celle en y.
        pente_champ = vec2((hauteur_champ(UV + du) - hauteur_champ(UV - du)) / (2.0 * champ_dx),
            -(hauteur_champ(UV + dv) - hauteur_champ(UV - dv)) / (2.0 * champ_dx));
    }
}

/// Le point de la scène vu au pixel `uv`, dans le monde (profondeur inversée de Godot 4.3+).
vec3 scene_monde(vec2 uv, mat4 inv_proj, mat4 inv_vue) {
    float d = texture(profondeur_ecran, uv).r;
    vec4 v = inv_proj * vec4(uv * 2.0 - 1.0, d, 1.0);
    v.xyz /= v.w;
    return (inv_vue * vec4(v.xyz, 1.0)).xyz;
}

float fresnel(float c, float n) {
    float st2 = (1.0 - c * c) / (n * n);
    float ct = sqrt(max(1.0 - st2, 0.0));
    float rs = (c - n * ct) / (c + n * ct);
    float rp = (n * c - ct) / (n * c + ct);
    return 0.5 * (rs * rs + rp * rp);
}

/// La pente des rides en (x, z) du monde de Godot : six trains d'onde courte (5 à 30 cm), phases qui avancent comme des
/// ondes de gravité-capillarité (`ω² = g·k + σ/ρ·k³`), amplitude réglée pour la pente quadratique moyenne `agitation`, et
/// un paquet de rides concentriques qui s'écarte du foyer.
vec2 pente_rides(vec2 p) {
    vec2 s = vec2(0.0);
    for (int i = 0; i < 6; i++) {
        float a = 2.399 * float(i) + 0.4;
        float lambda = 0.05 + 0.05 * float(i);
        float k = TAU / lambda;
        float w = sqrt(9.81 * k + 7.3e-5 * k * k * k);
        vec2 d = vec2(cos(a), sin(a));
        s += d * cos(k * dot(d, p) - w * temps + 1.7 * float(i)) * (agitation * 0.5774);
    }
    vec2 r = p - foyer.xz;
    float dist = length(r);
    if (foyer_pente > 0.0 && dist > 1e-3) {
        float enveloppe = exp(-dist * dist / (foyer_rayon * foyer_rayon));
        float k = TAU / 0.12;
        s += (r / dist) * foyer_pente * enveloppe * sin(k * dist - 18.0 * temps);
    }
    return s;
}

void fragment() {
    vec3 surface = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
    vec3 vue = normalize(surface - CAMERA_POSITION_WORLD);
    vec2 s = champ_actif ? pente_champ : pente_rides(surface.xz);
    float variance_pluie = 0.0;
    float coeur_pluie = 0.0;
    float empreinte_pluie = 1.0;
    if (pluie.x > 0.0) {
        // Condition uniforme : les dérivées d'écran y restent définies.
        vec2 q = surface.xz;
        empreinte_pluie = sqrt(length(dFdx(q)) * length(dFdy(q)));
        float coeur;
        vec3 r = pluie_rides(q, temps, dFdx(q), dFdy(q), coeur);
        coeur_pluie = coeur;
        s += r.xy;
        variance_pluie = r.z;
    }
    vec3 n = normalize(vec3(-s.x, 1.0, -s.y));
    // Le fond vu à travers : réfracté, relu à l'écran s'il est sous la surface.
    vec3 fond0 = scene_monde(SCREEN_UV, INV_PROJECTION_MATRIX, INV_VIEW_MATRIX);
    float h0 = surface.y - fond0.y;
    vec3 r = refract(vue, n, 1.0 / indice);
    vec2 uv_r = SCREEN_UV;
    vec3 fond = fond0;
    if (h0 > 0.0 && r.y < -0.05) {
        vec3 point = surface + r * (h0 / -r.y);
        vec4 clip = PROJECTION_MATRIX * (VIEW_MATRIX * vec4(point, 1.0));
        vec2 uv = clip.xy / clip.w * 0.5 + 0.5;
        if (uv.x > 0.0 && uv.x < 1.0 && uv.y > 0.0 && uv.y < 1.0) {
            vec3 f1 = scene_monde(uv, INV_PROJECTION_MATRIX, INV_VIEW_MATRIX);
            if (f1.y < surface.y) {
                uv_r = uv;
                fond = f1;
            }
        }
    }
    float h_fond = max(surface.y - fond.y, 0.0);
    vec3 t_colonne = exp(-kd * (h_fond + length(fond - surface)));
    // S379 : la pente non résolue de la pluie étale aussi le fond vu — l'écart du rayon réfracté, `(1 − 1/n)·σ` par
    // composante sur le trajet dans l'eau, rapporté à l'empreinte du pixel : le niveau de la chaîne floutée de l'écran.
    float lod_pluie = 0.0;
    if (variance_pluie > 0.0) {
        float etal = (1.0 - 1.0 / indice) * sqrt(0.5 * variance_pluie) * length(fond - surface);
        lod_pluie = log2(1.0 + etal / max(empreinte_pluie, 1e-5));
    }
    vec3 fond_vu = textureLod(ecran, uv_r, lod_pluie).rgb * t_colonne;
    // En coordonnées de B (x, −z, y) pour le ciel.
    vec3 rayon = vec3(vue.x, -vue.z, vue.y);
    vec3 nb = vec3(n.x, -n.z, n.y);
    vec3 soleil = normalize(SOLEIL_B);
    float f = fresnel(clamp(dot(-rayon, nb), 0.0, 1.0), indice);
    vec3 reflet = reflect(rayon, nb);
    float eclat = pow(max(dot(reflet, soleil), 0.0), 180.0);
    vec3 corps = R0 * gain_eau * (0.6 + 0.4 * max(dot(nb, soleil), 0.0));
    vec3 sous = corps * (1.0 - t_colonne) + fond_vu;
    if (variance_pluie > 0.0) {
        // La pente non résolue de la pluie, isotrope (σ² = variance/2 par composante) : Fresnel et ciel intégrés par
        // 3 × 3 nœuds de Gauss-Hermite (±√3·σ, poids 1/6, 2/3, 1/6) ; l'éclat, lobe `cos^m ≈ e^(−m·θ²/2)` convolé par
        // l'écart du reflet (deux fois celui de la normale) : exposant `1/(1/m + 4σ²)`, pic réduit d'autant.
        float sig = sqrt(0.5 * variance_pluie);
        vec3 somme = vec3(0.0);
        for (int i = -1; i <= 1; i++) {
            for (int k = -1; k <= 1; k++) {
                float w = (i == 0 ? 2.0 / 3.0 : 1.0 / 6.0) * (k == 0 ? 2.0 / 3.0 : 1.0 / 6.0);
                vec2 si = s + sig * 1.7320508 * vec2(float(i), float(k));
                vec3 ni = normalize(vec3(-si.x, 1.0, -si.y));
                vec3 nbi = vec3(ni.x, -ni.z, ni.y);
                float fi = fresnel(clamp(dot(-rayon, nbi), 0.0, 1.0), indice);
                somme += w * mix(sous, ciel_b(reflect(rayon, nbi), 2), fi);
            }
        }
        float m_eff = 1.0 / (1.0 / 180.0 + 4.0 * sig * sig);
        float eclat_pluie = (m_eff / 180.0) * pow(max(dot(reflet, soleil), 0.0), m_eff);
        ALBEDO = somme + vec3(1.0, 0.95, 0.85) * eclat_pluie * 1.2;
    } else {
        ALBEDO = mix(sous, ciel_b(reflet, 2), f) + vec3(1.0, 0.95, 0.85) * eclat * 1.2;
    }
    // Contrôle du taux (critère 2 de S379) : rouge pur sur un cœur d'anneau jeune, noir ailleurs.
    if (controle_pluie && pluie.x > 0.0) {
        ALBEDO = vec3(coeur_pluie, 0.0, 0.0);
    }
}
