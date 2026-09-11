//! Fermetures subcritiques S166/S169.
use super::residu::{State, G};

/// L'invariant sortant transporte son écart au fond local, puis est réancré au fantôme.
pub fn anchored(
    inner: [State; 2],
    background_inner: [State; 2],
    background_ghost: [State; 2],
) -> [State; 2] {
    std::array::from_fn(|side| {
        invariants(inner[side]);
        invariants(background_inner[side]);
        invariants(background_ghost[side]);
        let a = inner[side];
        let b = background_inner[side];
        let q = background_ghost[side];
        let direction = if side == 0 { -1.0 } else { 1.0 };
        let dr = a.q / a.h - b.q / b.h + direction * 2.0 * ((G * a.h).sqrt() - (G * b.h).sqrt());
        let du = 0.5 * dr;
        let dc = direction * 0.25 * dr;
        let c = (G * q.h).sqrt();
        assert!(c + dc > 0.0);
        let dh = (2.0 * c * dc + dc * dc) / G;
        let dq = q.h * du + (q.q / q.h) * dh + dh * du;
        let result = q.plus(State { h: dh, q: dq });
        invariants(result);
        result
    })
}
pub fn invariants(s: State) -> (f64, f64) {
    assert!(s.h.is_finite() && s.q.is_finite() && s.h > 1e-8);
    let u = s.q / s.h;
    let c = (G * s.h).sqrt();
    assert!(u.abs() < c, "fermeture subcritique seulement");
    (u + 2.0 * c, u - 2.0 * c)
}
pub fn characteristic(inner: [State; 2], outside: [State; 2]) -> [State; 2] {
    std::array::from_fn(|side| {
        let (ip, im) = invariants(inner[side]);
        let (op, om) = invariants(outside[side]);
        let (p, m) = if side == 0 { (op, im) } else { (ip, om) };
        let c = (p - m) * 0.25;
        assert!(c > 0.0 && c.is_finite());
        let u = (p + m) * 0.5;
        let h = c * c / G;
        let result = State { h, q: h * u };
        invariants(result);
        result
    })
}
