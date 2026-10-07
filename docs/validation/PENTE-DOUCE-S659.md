# 2.7, la bathymétrie 2D : le modèle parabolique de pente douce — S659

*S659, 2026-10-07, en autonomie, vers la v2.* Le point 2.7 attend la bathymétrie 2D et la **diffraction** derrière un haut-fond isolé,
là où les rayons (S583) font des caustiques. Première pièce : le modèle parabolique de pente douce (Radder 1979).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s659 -- --nocapture` (≈ 5 s).

## 1. Ce qui est construit

`pente_douce.rs` est un outil de cuisson (ADR-260 : O). Il porte :

- `propager(h, T, g, x0, x1, dx, y0, y1, dy)` : l'amplitude complexe d'une houle monochromatique d'incidence normale, marchée en `x` ;
- `Champ::amplitude` : le rapport d'amplitude au point ;
- `nombre_d_onde` ;
- `berkhoff` : la profondeur du bassin de Berkhoff.

L'équation, dérivée au plan :

`A_x = −(p·k̄)_x/(2p·k̄)·A + i/(2p·k̄)·[(p·A_y)_y + p·(k² − k̄²)·A]`, avec `p = C·C_g`.

Le schéma est Crank–Nicolson en `x`, tridiagonal complexe en `y`, avec des parois latérales réfléchissantes.

## 2. Mesuré

**Les deux cas analytiques** (l'instrument éprouvé, ADR-263 D2) :

| | référence | mesuré |
|---|---|---|
| (1) onde plane sur fond plat, 20 m | `|A|` = 1 | à **2·10⁻¹⁴** |
| (2) levée à incidence normale, 0,45 → 0,15 m sur 1:50 | 0,990551 (`√(C_g0/C_g)`, au plan) | **0,990551** |

**Le haut-fond de Berkhoff, Booy et Radder (1982)**, contre les mesures. Ce sont les rapports d'amplitude de l'exemple public de Basilisk,
`section-2, -3, -5, -7` ; l'axe `y` y est inversé.

| section | écart quadratique moyen (5 cm ; 2,5 cm) | pic mesuré | pic du modèle |
|---|---|---|---|
| 2 (x = 3 m) | 0,230 ; 0,231 | 1,462 | 1,912 |
| 3 (x = 5 m) | 0,197 ; 0,197 | 2,207 | 2,460 (+11 %) |
| 5 (x = 9 m) | 0,418 ; 0,419 | 1,838 | 1,747 |
| 7 (y = 0) | 0,287 ; 0,288 | 2,077 | 2,258 |

**Critères, écrits avant.**

- (1), (2) : **tenus**.
- (3) **Manqué.** L'écart dépasse 0,20 sur trois sections. Seul le pic de la section 3 tient, à 11 % pour une borne de 15 %.

## 3. L'écart, localisé (ADR-259 D1)

- **La maille ? Non** : les deux mailles donnent le même écart, au millième.
- **L'axe inversé des mesures ? Non.** Lues sans l'inversion, les mesures s'écartent davantage (0,351 ; 0,543 ; 0,563) : l'inversion est
  juste. Un essai le garde.
- **Restent nommées, non départagées** :
  - **l'approximation aux petits angles** : la pente est tournée de 20°, et les ondes diffractées partent en biais. Le modèle surestime la
    focalisation juste derrière le haut-fond (section 2) et la place mal plus loin (section 5) ;
  - **la non-linéarité de l'expérience**, qui étale la focalisation et que le modèle linéaire ne rend pas.

**Le témoin suivant** (S660) : le modèle à grand angle (Kirby 1986). Si l'écart des sections 2 et 5 tombe, c'était l'angle.

## 4. Ce que cela dit

Le cœur a désormais une houle qui **se réfracte et se diffracte** sur une bathymétrie 2D quelconque, exacte sur ses deux cas analytiques.
Derrière le haut-fond de Berkhoff, il rend la focalisation : un pic de 2,46 pour 2,21 mesuré, au bon endroit sur la section 3. Ailleurs,
il s'écarte des mesures de 0,2 à 0,4 en rapport d'amplitude.

Manquent : l'écart aux mesures réduit (le grand angle, S660), la diffraction couplée à B, la marée, la dissipation au déferlement.
