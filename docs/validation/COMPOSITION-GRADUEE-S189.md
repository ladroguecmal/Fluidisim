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

## 7. Relevés

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example graded_composition
```

Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.
Même bloc, même référence, mêmes dénominateurs que S186 et S188 :
`max |S| = 1,540547e-4 m/s²`, `max |u'(T)| = 7,993168e-5 m/s`, plancher 0,386 %.

Les réseaux éprouvés, et les indices que la graduation produit :

| réseau | nœuds | indices `z` |
|---|---:|---|
| plein | 2744 | 1…14 |
| graduée `Nz = 3` | 588 | `[1, 10, 14]` |
| graduée `Nz = 4` | 784 | `[1, 8, 12, 14]` |
| graduée `Nz = 5` | 980 | `[1, 6, 10, 12, 14]` |
| graduée `Nz = 8` | 1568 | `[1, 4, 7, 9, 11, 12, 13, 14]` |
| ancrée 8³ *(témoin S188)* | 512 | `[1, 3, 5, 7, 8, 10, 12, 14]` |
| ancrée 5³ *(témoin S188)* | 125 | `[1, 4, 8, 11, 14]` |

### 7.1 Réceptions — les six passent

1. **Reproductibilité.** Empreinte `0x30b0b9eee43f6255`, `diff` identique sur deux
   exécutions ; aucune durée mesurée.
2. **Le témoin ancré redonne S188** : 1,7160 % à 512 nœuds et 3,6805 % à 125 nœuds à `c = 1`,
   et les vingt-et-une erreurs temporelles pures redonnent S186 §6.2 — donc S185.
3. **La famille graduée redonne S187 §8.5** : 6,5503 / 3,4373 / 1,7164 / 0,6393 % pour
   `Nz = 3 / 4 / 5 / 8`, avec une erreur de tranche haute de **0,0000 à 0,0001 %**.
4. **Le résidu d'additivité est exactement nul** partout où l'un des deux termes est nul —
   à `c = 1` et au réseau plein. C'est le contrôle du calcul du résidu lui-même, avant de lui
   faire dire quoi que ce soit.
5. **Les quatre empreintes du support tiennent** après le déplacement du profil et des indices
   gradués : `cadence_error` `0x39567a1d4bc2ba4c`, `composed_error` `0x0e743846d4656870`,
   `graded_lattice` `0x6cf13183b4a240df` (sortie **entière** identique),
   `anchored_composition` `0x21bab548c7b9775c`.
6. **Tout reste fini.**

### 7.2 La séparation des pics est obtenue, et elle est nette

| réseau | pic spatial `(i,j,k)` | eU spatiale % | eU tranche 14 % |
|---|---|---:|---:|
| graduée `Nz = 3` | **(1, 1, 6)** | 6,5503 | 0,0001 |
| graduée `Nz = 4` | **(1, 1, 5)** | 3,4373 | 0,0001 |
| graduée `Nz = 5` | **(1, 1, 8)** | 1,7164 | 0,0001 |
| graduée `Nz = 8` | **(1, 1, 6)** | 0,6393 | 0,0000 |
| ancrée 8³ | (11, 4, **14**) | 1,7160 | 1,7160 |
| ancrée 5³ | (10, 6, **14**) | 3,6805 | 3,6805 |

Le pic **temporel** reste en `(1,1,14)` ou `(1,2,14)` pour les vingt-et-une cadences, tous
modes confondus : il est accroché au maximum du **champ**, qui est en haut du bloc quoi qu'on
fasse du réseau. La famille graduée sépare donc les deux pics de **6 à 10 mailles**, la
famille ancrée les laisse au même étage. C'est exactement l'épreuve qu'A232 réclamait, et elle
est obtenue par construction plutôt que par chance.

### 7.3 La révision de §1 est confirmée : les deux pics ne coïncident jamais

> **Sur 78 cases jugées, les deux pics tombent sur la même maille : 0 fois.**

