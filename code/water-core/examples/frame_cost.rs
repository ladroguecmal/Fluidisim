//! S206 : coût de l'eau **par image** sur une scène représentative de J1 (A247, ADR-127 D7).
//!
//! Scène déclarée avant mesure (EN-COURS S206) : observateur S201, mer S201 Hs 1,5 m, impact S203
//! à +3 s composé par le chemin hôte. La charge est une **grille de sommets projetée** — un sommet
//! par `c` pixels, intersection du rayon avec z = 0 jusqu'à 600 m — c'est-à-dire ce qu'un maillage
//! de surface évalué sur CPU demande à chaque image, et non le lancer de rayons hors ligne.
//!
//! La scène n'est pas recopiée : elle vient de `render_impact.rs`, inclus comme module.
#[allow(dead_code)]
#[path = "render_impact.rs"]
mod impact;
#[allow(dead_code)]
#[path = "support/ray_view.rs"]
mod ray_view;
use impact::{
    scene_s205, Scene, BIRTH, CAMERA_ORIGIN, CAMERA_TARGET, CELL, EMPRISE_AGE_US, EMPRISE_N,
    EMPRISE_RADIUS_M, FRAME, HEIGHT, WIDTH,
};
use ray_view::Camera;
use std::{hint::black_box, time::Instant};
use water_core::{
    impact_field::BREAKING_SLOPE,
    prepared_water::{BoundBackground, Context, Prepared},
    radial_impact::{Domain, RadialImpact},
    wave_journal::{Cause, Journal},
    PhaseQ32, SimTime, WaterSample, WorldPos,
};

/// Budget eau par image, ADR-125.
pub const BUDGET_MS: f64 = 2.0;
/// Âge de l'impact à l'image mesurée, comme S203/S205.
pub const AGE_US: u64 = 3_000_000;

/// Sommets de la grille projetée : positions monde et locales, et indices dans l'emprise.
pub struct Grid {
    pub step_px: usize,
    pub world: Vec<WorldPos>,
    pub inside: Vec<usize>,
    pub outside: Vec<usize>,
}

/// Un sommet par `step_px` pixels, au centre de sa case ; rayons montants et au-delà de 600 m
/// exclus — même limite que la marche S201.
pub fn grid(sc: &Scene, field: &RadialImpact<EMPRISE_N>, step_px: usize) -> Grid {
    let camera = Camera::new(CAMERA_ORIGIN, CAMERA_TARGET);
    let mut g = Grid {
        step_px,
        world: Vec::new(),
        inside: Vec::new(),
        outside: Vec::new(),
    };
    let half = step_px as f64 / 2.0;
    let mut y = half;
    while y < HEIGHT as f64 {
        let mut x = half;
        while x < WIDTH as f64 {
            let dir = camera.ray(x, y, WIDTH, HEIGHT);
            if dir[2] < -1e-8 {
                let t = -camera.origin[2] / dir[2];
                if t <= 600.0 {
                    let p = [camera.origin[0] + t * dir[0], camera.origin[1] + t * dir[1]];
                    let w = WorldPos::from_metres(p[0], p[1], 0.0);
                    let local = sc.background.local_point(w).expect("référentiel");
                    if field.admits(FRAME, CELL, [local[0], local[1]]) {
                        g.inside.push(g.world.len());
                    } else {
                        g.outside.push(g.world.len());
                    }
                    g.world.push(w);
                }
            }
            x += step_px as f64;
        }
        y += step_px as f64;
    }
    g
}

