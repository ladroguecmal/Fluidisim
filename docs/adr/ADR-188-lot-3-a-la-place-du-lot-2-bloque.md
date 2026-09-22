# ADR-188 — Le lot 3 prend la place du lot 2 dans l'alternance, tant que l'ordre E est bloqué

- **Statut : actée**, S324, 2026-09-22, **décision de l'utilisateur** : *« Je suis ta
  recommandation »*, en réponse à la proposition de
  [BILAN-GLOBAL-S321](../registres/BILAN-GLOBAL-S321.md) §7.1, reprise au compte rendu de S323.
- **Modifie l'application** d'[ADR-184](ADR-184-seconde-representation-en-parallele.md) D1 —
  l'alternance des sessions entre le lot 5 et le lot 2.
- **Laisse entiers** [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7 (l'ordre des
  lots, où le lot 3 suit le lot 2), [ADR-127](ADR-127-ambition-complete-construction-progressive.md)
  (le périmètre), [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) (v1 = porte D) et
  [ADR-186](ADR-186-apic-seconde-representation.md).

## 1. Pourquoi

Le lot 2 est **bloqué** : sous une vraie mer, δ croît jusqu'à trois fois la houle (A289,
[MER-S319](../validation/MER-S319.md)), et la voie qui le soignera est un choix de l'utilisateur,
non encore fait. S322 a montré que la croissance ne dépend pas du pas de temps : aucun remède
numérique du pas ne le débloquera à la place de ce choix.

Pendant ce temps, l'alternance d'ADR-184 donnait une session sur deux à un lot qui ne pouvait pas
avancer. Les lots 3 et 4 — faces coupées en 3D, puis corps rigides — mènent à la **porte D**, donc à
la **v1** d'ADR-174 D4, et **ne dépendent pas d'A289** : un corps qui flotte se construit et se reçoit
d'abord en eau calme.

## 2. Décisions

**D1 — Tant que l'ordre E est bloqué, le lot 3 prend la place du lot 2 dans l'alternance.** Les
sessions alternent entre le lot 3 et le lot 5 ; le lot 4 suit le lot 3 dans la même place.

**D2 — Le lot 2 reprend sa place dès que la voie d'A289 est choisie**, ou que son blocage est levé
autrement. Rien du lot 2 n'est retiré ni réordonné : l'ordre E attend, daté dans la file.

**D3 — La demande actuelle de l'utilisateur prime toujours** sur cette alternance, comme sur
toute suite déclarée (REPRISE §6).

## 3. Ce que cette décision ne tranche pas

- **La voie d'A289** — rappel lent de δ, durée de vie bornée des domaines, ou dispersion
  d'amplitude dans B : toujours proposée, jamais choisie ici.
- **La sauvegarde** par `git bundle` sur un support externe : proposée en S321, non autorisée.
- **Aucune réduction d'ambition** : le lot 2, V, les inondations et la grande échelle restent dus
  (ADR-127).

## 4. Ce qui suit

S324 ouvre le lot 3 : les faces coupées de δ en trois dimensions, la découpe de S232 portée à la
grille x-y-z. La session suivante de l'alternance revient au lot 5 — le raccord dynamique.
