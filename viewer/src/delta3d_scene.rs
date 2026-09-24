//! S302 / ADR-175 §4.3 — **la scène du critère 3** : une onde qui traverse une mer étalée dans un
//! domaine δ 3D, rendue en direct.
//!
//! Mer : `--houle` (S259, 64 composantes, Hs ≈ 2,5 m). Onde : un **paquet linéaire** — hauteur et
//! vitesses de la théorie linéaire en eau profonde sous une enveloppe gaussienne —, injecté à la
//! configuration et qui traverse le domaine vers la caméra. L'état initial est construit sur CPU
//! une fois (`O(N)` hors pas, I-06) ; ensuite tout vit sur la carte (`Step3`, S301).
//!
//! **S339** (revue R16) : `--impact` remplace le paquet par **un impact** — un cratère à bord relevé,
//! au repos, dont naissent des anneaux (problème de Cauchy–Poisson). R11 ne savait pas dire si
//! l'onde du front était « circulaire ou bien linéaire » : celle-ci dit d'où elle vient. Ses anneaux de
//! 15 cm se perdent dans une mer de `Hs` 2,5 m (S339 P4) ; `--anneau` prépare donc **un anneau** fort,
//! le paquet de S302 enroulé autour d'un point, comme le front était préparé.
//!
//! Rien ici n'est une grandeur de jeu (I-04), rien n'est sérialisé (I-17).
use crate::delta3d_step::{face_total, Diagnostics, Step3};
use water_core::background::Background;
use water_core::delta3d::{Domain3, Sponge3};

/// Paramètres de la scène, arrêtés par la mesure sans fenêtre (S302 P2).
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub domain: Domain3,
    /// Coin bas du domaine, en coordonnées monde (B est ancré à l'origine du monde).
    pub origin: [f32; 3],
    /// Niveau de repos de la surface, dans le repère du domaine.
    pub rest: f32,
    pub sponge: Sponge3,
    pub cycles: u32,
    pub step_us: u64,
    pub packet: Packet,
    /// S339 : l'impact, s'il y en a un — ajouté à la hauteur initiale, sans vitesse.
    pub impact: Option<Impact>,
    /// S339 P4 bis : l'anneau, s'il y en a un — hauteur et vitesses sortantes.
    pub ring: Option<Ring>,
}

/// Paquet linéaire : `η = a·cos(k·d·x)·G`, vitesses de la théorie en eau profonde, `G`
/// gaussienne (écart `sigma_long` le long de la propagation, `sigma_crest` le long des crêtes).
#[derive(Clone, Copy, Debug)]
pub struct Packet {
    pub amplitude: f32,
    pub wavelength: f32,
    /// Centre, coordonnées monde.
    pub center: [f32; 2],
    /// Direction de propagation, unitaire.
    pub direction: [f32; 2],
    pub sigma_long: f32,
    pub sigma_crest: f32,
}

/// S339 : **un impact** — le cratère à bord relevé qu'un objet laisse en entrant dans l'eau, lâché au
/// repos : `η₀ = −A·(1 − r²/2σ²)·e^{−r²/2σ²}`. Son volume net est nul — le bord rend ce que le
/// creux prend —, sa pente maximale vaut ≈ 0,98·A/σ, et son spectre culmine au nombre d'onde
/// `√2/σ` : les anneaux s'éloignent à la vitesse de groupe de celui-ci, dispersés.
#[derive(Clone, Copy, Debug)]
pub struct Impact {
    /// Profondeur du creux au centre, m.
    pub depth: f32,
    /// Rayon caractéristique, m : le creux s'annule à `√2·σ`, le bord culmine à `2·σ`.
    pub sigma: f32,
    /// Centre, coordonnées monde.
    pub center: [f32; 2],
}

impl Impact {
    /// La hauteur ajoutée en un point monde.
    pub fn height(&self, x: f32, y: f32) -> f32 {
        let (rx, ry) = (x - self.center[0], y - self.center[1]);
        let u = (rx * rx + ry * ry) / (2. * self.sigma * self.sigma);
        -self.depth * (1. - u) * (-u).exp()
    }

    /// Vitesse de groupe du nombre d'onde dominant `√2/σ`, eau profonde, m/s.
    pub fn group_velocity(&self) -> f32 {
        0.5 * (G * self.sigma / std::f32::consts::SQRT_2).sqrt()
    }
}

/// S339 P4 bis : **un anneau** — le paquet de S302 enroulé autour d'un point :
/// `η = a·cos(k·(r − r₀))·e^{−(r−r₀)²/2s²}`, vitesses de la théorie linéaire en eau profonde, dirigées
/// vers l'extérieur. Localement une onde plane : l'approximation vaut loin du centre ; au centre, la
/// projection du premier pas corrige ce qu'elle a de faux.
#[derive(Clone, Copy, Debug)]
pub struct Ring {
    pub amplitude: f32,
    pub wavelength: f32,
    /// Centre, coordonnées monde.
    pub center: [f32; 2],
    /// Rayon de la crête au départ, m.
    pub radius: f32,
    /// Écart-type de l'enveloppe radiale, m.
    pub sigma: f32,
}