La « coïncidence des maxima » qu'avançait S188 était un effet de **granularité** : il
localisait à la tranche, un plan de 196 mailles. Au niveau de la maille, les deux erreurs sont
toujours distinctes — y compris dans la famille ancrée, où elles partagent l'étage 14 mais pas
la colonne.

Rien de ce que S188 a mesuré n'est faux ; c'est son inférence qui allait trop vite, et §1
l'avait annoncé avant la mesure.

### 7.4 H1 : l'additivité locale tient, et elle explique tout

Résidu `max_x |Δu(r,c)(x) − Δu(r,1)(x) − Δu(1,c)(x)|` :

- **au plus 1,9263 %** de `max |u'(T)|` en absolu ;
- **au plus 10,0 %** de l'erreur de sa propre case ;
- **exactement nul** dans les cas dégénérés (réception 4).

Le résidu croît avec l'erreur, ce qui est la signature du terme croisé **advectif** annoncé au
protocole. H1 ne ferme donc pas à l'arrondi — elle n'était pas censée le faire — mais elle
tient à 10 %, et cela suffit pour que le comportement en norme en découle.

**Où tombe le pic composé**, sur 78 cases jugées : sur le pic **spatial** 19 fois, sur le pic
**temporel** 40 fois, **ailleurs** 19 fois. Les colonnes locales du relevé le montrent sans
détour :

| cas | pic composé | spatial **au pic** | temporel **au pic** | eU | rapport au max |
|---|---|---:|---:|---:|---:|
| mnt · graduée `Nz=5` · `c=4` | (1,1,14) | **0,0000** | 2,3000 | 2,3000 | **1,000** |
| mnt · graduée `Nz=5` · `c=8` | (1,1,14) | **0,0000** | 5,0549 | 5,0549 | **1,000** |
| mnt · graduée `Nz=5` · `c=2` | (14,4,13) | 1,2631 | 0,3106 | 1,5731 | 0,917 |
| int · graduée `Nz=3` · `c=64` | (1,1,6) | 6,5503 | 2,8585 | 9,6129 | **1,419** |

La lecture est mécanique. Quand le pic composé tombe sur le pic temporel et que l'erreur
spatiale y est **nulle**, le maximum est exact — rapport 1,000, et cela arrive 40 fois sur 78.
Quand les deux erreurs sont non nulles au même point, elles s'**ajoutent** localement — 9,6129
mesuré contre 9,4088 sommé — et le rapport au maximum monte à 1,419. Et quand elles s'y
opposent, le composé passe **sous** le maximum.

**Les trois « lois » de S186 cessent donc d'être trois lois concurrentes.** Ce sont trois
lectures d'une même structure : deux champs qui s'additionnent, et dont la position relative
des pics décide de ce que la norme maximum affiche.

### 7.5 A232 est confirmée, et attribuée à la géométrie

Deux familles mesurées côte à côte, mêmes cadences, même critère, même référence :

| famille | mode | additive | quadratique | maximum | retenue |
|---|---|---|---|---|---|
| **graduée** (pics séparés) | maintien | 0,437–0,981 | 0,607–1,000 | **0,730–1,000** | **aucune** |
| | extrapolation | 0,479–0,977 | 0,676–1,000 | 0,811–1,000 | maximum |
| | interpolation | 0,663–0,963 | 0,892–1,102 | 1,000–1,511 | quadratique |
| **ancrée** (pics au même étage) | maintien | 0,576–0,951 | 0,794–0,999 | **0,936–1,060** | **maximum** |
| | extrapolation | 0,468–0,941 | 0,659–0,998 | 0,869–1,000 | maximum |
| | interpolation | 0,845–0,955 | 1,066–1,245 | 1,074–1,707 | additive, quadratique |

**Sur le réseau que recommande ADR-118 — le gradué — la loi du maximum est rejetée pour le
maintien**, à 0,730 ; sur le réseau ancré elle tient, à 0,936. Même montage, même critère,
même référence : **la géométrie des pics change le verdict.** A232 n'était pas une précaution
de style.

