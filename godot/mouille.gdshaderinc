// S392 — **LES SURFACES MOUILLÉES** (ADR-205, pièce 5a ; demande de l'utilisateur, « Continue avec la pluie, pièce 5 »).
//
// Un matériau rugueux sous un film d'eau (Ångström 1925 ; Lekner et Dorf 1988, *Appl. Opt.* 27, 1278) : la lumière entre
// dans le film, se diffuse sur le matériau ; à chaque remontée, l'interface eau–air en renvoie vers le bas une part `r̄ᵢ`
// (réflexion totale au-delà de 48,6°, partielle en deçà). Rebonds sommés et sortie réfractée : la radiance diffuse vaut
// `(E/π)·a·(1 − r̄ᵢ)/(1 − a·r̄ᵢ)·(1 − R(θ))` — `R(θ)` le Fresnel du film vu de l'œil, `r̄ᵢ` = 0,47459 pour l'eau
// (`outils/sol_mouille.py`, deux intégrations et la réciprocité). S'y ajoute le **reflet du film**, `R(θ)·L(réfléchi)` : le
// ciel, ou l'occultant que le rayon réfléchi rencontre. `assombrissement` (1 par défaut) multiplie l'albédo du matériau
// sous l'eau — le second effet de Lekner et Dorf (l'indice relatif qui baisse, les pores remplis), non modélisé, laissé au
// jugement de R33.
//
// **Où** : la pluie tombe à la verticale (sans vent). Une face tournée vers le ciel est mouillée si la verticale au-dessus
// d'elle est libre (les occultants de S382, `occultation.gdshaderinc`) ; une face verticale l'est au pied, dans la bande
// des rejaillissements `exp(−h/0,1 m)` si le sol devant elle est exposé — **hypothèse déclarée**, à calibrer —, et sous
// le bord d'une face exposée (`ruissellement_haut`, les nez de margelle) ; une face tournée vers le bas, jamais ; une face
// dans l'emprise d'un bac, jamais (l'eau des bacs est rendue par leur nuanceur). Régime établi : sous toute pluie, les
// faces exposées sont mouillées ; mouiller et sécher dans le temps relève de la météo. À inclure après
// `occultation.gdshaderinc`.

const float MOUILLE_R_I = 0.47459;
const float MOUILLE_N = 1.333;
const float REJAILLISSEMENT_M = 0.1;
// Rugosité du film (demi-angle du cône des rayons réfléchis) : *à calibrer* ; elle adoucit le reflet des occultants.
const float RUGOSITE_FILM = 0.05;

uniform float mouillure_active = 0.0;
uniform float assombrissement = 1.0;
uniform float sol_mouille_y = 0.0;
// Les bacs : emprise (x0, z0, x1, z1) dans le monde de Godot, et cote du haut de leurs murs.
uniform int bacs_n = 0;
uniform vec4 bacs_emprise[4];
uniform float bacs_haut[4];
// Nez de margelle : une face verticale dont le haut de la boîte est exposé, à moins de 0,1 m sous ce haut, ruisselle ;
// −1 : pas de ruissellement.
uniform float ruissellement_haut = -1.0;
// Contrôle (S392) : 0 rendu ; 1 la mouillure ; 2 le diffus seul ; 3 le reflet seul.
uniform int controle_mouille = 0;

/// Fresnel non polarisé air → eau, `c` le cosinus d'incidence ; même formule que `outils/sol_mouille.py`.
float fresnel_eau(float c) {
    c = clamp(c, 0.0, 1.0);
    float st = sqrt(max(0.0, 1.0 - c * c)) / MOUILLE_N;
    float ct = sqrt(max(0.0, 1.0 - st * st));
    float rs = (c - MOUILLE_N * ct) / (c + MOUILLE_N * ct);
    float rp = (ct - MOUILLE_N * c) / (ct + MOUILLE_N * c);
    return 0.5 * (rs * rs + rp * rp);
}

/// Le rayon vertical montant depuis `q` rencontre-t-il un occultant ? Test exact : `q` dans l'emprise horizontale d'une boîte
/// et sous son haut. (Vu en S392 : le test de dalles avec des composantes nulles divise par zéro, indéfini sur la carte — la
/// verticale sous le débord de la margelle passait pour libre.)
bool verticale_bloquee(vec3 q) {
    for (int i = 0; i < min(occultants_n, OCCULTANTS_MAX); i++) {
        vec3 a = occultants_min[i];
        vec3 b = occultants_max[i];
        if (q.x > a.x && q.x < b.x && q.z > a.z && q.z < b.z && q.y < b.y) {
            return true;
        }
    }
    return false;
}

