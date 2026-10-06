# B6 — combien de points pour le proxy de flottabilité — S499 (liste 6.1)

*S499, 2026-10-06, en autonomie.* ADR-008 §5.1 laissait ouvert « le nombre et le placement des points d'échantillon par archétype
(20 à 60) → benchmark B6 ». La porte D a pris 16 × 8 × 4 points sans mesure ; S494–S495 ont vu une bouée de 4 × 4 × 4 rouler et chavirer.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s499 -- --nocapture` — ≈ 1,5 s, 2 880 grilles. Suite
  du cœur : 659 essais.

## 1. Ce qui est mesuré

Pour chaque archétype d'ADR-008 §3 en pavé droit (navire 60 × 10 × 8 m, 1 200 t ; barque 3 × 2 × 0,6 m, 400 kg ; caisse 1 × 1 × 0,2 m,
50 kg) et chaque grille `nx × ny × nz` de 2 × 2 × 1 à 16 × 16 × 8 : la raideur de pilonnement ; la **hauteur métacentrique** en roulis et
en tangage, tirée du moment de rappel d'une inclinaison de 10⁻⁴ rad à l'équilibre en eau calme, contre l'analytique `GM = KB + BM − KG` ;
la houle vue par la flottaison (moyenne de `cos k·x` sur les centres) contre le continu `sin(kL/2)/(kL/2)`, pour `λ = 2L` et `2B`.

## 2. Le modèle de l'erreur, vérifié

`GM_proxy = BM·(1 − 1/n²) + z_F` : l'inertie de flottaison discrète de `n` centres, plus la hauteur `z_F` (rapportée au centre de gravité)
où s'appliquent les poussées — la couche partielle pousse **en son milieu**, pas au centre de sa part immergée. **La prédiction tient à
3·10⁻⁸** sur les 2 880 grilles des trois archétypes. La raideur de pilonnement vaut `ρgA` à 10⁻⁹ pour toutes.

## 3. Mesuré

| archétype | GM roulis / tangage (analytique) | 4 × 4 × 4 | plus petit proxy (GM à 5 %, houle à 1 %) | sans compensation |
|---|---|---|---|---|
| navire | 1,246 / 150,7 m | −19,5 % / −6,4 % | **7 × 8 × 4 = 224** | 7 × 10 × 8 = 560 |
| barque | 4,858 / 11,26 m | −5,7 % / −6,0 % | 7 × 7 × 1 = 49 | 7 × 7 × 4 = 196 |
| caisse | 1,633 / 1,633 m | −6,5 % / −6,5 % | 7 × 7 × 1 = 49 | 7 × 7 × 3 = 147 |

- **Le navire sort des 20 à 60 points d'ADR-008** : son `GM` de roulis (1,25 m) est petit devant `BM` (4,27 m), et l'erreur `1/n²` sur
  `BM` y pèse `BM/GM` = 3,4 fois. Le terme concurrent était `GM`, pas `BM` (ADR-226 D3).
- **Une seule couche ne tient que par compensation** : `BM` discret trop faible de 2 %, poussée appliquée trop haut de 4,6 % de `GM`
  (le milieu de la boîte au lieu du centre de carène). « Sans compensation » exige chaque terme seul : `BM` à 4 % de GM, `z_F` à 1 %.
- La houle impose `n` ≥ 7 dans chaque direction pour 1 % à `λ = 2L`.

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) raideur `ρgA` à 10⁻⁹, toute grille | tenu | tenu |
| (2) l'erreur suit `1/n²` et la couche partielle à 10⁻³ | 3·10⁻⁸ | tenu |
| (3) le plus petit proxy par archétype, publié, comparé aux 20 à 60 | 224 / 49 / 49 ; sans compensation 560 / 196 / 147 | publié |

## 5. Ce que cela désigne

Tout le surcoût des couches vient de la couche partielle qui pousse en son milieu. **Pousser au centre de la part immergée** rendrait `z_F`
exact à une seule couche : le proxy ne paierait plus que `1/n²` sur `BM` et la houle — 7 × 10 × 1 = 70 points pour le navire, 49 pour la
barque et la caisse. C'est la suite (S500). **6.1 reste partielle** : l'amortissement des autres degrés de liberté ; B6 publié, à refaire
après ce remède.
