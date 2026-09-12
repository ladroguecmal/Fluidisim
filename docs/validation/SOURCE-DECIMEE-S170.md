# S170 — Source interpolée sur réseau indépendant

## 1. Protocole déclaré avant mesure

Suite S169-1/A50, SPEC-004 §6.2 et PLAN-BENCHMARK §B4. Véhicule Saint-Venant1D,
Q onde simple a0,05 centre55 m largeur8 m **figée**, T onde évolutive correspondante,
fenêtre[30,90], durée6 s. Source physique S=-∂xF(Q), connue analytiquement S167.
S exacte moyenne=[F(Q_gauche)-F(Q_droite)]/dx, reçue S168.

Comparer S exacte moyenne, S omise, et S ponctuelle échantillonnée aux nœuds d'un
réseau de pas H=1/2/4/8/16 m, origine0 ou H/2. Interpolation linéaire entre nœuds,
**intégrée exactement dans chaque cellule** pour fournir sa moyenne. Ne pas ajouter
une erreur de quadrature fine à l'erreur du réseau source. Pas H indépendant de dx.
Solveur N120/240/480 sur120 m (dx1/0,5/0,25), pas0,2 dx/sqrt(g) ; demi-pas N240.
Les ratios H/dx incluent4 mais ne sont pas fixés à4. Paramètres de banc, pas profils.

Q et ses flux restent évalués exactement en moyennes. Fantômes T exacts aux étages
pour isoler la source : ce montage ne teste pas l'échantillonnage simultané de Q et S,
ni le bord autonome. La source est figée, sans interpolation temporelle.

Mesurer erreur maximale hauteur/débit contre T moyen (échelle0,05 et0,05 sqrt(g)),
écart de champ au témoin S exacte sur mêmes mailles, norme d'erreur de source normalisée
par son maximum exact local (composantes séparées), bilan de volume et prédiction :

```
volume(t)-volume(0)-flux_total_cumule(t) = t · somme_i[(S_H-S_exact)_h dx]
```

La source erronée est connue et constante en temps. Vérifier cette identité **signée**
sur la trajectoire et non seulement ses maxima. Un bilan corrigé de cette injection peut
fermer sur un champ faux : conserver le bilan physique non corrigé dans le rapport.
Ne pas confondre écart à l'analytique et effet de la source interpolée, comparer les champs
avant les maxima. Seuil instrumental1e-10 pour bilans, aucun seuil physique B4 choisi.

Tests : intégration de l'interpolant sur fonction affine, témoin source exacte, omission,
prédiction du volume, variation H à dx fixé et dx à H fixé. La translation du réseau
éprouve l'aliasing ; ne pas déduire une règle universelle d'un seul alignement.
Le nombre de nœuds est un coût géométrique, pas un gain de temps runtime mesuré.

## 2. Résultats

### 2.1 Effet du réseau à solveur fixé

N240, dx0,5 m, pas nominal. E=max erreur hauteur/0,05 contre T exact ; D=max différence
de champ/0,05 par rapport à S exacte aux mêmes mailles. Source h normalisée par max|S_h|
des moyennes exactes. Volume relatif au volume initial (~60 m² par unité de largeur).

| H (m) | Décalage/H | Erreur source h | E | D | Défaut volume |
|---|---:|---:|---:|---:|---:|
|Exacte|—|0|0,135612|0|9,234265e-16|
|Omise|—|1|0,997949|0,989676|8,881982e-7|
|1|0|0,006100|0,137232|0,003768|4,254010e-8|
|1|0,5|0,006169|0,137231|0,003748|4,435084e-8|
|2|0|0,033642|0,142056|0,014667|1,664352e-7|
|2|0,5|0,031995|0,142037|0,015099|1,954109e-7|
|4|0|0,133058|0,160693|0,057859|1,076957e-6|
|4|0,5|0,133058|0,160684|0,059133|6,124864e-7|
|8|0|0,484443|0,302221|0,246223|6,309842e-5|
|8|0,5|0,335303|0,210372|0,172182|4,940147e-5|
|16|0|0,714807|0,480192|0,444852|6,024954e-3|
|16|0,5|1,129711|1,092567|1,067309|6,398198e-3|

