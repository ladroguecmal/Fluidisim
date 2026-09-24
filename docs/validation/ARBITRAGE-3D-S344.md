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
