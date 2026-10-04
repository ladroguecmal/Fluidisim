# ADR-222 — La méthode se révise elle-même : les frictions mesurées, les décisions techniques remplacées sans demander

- **Statut : actée**, S481, 2026-10-04 ; **demande de l'utilisateur** : *« L'objectif est que tu deviennes autonome, que tu prennes toi-même
  les solutions, car certaines choses évoluent avec le temps et il faut les changer »*. Les pistes, et ce qui reste à l'utilisateur,
  décidés ici ([ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D2).
- **Précise** [ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) (l'autonomie) et [ADR-221](ADR-221-la-structure-du-projet.md) (la
  structure) ; ne touche ni à l'ambition ([ADR-127](ADR-127-ambition-complete-construction-progressive.md)) ni à la fin
  ([ADR-218](ADR-218-le-systeme-de-l-eau-complet.md)).

## 1. Ce qui a freiné S480–S481, constaté

Deux calculs longs morts ensemble avec la session (7 h perdues) ; la référence CPU, sur un seul cœur, limite tous les bancs (700 s pour
0,15 s de bulle) ; un coût multiplié par quatre vu seulement à la main ; un défaut de la référence (les poches d'une maille) découvert
dans la vraie scène, tard.

## 2. Décisions

**D1 — Les calculs longs sortent de la session.** `outils/calcul.py` lance par WMI sous Windows ; le registre dit s'il est hors de la
session et traduit les codes de sortie (fait en S481). Un calcul de plus d'une heure garde un point de contrôle (à faire avec D2).

**D2 — La référence CPU devient parallèle** (les cœurs de ce PC), au bit de la séquentielle : elle fixe la vitesse de toute la validation.

**D3 — Un banc de non-régression rapide entre dans le rituel de fin** : le chemin sans les nouveautés au bit, la masse, le coût de
`--v1` contre un seuil écrit. Ce qui change sans qu'on le veuille est vu à la session qui le change.

**D4 — Toutes les cinq sessions, une revue de méthode faite par la session elle-même** : relire les frictions du journal (ce qui a coûté
du temps, ce qui a été refait), corriger METHODE, la boussole et les outils, l'écrire dans un ADR. **Une décision technique que la mesure
contredit est remplacée par un nouvel ADR sans demander** ; ADR-027 (les cinq arbitrages de S18) compris s'ils deviennent techniquement
intenables, avec la mesure qui le montre.

**D5 — Ce qui reste à l'utilisateur, et seulement cela** : l'ambition et le périmètre (une réduction), le jugement des rendus, les
téléchargements, ce qui touche la configuration persistante de sa machine ou de son compte, la publication. Tout le reste se tranche ici,
par écrit.

**D6 — Les scènes se dimensionnent pour ce qu'elles prouvent** : une propriété se montre là où elle est résolue (les poches : une maille
plus fine que les 5 cm de `--v1`, qui n'en fait presque que de moins de huit mailles).

## 3. Ce qui attend l'utilisateur (inscrit dans la boussole)

- Une **relance planifiée** (une tâche qui envoie « Reprends le projet » quand le jeton est libre ou interrompu) : configuration persistante.
- Les **téléchargements anticipés** — Godot 4.5 en double précision, une copie locale de DyingStar — maintenant plutôt qu'au point 13.4.

## Note du 2026-10-05 (S482) — les réponses

L'utilisateur : *« non pour la 1 sinon oui »* — **pas de relance planifiée** ; **les téléchargements accordés**. Faits : le Godot de
DyingStar (`Godot_v4.7-stable_mono_win64.zip`, 114,6 Mo, `DyingStar-game/godotandaddons`, version
`4.7.stable.mono.double.custom_build.5b4e0cb0f`) et DyingStar (`develop`, profondeur 1, sans les fichiers LFS, 3,6 Go), dans
`C:/Users/antoi/FluidisimExterne/`. **Fait nouveau** : DyingStar est passé de Godot 4.5 à **4.7** ; son rendu de bureau est Forward+, comme le nôtre
(GL Compatibility n'est que son réglage mobile). Le C# demande le SDK .NET 9, non installé.

