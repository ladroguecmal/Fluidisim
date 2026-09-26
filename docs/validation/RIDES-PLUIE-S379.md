# Les rides de la pluie, factices — S379

2026-09-26. Liste **8.9** (détails artificiels bon marché), *partielle* ; session de rendu. Décisions servies :
[ADR-202](../adr/ADR-202-niveau-de-detail-des-contenants.md) D3 et [ADR-203](../adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md)
D7 — les impacts de pluie restent **factices**, même près du joueur ; au loin, peu de ressources. La pluie qui remplit un
contenant est dans V depuis S378 ([PLUIE-V-S378](PLUIE-V-S378.md)) ; ici, ce qu'on en **voit** à la surface. Revue : R28
([REVUE-VISUELLE §33](REVUE-VISUELLE.md)).

## Reproduire

- Commit `d30286b9` (P5 de S379) ou plus récent ; Godot 4.4.1 (`Godot_v4.4.1-stable_win64_console.exe`, ci-dessous
  `<godot>`) ; données de la piscine (`examples/piscine_v.rs`, `examples/piscine_delta.rs`) et de la mer (`mer_b.json`).
- `<godot> --path godot res://piscine.tscn -- --controle-pluie` : le taux compté (critère 2), lignes `CONTROLE_PLUIE_S379`.
- `<godot> --path godot res://piscine.tscn -- --cout-pluie` et `<godot> --path godot -- --cout-pluie` : le temps GPU.
- Critère 1 : `VUES=ensemble,rasante,buse INSTANTS=6.5,200 <godot> --path godot res://piscine.tscn -- --captures`,
  `DELTA=0 VUES=ensemble INSTANTS=40 …`, `<godot> --path godot -- --captures`, avant et après, empreintes SHA-256.
- Sous la pluie : `PLUIE=10` (mm/h) devant les mêmes commandes, ou la touche P en direct (0, 2, 10, 50 mm/h) ; vues
  `pluie_proche` et `pluie_aplomb` ajoutées à la piscine.

## En une phrase

La pluie dessine sur le bassin et sur la mer des anneaux dont le **nombre** est celui de Marshall et Palmer × Atlas —
compté sur les images à 0,6 % près à 10 mm/h —, dont la **forme** est celle d'un paquet d'ondes capillaires-gravité
(deux trains, 1,71 cm à 0,231 m/s et 4,4 cm à 0,178 m/s, éteints en 0,6 s) et qui, quand le pixel ne peut plus les
montrer, deviennent la **rugosité** qu'ils sont en moyenne ; sans pluie, les images d'avant sont inchangées au bit.

## 1. La construction

- **`godot/pluie.gd`** — la météorologie, une source : `N(D) = N0·e^(−Λ·D)`, `N0` = 8 000 m⁻³·mm⁻¹, `Λ = 4,1·R^−0,21`
  (Marshall et Palmer 1948) ; `v(D) = 9,65 − 10,3·e^(−0,6·D)` m/s (Atlas, Srivastava et Sekhon 1973). Taux des gouttes
  d'au moins 1,5 mm (celles qui laissent un anneau net), forme close : **67,2 / 447,1 / 1 931,8 m⁻²·s⁻¹** à 2 / 10 /
  50 mm/h ; moments `Σ flux·f²` des petites gouttes (0,5 à 1,5 mm, rugosité seulement) et des anneaux, `f = (D/2 mm)^1,5`.
- **`godot/pluie.gdshaderinc`** — la géométrie : chaque anneau, deux trains (tête à la vitesse de phase minimale, λ 1,71 cm ;
  derrière, à la vitesse de groupe minimale, λ 4,4 cm), enveloppe `(1 − (x/2w)²)²`, amplitude `(1 − âge/0,6 s)²·f`, pentes
  de crête 0,12 et 0,08 pour une goutte de 2 mm (**réglées sur les photographies**) ; une bosse centrale de 6 mm le premier
  dixième de seconde. Tirage en couches de mailles de 0,38 m, décalées au hasard à chaque période : l'anneau ne sort jamais
  de sa maille ; `taux × 0,38² × 0,6` couches, la dernière partielle — le taux est exact par construction.
- **Niveau de détail directionnel** : l'empreinte du pixel **le long du rayon de chaque anneau** (`|Mᵀ·dir|`) ; crêtes
  jusqu'à λ/4, fondues jusqu'à λ/2 en variance de pente **le long du rayon** (anisotrope) ; ces bandes, fondues de `w` à
  `2w` dans la variance uniforme ; plus de boucle au-delà de 5 cm de plus petite empreinte.
- **`bassin.gdshader`** : pente ajoutée (surface plane de V comme surface de δ) ; covariance intégrée par 3 × 3 nœuds de
  Gauss-Hermite sur Cholesky (comme la mer) ; éclat élargi en forme close ; fond vu flouté par la chaîne de l'écran.
  **`eau.gdshaderinc`** : en coordonnées de **Lagrange** — les anneaux suivent le mouvement orbital —, pente avant le
  passage en Euler, covariance avant son transport (ADR-161). Horloge repliée sur l'heure (I-08).
