# B10 en 3D : une sphère entre dans l'eau — S393

2026-09-26. **C4b** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5), seconde part : le cas de [S320](B10-APIC-S320.md) — un corps **cinématique** entre dans l'eau et ouvre une cavité qui se
pince — porté sur APIC 3D ([APIC3D-S388](APIC3D-S388.md)), avec une **sphère** au lieu d'un cylindre. Session cloud, un fil
par calcul, sans carte graphique : les durées sont celles de ce conteneur. Liste **4.16** (surface non graphe), **4.12**
(cavité) ; lot 5.

## Reproduire

- Commit `bed30a42` (P5 de S393) ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s393 -- --nocapture` — trois essais, huit
  secondes ; ligne `S393 sphère au repos` (vitesse parasite).
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example apic3d_b10 -- <Fr> <D/dx>
  [quart|entier] [demi_largeur]` — ligne `APIC3D_B10` ; `APIC3D_TRACE=1` imprime chaque pas. Valeurs attendues : §3 ;
  durées : 40 s (`2 8`) à 816 s (`2 16`).
- Critère 1 : `… --example apic3d_ballottement -- 10 0.05` imprime la ligne de S389 P4, au chiffre près (§2).

## En une phrase

Une sphère qui entre dans l'eau à vitesse imposée ouvre une cavité qui se **pince à 2,08 √(R/g)** à la maille la plus fine —
dans la plage que la littérature mesure pour le pincement profond d'une sphère (1,72 à 2,29) —, ce temps tient à **0,6 %**
entre 12 et 16 mailles par diamètre, bouge peu avec la vitesse (1,99 à `Fr` = 4 contre 2,13 à `Fr` = 2, à 8 mailles), et la
masse est exacte ; la couronne, elle, suit encore la maille, comme en 2D.

## 1. La construction

**Le corps dans le cœur** (`code/water-core/src/apic3d.rs`, `Sphere3`, `set_body`). Le geste du banc 2D de S320, en 3D :
les mailles dont le centre est dans la sphère deviennent **solides** ; toute face qui en touche une prend la vitesse du
corps, après chaque opération qui écrit les faces ; la pression y voit une **paroi mobile** (ni coefficient ni correction
vers un solide ; le flux du corps entre dans la divergence) ; après l'advection, le corps avance de `v·dt` et repousse à
`R + 0,05·dx` les particules qu'il a atteintes, leur vitesse normale portée au moins à la sienne. L'hôte impose position et
vitesse à chaque pas ; aucune force ne revient au corps. Sans corps, le pas est celui de S389 (critère 1).

