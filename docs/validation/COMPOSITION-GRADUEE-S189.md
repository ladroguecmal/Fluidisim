# S189 — La composition sur réseau gradué, et l'additivité locale

2026-09-12. S188-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[COMPOSITION-ERREURS-S186](COMPOSITION-ERREURS-S186.md) a mesuré que l'erreur spatiale et
l'erreur temporelle se composent selon le **maximum** pour les modes causaux.
[COMPOSITION-ANCREE-S188](COMPOSITION-ANCREE-S188.md) a confirmé ce verdict sur un réseau
ancré et proposé une raison — les deux maxima vivent sur la même tranche — d'où **A232** : la
loi ne vaudrait que tant qu'ils coïncident.

Cette session fait deux choses. Elle éprouve A232 sur le seul réseau connu qui **sépare** les
deux maxima. Et elle révise le raisonnement de S188, qui localisait à une granularité trop
grossière pour trancher.

Comme en S183–S188, les conditions sont publiées **avant** la première exécution (§1–§6) ;
les relevés viennent après (§7).

## 1. La révision, d'abord, parce qu'elle change la question

S188 a localisé les maxima à la **tranche** : un plan de 14×14 mailles. Il en a conclu que les
deux erreurs culminent « au même endroit », et que la loi du maximum s'expliquait par cette
rencontre.

**Le raisonnement ne tient pas tel quel, et c'est son propre résultat qui le dit.** Si les deux
erreurs culminaient réellement sur la **même maille**, elles s'y ajouteraient — au signe près —
et la loi mesurée serait l'**additive**. Or la loi mesurée est le **maximum**, sur les deux
réseaux, pour les deux modes causaux. La lecture la plus simple de ce fait est l'inverse de
celle de S188 : **les deux erreurs ne se rencontrent pas**. La tranche était une granularité
trop grossière pour distinguer les deux situations.

Rien de ce que S188 a mesuré n'est faux : la tranche du maximum *est* la 14 dans les 84 cases.
C'est l'inférence qui allait trop vite, et c'est elle que cette session instruit.

## 2. L'hypothèse forte, dérivée avant d'être mesurée

L'état initial est nul et le pas du véhicule est **presque linéaire** en la source : `−S` entre
additivement, l'advection est quadratique en `u'` et la viscosité linéaire. Pour un champ qui
part de zéro et reste petit, l'écart de champ devrait donc être **additif maille par maille** :

```
H1 :   Δu(r,c)(x)  ≈  Δu(r,1)(x)  +  Δu(1,c)(x)     pour toute maille x
```

où `Δu(r,c) = u'(r,c)(T) − u'_réf(T)`, `Δu(r,1)` est l'erreur **purement spatiale** et
`Δu(1,c)` l'erreur **purement temporelle**. Si H1 ferme, alors les trois « lois » de S186 et
S188 **cessent d'être des lois concurrentes** : elles deviennent trois lectures de la
**géométrie des pics** de deux champs qui s'additionnent.

| situation des pics | max de la somme | loi de norme observée |
|---|---|---|
| pics **disjoints** | le plus grand des deux | **maximum** |
| pics **confondus**, mêmes signes | la somme | **additive** |
| pics partiellement recouvrants | entre les deux | quadratique « apparente » |

C'est une explication mécanique, et elle est **falsifiable** : il suffit que le résidu
d'additivité locale soit du même ordre que les erreurs elles-mêmes pour qu'elle tombe.

**Elle ne peut pas fermer exactement**, et le protocole le dit avant la mesure : l'advection
est quadratique, donc le produit croisé des deux erreurs subsiste. Le résidu attendu est celui
de cette non-linéarité, et il sera comparé au **plancher de la référence** (0,386 %,
[CADENCE-3D-S185](CADENCE-3D-S185.md) §6.1) plutôt que déclaré petit à vue d'œil.

## 3. Le réseau qui sépare les maxima

[RESEAU-GRADUE-S187](RESEAU-GRADUE-S187.md) §8.5 a relevé, sans en tirer cette conséquence,
la seule configuration du dépôt où le maximum de l'erreur spatiale **quitte la tranche haute** :
horizontale **pleine** et `z` **gradué**. L'erreur de tranche haute y tombe à 0,0000–0,0001 %
et le maximum se déplace vers le milieu du bloc.

La raison est simple et vaut d'être écrite : la graduation pose un nœud **sur** la tranche
haute et y resserre le pas, donc elle y annule l'erreur ; et l'horizontale pleine n'apporte
aucune erreur dont le pic serait en haut. L'erreur temporelle, elle, reste accrochée au maximum
du **champ**, qui est en haut quoi qu'on fasse du réseau. **Les deux pics sont donc séparés par
construction**, et c'est exactement l'épreuve qu'A232 réclame.

Deux familles, à `c = 1` puis sur toute la grille :

| famille | horizontale | verticale | nœuds | pic spatial attendu |
|---|---|---|---:|---|
| **graduée** | pleine, 14 nœuds ancrés | graduée, `Nz = 3` | 588 | milieu du bloc |
| | | `Nz = 4` | 784 | milieu |
| | | `Nz = 5` | 980 | milieu |
| | | `Nz = 8` | 1568 | milieu |
| **ancrée uniforme** *(témoin)* | 8 nœuds ancrés | 8 nœuds ancrés | 512 | tranche haute |
| | 5 nœuds ancrés | 5 nœuds ancrés | 125 | tranche haute |

Le témoin ancré reprend deux lignes de S188 et doit redonner leurs chiffres ; il sert à lire
la famille graduée contre quelque chose de connu, et non contre rien.

La règle de graduation et le profil `∂²_z S` qui la nourrit sont ceux de S187, **sortis dans
`support/`** pour que les deux sessions posent le même réseau — deux copies divergeraient
(L137). S187 est rejoué et son empreinte vérifiée.

## 4. Ce qui est mesuré

Mêmes bloc, pas, montage, **référence unique** et métriques que S186 et S188 : `eS`, `eU`,
plancher 0,386 %, dénominateurs `max |S| = 1,540547e-4 m/s²` et
`max |u'(T)| = 7,993168e-5 m/s`. Trois ajouts, tous destinés à H1 :

