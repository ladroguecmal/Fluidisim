# ADR-012 — Ordonnanceur, budget et dégradation contrôlée

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §7, §28`, `architecture_globale §17`
- **Dépend de** : ADR-006, ADR-007

---

## 1. Décision structurelle

`zones_ouvertes §7` hésitait entre un gestionnaire global décidant de tout et des cellules locales
affinant les décisions. La question se dissout dès qu'on identifie la nature du problème :

> L'activation des domaines est un **problème de sac à dos sous budget**, résolu chaque tick.

Il n'y a donc ni arbre de décision distribué ni règles locales : il y a des **candidats qui
soumissionnent** et un **ordonnanceur qui alloue**.

```
Chaque tick :
  1. chaque candidat (domaine existant ou domaine potentiel) publie  (priorité P, coût estimé C)
  2. l'ordonnanceur trie par P/C décroissant
  3. il alloue jusqu'à épuisement du budget
  4. il distribue à chaque domaine retenu un budget_ms individuel
  5. chaque solveur respecte son budget (contrat ADR-007) — le dépassement est impossible
```

Le point 5 est ce qui rend le système sûr : **il n'existe aucun chemin par lequel l'eau puisse
provoquer un pic de frame**. C'est la propriété à défendre en priorité dans toutes les revues.

## 2. Priorité

```
P = W_gameplay · W_perception · W_urgence
```

| Terme | Composition | Remarque |
|---|---|---|
| `W_gameplay` | conséquence sur un acteur, sur un objet joueur, sur un objectif | domine tout le reste ; une eau invisible mais qui pousse un joueur reste prioritaire |
| `W_perception` | fraction d'écran occupée × visibilité (frustum, occlusion) × facteur de regard | **surface à l'écran**, pas distance : c'est la seule métrique qui se compare entre un splash proche et un déferlement lointain |
| `W_urgence` | 1/(temps estimé avant que l'absence du domaine devienne visible) | fait le lien avec ADR-013 |

`W_perception` construit sur la surface écran, et non sur la distance, est une correction directe
de `architecture_globale §7` : à budget égal, la distance conduit systématiquement à sur-servir le
proche insignifiant et à sous-servir le lointain spectaculaire.

## 3. Budget

Le budget est déclaré par profil, pas découvert. Structure imposée ; les valeurs sont des points
de départ **à mesurer** (benchmark B7), pas des engagements.

```
profil "cible haut de gamme" :
    cpu_sim_ms      = 2,0      // tick 30 Hz, hors thread de rendu
    gpu_sim_ms      = 2,5
    memoire_blocs   = 384 Mo
    paquets_W_max   = 4096
    v_noeuds_actifs = 2048
