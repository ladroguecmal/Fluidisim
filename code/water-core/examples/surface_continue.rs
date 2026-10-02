//! **S450 — la surface continue** ([ADR-211](../../../docs/adr/ADR-211-les-trucages-retenus.md) D2) : une première image, hors ligne.
//!
//! Le champ `φ` d'`Apic3` est le champ unique de l'eau — `z − η` dans la zone des colonnes, la reconstruction des particules dans la
//! bande, `z − fond` sous le fond (`columns_label`). Son isosurface `φ = 0`, extraite d'un seul tenant, est **continue par
//! construction** : colonnes et bande y sont une même surface, seuls les jets s'en détachent.
//!
//! Le cas : B10 en quart (une sphère de 0,4 m entre dans l'eau à `Fr` = 2, `D/dx` = 8 ; l'axe au coin, deux plans de symétrie), la
//! zone des colonnes et la bascule (`ColumnsSwitch`, bande étroite : `fond` = 4). Aux instants `t·√(g/D)` = 0,5, 1, 2, 3 :
//! l'isosurface par **tétraèdres marchants** (six tétraèdres par cube, autour de la grande diagonale — un découpage cohérent d'un cube
//! à l'autre, donc étanche), sur les centres des mailles, une couche réfléchie aux plans de symétrie pour que les quatre quarts se
//! rejoignent ; le **rendu logiciel** — perspective, tampon de profondeur, ombrage de Lambert et reflet du ciel (Fresnel de Schlick),
//! la sphère en gris — en PPM (`captures/s450/`, ADR-124). Publié : les arêtes ouvertes à l'intérieur (0 attendu) et le saut de
//! hauteur au raccord bande | colonnes.
//!
//!     cargo run -p water-core --release --offline --example surface_continue -- [sortie]

#[path = "../../water-harness/src/host_impl.rs"]
#[allow(dead_code)]
mod host_impl;

use std::collections::HashMap;
use water_core::apic3d::{Apic3, ColumnsSwitch, Sphere3};
use water_core::delta3d::Domain3;
use water_core::host::HostServices;

const G: f64 = 9.81;
const D: f64 = 0.4;
const FR: f64 = 2.;
const N_D: usize = 8;
const W: usize = 960;
const H: usize = 600;

type V3 = [f64; 3];
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: V3) -> V3 {
    let l = dot(a, a).sqrt().max(1e-12);
    [a[0] / l, a[1] / l, a[2] / l]
}

/// Un triangle à rendre : trois sommets, sa normale (vers l'air), eau ou corps.
struct Tri {
    p: [V3; 3],
    n: V3,
    eau: bool,
}

