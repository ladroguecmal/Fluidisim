//! S161 — B4, premier volet : à partir de quel rapport d'amplitude l'addition cesse d'être vraie ?
//!
//! ADR-001 décompose l'eau en couches **additives** et pose un critère de bascule « à calibrer » :
//! `max|δ| > 0,35·Hs`. B4 est le banc qui doit le calibrer — ou infirmer la décomposition. Il
//! demande une **référence substitutive intégrale**, c'est-à-dire un solveur qui calcule le champ
//! total sans décomposition.
//!
//! Le dépôt en a un, et un seul qui convienne : `shallow.rs`, Saint-Venant 1D **non linéaire**.
//! Un solveur linéaire — `dispersif.rs` — ne pourrait pas servir : la superposition y est vraie
//! par construction, et l'écart mesuré serait nul quel que soit le rapport.
//!
//! Ce que ce volet peut rendre est une **infirmation** : si `simuler(A) + simuler(B)` s'écarte de
//! `simuler(A et B)`, l'additivité a un domaine de validité et la mesure en donne une borne dans
//! ce régime. Il ne peut pas valider : ni le 2D dispersif, ni les forces sur coque, ni la
//! visibilité ne sont ici.
//!
//! `cargo run -p water-core --release --example additivite_b4`
#[path = "../../water-harness/src/host_impl.rs"]
mod host_impl;
use water_core::{shallow::Shallow1D, HostServices};

const N: usize = 800;
const DX: f64 = 0.25;
const CFL: f64 = 0.45;
/// Positions et largeurs : `A` est l'onde de fond — large — et `B` la perturbation locale.
const XA: f64 = 60.0;
const SA: f64 = 6.0;
const XB: f64 = 100.0;
const SB: f64 = 3.0;

/// Élévation `eta = h - h0` après `t_fin` secondes, pour une somme de gaussiennes donnée.
fn simuler(h0: f64, bosses: &[(f64, f64, f64)], t_fin: f64) -> Vec<f64> {
    let mut allocator = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut s = Shallow1D::configure_bosses(
        &mut HostServices {
            alloc: &mut allocator,
            jobs: &jobs,
            sink: &sink,
        },
        N,
        DX,
        h0,
        bosses,
    )
    .expect("configuration");
    s.regler_ordre2(true);
    s.regler_rk2(true);
    s.avancer_jusqu_a(t_fin, CFL);
    (0..N).map(|i| s.hauteur(i) - h0).collect()
}

/// Même chose à **pas de temps imposé**. Le pas CFL dépend de la hauteur maximale, donc de la
/// présence de `B` : les trois simulations d'un même point n'avancent pas par la même suite de
/// pas, et leur écart contient une part purement numérique. Ce contrôle l'élimine.
fn simuler_pas_fixe(h0: f64, bosses: &[(f64, f64, f64)], t_fin: f64, dt: f64) -> Vec<f64> {
    let mut allocator = host_impl::ArenaAllocator::with_capacity(1 << 22);
    let jobs = host_impl::SequentialJobs;
    let sink = host_impl::StderrSink;
    let mut s = Shallow1D::configure_bosses(
        &mut HostServices {
            alloc: &mut allocator,
            jobs: &jobs,
            sink: &sink,
        },
        N,
        DX,
        h0,
        bosses,
    )
    .expect("configuration");
    s.regler_ordre2(true);
    s.regler_rk2(true);
    let pas = (t_fin / dt).ceil() as u64;
    for _ in 0..pas {
        s.pas(dt);
    }
    (0..N).map(|i| s.hauteur(i) - h0).collect()
}

fn ecart_pas_fixe(h0: f64, a_b: f64, a_d: f64, t_fin: f64, dt: f64) -> f64 {
    let seul_a = simuler_pas_fixe(h0, &[(a_b, XA, SA)], t_fin, dt);
    let seul_b = simuler_pas_fixe(h0, &[(a_d, XB, SB)], t_fin, dt);
    let ensemble = simuler_pas_fixe(h0, &[(a_b, XA, SA), (a_d, XB, SB)], t_fin, dt);
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..N {
        let somme = seul_a[i] + seul_b[i];
        num += (somme - ensemble[i]) * (somme - ensemble[i]);
        den += ensemble[i] * ensemble[i];
    }
    (num / den.max(f64::MIN_POSITIVE)).sqrt()
}

/// Écart quadratique relatif entre l'addition des deux champs et la simulation conjointe.
fn ecart(h0: f64, a_b: f64, a_d: f64, t_fin: f64) -> f64 {
    let seul_a = simuler(h0, &[(a_b, XA, SA)], t_fin);
    let seul_b = simuler(h0, &[(a_d, XB, SB)], t_fin);
    let ensemble = simuler(h0, &[(a_b, XA, SA), (a_d, XB, SB)], t_fin);
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..N {
        let somme = seul_a[i] + seul_b[i];
        num += (somme - ensemble[i]) * (somme - ensemble[i]);
        den += ensemble[i] * ensemble[i];
    }
    (num / den.max(f64::MIN_POSITIVE)).sqrt()
}