Le réseau16 m décalé donne plus d'erreur de champ que S omise : l'interpolation n'est
pas un simple affaiblissement monotone. Les signes d'injection massique changent avec
la phase : à H8, +6,384401e-4 contre−4,998521e-4 m²/s ; à H16, −0,06096147 contre
+0,06473801 m²/s. Ce sont des défauts numériques dans le banc, pas des volumes gameplay.

### 2.2 Séparer les résolutions

Origine0, effet D du réseau H8 m :0,239621 →0,246223 →0,250013 aux N120/240/480.
Le solveur plus fin ne supprime pas ce défaut. L'injection intégrée reste6,384401e-4 m²/s
et le défaut de volume6,309842e-5, car l'intégrale du même interpolant ne change pas
avec sa partition en cellules. Au demi-pas N240, D=0,246241 : le défaut n'est pas temporel.

Avec ratio H/dx fixé à4, les mêmes grilles donnent H4/2/1 m et D=0,055454/0,014667/0,003862.
**Un ratio de décimation ne décrit donc pas à lui seul la précision** : la taille physique
du réseau devant la variation de S compte. Le cas n'a pas de longueur d'onde unique ;
ne pas remplacer arbitrairement la largeur gaussienne8 m par λ_cut.
Les nombres de nœuds déclarés couvrent le canal étendu nécessaire au réseau, non seulement
la fenêtre locale ; ils ne prouvent ni un gain de temps, ni le facteur64 en3D.

### 2.3 Bilan prédit, pas bilan corrigé en cachette

À chaque pas, le défaut **signé** est comparé à t somme(S_H−S_exact)_h dx.
Écart maximal relatif<=2,20e-15 sur48 évolutions. Le flux de chaque calcul est celui
effectivement utilisé ; ni la prédiction ni le témoin ne modifient les états évolués.
La source interpolée injecte un terme volumique artificiel, identifié séparément de
l'erreur de transport ; le bilan physique brut est conservé dans la table.

L'omission a ici un petit défaut de volume malgré une grande erreur de champ : la
source exacte possède une petite intégrale nette, mais une structure locale importante.
Ni volume proche de zéro, ni conservation après ajout de la source ne reçoit cette structure.
Courant maximal0,217062 ; aucun état invalide ou écrêtage.

## 3. Réception et limites

Depuis `code/` :

```
cargo test -p water-core --example source_decimee
cargo run -p water-core --release --example source_decimee
```

Quatre nouveaux tests reçus : intégrale de l'interpolant affine à travers plusieurs
segments, omission et prédiction, raffinement source à solveur fixé, injection inchangée
au raffinement du solveur à source fixée. Campagne quatre configurations de solveur,
douze sources chacune=48 évolutions. La première lecture CSV a échoué parce que H et h
sont confondus par PowerShell ; colonne renommée source_spacing, campagne relancée.
Supports et bibliothèques inchangés ; tests précédents non rejoués, S165–S169 reçus
S169 et workspace299/cinq ignorés reçu S163. Aucun coût runtime reçu ni dépendance ajoutée.

**S169-1 réalisée, A50 reste partielle.** La sensibilité à la source décimée est mesurée
sur ce véhicule1D, pour une source spatiale figée exacte connue. L'interface réelle
échantillonne aussi les champs du fond ; cette interpolation conjointe et les dérivées
3D/temps ne sont pas reçues. Pas de seuil is_smooth_at, pas d'ADR ni profil adopté.
A216/A217 inchangées, I-01/04/12/14/15 inchangés ; B4 général non reçu.

