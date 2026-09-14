# S199 — Le premier candidat de δ : le noyau à projection

2026-09-12. **B3**, choisie par la file sous la règle des deux maillons (§6.8). δ n'a pas
reçu de code d'exécution depuis **S161**. **Choix, protocole et filtres déclarés avant tout
code.** Aucun verdict B3 n'est acquis ici, et aucune famille n'est éliminée.

## 1. Ce qui manque, et depuis quand

[PLAN-BENCHMARK §B3](PLAN-BENCHMARK.md) pose la question depuis S01 : *quel solveur
volumétrique ?* Il impose quatre scénarios — coque en mouvement, impact d'un objet lourd,
domaine substitutif de déferlement, compartiment inondé en référentiel accéléré. S192, S193
et S194 y ont chacun ajouté une **contrainte de sélection**, et l'ont écrit honnêtement :
*« Ce que ce lot apporte à B3 est une contrainte de sélection et non un candidat. »*

Cent quatre-vingt-dix-huit sessions, aucun candidat. C'est ce que cette session change, et
seulement cela.

## 2. Le choix, et pourquoi il ne préempte rien

[ADR-007 §5](../adr/ADR-007-interface-de-solveur-et-strategie-de-remplacement.md) liste cinq
familles sans en privilégier aucune : FLIP/APIC, MPM, eulérien à advection semi-lagrangienne,
grille + particules de surface, fluides à base de positions.

**Ce qui est construit ici n'est aucune des cinq : c'est ce que trois d'entre elles
partagent.** FLIP et APIC *sont* des solveurs à projection dont l'advection est portée par
des particules ; MPM projette de même ; l'eulérien projette après advection. Le noyau
— grille décalée, opérateurs d'ordre deux, fond coupé, **projection de pression** — est donc
du travail **commun**, et le bâtir ne parie sur rien.

Deux raisons de plus, et elles viennent du dépôt :

- **C'est la pièce que le corpus déclare manquante.** Le véhicule perturbatif de S184 dit en
  toutes lettres qu'il **ne projette pas**, et en fait sa faiblesse principale — « omettre la
  projection sous-estime le pas de solveur ». Construire la projection lève cette réserve.
- **C'est la pièce où les deux filtres d'entrée se décident.** L'équilibrage et l'ordre en
  espace se gagnent ou se perdent dans le couple gradient de pression / gravité, pas dans
  l'advection. Les filtrer ici, c'est les filtrer là où ils vivent.

## 3. Les deux filtres d'entrée, et comment ils seront passés

[ADR-038 §4](../adr/ADR-038-ce-que-les-deux-premiers-cas-de-solveur-ont-appris.md) impose deux
questions **avant tout banc**, parce qu'elles coûtent quelques minutes et qu'elles éliminent.
Chacune a déjà éliminé un premier jet écrit de bonne foi.

### 3.1 L'équilibrage — gagné par construction, pas par compensation

ADR-030 en fait un critère d'entrée : un candidat mal équilibré échoue C01, et **aucun budget
de calcul ne le rattrape** — le raffinement qui rachèterait le défaut coûte ×10 500 en 2D.

Le défaut classique vient de discrétiser `−∇p + ρ g` et d'espérer que les deux termes
s'annulent au repos. Ils ne s'annulent qu'à l'ordre du schéma, et sur un fond variable
l'erreur résiduelle met le lac en mouvement.

**Le noyau ne discrétise jamais cette différence.** La pression est portée en deux parts :

```
p  =  p_hydro + p_dyn        p_hydro(z) = ρ · g_eff · (z₀ − z)
```

`p_hydro` est **analytique** et n'est jamais différenciée numériquement ; seul `p_dyn`
entre dans le gradient discret. Au repos, `p_dyn ≡ 0`, et l'accélération est **exactement**
nulle — à l'arrondi, quelle que soit la forme du fond, parce qu'aucune annulation n'est
demandée à deux termes discrets.