**Trouvé en chemin — le corps doit refléter.** Au premier passage, une sphère **au repos** à demi immergée faisait courir
l'eau à **9,4 cm/s**, au ras de l'eau contre elle : c'est le biais de paroi de S389 — contre le corps, le noyau de la
reconstruction ne voit des particules que d'un côté, la surface y paraît plus basse, l'eau monte le long de la sphère. Le
remède est le même qu'aux parois : dans la reconstruction, le corps **reflète** les particules (image radiale `c + (2R −
d)·n`, cherchée seulement pour les centres à moins d'un rayon de noyau de la sphère).

**Le quart de domaine.** La sphère est axisymétrique ; l'axe posé au coin du domaine, les deux parois latérales sont deux
plans de symétrie (vitesse normale nulle, particules reflétées par la reconstruction), et le calcul coûte quatre fois
moins. Le critère 3 éprouve ce geste contre le domaine entier.

**Le banc** (`examples/apic3d_b10.rs`), celui de S320 en 3D : `D` = 0,4 m, la base part au ras de l'eau, vitesse `U =
Fr·√(g·D)`, arrêt quand la base atteint `3·Fr·D` ; eau sur `3·Fr·D + 2D`, air sur `2,5·D` ; parois à `2D` de l'axe (défaut).
**Air enfermé** : mailles sans particule, hors du corps, sous le repos, au-dessus du haut du corps, à moins d'un diamètre de
l'axe, qu'un remplissage depuis la rangée du haut n'atteint pas ; **pincement** : premier pas où il dépasse `D³/32` (domaine
entier), lu à **chaque pas**.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans corps, rien ne change | **tenu** : `apic3d_ballottement 10 0.05` — période 1,9964 s (+1,01 %), énergie +7,04 %, amortissement +0,21 %, 93,5 itérations, 500 pas : la ligne d'avant, au chiffre près |
| **2** — sphère au repos à demi immergée (`D/dx` = 8, quart, 2 s) : vitesse parasite ≤ 1 cm/s, masse exacte | **tenu de justesse** : **9,6 mm/s** au pire, 0,77 mm/s à la fin (2D, S320 : 5,4 mm/s) ; sans le reflet du corps, 9,4 cm/s ; **vu échouer** sans mailles solides (27,6 cm/s) |
| **3** — quart contre domaine entier (`Fr` = 2, `D/dx` = 8) : même pincement à un pas près | **tenu, à la limite** : 1,4803 √(D/g) (entier) contre 1,5067 (quart) — **un pas** (0,0264 pour un pas de 0,0266) |
| **4** — `Fr` = 2 : le pincement à 12 et 16 mailles par diamètre à 5 % l'un de l'autre | **tenu** : **0,63 %** (§3) |
| **5** — à la maille la plus fine, `t_p/√(R/g)` dans [1,72 ; 2,29] | **tenu** : **2,084** (§4) |
| **6** — masse exacte partout ; suite, zéro avertissement | **tenu** : masse exacte dans chaque calcul (vérifiée à la fin de chacun) ; suite **684 réussis**, 18 ignorés, zéro avertissement. Le banc 2D de S318–S320 (`lot5_comparaison.rs`) n'est pas touché depuis S354 : au bit |

## 3. Le pincement

`Fr` = 2 (`U²/(g·R)` = 8), quart, parois à 2 D :

| `D/dx` | pincement `t/√(D/g)` | `t/√(R/g)` | pas au pincement `/√(D/g)` | profondeur du pincement / D | base du corps / D | cavité max / D | couronne / D | mailles ; particules ; calcul |
|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 8 | 1,5067 | 2,131 | 0,027 | 1,31 | 3,01 | 1,94 | 0,21 | 21 504 ; 131 072 ; 40 s |
| 12 | 1,4645 | 2,071 | 0,018 | 1,13 | 2,93 | 1,88 | 0,20 | 72 576 ; 442 368 ; 233 s |
| 16 | **1,4738** | **2,084** | 0,014 | 1,09 | 2,95 | 1,91 | 0,31 | 172 032 ; 1 048 576 ; 816 s |
| 8, domaine entier | 1,4803 | 2,094 | 0,027 | 1,44 | 2,96 | 1,81 | 0,20 | 86 016 ; 524 288 ; 167 s |

**Le temps de pincement converge** : −2,8 % de 8 à 12, +0,6 % de 12 à 16 — dans le pas de temps (0,9 % à 16) ; la **cavité**
maximale et la **base du corps** au pincement aussi (à 2 % et 1 %). La **profondeur** du pincement converge plus lentement
(1,31 → 1,13 → 1,09 D). **La couronne ne converge pas** (0,21 → 0,20 → 0,31 D) : sans tension de surface, la maille arrête
la nappe — le constat de S320 (A312), inchangé en 3D. La vitesse la plus grande croît avec la maille (10,1 → 12,9 → 14,4 m/s) :
la fermeture du col, singulière sans air ni tension (A311).

**La vitesse, et les parois.** Mêmes mesures, publiées sans critère :

| cas | pincement `t/√(D/g)` | `t/√(R/g)` | pas `/√(D/g)` | profondeur / D | base du corps / D | cavité max / D | couronne / D | mailles ; calcul |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `Fr` = 4 (`U²/(g·R)` = 32), 8 | 1,4043 | 1,986 | 0,014 | 1,44 | 5,62 | 4,44 | 0,26 | 33 792 ; 130 s |
| `Fr` = 4, 12 | 1,4306 | 2,023 | 0,010 | 1,29 | 5,72 | 4,63 | 0,26 | 114 048 ; 809 s |
| `Fr` = 2, 8, parois à **3 D** | 1,4802 | 2,093 | 0,027 | 1,31 | 2,96 | 1,81 | 0,19 | 48 384 ; 91 s |

**Doubler la vitesse change à peine le pincement** : −2,3 % à 12 mailles (2,071 → 2,023 √(R/g)), quand la cavité, elle, devient
2,5 fois plus profonde — ce que la littérature dit du régime profond (un temps presque indépendant de la vitesse). **Les
parois** à 3 D au lieu de 2 D avancent le pincement d'un pas à 8 mailles (−1,8 %) : du même ordre que l'écart du quart au
domaine entier, et que le pas de temps ; à cette résolution, on ne les sépare pas.

## 4. Contre la mesure publiée

Pour une sphère, le pincement profond (*deep seal*) tombe à `t_p = β·√(R/g)` après l'impact, **presque indépendant de la
vitesse** ; `β` vaut de **1,72 à 2,29** selon les auteurs — Glasheen et McMahon (1996, disques : 2,285), Duclaux et al.
(2007, *J. Fluid Mech.* 591), Aristoff et Bush (2009) —, **plage lue par la recherche, dans des résumés** : les textes sont
bloqués par le réseau de cette session, ni la définition exacte de `β` de chaque auteur ni leurs nombres de Froude n'ont
été vérifiés ligne à ligne (statut : *documenté par un tiers*, pas *lu*). **Mesuré ici : 2,084** à 16 mailles par diamètre,
dans la plage, près de son haut.

Deux écarts de modèle, dits : les sphères des expériences **tombent** (et ralentissent peu), la nôtre descend à vitesse
**imposée** ; l'air des expériences a une pression, le nôtre n'en a pas — la cavité se ferme sous la seule pression
hydrostatique, ce que le régime profond suppose justement. En 2D, le cylindre de S320 se pinçait à 2,2 √(D/g) = 3,1 √(R/g) :
**la sphère se ferme plus vite** (1,47 √(D/g)), l'eau affluant de tout le tour.

## 5. Ce que ce document ne dit pas

- **Le jet, la couronne et la bulle** ne sont pas reçus : la couronne suit la maille ; la bulle enfermée s'effondre sans
  pression d'air (A311) ; le jet n'est pas mesuré.
- **Un corps reçu** : la sphère est cinématique, sans force en retour (ni flottaison ni ralentissement) ; mailles solides en
  escalier, sans faces coupées ; reflet valable pour un rayon de corps au moins égal au noyau (deux mailles).
- **Le raccord** aux colonnes (C5) et le critère de bascule (C6) : APIC tient seul le domaine ; rien sur la carte (C7).
- Les parois à 2 D de l'axe (à 3 D, le pincement tombe un pas plus tôt, −1,8 % à 8 mailles : leur effet n'est pas séparé de la résolution en temps) ; la plage publiée non lue dans ses textes (§4) ; durées d'un conteneur cloud.
