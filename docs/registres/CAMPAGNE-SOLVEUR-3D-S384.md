# La campagne du solveur volumique 3D temps réel — conception — S384

2026-09-26. **Décision de l'utilisateur** (S379) : *« une session du plus dur et complexe de la création d'un solveur […]
qui va s'occuper des simulations 3D volumétriques ultra réalistes et performantes en temps réel dynamiquement »* ; R30 :
*« ensuite le plus important le solveur 3D »* ; S384 : *« Solveur 3D ici »*. La [feuille de route](../FEUILLE-DE-ROUTE.md)
§3 ter demande **la conception d'abord** : état de l'art, ce que δ 3D et APIC donnent déjà, l'architecture, les cibles
chiffrées, le découpage en sessions.

Ce document est une **conception**, pas une preuve : il ne mesure rien de neuf. Chaque chiffre renvoie à la preuve qui
l'a mesuré ou à la publication qui l'annonce, marqué **publié** (annoncé par ses auteurs, non reproduit ici), **mesuré**
(par ce dépôt, lien) ou **estimé** (calcul de ce document, dit). Ce qui s'y décide est acté par
l'ADR de la campagne ; ce qui demande l'utilisateur est au §6.

---

## 1. Ce que le dépôt a déjà

### 1.1 δ 3D en colonnes — la représentation par défaut

Grille MAC x-y-z, surface **fonction hauteur** `η(x, y)` à fluide fantôme, pression scindée `p_hydro + p_dyn`, bandes
de couplage à B/W sur les quatre côtés, éponge ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D5).

| ce qui existe | où | chiffre, et sa preuve |
|---|---|---|
| **référence CPU** (`Volume3`, modes linéaire et mobile) | `water-core/src/delta3d*.rs` | réceptions 2D tenues à `ny` = 1 ; onde oblique d'une cuve, dispersion tenue ([S297](../validation/DELTA3D-COUPLEE-S297.md), [S298](../validation/DELTA3D-FOND-REEL-S298.md)) |
| **production GPU**, pas résident, travail borné | `viewer/src/delta3d*.rs`, `delta3d*.wgsl` | suit la référence à **3·10⁻⁷ m** (cas 3), **< 10⁻⁴ m** (cas 1, 2) pour 3 mm exigés ([S305](../validation/CUVE-GPU-S305.md) §9) |
| **coût** : porte C | scène `Config::review` : **120 × 112 × 28 mailles de 25 cm** (30 × 28 × 7 m, 376 320 mailles) | pas entier 4,5 ms (32 cycles de gradient conjugué préconditionné par Jacobi) ; **1,92 ms au 99ᵉ centile par image** à 30 Hz, un pas en deux parts ; projection = 0,087 + 0,062 ms par cycle, bornée par la mémoire (*estimé*) ([S341](../validation/COUT-DELTA3D-S341.md) §2, §11) |
| rendu en direct, interpolé à 30 Hz | afficheur | **1,49 ms** par image avec le rendu ([S341](../validation/COUT-DELTA3D-S341.md) §12) ; R16, R17 |
| **ordonnanceur** : deux domaines 3D, déplacement, redimensionnement, rang 1 | `water-core/src/scheduler.rs`, `viewer/src/delta3d_arbitrage.rs` | aucune image affamée contre 612 ; q99 **4,983 ms pour 5** ([S344](../validation/ARBITRAGE-3D-S344.md) §5–7) |
| **faces coupées** : fond, solide quelconque fixe ou mobile, corps rigide du jeu | `delta3d_cut.rs`, `rigid_body.rs` | ordre **1,956** sur une bosse ; Archimède exact au niveau discret ; `C_m` sphère **0,508** ; C10 tenu ([S324](../validation/FACES-COUPEES-3D-S324.md), [S331](../validation/CORPS-RIGIDE-S331.md), [S336](../validation/RAYONNEMENT-COQUE-S336.md)) ; sur la carte : le pas linéaire, solide fixe ([S358](../validation/LINEAIRE-GPU-S358.md)) |
| **masse** : compteurs exacts | `delta3d_balance.rs` | exacte par télescopage ; cuve fermée, dérive 7,4·10⁻¹² m sur 5 s ([S310](../validation/BILAN-MASSE-S310.md)) |
| **contenant** : bassin en δ, masse à V | `examples/piscine_delta.rs` | niveau de V suivi à **0,1 µm** ; 40 × 20 × 9 à 20 cm, **27 ms par pas** sur la référence CPU, dynamique de quelques millimètres, **invisible** ; il faut 5 à 10 cm, donc la carte ([S375](../validation/PISCINE-DELTA-S375.md)) |

