# ADR-226 — Troisième revue de méthode (S492–S495)

- **Statut : actée**, S496, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; les
  précédentes, [ADR-223](ADR-223-premiere-revue-de-methode.md) et [ADR-224](ADR-224-deuxieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| Un premier remède d'A327 posé dans la mesure elle-même changeait six essais au bit (S492) | un essai de la suite | vu par les essais au bit ; **rien à ajouter** |
| Le couvercle partiel sur la carte changeait les bits du couvercle plein : le compilateur réordonnait le chemin commun (S493) | une reprise | la protection du compilateur dans la boucle (L345) a joué ; **rien à ajouter** |
| **Trois remèdes essayés avant de localiser l'écart** : recette plus fine, bouée éloignée, coupure plus haute — chacun un essai de 8 min —, alors que le corps, le champ et l'état de départ se séparaient en un essai de 20 s sans pas du corps (S495) | ≈ 30 min et trois explications fausses | **protection nouvelle** (D1) |
| **Une bouée lâchée au repos dans une eau qui bouge** : le retard `u₀·t` pris pour un défaut — deux fois, sous la houle de B (S494) puis sous le sillage (S495) | un manqué, puis deux | **protection nouvelle** (D2) — une erreur répétée |
| **Un ordre de grandeur incomplet** : le second ordre jugé sur `k·a` alors qu'il s'accumule en `k·a·ω·t` (S494) ; la queue d'une source dite négligeable contre la source, pas contre le terme qu'elle concurrence — la pente du sillage, de même origine (S495) | deux manqués | **protection nouvelle** (D3) — la même faute deux fois |
| Une bouée haute et étroite, presque sans stabilité de forme avec quatre points par axe, a chaviré (S494) puis roulé (S495) | deux essais | l'instrument s'éprouve sur un cas connu (L360) ; **rien à ajouter** — mais une bouée d'essai est plate |
| Un saut de ligne d'une chaîne Rust mangé par Python (`\` en fin de ligne dans un texte entre triples guillemets, S494) | une reprise | le piège des échappements est dans la boussole ; **rien à ajouter** |
| Un essai de 8 min dans la suite du cœur | +8 min par suite | **fait** : `#[ignore]` avec sa raison, comme les autres essais longs |

## 2. Décisions

**D1 — Un écart à une référence se localise avant tout remède.** Séparer les chaînes — le modèle contre sa propre équation (le corps
contre `∫∫F/m`), la référence contre la sienne (le champ contre `∂u/∂t = −g∇η`), l'état de départ —, par le montage **le moins cher**
qui les isole ; un remède ne s'essaie qu'une fois la chaîne fautive nommée.

**D2 — Deux trajectoires se comparent depuis le même état** : positions **et** vitesses. Un corps lâché dans une eau qui bouge part à la
vitesse de cette eau, toutes ses parts composées.

**D3 — Un ordre de grandeur se compare au terme concurrent, sur la durée de la mesure** : un effet qui s'accumule se compte à la fin de la
mesure (`k·a·ω·t`, pas `k·a`) ; « négligeable » se dit contre ce qu'il concurrence, pas contre ce qui le produit — deux termes nés de la
même source restent dans le même rapport quand elle change.

## 3. La prochaine revue

S501.
