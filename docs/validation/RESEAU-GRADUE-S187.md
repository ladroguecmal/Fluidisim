# S187 — Le réseau d'échantillonnage gradué en profondeur

2026-09-12. S186-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[COMPOSITION-ERREURS-S186](COMPOSITION-ERREURS-S186.md) a trouvé, sans le chercher, que
l'erreur d'un réseau d'échantillonnage isotrope est **intégralement celle de sa tranche la
plus haute** : 2,54 % à `r = 2` pour le bloc entier, et 2,54 % pour la seule tranche
`z = −0,80 m`, quand la tranche du fond ne vaut que 0,11 %. Treize tranches sur quatorze sont
surrésolues, d'un facteur allant jusqu'à vingt-trois.

Un réseau isotrope dépense donc la même densité de nœuds là où le contenu est lisse et là où
il ne l'est pas. Cette session mesure ce que coûte cette uniformité, et ce qu'un réseau
**gradué** rend.

Comme en S183–S186, les conditions sont publiées **avant** la première exécution (§1–§7) ;
les relevés viennent après (§8).

## 1. Les deux questions, dans cet ordre

**Q1 — quelle part de l'erreur de la tranche haute vient de l'axe vertical ?** S186 n'a mesuré
qu'un réseau isotrope : il ne sait pas si les 2,54 % viennent de `x`, de `y` ou de `z`. La
réponse décide de tout le reste. Si elle est majoritairement **verticale**, graduer en `z`
réduit l'erreur *et* le nombre de nœuds. Si elle est majoritairement **horizontale**, aucune
graduation verticale ne l'abaisse — et le gain sera uniquement en nœuds, à erreur inchangée,
ce qui reste le but mais n'est pas la même annonce.

Cette question passe **avant** la graduation, et non après, parce qu'elle dit comment lire la
courbe qui suit.

**Q2 — à erreur égale, combien de nœuds un réseau gradué économise-t-il ?** C'est le livrable :
une courbe **erreur contre nombre de nœuds**, sur laquelle le réseau isotrope et le réseau
gradué sont deux familles de points. Le gain, s'il existe, est la distance horizontale entre
les deux familles à ordonnée égale — et non une amélioration d'erreur à réseau donné.

## 2. Le véhicule, et l'unique référence

Inchangés de S186, et c'est la condition pour que les chiffres se comparent : `examples/support/`,
bloc côté 16, 2744 mailles intérieures, `dx = 0,25 m`, `dt = 10 ms`, 100 pas, `T0 = 1,5 s`,
même montage d'eau, même pas. Mailles intérieures à `z ∈ [−4,05 ; −0,80] m`. Le solveur du
projet reste à B3 (ADR-007 §5).

**La même unique référence** : réseau plein (`r = 1`), source reconstruite à chaque pas,
chargée sans interpolation. Donc le même plancher, **0,386 %**, et les mêmes dénominateurs
`max |S| = 1,540547e-4 m/s²` et `max |u'(T)| = 7,993168e-5 m/s`. Toute ligne de S186 doit
pouvoir être reproduite par ce véhicule, et la réception 3 l'exige.

**La cadence est tenue à `c = 1` pour toute cette session.** S186 a montré que la composition
suit le maximum pour les modes causaux : l'axe temporel est donc séparable, et le mélanger ici
n'ajouterait que du bruit à une question spatiale. La graduation et la cadence se composeront
selon la loi déjà mesurée.

## 3. Ce qui est ajouté au support, et ce qui n'est pas touché

`nodes_per_axis`, `lattice_points` et `scatter` sont utilisés par S184 (coût) et S186
(composition), et S186 a publié deux empreintes qui en dépendent. **Ils ne sont pas
modifiés.** Deux formes sont ajoutées à côté :

1. **réseau à pas par axe** — `[rx, ry, rz]` au lieu d'un `r` unique. Sert Q1.
2. **réseau à indices quelconques par axe** — trois listes d'indices de mailles où les nœuds
   se posent. C'est la forme générale ; la graduation en est un cas.

La forme générale doit **reproduire la forme uniforme en bits** quand on lui donne des indices
uniformes. C'est une réception, pas une intention : une implémentation générale qui ne retrouve
pas son cas particulier est fausse quelque part, et l'écart se lirait ensuite comme un effet de
la graduation.

