//! S278 — **le banc de l'ordonnanceur** : ce qui décide qu'une zone est simulée, éprouvé sur un
//! cas où la décision change.
//!
//! Jusqu'ici, l'activation des domaines était une conception (ADR-006, ADR-012, ADR-013) que
//! personne n'avait exécutée. Un scénario suffit à savoir si elle tient : un bateau longe cinq
//! îlots serrés, chacun demande son domaine quand le bateau approche, et le budget n'en finance
//! jamais plus de deux à la fois.
//!
//! Ce que le banc doit montrer, et ce que personne ne pouvait affirmer avant lui :
//!
//! 1. **le budget n'est jamais dépassé** — la propriété qu'ADR-012 §1 demande de défendre avant
//!    toutes les autres, puisque c'est elle qui ferme le chemin vers un pic d'image ;
//! 2. **aucun domaine ne bat** — deux transitions par îlot, une pour naître, une pour mourir ;
//! 3. **aucun ne meurt avant sa durée de vie** de 750 ms (ADR-013 §5) ;
//! 4. **la décision est reproductible** — même empreinte à deux exécutions (I-03).
//!
//! Les poids sont ceux d'ADR-012 §2, bornés selon ADR-170. Ils sont **déclarés ici**, comme le
//! ferait un hôte : le cœur ne les calcule pas.

#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;

use water_core::{
    scheduler::{Bid, DomainId, Profile, Regime, Scheduler, LIFETIME_US},
    Hasher64, HostServices, SimTime,
};

/// 30 Hz pendant 40 s — la cadence de tick d'ADR-012 §3.
const TICK_US: u64 = 33_333;
const TICKS: u64 = 1_200;
/// Le bateau va de −300 m à +300 m le long de `x`, à 15 m/s.
const SPEED: f64 = 15.;
const START_X: f64 = -300.;
/// Les îlots, en mètres le long du même axe. **Ils sont serrés exprès** : espacés, le bateau ne
/// les rencontre qu'un par un, et le budget n'est jamais mis à l'épreuve — un banc qui ne peut pas
/// échouer ne prouve rien. Ici cinq zones se disputent de quoi en financer deux.
const ISLETS: [f64; 5] = [-40., -20., 0., 20., 40.];
/// Distance à laquelle un îlot occupe la moitié de ce qu'il occuperait collé à l'œil.
const HALF_SCREEN_M: f64 = 120.;
/// En deçà, le bateau interagit avec l'îlot : c'est du gameplay, et ça domine.
const CONTACT_M: f64 = 40.;
/// Poids de gameplay hors contact, et urgence d'un îlot que le bateau a déjà dépassé : l'eau
/// qu'il a remuée reste visible derrière lui, elle ne cesse pas d'exister quand il tourne le dos.
const GAMEPLAY_FAR: f64 = 0.8;
const URGENCY_BEHIND: f64 = 0.7;
/// Horizon de normalisation de `W_urgence` (ADR-170).
const HORIZON_S: f64 = 4.;
/// Coût annoncé par domaine, et budget du profil : deux domaines tiennent, pas trois.
const COST_MS: f32 = 0.8;
const BUDGET_MS: f32 = 2.0;

/// Ce qu'un hôte publierait pour un îlot : les trois poids d'ADR-012 §2, tous dans `[0,1]`.
fn weights(boat_x: f64, islet_x: f64) -> (f32, f32, f32) {
    let d = (boat_x - islet_x).abs();
    // Surface écran : décroît comme l'inverse du carré de la distance, jamais la distance nue
    // (ADR-012 §2 — à budget égal, la distance sur-sert le proche insignifiant).
    let perception = 1. / (1. + (d / HALF_SCREEN_M).powi(2));
    let gameplay = if d < CONTACT_M { 1. } else { GAMEPLAY_FAR };
    // Temps avant que l'absence du domaine devienne visible : le bateau s'approche-t-il ?
    let closing = boat_x < islet_x;
    let urgency = if closing {
        (HORIZON_S / ((islet_x - boat_x) / SPEED).max(1e-3)).min(1.)
    } else {
        // Il s'éloigne : plus rien ne presse, mais le sillage ne disparaît pas avec lui.
        URGENCY_BEHIND
    };
    (gameplay as f32, perception as f32, urgency as f32)
}

