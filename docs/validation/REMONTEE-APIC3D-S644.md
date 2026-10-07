# Le rouleau 3D, étape 2 : l'onde solitaire qui monte la pente dans APIC 3D — S644 (liste 4.14)

*S644, 2026-10-07, en autonomie* (la campagne du rouleau 3D, acceptée le 2026-10-07 ; « continue en autonomie jusqu'à la v2 »). Une
vague non déferlante monte une pente de 1:3 dans APIC 3D, fond en escalier de S639. Elle est jugée contre la loi de Synolakis (1987) et
contre Saint-Venant 2D (S613, à l'ordre deux de S620) sur la même plage.

## Reproduire

- 5 cm : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core a_solitary_wave_runs_up_the_slope_in_apic3d_s644 -- --nocapture`
  (≈ 1 min).
- 2,5 cm : `… a_solitary_wave_runs_up_the_slope_in_apic3d_fine_s644 -- --ignored --nocapture` (≈ 5 min).
- Le témoin du fond lisse : `… the_smooth_bed_witness_of_the_runup_s644 -- --ignored --nocapture` (≈ 6 min).
- Les essais d'APIC 3D : 50 passent, 3 longs ignorés.

## 1. Le montage (références au plan, `s644_plan.py`)

| grandeur | valeur |
|---|---|
| canal | 6,6 × 0,1 m × 0,8 m (quatre mailles de large) |
| fond | plat à 0,05 m, puis pente 1:3 depuis le pied (4,768 m) |
| niveau au repos | 0,40 m (`d` = 0,35 m) |
| onde solitaire | `H` = 0,07 m (`H/d` = 0,2), centrée à 2,80 m, la distance canonique `arccosh(√20)/γ` = 1,97 m en avant du pied |
| vitesse initiale | `c·η/(d + η)`, la même sur toute la colonne |
| non déferlante | `H/d` < 0,818·cot^(−10/9) = 0,241 (asserté) |
| Synolakis | **R = 0,2295 m** |

Les bornes sont assertées : la queue de l'onde au mur gauche, la remontée sous le couvercle, aucune égalité entre un centre de maille et le
fond.

## 2. Mesuré

| | 5 cm | 2,5 cm |
|---|---|---|
| particules gardées, aucune sous le fond | 25 280 | 101 040 |
| **remontée, lue par les étiquettes** (le critère du plan) | **0,150 m** | **0,150 m** |
| remontée, lue par les particules (ajoutée après la mesure) | 0,188 m | 0,187 m |
| Saint-Venant 2D, ordre deux, même maille | 0,219 m | 0,240 m |
| crête au pied, APIC / Saint-Venant (témoin) | 0,077 / 0,074 m | 0,077 / 0,075 m |
| *témoin : fond lisse (S640)*, par les particules | 0,200 m | 0,190 m |
| *témoin : Saint-Venant sur le même escalier* | 0,25 m (5 cm, 2,5 cm et 1,25 cm) | |

**Critères, écrits avant.**

- **(1) Tenu** : aucune particule perdue, aucune sous le fond.
- **(2) Manqué.** Lue comme le plan le prescrivait, la remontée d'APIC vaut 65 % de Synolakis, aux deux mailles. Lue par les particules,
  elle vaut 82 % de Synolakis (dans les 20 %), mais 78 % de Saint-Venant à 2,5 cm (hors des 20 %).
- **(3)** Les deux mailles sont rapportées. L'écart ne converge pas.
- **(4) Tenu** : la vague ne déferle pas, aucune gerbe n'est détachée.

**Deux erreurs du plan**, vues à la mesure :

- **la marche** fait une maille de haut, non `dx/3` : l'escalier arrondit chaque colonne à un nombre entier de mailles, et la remontée lue par
  les étiquettes en est quantifiée (0,150 m est un multiple des deux mailles) ;
- **la lecture par les étiquettes** ne voit pas un film d'eau plus mince qu'environ une demi-maille, car le centre de la maille au-dessus de
  la marche reste d'air. Le film du jet de rive est plus mince.

## 3. L'écart, localisé (ADR-259 D1)

- **Le plat ?** Non. La crête arrive intacte au pied, à 3 % de Saint-Venant.
- **Les contremarches ?** À maille fine, non. Sans contremarches (le fond lisse de S640), la remontée vaut 0,190 m, autant que sur l'escalier
  (0,187 m). L'écart à 5 cm (0,200 contre 0,188) disparaît en affinant. Saint-Venant posé sur le même escalier n'est pas freiné.
- **Il reste** environ 20 % sous Saint-Venant, sans convergence en maille, dans le jet de rive : la mince lame qui monte la pente au-delà du
  rivage. Les causes candidates sont nommées, non départagées ([A333](../registres/ANGLES-MORTS.md)) :
  - la vitesse tangentielle mise à zéro sur les faces fermées du fond, qui freine une lame d'une ou deux mailles comme une paroi
    non glissante ;
  - la reconstruction de la surface en eau mince (S640), qui rend mal la lame ;
  - une différence physique entre Euler et l'eau peu profonde sur une pente raide (1:3), peu probable à cette ampleur.

## 4. Ce que cela dit — et ne dit pas

APIC 3D porte une vague de la mer ouverte jusqu'au pied de la plage sans la perdre, et la fait monter jusqu'à 80 % de la remontée de
référence. La crête et la masse sont justes. Le jet de rive est court d'un cinquième : en hauteur, environ 4 cm sur 23.

Pour le rouleau (étape 3 : le déferlement, sa position et son type), c'est la vague avant la rive qui compte, et elle est juste. Le jet de
rive reste ouvert (A333).

Manquent : le jet de rive juste (A333), le déferlement (étape 3), le relais 2D → 3D (étape 4), le rouleau qui agit (étape 5).
