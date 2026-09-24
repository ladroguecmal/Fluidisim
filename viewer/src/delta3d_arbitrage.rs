//! S344, **porte A**, premier critère — *plusieurs candidats réels se disputent un budget*. Deux domaines δ 3D de production, la scène de la porte B posée deux fois, à 60 m l'un de
//! l'autre ; une caméra qui passe de l'un à l'autre. À chaque image, chacun soumissionne sa part d'écran
//! (`W_perception`, ADR-012 §2) et son coût **mesuré** ; l'ordonnanceur de S278 décide et alloue.
//!
//! Rien ici n'est une grandeur de jeu (I-04) ; rien n'est sérialisé (I-17).
use crate::delta3d_scene::{Config, G, RHO};
use crate::delta3d_step::Step3;
use crate::scene::host_impl;
use crate::scene::Camera;
use water_core::host::HostServices;
use water_core::scheduler::{Bid, DomainId, Profile, Regime, Scheduler};
use water_core::SimTime;

/// Pas de l'image et de δ, comme la scène de la porte B.
const FRAME_US: u64 = 16_667;
/// Le budget du banc : il tient un domaine à 3,7 ms (S343), pas deux. Ce n'est pas le profil du produit
/// (δ ≤ 2 ms, ADR-174 D3), que la porte C poursuit : c'est la contrainte qui oblige à choisir.
const BUDGET_MS: f32 = 5.;
/// Seuils calibrés en S344 P2 sur la part d'écran d'un domaine vu de face depuis la pose de la scène
/// (0,1629) : allumage 0,61 fois, extinction 0,31 fois (ADR-171 : les seuils appartiennent au profil).
const ALLUMAGE: f32 = 0.10;
const EXTINCTION: f32 = 0.05;
/// Premier coût annoncé d'un domaine qui n'a encore rien payé : le pas mesuré en S343. Ensuite, la
/// médiane de ses huit derniers pas payés, horodatés (ADR-012 §3).
const PREMIER_COUT_MS: f32 = 3.7;
const ECHANTILLONS: usize = 8;

/// Durée du trajet de caméra.
const SECONDES: f64 = 20.;
/// Décalage du second domaine en `x`, mètres.
const ECART_X: f32 = 60.;

/// Le trajet : l'œil **longe la côte**, comme un joueur — devant A (0–5 s), vers B à 15 m/s (5–9 s),
/// devant B (9–13 s), retour (13–17 s), devant A (17–20 s). Le regard reste celui de la scène de la
/// porte B. Tourner seulement la tête ne suffit pas : vu de devant A, B reste plus petit à l'écran que
/// A (S344 P2) — c'est la surface à l'écran qui décide, pas la direction du regard (ADR-012 §2).
pub fn camera(t: f64) -> Camera {
    let base = Camera::default();
    let f = |a: f64, b: f64| ((t - a) / (b - a)).clamp(0., 1.) as f32;
    let x = if t < 11. { ECART_X * f(5., 9.) } else { ECART_X * (1. - f(13., 17.)) };
    Camera { eye: [base.eye[0] + x, base.eye[1], base.eye[2]], ..base }
}

/// La projection de l'afficheur pour cette caméra, cadre 16/9 (S279).
pub fn projection(c: &Camera) -> crate::lod::Projection {
    let [forward, right, up] = c.vectors();
    crate::lod::Projection { eye: c.eye, forward, right, up, tan_half: (50.0f32.to_radians() / 2.).tan(), aspect: 16. / 9., far: 1500. }
}

/// Emprise au niveau de l'eau d'un domaine : coin bas et coin haut, mètres.
pub fn emprise(config: &Config) -> ([f32; 2], [f32; 2]) {
    let d = config.domain;
    let min = [config.origin[0], config.origin[1]];
    (min, [min[0] + d.nx as f32 * d.dx, min[1] + d.ny as f32 * d.dx])
}

/// Les deux domaines : la scène de la porte B, et la même décalée de 60 m en `x`. Sans paquet : leur
/// contenu ne décide de rien ici.
pub fn domaines() -> [Config; 2] {
    let a = Config::review().without_packet();
    let mut b = a;
    b.origin[0] += ECART_X;
    [a, b]
}