impl Ring {
    /// Vitesse de groupe, eau profonde, m/s.
    pub fn group_velocity(&self) -> f32 {
        0.5 * (G * self.wavelength / std::f32::consts::TAU).sqrt()
    }

    /// En un point monde : hauteur, quadrature, et direction radiale unitaire (nulle au centre).
    fn at(&self, x: f32, y: f32) -> (f32, f32, [f32; 2]) {
        let k = std::f32::consts::TAU / self.wavelength;
        let (rx, ry) = (x - self.center[0], y - self.center[1]);
        let r = (rx * rx + ry * ry).sqrt();
        let rho = r - self.radius;
        let envelope = self.amplitude * (-(rho * rho) / (2. * self.sigma * self.sigma)).exp();
        let dir = if r > 1e-3 { [rx / r, ry / r] } else { [0., 0.] };
        (envelope * (k * rho).cos(), envelope * (k * rho).sin(), dir)
    }
}

/// Ce que le rendu doit savoir de la couche delta 3D : ou elle est, son pas, son fondu, et si elle
/// est active. La **hauteur** reste sur la carte (D7) : le rendu lie le tampon publie.
#[derive(Clone, Copy, Debug)]
pub struct View {
    /// Coin bas du domaine en coordonnees monde.
    pub origin: [f32; 2],
    pub dx: f32,
    /// Largeur du fondu en cosinus depuis chaque bord, m.
    pub fade: f32,
    pub nx: u32,
    pub ny: u32,
    pub active: bool,
}

impl Config {
    pub fn view(&self, active: bool) -> View {
        View {
            origin: [self.origin[0], self.origin[1]],
            dx: self.domain.dx,
            fade: self.sponge.width_x.max(self.sponge.width_y),
            nx: self.domain.nx as u32,
            ny: self.domain.ny as u32,
            active,
        }
    }
}

pub const RHO: f32 = 1025.;
pub const G: f32 = 9.81;

impl Config {
    /// La scène de revue R11. 30 m × 28 m à 25 cm, boîte de 7 m, repos à 3,5 m sous le plan
    /// moyen : les creux et crêtes de Hs 2,5 m y tiennent. Devant la caméra par défaut
    /// (`[0, −18, 7]`), de 18 à 46 m. Le paquet part du fond et vient vers la caméra.
    ///
    /// **Ce qui plafonne la taille** : le tampon des faces du fond porte 26 flottants par face
    /// (S300) ; au-delà de ~1,15 million de faces il dépasse la limite standard de 128 Mio d'une
    /// liaison de stockage. Réduire cette charge utile — le pas n'a besoin que de la vitesse, d'une
    /// ligne de `grad_u`, du résidu et de la pression — est une optimisation identifiée, pas faite.
    pub fn review() -> Self {
        let domain = Domain3 { nx: 120, ny: 112, nz: 28, dx: 0.25 };
        let rest = 3.5;
        Config {
            domain,
            origin: [-15., 0., -rest],
            rest,
            sponge: Sponge3 { width_x: 3., width_y: 3., rate_per_s: 2. },
            cycles: 32,
            step_us: 16_667,
            // 65 cm d'amplitude pour 16 m : cambrure `ak` = 0,26, sous la limite de déferlement
            // (0,44) et sous le refus non diagnostiqué de 0,335 vu en 2D. Vitesse de groupe
            // 2,5 m/s : le front traverse les 28 m du domaine en une douzaine de secondes. Crête
            // longue de 12 m d'écart-type : un **front** qui barre le domaine, pas un point.
            packet: Packet {
                amplitude: 0.65,
                wavelength: 16.,
                center: [0., 24.],
                direction: [0., -1.],
                sigma_long: 9.,
                sigma_crest: 12.,
            },
            impact: None,
            ring: None,
        }
    }

    /// S339, revue R16 : la scène de R11, même domaine, même mer, même éponge — le front remplacé
    /// par un impact au milieu de la partie libre du domaine, à 32 m de la caméra. **A = 0,65 m,
    /// σ = 2,5 m**, choisis au banc sans fenêtre (S339 P3) : le plus grand cratère dont la pente
    /// reste sous celle du paquet de S302 (0,252 pour 0,26) et dont le profil tient dans le domaine
    /// — à σ = 3 m, sa queue tronquée par les bords laisse 1,9·10⁻³ m³ de volume net. `IMPACT=A,σ`
    /// les remplace, pour la mesure.
    pub fn impact_review() -> Self {
        let (depth, sigma) = std::env::var("IMPACT")
            .ok()
            .and_then(|v| {
                let mut it = v.split(',').filter_map(|x| x.parse::<f32>().ok());
                Some((it.next()?, it.next()?))
            })
            .unwrap_or((0.65, 2.5));
        let mut c = Self::review();
        c.packet.amplitude = 0.;
        c.impact = Some(Impact { depth, sigma, center: [0., 14.] });
        c
    }

