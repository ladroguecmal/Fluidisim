# La vague qui plonge à travers le relais au rivage — S690 (liste 4.14)

*S690, 2026-10-08, en autonomie, vers la v2.* L'étape 4 du relais au rivage ([conception](../registres/RELAIS-RIVAGE-S679.md)) : la
vague plongeante de S647 (pente 1:12, `S₀` = 0,231) dans le relais, à 2,5 cm. APIC 3D va jusqu'à 10,775 m, sur l'escalier avec l'air
balistique ; Saint-Venant 2D va au-delà. Elle est jugée contre le tout-3D de S647–S648 (ADR-273 D1).

## Reproduire

- `python outils/essai.py a_plunging_wave_breaks_through_the_shore_relay_s690 --ignore` (≈ 13,6 min).

## Ce qui a été fait

- **Le relais sur fond en escalier** (`fond_bord`).
- **Le calcul rendu praticable.** Le premier essai a tourné 8 h sans résultat, puis a été arrêté.
  - Les 16 fils de la machine (`set_jobs`, au bit du séquentiel, S483).
  - Le pas stable réel du relais (`pas_stable_us`, la célérité de Saint-Venant au lieu d'une borne fixe).
  - Une progression affichée, qui a montré l'effondrement du pas.
- **Le raccord borné par la physique.** Le diagnostic a localisé l'effondrement : à 2,98 s, la masse déferlée vide la dernière colonne
  3D. Le niveau lu tombe au fond et la hauteur à son plancher de 1 mm ; la vitesse imposée au bord monte à 1 644 m/s, et les particules
  posées partent à −138 m/s (le pas : 0,1 ms). Une éclaboussure soulevait aussi le niveau lu. Le remède :
  - le plancher de la hauteur porté à un quart de maille ;
  - les vitesses du bord bornées par la célérité `|u| + 2·√(g·h)` ;
  - le niveau borné par le volume de la colonne, plus une couche.

  Au repos, rien ne change : S684 est inchangé.
- **Le remboursement dans toute la rangée.** Quand le ressaut vide la dernière colonne, la particule rendue est la plus proche du bord,
  quelle que soit sa colonne. Sans cela, la dette atteignait 7,2 quanta.

## Mesuré

| | le relais | le tout-3D (S647–S648) | critère |
|---|---|---|---|
| (1) premier retournement | 2,624 s, 9,963 m | 2,642 s, 9,988 m | **tenu** (à 0,02 s et 0,15 m) |
| (2) air enfermé | 2,777 s, 10,325 m, après et en avant | 2,817 s, 10,375 m | **tenu** |
| (3) masse | ≈ 10⁻¹⁶ | — | **tenu** |
| (3) dette | sous un quantum (0,9995 ; 7,2 avant le remboursement dans toute la rangée) | — | **tenu** |
| (4) remontée (Saint-Venant, sous la maille) | 0,361 m | — | rapportée |
| (4) le calcul | 13,6 min pour 4 s, 16 fils | 23 min pour 3 s, un fil (S647) | rapporté |

Le pas, moment par moment : [ANALYSE-PHASES-S690](../registres/ANALYSE-PHASES-S690.md).

## Ce que cela dit

La vague déferle dans le relais comme dans le tout-3D : le même retournement, et l'air enfermé au même endroit, à quelques centimètres et
centièmes de seconde. Le raccord, placé 0,4 m au-delà de la chute du jet, ne trouble pas le déferlement. Le ressaut, puis la remontée,
passent dans Saint-Venant.

Deux leçons de calcul pour la suite (la revue S691) :

- le coût se mesure avant de s'annoncer, et un calcul long montre sa progression ;
- une division par une hauteur se borne par la célérité, pas par un epsilon.
