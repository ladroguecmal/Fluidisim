# B2 — bilan énergétique des quatre autres sources, S154, 2026-09-10

Campagne S153-1 : complète la source4m reçue en S153. Aucun changement de production.
Mêmes E0=0,01J, g9,81/rho1025, h20m, instants0/60s. B2 reste une réception locale
partielle du candidat radial, sans choix de technologie ou de lambda_cut.

## Protocole annoncé

| Source m | Collecteur m | Pas spatial fin m | Profil collecteur |
|---:|---:|---:|---|
| 2 | 88 | 0,0625 | N512 |
| 3 | 112 | 0,0625 | N512 |
| 5 | 136 | 0,125 | N512 |
| 6 | 152 | 0,125 | N512 |

Chaque collecteur dépasse le trajet de groupe maximal à60s ; cela motive un essai,
mais ne certifie pas sa fermeture. Le disque80m est intégré séparément. N256 pour5/6m
reste le profil S152 de ce disque seulement ; la réception du collecteur ne le remplace pas.

Oracle indépendant S127, paramètres de longueur explicités : spectral256/512,
directions1024 et2048 aux sentinelles tous les160 pas et à la frontière. Simpson fin
et pas doublé. Seuils inchangés : spectral/candidat<=1e-4 E0, spatial<=0,002 E0,
fermeture collecteur<=0,003 E0. Pas affiné pour les petites longueurs car le pic
initial se rétrécit avec la longueur. Aucune adaptation après résultats prévue.

Reproduction : depuis code/, `cargo run -p water-core --release --example b2_energy -- 2`
et arguments3/5/6. Sans argument conserve la fixture4m S153.
Les campagnes d'oracle peuvent s'exécuter simultanément : aucun coût n'est mesuré ici.
L'énergie totale conserve les termes croisés et leur intégrale verticale1/(ki+kj).
Le défaut de collecte n'est ni une dissipation ni une intégrale directe du flux de bord.

## Référence mesurée

Rapports à E0 ; chaque ligne donne les disques80m et collecteur au temps indiqué.

| Source m / temps s | E80/E0 | Ecollecteur/E0 | Anneau/E0 |
|---|---:|---:|---:|
| 2 /0 | 1,000060621948 | 1,000060621964 | 0,000000000016 |
| 2 /60 | 0,999999481614 | 0,999999975071 | 0,000000493457 |
| 3 /0 | 1,000011753834 | 1,000011754095 | 0,000000000261 |
| 3 /60 | 0,999111648388 | 0,999999976655 | 0,000888328266 |
| 5 /0 | 1,000024528388 | 1,000024532205 | 0,000000003817 |
| 5 /60 | 0,787762595879 | 0,999999630848 | 0,212237034970 |
| 6 /0 | 1,000011743876 | 1,000011753740 | 0,000000009864 |
| 6 /60 | 0,446944872471 | 0,999999694485 | 0,553054822015 |

Le disque80m manque environ21,224 % à5m et55,305 % à6m ; cette énergie se trouve
plus loin. À2m, presque toute l'énergie reste encore dans80m. La taille de collecte
ne se transpose donc pas d'une longueur d'onde à l'autre à durée identique.

## Réception du candidat

Observable S129 depuis les coefficients réellement cuits, phases et Bessel de production ;
assemblage énergétique f64 hors runtime. Les vitesses assemblées restent comparées à sample()
(seuil1e-6 normalisé). Chaque énergie est comparée à l'oracle indépendant ci-dessus.

| Source m / N / rayon | E80/E0 à0s | Ecollecteur/E0 à0s | E80/E0 à60s | Ecollecteur/E0 à60s |
|---|---:|---:|---:|---:|
| 2 /512 /88 | 1,000060262152 | 1,000060262170 | 0,999999796164 | 1,000000289417 |
| 3 /512 /112 | 1,000011906027 | 1,000011906301 | 0,999111707548 | 1,000000052974 |
| 5 /512 /136 | 1,000024809735 | 1,000024813544 | 0,787762620518 | 0,999999562700 |
| 6 /512 /152 | 1,000011896054 | 1,000011905932 | 0,446945053321 | 0,999999823405 |
| 5 /256 /80 | 1,000024469097 | même disque | 0,787762416677 | même disque |
| 6 /256 /80 | 1,000011811824 | même disque | 0,446944311999 | même disque |

Écart maximal candidat/oracle<=5,61e-7 E0, seuil1e-4 ; fermeture des collecteurs
à moins de0,003 E0 et raffinement spatial candidat à moins de0,002 E0.
Référence : écart spectral maximal1,076e-9 E0 ; spatial maximal1,019e-3 E0,
à la naissance2m. Densité angulaire aux sentinelles<=2,669e-18 E0 par m².
Le très petit excès initial relève de la quadrature de mesure (S153), pas d'une
création d'énergie démontrée. À60s les collecteurs sont à moins de4,38e-7 E0 de1
pour le candidat. Pas de preuve d'une queue exactement nulle hors collecteur.

Deux tests nouveaux, `energy_band_collectors_s154` et `energy_band_selected256_s154` :
références copiées à12 décimales, arrondi<=5e-13 E0. Omettre la cinétique ou supprimer
ses interférences échoue sur toutes les fixtures à60s ; garde-fous inchangés.
Commande : `cargo test -p water-core --release energy_band -- --nocapture`.

## Verdict et suite

S153-1 réalisée. Avec S153, le bilan initial/60s est reçu sur les cinq longueurs
sources2/3/4/5/6m du volet impact B2. Le profil256 reste reçu dans80m pour5/6m ;
N512 sert à vérifier l'énergie qui a dépassé ce domaine. Le fait d'observer le
collecteur plus grand ne certifie pas N256 sur celui-ci.
Aucun coût ni seuil de qualité modifié, aucun nouvel ADR ou changement de production.
Deux bancs restent partiels : B1 et B2. La réception d'impacts ne vaut pas Kelvin,
sillage long, réfraction, capacité4096 sources ou preuve multiplateforme.

**S154-1 : prochaine S155, B2 sillage.** Quantifier la couverture temporelle/spatiale
requise par B2-01 face au contexte de pression16s ; construire un premier scénario
de sillage prolongé recevable ou identifier par mesure le blocage numérique précis.
Ne pas transposer l'admission d'impact60s au noyau de pression. La prochaine production
reste une étape de B2 selon le dernier bilan, pas une sélection technologique globale.
