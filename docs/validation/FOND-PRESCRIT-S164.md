# S164 — Fond prescrit et clôture temporelle

2026-09-10. S163-1/A219 ; extension de l'instrument S163, aucune bibliothèque runtime modifiée.

## 1. Dérivation et expérience déclarées avant mesure

Soit `L` le résidu spatial Rusanov S163. Avec `T=Q+d`, l'équation est
`d_t=L(Q+d)-Q_t`. La partie spatiale reste le calcul développé de S163,
`L(Q+d)-L(Q)`, auquel est ajouté `L(Q)` : on ne supprime aucun couplage spatial.

RK2 du total : `T1=T0+dt L(T0)` puis `T+=0,5(T0+T1+dt L(T1))`.
Pour un fond prescrit, choisir `Q0=Q(t)`, `Q1=Q(t+dt)` et `Q+=Q1`.
Les corrections exactes au niveau des **incréments** sont alors :

```
d1 = d0 + dt L(Q0+d0) - (Q1-Q0)
d+ = 0,5 [d0+d1+dt L(Q1+d1) - (2Q+-Q0-Q1)]
```

Ici les deux différences valent `Q1-Q0`. Substituer dans `Q1+d1` et `Q++d+`
retrouve algébriquement RK2 du total. Le code doit néanmoins intégrer d avec ses propres
tableaux et recevoir cet accord ; aucun état total avancé n'est disponible dans le véhicule.
Ce n'est pas une nouvelle approximation physique, mais un changement de variables discret.

**Comparaison prévue :** remplacer ces différences par `dt Q_t(t)` et `dt Q_t(t+dt)`.
Cette voie est une discrétisation cohérente de l'équation continue du résidu, mais son premier
étage total diffère de l'étage RK2 du total de `Q1-Q0-dt Q_t(t)` ; elle n'est pas identique
au même pas. Le raffinement doit être mesuré avant de qualifier la différence d'instabilité.
Troisième voie : retirer la source temporelle, garder tout le calcul spatial.

Fond analytique stationnaire dans l'espace au sens d'une onde debout, mais **instationnaire** :
`H=h0+a cos(kx) cos(ωt)`, `M=a c sin(kx) sin(ωt)`, avec `c=sqrt(gh0)`, `ω=ck`.
Ces expressions satisfont les équations de Saint-Venant **linéarisées** sur h0 par dérivation
directe : `H_t+M_x=0`, `M_t+gh0 H_x=0`. Elles ne sont pas une solution non linéaire.
Choisir `k=mπ/L` entier respecte les murs pour M. Source temporelle analytique obtenue en
dérivant ces deux expressions. Dispersion peu profonde : SPEC-001 §1.

Même état total initial que S163 (deux gaussiennes, débits nuls), donc
`d(0)=T(0)-Q(0)` à l'initialisation uniquement. Cela sépare le choix du fond du phénomène total.
Les amplitudes et fréquences du fond sont des paramètres du montage de mesure, pas du runtime.

## 2. Réception annoncée

- Comparer hauteur et débit au total Shallow1D à **chaque pas**, à pas identiques. Critère
  d'arrondi normalisé 1e-10 de S163, pas de seuil architectural.
- Fixer N et diviser dt par 2/4/8 ; publier les deux composantes de l'écart pour chaque voie.
- Modifier amplitude et fréquence du fond en gardant le total initial identique ; la voie
  discrète doit retrouver le même total. La voie omise doit échouer, son témoin a=0 passer.
- Réévaluer le fond une seule fois par état d'étage, vérifier finitude, profondeur, Courant.
  Aucun écrêtage ni source provenant d'une référence totale.
- Tester les dérivées analytiques contre différences centrées et les témoins stationnaires.
  Rejouer les tests S163 puisque leur support est étendu.

Portée : même schéma 1D mouillé sur tout le domaine. Ni précision physique, ni domaines locaux,
ni eau profonde, ni interpolation des dérivées, ni conformité multiplateforme ne sont reçues.