C'est la même décomposition que la couche B emploie déjà : ADR-114 écrit que
*« hydrostatique et gravité se compensent déjà »*. Le noyau δ hérite du procédé plutôt que
d'en inventer un second.

### 3.2 L'ordre en espace

Grille décalée MAC, différences centrées, laplacien à cinq points : ordre deux à l'intérieur
**par construction**. Ce qui doit être mesuré est ce qui se passe **aux bords** — fond coupé
et condition de surface — car c'est là que les schémas perdent leur ordre sans le dire.

## 4. Ce que le candidat doit satisfaire, et qui ne vient pas de moi

| exigence | d'où elle vient | comment elle est tenue |
|---|---|---|
| `g_eff` **injectée** | ADR-007 §2 — *« un solveur qui code `−9,81·Z` en dur est disqualifié d'emblée »* | fournisseur passé à `configure`, jamais de constante |
| fond **fourni**, non deviné | ADR-007 §2 | hauteur par colonne, donnée à `configure` |
| **aucune allocation** à l'exécution | **I-06** | tous les tampons demandés à l'hôte avant `seal()` |
| **déterminisme** | **I-03** | toute accumulation flottante passe par `parallel_reduce_ordered_f64` (SPEC-004 §8.2) |
| **budget respecté** | ADR-007 §2 — *« un solveur qui peut dépasser son budget rend l'ordonnanceur inutile »* | le nombre d'itérations de pression est la variable de dégradation |
| **aucun état sérialisé** | **I-17** | rien de tel n'est écrit |

## 5. Réceptions — chiffrées, et déclarées avant d'être tentées

1. **Lac au repos, fond non plat** *(filtre 1)*. Fond en marches et en pente, `z₀` plat,
   `u = w = 0`. Après 1 000 pas, la vitesse maximale doit valoir **exactement zéro en bits**.
   Pas « petite » : **nulle**. Une seule composante non nulle disqualifie le schéma, et c'est
   le sens du mot « par construction » au §3.1.
2. **Le même, sous une autre gravité.** `g_eff = 1,62` (lunaire) : le repos reste exact. Ce
   contrôle vaut aussi contre un `9,81` oublié quelque part.
3. **Ordre en espace** *(filtre 2)*. Solution manufacturée sur le domaine avec fond coupé :
   trois résolutions, ordre mesuré par Richardson **sous la garde de S197** — un triplet non
   monotone ne rend aucun ordre. Attendu **≥ 1,8**. Sous `1,5`, le candidat est éliminé par
   ADR-038 §4 et la session le dit.
4. **Divergence après projection.** Le champ projeté doit être à divergence nulle à la
   tolérance du solveur : `max |div u|` rapporté à `max |u|/dx` sous **10⁻⁵**.
5. **Zéro allocation après `seal()`** *(I-06)*, mesuré par l'allocateur compteur, sur un pas
   complet et sur un pas dégradé.
6. **Déterminisme** *(I-03)* : deux exécutions bit pour bit, empreinte publiée.
7. **Budget** : à budget serré, `step` rend la main **en dégradant** et le déclare ; à budget
   large, il converge. Les deux états sont observables de l'extérieur.

## 6. Ce que ce candidat ne fait pas

Il faut l'écrire avant, pour que personne ne le lise comme davantage.

1. **Aucun des quatre scénarios de B3.** Ni coque mobile, ni impact, ni déferlement, ni
   référentiel accéléré. Le candidat n'est pas mesuré par B3 ; il devient **éligible** à B3.
2. **Pas de surface libre mobile.** Le noyau travaille sous un couvercle à `z₀` avec
   `p_dyn = 0` imposé. La surface libre est le lot suivant, et c'est elle qui rendra les
   scénarios atteignables.
3. **Deux dimensions**, `x`–`z`. Le passage en trois dimensions est mécanique pour les
   opérateurs et ne l'est pas pour le coût ; rien n'est promis là-dessus.
