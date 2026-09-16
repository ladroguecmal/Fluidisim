# ADR-151 — Affiner la divergence au plancher dans le pas à couvercle fixe, et compter tout le travail

Actée S252, 2026-09-16, autonomie S71. Suit la correction du défaut A285. Étend ADR-150,
conserve les portes d'acceptation d'ADR-143/144 et, pour ce lot, l'ordre d'ADR-147.

## Constat

Le gradient conjugué préconditionné par la multigrille (S245) formait
`β = ‖r_{n+1}‖²/⟨r_n, z_n⟩`, avant le cycle, au lieu de `⟨r_{n+1}, z_{n+1}⟩/⟨r_n, z_n⟩`.
Corrigé en S252, il converge en **6 à 8 itérations** aux cinq tailles du banc S245. Mais à
32 768 mailles il s'arrête au plancher d'ADR-143 avec `D = 1,585·10⁻⁵`, au-dessus de la
tolérance de S199 : **le pas redevient refusé**. L'acceptation obtenue depuis S245 tenait aux
relances du gradient fautif, qui repartaient chaque fois du vrai résidu. Elle ne venait pas de
« moins d'itérations, donc moins d'arrondi », comme ADR-147 l'expliquait.

## Décision

1. Le pas à couvercle fixe (`run`, donc `step`, `step_budgeted` et `step_measured`) applique
   **l'affinage d'ADR-150** quand il reste refusé **au plancher** après la projection ordinaire
   et son éventuel repli. Une seule fois. `q` est résolu avec le préconditionneur de la dernière
   projection : multigrille si le repli a eu lieu. Couvercle homogène, pression publiée `p+q`,
   restauration à l'expiration. Le pas couplé appelle la même fonction, avec `q` sans
   multigrille comme en S251, et garde ses bits.
2. Aucun affinage sur un refus au plafond d'itérations sans plancher. Aucun non plus en mode
   mobile : ADR-150 ne traite pas les lignes à fantôme.
3. `Report.iterations` compte **toutes** les projections du pas (ordinaire, repli, affinage),
   comme le pas couplé depuis ADR-150. `max_iters` vaut par projection : un pas en fait au plus
   trois fois le plafond. Jusqu'à S251, `run` ne rendait que la dernière projection, et le repli
   doublait le travail sans le dire.
4. Aucune porte d'acceptation ne bouge. L'ordre « ordinaire, puis repli » d'ADR-147 reste en
   place ici. Sa prémisse de vitesse (« la multigrille perd ») est réfutée par la re-mesure :
   ce choix se reprend dans un lot dédié, par un nouvel ADR s'il change.

## Conséquences

- 32 768 mailles reçues : 441 itérations, 392 ms, `D = 3,56·10⁻⁸`, contre 880 ms avec le
  gradient fautif. Forcée dès le départ, la multigrille reçoit le même pas en 24 itérations et
  137 ms.
- Les pas reçus au premier essai restent identiques au bit : `delta_precision` (dix cas) et
  empreinte `delta_filters` `0xfb12b2092df4ee6d`.
- Deux essais anciens affirmaient `iterations ≤ plafond` pour le pas entier. Ils sont ajustés
  au compte réel, avec leur motif.
- Le démarrage plat couplé 32×16 passe de 43,6 à 2,5 ms de médiane (A284).

[Preuve et mesures](../validation/MULTIGRILLE-BETA-S252.md). Pas de réception du mode mobile,
ni de la 3D, ni du budget I-05.
