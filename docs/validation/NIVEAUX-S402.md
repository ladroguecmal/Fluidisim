# Changer un domaine de niveau de `dx`, en référence — S402

2026-09-27. **C8c**, première part, de la campagne du solveur volumique 3D
([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5) : le rang 4 d'[ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md)
§4 — « descendre `dx` d'un niveau » — et le retour, mesurés par deux mécanismes contre le domaine fin tenu tout du long.
Session cloud, sans carte graphique. Liste **4.5**, **9.9** ; suite de [DOMAINE-EPARS-S401](DOMAINE-EPARS-S401.md) (C8b).

## Reproduire

- Commit `8630e9a0` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s402 -- --nocapture` — cinq essais, < 1 s ;
  lignes `S402 volume` et `S402 aller-retour` : §2.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_niveaux -- <bosse|source>
  <transfert|adr005>` — ligne `NIVEAUX_S402`, ≈ 6 min par cas (quatre en parallèle) : §3.
- `python outils/etat_projet.py --check` et `cd outils && python -m unittest test_etat_projet` : le contrôle des ADR (§1).

## En une phrase

Un domaine δ change de niveau de `dx` **sans saut visible** — 0,1 à 2 mm d'un pas à l'autre — par les deux mécanismes éprouvés ;
mais seul le **transfert d'état** garde ce que le domaine contient : sur une bosse de 5 cm, l'image reste à 1,8 mm du domaine
fin pendant le rang 4 et à 2,3 mm après le retour, quand le cycle de vie d'ADR-005 §5, qui fait naître le nouveau domaine à
δ = 0, la perd — 10,2 mm, puis 12,8. [ADR-210](../adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md) l'acte. En chemin,
ADR-005, qui porte ce §5, est restauré : son corps manquait au fichier depuis S35.

## 1. Trouvé en lisant le lot : ADR-005 avait perdu son corps

ADR-006 §3.2 dit qu'un changement de niveau « se traduit par une destruction/création de domaine, gratuite visuellement
(ADR-005 §5) » ; I-12 et ADR-012 §4 renvoient au même §5. **Ce paragraphe n'existait plus** : le commit de S35 (`c2eb75ba`)
avait écrit à la place d'[ADR-005](../adr/ADR-005-zone-de-transition.md) la note corrective qu'il devait lui ajouter — 202 lignes
effacées, le titre compris —, et celui de S39 (`16e48d60`) avait fait de même avec la sienne. Un audit par `git log --numstat`
ne trouve aucun autre ADR ni aucune spécification amputée. **Restauré** au bit, dans l'ordre : le texte de S16 (`c0df00f7`),
la note B-S26, la note B-S27 ; une note datée le dit. **Un contrôle** de `outils/etat_projet.py` tient désormais que chaque
ADR commence par son titre : vu échouer sur les versions réelles de S35 et de S39, tenu sur S16, sur la restaurée et sur les
autres ADR — un premier jet, trop strict, refusait neuf ADR intacts qui ont une ligne vide avant leur titre. Leçon L373.

**Ce que dit le §5 retrouvé** : création et croissance à δ = 0, coût visuel nul ; rétrécissement par « transduction δ→W puis
amortissement sur τ ≈ 0,3 s » ; destruction, « idem, τ ≈ 0,5–1,5 s selon l'énergie résiduelle ». Un changement de niveau fait
ainsi, **tout ce que le domaine contient est perdu** : seul ce qui sort par son bord passe à W.

## 2. Le transfert d'état entre niveaux

`Volume3::resample_from` (`code/water-core/src/delta3d_levels.rs`) : l'état d'un domaine recopié sur la même fenêtre à un autre
`dx`.

- **La surface** par **recouvrement** : chaque colonne d'arrivée reçoit l'intégrale, sur son aire, de la reconstruction
  **bilinéaire** des colonnes de départ qu'elle recouvre — hauteur compensée, pentes centrées (décentrées au bord) et terme
  croisé `∂²h/∂x∂y`, dont les intégrales sur leur colonne sont nulles : le volume se conserve exactement. Les positions se
  rapportent à la fenêtre d'arrivée.
- **Les vitesses**, trilinéaires au centre de chaque face d'arrivée ; moyennées sur `n × n` sous-faces quand l'arrivée est plus
  grossière — au rapport 2, exactement les faces de départ couvertes : le débit se conserve.
- **La pression de départ** remise à zéro ; murs refermés. Refus : étendue, repos, densité, gravité différents ; découpe ou
  ensemble épars.

| critère | résultat |
|---|---|
| **T1** — état uniforme, 25 → 50 → 25 cm | **tenu** : surface au bit ; vitesse uniforme au bit loin des murs |
| **T2** — volume à l'arrondi f64 | **tenu** à 25 ↔ 50 cm : écart **0,0** ; à 10 → 25 cm, −1,1·10⁻⁸ m³ = l'écart d'aire des deux fenêtres (0,1 m n'est pas exact en f32 : 3·10⁻⁸) — la **hauteur moyenne** se conserve au bit près |
| **T3** — aller-retour d'une onde de 8 m (16 mailles grossières), ≤ 1 % (prédiction 0,3 %) | **manqué d'abord : 1,031 %** — la reconstruction plane oubliait le terme croisé, 0,96 % calculé en 2D (`sin²(θ/4)`) ; avec lui, **0,304 %** ; à 16 m, 0,037 % : l'ordre trois sur cette mesure |
| vu échouer — sans pente | **10,20 %** (prédiction ≈ 10 %), ordre un (5,02 % à 16 m) |
| **T4** — refus | **tenu** ; rien n'est écrit |

