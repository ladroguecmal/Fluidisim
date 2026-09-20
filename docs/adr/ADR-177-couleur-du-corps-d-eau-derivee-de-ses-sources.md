# ADR-177 — La couleur du corps d'eau se dérive de ses sources

Actée S307, 2026-09-20, autonomie technique S71. Réponse au verdict « le rendu est toujours
mauvais » et à la consigne d'aller au-delà du guide reçu. Preuve et mesures :
[RENDU-ECART-S307](../validation/RENDU-ECART-S307.md) §4.

## Constat

`water.wgsl` portait deux couleurs d'eau écrites à la main : `vec3(0.012, 0.105, 0.13)` par
défaut et `vec3(0.004, 0.060, 0.170)` en habillage « ciel clair ». **Aucune provenance.** I-14
l'interdit — « aucune valeur physique sans provenance » — et ni les audits, ni les six revues
visuelles, ni les deux lots de correction de la mer (S304, S306) ne l'avaient relevé, parce que
toutes les mesures du dépôt portaient sur la **surface** ou sur des **empreintes d'octets**, et
aucune sur ce que l'image montre.

Mesuré : le rapport bleu/vert attendu pour une eau océanique claire vaut **10,9** ; nos
constantes donnent **1,24** et **2,83**. Neuf fois et 3,8 fois trop vert. Les pixels les plus
sombres de l'image — ceux où l'on voit dans l'eau et non le ciel — rendent exactement le rapport
de la constante, ce qui confirme que c'est bien elle qu'on voit.

## Décision

**D1 — La teinte vient du calcul, pas du goût.** La couleur du corps d'eau est la réflectance
d'irradiance sous la surface

```
R(0⁻) ≈ 0,33 · b_b / (a + b_b)
```

avec `a(λ)` l'absorption de l'eau pure ([Pope & Fry 1997](https://omlc.org/spectra/water/data/pope97.txt),
jeu de données conservé dans la preuve) et `b_b = b/2`, `b(λ) = 0,0029·(550/λ)^4,30` m⁻¹
(Morel 1974). Aux trois bandes du rendu :

| λ | `a` (m⁻¹) | `b_b` (m⁻¹) | `R(0⁻)` |
|---|---:|---:|---:|
| 450 nm | 0,00922 | 0,00344 | 0,08960 |
| 550 nm | 0,0565 | 0,00145 | 0,00826 |
| 650 nm | 0,340 | 0,00071 | 0,00068 |

Toute couleur d'eau du rendu se dérive de ce calcul. **Une couleur d'eau écrite à la main est
désormais un défaut**, pas un réglage.

**D2 — Le gain est un paramètre déclaré, et ce n'est pas un choix de couleur.** `R(0⁻)` est une
réflectance : elle doit être multipliée par l'irradiance descendante du ciel pour donner une
radiance. Le dépôt n'a pas d'irradiance de ciel — son ciel est procédural (A299). Le facteur qui
met `R(0⁻)` à l'échelle de l'image est donc **explicitement** un substitut d'`E/π` (la forme
qu'emploie l'implémentation de référence de Bruneton et al.), exposé par `--eau-physique=<gain>`
et **balayé, jamais supposé**. Sa valeur est un arbitrage visuel : **R14**.

**D3 — Le défaut ne bouge pas avant le verdict.** `--eau-physique` est éteint par défaut : les
réceptions et empreintes antérieures restent valides **au bit**. Le verdict R14 fixera le gain
et fera de la couleur dérivée le défaut, par une note datée à cet ADR.

**D4 — Ce que cette décision ne prétend pas.** Pas d'irradiance de ciel réelle ; pas de
particules ni de CDOM (l'eau pure est le cas le **plus bleu** possible, un océan réel l'est
moins) ; pas de diffusion multiple ; pas de rendu spectral — trois bandes, échantillonnées à 450,
550 et 650 nm, ce qui est une approximation déclarée et non une intégration colorimétrique.

## Ce que cette décision a coûté à établir, et qui vaut d'être consigné

La première tentative fut **fausse**, et l'image l'a montrée quand aucun chiffre ne l'avait dit :
j'avais remis `R(0⁻)` à la **luminance** de la constante historique, pour ne changer que la
teinte. Le bleu valait alors **0,40 de réflectance** — quatre fois le maximum physique — et le
rendu virait à l'outremer. La renormalisation compensait, sans le dire, l'irradiance de ciel
absente. C'est ce qui a produit D2 : le facteur d'échelle **devait** être nommé pour ce qu'il est.

## Conséquences

- I-14 est satisfaite pour cette grandeur, et elle ne l'était pas.
- **A299 reste ouverte et gagne en poids** : tant que le ciel est procédural et l'exposition
  absente, le gain de D2 absorbe leur absence. Une irradiance de ciel réelle le rendrait inutile.
- Les couleurs historiques restent dans le code comme **témoin**, pas comme alternative.
- Ne remplace ni ADR-161 (reflets filtrés) ni ADR-162 (ciel précalculé) ; les complète.

## Ce qu'il faudrait pour l'inverser

Une mesure montrant que la réflectance dérivée s'écarte de l'observation davantage que la
constante — par exemple une comparaison à une photographie **dont on connaît** vent, exposition
et focale. Le dépôt n'en a pas ; c'est une limite, pas un argument.
