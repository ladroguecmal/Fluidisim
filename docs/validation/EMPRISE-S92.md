# S92 — Emprise déclarée et réception échantillonnée du virage

2026-09-08. Réalisation limitée de S91-1, sans nouvelle décision physique.
Le prototype f64 reste hors runtime ; ADR-069/070 inchangées.

## Contrat construit

`BoundedGaussian` associe modèle, trajectoire immuable et `QueryDomain` : rectangle local et
intervalle temporel inclusifs. Bornes finies, ordre spatial et temporel, trajectoire complète
validés à la construction. `field` refuse hors intervalle avant préparation ; `BoundedField`
n'expose que `sample`, qui refuse hors rectangle et tout point non fini.
Le champ interne n'est pas exposé par cette enveloppe. Les méthodes de référence non bornées
restent accessibles aux diagnostics ; elles ne deviennent pas des requêtes reçues implicitement.

Le domaine est **déclaré par l'appelant**, pas certifié par le constructeur. Il est impossible
d'en déduire une précision pour un autre profil ou une autre trajectoire. La validation initiale
calcule actuellement un champ temporaire pour vérifier la trajectoire ; cette dépense et les
allocations seront à retirer lors du portage runtime, pas dissimulées comme un coût nul.

## Campagne reproductible

`code/water-core/examples/receive_gaussian.rs`, exécuté en release. Profil et virage S91 :
σ=1 m, pic 10 Pa, g=9,81, ρ=1025, vitesse (2,0) sur 0–2 s puis (0,2) sur 2–4 s.
Rectangle **[-8,12]² m**, intervalle **[0,8] s**. Grille de 11² points à pas de 2 m,
instants 0/1/1,999999/2/2,000001/3/4/6/8 s : 1089 comparaisons spatiales par raffinement.
Ces points comprennent les bords et les deux côtés du raccord temporel.

Base 128 rayons × 128 directions, k_max=6 rad/m. Chaque axe est changé seul :

| Comparaison | Max hauteur m | Max vitesse verticale m/s | Max énergie J | Max puissance W |
|---|---:|---:|---:|---:|
| Radial 128→256, directions 128 | 3,828318062e-7 | 1,016413563e-7 | 2,090660620e-6 | 8,871610867e-7 |
| Directions 128→256, radial 128 | 3,881443777e-17 | 9,020562075e-17 | 3,413935801e-15 | 5,551115123e-16 |
| Coupure 6→9, Δk constant (128→192 rayons) | 2,566401085e-11 | 2,170815125e-10 | 0 observé | 1,387778781e-17 |

Les quatre seuils de régression sont 1e-6 m, 1e-5 m/s, 1e-5 J et 1e-5 W, dans la continuité
des fixtures S90. Tous passent. Ils restent à calibrer pour le gameplay. Un zéro observé
signifie une différence non résolue en f64, pas une erreur mathématique exactement nulle.

La campagne reçoit ces échantillons pour ce profil ; elle ne borne pas l'erreur continue entre
les points, ne compare pas au champ exact et ne reçoit pas un rectangle universel. Un écart
entre deux résolutions peut sous-estimer leur erreur commune. S90 apporte aussi une troisième
résolution et une normalisation analytique, sans résoudre cette limite générale.

## Tests de contrat et suite

Test S92 : extrémités temporelles et spatiales acceptées, dépassement spatial de 1e-6 m et
temporel de 1 µs refusé, bornes inversées/infinies et point NaN refusés. Ces décalages sont des
fixtures de frontière, pas des tolérances d'acceptation. Suite historique repassée.

S91-1 réalisée comme enveloppe et réception échantillonnée du profil ; une certification
continue ou multiprofils reste ouverte. **S92-1, prochaine session S93 :** construire le noyau
de préparation sur mémoire hôte et sortir les allocations du chemin de requête de référence,
avec identité des résultats et refus atomiques. Cela prépare le portage sans prétendre que
f64/libm est déjà un runtime répliqué. Déterminisme f32, codec de trajectoire, pente, vitesses
horizontales et couplage B+W restent à construire. Aucun nouvel angle ni invariant ; L205.
