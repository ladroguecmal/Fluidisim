//! Couche `B` — le champ de fond. Version minimale de H1.
//!
//! **Ce que cette implémentation est.** Une somme de composantes de Gerstner, à ordre de sommation
//! fixé, dont la seule qualité est d'être **déterministe bit à bit** et de donner au harnais
//! quelque chose de réel à hacher. Elle exerce la chaîne complète : phases repliées, conversion de
//! position monde, réduction ordonnée, hash de conformité.
//!
//! **Ce qu'elle n'est pas.** Le champ de fond du projet. Le nombre de composantes, leur découpage
//! en bandes et le choix Gerstner sommé contre tuile FFT sont ouverts et se tranchent au banc **B1**
//! (ADR-004 §7.1 et §7.2). Rien ici ne préjuge de ce résultat.
//!
//! **Deux invariants sont mécaniques dans ce module.**
//!
//! - **I-02** — le fond ne stocke pas son état : aucune valeur par cellule n'existe, `eval` est une
//!   fonction pure de `(x, t)` et des paramètres ;
//! - **I-06** — aucune allocation à l'exécution : les composantes sont allouées à la configuration,
//!   avant `seal()`, et `eval` n'alloue rien.

use crate::hash::Hasher64;
use crate::host::{AllocError, HostServices};
use crate::phase::{freq_hz_to_q32, PhaseQ32};
use crate::types::{SimTime, WaterSample, WorldPos};

/// Une composante de houle. Paramètres figés à la configuration.
#[derive(Clone, Copy, Debug)]
pub struct Component {
    /// Amplitude, en mètres.
    pub amplitude: f32,
    /// Nombre d'onde, en **tours par mètre** — `1/λ`.
    pub k_turns_per_m: f32,
    /// Direction de propagation, unitaire.
    pub dir: [f32; 2],
    /// Fréquence en tours/s, virgule fixe `2³²`.
    pub freq_q32: u64,
    /// Déphasage initial.
    pub phase0: PhaseQ32,
}

/// Paramètres d'un état de mer — sous-ensemble de `HydroSample` (ADR-004 §2.2) suffisant pour H1.
#[derive(Clone, Copy, Debug)]
pub struct SeaState {
    /// Hauteur significative, en mètres.
    pub hs: f32,
    /// Période de pic, en secondes.
    pub tp: f32,
    /// Direction moyenne, en tours (`0` = +x).
    pub theta_turns: f32,
    /// Nombre de composantes.
    pub components: usize,
    /// Graine des phases ; même graine et même indice donnent la même phase.
    pub graine: u64,
}

