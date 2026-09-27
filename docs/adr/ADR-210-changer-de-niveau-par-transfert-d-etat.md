# ADR-210 — Changer un domaine de niveau par transfert d'état

- **Statut : actée**, S402, 2026-09-27 ; autonomie technique (S71). Campagne du solveur volumique 3D, **C8c**
  ([ADR-207](ADR-207-la-campagne-du-solveur-volumique-3d.md)) ; le rang 4 d'[ADR-012](ADR-012-ordonnanceur-budget-degradation.md) §4.
- **Remplace**, pour le seul changement de niveau, le mécanisme d'[ADR-006](ADR-006-cellules-domaines-solveurs.md) §3.2 : « le
  passage d'un niveau à l'autre […] se traduit par une destruction/création de domaine, gratuite visuellement (ADR-005 §5) ». La
  création, la croissance, le rétrécissement et la destruction d'un domaine qui cesse restent ceux
  d'[ADR-005](ADR-005-zone-de-transition.md) §5.
- **Mesure** : [NIVEAUX-S402](../validation/NIVEAUX-S402.md) ; code `Volume3::resample_from` (`delta3d_levels.rs`).

## 1. Ce qui a été mesuré

Le rang 4 — 25 → 50 cm à 2 s — puis le retour à 25 cm à 5 s, contre le domaine à 25 cm tenu tout du long ; l'image est ce que
le rendu montrerait, sur la grille fine.

| | transfert d'état | destruction/création (ADR-005 §5) |
|---|---:|---:|
| bosse de 5 cm, σ = 1 m : saut au passage / au retour | 0,24 / 0,13 mm | 0,36 / 0,49 mm |
| bosse : écart pendant la période à 50 cm / après le retour | **1,8 / 2,3 mm** | **10,2 / 12,8 mm** |
| source mobile d'une maille grossière : écart pendant / après | 15,7 / 16,7 mm | 17,0 / 23,6 mm |

Les deux mécanismes passent sans saut visible : le transfert parce que l'état continue, la destruction parce que l'ancien
domaine s'efface sur τ. Mais **la destruction perd tout ce que le domaine contient** : le nouveau naît à δ = 0, et la
transduction vers W (ADR-005 §3) ne rendrait que ce qui sort par le bord. Le transfert garde ce que le niveau d'arrivée sait
porter : sur une bosse qu'il résout, l'image reste sous la tolérance d'image (1,8 mm pour 3).

## 2. Décisions

**D1 — Un changement de niveau de `dx` d'un domaine perturbatif se fait par transfert d'état.** La surface se transfère par
recouvrement d'une reconstruction bilinéaire conservative — volume exact, erreur d'ordre deux au moins (0,30 % d'une onde de
seize mailles grossières, aller et retour) —, les vitesses s'interpolent aux faces et se moyennent sur la face quand
l'arrivée est plus grossière, la pression de départ repart de zéro. Le domaine de départ et celui d'arrivée existent ensemble
le temps du transfert : **tous deux sont réservés** à l'initialisation (I-06).

**D2 — Le prix du rang 4 est celui du contenu, et se publie.** Faible pour ce que le niveau d'arrivée résout, fort pour ce qu'il
ne résout pas (15 à 17 mm près d'une source large d'une maille grossière) : c'est le « visible de près » d'ADR-012 §4. Le
passage lui-même ne coûte rien de visible ; ce qu'on perd, c'est la résolution. Quand descendre — et quel domaine — est à
l'ordonnanceur, qui doit donc savoir **ce que le domaine contient** ; non tranché ici.

**D3 — La production reproduit le transfert** (C8, au poste), à 3 mm de la référence comme tout étage (ADR-175 D4).

## 3. Ce que la décision ne fait pas

Elle ne choisit pas quand descendre (l'ordonnanceur, la famine : C8, suite) ; le rapport 2,5 (10 ↔ 25 cm) n'est éprouvé que pour
la conservation (la hauteur moyenne, au bit près), pas au banc ; rien sur la carte ; aucun verdict visuel (C10). Elle ne touche
ni au cycle de vie d'ADR-005 §5 pour un domaine qui naît, grandit, rétrécit ou cesse, ni aux six niveaux d'ADR-006 §3.2.
Invariants relus : **I-12** — le passage n'est pas visible, et un domaine perturbatif qui change de niveau ne perd plus son
contenu ; I-04, I-06, I-17 ; aucun amendé.

**Note du 2026-09-27 (S404) — un domaine épars, en mer** ([preuve](../validation/MER-EPARS-S404.md) §4–5). Le transfert porte
l'ensemble épars : au départ, le bord de l'ensemble se lit comme le bord de la boîte, et une colonne dehors porte le repos ; à
l'arrivée, les colonnes dehors restent au repos. Le volume est exact quand l'ensemble d'arrivée couvre celui de départ
(`Follow::require_cover`), la perte publiée sinon ; un rectangle passe 25 → 50 → 25 cm au bit de son dense. Sous le pas couplé, en
mer, l'épars qui suit reste à 0,14 mm de l'entier qui fait les mêmes passages : l'ensemble n'ajoute rien au prix du niveau, qui
reste celui du contenu (D2). La décision ne change pas.
