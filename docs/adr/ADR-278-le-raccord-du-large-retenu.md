# ADR-278 — Le raccord du large retenu, et la tolérance de l'étape rapportée au juge

- **Statut : actée**, S705, 2026-10-08. Complète [ADR-275](ADR-275-le-lod-de-simulation.md) (D2, étape 1 ; D3) ;
  [ADR-273](ADR-273-quarante-et-unieme-revue-de-methode.md) D1 ; [ADR-277](ADR-277-quarante-quatrieme-revue-de-methode.md).

## Contexte

Les preuves, dans l'ordre :

| session | mesure | preuve |
|---|---|---|
| S693, S697 | le raccord du large à la façon de S650 (une zone de colonnes) : −0,113 s ; il fait tout l'écart | [S697](../validation/RACCORD-LARGE-SEUL-S697.md) |
| S698 | le bord à particules, nourri par le profil de SGN, posé par faces : −0,047 s | [S698](../validation/RACCORD-LARGE-PARTICULES-S698.md) |
| S699 | nourri exactement par la 3D (le rejeu) : −0,011 s ; le bord est transparent | [S699](../validation/RACCORD-LARGE-REJEU-S699.md) |
| S700 | la pose par faces +0,085 s et les vitesses de SGN −0,121 s se compensaient | [S700](../validation/ALIMENTATION-SGN-S700.md) |
| S702 | la pose par la grille (la vitesse et l'affine du G2P), nourrie par la 3D : +0,021 s | [S702](../validation/POSE-PAR-LA-GRILLE-S702.md) |
| S703 | la même pose nourrie par SGN : **−0,068 s, −0,13 m** ; la masse 2,9·10⁻¹⁵ | [S703](../validation/POSE-GRILLE-SGN-S703.md) |
| S704 | le juge sur fond plat : à 1,25 cm la crête est plus basse qu'à 2,5 cm (0,143 contre 0,149 m) ; SGN garde 0,150 m | [S704](../validation/JUGE-FOND-PLAT-S704.md) |

## Décision

**D1 — Le raccord du large retenu** (`Large::GrilleSgn`) :
- le bord gauche d'APIC par particules : sorties retirées et comptées, entrées posées par quanta (`apic3d_gauche.rs`) ;
- la pose par la grille : la particule naît dans la tranche balayée par le flux, et prend la vitesse et l'affine du G2P ;
- les données du porteur SGN : la vitesse de chaque face par le profil vertical `u(z) = ū + (h²/6 − z²/2)·ū_xx`, le volume par face.

La zone de colonnes de S650 n'est plus le raccord du large ; elle reste pour ce qu'elle fait bien (les eaux calmes, S398).

**D2 — La tolérance de temps de l'étape, rapportée au juge.** ADR-275 D3 juge chaque étape contre le déferlement du tout-3D. Les
critères de S693–S703 demandaient 0,02 s : c'est sous la convergence du juge lui-même. À 2,5 cm, la crête est ≈ 6 mm (4 %) au-dessus de
celle à 1,25 cm (S704). S697 et S703 montrent qu'un écart de crête de cet ordre déplace le retournement de plusieurs centièmes de seconde.
Désormais, pour toutes les étapes du LOD :
- **la position du retournement à 0,15 m** (inchangée) ;
- **son instant à 0,1 s**, soit 0,25 m à la célérité de l'onde (≈ 2,5 m/s), sous le visible sur une plage ;
- l'air après lui, la masse au bit, la dette sous un quantum (inchangés).

La tolérance se fonde sur la mesure du juge (S704) et sur l'usage, non sur l'écart obtenu. Elle se resserre quand un juge convergé
existe : le tout-3D à 1,25 cm, à lancer quand une étape en aura besoin (≈ 2 h).

**D3 — La question ouverte.** SGN garde la crête de l'onde de départ (0,150 m) ; la 3D convergente la réajuste plus bas (≈ 0,143 m).
L'onde de départ (un profil de Boussinesq) n'est l'onde solitaire exacte d'aucun des deux. À l'intégration, le large sera nourri par la
houle (B et W), non par une onde solitaire. La question se rouvre quand le raccord sera nourri par W.

**D4 — L'étape 1 du LOD (ADR-275 D2) est close** avec D1. L'étape 2, la bande 3D qui naît et meurt avec la vague, est conçue dans
[LOD-ETAPE-2-S705](../registres/LOD-ETAPE-2-S705.md).

*Note datée du 2026-10-09 (S719)* : D3 mise à l'épreuve ([preuve](../validation/MEME-ONDE-S719.md)). Avec la même onde de départ (Rayleigh)
pour SGN et la 3D, la vague de bout en bout remonte encore 3,0 cm plus haut que le tout-3D. La cause n'est ni l'onde de départ ni le volume
(la bande nourrie par SGN reçoit 3 % d'eau de moins), mais la dynamique propre de SGN sur une onde très non linéaire. La question reste
ouverte jusqu'à la houle (W).

*Note datée du 2026-10-09 (S730)* : la remontée de bout en bout, +3,3 cm contre le tout-3D (S718, attribuée au large SGN), devient −1,8 cm
quand le raccord du rivage est placé au-delà du jet, dans les deux montages ([preuve](../validation/RACCORD-AU-DELA-DU-JET-S730.md)). Une
part de l'écart venait du raccord du rivage.
