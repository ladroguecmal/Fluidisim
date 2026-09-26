# La caméra à demi immergée — S371

2026-09-26. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)), session de rendu de
l'alternance d'[ADR-191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) D3. Liste **8.6** (vue sous-marine et
passage de la surface), partielle depuis [S365](SOUS-MARIN-S365.md) ; [ADR-019](../adr/ADR-019-vue-sous-marine.md) §6,
« à traiter explicitement comme un cas nommé » ; banc B11. Machine de référence ; aucun téléchargement.

## Reproduire

- Commit de P6 de S371 (`6f0b7d69`) ou plus récent ; Godot 4.4.1 (`<godot>`) ; données exportées comme en S360
  (`cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`).
- **Le contrôle de la ligne** : `<godot> --path godot -- --controle-ligne-eau` — une centaine de secondes, lignes
  `CONTROLE_LIGNE_EAU_S371` : critère 1, `pire_px=0.078` ; critère 2, caméra fixe (`ligne_visible_au_milieu=42`,
  `deplacement_max_px_par_image=60.0`) et flottante (`60`, `0.2`), `pixels_mal_classes_hors_2px=0` les deux. Avec
  `MILIEU=exact` (Newton par pixel) : critère 1, `pire_px=0.020`.
- **Le coût** : `<godot> --path godot -- --cout-demi --cote` — une trentaine de secondes, lignes `COUT_DEMI_S371` :
  surcoût GPU −0,001 ms (`demi`) et 0,028 ms (`demi_dessous`) sur 1,45 et 1,55 ms ; `cpu_immersion_ms=0.473`. Avec
  `MILIEU=exact` : 4,66 et 5,10 ms.