/// Le rayon `q + t·d` rencontre-t-il un occultant ? Composantes nulles de `d` écartées de zéro (pas de division par zéro).
bool rayon_bloque(vec3 q, vec3 d) {
    d = mix(d, vec3(1e-6), lessThan(abs(d), vec3(1e-6)));
    for (int i = 0; i < min(occultants_n, OCCULTANTS_MAX); i++) {
        vec3 ta = (occultants_min[i] - q) / d;
        vec3 tb = (occultants_max[i] - q) / d;
        vec3 tmn = min(ta, tb);
        vec3 tmx = max(ta, tb);
        if (min(min(tmx.x, tmx.y), tmx.z) > max(max(max(tmn.x, tmn.y), tmn.z), 0.0)) {
            return true;
        }
    }
    return false;
}

/// La part d'un pixel dont la verticale est libre : quatre points décalés dans le pixel (grille tournée, comme
/// `soleil_vu`), un rayon vers le haut chacun.
float verticale_libre(vec3 p, vec3 n, vec3 dpx, vec3 dpy) {
    if (occultants_n <= 0) {
        return 1.0;
    }
    // Test précoce : le centre du pixel, boîtes élargies de l'empreinte du pixel ; s'il est libre, les quatre le sont.
    vec3 marge = vec3(0.5 * (length(dpx) + length(dpy)) + 1e-3);
    bool proche = false;
    for (int i = 0; i < min(occultants_n, OCCULTANTS_MAX); i++) {
        vec3 a = occultants_min[i] - marge;
        vec3 b = occultants_max[i] + marge;
        if (p.x > a.x && p.x < b.x && p.z > a.z && p.z < b.z && p.y < b.y) {
            proche = true;
            break;
        }
    }
    if (!proche) {
        return 1.0;
    }
    float libre = 0.0;
    for (int k = 0; k < 4; k++) {
        float signe = k < 2 ? 1.0 : -1.0;
        vec2 e = signe * (k % 2 == 0 ? vec2(0.25, 0.75) : vec2(0.75, 0.25));
        vec3 q = p + 5e-4 * n + 0.5 * (e.x * dpx + e.y * dpy);
        libre += verticale_bloquee(q) ? 0.0 : 0.25;
    }
    return libre;
}

/// La mouillure du point `p` (monde), de normale `n` : 0 sec, 1 mouillé.
float mouillure(vec3 p, vec3 n, vec3 dpx, vec3 dpy) {
    if (mouillure_active <= 0.0) {
        return 0.0;
    }
    for (int b = 0; b < min(bacs_n, 4); b++) {
        vec4 e = bacs_emprise[b];
        if (p.x > e.x && p.x < e.z && p.z > e.y && p.z < e.w && p.y <= bacs_haut[b] + 1e-3) {
            return 0.0;
        }
    }
    if (n.y > 0.5) {
        return verticale_libre(p, n, dpx, dpy);
    }
    if (n.y < -0.5) {
        return 0.0;
    }
    float w = 0.0;
    // Les rejaillissements : au pied, si le sol devant la face est exposé.
    float h = p.y - sol_mouille_y;
    if (h >= 0.0 && h < 6.0 * REJAILLISSEMENT_M) {
        vec3 pied = vec3(p.x, sol_mouille_y, p.z) + 0.1 * vec3(n.x, 0.0, n.z);
        w = exp(-h / REJAILLISSEMENT_M) * (verticale_bloquee(pied + vec3(0.0, 5e-4, 0.0)) ? 0.0 : 1.0);
    }
    // Le ruissellement sous un bord exposé.
    if (ruissellement_haut > -0.5 && ruissellement_haut - p.y < 0.1) {
        vec3 haut = vec3(p.x, ruissellement_haut + 5e-4, p.z) - 5e-3 * vec3(n.x, 0.0, n.z);
        w = max(w, verticale_bloquee(haut) ? 0.0 : 1.0);
    }
    return w;
}

