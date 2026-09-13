# ADR-136 — Borne locale de pente d'ordre deux, à Hessienne signée

- Statut : **actée**, S220, 2026-09-13, autonomie technique S71.
- Prolonge [ADR-135](ADR-135-borne-locale-de-pente-du-champ-prepare.md) sans le remplacer :
  sa branche d'ordre un reste publiée, inchangée au bit, et sert de repli.
- Répond à l'obstacle **A259** (S219) : aux grandes et moyennes mailles, la borne ne
  hiérarchise pas les régions, et aucun ordonnancement ne crée cette information.

## Problème

ADR-135 majore la variation de pente d'un rectangle par `Σ c_k min(2, D_k)`, avec
`c_k = |w_k|·|η_k|` et `D_k` la largeur de phase exécutée du mode sur le rectangle. C'est une
somme de **modules** : deux modes dont les variations se compensent y paient chacun le
maximum. L'excès est d'ordre un en taille de maille partout, **y compris au maximum de la
pente**, où la vraie variation de `|S|` est d'ordre deux. La partition S219 doit donc
descendre à des mailles de taille `O(ε)` pour gagner `ε` sur la borne.

## Décision

**1. Développer chaque mode à l'ordre deux autour de la phase exécutée au centre.**
La contribution d'un mode à la pente est `−w_k g_k`, `g_k = A s + B c = a sin(θ+β)` avec
`(A, B)` la réponse `η` et `(s, c)` le couple trigonométrique de la phase. Pour un écart de
phase exécuté `δ` :

```
g(θc + δ) = g(θc) + g'(θc)·δ + R,   |R| ≤ a·min(δ²/2, 2 + |δ|),   g'(θc) = A c − B s = η_k(c).
```

La dérivée est **l'élévation du mode au centre**, déjà calculée par l'échantillonnage.

**2. Assembler une Hessienne signée et prendre son maximum aux coins.**
Avec `u = p − c` et le modèle continu `δ_lin = 2π t_k·u` (`t_k` les tours par mètre stockés) :

```
S(p) = S(c) + M u + ρ,    M[i][j] = −Σ_k w_k[i] · η_k(c) · 2π t_k[j].
```

`M` somme des termes **signés** : les compensations entre modes y sont conservées.
`u ↦ |S(c) + M u|` est convexe ; son maximum sur le rectangle est atteint à l'un des quatre
coins. Le coût est `O(N)` pour `M` et `O(1)` pour les coins.

**3. Payer l'écart entre phase exécutée et phase linéaire.** `δ − δ_lin = e_k` provient de
l'arrondi des produits `t·x` (au plus `ε/2·|t|·(|p|+|c|)` par axe), des fractions et de Q32
(`8ε` tours, comme ADR-135 §3) et de l'arrondi des coins `fl(min − c)` (au plus
`ε/2·|t|·|min − c|`). Majorant retenu, en tours, arrondi vers le haut :

```
E_k = Σ_axes |t_k| · ε · (max(|min|, |max|) + |c|)  +  8ε,     puis ×2π.
```

À 4000 m et un tour par mètre, `E_k ≈ 3·10⁻³ rad` : **la phase quantifiée n'est pas un
détail** aux grandes coordonnées, elle y domine le reste d'ordre deux des petites mailles.

**4. Choisir mode par mode.** La décomposition `S(p) − S(c) = Σ_inclus (terme linéaire + reste)
+ Σ_exclus (variation)` est valide pour tout sous-ensemble. Un mode entre dans `M` si
`E_k + D_k²/2 < min(2, D_k)` ; il paie alors `c_k (E_k + D_k²/2)`, sinon `c_k min(2, D_k)`
comme ADR-135. `D_k` est la largeur d'ADR-135, qui majore aussi `|δ|` non replié.

**5. Retenir le minimum des branches, jamais pire qu'ADR-135.**

```
borne₂ = max_coins |S(c) + M u| + Σ_k reste_k + réserve₂
borne  = min(borne ADR-135, borne₂)
```

La branche ADR-135 (globale incluse) est calculée dans la même passe, **au bit** de
`local_slope_envelope`. `réserve₂ = (C + Σ_inclus c_k (D_k + E_k)) · (8γ_(N+64) + 32ε)` couvre
l'accumulation de `M`, les coins et l'écart trigonométrique de `phase.rs` sur `η_k(c)`.
Même statut qu'ADR-135 §4 : **précaution de réception, pas certificat f32** (A258).

**6. Exposer sans migrer.** `Field::local_slope_envelope_second_order` et son exposition liée
au contexte/instant dans `Prepared` ; la partition S219 reçoit l'ordre en paramètre, l'appel
existant reste l'ordre un au bit. Aucune admission ne change.

## Pourquoi pas autre chose

- **Ordonner autrement le tas** : A259 montre que les valeurs sont égales, pas mal classées.
- **Intervalles sur chaque mode** (`sin` sur un intervalle de phase) : resserre chaque terme,
  mais reste une somme de modules ; les compensations, qui sont le défaut, restent perdues.
- **Taylor d'ordre trois** : un tenseur `2×2×2` en `O(N)`, maximum non convexe sur le
  rectangle. Non exclu ; à instruire seulement si l'ordre deux plafonne à son tour.

## Réception requise

Sondes sous la borne : pic manqué au centre, multidirectionnel, translation près de 4000 m
avec phase quantifiée. Domination `borne ≤ borne ADR-135` sur tout rectangle ; branche
ADR-135 au bit de `local_slope_envelope`. Gain strict sur un rectangle autour d'un maximum
de pente. Partition S219 à l'ordre un inchangée au bit. Refus identiques à ADR-135.
Campagne S219 transposée, chaque ordre avec son coût complet et l'en-tête ADR-131.

## Ce qui reste ouvert

Gain de partition non promis ; migration d'admission non décidée ; A258 avant toute
migration ; somme sur les sources A254, loi GPU et mutualisation. Seconde cible non reçue.
