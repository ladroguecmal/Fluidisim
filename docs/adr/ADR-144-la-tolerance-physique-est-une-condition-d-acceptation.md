# ADR-144 — La tolérance physique de la projection est une condition d'acceptation, sur les lignes franches

- Statut : **actée**, S239, 2026-09-15 ; autonomie technique S71.
- Traite **A273**, ouverte par [PRESSION-PLANCHER-S238](../validation/PRESSION-PLANCHER-S238.md) §4.4.
- Précise [ADR-143](ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) §3 et
  requalifie, avec provenance, la portée du critère 4 de
  [CANDIDAT-DELTA-S199](../validation/CANDIDAT-DELTA-S199.md) §5.
- Mesures et réception : [TOLERANCE-PRESSION-S239](../validation/TOLERANCE-PRESSION-S239.md).

## Constat

ADR-143 exige la tolérance physique `D = max|div u|·dx/max|u| ≤ 10⁻⁵` **au plancher d'arrondi**, et
nulle part ailleurs. Le chemin premier — résidu relatif `ρ = ‖b−Ap‖₂/‖b‖₂ ≤ 10⁻⁶` — ne l'exigeait pas,
et S238 a mesuré des pas **convergés** à `D` = 1,02·10⁻⁵ et 1,58·10⁻⁵.

La mesure de S239 établit deux faits distincts, et ils ne se corrigent pas de la même façon.

1. **Les deux critères ne sont pas de la même norme, et rien ne les relie à taille variable.** De
   l'identité `div u = r/scale`, `D = ρ·θ·Λ` exactement, avec `θ = max|r|/‖r‖₂` la concentration du
   résidu et `Λ = ‖b‖₂·dx/(|scale|·max|u|)` la forme du second membre. Le critère premier ne borne que
   `ρ`. Mesuré de 128 à 32 768 mailles : **`Λ` double à chaque raffinement** — le second membre vit à
   l'échelle de la maille —, `θ` décroît plus lentement que `N^(−1/2)`, et leur produit croît d'environ
   30 % par raffinement. **La taille seule fait franchir la tolérance.**
2. **`D` n'a pas le même sens sur toutes les lignes.** En mode mobile, `max|div u|` est porté par les
   mailles de **surface**, dont la face haute porte un fluide fantôme : condition de Dirichlet de
   raideur `1/θ_surface`, diagonale environ 500 fois celle d'une ligne franche. Au premier pas du cas
   de topologie de S237, leur résidu vaut **exactement 2⁻⁵, 2⁻⁶ et 2⁻⁷ sur des lignes de magnitude
   ≈ 2¹⁸** : un ulp de leur propre ligne. `D` y plafonne à 1,5·10⁻⁵ pendant que `ρ` descend de
   8,1·10⁻⁷ à 8,5·10⁻⁹ ; les lignes **franches** du même pas passent de 7,5·10⁻⁵ à 4,5·10⁻⁷.

Deux corroborations écrites avant S239, et jamais rapprochées : S199 a inscrit dans son propre tableau
de réception que le `< 10⁻⁵` valait pour une « configuration unitaire testée », et **« pas borne
universelle »** ; S237 a desserré une de ses propres assertions de projection mobile à 10⁻⁴ sans dire
pourquoi. A273 partait donc d'une prémisse à moitié vraie.

## Décision

1. **La tolérance physique de S199 devient nécessaire dans tout chemin d'acceptation**, et non plus
   seulement au plancher d'ADR-143 :
   `accepté ⟺ b = 0`, ou `(ρ ≤ 10⁻⁶ ou plancher certifié)` **et** `D_franches ≤ 10⁻⁵`.
2. **La quantité qui décide est `D` restreinte aux lignes franches** — les mailles mouillées dont
   aucune face ouverte ne donne sur un fantôme de surface. Une ligne à fantôme n'énonce pas une
   conservation mais une condition de bord de raideur `1/θ` ; son résidu a un plancher f32 propre,
   mesuré, que nulle itération n'abaisse. Sans fantôme — mode à couvercle fixe —, `D_franches = D`.
3. **Tant que la tolérance n'est pas tenue alors que le critère premier l'est, la boucle poursuit.**
   La cible resserrée est `‖r‖₂² ← ‖r‖₂²·(10⁻⁵/D_franches)²` : `D` est proportionnelle à `max|r|` par
   l'identité, donc la réduire du rapport voulu demande, à concentration égale, de réduire `‖r‖₂` du
   même rapport. **Aucun facteur n'est choisi** — l'acceptation reste la valeur *exacte* de
   `D_franches` à la relance suivante, et la cible n'est qu'une allure.
4. **Un pas qui ne peut pas tenir la tolérance est dégradé** : plafond d'itérations (ADR-007 §2) ou
   plancher d'arrondi (ADR-143). Il le déclare, au lieu d'être annoncé reçu au-dessus d'elle.
5. `Report` publie `divergence_plain` à côté de `divergence`. `divergence` garde exactement son sens
   et sa valeur ; c'est `divergence_plain` qui décide.
6. Aucun nombre nouveau : `10⁻⁶` vient de S231, `10⁻⁵` de S199, `γ₈` d'ADR-143.

## Conséquences

- **Ce que cela débloque.** Jusqu'à 8 192 mailles, la tolérance déclarée est tenue pour **+6,6 %
  d'itérations** (128×64 sur fond en bosse : 370 au lieu de 347). Un consommateur peut désormais lire
  `degraded = false` comme « la projection tient la tolérance de S199 », ce qui était faux jusqu'ici.
- **Ce que cela refuse.** À 32 768 mailles, le certificat d'arrondi d'ADR-143 arrête le solveur à
  `D` = 1,09·10⁻⁵ : f32 manque la tolérance de 9 %, et le pas devient **dégradé**. Cette taille
  n'a jamais été reçue (S231 : 8 192 ; S238 : 16 384 en mode mobile). **Franchir 32 768 mailles en 2D
  demande un préconditionneur plus fort ou un résidu recalculé en précision double, pas un seuil
  relevé** — c'est le point dur du passage à la 3D, qui y arrivera d'emblée.
- **Ce que cela change au bit.** Tout pas dont `D_franches` était déjà sous la tolérance s'arrête
  exactement où il s'arrêtait. Les autres poursuivent : leurs bits changent, et c'est l'objet même
  de la décision. Les changements sont publiés dans le document de mesure.
- **Coût.** Une correction et une divergence de plus par relance qui atteint le critère premier —
  quelques passes O(N) pour quelques centaines d'itérations —, plus les itérations supplémentaires.
  Techniques présentes, absentes et domaine : TOLERANCE-PRESSION-S239 §4.
- **Ce que cela ne fait pas.** Rien pour les lignes à fantôme, dont le plancher reste à mesurer sur
  d'autres géométries, et rien en 3D (opérateur à six faces, `γ₁₀` à dériver). La tolérance de S199
  n'est toujours **pas** une borne universelle : elle est désormais tenue, ou refusée, explicitement.
