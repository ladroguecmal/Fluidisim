//! **S448 — c3 de C7d-3c : une vague qui déferle, dans la mer** ([ADR-214](../../../docs/adr/ADR-214-b-entre-dans-la-bande.md)).
//!
//! Une houle calme B (`λ` = 2 m, cambrure `ε_B`, 1 m d'eau) dans une mer `Volume3` relative de 24 m ; au milieu, une bande `Apic3` de
//! 8 m en **eau totale**, posée par le raccord conservatif (`BandInSea`, S447–S448). Dans la bande, à la phase de B, **un groupe de
//! Stokes d'ordre 3** (les formules de Chen et al. 1999, `apic3d_deferlement`) dont la cambrure monte de `ε_B` aux bords à `ε₀` au
//! centre, `ε(x) = ε_B + (ε₀ − ε_B)·e^{−((x − x_c)/σ)²}`, `σ` = 1,5 m : sa vague centrale déferle, loin des bords ; la couronne de
//! colonnes du raccord (`ANNEAU` colonnes de chaque côté) reste calme. Les particules et la bascule (`ColumnsSwitch`, ses défauts)
//! comme `apic3d_deferlement`. **Le témoin** : le même groupe dans une bande seule **deux fois plus large** (16 m), à parois — la
//! forme seule ; ses parois, loin du groupe, n'imposent pas leur vitesse normale nulle à la houle qui les touche (S448 : à 4 m du
//! centre, la projection du premier pas déplaçait le retournement de 13 %).
//!
//!     cargo run -p water-core --release --offline --example deferlement_en_mer -- [eps0] [eps_b] [durée_s]
//!
//! Publié, pour la bande dans la mer et pour la bande seule : l'instant et l'abscisse du premier retournement (eau, air, eau sur une
//! verticale de la fenêtre centrale de 4 m), la part de la fenêtre en particules à cet instant, le plus grand nombre de colonnes en
//! particules après le premier pas ; pour la mer : la masse au raccord (le volume de δ moins ce que son bilan explique).

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::apic3d::{Apic3, ColumnsSwitch};
use water_core::background::{Background, SeaState};
use water_core::band_in_sea::BandInSea;
use water_core::delta3d::{BackgroundGrid3, Domain3, Sponge3, Volume3};
use water_core::host::HostServices;
use water_core::{SimTime, WorldPos};

const G: f64 = 9.81;
const LAMBDA: f64 = 2.;
const DX: f64 = 0.1;
const PROFONDEUR: f64 = 1.;
const AIR: f64 = 0.6;
const NX_MER: usize = 240;
const NA: usize = 80;
const I0: usize = 80;
const NY: usize = 2;
const ANNEAU: usize = 4;
const MARGE: usize = 3;
const SIGMA: f64 = 1.5;
const RHO: f32 = 1000.;

struct Issue {
    retournement: Option<(f64, f64, f64)>,
    particules_max: usize,
    /// Le plus grand nombre de colonnes en particules après la première seconde (la bande initiale passée).
    apres_une_seconde: usize,
    raccord: f64,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let eps0: f64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.55);
    let eps_b: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.2);
    let duree: f64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.5);
    let mer = run(eps0, eps_b, duree, true, NA, I0)?;
    // `DEFERLEMENT_TEMOIN_ETROIT=1` : le témoin de la largeur de la bande (le premier jet de S448).
    let (na_t, i0_t) = if std::env::var("DEFERLEMENT_TEMOIN_ETROIT").is_ok() { (NA, I0) } else { (2 * NA, I0 - NA / 2) };
    let seule = run(eps0, eps_b, duree, false, na_t, i0_t)?;
    let k = std::f64::consts::TAU / LAMBDA;
    let demi = 2. * (eps_b / k) / k * (NY as f64 * DX);
    let f = |r: Option<(f64, f64, f64)>| match r {
        Some((t, x, p)) => format!("t={t:.3} x={x:.2} part_fenetre={p:.3}"),
        None => "aucun".into(),
    };
    println!(
        "DEFERLEMENT_EN_MER_S448 eps0={eps0} eps_b={eps_b} duree_s={duree} dx={DX} | mer : retournement {} particules_max={} \
         apres_1s={} raccord_m3={:.3e} part_demi_periode={:.4} | seule : retournement {} particules_max={} apres_1s={}",
        f(mer.retournement), mer.particules_max, mer.apres_une_seconde, mer.raccord, mer.raccord / demi, f(seule.retournement),
        seule.particules_max, seule.apres_une_seconde
    );
    Ok(())
}

