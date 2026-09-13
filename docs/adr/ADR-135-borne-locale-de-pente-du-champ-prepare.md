# ADR-135 — Borne locale de pente du champ préparé

- Statut : **actée**, S218, 2026-09-13, autonomie technique S71.
- Traite la part dynamique d'A255 sans loi temporelle universelle (S217).
- Prolonge ADR-134 ; ne remplace ni son majorant global ni les contrats d'admission.

## Problème

S217 reçoit la similitude du sillage, mais réfute une courbe unique du rapport de pente
en âge depuis extinction. Le champ préparé contient déjà toute l'histoire du forçage.
Il permet de borner une région directement, sans modéliser à nouveau cette histoire.

## Décision

**1. Publier une annonce locale sur un rectangle fermé inclus dans l'emprise.**
`Field::local_slope_envelope(min,max)` et son exposition liée au contexte/instant dans
`Prepared`. Le résultat distingue borne retenue, pente au centre, reste spatial et
réserve numérique. Aucun échantillon ni coefficient existant n'est modifié.

**2. Calculer un reste spatial, jamais prendre le maximum de la grille pour une borne.**
Pour le champ trigonométrique et un rectangle de centre c :

```
|grad eta(p)| <= |grad eta(c)| + sum |weighted_k| |eta_k| min(2, delta_phase_k).
```

La différence du vecteur `(sin,cos)` entre deux phases est au plus `min(2,|delta|)`.
Avec phases exactes, `delta_phase = |kx|hx+|ky|hy` : c'est la borne de Hessienne
intégrée le long du segment. La borne directionnelle globale reste une seconde borne ;
on retient la plus petite, avec la même réserve numérique ajoutée.

**3. Tenir compte du calcul de phase réellement exécuté.** `from_distance` arrondit
`turns*x` avant repliement. La largeur locale est donc calculée depuis les produits
arrondis aux deux extrémités et au centre, dont la monotonie encadre tous les points
intermédiaires. Ajouter une marge pour la fraction et Q32 ; ne pas dériver seulement
`k*dx`, qui ignore les sauts d'arrondi aux grandes coordonnées.

**4. Distinguer preuve algébrique et réception numérique.** Calcul f32, ordre fixé,
sans allocation ni libm. La réserve repose sur la somme L1, une marge trigonométrique
de `32*EPSILON` (réduction et polynômes de `phase.rs`, angles bornés), et
`gamma_(N+32)=(N+32)*EPSILON/(1-(N+32)*EPSILON)` pour les accumulations et opérations
locales. Les sommes positives du reste sont arrondies vers le haut opération par
opération. Cette précaution **n'est pas une certification formelle de toute la chaîne
f32**, notamment aux sous-normaux ; comme ADR-134, l'annonce est reçue par comparaison
au champ exécuté. Débordement ou réserve non représentable : refus `NonFinite`.

**5. Refuser explicitement les rectangles invalides.** Valeurs non finies, axes inversés,
sortie d'emprise : `Domain`. Le rectangle réduit à un point est admis. Contexte et
instant sont vérifiés même pour ce rectangle. Le calcul est une lecture, sans cache.

**6. Une partition complète peut resserrer la borne d'une emprise.** Le maximum des
bounds locaux couvre l'union de ses rectangles. La partition et son coût restent
du ressort de l'appelant ; aucune grille implicite ni allocation dans le cœur. Une
borne locale ne se substitue jamais au plancher global sur une emprise plus grande.

## Réception requise

Rectangle autour d'un zéro de pente avec pente intérieure non nulle : le centre seul
doit échouer, la borne avec reste doit tenir. Points, rectangles dégénérés, coins,
translation près de 4096 m et phase quantifiée ; champ nul ; refus ; sorties existantes
inchangées. Partition des fixtures S217, coût complet (préparation séparée), rapport au
majorant global et maximum de référence. Mesures avec en-tête ADR-131.

## Ce qui reste ouvert

Coût d'une partition assez fine et choix de sa géométrie ; migration des admissions
(non décidée ici), A254 et mutualisation GPU. Aucun gain de budget d'image déduit du
seul resserrement de la borne. Seconde cible et preuve formelle des arrondis restent
hors réception. Une annonce globale inchangée ne bénéficie pas automatiquement de
cette nouvelle annonce locale.

## Précisions d'implémentation S218 — 2026-09-13

La réserve employée vaut `C*(4*gamma_(N+32)+32*EPSILON)` : les deux composantes,
leur norme et l'évaluation de la branche globale partagent cette réserve. La largeur
de phase ajoute `8*EPSILON` tours pour fractions et Q32 avant conversion en radians.
Une norme ou contribution non nulle qui sous-passe exactement à zéro est refusée par
`NonFinite`, comme un débordement ; ce refus ne constitue toujours pas une certification
formelle de toutes les valeurs sous-normales. Les quatre tests S218 reçoivent ces refus.
