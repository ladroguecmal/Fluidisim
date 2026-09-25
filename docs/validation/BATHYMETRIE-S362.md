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
- **S364, l'entrée dans B** (§5) — commit de P5 de S364 ou plus récent. `cargo test -p water-core --release --offline
  s364 -- --nocapture` — deux essais, lignes `S364`, moins d'une seconde : pire hauteur 0,0542 / 0,2256 / 1,3926 /
  5,3532 mm aux pas de 1 / 2 / 5 / 10 m ; bord à λ₀/2 −1,01·10⁻², à λ₀ −4,15·10⁻⁵ ; au large 120 évaluations identiques,
  hash `1b8e763453ab96b9`, η à 0,133 mm. `cargo run -p water-core --release --offline --example bathymetrie_cote_cout` —
  lignes `COUT_S364`, une minute ; les nanosecondes dépendent de la machine, leurs rapports non.

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

## 5. S364 — l'entrée dans B

Décision : [ADR-196](../adr/ADR-196-la-bathymetrie-entre-dans-b-par-composante.md). `bathymetrie_cote.rs` : chaque
composante de B reçoit, cuites depuis cette référence le long de la normale à la côte, une **correction de phase
entière** (Q32), le facteur `K_s·K_r`, son `k_y` local et `coth(kh)` ; l'exécution les interpole — la phase en entiers,
le reste en f32. Critères écrits avant le code (EN-COURS S364).

| critère | mesure | verdict |
|---|---|---|
| 1. précision, houle d'1 m, 10 s, 30°, plage 1/50 jusqu'à 2 m : ≤ 3 mm, facteur à 1 % ; prédit 0,4 mm à 2 m, 2 mm à 5 m | **0,054 / 0,226 / 1,39 / 5,35 mm** aux pas de 1 / 2 / 5 / 10 m, en `Δ²` ; facteur à 1,5·10⁻⁵ | tenu au pas de 2 m |
| 2. au large des tables, B au bit | **120 évaluations identiques** (neuf grandeurs, trois instants) | tenu |
| 3. déterminisme | deux passes, même hash ; phase entière de bout en bout | tenu sur cette machine |
| 4. coût par composante, contre B | groupés **52 ns contre 33** (×1,58), dispersés **68 contre 58** (×1,18) ; identique aux pas de 1, 2 et 5 m | O(1) |
| 5. mémoire | **258 Ko par km de profil** au pas de 2 m, 32 composantes ; 2D régulière : 128 Mo/km² | la 2D demande un autre paramétrage |

**Sur une mer.** Huit composantes de 6 à 10 s, ±30°, Hs ≈ 1 m, plage 1/30 de 160 à 2 m, 711 points : η à **0,133 mm**
de la référence, pente à 1,4·10⁻⁵ (maximum 0,086), vitesse horizontale à 2,7·10⁻⁴ m/s (maximum 1,31).

**Trouvé : le bord du large.** Au « fond qui cesse de se sentir » des manuels, λ₀/2, le facteur de levée d'une houle de
10 s vaut encore **0,990** : 5 mm de marche entre B et la côte pour une houle d'un mètre. **Prédit puis mesuré** : à λ₀,
−4,15·10⁻⁵ (prédit −4,4·10⁻⁵), 0,02 mm. Les tables commencent à λ₀ de la plus longue composante (ADR-196 D3) ;
SPEC-005 §8 reçoit une note.

**Limites.** Isobathes droites ; ni marée, ni déferlement dissipé, ni diffraction ; le déterminisme entre plateformes
repose sur l'arithmétique (entiers et f32 à ordre fixé), non mesuré faute d'une seconde cible (A98). **Un consommateur**
: la requête de B sur la côte ; ni l'accélération, ni la publication pour l'image, ni la scène de Godot.
