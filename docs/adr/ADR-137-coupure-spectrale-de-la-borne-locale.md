# ADR-137 — Coupure spectrale de la borne locale de pente

- Statut : **actée**, S221, 2026-09-13, autonomie technique S71.
- Prolonge [ADR-136](ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md) et
  [ADR-134](ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) sans les remplacer :
  ADR-135 et ADR-136 restent publiées **au bit**.
- Traite **A260** ; vise le plateau **A259** à faible travail (≤ 8191 évaluations).

## Problème

ADR-135 et ADR-136 bornent la variation de chaque mode **isolément** : `2 c_k` pour un mode exclu
de la Hessienne (`D_k ≥ 2`), `c_k (E_k + D_k²/2)` pour un mode inclus, presque autant près de
`D = 2`. Sur une feuille 2 × 1,5 m, 1608 modes sur 4096 sont exclus et le reste total pèse 74 à
87 % de la borne globale (S220) : la borne locale ne peut pas descendre sous la globale.

La borne globale ADR-134 fait mieux pour ces mêmes modes, parce qu'elle les traite
**ensemble** : `G ≤ C`, et `G ≈ 0,85 C` sur les sillages mesurés.

## Décision

**1. Couper le spectre, pas le choisir mode par mode.** Pour toute partition des modes en
résolus `R` et non résolus `U`, en tout point `p` du rectangle :

```
|S(p)| ≤ |S_R(p)| + |S_U(p)|,
|S_U(p)| ≤ G(U) = √(C_U (C_U + R_U) / 2),   C_U = Σ_U c_k,   R_U = |Σ_U c_k e^{2iθ_k}|,
|S_R(p)| ≤ max_coins |S_R(c) + M_R u| + Σ_R reste_k          (ADR-136 restreint à R).
```

La première ligne est l'inégalité triangulaire. La deuxième est la preuve d'ADR-134 appliquée à
un sous-ensemble : elle ne dépend ni du point ni des phases. La troisième est ADR-136 sur les
modes de `R`, avec son choix par mode entre terme de Taylor et variation.

**2. Une famille de coupures en une passe.** Classes de largeur de phase
`D < ½`, `½ ≤ D < 1`, `1 ≤ D < 2`, `D ≥ 2`. Dans la passe O(N) d'ADR-136, chaque classe accumule
sa pente au centre, sa Hessienne, son reste, sa masse linéaire, sa masse `C` et ses moments en
`2θ`. Les coupures `U = {D ≥ D*}`, pour `D* ∈ {2, 1, ½}`, s'obtiennent en fin de passe par
sommes de classes, avec trois évaluations aux coins. La borne publiée est le minimum d'ADR-135,
d'ADR-136 et des trois coupures.

**3. La coupure `D* = 2` ne perd jamais sur les exclus.** Ils paient `2 c_k` dans ADR-136, et
leur valeur au centre y figure dans `S(c)`. Or `|S(c) + M u| ≥ |S_R(c) + M u| − |S_U(c)|`, et
`G(U) + |S_U(c)| ≤ 2 C_U`, puisque chaque terme est au plus `C_U`. Les coupures `1` et `½`
retirent aussi des modes inclus. Elles ne sont pas garanties, et c'est le minimum qui les rend
sûres.

**4. Réserve.** `(C + Σ_R c_k (D_k + E_k)) · (8γ_(N+64) + 32ε)`, soit la réserve d'ADR-136 restreinte
à la masse linéaire de `R`. `G(U)` est calculé avec arrondi vers le haut à chaque opération.
L'écart des polynômes trigonométriques sur `|(sin, cos)|` est couvert par le terme en `32ε·C`.
Même statut qu'ADR-135 §4 : **pas un certificat f32** (A258).

**5. Exposer sans migrer.** `Field::local_slope_envelope_spectral` publie la borne, l'annonce
d'ordre deux **au bit**, et par coupure la borne, `C_U` et `G(U)`. La partition reçoit
`SlopeOrder::Spectral`. L'ordre deux conserve son code machine : la passe est générique sur une
constante, et les accumulateurs de classes disparaissent de sa version. Aucune admission ne
change.

## Limite annoncée avant mesure

**`G(U)` ne dépend pas du point.** Une grande cellule loin du sillage garde au moins `G(U)` dès
que `U` porte l'essentiel de la masse. La localisation spatiale d'un paquet d'ondes vit dans la
**cohérence des phases** entre modes voisins, et aucune enveloppe de modules ne la voit, qu'elle
soit par mode ou par sous-ensemble. Cette décision peut donc abaisser le plateau des mailles
moyennes, pas celui des très grandes. Voir l'ouverture ci-dessous.

## Pourquoi pas autre chose

- **Seuil sur `|k|` avec tri et sommes préfixes** : la largeur de phase dépend de l'orientation
  du rectangle (`|t_x| h_x + |t_y| h_y`). Un tri par `|k|` exigerait une mémoire de l'appelant
  pour un gain en `O(log N)` qui ne sert à rien, puisque la passe est déjà en `O(N)`.
- **Optimiser la coupure mode par mode** : l'enveloppe d'un sous-ensemble n'est pas additive.
  Choisir `U` exactement demande une recherche combinatoire. Trois seuils imbriqués coûtent une
  douzaine d'additions par mode.

## Réception requise

Sondes sous la borne sur des rectangles grossiers à spectre étalé. Domination
`borne ≤ borne ADR-136`, et ADR-136 recalculé au bit. Gain strict d'au moins une coupure sur un
rectangle où les modes courts dominent la masse. S220 inchangée au bit. Refus identiques.
Décomposition `C_U` / `G(U)` publiée par classe et par taille de maille, sur les fixtures S217–S220,
**avant** toute lecture du gain de partition.

## Ce qui reste ouvert

Localisation spatiale aux très grandes mailles, par exemple une borne de champ lointain par
sommation par parties sur la grille polaire : non dérivée, et à instruire seulement si le
plateau persiste après cette décision. A258, borne d'erreur courante. Coût et validité
temporelle. A254, migration d'admission, loi GPU.
