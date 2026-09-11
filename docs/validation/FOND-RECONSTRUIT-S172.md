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

## Résultats reçus

N240, pas nominal. H=0 désigne le fond exact. R : erreur de représentation de h /0,05 m ;
E et Eq : erreurs maximales espace/temps de hauteur et débit, normalisées par0,05 m
et0,05√g ; D : écart de hauteur au témoin Physical/Q exact /0,05 m.
V : défaut du budget de flux /volume initial. Même total initial dans tous les cas.

| H | phase | source | R | E | Eq | D | V |
|---:|---:|---|---:|---:|---:|---:|---:|
| 0 | 0 | Physical | 0.000000e0 | 1.356122e-1 | 1.442323e-1 | 0.000000e0 | 9.234265e-16 |
| 0 | 0 | Discrete | 0.000000e0 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.734234e-16 |
| 1 | 0 | Physical | 2.575332e-3 | 1.351717e-1 | 1.437608e-1 | 2.949620e-3 | 9.101287e-16 |
| 1 | 0 | Discrete | 2.575332e-3 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.939791e-16 |
| 1 | 0.5 | Physical | 2.598072e-3 | 1.351291e-1 | 1.437190e-1 | 2.971038e-3 | 8.962732e-16 |
| 1 | 0.5 | Discrete | 2.598072e-3 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.933862e-16 |
| 2 | 0 | Physical | 1.420300e-2 | 1.350717e-1 | 1.436486e-1 | 7.242664e-3 | 7.933899e-16 |
| 2 | 0 | Discrete | 1.420300e-2 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.675942e-16 |
| 2 | 0.5 | Physical | 1.365264e-2 | 1.349930e-1 | 1.435742e-1 | 7.069449e-3 | 9.537063e-16 |
| 2 | 0.5 | Discrete | 1.365264e-2 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.896108e-16 |
| 4 | 0 | Physical | 5.704671e-2 | 1.346255e-1 | 1.431539e-1 | 1.661057e-2 | 7.618812e-16 |
| 4 | 0 | Discrete | 5.704671e-2 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.712767e-16 |
| 4 | 0.5 | Physical | 5.704671e-2 | 1.344774e-1 | 1.430250e-1 | 1.578045e-2 | 8.977331e-16 |
| 4 | 0.5 | Discrete | 5.704671e-2 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.704445e-16 |
| 8 | 0 | Physical | 1.502244e-1 | 1.325812e-1 | 1.410101e-1 | 3.225683e-2 | 6.860474e-16 |
| 8 | 0 | Discrete | 1.502244e-1 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.929254e-16 |
| 8 | 0.5 | Physical | 2.122024e-1 | 1.333788e-1 | 1.417208e-1 | 2.961593e-2 | 8.216131e-16 |
| 8 | 0.5 | Discrete | 2.122024e-1 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.806242e-16 |
| 16 | 0 | Physical | 6.165692e-1 | 1.281124e-1 | 1.363602e-1 | 4.611122e-2 | 7.366030e-16 |
| 16 | 0 | Discrete | 6.165692e-1 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.881285e-16 |
| 16 | 0.5 | Physical | 2.278437e-1 | 1.295664e-1 | 1.377982e-1 | 3.112190e-2 | 8.585194e-16 |
| 16 | 0.5 | Discrete | 2.278437e-1 | 1.295382e-1 | 1.379058e-1 | 4.856412e-2 | 8.871498e-16 |

Physical, H8m, phase0 : variation de dx et dt indépendamment du réseau du fond.
G∞=max|Lphys(Q)-Lnum(Q)| pour la masse, G1=Σ|Lphys(Q)-Lnum(Q)|dx.
Unités respectives m/s et m²/s. Aucune normalisation par une valeur arbitraire.

