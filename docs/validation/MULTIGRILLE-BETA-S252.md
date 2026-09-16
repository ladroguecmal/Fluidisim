# Multigrille : le β du gradient conjugué, et ce qu'il cachait — S252

Traite **A284** (coût du démarrage plat) et **A285** (défaut trouvé en l'attribuant).
Décision : [ADR-151](../adr/ADR-151-affinage-au-pas-fixe-et-travail-compte.md).
2026-09-16, Windows x86-64, release, un fil, sur secteur avant et après (état 2, 99 %).

## 1. Attribution d'A284

Test ignoré `flat_start_cost_attribution_s252`. Trace de test `COUPLED_TRACE`, compilée en test
seulement, autour de chaque projection du pas couplé. Trajectoire du banc
`delta_coupling --flat --fine` : 32×16, 20 pas de 1 ms, B/W réels.

| pas | ordinaire (it / ms) | repli multigrille (it / ms) | affinage (it / ms) | total |
|---|---|---|---|---|
| 0–6 | 80–81 / 0,84–1,36, refusé | **500–501 / 38,6–41,1**, refusé | 86–92 / 0,9–1,3 | 40,5–43,2 ms |
| 7–16 | 81–82 / 0,84–1,04, refusé | **500–501 / 37,7–42,9**, reçu | — | 38,7–44,0 ms |
| 17–19 | 82 / 0,85–1,28, reçu | — | — | 1,0–1,4 ms |

Le repli fait **94 %** du pas. Chaque vrai résidu recalculé est enregistré par `PRESSURE_TRACE` :
le repli relance **25 fois**. Son premier vrai résidu relatif vaut **0,616** à 18 itérations,
puis il ne baisse que d'un facteur ~0,6 par relance d'environ vingt itérations
(0,428 ; 0,246 ; … ; 3,6·10⁻⁶ à 500). Un gradient conjugué dont la récurrence est exacte ne
peut pas s'arrêter là. Le chemin ordinaire du même pas atteint 1,04·10⁻⁵ en 80 itérations
(1,07·10⁻⁵ dans l'essai du §2, dont les réductions ne sont pas découpées par le budget).

## 2. Le défaut A285

`project`, branche `multigrid_on`, depuis S245 P5 (`6dc0bfa`) :
`rz = multigrid_into_dir(rn / rz)`, où `rn = ‖r_{n+1}‖²` et `rz = ⟨r_n, z_n⟩`. La direction
recevait donc `β = ‖r_{n+1}‖²/⟨r_n, z_n⟩`, formé **avant** le cycle, au lieu de
`⟨r_{n+1}, z_{n+1}⟩/⟨r_n, z_n⟩`. Le commentaire du code décrivait la bonne formule. Le chemin
Jacobi du mode mobile la calcule correctement.

Mécanisme probable, non instrumenté : `⟨r, M⁻¹r⟩` est de l'ordre de `‖r‖²·dx²/4`. Le β fautif
vaut alors environ `4/dx²`, soit 64 à `dx = 0,25 m`. La direction croît d'autant à chaque
itération, jusqu'à ce que `⟨d, Ad⟩` ne soit plus fini et interrompe la boucle. Cela concorde
avec les ~20 itérations par relance ; seules les relances depuis le vrai résidu faisaient
progresser le calcul.

**Reproduction avant correction.** `multigrid_conjugate_gradient_keeps_its_recursion_s252`
résout le même système avec les deux préconditionneurs. Assertion structurelle, sans seuil
choisi : le premier vrai résidu multigrille ne dépasse pas le plancher atteint par le chemin
ordinaire. Avant : 0,616 contre 1,0735·10⁻⁵, 514 itérations, **échec**. Après : 4,03·10⁻⁶,
9 itérations, une relance, **réussi**.

**Correction.** `multigrid_into_dir` reçoit `⟨r_n, z_n⟩` et forme `β` après le cycle. Une valeur
nulle signale un départ ou une relance, avec `dir = z`.

## 3. Avant et après, même machine, mêmes commandes

Le banc S245 (`what_the_multigrid_buys_s245`, pas `1/60 s` depuis le repos, couvercle ondulé)
a été exécuté juste avant la correction (copie du fichier d'origine), puis sur le code final.

| mailles | niveaux | sans (ordinaire, puis repli) | avec, avant correction | avec, après |
|---|---|---|---|---|
| 128 | 1 | 30 it / 0,087 ms | 94 / 2,12 ms | **6 / 0,142 ms** |
| 512 | 2 | 61 / 0,677 | 177 / 13,8 | **7 / 0,602** |
| 2 048 | 3 | 114 / 5,04 | 158 / 50,1 | **7 / 2,44** |
| 8 192 | 4 | 220 / 37,2 | 120 / 159 | **8 / 15,0** |
| 32 768 | 5 | avant : 106 / 880, reçu → après : **441 / 392, reçu** | 106 / 595, reçu | **24 / 137, reçu** |

La colonne « sans » à 32 768 mailles comprend l'ordinaire refusé, le repli et, après ADR-151,
l'affinage. Les itérations sont désormais cumulées ; avant S252, seule la dernière projection
était rendue. Le premier passage est inclus dans la médiane de sept.

**32 768 mailles, sans ADR-151.** Le repli corrigé s'arrête au plancher à 8 itérations,
`D = 1,585·10⁻⁵`, refusé : le pas à 32 768 mailles serait redevenu refusé. Avec un affinage
(`largest_grid_is_received_by_refinement_s252`, test en release) : q en 8 itérations,
`D = 3,56·10⁻⁸`, pas reçu en 385 ms. Le témoin de ce test (multigrille seule) reste refusé au
plancher.

**Non-régression.** `delta_precision` : les dix cas ont itérations, résidus, divergences et
champs u/w/p **identiques**, temps exceptés. Empreinte `delta_filters` **0xfb12b2092df4ee6d**,
ordres 1,947/1,957/1,959. Suite release : **451 réussis, 17 ignorés, 0 échec**. Deux essais
ajustés au compte cumulé : `a_tight_budget_degrades_and_says_so` (2 = ordinaire + repli) et
`global_allocator_sees_counterexample_but_no_step_allocations` (≤ trois fois le plafond).

**Bancs couplés S251** (20 pas, médiane / maximum) :

| cas | avant | après | affinages |
|---|---|---|---|
| 32×16 plat | 43,6 / 48,2 ms | **2,53 / 3,51 ms** | 7 |
| 16×8 plat | 0,300 / 5,88 | 0,279 / 0,637 | 7 → 5 |
| 32×16 imposé | 1,12 / 2,10 | 1,15 / 1,71 | 0, D max identique au bit |
| 16×8 imposé | 0,171 / 0,206 | 0,177 / 0,346 | 0, D max identique au bit |

## 4. Coût (ADR-131)

- **Présentes** : gradient conjugué nu, multigrille en V corrigée en repli, affinage de
  divergence unique au plancher, f32, un fil.
- **Absentes** : multigrille comme solveur ordinaire au-delà d'une taille (mesurée ici, pas
  branchée), départ non nul, opérateur grossier de Galerkin, parallélisme, GPU, mode mobile
  multigrille.
- **Domaine** : 8×4 m, fond plat ou couvercle ondulé, 128 à 32 768 mailles ; démarrage couplé
  32×16 et 16×8. Une machine.

## 5. Ce qui change, et ce qui reste

1. **S245 et S246 ont mesuré un gradient conjugué fautif.** Leurs comptes d'itérations et
   coûts multigrille sont invalides : 99/252/220/167/134, puis 94/177/158/120/106, et
   1 006 ou 828 ms à 32 768 mailles. La conclusion « la multigrille ne gagne pas de vitesse »
   aussi. Les **taux par cycle** de S246 (0,63–0,67) sont hors de cause : ils emploient le
   cycle comme itération stationnaire, sans gradient conjugué. A281 garde donc son plafond,
   mais il pèse peu à 6–8 itérations.
2. **L'explication d'A275 était fausse.** La tolérance tenait grâce aux relances. Elle tient
   désormais par ADR-151.
3. **L'ordre d'ADR-147 est à reprendre.** Forcée, la multigrille est plus rapide dès 512
   mailles (0,60 contre 0,68 ms), deux fois plus vite à 2 048, 2,5 fois à 8 192 et 2,9 fois à
   32 768. C'est un lot de décision : bits des tailles reçues, seuil de taille justifié,
   réception. Pas ici.
4. A284 est attribuée et levée. Le démarrage plat coûte encore 2,2 fois le cas imposé
   (2,53 contre 1,15 ms) : affinage et repli.

## Reproduction

```powershell
cargo test --release --offline --manifest-path code/Cargo.toml
cargo test --release --offline --manifest-path code/Cargo.toml -p water-core --lib -- --ignored flat_start_cost_attribution_s252 --nocapture
cargo test --release --offline --manifest-path code/Cargo.toml -p water-core --lib -- --ignored what_the_multigrid_buys_s245 --nocapture
cargo test --release --offline --manifest-path code/Cargo.toml -p water-core --lib largest_grid_is_received_by_refinement_s252 -- --nocapture
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_coupling -- --flat --fine
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_precision
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_filters
```
