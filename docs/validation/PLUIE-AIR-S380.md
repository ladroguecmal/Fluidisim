# La pluie dans l'air : les gouttes qui tombent, l'extinction au loin — S380

2026-09-26. Pièces **1** et **2** d'[ADR-205](../adr/ADR-205-la-pluie-complete.md) (*« Pas de solveur continue la pluie
ajoute les manquants »*). Liste **8.4** (gouttes rendues, encore *absent* tant que rien n'est reçu) et **8.8** (le
lointain). Suite des rides de S379 ([RIDES-PLUIE-S379](RIDES-PLUIE-S379.md)) ; revue R29 ([REVUE-VISUELLE §34](REVUE-VISUELLE.md)).

## Reproduire

- Commit `65362664` (P7 de S380) ou plus récent ; Godot 4.4.1 (`<godot>` ci-dessous) ; données de la piscine et de la mer.
- `PLUIE=<mm/h> <godot> --path godot res://piscine.tscn -- --controle-gouttes` : le compte par classe (critère 2), lignes
  `CONTROLE_GOUTTES_S380` ; 10 mm/h par défaut.
- `<godot> --path godot res://piscine.tscn -- --controle-pluie` : l'extinction (critère 3), lignes `CONTROLE_PLUIE_S380` ;
  et les anneaux de S379, désormais sans gouttes ni brume sur leurs images de contrôle.
- `--cout-pluie` (piscine et mer) : le temps GPU ; critère 1 comme en S379 (12 images, SHA-256).

## En une phrase

La pluie tombe : près de l'œil, chaque goutte d'au moins 1 mm, au nombre de Marshall et Palmer — compté sur les images à
3 % près par classe de taille —, à la vitesse d'Atlas, tracée comme une caméra la voit (Garg et Nayar) ; au loin, son
extinction, `β = (π/2)·2·N0/Λ³`, voile la scène — 2,5 km de visibilité à 10 mm/h, 0,9 km à 50 ; par temps sec, rien ne
change au bit.

## 1. La construction

- **`pluie.gd`** : `densite_gouttes(R, d0, d1) = (N0/Λ)·(e^(−Λ·d0) − e^(−Λ·d1))` ; `extinction(R) = (π/2)·2·N0/Λ³·10⁻⁶` m⁻¹
  (efficacité d'extinction 2, gouttes grandes devant la longueur d'onde).
- **`pluie_air.gd`** : deux classes — [1 ; 2) mm dans une boîte de ±4 m × 6 m, [2 ; 6] mm dans ±8 m × 10 m —, chacune un
  `GPUParticles3D` de `densité × volume` particules (à 10 mm/h, ≈ 89 000 et 51 000), boîte aux trois quarts de son
  demi-côté devant la caméra, assez basse pour toucher le sol ; sous 1 mm, pas de traînée visible au-delà d'un mètre ou
  deux : ces gouttes ne sont que dans l'extinction.
- **`pluie_air.gdshader`** (particules) : tout par **hachage de l'indice** — position dans la boîte, diamètre par inversion
  de l'exponentielle tronquée à la classe, vitesse d'Atlas —, repli autour du centre (chaque goutte garde sa place dans le
  monde), arrêt au sol et à l'eau des bacs (niveaux du moment). Traînée de longueur `v·τ + D`, `τ` = 1/60 s, largeur au
  moins un pixel ; opacité de Garg et Nayar `min(1, D/s)·min(1, (D + s)/(v·τ))`, `s` la taille du pixel.
- **`goutte.gdshader`** : radiance ≈ 0,94 × la moyenne de l'environnement (Garg et Nayar : la goutte, lentille grand-angle,
  montre ≈ 165° de scène) — **approchée** : moitié ciel (30°, 60°, zénith), moitié sol (0,2 × le ciel au zénith).
- **L'extinction** : la brume de Godot (`1 − e^(−β·d)`, couleur du ciel) — à la piscine seulement sous la pluie, à la mer
  ajoutée à la brume sèche (0,00012 m⁻¹) et à notre brume par pixel.
