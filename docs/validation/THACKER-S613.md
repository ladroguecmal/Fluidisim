# Le mouillage et le séchage en 2D, jugés sur Thacker — S613 (liste 4.14)

*S613, 2026-10-07, en autonomie (ADR-247).* 4.14 était absent : « plage : rouleau 3D, mouillage et séchage (C04) » — C04 ne tournait que
sur un véhicule 1D (la rupture de barrage sur lit sec). Cette session porte le mouillage et le séchage en 2D et les juge sur la solution
analytique de Thacker : une nappe plane qui tourne dans une cuvette paraboloïde, le rivage en mouvement (SWASHES).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s613 -- --nocapture` (9 s) ; suite du cœur : 802 essais
  listés.

## 1. Ce qui est construit

Un module `saint_venant_2d.rs` : Saint-Venant 2D en volumes finis d'ordre un, reconstruction hydrostatique d'Audusse (positive,
équilibrée), flux de Rusanov, murs aux bords, vitesse désingularisée (Kurganov–Petrova, `ε` = (1 mm)⁴) ; `pas` refuse Courant > ½.

## 2. Mesuré (références calculées au plan par `s613_ref.py`, numpy indépendant)

`a` = 1 m, `h₀` = 0,1 m, domaine de 4 m, `η` = 0,5, période 4,4857 s, `10·N` pas par période.

| | référence | mesuré |
|---|---|---|
| écart L1 de `h` après une période, 50² ; 100² ; 200² | 0,427088 ; 0,242185 ; 0,128642 | à 10⁻¹⁵ |
| rapports de convergence | 1,763 ; 1,883 (≥ 1,8 au plus fin) : l'ordre un converge | idem |
| centres de masse à 100², à T/4, T/2, T | (2,0171 ; 2,4755), (1,5522 ; 2,0587), (2,3951 ; 1,9028) — exact (2 ; 2,5), (1,5 ; 2), (2,5 ; 2) | à 10⁻¹⁵ |
| masse ; positivité | < 10⁻¹³ relatif ; `h ≥ 0` à chaque pas | ≤ 2·10⁻¹⁵ ; tenu |
| le lac au repos (bords secs), 500 pas | vitesse < 10⁻¹⁴ m/s | 1,7·10⁻¹⁶ |
| refus | mailles, `dx`, `dt`, Courant > ½ | tenu |

Critères (écrits avant, amendés avant la mesure du code) : (1)–(5) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le rivage avance et recule sans hauteur négative, sans perte de masse, et le lac au repos à bords secs ne bouge pas : le mouillage et
le séchage tiennent en 2D. L'ordre un est diffusif — un quart d'écart après une période à 100², l'oscillation du centre de masse amortie à
82 % —, mais il converge vers Thacker à l'ordre attendu.

**En route** (amendement du plan, avant la mesure du code) : la vitesse `q/h` au-dessus de 10⁻⁶ m atteignait Courant 0,60 dans une maille
presque sèche ; désingularisée à la Kurganov–Petrova, Courant reste sous 0,19.

Manquent : **le rouleau 3D**, l'ordre deux (moins diffusif), le frottement, une houle incidente sur une plage réelle (C04 en 2D sur une
pente, le jet de rive), le branchement à δ et à la bibliothèque côtière (12.3).

> **Note du 2026-10-07 (S614).** Les murs de ce module n'exerçaient aucune pression : sans effet ici (Thacker et le lac ont des bords
> secs ; les références et les mesures ci-dessus sont inchangées, vérifié), mais faux à bord mouillé. Le flux de paroi `(0, ½gh², 0)` est
> ajouté en S614 ([preuve](GRAND-EVENEMENT-S614.md) §3).
