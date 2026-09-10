//! S165 : résidu sur une fenêtre ; seuls les fantômes reçoivent le total extérieur.
use super::residu::{delta_numerical, numerical, State, Terms, G};

pub struct Local {
    pub d: Vec<State>,
    d1: Vec<State>,
    rhs: Vec<State>,
    bg: Vec<State>,
    bg1: Vec<State>,
    offset: usize,
    dx: f64,
    pub courant: f64,
}

impl Local {
    pub fn new(d: Vec<State>, offset: usize, dx: f64) -> Self {
        let n = d.len();
        assert!(n >= 2 && offset > 0 && dx > 0.0);
        Self {
            d,
            d1: vec![State::default(); n],
            rhs: vec![State::default(); n],
            bg: vec![State::default(); n],
            bg1: vec![State::default(); n],
            offset,
            dx,
            courant: 0.0,
        }
    }

    // Retourne f_gauche.h-f_droite.h et la célérité maximale, fantômes compris.
    fn rhs(
        bg: &[State],
        d: &[State],
        out: &mut [State],
        bg_ghost: [State; 2],
        total_ghost: [State; 2],
        dx: f64,
    ) -> (f64, f64) {
        let n = d.len();
        let face = |bl, br, dl, dr| {
            numerical(bl, br).plus(delta_numerical(bl, br, dl, dr, 1.0, Terms::Complete))
        };
        let mut left = face(bg_ghost[0], bg[0], total_ghost[0].minus(bg_ghost[0]), d[0]);
        let first = left.h;
        let speed = |s: State| {
            assert!(s.h.is_finite() && s.q.is_finite() && s.h > 1e-8);
            (s.q / s.h).abs() + (G * s.h).sqrt()
        };
        let mut maximum = speed(total_ghost[0])
            .max(speed(total_ghost[1]))
            .max(speed(bg_ghost[0]))
            .max(speed(bg_ghost[1]));
        for i in 0..n {
            let (br, dr) = if i + 1 == n {
                (bg_ghost[1], total_ghost[1].minus(bg_ghost[1]))
            } else {
                (bg[i + 1], d[i + 1])
            };
            let right = face(bg[i], br, d[i], dr);
            out[i] = left.minus(right).times(1.0 / dx);
            maximum = maximum.max(speed(bg[i].plus(d[i]))).max(speed(bg[i]));
            left = right;
        }
        (first - left.h, maximum)
    }

    /// Renvoie l'intégrale temporelle du flux net de masse (pas une conservation fermée).
    pub fn step(
        &mut self,
        t: f64,
        dt: f64,
        sample: impl Fn(usize, f64) -> State,
        ghosts: [[State; 2]; 2],
    ) -> f64 {
        assert!(dt.is_finite() && dt > 0.0 && t.is_finite());
        let n = self.d.len();
        for i in 0..n {
            self.bg[i] = sample(self.offset + i, t);
            self.bg1[i] = sample(self.offset + i, t + dt);
        }
        let bg_ghost = |time| [sample(self.offset - 1, time), sample(self.offset + n, time)];
        let (f0, c0) = Self::rhs(
            &self.bg,
            &self.d,
            &mut self.rhs,
            bg_ghost(t),
            ghosts[0],
            self.dx,
        );
        for i in 0..n {
            self.d1[i] = self.d[i]
                .plus(self.rhs[i].times(dt))
                .minus(self.bg1[i].minus(self.bg[i]));
        }
        let (f1, c1) = Self::rhs(
            &self.bg1,
            &self.d1,
            &mut self.rhs,
            bg_ghost(t + dt),
            ghosts[1],
            self.dx,
        );
        for i in 0..n {
            self.d[i] = self.d[i]
                .plus(self.d1[i])
                .plus(self.rhs[i].times(dt))
                .minus(self.bg1[i].minus(self.bg[i]))
                .times(0.5);
            let total = self.bg1[i].plus(self.d[i]);
            assert!(total.h.is_finite() && total.q.is_finite() && total.h > 1e-8);
        }
        self.courant = self.courant.max(dt / self.dx * c0.max(c1));
        assert!(self.courant <= 0.45);
        0.5 * dt * (f0 + f1)
    }
}
