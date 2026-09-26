# Le bassin de la piscine en δ 3D — S375

2026-09-26. Décision de l'utilisateur : *« La dynamique de fluide doit se faire en 3D volumétrique »*
([ADR-200](../adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md)) ; premier pas d'ADR-200 D4 sur la piscine de
[S374](PISCINE-V-S374.md). Liste **5.10** (articulation V↔δ), porte **E**. Référence CPU du cœur, machine de référence.

## Reproduire

- Commit `62a7adfc` (P4 de S375) ou plus récent ; Godot 4.4.1 (`<godot>`).
- **Le cœur** : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core` — 539 réussis, 14 ignorés,
  une minute ; en particulier `column_volume_is_added_exactly_and_atomically_s375`,
  `a_uniform_rise_is_a_state_without_motion_s375`, `shifting_rest_to_the_mean_level_changes_no_physics_s375`.
- **La piscine** : `cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example piscine_delta` —
  **six minutes** (13 200 pas de δ) ; progression toutes les 30 s simulées ; lignes `PISCINE_DELTA_S375` : `cout
  domaine=40x20x9 dx=0.2 pas_delta=13200 … ms_par_pas≈27 iterations_moyennes=62 iterations_max=91 refus_repris=0`,
  `critere=2 ecart_niveau_max_mm=0.0000 … tenu`, `critere=3 distance_m=6.51 arrivee_s=2.475 ondes_longues_s=1.759
  ecart=40.7% manque`, `export images=6600 saturees=0`. Écrit `godot/donnees/piscine_delta.{json,bin}` (10,6 Mo, dérivés).
  `DX=0.1` : la maille de 10 cm, deux heures. Exige l'export de V d'abord (`--example piscine_v`, S374).
- **Godot** : `<godot> --path godot res://piscine.tscn --quit-after 3000 -- --controle-piscine` —
  `CONTROLE_PISCINE_S375 critere=4 hauteurs_lues=460 ecarts=0 tenu` (et le critère 3 de S374 sur le bac tampon ; `DELTA=0`
  : entier). Images : `VUES=… INSTANTS=… … -- --captures` ; `EXAGERE=100` est un **témoin de débogage**.

## En une phrase

Le bassin de la piscine est un domaine δ 3D dont V garde la masse — le jet de la pompe y entre en volume et en quantité
de mouvement, le déversoir en retire, le niveau suit celui de V à 0,1 µm près —, et sa surface calculée est rendue dans
Godot ; mais à la maille de 20 cm que permet la référence CPU, sa dynamique tient en quelques millimètres et **ne se voit
pas** à l'échelle réelle.

## 1. Ce qui est construit

- **`Volume3::add_column_volume`** : du volume par colonne entre deux pas, compensé comme le transport, pression et
  vitesses intactes, refus atomiques. **Critère 1** : 200 ajouts, écart de volume **nul au bit**.
- **`Volume3::shift_rest`** : le niveau de repos suit celui de V ; la pression des mailles mouillées perd `ρ·g·Δ` avec le
  fantôme de surface, le départ chaud reste exact. Essai : deux domaines, repos gardé et repos déplacé, surfaces à **un
  ulp** après 400 pas (le seuil écrit d'abord, 10⁻⁷ m, était sous la résolution f32 à 2 m, 2,4·10⁻⁷ : réécrit en deux
  ulps, et dit) ; pression moyenne 78,4 → −2,0 Pa.
- **`Volume3::last_refused_report`** : le rapport de la dernière projection refusée, pour le diagnostic.
- **`examples/piscine_delta.rs`** (la piscine de `support/piscine.rs`, partagée avec `piscine_v`, dont l'export reste
  identique au bit) : domaine 40 × 20 × 9 à 20 cm sur l'intérieur du bassin, pas de 25 ms, quatre par pas de V. À chaque
  pas de δ : le **jet** — son débit sur une gaussienne de 20 cm, sa quantité de mouvement **verticale** (vitesse de sortie
  `Q/A` de la buse, chute libre) dans les 60 cm du panache ; le **puits** du déversoir sur la bande contre le mur est ; le
  **forçage** vers V (`(V_V − V_δ)·dt/τ`, τ = 1 s) ; le **repos** sur le niveau de V. Export 20 Hz, entiers de 16 bits en
  dixièmes de millimètre.
- **Godot** : un maillage de hauteur aux centres des colonnes (`piscine.gd`), deux textures mélangées dans
  `bassin.gdshader`, pentes par différences centrées.

## 2. Deux défauts de la référence mobile 3D, trouvés par la piscine

