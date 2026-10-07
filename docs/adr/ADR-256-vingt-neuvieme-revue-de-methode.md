# ADR-256 — Vingt-neuvième revue de méthode (S621–S625)

- **Statut : actée**, S626, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-255](ADR-255-vingt-huitieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une tolérance d'accord qui ignorait la sensibilité du schéma** (S622) : l'accord avec numpy était demandé à 10⁻¹² m ; au-delà de la maille 1, un ulp de différence (les exponentielles de numpy et de Rust, A98) change de branche dans les pentes minmod des zones presque plates et se propage jusqu'à 10⁻⁸ m. Critère (1) manqué, consigné | un critère manqué, une relance | **protection élargie** (D1) |
| **Deux montages qui n'éprouvaient pas la propriété** (S622) : une onde solitaire de 2 cm plus large que le bassin, puis une impulsion où la propagation non linéaire au large s'ajoutait ; l'écart n'a été attribué au numérique qu'après avoir fait varier l'amplitude (il plafonnait : linéaire) et la maille (il convergeait : numérique) | trois explorations au plan | **protection nouvelle** (D2) |
| Une durée de simulation trop courte (S624 : à 130 s, l'onde n'avait pas fini sa course) — vue à des valeurs loin de la loi, avant la mesure du code | une relance | rien de nouveau : le plan lit ses propres valeurs avant de s'écrire (ADR-251 D1) |
| La tolérance de S625, posée avec la leçon de S622 (10⁻⁶ m), tenue à 10⁻¹³ ; la graine (S623), la chaîne (S624) au bit | — | **rien à changer** |

## 2. Décisions

**D1 — La tolérance d'accord avec une référence indépendante tient compte de la sensibilité du schéma.** Un schéma à branches (un limiteur,
un seuil de mouillage, un `max`) amplifie l'ulp : le plan mesure cette sensibilité — la référence relancée avec une entrée perturbée d'un
ulp — et pose la tolérance au-dessus de l'écart obtenu, jamais à une valeur choisie d'avance (élargit ADR-251 D1).

**D2 — Un écart attribué à un composant se vérifie par deux variations.** Avant de dire « c'est le bord » ou « c'est le numérique », le plan
fait varier l'amplitude (un effet linéaire s'y proportionne) et la maille (un effet numérique converge) ; un montage qui n'isole pas la
propriété se reconnaît là, avant la mesure du code.

## 3. La prochaine revue

S631.
