# Fond mobile et cohérence temporelle — S173

## Protocole déclaré

Suite S172-1/A50. Même véhicule 1D mouillé f64, fenêtre [30,90] m, durée6 s,
Q onde simple amplitude0,05 m, centre55 m. Deux totaux analytiques : amplitude0,05
(préservation du fond seul), puis0,06 (perturbation de0,01). Moyennes spatiales exactes,
frontières analytiques connues. Le support de reconstruction de S172 est extrait,
étendu au temps courant ; son consommateur figé sera rejoué.

Q_H est linéaire en (h,q), moyenné par segments, nœuds fixes et bornes30/90 exactes.
Même total initial dans chaque montage : d0=T0-Q_H0. Àchaque pas ΔQ=Q1-Q0.
Source utilisée dans les deux étages RK2 :

```text
Trapezoid : S0=Lphys(Q0)-ΔQ/dt ; S1=Lphys(Q1)-ΔQ/dt.
Integrated : S0=S1=(divergence des flux physiques intégrés en temps - ΔQ)/dt.
Discrete : S0=Lnum(Q0)-ΔQ/dt ; S1=Lnum(Q1)-ΔQ/dt.
Omitted : S0=S1=0.
```

La sécante commune fait coïncider le total prédicteur avec le prédicteur RK2 total
dans le cas Discrete, puis le total final aussi : témoin d’identité indépendante via
API S165 sans source. Avec Q exact solution physique, Integrated doit fournir S≈0
par deux quadratures indépendantes (volume spatial, flux temporel). La préservation
de Q ne se déduit pas du témoin Discrete, qui conserve la diffusion du solveur total.

Budget physique intégré : flux Rusanov total moins flux Rusanov du fond intégré
par trapèze, plus flux physique du fond intégré en temps par quadrature adaptative.
Discrete utilise son flux numérique total seul. Défaut signé prédit indépendamment :

```text
Trapezoid : somme des [flux physique trapézoïdal - flux physique intégré] aux bornes.
Omitted : variation du volume de Q_H - flux physique intégré aux bornes.
Integrated et Discrete : zéro dans leurs budgets respectifs.
```

Pas de recalage du total. Q_H exact aux bornes rend leur flux physique indépendant
du réseau grossier. Le défaut de volume ne suffit donc pas à recevoir le champ local.
H=0 (exact),4,8 m ; phase0/H/2. N120/240/480 et demi-pas àN240, amplitudes0,05/0,06 :
160 évolutions résiduelles et8 témoins totaux. Erreurs maximales de h,q, résidu, identité,
volume et défaut signé prédit. Préservation, transport et conservation restent distincts.
Tolérances de quadrature1e-12, assertions f64 : pas de seuil physique is_smooth_at.

## Réception et limites de méthode

Commandes depuis code/ :

```text
cargo test -p water-core --example fond_mobile --example fond_reconstruit
cargo run -p water-core --release --example fond_mobile
```

Trois nouveaux tests reçus : préservation du fond exact avec source intégrée et témoins
qui la perdent ; identité discrète et volume sur fond mobile grossier perturbé, défaut
d’omission prédit ; réduction du défaut trapézoïdal avec le pas temporel. Quatre tests
S172 rejoués après extraction du support commun. Aucun autre support ni bibliothèque
modifié ; workspace non rejoué (S163 :299 réussis,5 ignorés).

La quadrature temporelle interroge encore le fond analytique à des instants intermédiaires.
Ce coût et cet accès ne représentent pas une source recalculée rarement dans un runtime.
Les nœuds spatiaux sont grossiers, mais leur évolution temporelle reste connue pendant
chaque pas. La séparation entre cadence du fond et pas du solveur demeure à recevoir.
Pas de3D, choc, eau sèche, forces ou perception reçus ; pas de schéma runtime adopté.

## Résultats reçus

N240, pas nominal. H=0 : Q exact ; a : amplitude du total analytique.
E et Eq : erreurs maximales espace/temps, normalisées par0,05 m et0,05√g.
R : maximum de |d.h|/0,05 ; V : défaut maximal du budget /volume initial.