fn main() {
    println!("# S161 — B4, volet 1D : quand l'addition cesse-t-elle d'être vraie ?");
    println!();
    println!("Saint-Venant 1D non linéaire, {N} cellules de {DX} m, ordre 2 + RK2, CFL {CFL}.");
    println!("`A` : onde de fond, gaussienne sigma {SA} m en x = {XA} m.");
    println!("`B` : perturbation locale, sigma {SB} m en x = {XB} m.");
    println!("Écart = || (eta_A + eta_B) − eta_AB || / || eta_AB ||, en norme L2 sur le domaine.");
    println!();
    println!("## Écart d'additivité en fonction du rapport d'amplitude");
    println!();
    println!("Deux amplitudes de fond, pour que le rapport ne soit pas confondu avec l'amplitude");
    println!("absolue : dans Saint-Venant, la non-linéarité dépend aussi de `a/h0` (L235).");
    println!();
    let mut collecte: Vec<(f64, f64, f64, f64)> = Vec::new();
    for (h0, a_b) in [(1.0f64, 0.02f64), (1.0, 0.10), (1.0, 0.30), (1.0, 0.50)] {
        println!("### `h0` = {h0} m, `A_B` = {a_b} m — soit `A_B/h0` = {:.2}", a_b / h0);
        println!();
        println!("| `A_δ/A_B` | `A_δ/h0` | écart à 4 s | à 8 s | à 12 s |");
        println!("|---:|---:|---:|---:|---:|");
        for rapport in [0.05f64, 0.10, 0.20, 0.35, 0.50, 0.75, 1.00] {
            let a_d = rapport * a_b;
            let e: Vec<f64> = [4.0f64, 8.0, 12.0]
                .iter()
                .map(|t| ecart(h0, a_b, a_d, *t))
                .collect();
            println!(
                "| {rapport:.2} | {:.4} | {:.2e} | {:.2e} | {:.2e} |",
                a_d / h0,
                e[0],
                e[1],
                e[2]
            );
            collecte.push((a_b / h0, rapport, a_d / h0, e[1]));
        }
        println!();
    }
    println!("## Le rapport à Hs ne gouverne rien ; l'amplitude rapportée à la profondeur, si");
    println!();
    println!("Même écart, mêmes `A_δ/h0`, rapports `A_δ/A_B` différents d'un facteur cinq : si le");
    println!("critère d'ADR-001 était le bon paramètre, ces lignes ne se ressembleraient pas.");
    println!();
    println!("| `A_δ/h0` | `A_B/h0` | `A_δ/A_B` | écart à 8 s | écart / (`A_δ/h0`) |");
    println!("|---:|---:|---:|---:|---:|");
    let mut tri = collecte.clone();
    tri.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for (fond, rapport, delta, e) in &tri {
        println!(
            "| {delta:.4} | {fond:.2} | {rapport:.2} | {e:.2e} | {:.3} |",
            e / delta
        );
    }
    println!();
    println!("## Contrôle : la part numérique de l'écart");
    println!();
    println!("Le pas CFL dépend de la hauteur maximale, donc de la présence de `B`. Les trois");
    println!("simulations d'un même point n'avancent donc pas par la même suite de pas, et leur");
    println!("écart contient une part qui n'est pas de la physique. À pas imposé, elle disparaît.");
    println!();
    println!("| `A_B/h0` | `A_δ/A_B` | `A_δ/h0` | écart CFL | écart à pas fixe | part numérique |");
    println!("|---:|---:|---:|---:|---:|---:|");
    let dt = 0.45 * DX / (9.81f64 * 1.6).sqrt();
    for (h0, a_b) in [(1.0f64, 0.02f64), (1.0, 0.50)] {
        for rapport in [0.05f64, 0.20, 1.00] {
            let a_d = rapport * a_b;
            let libre = ecart(h0, a_b, a_d, 8.0);
            let fixe = ecart_pas_fixe(h0, a_b, a_d, 8.0, dt);
            println!(
                "| {:.2} | {rapport:.2} | {:.4} | {libre:.2e} | {fixe:.2e} | {:.0} % |",
                a_b / h0,
                a_d / h0,
                100.0 * (libre - fixe).abs() / libre.max(f64::MIN_POSITIVE)
            );
        }
    }
    println!();
    println!("## L'anomalie à faible amplitude de fond : plancher ou physique ?");
    println!();
    println!("Le rapport `écart / (A_δ/h0)` vaut 0,20 à 0,25 pour `A_B/h0 >= 0,10`, mais monte à");
    println!("0,90 pour `A_B/h0 = 0,02`. Si c'est un plancher d'intégration, il ne descendra plus");
    println!("quand la perturbation devient minuscule ; si c'est de la physique, l'écart suivra.");
    println!();
    println!("| `A_δ/A_B` | `A_δ/h0` | écart à 8 s | écart / (`A_δ/h0`) |");
    println!("|---:|---:|---:|---:|");
    for rapport in [1.0f64, 0.2, 0.05, 0.01, 0.002, 0.0004] {
        let a_d = rapport * 0.02;
        let e = ecart(1.0, 0.02, a_d, 8.0);
        println!(
            "| {rapport:.4} | {:.6} | {e:.2e} | {:.3} |",
            a_d,
            e / a_d
        );
    }
    println!();
    println!("## Lecture");
    println!();
    println!("ADR-001 propose `max|δ| > 0,35·Hs` comme **valeur de départ à calibrer**. La colonne");
    println!("`A_δ/A_B` la confronte directement. Aucun seuil unique n'est retenu ici : « visiblement");
    println!("fausse » est perceptuel, et B4 seul en juge — le tableau donne l'écart, le lecteur");
    println!("applique sa tolérance (L242).");
}