fn run(eps0: f64, eps_b: f64, duree: f64, en_mer: bool, na: usize, i0: usize) -> Result<Issue, String> {
    let k = std::f64::consts::TAU / LAMBDA;
    let nz = ((PROFONDEUR + AIR) / DX).round() as usize;
    let dv = Domain3 { nx: NX_MER, ny: NY, nz, dx: DX as f32 };
    let da = Domain3 { nx: na, ny: NY, nz, dx: DX as f32 };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 30);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    // B : une composante de 2 m (`tp` = 2·T), cambrure `ε_B`.
    let periode = (std::f64::consts::TAU * LAMBDA / G).sqrt();
    let houle = Background::configure(
        &mut hote,
        SeaState { hs: (eps_b / k * 2. * 2f64.sqrt()) as f32, tp: (2. * periode) as f32, theta_turns: 0., components: 1, graine: 7 },
        WorldPos::from_units(0, 0, 0),
    )
    .map_err(|e| format!("houle {e:?}"))?;
    let mut grille = BackgroundGrid3::configure(&mut hote, dv, [0., 0., -PROFONDEUR as f32], RHO).map_err(|e| format!("{e:?}"))?;
    grille.sample(&houle, SimTime(0)).map_err(|e| format!("{e:?}"))?;
    // La phase de B à t = 0, lue sur la mer : `η_B = a·cos(k·x + φ)`.
    let phi = {
        let bg = grille.view().ok_or("fond")?;
        let (mut c, mut s) = (0f64, 0f64);
        for i in 0..NX_MER {
            let x = (i as f64 + 0.5) * DX;
            let e = bg.w[i].eta as f64;
            c += e * (k * x).cos();
            s += e * (k * x).sin();
        }
        (-s).atan2(c)
    };
    // La bande : ses coordonnées partent de sa colonne 0, à `i0·dx` dans la mer.
    let x0 = i0 as f64 * DX;
    let xc = x0 + na as f64 * DX / 2.;
    let eps = |x: f64| eps_b + (eps0 - eps_b) * (-((x - xc) / SIGMA).powi(2)).exp();
    let theta = |x: f64| k * x + phi;
    let eta = |x: f64| {
        let (e, t) = (eps(x), theta(x));
        PROFONDEUR + ((e + e * e * e / 8.) * t.cos() + 0.5 * e * e * (2. * t).cos() + 0.375 * e * e * e * (3. * t).cos()) / k
    };
    let vitesse = |x: f64, z: f64| {
        let e = eps(x);
        let omega = (G * k).sqrt() * (1. + 0.5 * e * e);
        let amp = e / k * omega * (k * (z - PROFONDEUR)).exp();
        let (s, c) = theta(x).sin_cos();
        (amp * c, amp * s)
    };
    let champ = |p: [f32; 3]| -> ([f32; 3], [[f32; 3]; 3]) {
        let (x, z) = (p[0] as f64 + x0, p[2] as f64);
        let (u, w) = vitesse(x, z);
        let g = [[-k * w, 0., k * u], [0.; 3], [k * u, 0., k * w]];
        ([u as f32, 0., w as f32], g.map(|r| r.map(|v| v as f32)))
    };
    let haut = PROFONDEUR + 1.6 * eps0 / k;
    let capacite = na * NY * ((haut / DX).ceil() as usize + 2) * 8 * 2;
    let mut a = Apic3::configure(&mut hote, da, RHO, G as f32, capacite).map_err(|e| format!("{e:?}"))?;
    a.seed(&|p| (p[2] as f64) < eta(p[0] as f64 + x0)).map_err(|e| format!("{e:?}"))?;
    a.set_particle_velocities(&champ).map_err(|e| format!("{e:?}"))?;
    // La zone : la couronne du raccord ; le reste en particules jusqu'à la première bascule.
    let mask: Vec<u8> = (0..na * NY).map(|c| u8::from(c % na < ANNEAU || c % na >= na - ANNEAU)).collect();
    a.enable_columns(&mut hote, &mask).map_err(|e| format!("{e:?}"))?;
    let surface: Vec<f32> = (0..na * NY).map(|c| eta((c % na) as f64 * DX + 0.5 * DX + x0) as f32).collect();
    a.set_columns_surface(&surface).map_err(|e| format!("{e:?}"))?;
    {
        let (mut u, v, mut w) = (vec![0f32; (na + 1) * NY * nz], vec![0f32; na * (NY + 1) * nz], vec![0f32; na * NY * (nz + 1)]);
        for kk in 0..nz {
            for j in 0..NY {
                for i in 0..=na {
                    u[(kk * NY + j) * (na + 1) + i] = vitesse(i as f64 * DX + x0, (kk as f64 + 0.5) * DX).0 as f32;
                }
            }
        }
        for kk in 0..=nz {
            for j in 0..NY {
                for i in 0..na {
                    w[(kk * NY + j) * na + i] = vitesse((i as f64 + 0.5) * DX + x0, kk as f64 * DX).1 as f32;
                }
            }
        }
        a.set_grid_velocities(&u, &v, &w).map_err(|e| format!("{e:?}"))?;
    }
    let mut bascule = ColumnsSwitch::with_capacity(&mut hote, a.domain()).map_err(|e| format!("{e:?}"))?;
    let mut mer = Volume3::configure(&mut hote, dv, RHO, G as f32).map_err(|e| format!("{e:?}"))?;
    mer.set_free_surface(&vec![PROFONDEUR as f32; dv.columns()], PROFONDEUR as f32).map_err(|e| format!("{e:?}"))?;
    let eponge = Sponge3 { width_x: 4., width_y: 0., rate_per_s: 2. };
    let mut raccord = if en_mer {
        a.enable_open_boundaries(&mut hote).map_err(|e| format!("{e:?}"))?;
        Some(BandInSea::configure(&mut hote, dv, da, i0, MARGE).map_err(|e| format!("{e:?}"))?)
    } else {
        None
    };

    // La fenêtre centrale de 4 m.
    let (f0, f1) = (na / 4, 3 * na / 4);
    let mut occupation = vec![0u32; na * NY * nz];
    let (mut t_us, mut pas) = (0u64, 0u64);
    let (mut retournement, mut particules_max, mut apres_une_seconde) = (None, 0usize, 0usize);
    let (mut apports, mut raccord_max) = (0f64, 0f64);
    let volume = |m: &Volume3| m.surface().iter().map(|e| (*e - PROFONDEUR as f32) as f64).sum::<f64>() * DX * DX;
    while (t_us as f64) * 1e-6 < duree {
        let us = a.stable_step_us(10_000);
        let t = SimTime(t_us);
        grille.sample(&houle, t).map_err(|e| format!("{e:?}"))?;
        let bg = grille.view().ok_or("fond")?;
        if let Some(r) = raccord.as_mut() {
            if pas > 0 {
                r.feed_sea(&mut mer, &mut a, &bg).map_err(|e| format!("raccord, pas {pas} : {e:?}"))?;
            }
            r.feed_band(&mer, &mut a, &bg).map_err(|e| format!("{e:?}"))?;
        }
        a.step(us).map_err(|e| format!("bande, pas {pas} : {e:?}"))?;
        if en_mer {
            mer.step_perturbation_mobile(t, us, 4000, &bg, eponge, &jobs).map_err(|e| format!("mer, pas {pas} : {e:?}"))?;
            let b = mer.balance();
            apports += b.band_in + b.perturbation_in - b.sponge_out;
            raccord_max = raccord_max.max((volume(&mer) - apports).abs());
        }
        t_us += us;
        pas += 1;
        bascule.switch(t_us, &mut a).map_err(|e| format!("bascule {e:?}"))?;
        if pas == 1 {
            bascule.clear_counts();
            continue;
        }
        let en_particules = (0..na * NY).filter(|&c| !a.is_column(c % na, c / na)).count();
        particules_max = particules_max.max(en_particules);
        if t_us >= 1_000_000 {
            apres_une_seconde = apres_une_seconde.max(en_particules);
        }
        if retournement.is_some() {
            continue;
        }
        occupation.fill(0);
        for p in a.particles() {
            let f = |x: f32, m: usize| ((x as f64 / DX).max(0.) as usize).min(m - 1);
            occupation[(f(p[2], nz) * NY + f(p[1], NY)) * na + f(p[0], na)] += 1;
        }
        let surf = a.columns_surface().ok_or("zone")?.to_vec();
        let colonne = |i: usize, j: usize, kk: usize| a.is_column(i, j) && ((kk as f64 + 0.5) * DX) < surf[j * na + i] as f64;
        let eau = |i: usize, j: usize, kk: usize| occupation[(kk * NY + j) * na + i] >= 2 || colonne(i, j, kk);
        let air = |i: usize, j: usize, kk: usize| occupation[(kk * NY + j) * na + i] == 0 && !colonne(i, j, kk);
        'cherche: for j in 0..NY {
            for i in f0..f1 {
                let mut etat = 0;
                for kk in 0..nz {
                    etat = match (etat, eau(i, j, kk), air(i, j, kk)) {
                        (0, true, _) => 1,
                        (1, _, true) => 2,
                        (2, true, _) => 3,
                        (e, _, _) => e,
                    };
                }
                if etat == 3 {
                    let part = (0..NY).map(|j2| (f0..f1).filter(|&i2| !a.is_column(i2, j2)).count()).sum::<usize>() as f64
                        / ((f1 - f0) * NY) as f64;
                    retournement = Some((t_us as f64 * 1e-6, (i as f64 + 0.5) * DX + x0, part));
                    break 'cherche;
                }
            }
        }
    }
    Ok(Issue { retournement, particules_max, apres_une_seconde, raccord: raccord_max })
}