    /// S339 P4 bis, revue R16 : **l'anneau**, au fond de la partie libre du domaine pour que sa moitié
    /// avant ait 8 m à parcourir vers la caméra. 41 cm pour 10 m : `a·k` = 0,26, la cambrure du paquet de
    /// S302 ; enveloppe de 3,5 m ; crête à 5 m du centre, où la queue de l'enveloppe laisse un creux.
    pub fn ring_review() -> Self {
        let mut c = Self::review();
        c.packet.amplitude = 0.;
        c.ring = Some(Ring { amplitude: 0.41, wavelength: 10., center: [0., 16.], radius: 5., sigma: 3.5 });
        c
    }

    /// État initial : vitesses aux faces et surface absolue par colonne, dans le repère du domaine.
    /// Sans paquet (`amplitude = 0`), la mer seule : surface au repos, vitesses nulles — δ ne
    /// portera que la correction couplée de B.
    pub fn initial_state(&self) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
        let Domain3 { nx, ny, nz, dx } = self.domain;
        let p = self.packet;
        let k = std::f32::consts::TAU / p.wavelength;
        let omega = (G * k).sqrt();
        let [dxn, dyn_] = p.direction;
        // Hauteur et « amplitude de vitesse » en un point monde (x, y) et à `zeta` sous le repos.
        let wave = |x: f32, y: f32| -> (f32, f32) {
            let (rx, ry) = (x - p.center[0], y - p.center[1]);
            let along = rx * dxn + ry * dyn_;
            let across = -rx * dyn_ + ry * dxn;
            let envelope = (-(along * along) / (2. * p.sigma_long * p.sigma_long)
                - (across * across) / (2. * p.sigma_crest * p.sigma_crest))
                .exp();
            let theta = k * along;
            (p.amplitude * envelope * theta.cos(), p.amplitude * envelope * theta.sin())
        };
        let world = |i: f32, j: f32| (self.origin[0] + i * dx, self.origin[1] + j * dx);
        // Décroissance verticale plafonnée à la surface de repos : au-dessus, les faces sont
        // sèches et l'extrapolation du premier pas les réécrit.
        let decay = |k_face: f32| -> f32 { (k * (k_face * dx - self.rest).min(0.)).exp() };

        let mut eta = vec![self.rest; nx * ny];
        for j in 0..ny {
            for i in 0..nx {
                let (x, y) = world(i as f32 + 0.5, j as f32 + 0.5);
                eta[j * nx + i] += wave(x, y).0;
                if let Some(impact) = self.impact {
                    eta[j * nx + i] += impact.height(x, y);
                }
                if let Some(ring) = self.ring {
                    eta[j * nx + i] += ring.at(x, y).0;
                }
            }
        }
        let mut u = vec![0f32; (nx + 1) * ny * nz];
        let mut v = vec![0f32; nx * (ny + 1) * nz];
        let mut w = vec![0f32; nx * ny * (nz + 1)];
        let a_omega = omega;
        for kk in 0..nz {
            let d = decay(kk as f32 + 0.5);
            for j in 0..ny {
                for i in 1..nx {
                    let (x, y) = world(i as f32, j as f32 + 0.5);
                    u[(kk * ny + j) * (nx + 1) + i] = a_omega * d * wave(x, y).0 * dxn;
                }
            }
            for j in 1..ny {
                for i in 0..nx {
                    let (x, y) = world(i as f32 + 0.5, j as f32);
                    v[(kk * (ny + 1) + j) * nx + i] = a_omega * d * wave(x, y).0 * dyn_;
                }
            }
        }
        for kk in 1..=nz {
            let d = decay(kk as f32);
            for j in 0..ny {
                for i in 0..nx {
                    let (x, y) = world(i as f32 + 0.5, j as f32 + 0.5);
                    w[(kk * ny + j) * nx + i] = a_omega * d * wave(x, y).1;
                }
            }
        }
        // S339 P4 bis : les vitesses de l'anneau, sortantes, à sa propre décroissance verticale.
        if let Some(ring) = self.ring {
            let kr = std::f32::consts::TAU / ring.wavelength;
            let omega_r = (G * kr).sqrt();
            let decay_r = |k_face: f32| -> f32 { (kr * (k_face * dx - self.rest).min(0.)).exp() };
            for kk in 0..nz {
                let d = omega_r * decay_r(kk as f32 + 0.5);
                for j in 0..ny {
                    for i in 1..nx {
                        let (x, y) = world(i as f32, j as f32 + 0.5);
                        let (h, _, dir) = ring.at(x, y);
                        u[(kk * ny + j) * (nx + 1) + i] += d * h * dir[0];
                    }
                }
                for j in 1..ny {
                    for i in 0..nx {
                        let (x, y) = world(i as f32 + 0.5, j as f32);
                        let (h, _, dir) = ring.at(x, y);
                        v[(kk * (ny + 1) + j) * nx + i] += d * h * dir[1];
                    }
                }
            }
            for kk in 1..=nz {
                let d = omega_r * decay_r(kk as f32);
                for j in 0..ny {
                    for i in 0..nx {
                        let (x, y) = world(i as f32 + 0.5, j as f32 + 0.5);
                        w[(kk * ny + j) * nx + i] += d * ring.at(x, y).1;
                    }
                }
            }
        }
        (u, v, w, eta)
    }

    pub fn without_packet(mut self) -> Self {
        self.packet.amplitude = 0.;
        self.impact = None;
        self.ring = None;
        self
    }
}