fn main() -> Result<(), String> {
    let sortie = std::env::args().nth(1).unwrap_or_else(|| "captures/s450".into());
    std::fs::create_dir_all(&sortie).map_err(|e| format!("{e}"))?;
    let dx = D / N_D as f64;
    let r = 0.5 * D;
    let u = FR * (G * D).sqrt();
    let a_arret = 3. * FR * D;
    let h = a_arret + 2. * D;
    let lz = h + 2.5 * D;
    let nh = (2. * D / dx).round() as usize;
    let (nx, ny, nz) = (nh, nh, (lz / dx).round() as usize);
    let z0 = h + r;
    let echelle = (D / G).sqrt();
    let sphere = |t: f64| {
        let descente = (u * t).min(a_arret);
        let v = if u * t < a_arret { -u } else { 0. };
        Sphere3 { center: [0., 0., (z0 - descente) as f32], radius: r as f32, velocity: [0., 0., v as f32] }
    };
    let (jobs, sink) = (host_impl::SequentialJobs, host_impl::StderrSink);
    let mut arena = host_impl::ArenaAllocator::with_capacity(1 << 33);
    let mut hote = HostServices { alloc: &mut arena, jobs: &jobs, sink: &sink };
    let capacite = nx * ny * ((h / dx).ceil() as usize) * 8 + nx * ny * 8;
    let mut a = Apic3::configure(&mut hote, Domain3 { nx, ny, nz, dx: dx as f32 }, 1000., G as f32, capacite)
        .map_err(|e| format!("{e:?}"))?;
    a.seed(&|p| {
        let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64 - z0);
        (p[2] as f64) < h && x * x + y * y + z * z >= r * r
    })
    .map_err(|e| format!("{e:?}"))?;
    a.enable_columns(&mut hote, &vec![0u8; nx * ny]).map_err(|e| format!("{e:?}"))?;
    let mut s = ColumnsSwitch::with_capacity(&mut hote, a.domain()).map_err(|e| format!("{e:?}"))?;
    s.floor_cells = Some(4);
    a.set_body(Some(sphere(0.))).map_err(|e| format!("{e:?}"))?;
    s.switch(0, &mut a).map_err(|e| format!("{e:?}"))?;

    let instants = [0.5, 1.0, 2.0, 3.0];
    let (mut t, mut t_us, mut prochain) = (0f64, 0u64, 0usize);
    let (mut saut_max, mut saut_corrige_max, mut ouvertes_max) = (0f64, 0f64, 0usize);
    let debut = std::time::Instant::now();
    while prochain < instants.len() {
        a.set_body(Some(sphere(t))).map_err(|e| format!("{e:?}"))?;
        let us = a.stable_step_us(20_000);
        a.step(us).map_err(|e| format!("{e:?}"))?;
        t_us += us;
        t = t_us as f64 * 1e-6;
        s.switch(t_us, &mut a).map_err(|e| format!("{e:?}"))?;
        // Le saut au raccord, à chaque pas : la hauteur lue sur `φ` (la première traversée depuis le fond) de part et d'autre d'une
        // face bande | colonnes ; brut, et corrigé de la pente locale (la moyenne des différences voisines de chaque côté).
        let phi = a.distance();
        let lue = |i: usize, j: usize| -> Option<f64> {
            let f = |k: usize| phi[(k * ny + j) * nx + i] as f64;
            let k = (0..nz - 1).find(|&k| f(k) < 0. && f(k + 1) >= 0.)?;
            Some((k as f64 + 0.5) * dx + dx * f(k) / (f(k) - f(k + 1)))
        };
        for j in 0..ny {
            for i in 1..nx.saturating_sub(2) {
                if a.is_column(i, j) == a.is_column(i + 1, j) {
                    continue;
                }
                if let (Some(h0), Some(h1), Some(hm), Some(hp)) = (lue(i, j), lue(i + 1, j), lue(i - 1, j), lue(i + 2, j)) {
                    let brut = (h1 - h0).abs();
                    let corrige = ((h1 - h0) - 0.5 * ((h0 - hm) + (hp - h1))).abs();
                    saut_max = saut_max.max(brut);
                    saut_corrige_max = saut_corrige_max.max(corrige);
                }
            }
        }
        if t / echelle + 1e-9 < instants[prochain] {
            continue;
        }
        // L'isosurface.
        let (tris, ouvertes) = isosurface(&a, nx, ny, nz, dx);
        ouvertes_max = ouvertes_max.max(ouvertes);
        let c = sphere(t).center;
        let nom = format!("{sortie}/b10_t{:.1}.ppm", instants[prochain]);
        rendre(&tris, [c[0] as f64, c[1] as f64, c[2] as f64], r, h, &nom)?;
        let bande = (0..nx * ny).filter(|&q| !a.is_column(q % nx, q / nx)).count();
        println!(
            "SURFACE_CONTINUE_S450 t_sur_rac_d_g={:.2} triangles={} aretes_ouvertes_interieures={ouvertes} colonnes_en_bande={bande} \
             particules={} image={nom}",
            t / echelle,
            tris.len(),
            a.particle_count()
        );
        prochain += 1;
    }
    println!(
        "SURFACE_CONTINUE_S450 bilan aretes_ouvertes_max={ouvertes_max} saut_raccord_brut_max_sur_dx={:.3} \
         saut_raccord_corrige_max_sur_dx={:.3} calcul_s={:.1}",
        saut_max / dx,
        saut_corrige_max / dx,
        debut.elapsed().as_secs_f64()
    );
    Ok(())
}

