# S55 — Extrema de seiche sur un plateau

2026-09-07. Action S54-1. Véhicule `delta`, hauteurs f32 ; mesure f64.

## Cause et correction

Le détecteur comparait deux pentes consécutives. Une pente nulle effaçait le sens précédent :
montée, plateau, descente ne produisait aucun maximum. Sur `Bassin::c03(true)`, 60 s,
les deux résolutions 200/400 ont sept passages à zéro descendants, mais seulement huit/cinq
extrema détectés. Le minimum de six refusait donc la seconde mesure.

Conserver la dernière pente non nulle donne treize extrema aux deux résolutions. Un plateau
ne compte qu'à sa sortie dans le sens opposé ; un plateau final sans retournement ne compte pas.
Solveur, seuils et passages à zéro inchangés. L'instant retenu reste celui de détection ;
la localisation au sein d'un plateau reste limitée par l'échantillonnage.

| 60 s | Avant : demi-vie en périodes | Après | Période après (s) | R² après |
|---|---:|---:|---:|---:|
| nx=200 | 25,23784744 | 25,08627092 | 9,03017042 | 0,99979590 |
| nx=400 | refus | 50,34244725 | 9,02994616 | 0,99964488 |

## Campagne nominale

Commande depuis `code/` : `cargo run --release --offline -- physics scenarios/C02-dispersion.toml scenarios/C18-invariants.toml`.
Comparaison avec S54 ; seules les estimations issues des extrema changent, hors durées machine.

| Mesure (demi-vie en périodes sauf indication) | Avant | Après |
|---|---:|---:|
| C03 rampe, vingt périodes | 20,69 | 21,20 |
| R² rampe | 0,9920 | 0,9924 |
| C03 mode propre, vingt périodes | 24,40 | 24,45 |
| Résolution N=320 / 640 points par λ | 19,70 / 38,81 | 19,69 / 38,76 |
| Courant 0,45 / 0,60 / 0,70 / 0,90 | 10,10 / 13,72 / 17,91 / 45,33 | 10,11 / 13,72 / 17,91 / 45,29 |
| Amplitude a/h=0,0010 / 0,0025, nx=400 | refus / refus | 25,23 / 44,62 |
| Amplitude a/h=0,010 / 0,025 / 0,050 | 48,65 / 33,72 / 15,47 | 48,35 / 34,07 / 15,44 |
| k moyen, a/h=0,010 / 0,050 | 0,06253 / 0,04901 | 0,06251 / 0,04900 |
| Harmonique 1, périodes / secondes / écart théorie | 48,65 / 439,33 / −4,75 % | 48,35 / 436,58 / −5,35 % |
| Harmonique 2 | 24,41 / 110,23 / −4,40 % | 24,44 / 110,37 / −4,28 % |
| Harmonique 3 | 16,50 / 49,66 / −3,11 % | 16,50 / 49,67 / −3,08 % |
| Harmonique 4 | 12,49 / 28,19 / −2,21 % | 12,49 / 28,19 / −2,20 % |

Les rapports harmoniques en secondes deviennent environ 3,96 / 8,79 / 15,49 pour 4 / 9 / 16
prédits. La conclusion qualitative du filtre reste compatible avec ces mesures. La correction
ne démontre pas la cause du biais résiduel ni l'indépendance en amplitude : les nouvelles petites
amplitudes restent très différentes entre elles. Une mesure disponible ne vaut pas validation
de son interprétation physique.

## Vérification et limites

123 tests passent (38 cœur, 85 harnais), deux ignorés. Tests analytiques : plateau au maximum
et au minimum, extrema sans plateau, monotonie avec plateaux, signal constant, plateau final.
Témoins physiques nx=200/400 sur 60 s : présence, période à moins de 1 %, R² > 0,9.
Les tests à excitation nulle restent verts. Aucun seuil diminué.

Compilation release réussie. Verdicts de la campagne inchangés : sortie 1 pour le seul C04
ordre un attendu en échec ; douze assertions shallow et son diagnostic de cohérence réussis.
`check` : zéro échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e` inchangés.
La campagne coûteuse C22 dédiée n'est pas répétée ; S49-1 reste ouverte.