**La direction de l'échec compte.** 0,730 signifie que l'erreur composée est **30 % au-dessous**
du maximum des deux. La loi **surestime** : elle demeure une borne, mais cesse d'être une
estimation — et la règle « égaliser les deux axes puis s'arrêter » perd sa justification,
puisque l'optimum n'est plus à la parité.

**Ce qui survit à tout : l'additive.** Rapport maximal **0,981** ici, 0,984 en S188, 0,988 en
S186 : jamais dépassé, sur trois géométries de réseau et trois sessions. C'est la seule forme
portable, et c'est ce que fixe
[ADR-119](../adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md).

## 8. Ce que la session conclut, et ce qu'elle laisse ouvert

**Conclu**, et acté par [ADR-119](../adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md).

1. **La somme borne le total**, sur les trois géométries mesurées, sans exception. C'est la
   seule forme portable.
2. **Le maximum n'est pas une loi de composition** : c'est ce qu'affiche la norme quand les
   deux pics sont disjoints. Il est rejeté sur le réseau gradué pour le maintien.
3. **La règle « égaliser les deux axes puis s'arrêter » est abandonnée** (ADR-119 §3). Elle
   supposait le maximum.
4. **Le mécanisme est l'additivité locale** : les deux champs d'erreur s'additionnent maille
   par maille à 10 % près, et exactement dans les cas dégénérés.
5. **Les deux pics ne coïncident jamais** — 0 cas sur 78 — ce qui corrige l'inférence de S188
   sans toucher à ses mesures.
6. **Un budget conjoint reste licite**, ce que S186 cherchait à établir ; c'est sa répartition
   qui change.

**Non conclu, et pas contourné.**

- **H1 sur un solveur qui projette n'est pas mesurée**, et c'est la limite la plus sérieuse :
  la projection couple toutes les mailles à chaque pas, et toute la structure décrite ici
  découle de l'additivité. Le véhicule est explicite, sans projection ni surface libre.
- **La borne est lâche** : son rapport descend à 0,437, donc dimensionner par elle peut coûter
  jusqu'à **2,3 fois** la résolution nécessaire, et aucune estimation portable n'est connue
  (**A233**).
- **Aucun seuil de justesse.** A50 attend une décision ; `N` de SPEC-004 §6.2 reste à fixer.
- **Un seul montage, une seule profondeur de bloc, un seul instant de profil.**
- **Aucun coût.** La famille graduée a plus de nœuds que la famille ancrée : cette session
  sépare deux pics, elle ne compare pas des coûts.

**Suite recommandée — S190 : S189-1.** Mesurer l'additivité locale **avec une projection de
pression** dans le véhicule. C'est la seule limite qui menace l'ensemble : ADR-119, la lecture
mécanique de §7.4 et l'explication des trois sessions précédentes reposent toutes sur
l'additivité maille par maille, et la projection est exactement l'opération qui pourrait la
détruire — elle couple tout le bloc à chaque pas. Deux issues, et les deux instruisent : si
l'additivité survit à la projection, ADR-119 vaut pour un solveur réaliste ; si elle tombe,
la borne par la somme reste — elle ne suppose rien — mais l'explication tombe avec elle, et
il faudra le dire. **BILAN-B4-S176** reste le bilan actif et porté.

## Suivi S191 — 2026-09-12 : projection et portée de la somme

Le profil S190 est reçu avec projection discrète : budget 1,371947 %, ou 1,374540 %
avec résidu d'additivité, sous 2 %. Voir [PROJECTION-B4-S191](PROJECTION-B4-S191.md).
La projection à opérateur fixe est linéaire ; sa globalité ne détruit pas l'addition.
La somme des erreurs des axes ne découle de la triangulaire qu'avec le résidu de leur
décomposition : ADR-121 restreint cette revendication d'ADR-119. Les mesures antérieures
ne changent pas. S190-1/S189-1 closes sur le véhicule ; suite S191-1, surface libre2D.