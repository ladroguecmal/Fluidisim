
# ADR-098 — Trois causes, trois noms dans le budget de pente

- **Statut : actée**, S144, 2026-09-10, autonomie technique S71.
- **Traite** [A208](../registres/ANGLES-MORTS.md), ouverte en S140 et reportée trois fois.
- **Applique** ADR-082 (une borne, un nom, et le nom désigne ce qu'il faut revoir) à un endroit
  qu'il n'avait pas couvert.
- **Prolonge :** ADR-080 (budget de pente), ADR-094, ADR-095.
- **Mesure :** [REFUS-EMPRISE-S144](../validation/REFUS-EMPRISE-S144.md).

## Problème

A208 : à champ identique, l'emprise publiée décide de la part de budget consommée — ×10,9 mesuré,
sans borne — et le refus rendu désigne la **pente**, c'est-à-dire la seule chose que l'hôte n'a pas
à changer.

Elle proposait deux réparations. **Aucune n'est prise.** `Footprint` attribuerait la cause à
l'emprise, alors que le facteur d'alignement dépend aussi du spectre publié : la bibliothèque ne
peut pas trancher, et ADR-082 refuse un nom qui ment autant qu'un nom vague. Publier le rapport
des deux enveloppes ne dirait que le facteur de **forme**, que S141 a déjà retiré — la réparation
a été écrite en S140 et la session suivante l'a rendue sans objet.

## Ce que l'instruction a établi

**Les trois budgets refusent après avoir échantillonné le point demandé** : `composition`,
`mixed_water` et `bound_pressure` ont tous la pente réelle au point sous la main au moment du
test. Elle ne dit pas le maximum sur l'emprise — il ne se calcule pas (S140) — mais elle en est
une borne inférieure, et cela suffit à séparer deux situations qui n'ont pas le même remède.

**Et `Slope` recouvrait deux causes sans rapport** : un `max_slope` non fini ou négatif — faute
d'entrée de l'hôte — et un dépassement du budget. Le fourre-tout qu'ADR-082 a démonté ailleurs
était resté ici, parce que rien ne l'avait rouvert.

## Décision

**Trois causes, trois noms**, dans les trois budgets :

| cause | nom | ce qu'il faut revoir |
|---|---|---|
| `max_slope` non fini ou négatif | **`MaxSlope`** | le paramètre fourni |
| la pente réelle **au point** dépasse | **`Slope`** *(sens resserré)* | le champ, vraiment trop raide ici |
| la pente au point tient, seule la somme des majorants dépasse | **`SlopeEnvelope`** | l'enveloppe : emprise, spectre, ou marge acceptée |

`SlopeEnvelope` ne dit pas *pourquoi* l'enveloppe est large — la bibliothèque ne le sait pas. Il
dit **où regarder**, ce qu'ADR-082 demande, et s'arrête là. C'est moins que ce qu'A208 réclamait,
et c'est tout ce qui est vrai.

**Les constructions ne changent pas.** `RadialImpact::new` et `ImpactField::new` connaissent
exactement la pente réelle maximale de leur champ ; leur `Steepness` désigne bien la pente.

## Réception

275 tests, cinq ignorés — deux de plus.

`each_slope_verdict_is_reachable_and_names_its_own_cause_s144` vérifie qu'aucun des trois noms
n'est une promesse vide, sur un montage minimal — journal vide, aucun champ — où `steepness` porte
le majorant et la normale la pente réelle, les deux étant indépendants dans un `WaterSample`.

`the_same_envelope_names_the_field_or_itself_depending_on_the_point_s144` vérifie que le nouveau
nom **sert** : même champ de pression, même limite, même enveloppe, deux points — au plus raide
`Slope`, au plus plat `SlopeEnvelope`.

**Six essais existants ont changé d'attente**, et c'est le résultat le plus instructif : tous
attendaient `Slope`, et cinq exerçaient en réalité le majorant. L'un d'eux,
`normal_matches_spatial_difference_and_envelope_sees_cancellation`, construisait exactement le cas
d'A208 bien avant qu'elle soit ouverte — son nom dit « l'enveloppe voit une compensation » — sans
pouvoir le nommer autrement que `Slope`.

Aucun hachage touché, harnais H1 inchangé : les noms de refus ne sont pas des bits publiés.

## Note datée du 2026-09-13 (S205) — ADR-128

Les trois noms restent. Le sens de **`Slope`** se précise : la pente réelle **des perturbations**
au point dépasse `max_slope` — celle de B n'est plus ni au budget ni dans le verdict
([ADR-128](ADR-128-le-budget-de-pente-borne-les-perturbations.md)). L'essai
`each_slope_verdict_is_reachable_and_names_its_own_cause_s144` construisait ses verdicts avec le
fond seul ; il les construit désormais avec un champ d'impact, et vérifie qu'une pente de B de
0,5 ne change aucun verdict.
