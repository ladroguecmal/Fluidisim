# ADR-108 — Pas de garde-fou sans tolérance déclarée

Statut : acté, S157, 2026-09-10, délégation technique. Complète ADR-107 et traite A214 sans la
clore ; aucune décision antérieure n'est modifiée, aucun ADR réécrit.

## Constat

ADR-107 a établi que le domaine d'un sillage se déduit de sa recette, et **A214** a nommé ce qui
manquait pour en faire un garde-fou : la loi en durée n'était encadrée qu'en deux points. S157
devait la remplacer par une loi. **Elle n'existe pas**, et c'est le résultat de la session.

Trois observables ont été essayés, chacun écarté pour une raison distincte, et chaque raison est
elle-même un enseignement.

| observable | verdict |
|---|---|
| écart L2 entre profils, résolution contre sa voisine | attribue la panne à la mauvaise des deux quand c'est la plus fine qui défaille |
| écart L2 contre une référence fixe | **non monotone** — 3 à 5 reculs sur 15 intervalles — donc pas de dichotomie ; et un genou simultané à 20 s pour deux résolutions dont les périodes diffèrent d'un facteur deux, signature d'un artefact de **fenêtre d'échantillonnage** et non du champ |
| excès d'élévation en champ proche, en rapport | propre au mécanisme, quasi monotone après lissage — c'est celui qui est retenu |

Avec le troisième, trois seuils sont obtenus proprement. Aucun ne donne de loi.

| tolérance | radial 64 | radial 128 | radial 256 | exposant en `radial` |
|---:|---:|---:|---:|---:|
| 1,10 | 16 s | 36 s | 56 s | 0,90 |
| 1,25 | 16 s | 40 s | 58 s | 0,93 |
| 1,50 | 26 s | 42 s | 62 s | 0,63 |
| 2,00 | 32 s | 46 s | au-delà de 64 s | — |

**L'exposant dépend de la tolérance**, et les deux rapports d'une même ligne diffèrent d'un
facteur 1,5, ce qu'une loi de puissance interdit. La dégradation est **graduelle** : il n'y a pas
d'instant de rupture dont on pourrait mesurer la position, seulement une courbe qui monte, et
l'instant qu'on en tire est celui de la tolérance qu'on a choisie.

L'épreuve du `sigma` — trois sources à produit réduit `sigma·cutoff = 6` constant, donc même forme
spectrale à l'échelle près — confirme et aggrave.

| sigma | cutoff | radial 64 | radial 128 | radial 256 |
|---:|---:|---:|---:|---:|
| 0,25 m | 24 | 4 s | 13 s | 21 s |
| 1 m | 6 | 20 s | 36 s | 56 s |
| 4 m | 1,5 | 32 s | au-delà de 64 s | au-delà de 64 s |

Le groupement `t·dk` s'étale sur un facteur 4. Le groupe physiquement motivé
`t·½√(g·sigma)·dk` s'étale sur un facteur 2,5 — mieux, pas constant. Et l'exposant en `radial`
vaut 0,74 à sigma 1 m contre 1,20 à sigma 0,25 m : **il dépend de la source.**

Un ajustement libre `t = C·sigma^p·dk^q` rassemble les sept points à un facteur 1,38 et donne
`q = −1,04`, séduisant parce que proportionnel à la période spatiale. Il ne prouve rien : le plan
d'expérience est **dégénéré**, `dk = 6/(sigma·radial)` par construction, donc `sigma` et `dk` ne
sont pas des variables indépendantes et leurs exposants ne sont pas identifiables. Trois
paramètres pour sept points liés ajustent n'importe quoi.

## Décision

**1. Ne pas encoder de garde-fou.** Refuser à l'exécution un échantillonnage hors domaine gèlerait
dans l'API une tolérance que **personne n'a spécifiée**. Le tableau ci-dessus montre que l'instant
limite change du simple au double selon qu'on tolère 10 % ou 100 % d'excès ; choisir pour le
consommateur serait choisir à sa place la chose la plus lourde de conséquences.

**2. Publier une estimation conservatrice, dans la documentation et non dans le code.** Un appelant
qui veut une borne peut prendre la plus petite valeur observée du groupe physique :

> durée sûre ≈ 1,17 / (½·√(g·sigma)·(cutoff/radial))

Elle vaut 4 s pour sigma 0,25 m et radial 64 ; 20 s pour sigma 1 m et radial 128 ; et elle sous-
estime jusqu'à un facteur 2,5 les configurations les plus favorables. **C'est une estimation
accompagnée de sa dispersion, pas une loi**, et elle est écrite ainsi partout où elle apparaît.

**3. Nommer les trois manques, dont deux sont fermés par des décisions déjà prises.**

| ce qui manque | état |
|---|---|
| une **tolérance déclarée** par un consommateur | ouvert, et c'est le seul des trois qui ne coûte rien |
| une fenêtre d'observation **au-delà de 64 s** | fermé par ADR-106 ; la censure frappe sigma 4 m |
| des résolutions **au-delà de 512** | fermé par ADR-097 ; le bras de levier reste de deux doublements |

**4. A214 reste ouverte, mais change de nature.** Elle disait « il manque une mesure ». Elle dit
désormais « il manque une **spécification**, et deux bornes de grammaire limitent ce qu'on peut
mesurer ». Ce n'est pas la même chose à faire, et c'est ce qu'une session de mesure a rapporté.

## Ce que cette décision ne dit pas

- **Elle n'affirme pas qu'aucune loi n'existe.** Elle constate qu'aucune ne se laisse établir dans
  la fenêtre accessible — deux doublements de résolution, deux décades de sigma, une observation
  bornée à 64 s. Un plan d'expérience **non dégénéré**, où `cutoff` et `sigma` varieraient
  séparément, dirait peut-être autre chose ; il demande de sortir du produit réduit constant, donc
  de comparer des formes spectrales différentes, et ce n'est pas la même mesure.
- **Elle ne remet pas en cause ADR-107.** Le mécanisme — périodicité de pas radial, alias
  angulaire — reste établi ; c'est sa *quantification* qui échoue.
- **Elle ne touche ni le code, ni les invariants.** Aucun garde-fou n'étant ajouté, rien ne change
  en production.
