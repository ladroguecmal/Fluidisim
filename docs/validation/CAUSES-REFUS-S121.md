# S121 — Deux causes sous un seul refus : où le défaut est vraiment

2026-09-09. Suite de S120-1, sur A197.

## 1. Ce que A197 annonçait, et ce que la mesure a trouvé

A197 visait `RadialImpact::sample`, qui rend `Error::Domain` aussi bien pour une position hors
domaine que pour une sortie non finie. La cible était juste sur le principe et **fausse sur le
lieu** : ce bloc de finitude n'est pas atteignable.

### Le bloc de finitude de `sample` n'a pas d'entrée qui l'atteigne

`code/water-core/examples/probe_degenerate.rs` balaie énergie, longueur d'onde, rayon,
profondeur, pente maximale et horizon — chaque paramètre sur sa plage représentable, y compris
`f32::MAX`. Sur **18 719 champs construits et 673 884 échantillons, aucun refus**, et une sortie
maximale de 3,17 × 10²⁶ : douze ordres de grandeur sous le débordement `f32`.

La raison est structurelle, et la sonde la rend visible en suivant la direction où les sorties
croissent — les longueurs d'onde décroissantes, à énergie et pente maximales :

| λ | `slope_bound` | pic des sorties | refus |
|---|---|---|---|
| 10⁻⁴ | 6,17 × 10²⁶ | 3,49 × 10²⁵ | 0 |
| 10⁻⁶ | 6,17 × 10³⁰ | 3,49 × 10²⁹ | 0 |
| 10⁻⁸ | 6,17 × 10³⁴ | 3,49 × 10³³ | 0 |
| 10⁻⁹ | 6,17 × 10³⁶ | 3,49 × 10³⁵ | 0 |
| 10⁻¹⁰ | — | — | **la construction refuse** |

Le pic vaut constamment `slope_bound / 17,7`. Or la construction vérifie que `slope_bound` est
fini : **elle refuse donc avant que `sample` puisse déborder**, et avec plus d'un ordre de
grandeur de marge. Le bloc de finitude est du code défensif que rien n'atteint.

### Le même défaut existe, mais dans le constructeur

À λ = 10⁻¹⁰, la construction refuse par `Error::Steepness`. La pente maximale valait pourtant
`f32::MAX` : ce refus ne dit pas « trop raide », il dit **« la borne a débordé »**. La ligne

```rust
if !slope.is_finite() || slope > medium.max_slope { return Err(Error::Steepness); }
```

réunit deux situations qui n'ont rien de commun :

- **la pente dépasse la limite du milieu** — l'appelant a demandé un impact trop énergique pour
  ce milieu ; il peut réduire l'énergie, et le refus est un verdict physique ;
- **la borne n'est pas représentable en `f32`** — aucune énergie plus faible n'est en cause, le
  champ demandé sort du domaine numérique de la bibliothèque.

Un appelant qui traite `Steepness` comme « je réduis l'énergie et je réessaie » boucle
indéfiniment sur le second cas. C'est le défaut qu'A197 décrivait, à un étage au-dessus de
celui qu'elle désignait, et **celui-là est atteignable** : λ = 10⁻¹⁰, `energy_j = f32::MAX`,
`max_slope = f32::MAX`.

## 2. Décision

Voir [ADR-081](../adr/ADR-081-separer-limite-physique-et-limite-numerique.md).

## 3. Construction

`impact_field::Error::NotRepresentable` est ajoutée. Elle dit « la bibliothèque ne peut pas
représenter ce champ », par opposition à tout refus qui porte un verdict sur les données de
l'appelant. Trois sites l'emploient :

- **`RadialImpact::new`** — la borne de pente non finie, testée **avant** la comparaison à
  `max_slope`. Un `slope` infini n'est pas « supérieur à `max_slope` », il n'est comparable à
  rien.
- **`ImpactField::new`** — la même séparation, par cohérence de vocabulaire (voir §5).
- **`RadialImpact::sample`** — le bloc de finitude, qui rendait `Domain`. Non pas parce qu'il
  serait atteignable, mais parce qu'un type ne doit pas nommer « mauvaise position » ce qui
  serait un défaut de champ.

