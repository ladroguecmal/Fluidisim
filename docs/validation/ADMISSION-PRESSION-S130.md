# S130 — Admettre une source sans mentir sur le champ publié

2026-09-10. Suite de S129-1. [ADR-086](../adr/ADR-086-admission-dynamique-de-la-pression.md).

## 1. Inventaire : ce que l'état actuel impose

Le contrôleur d'ADR-078 emprunte un journal **figé** : `journal: &'v Journal`. Pour changer
l'admission, il faut libérer le contrôleur, donc perdre la publication en cours. Ce que la
lecture établit avant toute décision :

| fait | conséquence pour l'admission dynamique |
|---|---|
| `Controller` détient `&Journal` | admettre exige `&mut Journal` ; sept appelants, tous en tests ou exemples |
| `admit_authenticated` rend `Added`, `Unchanged`, `Epoch`, `Conflict`, `Pending`, `Full` | quatre refus et **deux succès de nature différente** |
| `Full` met la source **en attente** et la conserve | l'attente survit à l'échec ; elle n'est pas perdue |
| `from_journal` refuse tout journal dont l'attente est non vide | **une saturation bloque aussi les changements d'instant** |
| le journal n'offre aucun retrait | après une admission réussie, revenir en arrière demande une opération qui n'existe pas |

Deux points méritent d'être soulignés parce qu'ils décident de la forme de la réponse.

**L'emprunt mutable garantit l'invariant demandé, structurellement.** La consigne exige
« jamais `Unchanged` sur un journal différent ». Un compteur de version le détecterait après
coup ; l'emprunt mutable l'interdit d'avance, puisque personne d'autre ne peut muter le journal
pendant la vie du contrôleur. C'est la même préférence qu'ADR-079 et ADR-080 — tenir une
propriété par structure plutôt que par vigilance.

**Les deux succès d'`admit_authenticated` n'ont pas les mêmes conséquences.** `Added` change le
journal, donc le champ publié ne le représente plus : il faut recalculer. `Unchanged` signifie
que la source était déjà là, à l'octet près — le champ reste exact, et recalculer serait du
travail pur. La transaction doit distinguer les deux, sans quoi elle paierait une préparation
complète à chaque réadmission d'une source connue.

## 2. Décision

Voir [ADR-086](../adr/ADR-086-admission-dynamique-de-la-pression.md).

## 3. Construction et réception

*(à compléter)*
