# L'étape 3 du LOD : la 3D rallumée autour d'un corps — conception (S722)

*Écrit en S722, 2026-10-09* ([ADR-275](../adr/ADR-275-le-lod-de-simulation.md) D2, étape 3).

## Le but

Un corps (un joueur, un objet, un navire) entre dans une eau que porte Saint-Venant 2D : le large calme, ou la lame qui remonte sur le sable.
Autour de lui s'allume une **boîte de 3D**, qui suit le corps, rend les éclaboussures, le sillage, l'eau qui monte sur lui. Quand le corps
sort ou s'arrête, la boîte meurt et rend son eau à Saint-Venant.

À l'étape 2, la 3D était une bande de toute la largeur, raccordée à gauche et à droite seulement. Ici, la boîte est **raccordée sur ses
quatre côtés**. Saint-Venant a un **trou** là où la 3D est vivante.

## Ce qui est déjà là

- **Les deux bords en x d'APIC** : le bord à particules à gauche (S698, `apic3d_gauche.rs`, la pose par la grille S702) et le relais au
  rivage à droite (S680–S690 : la sortie, l'entrée, la dette au quantum).
- **La naissance et la mort par la surface** (S707, S715, S717).
- **Le corps libre** dans APIC (S653–S657 : la sphère, sa masse ajoutée).
- **Saint-Venant 2D** : le flux imposé au bord droit (`pas_avec_flux_droit`, S680).

## Les pièces, une par session au plus

Chaque pièce est jugée d'abord entre deux copies du même solveur (ADR-273 D1).

| pièce | ce qu'elle fait | son essai, entre deux copies d'abord |
|---|---|---|
| **B1 — Saint-Venant troué** | un rectangle de mailles inactives ; sur ses faces, le flux normal imposé de l'extérieur, comme le bord droit de S680, mais sur quatre côtés | un Saint-Venant troué, dont le trou est rempli par un second Saint-Venant qui lui rend ses flux : ensemble, ils redonnent le Saint-Venant entier (une onde qui traverse le trou en biais), à la masse au bit |
| **B2 — APIC à quatre bords** | le bord à particules en y (devant, derrière), comme en x (S698) : les particules qui sortent retirées et comptées, celles qui entrent posées par la grille | une boîte d'APIC au repos à quatre bords ouverts (µm/s) ; puis une onde plane en biais à travers une boîte, contre la même onde dans un APIC plus grand (le rejeu de S699, sur quatre côtés) |
| **B3 — le raccord de la boîte** | Saint-Venant troué ↔ APIC à quatre bords : chaque face du trou reçoit le flux d'APIC ; chaque bord d'APIC reçoit l'état de Saint-Venant (la vitesse, le niveau). Les coins à part | au repos ; puis une onde longue qui traverse la boîte, contre Saint-Venant seul (une onde longue, que les deux portent également), la masse au bit, la dette sous un quantum |
| **B4 — la boîte qui suit le corps** | la boîte naît autour du corps (par la surface), se déplace avec lui (des colonnes naissent devant, meurent derrière), meurt quand il sort | une sphère tirée à vitesse constante dans une eau au repos : le sillage et la vague d'étrave, contre le même corps dans un tout-3D ; le coût |
| **B5 — le déclencheur de présence** | la présence (un corps à moins d'une distance, une vitesse), l'hystérésis (ADR-275 D1) | un corps qui entre, s'arrête, sort : la boîte naît, vit, meurt, sans choc de masse ni de niveau |

## Les pièges connus

- **Les coins** : deux bords se touchent. Une particule qui sort par un coin est comptée une fois, et les flux de Saint-Venant aux coins
  sont partagés.
- **Les quatre bords de Saint-Venant** : `pas_avec_flux_droit` ne sait imposer que le bord droit. Le trou demande une généralisation.
- **Le coût** : une boîte de 1 m × 1 m sur 0,5 m d'eau compte ≈ 256 000 particules à 2,5 cm, l'ordre du tout-3D de S690. La boîte se
  dimensionne sur le corps.
- **La lecture** : par la surface (ADR-280 D1) ; chaque résultat montré au fil du calcul (ADR-281 D1).

*Note datée du 2026-10-09 (S723)* : **B1 acquis** — Saint-Venant troué, rempli par un second Saint-Venant, à la masse au bit et à 1,6 % du
Saint-Venant entier, à condition d'échanger le **flux complet** (la masse seule : 8,3 %) ([preuve](../validation/SV-TROUE-S723.md)).

*Note datée du 2026-10-09 (S724)* : **B2 acquis pour la masse et l'écoulement** — la boîte à quatre bords au repos au micron, un courant en
biais à 0,5 % ([preuve](../validation/BOITE-QUATRE-BORDS-S724.md)). Restent quelques millimètres de rides à l'entrée en y, à juger dans B3.

*Note datée du 2026-10-09 (S725)* : **B3 acquis** — la boîte de 3D au milieu de Saint-Venant, au repos au micron, la masse au bit, une
onde longue réfléchie à 9 % ([preuve](../validation/RACCORD-BOITE-S725.md)). Une onde dispersive réfléchit deux fois plus : c'est le
désaccord des modèles, non le raccord. Suivant : B4, la boîte qui suit un corps.

*Note datée du 2026-10-09 (S727)* : **B4a acquis** — un corps dans la boîte fixe : sa force à 2 %, l'eau autour de lui à 1,7 % de la 3D
entière, quatre fois moins de particules ([preuve](../validation/CORPS-DANS-LA-BOITE-S727.md)). Suivant : B4b, la boîte qui suit le corps.
