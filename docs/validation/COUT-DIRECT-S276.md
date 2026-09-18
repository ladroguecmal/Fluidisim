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
