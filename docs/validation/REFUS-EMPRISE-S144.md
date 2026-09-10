# S144 — Ce qu'un refus de pente peut dire, et ce qu'il ne peut pas

2026-09-10. Traite **A208**, ouverte en S140 et reportée trois fois.

## 1. Le reproche, et ce qu'il faut vérifier avant d'y répondre

A208 dit : à champ identique, l'emprise publiée décide de la part de budget consommée — ×10,9
mesuré sur une case, sans borne — et le refus rendu, `Slope` ou `Steepness`, désigne la **pente**,
c'est-à-dire la seule chose que l'hôte n'a pas à changer. ADR-082 exige qu'un nom de refus désigne
ce qu'il faut revoir.

Elle proposait deux réparations : nommer le cas (`Footprint`), ou publier le rapport des deux
enveloppes pour que l'appelant voie sa marge. **Aucune des deux n'est prise telle quelle**, et il
faut dire pourquoi avant de proposer autre chose.

## 2. Ce qui est calculable au moment du refus — le constat qui décide

Les trois budgets refusent **après** avoir échantillonné le point demandé :

| site | ce qui est disponible au moment du test |
|---|---|
| `composition.rs:80` | `slope[]` accumulé sur tous les champs, au point demandé |
| `mixed_water.rs:270` | idem, plus la part de pression |
| `bound_pressure.rs:658` | `w.slope` et la normale du fond ; le composé est formé trois lignes plus bas |

**La pente réelle au point demandé est donc connue.** Elle ne dit pas le maximum sur l'emprise —
S140 a montré qu'il ne se calcule pas, il se cherche — mais elle en est une **borne inférieure**,
et cela suffit à distinguer deux situations qui n'ont pas le même remède :

- la pente réelle **au point** dépasse déjà `max_slope` → le champ est vraiment trop raide ici,
  et le nom actuel dit vrai ;
- la pente réelle au point tient, et **seule la somme des majorants** dépasse → le refus vient de
  la marge, pas du champ.

Ces deux cas sont **décidables**, sans rien chercher ni rien supposer.

## 3. Pourquoi `Footprint` serait un nom qui ment

Le facteur d'alignement dépend de l'emprise, mais pas seulement : il dépend aussi de la structure
du spectre publié. Une emprise large sur un spectre défavorable donne le même refus qu'une emprise
étroite. **La bibliothèque ne peut pas attribuer la cause à l'emprise**, et ADR-082 refuse un nom
qui ment autant qu'un nom vague.

Ce qu'elle peut dire est plus modeste et exact : *ce n'est pas ta pente ici qui refuse, c'est le
majorant*. L'hôte sait alors où regarder — son emprise, son spectre, ou sa marge — sans que la
bibliothèque prétende choisir pour lui.

## 4. Pourquoi la seconde réparation d'A208 est périmée

Publier le rapport des deux enveloppes — L1 et resserrée — ne dit que le facteur de **forme**, et
**S141 l'a déjà retiré** : `bound_pressure::Prepared` retient la resserrée depuis. Le rapport
vaudrait donc ce qu'il vaut pour la forme seule, borné par 2, quand le sujet d'A208 est
l'alignement, qui n'est pas borné.

A208 a été écrite en S140, **avant** cette migration. Sa seconde réparation n'a plus d'objet : ce
n'est pas qu'elle soit fausse, c'est que la session suivante l'a rendue sans effet.

## 5. Ce que l'instruction a trouvé en passant : `Slope` recouvre deux causes sans rapport

`composition::compose` rend `Error::Slope` dans deux cas :

```rust
if !max_slope.is_finite() || max_slope <= 0.0 { return Err(Error::Slope); }   // ligne 31
…
if !bound.is_finite() || bound > max_slope    { return Err(Error::Slope); }   // ligne 80
```

Le premier est une **faute d'entrée de l'hôte** — un paramètre invalide ; le second un **verdict
sur le champ**. Les deux portent le même nom, et `mixed_water` comme `bound_pressure` reproduisent
le motif. C'est exactement le fourre-tout qu'ADR-082 a démonté ailleurs, resté ici parce que rien
ne l'a rouvert depuis.

## 6. La pesée

Le critère est celui de S143 : **qu'est-ce qui aurait aidé quelqu'un qui se fait refuser sans
comprendre.**

