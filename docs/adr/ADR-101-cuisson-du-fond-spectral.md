# ADR-101 — Cuisson explicite du fond spectral

- **Actée**, S148, 2026-09-10, délégation S71. Applique ADR-100.
- Produit `background_spectrum::{Recipe, Cooked, bake}` et `Background::from_spectrum`.

## Contrat

La recette V1 contient SeaState, gravité injectée, gamma, bande relative à fp et largeur
directionnelle. Le profil numérique admis est N32..256, gamma1..7 et
0,5≤min<1<max≤4. Ces limites encadrent l'instrument S147 ; elles ne calibrent pas le jeu.
Hs≥0, Tp>0, gravité>0, paramètres finis ; direction moyenne en tours [0,1), largeur [0,1].
Les valeurs non représentables sont refusées explicitement, jamais ramenées à un défaut.

`bake` travaille sur tableaux fixes et publie un objet immuable seulement après succès.
`from_spectrum` demande l'allocation persistante à l'hôte avant de copier les composantes ;
après seal, la construction est refusée. L'évaluation réutilise le chemin Background existant.
Le constructeur historique ne change ni ses coefficients ni ses scénarios.

La cuisson utilise l'exponentielle f32 bornée de S97 **sans changer ses opérations**,
un logarithme par décomposition binaire et série atanh à douze termes, et PhaseQ32 pour
les directions. Aucun exp/powf/sin/cos de libm ajouté. Les poids sont intégrés par Simpson
**en fréquence**, avec coupure au pic, puis normalisés ; la référence S147 intègre en
log-fréquence. Les opérations et leur ordre sont fixes. Cela ne remplace pas une réception
sur plusieurs plateformes : les hachages reçus ici sont locaux debug/release.

Diagnostics de troncature : intégrale sur la bande divisée par le total PM analytique
augmenté de l'excès JONSWAP intégré sur [0,5;4]. L'excès extérieur est négligeable dans
ce profil ; les diagnostics sont des approximations reçues à 2e-5 absolu sur les fixtures.
La coupure de l'exponentielle du pic à 32 omet un facteur inférieur à 2,5e-14 pour gamma≤7.
Formules physiques : SPEC-001 §1 bis ; limites d'approximation testées dans S148.

## Gravité et intégration

Background porte désormais la gravité utilisée pour ses fréquences et nombres d'onde.
Les trois chemins B+W (impacts, pressions, mixte) la comparent au milieu W au lieu de
comparer W au littéral 9,81. Le fond historique annonce toujours 9,81. La gravité injectée
du candidat respecte I-07 ; une autre gravité ne devient pas compatible par déclaration hôte.

Le hachage de cuisson inclut version, recette, diagnostics et coefficients. La reconstruction
depuis la même recette et la même ancre restitue le champ en bits. **Aucun codec de recette
spectrale n'est construit ici** ; WLIV ne contient déjà pas B. L'hôte doit conserver les
paramètres exacts et la version ; un transport binaire de ces paramètres reste à construire.

## Réception et suites

[FOND-SPECTRAL-S148](../validation/FOND-SPECTRAL-S148.md) : moments et pic reçus, dérivée
temporelle, pente, impact non nul et refus de milieu ; cinq essais debug/release.
I-01 à I-18 conservés. La bande et l'éventail restent des fixtures ; aucun seuil de B1
n'est transposé au nouveau spectre.

**S147-1 partielle** : constructeur et réception ponctuelle construits ; cycle de sauvegarde
du service avec recette spectrale, réception multisource/pression, statistiques sur plusieurs
graines et coûts restent ouverts. **S148-1** : exercer le cycle hôte spectral complet avec
transport explicite de la recette et comparer le rejeu, puis mesurer le coût B+W. A212 reste
partielle pour ces réceptions et le choix des bandes/directions du jeu. W4/sillage et B2 restent
la trajectoire système après ce lot ; aucun nouveau prérequis théorique ne leur est ajouté.
