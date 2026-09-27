# ADR-212 — La bande étroite en profondeur : une hauteur eulérienne sous les particules

- **Statut : actée**, S412, 2026-09-27 ; autonomie technique (S71), sur la décision de l'utilisateur
  [ADR-211](ADR-211-les-trucages-retenus.md) D1 (C6c = la bande étroite en profondeur, avant C7). Campagne du solveur volumique
  3D ([ADR-207](ADR-207-la-campagne-du-solveur-volumique-3d.md)).
- **Précise** A1 de la conception de la campagne (S384 §4.1 : « des particules APIC dans une bande **sous la surface** ») :
  la bande de S398–S410 est pleine hauteur ; elle devient étroite.
- **Réemploie** la zone des colonnes ([RACCORD-3D-S398](../validation/RACCORD-3D-S398.md), S398–S407) et la bascule
  ([BASCULE-S408](../validation/BASCULE-S408.md), S408–S410).
- **Source** : Ferstl, Ando, Wojtan, Westermann, Thuerey, « Narrow Band FLIP for Liquid Simulations », *Eurographics* 2016 —
  des particules dans une bande sous la surface, le reste du volume sur une grille ; leur masse n'y est pas exacte, la nôtre doit
  l'être.

## 1. Ce qui est constaté

Sur la vague de Chen (S410), une colonne de la bande porte ses particules **du fond à la surface** : 20 mailles d'eau pour 4 qui
bougent en particules ; sur B10, 64 pour 6. L'utilisateur (S411) : *« est il intéréssant de simuler les billes en dessous en
profondeur, car on ne les voit pas et ne sont pas en grand mouvement »*. Et la mesure (S410 §6.3) : **chaque conversion d'une
colonne entière au sommet de la crête perturbe le déferlement**.

La lecture du code (S412) montre que la zone des colonnes a déjà tout ce qu'il faut pour porter l'eau profonde sous des
particules — des vitesses eulériennes advectées au pied de la caractéristique (S398), une hauteur transportée par les débits
mouillés en `f64` (S408), des particules **virtuelles** qui complètent la reconstruction près d'elle (S399), un échange à masse
exacte à sa frontière (soldes, absorption, pose à la face ; S399–S407). Il lui manque d'être **tournée à la verticale**.

## 2. Décisions

**D1 — Une hauteur eulérienne `β` par colonne, sous laquelle l'eau est portée par la grille.** Dans chaque colonne, l'eau sous
`β` est **eulérienne** — mailles pleines, vitesses de la grille gardées et advectées, volume `β·dx²` exact ; au-dessus, s'il y en
a, des **particules**. Un seul paramètre couvre les trois états :

| état | `β` | au-dessus |
|---|---|---|
| **colonne** (la zone, S398) | la surface libre `η` | de l'air |
| **bande étroite** (C6c) | sous la surface la plus basse de la colonne, de `k` mailles | des particules, jusqu'à la surface et au-delà (jets) |
| **bande pleine** (S408–S410) | 0 | des particules du fond à la surface |

La bande pleine est le cas `β` = 0 : **sans `β`, S398–S410 au bit**.

**D2 — Sous `β`, la machinerie de la zone, telle quelle.** Les faces entièrement sous `β` prennent la vitesse advectée
(`columns_advect`) ; `β` se transporte par les débits mouillés jusqu'à `β`, en `f64`, reste compris (`columns_transport`) ; la
reconstruction de `φ` au-dessus de `β` compte les **particules virtuelles** de la part eulérienne — la sienne et celles des
voisines — (`virtual_column_sums`, étendu de `[0, η]` à `[0, β]`), pour que le bas de la bande ne soit pas lu comme une surface ;
les mailles sous `β` sont de l'eau pour la pression.

**D3 — La face à `β` est une frontière, comme la face latérale de S399.** Elle appartient à la part eulérienne (S406) ; le volume
qui la traverse pendant le pas charge un **solde vertical** par colonne : montant, la bande doit recevoir des particules — posées
**à la face** (S407), au sous-réseau le plus libre, à la vitesse de la grille ; descendant, elle en doit — la particule la plus
proche de la face est retirée. Une particule advectée sous `β` est **absorbée** et paie le solde. Chaque geste vaut `dx³/8`
exactement : **la masse se compte au bit**, comme à la frontière latérale.

**D4 — Le critère place `β`.** La bascule de S408 désigne toujours les colonnes de la bande (non convertibles, corps, pente,
dilatation) avec le **maintien de 0,3 s** retenu par R34 ; dans une colonne de la bande, `β` se place à **`k` mailles sous la
surface la plus basse** de la colonne (le fond d'une cavité, le dessous d'une lèvre, pas seulement le haut), avec une hystérésis :
`β` **descend** dès que la surface s'en approche à moins de `k` mailles, ne **remonte** qu'au-delà de `k + h`. Descendre ensemence
la tranche au réseau nominal ; remonter absorbe les particules de la tranche ; l'écart à la masse d'une tranche va à la
**réserve** de S408. `k` et `h` sont des **allocations** du profil (I-16) ; *à calibrer* — *prédiction* : `k` = 4 (le noyau de
reconstruction de deux mailles, plus deux de marge), `h` = 2.

**D5 — C7 porte cette version.** La carte reproduit la référence (ADR-175 D1) : ce qu'on porte sur la carte est la bande
étroite, non la bande pleine.

