# L'air enfermé dans APIC 3D — S479

2026-10-04. **K2-1** de la [campagne K2](../registres/CAMPAGNE-K2-S478.md) ([ADR-220](../adr/ADR-220-la-campagne-k2.md) D1) : l'air qu'une
eau referme devient une **poche** (le niveau T2 d'[ADR-015](../adr/ADR-015-air-poches-et-cavites.md)), dans la référence CPU
(`code/water-core/src/apic3d_poches.rs`). Jusque-là, une bulle enfermée gardait la pression atmosphérique à toute profondeur : l'eau
s'y engouffrait, et à maille fine le calcul s'emballait à la fermeture (A311). Points de la liste : **4.12**, **7.4**, **4.16**.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core apic3d` — 43 essais, dont
  `air_pocket_holds_a_bubble_s479`.
- La bulle : `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_bulle -- 4 0.15 500`
  (`CUVE=<L>,<h>,<z_b>` : une autre cuve ; `TRACE=1` : chaque pas ; `SANS_POCHES=1` : sans) — ligne `BULLE_S479 bilan`, dix minutes.
- B10 avec poches : `APIC3D_POCHES=1 APIC3D_APRES=1.5 … --example apic3d_b10 -- 2 16` — lignes `APIC3D_B10` et
  `APIC3D_B10_POCHES bilan`, une demi-heure.

## 1. La construction

`Apic3::enable_air_pockets` (mémoire réservée à la configuration, I-06 ; sans lui, le pas d'avant **au bit** : la projection avec
poches est une fonction à part). À chaque pas, après les étiquettes :

1. **Les composantes** : l'air libre est ce qu'un remplissage depuis la rangée du haut atteint ; chaque autre composante d'air
   (six voisines) est une poche — 64 au plus (au-delà, laissées libres et comptées).
2. **Le suivi** : une poche hérite de l'air des poches d'avant qu'elle recouvre (fusion : la somme ; scission : au prorata des
   mailles) ; sans recouvrement, elle **naît** à la pression de l'eau qui la borde au pas d'avant.
3. **L'air** : l'invariant adiabatique `a = V·P^(1/γ)`, γ = 1,4 — `P = (a/V)^γ`.
4. **Dans la projection, une inconnue par poche** : la loi linéarisée `P^(n+1) − Pⁿ = −(γPⁿ/Vⁿ)·ΔV`, `ΔV` le flux de ses faces
   après correction, donne la ligne `(s + Σ 1/θ)·p_b − Σ p_c/θ = s·p_bⁿ − (ρ·dx/dt)·Σ u*_sortant`, `s = ρ·Vⁿ/(γPⁿ·dt²·dx)` ; les
   mailles d'eau voisines reçoivent `−p_b/θ` : le système reste **symétrique défini positif**, le même gradient conjugué le résout,
   **implicite** — stable quelle que soit la raideur de la poche devant le pas.
5. Une poche de moins d'une maille se résorbe (son air redevient libre ; les microbulles, K2-7).

**Trouvé en chemin — le volume géométrique saute.** Mesuré seul sur la surface reconstruite (la fraction d'air `0,5 + φ/dx` de ses
mailles et des mailles d'eau qui la bordent), le volume d'une poche saute d'un pas à l'autre quand des mailles changent d'étiquette
— **jusqu'à 1 %, trois mailles d'un coup** ; chaque saut frappe la poche d'un coup de pression : la première bulle oscillait à
**56 Hz, × 1,34** de Minnaert. **Remède** : le volume **suivi par le flux** que la projection prévoit (lisse, cohérent avec les
vitesses), **rappelé** vers le volume géométrique en 0,1 s (contre la dérive).

## 2. Les mesures

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) sans poches, au bit | les tests d'APIC 3D passent ; la projection d'avant n'est pas touchée (une fonction à part) | tenu |
| (2) une bulle garde son volume, remonte, masse exacte | R = 0,08 m (R/dx = 4), son centre à 0,3 m de fond dans une cuve de 0,8 m : elle **remonte** (0,258 → 0,300 m en 0,15 s) ; volume moyen comprimé de **1,6 %** (l'adiabatique sous la charge `ρ·g·z` en attend 2,0 %), oscillation de ± 1,3 % ; masse exacte | tenu, sur 0,15 s (la bulle atteindrait la surface avant une seconde) |
| (3) Minnaert à 15 % | **42,5 Hz** contre **41,6** (× 1,021) ; la cuve, par la méthode des images (parois et fond rigides, surface libre), attendrait **38,4 Hz** (× 1,107) | tenu pour les deux |
| (4) A311 : B10 au bout à `D/dx` = 16 | **au bout**, 1,5 √(D/g) après le pincement, 31 min ; pincement **2,084 √(R/g)** (S393 sans poches : 2,08) ; la bulle **vit** : 0,283 D³ au plus, 0,231 D³ à la fin, pression de 82 à 155 kPa ; `D/dx` = 8 : de même (0,262 D³ ; 98 à 142 kPa) | tenu |
| (4 bis) `D/dx` = 24 | lancé (environ 2 h 30) ; consigné à son arrivée | en cours |

La fréquence de Minnaert pour une bulle de 8 cm sous 0,3 m d'eau : `f = (1/2πR)·√(3γP/ρ)` = 41,6 Hz ; la correction de la cuve se
calcule par la somme des sources images (`1/√(1 + R·Σ sᵢ/dᵢ)` = 0,9225 ; 0,9589 dans une cuve deux fois plus grande, dont le calcul,
trois millions de particules, est en cours).

## 3. Ce qui reste

- **La carte** (K2-2) : le même modèle sur le GPU, une réduction par poche.
- **La vie longue de la bulle** : la remontée jusqu'à la surface, la vitesse terminale (K2-3, contre Davies et Taylor) ; la
  fragmentation.
- **A312** : la gerbe suit toujours la maille (couronne 0,205 D à 8 mailles, 0,305 D à 16) — K2-4.
- Le mode relatif (B dans la bande) refuse les poches pour l'instant.

*Note corrective du 2026-10-05 (S484)* : le centre des poches était divisé par leur volume entier (mailles d'eau voisines comprises) et tiré vers l'origine d'un facteur ≈ 0,8 ; la remontée « 0,258 → 0,300 m » ci-dessus en est faussée (la bulle partait de 0,30 m). Corrigé ; voir [REMONTEE-S484](REMONTEE-S484.md).

*Note du 2026-10-05 (S485)* : **B10 à `D/dx` = 24** (critère 4 bis), arrivé : pincement à 2,024 √(R/g) (2,084 à 16 mailles), la bulle 0,328 D³ au plus, 0,243 à la fin, pression de 76,6 à 160,5 kPa. **La grande cuve** (S480) : 47,84 Hz contre 42,22 de Minnaert, rapport 1,133 — avec le rappel de 0,1 s et le partage de l'air de l'époque ; la bulle de la petite cuve, avec les règles de S485, oscille à 37,2 Hz (à 3 % de la valeur corrigée de la cuve). Voir [REMONTEE-S485](REMONTEE-S485.md).
