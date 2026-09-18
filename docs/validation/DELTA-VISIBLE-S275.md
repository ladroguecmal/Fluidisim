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
