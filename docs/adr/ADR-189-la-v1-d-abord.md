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

## Note datée du 2026-09-24 (S338) — la porte D reçue

**D1 est tenue.** Le verdict R15 — *« 1. Oui »*, *« 3. Pas forcément »* — puis *« Plus de coupure »* sur les
images de S337 reçoivent la porte D **sur la référence CPU** ([PORTE-D-S333](../validation/PORTE-D-S333.md) §9),
au bout des sessions S329 à S337. W derrière la requête du corps, la coque dans la production de δ et les
autres degrés de liberté restent des suites, non des critères de la porte.

**Un fait que le §1 n'écrivait pas.** [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) D4 définit la v1 par les
**portes A, B, C et D** de la feuille de route §3 bis ; le tableau du §1 ne nommait que la D. La v1 n'est donc
pas atteinte. Restent la porte B, dont ne manque que le verdict de l'utilisateur sur une mer jugée
convaincante ; la porte C, δ ≤ 2 ms GPU sur la scène de la porte B — 4,62 ms mesurés en S302 ; la porte A,
l'ordonnanceur sur des domaines 3D.

**Ce que ce fait laisse ouvert.** D2 suspend le lot 5 « jusqu'à la v1 », que le §1 lisait comme la porte D :
la suspension court-elle jusqu'à la porte D, désormais reçue, ou jusqu'à la v1 entière ? C'est
l'interprétation que D2 disait déjà à confirmer ; elle revient à l'utilisateur. D'ici là, rien ne change :
les sessions suivent la porte en cours de §3 bis, B.

**Correctif, même jour (S338).** La porte B n'attend pas que le verdict : les deux premiers cas du critère 1
d'[ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md) §4 — reproduction des réceptions 2D à `ny` = 1,
invariance en `y` sous une houle à crêtes longues — ne sont pas mesurés sur la production (critère 2,
[CUVE-GPU-S305](../validation/CUVE-GPU-S305.md) §7). La phrase ci-dessus reprenait la feuille de route, qui
l'écrivait depuis S305.

## Note datée du 2026-09-24 (S351) — le terme de D2

L'utilisateur a répondu : *« Continue, après la V1 ton objectif seras de completer entièrement la to do liste »*.
D2 se lit donc jusqu'à la **v1 entière** — les portes A, B, C et D ; le lot 5 reprend ensuite, comme partie de la
liste du projet fini ([ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md) D4).