**Horizontalement le pas reste uniforme.** Un pas horizontal qui dépendrait de `z` ne serait
plus un réseau mais un arbre, et le `scatter` trilinéaire ne s'y applique pas. Cette limite est
assumée et elle borne le gain annoncé.

## 4. La règle de graduation, dérivée avant d'être codée

L'erreur d'une interpolation linéaire sur une maille vaut, au second ordre,

```
e ≈ (1/8) · Σ_axes  h_axe² · |∂²S/∂x_axe²|
```

Égaliser la contribution d'une tranche à l'autre — c'est-à-dire ne plus laisser une seule
tranche fixer le maximum — demande donc

```
h_z(z) · √|∂²_z S(z)|  =  constante
```

Les nœuds se posent alors à **incréments égaux de** `Φ(z) = ∫ √|∂²_z S| dz`, et le nombre de
nœuds `N` fixe la constante. C'est l'équidistribution classique, et elle a ici une conséquence
chiffrable **avant** la mesure : avec le profil de S186, `|∂²_z S| = k_eff_z² · max|S|` passe
de `3,83e-6` à la tranche 2 à `4,84e-5` à la tranche 13, soit un rapport **12,6**. Le pas
vertical profond peut donc valoir **√12,6 ≈ 3,5 fois** celui du haut.

**Le profil n'est pas repris de S186 : il est remesuré par le programme**, au même instant
`T0`, par différences finies sur le réseau plein. Recopier un profil d'une session à l'autre
est exactement ce que ce dépôt a déjà payé (A185) ; et un profil mesuré ici est un profil dont
la provenance est le programme qui l'utilise.

Trois précisions qui font partie du protocole :

- les nœuds sont **accrochés aux indices de mailles** — le réseau ne peut pas se poser entre
  deux centres, puisque la référence, elle, est aux centres. L'accrochage est arrondi au plus
  proche, les doublons sont fusionnés, et le nombre de nœuds réellement obtenu est publié à
  côté du nombre demandé ;
- la tranche haute et la tranche basse **portent un nœud**, sinon les mailles du bord seraient
  extrapolées au lieu d'être interpolées ;
- `Φ` est calculé sur `|∂²_z S|` **agrégé par tranche en maximum**, cohérent avec la métrique
  du dépôt, qui est un maximum et non une moyenne quadratique.

## 5. Ce qui est mesuré

Trois campagnes, toutes à `c = 1`, toutes contre la même référence.

**Q1 — attribution par axe.** Pour `r ∈ {2, 4, 8}` : décimation **verticale seule**
(`[1, 1, r]`), **horizontale seule** (`[r, r, 1]`), et **isotrope** (`[r, r, r]`, qui doit
redonner S186). Erreur globale, erreur de la tranche haute, et nombre de nœuds pour chacune.

**Q2 — courbe iso-erreur.** Deux familles :

- **isotrope**, `r ∈ {1, 2, 4, 8}` — les quatre points de S186 ;
- **gradué**, pas horizontal `rh ∈ {1, 2, 4}` et nombre de nœuds verticaux demandés
  `Nz ∈ {3, 4, 5, 6, 8, 14}`.

Pour chaque point : nœuds, `eS`, `eU`, et l'erreur de la tranche haute. La lecture se fait à
ordonnée égale.

**Un témoin qui peut invalider la règle.** À nombre de nœuds verticaux égal, la graduation
dérivée est comparée à **deux graduations naïves** : le pas uniforme, et un pas géométrique de
raison fixe 2 posé à la main. Si la règle dérivée ne bat ni l'un ni l'autre, c'est la
dérivation de §4 qui est fausse — et il faudra le dire, non l'omettre.

## 6. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** Deux exécutions, mêmes bits ; une empreinte est publiée. Ce
   véhicule ne mesure aucune durée, donc sa sortie entière est un résultat.
2. **Le général reproduit le particulier.** Réseau à indices uniformes = réseau à pas uniforme
   = `scatter` de S186, **bit pour bit**, pour `r ∈ {1, 2, 4, 8}`.
3. **S186 est redonné.** Les trois erreurs isotropes — 2,5401 / 13,6043 / 32,9593 % — et le
   plancher 0,386 % doivent réapparaître, et les empreintes publiées de S185
   (`0x39567a1d4bc2ba4c`) et de S186 (`0x0e743846d4656870`) doivent être **inchangées** après
   l'extension du support. Vérifié par exécution des deux programmes.