- **Références** : six photographies libres (Wikimedia Commons), lues dans le navigateur, rien de téléchargé — *Rain in a
  pond at Zoo Schönbrunn 2018* a et b, *Waterwaves raindrops on water surface*, *Rain on the river, Warwick*, *Rain on the
  River Pang, near Tidmarsh*, *Rain falling into a swimming pool*. Ce qu'elles montrent : un paquet de 2 à 5 crêtes par
  impact ; visible par le seul reflet (la normale, pas la couleur) ; des éclats rasants à moyenne distance ; au loin un
  voile mat, d'autant plus que la pluie est forte.

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans pluie, identique au bit | 7 images du bassin (ensemble, rasante, buse à 6,5 et 200 s ; sans δ à 40 s), 5 poses de la mer | **12 / 12 identiques** (SHA-256 ; deux passes d'avant égales) |
| 2 — taux de naissance à ±10 % | cœurs d'anneaux de moins de 30 ms comptés sur 20 images d'aplomb, 32 m² | **−4,16 %** (1 237 / 1 290,6), **−0,61 %** (8 532 / 8 584,2), **−1,66 %** (36 477 / 37 091,4) à 2 / 10 / 50 mm/h — tenu |
| 3 — au loin, aucun motif | empreinte au seuil | crêtes éteintes à λ/2 (8,6 mm pour 17,1 ; 22 mm pour 44) ; bandes de 4 et 10 cm passées à l'uniforme à 2 et 5 cm ; bosse de ≈ 12 mm éteinte à 6 mm — tenu |
| 4 — photographies et jugement | six références ; R28 | **reçue** : *« Parfait »* ([§33](REVUE-VISUELLE.md)) |

**Le compte du critère 2.** Deux cœurs voisins font une seule tache : à 50 mm/h (58 cœurs jeunes par m²), le compte brut
perdait 5,2 %. Chaque tache compte pour `arrondi(aire / aire médiane)` ; il reste −1,66 % à 50 mm/h (3,2 σ), les fusions
de moins de 1,5 aire médiane. Les taux de forme close sont vérifiés par intégration numérique indépendante (67,22 /
447,09 / 1 931,84).

## 3. Deux impasses

- **L'empreinte isotrope** (la plus grande des deux) et le seuil λ/6–λ/3 effaçaient toute ride au-delà de 1 à 2 m en
  720p : rien de visible dans les vues de S374. D'où l'empreinte **directionnelle**, le seuil de Nyquist, et le flou du fond
  (vue à travers l'eau, la variance seule ne touchait que le reflet).
- **La mer ne changeait pas** (2 à 4 niveaux sur 765 à 10 mm/h) : les crêtes de 1,7 cm sont sous le pixel partout à 4 m de
  haut. D'où le second train (λ 4,4 cm) et la variance de bande anisotrope — des enrichissements physiques, pas un gain :
  mer à 50 mm/h, écart moyen 8,8 niveaux, p99 72.

## 4. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images)

| scène | sec | 2 mm/h | 10 mm/h | 50 mm/h |
|---|---|---|---|---|
| bassin, proche | 0,53 ms | 0,78 | 1,34 | 3,52 |
| bassin, aplomb | 0,48 | 0,72 | 1,43 | 4,20 |
| bassin, rasante | 0,46 | 0,53 | 0,68 | 1,29 |
| mer, proche | 1,60 | 1,84 | 2,94 | 7,76 |
| mer, rasante | 1,58 | 1,84 | 2,85 | 7,00 |
| mer, référence | 1,56 | 1,75 | 2,68 | 6,45 |

**Techniques présentes** : saut de la boucle au-delà de 5 cm d'empreinte ; hachage court par couche. **Absentes** : le
champ d'anneaux calculé une fois par image dans une texture à moments, filtrée par mipmaps comme la queue FFT (S360) —
coût indépendant de la résolution et de l'étendue ; les niveaux de détail d'ADR-202 (l'effet ne court que là où la pluie
tombe et se voit). **Domaine** : effet isolé, eau plein écran. L'utilisateur, en cours de session : *« les systèmes
d'optimisation tels que les LOD vont complètement bouleverser les performances »* — ce coût n'appelle ni réduction ni
optimisation improvisée ; la texture à moments est inscrite.

## 5. Limites

- **Réglé, pas mesuré** : les pentes de crête (0,12, 0,08, bosse 0,5), `f = (D/2)^1,5`, la durée de 0,6 s et la taille des
  paquets ; la taille tirée ignore le poids de la vitesse terminale (loi exponentielle de Λ) — la variance lointaine suit la
  même loi, l'énergie est donc conservée entre les niveaux, pas exactement celle de la nature.
- **Absents** : couronnes, gouttelettes rebondissantes, bulles (8.4) ; l'amortissement des capillaires du vent par la pluie ;
  la pluie dans l'air et le ciel de pluie (la météo, à la fin) ; les scintillements lointains (rugosité moyenne) ; le reflet
  des objets de la scène (seul le ciel se reflète) ; une intensité par lieu (une seule pour toute la scène) ; l'exposition
  (une bâche arrête V, pas encore les rides).
- Déterminisme vérifié sur cette machine seulement ; images de R28 locales (`viewer/captures/s379/`), non versionnées.
