// S382 — **L'OCCULTATION DU CIEL** (ADR-206 ; demande de l'utilisateur, R30). Une surface reçoit le ciel qu'elle voit,
// `E = ∫ L(ω)·V(ω)·max(n·ω, 0) dω` ; jusqu'ici `V = 1`. `V` se calcule sur des **occultants analytiques** — des boîtes
// alignées sur les axes du monde de Godot, que la scène déclare (`occultants_min`, `occultants_max`) —, et non en espace
// écran : nos matériaux de scène sont non éclairés par Godot, et l'espace écran oublie ce qui est hors champ.
//
// La discrétisation (ADR-206 D2) : 32 azimuts ; pour chacun et chaque boîte, l'intervalle d'élévation qu'elle masque (test
// de dalles en 2D, arête haute la plus proche, arête basse la plus lointaine), réuni dans un masque de 32 bandes **d'égal
// angle solide** — uniformes en `u = sin h`, puisque `dω = du·dφ` ; une bande est masquée si son centre l'est. La part
// masquée, pondérée par `max(n·ω, 0)` — et par `(1 + 2u)/3` pour le ciel couvert de la CIE —, est rapportée au total
// **exact** (`ciel_uniforme_incline`, `ciel_couvert_incline`, `ciel.gdshaderinc`). Une boîte dont l'arête la plus haute
// reste sous le centre de la première bande, vue du point, est écartée avant la boucle : **loin des occultants, la part
// vue vaut 1 exactement**, et `eclairage_vu` rend `eclairage` au bit. À inclure après `ciel.gdshaderinc`.

const int OCCULTANTS_MAX = 32;
const int AZIMUTS_OCC = 32;
const int BANDES_OCC = 32;
// tan de l'élévation du centre de la première bande, `u = 1/64` : 0,015625/√(1 − 0,015625²).
const float TAN_BANDE_0 = 0.0156269;

uniform int occultants_n = 0;
uniform vec3 occultants_min[OCCULTANTS_MAX];
uniform vec3 occultants_max[OCCULTANTS_MAX];
// Contrôle (S382, critère 2) : la surface rend (part du ciel uniforme, part du ciel de la CIE, 0) au lieu de sa couleur.
uniform bool controle_occultation = false;

/// Le masque d'élévation d'une boîte dans l'azimut de direction horizontale `d` (x, z), vu de `q` ; 0 si elle n'y est pas.
uint masque_boite(vec3 q, vec2 d, vec3 bmin, vec3 bmax) {
    vec2 ta = (bmin.xz - q.xz) / d;
    vec2 tb = (bmax.xz - q.xz) / d;
    vec2 tmn = min(ta, tb);
    vec2 tmx = max(ta, tb);
    float t0 = max(max(tmn.x, tmn.y), 0.0);
    float t1 = min(tmx.x, tmx.y);
    if (t1 <= t0) {
        return 0u;
    }
    float dy_hi = bmax.y - q.y;
    float dy_lo = bmin.y - q.y;
    // sin h de l'arête haute, la plus proche (t0), et de l'arête basse, la plus lointaine (t1) ; sous l'horizon : 0.
    float u_hi = t0 > 0.0 ? dy_hi * inversesqrt(dy_hi * dy_hi + t0 * t0) : 1.0;
    float u_lo = dy_lo > 0.0 ? dy_lo * inversesqrt(dy_lo * dy_lo + t1 * t1) : 0.0;
    int k0 = max(int(ceil(u_lo * float(BANDES_OCC) - 0.5)), 0);
    int k1 = min(int(floor(u_hi * float(BANDES_OCC) - 0.5)), BANDES_OCC - 1);
    if (k1 < k0) {
        return 0u;
    }
    uint haut = k1 >= 31 ? 0xFFFFFFFFu : ((1u << uint(k1 + 1)) - 1u);
    return haut & ~((1u << uint(k0)) - 1u);
}

