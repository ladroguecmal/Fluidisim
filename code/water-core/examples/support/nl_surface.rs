//! Véhicule x-z **non linéaire** S193, hors runtime. Voir SURFACE-LIBRE-NL-S193.md.
//!
//! État spectral en bande `q = 0..=Q` (symétrie hermitienne implicite), profondeur
//! résolue par le relèvement tridiagonal de S192 avec symbole horizontal exact
//! `mu = k_q² dz²`. Produits par convolution tronquée à la bande : la troncature est une
//! projection exacte, il n'y a aucun repliement. Ordre en amplitude `M ∈ {1,2,3}`.

/// Mode complexe : `[réel, imaginaire]`.
pub type C = [f64; 2];

fn cadd(a: C, b: C) -> C {
    [a[0] + b[0], a[1] + b[1]]
}
fn cmul(a: C, b: C) -> C {
    [a[0] * b[0] - a[1] * b[1], a[0] * b[1] + a[1] * b[0]]
}
fn cscale(a: C, s: f64) -> C {
    [a[0] * s, a[1] * s]
}
fn cconj(a: C) -> C {
    [a[0], -a[1]]
}
pub fn cabs(a: C) -> f64 {
    a[0].hypot(a[1])
}

#[derive(Clone)]
pub struct NlSurface {
    /// Dernier mode retenu : la bande est `q = -Q..Q`, stockée pour `q = 0..=Q`.
    pub band: usize,
    /// Intervalles verticaux du relèvement.
    pub levels: usize,
    pub l: f64,
    pub h: f64,
    pub g: f64,
    /// Ordre en amplitude retenu, 1, 2 ou 3.
    pub order: usize,
    /// `k_q = 2πq/L`.
    pub wave: Vec<f64>,
    /// Symbole de Dirichlet-Neumann discret `G_h(k_q)`, opérateur `A`.
    pub dn: Vec<f64>,
    eta: Vec<C>,
    psi: Vec<C>,
}

