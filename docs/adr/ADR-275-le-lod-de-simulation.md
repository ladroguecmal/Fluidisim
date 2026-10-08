# ADR-275 — Le LOD de simulation : l'eau représentée au niveau qu'elle demande

- **Statut : actée**, S692, 2026-10-08. Proposée par l'utilisateur le même jour, acceptée (*« Ok parfait »*). Conception :
  [LOD-SIMULATION-S692](../registres/LOD-SIMULATION-S692.md).

## Contexte

S690 a montré qu'une vague qui plonge ne demande la 3D que pendant son retournement. Avant, c'est une onde ; après, c'est un ressaut et un
jet de rive, que la 2D porte. L'utilisateur l'a formulé ainsi : l'écume qui roule jusqu'à la plage en 2D, la 3D aux jets et aux
contacts, un LOD de simulation par présence, des billes plus nombreuses là où l'eau bouge.

## Décision

**D1 — Six niveaux**, du moins cher au plus fin :

1. B ;
2. B et W ;
3. la côte 2D cuite (Cote2D) ;
4. Saint-Venant 2D ;
5. APIC 3D grossier (la bande étroite) ;
6. APIC 3D fin.

L'eau monte d'un niveau par la présence (un joueur, une caméra proche, un objet), l'agitation (le retournement prédit par la côte, la
vitesse, la courbure, l'air enfermé) ou le contact d'un corps. Elle descend avec hystérésis.

**D2 — L'ordre** :

1. les raccords complets (la zone de colonnes avec la sortie à droite) : la 3D réduite à la bande de déferlement ;
2. la bande 3D qui naît et meurt avec la vague ;
3. le rallumage local de la 3D autour d'un corps dans Saint-Venant ;
4. la bande étroite dans le relais ;
5. **les billes fusionnées et divisées** selon l'agitation, en dernier. Elles demandent une grille adaptative pour gagner, et une
   conservation exacte (masse, quantité de mouvement, la matrice `C`).

**D3** — Chaque étape est jugée sur la masse au bit, sur le déferlement et l'air du tout-3D, et sur son coût mesuré (ADR-274 D1).

## Conséquences

- La campagne suit le relais au rivage. Ses étapes touchent 4.14, 4.16, 1.6 (cellules, domaines, niveaux d'activité), les 9.x
  (l'activation) et 4.19 (le coût).
- L'écume n'est pas suspendue : seul son visuel attend validation (2026-10-08). Au niveau 4, elle se tire du ressaut de Saint-Venant
  (la dissipation de Battjes et Janssen, S669–S677).

*Note datée du 2026-10-08 (S705)* : l'étape 1 est close par [ADR-278](ADR-278-le-raccord-du-large-retenu.md) D4. La zone de colonnes du
large est remplacée par le bord à particules, nourri par SGN et posé par la grille. La tolérance de temps de D3 est rapportée à la
convergence du juge par ADR-278 D2 : la position à 0,15 m, l'instant à 0,1 s.
