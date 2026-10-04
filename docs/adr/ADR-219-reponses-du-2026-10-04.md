# ADR-219 — Réponses du 2026-10-04 : notre réseau, ce seul PC, le jeu DyingStar, l'écume par les vidéos

- **Statut : actée**, S476, 2026-10-04 ; **réponses de l'utilisateur** aux six faits du
  [plan de complétion](../registres/PLAN-COMPLETION-S475.md) §3, telles qu'écrites :

  > 1. je suis d'accord
  > 2. Je ne peut pas donc uniquement c'est ordinateur.
  > 3. Le jeu sera https://github.com/DyingStar-game mais je veut faire une suprise à l'équipe.
  > 4. j'ai rien en tête
  > 5. oui
  > 6. on

- **Précise** [ADR-218](ADR-218-le-systeme-de-l-eau-complet.md) (la liste à 100 %) et [ADR-197](ADR-197-reponses-du-2026-09-26.md)
  (D1 le réseau, D2 Godot moteur du jeu, D7 la seconde cible).

## 1. Décisions

**D1 — Le réseau est le nôtre (F1).** Un format de réplication par événements sources (10.1), validé entre des processus client et
serveur sur ce PC, sans infrastructure extérieure. Il est pensé pour se brancher sur le serveur du jeu (D3 : Horizon, en Rust, comme
notre cœur).

**D2 — Un seul ordinateur, celui-ci (F2).** Les points qui demandaient un second matériel ou un serveur réel sont **reformulés**, par
cette décision de l'utilisateur (ADR-218 D2 : un point s'allège par lui) ; chaque reformulation est écrite dans la liste, au point :

| point | énoncé d'avant | sur ce seul PC |
|---|---|---|
| 1.7, 10.3 | déterminisme bit à bit **entre plateformes** | entre les **chemins d'exécution** de ce PC : la référence sur le processeur contre la carte graphique, les deux pilotes graphiques (Vulkan et DirectX 12), les compilations optimisée et de mise au point, Godot en simple et en double précision ; le calcul en entiers (la masse en quanta, l'horloge) indépendant de la plateforme par construction, prouvé par des tests qui ne dépendent pas du matériel |
| 10.2 | le serveur n'exécute que V, **un serveur réel** | un **processus serveur sans fenêtre** sur ce PC, puis le serveur du jeu en local |
| 9.10 | adaptation **au matériel** | des profils mesurés sur ce PC en bridant le budget, la résolution et le pilote — l'adaptateur graphique logiciel de Windows (WARP) comme matériel faible |
| 11.5 | le matériel de livraison **et une seconde cible** | ce PC est la cible de livraison (ADR-174 : la référence) ; la seconde cible devient le bridage de 9.10 |

**D3 — Le jeu est DyingStar (F3)** — <https://github.com/DyingStar-game> — **et c'est une surprise pour son équipe.** D'où :

- **aucun contact** : ni question, ni ticket, ni fourche, ni étoile, ni message ; les dépôts publics du jeu se **lisent** seulement ;
  rien du système de l'eau ne se publie (le dépôt n'est jamais poussé ; L-règle de toujours) ;
- **ce que les dépôts publics disent** (lu le 2026-10-04) : Godot **4.5**, **C#**, **double précision** ; physique **Jolt**, gravité
  globale nulle (un jeu spatial : la gravité vient des planètes) ; audio **Wwise** ; serveur **Horizon**, en **Rust** ; planètes à
  l'échelle de la Terre (rayon 6 356 km), terrain découpé en tuiles HEALPix (arbre de profondeur 14, tuiles de 32 points), hauteurs
  tirées de données QGIS et de reliefs procéduraux ; des nuanceurs d'eau simples (océan, rivière, lacs — dont acide et méthane) ;
  licence AGPL-3.0 ; la documentation de l'équipe est en français ;
- **ce qui en découle** : le consommateur des requêtes de jeu (9.1, 9.4, 10.9) est DyingStar — d'ici là, un **jeu d'essai** dans
  Godot qui en reprend la pile (4.5, double précision, Jolt) ; l'intégration finale se fait dans une **copie locale** du jeu, jamais
  publiée par nous ; un point s'ajoute à la liste (D7).

**D4 — Le terrain est celui du jeu (F4 : « rien en tête »).** L'eau lit les hauteurs du terrain de DyingStar (ses tuiles HEALPix) ; pour
nos scènes, un terrain par carte de hauteurs, dans un format qui imite le sien (12.4).

**D5 — L'écume reprend, d'après les vidéos V2 et V3 (F5 : « oui »).** La suspension du 2026-09-26 (attente de photographies) est levée :
l'écume de déferlement (V2) et d'impact (V3) est la référence du banc (7.1, 8.4).

**D6 — Les verdicts aux jalons (F6).** Lu « on » comme « ok » : les verdicts visuels se demandent aux jalons du plan, avec les mesures du
banc.

**D7 — La liste grandit d'un point (ADR-218 D2 : ajouter ne demande pas l'utilisateur)** : **13.4 — L'eau dans le jeu** : le système
intégré à une copie locale de DyingStar (Godot 4.5, double précision, C#, Jolt, Horizon), sur une planète du jeu, sans régression
du jeu. Le périmètre passe à **120 points** (121 moins 5.11).

## 2. Conséquences

- Le registre des dépendances perd onze attentes extérieures (F1 à F5) ; les fronts se recalculent.
- 7.8 (l'audio de l'eau) se fera par **Wwise**, l'audio du jeu, et non par l'audio de Godot (ADR-197 D5 le disait faute de jeu).
- 5.7 (plusieurs liquides) prend un sens concret : les lacs d'acide et de méthane du jeu.
- 11.1 (le monde planétaire) a sa cible : une planète de 6 356 km, en double précision.
- Une copie locale de DyingStar et Godot 4.5 se téléchargeront le moment venu, **avec l'accord de l'utilisateur** à ce moment-là.

*Note corrective du 2026-10-05 (S482)* : DyingStar est passé à **Godot 4.7** (son propre build mono en double précision, `DyingStar-game/godotandaddons`) ; « Godot 4.5 » ci-dessus se lit 4.7. Le moteur et une copie de DyingStar sont téléchargés, avec l'accord de l'utilisateur ([ADR-222](ADR-222-la-methode-se-revise-elle-meme.md), note).
