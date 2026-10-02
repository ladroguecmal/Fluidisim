# La surface continue — S450 ([ADR-211](../adr/ADR-211-les-trucages-retenus.md) D2)

2026-10-02, au poste, sans carte. **L'exigence** : l'utilisateur juge C10 sur une surface sans interstice, dont seuls les jets se
détachent — jamais sur des particules (R34).

## Reproduire

`cargo build --release -p water-core --example surface_continue`, puis, depuis la racine du dépôt,
`code/target/release/examples/surface_continue captures/s450` (28 s) et `python outils/apercu_ppm.py captures/s450/<image>.ppm`
(ADR-124 : les images restent locales). Attendu : `aretes_ouvertes_max=0`, `saut_raccord_corrige_max_sur_dx=0.337`.

## Ce qui est fait

Le champ `φ` d'`Apic3` est **le champ unique de l'eau** — `z − η` dans la zone des colonnes (avec la lecture de la bande, S400), la
reconstruction des particules dans la bande, `z − fond` sous le fond (`columns_label`). Le banc `surface_continue` en extrait
l'isosurface `φ = 0` d'un seul tenant, par **tétraèdres marchants** (six par cube autour de la grande diagonale, un découpage cohérent
d'un cube à l'autre — étanche par construction, sans table d'ambiguïté), sur les centres des mailles, une couche réfléchie aux plans de
symétrie du quart ; puis un **rendu logiciel** — perspective, tampon de profondeur, ombrage de Lambert et reflet du ciel (Fresnel de
Schlick), la sphère en gris — en PPM. Le cas : B10 en quart (une sphère de 0,4 m entre dans l'eau, `Fr` = 2, `D/dx` = 8), la zone des
colonnes et la bascule en bande étroite (`fond` = 4).

## Mesures

| critère (écrit avant) | mesure | |
|---|---|---|
| (1) aucune arête ouverte à l'intérieur | **0** aux quatre instants | tenu |
| (2) au raccord bande \| colonnes, la hauteur lue sur `φ` ne saute pas de plus d'un quart de maille | **0,32** maille brut, **0,34** corrigé de la pente locale | **manqué** |
| (3) quatre images, jugées par l'utilisateur | `b10_t0.5`, `t1.0`, `t2.0`, `t3.0` | à juger |
| (4) le banc sous deux minutes | 28 s | tenu |

**Ce qui se voit.** La surface est d'un seul tenant : le cratère autour de la sphère (t = 0,5 et 1), puis le jet de Worthington (t = 2
et 3). Mais **des marches carrées** dessinent la frontière bande | colonnes autour de la zone agitée — le saut de (2), un tiers de
maille : la surface reconstruite des particules et `η` des colonnes ne se rejoignent pas exactement. L'ombrage est à facettes (une normale
par triangle). **Ce qui reste** : raccorder `φ` à la frontière pour le rendu (un fondu sur deux colonnes), des normales lissées — puis le
rendu en direct sur la carte, et le jugement de l'utilisateur.

## S451 — après R36 : normales lissées, `φ` fondu au raccord

