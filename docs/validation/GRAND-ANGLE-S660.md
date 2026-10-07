# Le témoin du grand angle sur le haut-fond de Berkhoff — S660 (liste 2.7)

*S660, 2026-10-07, en autonomie, vers la v2.* En S659, le modèle parabolique aux petits angles s'écartait des mesures de Berkhoff de 0,2 à
0,4 en rapport d'amplitude. La maille et l'axe des mesures avaient été écartés. Deux causes restaient nommées : l'angle, la
non-linéarité.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s660 -- --nocapture` (≈ 1 s).

## 1. Ce qui est construit

`propager_grand_angle` (dans `pente_douce.rs`, à côté de `propager`) repose sur l'approximation de Padé [1,1] de la racine de propagation
(Booij 1981, Kirby 1986) :

`(1 + X/4)·(A_x − lev·A) = (i·k̄/2)·X·A`, avec `X = [(k² − k̄²) + (1/p)·∂_y(p·∂_y)]/k̄²`.

Le schéma est Crank–Nicolson, avec les coefficients au demi-pas. L'amplitude incidente est complexe et quelconque.

## 2. Mesuré

**Les cas de l'instrument :**

| | référence | grand angle |
|---|---|---|
| le plat | `|A|` = 1 | 4·10⁻¹⁴ |
| la levée 0,45 → 0,15 m | 0,990551 | 0,990551 |
| l'onde oblique à 30°, `k_x` lu sur la phase | `k·cos 30°` = 3,64638 m⁻¹ | 3,64972 (**+0,091 %** ; les petits angles : +1,036 %) |

**Une erreur de mon plan, vue avant la mesure.** Le plan disait de juger l'onde oblique sur `|A|` = 1. Or une onde plane garde `|A|` = 1
dans les deux modèles, si bien que ce lecteur ne départage rien. C'est la phase qui départage, et elle est rapportée (ADR-263 D2,
ADR-266).

**Berkhoff** (écart quadratique moyen au rapport d'amplitude mesuré ; à 5 et 2,5 cm, au millième près) :

| section | petits angles (S659) | grand angle | baisse |
|---|---|---|---|
| 2 (x = 3 m) | 0,231 | **0,171** | −26 % |
| 3 (x = 5 m) | 0,197 | **0,161** | −18 % |
| 5 (x = 9 m) | 0,419 | 0,342 | −18 % |
| 7 (y = 0) | 0,288 | 0,233 | −19 % |
| pic de la section 3 (2,207 mesuré) | 2,460 | 2,425 (+10 %) | |

**Critères, écrits avant.**

- **(1) Tenu.**
- **(2) Le verdict du témoin.** La baisse des sections 2 et 5 (26 % et 18 %) n'atteint pas les 30 % fixés : **l'angle n'est qu'une part
  de l'écart**.
- **(3) Le critère de S659, rejugé.** Il tient sur les sections 2 et 3, ainsi que sur le pic. Il est **manqué** sur les sections 5 (0,342)
  et 7 (0,233).

## 3. Ce que cela dit

Le grand angle est le bon modèle : il ne coûte pas plus, il est juste à 0,1 % jusqu'à 30°, et il réduit l'écart de chaque section d'un
cinquième environ. Ce qui reste concerne surtout la section la plus lointaine (x = 9 m) et l'axe de focalisation.

La cause encore nommée est la **non-linéarité de l'expérience**. Une onde plus raide va plus vite (la dispersion d'amplitude), ce qui
étale la focalisation. Le témoin suivant : la dispersion d'amplitude de Kirby et Dalrymple (1984) dans le grand angle.

Manquent : l'écart des sections 5 et 7 (la non-linéarité), la diffraction couplée à B, la marée, la dissipation au déferlement.
