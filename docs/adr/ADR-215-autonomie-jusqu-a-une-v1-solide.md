# ADR-215 — Autonomie jusqu'à une v1 solide

- **Statut : actée**, S454, 2026-10-03 ; **décision de l'utilisateur** : *« je souhaite que tu gagnes en autonomie, j'aimerais que tu
  ne t'arrêtes pas de travailler jusqu'à une v1 solide visuellement et physiquement ; prends les décisions — s'il ne s'agit pas d'une
  version finale, fais ce qui te semble bon »*.
- **Amende** [ADR-213](ADR-213-accelerer-tolerance-plafond-rituel-bancs.md) (accélérer) et le rôle tenu jusqu'ici, qui renvoyait à
  l'utilisateur les arbitrages ; **ne change pas** la méthode (plan commité seul, critères écrits avant, preuve, un pas par commit) ni
  les garanties de fonctionnement (conservation exacte, production au bit, refus atomique).

## 1. Décisions

**D1 — Les sessions s'enchaînent.** Une session finie ouvre la suivante sans attendre de « Continue ». Les images sont montrées à
l'utilisateur au passage ; son verdict, s'il vient, est traité en priorité ; sans verdict, le travail continue.

**D2 — Les arbitrages techniques se tranchent ici.** Ce que [ADR-213](ADR-213-accelerer-tolerance-plafond-rituel-bancs.md) D1 et D2
renvoyaient encore à l'utilisateur (un écart au-delà de la tolérance, un problème plafonné dont l'aval attend) se décide dans la
session, par écrit dans la preuve : ce qui est choisi, pourquoi, ce que le choix coûte, comment revenir dessus. Restent à
l'utilisateur : **le jugement visuel** de la v1 et tout ce qui touche au périmètre du jeu (ADR-197, ADR-198).

**D3 — La v1 solide.** Le travail s'arrête — et seulement alors, hors impasse réelle — quand **une scène vivante** tient dans la
fenêtre de l'afficheur, sur la carte :

| | exigence | mesure |
|---|---|---|
| **physique** | une étendue d'eau à l'échelle d'une scène (4 m au moins, maille de 5 cm) ; un corps qui y tombe, relançable ; cavité, couronne, jet portés par la bande APIC, les colonnes ailleurs ; une houle | masse exacte ; **aucun refus ni arrêt sur 60 s simulées** enchaînant plusieurs sauts ; la houle propagée sans s'amortir de plus de 10 % sur 10 s |
| **temps réel** | la scène avance avec le temps | simulé / réel ≥ 0,9 ; une image en 33 ms au plus au 99ᵉ centile |
| **visuel** | la surface continue (R37) avec la lumière de l'eau reçue (R20, R24 : couleur, ciel, Fresnel) | les images montrées ; **le jugement de l'utilisateur** |

Ce qui n'y est pas : la coque et la gerbe d'étrave (C10-3), la lame du déversoir (C10-4), Godot (C11), l'écume (suspendue), la
pluie dans la scène — la suite après la v1.

**D4 — L'ordre.** (1) la scène de 4 m tient longtemps (le défaut d'après t = 4, S454) ; (2) le temps réel à 4 m ; (3) la houle ;
(4) la lumière de l'eau ; (5) la scène `--v1` et ses mesures. Chaque pas est une session ou moins, au plan commité seul.

## 2. Conséquences

- REPRISE, `Session suivante`, porte toujours la prochaine étape de D4 : une reprise après coupure continue sans question.
- Le rituel reste celui d'ADR-213 D3 (allégé, registres par lots de trois).
- Revenir dessus : l'utilisateur peut à tout moment arrêter, réorienter ou juger ; son message passe avant le plan.

**Note, 2026-10-03 (S458).** Les cinq étapes de D4 sont faites (S454–S458) : la scène `--v1` tient les exigences mesurables de D3 —
60 s de sauts sans arrêt, masse exacte, la houle sans décroissance, 0,99 du temps réel, 22,8 ms au 99ᵉ centile
([C10-SCENES-S454](../validation/C10-SCENES-S454.md) §7). Reste **le jugement de l'utilisateur** (R38, REVUE-VISUELLE §43) : la v1
est solide à son verdict.

**Note, 2026-10-03 — R38 reçu** : *« Correct pour une V1 »*. La v1 de D3 est atteinte ; le travail en continu de D1 s'achève avec elle. La suite
(C10-2, la bande dans la mer δ ; C10-3, la coque) reprend au rythme des sessions, à la demande de l'utilisateur.
