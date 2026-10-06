//! Volume sous un plan orienté et inversion hydrostatique (ADR-139, SPEC-001 §6).
//! Partition tétraédrique empruntée ; état entier, calcul local f64, aucun tas au pas.
use super::Error;

const UM_PER_LOCAL_LIMIT: i64 = 4_096_000_000; // I-08 : 4096 m, borne stricte.
const SIX_UM3_PER_ML: i128 = 6_000_000_000_000;
const BISECTIONS: usize = 64; // SPEC-001 §6 : intervalle / 2^64, ou stagnation f64.
const FACES: [[usize; 3]; 4] = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];
const EDGES: [[usize; 2]; 6] = [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];

/// Cellule intérieure. La construction vérifie les coordonnées et un volume strictement positif.
#[derive(Clone, Copy, Debug)]
pub struct Tetrahedron {
    vertices: [[i64; 3]; 4],
    volume6: i128,
}

fn difference(a: [i64; 3], b: [i64; 3]) -> [i128; 3] {
    std::array::from_fn(|i| a[i] as i128 - b[i] as i128)
}
fn cross(a: [i128; 3], b: [i128; 3]) -> [i128; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2],
     a[0] * b[1] - a[1] * b[0]]
}
fn dot(a: [i128; 3], b: [i128; 3]) -> i128 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn projection(p: [i64; 3], up: [f64; 3]) -> f64 {
    p[0] as f64 * up[0] + p[1] as f64 * up[1] + p[2] as f64 * up[2]
}

impl Tetrahedron {
    pub fn new(vertices_um: [[i64; 3]; 4]) -> Result<Self, Error> {
        if vertices_um.iter().flatten().any(|x| x.unsigned_abs() >= UM_PER_LOCAL_LIMIT as u64) {
            return Err(Error::Domain);
        }
        // Avec la borne locale ci-dessus, produits et sommes du déterminant tiennent en i128.
        let a = difference(vertices_um[1], vertices_um[0]);
        let b = difference(vertices_um[2], vertices_um[0]);
        let c = difference(vertices_um[3], vertices_um[0]);
        let volume6 = dot(a, cross(b, c)).abs();
        if volume6 == 0 { return Err(Error::Shape); }
        Ok(Self { vertices: vertices_um, volume6 })
    }

    pub fn vertices_um(&self) -> [[i64; 3]; 4] { self.vertices }
}

/// Plan local `up·x = offset_um`. Les plans fournis par `plane` ont une normale unitaire,
/// opposée à g_eff ; l'origine du contenant reste un point fixe de sa géométrie.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfacePlane {
    pub up: [f64; 3],
    pub offset_um: f64,
}

/// Géométrie d'un contenant à l'équilibre hydrostatique. Les cellules peuvent former un
/// polyèdre non convexe. Leur adéquation au contenant d'auteur et sa connexité hydraulique
/// relèvent de la cuisson ; le constructeur refuse les recouvrements, même partiels.
pub struct VolumeShape<'a> {
    cells: &'a [Tetrahedron],
    capacity_ml: i64,
}

// Théorème des axes séparateurs pour deux polyèdres convexes : normales des faces et produits
// croisés des directions d'arêtes. Calcul entier exact ; le contact sans volume est autorisé.
fn separated(a: &Tetrahedron, b: &Tetrahedron, axis: [i128; 3]) -> bool {
    if axis == [0; 3] { return false; }
    let range = |t: &Tetrahedron| {
        let project = |v: [i64; 3]| dot(v.map(i128::from), axis);
        let mut lo = project(t.vertices[0]);
        let mut hi = lo;
        for v in &t.vertices[1..] {
            let p = project(*v);
            lo = lo.min(p);
            hi = hi.max(p);
        }
        (lo, hi)
    };
    let (alo, ahi) = range(a);
    let (blo, bhi) = range(b);
    ahi <= blo || bhi <= alo
}

fn interiors_overlap(a: &Tetrahedron, b: &Tetrahedron) -> bool {
    for t in [a, b] {
        for [i, j, k] in FACES {
            let axis = cross(difference(t.vertices[j], t.vertices[i]),
                             difference(t.vertices[k], t.vertices[i]));
            if separated(a, b, axis) { return false; }
        }
    }
    for [i, j] in EDGES {
        for [k, l] in EDGES {
            let axis = cross(difference(a.vertices[j], a.vertices[i]),
                             difference(b.vertices[l], b.vertices[k]));
            if separated(a, b, axis) { return false; }
        }
    }
    true
}

