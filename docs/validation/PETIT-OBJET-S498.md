# Le petit objet léger — les régimes de flottabilité — S498 (C11, liste 6.1)

*S498, 2026-10-06, en autonomie.* ADR-008 §3 : « la flottabilité d'un petit objet léger est numériquement plus difficile que celle d'un
porte-conteneurs ». Le corps rigide (S331) n'avait qu'un régime, le pas symplectique, qui diverge dès `ω·dt > 2`.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s498 -- --nocapture` — trois essais, < 1 s. Suite du
  cœur : 658 essais.

## 1. La construction (`code/water-core/src/rigid_body.rs`)

- `heave_stiffness` : la raideur de flottaison mesurée sur le proxy, `ρg·Σ V/e` sur les points dans leur rampe (`ρg·A` exactement pour un
  pavé droit) ; `equilibrium_offset` : l'altitude d'équilibre du centre au-dessus de la surface, par dichotomie.
- `floating(dt)` → `Floating { omega, regime, offset }`, calculé à la création (ADR-008 §3) : **normal** jusqu'à `ω·dt` = 0,3, **sous-cyclé**
  (2 à 4 sous-pas) jusqu'à 1, **contraint** au-delà.
- `step_floating` : le pas du régime. Contraint : aucune force de flottabilité ; la vitesse horizontale relaxée vers celle de l'eau au taux
  `ω`, la position avancée puis projetée sur la surface de l'eau à la fin du pas — `z = η + c`, l'axe sur la normale (`surface_tilt`).

## 2. Mesuré

| archétype (ADR-008 §3, en pavé) | ω (= `√(ρgA/m)` à 10⁻⁹) | `ω·dt` à 30 Hz | régime |
|---|---|---|---|
| navire 60 m, 1 200 t | 2,242 rad/s | 0,075 | normal |
| barque, 6 m², 400 kg | 12,28 | 0,409 | sous-cyclé ×2 |
| caisse, 1 m², 50 kg | 14,18 | 0,473 | sous-cyclé ×2 |
| balle de ping-pong, 2,7 g | 69,58 | 2,32 | **contraint** |

(Masse ajoutée nulle ici : barque et caisse plus raides que dans le tableau d'ADR-008, qui la compte.) Les bascules tiennent de part et
d'autre des seuils 0,3 et 1, à 10⁻⁶ près.

| essai | mesuré |
|---|---|
| la balle contrainte, 120 s de houle de B (5 cm) à 30 Hz : `max\|z − (η + c)\|` ; axe contre normale | **0** ; 3,5·10⁻¹⁸ rad |
| la même, 10 s près d'un impact de W d'1 kJ | **0** ; 4,2·10⁻¹⁷ rad ; aucun refus |
| sa vitesse horizontale contre celle de l'eau (après 1 s) | 2,0·10⁻⁴ m/s |
| **le témoin** : la balle au pas normal à 30 Hz, 0,1 µm sous l'équilibre | **×3,052 par pas** (prédit `1 − x²/2 − √((1 − x²/2)² − 1)` = 3,052) |
| navire, barque, caisse lâchés 1 cm haut, 120 s : `\|G\|` par période | **1 à 10⁻¹²** (résidu de l'ajustement 2·10⁻¹⁵ m) |

**La mesure de `|G|`.** Les maxima interpolés par une parabole, essayés d'abord, portent un bruit d'échantillonnage de 6·10⁻⁴ : la barque
y sortait à 1 + 1,6·10⁻⁹, au-dessus du seuil écrit avant — un défaut de l'instrument, pas du pas. Le seuil n'a pas bougé ; l'instrument a
changé : une application linéaire de module 1 rend une sinusoïde exacte aux pas, de pulsation `cos(ω'·h) = 1 − (ω·h)²/2` ; l'enveloppe
ajustée à cette pulsation la mesure au résidu près. Le premier témoin, à 1 mm, sortait la balle de l'eau dès le premier pas (sa rampe
fait ±4,5 mm) : 0,1 µm la garde linéaire huit pas.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) `ω` à 10⁻⁹ ; bascule aux seuils 0,3 et 1 | tenu | tenu |
| (2) contraint : `max\|z − (η + c)\| = 0`, axe à 10⁻¹² rad, sur B 120 s et sous W | 0 ; ≤ 4,2·10⁻¹⁷ | tenu |
| (3) le témoin diverge, ≈ 2,7 par pas | 3,052 = la prédiction recalculée à `ω·dt` = 2,32 | tenu |
| (4) `\|G\| ≤ 1 + 10⁻⁹` | 1 à 10⁻¹² | tenu (après un instrument refait, §2) |

**C11 tenu** (ses assertions réécrites en S30). **6.1 reste partielle** : l'amortissement des autres degrés de liberté, B6 (le nombre de
points par archétype).
