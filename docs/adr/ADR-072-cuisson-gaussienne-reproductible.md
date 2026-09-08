# ADR-072 — Recette de cuisson gaussienne V1

- **Statut : ACTÉE**, S97, 2026-09-08, délégation technique.
- Complète ADR-070/071 ; ferme la cuisson implicite f64/libm du candidat S96.

## Décision et contrat

`gaussian_spectrum::bake` produit dans un pool hôte les nœuds consommés par
`spectral_pressure`. La recette porte sigma en mètres, coupure kmax en rad/m, nombres
radial et angulaire. La vue retournée expose les nœuds immuables, la recette et un hash.
Les nœuds restent ordonnés rayon puis direction ; aucun tri implicite après cuisson.

V1 admet sigma dans [1/16,64] m, sigma*kmax dans [1,8], radial 1–512 et directions
paires 4–512. Ce sont des limites techniques de ce premier candidat, **à calibrer par
réception hors des montages ci-dessous**. Elles ne certifient pas une précision uniforme.
Entrées invalides et capacité insuffisante refusées avant écriture ; queue du pool intacte.
Les limites garantissent des intermédiaires finis et des poids strictement positifs.

Directions aux milieux des intervalles, phase Q32 calculée en entier. Rayon et poids
suivent la quadrature polaire d'ADR-070. La transformée gaussienne utilise exp(-x), x dans
[0,32], sans libm : x=n ln2+r, n entier, polynôme de Taylor degré neuf en r, puis facteur
2^-n construit par les bits du f32. Coefficients et ordre des opérations sont fixes.
Le reste analytique sur r dans [0,ln2] est borné par (ln2)^10/10!, environ 7,1e-9 ;
cette borne ne couvre pas les arrondis de réduction ni d'évaluation, reçus séparément.

FNV-1a inclut, dans cet ordre : version u32=1, bits f32 sigma/coupure, radial/angulaire
u32, puis les quatre f32 de chaque nœud (kx, ky, transformée, poids), tous en petit boutiste.
Ce hash est un diagnostic de recette et de contenu ; il n'est ni une signature ni un
mécanisme d'authentification. Il ne remplace pas un futur codec ou contrôle de contexte.
Toute évolution du calcul qui change les bits requiert une nouvelle version de recette.

## Réception S97

Trois tests : approximation/profil/refus, limites de recette contre nœuds f64 indépendants,
et campagne S96 complète réutilisée avec les nouveaux nœuds. Les seuils précédents restent
inchangés ; aucune référence des autres couches n'est modifiée.

| Mesure | Maximum observé |
|---|---:|
| exp(-x), 32001 abscisses 0–32, erreur relative | 1,160470e-6 |
| Profil unitaire, quatre points S90, erreur absolue | 9,179109e-5 |
| Direction de nœud normalisée par k | 1,258966e-7 |
| Transformée de nœud, erreur relative | 2,109548e-6 |
| Poids de nœud, erreur relative | 1,305309e-7 |

Limites testées : sigma=1/16,1,64 ; coupure réduite=1,6,8 ; résolutions (1,4),
(17,18), (512,512), soit 27 recettes. Seuils de régression, à calibrer hors campagne :
3e-6 pour exp, 1e-4 pour le profil, 3e-7 / 1e-5 / 5e-7 pour les trois colonnes de nœuds.
Les tests utilisent libm uniquement comme oracle indépendant, jamais pour la cuisson candidate.

Virage S91, spectre 128², sigma=1, coupure=6, neuf instants 0–8 s et 121 points :
écarts maxima contre champ f64 complet, dans l'ordre eta/w/phi/pente/u :
8,079e-9 m / 3,918e-8 m/s / 6,031e-8 m²/s / 9,963e-9 / 2,953e-8 m/s.
Chaque grandeur reste sous 1e-7, énergie sous 2e-6 J. Découpage rectiligne également reçu.
Hash de la recette nominale : **20e64a392ae237a1**, figé par assertion debug/release locale.

## Limites et suite

S96-1 réalisée dans la bibliothèque : recette et contenu identifiés, cuisson sans allocation
ni libm. La conformité interplateforme reste ouverte ; identité locale ne la prouve pas.
Pas de réception universelle sur tous les profils, aucune intégration autoritaire.

**S97-1, S98 :** mesurer le coût de cuisson, préparation et interrogation du chemin complet
sur mémoire hôte, et sa mémoire effective ; identifier le coût dominant avant optimisation.
Puissance/travail candidat, codec de trajectoire, admission et LiveWater restent ouverts.
I-03/I-06/I-08 inchangés. Aucun nouvel angle numéroté.

> **Actualisation S98 — 2026-09-08 :** S97-1 réalisée,
> [COUT-GAUSSIEN-S98](../validation/COUT-GAUSSIEN-S98.md). Requête dominante ; candidat de
> réduction par conjugaison à recevoir en S99, S98-1. Aucun budget cible certifié.