1. **Une élévation uniforme était refusée**, sur tout domaine, dès 1 µm : le critère de divergence divisait deux arrondis.
   [ADR-201](../adr/ADR-201-plancher-de-l-echelle-de-vitesse-de-la-projection.md) : plancher de 10⁻⁴ m/s à l'échelle de
   vitesse ; 76 essais δ 3D inchangés ; essai nouveau (vitesses 10⁻¹⁰–10⁻⁹ m/s, surface uniforme au bit).
2. **Un pas calme refusé de justesse** à t = 305,4 s (divergence 1,06·10⁻⁵ pour 10⁻⁵, plancher f32 atteint) : le repos
   d'origine laissait 8,5 mm de décalage, 83 Pa uniformes qui consommaient la précision. Corrigé par `shift_rest`.

## 3. Le jet : ce qui a été essayé

**Premier essai** — la quantité de mouvement entière (≈ 56 N) dans les 30 cm du haut, sans dissipation : la surface sort
du domaine à t = 7,4 s. δ n'a ni turbulence ni viscosité, et l'énergie du jet (≈ 30 W pour sa seule composante
verticale) n'a pas de puits. **Retenu, à calibrer** : un panache (σ 20 cm, 60 cm de profondeur), la verticale seule,
l'horizontale dissipée ; amortissement de 0,5 s dans le panache, de 30 s partout (parois, viscosité, rides). Sans
référence mesurée, ces constantes ne sont que des ordres de grandeur.

## 4. Les critères

| critère | mesure | |
|---|---|---|
| 1 — volume par colonne | 200 ajouts | écart **nul au bit** — tenu |
| 2 — niveau de δ contre celui de V, au-delà de 5 τ | 13 200 pas | ≤ **0,0001 mm** — tenu |
| 3 — front de l'onde née de l'impact au mur est (6,51 m), premier dépassement de 1 mm, contre `d/√(g·h)` = 1,76 s, ±15 % | **2,475 s** (+40,7 %) | **manqué** |
| 4 — hauteurs chargées dans Godot contre le fichier | 460 valeurs | 0 écart — tenu |

**Le critère 3, relu sur l'export** (le fait reste manqué ; ceci est son explication) : au seuil de 0,3 mm, le front
arrive à **1,70 s (−3 %)** ; aux seuils de 0,1 et 0,03 mm, un précurseur arrive plus tôt que `√(g·h)` (1,50 et 1,20 s) —
la réponse instantanée d'un fluide incompressible, de faible amplitude. Le seuil de 1 mm était trop haut pour une onde de
quelques millimètres à 6,5 m.

**Comptabilité de masse avec et sans δ (C21)** : V n'est jamais lu en retour de δ — sa trajectoire est la même, par
construction ; le volume de δ suit le sien à 0,1 µm de hauteur près.

## 5. Ce qu'on voit

Cartes de hauteur (`viewer/captures/s375/cartes_de_hauteur_delta.png`, écart au niveau moyen, ±3 mm) : un dôme à l'impact
(5,3 s, le volume arrive d'abord), puis un creux (5,6 s, −4,7 mm), des anneaux qui s'étendent et se réfléchissent
(6,5–8 s), des interférences (40 s), un creux stable sous le jet en régime (200 s), des oscillations résiduelles après
l'arrêt (260 s). Écart-type 0,2 à 0,46 mm.

**À l'échelle réelle, la surface paraît plane** (`echelle_reelle_*.png`) : quelques millimètres sur des mailles de 20 cm
donnent des pentes de quelques millièmes. **Le témoin `EXAGERE=100`** (`debogage_hauteurs_x100_6s5.png`, jamais une image
de rendu) montre le creux et les anneaux : la chaîne de rendu est bonne ; c'est l'amplitude qui manque.

## 6. Limites, et ce qu'il faut pour voir

- **La maille de 20 cm** : la référence CPU séquentielle coûte 27 ms par pas (0,5 s à 10 cm). Une dynamique visible — le
  bouillonnement sous le jet, des rides de quelques centimètres — demande **5 à 10 cm**, donc **δ sur GPU** : la production
  existe dans l'afficheur (porte C, 1,5 ms par pas) ; la porter en nuanceur de calcul dans Godot, comme la FFT et l'écume,
  ne demande aucun téléchargement.
- **Le panache** : constantes à calibrer sur une mesure de jet plongeant (enfoncement, bouillonnement de surface).
- **Le jet dans l'air, la lame du déversoir, les bulles** : APIC (ADR-200 D3).
- **Le bac tampon** reste plan (V seul) ; rejeu d'un scénario, pas de commande en direct.
- Une géométrie, un débit, une résolution ; la référence CPU, pas la production.

## 7. Revue

R27 ([revue](REVUE-VISUELLE.md) §32).
