# Les crêtes de B : l'écume et la lumière qui les traverse — S356

2026-09-25. **Rendu 1** d'[ADR-191](../adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md) ; liste 8.4 ; l'ordre de
[RENDU-ECART-S307](RENDU-ECART-S307.md) §6, points 3 et 4 — ceux que le verdict R14 nommait : *diffusion aux crêtes,
écume*. Construit dans l'afficheur : `viewer/src/rendu_cretes.rs` (la loi, les seuils, le banc de coût) et le module
`viewer/src/water_cretes.wgsl`. **En cours de session, l'utilisateur a choisi Godot 4 pour le rendu final**
([ADR-192](../adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)) : ce module devient la **référence à porter**, et la revue
R19 se fera dans Godot.

## Reproduire

- Commit `e223f1aa` ou plus récent ; machine de référence, carte réelle.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --ecume-loi` — lignes `ECUME_LOI_S356`,
  `ECUME_SEUILS_S356`, `ECUME_CONTROLE_S356` ; ≈ 15 s.
- `… -- --meilleur --eau-physique=2 [--ecume[=réflectance]] [--cretes[=force,exposant]] --revue-mer=<étiquette>` — les
  quatre poses de R14 dans `viewer/captures/s304`. **Sans les options, empreintes inchangées depuis avant S356** :
  `0x6f8a1689225c1761`, `0x8e3355b1cd4b4281`, `0xdd8663c4e76c7a2f`, `0x7d28c1f5092f316a` (référence, proche, rasante,
  haute).
- `… -- --meilleur --eau-physique=2 --ecume=0.55 --cretes --cretes-bench` — le coût ; ≈ 30 s.
- Valeurs attendues : `W` = 0,00421 ; `s_t` à pleine résolution −2,3113 ; seuils de 7,8 mm à 64 m : −2,3220, −2,3519,
  −2,3789, −2,4055, −2,4436, −2,4573, −2,4900, −2,4722, −2,4832, −2,5112, −2,5082, −2,4924, −2,5434, −2,4311.

## 1. La couverture de l'écume

La mer de `--meilleur` — mer de vent S201 (`Hs` 1,5 m, `Tp` 6 s) et houle, 64 composantes de bande, 60 de queue,
modulation `M` = 2, retard −0,20 tour — est pleinement levée pour **`U₁₀` = 7,79 m/s** (Pierson–Moskowitz,
`Hs = 0,21·U₁₉,₅²/g`, `U₁₀ = U₁₉,₅/1,075`). Monahan & O'Muircheartaigh (1980) : **`W = 3,84·10⁻⁶·U₁₀^3,41` = 0,421 %**
de la surface en moutons.

Le masque est le jacobien `J` du déplacement CWM, que le rendu calcule déjà à chaque pixel (S260) : là où `J` descend,
la surface se comprime — la crête. **Loi de `s = (J − 1)/σ`**, `σ` l'écart-type de la partie linéaire de `J`, sur
1,2 million de tirages (2 km, huit instants) : moyenne 0,011, écart-type 1,004, mais une **queue basse plus lourde que
la gaussienne** — le quantile de 0,421 % vaut **−2,311** [−2,319 ; −2,305] quand la gaussienne dirait −2,635. `J` ne
descend jamais sous 0,215 : aucun repli.

**Une hypothèse contredite.** Normaliser par `σ` devait garder la couverture quand le rendu filtre les ondes courtes.
Non : avec le seuil de pleine résolution, la couverture monte à 1,55 × `W` à 10 cm d'empreinte, 1,77 à 0,5 m, 1,83 à
2 m, 2,02 à 8 m — le filtrage alourdit la queue basse de `s`. **Remède** : un seuil par empreinte, quatorze empreintes
`2^(i−7)` m de 7,8 mm à 64 m, interpolées en `log₂ h` ; 1,2 s de calcul au démarrage. **Sur un tirage indépendant** :

| empreinte | 0 | 1,2 cm | 5 cm | 35 cm | 1,4 m | 5,6 m | 22 m |
|---|---:|---:|---:|---:|---:|---:|---:|
| couverture / `W` | 0,93 | 0,97 | 1,03 | 1,02 | 1,00 | 1,04 | 1,03 |

## 2. Le module

`water_cretes.wgsl`, concaténé avant `water.wgsl`. Entrées explicites : `s`, sa variation dans le pixel (`fwidth`, en
flot uniforme : le bord de l'écume est antialiasé sans paramètre), l'empreinte, la normale, le rayon. `σ²` : la bande
au sommet (`Σ (a·k)²/2` aux poids du filtrage), la queue filtrée au fragment, modulée comme sa pente. Paramètres dans
cinq vecteurs de plus de l'uniforme. **Options éteintes, les images sont celles d'avant, au bit** (Reproduire).

- **Écume** : `--ecume[=r]` ; couleur `r·E/π`, le gain d'ADR-177 tenant lieu de `E/π` pour le corps d'eau comme pour
  elle.
- **Lumière des crêtes** : `--cretes[=force,exposant]` ; masque `clamp(−s/2)`, diffusion vers l'avant en azimut du
  soleil, teinte **`exp(−a·Hs)`**, la transmission de l'eau pure sur l'épaisseur d'une crête, `a` de Pope & Fry (1997)
  — les absorptions mêmes de la couleur d'ADR-177 : (0,599 ; 0,919 ; 0,986), un cyan clair. Force 0,15 et exposant 4,
  **à calibrer par la revue** (I-14).

## 3. Ce que les images ont dit

- **Réflectance 0,22** (Koepke 1984, effective sur tout le mouton) : luminance ≈ 0,41, **grise**, plus sombre que le
  ciel reflété près de l'horizon — invisible en pose rasante. **0,55**, l'écume fraîche (Whitlock et al. 1982) : des
  taches blanches, sur la crête de la grande houle, là où la surface se comprime le plus. Aucun partage cœur / frange
  publié n'a été trouvé (Monahan & Lu 1990 nomment les stades A et B sans rapport chiffré) : **les deux restent à
  juger**.
- **Lumière des crêtes** : la première teinte, celle du corps d'eau (B/G 10,9), faisait des taches **bleu
  électrique** ; la transmission cyan donne un éclaircissement discret des crêtes comprimées.

## 4. Coût

`--cretes-bench`, 1280 × 720, 120 images, secteur 96 % avant et après ; médiane du GPU de l'eau :

| pose | sans | écume | crêtes | les deux |
|---|---:|---:|---:|---:|
| référence | 2,106 ms | +0,018 | +0,012 | +0,026 |
| rasante | 2,095 ms | +0,010 | +0,008 | +0,012 |

Le calcul de `σ` et de `fwidth(s)`, fait sans condition, est dans les quatre états : non séparé.

## 5. Ce qui n'est pas reçu

- **Aucun jugement de l'utilisateur** : R19 devait se tenir dans l'afficheur ; l'utilisateur a choisi Godot. Le point
  8.4 reste *absent*.
- L'écume **n'a pas de durée** — ni traînée, ni décroissance, ni transport par les vitesses (le style de la référence
  FluidNinja) —, **ni texture**, **ni source** hors de B : ni impacts de W, ni δ, ni sillage ; aucun embrun.
- La lumière des crêtes a deux paramètres non calibrés.
- **La suite** : le module porté dans le nuanceur d'eau du prototype Godot (ADR-192 D2), jugé là.
