//! **Où peut aller le volume net qui sort d'un domaine δ ?** — S312, point 3 d'ADR-180.
//!
//! [ADR-180](../../../docs/adr/ADR-180-retour-delta-w-et-conservation-du-volume.md) D4 :
//! *« Je souhaite que vous étudiiez en priorité la possibilité de conserver le volume net à
//! travers V ou une modification du niveau moyen de B […] Il ne faut pas créer une nouvelle
//! primitive dans W avant d'avoir vérifié si cette responsabilité relève déjà d'une autre
//! couche. »*
//!
//! S311 avait mesuré **une** primitive — l'impact radial porte de l'énergie et pas de volume.
//! Cela ne dit pas si c'est une propriété de *cette* primitive ou de *la couche*. La différence
//! commande la suite : si c'est la primitive, on en ajoute une ; si c'est la couche, W n'est pas
//! le bon receveur et le chercher là serait une erreur d'architecture.
//!
//! # Ce que ce banc mesure
//!
//! 1. **`displaced_l` est inerte.** Le contrat `WaveEvent` porte un volume déplacé. Deux impacts
//!    identiques à ce champ près donnent-ils le même champ ? ADR-180 D3 interdit de le lire dans
//!    l'encodage : *« un champ présent dans un encodage ne prouve pas qu'un champ construit
//!    l'honore »*.
//! 2. **Le volume net de chaque production de W**, rapporté à son volume **absolu** — un net
//!    petit ne veut rien dire si le champ lui-même est petit. Trois productions : l'impact
//!    radial (ADR-060), le champ périodique (ADR-058), et la source de pression mobile du
//!    sillage (ADR-071/S96), assemblée par son spectre gaussien.
//! 3. **Le mode `k = 0`**, qui *est* le volume net : l'intégrale d'un champ sur le plan est
//!    exactement l'amplitude de son mode de nombre d'onde nul. Chaque production déclare son
//!    plus petit `k`, et `ModalPressure` est interrogée à `k = 0`.
//!
//! # Ce qu'il ne mesure pas
//!
//! Ni V ni B : leur réponse est dans leur **type**, et un type se lit. `HydroNode::volume_ml` est
//! un `i64` de millilitres — en V, le volume *est* la variable d'état. `SeaState` n'a aucun champ
//! de niveau moyen — en B, le plan de repos est la référence, et rien ne le porte. Ces deux faits
//! sont rappelés en fin de sortie pour que la comparaison soit lisible d'un seul coup d'œil, et
//! ils sont déclarés comme lus, pas comme mesurés.
//!
//!     cargo run -p water-core --release --example receveur_volume_net

use water_core::{
    gaussian_spectrum::{self, Recipe},
    impact_field::{ImpactField, Medium},
    modal_pressure::{self, Segment},
    radial_impact::{Domain, RadialImpact},
    spectral_pressure::{self, Node, Slot},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};

const LAMBDA: f32 = 4.0;
const ENERGIE_J: f32 = 0.01;

fn impact(displaced_l: f32) -> WaveEvent {
    WaveEvent::impact(Impact {
        id: 1,
        frame: FrameId(0),
        cell: 0,
        birth: SimTime(0),
        ttl_us: 10_000_000,
        position: [0.0; 3],
        energy_j: ENERGIE_J,
        wavelength_m: LAMBDA,
        direction_turns: 0.0,
        anisotropy: 0.0,
        displaced_l,
        material: 0,
        origin: Origin::Server,
        above_surface: true,
    })
    .expect("impact valide")
}

fn milieu() -> Medium {
    Medium {
        gravity: 9.81,
        density: 1025.0,
        depth: 20.0,
        max_slope: 0.1,
    }
}

