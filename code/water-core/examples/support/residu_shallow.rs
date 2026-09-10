//! Véhicule S163, lit plat et mouillé, hors runtime ; aucun appel au solveur total.
//! Flux dérivés dans docs/validation/RESIDU-COUPLE-S163.md.
pub const G: f64 = water_core::shallow::G;

#[derive(Clone, Copy, Debug, Default)]
pub struct State {
    pub h: f64,
    pub q: f64,
}
impl State {
    pub fn plus(self, b: Self) -> Self {
        Self {
            h: self.h + b.h,
            q: self.q + b.q,
        }
    }
    pub fn minus(self, b: Self) -> Self {
        Self {
            h: self.h - b.h,
            q: self.q - b.q,
        }
    }
    pub fn times(self, a: f64) -> Self {
        Self {
            h: self.h * a,
            q: self.q * a,
        }
    }
    fn reflected(self) -> Self {
        Self {
            h: self.h,
            q: -self.q,
        }
    }
    fn speed(self) -> f64 {
        assert!(
            self.h.is_finite() && self.q.is_finite() && self.h > 1e-8,
            "hors domaine mouille : {self:?}"
        );
        (self.q / self.h).abs() + (G * self.h).sqrt()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Background {
    Evolving,
    Frozen,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terms {
    Complete,
    NoPressureCross,
    NoNumericalCross,
    NoSource,
}

/// S164 : choix de la source temporelle pour un fond prescrit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Temporal {
    Discrete,
    Continuous,
    Omitted,
}

fn physical(a: State) -> State {
    State {
        h: a.q,
        q: a.q * (a.q / a.h) + 0.5 * G * a.h * a.h,
    }
}
pub fn numerical(l: State, r: State) -> State {
    let speed = l.speed().max(r.speed());
    physical(l)
        .plus(physical(r))
        .times(0.5)
        .minus(r.minus(l).times(0.5 * speed))
}
fn delta_physical(bg: State, d: State, h0: f64, terms: Terms) -> State {
    let u = bg.q / bg.h;
    let adv = 2.0 * u * d.q - u * u * d.h + (d.q - u * d.h).powi(2) / (bg.h + d.h);
    // Retirer seulement la pression croisée avec l'anomalie du fond, pas gh0*z.
    let height = if terms == Terms::NoPressureCross {
        h0
    } else {
        bg.h
    };
    State {
        h: d.q,
        q: adv + G * height * d.h + 0.5 * G * d.h * d.h,
    }
}
pub fn delta_numerical(bl: State, br: State, dl: State, dr: State, h0: f64, terms: Terms) -> State {
    let ab = bl.speed().max(br.speed());
    let at = bl.plus(dl).speed().max(br.plus(dr).speed());
    let cross = if terms == Terms::NoNumericalCross {
        State::default()
    } else {
        br.minus(bl).times(at - ab)
    };
    delta_physical(bl, dl, h0, terms)
        .plus(delta_physical(br, dr, h0, terms))
        .times(0.5)
        .minus(dr.minus(dl).times(at).plus(cross).times(0.5))
}

pub struct Coupled {
    pub bg: Vec<State>,
    pub d: Vec<State>,
    b1: Vec<State>,
    d1: Vec<State>,
    lb: Vec<State>,
    ld: Vec<State>,
    dx: f64,
    h0: f64,
    pub mode: Background,
    pub terms: Terms,
    pub max_courant: f64,
}
impl Coupled {
    /// S164 : fond reçu par callback (état, dérivée temporelle) ; aucun total de référence.
    /// Q1=Q(t+dt)=Q+, donc les deux corrections discrètes sont Q1-Q0.
    pub fn step_prescribed(
        &mut self,
        t: f64,
        dt: f64,
        temporal: Temporal,
        sample: impl Fn(usize, f64) -> (State, State),
    ) {
        assert!(t.is_finite() && dt.is_finite() && dt > 0.0);
        assert_eq!(self.mode, Background::Frozen);
        assert_eq!(self.terms, Terms::Complete);
        let n = self.bg.len();
        for i in 0..n {
            let (q0, derivative) = sample(i, t);
            self.bg[i] = q0;
            // b1 temporaire : dérivée à t, utilisée après rhs.
            self.b1[i] = derivative;
        }
        let c1 = Self::rhs(
            &self.bg,
            &self.d,
            &mut self.lb,
            &mut self.ld,
            self.dx,
            self.h0,
            Background::Frozen,
            Terms::Complete,
            dt,
        );
        for i in 0..n {
            let (q1, derivative1) = sample(i, t + dt);
            let increment = q1.minus(self.bg[i]);
            let correction = match temporal {
                Temporal::Discrete => increment,
                Temporal::Continuous => self.b1[i].times(dt),
                Temporal::Omitted => State::default(),
            };
            self.d1[i] = self.d[i].plus(self.ld[i].times(dt)).minus(correction);
            self.b1[i] = q1;
            // hôte indépendant, pas de nouvelle allocation : lb sert au second correcteur.
            self.lb[i] = match temporal {
                Temporal::Discrete => increment,
                Temporal::Continuous => derivative1.times(dt),
                Temporal::Omitted => State::default(),
            };
        }
        // rhs écrase lb : conserver les correcteurs dans bg (Q0 n'est plus nécessaire).
        self.bg.copy_from_slice(&self.lb);
        let c2 = Self::rhs(
            &self.b1,
            &self.d1,
            &mut self.lb,
            &mut self.ld,
            self.dx,
            self.h0,
            Background::Frozen,
            Terms::Complete,
            dt,
        );
        for i in 0..n {
            self.d[i] = self.d[i]
                .plus(self.d1[i])
                .plus(self.ld[i].times(dt))
                .minus(self.bg[i])
                .times(0.5);
            self.bg[i] = self.b1[i];
            self.bg[i].speed();
            self.bg[i].plus(self.d[i]).speed();
        }
        self.max_courant = self.max_courant.max(c1).max(c2);
    }
    pub fn new(
        bg: Vec<State>,
        d: Vec<State>,
        dx: f64,
        h0: f64,
        mode: Background,
        terms: Terms,
    ) -> Self {
        let n = bg.len();
        assert!(n >= 2 && d.len() == n && dx > 0.0 && h0 > 0.0);
        Self {
            bg,
            d,
            b1: vec![State::default(); n],
            d1: vec![State::default(); n],
            lb: vec![State::default(); n],
            ld: vec![State::default(); n],
            dx,
            h0,
            mode,
            terms,
            max_courant: 0.0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rhs(
        bg: &[State],
        d: &[State],
        lb: &mut [State],
        ld: &mut [State],
        dx: f64,
        h0: f64,
        mode: Background,
        terms: Terms,
        dt: f64,
    ) -> f64 {
        let n = bg.len();
        let mut courant = 0.0_f64;
        let mut left_b = numerical(bg[0].reflected(), bg[0]);
        let mut left_d =
            delta_numerical(bg[0].reflected(), bg[0], d[0].reflected(), d[0], h0, terms);
        for i in 0..n {
            let br = if i + 1 == n {
                bg[i].reflected()
            } else {
                bg[i + 1]
            };
            let dr = if i + 1 == n {
                d[i].reflected()
            } else {
                d[i + 1]
            };
            let right_b = numerical(bg[i], br);
            let right_d = delta_numerical(bg[i], br, d[i], dr, h0, terms);
            let source = left_b.minus(right_b).times(1.0 / dx);
            lb[i] = if mode == Background::Evolving {
                source
            } else {
                State::default()
            };
            ld[i] = left_d.minus(right_d).times(1.0 / dx);
            if mode == Background::Frozen && terms != Terms::NoSource {
                ld[i] = ld[i].plus(source);
            }
            left_b = right_b;
            left_d = right_d;
            courant = courant.max(dt / dx * bg[i].speed().max(bg[i].plus(d[i]).speed()));
        }
        assert!(
            courant.is_finite() && courant <= 0.45,
            "Courant hors budget : {courant}"
        );
        courant
    }

    pub fn step(&mut self, dt: f64) {
        assert!(dt.is_finite() && dt > 0.0);
        let n = self.bg.len();
        let c1 = Self::rhs(
            &self.bg,
            &self.d,
            &mut self.lb,
            &mut self.ld,
            self.dx,
            self.h0,
            self.mode,
            self.terms,
            dt,
        );
        for i in 0..n {
            self.b1[i] = self.bg[i].plus(self.lb[i].times(dt));
            self.d1[i] = self.d[i].plus(self.ld[i].times(dt));
        }
        let c2 = Self::rhs(
            &self.b1,
            &self.d1,
            &mut self.lb,
            &mut self.ld,
            self.dx,
            self.h0,
            self.mode,
            self.terms,
            dt,
        );
        for i in 0..n {
            self.bg[i] = self.bg[i]
                .plus(self.b1[i])
                .plus(self.lb[i].times(dt))
                .times(0.5);
            self.d[i] = self.d[i]
                .plus(self.d1[i])
                .plus(self.ld[i].times(dt))
                .times(0.5);
            // Aucun max(0) ni saturation : un état invalide fait échouer l'expérience.
            self.bg[i].speed();
            self.bg[i].plus(self.d[i]).speed();
        }
        self.max_courant = self.max_courant.max(c1).max(c2);
    }
}
