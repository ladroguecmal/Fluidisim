# APIC en trois dimensions, dans le cœur — S388

2026-09-26. **C4a** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5), la seconde représentation de δ ([ADR-186](../adr/ADR-186-apic-seconde-representation.md)). Session cloud, un fil, sans
carte graphique : les durées sont celles de ce conteneur. Liste **4.16** (surface non graphe), **4.12** ; lot 5.

## Reproduire

- Commit `897a9ee4` (P7 de S388) ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s388 -- --nocapture` — six essais, dix
  secondes ; lignes `S388` (rayons, surfaces lues, repos).
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_ballottement -- <10|11> <dx>`
  — ligne `APIC3D_S388` ; `APIC3D_TRACE=1` imprime l'énergie toutes les 25 pas ; témoins `APIC3D_RAYON=plan` et
  `APIC3D_SANS_SEPARATION`. Durées : (1, 0) 15 s à 5 cm, 153 s à 2,5 cm ; (1, 1) 18 s et 174 s. Valeurs attendues : §3.

## En une phrase

APIC entre dans le cœur en trois dimensions : il garde la masse exactement, conserve un champ affine, tient le repos à
5,6 mm/s, et fait ballotter une cuve à **+2,05 % puis +1,04 %** de la période exacte (5 et 2,5 cm) — mieux qu'en 2D à 5 cm,
moins bien à 2,5 cm — et un mode oblique à +2,69 % ; trois critères sont manqués (la surface à 1 %, le ballottement à 1 %,
l'énergie à 1 %), et la mesure dit pourquoi : **la lecture de la surface depuis les particules commande la période**.

## 1. La construction

`code/water-core/src/apic3d.rs`, **dans le cœur** et non plus dans un banc (METHODE : il y entre avec ses essais). Le candidat
2D de S318–S320 porté aux trois dimensions : particules à huit par maille (2 × 2 × 2), vitesse et matrice affine `C`
(APIC, Jiang et al. 2015), transferts **trilinéaires** sur les trois grilles décalées ; surface **reconstruite** des
particules (Zhu et Bridson 2005, noyau `(1 − s²/dx²)³`) ; projection à **fluide fantôme** sur l'iso-zéro, gradient conjugué
préconditionné par la diagonale, arrêt `‖r‖ ≤ 10⁻⁶‖b‖` ; extrapolation des vitesses vers l'air sur trois couches, puis
remise à zéro sauf les faces qu'une particule alimente (S318, fautes 5 et 6) ; advection RK2 ; séparation des particules à
0,4 maille, deux passes (S320). `f32` local (I-08), `g_eff` fourni (I-07), toute la mémoire réservée à la configuration et
comptée au flottant près (I-06). **Aucun lien encore avec `Volume3`** : le raccord est C5.

## 2. Ce qui est mesuré — critères écrits avant

| critère | résultat |
|---|---|
| **1** — un champ affine traverse les transferts | **tenu** : `v` à 10⁻⁵ relatif, `C` à 10⁻⁴, particules à une demi-maille des parois ; **vu échouer** sans le terme affine |
| **2** — l'iso-zéro d'une nappe plane à sa hauteur, à 1 % de maille | **manqué hors du cas réglé** (ci-dessous) |
| **3** — le repos : masse exacte, vitesse parasite ≤ 1 cm/s à 5 cm | **tenu** : cuve de 1 m, 0,5 m d'eau, 2 s — **5,6 mm/s** au pire, 1,2 mm/s à la fin (2D : 4,4 mm/s) ; 57 itérations, résidu 9,7·10⁻⁷ |

**La surface lue, et le rayon de la reconstruction.** Avec deux rangées de particules par maille, la hauteur d'eau que les
particules portent tombe soit sur une face de maille, soit sur un centre. La pression lit l'iso-zéro **interpolée entre deux
centres**. Mesuré en 3D et recalculé en f64 sur un réseau (les six cas concordent au centième de pourcent) :

| rayon | valeur à 10 cm | surface sur une face | surface sur un centre |
|---|---:|---:|---:|
| au point de la surface (S318, `d₀`) | 0,02743 m | **−15,05 %** de maille | 0,00 % |
| réglé sur les centres d'une face | 0,03843 m | 0,00 % | +9,91 % |
| **minimax (retenu)** | **0,03395 m** | **−6,12 %** | **+6,12 %** |

Aucun rayon ne tient 1 % partout : c'est la granularité de deux rangées par maille (S318, faute 2 : une onde plus petite
que l'espacement des particules n'existe pas). L'affirmation de S318 — « l'iso-zéro d'une eau au repos tombe exactement à
sa hauteur » — n'est vraie que pour une surface qui passe par un centre. **Faute d'instrument corrigée en chemin** : le
modèle f64 mettait `2·dx − r` là où la reconstruction met `dx` sans voisine, et lisait un centre à +4,5 % quand la
reconstruction lisait +7,2 %.

