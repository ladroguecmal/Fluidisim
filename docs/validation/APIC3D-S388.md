# APIC en trois dimensions, dans le cœur — S388

2026-09-26. **C4a** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5), la seconde représentation de δ ([ADR-186](../adr/ADR-186-apic-seconde-representation.md)). Session cloud, un fil, sans
carte graphique : les durées sont celles de ce conteneur. Liste **4.16** (surface non graphe), **4.12** ; lot 5.

## Reproduire

- Commit de P7 de S388 ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s388 -- --nocapture` — sept essais, dix
  secondes ; lignes `S388` (rayons, surfaces lues, repos).
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_ballottement -- <10|11> <dx>`
  — ligne `APIC3D_S388` ; `APIC3D_TRACE=1` imprime l'énergie toutes les 25 pas. Durées : (1, 0) à 5 cm, 15 s ; à 2,5 cm et
  (1, 1), quelques minutes (§3).

## En une phrase

REMPLACER

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