4. **Le réseau couvre le bloc.** Aucune maille intérieure n'est extrapolée : sur chaque axe, le
   premier nœud est au plus bas indice intérieur et le dernier au plus haut, ou au-delà.
5. **Tout reste fini**, source et champ, sur toutes les configurations.

## 7. Ce que la mesure ne prouvera pas

- **Aucun seuil de justesse.** A50 attend une décision, pas un chiffre de plus.
- **Un seul montage, une seule profondeur de bloc.** Le profil `∂²_z S` est celui de ce
  contenu à cette profondeur ; un bloc affleurant la surface donnerait un autre profil, et
  probablement un gain plus grand. La **règle** voyage, ses nœuds non.
- **Aucun gain de temps mesuré.** Le nombre de nœuds est un coût géométrique. S184 a établi
  que le coût suit exactement le nombre de nœuds pour la reconstruction — c'est donc une
  conversion légitime, mais elle est citée, pas remesurée, et l'interpolation d'un réseau
  gradué coûte un peu plus par maille qu'un réseau uniforme, ce qui n'est pas chiffré ici.
- **Pas de réseau horizontalement gradué**, ni d'arbre. Voir §3.
- **Le profil est mesuré à un seul instant.** Un contenu dont la structure verticale évoluerait
  fortement demanderait une graduation réactualisée, et le coût de la réactualisation n'est pas
  mesuré.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur (S184 §3).

## 8. Relevés

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example graded_lattice
```

Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.
Bloc 16³, 2744 mailles intérieures, `dx = 0,25 m`, `dt = 10 ms`, 100 pas, cadence tenue à 1.
`max |S| = 1,540547e-4 m/s²`, `max |u'(T)| = 7,993168e-5 m/s` — les valeurs de S186, donc la
même référence et le même plancher, 0,386 %.

**Amendement de protocole, déclaré et non tu.** §5 annonçait un pas horizontal
`rh ∈ {1, 2, 4}`. La première exécution a montré que la famille graduée n'avait alors **aucun
point sous 75 nœuds**, ce qui laissait l'isotrope `r = 8` — 27 nœuds — sans comparaison
possible, et rendait donc la question Q2 inrépondable à son extrémité grossière. `rh = 8` a
été ajouté. C'est une extension du balayage, pas un affaiblissement du critère, et c'est écrit
ici plutôt que dans un protocole réécrit après coup.

### 8.1 Réceptions

1. **Reproductibilité.** Deux exécutions, `diff` strict identique. Empreinte
   `0x6cf13183b4a240df`. Aucune durée n'est mesurée : la sortie entière est un résultat.
2. **Le général reproduit le particulier.** Pour `r ∈ {1, 2, 4, 8}`, le réseau à indices
   uniformes rend **les mêmes points** que `lattice_points` et **le même champ en bits** que
   `scatter`. Ce n'est pas fortuit : les poids valent `(i − idx)/span` d'un côté et
   `((i−1) mod r)·(1/r)` de l'autre, et `1/r` est exact pour une puissance de deux.
3. **S186 est redonné**, et le support historique est intact : `cadence_error` rend
   `0x39567a1d4bc2ba4c`, la sortie **entière** de `composed_error` est identique au `diff`
   strict après le déplacement de la reconstruction dans `support/source_snapshots.rs`, et les
   trois erreurs isotropes 2,5401 / 13,6043 / 32,9593 % réapparaissent ici à la décimale.
4. **Les réseaux verticaux couvrent le bloc** : premier nœud à l'indice 1, dernier à 14 ou
   au-delà, sur toutes les variantes ancrées.
5. **Le plancher est celui de S186**, 0,386 %, puisque c'est la même référence — il n'est pas
   remesuré, il est hérité par identité des dénominateurs (réception 3).
6. **Tout reste fini.**

### 8.2 Q1 — d'où vient l'erreur, et une compensation de plus

Décimation d'un axe à la fois, `c = 1`, contre la même référence.

| r | verticale seule | horizontale seule | isotrope |
|---:|---|---|---|
| 2 | 2,6635 % (1568 nœuds) | 1,6947 % (896) | **2,5401 %** (512) |
| 4 | 15,4059 % (980) | 6,1256 % (350) | **13,6043 %** (125) |
| 8 | 45,5067 % (588) | 13,4919 % (126) | **32,9593 %** (27) |

