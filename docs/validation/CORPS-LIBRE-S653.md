# Le corps libre que le rouleau emporte — S653 (listes 6.7, 6.4)

*S653, 2026-10-07, en autonomie, vers la v2.* Le point 6.7 attend « le corps libre que le rouleau emporte ». En S652, la sphère d'APIC
était fixe.

## Reproduire

- La flottaison, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core a_free_sphere_floats -- --nocapture`.
- Le corps sous le rouleau : `… the_plunging_roller_carries_a_free_body_s653 -- --ignored --nocapture`, depuis une copie du binaire
  (ADR-265 D1), ≈ 8 min.
- Les essais d'APIC 3D : 55 passent, 12 longs ignorés.

## 1. Ce qui est construit

- **`Apic3::body_force`** : l'instrument de S652, passé dans le cœur. C'est la pression extrapolée à la face, sur les faces du corps qui
  touchent l'eau.
- **`Apic3::set_body_mass`** : avec une masse, la sphère devient libre. À chaque pas, après la projection, `v += dt·(F/m + g)`, et le pas
  suivant impose cette vitesse à l'eau. Le couplage est **explicite**.
- **Un contact simple** : le corps ne descend pas sous le fond de sa colonne et ne sort pas du domaine ; la vitesse normale y est annulée.
- Sans masse, la sphère reste imposée, comme avant : les 55 essais passent.
- Refus : une masse nulle, négative ou non finie.

## 2. Mesuré

**La flottaison.** Une sphère de 0,1 m de rayon, densité 500 (2,094 kg), est lâchée 3 cm sous son équilibre, en eau au repos.

| seconde | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| `|v_z|` max, m/s | 0,232 | 0,130 | 0,073 | 0,041 | 0,020 | 0,057 | 0,033 | 0,021 |
| centre, m (niveau 0,40) | | | 0,385–0,391 | 0,386–0,388 | | | | 0,403–0,407 |

**Le rouleau.** La même sphère libre, posée à x = 10,4 m dans le relais de S652.

| | mesuré, 5 cm |
|---|---|
| retournement | 2,963 s, 10,575 m (2,592 s, 9,825 m avec la sphère fixe : le corps qui part avec l'onde change le déferlement) |
| avance dans les 1,5 s qui suivent | **2,03 m** |
| position à 4 s | x = 12,43 m, contre le mur du bout (12,7 m) ; z = 0,65 m |
| vitesse du corps au plus / de l'eau de la colonne au plus | **4,68 / 1,80 m/s** |
| la masse de l'eau (volume − entré) | −1,9·10⁻¹⁶ |

**Critères, écrits avant.**

- **(1) La flottaison, à moitié.** Le centre se tient à 2 cm du niveau (1,3 cm dessous à 3–4 s), et rien n'est non fini. Mais la vitesse
  entre 3 et 4 s vaut **4,1 cm/s pour une borne de 2 : manqué**. L'oscillation décroît de moitié environ chaque seconde ; le ballottement
  du bassin fermé la relance vers 6 s, puis elle redécroît. Elle est stable, sans croissance.
- **(2) Le rouleau, à moitié.** Le corps est emporté, de 2,03 m, et la masse est exacte. Mais **sa vitesse dépasse celle de l'eau : 4,68
  m/s contre 1,80, manqué.**
- **(3) Les refus : tenu.**
- **(4) Les essais d'APIC 3D inchangés : tenu.**

## 3. Ce que cela dit — et ne dit pas

Le rouleau d'APIC 3D **emporte** désormais un corps qui flotte : il le soulève et le porte de deux mètres vers la plage en une seconde et
demie. Le corps flotte à son tirant d'eau, et le couplage reste stable pour un corps deux fois plus lourd que sa masse ajoutée.

La vitesse du corps n'est pas physique. Il dépasse l'eau qui le porte, et le suspect est nommé : les chocs de deux pas de la force
(A334), qui frappent un corps léger. Le témoin suivant, celui de A334 : le même rouleau avec un pas deux fois plus court. Un choc physique
garde son impulsion ; un artefact dépend du pas.

Manquent : la vitesse du corps juste (A334), la rotation du corps, un corps plus léger que sa masse ajoutée (le couplage implicite).
