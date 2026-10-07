# ADR-273 — Quarante et unième revue de méthode (S681–S685)

- **Statut : actée**, S686, 2026-10-08 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-272](ADR-272-quarantieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S681 | la revue (ADR-272) | — | — |
| S682 | l'assertion du volume comparait au quantum de `dx` en `f64`, le code le tirait de `dx` en `f32` | une relance de quelques secondes | — |
| S683 | 126 particules posées pour 128 attendues : la hauteur mouillée lue aux étiquettes, un peu plus basse | — | rapporté |
| S684 | **le montage partait de deux niveaux** : Saint-Venant à 0,31 m, la 3D à 0,30 m (le réseau des particules). Le plan avait nommé le symptôme (« des niveaux mal accordés font couler un flux ») | une relance de 4 min | **D2** |
| S685 | **l'instrument ne départageait pas le raccord** : le relais était jugé contre le tout-Saint-Venant, qui diffère aussi par l'onde qu'il porte au large. Le calcul du plan n'avait tiré l'écart attendu que de la crête (4 %) ; la forme et la vitesse de l'onde d'APIC diffèrent bien davantage (−15 % sur la remontée). Le critère a manqué pour une autre raison que le raccord | une session et 15 min de calcul | **D1** |

## 2. Décisions

**D1 — Un raccord entre deux solveurs se juge d'abord entre deux copies du même solveur.** Le même code de raccord, le même solveur des
deux côtés : il doit redonner le domaine entier. C'est le témoin qui isole l'interface (ADR-259 D1). Seulement ensuite, le raccord entre
solveurs différents se juge contre une référence de chaque côté :

- celle qui porte l'onde au large ;
- celle qui la reçoit.

Un écart entre deux solveurs mêle le raccord et ce que chacun porte : ADR-267 D1 demande que l'instrument départage, et celui-là ne le
peut pas.

**D2 — Les deux côtés d'un montage couplé partent d'une seule source.** L'un lit l'autre (Saint-Venant part du niveau des particules),
au lieu du même nombre écrit deux fois. Deux discrétisations d'un même état ne coïncident pas : le réseau des particules, la
reconstruction, la maille.

## 3. La prochaine revue

S691.