/// Montage hôte de l'impact : journal confirmé, pool N256, emprise ADR-126.
pub struct Mount<'j> {
    pub field: RadialImpact<EMPRISE_N>,
    pub journal: Journal<'j>,
    pub pool: Vec<Option<RadialImpact<EMPRISE_N>>>,
    pub context: Context,
}
pub fn mount<'j>(sc: &Scene, slots: &'j mut [Option<water_core::wave_journal::Record>]) -> Mount<'j> {
    let domain = Domain {
        radius: EMPRISE_RADIUS_M,
        age_us: EMPRISE_AGE_US,
    };
    let field = RadialImpact::<EMPRISE_N>::new(sc.event, sc.medium, domain).expect("champ");
    let mut journal = Journal::new(0, slots);
    journal
        .confirm(
            0,
            Cause {
                entity: 0,
                command: 0,
                emission: 0,
            },
            sc.event,
        )
        .expect("journal");
    Mount {
        field,
        journal,
        pool: (0..1).map(|_| None).collect(),
        context: Context {
            frame: FRAME,
            cell: CELL,
            medium: sc.medium,
            domain,
        },
    }
}

/// Une image sur une tranche de sommets : B hors emprise, B+W par lot dans l'emprise.
/// `out` reçoit, dans l'ordre des indices donnés, les sommets hors emprise puis ceux dedans.
pub fn frame_slice(
    sc: &Scene,
    prepared: &Prepared<'_, '_, EMPRISE_N>,
    bound: &BoundBackground<'_>,
    g: &Grid,
    outside: &[usize],
    inside_world: &[WorldPos],
    time: SimTime,
    out: &mut [WaterSample],
    scratch: &mut [WaterSample],
) {
    for (o, &i) in out.iter_mut().zip(outside) {
        *o = sc.background.eval(g.world[i], time).expect("B");
    }
    let n = outside.len();
    if !inside_world.is_empty() {
        prepared
            .sample_world_batch(
                bound,
                inside_world,
                time,
                BREAKING_SLOPE,
                &mut out[n..n + inside_world.len()],
                &mut scratch[..inside_world.len()],
            )
            .expect("B+W");
    }
}

/// Temps mur en ms : médiane et maximum de 11 mesures après 3 chauffes.
pub fn timed(mut action: impl FnMut()) -> (f64, f64) {
    for _ in 0..3 {
        action();
    }
    let mut v: Vec<f64> = (0..11)
        .map(|_| {
            let t = Instant::now();
            action();
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    v.sort_by(f64::total_cmp);
    (v[5], v[10])
}

/// Découpe `0..n` en `parts` tranches contiguës de tailles voisines.
pub fn chunks(n: usize, parts: usize) -> Vec<std::ops::Range<usize>> {
    let parts = parts.max(1);
    let base = n / parts;
    let extra = n % parts;
    let mut start = 0;
    (0..parts)
        .map(|p| {
            let len = base + usize::from(p < extra);
            let r = start..start + len;
            start += len;
            r
        })
        .collect()
}

/// Noyau d'une table radiale à matrice de Bessel précalculée : `j[n*m+i] = (J0, J1)(k_n r_i)`,
/// fixe pour la vie du champ ; par image, N phases puis N×M produits. Les valeurs sont
/// synthétiques — le coût ne dépend que des tailles — et le résultat n'est pas une physique.
pub struct BesselKernel {
    pub n: usize,
    pub m: usize,
    pub freq: Vec<u64>,
    pub coef: Vec<f32>,
    pub k: Vec<f32>,
    pub j: Vec<[f32; 2]>,
    pub eta: Vec<f32>,
    pub slope: Vec<f32>,
}
impl BesselKernel {
    pub fn synthetic(n: usize, m: usize) -> Self {
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 40) as f32 / (1u64 << 24) as f32 - 0.5
        };
        Self {
            n,
            m,
            freq: (0..n).map(|i| 1_000_000_000 + 7_919 * i as u64).collect(),
            coef: (0..n).map(|_| next()).collect(),
            k: (0..n).map(|_| next() + 1.0).collect(),
            j: (0..n * m).map(|_| [next(), next()]).collect(),
            eta: vec![0.0; m],
            slope: vec![0.0; m],
        }
    }
    pub fn bytes(&self) -> usize {
        self.j.len() * std::mem::size_of::<[f32; 2]>()
            + self.n * (std::mem::size_of::<u64>() + 2 * std::mem::size_of::<f32>())
    }
    pub fn frame(&mut self, age: SimTime) {
        self.eta.fill(0.0);
        self.slope.fill(0.0);
        for node in 0..self.n {
            let c = PhaseQ32::from_time(self.freq[node], age).cos();
            let a = self.coef[node] * c;
            let b = -self.coef[node] * self.k[node] * c;
            let row = &self.j[node * self.m..(node + 1) * self.m];
            for ((e, s), jj) in self.eta.iter_mut().zip(self.slope.iter_mut()).zip(row) {
                *e += a * jj[0];
                *s += b * jj[1];
            }
        }
    }
}

