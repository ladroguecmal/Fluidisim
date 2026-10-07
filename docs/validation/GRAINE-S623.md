# La graine d'un domaine substitutif — S623 (liste 4.11 ; ADR-022 §3, I-17, I-09)

*S623, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* La moitié de 4.11 laissée par S609 : la restauration depuis une
graine. ADR-013 §4 : un domaine substitutif né faux met une minute à s'établir (60,65 s, S610) — aucune fenêtre de prévision ne le couvre.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s623 -- --nocapture` ; suite du cœur : 810 essais listés.

## 1. Ce qui est construit

Un module `graine.rs` : `SeedState` (genre, paramètres de cuisson, la hauteur totale et la vitesse en f16, l'identifiant = l'empreinte du
contenu) ; `condense` — **compilé seulement pour un hôte de cuisson** (`cfg(test)` ou la fonctionnalité `cuisson` du `Cargo.toml`) : un hôte
de jeu n'a pas la fonction (L19) ; `restaurer` — refusé hors tolérance de paramètres, la forme renormalisée sur le volume du nœud V
(autoritaire) ; `choisir` — la graine la plus proche dans la tolérance, ou aucune : jamais de mélange de champs (I-09). Et
`Domaine1D::depuis_etat`.

## 2. Mesuré (références calculées au plan par `s623_ref.py`, numpy)

Le domaine de S609/S610 né au repos sous B, avancé 120 s, cuit.

| | référence | mesuré |
|---|---|---|
| volume : le nœud ; la graine ; restauré | 399 998 810 ml ; 400,00488 m³ ; 399,998810 m³ (< 1 ml) | 399,99881 m³ |
| restauré avec B décalé de 120 s : écart initial à B | 2,478·10⁻³ m (sous 5 mm) | identique au bit |
| établissement | **un pas (0,05 s)** — 60,65 s depuis le repos (S610) | 0,05 s |
| tolérance (`hs` ±10 %, `tp` ±0,5 s, `θ` ±0,1 rad, la même phase) | dedans : restauré ; chacun dehors : refusé | tenu |
| choisir `hs` = 0,21 ; 0,3 m parmi 0,1 / 0,2 / 0,4 / 0,8 | 0,2 ; aucune | tenu |
| l'empreinte : le même contenu ; un f16 changé | la même ; une autre | tenu |
| refus : volume de nœud nul, tableaux de taille fausse | | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**. La lib se construit sans avertissement avec et sans `cuisson`.

## 3. Ce que cela dit — et ne dit pas

Une graine change une minute d'établissement en un pas : la condition de faisabilité d'ADR-013 §4. La masse vient du nœud V, au ml près ;
la graine ne donne que la forme. Le refus de condenser à l'exécution n'est pas une consigne mais une absence : sans la fonctionnalité
`cuisson`, `condense` n'existe pas.

Manquent : le 2D (`SaintVenant2D`), la graine côtière de 12.3 branchée sur `SeedState`, la tolérance mesurée (banc B4), sha256 (SPEC-005
§7.3) au lieu de FNV-1a, la bascule d'un domaine δ réel.
