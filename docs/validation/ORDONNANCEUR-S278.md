# S278 — l'ordonnanceur : ce qui décide qu'une zone est simulée

**Origine** : l'utilisateur a demandé si le système qui décide entre simulation volumétrique, haute
mer analytique et zone de transition existait. Il existait — **entièrement conçu depuis S01, jamais
écrit**. La bande δ que l'afficheur montre depuis S275 est câblée en dur : rien ne la décide, ne la
déplace ni ne l'éteint. Ce lot écrit la première pièce, et seulement elle.

## 1. Ce qui est construit

`code/water-core/src/scheduler.rs`, sans dépendance, sans allocation à l'exécution.

Le problème a une forme, tranchée par [ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md)
§1 : **l'activation des domaines est un sac à dos sous budget, résolu chaque pas**. Des candidats
soumissionnent `(priorité P, coût C)`, l'ordonnanceur trie par `P/C` décroissant, alloue jusqu'à
épuisement, et distribue à chaque retenu son `budget_ms`.

- **Priorité** `P = W_gameplay · W_perception · W_urgence` (ADR-012 §2), la perception en surface
  écran et non en distance.
- **Score d'activation** : `s = P`, hystérésis 0,60 / 0,40 (ADR-013 §5). Voir
  [ADR-170](../adr/ADR-170-les-trois-poids-sont-bornes.md) — les deux ADR ne se raccordaient pas.
- **Temps** : durée de vie minimale 750 ms, délai d'extinction 1,0 s de séjour **continu** sous le
  seuil. Une horloge qui recule est refusée.
- **Budget** : le budget donné est le coût annoncé, pas une part du reliquat. Un budget est une
  **borne**, pas une enveloppe à consommer ; la somme des bornes ne dépasse donc jamais le profil.

Le tri compare les rapports **en croix** — `P_a·C_b` contre `P_b·C_a` — donc sans division, sans
cas particulier pour un coût nul, et les égalités se départagent par identité : la décision ne
dépend pas de l'ordre de soumission.

## 2. Le banc — `code/water-core/examples/ordonnanceur_s278.rs`

Un bateau longe cinq îlots serrés (−40 à +40 m) à 15 m/s pendant 40 s, à 30 Hz. Chaque îlot
soumissionne à chaque pas, avec les trois poids déclarés comme le ferait un hôte. Chaque domaine
coûte 0,8 ms annoncé ; le profil en accorde 2,0.

| grandeur | mesure |
|---|---|
| pas | 1 200 |
| domaines vivants **en même temps**, au plus | **5** |
| demande simultanée correspondante | 4,0 ms |
| **budget distribué, au plus** | **1,600 ms** pour 2,0 déclarés |
| transitions par îlot | **2, 2, 2, 2, 2** — une naissance, une mort |
| plus courte vie observée | 10 267 ms, pour 750 minimum |
| empreinte des décisions | `6aebff024c734fc9`, identique à deux exécutions |

Le banc **refuse de passer si le budget n'a pas été disputé** : tant que la demande simultanée
tient dans le profil, le sac à dos n'est jamais sollicité et la propriété principale n'est pas
éprouvée. La première version du scénario espaçait les îlots de 150 m — un seul domaine vivait à la
fois, tout passait, et rien n'était prouvé. C'est ce garde qui l'a révélé.

## 3. Ce que ça ne dit pas

**Les seuils ne sont pas calibrés.** 0,60 / 0,40, 750 ms, 1,0 s sont les valeurs de départ
d'ADR-013 §5, explicitement « toutes à calibrer ». Le banc **B8** qui devait le faire n'existe pas.
Ce lot montre que le mécanisme tient et ne bat pas ; il ne montre pas que ces nombres sont les bons.

**Le scénario est déclaré, pas observé.** Les trois poids sont écrits à la main dans le banc. Aucun
hôte réel ne les produit encore, et `W_perception` n'a jamais été calculé depuis une vraie caméra.

**Le coût est constant dans le banc**, alors qu'ADR-012 §3 demande qu'il soit mesuré en continu et
réinjecté — « un ordonnanceur qui planifie sur des coûts théoriques dérive dès la première
optimisation ». Le branchement sur un coût mesuré reste à faire.

**Un gros candidat peut jeûner** : trop cher pour le reliquat, il est sauté tant que de petits se
présentent. Limite connue du glouton, non éprouvée ici.

**Un domaine vivant mais non financé reste vivant** et ne tourne simplement pas ce pas. Ce qu'il
devrait devenir après plusieurs pas sans budget relève de la dégradation (ADR-012 §4, sept rangs),
qui n'est pas écrite.

**Rien de la forme des domaines n'est fait** : grille de référence, blocs épars, fusion et
séparation géométriques (liste 1.5 et 1.6), régime substitutif et sa restauration depuis graine
(ADR-001 §3.3, dont le critère `max|δ| > 0,35·Hs_local` reste proposé et jamais calibré).
L'ordonnanceur décide **qu'un** domaine vit et avec quel budget — pas où il est, ni quelle forme
il prend.

**Il n'est branché sur rien.** La bande δ de l'afficheur reste câblée en dur ; aucun consommateur
ne passe encore par l'ordonnanceur.
