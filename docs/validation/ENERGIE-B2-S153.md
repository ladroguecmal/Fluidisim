# B2 — énergie transportée à60s, S153, 2026-09-10

**Verdict :** sur la fixture source4m/0,01J, N512, la perte apparente de3,516 %
dans le disque80m est retrouvée dans l'anneau80–120m. B2-03 reçu sur cette fixture
pour le bilan énergétique60s, pas pour tous les impacts ni toute technologie W.
Aucun changement de production ni nouvel ADR ; application ADR-105 et méthode S129.

## Protocole déclaré avant calcul

g9,81/rho1025, profondeur20m, naissance0, durée source4s sans suppression de l'onde.
Mesure aux temps0/60s ; disque120m, sous-disque80m. N512 admet ce domaine élargi
sans modifier le garde de résolution. Énergie totale : potentiel de surface plus
cinétique intégrée sur z<0 ; tous les termes croisés de vitesse portent1/(ki+kj).
Ne pas substituer le carré de la vitesse de surface à son intégrale verticale.

Oracle indépendant S127 : quadrature spectrale256/512, Bessel par intégrale angulaire
1024 directions,2048 aux sept rayons sentinelles ; Simpson480/960 sur120m.
Candidat : coefficients, phases et Bessel de production, observable assemblée en f64
hors runtime ; contrôle de reconstruction des vitesses contre sample(), seuil1e-6.
Comparaison énergie/oracle<=1e-4 E0 ; raffinement spatial<=0,002 E0 ; fermeture dans
120m<=0,003 E0. Seuils inchangés après exécution, première fixture seulement.

## Mesures

| Temps s | Oracle E80/E0 | Candidat E80/E0 | Oracle E120/E0 | Candidat E120/E0 |
|---:|---:|---:|---:|---:|
| 0 | 1,000060620645 | 1,000060260811 | 1,000060621813 | 1,000060261990 |
| 60 | 0,964843720099 | 0,964843755487 | 0,999999680964 | 0,999999791824 |

Anneau80–120 à60s : oracle0,035155960865 E0, candidat0,035156036337 E0.
Écart maximal candidat/oracle<=3,60e-7 E0. Écart spectral oracle<=7,03e-10 E0 ;
écart spatial maximal1,018607e-3 E0 à la naissance (près du pic),8,66e-7 à60s.
Écart de densité angulaire aux sentinelles<=1,06e-19 E0 par m².
Le léger excès initial relève de la quadrature de mesure, déjà observé S127/S129.
Le déficit dans120m à60s vaut environ3,19e-7 E0 pour l'oracle : fermeture dans la
tolérance, pas preuve d'une énergie strictement nulle au-delà de120m.

Contre-épreuves : omettre la cinétique, ou omettre ses interférences, échoue à60s.
Le candidat conserve aussi l'accord de ses vitesses assemblées avec la surface publiée.
L'écart entre les deux dates dans120m reste inférieur à0,003 E0 ; un bilan limité
à80m aurait conclu à tort à une dérive supérieure au critère2 % du dossier B2.

## Reproduction et limites

Depuis code/ : `cargo run -p water-core --release --example b2_energy`, puis
`cargo test -p water-core --release energy_leaves_disk -- --nocapture`.
Le test privé généralise Measurement par const N ; le test S129 garde N256 et ses
références. Aucun nouvel accès public aux coefficients et aucune allocation runtime.

On mesure une redistribution spatiale, pas directement l'intégrale temporelle du
flux à80m. Pas de choix de lambda_cut, capacité4096 sources, sillage stationnaire,
bathymétrie ou D1 interplateforme. Deux bancs restent partiellement exécutés (B1/B2).
**S153-1 : prochaine S154, étendre ce bilan aux sources2/3/5/6m de S152**, avec rayon
de collecte choisi et reçu pour chacune ; garder oracle et contre-épreuves avant
de conclure sur toute la bande de fixtures. B2 reste la trajectoire du dernier bilan.
