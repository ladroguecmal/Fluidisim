# Transport radial S76 — 2026-09-08

Reproduction : `cargo test --release --manifest-path code/Cargo.toml -p water-core radial_transport_and_periodic_copies_s76 -- --nocapture`.
Scénario S75 : énergie 1 J, lambda 4 m, carré 16 m, g=9,81, rho=1025, eau profonde.

## Densité mesurée

La densité cinétique locale est rho/2 intégrale de -infini à 0 de |grad phi|² dz.
Chaque mode porte phi_i(z)=psi_i exp(k_i z). L’intégration de chaque produit de gradients
introduit donc 1/(k_i+k_j). Additionner tous les produits donne une densité physique positive.
Ajouter rho*g*eta²/2 pour la partie potentielle. Les trigonométries de mesure sont en f64,
distinctes des polynômes de sample ; elles lisent les coefficients et fréquences effectivement
construits. Le bilan mesure donc le modèle construit, pas une référence indépendante de sa source.

**psi*deta_dt n’est pas une densité cinétique locale.** C’est une identité sous intégrale
spatiale ; l’utiliser comme poids radial pourrait produire des poids négatifs. S75 l’employait
correctement pour le total, mais cela ne justifiait pas son emploi pour localiser l’énergie.

Grilles 32² et 64², centrées sur la source. Rayon moyen : intégrale rE / intégrale E.
Fractions radiales calculées sur la grille 64² ; aucune convergence de seuil de fraction revendiquée.

| t (s) | Énergie (J) | Rayon moyen (m) | Fraction r≥4 m | Fraction r<2 m | Écart rayon 32/64 (m) |
|---|---|---|---|---|---|
| 0 | 1,00000003 | 2,216990 | 0,183856 | 0,467160 | 0,001540 |
| 1 | 1,00000004 | 2,486883 | 0,208061 | 0,446861 | 0,001028 |
| 2 | 1,00000005 | 3,144400 | 0,304030 | 0,362536 | 0,000115 |
| 4 | 1,00000003 | 4,907136 | 0,674086 | 0,111394 | 0,001134 |
| 6 | 1,00000005 | 6,477046 | 0,864904 | 0,031534 | 0,003065 |
| 8 | 1,00000003 | 7,039794 | 0,893059 | 0,022277 | 0,003871 |
| 12 | 1,00000003 | 6,292903 | 0,782124 | 0,031741 | 0,003289 |

Densité minimale positive à chaque instant, minimum global échantillonné 1,409e-9 J/m².
L’énergie s’éloigne du centre ; le rayon moyen décroît entre 8 et 12 s. Cela constate un retour
vers le centre sur le tore ; on ne lui attribue pas une heure unique de contamination.
Une copie exacte en x=16 m est déjà présente à t=0 et reste identique bit à bit à t=2 et 8 s.
C’est le critère décisif contre une utilisation régionale directe, indépendamment du rayon moyen.

La vitesse du rayon moyen n’est pas la vitesse de groupe d’un paquet étroit : le spectre est
large, le carré tronque le rayon et les copies existent dès l’initialisation. Aucune validation
c_g=c/2 n’est revendiquée. Les données ne fournissent pas un délai certifié sans contamination.

Le témoin automatique demande plus de 1 m de déplacement à 4 s (quart de lambda du scénario),
une simple régression de déplacement, pas un seuil de qualité. Une première écriture le demandait
à 2 s ; les mesures donnaient 0,927 m et ont réfuté cette anticipation. Le contrôle est porté
à 4 s, où l’écart vaut 2,690 m ; aucune équivalence avec c_g n’en est déduite. Tolérances de
calcul : énergie 2e-5 relatif, densité -1e-12 pour arrondi, rayon 32/64 écart <0,03 m.