**L'axe vertical domine** : 1,57, 2,51 puis 3,37 fois l'horizontal, et l'écart croît avec la
décimation. La réponse à Q1 est donc que graduer en `z` est le bon levier — la suite le
confirmera autrement qu'attendu.

Et un fait qui n'était pas cherché : **l'isotrope est sous l'axe vertical seul aux trois `r`**
— 2,54 contre 2,66 ; 13,60 contre 15,41 ; 32,96 contre 45,51. Décimer **aussi**
horizontalement rend le champ **plus juste** que décimer verticalement seul. C'est la famille
de **A229** transposée aux axes d'espace : deux approximations sur le même contenu se
compensent partiellement. Le piège de réglage est le même, et il est ici interne à une seule
grandeur.

### 8.3 Le profil vertical, remesuré

`|∂²_z S|` par tranche, à `T0`, en maximum sur la tranche. Les deux tranches de bord
empruntent leur valeur à leur voisine : la maille fantôme existe dans le bloc mais pas dans le
réseau plein, et recopier la voisine vaut mieux qu'inventer une valeur.

| k | z (m) | \|∂²_z S\| | √ | pas relatif `h(z)/h_haut` |
|---:|---:|---:|---:|---:|
| 2 | −3,80 | 3,8310e-6 | 1,9573e-3 | 3,554 |
| 4 | −3,30 | 4,9922e-6 | 2,2343e-3 | 3,113 |
| 6 | −2,80 | 6,7505e-6 | 2,5982e-3 | 2,677 |
| 8 | −2,30 | 9,6819e-6 | 3,1116e-3 | 2,235 |
| 10 | −1,80 | 1,5275e-5 | 3,9084e-3 | 1,780 |
| 12 | −1,30 | 3,0273e-5 | 5,5021e-3 | 1,264 |
| 13 | −1,05 | 4,8379e-5 | 6,9555e-3 | 1,000 |

Rapport extrême **12,63**, donc un pas vertical profond jusqu'à **3,55 fois** celui du haut.
Le protocole avait dérivé 12,6 et 3,5 depuis les `k_eff` de S186 : **le profil remesuré par le
programme qui l'utilise retombe sur la dérivation faite avant lui.** C'est le seul usage de ce
profil — il n'a pas été recopié d'une session à l'autre (A185).

### 8.4 Le résultat qui renverse la session : l'ancrage, pas la graduation

Le réseau historique — celui de S184 et S186 — pose son dernier nœud **hors du bloc** :
`nodes_per_axis` déborde, et à `r = 8` le dernier nœud vertical tombe à l'indice **17**, soit
`z = −0,05 m`, quand les mailles intérieures s'arrêtent à l'indice 14, `z = −0,80 m`. La
maille du haut est donc interpolée sur une portée de 2 m au lieu d'être échantillonnée.

À **nombre de nœuds verticaux égal**, pas horizontal 2 :

| Nz | débordante (historique) | ancrée uniforme | ancrée dérivée | gain de l'ancrage | gain de la graduation |
|---:|---:|---:|---:|---:|---:|
| 3 | **41,2153 %** | 6,7887 % | 6,5503 % | **× 6,07** | × 1,04 |
| 5 | **13,6044 %** | 2,5458 % | **1,7919 %** | **× 5,34** | × 1,42 |
| 8 | **2,5401 %** | 1,6947 % | 1,6947 % | **× 1,50** | × 1,00 |
| 14 | 1,6947 % | 1,6947 % | 1,6947 % | × 1,00 | × 1,00 |

**L'ancrage vaut jusqu'à un facteur six ; la graduation, au mieux 1,42.** Et l'ancrage est
gratuit : il ne change pas un seul nœud, seulement l'endroit où on le pose.

Le mécanisme est celui que S186 avait mis au jour sans en tirer cette conséquence : la
métrique est un **maximum**, et ce maximum vit **intégralement** sur la tranche la plus haute
(S186 §8.3). Un nœud posé sur cette tranche supprime le terme dominant — et les colonnes
« erreur de tranche haute » de la sortie le montrent directement : 41,2 % pour la débordante,
1,69 % pour toutes les ancrées, c'est-à-dire l'erreur horizontale seule, c'est-à-dire plus
aucune contribution verticale en haut.

