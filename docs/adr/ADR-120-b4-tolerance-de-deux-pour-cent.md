# ADR-120 — B4 : erreur acceptable de 2 %

- **Statut : actée**, S190, 2026-09-12, sur instruction explicite de l'utilisateur :
  « Donc on débloque avec 2% d'erreur acceptable, débloque B4 avec ça ».
- Complète ADR-112, ADR-118 et ADR-119 ; ne réécrit aucune de leurs mesures.
- Lève l'attente d'un arbitrage de justesse dans BILAN-B4-S176 et SPEC-004 §6.2.

## Décision

**La tolérance de champ perturbatif de B4 est ε = 0,02.** C'est une exigence de
réception décidée par l'utilisateur, pas une constante physique ni une erreur mesurée.
Elle n'est plus « à fixer » ou « à calibrer par une prochaine session ».

Pour le volet source actuellement instrumenté (S185–S189), le champ est la vitesse
perturbative `u'`, en m/s. On conserve exactement sa métrique :

```
eU = max_cellules,composantes |u'_candidat(T) − u'_référence(T)|
     / max_cellules,composantes |u'_référence(T)|
eU ≤ 0,02
```

Maximum des composantes, pas norme euclidienne ; cellules intérieures, pas fantômes.
Le dénominateur est celui de la perturbation de référence, jamais celui de B+W qui
masquerait une erreur sur une petite perturbation. La fenêtre et la référence font
partie du verdict. Un champ de référence nul ne reçoit aucun pourcentage : il exige
un contrôle absolu explicitement déclaré ; un non-fini est un refus.

**Les 2 % sont un budget conjoint**, pas 2 % par axe. Appliquer ADR-119 : mesurer les
axes seuls, dimensionner par leur somme, puis mesurer le couple retenu. Une compensation
favorable ne finance pas un profil. La somme mesurée sur ce véhicule n'est pas un
théorème valable pour un autre solveur : le contrôle composé reste obligatoire.

## Réception immédiate et qualification de la référence

Le protocole [B4-TOLERANCE-S190](../validation/B4-TOLERANCE-S190.md) est publié avant
exécution. Il rejoue le véhicule S185–S189, référence pleine à chaque pas. L'écart
au même véhicule à pas temporel moitié est ajouté comme **réserve mesurée** au budget :
`e_spatial + e_temporel + e_référence ≤ 0,02`. Vérifier aussi
`e_composé + e_référence ≤ 0,02` et la comparaison directe à la référence raffinée.
Cette réserve ne transforme pas deux résolutions d'un même schéma en oracle physique.

`N` de SPEC-004 §6.2 compte des points par longueur d'onde : **N ne vaut ni 2 ni 0,02**.
Le seuil est fixé ; le réseau et la cadence sont les résultats du banc sous ce seuil.
Un profil reçu porte ses indices, sa cadence, son contenu, sa profondeur et sa durée.
Il ne fournit pas un `N` universel ou un `is_smooth_at` permissif.

## Portée du déblocage

Le volet **erreur d'échantillonnage de la source** peut désormais être jugé, et un
profil de banc peut être choisi. Le seuil ne dépend pas de la future preuve de
l'additivité avec projection : ce travail doit tester une configuration contre les
2 % déjà décidés, sans reporter l'arbitrage.

**B4 complet** demande encore la comparaison perturbatif/substitutif intégral,
la surface libre, les forces sur coque et la perception. Une vitesse reçue ne reçoit
ni une hauteur ni une force. La tolérance de 2 % n'est pas le rapport de bascule
`|δ|/Hs`, ni `|δ|/h` ; les déduire de S161 contredirait ADR-112. Aucun solveur δ
de B3 n'est choisi par ce seul arbitrage. I-03, I-04 et I-15 restent inchangés.

## Ce qui reste ouvert et réversibilité

Appliquer ce seuil au véhicule projeté puis au candidat B3 avec référence indépendante,
surface et conditions aux limites ; recevoir ensuite les autres volets de B4.
Les limites de coût et de validité constatées sont des résultats à traiter, pas des
motifs pour oublier le seuil. Seul un nouvel arbitrage explicite peut changer les 2 %.
