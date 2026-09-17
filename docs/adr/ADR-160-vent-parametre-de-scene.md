# ADR-160 — Le vent est un paramètre de la scène, calibré à l'œil

Actée S263, 2026-09-17, autonomie S71. Répond au verdict R5
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §12). Généralise ADR-158 : sa coupure `b_Q` = 28
était un ajustement à 8,4 m/s. Ne modifie aucune scène existante.

## Décision

1. **Vent de scène `U`** (m/s, pris pour le vent de Pierson–Moskowitz ; Cox–Munk l'emploie au même
   nombre, l'écart de hauteur de mesure, environ 5 %, est négligé et déclaré). La mer de vent
   pleinement développée s'en déduit : `Hs = 0,21·U²/g`, `Tp = 2πU/(0,877·g)`, γ 3,3, bande
   `[0,5 ; 4] fp`, direction et étalement d'ADR-156 (`s_max` 10).
2. **Queue coupée par l'observation** : on garde, par `k` croissant, les composantes de la queue
   d'équilibre (ADR-157) jusqu'à ce que la `mss` totale — mer de vent, houle et queue — atteigne celle
   de Cox–Munk à `U`. Jamais au-delà de la limite gravité-capillarité (`λ` ≥ 1,7 cm). Cela remplace
   `b_Q` = 28, qui en était le cas particulier à 8,4 m/s. La cuisson s'arrête à cette limite.
3. **Inchangés** : houle (`Hs` 2 m, `Tp` 12 s), modulation `M` = 2 (ADR-158), CWM (ADR-157), requête de
   jeu (ADR-159), habillage.
4. **Calibration perceptive.** Le vent de la scène représentative se choisit par la revue de
   l'utilisateur, parmi des rendus conformes à Cox–Munk à leur vent. Le choix est un fait de revue,
   consigné, et non une loi.
5. **Variante** `--vent=U` sur `--vagues --modulation`. Sans elle, toutes les scènes au bit.
6. **Réception** : [VENT-S263](../validation/VENT-S263.md).

## Hors de cette décision

Mer non développée (fetch limité), vent variable dans l'espace ou le temps, rafales, écume selon le
vent (ADR-014), transition des ondes non résolues vers la BRDF.
