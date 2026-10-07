# ADR-253 — Vingt-sixième revue de méthode (S606–S610)

- **Statut : actée**, S611, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-252](ADR-252-vingt-cinquieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une phrase du plan qui dépendait d'un nombre calculé, sans assertion** : S608 écrivait « la cellule de B » quand le script en comptait deux ; S610 écrivait « réallouer si 86 > 86 » quand le déplacement choisi gardait 86 blocs. Les deux attrapées en lisant la sortie du script (ADR-251 D1), non par le script lui-même | deux relances du plan | **protection élargie** (D1) |
| Un seuil écrit avant sa référence (S609 : le reste d'une bosse « sous 1 % ») — le script l'a refusé (1,3 %) ; la référence a révélé un bord mal centré dans le temps (4,3 mm d'écart à B, ramené à 0,63) ; le seuil porté à 2 % **en le disant** au plan, avant la mesure du code | une relance, un défaut trouvé | rien à changer : le seuil fixé sur la référence s'écrit avec son origine (déjà fait) |
| La taille de la suite mesurée à chaque preuve depuis S607 (ADR-252 D1) | — | **rien à changer** : tenue |
| Les implémentations indépendantes du plan (numpy, rationnels, décimal) : S607–S610 rejoignent le code au bit ou à 10⁻¹⁴ | — | rien à changer |

## 2. Décision

**D1 — Une phrase du plan qui dépend d'un nombre calculé est assertée par le script qui l'écrit** : un compte (« deux cellules »), un
ordre (« 89 > 86 »), une appartenance (« dans la borne »). Le script refuse d'écrire un plan qui se contredit (élargit ADR-249 D1, qui
l'imposait pour le seul rapport au quantum, et ADR-251 D1, la relecture).

## 3. La prochaine revue

S616.
