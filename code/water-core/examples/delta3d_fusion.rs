//! **Fusion et séparation de domaines δ, sans rupture** — S396, C8a de la campagne du solveur volumique 3D
//! ([ADR-207](../../../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) ; [ADR-006](../../../docs/adr/ADR-006-cellules-domaines-solveurs.md) §3–4).
//!
//! Un bassin de 4 m de large à 25 cm, 2 m d'eau (8 couches) sous 1 m d'air, pas mobile de 20 ms. Deux bosses gaussiennes de
//! 5 cm. **La référence** : un seul domaine qui couvre tout, depuis le départ.
//!
//! - **`fusion`** (32 m, bosses à 5 et 27 m) : deux domaines — A sur `[0, 14 m)`, B sur `[18, 32 m)` — évoluent séparés, murs
//!   compris ; leurs **ensembles actifs** (blocs de 8 × 8 colonnes où la surface s'écarte du repos de plus de 1 mm) sont
//!   comparés à chaque pas ; quand leurs dilatations de `r_c` = 4 m se touchent (`BlockSet::touches`, deux blocs), les deux
//!   états sont recopiés dans un domaine de la forme de la référence (`Volume3::transplant`), le reste au repos, et le calcul
//!   continue 5 s. `FUSION_A=<s>` force l'instant (témoins : trop tard, les ondes ont frappé les murs de A et de B).
//! - **`separation`** (40 m, bosses à 5 et 31 m — asymétriques) : un seul domaine ; ses blocs actifs, partitionnés, comptent deux composantes ;
//!   après une seconde continue (`SplitClock`), il se coupe au milieu de l'écart en deux domaines, qui continuent 5 s.
//!
//! Écart publié : la plus grande différence de surface à la référence, par tranches, rapportée à l'amplitude.
//!
//!     cargo run -p water-core --release --offline --example delta3d_fusion -- <fusion|separation>

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use water_core::delta3d::{Domain3, Volume3};
use water_core::domain_blocks::{Block, BlockSet, SplitClock, BLOCK};
use water_core::host::HostServices;

const DX: f32 = 0.25;
const NY: usize = 16;
const NZ: usize = 12;
const REST: f32 = 2.0;
const DT_US: u64 = 20_000;
const A: f32 = 0.05;
const ACTIVE: f32 = 1e-3;
const R_C: i32 = 2;

struct Hote {
    arena: host_impl::ArenaAllocator,
}

impl Hote {
    fn domain(&mut self, nx: usize) -> Volume3 {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        let mut v = Volume3::configure(&mut HostServices { alloc: &mut self.arena, jobs: &jobs, sink: &sink },
            Domain3 { nx, ny: NY, nz: NZ, dx: DX }, 1000., 9.81).expect("domaine");
        v.clear_to_rest(REST).expect("repos");
        v
    }
    fn set(&mut self) -> BlockSet {
        let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
        BlockSet::with_capacity(&mut HostServices { alloc: &mut self.arena, jobs: &jobs, sink: &sink }, 256).expect("ensemble")
    }
}

/// Une surface à bosses gaussiennes de `A`, centrées aux abscisses `centres` (mailles, de l'origine du domaine monde), sur
/// `nx` colonnes dont la première est la colonne monde `x0`.
fn bumps(nx: usize, x0: usize, centres: &[f32]) -> Vec<f32> {
    (0..NY)
        .flat_map(|j| {
            (0..nx).map(move |i| {
                let y = j as f32 + 0.5 - NY as f32 / 2.;
                REST + centres
                    .iter()
                    .map(|c| {
                        let x = (x0 + i) as f32 + 0.5 - c;
                        A * (-(x * x + y * y) / 8.).exp()
                    })
                    .sum::<f32>()
            })
        })
        .collect()
}

/// Les blocs actifs d'un domaine dont la première colonne est la colonne monde `x0` (multiple de `BLOCK`).
fn active(v: &Volume3, x0: usize, out: &mut BlockSet) {
    out.clear();
    let nx = v.domain().nx;
    for (c, eta) in v.surface().iter().enumerate() {
        if (eta - REST).abs() > ACTIVE {
            let (i, j) = (c % nx, c / nx);
            out.insert(Block { i: ((x0 + i) / BLOCK) as i32, j: (j / BLOCK) as i32 }).expect("capacité");
        }
    }
}

/// La plus grande différence de surface entre `v` (colonnes monde `x0..`) et la référence.
fn ecart(v: &Volume3, x0: usize, reference: &Volume3) -> f32 {
    let (nx, nr) = (v.domain().nx, reference.domain().nx);
    let (a, b) = (v.surface(), reference.surface());
    let mut m = 0f32;
    for j in 0..NY {
        for i in 0..nx {
            m = m.max((a[j * nx + i] - b[j * nr + x0 + i]).abs());
        }
    }
    m
}

fn pas(v: &mut Volume3) {
    v.step_surface_mobile(DT_US, 20_000, &host_impl::SequentialJobs).expect("pas");
}

