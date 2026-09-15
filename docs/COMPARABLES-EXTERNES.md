# Comparables externes

**Ce fichier est le porteur unique des systèmes du commerce ou de la recherche que ce projet a
regardés.** Un comparable par section datée ; jamais un document par session — deux fiches qui se
ressemblent divergeraient, et le dépôt a déjà forké trois fois pour cette raison (L137).

## La règle qui gouverne ce fichier

1. **Une documentation d'éditeur est une affirmation de fournisseur, pas une mesure de ce projet.**
   REPRISE §2 interdit de transformer un fait externe inconnu en hypothèse acquise, et
   [ADR-028](adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) rappelle qu'il n'y a pas d'autres équipes :
   un éditeur n'est pas un interlocuteur, c'est une publication.
2. **Aucun nombre lu ici n'entre dans le dépôt comme seuil.** I-14 exige une provenance ; la
   provenance « untel le fait » n'en est pas une. Un chiffre externe peut *motiver* une mesure, jamais
   la remplacer.
3. **Chaque affirmation porte son statut** : **documenté** (la source l'écrit), **déduit** (la source
   ne l'écrit pas mais l'implique), **non trouvé** (cherché, pas trouvé — et c'est une information).
4. Un comparable ne rouvre **aucune** ambition : [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md)
   exige une décision explicite de l'utilisateur, et ce qu'un autre système ne fait pas n'est pas un
   argument de réduction.

---

## Niagara Fluids, dont *Pyro* — Unreal Engine 5.x

*Lu le 2026-09-15 (S241), sur demande de l'utilisateur. Documentation publique d'Epic Games,
versions 5.8 des pages consultées. Aucun code source lu, aucun essai exécuté ici.*

### Ce que c'est

Une extension d'Unreal Engine qui ajoute des simulations de fluides **sur grille** à Niagara, son
système d'effets. Elle fournit des gabarits : gaz en 2D, gaz en 3D — c'est le domaine de ce qu'on
appelle *Pyro*, fumée, feu et phénomènes gazeux —, eau peu profonde en champ de hauteur, et liquide
particulaire. Les simulations s'exécutent **sur le GPU**.

### Ce que la documentation dit

| affirmation | statut | source |
|---|---|---|
| Les gabarits **2D sont destinés aux jeux**, les **3D aux cinématiques** | **documenté** | [Niagara Fluids](https://dev.epicgames.com/documentation/en-us/unreal-engine/niagara-fluids-in-unreal-engine) |
| Le gaz 3D coûte davantage en mémoire et en GPU, et convient aux « effets vedettes » ou aux cinématiques | **documenté** | [Fluid Simulation Overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/fluid-simulation-in-unreal-engine---overview) |
| La pression se résout par un procédé **itératif**, « plus d'itérations, simulation plus précise » | **documenté** | [Overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/fluid-simulation-in-unreal-engine---overview) |
| Les réglages exposés sont un **nombre d'itérations** (`Pressure Solve Iterations`) et un **facteur de relaxation** (`Pressure Relaxation`, 0–1, à garder proche de 1) | **documenté** | [Niagara Fluids Reference](https://dev.epicgames.com/documentation/unreal-engine/niagara-fluids-reference-in-unreal-engine) |
| **Aucun critère de convergence n'est exposé** — ni résidu, ni divergence, ni tolérance | **documenté par absence** (les deux pages de référence n'en mentionnent aucun) | idem |
| La résolution se règle par le **nombre de mailles sur le plus grand axe** d'une boîte | **documenté** | idem |
| Une simulation coûteuse se **précalcule** et se rejoue : cuisson en texture de volume épars (SVT) relue par un acteur de volume, ou cuisson en planches d'images | **documenté** | [Niagara Fluids](https://dev.epicgames.com/documentation/en-us/unreal-engine/niagara-fluids-in-unreal-engine) ; [Unreal Fest 2023](https://dev.epicgames.com/community/learning/talks-and-demos/1V6r/unreal-engine-creating-visual-effects-with-niagara-fluids-sparse-volume-textures-and-heterogeneous-volumes-in-ue-unreal-fest-2023) |
| L'eau peu profonde est un **solveur de champ de hauteur** (équations de Saint-Venant), bon marché, destiné aux grandes surfaces, sillages de bateau et interactions simples | **documenté** | [Shallow Water](https://dev.epicgames.com/community/learning/tutorials/Ddwx/unreal-engine-shallow-water-simulation-with-niagara-fluids) |
| Les textures de volume épars sont **tuilées en 16³**, étendues à 18³ pour l'interpolation ; plafond d'allocation 2048³ en DirectX 12 | **documenté par un tiers**, pas par Epic | [arXiv 2504.07485](https://arxiv.org/html/2504.07485v1) |

### Ce qui n'a pas été trouvé

Cherché, et **non trouvé** dans les sources publiques consultées — donc inconnu, et non « absent » :

- la **méthode** du solveur de pression (Jacobi, Gauss–Seidel rouge-noir, multigrille) ;
- la **valeur par défaut** du nombre d'itérations de pression, et sa plage utile ;
- la **précision** employée sur la grille (16 ou 32 bits) ;
- un **coût par image** chiffré pour une résolution donnée de gaz 3D ;
- toute mesure d'**erreur physique** : la documentation ne confronte la simulation à aucune référence.

Le seul document chiffré trouvé ([arXiv 2504.07485](https://arxiv.org/html/2504.07485v1)) mesure le
**rendu** de volumes scientifiques dans Unreal, et contourne explicitement la simulation : ses images
par seconde ne disent rien du coût d'un solveur.