| a | H | phase | mode | E | Eq | R | V |
|---:|---:|---:|---|---:|---:|---:|---:|
| 0.05 | 0 | 0 | Trapezoid | 2.862816e-4 | 3.028259e-4 | 2.862816e-4 | 5.844851e-9 |
| 0.05 | 0 | 0 | Integrated | 2.269296e-12 | 2.680656e-12 | 2.268830e-12 | 9.688458e-16 |
| 0.05 | 0 | 0 | Discrete | 1.295382e-1 | 1.379058e-1 | 1.295382e-1 | 8.806563e-16 |
| 0.05 | 0 | 0 | Omitted | 0.000000e0 | 0.000000e0 | 0.000000e0 | 1.177952e-15 |
| 0.05 | 4 | 0 | Trapezoid | 2.640418e-2 | 2.829308e-2 | 6.039660e-2 | 5.844851e-9 |
| 0.05 | 4 | 0 | Integrated | 2.642115e-2 | 2.831067e-2 | 6.040140e-2 | 7.487597e-16 |
| 0.05 | 4 | 0 | Discrete | 1.295382e-1 | 1.379058e-1 | 1.167333e-1 | 8.802313e-16 |
| 0.05 | 4 | 0 | Omitted | 3.721295e-2 | 3.960588e-2 | 5.667531e-2 | 1.815718e-5 |
| 0.05 | 4 | 0.5 | Trapezoid | 2.517574e-2 | 2.698528e-2 | 5.673960e-2 | 5.844851e-9 |
| 0.05 | 4 | 0.5 | Integrated | 2.518115e-2 | 2.699042e-2 | 5.673983e-2 | 8.033594e-16 |
| 0.05 | 4 | 0.5 | Discrete | 1.295382e-1 | 1.379058e-1 | 1.081549e-1 | 8.807134e-16 |
| 0.05 | 4 | 0.5 | Omitted | 3.912629e-2 | 4.159916e-2 | 5.617982e-2 | 8.670553e-6 |
| 0.05 | 8 | 0 | Trapezoid | 5.494960e-2 | 5.908869e-2 | 2.129545e-1 | 5.844851e-9 |
| 0.05 | 8 | 0 | Integrated | 5.494991e-2 | 5.908760e-2 | 2.129694e-1 | 9.889571e-16 |
| 0.05 | 8 | 0 | Discrete | 1.295382e-1 | 1.379058e-1 | 1.866255e-1 | 8.791026e-16 |
| 0.05 | 8 | 0 | Omitted | 1.645769e-1 | 1.746521e-1 | 1.493918e-1 | 1.183949e-4 |
| 0.05 | 8 | 0.5 | Trapezoid | 6.375095e-2 | 6.846296e-2 | 2.146514e-1 | 5.844851e-9 |
| 0.05 | 8 | 0.5 | Integrated | 6.377213e-2 | 6.848401e-2 | 2.146520e-1 | 9.893571e-16 |
| 0.05 | 8 | 0.5 | Discrete | 1.295382e-1 | 1.379058e-1 | 2.146543e-1 | 8.803527e-16 |
| 0.05 | 8 | 0.5 | Omitted | 1.668033e-1 | 1.778858e-1 | 2.111495e-1 | 1.193235e-4 |
| 0.06 | 0 | 0 | Trapezoid | 3.110145e-2 | 3.341178e-2 | 1.996980e-1 | 5.831232e-9 |
| 0.06 | 0 | 0 | Integrated | 3.123801e-2 | 3.355441e-2 | 1.996981e-1 | 8.258407e-16 |
| 0.06 | 0 | 0 | Discrete | 1.581916e-1 | 1.703421e-1 | 1.988614e-1 | 7.236285e-16 |
| 0.06 | 0 | 0 | Omitted | 3.123801e-2 | 3.355441e-2 | 1.996981e-1 | 1.031909e-15 |
| 0.06 | 4 | 0 | Trapezoid | 5.513881e-2 | 5.982057e-2 | 2.560036e-1 | 5.831231e-9 |
| 0.06 | 4 | 0 | Integrated | 5.517502e-2 | 5.985286e-2 | 2.560077e-1 | 9.520936e-16 |
| 0.06 | 4 | 0 | Discrete | 1.581916e-1 | 1.703421e-1 | 2.531021e-1 | 7.239848e-16 |
| 0.06 | 4 | 0 | Omitted | 6.412316e-2 | 6.950279e-2 | 2.541120e-1 | 1.811487e-5 |
| 0.06 | 4 | 0.5 | Trapezoid | 5.160953e-2 | 5.565880e-2 | 2.552447e-1 | 5.831232e-9 |
| 0.06 | 4 | 0.5 | Integrated | 5.171158e-2 | 5.577114e-2 | 2.552449e-1 | 9.238132e-16 |
| 0.06 | 4 | 0.5 | Discrete | 1.581916e-1 | 1.703421e-1 | 2.552461e-1 | 7.233292e-16 |
| 0.06 | 4 | 0.5 | Omitted | 6.887788e-2 | 7.457328e-2 | 2.539478e-1 | 8.650349e-6 |
| 0.06 | 8 | 0 | Trapezoid | 7.946786e-2 | 8.570995e-2 | 4.053966e-1 | 5.831232e-9 |
| 0.06 | 8 | 0 | Integrated | 7.950854e-2 | 8.575571e-2 | 4.054109e-1 | 7.761346e-16 |
| 0.06 | 8 | 0 | Discrete | 1.581916e-1 | 1.703421e-1 | 3.797993e-1 | 7.248258e-16 |
| 0.06 | 8 | 0 | Omitted | 1.928671e-1 | 2.068301e-1 | 3.367063e-1 | 1.181190e-4 |
| 0.06 | 8 | 0.5 | Trapezoid | 9.161417e-2 | 9.969331e-2 | 4.136009e-1 | 5.831232e-9 |
| 0.06 | 8 | 0.5 | Integrated | 9.163707e-2 | 9.971673e-2 | 4.136015e-1 | 6.357324e-16 |
| 0.06 | 8 | 0.5 | Discrete | 1.581916e-1 | 1.703421e-1 | 4.136038e-1 | 7.234289e-16 |
| 0.06 | 8 | 0.5 | Omitted | 1.575937e-1 | 1.705156e-1 | 4.096546e-1 | 1.190454e-4 |

