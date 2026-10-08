# La feuille de route vivante — jusqu'à la v2 (la liste à 100 %)

*Écrite en S693, 2026-10-08, à la demande de l'utilisateur (« une vive roadmap pour me projeter »). **Tenue à jour à chaque lot.***
L'historique détaillé est dans la [FEUILLE-DE-ROUTE](../FEUILLE-DE-ROUTE.md) ; l'état point par point dans le
[TABLEAU-DE-BORD](TABLEAU-DE-BORD.md) ; l'ordre des campagnes dans le [plan de complétion](PLAN-COMPLETION-S475.md).

## Où l'on en est

- **133 points**, dont **10 validés**, 108 partiels, 15 absents.
- **Le rythme mesuré** : de S640 (2026-10-07, 13 h) à S693 (2026-10-08, 14 h), 53 sessions en 25 h, soit **environ 2 sessions par heure**
  quand on enchaîne.
- **Les estimations du plan (S475)** comptaient environ 300 sessions. L'expérience les double : la campagne K3 en prévoyait 30, elle en
  a pris environ 35 et n'est pas finie. **Il reste environ 500 à 650 sessions.**

**Le calendrier, selon le régime de travail :**

| régime | sessions par jour | jusqu'à la v2 |
|---|---|---|
| en continu, jour et nuit | ≈ 45 | ≈ 2 semaines |
| la journée (≈ 10 h), les calculs longs lancés au moment où ils servent | ≈ 20–25 | ≈ 4 à 5 semaines |
| quelques heures par jour | ≈ 8 | ≈ 2 à 3 mois |

## Les phases

Chaque phase finit par **un jalon visible** : une séance visuelle où l'utilisateur juge (les verdicts R).

| phase | campagnes | ce qui sera fait | sessions | jalon visible |
|---|---|---|---|---|
| **A. La plage complète** (maintenant) | K3, ADR-275 | le relais au rivage fini ; le LOD étapes 1 à 3 (la bande 3D qui naît et meurt avec la vague, la 3D rallumée autour d'un corps) ; le porteur dispersif du large (Serre–Green–Naghdi, 1D fait en S694), qui lève aussi A234 | ≈ 25 | **une vague de bout en bout** : la houle du large, le déferlement en 3D, le ressaut et l'écume en 2D, la remontée sur le sable |
| **B. L'eau qui se déchire** | K2, fin de K3 | les éclaboussures détachées, les gerbes, les bulles, la cavité, le rouleau qui agit ; la marée sur la côte, le précalcul côtier par phase de marée | ≈ 80 | un plongeon, un rocher dans le ressac, un objet qui tombe à l'eau |
| **C. Le temps réel** | K5, K8 | le LOD complet ; l'activation par présence, agitation et contact ; les budgets, la dégradation, 60 images/s ; **la carte graphique pour APIC** | ≈ 120 | **une scène jouable** : un joueur sur une plage, la 3D qui suit ses gestes, au budget |
| **D. Les eaux du monde** | K4, fin de K6 et K7 | les lacs, rivières et canaux, l'éditeur de rivières ; les grands navires, les brèches, les contenants réels, le réseau en charge | ≈ 90 | une rivière qui descend à la mer ; un navire qui prend l'eau |
| **E. La planète** | K10, K11, K9 | les régions de mer, les tsunamis, les très grands événements ; la cuisson reproductible, le précalcul côtier stocké, le terrain de DyingStar ; le réseau, la sauvegarde, le serveur | ≈ 110 | la mer d'une planète entière, son rivage cuit partout |
| **F. La fin de l'eau, et le jeu** | K12, K13, K1 | l'écume (son visuel à valider), la part de l'eau dans la météo, la glace et la vapeur, le son ; le harnais, les 23 cas canoniques ; **l'intégration dans une copie locale de DyingStar** ; le rendu final | ≈ 110 | **la v2** : l'eau entière dans DyingStar |

Les séances visuelles reviennent aussi entre les jalons, quand une avancée se montre.

## Ce qui dépend de l'utilisateur

- Les verdicts des séances visuelles : R42 **reçu** le 2026-10-08 (la côte qui déferle) ; le prochain au jalon de la phase A.
- Les choix qu'une session ne peut trancher seule : plus tard, le budget de la carte graphique et la cible de livraison. (La file de
  nuit : non, le 2026-10-08 — les calculs se lancent quand ils servent.)

## Les risques connus

- **Le coût de la 3D** : il commande le temps réel (phase C). Les leviers sont nommés dans
  [ANALYSE-PHASES-S690](ANALYSE-PHASES-S690.md) ; la carte graphique pour APIC est le plus gros.
- **Les raccords entre solveurs** (S685–S693) : chacun se juge d'abord entre deux copies du même solveur (ADR-273).
- **Les calculs longs** : lancés quand ils servent, explorés à 5 cm d'abord, montrés et mesurés (ADR-274).
