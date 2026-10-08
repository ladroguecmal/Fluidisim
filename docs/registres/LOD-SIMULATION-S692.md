# Le LOD de simulation — conception (S692)

*S692, 2026-10-08.* [ADR-275](../adr/ADR-275-le-lod-de-simulation.md) décide. Demandé par l'utilisateur le 2026-10-08 (*« Ok parfait »*) :

- l'eau d'après déferlement en 2D jusqu'à la plage ;
- la 3D aux jets et aux contacts (objet, joueur) ;
- un LOD par présence ;
- des billes fusionnées ou divisées selon l'agitation.

## 1. L'échelle des représentations (de la moins chère à la plus fine)

| niveau | représentation | où | état |
|---|---|---|---|
| N0 | B (spectral, analytique) | le large, partout | validé (v1) |
| N1 | B + W (événements, sillages) | près des sources | validé (v1) |
| N2 | Cote2D (B transformée par la côte : réfraction, déferlement, niveau, courant) | les côtes, cuite | S664–S677 |
| N3 | Saint-Venant 2D | le calme peu profond, les ressauts, le jet de rive, les rivières | S613–S628, raccords S650, S684–S690 |
| N4 | APIC 3D grossier (bande étroite, S413) | la surface qui bouge sans se retourner | S413–S415 |
| N5 | APIC 3D fin | le retournement, les jets, les contacts | S388–S690 |

## 2. Les critères d'activation (ce qui fait monter ou descendre)

- **La présence.** Un joueur, une caméra proche, un objet dynamique (ADR-202 : δ seulement près d'un joueur ou d'un perturbateur).
- **L'agitation.**
  - Le retournement prédit par N2 : la zone de déferlement (S677, `Q_b`) et le paramètre de Grilli (S647) ;
  - la vitesse, la courbure de la surface ;
  - la présence d'air enfermé.
- **Le contact.** Un corps qui entre dans l'eau (S653–S657 ; 6.x), même en zone N3 : on rallume localement N4 ou N5.
- **L'hystérésis.** Monter tôt, descendre tard (la bande étroite S414 : maintien de 0,3 s, R35).

## 3. Les raccords

- N3 ↔ N5 : le relais au rivage (S684–S690), dans les deux sens, la masse au bit.
- N3 ↔ N4/N5 au large : S650 (la zone de colonnes). Manque : la zone de colonnes avec la sortie à droite (refusées ensemble, S682), et
  avec le fond lisse.
- N2 → N3 : le bord de Saint-Venant nourri par Cote2D (S622 : le bord forcé).

## 4. Les billes adaptatives (la dernière étape)

- Fusion et division (Adams 2007 ; Solenthaler & Gross 2011 ; Winchenbach 2017), avec deux contraintes :
  - **la grille** : sans grille adaptative, fusionner sous deux billes par maille et par axe rend la surface bruitée ;
  - **la conservation** : la masse, la quantité de mouvement et la matrice `C` d'APIC ; la reconstruction de la surface pour des
    rayons variables.
- **Avant elle**, les gains disponibles :
  - la bande étroite (÷ 5 à 7) ;
  - la 3D réduite à la bande de déferlement (÷ 5) ;
  - la bande qui vit avec la vague.

## 5. Les étapes proposées

1. La zone de colonnes compatible avec la sortie à droite → les deux raccords ensemble, la 3D réduite à la bande de déferlement
   (S690 refait au large par Saint-Venant).
2. La bande 3D qui naît quand N2 prédit le retournement, et meurt quand l'eau est rendue à N3.
3. Le rallumage local de la 3D autour d'un corps dans N3.
4. La bande étroite (S413) dans le relais.
5. Les billes adaptatives (conception, puis un essai sur la vague de S647).

Chaque étape est jugée : la masse au bit ; le déferlement et l'air du tout-3D ; le coût mesuré (ADR-274 D1).

## 6. Ce que S690 a mesuré, qui le justifie

- Sur 4 s simulées, seul le retournement (2,6 à 2,9 s) demande la 3D.
- 75 % des particules portent une onde qui ne se retourne pas ([ANALYSE-PHASES-S690](ANALYSE-PHASES-S690.md)).
- Le relais au rivage rend le déferlement du tout-3D, le retournement à 0,02 s près, et porte le ressaut et la remontée en 2D.