/// Le domaine δ vivant d'une scène : le pas de production et son horloge entière.
pub struct Live {
    pub step: Step3,
    pub config: Config,
    /// Pas enregistrés depuis la dernière injection.
    pub steps: u64,
    /// Instant de B au pas zéro, en microseconds.
    pub start_us: u64,
    pub last: Option<Diagnostics>,
}

impl Live {
    pub fn new(step: Step3, config: Config, start_us: u64) -> Result<Self, String> {
        step.set_step(config.step_us, config.rest, config.sponge)?;
        let mut live = Live { step, config, steps: 0, start_us, last: None };
        live.inject(start_us)?;
        Ok(live)
    }

    /// (Ré)injecte l'état initial. Hors pas : `O(N)` sur CPU, à la demande de l'utilisateur.
    pub fn inject(&mut self, start_us: u64) -> Result<(), String> {
        let (u, v, w, eta) = self.config.initial_state();
        self.step.set_state(&u, &v, &w, &eta)?;
        self.steps = 0;
        self.start_us = start_us;
        Ok(())
    }

    /// Un pas de production. Ne lit rien ; les diagnostics reviennent en différé.
    pub fn advance(&mut self, background: &Background) -> Result<(), String> {
        let time = water_core::SimTime(self.start_us + self.steps * self.config.step_us);
        self.step.step(background, time, self.config.cycles)?;
        self.steps += 1;
        if let Some(d) = self.step.diagnostics()? {
            self.last = Some(d);
        }
        Ok(())
    }
}

