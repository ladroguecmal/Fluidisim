# A333 levée : le jet de rive d'APIC 3D, par l'air balistique — S645 (liste 4.14)

*S645, 2026-10-07, en autonomie, vers la v2.* En S644, une onde solitaire non déferlante remontait la pente 1:3 à 0,187 m, contre
0,240 m pour Saint-Venant 2D et 0,230 m selon Synolakis ([REMONTEE-APIC3D-S644](REMONTEE-APIC3D-S644.md), A333). La crête arrivait
pourtant intacte au pied de la pente.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s645 -- --nocapture` (≈ 1 min) ; la maille de 2,5 cm et le
  témoin du fond glissant avec `--ignored` (≈ 5 et 6 min).
- Les essais d'APIC 3D : 51 passent, 5 longs ignorés.

## 1. Deux hypothèses, deux témoins (ADR-259 D1)

**1. Le fond non glissant — écartée.** Hypothèse : l'interpolation vers une particule de la moitié basse de la première maille d'eau mêlait
la vitesse nulle des faces du fond. Témoin : `set_seabed_slip(true)`, qui recopie dans ces faces la vitesse tangentielle d'au-dessus.

| | 5 cm | 2,5 cm |
|---|---|---|
| remontée, par les particules | 0,1860 m | 0,1855 m |

Le fond glissant ne change rien. L'option reste, éteinte par défaut.

Entre-temps, une cause triviale est écartée : la mesure ne s'arrêtait pas trop tôt. Sur 5 s, la remontée reste la même (0,1883 m à 5 cm).
APIC atteint son maximum à 1,99 s, Saint-Venant à 2,11 s : la lame d'APIC s'arrête plus tôt.

**2. Le film asservi par l'extrapolation — retenue.** Le film du jet de rive, plus mince qu'une demi-maille, est étiqueté d'air. Ses faces
ne touchent aucune maille d'eau, et l'extrapolation de S318 y remplace la vitesse que les particules apportent par celle de l'eau
derrière, qui ralentit sous la pression. Le film ne monte donc pas en vol libre, sous la seule gravité, comme le fait une ligne de rivage.

Le témoin : `set_ballistic_air(true)`. Une face d'air qu'une particule a alimentée est tenue pour connue : elle garde sa vitesse, et la
gravité.

## 2. Mesuré, avec l'air balistique

| | 5 cm | 2,5 cm |
|---|---|---|
| remontée, par les étiquettes (le critère de S644) | 0,250 m (la marche fait 5 cm) | **0,225 m** |
| remontée, par les particules | 0,288 m (des gouttes volent, comptées) | **0,2307 m** |
| Saint-Venant 2D, même maille | 0,219 m | 0,2398 m |
| Synolakis | 0,2295 m | 0,2295 m |
| crête au pied, APIC / Saint-Venant | 0,081 / 0,074 m | 0,0730 / 0,0745 m |

**Critères (écrits avant)** :

- **(1) Tenu.** À 2,5 cm, la remontée est à 6 % de Saint-Venant (étiquettes) et à 4 % (particules), et plus proche qu'à 5 cm (14 %). À
  Synolakis : 98 % et 100,5 %. **A333 est levée.**
- **(2) Tenu.** Le repos de S639 et de S388 est inchangé : les essais d'APIC 3D passent avec l'air balistique actif partout.
- **(3) Les deux options restent éteintes par défaut.**
  - Le fond glissant n'aide pas.
  - L'air balistique, actif partout, casse **S406**. La surface tremble au raccord de la zone des colonnes : −5,77 mm/s, contre +0,11
    mm/s sans lui, pour une borne de 5 mm/s.
  - Le rouleau 3D n'emploie pas la zone des colonnes : la campagne l'active.

## 3. Ce que cela dit

APIC 3D fait désormais monter une vague sur une plage à la remontée de référence, à quelques pour cent, avec la masse exacte. La crête au
pied est juste, le jet de rive aussi.

La leçon tient au film mince. Une extrapolation pensée pour l'air vide (S318) asservissait une eau qui n'était d'air que par étiquette.
L'air balistique est le bon comportement physique pour toute eau en vol (gouttes, lames, jets). Sa limite connue : le raccord avec la zone
des colonnes (S406), qui demandera de la régler si les deux sont employés ensemble.

Manquent : le déferlement (étape 3), le relais 2D → 3D (étape 4), le rouleau qui agit (étape 5), l'air balistique avec la zone des
colonnes.