4. **Ni solide mobile, ni phase air, ni déferlement.** `SolverCaps` les déclarera **faux**,
   et c'est le rôle de ce champ.
5. **Aucune mesure de coût**, aucun `cost_per_block_ms`. Ce sera un lot propre, et S183 à
   S185 ont montré comment le faire.
6. **Aucune famille éliminée.** Si ce noyau passe les filtres, cela ne dit rien contre MPM ou
   les particules de surface — cela dit seulement que le projet a enfin quelque chose à
   mesurer.

## 7. Relevés

**Relevés de Claude, P3, repris par Codex le 2026-09-13 sans remesure.**
Commits9df2aa2,81f8d03,0c143a8 ; passation dans `notes/EN-COURS.md`.
Module final : `code/water-core/src/delta_projection.rs` ; huit tests dans
`tests_delta_projection.rs`, campagne `code/water-core/examples/delta_filters.rs`.

| contrôle | résultat transmis | portée vérifiée à la lecture des tests |
|---|---|---|
| lac au repos, fond coupé | vitesse exactement nulle en bits | 1000 pas à g=9,81 ; **50 pas** à chacune des gravités1,62/9,81/24,79 |
| gravité agissante | rapport de vitesse proche de2 quand g double | premier pas depuis le repos, surface inclinée ; pas seulement un lac immobile |
| fond plat, trois résolutions | ordre1,947 ; résidu Richardson0,025 % | filtre2 passé sur la fonctionnelle mesurée |
| fond lisse découpé | ordre0,898 ; résidu0,963 % | filtre2 échoué |
| fond avec marche | ordre0,895 ; résidu0,961 % | filtre2 échoué |
| projection | divergence rapportée<1e-5 | configuration unitaire testée ; diagnostic pondéré par ouvertures, pas borne universelle |
| rejeu | deux exécutions identiques, empreinte0x0ad3f695685ca27a | cible locale uniquement, pas conformité multiplateforme I-03 |
| workspace | **339 réussis, cinq ignorés** (246+93) | reçu transmis P3, huit tests supplémentaires ; non rejoué par Codex en P4/P5 |

Le triplet utilise32/64/128 cellules horizontales. La garde S197 refuse les
incréments de signes opposés ; elle ne transforme pas un triplet non convergent en ordre.
Le code mesure `Σ u_face·dx` au plan médian, **sans pondérer par l'ouverture**.
C'est la fonctionnelle publiée ; l'appeler débit physique sur les faces coupées serait
plus fort que ce que le programme calcule. L'ordre du flux ouvert reste à recevoir.
Le premier pas part du repos : l'advection y est nulle, donc ce filtre ne reçoit pas
son ordre ni sa stabilité en mouvement.

### Deux pistes éprouvées pendant P3

1. **Mantisse f32 de pression : hypothèse réfutée sur le relevé.** Le passage de la
   pression en f64 donne2,264122231e-4 contre2,264121986e-4 ; il ne résout pas le défaut.
   Ce changement reste dans le code, mais ne constitue pas une dérogation à I-08.
2. **Fond en escalier : correction partielle mesurée.** Le tout-ou-rien et
   `max(b[i−1],b[i])` produisaient des triplets non monotones. Le fond linéaire par
   morceaux avec fractions ouvertes rétablit une convergence, d'ordre un seulement.

Claude a identifié le décalage entre centre géométrique de face et moyenne sur sa
partie ouverte comme mécanisme restant : un champ variant sur la face peut produire
un écart O(dx). **Piste localisée, non corrigée ni isolée par une contre-épreuve** ;
la concordance de l'ordre ne démontre pas encore qu'elle explique tout le défaut.

### Rectifications de portée à la reprise P4

Ces constats viennent du code committé, sans nouvelle campagne :

- **I-06 n'est pas reçu.** `no_host_allocation_after_seal` compte les demandes à
  `Arena`, pas l'allocateur global. `project` clone `us/ws`, `dir` à chaque itération,
  puis `u/w` pour le diagnostic. Le pas alloue donc après scellement. La comptabilité
  initiale annonce aussi des tampons en unités f32 alors que cinq sont désormais f64.