/// Radiance d'un occultant vu dans le reflet : une face verticale de béton sec (0,42) tournée vers le point, sous le ciel
/// de la scène, à demi masquée — approximation déclarée (les occultants n'ont pas d'albédo déclaré).
vec3 radiance_occultant(vec3 r_b) {
    vec3 face = normalize(vec3(-r_b.x, -r_b.y, 0.0) + vec3(0.0, 0.0, 1e-4));
    return vec3(0.42) * gain_eau * eclairage_vu(face, normalize(SOLEIL_B), 1.0, vec3(0.5, 0.5, 1.0));
}

/// Le reflet du film : quatre rayons dans un cône de demi-angle `RUGOSITE_FILM` autour de la direction réfléchie ; chacun
/// prend le ciel (`ciel_b`, deux octaves) ou l'occultant qu'il rencontre.
vec3 reflet_film(vec3 p, vec3 n, vec3 vue) {
    vec3 r = reflect(vue, n);
    if (r.y <= 0.0) {
        r = normalize(vec3(r.x, 1e-3, r.z));
    }
    vec3 t1 = normalize(cross(r, abs(r.y) < 0.9 ? vec3(0.0, 1.0, 0.0) : vec3(1.0, 0.0, 0.0)));
    vec3 t2 = cross(r, t1);
    // Test précoce : le rayon central contre les boîtes élargies de ce dont le cône s'en écarte (≈ 5 cm par mètre, jusqu'à
    // 10 m) ; s'il les manque toutes, les quatre rayons voient le ciel.
    bool proche = false;
    if (occultants_n > 0) {
        vec3 q0 = p + 5e-4 * n;
        vec3 dc = mix(r, vec3(1e-6), lessThan(abs(r), vec3(1e-6)));
        for (int i = 0; i < min(occultants_n, OCCULTANTS_MAX); i++) {
            vec3 ta = (occultants_min[i] - vec3(0.5) - q0) / dc;
            vec3 tb = (occultants_max[i] + vec3(0.5) - q0) / dc;
            vec3 tmn = min(ta, tb);
            vec3 tmx = max(ta, tb);
            if (min(min(tmx.x, tmx.y), tmx.z) > max(max(max(tmn.x, tmn.y), tmn.z), 0.0)) {
                proche = true;
                break;
            }
        }
    }
    // Loin des occultants, sous le ciel couvert : la CIE est linéaire en `sin h`, et les quatre rayons symétriques ont pour
    // élévation moyenne celle du rayon central divisée par `√(1 + ε²)` — un seul appel, la même moyenne.
    if (!proche && couvert >= 1.0) {
        vec3 rb = vec3(r.x, -r.z, r.y);
        return ciel_couvert(vec3(rb.xy, rb.z * inversesqrt(1.0 + RUGOSITE_FILM * RUGOSITE_FILM)));
    }
    vec3 somme = vec3(0.0);
    for (int k = 0; k < 4; k++) {
        float a = TAU * (float(k) + 0.5) / 4.0;
        vec3 d = normalize(r + RUGOSITE_FILM * (cos(a) * t1 + sin(a) * t2));
        vec3 d_b = vec3(d.x, -d.z, d.y);
        bool bloque = proche && rayon_bloque(p + 5e-4 * n, d);
        // Sous le ciel couvert (la pluie), `ciel_b` rend `ciel_couvert` exactement : les nuages du ciel clair sont évités.
        vec3 ciel = couvert >= 1.0 ? ciel_couvert(d_b) : ciel_b(d_b, 2);
        somme += 0.25 * (bloque ? radiance_occultant(d_b) : ciel);
    }
    return somme;
}

/// La radiance d'une surface mouillée à la part `w`, depuis sa radiance sèche `sec` (`albedo·gain·E`), son albédo `a`, le
/// point `p`, la normale `n` et la direction de vue `vue` (de l'œil vers le point), dans le monde de Godot.
vec3 surface_mouillee(vec3 sec, vec3 a, float w, vec3 p, vec3 n, vec3 vue) {
    float R = fresnel_eau(dot(n, -vue));
    vec3 ka = assombrissement * a;
    vec3 facteur = assombrissement * (1.0 - MOUILLE_R_I) / (1.0 - ka * MOUILLE_R_I) * (1.0 - R);
    vec3 diffus = sec * facteur;
    vec3 reflet = R * reflet_film(p, n, vue);
    if (controle_mouille == 2) {
        return diffus;
    }
    if (controle_mouille == 3) {
        return reflet;
    }
    return mix(sec, diffus + reflet, w);
}
