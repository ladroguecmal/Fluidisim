# ADR-154 — Prolonger le fond au-dessus du plan moyen : vitesse horizontale constante, par mode

Actée S254, 2026-09-16, autonomie S71. Répond à A286. Complète ADR-152 (surface mobile couplée),
qui exige un fond **incompressible** prolongé au-dessus du plan moyen, avec `S` calculé sur ses
propres champs. Ne modifie pas ADR-113 : `differential_local` continue de refuser `z > 0`.

## Constat

Sous une crête, les faces d'un domaine δ couplé sont au-dessus du plan moyen. Trois prolongements
étaient sur la table ; aucun ne convient tel quel.

- **Analytique** (`e^{kz}`, ou `cosh k(z+h)` en profondeur finie) : incompressible et irrotationnel,
  employé par l'oracle S253 pour un seul mode. Dans une mer large bande, il multiplie une
  composante courte par `e^{kz}` sous la crête d'une longue : ×15 pour `λ` = 3,5 m sous 1,5 m de
  crête, la bande de B (S201). C'est la surestimation connue de l'extrapolation linéaire.
- **Taylor d'ordre un de toutes les grandeurs** : pas incompressible, `div = z·∂x∂zU` (S253).
- **Étirement** (Wheeler, delta) : il dépend de `η` instantanée, donc de toute la mer au point.
  Il n'est pas incompressible mode par mode et ne se somme pas linéairement.

## Décision

1. **Règle.** Au-dessus du plan moyen, pour chaque mode : **vitesse horizontale constante**, égale
   à sa valeur en `z = 0` ; **vitesse verticale fermée par continuité**, `W(z) = W(0) − z·(∂xU + ∂yV)(0)` ;
   **pression de Taylor d'ordre un**, `P(z) = P(0) + z·∂zP(0)`. La règle est linéaire, donc la
   somme des modes la vérifie.
2. **Pour une composante de B** (phase `θ`, `A = aω`, formules d'ADR-113 à `E = 1`), avec
   `m = 1 + kz` :

   ```text
   u      = (A sinθ d_x, A sinθ d_y, −A m cosθ)
   du_dt  = (−Aω cosθ d_x, −Aω cosθ d_y, −Aω m sinθ)
   grad_u[i][j] = Ak cosθ d_i d_j ; grad_u[i][z] = 0          (i, j horizontaux)
   grad_u[z][j] = Ak m sinθ d_j   ; grad_u[z][z] = −Ak cosθ
   p_dyn  = ρga m sinθ ; grad_p_dyn = (ρgak m cosθ d_x, ρgak m cosθ d_y, ρgak sinθ)
   laplacian_u = −k²|d|²·u
   ```

   `eta` et `grad_eta` sont inchangés. La divergence vaut `Ak cosθ (|d|² − 1)`, comme sous le plan
   moyen. Toutes les dérivées sont celles de ce champ, pas celles du champ analytique.
3. **Propriétés.** La vitesse ne croît pas avec `z`. `W` et `P` croissent linéairement, sans
   exponentielle. Les valeurs sont continues en `z = 0` ; `∂zU` saute de `Ak sinθ d` à zéro. Ce
   pli, δ le porte. Le champ est rotationnel au-dessus du plan moyen. Le résidu linéaire vaut
   `(gk·m − ω²)·a cosθ d` à l'horizontale et `(gk − ω² m)·a sinθ` à la verticale. Sous `ω² = gk`,
   il est `O(gak²z)`, donc d'ordre deux en amplitude, comme la part que δ reconstruit.
4. **Interface.** Nouvelle entrée `Background::differential_local_extended` (ponctuelle, locale),
   plus ses variantes monde et lot. Pour `z ≤ 0`, elle rend au bit `differential_local`. Pour
   `z > 0`, elle rend la règle, sur le même domaine horizontal et le même `|z| < 4096`. Les refus
   restent ceux d'ADR-113. Au-dessus de la surface, les valeurs sont finies et ne représentent
   rien de physique : le pas couplé ne lit que les faces mouillées et la rangée voisine.
5. **Réception** : [PROLONGEMENT-FOND-S254](../validation/PROLONGEMENT-FOND-S254.md), écrit avant le
   code. L'oracle S253 se rejoue avec cette règle à la place du prolongement analytique ; ses
   tolérances ne bougent pas.

## Hors de cette décision

- Les couches **W** (impacts radiaux, pression) au-dessus du plan moyen : même règle, mais leurs
  dérivées secondes à `z = 0` restent à écrire et à recevoir.
- Les frontières du total (`W(fond) ≠ 0`), les bords ouverts et la relaxation de `η'`.
- L'**exactitude** du prolongement : c'est une convention de partage entre fond et perturbation, pas
  un modèle de la cinématique des crêtes. Seule la réception du total contre un oracle la juge.
