# ADR-159 — La requête de jeu suit la surface rendue sous CWM

Actée S262, 2026-09-17, autonomie S71. Ferme A288. Complète ADR-157 (vagues pointues de Lagrange),
sans modifier les requêtes eulériennes linéaires existantes (ADR-113, ADR-065).

## Constat

Sous `--vagues`, un sommet rendu en `q` est placé en `x = q + D_B(q)`, à la hauteur `η_B(q) + W(q)`.
Une requête de jeu en `x` rendait `η_B(x) + W(x)`, jusqu'à 0,365 m plus haut ou plus bas que l'image
(S260). Un objet flottant posé par le jeu serait donc visiblement décalé de la vague.

## Décision

1. **Inversion.** `Background::cwm_query_local(x, t)` cherche `α` tel que `α + D_B(α) = x` par la
   méthode de Newton : `α ← α − J(α)⁻¹·(α + D_B(α) − x)`, avec `J = I + ∂D_B`, en partant de
   `α₀ = x − D_B(x)`. Elle rend `α`, l'élévation `η_B(α)`, la pente eulérienne `J⁻ᵀ·∇η_B(α)` et le
   nombre d'itérations. Mêmes phases entières que B (ADR-003), f32, sans allocation.
2. **Arrêt et refus.** Arrêt quand `|α + D_B(α) − x|` ≤ 0,1 mm. Refus explicite si la méthode ne
   converge pas en 8 itérations, si `det J ≤ 0` (repli : CWM hors de son domaine), si un résultat
   n'est pas fini, ou si le point est hors domaine. Aucun repli silencieux vers la requête linéaire.
3. **Composition.** Les couches W sont évaluées au point de Lagrange `α`, comme dans l'image
   (ADR-157 §3). Le consommateur appelle W en `α`, pas en `x`.
4. **Portée.** B entier, sans le filtre d'image d'ADR-148 : près de la caméra, ses poids valent 1.
   Au loin, l'image filtre des modes que la requête garde, et l'écart y est celui d'ADR-148. La
   queue ne porte que des pentes : elle n'entre pas dans les hauteurs.
5. **Réception** : [DEFAUTS-S262](../validation/DEFAUTS-S262.md) §3, écrit avant le code.
