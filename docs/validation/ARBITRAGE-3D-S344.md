# Deux domaines δ 3D se disputent un budget — S344

2026-09-24. **Porte A**, premier critère de §3 bis : *plusieurs candidats réels se disputent un budget*, sur des
domaines 3D ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D6). Chemin de la v1
([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D4). L'ordonnanceur est celui de S278
([ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md), [ADR-170](../adr/ADR-170-les-trois-poids-sont-bornes.md),
[ADR-171](../adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md)) ; jusqu'ici il n'avait arbitré qu'une
bande δ 2D ([S279](ORDONNANCEUR-S279.md)).

## Reproduire

- Commit `efab327d` ou plus récent ; machine de référence ; `viewer`.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-parts` — parts d'écran le long du
  trajet, toutes les 0,25 s.
- `… -- --delta3d-arbitrage` — lignes `DELTA3D_ARBITRAGE_S344` : transitions, bilans ; une vingtaine de secondes.
  Un premier passage après une carte au repos montre le démarrage à froid (§3).

## 1. Le montage

- **Deux domaines de production** : la scène de la porte B (`Config::review`, 120 × 112 × 28 mailles de 25 cm,
  32 cycles), posée en A et en B, à 60 m l'un de l'autre en `x`. Deux `Step3`, deux états.
- **Une caméra qui longe la côte**, comme un joueur : devant A (0–5 s), vers B à 15 m/s (5–9 s), devant B (9–13 s),
  retour (13–17 s), devant A (17–20 s) ; pose et regard de la scène de la porte B.
- **Chaque image** (16,667 ms) : chaque domaine soumissionne `W_perception` = sa **part d'écran**
  (`screen_fraction`, ADR-012 §2), `W_gameplay` = `W_urgence` = 1 (aucune source dans un banc), et son **coût** —
  la médiane de ses huit derniers pas payés, horodatés, 3,7 ms avant le premier (S343). L'ordonnanceur décide
  (hystérésis, durée de vie minimale 0,75 s, délai d'extinction 1 s, ADR-013 §5) et alloue sous **un budget de banc
  de 5 ms**, qui tient un domaine et pas deux. Chaque domaine accordé fait un pas horodaté. Un domaine qui naît
  repart de δ = 0 (I-12).
- **Seuils calibrés** (ADR-171) sur la part d'écran d'un domaine vu de face, **0,1629** : allumage **0,10**,
  extinction **0,05**.

**Un premier trajet écarté** : l'œil fixe, la tête tournée vers B. B, à 68 m, y reste plus petit à l'écran que A
(0,013 contre 0,034) : l'ordonnanceur servirait A, et il aurait raison — la surface décide, pas le regard.

## 2. Ce qui est mesuré

| critère, écrit avant le code | mesure | verdict |
|---|---|---|
| 1. le budget n'est jamais dépassé | accordé au pire **3,720 ms** pour 5, sur 1 200 images | tenu |
| 2. le plus visible est servi, hors transitions | non servi **0,700 s** deux fois : du croisement des parts (7,0 et 15,0 s) à l'allumage du nouveau (part ≥ 0,10) — la bande d'hystérésis, voulue | tenu |
| 3. transitions d'ADR-013 §5 | B allumé à **7,767 s**, sans délai ; A éteint à **7,867 s**, 1,000 s après son passage sous 0,05 ; **6 images** « vivant mais affamé » par bascule | tenu |
| 4. coûts mesurés, pas d'exclusion absorbante | médianes **3,691 / 3,677 ms**, q99 3,734 / 3,711 ; 720 et 480 pas payés ; chaque domaine rallumé est servi | tenu, avec l'oubli (§3) |

## 3. Ce que le premier passage a montré — l'exclusion absorbante, en 3D

Sans l'oubli des coûts, **le premier pas payé coûte 22,6 ms** : la carte sort du repos. Ce coût dépasse le budget ;
le domaine n'est plus accordé, ne paie plus de pas, garde donc ce coût, et n'est plus jamais servi — **l'exclusion
absorbante de [S279](ORDONNANCEUR-S279.md) §4**, que S280–S286 avaient levée pour la bande en faisant oublier ses
coûts à un domaine non servi. Le banc 3D reprend cet oubli (un échantillon par image non servie, le plus ancien
d'abord). Carte froide, le domaine alterne alors une demi-seconde — un pas à ~12 ms, une image sans pas — puis entre
en régime ; carte chaude, il y est d'emblée. **L'oubli vit aujourd'hui dans chaque hôte** (la bande, ce banc) :
un troisième hôte le recopierait. Sa place est à côté de l'ordonnanceur, dans le cœur.

## 4. Ce que ce document ne dit pas

- **Les deux autres critères de la porte A** : un domaine qui **se déplace et se redimensionne** au lieu d'être
  seulement allumé ou éteint ; la **dégradation de rang 1** d'ADR-012 §4 — rétrécir les domaines non focaux —, qui
  donne une issue à la famine. Ici, un domaine vit ou meurt.
- **Un budget de produit** : 5 ms est une contrainte de banc ; la porte C poursuit δ ≤ 2 ms par image.
- **Le rendu** : aucun pixel n'est produit ; la part d'écran est calculée, pas observée.
- **`W_gameplay` et `W_urgence`** : sans source dans un banc, déclarés à 1.

*Note S350 (2026-09-24)* : le déplacement et le redimensionnement sont au §5 et au §6 ; reste le rang 1.

---

## 5. S349 — un domaine qui se déplace

2026-09-24. Deuxième critère de la porte A, première moitié : un domaine qui **se déplace** au lieu d'être seulement
allumé ou éteint.

**Reproduire** : commit `47a8c5ec` ou plus récent ; `… -- --delta3d-decalage` (identité) ; `… --delta3d-suivi` (la
côte).

**Le mécanisme.** Dans le pas de production, la position du domaine n'entre que par l'évaluation du fond de B : le reste
travaille en indices locaux. `Step3::shift(di, dj)` décale l'état de mailles entières — les vitesses des trois familles
de faces, la surface et son reste compensé, la pression de départ, la surface publiée — et avance l'origine du fond.
Ce qui entre naît au repos, δ = 0 (I-12) ; ce qui sort est perdu. Un noyau de recopie, un tampon de travail réservé à
la configuration (I-06). **Au bit** : après un décalage de (+3, −2) sur la scène de B, les sept tableaux sont
identiques à l'ancien translaté dans le recouvrement (363 440 à 373 230 faces par famille, 12 870 colonnes, 360 360
mailles), au repos ailleurs ; le pas entier garde ses empreintes.

**Sur la côte de S344**, même caméra, même ordonnanceur, même budget — mais **un seul domaine**, qui se décale vers le
point regardé, au plus deux mailles par image :

| | deux domaines fixes (§2) | **un domaine qui suit** |
|---|---:|---:|
| naissances / extinctions sur le trajet | 2 / 2 | **1 (au départ) / 0** |
| part d'écran du domaine servi, en régime | de 0,1629 à 0,0389 aux croisements | **0,1629 constante** |
| décalages | — | **480** d'une maille, 120 m |
| colonnes hors bornes | 0 | **0** |

**Le coût du décalage** : 1,463 ms en médiane, 4,872 au pire, temps réel de la soumission à la fin sur la carte — sept
soumissions séparées, une par tableau, pour 6 Mo recopiés deux fois. Les grouper en une seule soumission, avec sept
uniformes, en retirerait l'essentiel ; tel quel, un décalage par image s'ajoute au pas.

**Ce que cela ne dit pas** : le **redimensionnement** — rétrécir un domaine non focal, le rang 1 d'ADR-012 §4 — reste ;
l'ordonnanceur ne commande pas encore le déplacement, l'hôte le fait (le centre sous l'œil) ; l'état qui sort du domaine
n'est pas rendu à W (A289).

**Correction S350 (2026-09-24).** Le décalage recopiait aussi les **faces normales du bord** — `u` en `i = 0` et
`i = nx`, `v` en `j = 0` et `j = ny` —, que le pas n'écrit jamais : les murs de δ, nuls depuis l'état initial. Les
vitesses intérieures qu'il y posait restaient figées : jusqu'à 0,17 m/s, 2,8 m³/s à travers le bord après l'aller du
suivi. La hauteur ne les transporte pas — la dérive du volume de δ est la même avant et après la correction, celle du
fond —, mais la divergence les lit, donc la projection : **un seul décalage de trois mailles**, figé 2 s, écarte la
surface de **7,2 cm** près du mur et de 3 cm au loin (`--delta3d-murs`, scène de B ; témoin identique au bit). L'identité
ci-dessus comparait à l'ancien translaté, murs compris, et ne pouvait pas le voir. **Corrigé** : un décalage laisse ces
faces nulles ; l'identité compte 6 272 murs `u` et 6 720 murs `v`, tous nuls, et aucune autre différence. Les résultats
d'ordonnancement du tableau restent vrais ; l'état du domaine qui suivait était faux près de ses bords.

## 6. S350 — un domaine qui se redimensionne

2026-09-24. Deuxième critère de la porte A, seconde moitié : un domaine qui **se redimensionne**. Avec le §5, le
deuxième critère de §3 bis est tenu ; reste le troisième, la dégradation de rang 1.

**Reproduire** : commit `e0dcb281` ou plus récent ; `… -- --delta3d-redimensionnement` (identité, lignes
`DELTA3D_REDIM_S350`, une vingtaine de secondes) ; `… -- --delta3d-cout-emprise` (coût, `DELTA3D_EMPRISE_S350`,
alimentation relevée avant et après — A270) ; non-régression : `… -- --delta3d-empreinte` (empreintes S343) et
`… -- --delta3d-decalage`.

**Le mécanisme.** Les noyaux lisent les dimensions dans des uniformes ; les tampons sont réservés à la **capacité**,
la forme de création (I-06). Une **forme courante** — `nx`, `ny` au plus la capacité, `nz` égal — commande comptes,
dispatchs, copies et relectures ; quand elle vaut la capacité, les empreintes de S343 (60 et 600 pas) sont
inchangées. `Step3::resize(di, dj, nx, ny)` recopie tout l'état de la forme courante dans le tampon de travail, puis
réécrit chaque tableau dans la nouvelle disposition — l'ancien `(i + di, j + dj, k)` s'il existe, le repos sinon, les
murs nuls — en **une seule soumission**, un uniforme par tableau ; les uniformes du fond, de la projection et du pas
suivent la forme, l'origine avance. `shift` en est le cas à forme égale.

**Au bit**, scène de B après 30 pas, deux redimensionnements successifs du même domaine :

| | rétréci 120×112 → 90×84, depuis (13, 17) | élargi → 110×96, depuis (−6, −9) |
|---|---|---|
| (a) état réécrit, sept tableaux | **0** différence ; murs nuls, 4 704 `u` et 5 040 `v` | **0** ; entrant au repos, 78 960 `u` et 3 000 colonnes |
| (b) un domaine **créé** à cette forme et à cette origine reçoit le même état ; 60 pas chacun | **0** différence sur 881 832 valeurs ; volume 29,74 m³ des deux côtés | **0** sur 1 230 728 ; 31,65 m³ |
| (c) allocateur de la carte, avant et après | 89 784 320 octets, 27 allocations — inchangé | inchangé |

(b) est ce qui fait **un domaine, et non une vue** : rien dans le pas ne se souvient de la capacité.

**Le coût suit l'emprise** — secteur aux deux bornes (`BatteryStatus` 2, 97 %, `PowerOnline` vrai), un domaine
redimensionné en place, centré, 500 pas horodatés par forme :

| forme | surface | pas, médiane / q99 | rapport à la pleine forme |
|---|---:|---:|---:|
| 120×112 | 1,00 | 3,695 / 3,736 ms | 1,000 |
| 104×97 | 0,75 | 2,743 / 2,787 | 0,742 |
| 85×79 | 0,50 | 1,834 / 1,877 | 0,496 |
| 60×56 | 0,25 | 1,019 / 1,064 | 0,276 |

Moindres carrés : **0,09 ms + 3,57 ms × surface** ; les trois passes suivent la surface, aucune colonne hors bornes ;
témoin à la pleine forme 3,695 ms contre 3,679 en S343. **Ce que le rang 1 en attend** : rétrécir un domaine de
moitié libère la moitié de son coût, à 0,09 ms près.

**Le décalage, remesuré** en temps mural, soumission et attente comprises : **0,375 ms** en médiane, q99 1,05, max
1,22 — contre 1,463 en S349, en sept soumissions ; un changement de forme 100 ↔ 75 % : 0,360 ms, q99 1,31.

**Ce que cela ne dit pas** : l'ordonnanceur ne **décide** pas encore de rétrécir — c'est le rang 1 ; un domaine ne
dépasse pas sa capacité, fixée à la création, et `nz` ne change pas ; la bande qui naît au repos en s'élargissant
n'a reçu aucun verdict visuel ; ce qui sort n'est pas rendu à W (A289).


## 7. S351 — la dégradation de rang 1 : la famine a une issue

2026-09-24. Troisième critère de la porte A : *la dégradation d'ADR-012 §4 rang 1 existe, donc la famine a une issue*.
Au §2, le budget de 5 ms ne tenait qu'un domaine : l'autre, vivant, n'était pas servi.

**Reproduire** : commit `1c1a32e2` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml -p water-core --release --offline scheduler` — 28 essais, dont dix
  `_s351` ; `… --example ordonnanceur_s278` — empreinte **`6aebff024c734fc9`**, celle de S278.
- `PRECHAUFFE=1 COUT=max cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-rang1` — lignes
  `DELTA3D_RANG1_S351`, une trentaine de secondes ; `RANG1=0` pour le témoin, `COUT=med` pour la médiane, sans
  `PRECHAUFFE` pour la carte froide. Alimentation relevée avant et après (A270).

**Le mécanisme, dans le cœur** (`scheduler.rs`). Un candidat **déclare** ce qu'il peut céder — `Shrink` : l'échelle
minimale de sa surface et la part fixe de son coût, qui suit alors `fixe + (plein − fixe)·échelle` (§6). Si le sac à dos
de S278 laisse un vivant sans budget, le **focal** — la plus forte priorité — est servi entier ; les autres, par `P/C`
décroissant, à leur échelle minimale autant qu'il en tient ; puis une **échelle commune** monte au plus haut que le
budget permet. ADR-012 §5 : descente immédiate, remontée d'au plus 1 par seconde et seulement une seconde après la
dernière descente ; focal engagé une seconde. Sans aucune déclaration, la décision est **celle de S278 au bit** —
l'empreinte de son exemple n'a pas bougé. Un substitutif ne peut pas déclarer : rétrécir son emprise n'est pas gratuit.

**Le banc.** La côte de S344, deux domaines de production, 5 ms, mais **36 m d'écart** et une **pause de 8 s** où l'œil
(x = 20 m) les voit tous deux au-dessus de leurs seuils — à 60 m, aucune pose ne les rend voulus ensemble. Chacun
déclare pouvoir descendre jusqu'à ce que son éponge tienne (24 × 24 mailles, 4,3 % de la surface), part fixe 0,09 ms.
L'hôte fait de l'échelle une emprise centrée, arrondie par défaut, par `Step3::resize` ; la part d'écran se mesure sur
l'emprise **pleine** — ce que le domaine doit couvrir, sans quoi rétrécir le ferait mourir ; le coût mesuré est ramené
à la pleine emprise.

| carte préchauffée, 1 200 images | témoin | rang 1, médiane des 8 | **rang 1, maximum des 8** |
|---|---:|---:|---:|
| images « vivant mais affamé », A / B | 612 / 3 | 0 / 0 | **0 / 0** |
| images où les deux domaines sont servis | 0 | 615 | **615** |
| focal rétréci ; plus visible non servi | 0 ; 0 | 0 ; 0 | **0 ; 0** |
| accordé au pire | 4,014 ms | 5,000 | 5,000 |
| image mesurée après 1 s : médiane / q99 / max | 3,694 / 3,737 / 3,762 | 4,946 / 5,009 / 5,045 | 4,912 / **4,983** / 5,004 |
| images mesurées au-dessus de 5 ms | 0 | 20, +0,045 ms au pire | **1, +0,004 ms** |
| redimensionnements (descentes / montées de A) | 0 | 14 (7 / 5) | 19 (11 / 6) |

Pendant la pause, A tourne à l'échelle 0,33–0,35 (69 × 64 mailles environ), B entier ; un redimensionnement coûte
0,74 à 0,83 ms en temps mural.

**Deux faits, publiés à part.** *La carte froide* : sans préchauffe, le premier pas d'une carte au repos coûte 22,4 ms ;
le focal, seul, ne tient pas le budget, et l'oubli de S286 le rend affamé une image sur deux pendant 0,57 s — 17 images,
16 au témoin. C'est le démarrage du §3, étranger au rang 1, et une carte qui rend chaque image n'est pas froide ; le
critère « aucune image affamée », écrit avant la mesure, est donc **tenu carte chaude et manqué carte froide**, sur ce
seul point. *L'estimateur* : le rang 1 remplit le budget jusqu'au coût annoncé ; annoncé à la médiane des huit derniers
pas, le temps mesuré le dépasse une image sur cinquante-sept ; au maximum des huit — plus près du 99ᵉ centile qu'ADR-012
§3 vise —, une sur 1 140, de 0,004 ms.

**Le prix, sans seuil.** La première descente (120 × 112 → 69 × 65) retire **jusqu'à 11,3 cm** de δ — la correction
couplée d'une mer de `Hs` 2,5 m, qui monte à 18,6 cm dans ce domaine — dans une bande **intérieure**, que le rendu
pondérait entièrement. Les suivantes, d'une maille, tombent dans le fondu de 3 m : 0,3 à 10 cm coupés, médiane 3,5 cm,
à faible poids. **Aucun verdict visuel** : qu'ADR-012 §4 ait raison de dire cette perte « quasi nulle » reste à juger.

**La porte A.** Ses trois critères tiennent : plusieurs candidats réels (§2), un domaine qui se déplace et se
redimensionne (§5–6), le rang 1 qui donne une issue à la famine (ici). **Porte A reçue sur le banc** (ADR-175 §4.4),
comme la porte C.

**Ce que cela ne dit pas** : l'échelle oscille d'une maille autour de sa cible, au rythme du bruit du coût — une bande
morte reste à écrire ; l'estimateur au maximum vit dans l'hôte, comme l'oubli ; les rangs 2 à 7 et le régulateur PI
d'ADR-012 §5 n'existent pas ; B8 non plus ; tout est au banc, sans rendu concurrent.