/// SplitMix64 indexé : accès direct, aucun état partagé ni dépendance à l'ordre d'appel.
/// Mix13 de Stafford, constantes de SplittableRandom (OpenJDK) ; formule documentée S65.
fn phase_initiale(graine: u64, indice: u64) -> PhaseQ32 {
    let mut z = graine.wrapping_add(indice.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    PhaseQ32(((z ^ (z >> 31)) >> 32) as u32)
}

pub struct Background {
    components: Vec<Component>,
    anchor: WorldPos,
}

impl Background {
    /// Construit le champ de fond. **À l'initialisation uniquement**, avant `seal()`.
    ///
    /// La dispersion en eau profonde donne `ω² = g·k` (SPEC-001 §1) ; les composantes sont réparties
    /// géométriquement autour de la période de pic, et leurs amplitudes suivent `E = ρgHs²/16`
    /// réparti uniformément — c'est grossier, et c'est assumé : B1 tranchera.
    pub fn configure(
        host: &mut HostServices,
        sea: SeaState,
        anchor: WorldPos,
    ) -> Result<Background, AllocError> {
        const G: f64 = 9.81;
        let n = sea.components.max(1);

        // I-06 : l'allocation est demandée à l'hôte, et elle échouera si le système est scellé.
        let bytes = n * core::mem::size_of::<Component>();
        host.alloc.alloc_persistent(bytes)?;

        let mut components = Vec::with_capacity(n);
        // Amplitude par composante : Hs = 4·√(m0), m0 = Σ a²/2  ⇒  a = Hs/(2·√(2n)).
        let amp = (sea.hs as f64) / (2.0 * (2.0 * n as f64).sqrt());

        for i in 0..n {
            // Périodes réparties géométriquement sur [Tp/2, 2·Tp].
            let f = if n == 1 {
                0.0
            } else {
                i as f64 / (n - 1) as f64
            };
            let period = (sea.tp as f64) * 0.5 * 4f64.powf(f);
            let omega = core::f64::consts::TAU / period;
            let k = omega * omega / G; // rad/m
            let k_turns = k / core::f64::consts::TAU; // tours/m
            let hz = 1.0 / period;

            // Éventail directionnel déterministe autour de theta, sans aléa : ±15°.
            let spread = if n == 1 {
                0.0
            } else {
                (f - 0.5) * (30.0 / 360.0)
            };
            let ang = (sea.theta_turns as f64 + spread) * core::f64::consts::TAU;

            components.push(Component {
                amplitude: amp as f32,
                k_turns_per_m: k_turns as f32,
                dir: [ang.cos() as f32, ang.sin() as f32],
                freq_q32: freq_hz_to_q32(hz),
                phase0: phase_initiale(sea.graine, i as u64),
            });
        }

        Ok(Background { components, anchor })
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Évalue le champ de fond en un point, à un instant. **Fonction pure — I-02.**
    ///
    /// L'ordre de sommation est celui du tableau, fixé à la configuration : c'est ce qu'exige
    /// ADR-003 §2 pour le déterminisme bit à bit.
    pub fn eval(&self, p: WorldPos, t: SimTime) -> Option<WaterSample> {
        let local = p.to_local(self.anchor)?;
        let mut s = WaterSample::default();
        let mut steep = 0.0f32;

        for c in &self.components {
            let d = local[0] * c.dir[0] + local[1] * c.dir[1];
            let phase = PhaseQ32::from_distance(c.k_turns_per_m, d)
                .wrapping_add(PhaseQ32(c.phase0.0.wrapping_sub(
                    PhaseQ32::from_time(c.freq_q32, t).0,
                )));
            let sn = phase.sin();
            let cs = phase.cos();

            s.eta += c.amplitude * sn;

            // Vitesse orbitale de surface, théorie d'Airy en eau profonde.
            //
            // Avec `η = a·sin(φ)`, le potentiel donne `u = a·ω·sin(φ)` — **en phase avec
            // l'élévation** — et `w = a·ω·cos(φ)`, en quadrature. La conséquence physique est
            // directe : **sous une crête, l'eau avance**.
            //
            // Ces deux lignes étaient inversées jusqu'en S21. La relecture ne l'avait pas vu ; le
            // cas analytique `u/η = ω` l'a fait tomber au premier passage.
            let omega = (c.freq_q32 as f64 / 4_294_967_296.0 * core::f64::consts::TAU) as f32;
            let uo = c.amplitude * omega;
            s.u_total[0] += uo * sn * c.dir[0];
            s.u_total[1] += uo * sn * c.dir[1];
            s.u_total[2] += uo * cs;

            // Pente locale : ∂η/∂x = a·k·cos(φ), avec k en radians par mètre.
            let k_rad = c.k_turns_per_m * core::f32::consts::TAU;
            let slope = c.amplitude * k_rad * cs;
            s.normal[0] -= slope * c.dir[0];
            s.normal[1] -= slope * c.dir[1];

            s.deta_dt -= uo * cs;
            // Cambrure : H/λ = 2·a·k_turns.
            steep += 2.0 * c.amplitude * c.k_turns_per_m;
        }

        s.normal[2] = 1.0;
        let n = &mut s.normal;
        let inv = 1.0 / (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        n[0] *= inv;
        n[1] *= inv;
        n[2] *= inv;
        s.steepness = steep;
        Some(s)
    }

    /// Hash de conformité du champ de fond — I-03, cas canonique C18.
    ///
    /// Échantillonne une grille régulière, ancrée sur la position de référence, et hache les octets
    /// des grandeurs produites. Deux plateformes qui divergent d'un seul bit donnent deux hashs
    /// différents, et la bisection automatique de SPEC-003 §7.2 désigne le commit fautif.
    pub fn conformance_hash(&self, t: SimTime, side: u32, step_m: f64) -> u64 {
        let mut h = Hasher64::new();
        h.write_u64(t.micros());
        h.write_u64(self.components.len() as u64);
        for iy in 0..side {
            for ix in 0..side {
                let x = (ix as f64 - side as f64 * 0.5) * step_m;
                let y = (iy as f64 - side as f64 * 0.5) * step_m;
                let p = WorldPos::from_units(
                    self.anchor.x + (x * crate::types::WORLD_UNITS_PER_METRE as f64) as i64,
                    self.anchor.y + (y * crate::types::WORLD_UNITS_PER_METRE as f64) as i64,
                    self.anchor.z,
                );
                if let Some(s) = self.eval(p, t) {
                    h.write_f32(s.eta);
                    h.write_f32(s.u_total[0]);
                    h.write_f32(s.u_total[1]);
                    h.write_f32(s.u_total[2]);
                    h.write_f32(s.normal[0]);
                    h.write_f32(s.normal[1]);
                    h.write_f32(s.deta_dt);
                }
            }
        }
        h.finish()
    }
}

#[cfg(test)]
mod tests_phases {
    use super::*;

    #[test]
    fn vecteurs_entiers_et_acces_direct() {
        // Trois premières sorties SplitMix64, graine zéro, 32 bits de poids fort.
        let attendu = [0xe220_a839, 0x6e78_9e6a, 0x06c4_5d18];
        for i in [2, 0, 1, 2] {
            assert_eq!(phase_initiale(0, i).0, attendu[i as usize]);
        }
        // Les 32 bits hauts de la graine ne sont pas perdus ; zéro n'est pas un mode spécial.
        assert_ne!(phase_initiale(0, 0), phase_initiale(1 << 32, 0));
        assert_ne!(phase_initiale(0, 0), phase_initiale(u64::MAX, 0));
        assert_eq!(phase_initiale(0, u64::MAX).0, 0); // Arithmétique modulo 2^64 explicite.
    }
}

#[cfg(test)]
mod diagnostic_homogeneite_s66 {
    use super::*;
    use crate::host::{Allocator, AllocStats, JobSystem, Sink};

    struct Hote;
    impl Allocator for Hote {
        fn alloc_persistent(&mut self, _: usize) -> Result<usize, AllocError> { Ok(0) }
        fn seal(&mut self) {}
        fn is_sealed(&self) -> bool { false }
        fn stats(&self) -> AllocStats { AllocStats::default() }
    }
    impl Sink for Hote {
        fn warn(&self, _: &str) {}
        fn metric(&self, _: &str, _: f64) {}
    }
    impl JobSystem for Hote {
        fn worker_count(&self) -> u32 { 1 }
        fn parallel_reduce_ordered_f64(&self, n: usize, _: usize,
            reduce: &dyn Fn(usize, usize) -> f64, merge: &dyn Fn(f64, f64) -> f64,
            init: f64) -> f64 { merge(init, reduce(0,n)) }
    }

    fn fond(graine: u64, n: usize) -> Background {
        let mut alloc = Hote;
        let services = Hote;
        let mut host = HostServices { alloc: &mut alloc, jobs: &services, sink: &services };
        Background::configure(&mut host, SeaState { hs: 1.2, tp: 6.0,
            theta_turns: 0.0, components: n, graine }, WorldPos::from_metres(0.,0.,0.)).unwrap()
    }

    // Même table de composantes et même phase temporelle entière ; seule l'évaluation
    // spatiale et la sommation passent en f64. Pas de prétention à un oracle du spectre.
    // Rend variance production / référence / somme des variances individuelles / max écart eta.
    fn variances(bg: &Background, cote: usize, pas: f64, ox: f64) -> [f64; 4] {
        let t = SimTime::from_micros(1_735_689_600_000_000);
        let mut somme = [0.0; 2];
        let mut carres = [0.0; 2];
        let mut sommes_i = vec![0.0; bg.components.len()];
        let mut carres_i = vec![0.0; bg.components.len()];
        let mut max_ecart = 0.0f64;
        for iy in 0..cote {
            for ix in 0..cote {
                let x = ox + (ix as f64 - cote as f64*0.5)*pas;
                let y = (iy as f64 - cote as f64*0.5)*pas;
                let prod = bg.eval(WorldPos::from_metres(x,y,0.0),t).unwrap().eta as f64;
                let mut reference = 0.0;
                for (j,c) in bg.components.iter().enumerate() {
                    let temporelle = c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32,t).0);
                    let tours = c.k_turns_per_m as f64 * (x*c.dir[0] as f64+y*c.dir[1] as f64)
                        + temporelle as f64 / 4_294_967_296.0;
                    let v = c.amplitude as f64 * (core::f64::consts::TAU*tours.fract()).sin();
                    reference += v;
                    sommes_i[j] += v;
                    carres_i[j] += v*v;
                }
                max_ecart = max_ecart.max((prod-reference).abs());
                for (j,v) in [prod,reference].into_iter().enumerate() {
                    somme[j] += v;
                    carres[j] += v*v;
                }
            }
        }
        let n = (cote*cote) as f64;
        let diag = sommes_i.iter().zip(&carres_i).map(|(s,s2)| s2/n-(s/n).powi(2)).sum();
        [carres[0]/n-(somme[0]/n).powi(2), carres[1]/n-(somme[1]/n).powi(2), diag,max_ecart]
    }

    // Moyenne de exp(i*2pi*(k.x+phase)) sur la grille centrée comme variances().
    // Réduction de l'incrément modulo un tour, puis somme géométrique finie.
    fn moyenne_cos(k: [f64;2], phase: f64, cote: usize, pas: f64) -> f64 {
        let mut centre = phase;
        let mut facteur = 1.0;
        for v in k {
            let a = v*pas;
            let r = a-a.round();
            centre += -a*cote as f64*0.5 + r*(cote-1) as f64*0.5;
            if r != 0.0 {
                facteur *= (core::f64::consts::PI*cote as f64*r).sin()
                    / (cote as f64*(core::f64::consts::PI*r).sin());
            }
        }
        facteur*(core::f64::consts::TAU*centre).cos()
    }

    // Var totale, somme des variances individuelles, contribution croisée des voisins i,i+1.
    fn moments_exacts(bg: &Background, cote: usize, pas: f64) -> [f64;3] {
        let t = SimTime::from_micros(1_735_689_600_000_000);
        let c: Vec<_> = bg.components.iter().map(|c| {
            let k = [c.k_turns_per_m as f64*c.dir[0] as f64,
                c.k_turns_per_m as f64*c.dir[1] as f64];
            let p = c.phase0.0.wrapping_sub(PhaseQ32::from_time(c.freq_q32,t).0) as f64
                / 4_294_967_296.0;
            let a = c.amplitude as f64;
            (k,p,a,a*moyenne_cos(k,p-0.25,cote,pas))
        }).collect();
        let (mut diag,mut croise,mut voisins) = (0.0,0.0,0.0);
        for (i,&(ki,pi,ai,mi)) in c.iter().enumerate() {
            for (j,&(kj,pj,aj,mj)) in c.iter().enumerate().skip(i) {
                let diff = [ki[0]-kj[0],ki[1]-kj[1]];
                let sum = [ki[0]+kj[0],ki[1]+kj[1]];
                let cov = ai*aj*0.5*(moyenne_cos(diff,pi-pj,cote,pas)
                    - moyenne_cos(sum,pi+pj,cote,pas))-mi*mj;
                if i==j { diag+=cov; } else {
                    croise+=2.0*cov;
                    if j==i+1 { voisins+=2.0*cov; }
                }
            }
        }
        [diag+croise,diag,voisins]
    }

    #[test]
    fn moments_finits_contre_sommation_et_alias() {
        for (k,p) in [([0.,0.],0.1),([1.0/3.0,0.],0.3),([0.023,-0.015],0.7)] {
            let mut somme = 0.0;
            for y in 0..16 { for x in 0..16 {
                somme += (core::f64::consts::TAU*(p+k[0]*(x as f64-8.0)*3.0
                    +k[1]*(y as f64-8.0)*3.0)).cos();
            }}
            assert!((moyenne_cos(k,p,16,3.0)-somme/256.0).abs()<1e-12);
        }
        for n in [1,32,256] {
            let bg = fond(20260905,n);
            let direct = variances(&bg,48,3.0,0.0);
            let exact = moments_exacts(&bg,48,3.0);
            assert!((direct[1]-exact[0]).abs()<1e-12);
            assert!((direct[2]-exact[1]).abs()<1e-12);
            if n==1 { assert_eq!(exact[0],exact[1]); assert_eq!(exact[2],0.0); }
        }
    }

    #[test]
    #[ignore = "diagnostic S67 : spectre dense, phases historiques et six graines ; release"]
    fn spectre_dense_s67() {
        for n in [32,256] {
            for g in [None,Some(0),Some(1),Some(2),Some(3),Some(20260905),Some(u64::MAX)] {
                let mut bg = fond(g.unwrap_or(0),n);
                if g.is_none() { for (i,c) in bg.components.iter_mut().enumerate() {
                    c.phase0 = PhaseQ32((i as u32).wrapping_mul(0x9E37_79B9));
                }}
                for cote in [1024,2048] {
                    let v = moments_exacts(&bg,cote,3.0);
                    println!("S67 n={n} g={g:?} cote={cote} Hs={:.9} diag_Hs={:.9} cross={:.9} voisins={:.9}",
                        4.0*v[0].sqrt(),4.0*v[1].sqrt(),v[0]-v[1],v[2]);
                    if n==256 && cote==1024 && (g.is_none() || g==Some(20260905)) {
                        let d = variances(&bg,cote,3.0,0.0);
                        println!("S67 direct g={g:?} Hs_prod={:.9} Hs_f64={:.9} erreur_var={:.9e}",
                            4.0*d[0].sqrt(),4.0*d[1].sqrt(),d[1]-v[0]);
                        assert!((d[1]-v[0]).abs()<1e-10);
                    }
                }
                if g.is_none() {
                    let mut beats: Vec<_> = bg.components.windows(2).map(|p| {
                        let dx=p[1].k_turns_per_m as f64*p[1].dir[0] as f64
                            -p[0].k_turns_per_m as f64*p[0].dir[0] as f64;
                        let dy=p[1].k_turns_per_m as f64*p[1].dir[1] as f64
                            -p[0].k_turns_per_m as f64*p[0].dir[1] as f64;
                        1.0/(dx*dx+dy*dy).sqrt()
                    }).collect();
                    beats.sort_by(|a,b| a.partial_cmp(b).unwrap());
                    println!("S67 n={n} battements_voisins min={:.3} max={:.3} m",beats[0],beats[beats.len()-1]);
                }
            }
        }
    }

    #[test]
    fn interferences_distinguees_de_la_precision() {
        let bg = fond(20260905,32);
        let p = variances(&bg,48,3.0,0.0);
        let l = variances(&bg,48,3.0,3000.0);
        let ratio = l[0]/p[0];
        let reference = l[1]/p[1];
        assert!((ratio-reference).abs() < 1e-4,
            "la précision ne doit pas expliquer cet écart de variance");
        assert!(reference > 1.3); // Refus nominal présent aussi en f64.
        assert!((l[2]/p[2]-1.0).abs() < 0.03); // Sans termes croisés, écart sous 3 %.
        // Témoin sans interférence : variance de la somme = variance de la seule composante.
        let mono = fond(20260905,1);
        for ox in [0.0,3000.0] {
            let v = variances(&mono,48,3.0,ox);
            assert_eq!(v[1],v[2]);
        }
    }

    #[test]
    #[ignore = "diagnostic S66 : six graines, trois fenêtres ; lancer en release"]
    fn balayer_homogeneite() {
        for graine in [0,1,2,3,20260905,u64::MAX] {
            let bg = fond(graine,32);
            for cote in [48,192,512] {
                let p = variances(&bg,cote,3.0,0.0);
                let l = variances(&bg,cote,3.0,3000.0);
                println!("S66 g={graine} cote={cote} ratio={:.9} f64={:.9} diag={:.9} proche={:.9} loin={:.9} cross_p={:.9} cross_l={:.9} max_eta={:.9e}",
                    l[0]/p[0], l[1]/p[1], l[2]/p[2],p[1],l[1],p[1]-p[2],l[1]-l[2],p[3].max(l[3]));
                assert!(p.iter().chain(&l).all(|x| x.is_finite()));
            }
        }
    }
}
