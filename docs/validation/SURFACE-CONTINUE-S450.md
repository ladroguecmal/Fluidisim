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
