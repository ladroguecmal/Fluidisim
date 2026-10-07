# Le niveau du large imposé au bord — S622 (liste 11.3)

*S622, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* Un manque nommé en S614 : le domaine local recevait le tsunami en
condition initiale, non par son bord. Cette session écrit une frontière caractéristique génératrice et absorbante.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s622 -- --nocapture` (≈ 80 s) ; suite du cœur : 809 essais
  listés.

## 1. Ce qui est construit

`SaintVenant2D::pas_avec_bord(dt, t, extérieur)` (ordre deux ; refusé à l'ordre un, non éprouvé) : la face gauche reçoit l'invariant entrant
de l'extérieur `w⁺ = u_e + 2√(g·h_e)` et rend le sortant de la maille de bord `w⁻ = u₀ − 2√(g·h₀)` ; l'état fantôme `h = ((w⁺ − w⁻)/4)²/g`,
`u = (w⁺ + w⁻)/2` ; le flux de Rusanov fantôme–maille remplace la pression de paroi.

## 2. Mesuré (références calculées au plan par `s622_ref.py`, numpy)

Une impulsion d'onde longue de 2 mm (`σ` = 20 m) sur 10 m de fond, un bassin plat de 400 m fermé à droite, ordre deux. **Forcé** (l'impulsion
entre par le bord) contre **étendu** (le bassin commence 900 m plus tôt, l'impulsion posée dedans).

| | référence | mesuré |
|---|---|---|
| écart à la jauge (200 m) jusqu'à 120 s, rapporté à la crête — maille 1 ; ½ ; ¼ m | 0,02914 ; 0,01312 ; 0,00593 — divisé par plus de 2 à chaque raffinement, sous 1 % | 0,02914 ; 0,01312 ; 0,00594 |
| après la sortie de l'onde réfléchie : ce qui reste dans le domaine forcé | 1,2·10⁻⁸ ; 1,6·10⁻⁹ ; 1,0·10⁻⁹ m (< 10⁻⁵ de l'amplitude) | 1,2·10⁻⁸ ; 1,6·10⁻⁹ ; 1,9·10⁻⁹ m |
| un bassin au repos, l'extérieur au repos, 500 pas | vitesse < 10⁻¹⁴ m/s | 2,4·10⁻¹⁵ |
| l'accord avec numpy à 10⁻¹² m (critère 1) | | **tenu à la maille 1 ; manqué au-delà** (voir §3) |

Critères (écrits avant) : (2), (3), (4), (5) **tenus** ; (1) **tenu à 1 m, manqué à ½ et ¼ m**.

## 3. Ce que cela dit — et ne dit pas

Le large entre par le bord comme si le domaine s'étendait au large : l'écart au domaine étendu converge avec la maille — il est la diffusion
numérique que l'étendu accumule sur ses 150 m de trajet en plus. Et ce qui sort ne revient pas : il reste un milliardième de mètre.

**Le critère (1) manqué** : la crête de l'étendu diffère de numpy de 6·10⁻¹¹ m à ½ m ; l'écart et le reste de 10⁻⁸ et 10⁻⁹ m à ¼ m. Dans les
zones presque plates, le signe d'une pente minmod se joue au bruit d'arrondi : une différence d'un ulp au départ — l'exponentielle de numpy
et celle de Rust ne sont pas spécifiées au bit (A98) — change de branche et se propage. L'essai n'affirme que ce qui a tenu. **En route** (au
plan) : une onde solitaire de 2 cm, plus large que le bassin, puis une impulsion de 2 cm, où la propagation non linéaire au large s'ajoutait,
n'éprouvaient pas le bord ; à 2 mm, l'écart relatif plafonne et converge avec la maille.

Manquent : les trois autres faces, une frontière oblique, l'extérieur lu dans W en 2D, la remontée d'un tsunami entré par le bord.