/// Banc P2 : **la scène sans fenêtre**. Deux domaines sous la mer `--houle`, l'un avec le paquet,
/// l'autre sans (témoin) ; douze secondes à 60 Hz par le seul chemin de production. On relève ce
/// qui décide si la scène est montrable : colonnes hors bornes, pas dégradés, amplitude de δ avec
/// et sans paquet, et l'onde isolée (paquet − témoin), qui doit traverser le domaine.
pub fn mesurer(impact: bool, anneau: bool) -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = if anneau {
            Config::ring_review()
        } else if impact {
            Config::impact_review()
        } else {
            Config::review()
        };
        let Domain3 { nx, ny, nz, dx } = config.domain;
        let secondes: f64 = std::env::var("SECONDES").ok().and_then(|v| v.parse().ok()).unwrap_or(12.);
        let pas = (secondes * 1e6 / config.step_us as f64) as u64;
        println!(
            "DELTA3D_SCENE_S302 mer=--houle composantes={} domaine={nx}x{ny}x{nz} dx={dx} mailles={} faces={} repos={} eponge={:?} cycles={} dt_us={} paquet={:?} pas={pas}",
            background.components().len(), config.domain.cells(), face_total(config.domain),
            config.rest, config.sponge, config.cycles, config.step_us, config.packet
        );
        let mut avec = Live::new(Step3::new(background, config.domain, config.origin, RHO, G).await?, config, 0)?;
        let temoin_cfg = config.without_packet();
        let mut sans = Live::new(Step3::new(background, config.domain, config.origin, RHO, G).await?, temoin_cfg, 0)?;
        println!("DELTA3D_SCENE_S302 carte={:?} backend={}", avec.step.adapter, avec.step.backend);
        // S339 : l'impact au départ — volume net et pente maximale de la hauteur ajoutée, discrets.
        if config.impact.is_some() || config.ring.is_some() {
            let (eta0, rest) = (config.initial_state().3, config.rest);
            let volume: f64 = eta0.iter().map(|h| (*h - rest) as f64).sum::<f64>() * (dx * dx) as f64;
            let mut pente = 0f32;
            for j in 1..ny - 1 {
                for i in 1..nx - 1 {
                    let c = j * nx + i;
                    let (gx, gy) = ((eta0[c + 1] - eta0[c - 1]) / (2. * dx), (eta0[c + nx] - eta0[c - nx]) / (2. * dx));
                    pente = pente.max((gx * gx + gy * gy).sqrt());
                }
            }
            if let Some(im) = config.impact {
                println!(
                    "DELTA3D_IMPACT_S339 depart profondeur_m={} sigma_m={} centre={:?} volume_net_m3={volume:.3e} pente_max={pente:.3} pente_prevue={:.3} vitesse_groupe_ms={:.3}",
                    im.depth, im.sigma, im.center, 0.98 * im.depth / im.sigma, im.group_velocity()
                );
            }
            if let Some(g) = config.ring {
                println!(
                    "DELTA3D_IMPACT_S339 depart anneau={g:?} volume_net_m3={volume:.3e} pente_max={pente:.3} ak={:.3} vitesse_groupe_ms={:.3}",
                    g.amplitude * std::f32::consts::TAU / g.wavelength, g.group_velocity()
                );
            }
        }

        let colonne = |c: usize| {
            let (i, j) = (c % nx, c / nx);
            (config.origin[0] + (i as f32 + 0.5) * dx, config.origin[1] + (j as f32 + 0.5) * dx)
        };
        let (mut degrades, mut hors, mut franche_max, mut recus) = ([0usize; 2], [0u32; 2], [0f32; 2], [0usize; 2]);
        for n in 0..=pas {
            if n % 60 == 0 {
                let (pa, ps) = (avec.step.published()?, sans.step.published()?);
                let crete = |v: &[f32]| v.iter().fold(0f32, |m, x| m.max(x.abs()));
                // L'onde isolée : où est son maximum, et combien pèse-t-elle.
                let (mut onde, mut lieu) = (0f32, 0usize);
                for c in 0..pa.len() {
                    let d = (pa[c] - ps[c]).abs();
                    if d > onde { onde = d; lieu = c; }
                }
                let (x, y) = colonne(lieu);
                println!(
                    "DELTA3D_SCENE_S302 t={:.1} delta_max_avec={:.4} delta_max_temoin={:.4} onde_max={onde:.4} onde_en=({x:.1},{y:.1}) degrades={degrades:?} hors_bornes_max={hors:?} franche_max={franche_max:?}",
                    n as f64 * config.step_us as f64 * 1e-6, crete(&pa), crete(&ps)
                );
                // S339 : les anneaux — moyenne azimutale de l'onde isolée par couronnes de 50 cm autour
                // du point d'impact, rayon et valeur du maximum au-delà du bord du cratère (`2·σ`),
                // comparés au trajet de la vitesse de groupe dominante.
                // S339 P4 bis : l'anneau se mesure comme l'impact, autour de son centre, au-delà de la moitié
                // de son rayon de départ — en deçà, le creux central.
                let centre_bord = config
                    .impact
                    .map(|im| (im.center, 2. * im.sigma, im.group_velocity(), 0f32))
                    .or(config.ring.map(|g| (g.center, 0.5 * g.radius, g.group_velocity(), g.radius)));
                if let Some((centre, bord, vitesse, depart)) = centre_bord {
                    let largeur = 0.5f32;
                    let (mut somme, mut compte) = (vec![0f64; 64], vec![0u32; 64]);
                    for c in 0..pa.len() {
                        let (x, y) = colonne(c);
                        let r = ((x - centre[0]).powi(2) + (y - centre[1]).powi(2)).sqrt();
                        let b = (r / largeur) as usize;
                        if b < somme.len() {
                            somme[b] += (pa[c] - ps[c]).abs() as f64;
                            compte[b] += 1;
                        }
                    }
                    let (mut rayon, mut valeur) = (0f32, 0f64);
                    for b in 0..somme.len() {
                        let r = (b as f32 + 0.5) * largeur;
                        if r >= bord && compte[b] > 0 && somme[b] / compte[b] as f64 > valeur {
                            valeur = somme[b] / compte[b] as f64;
                            rayon = r;
                        }
                    }
                    let t = n as f32 * config.step_us as f32 * 1e-6;
                    println!(
                        "DELTA3D_IMPACT_S339 t={t:.1} rayon_max_m={rayon:.2} moyenne_azimutale_m={valeur:.4} centre_m={:.4} trajet_groupe_m={:.2}",
                        somme[0] / compte[0].max(1) as f64, depart + vitesse * t
                    );
                }
            }
            if n == pas { break; }
            for (s, live) in [&mut avec, &mut sans].into_iter().enumerate() {
                live.advance(background)?;
                if let Some(d) = live.last.take() {
                    recus[s] += 1;
                    degrades[s] += d.degraded() as usize;
                    hors[s] = hors[s].max(d.columns_outside);
                    franche_max[s] = franche_max[s].max(d.divergence_plain);
                }
            }
        }
        println!("DELTA3D_SCENE_S302 bilan diagnostics_recus={recus:?} degrades={degrades:?} hors_bornes_max={hors:?} franche_max={franche_max:?}");
        // Coût du pas de cette scène, même forme que S301 P6.
        let mut mesures = Vec::new();
        for n in 0..31u64 {
            avec.step.publish_time(background, water_core::SimTime(n * config.step_us))?;
            if let Some(ms) = avec.step.timed_step(config.cycles)? {
                if n > 0 { mesures.push(ms); }
            }
        }
        mesures.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "DELTA3D_SCENE_S302 cout gpu_mediane_ms={:.3} gpu_max_ms={:.3} dispatchs={}",
            mesures.get(mesures.len() / 2).copied().unwrap_or(f64::NAN),
            mesures.last().copied().unwrap_or(f64::NAN),
            Step3::dispatches(config.cycles, crate::delta3d_step::Upto::Full)
        );
        Ok(())
    })
}

