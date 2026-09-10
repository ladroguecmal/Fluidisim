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