/// L'isosurface `φ = 0` par tétraèdres marchants, sur les centres des mailles, une couche réfléchie aux plans `x = 0` et `y = 0`
/// (indice −1 lu comme 0). Rend les triangles du quart et le nombre d'arêtes ouvertes **à l'intérieur** (hors du bord de
/// l'échantillonnage).
fn isosurface(a: &Apic3, nx: usize, ny: usize, nz: usize, dx: f64) -> (Vec<Tri>, usize) {
    let phi = a.distance();
    // Les points de la grille d'échantillonnage : `i` de −1 à nx − 1 (le −1, la réflexion de 0).
    let (mx, my, mz) = (nx + 1, ny + 1, nz);
    let id = |i: usize, j: usize, k: usize| (k * my + j) * mx + i;
    let val = |i: usize, j: usize, k: usize| -> f64 {
        let (ii, jj) = (i.saturating_sub(1), j.saturating_sub(1));
        phi[(k * ny + jj) * nx + ii] as f64
    };
    let pos = |i: usize, j: usize, k: usize| -> V3 { [(i as f64 - 0.5) * dx, (j as f64 - 0.5) * dx, (k as f64 + 0.5) * dx] };
    const COIN: [[usize; 3]; 8] = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
    const TETS: [[usize; 4]; 6] = [[0, 5, 1, 6], [0, 1, 2, 6], [0, 2, 3, 6], [0, 3, 7, 6], [0, 7, 4, 6], [0, 4, 5, 6]];
    let mut tris = Vec::new();
    // Les arêtes des triangles, par sommet (une arête de la grille : la paire de ses deux points), et leur compte.
    let mut aretes: HashMap<((usize, usize), (usize, usize)), u32> = HashMap::new();
    for k in 0..mz - 1 {
        for j in 0..my - 1 {
            for i in 0..mx - 1 {
                let g: [(usize, V3, f64); 8] = COIN.map(|c| {
                    let (a_, b_, c_) = (i + c[0], j + c[1], k + c[2]);
                    (id(a_, b_, c_), pos(a_, b_, c_), val(a_, b_, c_))
                });
                for t in TETS {
                    let v = t.map(|q| g[q]);
                    let dedans: Vec<usize> = (0..4).filter(|&q| v[q].2 < 0.).collect();
                    let dehors: Vec<usize> = (0..4).filter(|&q| v[q].2 >= 0.).collect();
                    let point = |p: usize, q: usize| -> ((usize, usize), V3) {
                        let (a_, b_) = (v[p], v[q]);
                        let s = a_.2 / (a_.2 - b_.2);
                        let cle = if a_.0 < b_.0 { (a_.0, b_.0) } else { (b_.0, a_.0) };
                        (cle, [a_.1[0] + s * (b_.1[0] - a_.1[0]), a_.1[1] + s * (b_.1[1] - a_.1[1]), a_.1[2] + s * (b_.1[2] - a_.1[2])])
                    };
                    let mut poly: Vec<((usize, usize), V3)> = Vec::new();
                    match dedans.len() {
                        1 | 3 => {
                            let (seul, autres) = if dedans.len() == 1 { (dedans[0], dehors.clone()) } else { (dehors[0], dedans.clone()) };
                            for o in autres {
                                poly.push(point(seul, o));
                            }
                        }
                        2 => {
                            let (a0, a1, b0, b1) = (dedans[0], dedans[1], dehors[0], dehors[1]);
                            poly.push(point(a0, b0));
                            poly.push(point(a0, b1));
                            poly.push(point(a1, b1));
                            poly.push(point(a1, b0));
                        }
                        _ => continue,
                    }
                    // La normale vers l'air : le côté où `φ` croît (le centre des points dehors moins celui des points dedans).
                    let centre = |q: &[usize]| -> V3 {
                        let n = q.len() as f64;
                        let mut c = [0.; 3];
                        for &x in q {
                            for d in 0..3 {
                                c[d] += v[x].1[d] / n;
                            }
                        }
                        c
                    };
                    let air = sub(centre(&dehors), centre(&dedans));
                    let faces: Vec<[usize; 3]> = if poly.len() == 3 { vec![[0, 1, 2]] } else { vec![[0, 1, 2], [0, 2, 3]] };
                    for f in faces {
                        let p = [poly[f[0]].1, poly[f[1]].1, poly[f[2]].1];
                        let mut n = norm(cross(sub(p[1], p[0]), sub(p[2], p[0])));
                        if dot(n, air) < 0. {
                            n = [-n[0], -n[1], -n[2]];
                        }
                        for e in [(f[0], f[1]), (f[1], f[2]), (f[2], f[0])] {
                            let (x, y) = (poly[e.0].0, poly[e.1].0);
                            let cle = if x < y { (x, y) } else { (y, x) };
                            *aretes.entry(cle).or_insert(0) += 1;
                        }
                        tris.push(Tri { p, n, eau: true });
                    }
                }
            }
        }
    }
    // Une arête ouverte est vue une fois ; elle est à l'intérieur si aucun de ses quatre points de grille n'est sur le bord de
    // l'échantillonnage — `i` ou `j` à 0 (la couche réfléchie, que la copie miroir recouvre) ou au dernier, `k` = 0 ou nz − 1.
    let bord = |g: usize| {
        let (i, j, k) = (g % mx, (g / mx) % my, g / (mx * my));
        i == 0 || j == 0 || i == mx - 1 || j == my - 1 || k == 0 || k == mz - 1
    };
    let ouvertes = aretes
        .iter()
        .filter(|(c, n)| **n == 1 && ![c.0 .0, c.0 .1, c.1 .0, c.1 .1].iter().any(|g| bord(*g)))
        .count();
    (tris, ouvertes)
}