## 3. Le banc — deux mécanismes contre le domaine fin

`delta3d_niveaux` : bassin de **24 × 16 m** à 25 cm (96 × 64 × 12 mailles), 2 m d'eau sous 1 m d'air, pas mobile de 20 ms,
multigrille. **La référence** : le domaine à 25 cm tout du long. **Le domaine qui change** : à 50 cm de 2 à 5 s (le rang 4),
à 25 cm avant et après. **L'image** : ce que le rendu montrerait, sur la grille fine — un domaine à 50 cm y est reconstruit par
le même transfert. **Le saut** d'un pas : la variation de l'image sur ce pas, moins celle de la référence. **L'écart** : l'image
contre la référence. **ADR-005 §5** : au passage, le nouveau domaine naît au repos, l'ancien continue sans la source et s'efface
linéairement en τ = 0,5 s ; la transduction vers W n'existe pas en référence — ce qui en sortirait est perdu ici, et c'est peu :
le contenu est à l'intérieur.

| cas | mécanisme | saut au passage | saut au retour | saut d'un autre pas, max | écart pendant la période à 50 cm | écart après le retour |
|---|---|---:|---:|---:|---:|---:|
| **bosse** 5 cm, σ = 1 m | **transfert** | **0,24 mm** | **0,13 mm** | 0,18 mm | **1,82 mm** | **2,32 mm** |
| bosse | ADR-005 §5 | 0,36 mm | 0,49 mm | 0,65 mm (le fondu) | 10,2 mm | 12,8 mm |
| **source** mobile (S401), 24 mm | transfert | 1,95 mm | 1,32 mm | 1,54 mm | 15,7 mm | 16,7 mm |
| source | ADR-005 §5 | 0,68 mm | 1,01 mm | 2,28 mm | 17,0 mm | 23,6 mm |

Quatre calculs en parallèle sur quatre cœurs, ≈ 6 min chacun.

| critère | résultat |
|---|---|
| **B1** — saut au passage 25 → 50 cm ≤ 3 mm sur la bosse (prédiction ≈ 1 mm) | **tenu : 0,24 mm** ; tenu aussi partout ailleurs |
| **B2** — saut au retour ≤ 3 mm | **tenu : 0,13 mm** ; partout |
| **B3** — publiés | ci-dessus |

**Ce que les chiffres disent.**

- **Aucun des deux mécanismes ne saute** : le transfert parce que l'état continue, le fondu parce qu'il est progressif — mais un
  fondu de 0,5 s change l'image de 0,65 à 2,3 mm à chaque pas, plus qu'un passage par transfert.
- **Le contenu fait la différence.** Sur la bosse, le transfert garde les anneaux que 50 cm résout (σ = 1 m, deux mailles
  grossières) : 1,8 mm, sous la tolérance d'image ; ADR-005 §5 les perd : 10 mm, un cinquième de l'amplitude.
- **Le rang 4 se voit sur ce que le niveau d'arrivée ne résout pas.** La source — des gaussiennes d'écart type 0,5 m, une maille
  grossière — coûte 15 à 17 mm aux deux mécanismes pendant la période à 50 cm : c'est le « visible de près » d'ADR-012 §4. Après le
  retour, le transfert repart de ce que 50 cm avait gardé (16,7 mm) ; ADR-005 §5 repart de rien (23,6 mm).

## 4. Ce que ce document ne dit pas

- **Quand descendre** : l'ordonnanceur, la famine, et le choix du domaine selon son contenu (ADR-210 D2) — la suite de C8.
- **Le rapport 2,5** (10 ↔ 25 cm, 2 ↔ 5 cm) : conservation éprouvée, pas le banc.
- **Le pas couplé** (δ sous B + W), un corps, un ensemble épars : le transfert les refuse ou ne les a pas éprouvés.
- **La transduction δ → W** d'ADR-005 §3, qui rendrait à W ce qui sort d'un domaine détruit : absente de la référence.
- **La mémoire** : pendant le transfert, les deux domaines existent ; tous deux se réservent à l'initialisation (ADR-210 D1).
- Rien sur la carte ; aucun verdict visuel.