/// **P2 — les parts d'écran le long du trajet**, toutes les 0,25 s : ce sur quoi les seuils se calibrent.
/// `--delta3d-parts`.
pub fn parts() -> Result<(), String> {
    let doms = domaines();
    let emprises = [emprise(&doms[0]), emprise(&doms[1])];
    let mut t = 0.;
    while t <= SECONDES + 1e-9 {
        let c = camera(t);
        let p = projection(&c);
        let (a, b) = (p.screen_fraction(emprises[0].0, emprises[0].1), p.screen_fraction(emprises[1].0, emprises[1].1));
        println!("DELTA3D_PARTS_S344 t={t:.2} oeil_x={:.1} part_a={a:.4} part_b={b:.4}", c.eye[0]);
        t += 0.25;
    }
    Ok(())
}

/// **P3 — le banc d'arbitrage**, `--delta3d-arbitrage`. Deux pas de production réels, l'ordonnanceur de
/// S278, un budget de 5 ms. À chaque image : parts d'écran, soumissions, décision, allocation ; chaque
/// domaine accordé fait un pas horodaté, qui nourrit son coût. Un domaine qui (re)naît repart de δ = 0
/// (I-12). Publiés : transitions, budget accordé au pire, images « vivant mais affamé », images où le
/// domaine le plus visible n'est pas servi, coûts par domaine.
pub fn arbitrage() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let doms = domaines();
        let emprises = [emprise(&doms[0]), emprise(&doms[1])];
        let mut cartes = Vec::new();
        let mut etats = Vec::new();
        for c in &doms {
            let carte = Step3::new(background, c.domain, c.origin, RHO, G).await?;
            carte.set_step(c.step_us, c.rest, c.sponge)?;
            let etat = c.initial_state();
            carte.set_state(&etat.0, &etat.1, &etat.2, &etat.3)?;
            cartes.push(carte);
            etats.push(etat);
        }
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 12);
        let profil = Profile { cpu_sim_ms: BUDGET_MS, blocks: 2, on: ALLUMAGE, off: EXTINCTION };
        let mut ordonnanceur = Scheduler::with_capacity(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, profil, 2)
            .map_err(|e| format!("ordonnanceur : {e:?}"))?;
        println!(
            "DELTA3D_ARBITRAGE_S344 carte={:?} domaines=2 ecart_x={ECART_X} budget_ms={BUDGET_MS} allumage={ALLUMAGE} extinction={EXTINCTION} premier_cout_ms={PREMIER_COUT_MS} cycles={}",
            cartes[0].adapter, doms[0].cycles
        );
        let mediane = |v: &Vec<f64>| -> f32 {
            if v.is_empty() {
                return PREMIER_COUT_MS;
            }
            let mut w = v.clone();
            w.sort_by(|a, b| a.partial_cmp(b).unwrap());
            w[w.len() / 2] as f32
        };
        let mut couts: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        let mut tous: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        // L'oubli de S286 (L336) : un domaine non servi oublie un échantillon par image de temps de scène, le
        // plus ancien d'abord, et retombe sur le premier coût annoncé. Sans lui, un seul pas cher l'exclut
        // pour toujours — le premier essai de S344 l'a reproduit en 3D.
        let mut oubli_depuis: [Option<u64>; 2] = [None; 2];
        let (mut vivant_avant, mut accorde_avant) = ([false; 2], [false; 2]);
        let (mut pire_accorde, mut affame, mut mal_servi, mut pas_payes) = (0f32, [0usize; 2], 0usize, [0usize; 2]);
        let mut mal_servi_debut: Option<f64> = None;
        let images = (SECONDES * 1e6 / FRAME_US as f64) as u64;
        for n in 0..=images {
            let t = n as f64 * FRAME_US as f64 * 1e-6;
            let vue = projection(&camera(t));
            let parts = [0, 1].map(|d| vue.screen_fraction(emprises[d].0, emprises[d].1));
            ordonnanceur.begin();
            for d in 0..2 {
                let bid = Bid { id: DomainId(d as u32), gameplay: 1., perception: parts[d], urgency: 1.,
                    cost_ms: mediane(&couts[d]), blocks: 1, regime: Regime::Perturbative };
                ordonnanceur.submit(bid).map_err(|e| format!("soumission : {e:?}"))?;
            }
            ordonnanceur.decide(SimTime(n * FRAME_US)).map_err(|e| format!("décision : {e:?}"))?;
            ordonnanceur.allocate();
            pire_accorde = pire_accorde.max(ordonnanceur.granted_ms());
            let accordes: Vec<u32> = ordonnanceur.grants().iter().map(|g| g.id.0).collect();
            for d in 0..2 {
                let vivant = ordonnanceur.is_active(DomainId(d as u32));
                let accorde = accordes.contains(&(d as u32));
                if vivant != vivant_avant[d] || accorde != accorde_avant[d] {
                    println!(
                        "DELTA3D_ARBITRAGE_S344 t={t:.3} domaine={} vivant={vivant} accorde={accorde} part={:.4} cout_annonce_ms={:.3} accorde_total_ms={:.3}",
                        ["A", "B"][d], parts[d], mediane(&couts[d]), ordonnanceur.granted_ms()
                    );
                }
                if vivant && !vivant_avant[d] {
                    // Naissance : un domaine perturbatif renaît à δ = 0 (I-12) ; son coût se réapprend.
                    let e = &etats[d];
                    cartes[d].set_state(&e.0, &e.1, &e.2, &e.3)?;
                }
                if vivant && !accorde {
                    affame[d] += 1;
                }
                let now = n * FRAME_US;
                match oubli_depuis[d] {
                    Some(depuis) if !accorde && now >= depuis => {
                        let images_ = ((now - depuis) / FRAME_US) as usize;
                        let k = images_.min(couts[d].len());
                        couts[d].drain(..k);
                        oubli_depuis[d] = Some(depuis + images_ as u64 * FRAME_US);
                    }
                    _ => oubli_depuis[d] = Some(now),
                }
                if accorde {
                    cartes[d].publish_time(background, SimTime(n * FRAME_US))?;
                    if let Some(ms) = cartes[d].timed_step(doms[d].cycles)? {
                        couts[d].push(ms);
                        tous[d].push(ms);
                        if couts[d].len() > ECHANTILLONS {
                            couts[d].remove(0);
                        }
                    }
                    pas_payes[d] += 1;
                }
                vivant_avant[d] = vivant;
                accorde_avant[d] = accorde;
            }
            // Le plus visible est-il servi ? Seulement quand l'écart de part est net.
            let plus = if parts[0] > parts[1] { 0 } else { 1 };
            let net = (parts[0] - parts[1]).abs() > 0.01;
            if net && !accordes.contains(&(plus as u32)) {
                mal_servi += 1;
                mal_servi_debut.get_or_insert(t);
            } else if let Some(debut) = mal_servi_debut.take() {
                println!("DELTA3D_ARBITRAGE_S344 plus_visible_non_servi de={debut:.3} a={t:.3} duree_s={:.3}", t - debut);
            }
        }
        for d in 0..2 {
            let mut v = tous[d].clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let q = |f: f64| if v.is_empty() { f64::NAN } else { v[(((v.len() - 1) as f64) * f).round() as usize] };
            println!(
                "DELTA3D_ARBITRAGE_S344 bilan domaine={} pas_payes={} images_affame={} cout_mediane_ms={:.3} cout_q99_ms={:.3}",
                ["A", "B"][d], pas_payes[d], affame[d], q(0.5), q(0.99)
            );
        }
        println!(
            "DELTA3D_ARBITRAGE_S344 bilan images={} budget_ms={BUDGET_MS} accorde_pire_ms={pire_accorde:.3} images_plus_visible_non_servi={mal_servi}",
            images + 1
        );
        Ok(())
    })
}