/// Le rendu logiciel : le quart reflété en entier, la sphère, une caméra en perspective ; un PPM.
fn rendre(tris: &[Tri], centre: V3, rayon: f64, niveau: f64, nom: &str) -> Result<(), String> {
    let mut tout: Vec<Tri> = Vec::with_capacity(tris.len() * 4 + 4000);
    for sx in [1., -1.] {
        for sy in [1., -1.] {
            for t in tris {
                let m = |p: V3| [p[0] * sx, p[1] * sy, p[2]];
                let mut p = [m(t.p[0]), m(t.p[1]), m(t.p[2])];
                if sx * sy < 0. {
                    p.swap(1, 2);
                }
                tout.push(Tri { p, n: [t.n[0] * sx, t.n[1] * sy, t.n[2]], eau: true });
            }
        }
    }
    // La sphère : un maillage en latitude et longitude.
    let (nl, nm) = (24usize, 48usize);
    let sp = |a: usize, b: usize| -> V3 {
        let th = std::f64::consts::PI * a as f64 / nl as f64;
        let ph = std::f64::consts::TAU * b as f64 / nm as f64;
        [centre[0] + rayon * th.sin() * ph.cos(), centre[1] + rayon * th.sin() * ph.sin(), centre[2] + rayon * th.cos()]
    };
    for a in 0..nl {
        for b in 0..nm {
            let (p0, p1, p2, p3) = (sp(a, b), sp(a + 1, b), sp(a + 1, b + 1), sp(a, b + 1));
            for p in [[p0, p1, p2], [p0, p2, p3]] {
                let c = [(p[0][0] + p[1][0] + p[2][0]) / 3., (p[0][1] + p[1][1] + p[2][1]) / 3., (p[0][2] + p[1][2] + p[2][2]) / 3.];
                tout.push(Tri { p, n: norm(sub(c, centre)), eau: false });
            }
        }
    }
    // La caméra : de côté et d'au-dessus, vers le point d'entrée.
    let oeil: V3 = [1.7, -2.1, niveau + 1.1];
    let cible: V3 = [0., 0., niveau - 0.15];
    let avant = norm(sub(cible, oeil));
    let droite = norm(cross(avant, [0., 0., 1.]));
    let haut = cross(droite, avant);
    let f = 1. / (0.5 * 40f64.to_radians()).tan();
    let aspect = W as f64 / H as f64;
    let projeter = |p: V3| -> Option<[f64; 3]> {
        let d = sub(p, oeil);
        let z = dot(d, avant);
        if z < 0.05 {
            return None;
        }
        let x = dot(d, droite) * f / z / aspect;
        let y = dot(d, haut) * f / z;
        Some([(x * 0.5 + 0.5) * W as f64, (0.5 - y * 0.5) * H as f64, z])
    };
    let soleil = norm([0.4, -0.3, 0.85]);
    let ciel = |dir: V3| -> V3 {
        let h = dir[2].clamp(-1., 1.);
        let t = (0.5 + 0.5 * h).powf(0.8);
        [0.55 + 0.25 * t, 0.68 + 0.2 * t, 0.82 + 0.15 * t]
    };
    let mut image = vec![[0f64; 3]; W * H];
    let mut prof = vec![f64::INFINITY; W * H];
    for y in 0..H {
        for x in 0..W {
            let dir = norm([
                avant[0] + droite[0] * ((x as f64 / W as f64) * 2. - 1.) * aspect / f + haut[0] * (1. - 2. * y as f64 / H as f64) / f,
                avant[1] + droite[1] * ((x as f64 / W as f64) * 2. - 1.) * aspect / f + haut[1] * (1. - 2. * y as f64 / H as f64) / f,
                avant[2] + droite[2] * ((x as f64 / W as f64) * 2. - 1.) * aspect / f + haut[2] * (1. - 2. * y as f64 / H as f64) / f,
            ]);
            image[y * W + x] = ciel(dir);
        }
    }
    for t in &tout {
        let Some(a) = projeter(t.p[0]) else { continue };
        let Some(b) = projeter(t.p[1]) else { continue };
        let Some(c) = projeter(t.p[2]) else { continue };
        let centre_t = [(t.p[0][0] + t.p[1][0] + t.p[2][0]) / 3., (t.p[0][1] + t.p[1][1] + t.p[2][1]) / 3., (t.p[0][2] + t.p[1][2] + t.p[2][2]) / 3.];
        let vue = norm(sub(oeil, centre_t));
        let mut n = t.n;
        if dot(n, vue) < 0. {
            n = [-n[0], -n[1], -n[2]];
        }
        let couleur = if t.eau {
            let cos = dot(n, vue).clamp(0., 1.);
            let fresnel = 0.02 + 0.98 * (1. - cos).powi(5);
            let refl = sub([2. * dot(n, vue) * n[0], 2. * dot(n, vue) * n[1], 2. * dot(n, vue) * n[2]], vue);
            let sky = ciel(norm(refl));
            let lambert = dot(n, soleil).max(0.);
            let fond = [0.02 + 0.10 * lambert, 0.16 + 0.18 * lambert, 0.24 + 0.20 * lambert];
            let spec = dot(norm(refl), soleil).max(0.).powi(60) * 0.8;
            [
                fond[0] * (1. - fresnel) + sky[0] * fresnel + spec,
                fond[1] * (1. - fresnel) + sky[1] * fresnel + spec,
                fond[2] * (1. - fresnel) + sky[2] * fresnel + spec,
            ]
        } else {
            let l = 0.25 + 0.65 * dot(n, soleil).max(0.);
            [0.55 * l, 0.55 * l, 0.58 * l]
        };
        let (x0, x1) = (a[0].min(b[0]).min(c[0]).floor().max(0.) as usize, a[0].max(b[0]).max(c[0]).ceil().min(W as f64 - 1.) as usize);
        let (y0, y1) = (a[1].min(b[1]).min(c[1]).floor().max(0.) as usize, a[1].max(b[1]).max(c[1]).ceil().min(H as f64 - 1.) as usize);
        let aire = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if aire.abs() < 1e-12 || x0 > x1 || y0 > y1 {
            continue;
        }
        for y in y0..=y1 {
            for x in x0..=x1 {
                let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
                let w0 = ((b[0] - px) * (c[1] - py) - (b[1] - py) * (c[0] - px)) / aire;
                let w1 = ((c[0] - px) * (a[1] - py) - (c[1] - py) * (a[0] - px)) / aire;
                let w2 = 1. - w0 - w1;
                if w0 < 0. || w1 < 0. || w2 < 0. {
                    continue;
                }
                let z = w0 * a[2] + w1 * b[2] + w2 * c[2];
                let q = y * W + x;
                if z < prof[q] {
                    prof[q] = z;
                    image[q] = couleur;
                }
            }
        }
    }
    let mut ppm = format!("P6\n{W} {H}\n255\n").into_bytes();
    for px in &image {
        for c in px {
            ppm.push((c.clamp(0., 1.).powf(1. / 2.2) * 255.).round() as u8);
        }
    }
    std::fs::write(nom, ppm).map_err(|e| format!("{e}"))
}
