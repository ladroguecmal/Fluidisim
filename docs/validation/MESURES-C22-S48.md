# C22 sur le second véhicule — S48, 2026-09-07

Action S47-1. Mesure de convergence sur un support régulier, indépendante du diagnostic
Ritter conservé en S47. Aucun résultat de production ni validation du modèle 3D.

## Montage exécuté

- Domaine [0,40] m, murs réfléchissants, h0=1 m, u0=0 ; centre de bosse à 20 m.
- Gaussienne h=1+0,01 exp(-(x-20)^2/2), écart-type 1 m, temps final 1 s.
- `shallow.rs`, HLL, MUSCL/minmod et SSP-RK2, CFL=0,45, précision f64.
- Cinq grilles emboîtées : 100, 200, 400, 800, 1600 cellules ; erreur L1 relative sur h.
- Initialisation aux centres comme les constructeurs existants ; l'erreur mesurée inclut donc
  cette approximation de la moyenne initiale. Aucune nouvelle moyenne analytique introduite.
- Chaque champ grossier est comparé à la moyenne conservative des cellules de l'oracle.
  NaN, infinis, tailles non emboîtées, temps final non atteint et saturations sont refusés.

**La largeur demandait une conversion.** `Bassin::c08_regulier` utilise exp(-d²/2),
`configure_bosse` exp(-d²). Le second reçoit donc sqrt(2) m pour représenter l'écart-type
de 1 m du cas. Passer 1 m des deux côtés aurait changé le montage.

## Influence de l'oracle

Chaque campagne utilise deux oracles n et 2n. L'écart L1 entre eux est imprimé ; le facteur
30 de C22 est appliqué à cet écart mesuré. Seul le préfixe de grilles dépassant ce seuil est
retenu, pour ne pas reconstruire des triplets séparés par une grille rejetée.

**Ce filtre est empirique.** L'écart de deux solutions numériques n'est pas une borne prouvée
de leur erreur commune. Il remplace ici l'extrapolation fondée sur p du premier véhicule ;
les deux procédures sont donc distinguées. Le raffinement de l'oracle teste cette sensibilité.
Le plancher arithmétique est n_fin·epsilon_f64, indication conservatrice du cumul de sommation.

| Oracles | Écart relatif entre oracles | Grilles retenues | Ordres des triplets | Verdict |
|---|---:|---:|---|---|
| 6400 / 12800 | 2,520893353e-8 | 4/5 | 1,637583 ; 1,631704 | stabilité non établie |
| 12800 / 25600 | 6,298374276e-9 | 5/5 | 1,637633 ; 1,631728 ; 1,849853 | non asymptotique |
| 25600 / 51200 | 1,717298105e-9 | 5/5 | 1,637646 ; 1,631733 ; 1,849841 | non asymptotique |

Sur la deuxième campagne, erreurs contre l'oracle fin :

| Cellules | Erreur L1 relative |
|---|---:|
| 100 | 7,514815926e-5 |
| 200 | 2,380587704e-5 |
| 400 | 7,305325781e-6 |
| 800 | 1,980573696e-6 |
| 1600 | 5,033773948e-7 |

Le verdict utilise le même `Convergence` et le même bilan que le premier véhicule : ordre
retenu sur le triplet grossier (référence Oracle), stabilité requise avant comparaison à 0,8.
L'ordre retenu vaut environ 1,64, mais sa suite n'est pas stable. **Cinq grilles sont une
condition nécessaire, pas une garantie de conclusion.** Le solveur n'est déclaré ni validé
ni faux par cette campagne.

## Exécution et coût

Depuis code/ : `cargo run --offline --release -- c22-shallow 12800`.
Le paramètre désigne le premier oracle ; le second a deux fois plus de cellules.
Les valeurs admises sont les multiples de 1600 de 3200 à 51200. Sans paramètre : 12800.
Ce mode est séparé de physics pour rendre son coût explicite et préserver le budget courant.
Le code de sortie zéro indique l'absence d'échec, pas une validation ; lire le bilan sans-verdict.

Coûts mesurés localement, dépendants de la machine et de sa charge : 5,416 s pour 6400/12800,
22,159 s pour 12800/25600, 102,612 s pour 25600/51200 (suite de tests exécutée en parallèle
pendant une partie de cette dernière mesure). Ces coûts sont compatibles avec la croissance
en n² d'ADR-032, sans constituer un benchmark de performance contrôlé.

La dernière campagne conserve les trois ordres à moins de 0,000013 près par rapport à la
précédente. Affiner seulement l'oracle ne stabilise donc pas cette famille de grilles.
Les erreurs finales contre 51200 sont 7,514852894e-5 ; 2,380667708e-5 ; 7,306411789e-6 ;
1,981772242e-6 ; 5,045954241e-7 (100 à 1600 cellules).

**Vérifications :** 104 tests réussis (38 cœur + 66 harnais), deux ignorés. Essai sans bosse
exactement à h=1 ; projection conservative sur moyennes connues ; refus de tailles invalides,
non-finis et norme nulle. check : deux scénarios, zéro échec, hashs inchangés. Taille d'oracle
invalide refusée par la commande avec code de sortie 1. Les solveurs et anciens montages ne
sont pas modifiés ; la nouvelle campagne est appelée par son mode dédié.

## Limites et suite

Le montage mesure l'ordre du couple shallow/C22 à ce temps, avec minmod, en 1D.
Il ne remplace pas B3. Pour éprouver la stabilisation vers l'ordre deux, déplacer la fenêtre
de grilles vers les plus fines tout en vérifiant leur séparation de l'erreur d'oracle ;
ne pas changer le seuil de stabilité pour obtenir un succès. Action S48-1.
