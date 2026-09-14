# Surface linéarisée — S233, 2026-09-14

## Protocole avant construction

ADR-141 : pression puis hauteur, géométrie fixe et modèle linéaire sans advection.
Q_i = somme_k ouverture_u(i,k) u(i,k) dx ; η_i nouveau = η_i − dt(Q_{i+1}−Q_i)/dx.
Faces latérales fermées : somme des déplacements nulle en arithmétique exacte.
La somme se télescope indépendamment du résidu de projection ; vérifier sa dérive d'arrondi.
Les flux sont ceux du candidat, pas d'un second solveur de surface.

Oracle indépendant : onde stationnaire linéaire en bassin rectangulaire, η−z₀ =
A cos(kx) cos(ωt), k=π/L, ω²=gk tanh(kh), dispersion de SPEC-001 §1.
L8m, h4m, fond0, A0,01m, g9,81 et1,62 ; vitesse initiale nulle. Comparer sur1s,
résolutions16/32/64 et pas2ms/1ms. Publier erreurs maximales normalisées par A,
réduction de l'erreur et masse ; critère de qualité1% sur le cas fin, tolérance de banc
déclarée ici (pas seuil de bascule B4 ni admissibilité universelle).
Témoin discriminant : hauteur imposée inchangée ne peut reproduire cos(ωt).

Recevoir repos exact, évolution/retour de signe et pression consommant η modifiée ;
non-convergence et tout point d'expiration doivent préserver les quatre champs puis
autoriser une reprise identique, sans allocation. Le mode exige un couvercle entièrement
ouvert, g positif et une durée satisfaisant dt²g/dx≤1, garde conservatrice du schéma linéaire
sur grille fixe ; aucune stabilité non linéaire revendiquée. Temps en microsecondes entières.
Le stockage supplémentaire, le coût complet et les limites sont publiés à la réception.

## Chemin construit

`Volume::step_surface_linear(duration_us, max_iters, budget_us, jobs, clock)` emprunte les
vitesses du pas précédent, projette sans advection quadratique puis transporte la hauteur.
`surface()` expose le résultat. L'ancienne API imposée demeure disponible : aucun mode
persistant implicite, l'appelant choisit explicitement le chemin. `SurfaceReport` annonce
durée avancée/restante entière ; expiration = zéro avancée, `Convergence` = refus atomique.
Une nouvelle hauteur extérieure via `set_surface` réinitialise aussi son reste d'arrondi.

Les déplacements inférieurs à l'ulp de la hauteur absolue sont conservés par une somme
compensée f32. Ce reste intervient dans la hauteur dynamique de pression au pas suivant,
et fait partie de la sauvegarde atomique. Aucune compensation f64 des champs.
La durée ne devient jamais f32 : seuls les coefficients ρ/dt, dt/ρ, dt/dx le deviennent,
suivant ADR-141. Pas d'horloge simulée cachée ; le consommateur compte les microsecondes.

La garde dt²g/dx≤1 vient du système linéaire fixe : le complément de Schur de l'opérateur
de pression symétrique positif au couvercle a ses valeurs propres entre0 et2/dx. Donc
ω_max²≤2g/dx ; Euler symplectique pression puis hauteur exige dt²ω²<4. La garde conserve
une marge (≤2). Elle suppose le système résolu, un couvercle entièrement ouvert et g>0 ;
elle ne reçoit ni l'advection ni un changement de topologie. Le code refuse ces entrées
géométriques incompatibles ; la réception numérique reste celle des cas ci-dessous.

## Contre-épreuves rencontrées

1. Première boucle : à64×32 et1ms, refus au pas307, résidu réel1,02063e-6. Le seuil1e-6
   est conservé. L'arrêt S231 dès une hausse isolée du résidu réel était trop précoce en
   f32 : les corrections continuent désormais sous le même plafond global, avec arrêt
   si CG n'effectue plus aucune itération. Aucun nombre de corrections hors budget.
2. Sans compensation des hauteurs, erreur lunaire0,597% à64/1ms, croissante en raffinant.
   La précision de l'accumulation de déplacements était limitante. Le reste f32 conservé
   et consommé par la pression rend le raffinement utile, sans changer le modèle physique.

## Onde stationnaire et repos

