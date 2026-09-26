# Le raccord en 3D, première part : la zone des colonnes dans APIC 3D — S398

2026-09-27. **C5b** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5 ; [conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §4.1 A1, §4.2), après C5a en 2D
([B10-APIC-S320](B10-APIC-S320.md) §14–16). Session cloud, un fil par calcul, sans carte graphique : les durées sont celles de
ce conteneur. Liste **4.16**, 4.12 ; A316.

## Reproduire

- Commit `9eea866c` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s398 -- --nocapture` — trois essais, deux
  secondes ; lignes `S398`.
- `APIC3D_COLONNES=1 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_ballottement
  -- <10|11> <dx>` (tout en colonnes) et `APIC3D_DELTA=1 …` (δ, même cuve) — ligne `APIC3D_S398`. Sans variable, la ligne de
  S389. Durées : 3 à 65 s. Valeurs attendues : §3.

## En une phrase

Le raccord demande une seule projection pour deux représentations ; `Apic3` reçoit une **zone de colonnes** — surface `η` par
colonne, vitesse eulérienne advectée, débits mouillés comme δ — qui, seule, ballotte **à 0,03 point de δ au plus** sur les cas
que δ calcule, garde son volume à 10⁻¹⁰, tient le repos à 2,4·10⁻⁵ m/s, et ne change rien quand elle est absente.

## 1. La construction

**Pourquoi dans `Apic3`.** La conception (§4.2) veut, dans un domaine, des colonnes où la surface est un graphe et des
particules dans une bande, **sous une même projection** — fluide fantôme sur `η` pour les unes, sur la surface reconstruite
pour les autres. `Apic3` a la projection à fluide fantôme, la reconstruction, les transferts, les parois et le corps ; `Volume3`
porte des modes nombreux (fond coupé, colonne graduée, couplage à B) qu'une bande heurterait. La référence la plus simple
d'abord ; la production (C7) aura son portage.

**La zone** (`code/water-core/src/apic3d_columns.rs`), désignée par un masque de colonnes, réservée à la configuration (I-06) :

- **surface `η` par colonne** : `φ = z − η` et l'eau sous `η` — sans reconstruction, donc sans le biais qui dépend de
  l'arrangement des particules (S323) ; les fractions fantômes qui en sortent sont celles de δ, verticale `(η − z)/dx` et
  latérale `(η_c − z)/(η_c − η_v)` ;
- **vitesse eulérienne**, gardée sur la grille d'un pas à l'autre et **advectée au pied de la caractéristique**, `u(x_f) ←
  u⁻(x_f − dt·u⁻(x_f))` — la leçon de S397 : des colonnes qui n'advectent pas entretiennent une circulation à la frontière ;
- **`η` transporté par les débits mouillés**, hauteur de face moyenne des deux colonnes et somme compensée, comme
  `transport_mobile3` de δ.

Quatre crochets dans le pas, **inertes sans masque**. Une face entre une colonne et une colonne de particules n'échange encore
rien : c'est la deuxième part.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans masque, au bit | **tenu** : `apic3d_ballottement 10 0.05` imprime la ligne de S389 au chiffre près (période 1,9964 s, +1,01 %, énergie +7,04 %, amortissement +0,21 %, 93,5 itérations) ; suite **694 réussis**, 18 ignorés, zéro avertissement |
| **2** — toutes colonnes, repos 2 s : vitesse ≤ 1 mm/s ; volume à 10⁻⁶ | **tenu** : **2,4·10⁻⁵ m/s**, volume à 7·10⁻¹⁵ ; une bosse de 5 cm qui se déploie une seconde garde son volume à 2·10⁻¹¹ |
| **3** — toutes colonnes, ballottements (1, 0) et (1, 1) à 5 et 2,5 cm : période à 1 point de δ ; amortissement ≥ 0 | **tenu** sur les trois cas que δ calcule : **≤ 0,03 point** ; le quatrième, que δ refuse, à −0,05 % de l'exacte (§3) |

## 3. Les ballottements — colonnes contre δ

Cuves de S388 : (1, 0) 2 × 0,2 m, 0,5 m d'eau, 2 cm, 10 s ; (1, 1) 1 × 1 m, 5 s. Période sur le **moment** de la surface,
amortissement par régression sur les pics (S389). δ : `Volume3`, pas mobile, 20 ms.

| cas | colonnes : erreur de période ; amortissement par période | δ | écart de période |
|---|---|---|---:|
| (1, 0), 5 cm | +0,02 % ; +0,49 % | +0,02 % ; −0,02 % | 0,00 point |
| (1, 0), 2,5 cm | −0,05 % ; +0,32 % | **refus** (`Convergence`) | à l'exacte : −0,05 % |
| (1, 1), 5 cm | +0,31 % ; +1,36 % | +0,33 % ; −0,04 % | 0,02 point |
| (1, 1), 2,5 cm | +0,04 % ; +0,89 % | +0,07 % ; +0,08 % | 0,03 point |

Pour mémoire, APIC 3D **en particules** sur les mêmes cas (S389) : (1, 0) +1,01 et +0,39 % ; (1, 1) +2,63 et +1,01 %. La zone
de colonnes a la dispersion de δ — même grille, même géométrie fantôme — et **dissipe davantage** : l'advection
semi-lagrangienne, du premier ordre, amortit de 0,3 à 1,4 % par période là où l'advection centrée de δ ne dissipe presque pas.

**Observé sur δ, non étudié ici** : sa projection mobile **refuse** la cuve mince (1, 0) à 2,5 cm (80 × 8 × 40 mailles) —
Jacobi, même à 200 000 itérations ; la multigrille de S385 la refuse aussi, et refuse la même cuve à 5 cm (4 mailles de large),
que Jacobi accepte.

## 4. Ce que ce document ne dit pas

- **Le raccord lui-même** : aucune bande de particules, aucun échange à la frontière — la deuxième part de C5b, avec les
  instruments de S394–S397 (masse de chaque côté, densité, profil de la face, période sur 30 s).
- **La dissipation** de l'advection semi-lagrangienne : une advection d'ordre plus élevé (MacCormack, ou celle de δ avec le
  terme de Lax-Wendroff, ADR-209) l'abaisserait ; elle n'est pas faite.
- Le refus de δ sur la cuve mince, ni sa cause ; rien sur la carte.
