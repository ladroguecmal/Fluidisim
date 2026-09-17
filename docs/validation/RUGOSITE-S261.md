# Rugosité ajustée à Cox–Munk : coupure et modulation — S261

Contrat : [ADR-158](../adr/ADR-158-rugosite-ajustee-a-cox-munk.md). Origine : verdict R4,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §11, où le candidat a été choisi par le calcul.

## 1. Protocole, écrit avant construction

1. **GPU contre CPU** : aux sondes de `--cwm-verify`, sous `--vagues --modulation`, la pente
   eulérienne du GPU égale la référence CPU, modulée et coupée de la même façon, à 5·10⁻⁴ près.
   Déplacement à 3 mm près. Aucun repli.
2. **Coupure** : le nombre de lignes de la queue lues égale le nombre de composantes à au plus
   `28 fp`, compté sur la cuisson.
3. **Scènes existantes au bit** : R2, R3 et R4 rejoués avec leurs empreintes, et `--cwm-verify
   --vagues` identique à S260.
4. **Coût** : GPU eau sous `--vagues --modulation --ciel-clair` contre `--vagues`, 1280×720, deux
   poses, secteur. L'habillage (nuages dans les reflets) est compté à part.
5. **Rendus R5** aux poses de R1–R4, habillage ciel clair, envoyés à l'utilisateur.

Un critère manqué est publié tel quel.