```

> **Correction (S05, écart R04).** Le profil déclarait aussi `domaines_max = 24`. C'était une
> erreur de nature : `domaines_max` n'est pas une ressource, c'est un **résultat**.
>
> ```
> domaines_max = min( memoire_blocs / taille_domaine ,  budget_ms / coût_domaine_ms )
> ```
>
> Les deux termes étaient contradictoires. La mémoire autorise ≈24 domaines de type bateau
> (SPEC-001 §2.4). Le temps, lui, donne **0,10 ms par domaine** pour 24 domaines dans 2,5 ms, soit
> de l'ordre de 4·10⁹ mises à jour de cellule par seconde de GPU pour les 432 000 cellules éparses
> d'un domaine à `dx = 0,10 m` — optimiste d'au moins un ordre de grandeur pour un solveur à
> surface libre avec projection de pression. Le budget mémoire autorisait environ six fois plus de
> domaines que le budget de temps ne pouvait en servir.
>
> `domaines_max` est désormais **calculé à l'initialisation** à partir de
> `SolverCaps::cost_per_block_ms` mesuré (ADR-007 §2), et n'apparaît plus dans le profil. Un profil
> ne déclare que des ressources ; toute capacité dérivée qu'on y inscrit finira par contredire les
> ressources qui l'entourent.

Deux règles de méthode :

- La cible de mesure est le **99ᵉ centile** de la contribution par frame, jamais la moyenne.
  Une moyenne à 2 ms avec un centile 99 à 9 ms produit un jeu qui saccade et un tableau de bord
  qui rassure.
- Le coût par bloc de chaque solveur est **mesuré en continu** et réinjecté dans l'estimation `C`.
  Un ordonnanceur qui planifie sur des coûts théoriques dérive dès la première optimisation.

## 4. Ordre de dégradation

`architecture_globale §17` propose : réduire la taille des domaines, puis la résolution, puis les
interactions lointaines, puis la fréquence, puis les particules.

Cet ordre est **contre-intuitif mais correct**, et il ne l'est que grâce à ADR-001 : en régime
perturbatif, rétrécir un domaine est visuellement gratuit (ADR-005 §5), alors que baisser la
résolution change l'aspect d'une gerbe. L'intuition habituelle — « couper les particules d'abord » —
serait ici moins bonne. L'ordre est donc conservé, avec deux précisions.

| Rang | Action | Perte perceptuelle | Gain |
|---|---|---|---|
| 1 | Rétrécir l'emprise des domaines **non focaux** | quasi nulle | fort |
| 2 | Réduire spray/écume secondaires sur les domaines non focaux | faible | moyen |
| 3 | Réduire la fréquence de pas des domaines non focaux (avec interpolation) | faible | moyen |
| 4 | Descendre `dx` d'un niveau sur les domaines non focaux | visible de près | fort |
| 5 | Détruire les domaines non focaux → repli sur W | visible seulement si l'on regarde | très fort |
| 6 | Réduire le nombre de paquets W (fusion des plus faibles) | visible de loin | moyen |
| 7 | Réduire le nombre de composantes de B | visible partout | faible |

**Jamais dégradé** : la couche V, les composantes longues de B qui portent la flottabilité,
l'échantillonnage de flottabilité, la couche W répliquée. Ce sont les données de gameplay.

> **Corrections S05, issues de la revue croisée.**
>
> **Rang 1 — le rétrécissement emprunte un chemin dégradé (écart R06).** Rétrécir un domaine est
> visuellement gratuit (ADR-005 §5), mais pas *calculatoirement* : la procédure normale mesure un
> flux, l'agrège par secteurs et émet des paquets — c'est-à-dire du travail supplémentaire à
> l'instant précis où le budget manque. Sous contrainte de budget, le rétrécissement **saute la
> transduction** et se contente d'amortir, en assumant la perte d'énergie. Cela répond au passage
> à la question laissée ouverte en ADR-005 §7.4 : la transduction ne perd rien en régime normal,
> et tout sous pression.
>
> **Rang 6 — l'élagage de W est restreint (écart R05).** « Réduire le nombre de paquets W » ne
> s'applique qu'à `W_local` et aux paquets `W_rep` déjà passés sous le seuil de pertinence
> gameplay. Un paquet répliqué au-dessus du seuil n'est **jamais** élagué, quel que soit le profil
> de qualité : deux joueurs verraient sinon deux sillages différents alors qu'une conséquence
> gameplay — la détection — en dépend. Voir ADR-021 §4.

**Notion de domaine focal** : celui qui contient ou touche l'acteur du joueur local, ou qui occupe
plus de X % de l'écran. Un seul domaine focal par joueur, protégé des rangs 1 à 5.

## 5. Régulation et anti-pompage

Le pilotage se fait par un **régulateur PI** sur une manette de qualité globale `q ∈ [0,1]`,
alimenté par le budget réellement consommé filtré passe-bas (τ ≈ 0,5 s).

- Descente autorisée rapide (une frame) : il faut protéger la fréquence d'images.
- Remontée **lente et rampée** (≈1 s) : c'est la seule protection contre le pompage
  dégradation/restauration, qui est bien plus visible que la dégradation elle-même.
- Toute décision de dégradation est **engagée pour au moins 30 frames**.

## 6. Dépassement autorisé

`architecture_globale §17` autorise le dépassement temporaire pour un événement critique. Formulé
proprement : le budget porte une **réserve d'événement** (proposition : +50 % pendant 0,5 s
maximum, avec un temps de rechargement de 5 s), utilisable uniquement par un domaine dont
`W_gameplay` est maximal. Sans rechargement, la réserve devient le budget nominal.

## 7. Ordonnancement temporel

- Tick de simulation **fixe à 30 Hz**, indépendant du taux d'images. Le rendu interpole.
- δ tourne sur un fil de travail, avec au plus **une frame de retard** ; l'ordonnanceur lance le
  travail du tick N pendant la frame N et consomme au début du tick N+1.
- B et W sont évalués à la demande, sans état, donc sans latence.
- Conséquence : la flottabilité (qui n'utilise que B+W, ADR-008) est **synchrone et sans retard**,
  alors que le visuel de δ a une frame de retard. La séparation d'autorité paie ici une seconde
  fois.

## 8. Ce qui reste ouvert

1. Toutes les valeurs numériques → benchmark B7, sur matériel cible.
2. Faut-il un budget séparé par joueur en écran partagé / serveur d'écoute ? Probablement oui.
3. Politique quand le budget est saturé par un seul événement légitime (bataille navale) :
   dégradation homogène ou sacrifice des acteurs distants ? Proposition : sacrifice des distants,
   à confronter au ressenti.