/// Fraction sous le plan. Les cas se calculent dans le tétraèdre de référence : aucune
/// soustraction de grands volumes signés, ni division entre deux projections du même côté.
fn wet_fraction(z: [f64; 4], h: f64) -> f64 {
    let (mut wet, mut dry) = ([0usize; 4], [0usize; 4]);
    let (mut nw, mut nd) = (0, 0);
    for (i, p) in z.iter().enumerate() {
        if *p <= h { wet[nw] = i; nw += 1; }
        else { dry[nd] = i; nd += 1; }
    }
    let t = |i: usize, j: usize| (h - z[i]) / (z[j] - z[i]);
    match nw {
        0 => 0.0,
        1 => t(wet[0], dry[0]) * t(wet[0], dry[1]) * t(wet[0], dry[2]),
        2 => {
            let a = t(wet[0], dry[0]);
            let b = t(wet[0], dry[1]);
            let c = t(wet[1], dry[0]);
            let e = t(wet[1], dry[1]);
            a * b + a * e * (1.0 - b) + c * e * (1.0 - a)
        }
        3 => 1.0 - t(dry[0], wet[0]) * t(dry[0], wet[1]) * t(dry[0], wet[2]),
        _ => 1.0,
    }
}

pub(super) fn vertical(g_eff: [f32; 3]) -> Result<([f64; 3], f64), Error> {
    let norm2 = g_eff.iter().map(|v| *v as f64 * *v as f64).sum::<f64>();
    if !(norm2 > 0.0) || !norm2.is_finite() { return Err(Error::Domain); }
    let magnitude = norm2.sqrt();
    Ok((g_eff.map(|g| -(g as f64) / magnitude), magnitude))
}

impl<'a> VolumeShape<'a> {
    /// Validation à la construction O(cellules²), jamais répétée dans le pas.
    /// Capacité issue du déterminant entier : arrondi au millilitre, pas un facteur d'échelle.
    pub fn new(cells: &'a [Tetrahedron]) -> Result<Self, Error> {
        if cells.is_empty() { return Err(Error::Shape); }
        let mut total = 0i128;
        for (i, t) in cells.iter().enumerate() {
            total = total.checked_add(t.volume6).ok_or(Error::Shape)?;
            for other in &cells[..i] {
                if interiors_overlap(t, other) { return Err(Error::Shape); }
            }
        }
        let capacity = total.checked_add(SIX_UM3_PER_ML / 2).ok_or(Error::Shape)? / SIX_UM3_PER_ML;
        if capacity <= 0 || capacity > i64::MAX as i128 { return Err(Error::Shape); }
        Ok(Self { cells, capacity_ml: capacity as i64 })
    }

    pub fn capacity_ml(&self) -> i64 { self.capacity_ml }
    pub fn tetrahedra(&self) -> &[Tetrahedron] { self.cells }

    fn volume6_below(&self, up: [f64; 3], h: f64) -> f64 {
        let mut sum = 0.0;
        for cell in self.cells {
            let z = cell.vertices.map(|p| projection(p, up));
            sum += cell.volume6 as f64 * wet_fraction(z, h);
        }
        sum
    }

    /// Volume géométrique en ml. Diagnostic flottant, jamais l'état autoritaire du nœud.
    pub fn volume_below_ml(&self, plane: SurfacePlane) -> Result<f64, Error> {
        if !plane.offset_um.is_finite() || !plane.up.iter().all(|x| x.is_finite())
            || plane.up == [0.0; 3] {
            return Err(Error::Domain);
        }
        let result = self.volume6_below(plane.up, plane.offset_um) / SIX_UM3_PER_ML as f64;
        if !result.is_finite() { return Err(Error::NonFinite); }
        Ok(result)
    }

