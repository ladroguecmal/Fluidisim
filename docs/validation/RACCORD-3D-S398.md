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

## 5. S399 — la bande et l'échange : le raccord presque reçu à la maille fine

2026-09-27. **C5b, deuxième part.** Critères écrits avant, dans le plan de la session.

**Reproduire** : commit `aa7644d1` ou plus récent ; `cargo test … -p water-core s399 -- --nocapture` (trois essais : surface lue
contre la zone, repos échangé et volume, onde qui traverse) ; `cargo run … --example apic3d_raccord -- <0.05|0.025>
<seul|raccord> [durée]` — ligne `APIC3D_RACCORD_S399`, 30 s par défaut ; 46 s à 11,7 min.

**Deux gestes**, les leçons de 2D portées. (1) **La reconstruction voit les colonnes** : pour une maille de la bande à portée
de la zone, chaque colonne compte `2 × 2` particules **virtuelles** par rangée, `round(2η/dx)` rangées étirées sur `[0, η]`,
images aux parois comprises — l'idée du champ de densité de Chentanez, Müller et Kim (la grille ajoute sa part à celle des
particules) ; sans elles, la frontière serait une paroi vue d'un seul côté. (2) **L'échange par le flux de la face** : une face
bande | zone est une frontière — hauteur mouillée de la colonne, le volume passé porté au **solde** de la face-maille (`f64`) ;
après l'advection, une particule entrée dans la zone est absorbée (le solde payé d'avance), un solde dû retire la particule de
la bande **la plus proche de la face**, un solde reçu en **pose** une contre la face, au sous-réseau le plus libre de sa maille.

| critère | résultat |
|---|---|
| **1** — sans zone, au bit ; toutes colonnes, les chiffres de S398 | **tenu** : les deux lignes au chiffre près ; suite **697 réussis**, 18 ignorés, zéro avertissement |
| **2** — repos, moitié particules moitié colonnes : surface lue ≤ 5 % de maille ; vitesse ≤ 1 cm/s | **surface tenue** : **2,19 %** contre la zone comme au milieu de la bande (**14,7 %** sans les virtuelles, vu échouer) ; **vitesse manquée** : **1,007 cm/s** (§ ci-dessous) |
| **3** — volume (particules + `η` + soldes) à 10⁻⁶ sur 30 s | **tenu** : 2,5·10⁻¹⁰ (5 cm), 1,5·10⁻¹⁰ (2,5 cm) |
| **4** — ballottement (1, 0), frontière au nœud, 30 s, contre APIC seul | **tenu à 2,5 cm sauf la densité** ; **à 5 cm, migration et courant de surface manqués** (tableau) |

| critère 4 (écart à APIC seul) | 5 cm | 2,5 cm |
|---|---|---|
| niveau équivalent de la bande, par 10 s (±2 mm) | +1,21 / **+2,47 / +3,14** | +0,21 / +0,63 / −0,04 |
| particules par maille, dernière colonne de la bande (8 ± 0,4) | 7,70 / 7,89 / 8,18 | **7,32 / 7,33 / 7,38** |
| saut de surface max (< 0,5 maille) | 0,105 (APIC seul 0,093) | 0,154 (APIC seul **0,785**) |
| période (1 point) | +0,60 contre +0,98 % | +0,35 contre +0,36 % |
| amortissement (1 point) | +0,35 contre +0,32 % | +0,17 contre +0,08 % |
| courant moyen sur la face (≤ 5 mm/s) | ≤ 0,9 en profondeur, **−7,0 à la surface** | ≤ 0,9, −3,3 à la surface |

**À la maille fine, la migration de masse a disparu** — elle valait +6 mm en 30 s dans le banc 2D (§16 de B10-APIC-S320) —, la
période est à 0,01 point d'APIC seul et le saut à la frontière est **plus petit** que celui qu'APIC seul a entre deux colonnes
voisines. **Ce qui manque**, attribué mais pas encore éprouvé :

- **Le repos et la migration à 5 cm.** La bande lit sa surface avec le biais de la reconstruction (−2,19 % de maille à 0,5 m,
  S389), les colonnes la leur exactement : au repos, une marche de 1,1 mm, qui excite une seiche d'un millimètre (0,5 à
  1 cm/s sur 4 s, sans décroître, aucune particule échangée) ; en mouvement, un courant moyen de surface vers la bande (−7 mm/s),
  qui la remplit. Remède proposé : que la zone lise sa surface comme la bande la lirait, `η + e(η)`, `e` le biais du réseau
  nominal (`lattice_read_error`, S389) — la masse restant exacte.