## 3. Critères « reçu si » — C6c

1. **Sans `β`** (bande pleine), au bit : les essais de S398 à S410, et les bancs B10 (S408) et `apic3d_deferlement` (S410) au
   chiffre près.
2. **Repos** : un bassin dont les colonnes sont en bande étroite (`β` à `k` mailles sous la surface) reste au repos — les critères
   de S398 ; volume exact (≤ 10⁻⁹) ; densité au bas de la bande dans la tolérance de S407 (8 ± 0,4 particules par maille).
3. **Ballottement** traversant des colonnes en bande étroite : période et amortissement à un point d'APIC seul, sur 30 s (le
   critère 4 de S399).
4. **B10** : pincement à un pas d'APIC seul (comme S408) ; particules **÷ 3 au moins** contre S408 (29 120) ; volume exact.
5. **La vague de Chen** (S410) : retournement et impact comme la bande pleine au maintien de 0,3 s, à la précision de l'image —
   **jugé sur la planche** (R35, à la demande de l'utilisateur : la qualité se juge à l'image, S410) ; particules et temps de
   calcul contre APIC seul publiés ; aucune hésitation (retours rapides) ajoutée par `β`.
6. Suite entière, zéro avertissement.

**Arrêt** : si la densité au bas de la bande dérive (le risque d'A316, huit sessions sur la frontière latérale), la publier et
ne rien rendre défaut qui ne soit éprouvé.

## 4. Découpage

| | session | reçu si |
|---|---|---|
| **C6c-1** | `β` par colonne, fixe : étiquettes, faces advectées sous `β`, transport de `β`, particules virtuelles jusqu'à `β`, solde vertical, absorption et pose à la face | 1, 2, 3 (avec `β` posé à la main) |
| **C6c-2** | `β` placé par le critère (D4) : descente, remontée, hystérésis, réserve | 4, 5, 6 |

## 5. Ce que la décision ne fait pas

- Pas de subdivision de la maille (4.10 le garde) ; pas de particules diffuses (C9) ; pas de surface continue pour l'image
  (ADR-211 D2, avant C10).
- Le coût de la projection ne baisse pas : toutes les mailles restent dans une seule projection ; ce sont les **particules** qui
  diminuent (÷ 4 à 10, estimé, [TRUCAGES-TEMPS-REEL-S411](../registres/TRUCAGES-TEMPS-REEL-S411.md) §3). Les mailles hautes sous
  la bande ([ADR-208](ADR-208-la-colonne-graduee.md)) sont une étape ultérieure, non décidée ici.

## Note datée du 2026-09-27 (S413) — l'implémentation de D2 et D3

D2 disait « `β` se transporte par les débits mouillés jusqu'à `β`, en `f64` » et D3 « le volume qui traverse [la face à `β`]
charge un solde vertical ». **Construit autrement, plus simple, à masse toujours exacte** : `β` est **arrondi à une face de
maille** (`set_band_floor`) et **ne bouge pas** entre deux placements ; la part eulérienne est un **contenant de mailles pleines**
— tout débit qui y entre ou en sort (d'une part eulérienne voisine, d'une colonne de la zone, ou des particules d'à côté) charge
le **solde vertical** de sa colonne, réglé par des particules posées ou retirées juste au-dessus de `β`. Aucun flux vertical n'est
estimé, aucune maille n'est mixte ; chaque volume est compté des deux côtés. Pour un écoulement à divergence nulle, c'est ce que
ferait la vitesse verticale à la face : le solde en est la version exacte. C6c-2 déplacera `β` d'une maille entière à la fois.
[Preuve](../validation/BANDE-ETROITE-S413.md).

## Note datée du 2026-09-27 (S414) — D4 construit, et deux idées de l'utilisateur

D4 est construit ([preuve](../validation/BANDE-ETROITE-S413.md) §5) : `floor_cells`, `floor_hysteresis`, la cible sous la première
maille non-eau depuis le bas. **La prédiction du corps**, demandée par l'utilisateur (*« si un évènement va aller en profondeur
mettre le fond a bonne distance »*), est une option (`floor_prediction`) : à horizon court (0,05 s), B10 se pince au pas même
d'APIC seul ; à 0,2 s, deux pas trop tôt. **Le fond qui suit l'écoulement** (*« malaxable en fonction du courant, les particules
peuvent naitres et disparaitre en fonction de leurs vitesse »*) est proposé comme C6c-3, non décidé ici.

## Note datée du 2026-09-27 (S415) — le fond qui suit l'écoulement (C6c-3)

Construit, éteint par défaut ([preuve](../validation/BANDE-ETROITE-S413.md) §6) : la vorticité, la vitesse, la part de rotation
(critère Q sans dimension) placent aussi la bande et son fond. Mesuré : sur un tourbillon enfoui, la grille perd 2,5 fois l'énergie
d'APIC ; la vorticité garde le cœur ; **la vitesse** — l'idée de l'utilisateur — rend l'énergie d'APIC seul avec 2,4 à 3 fois moins
de particules ; mais sous une houle raide, vorticité absolue et vitesse prennent tout. **La voie retenue pour la suite** : la vitesse
**propre de δ, relative à B** (ADR-198), gratuite sous la houle — à construire avec la bande sur la production (C7, C10).
