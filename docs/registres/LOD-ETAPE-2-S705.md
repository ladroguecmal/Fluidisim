# L'étape 2 du LOD : la bande 3D qui naît et meurt avec la vague — conception (S705)

*Écrit en S705, 2026-10-08* ([ADR-275](../adr/ADR-275-le-lod-de-simulation.md) D2, étape 2 ; [ADR-278](../adr/ADR-278-le-raccord-du-large-retenu.md)).

## Le but

Aujourd'hui, la 3D couvre une bande fixe : [5,0 ; 10,775] m, du raccord du large au relais du rivage. L'étape 2 la fait **naître**
devant la vague qui va déferler, et **mourir** quand la vague a passé dans Saint-Venant. Entre deux vagues, la 3D est éteinte et rien
ne coûte que la 2D.

## Les pièces, une par session au plus

Chaque pièce est jugée d'abord entre deux copies du même solveur (ADR-273 D1), puis dans le montage de la vague (ADR-275 D3, ADR-278
D2 : la position à 0,15 m, l'instant à 0,1 s, l'air, la masse au bit).

| pièce | ce qu'elle fait | son essai |
|---|---|---|
| **N1 — la naissance au repos** | une bande de 3D née d'un état 2D au repos : les particules posées sous `h` (la grille des sous-mailles, le fond lisse ou l'escalier), la vitesse et l'affine du G2P sur une grille remplie du profil vertical du porteur | l'eau au repos reste au repos, à la masse au bit ; la naissance ne crée ni vague ni courant (µm/s, comme S684) |
| **N2 — la naissance dans l'onde** | la même dans une onde en marche, le porteur SGN | l'onde de S704 traverse la naissance ; sa crête en aval contre celle d'une 3D née au départ (le tout-3D sur fond plat), au quantum de lecture |
| **M1 — la mort vers Saint-Venant** | la bande éteinte : chaque colonne rend son volume (`h`) et sa quantité de mouvement (`h·ū`) à Saint-Venant ; les particules retirées | l'eau au repos ; puis le ressaut après le déferlement : la masse au bit, la quantité de mouvement à 10⁻⁶ ; Saint-Venant reprend sans choc |
| **D1 — le déclencheur** | le moment et le lieu de la naissance, tirés du porteur : la cambrure, ou le critère de déferlement de Battjes et Janssen (S669), à une distance de la ligne de déferlement prévue | sur la plage de S690 : la naissance avant que le retournement de la 3D ne commence, avec une marge mesurée |
| **E1 — l'ensemble** | la vague de S690, du large au sable : la 2D seule, la 3D qui naît, déferle, meurt | ADR-278 D2 contre le tout-3D ; le coût mesuré et montré (ADR-274 D1), comparé au tout-3D et à la bande fixe |

## Ce qui est déjà là

- le bord à particules des deux côtés : à gauche (S698, `apic3d_gauche.rs`), à droite (S682–S683) ;
- la pose par la grille (S702) ;
- le relais au rivage, qui rend la 3D à Saint-Venant (S680–S690) ;
- le porteur SGN (S694) et la côte cuite qui prévoit la zone de déferlement (S677).

## Les pièges connus

- **La masse au bit à chaque passage** : un volume rendu par particules est un compte de quanta. La part sous le quantum va à une dette,
  comme au rivage (S684).
- **La vitesse d'une particule née** : prise au G2P, sur une grille remplie avant la naissance. Une vitesse posée à la main (le profil
  seul) a coûté 0,1 s en S700.
- **La hauteur rendue à la mort** : lue par le volume des particules, non par la plus haute (S700, S704).

*Note datée du 2026-10-08 (S707), à la demande de l'utilisateur* : « la 3D peut s'allumer à condition qu'il y ait besoin d'elle, comme
un système de prédiction ; une vague qui ne pourra pas déferler, la 2D suffira peut-être ». Le déclencheur (D1) décide donc d'abord
**s'il faut** allumer la 3D, puis quand et où. Une vague dont la côte cuite ne prévoit pas le déferlement (Battjes et Janssen, la
fraction déferlée `Q_b` nulle sur son trajet) reste en 2D du large au sable. La 3D ne s'allume alors que pour une présence proche ou un
contact (ADR-275 D1).

Un essai s'ajoute à D1 : une petite vague qui ne déferle pas, en 2D seule, contre le tout-3D. Sa remontée sur le sable et son niveau au
rivage doivent tenir à la tolérance d'ADR-278 D2 (la position à 0,15 m), et son coût doit être mesuré.

*Note datée du 2026-10-08 (S707)* : **N1 tenue** (la naissance au repos, [preuve](../validation/NAISSANCE-3D-S707.md)). **N2 reportée** :
la renaissance depuis la 3D doit lire et reposer la surface, non le compte, car APIC tasse ses particules sous la crête (+3,8 %). M1 aussi
devra rendre la surface. D'abord S708 : le volume d'APIC par la surface, contre le compte.

*Note datée du 2026-10-09 (S715)* : **N2 acquis** — la renaissance par la surface tient à 1 % de la 3D ininterrompue
([preuve](../validation/RENAISSANCE-SURFACE-S715.md)). Née de SGN, la 3D garde la phase et naît avec l'onde de SGN, 10 % plus haute,
qui se repose en ≈ 1 s. Suivant : M1, la mort par la surface.

*Note datée du 2026-10-09 (S717)* : **M1 acquis** — la 3D morte à 3,2 s rend une remontée à 0,2 mm et 2 ms du tout-3D, pour 2,3 fois moins
de calcul sur 5 s ([preuve](../validation/MORT-3D-S717.md)). Suivant : D1 au plus simple (l'élévation de SGN au bord de la bande) et E1,
la vague de bout en bout.

*Note datée du 2026-10-09 (S718)* : **E1, la vague de bout en bout, fonctionne**, pour 4 fois moins de calcul que le tout-3D
([preuve](../validation/BOUT-EN-BOUT-S718.md)). Le déferlement est dans la tolérance. La remontée, +3,3 cm, vient du large (SGN, l'onde de
départ plus haute), non de la mort. Reste D1, le déclencheur, au plus simple, puis la question d'ADR-278 D3 (S719).

*Note datée du 2026-10-09 (S729), à la demande de l'utilisateur* : « la simulation 3D peut se réaliser jusqu'à la plage si une vague éclate
trop proche du bord ». Le déclencheur (D1) fixe donc aussi **où s'arrête la bande** : le point de déferlement prévu, plus la distance de
chute du jet, plus une marge. Si cela dépasse le rivage, la bande couvre la plage. La difficulté est mesurée : sur sable sec, la lame mince
fait tomber le pas de la 3D sous 0,3 ms (S712). Il faudra la traiter en film mince (S678), ou éteindre la 3D sur le sable dès que le jet est
retombé.

*Note datée du 2026-10-09 (S730)* : R43 reçu ; ADR-284 décide. Le raccord du rivage, placé au-delà du jet (12,0 m), efface le mur de R43
pour 6 % de calcul ([preuve](../validation/RACCORD-AU-DELA-DU-JET-S730.md)). Le déclencheur (D1) placera aussi les frontières.
