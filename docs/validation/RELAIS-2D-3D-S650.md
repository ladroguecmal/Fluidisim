# Le rouleau 3D, étape 4 : le relais 2D → 3D — S650 (liste 4.14)

*S650, 2026-10-07, en autonomie, vers la v2.* La 3D seulement là où elle sert : Saint-Venant 2D porte la vague du large, APIC 3D prend
la pente où elle déferle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_2d_to_3d_relay -- --ignored --nocapture` (≈ 7 min :
  le relais puis le tout-3D, chronométrés dans la même exécution).

## 1. Le montage

L'onde de S647 (`H/d` = 0,3, pente 1:12, plongeante selon Grilli et al. 1997) est coupée en deux à `x_r` = 5,0 m, 0,7 m avant le pied.

- **Saint-Venant 2D** (S613, à l'ordre deux de S620) porte l'onde sur toute la plage, depuis le même état initial.
- **APIC 3D** ne couvre que `[x_r ; 12,8]` m :
  - une **zone de colonnes** (S398) borde le large sur 0,6 m ;
  - les **particules** occupent la pente, avec l'air balistique (S645) ;
  - le bord gauche est **ouvert** (S446) et reçoit à chaque pas la vitesse de Saint-Venant à `x_r` (`q/h`, uniforme sur la
    verticale) ; la zone de colonnes compte le débit qui entre ;
  - le fond plat est posé à z = 0, sans maille solide sous les colonnes.
- L'état initial de la 3D est celui de l'onde (`η` des colonnes, vitesses de la grille et des particules).
- Les lecteurs de S647 et S648 sont réemployés tels quels.

## 2. Mesuré, à 5 cm

| | relais | tout-3D (S647–S648) |
|---|---|---|
| premier retournement | **2,575 s, 9,825 m** | 2,571 s, 9,675 m |
| premier air enfermé | 2,667 s, 10,075 m | 2,872 s, 10,525 m |
| volume de la 3D moins le volume entré compté | −3,7·10⁻¹⁷ du total | — |
| volume entré : compté par les colonnes / débit de Saint-Venant | 0,05237 / 0,05226 m³ (0,2 %) | — |
| particules hors du fond | toutes | toutes |
| temps de calcul | **163 s** | 256 s |

Les particules passent de 19 840 à 23 312 : la zone de colonnes en pose pour l'eau qui entre (S399).

**Critères, écrits avant.**

- **(1) Tenu** : la masse est exacte au débit compté près.
- **(2) Tenu** : le retournement se produit avant le rivage, à 0,004 s du tout-3D et à 0,150 m, à la limite de la borne de 0,15 m.
- **(3) Tenu** : l'air enfermé apparaît après le retournement, en avant de lui.
- **(4)** Le relais prend 64 % du temps du tout-3D.

## 3. Ce que cela dit — et ne dit pas

Une vague portée par Saint-Venant jusqu'au pied de la plage, puis par APIC 3D, déferle au même instant que la vague tout en 3D, 15 cm plus
loin. La masse se compte au bit à travers le raccord. Le rouleau qui suit a la même signature : le retournement, puis l'air enfermé. L'air
apparaît plus tôt dans le relais (+0,09 s au lieu de +0,30 s) ; la cause n'est pas départagée : l'onde portée par Saint-Venant, raidie
sans dispersion, ou le raccord.

Ici, la 3D commence 0,7 m avant le pied, et l'économie n'est que d'un tiers. Sur une vraie mer, la part de Saint-Venant fait des
kilomètres, et la 3D se réduit à la bande de déferlement.

Ne fait pas encore :

- le relais en sens inverse (la vague réfléchie qui ressort vers Saint-Venant) ;
- la bande 3D qui naît et meurt avec la vague (le cycle de vie de S637–S638 appliqué ici) ;
- la maille de 2,5 cm.

Manquent : le relais dans les deux sens, la bande qui vit avec la vague, le rouleau qui agit (étape 5).
