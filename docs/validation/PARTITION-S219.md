# S219 — Partition adaptative de l'annonce locale de pente

## Contrat et preuve de couverture

Suite de construction d'ADR-135, sans modification des admissions. `Field` et
`Prepared` proposent `partition_slope_envelope`, avec un pool de `SlopeCell` fourni
par l'appelant et un plafond d'évaluations locales. Le résultat publie borne, feuilles,
évaluations consommées et motif d'arrêt ; seule la tranche `pool[..leaves]` est valide.

La racine couvre le rectangle demandé. Chaque division binaire coupe le grand côté
au milieu représentable et remplace le parent par ses deux enfants fermés : leur union
est le parent. Chaque enfant prend le minimum de sa borne locale et de celle du
parent, toutes deux valides sur l'enfant sous les mêmes limites numériques. Par
induction, la couverture est conservée et son maximum ne peut augmenter. Un tas maximal
choisit la borne la plus haute ; à égalité, le grand côté le plus long passe d'abord.
Les autres égalités suivent un ordre de parcours fixe, sans aléa.

Une racine coûte1 évaluation, chaque division2 ; aucun appel ne dépasse son plafond.
Un budget pair laisse donc une unité inutilisée. La division ne commence pas si le
pool est plein ou si deux évaluations ne restent pas. Un point non divisible retourne
`Precision` ; un champ exactement nul `Zero`. Pool vide et budget nul sont refusés,
ainsi que les refus propagés du calcul local. Les deux enfants sont calculés avant
mutation du pool. Une erreur ne publie aucun résultat valide de cet appel.

Complexité O(E*(N+log M)), E évaluations, N modes, M capacité ; mémoire20M octets pour
les cellules sur cette cible. Pas d'allocation dans le parcours, inspection du code
sans instrumentation. Le plafond est un nombre d'évaluations, **pas un budget en ms**.
Aucune certification nouvelle de l'arrondi : A258 et les limites d'ADR-135 restent.

Chaque appel repart de la racine à l'instant du champ préparé. **La reprise entre
appels n'est pas construite** : aucun pool antérieur n'est accepté comme une preuve
pour un nouveau champ ou un nouvel instant. Une future reprise devra lier cet état
à la publication ; l'arrêt présent conserve une couverture exploitable immédiatement.

## Critères de réception

Avant campagne : couverture sans trou aux arrêts, aire totale conservée, borne
non croissante quand le plafond augmente, bornes au-dessus des sondes, déterminisme
bits et rectangles, budget strict, capacité/point/zéro/refus exercés. Deux tests dédiés
S219, et contrôle contexte/instant dans le test Prepared existant S218.
La couverture est une propriété structurelle prouvée ci-dessus ; les sondes sont des
contre-épreuves numériques supplémentaires, pas sa preuve.

## Protocole de coût

Fixtures et références S218/S217 conservées : emprise128×96m,4096 demi-modes,
sigma2/cutoff3,force19620N ; base/lente/longue à tau4, tardive à tau24 hors durée
d'image. AMD Ryzen AI7 350, Windows, CPU release un fil. Présents : préparation
modale, annonce locale avec reste/réserve, tas adaptatif, borne parent héritée.
Absents : GPU, LOD, visibilité, mutualisation, cache temporel. Préparation séparée,
100 appels locaux de chauffe, pool alloué avant mesure. Plafonds8191/32767/65535 ;
32768 cellules,655360 octets. Exécutions isolées après fin des tests.
Le gain est globale ADR-134/borne obtenue. Comparaison aux partitions S218 avec leur
nombre d'évaluations et leur qualité, sans conclure le budget d'un futur chemin GPU.

## Reproduction

Depuis `code/` :

```
cargo test --offline --release --workspace
cargo run --offline --release -p water-core --example partition_s219
```

Argument optionnel `base`, `lent`, `long` ou `base_tard` pour une fixture seule.

## Vérification logicielle

Deux tests dédiés réussissent en debug. Suite complète release : **362 réussis,5 ignorés** (264 cœur +4 intégration δ +1 table radiale +93 harnais), zéro échec. Le contrôle Prepared contexte/instant est inclus. Avertissements préexistants inchangés ; aucune nouvelle dépendance.

## Résultats

| cas | borne à32767 évaluations | borne à65535 | globale/borne à65535 | borne/max S217 | coût32767 /65535 (ms) |
|---|---:|---:|---:|---:|---:|
| base | 0,098479681 | 0,077374868 | 1,488482 | 1,100277 | 17790 /35637 |
| lente | 0,030864052 | 0,021462433 | 1,520529 | 1,495401 | 18065 /35930 |
| longue | 0,114180297 | 0,090740532 | 1,480557 | 1,354589 | 17808 /35395 |
| tardive, hors durée d'image | 0,098750442 | 0,077684335 | 1,496047 | 1,743644 | 17505 /35544 |

À8191 évaluations, aucun cas n'est resserré : la branche globale augmentée de sa
réserve numérique est encore retenue, pour4,4–4,5s de travail. À65535 évaluations,
la réduction sur les trois cas recevables atteint32,5–34,2 %. Toutes les références
restent sous les bornes. Tous les appels finissent sur leur plafond d'évaluations.

Sur la base,32767 évaluations adaptatives donnent0,098479681 en17,79s contre
0,112293623 en27,55–28,21s pour49152 évaluations uniformes S218 : meilleur résultat
avec moins de travail. Ce verdict ne se transporte pas à toutes les fixtures : sur
la lente,32767 évaluations adaptatives donnent0,030864052, moins serré que0,026652928
sur la grille uniforme0,5m. À65535 évaluations, toutes les bornes sont meilleures,
mais le travail est supérieur. Ni gain de coût universel ni budget d'image reçu.

**Obstacle identifié (A259).** La borne locale est plafonnée par une valeur globale
identique sur les grandes régions. Le tas ne peut les distinguer : l'ordre reste
largement géométrique jusqu'à ce que leur reste spatial diminue assez. Un ordonnanceur
adaptatif ne crée pas une information que la borne ne lui donne pas. Le levier suivant
est la qualité de la borne à grande maille, pas un autre ordre de tas à valeurs égales.

**Suite proposée S220, file J1/W.** Construire une borne locale qui conserve les
annulations de la somme : pente et Hessienne signée au centre, reste d'ordre supérieur
borné séparément ; dériver le traitement des phases quantifiées avant intégration.
Recevoir cette branche sur les mêmes rectangles puis avec le parcours S219, à coût
complet, sans promettre un gain. Conserver ADR-135 comme branche de repli. A258 doit
être traitée avant migration d'admission ; A255, somme A254 et loi GPU restent ouvertes.
J2/δ général et V-noyau conservent leurs déclencheurs dans la file active.

Second passage isolé base : préparation6,454ms ; évaluations8191/32767/65535 en4450,718/17967,317/35753,648ms. Bornes et nombres de feuilles imprimés identiques au premier passage. [Relevés bruts](PARTITION-S219-MESURES.md).
