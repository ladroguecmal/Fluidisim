# Levier temporel du sillage — S213, 2026-09-13

Premier élément construit de l'espace d'optimisation du rendu
([ADR-131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md), FEUILLE-DE-ROUTE J1-bis).
Il déplace **la loi CPU** mesurée en S212 ; il ne touche pas la loi GPU. Aucun verdict de budget
n'est tiré d'un levier seul : 2 ms s'éprouve sur la combinaison (ADR-131 D4).

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : phases repliées de B au GPU (S211) ; table de Bessel de l'impact à
  λ/16 (ADR-129) ; **repli temporel du sillage** — tronçons achevés repliés par nœud, rotation
  entière par image, modes préconstruits, tronçon en cours par `ModalPressure::sample` (S213) ;
  publication `[A, B, kx, ky]` rebasée ; somme modale par sommet sur GPU dans l'emprise ; grille
  projetée à 2 px.
- **Techniques absentes** : grille locale et transformée (espace) ; LOD spatial ; LOD spectral ; LOD
  temporel ; visibilité ; mutualisation ; parallélisme CPU (un fil) ; SIMD explicite.
- **Domaine de validité** : fixture S212 inchangée — une source prescrite, 8 tronçons de 2 s à
  3 m/s sous 19 620 N, σ 2 m, contexte 40 s ; recettes 64×128 (4 096 nœuds) et 128×256 (16 384) ;
  caméra S201 ; AMD Ryzen AI 7 350, RTX 5070 Laptop, DX12, Windows, release ; réception numérique
  sur deux recettes réduites (8×16, 32×64) et quatre tronçons à virages, arrêt et charge nulle.
  Ne dit rien d'un autre nombre de sources, d'une émission progressive en jeu (ADR-104), d'une
  autre machine ni d'une autre combinaison.

## Construction

`water_core::pressure_timeline::Timeline`, construit une fois depuis le journal de pression (mêmes
contrôles que `Prepared::from_journal`, journal emprunté), avec deux pools d'hôte : un
`NodeState` par nœud (56 octets) et un `Option<ModalPressure>` par nœud et par tronçon (88 octets).

Dans la solution de Duhamel (ADR-069), l'état `(η, v/ω)` d'un tronçon achevé subit la rotation
`R(ω·Δt)`. À chaque image, pour l'instant `t` :

1. l'ensemble `{ j : fin_j ≤ t }` est replié par nœud à l'instant de référence (début du contexte),
   chaque état de fin ramené par `R(−ω(fin_j − réf))` ; repli **incrémental** quand les nouveaux
   tronçons suivent les anciens dans l'ordre canonique, **complet** sinon (retour arrière) ;
2. `η(t) = R(ω(t − réf))·état replié + Σ tronçons en cours ModalPressure::sample(t)` ;
3. `A + iB = poids · η(t) · exp(i k·origine)`, phases entières PhaseQ32 (I-08).

La résonance `Ω = ±ω`, où vit le sillage de Kelvin, garde la forme sinc du cœur : le tronçon actif
n'est pas décomposé en phaseurs, qui y seraient mal conditionnés. Chemin **cosmétique** (ADR-129
§3) : grandeurs de jeu et références restent servies par `bound_pressure::Prepared`.

Primitives exposées au crate, sans changement de comportement : `modal_pressure::{frequency,
phase}`, `Complex::{phase, scale, add, mul}`, `ModalPressure::{forcing_end, birth}` et `Copy`.

## Réception

Critères déclarés avant mesure (EN-COURS S213). Quatre tests, debug :

| test | ce qu'il reçoit | résultat |
|---|---|---|
| `timeline_matches_prepared_across_instants_and_jumps_s213` | η et pentes reconstruits contre `from_journal` + `render_components`, 19 instants : naissance, bornes de tronçon à ±1 µs, fin de forçage, fin de contexte, **retour arrière**, répétition, reprise ; deux origines ; recettes 8×16 et 32×64 | écart relatif max **6,07e-8** et **2,23e-8** contre 1e-5 |
| `incremental_fold_equals_full_fold_bitwise_s213` | pas de 250 ms jusqu'à 19 s contre saut direct | **identiques au bit** |
| `refusals_leave_output_and_state_usable_s213` | journal vide, capacités, contexte, instant, origine ; sortie inchangée, vue réutilisable | reçu |
| `wake_sources_cannot_start_before_the_reference_s213` | la garde « naissance avant le contexte » repose sur le refus amont de `Source::new` | reçu |

**Témoin vérifié** : rotation de repli inversée → échec à la première borne (2 s), écart 0,365.

Dans l'hôte, `--verify` compare le GPU alimenté par le repli à la référence **préparée** du cœur :
lignes identiques à S212, erreur max **0,089370 mm** (tolérance 3 mm), sauts arrière compris. Les
contrôles de validité S212 sont conservés et inchangés (témoin 128×256, couture, admission).
`--smoke` : fenêtre ouverte, 120 images, code 0. Suite complète `code/` hors réseau : 354 réussis
(256+4+1+93), 5 ignorés, aucun échec.

## Coûts

### Levier seul, CPU un fil (`cargo run -p water-core --release --example wake_timeline_cost`)

Médiane / p95 / max en ms, 60 images/s simulées, chauffe séparée.