/// Banc P3 : **à-coups d'A297 sur la scène**. La hauteur publiée est relue à chaque pas ; par
/// colonne, la dérivée seconde temporelle `η(n+1) − 2η(n) + η(n−1)`. Une onde lisse d'amplitude `a`
/// et de pulsation `ω` en donne au plus `a·ω²·dt²` ; une bascule de mouillure (S301 : ~0,1 m/s de
/// saut de débit) en donne ~`dt·0,1` d'un coup, soit ~1,7 mm à 60 Hz. On publie la distribution,
/// le maximum et son caractère **local** (rapport à la moyenne de ses huit voisines).
pub fn acoups() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let secondes: f64 = std::env::var("SECONDES").ok().and_then(|v| v.parse().ok()).unwrap_or(6.);
        // Balayage de cycles : la rugosité à l'échelle de la maille est-elle un à-coup de schéma
        // (A297) ou une pression sous-convergée ? `CYCLES=32,64,128` pour trancher.
        let cycles: Vec<u32> = std::env::var("CYCLES")
            .ok()
            .map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect())
            .unwrap_or_else(|| vec![Config::review().cycles]);
        let mut cas = Vec::new();
        for c in &cycles {
            for (nom, mut config) in [("temoin", Config::review().without_packet()), ("paquet", Config::review())] {
                config.cycles = *c;
                cas.push((format!("{nom}_c{c}"), config));
            }
        }
        for (nom, config) in cas {
            let Domain3 { nx, ny, dx, .. } = config.domain;
            let pas = (secondes * 1e6 / config.step_us as f64) as u64;
            let mut live = Live::new(Step3::new(background, config.domain, config.origin, RHO, G).await?, config, 0)?;
            let dt = config.step_us as f32 * 1e-6;
            let p = config.packet;
            let (mut avant, mut courant): (Vec<f32>, Vec<f32>) = (live.step.published()?, Vec::new());
            let mut d2s: Vec<f32> = Vec::with_capacity((pas as usize) * nx * ny);
            let (mut pire, mut lieu, mut quand, mut localite) = (0f32, 0usize, 0u64, 0f32);
            for n in 0..pas {
                live.advance(background)?;
                let apres = live.step.published()?;
                if !courant.is_empty() {
                    for c in 0..nx * ny {
                        let d2 = apres[c] - 2. * courant[c] + avant[c];
                        d2s.push(d2.abs());
                        if d2.abs() > pire {
                            pire = d2.abs();
                            lieu = c;
                            quand = n;
                            let (i, j) = (c % nx, c / nx);
                            let (mut somme, mut compte) = (0f32, 0f32);
                            for (di, dj) in [(-1i32, -1i32), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                                let (a, b) = (i as i32 + di, j as i32 + dj);
                                if a >= 0 && b >= 0 && (a as usize) < nx && (b as usize) < ny {
                                    let v = b as usize * nx + a as usize;
                                    somme += (apres[v] - 2. * courant[v] + avant[v]).abs();
                                    compte += 1.;
                                }
                            }
                            localite = pire / (somme / compte).max(1e-12);
                        }
                    }
                    avant = std::mem::take(&mut courant);
                } else {
                    // Premier pas : pas encore de dérivée seconde.
                }
                courant = apres;
            }
            // Rugosité à l'échelle de la maille sur la dernière hauteur publiée : |η − moyenne des
            // quatre voisines|. C'est ce qu'un à-coup laisse **voir** ; le signal physique en donne
            // `a·k²·dx²` pour une onde de longueur `λ`.
            let (mut rug_max, mut rug_somme, mut rug_n) = (0f32, 0f64, 0u64);
            for j in 1..ny - 1 {
                for i in 1..nx - 1 {
                    let c = j * nx + i;
                    let moyenne = 0.25 * (courant[c - 1] + courant[c + 1] + courant[c - nx] + courant[c + nx]);
                    let r = (courant[c] - moyenne).abs();
                    rug_max = rug_max.max(r);
                    rug_somme += (r as f64) * (r as f64);
                    rug_n += 1;
                }
            }
            let rug_rms = (rug_somme / rug_n as f64).sqrt();
            let courbure_paquet = p.amplitude * (std::f32::consts::TAU / p.wavelength).powi(2) * dx * dx;
            println!(
                "DELTA3D_ACOUPS_S302 cas={nom} rugosite_maille_rms={rug_rms:.3e} m max={rug_max:.3e} courbure_paquet_attendue={courbure_paquet:.3e}"
            );
            d2s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let q = |f: f64| d2s[((d2s.len() as f64 - 1.) * f) as usize];
            let rms = (d2s.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>() / d2s.len() as f64).sqrt();
            let au_dessus = |s: f32| d2s.iter().filter(|x| **x > s).count();
            let (x, y) = (config.origin[0] + ((lieu % nx) as f32 + 0.5) * dx, config.origin[1] + ((lieu / nx) as f32 + 0.5) * dx);
            let lisse_paquet = p.amplitude * G * std::f32::consts::TAU / p.wavelength * dt * dt;
            println!(
                "DELTA3D_ACOUPS_S302 cas={nom} pas={pas} echantillons={} d2_rms={rms:.3e} m q99={:.3e} q99_99={:.3e} max={pire:.3e} au_pas={quand} en=({x:.1},{y:.1}) localite={localite:.1} au_dessus_1mm={} au_dessus_0_5mm={} reference_lisse_paquet={lisse_paquet:.3e}",
                d2s.len(), q(0.99), q(0.9999), au_dessus(1e-3), au_dessus(5e-4)
            );
        }
        Ok(())
    })
}

