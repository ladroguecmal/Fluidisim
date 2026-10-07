# Saint-Venant 2D d'ordre deux — S620 (listes 4.14, 11.3)

*S620, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* L'ordre un de S613 est diffusif — un quart d'écart à Thacker après
une période à 100², une remontée sous-estimée de 18 % à la maille de 1 m (S614). Cette session écrit l'ordre deux.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s620 -- --nocapture` (≈ 25 s) ; suite du cœur : 808 essais
  listés.

## 1. Ce qui est construit

`SaintVenant2D::regler_ordre_deux(ε)` : reconstruction MUSCL (minmod) de `h`, `η = h + z`, `u`, `v` par direction, pente nulle aux mailles
de bord ; reconstruction hydrostatique d'ordre deux (Audusse 2004) — états de face reconstruits, fond de face `η − h`, terme source centré
`−g·h·Δz` ; Heun en temps ; tableaux alloués au réglage, jamais au pas (I-06). L'ordre un et ses références ne changent pas.

## 2. Mesuré (références calculées au plan par `s620_ref.py` et `s620_remontee.py`, numpy)

| | ordre un (S613, S614) | ordre deux | mesuré |
|---|---|---|---|
| Thacker, écart L1 après une période, 50² ; 100² ; 200² | 0,427 ; 0,242 ; 0,129 | 0,04783 ; 0,01573 ; 0,00588 — ×8,9 ; ×15,4 ; ×21,9 | à 10⁻¹⁵ |
| rapport de convergence 100 → 200 | 1,88 | 2,68 (≥ 2,5) | idem |
| masse ; positivité ; Courant | exacte ; `h ≥ 0` | exacte ; `h ≥ 0` ; ≤ 0,19 (Thacker), ≤ 0,41 (plage) | tenu |
| lac au repos à bords secs, 200 pas | 1,7·10⁻¹⁶ m/s | 3,1·10⁻¹⁶ (numpy) | 3,5·10⁻¹⁶ |
| remontée (Synolakis 0,8614 m), maille 1 ; ½ ; ¼ m | 0,705 ; 0,793 ; 0,850 | 0,806 ; 0,844 ; 0,875 | au bit |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

L'ordre deux divise l'écart à Thacker par 9 à 22 et converge plus vite ; sur la plage, la maille de 1 m fait mieux que l'ordre un à ½ m.
À ¼ m, la remontée dépasse Synolakis de 1,6 % — un quantum de cote : la loi elle-même est une approximation à quelques pour cent.
**En route** (au plan) : l'`ε` de S613, (1 mm)⁴, amortissait les couches minces du rivage et figeait l'écart à 0,030 dès 100² ; à
(0,1 mm)⁴, l'ordre deux converge — Courant reste sous ½ (mesuré sur la référence, ADR-254 D1). Les murs mouillés sont éprouvés par la plage
(ADR-254 D2).

Manquent : un limiteur moins diffusif que minmod, le frottement, la houle incidente sur une plage réelle, le rouleau 3D.
