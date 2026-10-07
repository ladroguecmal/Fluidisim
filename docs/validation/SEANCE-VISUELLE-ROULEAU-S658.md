# La séance visuelle du rouleau — S658 (R40)

*S658, 2026-10-07, sur la demande de l'utilisateur* : *« J'aimerais des sessions visuelles grâce à toutes les nouvelles avancées »*.

## Reproduire

1. L'enregistrement (≈ 8 min), dans `code/water-core` :
   `cargo test --release --offline -p water-core record_the_roller_for_the_visual_session_s658 -- --ignored --nocapture`.
   Il écrit `calculs/s658_rouleau.bin` (73 Mo, non versionné).
2. Le contrôle : `python outils/rendu_rouleau.py --controle calculs/s658_rouleau.bin "<t x z>"…`, avec les lignes « S658 trajet » de
   l'étape 1.
3. Le rendu : `python outils/rendu_rouleau.py calculs/s658_rouleau.bin calculs/s658`. Il écrit `calculs/s658_plage.gif` (1,2 Mo) et
   `calculs/s658_rouleau.gif` (0,9 Mo).

## Ce qui est montré

Le montage complet de S650 à S657, sur 4 s, une image toutes les 0,04 s :

- Saint-Venant 2D porte l'onde solitaire (`H/d` = 0,3) au large ;
- APIC 3D la reçoit à `x_r` = 5,0 m, par la zone des colonnes, puis sur la pente 1:12 ;
- la vague se retourne et plonge ;
- la sphère libre (densité 500) est soulevée puis emportée vers la plage.

Ce rendu d'atelier sert à juger la physique, non le réalisme final (ADR-262, ADR-216). Toute la largeur du canal est projetée de côté, et
la couleur dit la vitesse, bornée à 3 m/s.

## Contrôles (ADR-266)

| | attendu | relu |
|---|---|---|
| images, durée | 100, de 0,04 à 4,00 s | 100, de 0,040 à 4,000 s |
| particules à la fin | celles du calcul : 46 521 | 46 521 |
| la sphère, enregistrement contre relevé du calcul | à 1 mm | à **0,048 mm** |
| taille des fichiers | sous 15 Mo | 1,2 et 0,9 Mo |

**Vu avant l'envoi, et corrigé.** Le premier rendu laissait vide la zone des colonnes, dont la surface n'était pas enregistrée. Les
particules y étaient aussi trop fines, et un caractère de la légende ne s'affichait pas. L'enregistrement a été refait avec la surface des
colonnes.

Les images vues, aux instants clés :

- 2,20 s : l'onde sur la pente, le relais continu ;
- 2,61 s : la vague se lève devant la sphère ;
- 2,85 à 2,93 s : la crête s'enroule par-dessus la sphère ;
- 3,24 s : la sphère portée sur le front du ressaut, des gouttes rapides devant.

## Le verdict

**R40, envoyé le 2026-10-07 — reçu le même jour** : *« tout parait crédible »*. Le déferlement plongeant, le jet, la sphère emportée et le
relais 2D → 3D sont jugés crédibles à l'œil, sur le rendu d'atelier. Ce verdict porte sur la physique montrée, non sur le rendu final de
Godot.
