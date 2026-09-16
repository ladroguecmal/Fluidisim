# ADR-149 — Premier raccordement volumique de B/W au candidat δ

- Actée S250, 2026-09-16 ; autonomie technique S71. Applique SPEC-004 §6.1,
  ADR-114/117 (source après somme), ADR-141 (coefficients temporels) et ADR-046.
- Ne modifie aucun chemin reçu, aucune autorité ni l'ambition ADR-127.

## Équation et domaine

Le nouveau pas du candidat MAC x-z porte **la perturbation** `v = u'` :

```
v* = v - dt [(v·D)v + (U·D)v + (v·∇)U + S]
S = U_t + (U·∇)U + ∇P_dyn/ρ    (ν = 0 pour ce premier raccordement)
v* ← exp(-σ(x) dt) v*
v_next = projection(v*, p'_couvercle)
```

`D` est la différence centrée MAC existante (ordre intérieur deux), `∇U` et `S`
proviennent des fournisseurs différentiels analytiques. Les coefficients B/W se somment
**avant** `momentum_residual`. Une pression imposée W est déjà dans P_dyn : ne pas
l'ajouter une seconde fois. Source continue, pas résidu du schéma complet : aucune
identité au solveur total discret ni réception B4 globale n'en découle (L244/L245).
Si v=0 et S≠0, une correction doit naître ; effacer S pour préserver le fond serait faux.

Géométrie fixe, surface **imposée** : `eta` existante encode `z0 + eta_delta` et impose
`p' = ρg eta_delta`. Le nouveau chemin refuse un Volume configuré à surface mobile.
Les faces fermées imposent v·n=0, non l'imperméabilité du **total** U+v : ce sont des
bords de perturbation, pas une réception de coque dans un fond incident.
Pas de surface libre non linéaire, de déplacement de fond ni de transport de hauteur
dans ce lot. Leurs résidus de bord doivent être construits avant couplage mobile.

## Entrée et budget

L'hôte fournit les échantillons aux positions MAC, z relatif au couvercle, sans
décimation. Domaine, instant entier, densité et gravité doivent correspondre au pas.
Les échantillons doivent être planaires (composante y et dépendances en y nulles) :
la coupe centrale d'un impact radial 3D **n'est pas** un champ incompressible x-z.
Refus explicite, sans suppression silencieuse de composantes.

Les tampons appartiennent à l'appelant ; aucun échantillonnage caché dans le pas et
aucune allocation ajoutée. Son budget inclut validation, advection/source/éponge,
projection et publication, **pas la préparation des échantillons par l'hôte**.
Durée/budget en microsecondes entières. Expiration, non-fini et pression dégradée
conservent u'/w'/p' ; la hauteur imposée ne bouge pas. I-05 mural reste non reçu.

## Éponge

Largeur L et taux maximal σ_max fournis, finis et non négatifs, L≤demi-largeur du
domaine. `σ(x)=σ_max max(1-d(x,bord)/L,0)^3`. Profil cubique d'ADR-005/046 ; aucune
largeur ni vitesse inventée. L=0 n'est admis qu'avec taux nul. L'amortissement porte
sur le prédicteur perturbatif **avant** projection, pour ne pas casser sa divergence
après réception. Le taux de réflexion dépend des modes et des bords ; aucun 1 %
hérité du véhicule B-S27 n'est revendiqué ici. Cette éponge ne transporte pas η.

Réception : [RACCORDEMENT-DELTA-S250](../validation/RACCORDEMENT-DELTA-S250.md).

**Correction factuelle S250, 2026-09-16.** L'attribution du profil cubique à ADR-005/046
ci-dessus est erronée : ADR-046 §8, point 4, indique un profil quadratique. Le code
applique donc `σ=σ_max max(1-d/L,0)^2`. Aux bords, D reprend les extensions du MAC ;
sur la face verticale du couvercle, sa dérivée verticale est unilatérale et la vitesse
horizontale interpolée depuis la dernière rangée. Aucun ordre deux au bord n'est revendiqué.

**Note S253, 2026-09-16.** Les résidus de surface annoncés ci-dessus « avant couplage mobile » sont
construits par [ADR-152](ADR-152-surface-mobile-couplee.md) : valeurs fantômes corrigées de la
pression du fond et bande cinématique. Ils sont reçus sous un fond linéaire analytique. Les
frontières du total restent à construire.