impl NlSurface {
    pub fn new(
        band: usize,
        levels: usize,
        l: f64,
        h: f64,
        g: f64,
        order: usize,
        eta: &[C],
        psi: &[C],
    ) -> Result<Self, &'static str> {
        if band < 1
            || levels < 2
            || !(1..=3).contains(&order)
            || eta.len() != band + 1
            || psi.len() != band + 1
            || [l, h, g].iter().any(|v| !v.is_finite() || *v <= 0.)
            || eta
                .iter()
                .chain(psi)
                .flatten()
                .any(|v: &f64| !v.is_finite())
        {
            return Err("configuration");
        }
        let dz = h / levels as f64;
        let mut wave = Vec::with_capacity(band + 1);
        let mut dn = Vec::with_capacity(band + 1);
        for q in 0..=band {
            let kq = std::f64::consts::TAU * q as f64 / l;
            wave.push(kq);
            dn.push(Self::lift_symbol(kq, levels, dz));
        }
        if wave.iter().chain(&dn).any(|v| !v.is_finite()) || dn[0] != 0. || dn[1..].iter().any(|v| *v <= 0.)
        {
            return Err("relèvement");
        }
        let mut me = Self {
            band,
            levels,
            l,
            h,
            g,
            order,
            wave,
            dn,
            eta: eta.to_vec(),
            psi: psi.to_vec(),
        };
        // Jauge : psi_0 n'intervient dans aucun terme (G_h(0)=0 et k_0=0), il est fixé à zéro
        // pour que l'énergie du système tronqué soit définie sans ambiguïté.
        me.psi[0] = [0.; 2];
        Ok(me)
    }

    /// Relèvement tridiagonal de S192, `mu = k² dz²` : `K` inconnues dans la profondeur,
    /// fond imperméable par l'image `phi_{-1}=phi_1`, flux de surface en demi-volume.
    fn lift_symbol(kq: f64, levels: usize, dz: f64) -> f64 {
        let mu = kq * kq * dz * dz;
        let mut diag = vec![2. + mu; levels];
        diag[0] = 1. + mu / 2.;
        let mut rhs = vec![0.; levels];
        rhs[levels - 1] = 1.;
        for j in 1..levels {
            let f = -1. / diag[j - 1];
            diag[j] += f;
            rhs[j] -= f * rhs[j - 1];
        }
        let mut shape = vec![1.; levels + 1];
        shape[levels - 1] = rhs[levels - 1] / diag[levels - 1];
        for j in (0..levels - 1).rev() {
            shape[j] = (rhs[j] + shape[j + 1]) / diag[j];
        }
        (1. - shape[levels - 1]) / dz + dz * kq * kq / 2.
    }

    /// **A242, S197.** Écart relatif maximal entre le symbole **discret** que ce véhicule
    /// emploie et le symbole **continu** `k·tanh(k·h)` qu'il approche, sur les modes
    /// `1..=upto`. Rend `(écart, mode fautif)`.
    ///
    /// Le module vérifiait déjà que `G_h` tend vers `k tanh(k h)` quand `K` croît — mais à
    /// `K = 512` sur une bande de 8, une configuration que personne n'exécute. Il ne disait
    /// nulle part ce que vaut l'écart **à la configuration employée**, et c'est là que
    /// S196 s'est fait prendre : un symbole faux donne une évolution lisse, conservative et
    /// fausse, qu'aucun critère d'énergie ne signale (**L277**).
    ///
    /// À déclarer par tout banc, **à côté** de sa dérive d'énergie et jamais à sa place.
    /// Gratuit : le symbole est précalculé, `K` n'entre pas dans le coût d'un pas.
    pub fn dispersion_error(&self, upto: usize) -> (f64, usize) {
        let top = upto.min(self.band);
        let mut worst = (0., 0);
        for q in 1..=top {
            let exact = self.wave[q] * (self.wave[q] * self.h).tanh();
            let rel = (self.dn[q] - exact).abs() / exact;
            if rel > worst.0 {
                worst = (rel, q);
            }
        }
        worst
    }

    /// **A242, S197.** Ordre et résidu de Richardson d'un triplet de raffinement, ou `None`
    /// si le triplet ne converge pas — incréments de signes opposés, ou second incrément
    /// plus grand que le premier. Dans ce cas le niveau grossier est hors de son domaine,
    /// et la formule rendrait un chiffre d'apparence excellente sans objet : S196 a vu
    /// « ordre 6,552 » sortir d'un niveau faux d'un facteur cinq.
    ///
    /// Rend `(ordre, résidu relatif au niveau fin)`. Même famille qu'**A238** et **L274**.
    pub fn richardson(coarse: f64, mid: f64, fine: f64) -> Option<(f64, f64)> {
        let (d1, d2) = (mid - coarse, fine - mid);
        if d1 * d2 <= 0. || d1.abs() <= d2.abs() || fine == 0. {
            return None;
        }
        let order = (d1 / d2).abs().log2();
        Some((order, (d2 / (2f64.powf(order) - 1.)).abs() / fine.abs()))
    }

    /// Profil du relèvement d'un mode, `phi_j/psi`, pour la vérification algébrique.
    pub fn lift_profile(&self, q: usize) -> Vec<f64> {
        let dz = self.h / self.levels as f64;
        let mu = self.wave[q] * self.wave[q] * dz * dz;
        let mut diag = vec![2. + mu; self.levels];
        diag[0] = 1. + mu / 2.;
        let mut rhs = vec![0.; self.levels];
        rhs[self.levels - 1] = 1.;
        for j in 1..self.levels {
            let f = -1. / diag[j - 1];
            diag[j] += f;
            rhs[j] -= f * rhs[j - 1];
        }
        let mut shape = vec![1.; self.levels + 1];
        shape[self.levels - 1] = rhs[self.levels - 1] / diag[self.levels - 1];
        for j in (0..self.levels - 1).rev() {
            shape[j] = (rhs[j] + shape[j + 1]) / diag[j];
        }
        shape
    }

    /// Convolution tronquée à la bande : projection exacte `P_Q`, aucun repliement.
    pub fn conv(&self, f: &[C], g: &[C]) -> Vec<C> {
        let q = self.band as isize;
        let get = |v: &[C], p: isize| {
            if p >= 0 {
                v[p as usize]
            } else {
                cconj(v[(-p) as usize])
            }
        };
        (0..=self.band)
            .map(|out| {
                let o = out as isize;
                let mut s = [0.; 2];
                for p in -q..=q {
                    let r = o - p;
                    if r.abs() <= q {
                        s = cadd(s, cmul(get(f, p), get(g, r)));
                    }
                }
                s
            })
            .collect()
    }

    fn symbol(&self, sym: &[f64], f: &[C]) -> Vec<C> {
        f.iter()
            .zip(sym)
            .map(|(v, s)| cscale(*v, *s))
            .collect()
    }
    /// `A = ∂_z` en z=0.
    fn dn_of(&self, f: &[C]) -> Vec<C> {
        self.symbol(&self.dn, f)
    }
    /// `B = ∂_z²` en z=0, de symbole `k_q²` par l'équation de Laplace.
    fn lap_of(&self, f: &[C]) -> Vec<C> {
        f.iter()
            .zip(&self.wave)
            .map(|(v, k)| cscale(*v, k * k))
            .collect()
    }
    fn ddx(&self, f: &[C]) -> Vec<C> {
        f.iter()
            .zip(&self.wave)
            .map(|(v, k)| [-k * v[1], k * v[0]])
            .collect()
    }
    fn axpy(a: &[C], b: &[C], s: f64) -> Vec<C> {
        a.iter()
            .zip(b)
            .map(|(x, y)| cadd(*x, cscale(*y, s)))
            .collect()
    }

    /// Membre de droite du système tronqué à l'ordre `M` (SURFACE-LIBRE-NL-S193 §3.3).
    pub fn rhs(&self, eta: &[C], psi: &[C]) -> (Vec<C>, Vec<C>) {
        let w1 = self.dn_of(psi);
        let grav: Vec<C> = eta.iter().map(|v| cscale(*v, -self.g)).collect();
        if self.order == 1 {
            return (w1, grav);
        }
        let bpsi = self.lap_of(psi);
        let etax = self.ddx(eta);
        let psix = self.ddx(psi);
        let ea = self.conv(eta, &w1);
        let w2 = Self::axpy(&self.conv(eta, &bpsi), &self.dn_of(&ea), -1.);
        let mut eta_t = Self::axpy(&w1, &w2, 1.);
        eta_t = Self::axpy(&eta_t, &self.conv(&etax, &psix), -1.);
        let mut psi_t = Self::axpy(&grav, &self.conv(&psix, &psix), -0.5);
        psi_t = Self::axpy(&psi_t, &self.conv(&w1, &w1), 0.5);
        if self.order >= 3 {
            let eta2 = self.conv(eta, eta);
            let mut w3 = Self::axpy(
                &vec![[0.; 2]; self.band + 1],
                &self.conv(&eta2, &self.dn_of(&bpsi)),
                0.5,
            );
            w3 = Self::axpy(&w3, &self.conv(eta, &self.lap_of(&ea)), -1.);
            w3 = Self::axpy(&w3, &self.dn_of(&self.conv(eta, &self.dn_of(&ea))), 1.);
            w3 = Self::axpy(&w3, &self.dn_of(&self.conv(&eta2, &bpsi)), -0.5);
            eta_t = Self::axpy(&eta_t, &w3, 1.);
            eta_t = Self::axpy(&eta_t, &self.conv(&w1, &self.conv(&etax, &etax)), 1.);
            psi_t = Self::axpy(&psi_t, &self.conv(&w1, &w2), 1.);
        }
        (eta_t, psi_t)
    }

    /// Borne de stabilité déclarée : RK4 sur la fréquence linéaire la plus rapide de la bande.
    pub fn stability_limit(&self) -> f64 {
        let max = self.dn.iter().copied().fold(0., f64::max);
        2.5 / (self.g * max).sqrt()
    }

    /// Un pas de Runge-Kutta 4. Refus atomique : aucun état modifié si le pas est refusé
    /// ou si le résultat n'est pas fini.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if !dt.is_finite() || dt <= 0. || dt >= self.stability_limit() {
            return Err("pas instable");
        }
        let (e0, p0) = (self.eta.clone(), self.psi.clone());
        let k1 = self.rhs(&e0, &p0);
        let k2 = self.rhs(
            &Self::axpy(&e0, &k1.0, dt / 2.),
            &Self::axpy(&p0, &k1.1, dt / 2.),
        );
        let k3 = self.rhs(
            &Self::axpy(&e0, &k2.0, dt / 2.),
            &Self::axpy(&p0, &k2.1, dt / 2.),
        );
        let k4 = self.rhs(&Self::axpy(&e0, &k3.0, dt), &Self::axpy(&p0, &k3.1, dt));
        let mix = |a: &[C], b: &[C], c: &[C], d: &[C], e: &[C]| -> Vec<C> {
            (0..a.len())
                .map(|i| {
                    let s = [
                        b[i][0] + 2. * c[i][0] + 2. * d[i][0] + e[i][0],
                        b[i][1] + 2. * c[i][1] + 2. * d[i][1] + e[i][1],
                    ];
                    cadd(a[i], cscale(s, dt / 6.))
                })
                .collect()
        };
        let eta = mix(&e0, &k1.0, &k2.0, &k3.0, &k4.0);
        let mut psi = mix(&p0, &k1.1, &k2.1, &k3.1, &k4.1);
        if eta
            .iter()
            .chain(&psi)
            .flatten()
            .any(|v: &f64| !v.is_finite())
        {
            return Err("état non fini");
        }
        psi[0] = [0.; 2];
        self.eta = eta;
        self.psi = psi;
        Ok(())
    }

    pub fn eta_modes(&self) -> &[C] {
        &self.eta
    }
    pub fn psi_modes(&self) -> &[C] {
        &self.psi
    }
    /// Défaut de réalité du mode nul : purement d'arrondi, mesuré et non masqué.
    pub fn real_defect(&self) -> f64 {
        self.eta[0][1].abs().max(self.psi[0][1].abs())
    }

    /// `V = ∫η dx = L η̂₀`.
    pub fn volume(&self) -> f64 {
        self.l * self.eta[0][0]
    }

    /// `E = ½∫(gη² + ψ η_t)dx`, énergie du système **tronqué** ; non conservée exactement.
    pub fn energy(&self) -> f64 {
        let (eta_t, _) = self.rhs(&self.eta, &self.psi);
        let pair = |a: &[C], b: &[C]| -> f64 {
            a[0][0] * b[0][0]
                + 2. * (1..=self.band)
                    .map(|q| a[q][0] * b[q][0] + a[q][1] * b[q][1])
                    .sum::<f64>()
        };
        self.l / 2. * (self.g * pair(&self.eta, &self.eta) + pair(&self.psi, &eta_t))
    }

    /// Échantillonnage de `η` sur `n` points équirépartis, pour comparaison de profil.
    pub fn sample(&self, n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| {
                let x = self.l * i as f64 / n as f64;
                self.eta[0][0]
                    + 2. * (1..=self.band)
                        .map(|q| {
                            let t = self.wave[q] * x;
                            self.eta[q][0] * t.cos() - self.eta[q][1] * t.sin()
                        })
                        .sum::<f64>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    fn model(order: usize, g: f64) -> NlSurface {
        let mut eta = vec![[0.; 2]; 9];
        eta[1] = [0.001, 0.];
        NlSurface::new(8, 16, 8., 2., g, order, &eta, &vec![[0.; 2]; 9]).unwrap()
    }

    /// **A242, S197.** Le véhicule doit pouvoir dire de combien son symbole discret
    /// s'écarte du symbole continu **à la configuration employée**, et non seulement à une
    /// configuration bien résolue que personne n'exécute.
    #[test]
    fn dispersion_error_names_the_offending_mode() {
        // `L = h = 8`, `K = 64` : la configuration de S193 à S196.
        let zeros = vec![[0.; 2]; 25];
        let m = NlSurface::new(24, 64, 8., 8., 9.81, 3, &zeros, &zeros).unwrap();
        // Valeurs arithmétiques du protocole S197 §2.1, indépendantes de toute simulation.
        for (upto, want) in [(2usize, 0.0048), (3, 0.0108), (9, 0.0932), (24, 0.5453)] {
            let (err, mode) = m.dispersion_error(upto);
            assert!(
                (err - want).abs() < 0.001,
                "q<={upto} : {err} attendu {want}"
            );
            // Le pire mode est toujours le plus haut : l'écart croît avec `k·dz`.
            assert_eq!(mode, upto);
        }
        // Ordre deux : quadrupler `K` divise l'écart par seize.
        let fine = NlSurface::new(24, 256, 8., 8., 9.81, 3, &zeros, &zeros).unwrap();
        let ratio = m.dispersion_error(9).0 / fine.dispersion_error(9).0;
        assert!((ratio - 16.).abs() < 1.5, "ordre deux attendu, rapport {ratio}");
    }

    /// **A242, S197.** Un triplet qui ne converge pas ne porte pas d'ordre. Le cas est
    /// celui que S196 a rencontré : `K=32` hors domaine, incréments de signes opposés,
    /// et la formule aurait rendu « ordre 6,552 ».
    #[test]
    fn richardson_refuses_a_triple_that_does_not_converge() {
        // Cas sain : incréments de même signe, décroissants d'un facteur quatre.
        let (order, residue) = NlSurface::richardson(1.0, 1.4, 1.5).unwrap();
        assert!((order - 2.).abs() < 1e-12, "ordre {order}");
        assert!(residue > 0. && residue < 0.05, "residu {residue}");
        // Le triplet de S196, aux valeurs mesurées.
        assert!(NlSurface::richardson(1.376549713e-2, 2.717401225e-3, 2.835160449e-3).is_none());
        // Incréments croissants : le raffinement éloigne, il ne converge pas.
        assert!(NlSurface::richardson(1.0, 1.1, 1.5).is_none());
    }

    #[test]
    fn lift_symbol_residual_and_band_projection() {
        let m = model(3, 9.81);
        let dz = m.h / m.levels as f64;
        assert_eq!(m.dn[0], 0.);
        for q in 1..=m.band {
            let mu = m.wave[q] * m.wave[q] * dz * dz;
            let gamma = (1. + mu / 2.).acosh();
            // Symbole indépendant : G_h = sinh(gamma) tanh(K gamma)/dz.
            let closed = gamma.sinh() * (m.levels as f64 * gamma).tanh() / dz;
            assert!((m.dn[q] - closed).abs() < 1e-12 * closed);
            let v = m.lift_profile(q);
            assert_eq!(v[m.levels], 1.);
            // Fond imperméable par l'image, pas phi_0 = 0.
            assert!(((1. + mu / 2.) * v[0] - v[1]).abs() < 1e-13);
            for j in 1..m.levels {
                assert!((-v[j - 1] + (2. + mu) * v[j] - v[j + 1]).abs() < 1e-13);
            }
            // Profil fermé cosh(j gamma)/cosh(K gamma).
            for j in 0..=m.levels {
                let want = (j as f64 * gamma).cosh() / (m.levels as f64 * gamma).cosh();
                assert!((v[j] - want).abs() < 1e-12);
            }
            assert!(m.dn[q] > 0.);
        }
        // G_h -> k tanh(kh) quand K croît.
        let fine = NlSurface::new(8, 512, 8., 2., 9.81, 1, &vec![[0.; 2]; 9], &vec![[0.; 2]; 9])
            .unwrap();
        for q in 1..=8 {
            let exact = fine.wave[q] * (fine.wave[q] * fine.h).tanh();
            assert!((fine.dn[q] - exact).abs() < 1e-4 * exact);
        }

        // Convolution tronquée : projection exacte, aucun repliement.
        let mut f = vec![[0.; 2]; 9];
        let mut g = vec![[0.; 2]; 9];
        f[1] = [0.5, 0.25];
        g[2] = [-0.75, 0.125];
        let wide = m.conv(&f, &g);
        assert!((wide[3][0] - (f[1][0] * g[2][0] - f[1][1] * g[2][1])).abs() < 1e-18);
        assert!((wide[1][0] - (f[1][0] * g[2][0] + f[1][1] * g[2][1])).abs() < 1e-18);
        let narrow = NlSurface::new(
            2,
            16,
            8.,
            2.,
            9.81,
            3,
            &vec![[0.; 2]; 3],
            &vec![[0.; 2]; 3],
        )
        .unwrap();
        let cut = narrow.conv(&f[..3], &g[..3]);
        for q in 0..=2 {
            assert_eq!(cut[q], wide[q]);
        }
    }

    #[test]
    fn linear_order_matches_independent_rk4_map() {
        // À M=1 le système est découplé mode à mode : η' = G_h ψ, ψ' = -gη.
        // Oracle indépendant : puissance exacte de l'amplification RK4, R = c I + s J,
        // J² = -ω² I, donc R^n = ρ^n (cos nθ I + sin nθ J/ω).
        let mut eta = vec![[0.; 2]; 9];
        let mut psi = vec![[0.; 2]; 9];
        eta[1] = [0.002, 0.];
        eta[3] = [0., 0.0007];
        psi[2] = [0.0005, -0.0003];
        psi[3] = [0.0009, 0.];
        let mut m = NlSurface::new(8, 16, 8., 2., 9.81, 1, &eta, &psi).unwrap();
        let dt = 0.002;
        let n = 1000;
        for _ in 0..n {
            m.step(dt).unwrap();
        }
        for q in 1..=m.band {
            let w = (m.g * m.dn[q]).sqrt();
            let x = w * dt;
            let c = 1. - x * x / 2. + x.powi(4) / 24.;
            let s = dt * (1. - x * x / 6.);
            let rho = (c * c + (s * w).powi(2)).sqrt();
            let theta = (s * w).atan2(c);
            let (amp, ang) = (rho.powi(n as i32), theta * n as f64);
            let (ratio_e, ratio_p) = ((m.dn[q] / w), (m.g / w));
            for c2 in 0..2 {
                let want_e = amp * (ang.cos() * eta[q][c2] + ang.sin() * ratio_e * psi[q][c2]);
                let want_p = amp * (ang.cos() * psi[q][c2] - ang.sin() * ratio_p * eta[q][c2]);
                let scale = eta[q][c2].abs().max(psi[q][c2].abs()).max(1e-6);
                assert!((m.eta_modes()[q][c2] - want_e).abs() < 1e-11 * scale);
                assert!((m.psi_modes()[q][c2] - want_p).abs() < 1e-11 * scale);
            }
        }
        assert!(m.real_defect() < 1e-18);
        // Gravité quadruplée, pas de temps moitié : même η à M=1, amplitude seule en jeu.
        let mut slow = model(1, 9.81);
        let mut fast = model(1, 4. * 9.81);
        for _ in 0..200 {
            slow.step(0.004).unwrap();
            fast.step(0.002).unwrap();
        }
        for q in 0..=8 {
            assert!((slow.eta_modes()[q][0] - fast.eta_modes()[q][0]).abs() < 1e-18);
        }
    }

    #[test]
    fn invariances_and_atomic_refusals() {
        for order in 1..=3 {
            // Repos strict.
            let mut rest = NlSurface::new(
                8,
                16,
                8.,
                2.,
                9.81,
                order,
                &vec![[0.; 2]; 9],
                &vec![[0.; 2]; 9],
            )
            .unwrap();
            rest.step(0.01).unwrap();
            assert_eq!(rest.eta_modes(), vec![[0.; 2]; 9].as_slice());
            assert_eq!(rest.energy(), 0.);
            // Lac plat non nul : immobile, volume exact, énergie constante.
            let mut lake = vec![[0.; 2]; 9];
            lake[0] = [0.003, 0.];
            let mut m = NlSurface::new(8, 16, 8., 2., 9.81, order, &lake, &vec![[0.; 2]; 9]).unwrap();
            let e0 = m.energy();
            for _ in 0..500 {
                m.step(0.01).unwrap();
            }
            assert_eq!(m.eta_modes()[0], [0.003, 0.]);
            assert!((m.volume() - 8. * 0.003).abs() < 1e-18);
            assert_eq!(m.energy(), e0);
        }
        // Refus de configuration.
        let z = vec![[0.; 2]; 9];
        assert!(NlSurface::new(0, 16, 8., 2., 9.81, 3, &[[0.; 2]], &[[0.; 2]]).is_err());
        assert!(NlSurface::new(8, 1, 8., 2., 9.81, 3, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., 9.81, 0, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., 9.81, 4, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, -8., 2., 9.81, 3, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 0., 9.81, 3, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., -9.81, 3, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., f64::NAN, 3, &z, &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., 9.81, 3, &z[..8], &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., 9.81, 3, &[[f64::NAN, 0.]; 9], &z).is_err());
        assert!(NlSurface::new(8, 16, 8., 2., 9.81, 3, &z, &[[0., f64::INFINITY]; 9]).is_err());
        // Refus de pas : état inchangé, y compris au plafond de stabilité.
        let mut m = model(3, 9.81);
        let before = m.eta_modes().to_vec();
        let limit = m.stability_limit();
        for dt in [0., -1., f64::NAN, f64::INFINITY, limit, 10. * limit] {
            assert!(m.step(dt).is_err());
            assert_eq!(m.eta_modes(), before.as_slice());
        }
        assert!(m.step(limit * 0.5).is_ok());
        // Débordement : état extrême refusé sans publication.
        let mut huge = vec![[0.; 2]; 9];
        huge[1] = [f64::MAX / 4., 0.];
        let mut over = vec![[0.; 2]; 9];
        over[1] = [-f64::MAX / 4., 0.];
        let mut m = NlSurface::new(8, 16, 8., 2., 9.81, 3, &huge, &over).unwrap();
        let before = m.eta_modes().to_vec();
        assert!(m.step(0.001).is_err());
        assert_eq!(m.eta_modes(), before.as_slice());
    }

    #[test]
    fn nonlinear_orders_differ_and_psi_gauge_is_inert() {
        // Les trois ordres doivent produire des états distincts sur la même donnée.
        let a = 0.05;
        let wave = TAU / 8.;
        let mut eta = vec![[0.; 2]; 9];
        eta[1] = [a / 2., 0.];
        eta[2] = [wave * a * a * 0.5 / 2., 0.];
        let omega = (9.81 * wave * (wave * 2.).tanh()).sqrt();
        let mut psi = vec![[0.; 2]; 9];
        psi[1] = [0., -9.81 * a / omega / 2.];
        let mut second: Vec<f64> = Vec::new();
        for order in 1..=3 {
            let mut m = NlSurface::new(8, 16, 8., 2., 9.81, order, &eta, &psi).unwrap();
            for _ in 0..200 {
                m.step(0.002).unwrap();
            }
            second.push(cabs(m.eta_modes()[3]));
        }
        // Le troisième harmonique est nul à M=1 et croissant avec l'ordre.
        assert!(second[0] < 1e-18);
        assert!(second[1] > 1e-9);
        assert!((second[2] - second[1]).abs() > 1e-12);
        // La jauge psi_0 n'a aucune influence : imposer une valeur non nulle ne change rien.
        let mut shifted = psi.clone();
        shifted[0] = [123.456, 0.];
        let mut m = NlSurface::new(8, 16, 8., 2., 9.81, 3, &eta, &shifted).unwrap();
        let mut ref_m = NlSurface::new(8, 16, 8., 2., 9.81, 3, &eta, &psi).unwrap();
        for _ in 0..50 {
            m.step(0.002).unwrap();
            ref_m.step(0.002).unwrap();
        }
        assert_eq!(m.eta_modes(), ref_m.eta_modes());
    }
}
