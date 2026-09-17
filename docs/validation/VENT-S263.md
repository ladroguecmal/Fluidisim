# Le vent comme paramètre de scène — S263

Contrat : [ADR-160](../adr/ADR-160-vent-parametre-de-scene.md). Origine : verdict R5,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §12.

## 1. Protocole, écrit avant construction

1. **Recette** : `Hs` et `Tp` de la mer de vent égaux aux formules de Pierson–Moskowitz, à l'arrondi f32.
2. **Rugosité** : `mss` totale (mer de vent + houle + queue gardée) égale à Cox–Munk au même vent, à
   une composante de queue près (écart publié), ou limitée par la coupure capillaire, ce qui se dit.
3. **Pointe et replis**, par vent (3, 5, 8,4 m/s), instrument statistique, 10⁶ points : `c40`, `c22`,
   `c04` publiés en regard de Cox–Munk ; aucun repli CWM.
4. **Scènes existantes au bit** : R2 à R4 rejoués, et `--vagues --modulation` sans `--vent` identique
   à S262.
5. **Accord CPU/GPU** : `--cwm-verify` et `--cwm-query-verify` sous `--vent=5`, tolérances de S260 et S262.
6. **Rendus de calibration R6** : trois vents × trois poses, ciel clair, envoyés à l'utilisateur, qui
   choisit.
