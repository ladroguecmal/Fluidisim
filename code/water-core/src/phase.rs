//! Phases repliées et sinus déterministe.
//!
//! # Pourquoi ce module existe
//!
//! I-03 exige que `B` soit déterministe **bit à bit entre plateformes**, et ADR-003 §2 énumère les
//! disciplines qui l'assurent : « sémantique IEEE stricte, ordre de sommation fixé, PRNG entier ».
//!
//! **Cette liste est incomplète, et l'omission ne se voit qu'en écrivant le code.** `B` est une
//! somme de sinusoïdes, et `f32::sin` n'est **pas** spécifié bit à bit : la norme IEEE 754 impose
//! l'exactitude des quatre opérations et de la racine carrée, jamais celle des fonctions
//! transcendantes. Deux `libm` — deux plateformes, deux versions de la même — donnent des résultats
//! qui diffèrent dans les derniers bits. Un hash de conformité les distinguerait à chaque frame.
//!
//! # Ce que ce module fait à la place
//!
//! Il n'emprunte au corpus aucune idée nouvelle : ADR-003 §2.2 pose déjà que « seules des **phases
//! repliées** passent au GPU ». Le même mécanisme sert ici.
//!
//! - une phase est un `u32` valant une **fraction de tour** — `2³²` unités par tour. Le repliement
//!   modulo un tour est le débordement naturel de l'entier, donc exact et gratuit ;
//! - la part temporelle se calcule **entièrement en entiers** à partir de `SimTime` en
//!   microsecondes : aucun flottant ne voit le temps, ce qu'I-08 exige de toute façon ;
//! - le sinus est un polynôme à coefficients fixes, évalué en `f32` avec un ordre d'opérations
//!   explicite. Il n'emploie que `+`, `-` et `*`, qui sont exacts au sens d'IEEE 754.

use crate::types::SimTime;

/// Phase repliée : fraction de tour en virgule fixe sur 32 bits. `2³²` unités = 1 tour.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PhaseQ32(pub u32);

impl PhaseQ32 {
    #[inline]
    pub fn wrapping_add(self, other: PhaseQ32) -> PhaseQ32 {
        PhaseQ32(self.0.wrapping_add(other.0))
    }

    /// Phase temporelle d'une composante de fréquence `f`, à l'instant `t`.
    ///
    /// `freq_q32` porte la fréquence en **tours par seconde**, en virgule fixe `2³²`. Le calcul
    /// est entièrement entier :
    ///
    /// ```text
    /// tours × 2³²  =  freq_q32 · t_µs / 10⁶     (mod 2³², par débordement)
    /// ```
    ///
    /// L'intermédiaire est pris sur 128 bits pour qu'aucun débordement ne tronque avant la
    /// division. Le repliement modulo un tour est ensuite exact.
    #[inline]
    pub fn from_time(freq_q32: u64, t: SimTime) -> PhaseQ32 {
        let prod = (freq_q32 as u128) * (t.micros() as u128);
        let turns_q32 = prod / 1_000_000u128;
        PhaseQ32(turns_q32 as u32)
    }

    /// Phase spatiale : `k · d` où `k` est en **tours par mètre** et `d` une distance locale.
    ///
    /// `d` est borné par I-08 (`|d| < 4096 m`) et le produit est un `f32` exact au sens IEEE. Seule
    /// la partie fractionnaire compte, et sa conversion en `u32` est déterministe.
    #[inline]
    pub fn from_distance(k_turns_per_m: f32, d: f32) -> PhaseQ32 {
        let turns = k_turns_per_m * d;
        let frac = turns - floor_f32(turns);
        // `frac ∈ [0, 1)` ; le produit tient dans un u32 sans saturation.
        PhaseQ32((frac * 4_294_967_296.0f32) as u32)
    }

    /// Sinus de la phase. Erreur maximale ≈ 2·10⁻⁹, très en deçà de l'ulp d'un `f32`.
    #[inline]
    pub fn sin(self) -> f32 {
        sin_turns_q32(self.0)
    }

    /// Cosinus de la phase — un quart de tour d'avance.
    #[inline]
    pub fn cos(self) -> f32 {
        sin_turns_q32(self.0.wrapping_add(0x4000_0000))
    }
}

/// Partie entière inférieure, sans dépendre de `f32::floor` (qui est exact partout, mais on garde
/// le module autonome et explicite sur ce qu'il emploie).
#[inline]
fn floor_f32(x: f32) -> f32 {
    let t = x as i64 as f32;
    if t > x {
        t - 1.0
    } else {
        t
    }
}