fn main() {
    let cas = std::env::args().nth(1).unwrap_or_else(|| "fusion".into());
    let mut h = Hote { arena: host_impl::ArenaAllocator::with_capacity(1 << 30) };
    let dt = DT_US as f64 * 1e-6;
    if cas == "separation" {
        let nx = 160;
        // Asymétriques : des bosses symétriques autour de la coupure en feraient un plan de symétrie, où un mur ne change rien.
        let centres = [20., 124.];
        let mut reference = h.domain(nx);
        reference.set_free_surface(&bumps(nx, 0, &centres), REST).expect("surface");
        let mut seul = h.domain(nx);
        seul.set_free_surface(&bumps(nx, 0, &centres), REST).expect("surface");
        let (mut actifs, mut horloge) = (h.set(), SplitClock::default());
        let mut labels = vec![0u32; 256];
        let mut t = 0.;
        // Un seul domaine, jusqu'à la séparation due.
        let coupure = loop {
            pas(&mut seul);
            pas(&mut reference);
            t += dt;
            active(&seul, 0, &mut actifs);
            let n = actifs.components(R_C, &mut labels);
            if horloge.advance(DT_US, n) {
                // La coupure : au milieu de l'écart entre les deux premières composantes, sur une frontière de bloc.
                let blocs = actifs.blocks();
                let fin0 = blocs.iter().zip(&labels).filter(|(_, l)| **l == 0).map(|(b, _)| b.i).max().unwrap();
                let deb1 = blocs.iter().zip(&labels).filter(|(_, l)| **l == 1).map(|(b, _)| b.i).min().unwrap();
                break (((fin0 + 1 + deb1) / 2) as usize) * BLOCK;
            }
            assert!(t < 10., "aucune séparation");
        };
        let t_sep = t;
        let (mut a, mut b) = (h.domain(coupure), h.domain(nx - coupure));
        a.transplant(&seul, [0, 0]).expect("A");
        b.transplant(&seul, [-(coupure as isize), 0]).expect("B");
        let mut tranches = [0f32; 5];
        for s in 0..(5. / dt).round() as usize {
            pas(&mut a);
            pas(&mut b);
            pas(&mut reference);
            let e = ecart(&a, 0, &reference).max(ecart(&b, coupure, &reference));
            let k = ((s as f64 * dt) as usize).min(4);
            tranches[k] = tranches[k].max(e);
        }
        println!(
            "FUSION_S396 cas=separation largeur_m={} separation_s={t_sep:.2} coupure_m={:.1} ecart_max_sur_amplitude_par_seconde={}",
            nx as f32 * DX,
            coupure as f32 * DX,
            tranches.iter().map(|e| format!("{:.4}", e / A)).collect::<Vec<_>>().join("/")
        );
        return;
    }
    let (nx, xa, xb) = (128usize, 56usize, 72usize);
    let centres = [20., 108.];
    let force: Option<f64> = std::env::var("FUSION_A").ok().and_then(|v| v.parse().ok());
    let mut reference = h.domain(nx);
    reference.set_free_surface(&bumps(nx, 0, &centres), REST).expect("surface");
    let (mut a, mut b) = (h.domain(xa), h.domain(nx - xb));
    a.set_free_surface(&bumps(xa, 0, &centres), REST).expect("A");
    b.set_free_surface(&bumps(nx - xb, xb, &centres), REST).expect("B");
    let (mut sa, mut sb) = (h.set(), h.set());
    let (mut t, mut ecart_avant) = (0., 0f32);
    loop {
        active(&a, 0, &mut sa);
        active(&b, xb, &mut sb);
        let due = match force {
            Some(tf) => t >= tf - 1e-9,
            None => sa.touches(&sb, R_C),
        };
        if due {
            break;
        }
        pas(&mut a);
        pas(&mut b);
        pas(&mut reference);
        t += dt;
        ecart_avant = ecart_avant.max(ecart(&a, 0, &reference)).max(ecart(&b, xb, &reference));
        assert!(t < 10., "aucune fusion");
    }
    let t_fusion = t;
    let mut fused = h.domain(nx);
    fused.transplant(&a, [0, 0]).expect("A");
    fused.transplant(&b, [xb as isize, 0]).expect("B");
    let au_raccord = ecart(&fused, 0, &reference);
    let mut tranches = [0f32; 5];
    for s in 0..(5. / dt).round() as usize {
        pas(&mut fused);
        pas(&mut reference);
        let k = ((s as f64 * dt) as usize).min(4);
        tranches[k] = tranches[k].max(ecart(&fused, 0, &reference));
    }
    let pire = tranches.iter().fold(0f32, |m, e| m.max(*e));
    println!(
        "FUSION_S396 cas=fusion force={} fusion_s={t_fusion:.2} ecart_avant_sur_amplitude={:.5} ecart_a_la_fusion_sur_amplitude={:.5} \
         ecart_max_sur_amplitude_par_seconde={} pire_sur_amplitude={:.5} critere_1_pourcent={}",
        force.map_or("non".to_string(), |f| format!("{f}")),
        ecart_avant / A,
        au_raccord / A,
        tranches.iter().map(|e| format!("{:.5}", e / A)).collect::<Vec<_>>().join("/"),
        pire / A,
        if pire / A <= 0.01 { "tenu" } else { "manque" }
    );
}