- **La densité à 2,5 cm** (7,3 par maille). Suspect : la séparation des particules ne voit pas les colonnes, et pousse à travers
  la frontière des particules que l'échange absorbe ; la tenir du côté de la bande.

**Ce que la section ne dit pas** : une seule géométrie (frontière droite, au nœud) ; ni cavité ni gerbe à la frontière ; les
particules virtuelles coûtent un balayage de plus près de la zone ; rien sur la carte.

## 6. S400 — la zone lit comme la bande : le repos et la migration reçus, la densité et le courant non

2026-09-27. **C5b, troisième session.** Les deux remèdes attribués au §5, critères de S399 **inchangés**.

**Reproduire** : commit `f01e5cf7` ou plus récent — **depuis S406, avec `APIC3D_ESSAI=1`** (`Apic3::TRIAL_S400`), la frontière de
cette section ; sans lui, celle du §7 ; `cargo test … -p water-core s399 -- --nocapture` (le repos, ligne `S399 repos`) ;
`cargo run … --example apic3d_raccord -- <0.05|0.025> <seul|raccord|colonnes> [durée]` — ligne `APIC3D_RACCORD_S399`, qui porte
désormais la **marche lue** signée moyenne (`marche_lue_mm`, bande − zone, sur `φ` des deux côtés) et les **particules par
rangée** de la dernière colonne de la bande ; `colonnes` est le témoin tout en colonnes. 45 s à 11,6 min.

**Deux gestes.** (E) **La zone lit sa surface comme la bande la lirait** : `φ = z − (η + e(η))`, `e` l'erreur de lecture d'un
réseau nominal à la même position dans la maille (`lattice_read_error` de S389, tabulée sur 32 positions à la configuration, pour
le noyau et le rayon courants) ; la masse reste `η`. Seulement **s'il y a une bande** : toute en colonnes, la zone n'a rien à
imiter et lit `η`. (F) **La séparation tenue du côté de la bande** : une particule qu'elle pousserait dans une colonne de la zone
garde sa position horizontale.

| critère (S399) | résultat |
|---|---|
| **1** — sans zone, au bit ; toutes colonnes, les chiffres de S398 | **tenu** : les deux lignes au chiffre près ; suite **697 réussis**, 18 ignorés, zéro avertissement |
| **2** — repos, moitié particules moitié colonnes : vitesse ≤ 1 cm/s | **tenu** : **2,0·10⁻⁵ m/s** (S399 : 1,007·10⁻² m/s) ; **vu échouer** sans la table, 1,007·10⁻² m/s |
| **3** — volume à 10⁻⁶ sur 30 s | **tenu** : 6,4·10⁻¹⁰ (5 cm), 9,2·10⁻¹⁰ (2,5 cm) |
| **4** — ballottement (1, 0), 30 s, contre APIC seul | **manqué** : la densité aux deux mailles, le courant de surface à 5 cm (tableau) |

| critère 4 (écart à APIC seul) | 5 cm | 2,5 cm |
|---|---|---|
| niveau de la bande, par 10 s (±2 mm) | +0,51 / +1,44 / +1,83 (S399 : +1,21 / **+2,47 / +3,14**) | +0,12 / +0,34 / −0,45 |
| particules par maille (8 ± 0,4) | **7,54** / 7,77 / 7,95 (S399 : 7,70 / 7,89 / 8,18) | **7,28 / 7,20 / 7,18** (APIC seul : 8,00 / 7,94 / 7,62) |
| saut max (< 0,5 maille) | 0,101 (seul 0,093) | 0,170 (seul 0,785) |
| période (1 point) | +0,77 contre +0,98 % | +0,27 contre +0,36 % |
| amortissement (1 point) | +0,39 contre +0,32 % | +0,23 contre +0,08 % |
| courant moyen sur la face (≤ 5 mm/s) | +0,8 en profondeur, **−6,7 à la surface** (seul : −0,5) | +0,9, −1,8 |

**Les témoins** (5 cm, 30 s) :

| montage | niveau (mm, écart à APIC seul) | densité | courant de surface | marche lue |
|---|---|---|---|---|
| (E) + (F) | +0,51 / +1,44 / +1,83 | 7,54 / 7,77 / 7,95 | −6,7 | +0,07 mm |
| sans (E) | +1,21 / +2,48 / +3,07 | 7,70 / 7,89 / 8,15 | −7,0 | +0,02 mm |
| sans (F) | +0,52 / +1,46 / +1,80 | 7,54 / 7,77 / 7,95 | −6,8 | +0,07 mm |
| toutes colonnes | — | — | **+0,5**, uniforme sur la profondeur | — |

