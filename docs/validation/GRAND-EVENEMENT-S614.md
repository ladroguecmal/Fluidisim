# Un très grand événement, du large à la plage — S614 (liste 11.3 ; ADR-001 §3.1)

*S614, 2026-10-07, en autonomie (ADR-247).* 11.3 était absent : « très grands événements (tsunami, crash, très grand navire) :
macroscopiques au large, 3D locaux à l'interaction ». Le tsunami : le modèle macroscopique (S582) donne la hauteur au bord d'un domaine
local ; le domaine local (Saint-Venant 2D, S613) calcule la remontée sur la plage, jugée par la loi de Synolakis (1987).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s614 -- --nocapture` (≈ 5 s) ; suite du cœur : 803 essais
  listés.

## 1. Ce qui est construit

Un module `grand_evenement.rs` : `hauteur_au_bord` (la levée de Green du rayon de S582) ; `OndeSolitaire` ; `remontee_synolakis` ;
`Plage` — une bande de Saint-Venant 2D, fond plat puis pente, l'onde posée à la distance canonique du pied ; `cote_mouillee`. Et, dans
`saint_venant_2d.rs`, **la pression des murs** (voir §3).

## 2. Mesuré (références calculées au plan par `s614_ref.py`, numpy)

Un tsunami de 4,14 cm à 4 000 m de fond, levé jusqu'à 10 m ; plage 1:19,85.

| | référence | mesuré |
|---|---|---|
| hauteur au bord | 0,185146429 m (`H/d` = 0,0185, non déferlante) | à 10⁻¹² |
| Synolakis | R = 0,861419 m | — |
| remontée, maille 1 ; ½ ; ¼ m | 0,705290 ; 0,793451 ; 0,850126 m — croissante, −1,3 % au plus fin (seuil : dix quanta, 0,126 m) | au bit |
| masse ; positivité | < 10⁻¹³ ; `h ≥ 0` | ≤ 1,4·10⁻¹⁴ ; tenu |
| un bassin au repos à murs mouillés, 500 pas | vitesse < 10⁻¹² m/s | 1,1·10⁻¹⁵ |
| refus | `H`, `d`, pente | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Le large et la plage se parlent par une seule grandeur — la hauteur au bord —, et le domaine local converge vers la remontée de
Synolakis (0,705 → 0,793 → 0,850 m pour 0,861). **En route** : les murs de S613 n'exerçaient aucune pression ; sans effet à bords secs (S613
inchangé, vérifié), ils faisaient accélérer une maille de bord mouillée et la référence divergeait. Le flux de paroi `(0, ½gh², 0)` les
corrige ; le bassin au repos à murs mouillés le garde.

Manquent : le niveau macroscopique imposé au bord comme condition aux limites (ici, une onde solitaire de la hauteur donnée), le
déferlement (au-delà de `H/d` = 0,044 sur cette pente), le domaine local 3D, le crash et le très grand navire.
