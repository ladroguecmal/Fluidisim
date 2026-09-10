# ADR-107 — Le domaine d'un sillage se déduit de sa recette, il ne se déclare pas

Statut : acté, S156, 2026-09-10, délégation technique. Complète ADR-097, ADR-105 et ADR-106 ;
aucune décision antérieure n'est modifiée, aucun ADR réécrit.

## Constat

ADR-106 vient de porter l'horizon d'observation à 64 s pour que B2 puisse mesurer un sillage à la
durée où il mesure ses impacts. La mesure de S156
([SILLAGE-DOMAINE-S156](../validation/SILLAGE-DOMAINE-S156.md)) montre que **cela ne suffit pas,
et que ce qui manque n'est pas du temps mais de la résolution spectrale**.

L'énergie spectrale, elle, se conserve parfaitement : puissance exactement nulle dès l'extinction,
énergie identique au bit près à 16, 20, 30, 45 et 60 s. **C'est un résultat vide.** Après
extinction chaque mode tourne, et la rotation laisse `g|eta|² + |v|²/k` invariant : le bilan ne
pouvait pas ne pas se conserver. Il confirme l'implémentation et ne dit rien de la validité
spatiale du champ.

La validité spatiale, elle, est bornée par **deux mécanismes indépendants**, qui n'ont ni la même
loi ni le même remède.

| borne | mécanisme | loi mesurée |
|---|---|---|
| rayon | le pas angulaire ne résout plus `exp(i k·x)` sur le cercle | rayon honnête **proportionnel à `angular`** : 20 / 45 / >200 m pour 64 / 128 / 256, à 8 s |
| durée | le pas radial rend le champ **périodique**, de période `2π·radial/cutoff` | radial 128 décroche **entre 15 et 20 s**, radial 256 **entre 45 et 50 s** |

Le paquet ne part pas : **il revient par l'autre bord**. À 4 s, deux résolutions voisines cessent
de s'accorder à 45 m (64 contre 128) et 100 m (128 contre 256) — les deux tiers de la période
spatiale de la plus grossière, 67 m et 134 m.

**Laquelle des deux mord dépend de l'instant**, et c'est ce qui n'était pas anticipé. À 8 s,
512 radial / 128 angulaire n'est honnête qu'à 45 m tandis que 128 radial / 512 angulaire l'est
au-delà de 200 m : c'est l'angulaire qui borne. À 60 s, exactement l'inverse — 512/128 reproduit
512×512 au bit près en champ proche, et 128/512 se trompe d'un facteur 75. Une seule des deux
résolutions suffit à ruiner le champ, mais pas la même selon la date.

## Décision

**1. Le domaine de validité d'un sillage est un couple `(rayon, durée)` déduit de sa recette, et
non une constante globale.** Aucune valeur unique ne peut être écrite dans le code ou dans un
document : elle dépend de `radial`, de `angular` et de `cutoff`, et les deux bornes suivent des
lois différentes.

**2. Les domaines mesurés sont attachés aux recettes en usage, et publiés avec elles.**

| recette | rayon honnête | durée honnête | usage |
|---|---|---|---|
| 128×128 | ~45 m | 15–20 s | réception de sillage S150, S151 |
| 256×256 | ~90 m et au-delà | 45–50 s | — |
| 512×256 | ~90 m et au-delà | > 50 s, non mesurable | ce que 60 s exigerait |

**3. Ne pas relever le plafond de 512 de la grammaire de recette** (`validate_recipe`,
ADR-097). Deux raisons, chacune suffisante. La première est qu'**il n'existe aucune référence
plus fine** : `radial` et `angular` plafonnent tous deux à 512, donc la durée honnête de 512 n'est
pas mesurable, seulement extrapolable. La seconde est le prix, mesuré ci-dessous.

**4. Le volet sillage de B2 reçoit un verdict partiel et négatif à 60 s**, avec ses nombres :
la recette qu'il faudrait coûte 205 ms de préparation et 150 ms par lot de 64 points. Ce n'est
pas un report faute de mesure ; c'est un refus fondé sur une mesure.

## Prix mesuré

Médianes sur cent répétitions, après un bloc de chauffe séparé (A195), en release.

| recette | nœuds du demi-spectre | préparation p50 | lot 64 points p50 |
|---|---:|---:|---:|
| 128×128 | 8 192 | 25,6 ms | 19,5 ms |
| 256×256 | 32 768 | 102,0 ms | 75,0 ms |
| 512×256 | 65 536 | 205,5 ms | 150,4 ms |
| 512×512 | 131 072 | 402,9 ms | 298,3 ms |

Le coût est **linéaire en nombre de nœuds**, sans surprise et sans effet de cache visible à ces
tailles. Porter la recette de 128×128 à 512×256 multiplie donc le prix par huit — et le point de
départ est déjà de 19,5 ms pour soixante-quatre points.

Ces chiffres sont ceux d'une machine de développement et d'un chemin scalaire ; ils ne sont pas un
budget matériel cible. Ils suffisent néanmoins à la décision, parce que l'écart au raisonnable
n'est pas d'un facteur deux.

## Ce que cette décision ne dit pas

- **Elle n'encode aucun garde-fou.** Refuser à l'exécution un échantillonnage hors domaine serait
  conforme à l'habitude du dépôt — annoncer l'admissibilité plutôt que mentir en silence
  (ADR-091). Mais la loi en durée n'est **encadrée qu'en deux points**, et un garde bâti sur une
  loi non vérifiée refuserait des configurations valides ou en admettrait d'invalides. Enregistré
  en **A214** ; c'est une mesure qui manque, pas une décision.
- **Elle ne condamne pas le sillage spectral**, seulement son emploi à 60 s à résolution
  suffisante. Aux durées où il a été reçu — 8 s en S150, 8 s en S151 — il est dans son domaine, et
  les réceptions restent valides.
- **Elle ne propose pas de remède.** Une quadrature non uniforme en k, plus dense près de zéro où
  vivent les modes rapides, déplacerait la borne sans multiplier les nœuds ; une fenêtre spatiale
  amortissant le retour par l'autre bord aussi. Ni l'une ni l'autre n'est mesurée, et les nommer
  n'est pas les recevoir.
- **Elle ne touche aucun invariant** ni aucun code : rien n'est modifié en production.
