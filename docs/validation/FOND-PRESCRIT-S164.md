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

## 3. Résultats mesurés

`examples/fond_prescrit.rs` interroge Shallow1D uniquement pour l'état initial et la réception.
`step_prescribed` dans le support partagé ne connaît pas cette référence : les incréments
portent exclusivement sur le fond prescrit. Le calcul spatial de S163 est réutilisé, sans copie.
Les valeurs de Q aux deux étages sont mises en tampon. Le temps est donné par indice de pas
en f64 ; l'écart d'arrondi entre `t+dt` et le temps du pas suivant reste dans l'erreur publiée.

Campagne release du 2026-09-10 : L=120 m, h0=1 m, g=9,81 repris de `shallow::G`, durée 8 s,
deux gaussiennes initiales de S163 ; N240, dt nominal ≈0,2 dx/sqrt(g), ajusté par nombre entier
de pas. Fond : amplitude 0 / 0,05 / 0,2 m et mode 2 / 8 ; division de dt par 1 / 2 / 4 / 8.
Trois sources temporelles par montage : **72 exécutions**. Erreurs maximales sur cellules et
pas, normalisées par 0,3 m pour h et `0,3 sqrt(g)` pour q, comme S163.

### Incréments, dérivée continue et omission

Cas a=0,2 m, mode 8, N240 :

| division dt | incréments : h | incréments : q | dérivée continue : h | dérivée continue : q | omission : h |
|---:|---:|---:|---:|---:|---:|
| 1 | 8,88178e-15 | 4,52051e-15 | 2,19421e-4 | 1,59769e-4 | 2,762789 |
| 2 | 8,14164e-15 | 7,23009e-15 | 5,48012e-5 | 3,98645e-5 | 2,762767 |
| 4 | 3,47870e-14 | 2,37558e-14 | 1,37203e-5 | 9,97591e-6 | 2,762761 |
| 8 | 1,59872e-13 | 9,53516e-14 | 3,43258e-6 | 2,49520e-6 | 2,762759 |

Les incréments retrouvent le schéma total à l'arrondi près. La dérivée continue perd l'identité
mais son écart décroît d'un facteur proche de quatre : **ce n'est pas une source manquante**,
c'est une erreur temporelle de discrétisation, cohérente avec l'ordre deux. L'omission ne
disparaît pas au raffinement. Les deux échouent au critère d'identité numérique annoncé ; ce
verdict ne les met pas dans la même catégorie physique. Aucun critère d'acceptabilité de jeu
n'est déduit de cette tolérance d'instrument.

À amplitude nulle, les trois voies passent et rendent les mêmes chiffres. Les tests exigent
ce témoin. À même total initial, deux fonds distincts (a=0,05/mode2 et a=0,2/mode8) donnent
le même total reconstruit à la tolérance 1e-10 ; l'amplitude du résidu n'est donc pas une
mesure universelle de l'erreur du total, elle dépend aussi de la représentation du fond.

### Sensibilité au fond

À dt nominal, erreur hauteur de la source continue :

| a (m) | mode 2 | mode 8 |
|---:|---:|---:|
| 0,05 | 1,05840e-6 | 5,48545e-5 |
| 0,2 | 4,23362e-6 | 2,19421e-4 |

Amplitude multipliée par quatre, erreur multipliée par quatre sur cette famille. Fréquence
quadruplée, erreur multipliée par environ 52 ; ne pas appeler cela une loi universelle en
fréquence : la forme spatiale et le trajet dans le domaine changent aussi.
La masse reste à moins de 2e-15 de sa valeur initiale dans ces 72 exécutions, omission comprise.
Courant maximal 0,3224 ; aucune saturation ni état sec dans la référence.

### Raffinement de grille complémentaire

a=0,2/mode8, dt proportionnel à dx :

| N | incréments : h | incréments : q | dérivée continue : h |
|---:|---:|---:|---:|
| 120 | 8,14164e-15 | 6,26963e-15 | 7,46223e-4 |
| 480 | 1,25825e-14 | 8,36740e-15 | 6,05613e-5 |
| 960 | 8,88178e-14 | 5,13125e-14 | 1,61828e-5 |

Ces six exécutions ajoutées aux 72 précédentes vérifient la reconstruction sur quatre maillages
avec N240 ; elles ne séparent pas erreurs spatiale et temporelle de la référence.

## 4. Verdict et limites

**S163-1 réalisée, A219 traitée sur le véhicule RK2.** L'identité discrète annoncée est
construite et reçue ; la source continue converge et son omission ne converge pas.
Choix de l'instrument : employer les incréments pour les futures comparaisons voulant isoler
le couplage spatial au pas donné. Ce choix local n'impose pas une API au solveur 3D ; aucun ADR
nouveau ni calcul de bibliothèque modifié. ADR-112 reste applicable.

**A50 reste partielle** : sources temporelle et spatiale exactes dans ce montage, sans
interpolation grossière, pression 3D ni domaine local. Q n'est pas le B du runtime : c'est une
onde debout linéaire 1D, évaluée en f64/libm. Réception du même schéma, jamais preuve de sa
précision physique. Aucun seuil de bascule ni réception complète de B4.

Tests : quatre nouveaux tests propres à S164, trois host importés ; huit tests de S163
rejoués (cinq propres et trois host). Tous passent en debug. Campagne release complète avec
assertions reçue. Suite workspace 299/cinq ignorés reçue en S163, **non relancée** ici : seules
les sondes et leur support hors bibliothèque ont changé.

**Suite S165 : S164-1/A220**, localiser le domaine du résidu. Premier essai : fenêtre interne
sur le même canal, frontière alimentée par le fond seul contre frontière témoin alimentée par
la référence totale. Mesurer quand et comment une perturbation traversant la frontière
dégrade la reconstruction ; distinguer défaut de frontière et source intérieure.
Ne pas appeler la frontière témoin une solution utilisable en production : elle exige l'oracle.