fn run() -> Result<(), String> {
    let sc = scene_s205(1.5, EMPRISE_AGE_US)?;
    let mut slots = vec![None; 1];
    let mut m = mount(&sc, &mut slots);
    let prepared = Prepared::build(&m.journal, &mut m.pool, m.context).map_err(|e| format!("{e:?}"))?;
    let bound = BoundBackground::new(&sc.background, FRAME, CELL);
    let time = SimTime(BIRTH.0 + AGE_US);
    let threads_available = std::thread::available_parallelism().map_or(1, |n| n.get());
    println!("# S206 cout d'image, scene J1 : mer S201 Hs1,5 N32, impact S203 N256 R52 A56 a +3 s, budget {BUDGET_MS} ms");
    println!("# fils materiels disponibles : {threads_available}");
    println!("pas_px sommets dans_R hors_R | B_seul_med_ms B_seul_max_ms | image_1fil_med_ms image_1fil_max_ms | B_ns_pt BW_ns_pt | image/budget");
    for step in [8usize, 4, 2] {
        let g = grid(&sc, &m.field, step);
        let inside_world: Vec<WorldPos> = g.inside.iter().map(|&i| g.world[i]).collect();
        let n = g.world.len();
        let mut out = vec![WaterSample::default(); n];
        let mut scratch = vec![WaterSample::default(); n];
        let b_only = timed(|| {
            for w in &g.world {
                black_box(sc.background.eval(*w, time));
            }
        });
        let full = timed(|| {
            frame_slice(&sc, &prepared, &bound, &g, &g.outside, &inside_world, time, &mut out, &mut scratch);
            black_box(&out);
        });
        let b_ns = b_only.0 * 1e6 / n as f64;
        let bw_ns = (full.0 - b_only.0 * g.outside.len() as f64 / n as f64) * 1e6 / g.inside.len().max(1) as f64;
        println!(
            "{step} {n} {} {} | {:.3} {:.3} | {:.3} {:.3} | {:.0} {:.0} | {:.1}",
            g.inside.len(),
            g.outside.len(),
            b_only.0,
            b_only.1,
            full.0,
            full.1,
            b_ns,
            bw_ns,
            full.0 / BUDGET_MS
        );
    }
    Ok(())
}

