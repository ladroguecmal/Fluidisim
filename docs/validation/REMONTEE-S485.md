# La remontée lente attribuée — S485 (K2-3b)

*S485, 2026-10-05, en autonomie.* La suite de [REMONTEE-S484](REMONTEE-S484.md) (0,338 m/s contre 0,603 de Davies et Taylor à R/dx = 6,
dans un quart de cuve). **Attribuée** : la bulle perdait son air, et le quart de cuve n'est pas une symétrie. **Dans une cuve entière,
U = 0,548 m/s, 0,90 de Davies et Taylor.**

## Reproduire

- `CAS=remontee [R_DX=4] [QUART=0|1] [DUREE=1.0] [PAS_US=1000] [TRACE=1] [TRACE_POCHES=t0,t1] [ENSEMBLE=0] water-viewer --apic3d-poches` —
  la cuve d'`apic3d_remontee`, ensemencée par la référence, menée par la carte seule (elle suit la référence à 10⁻⁴, S481–S482) ; la
  bulle = toutes ses poches (le centre pondéré par les volumes ; `ENSEMBLE=0` : la plus grande seule).
- `FILS=16 code/target/release/examples/apic3d_bulle.exe 4 0.15 500` — la bulle de S479 (Minnaert).

## 1. La bulle perdait son air — deux défauts de la référence de S479

**(a) Les fragments emportaient leur part.** L'air d'une poche se partageait, au prorata des mailles recouvertes, entre **toutes** les
nouvelles poches, y compris les fragments d'une à sept mailles que le bruit des étiquettes détache à chaque pas — résorbés aussitôt, avec
leur part. **Corrigé** (référence et carte) : l'air se partage entre les seules poches gardées ; il ne se perd que si aucune poche gardée ne
recouvre l'ancienne (la bulle qui crève).

**(b) La calotte envahie.** Vers 0,24 s (R/dx = 8), la calotte se scinde ; dans la poche haute, les particules entrent (elles suivent les
vitesses extrapolées dans l'air, que la projection ne contraint pas) : **160 → 21 mailles en 30 ms**, quand le volume suivi par le flux ne
bouge pas — la pression ne réagit pas, la poche passe sous huit mailles et se résorbe : **42 % de l'air perdu**. **Corrigé** : le rappel du
volume suivi vers la géométrie, `RAPPEL_VOLUME_S`, de 0,1 à **0,02 s** — la pression monte quand la géométrie se vide. L'air de la bulle est
conservé à 0,3 % sur 0,6 s, scindée en quatre poches.

La bulle de S479 avec le rappel de 0,02 s : **37,2 Hz**, 0,89 de Minnaert en eau infinie, **à 3 % de la valeur corrigée de la cuve**
(38,4 Hz, S479) — le critère de S479 (15 %) tient.

## 2. Les causes de S484, mesurées

Carte, pas de 1 ms, air conservé, la bulle entière :

| cuve | R/dx | U (m/s) | Davies–Taylor (`d_e` mesuré) | rapport |
|---|---|---|---|---|
| quart | 4 | 0,343 (se résorbe à 0,9 s) | 0,598 | 0,57 |
| quart | 6 | 0,414 | 0,614 | 0,67 |
| quart | 8 | 0,376 | 0,618 | 0,61 |
| **entière** | **4** | **0,548** | **0,607** | **0,90** |

**Le quart de cuve était le défaut principal** : à la même résolution, la bulle entière monte 1,6 fois plus vite. Les parois d'APIC ne
sont pas des plans de symétrie pour une bulle qui les longe (la reconstruction les reflète, la séparation et l'extrapolation non ; la calotte
s'y étale). Dans le quart, l'affinement ne converge pas vers la référence publiée — un signe de plus que le montage, non la maille, était en
cause.

**La cuve entière à R/dx ≥ 6 dépasse la carte** : au-delà d'environ 8 millions de particules, une passe demande plus de 65 535 groupes
(G2P, l'advection : des dispatchs à une dimension). Le passage en deux dimensions est la suite (la reconstruction le fait déjà, S423).

## 3. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) le rejeu de S484 sur la carte à 5 % | les deux calculs coïncident au chiffre à 0,05 s, puis divergent (0,440 contre 0,338 à 0,4 s) | **manqué** |
| (2) chaque cause mesurée | la perte d'air (deux mécanismes, corrigés) ; le quart contre la cuve entière ; la résolution (dans le quart) ; le pas — non mesuré séparément (1 ms partout) | tenu, sauf le pas |
| (3) U à 15 % de Davies–Taylor dans une configuration résolue | cuve entière, R/dx = 4 : **× 0,90** | **tenu** |

## 4. B10 à 24 mailles (lancé en S480, arrivé en S485)

`APIC3D_POCHES=1 APIC3D_APRES=1.5 FILS=16 apic3d_b10 2 24` (avec les règles d'avant S485) : **pincement à 2,024 √(R/g)** (2,084 à 16 mailles :
2,9 %) ; la bulle vit — **0,328 D³ au plus, 0,243 à la fin** (0,283 et 0,231 à 16), pression de 76,6 à 160,5 kPa. Consigné aussi dans
[POCHES-AIR-S479](POCHES-AIR-S479.md).

## 5. Ce qui reste

- La cuve entière à R/dx = 6 et 8 (les dispatchs à deux dimensions sur la carte) : la convergence de U.
- Le pas (1 contre 2 à 4 ms).
- La fragmentation et la résorption en champ (K2-3, suite), C13 (les petites bulles, K2-7).