**A225 / suite S171 : S170-1**, comparer à une source obtenue par différence d'un **flux
reconstruit commun aux faces**, pour que son intégrale télescope. Mesurer si cela corrige
l'injection tout en gardant une erreur locale ; ne pas présenter une source conservative
comme précise par construction. Garder source exacte, interpolation directe et omission
comme témoins, phase du réseau et raffinement indépendants. Aucun recalage global uniforme
de S pour cacher son intégrale : l'amélioration doit venir d'une discrétisation explicite.

**Suivi S171 : S170-1 réalisée sur véhicule1D**, voir
[SOURCE-FLUX-PARTAGES-S171](SOURCE-FLUX-PARTAGES-S171.md). La différence de flux
partagés ferme le bilan physique si ses bornes portent les flux exacts ; précision
locale encore sensible àH et àla phase. A225 traitée dans ce périmètre, A50 partielle.

**Suivi S184 — 2026-09-12 : le gain de temps que ce document refusait d'annoncer est mesuré.**
§2.2 disait, et avait raison de le dire, que le nombre de nœuds est *« un coût géométrique, pas
un gain de temps runtime mesuré »* et qu'il ne prouvait *« ni un gain de temps, ni le facteur 64
en 3D »*. [CONSOMMATION-S184](CONSOMMATION-S184.md) le chiffre sur le fournisseur réel, en 3D :
le gain **est exactement** le rapport des nombres de nœuds, à l'interpolation près (10–40 ns par
maille). Ce que S170 ne pouvait pas savoir, et qui change la lecture : la décimation spatiale est
plafonnée à `r = 2` par le **contenu** de la source — la coupure de pression donne
`λ_min = 1,081 m`, soit 2,16 points par longueur d'onde à `H = 0,5 m`. La cadence temporelle, en
revanche, divise exactement par `c` et porte sur un contenu lent. **L'avertissement de §2.2 —
un ratio ne décrit pas à lui seul la précision — reste entier** : S184 mesure le temps, pas
l'erreur, et ne recommande aucun `H`.


**Suivi S186 — 2026-09-12 : l'avertissement de §2.2 est désormais chiffré, et il avait
raison.** §2.2 disait qu'un ratio de décimation ne décrit pas à lui seul la précision, parce
que la taille physique du réseau devant la variation de `S` compte.
[COMPOSITION-ERREURS-S186](COMPOSITION-ERREURS-S186.md) le mesure en 3D sur le fournisseur
réel et le confirme deux fois :

- **le ratio ne suffit pas**, et la grandeur qui gouverne est `h · k_eff` où `k_eff` est le
  contenu **réellement présent** là où le consommateur se trouve — 5 à 16 fois plus lisse que
  la coupure de la recette, parce qu'un mode profond décroît en `exp(k z)` (§8.2, **A230**) ;
- **la précision n'est pas répartie** : l'erreur globale est intégralement celle de la tranche
  la plus haute du bloc, les treize autres sur quatorze étant surrésolues (§8.3).

Et la composition avec l'erreur temporelle, que ce document ne pouvait pas mesurer sur une
source figée, suit le **maximum** pour un consommateur causal : les deux erreurs ne
s'additionnent pas (§8.5, **L266**). L'addition que §2.2 rendait douteuse est effectivement
fausse — elle surestime jusqu'à 1,9 fois — mais elle reste une enveloppe sûre.

**Suivi S187 — 2026-09-12 : le ratio ne dit rien du placement, et le placement pèse plus.**
Le §2.2 de ce document avertissait qu'un ratio de décimation ne décrit pas à lui seul la
précision. [RESEAU-GRADUE-S187](RESEAU-GRADUE-S187.md) en donne la forme la plus nette : à
nombre de nœuds **identique**, déplacer le dernier nœud de l'extérieur du domaine vers sa
maille de bord fait passer l'erreur de **41,2 % à 6,8 %**. Ce n'est donc pas seulement la
taille physique du réseau qui compte, comme §2.2 le disait, mais **où ses nœuds se posent** —
et quand la métrique est un maximum, là où vit ce maximum. Voir **A231**, **L267** et
[ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md).