`mixed::sample_world_batch` traduisait toute erreur de champ en `composition::Error::Domain` ;
`NotRepresentable` y devient `NonFinite`, qui existait déjà et dit la bonne chose.

## 4. Réception

**Le cas de débordement est construit, pas supposé.** λ = 10⁻¹⁰, `energy_j = f32::MAX`,
`max_slope = f32::MAX` : la construction rend `NotRepresentable`. La pente maximale étant au
plus haut, ce n'est pas elle qui refuse. Une décade plus haut, le même montage se construit et
sa borne dépasse 10³⁶ — la frontière est bien celle de la représentation, et elle est franche.
Sur ce champ représentable, une limite de milieu basse redonne `Steepness` : le nom retrouve
son sens, celui d'un verdict que l'appelant peut lever.

**L'invariant mesuré est devenu un test.** « Tout champ que la construction accepte produit des
sorties finies sur son domaine » ne tenait que par une conjonction de bornes disséminées dans
trois fonctions. Une grille bornée — 37 champs construits, 592 échantillons, pic à 7,27 × 10³⁵ —
le vérifie à chaque exécution de la suite, et surveille la marge (facteur 468 avant
débordement). Sans lui, la mesure de S121 se périmerait en silence à la première modification
des bornes de construction.

**Aucun refus atteignable n'a changé de nom.** Les 158 tests antérieurs passent inchangés et
les hachages de la campagne `cycle_mixed` sont identiques à ceux de S118. Le seul refus
renommé dans `sample` est celui qu'aucune entrée n'atteint.

## 5. Ce que la construction a appris, et qui n'était pas dans la décision

**Le second site n'est pas atteignable non plus, et pas pour la même raison.**
`ImpactField::new` reçoit la même séparation, mais aucune entrée explorée ne l'y fait basculer :
à énergie et pente maximales, la descente en longueur d'onde passe de `Domain` — pour λ ≤ 3 mm —
directement à un champ construit, sans jamais déborder. Sa structure diffère : `side = 4λ` et le
contrôle de `scale` bornent plus tôt. Le test qui l'accompagne verrouille donc l'autre moitié de
la propriété — `Steepness` y reste un verdict sur le milieu — et constate le refus `Domain` sans
prétendre qu'il s'agit du cas visé.

Porté en note datée dans ADR-081, qui n'énonçait la séparation que pour `RadialImpact::new`.

## 6. Ce qui n'est pas revendiqué

Aucune borne de construction n'est modifiée : les mêmes champs sont acceptés et refusés
qu'avant, seuls deux refus changent de nom. Aucun résultat numérique ne bouge.

`NotRepresentable` ne couvre pas toutes les défaillances numériques de la bibliothèque — elle
nomme celles de ce chemin. Les autres couches ont leur propre vocabulaire, et les unifier serait
un travail distinct, sans demande aujourd'hui.

**Une sonde qui ne trouve pas de contre-exemple n'est pas une démonstration.** Ce qui est acquis
est une mesure, avec sa marge et sa direction de croissance identifiée, plus un test qui la
surveille. Le code défensif de `sample` est conservé pour cette raison.

## 7. Vérification

161 core + 93 harnais = **254 tests réussis, cinq ignorés** ; les tests de `radial_impact`
passent aussi en release. Quatre avertissements préexistants, aucun nouveau. Campagne
`cycle_mixed` relancée : hachages inchangés.

## 8. Suite

**S121-1, S122 :** la sonde a montré que les bornes de construction se recouvrent mal — selon
la famille de paramètres, c'est `Domain`, `Steepness`, `NotRepresentable` ou `Medium` qui refuse
en premier, et rien ne documente laquelle borne quoi. Un appelant qui veut savoir *quel*
paramètre réduire n'a pas cette information. Cartographier ces bornes, ou nommer celle qui a
mordu, est le prolongement naturel. Restent ouverts : admission dynamique dans le contrôleur,
renouvellement de fenêtre, profondeur finie de pression (S116-2), bilan mixte, durabilité disque.

81 ADR, 198 angles, 17 invariants, 6 spécifications, 23 cas.

