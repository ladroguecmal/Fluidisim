# ADR-134 — L'enveloppe de pente d'un champ de pression tient compte de l'étalement des directions

- Statut : **actée**, S216, 2026-09-13 ; autonomie technique S71.
- Traite la part **statique** d'**A255** ; A255 reste ouverte sur sa part dynamique.
- Prolonge [ADR-095](ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) (borne resserrée,
  S141) et applique à W la même distinction qu'[ADR-133](ADR-133-le-majorant-de-pente-suit-la-dispersion.md)
  a faite pour l'impact : séparer ce qu'une inégalité corrige de ce qu'une mesure seule peut dire.
- Ne réécrit aucun ADR. Ne change ni `slope_envelope_tight()`, qui reste publiée, ni un bit de
  hauteur, de vitesse ou de pente.
- Mesures : [ENVELOPPE-SILLAGE-S216](../validation/ENVELOPPE-SILLAGE-S216.md).

## Constat

Après ADR-133, le majorant du sillage pèse **88 %** du budget de pente de la scène J1, et son
pessimisme total atteint 1,9 à 16 s et 4,8 à 39 s. La suite écrite en S215 disait : refaire sur la
famille du sillage la campagne mesurée qui a traité l'impact.

**La lecture du code déplace la question avant toute mesure.** `slope_envelope_tight` somme

```rust
for s in self.slots { bound += |k_s| * |eta_s|; }
```

c'est-à-dire **scalairement**, sur des modes dont les vecteurs d'onde pointent dans des directions
différentes — le demi-spectre couvre un demi-disque. Or la pente est un **vecteur** : sa norme est
celle de la somme vectorielle. Ce majorant ne peut donc être atteint que si les modes s'alignent en
phase **et** en direction, et la seconde condition est fausse par construction de la recette.

ADR-095 avait retiré deux facteurs, chacun entre 1 et √2 : la direction du vecteur d'onde **au sein
d'un mode** (`|kx|+|ky| → |k|`) et la phase de sa réponse. L'étalement des directions **entre**
modes n'a jamais été touché — A206 nommait déjà ce qui restait, sans le décomposer.

Il y a donc deux pessimismes, et les confondre ferait calibrer une table là où une inégalité suffit.

## Ce que la mesure a séparé

| âge (s) | phase | scalaire | directionnel | réel | part statique | résidu |
|---:|---|---:|---:|---:|---:|---:|
| 4 | forçage | 0,135072 | 0,107316 | 0,097430 | **1,2586** | 1,1015 |
| 16 | forçage | 0,164995 | 0,135757 | 0,086934 | 1,2154 | 1,5616 |
| 30 | après | 0,156765 | 0,131012 | 0,038229 | 1,1966 | 3,4270 |
| 39 | après | 0,157981 | 0,131885 | 0,033152 | **1,1979** | **3,9782** |

**La part statique ne dépend pas de la quadrature** : 1,2586 à quatre décimales pour `angular`
64 / 128 / 256, `radial` 32 / 128, `cutoff` 2 / 4. C'est une propriété du champ, pas de sa
discrétisation — donc elle se **calcule**, elle ne se tabule pas. Elle bouge avec ce qui change le
champ : 1,31 à σ = 1 ; 1,40 à 1,52 à 1,5 m/s ; 1,44 à 1,48 à 6 m/s.

**Le résidu n'est pas une limite d'emprise.** À âge fixé et à pas de grille constant, agrandir
l'emprise d'un facteur 4 en côté — **seize fois en aire**, 512 × 416 m — laisse le maximum réel
identique à six décimales. Le maximum est intérieur et saturé : ce qui reste est bien la
décohérence de phase (L290), et aucun remède d'emprise ne l'atteindra.

## Décision

**1. `Field::slope_envelope_directional()` est publiée, et c'est une inégalité, pas une mesure.**
Pour toute direction `e_θ`, la pente projetée vaut au plus `Σ c_i |cos(θ − θ_i)|` avec
`c_i = |k_i| |η_i|`, et la norme de la pente est le maximum de cette quantité sur `θ`. Par
Cauchy–Schwarz :

```
Σ c_i |cos(θ − θ_i)|  ≤  √( C · Σ c_i cos²(θ − θ_i) )  =  √( C · (C + R cos 2(θ−φ)) / 2 )
                      ≤  √( C · (C + R) / 2 )
```

avec `C = Σ c_i` et `R = |Σ c_i e^{2iθ_i}|`. **Un seul passage, deux accumulateurs, aucune
arc-tangente** : `c_i cos 2θ_i = |η_i| (k_x² − k_y²)/|k_i|` et `c_i sin 2θ_i = 2 |η_i| k_x k_y/|k_i|`.
`O(N)`, sans grille de directions, sans table, sans garde, sans domaine de validité.

**2. `bound_pressure::Prepared` la retient**, aux trois sites de préparation, à la place de la
borne resserrée. C'est la substitution qu'ADR-095 réservait à une décision : elle déplace des refus,
et c'est celle-ci.

**3. `slope_envelope_tight()` reste publiée et inchangée.** Elle borne la même grandeur et reste
le témoin par lequel le gain se lit ; aucun consommateur n'est forcé de migrer.

**4. Les deux bouts sont vérifiés, pas argumentés.** Directions toutes égales ⟹ `R = C` ⟹ la borne
vaut exactement la somme scalaire, qui est alors atteignable ; directions équiréparties ⟹ `R = 0`
⟹ la borne vaut `C/√2 ≈ 0,707 C` quand le maximum vrai vaut `2C/π ≈ 0,637 C`. Le test
`directional_envelope_brackets_the_real_slope_s216` fixe les deux, plus la sûreté — jamais sous la
pente réelle échantillonnée — et l'existence d'un gain réel.

## Ce que cette décision ne fait pas

- **Elle ne touche pas au résidu**, qui est la part dynamique d'A255 : 1,10 à 4 s mais **3,98 à
  39 s**, et gouverné par le temps écoulé **depuis l'extinction** de la source (6,29 à 30 s pour un
  sillage de deux tronçons, 1,75 pour seize encore en forçage). C'est une décohérence, elle demande
  une loi mesurée comme ADR-133, et **A255 reste ouverte pour cela**.
- **Elle n'est pas la borne la plus serrée possible.** Le maximum exact sur `θ` est calculable : le
  demi-spectre ne porte que `angular/2` directions distinctes — 64 pour la recette de la fixture —
  donc `O(A²)`, quelques milliers d'opérations. Cauchy–Schwarz est préférée ici parce qu'elle est
  `O(N)` sans regroupement, sans tableau intermédiaire et sans hypothèse sur la disposition des
  emplacements. Le maximum exact reste disponible si la marge redevient contraignante.
- **Elle ne dit rien de l'impact** : `RadialImpact` publie un majorant de pente réelle (ADR-094,
  ADR-133), pas une somme modale, et ce raisonnement ne s'y applique pas.
- **Elle ne change aucun bit publié** : `sample`, `render_components` et les valeurs de hauteur,
  vitesse et pente sont intacts. Seules l'annonce `slope_envelope` et la frontière d'admission
  qu'elle borne bougent.

## Réversibilité

Un accumulateur de plus dans une méthode, et trois lignes de préparation qui appellent l'une plutôt
que l'autre. Revenir consiste à rappeler `slope_envelope_tight`, qui n'a pas bougé. Remplacer la
borne par le maximum exact sur la grille angulaire est une amélioration du même objet et demande un
ADR qui remplace celui-ci, pas une réécriture.