### 1.2 APIC — la seconde représentation, sur banc 2D

Particules sur la grille MAC de δ, la même pression ; surface reconstruite des particules (Zhu et Bridson 2005) ;
**choisie** par l'utilisateur, **pas reçue** ([ADR-186](../adr/ADR-186-apic-seconde-representation.md) §4).

| ce qui existe | chiffre, et sa preuve |
|---|---|
| comparaison au même niveau contre ensemble de niveaux et SPH faiblement compressible | masse exacte, aucune énergie créée, ballottement à **0,15 %** de période, le moins cher : 10 s par seconde simulée contre 27 et 405 ([S318](../validation/COMPARAISON-LOT5-S318.md)) |
| B10 : un cylindre entre dans l'eau | cavité qui se pince à `t ≈ 2,2–2,5 √(D/g)`, à 10⁻⁴ d'une échelle à l'autre, **5 %** d'une maille à sa moitié ; couronne et jet **non convergés** (40 à 60 %) : sans tension de surface, la maille les arrête ([S320](../validation/B10-APIC-S320.md)) |
| raccord particules ↔ colonnes | masse exacte ; **frontière non reçue** : surface décalée de 1,8 à 2,8 mailles, ballottement amorti jusqu'à 16 % par période (S325) ; sur 30 s, la frontière ne tient pas la **densité** des particules, la masse migre (A316, [§13](../validation/B10-APIC-S320.md)) |

**Ce qu'APIC n'a pas vu** : la troisième dimension, la carte graphique, une référence expérimentale.

### 1.3 Les limites connues, qui commandent la conception

| limite | ce qu'elle dit | registre |
|---|---|---|
| **une seule surface par colonne** | cavité, jet, déferlement, gerbe, lame de déversoir : hors de portée des colonnes | ADR-175 D5, ADR-186 |
| **A297** — bascule de mouillure | grain de maille 2,0–2,2 mm RMS, du schéma, inchangé de 32 à 512 cycles | [file](QUESTIONS-OUVERTES.md#file-active) |
| **A298** — écart séculaire carte / référence | ≈ 1,2·10⁻⁷ m par seconde sur la cuve ; suspect non démontré | file |
| **A316** — densité des particules à la frontière | la masse migre de part et d'autre du raccord | file |
| **A320** — perturbation qui croît sous houle raide | 0,05–0,06 s⁻¹ sous `ak` = 0,079, convective ; bloque l'ordre E du retour δ → W | [S369](../validation/MER-S369.md) |
| **pression sans multigrille en 3D** | 32 cycles de gradient conjugué, déjà déclarés dégradés ; la multigrille n'existe qu'en 2D mobile ([ADR-167](../adr/ADR-167-multigrille-du-mode-mobile.md)) | S341 §3 |
| **une boîte dense par domaine** | 376 320 mailles pour 30 × 28 × 7 m ; aucune grille éparse (B5 non fait) | ADR-175 D6 |
| **δ n'est pas dans Godot** | la production vit dans l'afficheur ; *« pas maintenant »* (R27) — c'était le 2026-09-26, avant R30 | ADR-202 |