2026-10-02. **R36** (l'utilisateur) : *« Alors le problème est que l'on voit des divisions faces plane »* — deux causes : l'ombrage à
facettes, les marches au raccord. **Fait** au banc (le calcul n'est pas touché) : **le champ rendu** — `φ` fondu au raccord, deux passes
d'une moyenne horizontale 3 × 3 sur les colonnes à moins de deux mailles d'une frontière bande | colonnes (`SURFACE_SANS_FONDU=1` rend
`φ` tel quel) ; **des normales lissées** — le gradient de `φ` aux points de la grille, interpolé le long de l'arête au sommet, puis aux
coordonnées barycentriques **par pixel** (Phong). **Reproduire** : comme ci-dessus, sortie `captures/s451` ; attendu
`saut_raccord_corrige_max_sur_dx=0.083`, `aretes_ouvertes_max=0`.

| critère (écrit avant) | S450 | **S451** |
|---|---:|---:|
| saut au raccord, corrigé de la pente (au plus ¼ de maille) | 0,337 | **0,083** — tenu |
| arêtes ouvertes à l'intérieur | 0 | **0** — tenu |
| ni facettes ni marches visibles | R36 : non | **R37 : au jugement de l'utilisateur** |

Les images `captures/s451/b10_t{0.5,1.0,2.0,3.0}.png` : le cratère et le jet de Worthington lisses, sans les marches carrées.

*2026-10-02, S452 — l'utilisateur* : *« Je valide le render »* — **R37 reçu**.

## S452 — en direct, sur la carte

2026-10-02. Le rendu reçu (R37), porté dans l'afficheur sur le device de la carte de la bande (`ApicCarte`), sans retour au CPU :
`viewer/src/surface_carte.{rs,wgsl}`. **Le fondu** — deux passes de calcul, le `champ_rendu` de S451 maille par maille, dans le
même ordre des sommes ; **le lancer de rayons** — une passe de fragments : un rayon par pixel dans le champ fondu (le quart reflété,
trilinéaire aux centres des mailles), le premier changement de signe par pas d'une demi-maille puis huit bissections, la normale
lissée (le gradient de `φ` aux points de la grille, interpolé — celui de S451), l'ombrage de R37, la sphère en intersection analytique.
Pas de maillage : l'isosurface trilinéaire remplace les tétraèdres marchants.

**Reproduire** : `cargo build --release` dans `viewer/`, puis, depuis la racine, `viewer/target/release/water-viewer.exe
--surface-carte` (68 s ; `SORTIE`, `FR`, `ND`) et `python outils/apercu_ppm.py captures/s452/<image>.ppm`. Attendu :
`ecart_fondu_max=7.153e-7`, `rendu_carte_ms_mediane≈0.23`.

| critère (écrit avant) | mesure (RTX 5070 portable, Dx12) | |
|---|---|---|
| (1) le champ fondu de la carte égale le fondu du CPU sur le même `φ`, à 10⁻⁵ | **7,2·10⁻⁷** aux quatre instants | tenu |
| (2) une image de 960 × 600 en 1 ms au plus (médiane, sur la carte) | **0,23 ms** horodatée, fondu compris (mur : 0,69 ms) | tenu |
| (3) quatre images aux instants de R37, comparables | `captures/s452/carte_t{0.5,1.0,2.0,3.0}.png` | envoyées |

**Ce qui se voit** : les images de R37 — le cratère et la sphère, puis le jet de Worthington —, un peu plus lisses (l'interpolation
trilinéaire ne marque pas les arêtes des tétraèdres). **Ce qui reste** : brancher ce rendu dans la boucle vivante de l'afficheur (la
fenêtre, la caméra libre) — puis C10.

## S453 — dans la boucle vivante, la carte seule

2026-10-02. **La carte seule** : jusqu'ici la référence CPU choisissait le pas de la carte ; `ApicCarte::stable_step_us` applique la même
formule (`0,5·dx / (v_max + √(|g|·dx))`) aux vitesses relues sur la carte — particules et faces. B10 avance alors **sans référence**
(elle ne sert plus qu'à l'ensemencement et aux réglages de la bascule). **La fenêtre** (`viewer/src/surface_direct.rs`) : sur le
device de la carte (`ApicCarte::with_instance`, un adaptateur compatible avec la fenêtre), la simulation suit le temps réel (deux pas
au plus par image), puis `surface_carte` rend dans l'image de la fenêtre.

**Reproduire** : depuis la racine, `viewer/target/release/water-viewer.exe --surface-direct-banc` (49 s, dont ~47 s de mise en route :
l'ensemencement et la compilation des pipelines de la carte) ; attendu `quanta_ecart=0` aux quatre instants, `ecart_s452_moyen` 0,00 /
0,00 / 0,07 / 0,04. La fenêtre : `viewer/target/release/water-viewer.exe --surface-direct` (`DUREE=15` : se ferme seule et imprime
le bilan).

| critère (écrit avant) | mesure (RTX 5070 portable, Dx12) | |
|---|---|---|
| (1) la carte seule tient jusqu'à t = 3 : masse exacte, `φ` fini, images de S452 à l'œil | quanta : écart **0** ; `φ` fini ; pixels changés de plus de 8 niveaux : **0 %**, 0 %, 0,1 %, 0,05 % | tenu |
| (2) une image en 33 ms au plus (médiane, pas compris) ; simulé / réel publié | **1,3 ms** (99e centile 14 ms, une fois 133 ms au départ) ; **0,97** | tenu |
| (3) la commande à l'utilisateur | `--surface-direct` | à son jugement |

**Ce qui se voit** : les images du banc (`captures/s453/direct_t*.ppm`) sont celles de S452 — la carte seule suit le chemin qu'elle
suivait au pas de la référence. Le pas de la carte coûte 7,3 ms au mur (médiane, relectures comprises) pour 4,9 ms simulées : la
simulation tient le temps réel parce que la plupart des images n'en portent pas. **Ce qui reste** : la relecture des vitesses (le pas
stable) se ferait sur la carte par une réduction ; C10.