/// Tampons préalloués d'un fil : aucune allocation pendant la mesure.
pub struct Lane {
    pub outside: Vec<usize>,
    pub inside_world: Vec<WorldPos>,
    pub inside: Vec<usize>,
    pub out: Vec<WaterSample>,
    pub scratch: Vec<WaterSample>,
}
pub fn lanes(g: &Grid, parts: usize) -> Vec<Lane> {
    let co = chunks(g.outside.len(), parts);
    let ci = chunks(g.inside.len(), parts);
    co.into_iter()
        .zip(ci)
        .map(|(o, i)| {
            let outside = g.outside[o].to_vec();
            let inside = g.inside[i].to_vec();
            let n = outside.len() + inside.len();
            Lane {
                inside_world: inside.iter().map(|&k| g.world[k]).collect(),
                outside,
                inside,
                out: vec![WaterSample::default(); n],
                scratch: vec![WaterSample::default(); n],
            }
        })
        .collect()
}
/// Une image répartie sur `lanes.len()` fils, lancés et rejoints dans l'image — ce qu'un hôte
/// sans groupe de fils persistant paierait. Le coût de lancement est mesuré à part.
pub fn frame_parallel(
    sc: &Scene,
    prepared: &Prepared<'_, '_, EMPRISE_N>,
    bound: &BoundBackground<'_>,
    g: &Grid,
    lanes: &mut [Lane],
    time: SimTime,
) {
    std::thread::scope(|s| {
        for lane in lanes.iter_mut() {
            s.spawn(move || {
                let Lane {
                    outside,
                    inside_world,
                    out,
                    scratch,
                    ..
                } = lane;
                frame_slice(sc, prepared, bound, g, outside, inside_world, time, out, scratch);
            });
        }
    });
}
/// Résultat par sommet, quel que soit le découpage : pour vérifier qu'il ne change rien.
pub fn by_vertex(g: &Grid, lanes: &[Lane]) -> Vec<[u32; 10]> {
    let mut v = vec![[0u32; 10]; g.world.len()];
    for lane in lanes {
        for (slot, &k) in lane.outside.iter().chain(lane.inside.iter()).enumerate() {
            let s = lane.out[slot];
            v[k] = [
                s.eta, s.deta_dt, s.u_total[0], s.u_total[1], s.u_total[2], s.normal[0], s.normal[1],
                s.normal[2], s.steepness, s.aeration,
            ]
            .map(f32::to_bits);
        }
    }
    v
}

fn levers() -> Result<(), String> {
    let sc = scene_s205(1.5, EMPRISE_AGE_US)?;
    let mut slots = vec![None; 1];
    let mut m = mount(&sc, &mut slots);
    let prepared = Prepared::build(&m.journal, &mut m.pool, m.context).map_err(|e| format!("{e:?}"))?;
    let bound = BoundBackground::new(&sc.background, FRAME, CELL);
    let time = SimTime(BIRTH.0 + AGE_US);

    println!("# L1 parallelisme, temps mur par image (fils lances et rejoints dans l'image)");
    println!("pas_px fils | image_med_ms image_max_ms | lancement_seul_med_ms | acceleration | image/budget | identique_1fil");
    for step in [8usize, 4, 2] {
        let g = grid(&sc, &m.field, step);
        let mut reference = lanes(&g, 1);
        frame_parallel(&sc, &prepared, &bound, &g, &mut reference, time);
        let reference_bits = by_vertex(&g, &reference);
        let mut one = 0.0;
        for threads in [1usize, 2, 4, 8, 16] {
            let mut ls = lanes(&g, threads);
            let t = timed(|| frame_parallel(&sc, &prepared, &bound, &g, &mut ls, time));
            let spawn = timed(|| {
                std::thread::scope(|s| {
                    for _ in 0..threads {
                        s.spawn(|| black_box(0));
                    }
                })
            });
            if threads == 1 {
                one = t.0;
            }
            let same = by_vertex(&g, &ls) == reference_bits;
            println!(
                "{step} {threads} | {:.3} {:.3} | {:.3} | {:.2} | {:.1} | {same}",
                t.0,
                t.1,
                spawn.0,
                one / t.0,
                t.0 / BUDGET_MS
            );
            if !same {
                return Err("le decoupage en fils change un resultat".into());
            }
        }
    }

    println!("# L2 table radiale a matrice de Bessel precalculee (noyau N x M, valeurs synthetiques)");
    println!("pas_table_m M | noyau_image_med_ms noyau_max_ms | memoire_ko_par_impact | erreur_max_table_mm | hermite_ns_pt | impacts_dans_2ms_noyau_seul | paquets_W_max_4096_ms");
    let lambda = sc.wavelength_m;
    let age = SimTime(AGE_US);
    for divisor in [16.0f32, 8.0] {
        let step = lambda / divisor;
        let m_count = (EMPRISE_RADIUS_M / step) as usize + 1;
        let mut kernel = BesselKernel::synthetic(EMPRISE_N, m_count);
        let k = timed(|| {
            kernel.frame(black_box(age));
            black_box(&kernel.eta);
        });
        // Exactitude : table construite par échantillons directs aux nœuds (S203), même valeurs
        // qu'une matrice précalculée à l'ordre de sommation près, puis Hermite entre les nœuds.
        let mut table = Vec::with_capacity(m_count + 1);
        impact::radial_table(&m.field, time, step, &mut table)?;
        let v = *m.field.event().data();
        let mut err = 0.0f64;
        for i in 0..20_000u32 {
            let r = (i as f32 + 0.37) / 20_000.0 * (EMPRISE_RADIUS_M - step - 1e-3);
            let d = m
                .field
                .sample(FRAME, CELL, [v.position[0] + r, v.position[1]], time)
                .map_err(|e| format!("{e:?}"))?;
            let t = impact::table_eta(&table, step, r).ok_or("table")?;
            err = err.max((d.eta as f64 - t as f64).abs());
        }
        let g = grid(&sc, &m.field, 4);
        let radii: Vec<f32> = g
            .inside
            .iter()
            .map(|&i| {
                let l = sc.background.local_point(g.world[i]).unwrap();
                (l[0] - v.position[0]).hypot(l[1] - v.position[1])
            })
            .collect();
        let h = timed(|| {
            for r in &radii {
                black_box(impact::table_eta(&table, step, *r));
            }
        });
        println!(
            "{step:.6} {m_count} | {:.4} {:.4} | {:.0} | {:.4} | {:.1} | {:.0} | {:.1}",
            k.0,
            k.1,
            kernel.bytes() as f64 / 1024.0,
            err * 1e3,
            h.0 * 1e6 / radii.len() as f64,
            BUDGET_MS / k.0,
            4096.0 * k.0
        );
    }
    Ok(())
}

