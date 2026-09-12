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

*À recevoir en P3b. Aucun chiffre n'est écrit avant l'exécution.*

## 8. Ce qui est reçu

*À recevoir en P4.*
