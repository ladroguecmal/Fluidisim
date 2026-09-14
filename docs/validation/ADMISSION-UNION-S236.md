# Composition sur l'union et admission de la scène S235 — S236, 2026-09-15

Réception d'[ADR-142](../adr/ADR-142-composition-sur-l-union-des-emprises.md). Suite de
[SCENE-MULTI-S235](SCENE-MULTI-S235.md) §2 (49 refus sur 161, tous par majorant). Code :
`code/water-core/src/mixed_union.rs` ; tests dans `tests_mixed_water.rs` ; diagnostics de l'hôte
`--bornes-union` et `--multi --union-coeur`.

## 1. Ce qui manquait, établi avant de construire

**Deux limites du cœur, pas une.** (1) La requête mixte ne compose que l'**intersection** des
emprises : un point hors du disque d'un seul impact est refusé par le domaine. Sur huit disques
répartis sur 70 m, presque aucun point de la scène n'est servable. (2) Le balayage d'ADR-138 ne
parcourt que le disque de l'ancre — sûr pour l'intersection, **faux pour l'union**.

**Quelle borne suffit** — série S235, tous les 0,25 s sur 40 s :

| plancher, sur l'union | refus / 161 | pire / π/7 |
|---|---:|---:|
| actuel (intersection, ADR-138) | 49 | 1,251 |
| ADR-138 étendu à l'union (termes nuls au-delà de leur domaine) | 40 | 1,190 |
| séparation 2D, pression globale | 32 | 1,153 |
| valeur atteinte par la fonction bornée `G` | 31 | 1,152 |
| séparation 2D + pression locale ADR-137 sur cellules critiques | **0** | ≤ 0,9994 |

Même une somme d'impacts exacte en position refuse 31 instants : les termes sont trop larges, la
pression globale surtout (0,19 contre ≈ 0,07 réelle). La pression **locale**, appliquée aux seules
cellules de demi-côté ≤ 1 m qui dépassent, certifie les 32 instants restants en 92 à 128 cellules,
3 à 15 appels et 3 à 13 ms.

## 2. Réception de la borne (tests du cœur)

| essai | résultat |
|---|---|
| **trou d'ADR-138 sur l'union** — ancre ×1,3 d'énergie en (0, 0), deux impacts confondus à 100 m, nés au même instant | pente réelle **0,4252** ; plancher ADR-138 **0,2837** — faux d'un tiers ; plancher union non certifié à 0,99·réelle (0,4252, 144 cellules), certifié à 1,05·paire (0,4252, 64 cellules) |
| échelle de huit seuils, quatre impacts N64 recouvrants (réelle 0,00118, somme 0,00461, max de `G` 0,00225) | certifié ⟹ réelle ≤ plancher ≤ seuil ; non certifié ⟹ plancher > seuil ; un certificat par séparation obtenu |
| pool de 64 cellules / pool vide | non certifié, borne ≥ réelle ; refus sans cellule |
| chemin rapide, un impact | identique **au bit** à `slope_floor` |

Un premier passage de l'échelle n'avait certifié que par le chemin rapide : les seuils testés étaient
tous sous le maximum de `G`. Deux seuils plus hauts ont été ajoutés et un certificat par séparation
est désormais exigé — sans quoi le test n'aurait rien reçu de la séparation.

## 3. Réception de la requête (tests du cœur)

- **Couverture** : sur le montage B + un impact de 16 m + pression d'emprise [−8 ; 12]², quatre points
  (disque et emprise, disque seul, emprise seule, aucun) donnent **au bit** la somme à la main des seules
  perturbations couvrantes ; la requête intersection sert le premier et refuse les trois autres.
  Premier passage en échec **du test** : un point non représentable au 1/2048 m, sommé à la main au
  point brut au lieu du point local quantifié — le piège d'A253 (S214), corrigé.
- **Deux sens d'ADR-128** : sur 25 seuils de 0,9·réelle à 1,01·somme, plancher certifié ⟹ requête
  acceptée et `UnionFloor` rendu identique à l'annonce ; non certifié ⟹ `Slope` ou `SlopeEnvelope`.
  Les deux issues présentes, pression locale sollicitée.
- **Contrat de pente S143** : la garde a refusé la première version — la sélection des cellules
  critiques comparait à `max_slope` sans déclaration. Les trois sites sont écrits
  `budget > max_slope` (`budget` ne somme que des majorants de pente réelle déjà convertis) et
  inscrits.
- Mode intersection : aucune fonction modifiée ; suite du cœur **419 réussis, 5 ignorés** (413 avant).

## 4. La scène S235 admise par le cœur

`--multi --union-coeur`, secteur au début et à la fin (A270), 161 instants :

| grandeur | valeur |
|---|---:|
| planchers certifiés | **161 / 161** |
| requêtes refusées (6 988 sondes chacune) | **0** |
| pire plancher | 0,9995 π/7 |
| cellules au plus / appels locaux au total | 128 / 355 |
| plancher, médiane / maximum | 0,001 ms (chemin rapide) / **12,0 ms** |
| requête de 6 988 points, médiane / maximum | 1 421 / 1 640 ms |
| **écart à la somme de référence de l'image**, hauteur / pente | **< 1e-9 m / 3·10⁻⁸** |

Le dernier chiffre est la raison d'être du mode : **la requête du cœur et l'image décrivent désormais la
même eau** sur la scène représentative, là où l'intersection ne servait presque aucun de ces points.

**Le coût de la requête** vaut ≈ 0,2 ms par point, et vient de l'évaluation de la pression — 4 096
modes par point, sur CPU, un fil — et non du mode union : l'intersection paierait le même prix par
point admis. Il n'est pas un coût d'image (l'hôte rend par la grille GPU) ; il borne ce qu'une requête
gameplay peut demander par instant. Le plancher, lui, coûte au pire 12 ms lorsqu'il sollicite la
pression locale, une fois par lot.

## 5. Limites

- **Statut de la garantie.** Là où la pression locale intervient, la borne hérite de la réception
  numérique d'ADR-137, pas d'un certificat f32 (**A258**). Ailleurs, statut d'ADR-138.
- **A261 reste ouverte** : la pression globale domine partout ailleurs, et une scène où la pression
  seule sature ne gagnerait rien.
- **Aucun cache** entre lots d'un même instant ; le plancher est recalculé à chaque requête.
- **Une machine, une scène** ; seconde cible non reçue.