/// S208 : le chemin d'image construit (ADR-129) — table réelle de la bibliothèque, pas le noyau
/// synthétique de S206. Par image : B sur tous les sommets, un profil, une évaluation d'Hermite
/// par sommet dans l'emprise.
fn table_frames() -> Result<(), String> {
    let sc = scene_s205(1.5, EMPRISE_AGE_US)?;
    let mut slots = vec![None; 1];
    let m = mount(&sc, &mut slots);
    let time = SimTime(BIRTH.0 + AGE_US);
    println!("# S208 chemin d'image par table (bibliotheque), un fil, release");
    println!("pas_table M memoire_ko cuisson_ms | pas_px sommets dans_R | B_med_ms profil_med_ms eval_med_ms image_med_ms image_max_ms | image/budget | direct_S206_ms");
    for divisor in [16.0f32, 8.0] {
        let step = sc.wavelength_m / divisor;
        let len = m.field.table_len(step).map_err(|e| format!("{e:?}"))?;
        let mut storage = vec![[0.0f32; 2]; EMPRISE_N * len];
        let bake = {
            let t = Instant::now();
            m.field.bake_table(step, &mut storage).map_err(|e| format!("{e:?}"))?;
            t.elapsed().as_secs_f64() * 1000.0
        };
        let memory_kb = (storage.len() * std::mem::size_of::<[f32; 2]>()) as f64 / 1024.0;
        let table = m.field.bake_table(step, &mut storage).map_err(|e| format!("{e:?}"))?;
        let mut profile = vec![(0.0f32, 0.0f32); len];
        for (px, direct) in [(4usize, 72.4f64), (2, 280.0)] {
            let g = grid(&sc, &m.field, px);
            let local: Vec<[f32; 2]> = g
                .inside
                .iter()
                .map(|&i| {
                    let l = sc.background.local_point(g.world[i]).unwrap();
                    [l[0], l[1]]
                })
                .collect();
            let b = timed(|| {
                for w in &g.world {
                    black_box(sc.background.eval(*w, time));
                }
            });
            let p = timed(|| {
                table.profile(time, &mut profile).unwrap();
                black_box(&profile);
            });
            table.profile(time, &mut profile).map_err(|e| format!("{e:?}"))?;
            let e = timed(|| {
                for l in &local {
                    black_box(table.eval(&profile, FRAME, CELL, *l).unwrap());
                }
            });
            let full = timed(|| {
                for w in &g.world {
                    black_box(sc.background.eval(*w, time));
                }
                table.profile(time, &mut profile).unwrap();
                for l in &local {
                    black_box(table.eval(&profile, FRAME, CELL, *l).unwrap());
                }
            });
            println!(
                "{step:.6} {len} {memory_kb:.0} {bake:.3} | {px} {} {} | {:.3} {:.4} {:.3} {:.3} {:.3} | {:.1} | {direct}",
                g.world.len(),
                g.inside.len(),
                b.0,
                p.0,
                e.0,
                full.0,
                full.1,
                full.0 / BUDGET_MS
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_deref() {
        None | Some("frame") => run()?,
        Some("levers") => levers()?,
        Some("table") => table_frames()?,
        Some(other) => return Err(format!("mode inconnu {other} : frame | levers | table").into()),
    }
    Ok(())
}

#[cfg(test)]
mod frame_tests {
    use super::*;
    #[test]
    fn chunks_cover_every_index_once() {
        for (n, parts) in [(0, 4), (7, 3), (16, 16), (5, 8), (1000, 7)] {
            let c = chunks(n, parts);
            assert_eq!(c.len(), parts);
            let mut next = 0;
            for r in c {
                assert_eq!(r.start, next);
                next = r.end;
            }
            assert_eq!(next, n);
        }
    }
    #[test]
    fn grid_splits_by_the_footprint_predicate() {
        let sc = scene_s205(1.5, EMPRISE_AGE_US).unwrap();
        let mut slots = vec![None; 1];
        let m = mount(&sc, &mut slots);
        let g = grid(&sc, &m.field, 16);
        assert_eq!(g.inside.len() + g.outside.len(), g.world.len());
        assert!(!g.inside.is_empty() && !g.outside.is_empty());
        assert!(g.world.len() <= (WIDTH / 16) * (HEIGHT / 16));
    }
    #[test]
    fn threads_do_not_change_any_bit() {
        let sc = scene_s205(1.5, EMPRISE_AGE_US).unwrap();
        let mut slots = vec![None; 1];
        let mut m = mount(&sc, &mut slots);
        let prepared = Prepared::build(&m.journal, &mut m.pool, m.context).unwrap();
        let bound = BoundBackground::new(&sc.background, FRAME, CELL);
        let g = grid(&sc, &m.field, 32);
        let time = SimTime(BIRTH.0 + AGE_US);
        let mut one = lanes(&g, 1);
        frame_parallel(&sc, &prepared, &bound, &g, &mut one, time);
        let mut three = lanes(&g, 3);
        frame_parallel(&sc, &prepared, &bound, &g, &mut three, time);
        assert_eq!(by_vertex(&g, &one), by_vertex(&g, &three));
        // Et chaque sommet est bien calculé : aucun reste à zéro.
        assert!(by_vertex(&g, &three).iter().all(|b| b[7] != 0));
    }
    #[test]
    fn bessel_kernel_sums_rows_in_order() {
        let mut k = BesselKernel::synthetic(3, 4);
        k.frame(SimTime(0));
        // À t = 0, cos = 1 : eta_i = Σ_n coef_n J0[n][i], dans l'ordre des nœuds.
        for i in 0..4 {
            let mut e = 0.0f32;
            for n in 0..3 {
                e += k.coef[n] * k.j[n * 4 + i][0];
            }
            assert_eq!(e.to_bits(), k.eta[i].to_bits());
        }
        assert_eq!(k.bytes(), 3 * 4 * 8 + 3 * 16);
    }
}