**Contre-épreuve horizontale, et elle était nécessaire.** Si « ancrer est mieux » était une
règle générale, elle vaudrait aussi en `x` et `y`. Vertical tenu plein :

| nœuds par axe horizontal | débordante | ancrée uniforme | écart |
|---:|---:|---:|---:|
| 3 | 13,4919 % | 13,1488 % | −2,5 % |
| 5 | 6,1256 % | 3,6805 % | −40 % |
| 8 | 1,6947 % | 1,7160 % | **+1,3 %** |

Non monotone, sans facteur six, et une fois **défavorable**. Donc ce n'est pas l'ancrage en
soi : c'est **poser un nœud là où vit le maximum de la métrique**. Pour une source qui décroît
avec la profondeur, cet endroit est la frontière haute du domaine, et l'ancrage en est la
forme pratique. C'est ce que fixe [ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md).

### 8.5 Q2 — la courbe, le plancher, et le point d'arrêt

Famille graduée, pas horizontal `rh`, nœuds verticaux demandés `Nz`. `eU` en % de
`max |u'(T)|`.

| rh | Nz demandé | forme | nœuds | eU % | eU tranche haute % | indices z |
|---:|---:|---|---:|---:|---:|---|
| 1 | 5 | 14×14×5 | 980 | 1,7164 | 0,0001 | [1, 6, 10, 12, 14] |
| 1 | 8 | 14×14×8 | 1568 | 0,6393 | 0,0000 | [1, 4, 7, 9, 11, 12, 13, 14] |
| 1 | 14 | 14×14×12 | 2352 | 0,2951 | 0,0000 | [1, 3, 4, 6, …, 14] |
| 2 | 4 | 8×8×4 | 256 | 3,4373 | 1,6947 | [1, 8, 12, 14] |
| 2 | 5 | 8×8×5 | 320 | **1,7919** | 1,6947 | [1, 6, 10, 12, 14] |
| 2 | 6 | 8×8×6 | 384 | **1,6947** | 1,6947 | [1, 5, 9, 11, 13, 14] |
| 2 | 8 | 8×8×8 | 512 | 1,6947 | 1,6947 | — |
| 4 | 4 | 5×5×4 | 100 | **6,1256** | 6,1256 | [1, 8, 12, 14] |
| 8 | 3 | 3×3×3 | **27** | **13,4919** | 13,4919 | [1, 10, 14] |

**Le plancher est l'erreur de l'axe le plus grossier, et il s'atteint vite.** À `rh = 2` le
plancher est 1,6947 % — l'erreur horizontale seule de §8.2 — atteint dès **six** nœuds
verticaux gradués ; `Nz = 8` et `Nz = 14` ne changent plus rien. À `rh = 4` le plancher est
6,1256 %, atteint à quatre nœuds. À `rh = 8`, 13,4919 %, atteint dès trois. **Raffiner un axe
au-delà du point où sa contribution passe sous celle de l'axe le plus grossier ne paie rien**,
et c'est la règle 3 d'ADR-118.

Les lignes `rh = 1` montrent la graduation en train de faire son travail : l'erreur de la
tranche haute tombe à **0,0000** et le maximum se déplace vers le milieu du bloc. C'est
exactement l'effet visé par l'équidistribution — répartir au lieu de concentrer.

**Lectures à ordonnée égale**, qui sont le livrable annoncé en §1 :

| cible isotrope | nœuds | eU % | meilleur gradué | nœuds | eU % | gain |
|---|---:|---:|---|---:|---:|---:|
| `r = 2` | 512 | 2,5401 | `rh=2, Nz=5` | 320 | 1,7919 | **−37,5 %** de nœuds, et −29 % d'erreur |
| `r = 4` | 125 | 13,6043 | `rh=8, Nz=3` | 27 | 13,4919 | **−78,4 %** de nœuds |
| `r = 8` | 27 | 32,9593 | `rh=8, Nz=3` | 27 | 13,4919 | nœuds identiques, erreur **÷ 2,44** |

### 8.6 Les deux témoins : la règle dérivée est validée, sans être spectaculaire

À nœuds égaux, pas horizontal 2 :