**Ce que les témoins disent.** (1) **La migration était bien la lecture** : sans (E), la marche lue moyenne est déjà nulle
(+0,02 mm) — la pression égalise ce qu'elle voit, et la bande, qui lit 1,1 mm sous sa surface, se remplit jusqu'à lire comme
la zone ; avec (E), elle n'a plus à le faire, et la migration rentre dans le critère. (2) **Le courant n'est pas la lecture** :
il reste à −6,7 mm/s sans marche lue ; la zone seule n'en a pas (+0,5 uniforme) ; **il est au raccord**. Une circulation fermée
— l'eau entre dans la bande par la rangée du haut, en ressort par-dessous (+0,8 sur neuf rangées) — qui divise par 3,7 quand la
maille divise par deux (−6,7 puis −1,8). (3) **(F) ne change rien**, ni à 5 cm ni à 2,5 cm (sans elle : densité 7,29 / 7,18 /
7,13, courant −1,5) ; la densité ne vient pas de là. Par rangée, la dernière colonne de la bande compte, à 5 cm, 7,7 à 8,1
particules par maille en profondeur et **7,32 et 7,02 dans les deux rangées du haut** ; à 2,5 cm, 7,4 au fond, **6,8 en haut** :
le déficit est **le plus fort à la surface, contre la face**, là où passe le courant — et, à la maille fine, s'étend à toute la
profondeur, sans que la masse de la bande bouge.

**Ce qui reste**, non attribué : le courant de surface et le déficit de densité qui l'accompagne. Suspects, à éprouver par un
témoin court (5 cm, 45 s) avant tout calcul long : la face de frontière, dont la vitesse vient du transfert des seules
particules de la bande (un seul côté) ; le transport de la rangée du haut, mouillée à la hauteur de la colonne quand les
particules qui la traversent suivent la surface de la bande ; la quantité de mouvement des particules absorbées, perdue.

**Ce que la section ne dit pas** : toujours une seule géométrie (frontière droite, au nœud) ; rien sur la carte.

## 7. S406 — la face de frontière appartient à la zone : le courant de surface reçu, la densité non

2026-09-27. **C5c.** Les trois suspects du §6, chacun une option d'essai (`Apic3::TRIAL_…`), éprouvés par des témoins courts (5 cm,
30 s, ≈ 1 min) avant tout calcul long ; critères de S399 **inchangés** ; décision de l'utilisateur du jour : la priorité du
solveur passe avant la règle des maillons.

**Reproduire** : commit `2056e410` ou plus récent — **depuis S407, avec `APIC3D_ESSAI=16`** (`TRIAL_POSE_QUARTER`, la pose de
cette section) ; `cargo test … -p water-core s406 -- --nocapture` (ligne `S406 courant`, 16 s) ;
`cargo run … --example apic3d_raccord -- <0.05|0.025> <seul|raccord> 30` — ligne `APIC3D_RACCORD_S399` ; `APIC3D_ESSAI=1`, la
frontière de S400 (§6, au chiffre près) ; 1 min à 5 cm, 7 à 10 min à 2,5 cm.

**Les témoins** (5 cm, 30 s ; courant moyen sur la face, rangée du haut mouillée ; S400 : −6,7 mm/s ; APIC seul : −0,5) :

| geste éprouvé | courant | densité de la dernière colonne | niveau, écart à APIC seul |
|---|---:|---|---|
| aucun (S400) | −6,7 | 7,542 / 7,767 / 7,951 | +0,51 / +1,44 / +1,83 |
| (a) la face prend la **moyenne** du transfert de la bande et de la vitesse advectée de la zone | −3,6 | 7,577 / 7,766 / 8,081 | +0,60 / +1,47 / +2,25 |
| (b) le débit de la face mouillé à la hauteur **moyenne** de la colonne et de la bande | −7,2 | 7,544 / 7,702 / 7,890 | +0,52 / +1,27 / +1,74 |
| (c) la quantité de mouvement d'une particule absorbée **rendue** aux faces de la zone | −6,3 | 7,555 / 7,726 / 7,900 | +0,55 / +1,37 / +1,70 |
| **(a′) la face prend la seule vitesse advectée de la zone** | **−0,4** | 7,577 / 7,579 / 7,812 | +0,63 / +1,34 / +2,07 |
| (a′) + (b) | −0,4 | 7,589 / 7,623 / 7,728 | +0,62 / +1,25 / +1,85 |
| **(a′) + (c)** | **−0,6** | 7,593 / 7,617 / 7,685 | +0,62 / +1,22 / **+1,76** |
| retrait dans la plus pleine des deux dernières colonnes, avec (a′) + (c) | +1,2 | 7,519 / 7,668 / 7,850 | +0,12 / +0,58 / +0,88 |

