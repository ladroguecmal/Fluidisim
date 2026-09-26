//! S374–S375 — **la piscine de V**, une seule définition pour `piscine_v` (V seul, S374) et `piscine_delta` (V et δ 3D,
//! S375, ADR-200). Une piscine à débordement : un bassin de 8 × 4 m, un déversoir de 4 m sur son petit côté est, qui
//! tombe dans un bac tampon plus bas, et une pompe qui rend l'eau au bassin par une buse au-dessus de sa surface, côté
//! ouest. Coordonnées de B : x à l'est, y au nord, z en haut ; le bassin est centré sur l'origine.
use water_core::hydro_network::*;

pub const G: [f32; 3] = [0.0, 0.0, -9.81];

/// Le bassin : 8 × 4 m au sol, 1,5 m de haut ; le fond au niveau 0.
pub const BASSIN_TAILLE_M: [f64; 3] = [8.0, 4.0, 1.5];
pub const BASSIN_FOND_M: [f64; 3] = [0.0, 0.0, 0.0];
/// Le bac tampon : 1 × 4 m, 1,2 m de haut, contre la face extérieure du mur est (épais de 0,2 m), fond à −1,3 m.
pub const TAMPON_TAILLE_M: [f64; 3] = [1.0, 4.0, 1.2];
pub const TAMPON_FOND_M: [f64; 3] = [4.7, 0.0, -1.3];
/// Le déversoir : toute la largeur du mur est, seuil à 1,40 m.
pub const SEUIL_M: [f64; 3] = [4.0, 0.0, 1.4];
pub const DEVERSOIR_LARGEUR_M: f64 = 4.0;
/// La pompe : prise à 5 cm du fond du bac tampon, buse sur le mur ouest à 1,7 m ; 12 l/s, hauteur de barrage 8 m (données
/// d'auteur, ADR-199 D3). La buse, 5 cm de diamètre, n'existe pas dans V : elle donne la vitesse du jet.
pub const PRISE_M: [f64; 3] = [4.7, 0.0, -1.25];
pub const SORTIE_M: [f64; 3] = [-4.0, 0.0, 1.7];
pub const POMPE_QMAX_MLPS: i64 = 12_000;
pub const POMPE_H0_M: f64 = 8.0;
pub const BUSE_DIAMETRE_M: f64 = 0.05;
/// Niveaux de départ : le bassin 5 mm sous le seuil, le bac tampon à 0,80 m.
pub const BASSIN_DEPART_M: f64 = 1.395;
pub const TAMPON_DEPART_M: f64 = 0.80;
/// Le scénario : la pompe arrêtée 5 s, lancée, arrêtée à 240 s ; 330 s en tout.
pub const POMPE_MARCHE_S: f64 = 5.0;
pub const POMPE_ARRET_S: f64 = 240.0;
pub const DUREE_S: f64 = 330.0;

pub fn um(m: f64) -> i64 {
    (m * 1e6).round() as i64
}
pub fn um3(p: [f64; 3]) -> [i64; 3] {
    p.map(um)
}
fn prisme(hauteur_um: i64) -> [i64; SHAPE_ENTRIES] {
    let mut t = [0i64; SHAPE_ENTRIES];
    for (i, v) in t.iter_mut().enumerate() {
        *v = hauteur_um * i as i64 / (SHAPE_ENTRIES - 1) as i64;
    }
    t
}

/// La commande de la pompe au temps `t` du scénario.
pub fn commande_pompe(t: f64) -> i64 {
    if (POMPE_MARCHE_S..POMPE_ARRET_S).contains(&t) { CONTROL_FULL } else { 0 }
}

/// Les tables de forme (bassin, bac tampon), les deux nœuds, les deux arêtes (déversoir, pompe arrêtée).
pub fn construire() -> (Vec<i64>, [HydroNode; 2], [Opening; 2]) {
    let tables = [prisme(um(BASSIN_TAILLE_M[2])), prisme(um(TAMPON_TAILLE_M[2]))].concat();
    let aire_b = BASSIN_TAILLE_M[0] * BASSIN_TAILLE_M[1];
    let aire_t = TAMPON_TAILLE_M[0] * TAMPON_TAILLE_M[1];
    let ml = |m3: f64| (m3 * 1e6).round() as i64;
    let nodes = [
        HydroNode {
            volume_ml: ml(aire_b * BASSIN_DEPART_M),
            capacity_ml: ml(aire_b * BASSIN_TAILLE_M[2]),
            origin_um: um3(BASSIN_FOND_M),
            shape: 0,
        },
        HydroNode {
            volume_ml: ml(aire_t * TAMPON_DEPART_M),
            capacity_ml: ml(aire_t * TAMPON_TAILLE_M[2]),
            origin_um: um3(TAMPON_FOND_M),
            shape: 1,
        },
    ];
    let edges = [
        Opening {
            from: 0,
            to: Some(1),
            flow: Flow::Weir { width_mm: (DEVERSOIR_LARGEUR_M * 1e3).round() as i64 },
            position_um: um3(SEUIL_M),
            discharge: WEIR_DISCHARGE,
            residue_nl: 0,
            control_pm: CONTROL_FULL,
        },
        Opening {
            from: 1,
            to: Some(0),
            flow: Flow::Pump { max_flow_mlps: POMPE_QMAX_MLPS, shutoff_head_um: um(POMPE_H0_M), outlet_um: um3(SORTIE_M) },
            position_um: um3(PRISE_M),
            discharge: 0.0,
            residue_nl: 0,
            control_pm: 0,
        },
    ];
    (tables, nodes, edges)
}