| recette | scénario | repli temporel | préparation par image (S212) |
|---|---|---:|---:|
| 64×128 | fenêtre S212, 3,17–5,15 s, forçage | 1,260 / 1,686 / 2,092 | 7,696 / 9,477 / 14,659 |
| 64×128 | après forçage, 24,17–26,15 s | 0,360 / 0,448 / 0,830 | 13,357 / 15,091 / 17,834 |
| 64×128 | balayage 0–40 s, 2 392 images ordinaires | 0,463 / 1,834 / 7,299 | — |
| 64×128 | balayage, 8 images franchissant une fin de tronçon | 2,119 / 2,162 / 2,162 | — |
| 64×128 | saut 39 → 3 s (repli refait) · saut 0,5 → 39 s | 3,101 · 12,123 | — |
| 128×256 | forçage | 4,994 / 5,854 / 10,217 | 30,895 / 37,300 / 57,699 |
| 128×256 | après forçage | 1,386 / 1,624 / 2,274 | 52,894 / 57,289 / 74,666 |
| 128×256 | balayage ordinaires · fin franchie | 1,411 / 5,480 / 9,243 · 6,031 / 6,526 | — |
| 128×256 | sauts 39 → 3 s · 0,5 → 39 s | 9,462 · 37,920 | — |

Construction : 4,433 ms (64×128), 18,651 ms (128×256). Mémoire d'hôte : 3 112 960 et 12 451 840
octets. Le maximum de 7,3 ms parmi les images ordinaires 64×128 n'est pas attribué (bruit
d'ordonnancement probable, non vérifié).

**Lois déplacées.** Par image : `nœuds × (1 + tronçons en cours)` au lieu de `nœuds × tronçons
publiés` ; aux bornes, un pic `nœuds × tronçons nouvellement achevés` ; un retour arrière paie le
repli complet. La préparation S212 **renchérit avec l'âge** (tous les tronçons nés s'évaluent) ;
le repli, lui, devient moins cher après le forçage : gain ×6 pendant, ×37 après, à 4 096 nœuds.

**Prédiction contredite**, écrite avant mesure : « quelques dixièmes de ms pendant le forçage,
dizaines de µs après ». Mesuré : 1,26 et 0,36 ms. Après forçage, ≈ 88 ns par nœud pour une phase
temporelle (produit i128 divisé par 10⁶), une rotation, une phase d'origine et l'écriture — parts
non séparées, donc aucune optimisation fine proposée ici.

### Dans l'hôte (`--verify`, 120 images, âges 3,17–5,15 s)

| nœuds | format | CPU sillage médiane / max | CPU total médiane | GPU eau médiane / p95 / max |
|---:|---|---:|---:|---:|
| 4 096 | 640×360 | 1,674 / 2,973 | 2,286 | 1,907 / 1,922 / 1,931 |
| 4 096 | 960×540 | 1,777 / 2,893 | 2,491 | 4,185 / 4,217 / 4,242 |
| 16 384 | 640×360 | 6,903 / 11,514 | 8,102 | 8,181 / 8,839 / 9,011 |
| 16 384 | 960×540 | 6,820 / 9,313 | 8,150 | 17,412 / 19,159 / 20,847 |

S212, même domaine, préparation par image : CPU sillage 10,851 / 10,577 ms (4 096), 38,4 / 41,5 ms
(16 384) ; GPU inchangé à la mesure près. L'hôte mesure le repli plus lent que l'exemple (1,67–1,78
contre 1,26 ms) : cause non attribuée.

## Ce que ce résultat ne dit pas

- **Pas un verdict de budget.** À 4 096 nœuds et 960×540, la chaîne construite coûte ~1,8 ms de CPU
  pour le sillage et ~4,2 ms de passe GPU — **avec** les techniques absentes listées plus haut.
  C'est une coordonnée de l'espace d'optimisation, pas sa frontière.
- **La loi GPU n'a pas bougé** : elle reste `sommets × nœuds`, visée par l'espace, les LOD, la
  visibilité et la mutualisation.
- **Aucune limite de validité n'est levée.** La recette 64×128 reste honnête ~16 s et la couture
  atteint 12,7 mm à 39 s (**A251**) ; la composition impact + sillage par le cœur n'est toujours
  pas exercée. Accélérer ne les masque pas (ADR-131 D6) : les contrôles de `--verify` restent.
- Un seul sillage scripté ; l'émission progressive d'ADR-104 ajoute des sources au journal et
  obligerait à reconstruire la vue — coût de reconstruction à mesurer quand un hôte émet en jeu.

## Suite

*Note datée du 2026-09-13 (S214) : les deux travaux nommés ci-dessous sont faits —
[COMPOSITION-J1-S214](COMPOSITION-J1-S214.md). La composition est exacte au bit une fois le point
commun rétabli (A253) ; A251 est traitée par [ADR-132](../adr/ADR-132-domaine-d-image-d-un-sillage.md).
La mesure a ouvert **A254** (sévérité 1), qui passe avant la loi GPU : le budget de pente est une
somme sur les sources, et deux en consomment 84 %.*

Travaux nécessaires de J1 avant d'accélérer davantage, pour que l'accélération ne masque rien :
**composition impact + sillage** par `mixed_water` sur la scène (budget conjoint ADR-119), et
**A251** — emprise et durée d'image du sillage déduites de sa recette et reçues par coutures.
Puis la loi GPU : espace (grille et transformée), LOD, visibilité, mutualisation, chacun mesuré
avec son en-tête.
