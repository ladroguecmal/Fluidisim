# La séance visuelle du haut-fond de Berkhoff — S663 (R41)

*S663, 2026-10-07.* Une séance visuelle, comme l'utilisateur les a demandées (R40 reçu : « tout parait crédible »). Elle montre la houle sur
le haut-fond de Berkhoff, des sessions S659 à S662.

## Reproduire

1. `cargo test --release --offline -p water-core record_berkhoff_for_the_visual_session_s663 -- --ignored --nocapture`, depuis
   `code/water-core` (≈ 1 s). L'essai écrit `calculs/s663_berkhoff.bin` (les trois modèles à 5 cm) et `calculs/s663_controle.csv`.
2. `python outils/rendu_berkhoff.py calculs/s663_berkhoff.bin calculs/s663_controle.csv calculs/s663`. Il écrit `s663_carte.png`,
   `s663_sections.png` et `s663_surface.gif` (6,5 Mo).

## Ce qui est montré

- **La surface animée** sur une période, `η = a₀·Re(A·e^(i(ψ − ωt)))`, avec le modèle non linéaire. Les crêtes se courbent au-dessus du
  haut-fond et se concentrent derrière lui. C'est le champ linéarisé, sans les harmoniques de l'onde non linéaire.
- **La carte de l'amplitude** : la focalisation derrière le haut-fond, double de l'incidente. La tache en bas à droite vient de la paroi
  latérale réfléchissante du modèle.
- **Les quatre sections** : les mesures de Berkhoff contre les trois modèles (S659, S660, S662).

## Contrôles (ADR-266, ADR-267)

| | attendu | relu |
|---|---|---|
| les écarts recalculés par le rendu | ceux de S662 (au millième) | 0,090 ; 0,106 ; 0,091 ; 0,101 — égaux |
| l'amplitude relue contre `Champ::amplitude` | 10⁻¹² (le plan) | **8,96·10⁻⁸ — la borne du plan manquée** |

La borne était mal posée. Le champ est écrit en `f32` (un écart de 10⁻⁷ à l'amplitude 1), alors que le plan exigeait 10⁻¹² sans tenir
compte de la précision de l'enregistrement. L'instrument est juste ; c'est la borne qui l'ignorait (famille 2, ADR-266).

**Vu avant l'envoi, et corrigé** : les accents s'affichaient en carrés (la police par défaut de PIL). Une police Windows, Segoe UI, les
porte.

## Le verdict

**R41, envoyé le 2026-10-07 — reçu le même jour** : *« Je valide, continue »*. La houle sur le haut-fond (la surface, la carte, les
sections contre les mesures) est validée à l'œil, sur le rendu d'atelier.