/// Une exécution complète. Rend l'empreinte des décisions, les transitions par îlot, le budget
/// maximal distribué, la plus courte vie observée, et combien de domaines ont voulu vivre en même
/// temps — sans ce dernier, on ne sait pas si le budget a seulement été mis à l'épreuve.
fn run() -> (u64, [u32; ISLETS.len()], f32, u64, usize) {
    let mut alloc = host_impl::ArenaAllocator::with_capacity(1 << 16);
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut host = HostServices { alloc: &mut alloc, jobs: &jobs, sink: &sink };
    let profile = Profile::default_thresholds(BUDGET_MS, 64);
    let mut scheduler = Scheduler::with_capacity(&mut host, profile, ISLETS.len()).expect("ordonnanceur");

    let mut hash = Hasher64::new();
    let mut transitions = [0u32; ISLETS.len()];
    let mut was_active = [false; ISLETS.len()];
    let mut born_us = [0u64; ISLETS.len()];
    let (mut worst_ms, mut shortest_us, mut most_active) = (0f32, u64::MAX, 0usize);

    for tick in 0..TICKS {
        let now_us = tick * TICK_US;
        let boat_x = START_X + SPEED * (now_us as f64 * 1e-6);
        scheduler.begin();
        for (i, islet_x) in ISLETS.iter().enumerate() {
            let (gameplay, perception, urgency) = weights(boat_x, *islet_x);
            scheduler
                .submit(Bid {
                    id: DomainId(i as u32),
                    gameplay,
                    perception,
                    urgency,
                    cost_ms: COST_MS,
                    blocks: 8,
                    regime: Regime::Perturbative,
                })
                .expect("soumission");
        }
        scheduler.decide(SimTime(now_us)).expect("décision");
        scheduler.allocate();

        worst_ms = worst_ms.max(scheduler.granted_ms());
        most_active = most_active.max(scheduler.active().count());
        for i in 0..ISLETS.len() {
            let active = scheduler.is_active(DomainId(i as u32));
            if active != was_active[i] {
                transitions[i] += 1;
                if active {
                    born_us[i] = now_us;
                } else {
                    shortest_us = shortest_us.min(now_us - born_us[i]);
                }
                was_active[i] = active;
            }
            hash.write_u8(active as u8);
        }
        for g in scheduler.grants() {
            hash.write_u32(g.id.0);
            hash.write_u32(g.budget_ms.to_bits());
        }
    }
    (hash.finish(), transitions, worst_ms, shortest_us, most_active)
}

fn main() {
    let (hash, transitions, worst_ms, shortest_us, most_active) = run();
    let (again, ..) = run();

    println!(
        "ORDONNANCEUR_S278 ticks={TICKS} ilots={} budget_ms={BUDGET_MS} cout_par_domaine_ms={COST_MS} \
         budget_max_distribue_ms={worst_ms:.3} vivants_simultanes_max={most_active} transitions={transitions:?} \
         plus_courte_vie_ms={:.0} empreinte={hash:016x} reproductible={}",
        ISLETS.len(),
        shortest_us as f64 * 1e-3,
        hash == again
    );

    assert!(worst_ms <= BUDGET_MS, "budget dépassé : {worst_ms} ms");
    // Un banc qui ne peut pas échouer ne prouve rien : si la demande simultanée tient dans le
    // budget, le sac à dos n'a jamais été sollicité et la propriété 1 n'est pas éprouvée.
    assert!(
        most_active as f32 * COST_MS > BUDGET_MS,
        "le budget n'a jamais été disputé : {most_active} domaines à {COST_MS} ms tiennent dans {BUDGET_MS} ms"
    );
    assert!(
        transitions.iter().all(|t| *t == 2),
        "un domaine bat : {transitions:?} — attendu deux transitions par îlot"
    );
    assert!(
        shortest_us >= LIFETIME_US,
        "un domaine est mort avant sa durée de vie : {shortest_us} µs"
    );
    assert_eq!(hash, again, "la décision n'est pas reproductible");
    println!("ORDONNANCEUR_S278 recu=oui");
}
