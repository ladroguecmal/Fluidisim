//! S191 : projecteur de banc collocatif, extension nulle, D central et G=-D^T.
//! DD^T est appliqué par composition, pas remplacé par le Laplacien sept points.
//! Algèbre f64 de réception, publication f32. Aucun solveur runtime adopté.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Invalid,
    NonConverged,
    Divergence,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub iterations: usize,
    pub relative_residual: f64,
    pub scaled_divergence: f64,
}

pub struct Projector {
    side: usize,
    n: usize,
    dx: f64,
    phi: Vec<f64>,
    r: Vec<f64>,
    d: Vec<f64>,
    ad: Vec<f64>,
    g: Vec<[f64; 3]>,
}

fn neighbors(n: usize, i: usize, j: usize, k: usize) -> [(Option<usize>, Option<usize>); 3] {
    let c = (i * n + j) * n + k;
    [
        (
            if i > 0 { Some(c - n * n) } else { None },
            if i + 1 < n { Some(c + n * n) } else { None },
        ),
        (
            if j > 0 { Some(c - n) } else { None },
            if j + 1 < n { Some(c + n) } else { None },
        ),
        (
            if k > 0 { Some(c - 1) } else { None },
            if k + 1 < n { Some(c + 1) } else { None },
        ),
    ]
}

pub fn gradient(n: usize, dx: f64, p: &[f64], out: &mut [[f64; 3]]) {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let c = (i * n + j) * n + k;
                for (a, (lo, hi)) in neighbors(n, i, j, k).iter().enumerate() {
                    out[c][a] = (hi.map_or(0.0, |h| p[h]) - lo.map_or(0.0, |l| p[l])) / (2.0 * dx);
                }
            }
        }
    }
}

pub fn divergence(n: usize, dx: f64, u: &[[f64; 3]], out: &mut [f64]) {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let c = (i * n + j) * n + k;
                out[c] = 0.0;
                for (a, (lo, hi)) in neighbors(n, i, j, k).iter().enumerate() {
                    out[c] +=
                        (hi.map_or(0.0, |h| u[h][a]) - lo.map_or(0.0, |l| u[l][a])) / (2.0 * dx);
                }
            }
        }
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn max_abs(a: &[f64]) -> f64 {
    a.iter().fold(0.0_f64, |m, x| m.max(x.abs()))
}

fn apply(n: usize, dx: f64, p: &[f64], g: &mut [[f64; 3]], out: &mut [f64]) {
    gradient(n, dx, p, g);
    divergence(n, dx, g, out);
    for x in out {
        *x = -*x;
    }
}

impl Projector {
    pub fn new(side: usize, dx: f64) -> Result<Self, Error> {
        if side < 4 || side > 64 || side % 2 != 0 || !dx.is_finite() || dx <= 0.0 {
            return Err(Error::Invalid);
        }
        let n = side - 2;
        let cells = n * n * n;
        Ok(Self {
            side,
            n,
            dx,
            phi: vec![0.0; cells],
            r: vec![0.0; cells],
            d: vec![0.0; cells],
            ad: vec![0.0; cells],
            g: vec![[0.0; 3]; cells],
        })
    }

