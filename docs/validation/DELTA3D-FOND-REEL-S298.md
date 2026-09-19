# Fond spectral réel et frontières de la référence 3D — S298

2026-09-19. Consommateur : référence CPU d’ADR-175 et son aperçu ; production GPU distincte.
Aucune tolérance antérieure déplacée. Aucun état δ sérialisé, aucune dépendance ajoutée.

## 1. Entrée réelle du fond

`BackgroundGrid3` réserve avant scellement les trois familles MAC, un jeu en préparation et
un jeu publié. Les coordonnées sont locales à l’ancre de B, axes communs, origine au fond du
domaine ; z est relatif au plan moyen de B. Le fond est recalculé à chaque instant, jamais
sauvegardé. Après refus, champs, gravité et instant publiés restent ceux du dernier succès.
Avant le premier succès, `view()` ne fournit rien ; un instant périmé est refusé par le pas.

Le chemin consomme `Background::differential_grid_extended` (S276, ADR-154), une grille x-z
par rangée y, puis range les faces dans l’ordre MAC. Phase et atténuation ne sont plus
recalculées par point. Scratch, coordonnées et deux jeux MAC sont comptés à la configuration.
Les tests comparent chaque champ au ponctuel prolongé aux trois familles, sur un domaine
non carré, sous et au-dessus du plan moyen, à trois instants dont 9 876 s ; test des refus,
réserve et absence d’allocation de l’échantillonnage puis du vrai pas couplé.

**Portée** : B réel seulement. Le prolongement de W au-dessus du plan moyen reste A286.
Le fond B est profond : il n’impose pas une paroi au fond de la boîte δ. L’atténuation et
la vitesse verticale résiduelle doivent donc être publiées pour chaque scénario consommateur.

## 2. Cas limites des frontières — critères avant mesure

`delta3d_boundaries` compare le pas 3D au pas 2D reçu, avec son préconditionneur ordinaire.
Les coordonnées et fixtures physiques sont identiques ; la direction se transpose de x à y.
Un seul élément dans la direction transverse : cas limite, pas un test d’incidence oblique.

- Houle progressive S272–S274 : h=1 m, λ=4 m, amplitude 1 cm, emprise 4 m,
  2 000 pas de 1 ms ; dx = 0,125 / 0,0625 / 0,03125 m, deux axes.
- Paquet S269 : spectre initial et vitesses du potentiel inchangés, désormais partagés par
  `support/reflection_packet.rs`. Domaine 24 m, profondeur 1 m, amplitude 2 mm, jauge 12 m ;
  7 200 pas de 5 ms, dx=0,25 puis 0,125 m ; mur et éponge, sur chaque axe.
  Éponge 4 m, taux `10 cg(π)/4`, relaxation de hauteur et vitesse.
- Aucun refus, hauteur 3D−2D <3 mm (repère S201). Pour le paquet,
  `sqrt(∫(jauge3−jauge2)² dt / E_incident2)` ≤0,001 : réserve d’instrument S269.
  Maxima de pente et vitesse publiés, aucun seuil nouveau inventé.

## 3. Résultats des cas limites

Six trajectoires progressives et huit de paquet, chacune calculée en 2D et en 3D :
**139 200 pas acceptés**, aucun refus. L’écart maximal de hauteur vaut **1,192093·10⁻⁷ m**
(0,000119 mm), tous cas et axes confondus. Sur mur/éponge grossiers dans x : identité des
hauteurs et vitesses à chaque pas. Le changement de préconditionneur ne déplace pas le reçu.

| cas | dx (m) | max pente 3D−2D, deux axes | max vitesse (m/s), deux axes | erreur énergétique jauge x / y |
|---|---:|---:|---:|---:|
| progressive | 0,125 | 9,537e-7 | 9,095e-10 | — |
| progressive | 0,0625 | 1,907e-6 | 1,384e-9 | — |
| progressive | 0,03125 | 3,815e-6 | 5,675e-9 | — |
| mur | 0,25 | 9,537e-7 | 1,537e-8 | 0 / 8,560e-6 |
| mur | 0,125 | 1,907e-6 | 5,215e-8 | 1,591e-5 / 1,581e-5 |
| éponge | 0,25 | 9,537e-7 | 1,164e-8 | 0 / 8,704e-6 |
| éponge | 0,125 | 9,537e-7 | 2,841e-8 | 1,017e-5 / 1,030e-5 |

Critères d’équivalence tenus. La trace complète reste en mémoire ; seules les mesures sont
imprimées. Le maximum d’itérations 3D est 235 pour la houle fine, 65 pour le paquet.

**Ce que ce reçu ne dit pas.** Les doubles gardes 48/72 m de S269 ne sont pas rejouées ici :
aucun nouveau coefficient de réflexion n’est annoncé. On reçoit la reproduction des trajectoires
2D dans la boîte courte, dans les deux orientations. Ni absorption oblique/aux coins, ni mer
large bande à la frontière, ni coefficient d’ordre deux S274 au budget de 2 % ne deviennent
reçus par cette équivalence. Ces limites physiques restent celles de S269/S274.

## 4. Reproduction

```powershell
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_boundaries -- progressive
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_boundaries -- packet
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline real_background_
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline --example delta_reflection
```

Le potentiel partagé garde ses tests d’incompressibilité, de cinématique et de direction :
6 tests d’exemple réussis, 1 ignoré. Les temps muraux de calculs concurrents ne reçoivent
aucune performance de production.

## 5. Aperçu sur le fond réel — et la résolution qu'il exige

