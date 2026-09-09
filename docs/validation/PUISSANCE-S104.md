# S104 — Puissance et bilan du candidat

2026-09-08. Prolonge ADR-071/073 sans changer leurs décisions. S103-1 réalisée sur fixture.

## Construction

`ModalPressure::pressure` expose la pression complexe avec les mêmes phases Q32 que la
réponse. Active sur [naissance, extinction), nulle avant/après ; horizon dépassé refusé.
`spectral_pressure::Field::power_w` vaut -Σ poids Re(P conj(qdot_total)). La vitesse
est additionnée avant le produit : le travail inclut les interférences du virage.
Somme compensée f32, ordre fixé ; aucune allocation nouvelle. La vue n'est publiée
qu'après contrôle de finitude de l'énergie et de la puissance. Le pool candidat peut
être modifié au refus, comme auparavant. Reconstruction et recette restent inchangées.

Il s'agit du bilan global de la quadrature de Fourier du profil localisé, non de
l'énergie d'une somme finie d'ondes planes intégrée sur un plan infini. La pression
spectrale incorpore la transformée spatiale. Aucun bilan entre couches B/W/V ajouté.

## Réception

`receive_power` en release : virage S91/S103, σ=1 m, coupure6/m, pression10 Pa,
g=9,81f32, densité1025, départ(0,0), vitesse(2,0) pendant2 s puis(0,2) pendant2 s.
Demi-spectre ; comparaison f64 à même résolution en dix instants, dont ±1 µs autour
du virage et extinction. Seuil puissance1e-7 W. Le test modal couvre aussi une époque
proche de u64::MAX et le refus au-delà de l'horizon.

| Résolution | Énergie à4 s J | Erreur max puissance W | Écart travail/énergie, 200 pas J | 400 pas J | 800 pas J |
|---|---:|---:|---:|---:|---:|
|128×128|0,1262611448765|1,272e-8|1,921e-6|4,883e-7|1,303e-7|
|112×80|0,1262627243996|7,721e-9|1,913e-6|4,807e-7|1,220e-7|

Intégration aux milieux, pas20/10/5 ms sur0–4 s ; accumulateur f64 d'instrumentation,
pas une conversion du temps dans le runtime. Assertions : résidu<3e-6 J et réduction
à moins de35 % du précédent à chaque raffinement. À4 et8 s puissance exactement
nulle ; énergies identiques en f32 aux deux instants (seuil de test1e-7 J).

Contrôle spatial indépendant à3 s : grille40², pas0,3 m, centrée(4,2), ±6σ,
intégrale de -p*w. Écarts au bilan spectral7,232e-9 et6,987e-9 W, seuil1e-7 W.
La gaussienne de cet instrument utilise exp f64, hors runtime. Cette réception
n'établit aucune borne continue ni validité d'autres trajectoires, durées ou profils.
L'énergie publiée en S91 avec32×48 ne doit pas être substituée à ces quadratures.

Suite debug complète :125 core +93 harnais =218 réussis, cinq ignorés ; quatre
avertissements préexistants. Les trois réceptions spectrales existantes vérifient
maintenant aussi la puissance contre f64, spectres fourni/cuit/réduit.

## Coût et suite

Banc S103 relancé après fin des autres calculs, même machine,21 mesures release :
préparation médiane6528,6 µs (128²),3597,5 µs (112×80) ; lot64 respectivement
20168,2 et11737,4 µs. Première série concurrente aux tests écartée. Les coûts S103
restent des mesures de la version S103, sans puissance ; aucune garantie de gain
ni budget cible. Taille des pools de champs inchangée, scalaire supplémentaire dans
la vue ; aucun compteur global d'allocations utilisé.

**S104-1, S105 :** construire l'enveloppe candidate de pression mobile liée à un
contexte explicite (recette, gravité, densité, domaine et horizon), avec publication
atomique sur pools hôte et refus de contexte incompatible. Préparer ainsi son
raccordement B+W sans annoncer un codec ni une intégration LiveWater déjà réalisés.
Conformité interplateforme et sauvegarde autoritaire du sillage restent ouvertes.
73 ADR,193 angles morts,17 invariants,6 spécifications,23 cas : inchangés.

**Mise à jour S105, 2026-09-09 :** S104-1 réalisée comme enveloppe empruntée ; voir
[CONTEXTE-PRESSION-S105](CONTEXTE-PRESSION-S105.md). Contexte et instant liés aux
requêtes, publication précédente conservée au refus. Suite S105-1 : requête monde B+pression.