    pub fn project(
        &mut self,
        u: &mut [[f32; 3]],
        tolerance: f64,
        max_iter: usize,
    ) -> Result<Stats, Error> {
        if u.len() != self.side.pow(3)
            || !tolerance.is_finite()
            || tolerance <= 0.0
            || tolerance >= 1.0
        {
            return Err(Error::Invalid);
        }
        let mut input = Vec::with_capacity(self.n.pow(3));
        let mut norm = 0.0_f64;
        for i in 0..self.side {
            for j in 0..self.side {
                for k in 0..self.side {
                    let v = u[(i * self.side + j) * self.side + k];
                    if !v.iter().all(|x| x.is_finite()) {
                        return Err(Error::Invalid);
                    }
                    let interior = i > 0
                        && j > 0
                        && k > 0
                        && i + 1 < self.side
                        && j + 1 < self.side
                        && k + 1 < self.side;
                    if interior {
                        for x in v {
                            norm = norm.max((x as f64).abs());
                        }
                        input.push(v.map(f64::from));
                    } else if v.iter().any(|&x| x != 0.0) {
                        return Err(Error::Invalid);
                    }
                }
            }
        }
        divergence(self.n, self.dx, &input, &mut self.r);
        for x in &mut self.r {
            *x = -*x;
        }
        let rhs = self.r.clone();
        let rr0 = dot(&rhs, &rhs);
        if rr0 == 0.0 {
            return Ok(Stats::default());
        }
        self.phi.fill(0.0);
        self.d.copy_from_slice(&rhs);
        let mut rr = rr0;
        let mut iterations = 0;
        while rr > tolerance * tolerance * rr0 && iterations < max_iter {
            apply(self.n, self.dx, &self.d, &mut self.g, &mut self.ad);
            let denom = dot(&self.d, &self.ad);
            if !denom.is_finite() || denom <= 0.0 {
                return Err(Error::NonConverged);
            }
            let alpha = rr / denom;
            for c in 0..self.r.len() {
                self.phi[c] += alpha * self.d[c];
                self.r[c] -= alpha * self.ad[c];
            }
            let next = dot(&self.r, &self.r);
            let beta = next / rr;
            for c in 0..self.d.len() {
                self.d[c] = self.r[c] + beta * self.d[c];
            }
            rr = next;
            iterations += 1;
        }
        // Le résidu recalculé, pas seulement celui de la récurrence de CG.
        apply(self.n, self.dx, &self.phi, &mut self.g, &mut self.ad);
        for c in 0..self.r.len() {
            self.r[c] = rhs[c] - self.ad[c];
        }
        let relative_residual = (dot(&self.r, &self.r) / rr0).sqrt();
        if !relative_residual.is_finite() || relative_residual > 2.0 * tolerance {
            return Err(Error::NonConverged);
        }
        gradient(self.n, self.dx, &self.phi, &mut self.g);
        let candidate: Vec<[f32; 3]> = input
            .iter()
            .zip(&self.g)
            .map(|(v, g)| std::array::from_fn(|a| (v[a] - g[a]) as f32))
            .collect();
        if !candidate.iter().flatten().all(|x| x.is_finite()) {
            return Err(Error::Invalid);
        }
        let published: Vec<_> = candidate.iter().map(|v| v.map(f64::from)).collect();
        divergence(self.n, self.dx, &published, &mut self.r);
        let scaled_divergence = self.dx * max_abs(&self.r) / norm;
        if !scaled_divergence.is_finite() || scaled_divergence > 2e-6 {
            return Err(Error::Divergence);
        }
        let mut c = 0;
        for i in 1..self.side - 1 {
            for j in 1..self.side - 1 {
                for k in 1..self.side - 1 {
                    u[(i * self.side + j) * self.side + k] = candidate[c];
                    c += 1;
                }
            }
        }
        Ok(Stats {
            iterations,
            relative_residual,
            scaled_divergence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const N: usize = 4;
    const SIDE: usize = N + 2;
    const DX: f64 = 0.25;
    fn padded(u: &[[f64; 3]]) -> Vec<[f32; 3]> {
        let mut out = vec![[0.0; 3]; SIDE.pow(3)];
        let mut t = 0;
        for i in 1..SIDE - 1 {
            for j in 1..SIDE - 1 {
                for k in 1..SIDE - 1 {
                    out[(i * SIDE + j) * SIDE + k] = u[t].map(|x| x as f32);
                    t += 1;
                }
            }
        }
        out
    }
    fn compact(u: &[[f32; 3]]) -> Vec<[f64; 3]> {
        let mut out = Vec::new();
        for i in 1..SIDE - 1 {
            for j in 1..SIDE - 1 {
                for k in 1..SIDE - 1 {
                    out.push(u[(i * SIDE + j) * SIDE + k].map(f64::from));
                }
            }
        }
        out
    }
    fn gap(a: &[[f64; 3]], b: &[[f64; 3]]) -> f64 {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .fold(0.0_f64, |m, (x, y)| m.max((x - y).abs()))
    }
    fn manufactured() -> (Vec<[f64; 3]>, Vec<[f64; 3]>) {
        let p: Vec<_> = (0..N.pow(3))
            .map(|i| ((i * 17) % 31) as f64 / 64.0)
            .collect();
        let mut g = vec![[0.0; 3]; p.len()];
        gradient(N, DX, &p, &mut g);
        let curl: Vec<_> = g.iter().map(|v| [v[1], -v[0], 0.0]).collect();
        (g, curl)
    }
    #[test]
    fn gradient_curl_adjoint_linearity_and_idempotence() {
        let (g, curl) = manufactured();
        let mut d = vec![0.0; N.pow(3)];
        divergence(N, DX, &curl, &mut d);
        assert_eq!(max_abs(&d), 0.0);
        divergence(N, DX, &g, &mut d);
        assert!(max_abs(&d) > 1.0, "temoin sans projection vacu");
        let inverted: Vec<_> = g.iter().map(|v| v.map(|x| 2.0 * x)).collect();
        divergence(N, DX, &inverted, &mut d);
        assert!(max_abs(&d) > 1.0, "signe inverse non detecte");
        let p: Vec<_> = (0..N.pow(3)).map(|i| (i % 7) as f64).collect();
        let mut gp = vec![[0.0; 3]; p.len()];
        gradient(N, DX, &p, &mut gp);
        divergence(N, DX, &g, &mut d);
        let inner: f64 = gp
            .iter()
            .flatten()
            .zip(g.iter().flatten())
            .map(|(a, b)| a * b)
            .sum();
        assert!((dot(&p, &d) + inner).abs() < 1e-10);
        let sum: Vec<_> = g
            .iter()
            .zip(&curl)
            .map(|(a, b)| std::array::from_fn(|x| a[x] + b[x]))
            .collect();
        let mut pr = Projector::new(SIDE, DX).unwrap();
        let mut a = padded(&g);
        let mut b = padded(&curl);
        let mut ab = padded(&sum);
        pr.project(&mut a, 1e-12, 256).unwrap();
        pr.project(&mut b, 1e-12, 256).unwrap();
        let stats = pr.project(&mut ab, 1e-12, 256).unwrap();
        assert!(stats.scaled_divergence < 2e-6);
        assert!(gap(&compact(&a), &vec![[0.0; 3]; N.pow(3)]) < 2e-6);
        assert!(gap(&compact(&b), &curl) < 2e-6);
        assert!(gap(&compact(&ab), &curl) < 2e-6);
        let projected = ab.clone();
        pr.project(&mut ab, 1e-12, 256).unwrap();
        assert!(gap(&compact(&ab), &compact(&projected)) < 2e-6);
        let energy = |u: &[[f64; 3]]| u.iter().flatten().map(|x| x * x).sum::<f64>();
        assert!(energy(&compact(&ab)) <= energy(&sum) * (1.0 + 2e-6));
    }
    #[test]
    fn independent_dense_projection_including_boundary_cells() {
        let cells = N.pow(3);
        let width = 3 * cells;
        // Assemblage par colonne de vitesse : D n'appelle aucun opérateur du support.
        let mut mat = vec![vec![0.0; width]; cells];
        for row in 0..cells {
            let xyz = [row / (N * N), (row / N) % N, row % N];
            for axis in 0..3 {
                for sign in [-1isize, 1] {
                    let mut q = xyz.map(|v| v as isize);
                    q[axis] += sign;
                    if q.iter().all(|&v| v >= 0 && v < N as isize) {
                        let col =
                            ((q[0] as usize * N + q[1] as usize) * N + q[2] as usize) * 3 + axis;
                        mat[row][col] = sign as f64 / (2.0 * DX);
                    }
                }
            }
        }
        let raw: Vec<_> = (0..width)
            .map(|i| ((i * 13 + 7) % 43) as f64 / 128.0)
            .collect();
        let mut system = vec![vec![0.0; cells + 1]; cells];
        for i in 0..cells {
            for j in 0..cells {
                system[i][j] = dot(&mat[i], &mat[j]);
            }
            system[i][cells] = dot(&mat[i], &raw);
        }
        for k in 0..cells {
            let pivot = (k..cells)
                .max_by(|&i, &j| system[i][k].abs().total_cmp(&system[j][k].abs()))
                .unwrap();
            system.swap(k, pivot);
            let value = system[k][k];
            assert!(value.abs() > 1e-10);
            for j in k..=cells {
                system[k][j] /= value;
            }
            for i in 0..cells {
                if i != k {
                    let factor = system[i][k];
                    for j in k..=cells {
                        system[i][j] -= factor * system[k][j];
                    }
                }
            }
        }
        let expected: Vec<[f64; 3]> = (0..cells)
            .map(|i| {
                std::array::from_fn(|a| {
                    let col = 3 * i + a;
                    raw[col]
                        - (0..cells)
                            .map(|r| mat[r][col] * system[r][cells])
                            .sum::<f64>()
                })
            })
            .collect();
        let initial: Vec<_> = raw.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect();
        let mut u = padded(&initial);
        Projector::new(SIDE, DX)
            .unwrap()
            .project(&mut u, 1e-12, 256)
            .unwrap();
        assert!(gap(&compact(&u), &expected) < 2e-6);
    }
    #[test]
    fn zero_and_atomic_refusals() {
        assert!(Projector::new(5, DX).is_err());
        let mut pr = Projector::new(SIDE, DX).unwrap();
        let mut zero = vec![[0.0; 3]; SIDE.pow(3)];
        assert_eq!(pr.project(&mut zero, 1e-9, 0).unwrap().iterations, 0);
        for case in 0..4 {
            let mut u = padded(&manufactured().0);
            if case == 0 {
                u[0][0] = 1.0;
            }
            if case == 1 {
                u[SIDE * SIDE + SIDE + 1][0] = f32::NAN;
            }
            if case == 2 {
                u.pop();
            }
            let before = u.clone();
            assert!(pr
                .project(&mut u, 1e-9, if case == 3 { 0 } else { 256 })
                .is_err());
            assert!(u
                .iter()
                .flatten()
                .zip(before.iter().flatten())
                .all(|(a, b)| a.to_bits() == b.to_bits()));
        }
    }
}