- **Les images de R26** : `POSES=demi,demi_soleil,demi_dessus,demi_dessous <godot> --path godot -- --captures --cote` ;
  `MENISQUE=0` et `DEMI=0` (la bascule d'un bloc de S365) pour les témoins. Copies : `viewer/captures/s371/`.
- **Non-régression** : `PROFONDEUR=s365 POSES=sous_eau,sous_eau_zenith,proche <godot> --path godot -- --captures --cote`
  et `POSES=proche,rasante <godot> --path godot -- --captures`, contre les mêmes rendus au commit `d66ac1f1` : identiques
  au bit, sauf 33 pixels à ±1 au zénith sous l'eau (§4). **La brume** : jusqu'au commit `2bb37060`, `BRUME=1` la garde en
  mode demi (§5) ; **depuis S373**, l'eau et le fond y passent à leur variante `_demi`, qui écrit sa propre brume :
  `BRUME=1` ne rend plus la mesure du §5 (la rejouer à `2bb37060`) ; voir §10.

## En une phrase

Quand la ligne d'eau traverse l'objectif, chaque pixel est vu de l'eau ou de l'air selon que le point où son rayon
traverse le plan proche est sous la surface ou au-dessus ; la ligne rendue tombe à 0,08 pixel de l'intersection
analytique, aucun pixel n'est mal classé sur soixante images, pour 0,03 ms GPU au plus ; un ménisque calé sur une
photographie la souligne.

## 1. Le milieu, décidé à l'objectif

Jusqu'ici ([S365](SOUS-MARIN-S365.md) §3), `mer.gd` disait si l'œil était dans l'eau, et **tout le cadre basculait d'un
bloc** — faux dès que la ligne d'eau traverse l'image (`viewer/captures/s371/avant_d_un_bloc.png` : l'œil au ras de l'eau,
le ciel entier rendu comme de l'eau).

**Le milieu se décide par pixel, à l'objectif** : le point `p` où le rayon du pixel traverse le plan proche, sous ou
au-dessus de la surface de B à son aplomb. Rien avant le plan proche n'est rendu ; la ligne d'eau sur l'objectif est
l'intersection de ce plan et de la surface. `surface_b.gdshaderinc`, une source pour l'eau, le fond et le ciel :
`milieu_du_pixel` rend la part d'eau (transition de 0,6 % de la hauteur d'image, §6), la profondeur de l'objectif, et la
distance signée à la ligne en pixels, `s/|∇s|` pris d'un pixel au suivant sur le plan proche. Dans la transition, les deux
vues se mélangent. La surface est celle du maillage : la bande de B, déplacement horizontal compris — l'aplomb de `p` est
un point **lagrangien** `q` tel que `q + d(q) = p`, trouvé par Newton — et second ordre de Tayfun.

**Quand le mode s'allume.** Hors de la zone où le plan proche peut couper la surface, le milieu reste d'un bloc et les
rendus sont ceux d'avant. **Trouvé** : la borne de |η| de la bande (`Σ|a|` et Tayfun) vaut **6,43 m** pour Hs 2,5 m ; la
pose proche, à 4 m, passait en mode demi (brume éteinte, jusqu'à 13 niveaux d'écart). **Test retenu, rigoureux et serré** :
la surface exacte au centre du plan proche, plus sa variation sur l'étendue horizontale `R` du plan, bornée par
`pente·R/(1 − G)` (le point lagrangien bouge d'au plus `R/(1 − G)` si `G = Σ a·k < 1`) ; ici `G` = 0,726, pente bornée
1,238, `R` = 0,1 m : **0,43 m**. Le milieu d'un bloc et la profondeur de la caméra se prennent désormais au même endroit
— l'objectif, sur la surface exacte — et non plus à l'œil sur la bande sans déplacement (§7).

Le fond relu par réfraction depuis la surface vue d'au-dessus n'est pris que s'il a lui-même été vu de l'air ; sinon, il
porte déjà la colonne d'eau vue de l'objectif, et le fond non réfracté du pixel le remplace. Le ciel reçoit la position de
la caméra en uniforme : lire `POSITION` le ferait passer en mise à jour continue et changerait le cube de radiance.

## 2. Le contrôle

`--controle-ligne-eau` : l'eau, le fond et le ciel rendent la part d'eau (`controle_milieu`), tonalité linéaire, sans
halo ; la ligne rendue — le passage à ½, interpolé entre deux rangées — contre l'intersection de la surface et du plan
proche **recalculée en double dans `mer.gd`** (`surface_exacte`), par balayage de la colonne puis dichotomie à 10⁻³ px.

**Critères, écrits avant** (ADR-019 §6 : *« une ligne de flottaison qui scintille »* est le défaut à éviter) : (1) ≤ 1
pixel sur au moins 16 colonnes et trois états ; (2) sur 60 images consécutives à 1/60 s, aucun pixel dont le milieu
change à plus de 2 pixels de la ligne analytique — écrit en contrôle par image : aucun pixel dont le milieu rendu diffère
de l'analytique à plus de 2 pixels de la ligne.

| critère | cas | Newton par pixel | ordre 2 (§3) |
|---|---|---:|---:|
| 1 | pose `demi`, t₀ | 0,014 px | 0,031 px |
| 1 | la même, t₀ + 2,3 s | 0,020 px | 0,020 px |
| 1 | autre position, visée oblique, t₀ + 5,7 s | 0,010 px | **0,078 px** |
| 1 | roulis de 0,4 rad (ligne en diagonale), t₀ + 1,1 s | 0,015 px | 0,028 px |
| 2 | caméra fixe, 60 images : la ligne visible au milieu 42 fois, jusqu'à **60 px par image** | 0 mal classé | **0 mal classé** |
| 2 | caméra flottante (recalée sur la surface, comme la tête d'un nageur), 60 images, 0,2 px par image | 0 mal classé | **0 mal classé** |

**Écart au critère écrit** : « trois états de mer » est tenu par quatre instants et positions d'**un seul** état exporté
(Hs 2,5 m) — l'export n'en porte qu'un. La caméra fixe au niveau moyen voit la mer monter et descendre de part et d'autre
d'un plan proche de ±4,7 cm : la ligne traverse le cadre, jusqu'à 60 pixels d'une image à la suivante (0,47 m/s à
l'objectif). C'est le phénomène, pas un scintillement : chaque image reste juste.

## 3. Le coût, et la surface à l'ordre 2

`--cout-demi` : le temps GPU du rendu entier mesuré par Godot, médiane de 240 images, pose fixe, milieu par pixel contre
le même cadre d'un bloc.

| pose | d'un bloc | Newton par pixel | ordre 2 |
|---|---:|---:|---:|
| `demi` | 1,452 ms | 6,129 ms (**+4,66**) | 1,449 ms (−0,001) |
| `demi_dessous` | 1,557 ms | 6,660 ms (**+5,10**) | 1,577 ms (+0,028) |

**Newton par pixel** — quatre évaluations de 64 composantes, jusqu'à trois fois par pixel (fond, eau, réfraction) —
coûtait trois fois le cadre, au-delà du profil de l'eau (≤ 4 ms GPU, ADR-174 D3). **Le plan proche ne mesure que ±10 cm à
50°** : la surface y est un paraboloïde. `surface_ordre2` (`mer.gd`) calcule une fois par image, en double, au centre du
plan proche, la hauteur, la pente et la **hessienne eulériennes** : en `q`, `∇²η` (premier ordre et second de Tayfun,
`k̄·(sa·saᵀ + e2·Hs − sq·sqᵀ − qd·Hc)`), puis `∇²η_E = J⁻¹·(∇²η − C)·J⁻¹` avec
`C = −Σ a·k²·cos·(n·u)·u·uᵀ`, `n = ∇η_E` — la dérivée de `J⁻¹`. Le nuanceur n'évalue que le paraboloïde, en coordonnées
relatives au centre (aucun grand nombre en f32, I-08). **Borne du terme d'ordre 3** : `Σ a·k³·r³/6` = **0,37 pixel** au
coin du cadre à 50° ; 2,4 à 100°, 5,1 à 120° (§8). CPU : `immersion()` 0,47 ms par image en GDScript, en mode demi
seulement. Image contre Newton : les écarts ne tiennent qu'à la ligne, ≤ 10 niveaux sauf deux pixels.

## 4. Deux défauts vus sur les images, corrigés

Pose `demi_dessous` (4 cm sous la surface, contre-plongée de 7°) : le bord de la fenêtre de Snell, vu de tout près, en
**escalier**.

- **Supposé d'abord** : l'éventail central de la grille polaire, un seul anneau de 0,25 m vu à 4 cm. Cent anneaux ajoutés
  sous R_MIN au même pas logarithmique (3 % du rayon, jusqu'à 1,2 cm), les anneaux de R_MIN à R_MAX inchangés — l'escalier
  **reste**. Gardé : il supprime l'éventail vu au zénith sous l'eau (33 pixels à ±1, les seuls qui changent parmi les
  témoins).
- **La cause** (`DETAIL=0` l'efface) : la surface fine par FFT lue en **bilinéaire** — un texel de la cascade de 4 m
  (1,6 cm) couvre des dizaines de pixels, et une pente bilinéaire, continue mais anguleuse, dessine le seuil de la
  réflexion totale en polygones. Lecture en **B-spline bicubique** (Sigg et Hadwiger 2005, quatre lectures bilinéaires)
  au-delà d'un grossissement de 32, fondue jusqu'à 64 : la surface à moins de ≈ 3 m pour la cascade de 32 m, ≈ 0,4 m
  pour celle de 4 m. **Impasse** : au seuil 4, les poses validées changeaient, jusqu'à 69 niveaux — la cascade de 32 m y
  est déjà grossie de 5 à 15 fois, en bilinéaire (R21 l'a validée ainsi) ; ce régime reste à examiner.

## 5. La brume

La brume de Godot lit le cube de radiance et se calcule hors du nuanceur ; écrire `FOG` la supprime entièrement pour le
matériau (`scene_forward_clustered.glsl`, 4.4-stable, `CUSTOM_FOG_USED`) : **elle ne se règle pas par pixel**. Décision :
éteinte dès que la caméra est à demi immergée, comme sous l'eau. Mesurée (`BRUME=1` la garde, pose `demi`, de part et
d'autre de la ligne à 3 pixels près) :

| | pixels qui changent | pire | p99,9 |
|---|---:|---:|---:|
| au-dessus de la ligne | 0,8 % | 17 niveaux | 2 |
| au-dessous | 12 % | **48 niveaux** | 35 |

Au ras de l'eau, la mer vue d'au-dessus est rasante et reflète l'horizon, que la brume change à peine ; au-dessous, elle
voilait la face inférieure lointaine de la surface — le défaut de S366, évité.

## 6. Les références, et le ménisque

Photographies « dessus-dessous » libres, lues dans le navigateur et chiffrées sur un canevas (luminance sRGB décodée), rien
téléchargé :

| | source, licence | ce qu'elle montre |
|---|---|---|
| **A** | *Reef Scenic Split Shot in the Bird's Head Seascape*, Jones/Shimlock (Secret Sea Visions), Wikimedia Commons, CC BY-SA 4.0 ; 1 000 × 670, dôme, soleil | de l'air vers l'eau : la surface vue en rasant, tassée sur ≈ 12 px ; **un trait sombre** de 1 à 2 px (0,27 fois ses voisins) ; **une bande claire** de 5 à 7 px (1,38 fois l'air qui la précède, la plus lumineuse de la région) ; le passage à l'eau en ≈ 3 px ; l'eau uniforme, ≈ 0,5 fois la surface vue d'au-dessus |
| **B** | *Over-under with flippers*, Gerry Thomasen, Wikimedia Commons, CC BY 2.0 ; 2 592 × 1 944, compact, rivière à l'ombre | passage continu sur ≈ 24 px (**1,2 %** de la hauteur), sans bande ; l'eau 40 fois plus sombre que la roche |

Communs : une ligne **continue et lisse**, une transition d'environ 1 % de la hauteur — la surface qui touche le hublot
est hors de toute mise au point. **Retenu** : transition de 0,6 % (entre 0,45 % pour A et 1,2 % pour B) ; **hublot en
dôme**, déclaré — pas de grossissement de la moitié immergée ; **le ménisque de A** — l'eau mouille le hublot et y monte
au-dessus de son niveau, **côté air** —, bande claire ×1,38 sur 0,9 % de la hauteur, trait sombre ×0,27 sur 0,25 %, sur
l'eau et le ciel (le fond n'est vu côté air qu'à travers la surface) ; `MENISQUE=0` l'éteint, ce que montre B.

## 7. Non-régression

Au-dessus de l'eau (proche, rasante, proche de la scène côtière) : **identiques au bit** au commit de départ. Sous l'eau,
avec `PROFONDEUR=s365` : identiques au bit, sauf les 33 pixels du zénith (§4). Sans elle, la profondeur est prise à
l'objectif sur la surface exacte — 2,949 m au lieu de 2,969 m (pose `sous_eau`), 4,115 au lieu de 4,233 (zénith, dont
0,1 m d'objectif) — : **≤ 1 niveau** sur 5 à 9 % des pixels. Correction voulue : l'ancienne estimation, sans déplacement
horizontal, pouvait se tromper de côté à moins d'un mètre de la surface.

## 8. Limites

- **Un seul état de mer** exporté (Hs 2,5 m) ; une scène fixe, sans objet qui traverse la ligne.
- **Champ large** : l'ordre 2 est borné par 2,4 pixels à 100° et 5,1 à 120° (au coin, dans le pire alignement) ;
  `MILIEU=exact` reste exact partout, à 5 ms.
- La brume est éteinte **au-dessus** de la ligne aussi quand la caméra est à demi immergée (17 niveaux au pire) : une
  perspective aérienne calculée par nos nuanceurs la rendrait réglable par pixel. *Levée en S373 (§10).*
- Le maillage près de l'objectif interpole la surface ; la ligne, elle, est analytique : un pixel peut voir un fragment de
  surface qui n'est pas exactement celui du milieu décidé, dans la transition.
- Le ménisque est calé sur **une** photographie, prise de vue inconnue ; ni gouttes sur la partie émergée du hublot, ni
  lame d'eau qui ruisselle, ni aberration chromatique au bord de la fenêtre.
- Les deux mixages audio d'ADR-019 §6 : à la fin, avec le son (ADR-197 D5).
- La cascade de 32 m en bilinéaire aux grossissements de 5 à 15 (§4).

## 9. Revue

R26 ([revue](REVUE-VISUELLE.md) §31).

## 10. S373 — la brume réglée par pixel

**Reproduire** : commit de P3 de S373 (`6d095280`) ou plus récent ; `POSES=proche,rasante,reference,haute,plongeante,demi
<godot> --path godot -- --captures` et `POSES=proche,sous_eau,sous_eau_zenith,demi … --captures --cote`, contre les mêmes
rendus au commit `2bb37060` ; la brume de Godot en mode demi, pour comparer : `BRUME=1 POSES=demi` à `2bb37060`.

**Ce qui est fait.** `brume_air` (`ciel.gdshaderinc`) réécrit la perspective aérienne de Godot (`fog_process`,
4.4-stable) : quantité `1 − exp(−ρ·d)`, ρ = 0,00012, `d` la distance de l'œil au fragment ; couleur, la radiance du ciel
dans la direction de visée ; **multipliée par la part d'air du pixel** (`milieu_du_pixel`), rendue par la sortie `FOG`.

**Critères, écrits avant** : (1) poses au-dessus contre les rendus d'avant, p99,9 ≤ 2 niveaux et pire ≤ 8 ; (2) pose
`demi`, côté air ≤ 8 niveaux de la brume de Godot, côté eau au bit ; (3) sous l'eau, au bit.

**Critère 1 manqué par la brume réécrite partout** :

| pose | p99,9 | pire | pixels touchés |
|---|---:|---:|---:|
| proche | 6 | 10 | 22 % |
| rasante | 5 | 8 | 12 % |
| référence | 6 | 11 | 30 % |
| haute | 9 | 10 | 72 % |
| plongeante | 2 | 2 | 37 % |

La cause, relue : Godot lit son cube de radiance au niveau `mip = mix(1/MAX, 1, 1 − (|z| − near)/(far − near))` — pour
tout fragment bien plus proche que le plan lointain (20 km), **le niveau le plus flou**, une moyenne diffuse du ciel ; la
nôtre prend la radiance de l'horizon dans la direction. Le seuil ne se relève pas. **Retenu** : deux variantes de l'eau
et du fond (`eau.gdshader`, `eau_demi.gdshader` ; `sol` de même ; corps commun dans `*.gdshaderinc`, `#define
BRUME_PAR_PIXEL`), que `mer.gd` échange quand le mode demi change. **La brume du moteur partout où elle peut servir ; la
nôtre seulement à demi immergée**, où celle du moteur ne peut pas se régler par pixel.

| critère | mesure | |
|---|---|---|
| 1 | six poses au-dessus (dont la côtière) | **identiques au bit** |
| 2 | `demi`, côté air, contre la brume de Godot | **7 niveaux** au pire (p99,9 = 3) ; S371 : 15 |
| 2 | `demi`, côté eau, contre S371 | **identique au bit** |
| 3 | `sous_eau`, `sous_eau_zenith` | **identiques au bit** |

`--controle-ligne-eau` inchangé. **Limites** : à l'entrée du mode demi, la brume de l'air change de modèle — jusqu'à 7
niveaux sur l'horizon lointain — et la variante se compile au premier passage (un à-coup possible, non mesuré) ; la brume
réécrite n'est pas celle de Godot — la remplacer partout serait un choix visuel à soumettre (elle est plus bleue : la
radiance de l'horizon plutôt qu'une moyenne diffuse).

