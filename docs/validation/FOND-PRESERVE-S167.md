# S167 — Préserver un fond exact sans supprimer son défaut physique

## 1. Dérivation et protocole avant mesure

Suite S166-1/A222 ; mêmes équations et fenêtre [30,90] m que S166.
Noter L(Q)=-∂xF(Q), d=T-Q. L'équation continue est
d_t=-∂x[F(Q+d)-F(Q)] + S, avec **S=L(Q)-Q_t**.
La source physique est nulle pour une solution exacte, pas pour un fond quelconque.

Le candidat discrétise la différence de flux par `delta_numerical` S163, puis intègre
d seul en RK2, Q réévalué à chaque étage et S physique injectée aux mêmes temps :

```
d1 = d0 + dt [D(Q0,d0)+S0]
d+ = (d0+d1+dt [D(Q1,d1)+S1])/2
```

Aucun incrément temporel de Q n'est soustrait : il figure déjà dans S. Si S=0 et d=0
jusqu'aux fantômes, D=0 exactement. C'est une propriété d'équilibrage, différente de
l'identité au schéma total S164. Conserver ce schéma comme témoin, pas comme verdict.

Trois cas, référence d'onde simple S166 commune, centre initial 60 m, largeur 8 m,
direction droite, durée 6 s avant choc :

1. Q onde exacte a=0,05, total T=Q : résidu initial nul, S=0.
2. Q même onde, total exact de la même famille avec a=0,06 : perturbation non nulle,
   évolutive et non linéaire ; comparer directement à T analytique, pas à une superposition.
3. Q onde de a=0,05 **figée à t=0**, total T onde évolutive a=0,05. d initial nul,
   S=-∂xF(Q), dérivée analytique. Contre-épreuve omettant S dans le candidat.

Pour le fond figé, H'=-(x-60)·(H-1)/32 ; M=2H(c-c0), M'=(3c-2c0)H'.
S_h=-M', S_q=-[2M M'/H-M² H'/H²+gH H']. La conservation fournit ces expressions.
Une source omise doit échouer : préserver arbitrairement un fond faux n'est pas un succès.

Fantômes T analytique aux deux étages pour isoler la source intérieure ; ce n'est pas
une nouvelle réception du bord autonome S166. N global équivalent 120/240/480/960,
dt<=0,2 dx/sqrt(g), contrôle demi-pas N240. Échantillons aux centres comme S166.
Normalisations hauteur 0,05 m, débit 0,05 sqrt(g) m²/s pour les trois cas ; publier aussi
l'erreur de perturbation normalisée par 0,01 m pour le deuxième cas.

## 2. Bilans et critères

Attendre résidu nul au seuil 1e-10 pour le fond exact seul ; convergence spatiale contre
T pour les deux cas non nuls, sans fixer de seuil physique. Vérifier les tests S165/S166
après extension du support. Aucun choix d'API ou de schéma de production.

Le bilan du candidat est celui du **résidu avec source** : somme(d+−d0)dx égale
l'intégrale RK2 des flux différentiels et de S. Pour le total, ajouter somme(Q+−Q0)dx.
Ce n'est pas nécessairement le bilan des flux Rusanov du total aux frontières : publier
aussi cet écart, ne pas appeler « conservatif pour le total » un simple bilan résiduel.
Les sommes d'échantillons de Q ne sont pas des intégrales exactes en cellules.

## 3. Résultats

À recevoir en P3/P4.
