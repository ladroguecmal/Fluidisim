# ADR-248 — Vingt et unième revue de méthode (S581–S585)

- **Statut : actée**, S586, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-246](ADR-246-vingtieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Deux trajectoires comparées qui n'étaient pas de la même famille** (S583 : le rayon voisin lancé avec un autre invariant de Snell ; `K_r` 0,981 pour 0,935). Un tracé indépendant a montré le module juste. ADR-232 D1 le dit pour un corps d'essai (« il n'a que les degrés de liberté que la référence décrit ») ; ici, une **paire** de trajectoires | une mesure refaite | **protection élargie** (D1) |
| **Un critère vrai par construction** (S584 : le flux `A²·√h·b`, que la formule implémentée conserve d'office). Écrit tel quel dans la preuve ; c'est le critère contre la forme fermée qui jugeait | aucun | **protection élargie** (D2) |
| Un seuil sous son quantum relevé **avant** la mesure et écrit aux notes (S582, ADR-236 D1) | — | a tenu ; **rien à changer** |
| La décision de l'utilisateur (S585, ADR-247) : une question posée parce que la portée de la v2 n'était écrite nulle part | — | **rien à changer** : une portée se demande |

## 2. Décisions

**D1 — Une paire de trajectoires comparée à une référence appartient à la famille que la référence décrit** (le même invariant, le même
front, la même source) : le montage la construit ainsi, et le plan le dit (élargit ADR-232 D1).

**D2 — Un critère qui découle de la formule implémentée ne juge rien** : le plan le repère et le remplace par une comparaison à une
référence indépendante, ou le marque « vérification d'assemblage » (élargit L378).

## 3. La prochaine revue

S591.
