# Un décor fixe qui perce la surface — S490 (liste 6.5)

*S490, 2026-10-06, en autonomie.* La référence CPU de δ 3D, mode linéaire, un décor posé sur le fond qui traverse la surface. **Le cas
général tient ; un mur aligné sur la grille fait échouer la projection — A327. 6.5 reste partielle.**

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s490 -- --nocapture` — la cloison en milieu de maille.
- `… s490 -- --ignored --nocapture` — le défaut (A327). Variables de diagnostic : `CLOISON_E` (demi-épaisseur), `CLOISON_Z0` (bas).

## 1. Le montage

Une cuve de 2,4 m × 0,2 m, 0,8 m d'eau (48 × 4 × 16 mailles de 5 cm) ; une cloison pleine largeur au milieu, **posée sur le fond et
dépassant la surface** (`configure_with_floating_solid` : le solide perce le couvercle ; un solide sur le fond est accepté). (1) Au repos ;
(2) une seiche dans la moitié gauche, `η = h + a·cos(π·x/L)`, a = 1 cm, sur 3,2 périodes ; la colonne du bord gauche donne la période.

## 2. Les mesures

| cloison | période (s) | théorie `ω² = g·k·tanh(k·h)` | écart | moitié droite au pire |
|---|---|---|---|---|
| **paroi en milieu de maille** (demi-épaisseur 7,5 cm) | 1,2111 | 1,2143 | **0,27 %** | **0** (au bit) |
| une maille (2,5 cm) | 1,2394 | 1,2440 | 0,37 % | 0 |
| paroi 1 µm **en deçà** d'un plan de la grille | 1,2312 | 1,2292 | 0,16 % | 0 |
| suspendue, un jour de 10 cm au fond | 1,2256 | 1,2292 | 0,29 % | 0,85 mm (l'eau passe dessous, attendu) |
| paroi **exactement sur** un plan de la grille, ou 1 µm **au-delà** | — | — | — | **la projection se dit dégradée au pas 307 (0,616 s)** |

Au repos, le repos au bit (50 pas). **Aucune eau ne traverse le décor** quand il descend jusqu'au fond : la moitié droite reste nulle au bit.

## 3. A327 — le mur aligné sur la grille

Une paroi plane qui tombe exactement sur un plan de la grille, ou qui en dépasse d'un liseré (le solide entre d'un micromètre dans la maille
d'eau voisine), fait échouer la projection linéaire à la demi-période de la seiche, quand la surface contre la cloison est à son extrême —
quel que soit le plafond d'itérations (100 000) : le gradient conjugué stagne. Un micromètre en deçà, tout tient. Premier suspect : le
couvercle en partie couvert (S334–S335) d'une colonne presque entièrement libre. **Un mur aligné sur la grille est le cas courant d'un décor
de jeu** : 6.5 ne se valide pas tant qu'A327 est ouverte.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) repos au bit | tenu | tenu |
| (2) la moitié droite sous 10⁻⁶ m | 0 au bit (cloison jusqu'au fond) | tenu |
| (3) la période à 3 % | 0,16 à 0,37 % selon la cloison | tenu |
| (4) un solide sur le fond accepté ? | oui | dit |
| 6.5 validée si (1)–(3) | tenus en milieu de maille — **mais pas pour un mur aligné sur la grille** (A327) | **non** — partielle |
