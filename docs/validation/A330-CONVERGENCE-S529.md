# A330 localisée : le sillage de la coque dans δ à trois mailles — S529

*S529, 2026-10-06, en autonomie.* A330 (S520) : le sillage de la coque dans δ est 3,5 fois moins ample que la théorie d'une pression `ρ g d`
sur son empreinte. Deux causes possibles, séparées ici avant tout remède (ADR-226 D1) : δ sous-résolu (A317), ou un modèle de référence
inadapté.

## Reproduire

- `SILLAGE_DX=<dx> SILLAGE_NX=… SILLAGE_NY=… SILLAGE_NZ=… DT_US=<dt> DUREE=15 SORTIE=calculs/conv_s529_<n>.bin water-viewer --lineaire-sillage`
  pour (0,5 m ; 112 × 80 × 6 ; 20 ms), (0,25 ; 224 × 160 × 12 ; 10 ms), (0,125 ; 448 × 320 × 24 ; 5 ms) — 56 × 40 × 3 m, 3 m/s ; puis
  `python outils/reference_sillage.py convergence calculs/conv_s529_50.bin calculs/conv_s529_25.bin calculs/conv_s529_12.bin`.

## 1. Deux limites levées en route

- Le banc reçoit la maille, la profondeur en mailles et le pas (`SILLAGE_DX`, `SILLAGE_NZ`, `DT_US`) ; ses défauts rendent S518 au bit
  (empreinte `e41630abd739b189` ; S504 inchangé).
- À 3,4 M mailles, un tampon de la carte (230 Mo) dépassait la liaison de 128 Mo de wgpu par défaut : la carte demande désormais les limites
  de l'adaptateur (comme `apic3d_carte`). La limite des noyaux de mailles (4,19 M, S520) fixe la taille du domaine.

## 2. Mesuré (le réseau commun : tous les 50 cm, de 4 à 22 m derrière la coque, `|y|` ≤ 12 m)

| maille | 50 cm | 25 cm | 12,5 cm |
|---|---|---|---|
| rms de η sur le réseau | 0,0504 m | 0,0617 m | **0,0612 m** |
| maximum du profil des rayons (10–20 m) | 0,091 | 0,102 | **0,093** (−8 %) |
| écart moyen au voisin plus fin | 0,064 m | 0,073 m | — |
| pic d'étrave (fin / mi-parcours) | 0,30 / 0,32 m | 0,72 / 0,93 m | **1,47 / 2,48 m** |
| pas complet | — | 4,7 ms | 67,8 ms |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `‖η₂₅ − η₁₂,₅‖ < ‖η₅₀ − η₂₅‖`, ordre ≥ 0,5 | 0,073 contre 0,064 ; ordre −0,18 | **manqué** — le champ ne converge pas point par point |
| (2) la règle d'attribution : le maximum du profil à 12,5 cm à moins de 10 % de celui à 25 cm → l'écart à la théorie revient au modèle | −8 % | **le modèle** |

## 4. Ce que cela dit

- **L'amplitude du sillage de δ est stable en maille** (rms à 1 %, maximum du profil à 8 % entre 25 et 12,5 cm) : le facteur 3,5 de S520
  n'est pas une sous-résolution de δ. Selon la règle déclarée, il revient au modèle de référence — une pression sur l'empreinte n'est pas
  un corps qui perce la surface. La suite d'A330 : une référence de corps (Michell).
- **Le champ ne converge pas point par point** : à amplitude égale, les écarts entre mailles sont de l'ordre du champ — un déphasage. Le
  pas de temps suit la maille (20 / 10 / 5 ms) et δ linéaire est d'ordre 1 en temps près d'une coque qui bouge (A328, S507) : cause
  probable, **non éprouvée ici**.
- **Le pic d'étrave diverge** : il double à chaque raffinement (0,30 / 0,72 / 1,47 m) — la stagnation au coin vif d'une coque en boîte, sous
  couvercle partiel. La vague d'étrave de 4.13 n'a pas de valeur convergée pour cette coque ; une étrave effilée (ou un pic borné par le
  déferlement) le demanderait. Noté sur A317.
