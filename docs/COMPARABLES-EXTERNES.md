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

### Confrontation à nos propres nombres — S241

Aucun chiffre d'Epic n'entre ici. Ce qui suit oppose leurs **choix** à nos **mesures**.

#### 1. L'architecture en couches a un témoin indépendant

[ADR-001](adr/ADR-001-decomposition-en-couches.md) sépare une couche analytique et une couche d'ondes
**partout et bon marché** (B, W) d'un solveur eulérien **cher et borné** (δ). Epic livre exactement ce
partage : champ de hauteur bon marché pour les grandes surfaces, grille coûteuse réservée aux volumes
bornés d'« effets vedettes ». Deux équipes sans lien arrivent au même découpage.

Ce n'est **pas une preuve** — c'est une corroboration, et ADR-001 était déjà acté. Ce qu'elle vaut :
la décision qui commande tout le reste n'est pas une singularité de ce projet.

#### 2. La question qu'ils ne posent pas

Epic règle sa pression par un **nombre d'itérations** et un **facteur de relaxation**. Aucun critère
de convergence n'est exposé : la simulation s'arrête quand le compteur est épuisé, pas quand le
résidu le permet.

Ce projet a fait l'inverse, et l'a fait exprès. [ADR-143](adr/ADR-143-la-pression-f32-converge-a-sa-precision-representable.md)
arrête la pression à sa précision représentable ; [ADR-144](adr/ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md)
fait de la tolérance physique de S199 une **condition d'acceptation**, et déclare dégradé le pas qui
ne la tient pas. Le prix de cette exigence est mesuré :
[TOLERANCE-PRESSION-S239](validation/TOLERANCE-PRESSION-S239.md).

**Ce que le comparable établit n'est pas que notre critère est faux, c'est que notre classe de
fidélité est un choix.** δ est reçu contre un oracle indépendant — onde stationnaire HOS d'ordre 3 à
**0,252 %** de profil ([SURFACE-MOBILE-S237](validation/SURFACE-MOBILE-S237.md),
[S238](validation/PRESSION-PLANCHER-S238.md)) —, et la documentation d'un solveur d'effets ne
confronte sa simulation à aucune référence. **Les itérations fixes sont le moyen le moins cher de
rendre une pression abordable, et c'est exactement ce qu'ADR-144 interdit aujourd'hui.** Si un lot
futur en a besoin, ce sera un ADR nouveau avec sa dégradation mesurée, jamais un relâchement
silencieux.

#### 3. Ce que notre coût mesuré dit de l'ordre des obstacles

Tous les chiffres ci-dessous sont **les nôtres**, déjà publiés.

| mailles | coût d'un pas | source | pas de temps simulé |
|---:|---:|---|---|
| 128 | 0,0842 ms | [BUDGET-DELTA-S230](validation/BUDGET-DELTA-S230.md) | 1/60 s |
| 512 | 0,6822 ms | S230 | 1/60 s |
| 2 048 | **5,5125 ms** | S230 | 1/60 s |
| 128 | 0,107 ms | `delta_precision`, relevé S239 | 2 ms |
| 512 | 0,602 ms | idem | 2 ms |
| 2 048 | 4,234 ms | idem | 2 ms |
| 8 192 | **35,51 ms** (fond plat), 54,15 ms (fond en bosse) | idem | 2 ms |
| 32 768 | **non mesuré** (417 itérations mesurées en S239) | — | — |

[ADR-125](adr/ADR-125-budget-image-60hz-deux-ms.md) donne **2 ms par image à toute l'eau** — B, W,
δ et le rendu associé. Le banc S230 avance exactement une image par pas. Donc :

- **à 2 048 mailles, δ seul vaut déjà 2,8 fois le budget de toute l'eau**, en médiane ;
- à 8 192 mailles, un pas de 2 ms coûte 35,5 ms ; une image en demande 8,3, soit **≈ 296 ms**.

La loi est régulière : **quadrupler les mailles multiplie le temps par ≈ 8** (8,10 puis 8,08 en S230 ;
5,6 / 7,0 / 8,4 en S239), ce qu'on attend d'un gradient conjugué — `O(N)` par itération et `O(√N)`
itérations —, et ce que confirment les itérations mesurées (28 / 58 / 112 / 219 / 417, qui doublent
avec `n_x`).

**D'où une conclusion de priorité, et c'est l'apport principal de ce comparable.** A275 dit que f32
ne tient plus la tolérance à 32 768 mailles — elle la manque de 9 à 34 %. Mais **à cette taille, le
coût est déjà deux à trois ordres de grandeur au-dessus du budget**. Un lot qui ne corrigerait que la
précision à 32 768 mailles **ne débloquerait rien**. Les deux obstacles appellent la même famille de
remèdes — solveur plus fort, GPU, parallélisme — et c'est une bonne nouvelle ; mais l'ordre compte,
et il est renversé par rapport à ce que la file disait.

**Ce que ces chiffres ne disent pas** ([ADR-131](adr/ADR-131-un-depassement-qualifie-une-implementation.md)) :
δ n'a reçu **aucune** technique de coût. Présentes : préconditionnement diagonal, f32. Absentes : GPU,
parallélisme, multigrille, factorisation incomplète, itérations fixes, cuisson. Domaine : une machine,
CPU séquentiel, 2D. Un dépassement qualifie cette implémentation, pas la fonctionnalité, et il
s'éprouve sur la **combinaison** des techniques — dont aucune n'a été tentée.

#### 4. Volumes bornés (J2)

Le tuilage des textures de volume épars d'Unreal — 16³, étendu à 18³ pour l'interpolation — est un
**point de repère documenté** au moment où J2 choisira la granularité de ses domaines bornés. Il n'est
ni une contrainte, ni un seuil : il dit seulement qu'un système du commerce a jugé cette taille
praticable pour du volume épars sur GPU.

### Ce que ce comparable n'autorise pas

1. **Aucune réduction d'ambition.** « Epic réserve la grille 3D aux cinématiques » n'est pas un
   argument : ADR-127 exige une décision explicite de l'utilisateur, et la pratique d'un tiers n'en
   est pas une.
2. **Aucun nombre importé.** Rien de ce qui est lu ici ne devient un seuil, un défaut ou une cible.
3. **Aucune conclusion sur leur fidélité.** Leur documentation ne confronte leur simulation à aucune
   référence ; nous ignorons donc leur erreur, et c'est précisément l'asymétrie utile — la nôtre est
   mesurée contre HOS, f64 et Richardson.
