# Boussole — pourquoi, vers quoi, selon quelles décisions

*Ouverte en S480, 2026-10-04, à la demande de l'utilisateur* : *« avoir en contexte les choses essentielles nouvelles mais aussi les
intentions initiales […] pour ne pas travailler dans le flou »*. **Deux pages, lues en premier** (AGENTS.md). Elle ne remplace rien :
chaque ligne renvoie à sa source ; une décision nouvelle s'y ajoute en une ligne, le jour où elle est prise.

## Pourquoi ce projet existe

Un **système d'eau temps réel** pour un jeu de très grande échelle — mers, lacs, rivières, contenants, inondations, interactions
avec les corps —, dont le principe d'origine est : *l'eau reste aussi simple que possible tant que cela n'altère pas sa
crédibilité ; la simulation physique n'apparaît que localement, quand l'interaction l'exige* ([intentions d'origine](docs/sources/systeme_eau_architecture_globale.md) §1,
non modifiables). Le critère principal est le **rendu perçu** : pas de retard, des mouvements continus, des effets conservés.

**Le jeu est DyingStar** (<https://github.com/DyingStar-game>), un MMO spatial open source — Godot 4.5, C#, double précision, Jolt,
Wwise, serveur Horizon en Rust, planètes de 6 356 km. **C'est une surprise pour son équipe** : aucun contact, rien de publié, ses
dépôts se lisent seulement ([ADR-219](docs/adr/ADR-219-reponses-du-2026-10-04.md)).

## Vers quoi

**Le système de l'eau est fini quand la [liste du projet fini](docs/LISTE-PROJET-FINI.md) est validée à 100 %** — 120 points sur leur
périmètre final, pas moins ; la liste peut grandir ou changer en chemin, tracé ([ADR-218](docs/adr/ADR-218-le-systeme-de-l-eau-complet.md)).
L'ordre : le [plan de complétion](docs/registres/PLAN-COMPLETION-S475.md), treize campagnes K1 à K13 ; **où on en est** : le
[tableau de bord](docs/registres/TABLEAU-DE-BORD.md) (généré) ; **la prochaine session** : le jeton de [REPRISE](REPRISE.md).

## L'architecture, en quatre lignes

- **Quatre couches** : B le fond analytique (la mer, la houle), W les ondes, δ la perturbation volumique locale, V les volumes finis
  ([ADR-001](docs/adr/ADR-001-decomposition-en-couches.md)).
- **Trois systèmes** : A haute mer (B + W), B volumique 3D (δ), C leur couplage ([ADR-178](docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md)).
- δ : colonnes par défaut, **APIC** là où la surface n'est plus un graphe ([ADR-186](docs/adr/ADR-186-apic-seconde-representation.md)) ;
  sur la carte graphique en production, une référence CPU pour la preuve.
- Le cœur en Rust sans dépendance (`code/`), l'afficheur GPU pour les bancs (`viewer/`), **le rendu final dans Godot** (`godot/`).

## Les décisions en vigueur — une ligne chacune

| | décision | source |
|---|---|---|
| ambition | complète ; une priorité ou un budget ne retirent rien ; seul l'utilisateur réduit | ADR-127 |
| fin | la liste à 100 % | ADR-218 |
| machine | **ce PC seul** — référence, cible de livraison ; les points à second matériel reformulés sur lui | ADR-174, ADR-219 D2 |
| dépôt | **jamais poussé**, aucun dépôt distant | ADR-174, REPRISE §9 |
| moteur | Godot moteur du jeu entier ; le rendu final de l'eau dans Godot, l'afficheur reste le banc | ADR-192, ADR-197 |
| alternance | une session de rendu, une de physique | ADR-191 |
| autonomie | les sessions s'enchaînent sans « Continue » ; les arbitrages techniques se tranchent ici, par écrit | ADR-215 D1, D2 |
| visuel | mesurer plutôt que regarder : le banc visuel, des vidéos de référence, à traitement égal (le même codec) | ADR-216 |
| type d'eau | une option d'édition de la carte, qui varie dans l'espace | ADR-217 |
| atmosphère | nuages, climat, météo : après l'eau ; la part de l'eau (ce qu'elle reçoit du temps qu'il fait, son son) reste dans les 100 % | ADR-217 D3, ADR-218 D4 |
| réseau | le nôtre, validé client et serveur sur ce PC, prêt pour Horizon | ADR-219 D1 |
| terrain | celui de DyingStar (tuiles HEALPix) ; une carte de hauteurs qui l'imite pour nos scènes | ADR-219 D4 |
| écume | reprise, d'après les vidéos V2 et V3 | ADR-219 D5 |
| hors périmètre | les eaux souterraines (5.11) | ADR-197 D4 |
| air | poches adiabatiques (T2), jamais un solveur diphasique | ADR-015, ADR-220 |
| méthode | elle se révise elle-même toutes les cinq sessions ; une décision technique que la mesure contredit est remplacée sans demander ; à l'utilisateur : ambition, rendus, téléchargements, configuration de sa machine | ADR-222 |
| structure | la boussole d'abord ; l'état dans les registres générés ; les calculs longs par `calcul.py` ; le rituel par `rituel.py` | ADR-221 |

## Ce que l'utilisateur décide, et ce qui attend de lui

- **Il juge** : les rendus, aux jalons (avec les mesures du banc) ; le périmètre du jeu ; toute réduction d'ambition.
- **Il accorde** : chaque téléchargement — au moment de 13.4 : **Godot 4.5 en double précision et une copie locale de DyingStar**.
- **En attente de lui aujourd'hui** (S481, [ADR-222](docs/adr/ADR-222-la-methode-se-revise-elle-meme.md) §3) : une **relance planifiée**
  (« Reprends le projet » quand le jeton est libre ou interrompu) ; les **téléchargements anticipés** (Godot 4.5 double, DyingStar).
  Une question nouvelle s'inscrit ici, une ligne, et se retire quand il a répondu.

## Comment on travaille — l'essentiel

- Le plan d'une session est écrit et committé **avant** le travail, ses critères avec ; **il déclare ses entrées et comment il les
  vérifie** (S480 — la leçon de S472, où une mer lisait le détail d'une autre).
- Une étape par commit `S<n> P<k> — …` ; la preuve dans `docs/validation/` ; le rituel de fin par `python outils/rituel.py fin`.
- Une propriété numérique revendiquée s'écrit en code et se mesure, contre une référence publiée.
- Les calculs longs se lancent par `python outils/calcul.py lancer` — hors de la session (WMI, S481), ils survivent à la conversation ; leur trace est dans
  [`notes/CALCULS.md`](notes/CALCULS.md).

## Les pièges connus

- PowerShell : jamais `Get-Content`/`Set-Content` sur les fichiers du dépôt (l'encodage) ; écrire par Python.
- Jamais `cargo fmt` (il réécrit tout le cœur).
- Un export de mer de l'afficheur a son propre fichier de détail ; une mer copiée sans le sien lit celui d'une autre (S472–S473).
- Une vidéo de référence est compressée : ses grandeurs fines et temporelles ne se comparent qu'après le même codec (ADR-216 D8).
- Le dossier temporaire d'une conversation disparaît avec elle : rien de durable n'y va.
