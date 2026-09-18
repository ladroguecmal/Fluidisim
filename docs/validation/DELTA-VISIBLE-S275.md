# δ visible dans l'afficheur — S275

Réception d'[ADR-168](../adr/ADR-168-premier-rendu-de-delta.md). Protocole écrit avant le code.

## Critères de fonctionnement

1. **Précalcul** : tous les pas reçus aux deux pas de temps, sans refus ; échantillons plans ;
   `η'` fini. Publier `max|η'|`, `η'` rms et leur rapport à `Hs`.
2. **Écart entre pas de temps** : `η'(16 ms) − η'(4 ms)` en rms et en maximum, en mm, et
   rapporté à `η'` rms. Comparé aux 3 mm de hauteur d'image (S201) : **mesure**, pas verdict
   visuel.
3. **Couche GPU contre CPU** : hauteur et pentes de la couche δ (fondus compris) sur des sondes,
   GPU contre évaluation CPU f64 de la même lecture Hermite : écart ≤ 0,1 mm et ≤ 10⁻⁵ en pente.
4. **Rien ne change hors `--delta`** : scène S201 et ses vérifications au bit (`--verify`).

## Revue demandée à l'utilisateur

Poses de jeu à 1280 × 720, trois variantes par pose (B seul, B+δ 4 ms, B+δ 16 ms), mêmes instants.
Questions, fixées avant de voir les images :

- la bande B+δ se distingue-t-elle de B seul, et comment (crêtes, creux, reflets) ?
- la limite de la bande est-elle visible ?
- les deux pas de temps se distinguent-ils ?

Références utiles : houle longue sans mer de vent marquée, vue d'un pont ou d'une côte à 5–20 m
de hauteur, crêtes vues de travers.

## Résultats de fonctionnement

Rejeux calculés une fois (258 s, échantillonnage de B réparti sur six fils) et relus depuis
`viewer/captures/s275/rejeu_<dt>.bin`, en-tête des constantes vérifié.

| critère | résultat |
|---|---|
| 1. précalcul | **7 500 pas à 4 ms et 1 875 à 16 ms, tous reçus**, 25 et 24 itérations au pire ; `η'` rms **15,9 mm**, max 98,7 et 105,8 mm (4,9 et 5,3 % de Hs) ; échantillons plans (essai) |
| 2. pas d'image contre 4 ms | hors éponge : **0,25 mm rms, 1,33 mm au maximum** (1,6 % de `η'` rms) ; sur tout le domaine 0,37 mm rms, **11,1 mm dans l'éponge amont** (x = −123 m, t = 30 s), que le fondu de rendu ramène sous 1 mm |
| 3. couche GPU contre CPU | 19 630 sondes dans la bande, deux poses et deux âges : **7,4·10⁻⁸ m** en hauteur, **1,2·10⁻⁸** en pente |
| 4. rien ne change hors `--delta` | sept images de la revue S254 identiques au bit (empreintes avant/après), `--verify` reçu ; essais de l'afficheur 21/21 |

Le critère 2 est une mesure : sous les 3 mm de hauteur d'image hors éponge, sans verdict visuel.
Le bord amont réagit au pas de temps dix fois plus que l'intérieur : limite de la frontière
(liste 4.7), non corrigée ici.

**Contrainte rencontrée** : un tampon de stockage de plus dépassait la limite de huit par étage ;
la bande δ suit donc les impacts dans le même tampon, sa position passée par `reflection.z`
(nulle hors `--delta`).