/// S342, porte C, critère 1 — **le fond par tuiles, au bit du fond face par face.** Scène de la porte B :
/// les 26 champs des 1 148 896 faces évalués par les deux noyaux, à trois instants, comparés bit à bit ;
/// puis deux pas de production depuis le même état, l'un par tuiles, l'autre face par face, 60 pas,
/// surface publiée comparée bit à bit. Un écart est publié par champ, jamais masqué.
/// `--delta3d-fond-tuiles`.
pub fn identite_fond() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = Config::review();
        let (u, v, w, eta) = config.initial_state();
        let mut carte = Step3::new(background, config.domain, config.origin, RHO, G).await?;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        carte.set_state(&u, &v, &w, &eta)?;
        println!("DELTA3D_FOND_TUILES_S342 carte={:?} composantes={} tuiles_par_defaut={}",
            carte.adapter, background.components().len(), carte.tiled_background());
        for n in [0u64, 50, 500] {
            carte.publish_time(background, water_core::SimTime(n * config.step_us))?;
            carte.set_tiled_background(false)?;
            carte.run_background_for_bench()?;
            let a = carte.faces_values()?;
            carte.set_tiled_background(true)?;
            carte.run_background_for_bench()?;
            let b = carte.faces_values()?;
            let mut par_champ = [0usize; crate::delta3d_step::STEP_FIELDS];
            let (mut differents, mut pire) = (0usize, 0f32);
            for (idx, (x, y)) in a.iter().zip(&b).enumerate() {
                if x.to_bits() != y.to_bits() {
                    differents += 1;
                    par_champ[idx % crate::delta3d_step::STEP_FIELDS] += 1;
                    if x.is_finite() && y.is_finite() {
                        pire = pire.max((x - y).abs());
                    }
                }
            }
            println!(
                "DELTA3D_FOND_TUILES_S342 pas={n} valeurs={} differentes_au_bit={differents} pire_ecart={pire:.3e} par_champ={par_champ:?}",
                a.len()
            );
        }
        // Deux pas de production, 60 pas.
        let mut cartes = Vec::new();
        for tuiles in [true, false] {
            let c = Step3::new(background, config.domain, config.origin, RHO, G).await?;
            c.set_step(config.step_us, config.rest, config.sponge)?;
            c.set_state(&u, &v, &w, &eta)?;
            c.set_tiled_background(tuiles)?;
            cartes.push(c);
        }
        for n in 0..60u64 {
            for c in cartes.iter_mut() {
                c.step(background, water_core::SimTime(n * config.step_us), config.cycles)?;
            }
        }
        let (a, b) = (cartes[0].published()?, cartes[1].published()?);
        let differentes = a.iter().zip(&b).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
        println!("DELTA3D_FOND_TUILES_S342 pas_de_production=60 colonnes={} differentes_au_bit={differentes}", a.len());
        Ok(())
    })
}

/// S343, porte C — **l'empreinte du pas de production** sur la scène de la porte B : après 60 et 600 pas, une
/// empreinte FNV des bits de la surface publiée et des vitesses. Relevée avant un changement qui se veut
/// « au bit », rejouée après : les deux doivent être égales. `--delta3d-empreinte`.
pub fn empreinte() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let config = Config::review();
        let (u, v, w, eta) = config.initial_state();
        let mut carte = Step3::new(background, config.domain, config.origin, RHO, G).await?;
        carte.set_step(config.step_us, config.rest, config.sponge)?;
        carte.set_state(&u, &v, &w, &eta)?;
        let fnv = |valeurs: &[f32]| -> u64 {
            valeurs.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, x| {
                x.to_bits().to_le_bytes().iter().fold(h, |h, b| (h ^ *b as u64).wrapping_mul(0x100_0000_01b3))
            })
        };
        println!("DELTA3D_EMPREINTE_S343 carte={:?} fond_par_tuiles={}", carte.adapter, carte.tiled_background());
        for n in 0..600u64 {
            carte.step(background, water_core::SimTime(n * config.step_us), config.cycles)?;
            if n + 1 == 60 || n + 1 == 600 {
                let (h, vit) = (carte.published()?, carte.velocities()?);
                println!(
                    "DELTA3D_EMPREINTE_S343 pas={} surface=0x{:016x} vitesses=0x{:016x} h0={:.9} h_milieu={:.9}",
                    n + 1, fnv(&h), fnv(&vit), h[0], h[h.len() / 2]
                );
            }
        }
        Ok(())
    })
}

