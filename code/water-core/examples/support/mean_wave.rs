//! Quadratures indépendantes S168 partagées avec S169.
use super::residu::{State, G};
use super::simple_wave::Wave;
pub fn integral(f: impl Fn(f64) -> State, a: f64, b: f64, tol: f64) -> State {
    fn panel(a: f64, b: f64, fa: State, fm: State, fb: State) -> State {
        fa.plus(fm.times(4.0)).plus(fb).times((b - a) / 6.0)
    }
    fn recurse(
        f: &impl Fn(f64) -> State,
        a: f64,
        b: f64,
        v: [State; 3],
        whole: State,
        tol: f64,
        depth: usize,
    ) -> State {
        let mid = (a + b) * 0.5;
        let l = f((a + mid) * 0.5);
        let r = f((mid + b) * 0.5);
        let left = panel(a, mid, v[0], l, v[1]);
        let right = panel(mid, b, v[1], r, v[2]);
        let sum = left.plus(right);
        let error = sum.minus(whole);
        assert!(error.h.is_finite() && error.q.is_finite());
        if error.h.abs().max(error.q.abs()) <= 15.0 * tol {
            return sum.plus(error.times(1.0 / 15.0));
        }
        assert!(depth > 0, "quadrature non convergente");
        recurse(f, a, mid, [v[0], l, v[1]], left, tol * 0.5, depth - 1).plus(recurse(
            f,
            mid,
            b,
            [v[1], r, v[2]],
            right,
            tol * 0.5,
            depth - 1,
        ))
    }
    assert!(a.is_finite() && b.is_finite() && b > a && tol > 0.0 && tol.is_finite());
    let v = [f(a), f((a + b) * 0.5), f(b)];
    recurse(&f, a, b, v, panel(a, b, v[0], v[1], v[2]), tol, 16)
}
pub fn flux(s: State) -> State {
    State {
        h: s.q,
        q: s.q * s.q / s.h + 0.5 * G * s.h * s.h,
    }
}
pub fn value(w: Wave, i: usize, dx: f64, t: f64, mean: bool, tol: f64) -> State {
    if mean {
        integral(|x| w.at(x, t), i as f64 * dx, (i + 1) as f64 * dx, tol).times(1.0 / dx)
    } else {
        w.at((i as f64 + 0.5) * dx, t)
    }
}
