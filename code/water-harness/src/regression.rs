//! Régression centrée commune ; sélection des points et changement d'unités chez l'appelant.

/// Pente et R² d'une droite avec intercept. Une paire inexploitable refuse toute la série.
/// Les abscisses constantes ne définissent pas de pente ; les ordonnées constantes pas de R².
pub fn centree(points: &[(f64, f64)]) -> Option<(f64, f64)> {
    if points.len() < 2 || points.iter().any(|(x, y)| !x.is_finite() || !y.is_finite()) {
        return None;
    }
    let n = points.len() as f64;
    let (sx, sy) = points.iter().fold((0.0, 0.0), |(x, y), p| (x + p.0, y + p.1));
    let (mx, my) = (sx / n, sy / n);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for (x, y) in points {
        sxy += (x - mx) * (y - my);
        sxx += (x - mx) * (x - mx);
        syy += (y - my) * (y - my);
    }
    if ![sxy, sxx, syy].iter().all(|v| v.is_finite()) || sxx <= 0.0 || syy <= 0.0 {
        return None;
    }
    let pente = sxy / sxx;
    let num = sxy * sxy;
    let den = sxx * syy;
    if !num.is_finite() || !den.is_finite() || den <= 0.0 { return None; }
    let r2 = num / den;
    if pente.is_finite() && r2.is_finite() { Some((pente, r2)) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn droite_et_residus_connus() {
        assert_eq!(centree(&[(0.0, 3.0), (1.0, 1.0), (2.0, -1.0)]), Some((-2.0, 1.0)));
        // y=x+résidu, résidus (1,-2,1) orthogonaux à x centré : SSE=6, SST=8.
        assert_eq!(centree(&[(0.0, 1.0), (1.0, -1.0), (2.0, 3.0)]), Some((1.0, 0.25)));
        // Covariance nulle mais variances non nulles : pente zéro et R² zéro sont mesurables.
        assert_eq!(centree(&[(-1.0, 1.0), (0.0, -2.0), (1.0, 1.0)]), Some((0.0, 0.0)));
    }

    #[test]
    fn serie_inexploitable_refusee_sans_retirer_de_point() {
        for p in [vec![], vec![(0.0, 1.0)], vec![(1.0, 1.0), (1.0, 2.0)],
            vec![(0.0, 1.0), (1.0, 1.0)],
            vec![(0.0, 0.0), (1.0, f64::NAN), (2.0, 2.0)],
            vec![(0.0, 0.0), (f64::INFINITY, 1.0)],
            vec![(-f64::MAX, 0.0), (f64::MAX, 1.0)],
            vec![(-1e100, -1e100), (1e100, 1e100)]] {
            assert!(centree(&p).is_none(), "{p:?}");
        }
    }

    #[test]
    fn logarithmes_et_unites_restent_chez_appelant() {
        let t: Vec<_> = (0..6).map(|i| {
            let x = i as f64;
            (x, (-0.5*x).exp().ln())
        }).collect();
        let x: Vec<_> = (0..6).map(|i| {
            let x = i as f64;
            (x, 2f64.powf(-x/4.0).log2())
        }).collect();
        let (taux, r2) = centree(&t).unwrap();
        assert!((std::f64::consts::LN_2 / -taux - 2.0*std::f64::consts::LN_2).abs() < 1e-14);
        assert!((r2-1.0).abs() < 1e-14);
        let (pente, r2) = centree(&x).unwrap();
        assert!((-1.0/pente - 4.0).abs() < 1e-14);
        assert!((r2-1.0).abs() < 1e-14);
    }
}
