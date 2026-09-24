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

