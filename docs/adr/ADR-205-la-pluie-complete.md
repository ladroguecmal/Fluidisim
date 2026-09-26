# ADR-205 — La pluie complète : l'inventaire de ce qui manque, son ordre, ses sources

- **Statut : actée**, S380, 2026-09-26. **Décision de l'utilisateur** : *« Pas de solveur continue la pluie ajoute les
  manquants »* (après R28, *« Parfait »*) ; l'inventaire, l'ordre et les sources sont du projet (ADR-028).
- **S'appuie sur** [ADR-202](ADR-202-niveau-de-detail-des-contenants.md) D3 (effets factices au loin, impacts de pluie
  toujours factices), [ADR-203](ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) (bâches en temps réel ; la météo, aussi
  poussée que l'eau, **à la fin**), [ADR-204](ADR-204-la-pluie-arete-de-v.md) (la pluie dans V). Preuves de départ :
  [PLUIE-V-S378](../validation/PLUIE-V-S378.md), [RIDES-PLUIE-S379](../validation/RIDES-PLUIE-S379.md).
- **Diffère** la campagne du solveur volumique 3D temps réel inscrite en S379 : elle attend que la pluie soit complète.

## 1. Décisions

**D1 — « La pluie » est ce que l'eau reçoit du ciel et ce qu'on en voit ; « la météo » est ce qui décide où, quand et
combien.** La pluie se complète maintenant ; la météo reste à la fin (ADR-197 D5, ADR-203 D5). Frontière : tout ce qui
prend **une intensité donnée** (mm/h, éventuellement par lieu) et en tire un effet sur l'eau ou à l'image est de la pluie ;
ce qui produit cette intensité — nuages, fronts, prévision, précipitations en amont — est de la météo. Le ciel de pluie est
de la pluie (l'apparence d'un ciel couvert quand il pleut), le ciel qui change de lui-même est de la météo.

**D2 — L'inventaire, en trois familles, et le coût.**

| n° | pièce | famille | source physique ou de mesure | liste |
|---|---|---|---|---|
| 1 | **Les gouttes dans l'air**, près de l'œil : traînées | air | Marshall et Palmer (nombre, tailles), Atlas (vitesse), Garg et Nayar 2007 (apparence) | 8.4 |
| 2 | **L'extinction** au loin (le rideau, la visibilité) | air | `β = (π/2)·∫N(D)·D² dD`, efficacité d'extinction 2 | 8.8 |
| 3 | **Le ciel de pluie** : couvert, soleil voilé, éclairage diffus | air | ciel couvert normalisé de la CIE (Moon et Spencer 1942) | — |
| 4 | **Les gerbes** : couronnes, jets, gouttelettes des impacts | surfaces | Worthington ; seuil de Mundo, Sommerfeld et Tropea (1995) ; les mêmes impacts que les rides | 8.4 |
| 5 | **Les surfaces mouillées** : sol, margelles, murs — plus sombres, plus brillants | surfaces | assombrissement des matériaux mouillés (Lekner et Dorf 1988) | — |
| 6 | **Les reflets des objets** dans l'eau (pas seulement le ciel) | surfaces | réflexion en espace écran sur la profondeur, Fresnel déjà là | 8.5 |
| 7 | **Les scintillements** au loin (anneaux sous le pixel qui prennent le soleil) | surfaces | statistique discrète des facettes | 8.9 |
| 8 | **Les bâches sur les rides** : pas d'anneau sous une bâche | surfaces | l'exposition de V (ADR-204 D3), lue par le rendu | 8.9, 5.5 |
| 9 | **L'amortissement des capillaires du vent** par la pluie, en mer | surfaces | mesures en bassin (Tsimplis et Thorpe 1989 ; Poon *et al.* 1992) | 8.9 |
| 10 | **L'exposition calculée** depuis les objets posés (toit, bâche, arbre) | V | lancer de rayons vers le ciel, occultants mobiles | 5.5 |
| 11 | **L'absorption par le sol**, le ruissellement | V | infiltration (Horton ; Green et Ampt) | 5.5 |
| 12 | **Les flaques** : la pluie hors contenant, dans les creux | V | ADR-010 §5, nœuds de V dans les creux du terrain | 5.5 |
| 13 | **Le coût** des rides : une texture à moments par image | coût | LEAN, comme la queue FFT (S360) | 8.9 |

Hors de la pluie : l'intensité par lieu et dans le temps, la neige, la grêle (la météo) ; le son (à la fin, ADR-197 D5).

**D3 — L'ordre** : d'abord ce qui se voit le plus, une pièce ou deux par session, en alternant l'image et V quand une
pièce de V est prête : **1 et 2** (S380), **3** puis **4**, **5**, **8**, **10**, **6**, **7**, **9**, **11** et **12**
(qui demandent un terrain : ils prennent le sol simple des scènes, la topologie d'un territoire viendra après la météo),
**13** quand une scène de jeu en a besoin — l'utilisateur l'a dit en S379 : *« les LOD vont complètement bouleverser les
performances »*. L'utilisateur peut réordonner ; chaque pièce passe par sa revue.

**D4 — Chaque pièce a sa provenance et ses critères écrits avant**, comme S379 : un nombre qui vient d'une loi publiée
ou d'une mesure, un contrôle qui le retrouve sur l'image, le temps sec identique au bit, des photographies réelles
cherchées par nous, lues sans téléchargement, et le jugement de l'utilisateur. Un réglage à l'œil est dit comme tel.

## 2. Conséquences

- La file porte la campagne « pluie complète » en tête ; la campagne du solveur attend (feuille de route).
- Les pièces 10 à 12 font avancer 5.5 ; 1 et 4, le point 8.4 (gouttes, spray), encore *absent* ; les autres, des points
  de rendu déjà partiels (8.5, 8.8, 8.9). Le ciel de pluie et les surfaces mouillées n'ont pas de point à eux dans la
  liste : ils servent 8.10 (la crédibilité perçue).