| voie | ce qu'elle donne | coût | retenue |
|---|---|---|---|
| fermer A208 sans rien faire | rien | nul | **non** — §2 montre qu'un diagnostic exact est possible |
| `Footprint` | un nom | faible | **non** — il attribue une cause que la bibliothèque ne peut pas établir (§3) |
| publier le rapport des deux enveloppes | le facteur de forme | faible | **non** — périmé, S141 l'a déjà retiré (§4) |
| **distinguer les causes décidables par des noms** | *ce n'est pas ta pente ici, c'est le majorant* | 3 énumérations, 6 essais | **oui** |
| une fonction de diagnostic détaillée | la marge par couche | moyen | **non** — surface publique sans lecteur (ADR-082) ; l'hôte a déjà les deux enveloppes et `sample` |

**Les constructions ne sont pas concernées.** `RadialImpact::new` et `ImpactField::new` connaissent
**exactement** la pente réelle maximale de leur champ — c'est la borne L1 divisée par le rapport
mesuré — donc leur `Steepness` désigne bien la pente, et il dit vrai. A208 ne portait que sur les
budgets, où la part de pression dépend de l'emprise.

## 7. Les trois noms

| cause | nom | ce qu'il faut revoir |
|---|---|---|
| `max_slope` non fini ou négatif | **`MaxSlope`** | le paramètre fourni par l'hôte |
| la pente réelle **au point** dépasse `max_slope` | **`Slope`** *(sens resserré)* | le champ, qui est vraiment trop raide ici |
| la pente au point tient, seule la somme des majorants dépasse | **`SlopeEnvelope`** | l'enveloppe : emprise publiée, spectre, ou marge acceptée |

Le troisième nom ne dit pas *pourquoi* l'enveloppe est large — la bibliothèque ne le sait pas —
mais il dit **où regarder**, ce qu'ADR-082 demande. Et il retire au premier ce qu'il n'aurait
jamais dû porter.

## 8. Ce que la garde de S143 a fait dès le premier changement

Le premier `cargo test` après la séparation des causes a échoué sur
`no_undeclared_comparison_to_max_slope_s143` — la garde écrite la veille — en signalant **trois
sites nouveaux** :

```
("composition.rs", "reelle>max_slope")
("mixed_water.rs", "reelle>max_slope")
("bound_pressure.rs", "reelle>max_slope")
```

Ils sont légitimes : ils comparent une pente réelle sans conversion, ce qu'I-18 demande. Mais
c'est **la garde qui l'a demandé**, pas ma mémoire, et elle l'a demandé le jour même. Une garde
écrite pour un troisième champ hypothétique a servi le lendemain, sur une modification qui n'avait
rien à voir avec elle.

## 9. La vérification : le nouveau nom sert-il ?

Une aide qu'on n'a pas vue aider ne vaut pas mieux qu'une garde qu'on n'a pas vue échouer.

**Sur le chemin même d'A208**, `the_same_envelope_names_the_field_or_itself_depending_on_the_point_s144`
prend un champ de pression, balaye une ligne pour y trouver le point le plus raide et le plus
plat, puis pose une limite **entre les deux pentes réelles** et sous l'enveloppe. Les deux points
sont refusés — l'enveloppe dépasse partout, elle ne dépend pas du point — mais :

| point | pente réelle | verdict |
|---|---|---|
| le plus raide | au-dessus de la limite | `Slope` — le champ est vraiment trop raide ici |
| le plus plat | sous la limite | **`SlopeEnvelope`** — c'est le majorant qui refuse |

C'est exactement la distinction qu'A208 réclamait : l'hôte qui publie sur une emprise où son champ
est plat lisait `Slope`, c'est-à-dire « ta pente », alors que sa pente était plate.

**Et un essai qui existait déjà décrivait le cas sans pouvoir le nommer** :
`normal_matches_spatial_difference_and_envelope_sees_cancellation`, écrit bien avant A208,
construisait une limite entre la pente locale et l'enveloppe — « l'enveloppe voit une
compensation », dit son nom. Il attend désormais `SlopeEnvelope`.

## 10. Réception

275 tests, cinq ignorés — deux de plus : l'essai d'atteignabilité des trois verdicts et la
démonstration ci-dessus. Aucun hachage touché, harnais H1 inchangé : les noms de refus ne sont pas
des bits publiés.

Six essais existants ont changé d'attente, et **c'est le résultat le plus instructif de la
session** : tous attendaient `Slope`, et cinq d'entre eux exerçaient en réalité le majorant. Le
dépôt testait le conservatisme en croyant tester la pente, depuis que ces essais existent.