Transport perturbé (a0,06), Integrated : résolutions indépendantes.

| N | facteur dt | H | phase | E | Eq |
|---:|---:|---:|---:|---:|---:|
| 120 | 1 | 0 | 0 | 5.043816e-2 | 5.435361e-2 |
| 120 | 1 | 4 | 0 | 8.746128e-2 | 9.472599e-2 |
| 120 | 1 | 4 | 0.5 | 8.249140e-2 | 8.934567e-2 |
| 120 | 1 | 8 | 0 | 1.297915e-1 | 1.410104e-1 |
| 120 | 1 | 8 | 0.5 | 1.469195e-1 | 1.593671e-1 |
| 240 | 1 | 0 | 0 | 3.123801e-2 | 3.355441e-2 |
| 240 | 1 | 4 | 0 | 5.517502e-2 | 5.985286e-2 |
| 240 | 1 | 4 | 0.5 | 5.171158e-2 | 5.577114e-2 |
| 240 | 1 | 8 | 0 | 7.950854e-2 | 8.575571e-2 |
| 240 | 1 | 8 | 0.5 | 9.163707e-2 | 9.971673e-2 |
| 480 | 1 | 0 | 0 | 1.779605e-2 | 1.913757e-2 |
| 480 | 1 | 4 | 0 | 3.166190e-2 | 3.437571e-2 |
| 480 | 1 | 4 | 0.5 | 2.992324e-2 | 3.229417e-2 |
| 480 | 1 | 8 | 0 | 4.637397e-2 | 5.004264e-2 |
| 480 | 1 | 8 | 0.5 | 5.252509e-2 | 5.724962e-2 |
| 240 | 2 | 0 | 0 | 3.124210e-2 | 3.355838e-2 |
| 240 | 2 | 4 | 0 | 5.517695e-2 | 5.985471e-2 |
| 240 | 2 | 4 | 0.5 | 5.173732e-2 | 5.579778e-2 |
| 240 | 2 | 8 | 0 | 7.953790e-2 | 8.578673e-2 |
| 240 | 2 | 8 | 0.5 | 9.161498e-2 | 9.969404e-2 |

## Lecture des résultats

Les160 prédictions du défaut signé sont reçues à<=2,42e-15 relatif. Les40 variantes
Integrated ferment le budget à<=2,42e-15. Les40 variantes Discrete retrouvent le
solveur total à<=2,23e-16 dans max(|Δh|,|Δq|/√g). Courant maximal0,220128.
Le fond exact seul est préservé par Integrated à E<=2,75e-12 (quadratures comprises).
Discrete, pourtant identique au solveur total, donne E0,12954 àN240 : son identité
ne constitue pas une préservation du fond physique.

ÀN240/Q exact/a0,05, Trapezoid donne E2,862816e-4 et V5,844851e-9 ; demi-pas :
E7,158499e-5 et V1,461244e-9, environ un quart. Le défaut temporel est mesuré sans
le confondre avec le volume de Q échantillonné ni avec le transport d’une perturbation.
Le flux de bord de Q_H étant exact, V trapézoïdal ne dépend pas du réseau grossier
pour une amplitude et un pas donnés ; ses champs locaux, eux, changent.

Omitted préserve trivialement Q exact seul : sans perturbation, son résidu reste nul.
Ce témoin ne suffit pas. Sur H8/phase0,5/a0,05, il laisse V1,193235e-4 àN240, contre
9,893571e-16 avec Integrated. Le défaut est expliqué par Δvolume(Q_H)-flux physique
intégré, sans correction globale. Aucun signe de production réelle allégué.

Pour la perturbation0,01 m, Integrated/Q exact donne E0,031238 àN240, soit15,62 %
de l’amplitude perturbée. H8/phase0,5 porte Eà0,091637, soit45,82 %. Raffiner N120→480
fait descendre cette dernière erreur0,146920→0,052525 ; diviser seulement dt par deux
àN240 la change peu. Le transport spatial et la reconstruction restent donc à distinguer
de la cohérence temporelle, même quand le bilan est fermé.

**S172-1 réalisée sur véhicule à fond mobile connu ; A50 reste partielle.** Aucun ADR,
angle ou runtime nouveau. A225 reste traitée dans son périmètre ; A216/A217 inchangées.

**Suite S174 : S173-1/A50**, séparer la cadence de réévaluation du fond du pas du solveur.
Échantillonner Q à une cadence grossière, interpoler entre instantanés et construire
la source depuis cette même représentation temporelle ; comparer à l’accès analytique
continu de S173. Mesurer erreurs aux réactualisations, transport et volume, sans seuil
universel. Porteur : construction, poursuite B4/BILAN-S145.
