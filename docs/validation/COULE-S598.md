# La descente d'un objet qui coule et l'enveloppe de son domaine — S598 (liste 4.4)

*S598, 2026-10-07, en autonomie (ADR-247).* 4.4 était absent : « profondeur adaptative, domaine qui suit un objet qui coule ». Première
pièce : le plan — la descente prévue, l'enveloppe verticale du domaine ; pas encore l'exécution dans δ.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s598 -- --nocapture` ; suite du cœur : 773.

## 1. Ce qui est construit

`coule.rs` : `Chute` — la descente d'une sphère plus dense que l'eau (la masse ajoutée, Schiller–Naumann puis Newton au-delà de `Re` =
1 000), RK4 à pas fixe ; `Enveloppe` — le bas du domaine sous l'objet **prévu** à l'horizon d'anticipation, plus une marge, au quantum, jamais
remonté, les agrandissements comptés.

## 2. Mesuré (références écrites au plan par son script, avec son propre code)

Une boule d'acier de 0,1 m de rayon dans l'eau de mer, lâchée à la surface.

| | référence | mesuré |
|---|---|---|
| la vitesse terminale (Newton, `Re` = 1,3·10⁶) | 6,268812 m/s | **6,268812 m/s** |
| la profondeur à 1, 2, 5, 10 s | 3,23120 ; 9,16062 ; 27,93725 ; 59,28130 m | **identiques** (10⁻⁴) |
| l'enveloppe (anticipation 2 s, marge 0,5 m, quantum 1 m, mise à jour toutes les 0,1 s) | l'objet dedans avec sa marge, jamais remonté ; 64 agrandissements, le bas à 73 m (comptés par le script avant l'essai) | **tenu ; 64, 73 m** |
| moins dense que l'eau, rayon nul, quantum nul | refus | tenu |

Critères (écrits avant) : (1)–(4) — **tenus**. Le plan annonçait un compte fait par le script sans l'avoir fait : comblé aux notes avant
l'essai.

## 3. Ce que cela dit — et ne dit pas

On sait, à l'avance, jusqu'où un domaine δ doit descendre pour garder un objet qui coule, et quand. **Un constat** : 64 agrandissements en
10 s (un toutes les 0,16 s) — trop pour δ, dont chaque changement de niveau transfère un état (ADR-210) ; l'enveloppe devra grandir par
paliers plus larges (une hystérésis, ou le quantum qui double). Manquent pour 4.4 : l'exécution dans δ, un objet qui remonte, les
objets non sphériques, la politique de paliers.