fn main() -> Result<(), String> {
    let medium = milieu();

    // ── 1. `displaced_l` est-il honoré par un champ construit ? ───────────────────────────────
    //
    // Mille litres — un mètre cube — contre zéro, toutes choses égales par ailleurs. Si le champ
    // en tenait le moindre compte, `η` bougerait quelque part.
    let (vide, plein) = (impact(0.0), impact(1000.0));
    let domaine = Domain {
        radius: 16.0,
        age_us: 4_000_000,
    };
    let (ra, rb) = (
        RadialImpact::<128>::new(vide, medium, domaine).map_err(|e| format!("radial {e:?}"))?,
        RadialImpact::<128>::new(plein, medium, domaine).map_err(|e| format!("radial {e:?}"))?,
    );
    let (ia, ib) = (
        ImpactField::new(vide, medium).map_err(|e| format!("periodique {e:?}"))?,
        ImpactField::new(plein, medium).map_err(|e| format!("periodique {e:?}"))?,
    );
    let (mut ecart_radial, mut ecart_periodique, mut bits_identiques) = (0f32, 0f32, true);
    for i in 0..256 {
        let r = (i as f32 + 0.5) * (15.9 / 256.);
        for us in [0u64, 1_000_000, 4_000_000] {
            let t = SimTime(us);
            let a = ra.sample(FrameId(0), 0, [r, 0.0], t).map_err(|e| format!("{e:?}"))?.eta;
            let b = rb.sample(FrameId(0), 0, [r, 0.0], t).map_err(|e| format!("{e:?}"))?.eta;
            ecart_radial = ecart_radial.max((a - b).abs());
            bits_identiques &= a.to_bits() == b.to_bits();
            let c = ia.sample(FrameId(0), 0, [r, 0.0], t).map_err(|e| format!("{e:?}"))?.eta;
            let d = ib.sample(FrameId(0), 0, [r, 0.0], t).map_err(|e| format!("{e:?}"))?.eta;
            ecart_periodique = ecart_periodique.max((c - d).abs());
            bits_identiques &= c.to_bits() == d.to_bits();
        }
    }
    println!(
        "RECEVEUR_S312 deplace_l volume_demande_m3=1.0 ecart_radial_m={ecart_radial:e} \
         ecart_periodique_m={ecart_periodique:e} identiques_au_bit={bits_identiques}"
    );

    // ── 2a. Impact radial : volume net et volume absolu sur le disque. ────────────────────────
    //
    // Le champ est isotrope : une quadrature radiale suffit, et elle est exacte en angle.
    // `volume = ∫ 2π η r dr`, `absolu = ∫ 2π |η| r dr`. C'est le **rapport** qui dit si un net
    // nul est une propriété du champ ou la petitesse du champ.
    // Le rayon du disque **se balaye**, et c'est tout l'argument. Un disque tronque toujours un
    // champ qui n'est pas nul à son bord : chaque mode y contribue `2πR·J₁(kR)/k`, qui **oscille**
    // avec `R` au lieu de converger. Si le net oscille avec le rayon et reste sous la queue,
    // c'est la troncature ; s'il converge vers une valeur non nulle, le champ porte vraiment un
    // volume. La question ne se tranche pas sur un seul rayon — S311 n'en avait qu'un.
    for us in [0u64, 1_000_000, 4_000_000] {
        for rmax in [4.0f64, 8.0, 12.0, 15.9] {
            let n = (rmax * 256.0) as usize;
            let dr = rmax / n as f64;
            let (mut net, mut absolu, mut bord) = (0f64, 0f64, 0f32);
            for i in 0..n {
                let r = (i as f64 + 0.5) * dr;
                let eta = ra
                    .sample(FrameId(0), 0, [r as f32, 0.0], SimTime(us))
                    .map_err(|e| format!("{e:?}"))?
                    .eta as f64;
                net += core::f64::consts::TAU * eta * r * dr;
                absolu += core::f64::consts::TAU * eta.abs() * r * dr;
                if r > rmax - 1.0 {
                    bord = bord.max(eta.abs() as f32);
                }
            }
            // Borne de troncature : ce qu'un anneau d'une longueur d'onde, à l'amplitude du
            // bord, porterait à lui seul. Si `|net|` lui reste inférieur, il n'y a rien d'autre.
            let borne = core::f64::consts::TAU * rmax * bord as f64 * LAMBDA as f64;
            println!(
                "RECEVEUR_S312 impact_radial age_s={:.1} rayon_m={rmax} net_m3={net:e} \
                 absolu_m3={absolu:e} net_sur_absolu={:e} eta_au_bord_m={bord:e} \
                 borne_de_troncature_m3={borne:e} sous_la_borne={}",
                us as f64 * 1e-6,
                net.abs() / absolu,
                net.abs() <= borne
            );
        }
    }

    // ── 2b. Champ périodique : intégrale sur **une** cellule, donc exacte en analytique. ──────
    let side = ia.side();
    for us in [0u64, 1_000_000, 4_000_000] {
        let n = 256usize;
        let d = side as f64 / n as f64;
        let (mut net, mut absolu) = (0f64, 0f64);
        for j in 0..n {
            for i in 0..n {
                let p = [(i as f64 + 0.5) * d, (j as f64 + 0.5) * d];
                let eta = ia
                    .sample(FrameId(0), 0, [p[0] as f32, p[1] as f32], SimTime(us))
                    .map_err(|e| format!("{e:?}"))?
                    .eta as f64;
                net += eta * d * d;
                absolu += eta.abs() * d * d;
            }
        }
        println!(
            "RECEVEUR_S312 champ_periodique age_s={:.1} cote_m={side} net_m3={net:e} \
             absolu_m3={absolu:e} net_sur_absolu={:e}",
            us as f64 * 1e-6,
            net.abs() / absolu
        );
    }

    // ── 2c. Source de pression mobile : le sillage, assemblé par son spectre gaussien. ────────
    let recette = Recipe {
        sigma: 1.0,
        cutoff: 6.0,
        radial: 16,
        angular: 16,
    };
    let compte = recette.radial * recette.angular;
    let mut noeuds = vec![Node::default(); compte];
    let spectre =
        gaussian_spectrum::bake(recette, &mut noeuds).map_err(|e| format!("spectre {e:?}"))?;
    let k_min = spectre
        .nodes()
        .iter()
        .map(|n| (n.k[0] * n.k[0] + n.k[1] * n.k[1]).sqrt())
        .fold(f32::INFINITY, f32::min);
    let chemin = [Segment {
        birth: SimTime(0),
        duration_us: 2_000_000,
        origin: [0.0; 2],
        velocity: [2.0, 0.0],
        pressure_pa: 100.0,
    }];
    let mut slots = vec![Slot::default(); compte];
    // Même balayage, même raison : la boîte grandit à **maille constante**, pour que ce qui
    // change soit la troncature et rien d'autre.
    for demi in [20.0f32, 40.0, 80.0] {
        let champ = spectral_pressure::prepare(
            spectre.nodes(),
            &chemin,
            9.81,
            1025.0,
            SimTime(1_500_000),
            SimTime(2_000_000),
            [-demi; 2],
            [demi; 2],
            &mut slots,
        )
        .map_err(|e| format!("sillage {e:?}"))?;
        let n = (2.0 * demi / 0.25) as usize;
        let d = 2.0 * demi as f64 / n as f64;
        let (mut net, mut absolu) = (0f64, 0f64);
        for j in 0..n {
            for i in 0..n {
                let p = [
                    -demi as f64 + (i as f64 + 0.5) * d,
                    -demi as f64 + (j as f64 + 0.5) * d,
                ];
                let eta = champ
                    .sample([p[0] as f32, p[1] as f32])
                    .map_err(|e| format!("{e:?}"))?
                    .eta as f64;
                net += eta * d * d;
                absolu += eta.abs() * d * d;
            }
        }
        println!(
            "RECEVEUR_S312 sillage demi_boite_m={demi} maille_m={d:.3} net_m3={net:e} \
             absolu_m3={absolu:e} net_sur_absolu={:e}",
            net.abs() / absolu
        );
    }

    // ── 3. Le mode `k = 0` : c'est *lui*, le volume net. ──────────────────────────────────────
    //
    // `∫ η dA` sur le plan **est** l'amplitude du mode de nombre d'onde nul. Une couche qui n'a
    // pas ce mode n'a pas de volume net, et cela ne se corrige pas par un réglage.
    let k0 = modal_pressure::ModalPressure::new(
        [0.0, 0.0],
        9.81,
        1025.0,
        chemin[0],
        2_000_000,
    );
    println!(
        "RECEVEUR_S312 mode_k0 modal_pressure={:?} k_min_spectre_rad_par_m={k_min:e} \
         k_min_impact_radial_rad_par_m={:e} k_min_champ_periodique_rad_par_m={:e}",
        k0.err(),
        core::f32::consts::PI / LAMBDA,
        core::f32::consts::TAU / side
    );

    // ── Les deux autres couches, lues dans leur type. ─────────────────────────────────────────
    println!(
        "RECEVEUR_S312 lu_dans_le_type V.HydroNode.volume_ml=i64_millilitres \
         B.SeaState.champs=hs,tp,theta_turns,components,graine niveau_moyen_dans_B=absent"
    );
    Ok(())
}
