# La bande étroite en profondeur — le fond de la bande, fixe — S413

2026-09-27. **C6c-1** de la campagne du solveur volumique 3D ([ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) §4 ;
décision de l'utilisateur, [ADR-211](../adr/ADR-211-les-trucages-retenus.md) D1 : *« est il intéréssant de simuler les billes en
dessous en profondeur, car on ne les voit pas »*). Référence CPU (`apic3d.rs`, `apic3d_columns.rs`), au poste.

## Reproduire

- Commit `@P8@` ou plus récent.
- Essais : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s413 -- --nocapture` — cinq
  essais, 5 s ; lignes `S413 …` : §2.
- Le ballottement : `APIC3D_FOND=4 cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example
  apic3d_raccord -- <0.05|0.025> <raccord|seul> 30` — ligne `APIC3D_RACCORD_S399` ; sans la variable, S408 au caractère près ;
  40 s à 5 cm, @DUREE25@ à 2,5 cm : §3.
- Sans fond, au caractère près : `APIC3D_BASCULE= … --example apic3d_b10 -- 2 8` (BASCULE-S408 §3.1) et `APIC3D_BASCULE= …
  --example apic3d_deferlement -- 40 4` (BASCULE-S408 §6) : §1.

## En une phrase

Une colonne de la bande garde désormais **l'eau profonde sur la grille** — un contenant de mailles pleines sous un fond posé à
quatre mailles sous la surface — et **les particules seulement au-dessus** ; au repos rien ne bouge (8,7·10⁻⁶ m/s), la masse se
compte au bit, la densité tient à la frontière du fond, et sur un ballottement de 30 s la période et l'amortissement restent ceux
d'APIC seul à moins d'un point, avec **2,4 à 4,7 fois moins de particules** et un calcul **2,5 fois plus court**.

## 1. La construction

**Le fond** (`floor`, par colonne de particules, **arrondi à une face de maille** ; `set_band_floor`) : les mailles dont le centre
est dessous sont **à la grille** (`grid_cell`), comme une maille de la zone des colonnes :

- **étiquettes** : eau, `φ = z − fond` ; la reconstruction les saute et compte, au-dessus, les **particules virtuelles** de la part
  eulérienne — la sienne et celles des voisines (`virtual_column_sums`, de `[0, η]` étendu à `[0, fond]`) ;
- **vitesses** : une face-maille `u` ou `v` est à la grille si l'une de ses deux mailles l'est (la règle de S406, maille par maille) ;
  une face `w` si la maille au-dessous l'est — la face du fond comprise ; elles sont advectées au pied de la caractéristique ;
- **la masse** : la part eulérienne est un **contenant plein et fixe** ; tout débit qui y entre ou en sort — d'une part eulérienne
  voisine, d'une colonne de la zone, des particules d'à côté — charge le **solde vertical** de sa colonne ; la part d'une face entre
  une maille à la grille et une maille de particules charge aussi le **solde latéral** de la face-maille ;
- **l'échange** : une particule qui passe sous le fond est **absorbée** et paie le solde vertical (sa quantité de mouvement rendue
  aux faces à la grille) ; un solde vertical dû retire la particule la plus basse au-dessus du fond, reçu en **pose** une à
  `fond + dx/16` (la pose à la face de S407) ; la frontière latérale se règle maille par maille.

Chaque geste vaut `dx³/8` et chaque volume est compté des deux côtés : **la masse se compte au bit**. Le fond ne bouge pas en
C6c-1 ; la bascule refuse une colonne à fond (C6c-2 placera le fond). Choix d'implémentation plus simple que ce qu'ADR-212 D2
décrivait (le fond transporté) : [note datée d'ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md).

**Critère 1 — sans fond, au bit** : les 25 essais d'APIC 3D de S388 à S410 tels quels ; le raccord à 5 cm rend S408 au caractère
près (densité 7,790 / 7,868 / 8,090, saut 0,113, période +0,61 %, amortissement +0,45 %) ; **B10** à bande dynamique (défauts) :
pincement 1,4797, part 0,225, 29 120 particules, volume 1,09·10⁻¹², une bascule — S408 ; **la vague** de S410 (défauts) :
retournement 0,7020, impact 1,3124 à 4,225, part 0,682, 92 104 particules — S410.

## 2. Critère 2 — le repos, et l'onde

Bassin de 20 × 8 × 20 à 5 cm, 0,5 m d'eau, fond à 0,3 m (quatre mailles sous la surface), 2 s (`_s413`) :

| montage | vitesse max | volume | densité au-dessus du fond (contre lui) | particules |
|---|---:|---:|---|---|
| tout en bande étroite | **8,7·10⁻⁶ m/s** | **0** | 8,000 (8,000) | 5 120 — la bande pleine en porterait 12 800 |
| mi-zone, mi-bande étroite | 2,1·10⁻⁵ m/s | 1,3·10⁻¹⁵ | 8,000 (8,000) | 2 560 |
| l'onde de 2 cm de S399, mi-zone | 0,18 m/s (l'onde) | −4,4·10⁻¹⁶ | 7,733 (8,000) | 2 736 → 2 456 |

Seuils (écrits avant) : 1 cm/s, 10⁻⁹, 8 ± 0,4 — **tenus**. La lecture de la surface au-dessus d'un fond est **au bit** celle de la
bande pleine (0,3989 m) : les particules virtuelles tombent sur le réseau nominal.

## 3. Critère 3 — le ballottement de 30 s

Le banc du raccord (S399) : cuve de 2 × 0,2 m, 0,5 m d'eau, mode (1, 0) de 2 cm, frontière zone | bande au nœud ; `APIC3D_FOND=4`
pose le fond à quatre mailles sous le creux (0,30 m à 5 cm). Écart de période au mode exact et amortissement par période, contre
APIC seul :

@TABLE3@

## 4. Ce que ce document ne dit pas

- **Le fond ne bouge pas** : placé à la main, uniforme. C6c-2 le placera sous la surface la plus basse de chaque colonne, avec son
  hystérésis ; B10 et la vague de Chen l'éprouveront (critères 4 et 5 d'ADR-212).
- **La projection garde toutes les mailles** : le gain est en particules et en temps de particules, pas en pression.
- Un fond sous la surface d'une colonne voisine de la zone plus haute que cette surface (une frontière très raide) n'est pas traité
  finement : ces rangées-là n'échangent rien.