**Livré localement**, deux fois la même scène, seules les périodes du spectre changeant :
`viewer/captures/s298/` (fixture d'origine) et `viewer/captures/s298-resolu/`, chacun avec
`couplage_3d.gif` (121 images, 20 im/s, six secondes physiques lues à vitesse réelle) et quatre
PNG aux instants 0 / 1,5 / 3 / 6 s. Aucun état δ sérialisé (I-17). Depuis la racine :

```powershell
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_preview -- viewer/captures/s298 --spectral
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_preview -- viewer/captures/s298-resolu --spectral --resolu
python outils/apercu_delta3d.py viewer/captures/s298 --spectral
```

Domaine 32 × 24 × 36, maille 0,25 m, emprise 8 × 6 m, repos **8 m** ; 1 200 pas de 5 ms,
plafond 4 000, éponge de 1 m sur les quatre côtés à 2 s⁻¹. Impulsion, témoin et vue
orthographique identiques à S297 (§5 de sa preuve) : seul le fond change.

**Le fond est maintenant celui du cœur.** Deux systèmes JONSWAP directionnels de
`background_spectrum::bake_directional` (ADR-156, loi cos^2s), assemblés : 32 + 32 = **64
composantes**, `s_max` 10 (mer de vent) et 25 (houle), directions 0,02 et 0,22 tour — **79°
d'écart**, `gamma` 3,3, bande 0,8–1,4 fp, graines 298 et 299. C'est la première fois que δ en 3D
est calculé sous une mer réellement **étalée**, et non sous des ondes analytiques choisies.

| | fixture d'origine | fixture résolue (`--resolu`) |
|---|---:|---:|
| `Tp` des deux systèmes | 1,4 s / 1,1 s | 1,6 s / 1,75 s |
| empreinte de la recette | `4413aa00b9029fcf` | `2a9785f2c177e596` |
| λ la plus courte | 0,9809 m | **2,0752 m** |
| mailles par λ la plus courte | **3,92** | **8,30** |
| atténuation maximale au fond (−8 m) | 2,2591731·10⁻⁵ | 1,0629863·10⁻³ |
| vitesse verticale résiduelle au fond | 1,193644·10⁻⁶ m/s | 3,4031687·10⁻⁵ m/s |
| itérations maximales / affinages | 147 / 0 | 140 / 0 |
| divergence franche maximale | 9,39682·10⁻⁶ | **4,887259·10⁻⁶** |
| pic de l'écart au témoin à 6 s | 0,090666 m | **0,026775 m** |
| tendance de ce pic après 3 s | **croissante** | décroissante |

**Ce que la vérification des images a trouvé.** Sur la fixture d'origine, l'écart au témoin
dégénère en damier à l'échelle de la maille et **remonte** après son minimum de 3,3 s
(0,047 → 0,091 m). Ce n'est pas de la dispersion : la maille de 25 cm ne porte la composante la
plus courte que sur 3,92 points. Sur la fixture résolue — 8,30 mailles, tout le reste identique —
le champ redevient un anneau qui se disperse, légèrement marqué par le fond, le pic décroît
jusqu'à 0,027 m et la divergence franche maximale est divisée par 1,9.

**Confusion assumée** : à `Hs` constant, allonger les périodes abaisse aussi la cambrure.
L'expérience ne sépare donc pas la résolution de la cambrure ; elle montre seulement que la
croissance disparaît quand la maille porte le spectre. Aucun seuil de mailles par longueur
d'onde n'est reçu ici, et la fixture d'origine n'est **pas** un résultat de physique.

Empreintes FNV-1a des PPM complets (en-tête et pixels) :

| image | âge | fixture d'origine | fixture résolue |
|---|---:|---|---|
| frame_0000.ppm | 0,00 s | `80538debe3e3a074` | `034a6ff9738edd21` |
| frame_0030.ppm | 1,50 s | `8684e0107930630e` | `db892ccb06a507e3` |
| frame_0060.ppm | 3,00 s | `970d9727ae20b504` | `49c5bcbab248e5d8` |
| frame_0120.ppm | 6,00 s | `8f71fe87b0ca7c0f` | `150e1cd9cd3afe97` |

### Ce que cet aperçu dit de la porte B

Le critère 3 d'[ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4 demande
« une onde traverse une mer étalée et s'y déforme », jugée par l'utilisateur. **Ce banc ne peut
pas porter ce jugement**, et le mesurer était l'apport de ce lot : à `dx` 0,25 m dans 8 × 6 m, la
bande à la fois résolue et contenue tient dans λ ∈ [2,1 ; 7,3] m, soit **moins de deux longueurs
d'onde en travers de la boîte**. La mer résolue cesse alors de *ressembler* à une mer étalée —
on la voit sur le panneau de gauche de `s298-resolu`, presque plan. Une mer étalée visible et
résolue demande plus de mailles que ce banc CPU séquentiel n'en calcule en six minutes : c'est
le domaine de production GPU d'ADR-175 §4.2, pas un réglage de fixture.

Cet aperçu montre donc le **chemin** — B réel du cœur consommé par la référence 3D, sans
allocation et sans refus — et la **contrainte de maille** qu'il impose. Il ne reçoit ni le
critère 3, ni un verdict perceptif, ni un coût de production.

## 6. Vérification globale

`cargo test --manifest-path code/Cargo.toml --workspace --release --offline` : **539 réussis**
(421 cœur, 20 exécution δ, 2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec.
Afficheur inchangé. Les bancs `delta3d_boundaries` et les deux aperçus ont été exécutés
explicitement en plus des tests. Les durées murales, avec tâches concurrentes, ne reçoivent
aucun coût de production.