| N | facteur temporel | G∞ | G1 | E | D |
|---:|---:|---:|---:|---:|---:|
| 120 | 1 | 1.267492e-2 | 2.589395e-2 | 2.211411e-1 | 5.792168e-2 |
| 240 | 1 | 1.268475e-2 | 1.300650e-2 | 1.325812e-1 | 3.225683e-2 |
| 480 | 1 | 1.268967e-2 | 6.518162e-3 | 7.364631e-2 | 1.699716e-2 |
| 240 | 2 | 1.268475e-2 | 1.300650e-2 | 1.326178e-1 | 3.225871e-2 |

## Ce que la campagne établit

Le total initial est restitué à<=3,47e-18 (maximum des écarts absolus h et q, chacun
dans son unité). Les44 variantes Discrete restent identiques au solveur total
indépendant à<=2,23e-16 dans max(|Δh|,|Δq|/√g). Les88 budgets ferment à<=2,29e-15
relatif ; Courant maximal0,215253. Un gros écart de représentation ne change donc
pas nécessairement le total : àH16/phase0, le résidu initial porte61,7 % de l’amplitude
sans erreur initiale de hauteur. Il compense ici une représentation, pas un nouvel impact.

Physical dépend encore de Q_H par le défaut Lphys(Q_H)-Lnum(Q_H). ÀN240/H8/phase0,
D=0,03226 ; S171, avec Q exact et seulement le flux grossier ancré, donnait0,22805.
Ce sont deux montages différents : leur comparaison motive la cohérence conjointe,
elle ne constitue pas une preuve universelle de supériorité de cet interpolant.

ÀH8 fixé, raffiner dx divise environ par deux G1 alors que G∞ reste près de0,01269.
D diminue aussi (0,05792→0,016997). Ce comportement est compatible avec un défaut
localisé autour des ruptures de pente du fond linéaire : le maximum seul ne rend pas
compte de son étendue. La mesure de G1 a été ajoutée après ce constat ; les quatre tests
et la campagne ont été relancés. Aucun ordre asymptotique général n’est déclaré.

Discrete donne ici E0,12954 contre0,13561 pour Physical/Q exact àN240. Cela ne révoque
pas S167 : son fond figé asymétrique doit évoluer. Sur un fond qui est déjà une solution
physique, substituer Lnum à Lphys réintroduirait précisément la diffusion du solveur que
la formulation équilibrée cherchait à éviter. Il faut recevoir aussi un fond mobile.

## Réception et portée

Depuis code/ :

```text
cargo test -p water-core --example fond_reconstruit
cargo run -p water-core --release --example fond_reconstruit
```

Quatre nouveaux tests reçus : moyennes traversant les nœuds et bornes exactes ; même
total initial et identité discrète malgré le fond grossier ; effet du raffinement du
fond ; comportement distinct du maximum et de la norme intégrée du défaut de source.
88 évolutions résiduelles et4 témoins totaux reçus. Aucun support partagé modifié ;
les tests antérieurs ne sont pas rejoués (workspace S163 :299 réussis,5 ignorés).
Les assertions1e-10/1e-12 servent au contrôle numérique f64, pas à is_smooth_at.

**S171-1 réalisée sur véhicule figé ; A50 reste partielle.** Sources et Q construits
ensemble, bords analytiques connus, pas de3D, eau sèche, choc, force ou perception reçus.
A225 reste traitée dans son périmètre ; pas de nouvel ADR, angle ou runtime adopté.

**Suite S173 : S172-1/A50**, faire évoluer Q_H en temps depuis le fond analytique mobile.
Dériver une source cohérente avec sa variation temporelle aux étages RK2, conserver
le témoin d’identité discrète et mesurer préservation, transport et volume. Varier le
réseau spatial et le pas temporel séparément. Porteur : session de construction,
poursuite B4/BILAN-S145 ; aucun résidu entrant inconnu supposé disponible.

**Suivi S173 : S172-1 réalisée sur véhicule mobile connu**, voir
[FOND-MOBILE-S173](FOND-MOBILE-S173.md). Sécante commune aux étages, flux intégrés
indépendamment ; fond exact préservé, défauts temporels et omission prédits. A50 partielle :
la cadence de réévaluation de Q reste à séparer du pas du solveur (S173-1).
