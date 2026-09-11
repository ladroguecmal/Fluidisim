# Fond reconstruit et source conjointe — S172

## Protocole déclaré

Suite S171-1/A50, véhicule Saint-Venant 1D, f64, mouillé, fond figé déterministe.
Même onde simple analytique de S170–S171 (amplitude 0,05 m, centre 55 m, largeur 8 m),
fenêtre [30,90] m, durée 6 s. Aucun schéma runtime ni stockage de B décidé.

Q_H interpole linéairement les variables conservatives (h,q) aux nœuds espacés de H.
On ajoute les échantillons exacts à 30 et 90 m, en conservant les nœuds extérieurs pour
les moyennes fantômes. Moyennes spatiales exactes de cet interpolant, y compris quand
une cellule traverse un nœud. Q exact reste un témoin distinct, intégré par quadrature.
Le total initial reste la même moyenne analytique T0 : d0=T0-Q_H dans chaque cellule.
L’erreur de représentation est donc une charge du résidu initial, pas une erreur de T0.

Deux sources pour chaque fond :

- Physical : S_i=[F(Q_H(x_i))-F(Q_H(x_{i+1}))]/dx, chaque face évaluée une fois.
  Il s’agit de F du fond interpolé, pas de l’interpolation de F utilisée en S171.
- Discrete : S_i=L_num(Q_H)_i, différence du flux Rusanov des moyennes voisines.
  Témoin de l’identité au solveur total, pas candidat automatiquement adopté.

Puisque D_num(Q,d)=L_num(Q+d)-L_num(Q), le total figé vérifie :

```text
T_t = L_num(T) + [S - L_num(Q_H)].
Physical : défaut local = L_phys(Q_H) - L_num(Q_H).
Discrete : défaut nul, même évolution RK2 que le solveur total pour tout Q_H.
```

Une évolution totale indépendante par API S165, fond au repos et sans source explicite,
sert de référence discrète. L’onde analytique reste la référence physique indépendante.
La source Discrete annule aussi la correction qui protégeait un fond exact des erreurs
du solveur ; son identité ne prouve donc pas une meilleure précision physique.

Budget Physical : flux total Rusanov effectivement utilisé + flux physique net de Q_H
moins flux Rusanov net de Q_H. Budget Discrete : flux total Rusanov seul. Ces budgets
sont calculés depuis les flux, sans inférence depuis le volume évolué. Les deux bornes
de Q_H sont exactes ; la télescopie doit fermer chaque budget à l’arrondi.

H=1,2,4,8,16 m, phase 0/H/2, plus Q exact ; deux sources par fond.
N=120/240/480, et demi-pas à N240 : 88 évolutions résiduelles et 4 témoins totaux.
Mesures : représentation initiale h/q, restitution du total initial, défaut local de
source (unités physiques), erreurs de hauteur/débit, écart au témoin Physical/Q exact,
identité discrète, volume, Courant. Aucun seuil physique déduit des tolérances de test.