| nœuds | dérivée | uniforme | géométrique de raison 2 |
|---:|---:|---:|---:|
| 192 (3 en z) | 6,5503 % | 6,7887 % | 6,5503 % |
| 320 (5 en z) | **1,7919 %** | 2,5458 % | 3,4373 % |
| 384 (6 en z) | **1,6947 %** | 2,5458 % | 3,4373 % |
| 512 (8 en z) | 1,6947 % | 1,6947 % | 3,4373 % |

La dérivation gagne **là où elle sert** — entre quatre et six nœuds — d'un facteur jusqu'à
1,42 contre le pas uniforme et 1,92 contre le pas géométrique. À trois nœuds elle coïncide
avec le géométrique ; à huit, le pas uniforme sature aussi et l'écart disparaît. Le pas
géométrique de raison 2, lui, est **toujours** battu dès cinq nœuds : une graduation plausible
posée à la main n'est pas une graduation dérivée du contenu, et l'écart se mesure.

La règle de §4 n'est donc pas fausse — le témoin déclaré ne l'a pas renversée — mais elle
pèse cinq fois moins que l'ancrage, et c'est l'inverse de ce que cette session attendait.

## 9. Ce que la session conclut, et ce qu'elle laisse ouvert

**Conclu**, et acté par [ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md).

1. **Un réseau d'échantillonnage s'ancre sur les frontières de son domaine.** Gratuit, et
   jusqu'à un facteur **6** sur l'erreur. Le réseau du dépôt ne le faisait pas.
2. **La raison n'est pas l'ancrage mais le maximum** : la métrique est un maximum, il vit sur
   la tranche haute, et un nœud posé là supprime le terme dominant. La contre-épreuve
   horizontale le prouve par son absence d'effet (−2,5 %, −40 %, **+1,3 %**).
3. **La graduation dérivée vaut ensuite 1,4×**, et bat les deux témoins naïfs à nœuds égaux.
4. **L'axe vertical domine l'horizontal** d'un facteur 1,6 à 3,4, et l'écart croît avec la
   décimation.
5. **Raffiner un axe sature** sur l'axe le plus grossier : six nœuds verticaux suffisent à
   `rh = 2`, quatre à `rh = 4`, trois à `rh = 8`.
6. **Gains à erreur égale** : −37,5 % de nœuds et −29 % d'erreur contre l'isotrope `r = 2` ;
   −78,4 % contre `r = 4` ; erreur divisée par 2,44 à nœuds identiques contre `r = 8`.
7. **Les axes d'espace se compensent partiellement**, comme les axes espace/temps de A229 :
   l'isotrope est sous le vertical seul aux trois ratios.

**Non conclu, et pas contourné.**

- **Aucun seuil de justesse.** A50 attend une décision, et ADR-118 ne la prend pas : il dit où
  poser les nœuds, pas combien en payer.
- **La loi de composition de S186 n'est pas rejouée sur un réseau ancré.** Le maximum pour les
  modes causaux a été établi sur le réseau débordant ; les magnitudes changent, la loi n'est
  pas revérifiée. C'est le premier candidat pour la suite.
- **Le surcoût d'interpolation d'un réseau gradué n'est pas chiffré.** Les poids ne sont plus
  constants ; S184 n'a mesuré que le réseau uniforme, et le gain en nœuds est converti en gain
  de coût par citation, pas par mesure.
- **Un seul montage, une seule profondeur de bloc, un seul instant de profil.** Un bloc
  affleurant la surface donnerait un profil plus raide et probablement un gain plus grand ;
  la règle voyage, ses nœuds non.
- **Pas de réseau horizontalement gradué**, ni d'arbre.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur.

**Suite recommandée — S188 : S187-1.** Rejouer la composition de S186 sur un réseau **ancré**.
Deux raisons, et la seconde est la vraie : les magnitudes spatiales ont changé d'un facteur
allant jusqu'à six, donc la parité entre axe spatial et axe temporel — qui est la règle de
dimensionnement d'ADR-118 et de S186 — se déplace entièrement ; et la loi elle-même, le
maximum pour les modes causaux, a été mesurée sur un réseau dont on sait maintenant qu'il
concentrait son erreur sur une seule tranche. Une loi de composition mesurée sur une erreur
concentrée n'est pas nécessairement celle d'une erreur répartie, et §8.5 montre que la
graduation **répartit** — l'erreur de tranche haute tombe à zéro et le maximum se déplace.
