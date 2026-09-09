//! S121 P2 — le bloc de finitude de `RadialImpact::sample` est-il seulement atteignable ?
//! Recherche systématique d'un champ que `new` accepte et dont `sample` produit une sortie
//! non finie. Sonde jetable : elle répond à une question, elle ne fait pas partie du harnais.
use water_core::{
    impact_field::{ImpactField, Medium},
    radial_impact::{Domain, RadialImpact},
    wave_event::{Impact, Origin, WaveEvent},
    FrameId, SimTime,
};
fn main() {
    let decades = [
        1e-6f32, 1e-3, 1.0, 1e3, 1e6, 1e9, 1e12, 1e18, 1e24, 1e30, 1e34, 1e37, 3e38,
    ];
    let lengths = [1e-4f32, 1e-3, 0.01, 0.1, 1.0, 4.0, 100.0, 1e4, 1e10, 1e18];
    let radii = [1e-4f32, 1e-3, 0.01, 1.0, 16.0, 1000.0, 4000.0];
    let depths = [1e-3f32, 1.0, 20.0, 1e6, 1e20, 1e30];
    let slopes = [0.1f32, 1e6, 1e18, 1e30, 3e38];
    let ages = [1u64, 1_000, 4_000_000, 1_000_000_000];
    let (mut built, mut sampled, mut worst) = (0u64, 0u64, 0.0f32);
    let mut found = 0u64;
    let mut argmax = String::new();
    for &energy in &decades {
        for &wavelength in &lengths {
            for &radius in &radii {
                for &depth in &depths {
                    for &max_slope in &slopes {
                        for &age_us in &ages {
                            let Ok(event) = WaveEvent::impact(Impact {
                                id: 1,
                                frame: FrameId(7),
                                cell: 9,
                                birth: SimTime(0),
                                ttl_us: 4_000_000,
                                position: [0.0; 3],
                                energy_j: energy,
                                wavelength_m: wavelength,
                                direction_turns: 0.0,
                                anisotropy: 0.0,
                                displaced_l: 0.0,
                                material: 0,
                                origin: Origin::Server,
                                above_surface: true,
                            }) else {
                                continue;
                            };
                            let medium = Medium {
                                gravity: 9.81,
                                density: 1025.0,
                                depth,
                                max_slope,
                            };
                            let domain = Domain { radius, age_us };
                            let Ok(field) = RadialImpact::<64>::new(event, medium, domain) else {
                                continue;
                            };
                            built += 1;
                            // Points admis : dans le disque, jusqu'au bord ; temps dans l'horizon.
                            for i in 0..12 {
                                let r = radius * i as f32 / 11.0;
                                for t in [0u64, age_us / 3, age_us] {
                                    match field.sample(FrameId(7), 9, [r, 0.0], SimTime(t)) {
                                        Ok(s) => {
                                            sampled += 1;
                                            for v in [
                                                s.eta,
                                                s.deta_dt,
                                                s.potential,
                                                s.slope[0],
                                                s.horizontal_velocity[0],
                                            ] {
                                                if v.abs().is_finite() && v.abs() > worst {
                                                    worst = v.abs();
                                                    argmax = format!(
                                                        "energy={energy:e} lambda={wavelength:e} radius={radius:e} depth={depth:e} slope={max_slope:e} age={age_us} r={r:e} t={t} bound={:e}",
                                                        field.slope_bound()
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            found += 1;
                                            if found <= 5 {
                                                println!(
                                                    "REFUS {e:?} energy={energy:e} lambda={wavelength:e} radius={radius:e} depth={depth:e} slope={max_slope:e} age={age_us} r={r:e} t={t}"
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("champs construits={built} echantillons={sampled} refus={found} max_sortie={worst:e}");
    println!("cas maximal : {argmax}");
    // La sortie croît quand la longueur d'onde décroît : suivre cette direction jusqu'au bout,
    // en couplant le rayon à la contrainte hi*radius <= 64 qui borne l'argument de Bessel.
    println!("--- descente en longueur d'onde, energie et pente maximales ---");
    for e in 4..=30 {
        let wavelength = 10f32.powi(-e);
        let radius = 5.0 * wavelength;
        let Ok(event) = WaveEvent::impact(Impact {
            id: 1,
            frame: FrameId(7),
            cell: 9,
            birth: SimTime(0),
            ttl_us: 4_000_000,
            position: [0.0; 3],
            energy_j: f32::MAX,
            wavelength_m: wavelength,
            direction_turns: 0.0,
            anisotropy: 0.0,
            displaced_l: 0.0,
            material: 0,
            origin: Origin::Server,
            above_surface: true,
        }) else {
            println!("lambda=1e-{e} : evenement refuse");
            continue;
        };
        let medium = Medium {
            gravity: 9.81,
            density: 1025.0,
            depth: 1.0,
            max_slope: f32::MAX,
        };
        match RadialImpact::<64>::new(event, medium, Domain { radius, age_us: 1 }) {
            Err(why) => println!("lambda=1e-{e} : champ refuse ({why:?})"),
            Ok(field) => {
                let mut peak = 0.0f32;
                let mut refused = 0;
                for i in 0..12 {
                    match field.sample(FrameId(7), 9, [radius * i as f32 / 11.0, 0.0], SimTime(0)) {
                        Ok(s) => {
                            for v in [s.eta, s.deta_dt, s.potential, s.slope[0]] {
                                if v.is_finite() {
                                    peak = peak.max(v.abs());
                                } else {
                                    refused += 1;
                                }
                            }
                        }
                        Err(_) => refused += 1,
                    }
                }
                println!(
                    "lambda=1e-{e} bound={:e} pic={peak:e} refus={refused}",
                    field.slope_bound()
                );
            }
        }
    }
    println!("marge avant depassement f32 : {:e}", f32::MAX / worst);
    // Second site de la même confusion : ImpactField::new. Où bascule-t-il, et vers quoi ?
    println!("--- ImpactField, energie et pente maximales ---");
    for e in -6..=3 {
        for m in [1.0f32, 3.0, 5.0] {
            let wavelength = m * 10f32.powi(e);
            let Ok(event) = WaveEvent::impact(Impact {
                id: 1,
                frame: FrameId(7),
                cell: 9,
                birth: SimTime(0),
                ttl_us: 4_000_000,
                position: [0.0; 3],
                energy_j: f32::MAX,
                wavelength_m: wavelength,
                direction_turns: 0.0,
                anisotropy: 0.0,
                displaced_l: 0.0,
                material: 0,
                origin: Origin::Server,
                above_surface: true,
            }) else {
                continue;
            };
            let medium = Medium {
                gravity: 9.81,
                density: 1025.0,
                depth: (10.0 * wavelength).max(1.0),
                max_slope: f32::MAX,
            };
            match ImpactField::new(event, medium) {
                Ok(_) => println!("lambda={wavelength:e} : construit"),
                Err(why) => println!("lambda={wavelength:e} : {why:?}"),
            }
        }
    }
}
