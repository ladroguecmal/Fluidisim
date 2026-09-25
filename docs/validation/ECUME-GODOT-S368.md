# L'écume qui dure, dans Godot — S368

2026-09-26. Rendu dans Godot 4.4.1 ([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)), session de rendu de
l'alternance d'[ADR-191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) D3. Liste **8.4** (écume, spray,
gouttes, bulles rendus), *absente* ; l'écume de S360 a été **refusée** en R21 ([revue](REVUE-VISUELLE.md) §26) : sans
mémoire, des taches instantanées au bord lisse. Ici, la **production** sur la carte du champ d'écume dont S367 a posé la
référence ([ECUME-S367](ECUME-S367.md), [ADR-014](../adr/ADR-014-mousse-spray-bulles.md), SPEC-006 §4), et l'écume
rendue qui en naît. Machine de référence ; aucun téléchargement.

## Reproduire

- Commit de P5 de S368 ou plus récent ; Godot 4.4.1 (`<godot>`) ; données exportées comme en S360
  (`cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --export-godot`).
- `<godot> --path godot -- --controle-ecume-champ` — lignes `CONTROLE_ECUME_S368`, dix secondes : décroissance, pire
  relatif 8,6·10⁻⁶ ; advection, écart du centre 4,7·10⁻⁴ m.
- `<godot> --path godot -- --controle-ecume-couverture` — la couverture active au régime, huit secondes : κ = 3,09,
  0,405 % pour 0,421 %. `KAPPA=2.978` : la relation de S367, 0,622 %.
- `ECUME=champ <godot> --path godot -- --captures` — l'écume de S368 ; **sans `ECUME`, pas d'écume** (décision du
  2026-09-26, §5) ; `ECUME=ancienne` rend celle de S360 ; `SEQUENCE=3 POSES=plongeante` : trois images à 2 s d'intervalle.
  Images : `viewer/captures/s368/`, locales, non soumises en revue.

## En une phrase

L'écume a maintenant une vie : des moutons au bord irrégulier, à la couverture de Monahan, qui pâlissent en se trouant en
quelques secondes et laissent derrière eux une dentelle étirée le long des vagues — le champ de S367 porté sur la carte
au même pas, sa décroissance à 10⁻⁵ près.

## 1. Le champ sur la carte

