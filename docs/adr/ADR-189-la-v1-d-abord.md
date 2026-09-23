# ADR-189 — La v1 d'abord

- **Statut : actée**, S329, 2026-09-23, **décision de l'utilisateur** : *« continue, jusqu'à la v1 »*,
  après le compte rendu de S328.
- **Suspend l'application** d'[ADR-184](ADR-184-seconde-representation-en-parallele.md) D1 et
  d'[ADR-188](ADR-188-lot-3-a-la-place-du-lot-2-bloque.md) D1 — l'alternance des sessions avec le lot 5 —
  jusqu'à la v1 (D2, interprétation à confirmer).
- **Laisse entiers** [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) D4 (v1 = porte D),
  [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7 (l'ordre des lots),
  [ADR-008](ADR-008-flottabilite-et-autorite.md) (I-04 : aucune force de jeu ne vient de δ) et
  [ADR-127](ADR-127-ambition-complete-construction-progressive.md) (le périmètre).

## 1. Ce que la v1 exige, lu dans le dépôt

La v1 est la **porte D franchie** (ADR-174 D4) ; ADR-178 D7 y mène par les **lots 3 et 4** :

| lot | ce qui reste | reçu si |
|---|---|---|
| **3** — faces coupées 3D | fond coupé reçu, modes linéaire et mobile (S324–S328) ; manquent les **obstacles qui ne sont pas un fond**, fixes puis à **frontière mobile** | essais 2 et 3 de la piscine : boule immergée — volume déplacé, flottabilité ; boule à mouvement imposé — forces, vagues |
| **4** — corps rigides | tout : intégrateur, forces rendues, masse ajoutée | une boule libre flotte à son tirant d'eau théorique et oscille à la période que la raideur hydrostatique implique ([C10](../validation/CAS-CANONIQUES.md) : tirant ± 1 %, période ± 5 %, rapport √2 ± 15 % avec la masse ajoutée) |
| **porte D** | le bateau dans B + W + δ | un bateau flotte et perturbe l'eau qui le porte, **sans autorité de δ sur le jeu** — le jeu le fait flotter sur B + W ; δ le voit comme une paroi mobile et ne lui rend qu'un décalage visuel borné (ADR-008 §1) |

## 2. Décisions

**D1 — Les sessions suivent le chemin de la porte D, l'une après l'autre, jusqu'à sa réception.**
Chaque session garde son protocole entier — plan déclaré avant le travail, une étape par commit,
preuve avec « Reproduire », rituel — et désigne la suivante sur ce chemin.

**D2 — L'alternance avec le lot 5 est suspendue jusqu'à la v1.** *Interprétation de « jusqu'à la v1 »,
à confirmer par l'utilisateur* : le lot 5 ne mène pas à la porte D, et une session sur deux lui revenait.
Il reprend après la v1, par A316, là où S327 l'a laissé. Un mot de l'utilisateur rétablit l'alternance.

**D3 — Une session s'arrête et demande** quand la suite appartient à l'utilisateur : un verdict visuel,
la voie d'A289 si elle devient nécessaire, une réduction d'ambition, une action d'infrastructure ; ou
quand un défaut bloquant n'a plus de chemin qu'elle puisse éprouver.

## 3. Ce que cette décision ne tranche pas

- **La voie d'A289** et la **sauvegarde** par `git bundle` : toujours en attente. La porte D ne dépend
  pas d'A289 — un corps flotte d'abord en eau calme (ADR-188 §1).
- **La forme du bateau** de la porte D : une coque simple d'abord ; la session qui l'ouvre la déclare.
- **Le budget** : ADR-178 D4 le garde mesuré et publié, non opposable avant la porte C.
