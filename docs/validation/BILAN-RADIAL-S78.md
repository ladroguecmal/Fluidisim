# Bilan temporel radial S78 — 2026-09-08

Reproduction : `cargo test --release --manifest-path code/Cargo.toml -p water-core temporal_energy_transport_and_truncation_s78 -- --nocapture`.
Scénario S77 : E=0,01 J, lambda=4 m, g=9,81, rho=1025, h=20 m, isotrope.
Domaine de calcul étendu à rayon 20 m, âge 4 s ; N64 et N128 admis par les contrôles.

## Mesure physique

Pour chaque nœud i, le coefficient de potentiel vaut -c_i omega_i/k_i sin(phase_i).
Son gradient radial vaut c_i omega_i J1(k_i r) sin(phase_i), son gradient vertical
-c_i omega_i J0(k_i r) sin(phase_i), multipliés par exp(k_i z).
L’intégration en profondeur des produits de gradients introduit 1/(k_i+k_j).
La densité totale est rho/2*(g eta² + intégrale |grad phi|² dz), positive.
L’énergie du disque s’obtient par 2pi intégrale r*density dr ; le rayon moyen en est le
premier moment divisé par le total. Aucun poids psi*deta_dt local n’est utilisé (L190).

La densité est calculée en f64 depuis les coefficients effectivement construits et les Bessel
et phases de production. Elle vérifie leur bilan assemblé ; elle n’est pas une référence
indépendante de ces noyaux, contrôlés séparément en S77. La profondeur reste celle du modèle
profond infini, pas une simulation de la limite basse à h=20 m.

## Résultats

Énergie rapportée aux 0,01 J prescrits. Colonnes R=8/16 : N64, 256 anneaux.
Colonnes R=20 et rayon moyen : N128, 512 anneaux.

| Temps (s) | E(R8)/E0 | E(R16)/E0 | E(R20)/E0 | Rayon moyen R20 (m) |
|---|---|---|---|---|
| 0 | 0,99998461 | 1,00052365 | 1,00020484 | 1,160128 |
| 1 | 0,99992087 | 1,00021345 | 1,00008352 | 1,555775 |
| 2 | 0,99951622 | 1,00017979 | 1,00007093 | 2,320764 |
| 4 | 0,99318964 | 0,99999500 | 1,00000146 | 4,456888 |

| Temps (s) | Écart anneaux 256→512, N64 R20 / E0 | Écart N64→128, 512 anneaux R20 / E0 |
|---|---|---|
| 0 | 0,00061947 | 0,00000002 |
| 1 | 0,00025503 | 0,00000007 |
| 2 | 0,00021800 | 0,00000002 |
| 4 | 0,00001293 | 0,00000007 |

Densités minimales échantillonnées positives à chaque mesure ; minimum global 1,469e-14 J/m².
Le déficit d’environ 0,681 % dans R8 à 4 s est récupéré en élargissant le disque. Le résultat
R20 n’indique pas de perte d’énergie de cet ordre ; son biais est dominé par la quadrature des
anneaux, pas le nombre de nœuds spectraux. Une valeur légèrement supérieure à 1 n’est pas une
création physique d’énergie : elle décroît quand on raffine la quadrature spatiale.

La croissance du rayon moyen démontre un transport dans ce scénario. Elle ne mesure pas la
vitesse de groupe d’un paquet étroit, le spectre ayant une bande large. Pas de preuve de c_g=c/2
ni d’horizon de rétention au-delà de 4 s. Une mesure à quatre instants ne certifie pas tous les
instants intermédiaires ni d’autres paramètres de milieu et de source.

Tolérances de régression annoncées avant mesure : total R20 à 0,003 relatif ; raffinement
anneaux <0,002 ; raffinement spectral <1e-4 ; déplacement moyen à 4 s >1 m ; négativité
permise uniquement à -1e-12 pour arrondi. Elles sont des contrôles numériques du scénario,
pas des seuils gameplay B2. Aucun seuil déplacé après les résultats.