- **Références** (lues sans téléchargement) : *Downpour (4390180547)* — averse sur des piscines : voile gris en quelques
  centaines de mètres, traînées fines visibles seulement devant les fonds sombres ; *Rain over the Sea, Mundesley* ; *Rain
  in Malta 03* ; *Downpour in New York*. Chiffré : la visibilité de l'averse (≈ 300 à 600 m) la place vers 100 mm/h par la
  loi retenue (570 m).

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans pluie, identique au bit | les 12 images de S379 | **12 / 12** |
| 2 — nombre et tailles à ±5 % | points d'une tranche de 0,5 m, vue d'aplomb, dix instants, par classe [1 ; 1,5) / [1,5 ; 2) / [2 ; 3) / [3 ; 6] mm | 2 mm/h : −0,84 / +1,91 / +5,21 / +6,11 % ; **10 mm/h : −0,73 / +0,49 / −1,48 / +1,33 %** ; 50 mm/h : −2,95 / −2,09 / −2,15 / −2,64 % — tenu (voir l'erreur du critère) |
| 3 — extinction égale à β, nulle par temps sec | brume posée dans la scène | 0 / 5,64335·10⁻⁴ / 1,55557·10⁻³ / 4,28785·10⁻³ m⁻¹ = la loi = l'intégration numérique indépendante ; visibilité 6,9 / 2,5 / 0,9 km |
| 4 — photographies et jugement | quatre références ; R29 | **reçue** : *« Je valide »* ([§34](REVUE-VISUELLE.md)) |

**Une erreur du critère écrit.** « ±5 % » oubliait le bruit de Poisson : à 2 mm/h, les deux grosses classes n'ont que
1 014 et 30 gouttes attendues (écarts-types 3,1 % et 18 %) — un écart de 5 % n'y est pas mesurable. Chaque classe est
jugée à ±5 % quand son écart-type relatif est sous 2,5 %, sinon à deux écarts-types : pire écart mesurable 1,91 / 1,48 /
2,95 %. À 50 mm/h, −2 à −3 % partout : des fusions de points que l'aire médiane ne rattrape pas.

**Une impasse du contrôle.** Avec le décor, les margelles (≈ 0,9 en tonalité linéaire) comptaient comme gouttes blanches,
et l'anticrénelage mêlait le bord des points au fond (aire médiane 1 pixel, comptes gonflés de 50 %) : décor caché, fond
noir, anticrénelage coupé.

## 3. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images ; rides de S379 comprises)

| scène | sec | 2 mm/h | 10 mm/h | 50 mm/h |
|---|---|---|---|---|
| bassin, proche | 0,53 ms | 0,80 | 1,35 | 3,57 |
| bassin, rasante | 0,46 | 0,56 | 0,80 | 1,74 |
| mer, proche | 1,60 | 1,90 | 3,24 | 8,94 |
| mer, référence | 1,56 | 1,82 | 2,97 | 7,84 |

Les gouttes seules : +0,1 à +0,3 ms à 10 mm/h, +0,45 à +1,2 ms à 50 mm/h. **Absentes** : les niveaux de détail d'ADR-202
(l'utilisateur, S379 : *« les LOD vont complètement bouleverser les performances »*).

## 4. Limites

- **Le ciel reste ensoleillé** (pièce 3, à venir) : le voile prend la couleur du ciel clair, bleu pâle, et non le gris des
  photographies ; c'est la pièce suivante.
- Rien entre ≈ 8 m et le lointain que l'extinction : pas de rideau de traînées à moyenne distance.
- Radiance de la goutte approchée ; pas de vent (traînées verticales) ; pas d'éclairage nocturne (les gouttes qui
  s'allument devant une lampe) ; une intensité pour toute la scène.
- Déterminisme de la pluie dans l'air non exigé (les images sous la pluie ne sont pas comparées au bit) ; vérifié sur
  cette machine seulement ; images de R29 locales (`viewer/captures/s380/`).
