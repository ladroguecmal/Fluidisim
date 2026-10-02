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