    /// **S549 — le centre de la part mouillée** (sous `plane`), en µm dans le repère local de la forme : chaque tétraèdre découpé par le
    /// plan — un sommet mouillé (le petit tétraèdre), deux (le coin, en trois tétraèdres), trois (le tétraèdre moins le petit du sommet
    /// sec). Pour la carène libre (6.6) : l'eau d'un compartiment pèse en ce point. `Capacity` sans eau ; `Domain` sur un plan invalide.
    pub fn centroid_below_um(&self, plane: SurfacePlane) -> Result<[f64; 3], Error> {
        if !plane.offset_um.is_finite() || !plane.up.iter().all(|x| x.is_finite()) || plane.up == [0.0; 3] {
            return Err(Error::Domain);
        }
        let (mut v_tot, mut m) = (0.0f64, [0.0f64; 3]);
        let mut ajoute = |q: [[f64; 3]; 4], signe: f64| {
            let d = |a: [f64; 3], b: [f64; 3]| [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let (a, b, c) = (d(q[0], q[1]), d(q[0], q[2]), d(q[0], q[3]));
            let v = (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])).abs() / 6.0;
            v_tot += signe * v;
            for k in 0..3 {
                m[k] += signe * v * (q[0][k] + q[1][k] + q[2][k] + q[3][k]) / 4.0;
            }
        };
        for cell in self.cells {
            let p = cell.vertices.map(|v| v.map(|x| x as f64));
            let z = cell.vertices.map(|v| projection(v, plane.up) - plane.offset_um);
            // Sans allocation (I-06) : les indices mouillés, puis les secs.
            let (mut mouilles, mut secs, mut nm, mut ns) = ([0usize; 4], [0usize; 4], 0usize, 0usize);
            for i in 0..4 {
                if z[i] < 0.0 { mouilles[nm] = i; nm += 1; } else { secs[ns] = i; ns += 1; }
            }
            // Le point du plan sur l'arête (i, j), i mouillé, j sec.
            let coupe = |i: usize, j: usize| {
                let t = z[i] / (z[i] - z[j]);
                [p[i][0] + t * (p[j][0] - p[i][0]), p[i][1] + t * (p[j][1] - p[i][1]), p[i][2] + t * (p[j][2] - p[i][2])]
            };
            match nm {
                0 => {}
                4 => ajoute(p, 1.0),
                1 => {
                    let a = mouilles[0];
                    ajoute([p[a], coupe(a, secs[0]), coupe(a, secs[1]), coupe(a, secs[2])], 1.0);
                }
                3 => {
                    let d = secs[0];
                    ajoute(p, 1.0);
                    let c = |w: usize| coupe(w, d);
                    ajoute([p[d], c(mouilles[0]), c(mouilles[1]), c(mouilles[2])], -1.0);
                }
                _ => {
                    // Le coin : le prisme (A, P_AC, P_AD) – (B, P_BC, P_BD), en trois tétraèdres.
                    let (a, b, c, d) = (mouilles[0], mouilles[1], secs[0], secs[1]);
                    let (pac, pad, pbc, pbd) = (coupe(a, c), coupe(a, d), coupe(b, c), coupe(b, d));
                    ajoute([p[a], pac, pad, p[b]], 1.0);
                    ajoute([p[b], pac, pad, pbd], 1.0);
                    ajoute([p[b], pac, pbd, pbc], 1.0);
                }
            }
        }
        if !(v_tot > 0.0) {
            return Err(Error::Capacity);
        }
        let c = m.map(|x| x / v_tot);
        if !c.iter().all(|x| x.is_finite()) {
            return Err(Error::NonFinite);
        }
        Ok(c)
    }

    pub fn plane(&self, volume_ml: i64, g_eff: [f32; 3]) -> Result<SurfacePlane, Error> {
        let (up, _) = vertical(g_eff)?;
        self.plane_up(volume_ml, up)
    }

    pub(super) fn plane_up(&self, volume_ml: i64, up: [f64; 3]) -> Result<SurfacePlane, Error> {
        if !(0..=self.capacity_ml).contains(&volume_ml) { return Err(Error::Capacity); }
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for cell in self.cells {
            for vertex in cell.vertices {
                let z = projection(vertex, up);
                lo = lo.min(z);
                hi = hi.max(z);
            }
        }
        if volume_ml == 0 { return Ok(SurfacePlane { up, offset_um: lo }); }
        if volume_ml == self.capacity_ml { return Ok(SurfacePlane { up, offset_um: hi }); }
        let target = volume_ml as f64 * SIX_UM3_PER_ML as f64;
        for _ in 0..BISECTIONS {
            let mid = lo + (hi - lo) * 0.5;
            if mid == lo || mid == hi { break; }
            if self.volume6_below(up, mid) < target { lo = mid; } else { hi = mid; }
        }
        let plane = SurfacePlane { up, offset_um: lo + (hi - lo) * 0.5 };
        let got = self.volume_below_ml(plane)?;
        // Comparer au véritable entier, même au-delà de 2^53 : sa conversion f64 pourrait
        // masquer une erreur de plusieurs ml. Le résidu reste un contrôle du calcul direct.
        let whole = got as i128;
        let residual = (whole - volume_ml as i128) as f64 + (got - whole as f64);
        if residual.abs() > 0.5 { return Err(Error::Resolution); }
        Ok(plane)
    }
}

#[cfg(test)]
#[path = "tests_hydro_geometry.rs"]
mod tests;
