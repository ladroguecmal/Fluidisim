# S170 — Source interpolée sur réseau indépendant

## 1. Protocole déclaré avant mesure

Suite S169-1/A50, SPEC-004 §6.2 et PLAN-BENCHMARK §B4. Véhicule Saint-Venant1D,
Q onde simple a0,05 centre55 m largeur8 m **figée**, T onde évolutive correspondante,
fenêtre[30,90], durée6 s. Source physique S=-∂xF(Q), connue analytiquement S167.
S exacte moyenne=[F(Q_gauche)-F(Q_droite)]/dx, reçue S168.

Comparer S exacte moyenne, S omise, et S ponctuelle échantillonnée aux nœuds d'un
réseau de pas H=1/2/4/8/16 m, origine0 ou H/2. Interpolation linéaire entre nœuds,
**intégrée exactement dans chaque cellule** pour fournir sa moyenne. Ne pas ajouter
une erreur de quadrature fine à l'erreur du réseau source. Pas H indépendant de dx.
Solveur N120/240/480 sur120 m (dx1/0,5/0,25), pas0,2 dx/sqrt(g) ; demi-pas N240.
Les ratios H/dx incluent4 mais ne sont pas fixés à4. Paramètres de banc, pas profils.

Q et ses flux restent évalués exactement en moyennes. Fantômes T exacts aux étages
pour isoler la source : ce montage ne teste pas l'échantillonnage simultané de Q et S,
ni le bord autonome. La source est figée, sans interpolation temporelle.

Mesurer erreur maximale hauteur/débit contre T moyen (échelle0,05 et0,05 sqrt(g)),
écart de champ au témoin S exacte sur mêmes mailles, norme d'erreur de source normalisée
par son maximum exact local (composantes séparées), bilan de volume et prédiction :

```
volume(t)-volume(0)-flux_total_cumule(t) = t · somme_i[(S_H-S_exact)_h dx]
```

La source erronée est connue et constante en temps. Vérifier cette identité **signée**
sur la trajectoire et non seulement ses maxima. Un bilan corrigé de cette injection peut
fermer sur un champ faux : conserver le bilan physique non corrigé dans le rapport.
Ne pas confondre écart à l'analytique et effet de la source interpolée, comparer les champs
avant les maxima. Seuil instrumental1e-10 pour bilans, aucun seuil physique B4 choisi.

Tests : intégration de l'interpolant sur fonction affine, témoin source exacte, omission,
prédiction du volume, variation H à dx fixé et dx à H fixé. La translation du réseau
éprouve l'aliasing ; ne pas déduire une règle universelle d'un seul alignement.
Le nombre de nœuds est un coût géométrique, pas un gain de temps runtime mesuré.

## 2. Résultats

À produire en P3/P4.