/// S349, **porte A, critère 2 — un domaine qui se déplace** au lieu d'être allumé ou éteint. La côte de S344, la même
/// caméra, le même ordonnanceur et le même budget ; mais **un seul domaine**, qui se décale vers le point regardé —
/// son centre sous l'œil en `x` —, au plus deux mailles par image. Publiés : naissances et extinctions, décalages,
/// part d'écran, colonnes hors bornes, coût du pas et du décalage. `--delta3d-suivi`.
pub fn suivi() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = domaines()[0];
        let dx = config.domain.dx;
        let mut carte = Step3::new(background, config.domain, config.origin, RHO, G).await?;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        let etat = config.initial_state();
        carte.set_state(&etat.0, &etat.1, &etat.2, &etat.3)?;
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 12);
        let profil = Profile { cpu_sim_ms: BUDGET_MS, blocks: 1, on: ALLUMAGE, off: EXTINCTION };
        let mut ordonnanceur = Scheduler::with_capacity(&mut HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink }, profil, 1)
            .map_err(|e| format!("ordonnanceur : {e:?}"))?;
        let largeur = config.domain.nx as f32 * dx;
        println!("DELTA3D_SUIVI_S349 carte={:?} budget_ms={BUDGET_MS} allumage={ALLUMAGE} extinction={EXTINCTION} decalage_max_par_image=2", carte.adapter);
        let mut couts: Vec<f64> = Vec::new();
        let (mut vivant_avant, mut naissances, mut extinctions) = (false, 0usize, 0usize);
        let (mut decalages, mut mailles, mut hors, mut part_min_regime) = (0usize, 0i64, 0u32, f32::INFINITY);
        let mut duree_decalage: Vec<f64> = Vec::new();
        let mut oubli_depuis: Option<u64> = None;
        let images = (SECONDES * 1e6 / FRAME_US as f64) as u64;
        for n in 0..=images {
            let t = n as f64 * FRAME_US as f64 * 1e-6;
            let cam = camera(t);
            let vue = projection(&cam);
            // Le décalage vers le point regardé : le centre du domaine sous l'œil, en mailles entières.
            let o = carte.origin();
            let cible = cam.eye[0] - largeur / 2.;
            let di = (((cible - o[0]) / dx).round() as i32).clamp(-2, 2);
            if di != 0 {
                let debut = std::time::Instant::now();
                carte.shift(di, 0)?;
                carte.wait()?;
                duree_decalage.push(debut.elapsed().as_secs_f64() * 1e3);
                decalages += 1;
                mailles += di.unsigned_abs() as i64;
            }
            let o = carte.origin();
            let part = vue.screen_fraction([o[0], o[1]], [o[0] + largeur, o[1] + config.domain.ny as f32 * dx]);
            if t > 1. {
                part_min_regime = part_min_regime.min(part);
            }
            ordonnanceur.begin();
            let cout = if couts.is_empty() { PREMIER_COUT_MS } else {
                let mut w = couts.clone();
                w.sort_by(|a, b| a.partial_cmp(b).unwrap());
                w[w.len() / 2] as f32
            };
            ordonnanceur.submit(Bid { id: DomainId(0), gameplay: 1., perception: part, urgency: 1., cost_ms: cout, blocks: 1, regime: Regime::Perturbative })
                .map_err(|e| format!("soumission : {e:?}"))?;
            ordonnanceur.decide(SimTime(n * FRAME_US)).map_err(|e| format!("décision : {e:?}"))?;
            ordonnanceur.allocate();
            let vivant = ordonnanceur.is_active(DomainId(0));
            let accorde = !ordonnanceur.grants().is_empty();
            if vivant && !vivant_avant { naissances += 1; }
            if !vivant && vivant_avant { extinctions += 1; }
            vivant_avant = vivant;
            let now = n * FRAME_US;
            match oubli_depuis {
                Some(depuis) if !accorde && now >= depuis => {
                    let k = (((now - depuis) / FRAME_US) as usize).min(couts.len());
                    couts.drain(..k);
                    oubli_depuis = Some(depuis + ((now - depuis) / FRAME_US) * FRAME_US);
                }
                _ => oubli_depuis = Some(now),
            }
            if accorde {
                carte.publish_time(background, SimTime(n * FRAME_US))?;
                if let Some(ms) = carte.timed_step(config.cycles)? {
                    couts.push(ms);
                    if couts.len() > ECHANTILLONS { couts.remove(0); }
                }
                hors = hors.max(carte.diagnostics_now()?.columns_outside);
            }
            if n % 30 == 0 {
                println!(
                    "DELTA3D_SUIVI_S349 t={t:.2} oeil_x={:.2} origine_x={:.2} part={part:.4} vivant={vivant} accorde={accorde} decalages={decalages}",
                    cam.eye[0], o[0]
                );
            }
        }
        duree_decalage.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q = |v: &Vec<f64>, f: f64| if v.is_empty() { f64::NAN } else { v[(((v.len() - 1) as f64) * f).round() as usize] };
        couts.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "DELTA3D_SUIVI_S349 bilan images={} naissances={naissances} extinctions={extinctions} decalages={decalages} mailles_parcourues={mailles} part_min_apres_1s={part_min_regime:.4} hors_bornes_max={hors} decalage_mediane_ms={:.3} decalage_max_ms={:.3}",
            images + 1, q(&duree_decalage, 0.5), q(&duree_decalage, 1.)
        );
        Ok(())
    })
}