1. **La maille du maximum**, et non la tranche : l'indice `(i, j, k)` complet, pour l'erreur
   spatiale seule, l'erreur temporelle seule et l'erreur composée. Plus la **distance** entre
   les mailles des deux premiers, en mailles.
2. **Le résidu d'additivité locale** :
   `max_x |Δu(r,c)(x) − Δu(r,1)(x) − Δu(1,c)(x)|`, rapporté à `max |u'(T)|`. C'est le juge de
   H1.
3. **La décomposition au point du maximum composé** : à la maille où `Δu(r,c)` culmine, les
   valeurs locales de `Δu(r,1)` et `Δu(1,c)`. Si l'une est négligeable devant l'autre, la
   disjonction des pics est constatée directement et non déduite.

Les trois lois de norme de S186 §5 — additive, quadratique, maximum — sont rejouées **avec le
même critère `[0,80 ; 1,25]`**, pour la continuité, et parce que leur verdict sur la famille
graduée est la réponse littérale à A232.

## 5. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** Deux exécutions, mêmes bits ; une empreinte est publiée.
   Aucune durée n'est mesurée.
2. **Le témoin ancré redonne S188** : 1,7160 % à 512 nœuds et 3,6805 % à 125 nœuds à `c = 1`,
   et les erreurs temporelles pures redonnent S186 §6.2 — donc S185.
3. **La famille graduée redonne S187 §8.5** à `c = 1` : 6,5503 / 3,4373 / 1,7164 / 0,6393 %
   pour `Nz = 3 / 4 / 5 / 8`, avec une erreur de tranche haute nulle.
4. **H1 est exacte quand elle doit l'être.** À `c = 1` l'erreur temporelle est nulle, donc
   `Δu(r,1) = Δu(r,1) + 0` : le résidu d'additivité doit être **exactement zéro**, en bits. De
   même à réseau plein. C'est le contrôle qui vérifie le calcul du résidu lui-même, avant de
   lui faire dire quoi que ce soit.
5. **Le support historique est intact.** `cadence_error` (`0x39567a1d4bc2ba4c`),
   `composed_error` (`0x0e743846d4656870`), `graded_lattice` (`0x6cf13183b4a240df`) et
   `anchored_composition` (`0x21bab548c7b9775c`) rendent leurs empreintes publiées après le
   déplacement du profil et des indices gradués dans `support/`.
6. **Tout reste fini.**

## 6. Ce que la mesure ne prouvera pas

- **Aucun seuil de justesse.** A50 attend une décision.
- **H1 est une propriété du véhicule, pas du solveur du projet.** Le pas d'essai est presque
  linéaire parce qu'il est explicite, sans projection et sans surface libre. Un solveur avec
  projection de pression couple les mailles à chaque pas, et l'additivité locale y serait à
  remesurer. Ce que H1 donnerait, c'est l'explication des mesures **déjà publiées**, pas une
  garantie pour le système à écrire.
- **Un seul montage, une seule profondeur de bloc, un seul instant de profil.**
- **Pas de réseau horizontalement gradué**, ni d'arbre (ADR-118).
- **Aucun coût.** Les nombres de nœuds de la famille graduée sont supérieurs à ceux de la
  famille ancrée : cette session ne compare pas des coûts, elle sépare deux pics.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur — c'est
  précisément ce qui rend H1 plausible, et donc ce qui en borne la portée.