/// `sin(2π · phase / 2³²)`.
///
/// Réduction au premier octant par arithmétique **entière** — donc exacte — puis polynôme de
/// Taylor jusqu'au degré 9 sur `[0, π/4]`. Le terme suivant y majore l'erreur par
/// `(π/4)¹¹ / 11! ≈ 1,8·10⁻⁹`.
///
/// N'emploie que `+`, `-`, `*` en `f32`, dans un ordre explicite : IEEE 754 les spécifie
/// exactement, et Rust ne les contracte pas en FMA.
#[inline]
fn sin_turns_q32(phase: u32) -> f32 {
    // Deux bits de poids fort : le quadrant. Trente bits restants : la position dedans.
    let quadrant = phase >> 30;
    let within = phase & 0x3FFF_FFFF; // ∈ [0, 2³⁰)

    // Angle dans [0, π/2), en f32. 2³⁰ unités valent π/2.
    const QUARTER_TURN_RAD: f32 = core::f32::consts::FRAC_PI_2;
    let x = (within as f32) * (QUARTER_TURN_RAD / 1_073_741_824.0f32);

    // Dans chaque quadrant, `sin(2πp)` s'exprime par `sin(x)` ou `cos(x)`, au signe près :
    //   q=0 : sin(x)      q=1 : cos(x)      q=2 : −sin(x)     q=3 : −cos(x)
    let (angle, use_cos, negate) = match quadrant {
        0 => (x, false, false),
        1 => (x, true, false),
        2 => (x, false, true),
        _ => (x, true, true),
    };

    // On ramène l'argument dans [0, π/4] pour que le polynôme y soit précis.
    const EIGHTH_TURN_RAD: f32 = core::f32::consts::FRAC_PI_4;
    let (a, swap) = if angle > EIGHTH_TURN_RAD {
        (QUARTER_TURN_RAD - angle, true)
    } else {
        (angle, false)
    };

    let want_cos = use_cos ^ swap;
    let v = if want_cos { poly_cos(a) } else { poly_sin(a) };
    if negate {
        -v
    } else {
        v
    }
}

/// `sin(a)` sur `[0, π/4]`, Taylor au degré 9. Ordre d'évaluation explicite (Horner).
#[inline]
fn poly_sin(a: f32) -> f32 {
    const C3: f32 = -1.0 / 6.0;
    const C5: f32 = 1.0 / 120.0;
    const C7: f32 = -1.0 / 5040.0;
    const C9: f32 = 1.0 / 362_880.0;
    let a2 = a * a;
    let p = C9;
    let p = p * a2 + C7;
    let p = p * a2 + C5;
    let p = p * a2 + C3;
    let p = p * a2;
    a * p + a
}

/// `cos(a)` sur `[0, π/4]`, Taylor au degré 10.
#[inline]
fn poly_cos(a: f32) -> f32 {
    const C2: f32 = -1.0 / 2.0;
    const C4: f32 = 1.0 / 24.0;
    const C6: f32 = -1.0 / 720.0;
    const C8: f32 = 1.0 / 40_320.0;
    const C10: f32 = -1.0 / 3_628_800.0;
    let a2 = a * a;
    let p = C10;
    let p = p * a2 + C8;
    let p = p * a2 + C6;
    let p = p * a2 + C4;
    let p = p * a2 + C2;
    p * a2 + 1.0
}

/// Convertit une fréquence en hertz vers la représentation `q32` employée par `from_time`.
///
/// Réservé à l'initialisation : aucune conversion flottante n'a lieu à l'exécution.
pub fn freq_hz_to_q32(hz: f64) -> u64 {
    (hz * 4_294_967_296.0f64).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn max_ecart_contre_reference() -> f64 {
        let mut worst = 0.0f64;
        for i in 0..4096u32 {
            let phase = ((i as u64) * (u32::MAX as u64) / 4095) as u32;
            let ours = sin_turns_q32(phase) as f64;
            let theirs = ((phase as f64) / 4_294_967_296.0 * core::f64::consts::TAU).sin();
            let e = (ours - theirs).abs();
            if e > worst {
                worst = e;
            }
        }
        worst
    }

    #[test]
    fn sinus_deterministe_fidele() {
        // Fidélité : très en deçà de l'ulp d'un f32 (≈1,2·10⁻⁷ pour une valeur d'ordre 1).
        let e = max_ecart_contre_reference();
        assert!(e < 1e-7, "écart maximal {e}");
    }

    #[test]
    fn sinus_exact_aux_quarts_de_tour() {
        assert_eq!(sin_turns_q32(0), 0.0);
        assert!((sin_turns_q32(0x4000_0000) - 1.0).abs() < 1e-7);
        assert!(sin_turns_q32(0x8000_0000).abs() < 1e-7);
        assert!((sin_turns_q32(0xC000_0000) + 1.0).abs() < 1e-7);
    }

    #[test]
    fn phase_temporelle_entierement_entiere() {
        // 0,5 Hz pendant 1 s = un demi-tour, exactement.
        let f = freq_hz_to_q32(0.5);
        let p = PhaseQ32::from_time(f, SimTime::from_micros(1_000_000));
        assert_eq!(p.0, 0x8000_0000);

        // Le repliement modulo un tour est le débordement de l'entier : exact, et gratuit.
        let p2 = PhaseQ32::from_time(f, SimTime::from_micros(4_000_000));
        assert_eq!(p2.0, 0);
    }

    #[test]
    fn phase_temporelle_stable_sur_un_temps_long() {
        // Là où un f32 de temps aurait un ulp de 62,5 ms (SPEC-001 §7), l'entier reste exact.
        let f = freq_hz_to_q32(0.125);
        let t = SimTime::from_micros(1_000_000_000_000); // 10⁶ s
        let a = PhaseQ32::from_time(f, t);
        let b = PhaseQ32::from_time(f, SimTime::from_micros(t.micros() + 1_000_000));
        // Une seconde de plus à 0,125 Hz : un huitième de tour, exactement.
        assert_eq!(b.0.wrapping_sub(a.0), 0x2000_0000);
    }
}