Erreurs maximales sur tous les points et tous les pas jusqu'à1s, normalisées par A ;
la référence cos(kx)cos(ωt) ne réutilise ni pression ni flux numériques.

| nx | pas (µs) | erreur Terre (%) | erreur Lune (%) |
|---|---:|---:|---:|
| 16 | 2000 | 0,4690 | 0,1371 |
| 32 | 2000 | 0,0854 | 0,0176 |
| 64 | 2000 | 0,1573 | 0,0430 |
| 64 | 1000 | 0,0659 | 0,0173 |

Erreur64/2ms inférieure à16/2ms et erreur64/1ms inférieure à64/2ms pour les deux gravités.
Le niveau32 meilleur que64 à2ms indique une compensation d'erreurs spatiale/temporelle :
aucun ordre spatial n'est déduit de ce tableau. Le critère1% déclaré avant construction
est satisfait. Sous gravité terrestre, la hauteur de la première colonne change de signe
relatif à z₀, ce qu'un couvercle imposé immobile ne peut produire. Dérive de hauteur moyenne
≤4,48e-8m sur ces huit trajectoires ; ce n'est pas une conservation entière de masse V.
Repos exact en bits reçu sur100pas à fond coupé, en plus des contrôles historiques du noyau.

## Limites de réception

Géométrie de surface encore fixe à z₀ ; η est son déplacement **linéarisé**. Pas de cuisson
de géométrie mobile, hauteur multivaluée, mouillage/séchage, cavité, air ni 3D. Aucun couplage
B/W et aucune autorité gameplay. Aucun domaine d'amplitude non linéaire reçu par l'erreur
de l'onde de1cm ; l'API nomme le modèle, elle ne transforme pas de grandes amplitudes en
petites amplitudes valides. Une future représentation mobile reste obligatoire (ADR-127).
I-05 mural, admission des charges et migration des API temporelles historiques restent ouverts.

## Intégrité, stockage et coût

213 points d'expiration parcourus sur un état déjà avancé, avec contrôle de u/w/p/η en bits
et reprise complète identique (donc reste d'arrondi conservé aussi). Zéro allocation avec
témoin positif, refus de durée/budget invalides, pression non convergée, horloge reculant,
entrée provoquant débordement numérique ; repos/refus de g négatif et couvercle partiel.
Suite release : **413 réussis, 5 ignorés**. Les deux tests S233 ont ensuite été renforcés
sur les refus et rejoués seuls, sans changement de code d'exécution. Filtre S232 inchangé :
`0xc5ab1eadb094d058`.

Trois tableaux de nx f32 supplémentaires (sauvegarde η, compensation, sa sauvegarde) :
**12 nx octets**. Comptabilité exacte reçue par le test runtime de stockage.

Banc `delta_surface` : CPU séquentiel Windows x86_64, domaine8×4m, fond0, onde cos d'amplitude
1cm, g9,81, durée2000µs, plafond2000itérations, budget1s. Trois échauffements et onze
mesures après réinitialisation de surface/vitesse ; sauvegarde, projection, flux, hauteur,
contrôles et retour inclus. Pas de rendu, B/W, configuration ou impressions dans la fenêtre.

| nx | octets | itérations | médiane / max passage1 (ms) | médiane / max passage2 (ms) |
|---|---:|---:|---:|---:|
| 16 | 8 384 | 14 | 0,0882 / 0,1008 | 0,1302 / 0,8760 |
| 32 | 32 128 | 27 | 0,3936 / 0,4016 | 0,5623 / 0,6807 |
| 64 | 125 696 | 53 | 2,8638 / 3,8652 | 6,2104 / 8,9980 |

Le premier passage chevauchait la fin des tests ; le second les suit. La variation est
conservée sans attribution causale gratuite. Pas de borne murale, de gain reçu ni de
transfert de ces coûts à une trajectoire entière. Le cas64 dépasse déjà2ms au premier pas.
Un budget trop court annule ce pas ; le recommencer sans adapter la charge ne le fait pas avancer.

## Reproduction

Depuis la racine :

```text
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline linear_surface_matches -- --nocapture
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline --test delta_runtime evolving_surface -- --nocapture
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta_surface
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta_filters
cargo test --manifest-path code/Cargo.toml --workspace --release --offline --quiet
```