## 3. Les ballottements

`examples/apic3d_ballottement.rs`. Mode (1, 0) de S318 — cuve de 2 m, 0,5 m d'eau, 2 cm, invariant en `y` (0,2 m de large),
10 s — et mode **oblique** (1, 1) d'une cuve carrée de 1 m, 5 s. Période lue sur un **moment** de la masse (lisse), passages
par zéro interpolés.

| cas | 5 cm | 2,5 cm | critère (écrit avant) |
|---|---|---|---|
| (1, 0) — exacte 1,9765 s | **+2,05 %** (2D : +5,9 %) | **+1,04 %** (2D : −0,15 %) | ≤ 7 % **tenu** ; ≤ 1 % **manqué de 0,04 point** ; décroissante **tenu** |
| (1, 1) — exacte 0,9630 s | +7,64 % | **+2,69 %** | ≤ 2 % **manqué** |
| énergie créée, % de l'onde analytique | +22,4 % ; (1, 1) +26,9 % | +7,0 % ; (1, 1) +12,7 % | ≤ 1 % **manqué** |
| masse | exacte | exacte | — |
| itérations moyennes ; calcul | 93 ; 15 s | 189 ; 153 s | — |

À 10 cm, l'amplitude de 2 cm est sous l'espacement des particules (5 cm) : l'onde n'existe pas, la « période » lue est du
bruit (−80 %) — non retenu, comme en S318.

**L'énergie.** Elle **oscille** dans chaque période — de +0,15 à −0,56 fois l'énergie analytique de l'onde à 5 cm — et
**décroît** sur la durée : rien ne croît. L'onde que portent les particules, posées en rangées de 2,5 cm, a 1,64 fois
l'énergie analytique (énergie cinétique de pointe ; S318 : 2,66 cm représentés pour 2 cm posés). Et 1 % de l'énergie de
l'onde vaut quelques micromètres de centre de masse, sous ce que la position des particules sait dire : le critère était
plus fin que sa mesure. Il reste **manqué** ; il est à réécrire sur une mesure qui le porte (l'amortissement par période,
comme S354).

**Attribution** (témoins, (1, 0) à 5 cm) : avec le rayon de S318, **+7,87 %** au lieu de +2,05 % ; sans séparation, +2,29 %,
énergie +22,3 %. **La lecture de la surface commande la période** ; la séparation n'y est presque pour rien, et ni l'une ni
l'autre n'explique l'énergie.

## 4. Ce que ce document ne dit pas

- **Rien du déferlement, des gerbes ni de la cavité** : aucune surface non graphe n'est éprouvée ici (4.16 reste absent).
  B10 en 3D — une sphère qui entre dans l'eau — est **C4b**.
- **Aucun raccord aux colonnes** (C5) ni critère de bascule (C6) : APIC tient seul le domaine.
- **La piste d'amélioration**, mesurée mais non faite : la surface — plus de particules par maille, ou la surface portée
  par un ensemble de niveaux (ADR-186 D4).
- Largeur de 0,2 m pour (1, 0) : les parois en `y` sont proches ; le mode ne dépend pas de `y`, mais leur effet n'est pas
  mesuré. Rien sur la carte ; durées d'un conteneur cloud.