/// **La part du ciel que voit** le point `p` (monde de Godot) de normale `n` (monde, unitaire) : x, ciel uniforme ; y, ciel
/// couvert de la CIE. `vec2(1.0)` exactement quand aucun occultant n'atteint la première bande.
///
/// Deux passes. Les boîtes d'abord : celles qui atteignent la première bande, et les cellules d'azimut que couvre leur
/// emprise vue du point (quatre coins autour du centre ; toutes si le point est dedans) ; les cellules qui contiennent un
/// **bord** d'emprise sont marquées. Puis les cellules : une direction au centre, ou **quatre** dans une cellule marquée —
/// là, l'occultation saute d'une valeur à zéro au bord de la boîte, et un seul azimut par cellule de 11,25° faussait la
/// part vue de 1,5 % (contrôle de S382, P6). Dans chaque direction, le masque des bandes réuni sur les boîtes de la cellule.
vec2 ciel_vu(vec3 p, vec3 n) {
    if (occultants_n <= 0) {
        return vec2(1.0);
    }
    // Hors de la face qui porte le point : 2 mm le long de la normale.
    vec3 q = p + 2e-3 * n;
    float dphi = TAU / float(AZIMUTS_OCC);
    int proches[OCCULTANTS_MAX];
    int debut_j[OCCULTANTS_MAX];
    int etendue_j[OCCULTANTS_MAX];
    int np = 0;
    uint bords = 0u;
    for (int i = 0; i < min(occultants_n, OCCULTANTS_MAX); i++) {
        vec3 bmin = occultants_min[i];
        vec3 bmax = occultants_max[i];
        float dy_hi = bmax.y - q.y;
        if (dy_hi <= 0.0) {
            continue;
        }
        vec2 ecart = max(max(bmin.xz - q.xz, q.xz - bmax.xz), vec2(0.0));
        float dh = length(ecart);
        if (dy_hi < TAN_BANDE_0 * dh) {
            continue;
        }
        int ja = 0;
        int etendue = AZIMUTS_OCC - 1;
        if (dh > 0.0) {
            vec2 c = 0.5 * (bmin.xz + bmax.xz) - q.xz;
            float phi_c = atan(c.y, c.x);
            float dmin = 0.0;
            float dmax = 0.0;
            for (int k = 0; k < 4; k++) {
                vec2 coin = vec2(k % 2 == 0 ? bmin.x : bmax.x, k < 2 ? bmin.z : bmax.z) - q.xz;
                float dl = atan(coin.y, coin.x) - phi_c;
                dl = dl - TAU * floor((dl + PI) / TAU);
                dmin = min(dmin, dl);
                dmax = max(dmax, dl);
            }
            ja = int(floor((phi_c + dmin) / dphi));
            int jb = int(floor((phi_c + dmax) / dphi));
            etendue = jb - ja;
            ja = ja - AZIMUTS_OCC * int(floor(float(ja) / float(AZIMUTS_OCC)));
            jb = jb - AZIMUTS_OCC * int(floor(float(jb) / float(AZIMUTS_OCC)));
            bords |= (1u << uint(ja)) | (1u << uint(jb));
        }
        proches[np] = i;
        debut_j[np] = ja;
        etendue_j[np] = etendue;
        np++;
    }
    if (np == 0) {
        return vec2(1.0);
    }
    float bloque_u = 0.0;
    float bloque_c = 0.0;
    for (int j = 0; j < AZIMUTS_OCC; j++) {
        int sous = ((bords >> uint(j)) & 1u) == 1u ? 4 : 1;
        for (int s = 0; s < sous; s++) {
            float phi = (float(j) + (float(s) + 0.5) / float(sous)) * dphi;
            vec2 d = vec2(cos(phi), sin(phi));
            uint m = 0u;
            for (int b = 0; b < np; b++) {
                int decale = j - debut_j[b];
                decale = decale - AZIMUTS_OCC * int(floor(float(decale) / float(AZIMUTS_OCC)));
                if (decale <= etendue_j[b]) {
                    int i = proches[b];
                    m |= masque_boite(q, d, occultants_min[i], occultants_max[i]);
                }
            }
            float poids = 1.0 / float(sous);
            while (m != 0u) {
                int k = int(findLSB(m));
                m &= m - 1u;
                float u = (float(k) + 0.5) / float(BANDES_OCC);
                float ch = sqrt(1.0 - u * u);
                float c = max(dot(n, vec3(ch * d.x, u, ch * d.y)), 0.0) * poids;
                bloque_u += c;
                bloque_c += c * (1.0 + 2.0 * u) / 3.0;
            }
        }
    }
    float cellule = dphi / float(BANDES_OCC);
    float total_u = ciel_uniforme_incline(n.y);
    float total_c = ciel_couvert_incline(n.y);
    return vec2(total_u > 0.0 ? clamp(1.0 - bloque_u * cellule / total_u, 0.0, 1.0) : 1.0,
                total_c > 0.0 ? clamp(1.0 - bloque_c * cellule / total_c, 0.0, 1.0) : 1.0);
}

// S382 P6c — la part vue **cuite** (`cuisson_ciel.gdshader`) : le texel du rang `vu_decalage + VERTEX_ID` ; −1, pas de
// cuisson (aucun occultant) : 1.
uniform sampler2D vu_cuite : filter_nearest;
uniform int vu_decalage = -1;

vec2 ciel_vu_cuit(int sommet) {
    if (vu_decalage < 0) {
        return vec2(1.0);
    }
    ivec2 taille = textureSize(vu_cuite, 0);
    int i = vu_decalage + sommet;
    return texelFetch(vu_cuite, ivec2(i % taille.x, i / taille.x), 0).rg;
}

/// S382 P6b — **la part vue, par sommet** : évaluée dans `vertex()` et interpolée (elle varie lentement ; à chaque pixel,
/// elle coûtait 8 à 9 ms dans les vues proches de la piscine). L'interpolation de sommets qui valent tous 1 peut rendre
/// 0,99999994 : ramené à 1 au-dessus de 1 − 10⁻⁵, pour que l'image loin des occultants reste celle d'avant, au bit.
vec2 ciel_vu_interpole(vec2 v) {
    return mix(v, vec2(1.0), step(vec2(0.99999), v));
}

/// Le facteur d'éclairement d'`eclairage()` (`ciel.gdshaderinc`), le ciel masqué : `vu` = (part du ciel uniforme, part du
/// ciel couvert de la CIE, part du soleil). Ciel clair : `0,6·vu.x + 0,4·max(n·s, 0)·direct·vu.z` ; couvert :
/// `E_h·(E(β)/E(0)·vu.y + sol)` — la lumière renvoyée par le sol n'est pas masquée. `vu` = (1, 1, 1) : `eclairage()`, au bit.
float eclairage_vu(vec3 n, vec3 soleil, float direct, vec3 vu) {
    if (vu.x >= 1.0 && vu.y >= 1.0 && vu.z >= 1.0) {
        return eclairage(n, soleil, direct);
    }
    float clair = 0.6 * vu.x + 0.4 * max(dot(n, soleil), 0.0) * direct * vu.z;
    if (couvert <= 0.0) {
        return clair;
    }
    float ciel = ciel_couvert_incline(n.z) / (7.0 * PI / 9.0);
    float sol = ALBEDO_SOL_COUVERT * 0.5 * (1.0 - n.z);
    return mix(clair, eclairement_horizontal_clair() * (ciel * vu.y + sol), couvert);
}