**Ce que les témoins disent.** (1) **Le courant est porté par la face de frontière** : prise au seul transfert des particules de la
bande — un seul côté —, elle entretient la circulation ; prise pour moitié à la zone, le courant tombe de moitié ; prise à la zone,
il disparaît (−0,4 mm/s, le niveau d'APIC seul). La prédiction écrite avant désignait (b) : **manquée** — (b) aggrave. (2) Rendre la
quantité de mouvement absorbée ramène la migration au critère (+1,76 mm au lieu de +2,07). (3) **La densité ne vient d'aucun des
quatre** — ni de la face, ni du débit, ni de la quantité de mouvement, ni du lieu du retrait.

**Le défaut** (`apic3d_columns.rs`) : la face de frontière **appartient à la zone** — sa vitesse avant projection est advectée
comme ses autres faces — et une particule absorbée rend sa quantité de mouvement aux faces de la zone (1/8, réparti comme le
transfert). `TRIAL_S400` rend le §6 au chiffre près ; le défaut, l'essai (a′) + (c) au chiffre près.

| critère 4 (écart à APIC seul, 30 s) | 5 cm | 2,5 cm |
|---|---|---|
| niveau de la bande, par 10 s (±2 mm) | +0,62 / +1,22 / +1,76 | +0,14 / +0,38 / +0,45 |
| particules par maille (8 ± 0,4) | **7,593** / 7,617 / 7,685 | **7,26 / 7,24 / 7,31** (APIC seul 8,00 / 7,94 / 7,62) |
| saut max (< 0,5 maille) | 0,103 (seul 0,093) | 0,123 (seul 0,785) |
| période (1 point) | +0,70 contre +0,98 % | +0,22 contre +0,36 % |
| amortissement (1 point) | +0,52 contre +0,32 % | +0,19 contre +0,08 % |
| **courant moyen sur la face (≤ 5 mm/s)** | **≤ 0,6** (S400 : −6,7) | **≤ 1,1** (S400 : −1,8) |

**Tout le critère 4 est tenu aux deux mailles, sauf la densité** : 7,59 à 5 cm (la première tranche, de 0,007) et 7,24 à 7,31 à
2,5 cm, sur toute la profondeur de la dernière colonne, sans que la masse de la bande bouge. Critères 1 à 3 tenus (repos 2·10⁻⁵ m/s,
volume au plancher) ; suite **733 réussis**, zéro avertissement ; essai `_s406` : courant de la rangée du haut −0,78 mm/s contre
−4,78 avec la frontière de S400 (6 s).

**Ce que la section ne dit pas** : d'où vient la densité — les quatre suspects sont écartés ; restent, à éprouver : la pose, contre
la face, à la vitesse de la grille, et la séparation des particules de la dernière colonne, que les particules virtuelles des
colonnes repoussent ; une seule géométrie (frontière droite, au nœud) ; rien sur la carte.

## 8. S407 — la pose à la face : la densité reçue, le critère de S399 tenu aux deux mailles

2026-09-27. **C5d.** Condition d'une cinquième session sur A316 (§7) : un témoin court qui relève la densité à 7,6 par un geste
nommé d'avance. **D'abord le diagnostic**, sans rien changer au calcul ; **puis le geste**, écrit dans les notes avant le calcul.

**Reproduire** : commit `fcc7919a` ou plus récent ; `cargo test … -p water-core s407 -- --nocapture` (ligne `S407 pose à la face`,
16 s) ; `cargo run … --example apic3d_raccord -- <0.05|0.025> <seul|raccord> 30` — ligne `APIC3D_RACCORD_S399`, qui porte
désormais `quatre_colonnes` (densité des quatre dernières colonnes de la bande, de la frontière vers l'intérieur) et
`echange_abs_ret_pos` (particules absorbées, retirées, posées, par tranche de 10 s) ; `APIC3D_ESSAI=16` rend le §7, `=1` le §6,
au chiffre près ; 1 min à 5 cm, 7 à 11 min à 2,5 cm.

**Le diagnostic** (frontière du §7, 30 s ; APIC seul : 8,0 partout, aucun échange) :

| | 5 cm | 2,5 cm |
|---|---|---|
| densité des quatre dernières colonnes, dernière tranche | 7,69 : **8,66** : 8,08 : 8,00 | 7,31 : **8,55** : 8,36 : 7,97 |
| absorbées / retirées / posées, par tranche | 377 / 1 398 / 1 785 ; 1 / 1 591 / 1 569 ; 14 / 1 531 / 1 559 | 2 957 / 10 174 / 13 042 ; 366 / 12 021 / 12 433 ; 469 / 12 324 / 12 304 |

**La densité n'est pas perdue, elle est déplacée** : ce qui manque à la dernière colonne est dans l'avant-dernière (7,59 + 8,45 ≈ 16
à 5 cm). Et **aucune particule ne traverse la face** : tout passe par des retraits — la particule la plus proche de la face, celle
qui allait traverser — et des poses. Prédictions écrites avant : le déficit local, **tenue** ; les retraits au-dessus des poses de
10 %, **manquée** (égaux).

**Le geste, nommé avant le calcul** : la pose se fait **à la face**. Une particule posée pour un solde reçu vaut une tranche d'eau
de `dx/8` sur la face-maille ; l'eau entrée est contre la face, son centre à `dx/16`. Posée à `dx/4` (S399–S406), chaque volume entré
l'était un quart de maille trop loin : les particules posées n'étaient jamais les plus proches de la face, le reflux suivant
retirait les anciennes, et l'aller-retour de l'écoulement au nœud (±½ maille) amassait l'eau dans l'avant-dernière colonne.

| critère 4 (écart à APIC seul, 30 s), la pose à la face | 5 cm | 2,5 cm |
|---|---|---|
| niveau de la bande, par 10 s (±2 mm) | −0,15 / −0,02 / +0,46 (§7 : +1,76) | −0,29 / −0,09 / −0,82 |
| **particules par maille (8 ± 0,4)** | **7,785 / 7,847 / 8,002** (§7 : 7,59) | **7,775 / 7,825 / 7,831** (§7 : 7,24–7,31) |
| les quatre dernières colonnes, dernière tranche | 8,00 : 8,05 : 8,02 : 8,02 | 7,83 : 7,94 : 7,94 : 7,93 |
| absorbées / retirées, par tranche | 1 810 / 108 ; 1 726 / 1 ; 1 714 / 0 | 13 487 / 712 ; 12 776 / 16 ; 13 360 / 237 |
| saut max (< 0,5 maille) | 0,113 (seul 0,093) | 0,134 (seul 0,785) |
| période (1 point) | +0,59 contre +0,98 % | +0,27 contre +0,36 % |
| amortissement (1 point) | +0,43 contre +0,32 % | +0,18 contre +0,08 % |
| courant moyen sur la face (≤ 5 mm/s) | ≤ 0,2 | ≤ 0,7 |

**Tout le critère 4 de S399 est tenu, aux deux mailles, pour la première fois** ; les particules traversent de nouveau la face (plus
de 13 000 absorptions par tranche à 2,5 cm, presque aucun retrait) ; la migration tombe sous le demi-millimètre. Critères 1 à 3
tenus (volume à 10⁻⁹, repos) ; la pose à la face est le défaut ; suite **734 réussis**, zéro avertissement ; essai `_s407` (6 s) : à
la face, 1 833 absorptions pour 104 retraits ; à `dx/4`, 380 pour 1 305.

**Au critère de la campagne** (C5 : masse exacte, surface continue à la frontière sous 3 mm, le ballottement à la période d'APIC
seul à la maille fine, 30 s) : la masse et la période tiennent (0,09 point à 2,5 cm) ; **le saut max vaut 3,35 mm à 2,5 cm** (5,65 à
5 cm) — le plus grand écart, sur 30 s, entre la hauteur que lit la bande et celle de la colonne voisine —, au-dessus des 3 mm, quand
**APIC seul a 19,6 mm entre deux colonnes voisines** (4,65 à 5 cm) : la frontière est plus lisse que la bande elle-même.

**Ce que la section ne dit pas** : une seule géométrie — frontière droite, fixe, au nœud ; ni frontière qui bouge (C6), ni cavité ou
gerbe à la frontière ; la baisse de densité d'APIC seul à 2,5 cm dans la dernière tranche (7,62, toute la bande), sans raccord ;
rien sur la carte.
