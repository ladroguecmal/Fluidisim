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
