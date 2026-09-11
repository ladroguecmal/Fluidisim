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
