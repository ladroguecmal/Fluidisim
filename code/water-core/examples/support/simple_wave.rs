//! Référence analytique S166, partagée avec S167.
use super::residu::{State, G};
#[derive(Clone, Copy)]
pub struct Wave {
    pub a: f64,
    pub center: f64,
    pub sign: f64,
}
impl Wave {
    pub fn at(self, x: f64, t: f64) -> State {
        assert!(self.a >= 0.0 && t >= 0.0);
        let slope_bound = 1.5 * G.sqrt() * self.a * (2.0 / std::f64::consts::E).sqrt() / 8.0;
        assert!(
            t * slope_bound < 1.0,
            "reference apres croisement des caracteristiques"
        );
        let xp = self.sign * (x - self.center);
        let h0 = |y: f64| 1.0 + self.a * (-(y / 8.0).powi(2)).exp();
        let mut lo = xp - (3.0 * (G * (1.0 + self.a)).sqrt() - 2.0 * G.sqrt()) * t;
        let mut hi = xp - G.sqrt() * t;
        for _ in 0..48 {
            let y = (lo + hi) * 0.5;
            if y + (3.0 * (G * h0(y)).sqrt() - 2.0 * G.sqrt()) * t > xp {
                hi = y;
            } else {
                lo = y;
            }
        }
        let h = h0((lo + hi) * 0.5);
        State {
            h,
            q: h * self.sign * 2.0 * ((G * h).sqrt() - G.sqrt()),
        }
    }
}
