# ADR-249 — Vingt-deuxième revue de méthode (S586–S590)

- **Statut : actée**, S591, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-248](ADR-248-vingt-et-unieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un seuil sous son bruit, imprimé par le script du plan et non lu** (S589 : « rapport au bruit : 1 » écrit au plan ; S582 : le quantum d'un pic échantillonné, au-dessus du seuil). Deux fois relevé **avant** la mesure, la procédure corrigée et le seuil gardé — mais après le commit du plan | deux corrections de plan, sans fausse mesure | **un outil** (D1) : le script calcule le rapport, il doit aussi le refuser |
| Un paramètre que la loi reçoit en entier, donné décimal au plan (S590 : l'évaporation, 57,87 nm/s pour une loi en nm/s entiers) | la référence recalculée dans l'essai | **protection élargie** (D2) |
| Les références des plans par des formules indépendantes du code (S587 : Rayleigh intégré ; S588 : dispersion, levée, réfraction réécrites ; S590 : RK4 du lac) | — | **rien à changer** |

## 2. Décisions

**D1 — Le script du plan refuse un seuil sous son quantum** : chaque critère y porte son seuil et son quantum, et le script s'arrête
(`assert`) si leur rapport est sous 10 — avant d'écrire le plan (ADR-236 D1, rendu exécutoire ; ADR-243 D1 : le script écrit déjà les
nombres).

**D2 — Les paramètres d'un montage s'écrivent au plan dans les unités et le type de la loi qui les reçoit** (lus dans le code avant
d'écrire) : une valeur que la loi arrondit change la référence (élargit ADR-237 D1).

## 3. La prochaine revue

S596.
