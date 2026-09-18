# δ en direct : coût d'un pas dans l'image — S276

Objectif : faire tourner la bande δ de S275 ([ADR-168](../adr/ADR-168-premier-rendu-de-delta.md))
pendant l'affichage, un pas par image, au lieu d'un rejeu précalculé. Critères écrits avant la
mesure et avant toute technique.

## Carte du coût (P2)

Scène `--delta` (128 × 52 mailles, 6 656), houle S275, pas de 16 ms, 200 pas depuis le repos,
un fil, secteur, sans autre charge, deux passages. Postes : **échantillonnage** des faces u
(6 708) et w (6 784) par `differential_local_extended`, et **pas couplé** mobile complet.
Médiane et maximum par pas, itérations au pire. Le poste dominant est traité en premier ; aucune
cible n'est revendiquée avant mesure.

## Critères des techniques (P3–P5)

- **Échantillonnage par grille** : identique au bit, champ par champ, à l'évaluation ponctuelle
  sur des grilles couvrant `z` négatif, nul et positif ; mêmes refus (domaine, non fini) ;
  zéro allocation. Aucune réception physique à refaire si l'identité tient.
- **Technique sur le pas** : mêmes portes d'acceptation (ADR-143/144) ; accord avec le chemin
  actuel sous la tolérance de S274 (vitesse ≤ 10⁻⁴ du maximum sur vingt pas) ; itérations et coût.
- **δ en direct** : un pas par image au pas simulé fixe de 16 ms ; `η'` identique au bit au rejeu
  de 16 ms aux mêmes instants (même fond, même pas) ; cadence et coût CPU par image publiés,
  techniques présentes/absentes/domaine (ADR-131). Le budget de 2 ms n'est pas revendiqué.

## Résultat de la carte (P2)

`water-viewer --delta-cout`, secteur, deux passages :

| poste | médiane (ms) | p95 (ms) | maximum (ms) |
|---|---:|---:|---:|
| échantillonnage de B, 13 492 faces | **33,7 ; 33,2** | 38,6 ; 38,2 | 43,9 ; 42,5 |
| pas couplé mobile | **24,8 ; 24,2** | 32,8 ; 31,2 | 40,9 ; 41,7 |

Itérations au pire : 23. L'échantillonnage domine (58 % du total, ≈ 58 ms par image, ≈ 17 images/s).
Il passe en premier. Le fil unique est lancé par `thread::scope`, coût négligeable ici (S243).

## Échantillonnage par grille (P3)

`Background::differential_grid_extended` : la boucle des composantes passe à l'extérieur, la
phase et son sinus/cosinus sont calculés une fois par colonne, l'atténuation une fois par couche ;
chaque face reçoit les mêmes termes dans le même ordre. Essai du cœur : identité au bit, champ
par champ, sur 37 × 10 points (z de −96 à +3 m, plan moyen compris), fond directionnel et houle
plane, `y` non nul ; refus de capacité, domaine et densité. Dans l'afficheur, les 13 492 faces des
200 pas sont comparées aux deux chemins : **identiques**.

| poste | médiane (ms) | p95 (ms) | maximum (ms) |
|---|---:|---:|---:|
| échantillonnage ponctuel | 33,0 ; 33,0 | 38,1 ; 38,6 | 50,3 ; 44,8 |
| **échantillonnage par grille** | **5,14 ; 5,11** | 6,84 ; 6,93 | 10,8 ; 9,7 |
| pas couplé | 24,2 ; 24,2 | 29,8 ; 30,3 | 36,8 ; 40,4 |

Facteur **6,4**, sans aucune réception physique à refaire. Le pas couplé domine désormais (83 %).

## Départ depuis la pression publiée (P4, ADR-169)

Essai du cœur, vingt pas couplés à 64 colonnes sous l'onde stationnaire S253 : **134 itérations
contre 266**, vitesse à 8,0·10⁻⁸ m/s du départ nul (1,4·10⁻⁵ du maximum), hauteur identique.
Identité fond nul / S237 conservée ; harmonique S253 2,14 % inchangée ; premier pas à 128 colonnes
inchangé (18 itérations, pression nulle au repos).

| poste, scène `--delta` | médiane (ms) | p95 (ms) | maximum (ms) |
|---|---:|---:|---:|
| échantillonnage par grille | 5,30 ; 5,24 | 6,83 ; 7,13 | 8,6 ; 12,3 |
| **pas couplé** | **17,7 ; 17,4** | 25,7 ; 23,9 | 30,4 ; 29,3 |

Itérations au pire 23 → 17. **Un pas par image ≈ 23 ms**, contre 58 ms à l'ouverture de S276.