/// S345, porte C, critère 2 — **la scène de B à 30 Hz contre 60 Hz.** Quatre domaines : le front de S302 et son
/// témoin, au pas de 16,667 ms, puis de 33,333 ms. Chaque seconde : l'onde isolée (avec − témoin) — la position
/// de son maximum et sa valeur — aux deux cadences, et les colonnes hors bornes. `--delta3d-cadence-scene`.
pub fn cadence_scene() -> Result<(), String> {
    pollster::block_on(async {
        let scene = crate::scene::Scene::build(true, false, None);
        let background = &scene.background;
        let secondes: u64 = std::env::var("SECONDES").ok().and_then(|v| v.parse().ok()).unwrap_or(12);
        // `CADENCES=pas_us:cycles,…` — la première est la référence. Défaut : 60 Hz et 30 Hz à 32 cycles. Le témoin
        // de sensibilité, `16667:32,16667:64`, change la projection au lieu de la cadence (S345 P3).
        let liste: Vec<(u64, u32)> = std::env::var("CADENCES")
            .unwrap_or_else(|_| "16667:32,33333:32".into())
            .split(',')
            .filter_map(|x| {
                let mut it = x.split(':');
                Some((it.next()?.parse().ok()?, it.next()?.parse().ok()?))
            })
            .collect();
        let mut cadences = Vec::new();
        for &(pas_us, cycles) in &liste {
            let mut config = Config::review();
            config.step_us = pas_us;
            config.cycles = cycles;
            let mut avec = Live::new(Step3::new(background, config.domain, config.origin, RHO, G).await?, config, 0)?;
            let temoin_cfg = config.without_packet();
            let mut sans = Live::new(Step3::new(background, config.domain, config.origin, RHO, G).await?, temoin_cfg, 0)?;
            let (nx, dx) = (config.domain.nx, config.domain.dx);
            let colonne = |c: usize| (config.origin[0] + ((c % nx) as f32 + 0.5) * dx, config.origin[1] + ((c / nx) as f32 + 0.5) * dx);
            let mut releves = Vec::new();
            let mut hors = 0u32;
            // Pas par seconde arrondis : 60 et 30, pour des relevés aux mêmes instants à 30 µs près.
            let pas_par_seconde = (1_000_000 + pas_us / 2) / pas_us;
            let total = secondes * pas_par_seconde;
            for n in 0..=total {
                if n % pas_par_seconde == 0 {
                    let (pa, ps) = (avec.step.published()?, sans.step.published()?);
                    let (mut onde, mut lieu) = (0f32, 0usize);
                    for c in 0..pa.len() {
                        let d = (pa[c] - ps[c]).abs();
                        if d > onde { onde = d; lieu = c; }
                    }
                    releves.push((n / pas_par_seconde, onde, colonne(lieu)));
                }
                if n == total { break; }
                for live in [&mut avec, &mut sans] {
                    live.advance(background)?;
                    if let Some(d) = live.last.take() {
                        hors = hors.max(d.columns_outside);
                    }
                }
            }
            println!("DELTA3D_CADENCE_S345 scene pas_ms={:.3} cycles={cycles} hors_bornes_max={hors}", pas_us as f64 * 1e-3);
            cadences.push(releves);
        }
        let (mut pire_position, mut pire_amplitude) = (0f32, 0f32);
        for (a, b) in cadences[0].iter().zip(&cadences[1]) {
            let ecart = ((a.2 .0 - b.2 .0).powi(2) + (a.2 .1 - b.2 .1).powi(2)).sqrt();
            let rapport = if a.1 > 0. { b.1 / a.1 - 1. } else { 0. };
            if a.0 > 0 {
                pire_position = pire_position.max(ecart);
                pire_amplitude = pire_amplitude.max(rapport.abs());
            }
            println!(
                "DELTA3D_CADENCE_S345 scene t={}s onde_60hz={:.4} en=({:.2},{:.2}) onde_30hz={:.4} en=({:.2},{:.2}) ecart_position_m={ecart:.3} ecart_amplitude_pct={:.2}",
                a.0, a.1, a.2 .0, a.2 .1, b.1, b.2 .0, b.2 .1, 100. * rapport
            );
        }
        println!("DELTA3D_CADENCE_S345 scene bilan pire_ecart_position_m={pire_position:.3} pire_ecart_amplitude_pct={:.2}", 100. * pire_amplitude);
        Ok(())
    })
}