`godot/ecume.comp` et `godot/ecume.gd` : 1 024 × 1 024 texels de 0,25 m (256 m) autour de la caméra ; deux canaux
(actif, résiduel) en RGBA32F, deux images alternées ; le pas de `ecume.rs` à l'identique — advection semi-lagrangienne
par la vitesse orbitale de la bande (64 composantes de l'export), décroissance exacte à deux canaux, sources aux crêtes
les plus accélérées. L'état interne reste en f32 : la décroissance pas à pas ne tiendrait pas en demi-précision (SPEC-006
§4 publie en RG16F ; ici, rien n'est encore publié hors du rendu). À chaque recentrage, **60 s de passé** sont rejouées
au pas de 0,1 s — B est analytique — : l'écume de la première image est celle d'un régime établi ; puis deux demi-pas par
image.

| critère 1, écrit avant | mesure |
|---|---|
| décroissance d'un champ uniforme, 600 pas de 1/60 s, contre le fermé, 10⁻⁵ relatif | **8,6·10⁻⁶** (après correction, ci-dessous) |
| advection d'une bosse gaussienne, cent pas de 0,1 s à (0,37 ; −0,21) m/s, centre à 1 cm | **0,47 mm** ; masse à 8·10⁻⁷ |

**Défaut trouvé, corrigé.** Premier passage : **2,5·10⁻⁵** sur le résiduel. Au pas de 1/60 s, le coefficient de transfert
`e^(−λr·dt) − e^(−λa·dt)` soustrait deux nombres voisins de 1 : en f32, 3·10⁻⁵ relatif, à chaque pas. Les trois
coefficients du pas se calculent désormais en double par le script ; la référence de S367 (pas de 0,25 s) n'y était pas
exposée.

## 2. La couverture

Seuil de déferlement `κ·σ_a` (S367) sur la bande de l'export (`σ_a/g` = 0,0884) ; cible, `couverture_monahan` de
l'export : **0,421 %** (U10 = 7,8 m/s). Couverture active relue sur la carte au régime, 40 s :

| κ | couverture | rapport |
|---:|---:|---:|
| 2,978 (relation de S367) | 0,622 % | 1,48 |
| 3,00 | 0,573 % | 1,36 |
| **3,09** (prédit par la pente de S367) | **0,405 %** | **0,96** |
| 3,20 | 0,260 % | 0,62 |

La bande de l'afficheur n'est pas celle du cœur : κ se recale, comme S367 l'avait annoncé ; la pente mesurée en S367
donne le bon κ du premier coup.

## 3. L'écume rendue

`eau.gdshader` lit le champ là où l'eau est dans le monde. **L'actif** : écume vive, blanche (réflectance 0,55), bord
irrégulier par un bruit à l'échelle des flocons (0,3 et 1,2 m) ; montée large, 0,15 à 0,75 : le mouton **pâlit en se
trouant** au lieu de s'éteindre au seuil ½ (vu sur la première séquence), sa mi-opacité restant au seuil de la
couverture. **Le résiduel** : une dentelle translucide. Trois motifs essayés, **deux rejetés sur les images** :

1. les crêtes d'un bruit de valeurs — un **labyrinthe rectiligne** qui suit la grille du bruit, et le résiduel (moyen
   0,40, S367) voilant toute la mer ;
2. les bords de cellules de Worley (1996) — une **résille de verre fêlé**, régulière, partout ;
3. **retenu** : les mêmes cellules déformées par un bruit (±0,3 m), étirées 2,2 fois le long des vagues — direction
   `Σ a²·k̂` de la bande, celle des traînées de S367 —, rompues par un masque de 3,5 m, visibles là où le résiduel est
   dense (0,3 à 0,9) : derrière les moutons récents, pas ailleurs.

Niveau de détail : quand le pixel couvre le motif, chacun tend vers sa moyenne. Séquence (plongeante, 0, 2, 4 s) : le
mouton blanc, puis troué et pâli, puis translucide dans sa dentelle qui dérive.

## 4. Limites

- Un seul niveau de champ, 256 m autour de la caméra, fondu à ses bords : au-delà, pas d'écume (SPEC-006 prévoit des
  cascades) ; un recentrage rejoue le passé, sans raccord avec l'écume d'avant.
- Sources de B seules (la bande) ; ni W, ni δ, ni le vent ; ni spray, ni gouttes, ni bulles (le reste de 8.4).
- Le résiduel reste dense (transfert 1 : 1) ; sa visibilité est réglée au rendu — seuil et translucidité à juger.
- Motifs de bruit : aucune texture photographique d'écume ; coût non mesuré.

## 5. Décision de l'utilisateur, en cours de session

2026-09-26, pendant P5 : *« Oublie l'ecume sauf si tu trouve des photos qui informe de la forme et couleur et position dans la topologie »*. **Cherchée aussitôt** : la série de l'échelle de Beaufort de la NOAA (domaine public,
Wikimedia Commons, *Beaufort scale 4.jpg* — la force de notre mer, U10 = 7,8 m/s) : 400 × 386 pixels, une mer sombre où
les moutons se devinent à peine — **elle ne renseigne ni la forme, ni la couleur, ni la place de l'écume sur la vague**.
Donc : l'écume **s'arrête** ; le champ et son rendu restent dans le code, mesurés (§1–2), **éteints par défaut**
(`ECUME=champ` les rallume) ; aucune revue R26. Reprise si des photographies qui renseignent ces trois choses se
trouvent.
