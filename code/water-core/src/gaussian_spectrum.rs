//! S97 : recette versionnée, cuisson f32 sur pool, sans libm.
use crate::{spectral_pressure::Node, Hasher64, PhaseQ32};
#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub sigma: f32,
    pub cutoff: f32,
    pub radial: usize,
    pub angular: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Domain,
    Capacity,
    NotConjugate,
}
/// Demi-spectre reçu : poids doublés, hash de la recette complète conservé pour provenance.
pub struct HalfSpectrum<'a> {
    nodes: &'a [Node],
    source_hash: u64,
    recipe: Recipe,
}
impl HalfSpectrum<'_> {
    pub fn nodes(&self) -> &[Node] {
        self.nodes
    }
    pub fn source_hash(&self) -> u64 {
        self.source_hash
    }
    pub fn recipe(&self) -> Recipe {
        self.recipe
    }
}
pub struct Spectrum<'a> {
    nodes: &'a [Node],
    recipe: Recipe,
    hash: u64,
}
impl Spectrum<'_> {
    /// Réduction exacte de la géométrie cuite, avant tout calcul modal.
    /// Tous les contrôles précèdent l'écriture ; pas de correction silencieuse des directions.
    pub fn half_into<'a>(&self, pool: &'a mut [Node]) -> Result<HalfSpectrum<'a>, Error> {
        let angular = self.recipe.angular;
        let half = angular / 2;
        if pool.len() < self.nodes.len() / 2 {
            return Err(Error::Capacity);
        }
        for ring in self.nodes.chunks_exact(angular) {
            for j in 0..half {
                let a = ring[j];
                let b = ring[j + half];
                if a.k[0] != -b.k[0]
                    || a.k[1] != -b.k[1]
                    || a.transform.to_bits() != b.transform.to_bits()
                    || a.weight.to_bits() != b.weight.to_bits()
                {
                    return Err(Error::NotConjugate);
                }
            }
        }
        for (i, ring) in self.nodes.chunks_exact(angular).enumerate() {
            for j in 0..half {
                pool[i * half + j] = Node {
                    weight: ring[j].weight * 2.0,
                    ..ring[j]
                };
            }
        }
        Ok(HalfSpectrum {
            nodes: &pool[..self.nodes.len() / 2],
            source_hash: self.hash,
            recipe: self.recipe,
        })
    }
    pub fn nodes(&self) -> &[Node] {
        self.nodes
    }
    pub fn recipe(&self) -> Recipe {
        self.recipe
    }
    /// FNV-1a de la version, recette et valeurs cuites ; diagnostic, pas authentification.
    pub fn hash(&self) -> u64 {
        self.hash
    }
}
// exp(-x), 0<=x<=32. Réduction x=n ln2+r, puis Taylor degré 9 et puissance de deux exacte.
fn decay(x: f32) -> f32 {
    let n = (x / core::f32::consts::LN_2) as u32;
    let r = x - n as f32 * core::f32::consts::LN_2;
    let p = -1.0 / 362880.0;
    let p = p * r + 1.0 / 40320.0;
    let p = p * r - 1.0 / 5040.0;
    let p = p * r + 1.0 / 720.0;
    let p = p * r - 1.0 / 120.0;
    let p = p * r + 1.0 / 24.0;
    let p = p * r - 1.0 / 6.0;
    let p = p * r + 0.5;
    let p = p * r - 1.0;
    let p = p * r + 1.0;
    p * f32::from_bits((127 - n) << 23)
}
/// Recette V1 : sigma 1/16..64 m, coupure réduite sigma*kmax 1..8,
/// radial 1..512, directions paires 4..512. Représentabilité, pas réception universelle.
pub fn bake(recipe: Recipe, pool: &mut [Node]) -> Result<Spectrum<'_>, Error> {
    let r = recipe;
    let reduced = r.sigma * r.cutoff;
    if !r.sigma.is_finite()
        || !(0.0625..=64.0).contains(&r.sigma)
        || !r.cutoff.is_finite()
        || !(1.0..=8.0).contains(&reduced)
        || !(1..=512).contains(&r.radial)
        || !(4..=512).contains(&r.angular)
        || r.angular % 2 != 0
    {
        return Err(Error::Domain);
    }
    let count = r.radial * r.angular;
    if pool.len() < count {
        return Err(Error::Capacity);
    }
    let dk = r.cutoff / r.radial as f32;
    let da = core::f32::consts::TAU / r.angular as f32;
    let mut hash = Hasher64::new();
    hash.write_u32(1);
    hash.write_f32(r.sigma);
    hash.write_f32(r.cutoff);
    hash.write_u32(r.radial as u32);
    hash.write_u32(r.angular as u32);
    for i in 0..r.radial {
        let k = (i as f32 + 0.5) * dk;
        let sk = r.sigma * k;
        let transform = core::f32::consts::TAU * r.sigma * r.sigma * decay(0.5 * sk * sk);
        let weight = k * dk * da / (core::f32::consts::TAU * core::f32::consts::TAU);
        for j in 0..r.angular {
            let angle =
                PhaseQ32((((2 * j + 1) as u64 * (1u64 << 32)) / (2 * r.angular) as u64) as u32);
            let node = Node {
                k: [k * angle.cos(), k * angle.sin()],
                transform,
                weight,
            };
            for v in [node.k[0], node.k[1], node.transform, node.weight] {
                hash.write_f32(v);
            }
            pool[i * r.angular + j] = node;
        }
    }
    Ok(Spectrum {
        nodes: &pool[..count],
        recipe: r,
        hash: hash.finish(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pairs_and_refusal_s99() {
        for angular in (4..=512).step_by(2) {
            let mut pool = vec![Node::default(); angular];
            let s = bake(
                Recipe {
                    sigma: 1.0,
                    cutoff: 6.0,
                    radial: 1,
                    angular,
                },
                &mut pool,
            )
            .unwrap();
            let mut out = vec![Node::default(); angular / 2 + 1];
            out[angular / 2].weight = 123.0;
            assert!(s.half_into(&mut out).is_ok(), "angular={angular}");
            assert_eq!(out[angular / 2].weight, 123.0);
            assert!(matches!(s.half_into(&mut []), Err(Error::Capacity)));
        }
        let mut nodes = [Node::default(); 4];
        let s = bake(
            Recipe {
                sigma: 1.0,
                cutoff: 6.0,
                radial: 1,
                angular: 4,
            },
            &mut nodes,
        )
        .unwrap();
        let recipe = s.recipe;
        let hash = s.hash;
        nodes[2].k[0] += 0.01;
        let bad = Spectrum {
            nodes: &nodes,
            recipe,
            hash,
        };
        let mut out = [Node {
            weight: 123.0,
            ..Node::default()
        }; 2];
        assert!(matches!(bad.half_into(&mut out), Err(Error::NotConjugate)));
        assert_eq!(out[0].weight, 123.0);
        assert_eq!(out[1].weight, 123.0);
    }
    #[test]
    fn decay_profile_and_recipe_s97() {
        let mut worst = 0.0f64;
        for i in 0..32001 {
            let x = i as f32 / 1000.0;
            worst = worst.max((decay(x) as f64 / (-(x as f64)).exp() - 1.0).abs());
        }
        println!("S97 exp relative={worst:.9e}");
        assert!(worst < 3e-6);
        let mut pool = vec![Node::default(); 16385];
        pool[16384].weight = 123.0;
        let r = Recipe {
            sigma: 1.0,
            cutoff: 6.0,
            radial: 128,
            angular: 128,
        };
        let s = bake(r, &mut pool).unwrap();
        let hash = s.hash();
        let mut worst = 0.0f64;
        for p in [[0.0, 0.0], [1.0, 0.0], [2.0, 1.0], [5.0, 0.0]] {
            let profile: f64 = s
                .nodes()
                .iter()
                .map(|n| {
                    n.weight as f64
                        * n.transform as f64
                        * (n.k[0] as f64 * p[0] + n.k[1] as f64 * p[1]).cos()
                })
                .sum();
            worst = worst.max((profile - (-0.5f64 * (p[0] * p[0] + p[1] * p[1])).exp()).abs());
        }
        println!("S97 profile error={worst:.9e} hash={hash:016x}");
        assert!(worst < 1e-4);
        assert_eq!(pool[16384].weight, 123.0);
        assert_eq!(bake(r, &mut pool).unwrap().hash(), hash);
        assert_eq!(hash, 0x20e6_4a39_2ae2_37a1);
        assert!(matches!(bake(r, &mut []), Err(Error::Capacity)));
        for bad in [
            Recipe {
                sigma: f32::NAN,
                ..r
            },
            Recipe { cutoff: 9.0, ..r },
            Recipe { angular: 3, ..r },
            Recipe { radial: 0, ..r },
        ] {
            assert!(matches!(bake(bad, &mut pool), Err(Error::Domain)));
            assert_eq!(pool[16384].weight, 123.0);
        }
    }
    #[test]
    fn recipe_limits_against_independent_nodes_s97() {
        let mut worst = [0.0f64; 3];
        for sigma in [0.0625f32, 1.0, 64.0] {
            for reduced in [1.0, 6.0, 8.0] {
                for (radial, angular) in [(1, 4), (17, 18), (512, 512)] {
                    let r = Recipe {
                        sigma,
                        cutoff: reduced / sigma,
                        radial,
                        angular,
                    };
                    let mut pool = vec![Node::default(); radial * angular];
                    let s = bake(r, &mut pool).unwrap();
                    let dk = r.cutoff as f64 / radial as f64;
                    let da = core::f64::consts::TAU / angular as f64;
                    for (index, n) in s.nodes().iter().enumerate() {
                        let k = (index / angular) as f64 * dk + 0.5 * dk;
                        let a = ((index % angular) as f64 + 0.5) * da;
                        let t = core::f64::consts::TAU
                            * (sigma as f64).powi(2)
                            * (-0.5 * (sigma as f64 * k).powi(2)).exp();
                        let w = k * dk * da / core::f64::consts::TAU.powi(2);
                        worst[0] = worst[0]
                            .max((n.k[0] as f64 / k - a.cos()).abs())
                            .max((n.k[1] as f64 / k - a.sin()).abs());
                        worst[1] = worst[1].max((n.transform as f64 / t - 1.0).abs());
                        worst[2] = worst[2].max((n.weight as f64 / w - 1.0).abs());
                    }
                }
            }
        }
        println!("S97 nodes direction/transform/weight errors={worst:?}");
        assert!(worst[0] < 3e-7 && worst[1] < 1e-5 && worst[2] < 5e-7);
    }
}
