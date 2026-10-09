# Saint-Venant troué, rempli par un second Saint-Venant — S723 (liste 4.14 ; LOD, étape 3, B1)

*S723, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-3-S722, B1. Pour rallumer la 3D autour d'un corps, Saint-Venant doit avoir
un trou là où la 3D est vivante, et recevoir d'elle un flux sur les quatre faces de ce trou. B1 le construit et le juge entre deux copies du
même solveur (ADR-273 D1).

## Ce qui est fait (`saint_venant_2d.rs`, à l'ordre deux)

- **`regler_trou(i0, i1, j0, j1)`** : un rectangle de mailles gelées. Leurs faces avec l'eau active sont des parois (la pression de la
  maille active, aucun flux calculé). La reconstruction des mailles voisines n'y lit rien : les pentes y sont nulles.
- **`pas_avec_flux_trou(dt, flux)`** : sur chaque face du trou, le **flux complet** `[masse, normale, tangentielle]`, orienté vers `+x`
  ou `+y`. La pression de paroi est retirée, le flux la remplace.
- **`pas_avec_flux_bords4(dt, flux)`** : le même flux imposé sur les quatre bords du domaine.
- Sans trou ni flux, le code ne change pas : le banc de non-régression est au bit, et les essais de Saint-Venant passent.

## Reproduire

- `cargo test --release -p water-core --lib holed_saint_venant -- --nocapture` (≈ 0,3 s).

## Mesuré

Une bosse de 2 cm (rayon 0,3 m) sur 0,5 m d'eau, un fond plat de 3 m × 3 m, `dx` = 5 cm. Un trou de 0,75 m × 1,25 m sur son chemin, rempli
par un second Saint-Venant. À chaque pas, le flux de Rusanov de chaque face d'interface va, le même, aux deux côtés. On compare au
Saint-Venant entier, à 1,5 s.

| l'échange | la masse | l'écart maximal de `h` |
|---|---|---|
| la masse seule, chaque côté avec sa pression de paroi (premier passage) | 3,5·10⁻¹⁵ | 1,66 mm (**8,3 %** de la bosse) |
| **le flux complet, le même vecteur aux deux côtés** | **2,8·10⁻¹⁵** | **0,32 mm (1,6 %)** |

**Critères : tenus** (la masse à 10⁻¹², l'écart sous 5 %, le banc au bit).

## Ce que cela dit

- **Saint-Venant sait maintenant avoir un trou** et le raccorder sur ses quatre côtés, la masse et la quantité de mouvement conservées.
- **Le raccord échange le flux complet**, non la masse seule. N'échanger que la masse laisse à l'interface une paroi qui réfléchit l'onde
  (8,3 % au lieu de 1,6 %). La règle vaudra pour B3 : ce que la 3D rend à Saint-Venant sur les faces du trou est un flux complet.
- **L'écart restant (1,6 %)** vient du flux d'interface, d'ordre un et gelé sur le pas de Heun.

## La suite

**B2** : APIC à quatre bords. Le bord à particules en y, comme en x (S698) ; jugé au repos, puis par une onde en biais rejouée (S699).
