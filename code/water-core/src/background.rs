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
                // Déphasage dérivé de l'indice : reproductible, sans PRNG et sans horloge.
                phase0: PhaseQ32((i as u32).wrapping_mul(0x9E37_79B9)),
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
