# La houle qui sent le fond : la référence — S362

2026-09-25. Liste **2.7** (bathymétrie : « hauts-fonds, effet sur les vagues avant la zone physique »), *absente*
jusqu'ici ; vingt points en dépendent ([registre](../registres/DEPENDANCES-LISTE.md)). Choisie à deux maillons parce
qu'elle fait avancer un point de la liste, et parce que la scène côtière de [S359](EPAISSEUR-EAU-S359.md) la montre
manquer : B ne voit pas le fond.

## Reproduire

- Commit `bf916c64` ou plus récent ; aucune carte graphique nécessaire.
- `cargo test -p water-core --release --offline s362 -- --nocapture` (dans `code/`) — quatre essais, lignes `S362`,
  moins d'une seconde. Valeurs attendues : résidu de dispersion 4,39·10⁻¹⁶ ; Fenton–McKee 0,0163 ; levée minimale 0,91299
  à kh = 1,1995 ; dérivée de la phase 3,27·10⁻⁸ ; déferlement h_b = 1,7833 m et 1,6880 m.

## En une phrase

Le cœur sait désormais ce qu'une houle devient au-dessus d'un fond qui remonte — sa longueur d'onde qui raccourcit, son
amplitude qui baisse puis croît, sa direction qui tourne vers la côte, la profondeur où elle déferle —, tenu contre les
résultats publiés ; il ne sait pas encore où le dire à B.

## 1. Ce qui est construit

`code/water-core/src/bathymetrie.rs`, une **référence** en f64 — le juge de tout candidat, pas le chemin déterministe
de B (I-03). Théorie linéaire (Airy) sur un fond lentement variable (WKB), formules de Dean et Dalrymple (1991, ch. 3–4) :

- `nombre_d_onde` : la racine de `ω² = g·k·tanh(k·h)` — départ d'Eckart (1952), Newton ;
- `vitesse_de_groupe`, `coefficient_de_levee` `K_s = √(c_g0/c_g)` (Green : le flux d'énergie se conserve) ;
- `transformer` : Snell sur des isobathes droites, `k·sin θ = k₀·sin θ₀` (Munk et Arthur 1952), `K_r = √(cos θ₀/cos θ)`,
  l'amplitude `a₀·K_s·K_r` ;
- `phase_transversale` : `∫ k_y dy` à travers le profil — avec `k_x·x − ω·t`, la phase WKB d'une composante ;
- `profondeur_de_deferlement` : où `H = 0,78·h` (McCowan 1894) — la ligne de déferlement que SPEC-005 et ADR-005 §4
  veulent dériver de la bathymétrie.

## 2. Critères

| critère, écrit avant | mesure | verdict |
|---|---|---|
| dispersion : résidu ≤ 10⁻¹² ; limites à 10⁻⁶ ; Fenton et McKee (1990) ≤ 1,7 % | résidu **4,4·10⁻¹⁶** ; profond 0, peu profond 6,7·10⁻⁷ ; **1,63 %** à k₀h = 0,34 | tenu |
| levée minimale 0,913 vers kh ≈ 1,2 | **0,91299 à kh = 1,1995** | tenu |
| Snell, flux d'énergie à 10⁻¹⁰, dérivée de la phase à 10⁻⁶ (plage 1/50, 8 s, 30°) | 1,1·10⁻¹⁶ ; **4,9·10⁻¹⁶** ; **3,3·10⁻⁸** | tenu |
| déferlement `H = 0,78·h` à la profondeur trouvée | houle de 1 m, 10 s : **h_b = 1,783 m**, H_b = 1,391 m ; à 30°, 1,688 m ; écart 2,5·10⁻¹⁶ | tenu |

Sur la plage, la houle arrivée à 30° ne fait plus que **10° par 2 m de fond**.

**L'instrument, corrigé deux fois au premier passage ; le seuil, jamais.** La dérivée de la phase se prenait par
différence centrée. Au coin du profil (la plage s'arrête à 1 m), elle se trompait de 2,45·10⁻³ par construction ; à
±0,5 m, sa troncature `k_y''·d²/(6·k_y)`, calculée à 3·10⁻⁶ par 2 m de fond, dépassait le critère — mesurée 3,11·10⁻⁶.
Hors du coin et à ±5 cm, la troncature prévue est 3·10⁻⁸ ; mesuré, 3,27·10⁻⁸.

## 3. Ce que la référence ne tranche pas

**Où la bathymétrie entre.** ADR-004 §2.1 garde les composantes de B identiques sur toute la planète — seules leurs
amplitudes varient — et §5 place levée et réfraction dans W ; ADR-054 renvoie la bathymétrie à B2, ADR-156 à J5. Faire
varier le nombre d'onde et la direction d'une composante avec le fond, comme cette référence le fait, **contredit
ADR-004 §2.1** : il faudra un ADR, avec sa mesure (coût, requêtes de jeu, déterminisme), pour choisir entre B transformé
par composante, W comme couche côtière, ou un précalcul côtier (ADR-013).

## 4. Limites

- Isobathes droites et parallèles seulement : ni haut-fond isolé (Berkhoff 1982 demande la diffraction), ni réflexion.
- Linéaire : la non-linéarité en faible profondeur n'a toujours pas d'oracle (A234).
- Le déferlement se **localise** ; sa dissipation, son écume et le jet d'eau ne sont pas modélisés.
- Aucun consommateur encore : ni B, ni W, ni la scène côtière de Godot.