- **Le plafond d'itérations est reçu ; le budget en millisecondes ne l'est pas.**
  `step(dt,max_iters,jobs)` ne reçoit aucune échéance. `degraded` annonce correctement
  la non-convergence testée, sans prouver le respect du contrat temporel ADR-007.
- **Les refus d'entrée testés ne prouvent pas l'atomicité générale.** Un non-fini
  produit pendant le pas est détecté après mutation de la vitesse, sans restauration.
- **Le couvercle est fixe, mais sa pression n'est pas toujours nulle** :
  `p_dyn=ρ g_eff(η−z₀)` y impose une hauteur fournie. `η` n'est pas intégrée.
  `supports_frame_accel=true` ne reçoit pas un référentiel accéléré général : g est
  un scalaire fourni à la construction. Les bornes de `Caps`, dont CFL0,5, ne sont
  pas une mesure de stabilité. Le schéma déclare lui-même celle-ci non reçue.

Ces restrictions remplacent les promesses des §4–§6 et de la passation lorsqu'elles
dépassent les contrôles effectifs. Elles sont portées par **A244** ; aucune correction
de solveur n'est cachée dans cette étape documentaire.

## 8. Ce qui est reçu

**Suivi S200 — 2026-09-13.** [CONTRATS-DELTA-S200](CONTRATS-DELTA-S200.md) corrige
les allocations du pas, la restauration sur refus numérique et les capacités non
reçues. Trois tests avec compteur global, workspace342/cinq ignorés ; empreinte
S199 inchangée. A244 partielle pour précision et budget temporel ; les défauts
énumérés au §7 restent le constat historique S199, pas tous l'état du code actuel.

**Filtre1 passé ; filtre2 passé sur fond plat, échoué au fond découpé.** Le premier
jet n'est **pas encore éligible à B3**. Aucune famille n'est éliminée et aucun
solveur de production n'est retenu. Aucun ADR nouveau : ADR-038 §4 porte déjà ce verdict.
Le critère2 % de B4 reste acquis ; ces filtres ne constituent pas sa réception complète.

**La couche δ a avancé** : un noyau MAC x-z avec advection/projection, fond fourni et
couvercle imposé existe dans la bibliothèque. Il reste un candidat incomplet :
surface mobile, source B+W, coque, air, coût et quatre scénarios B3 non reçus.
Le compteur `Maillons` revient à zéro selon REPRISE §6.8.

**S199-1 / prochaine session : corriger les contrats du noyau en bibliothèque (A244)**,
en priorité supprimer les allocations du pas et les mesurer avec un compteur global,
apparier les capacités et refus au comportement réel ; choisir explicitement le
traitement du f64 et du budget temporel sans les déclarer conformes par défaut.
**S199-2 : reconstruire les flux sur faces partiellement ouvertes**, contrôler leur
intégrale et refaire le triplet fond plat/lisse/marche avant surface libre mobile.
Ce second lot reste nommé dans la file ; il ne disparaît pas derrière les contrats.

**Rectification S232 — 2026-09-14.** Le débit pondéré par les ouvertures retrouve un ordre
1,95–1,98 sans modifier le solveur : l'ancien ordre≈0,90 ne recevait pas cette grandeur.
Le décentrage supposé ne se déduit donc pas de ce chiffre. Un défaut distinct, suppression
de triangles fluides par sous-échantillonnage, est reproduit puis corrigé dans la géométrie
consommée par le pas. Ordres finaux1,947/1,957/1,966 ;
[FLUX-COUPES-S232](FLUX-COUPES-S232.md). La forme dite « marche » est une tanh lisse et le
triplet est une auto-convergence, pas une référence manufacturée indépendante. Le filtre
de débit ouvert est reçu sur ces trois fonds ; ni ordre local ni scénarios B3 reçus.
