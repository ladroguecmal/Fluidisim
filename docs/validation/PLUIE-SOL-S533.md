# La pluie hors contenant : submersion, infiltration, ruissellement — S533 (liste 5.5)

*S533, 2026-10-06, en autonomie.* La pluie tombait dans les contenants (S378) et s'infiltrait depuis une flaque posée par l'auteur (S530).
Hors contenant, elle tombe sur le sol : elle entre, puis, quand le sol ne suit plus, remplit les creux et ruisselle.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s533 -- --nocapture`.

## 1. Le montage — trois pièces de V, aucune loi nouvelle

Une **rétention de surface** (0,5 mm sur 1 m², les creux du sol) reçoit la pluie (`Rain`) ; l'**infiltration** de Green–Ampt (S530) la
mène au **sol** ; son **débordement** (`Spill`, S489) part au-dehors — le **ruissellement**, que l'hôte dépose où le terrain le mène.

## 2. Mesuré — limon sableux (K = 10,9 mm/h, ψ = 11 cm, Δθ = 0,3), 30 mm/h pendant 2 h

| | mesuré | théorie |
|---|---|---|
| submersion, instrument déclaré (la rétention passe 1 ml) | **29,1 min** | Mein–Larson : 37,67 min |
| *diagnostic, après coup, sans critère* : la rétention passe 10 ml | 39,13 min | (la lame monte après 37,67 min) |
| rétention pleine (0,5 mm) — le ruissellement commence | 50,0 min | — |
| lame infiltrée à 2 h | **48,928 mm** | Green–Ampt décalé : 48,880 mm (écart **9,8·10⁻⁴**) |
| pluie = sol + rétention + ruissellement | 59 999 ml, exacte à chaque pas ; 10 572 ml ruisselés | — |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la submersion (la rétention passe 1 ml) à 1 % de `t_p` | −23 % | **manqué** — 1 ml est le quantum de V : la pluie (0,83 ml par pas) et l'infiltration arrivent par 0 ou 1 ml, et la rétention touche 2 ml par la quantification dès 29 min. Un seuil au plancher du bruit (ADR-234 D2, appliquée à un instrument que j'ai écrit sans l'éprouver contre le quantum) |
| (2) `F` à 2 h à 0,5 % de Green–Ampt décalé | 0,098 % | tenu |
| (3) la masse au millilitre ; le ruissellement nul avant la rétention pleine | exacte ; nul jusqu'à 50 min | tenu |

## 4. Ce que cela dit

La pluie hors contenant se compose de pièces de V déjà éprouvées : avant la submersion tout entre, ensuite le sol suit Green–Ampt décalé
à 0,1 % près, et l'excédent ruisselle quand les creux sont pleins — la masse exacte. **5.5 avance.** Le critère manqué est celui de
l'instrument, pas du modèle : la leçon (un seuil au quantum) va à la revue de S536. Manquent le calcul de l'exposition depuis les objets
posés, l'assèchement du sol, et la météo.
