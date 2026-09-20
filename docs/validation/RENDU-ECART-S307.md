# L'écart entre notre rendu et l'état de l'art — S307

2026-09-20. Réponse au verdict « **le rendu actuel est toujours mauvais** » et à la consigne qui
l'accompagnait : aller au-delà du [guide reçu](../sources/guide_topologie_ocean_haute_mer_plage.md),
lire ses références et les références de ses références. Machine de référence
([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) : RTX 5070 Laptop, Dx12, secteur.

**Ce document n'est pas une revue de littérature.** C'est une liste de ce qui manque à *notre*
rendu, chaque poste rattaché à une source primaire, à une ligne de notre code, et à un écart
mesuré sur nos propres images.

## 1. Ce qui passe avant tout le reste : les revues ne montraient pas notre meilleur rendu

**Trois revues consécutives ont été soumises avec des fonctionnalités déjà acceptées, éteintes.**

| revue | manquait | acceptée depuis | effet mesuré |
|---|---|---|---|
| R11 | `--vagues` | S260–S261 | trouvé par S303 : la plus pauvre des trois mers du dépôt |
| R12, R13 | `--ciel-clair` | **S261, construit d'après la photo de référence de l'utilisateur** | voir §2 |
| R12, R13 | `--reflets-filtres` ([ADR-161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md)) | **acceptée par l'utilisateur en R7**, S266 | énergie haute fréquence **÷ 2,5** |

Personne n'avait **regardé** l'image avant de la mesurer. Toutes les mesures du dépôt portaient
sur la surface (statistiques de pente) ou sur des empreintes d'octets ; aucune sur ce que l'image
montre. En la regardant, puis en lisant le nuanceur, les trois oublis sortent en quelques minutes.

Remède construit : **`--meilleur`**, qui active tout ce qui est accepté, la liste vivant dans
`viewer/src/main.rs` à côté du code qui la consomme ; et une ligne de capture qui publie
**toutes** les options. Règle et raisonnement : [REVUE-VISUELLE, règle de protocole](REVUE-VISUELLE.md#règle-de-protocole--s307-2026-09-20) ;
leçon **L349**.

## 2. Ce que les options oubliées changent, mesuré

Même mer, même instant, même caméra (pose `proche`), scène `--vagues --modulation` :

| | luma moyenne | `hf_rms` | `hf_part` | B/G | B/R |
|---|---:|---:|---:|---:|---:|
| **tel qu'envoyé en R12/R13** | 162,1 | 9,82 | 0,191 | **1,18** | 2,3 |
| `+ --ciel-clair` | 137,6 | 15,05 | 0,272 | **2,02** | 4,2 |
| `+ --ciel-clair --reflets-filtres` (= `--meilleur`) | 142,1 | **5,97** | **0,116** | — | — |

- **`--ciel-clair`** remplace une brume dont la longueur caractéristique est **500 m** — sur une
  scène qui porte à 1 500 m — par une brume à **6 km**, et un ciel à deux couleurs par un ciel à
  dégradé et nuages. La mer étant un miroir, c'est la moitié de l'image.
- **`--reflets-filtres`** divise l'énergie haute fréquence par **2,5** (15,05 → 5,97) et le 99ᵉ
  centile du gradient par 2,7. **Sans retirer d'énergie de pente**, contrairement à la coupure de
  S306, qui n'atteignait 2,2 qu'en en supprimant.

**Conséquence pour S306** : `--coupure` reste une capacité utile et moins chère, mais **ce n'était
pas le bon levier** — le bon existait déjà et était éteint. À dire, parce que S306 concluait
l'inverse.

## 3. Ce que les références du guide contiennent — et ce qu'elles ne contiennent pas

Lues à la source, les douze références du guide portent sur la **forme** (FFT, Gerstner, clipmap,
projected grid) et sur la **physique côtière** (SWAN, TMA, Celeris, SWE, CEM). Repère : GPU Gems
ch. 1 (2004) propose **quatre** vagues géométriques et ~15 vagues de texture ; notre rendu est
très au-delà.

**Aucune des douze ne traite la couleur de l'eau, l'absorption, la diffusion, l'écume, le ciel
physique ni l'exposition.** Or ce sont exactement les postes que notre rendu traite par des
constantes écrites à la main. Le guide les mentionne en §10.2, sans sources. **C'est le niveau 2
et le niveau 3 de la recherche qui les couvrent.**

## 4. La couleur du corps d'eau — fausse d'un facteur 4 à 9, et sans provenance

`water.wgsl` porte `vec3(0.012, 0.105, 0.13)` (défaut) et `vec3(0.004, 0.060, 0.170)`
(ciel clair). **Aucune provenance**, ce qu'I-14 interdit, et qu'aucun audit n'avait relevé.

Dérivation depuis les sources primaires :

- absorption de l'eau pure, [Pope & Fry 1997](https://omlc.org/spectra/water/data/pope97.txt)
  (jeu de données téléchargé, valeurs converties de cm⁻¹ en m⁻¹) ;
- diffusion moléculaire `b(λ) = 0,0029·(550/λ)^4,30` m⁻¹, rétrodiffusion `b_b = b/2` (Morel 1974,
  confirmé par la littérature récente sur la diffusion de l'eau de mer pure) ;
- réflectance d'irradiance sous la surface `R(0⁻) ≈ 0,33·b_b/(a + b_b)`.

| λ | `a` (m⁻¹) | `b_b` (m⁻¹) | `R(0⁻)` |
|---|---:|---:|---:|
| 450 nm (bleu) | 0,00922 | 0,00344 | **0,0896** |
| 550 nm (vert) | 0,0565 | 0,00145 | **0,00826** |
| 650 nm (rouge) | 0,340 | 0,00071 | **0,00068** |

**Attendu : B/G = 10,9, B/R = 131.** Obtenu : **1,24** (défaut) et **2,83** (ciel clair) — soit
**9 fois et 3,8 fois trop vert**. La mesure le confirme dans l'image : les pixels les plus sombres,
ceux où l'on voit dans l'eau plutôt que le ciel, rendent exactement le rapport de la constante.

À titre de comparaison, l'implémentation de référence de Bruneton et al. emploie
`(0.0039, 0.046, 0.09)` — B/G = 1,96 — **mais multipliée par l'irradiance du ciel**, elle-même
bleue, là où la nôtre multiplie une constante. La nôtre est donc plus fausse que ce rapport ne le
laisse croire.

*Vérification d'une fausse piste* : j'avais cru voir un artefact brun-olive en moyenne distance.
**Il n'existe pas** — zéro pixel sur 921 600 n'a `R > B`, et ces taches portent le `B/G` du ciel
(1,36 contre 1,47). Ce que l'œil voyait est réel, mais c'est la teinte trop verte du corps d'eau
là où la surface ne renvoie pas le ciel.

## 5. Ce que notre rendu ne contient pas du tout

| poste | source primaire | état chez nous | ordre de grandeur |
|---|---|---|---|
| **écume / moutons** | Monahan & O'Muircheartaigh 1980 : `W = 3,84·10⁻⁶·U₁₀^3,41` ; réflectance effective 0,22 (Koepke) | **absent** | **0,42 %** de couverture à `U₁₀ ≈ 7,8 m/s` — faible, mais c'est le seul objet qui donne l'échelle |
| **diffusion sous la surface aux crêtes** | Sea of Thieves, SIGGRAPH 2018 : mélange eau profonde / sous-surface piloté par un **masque de crête tiré de la compression horizontale** | **absent** — alors que le masque existe déjà : c'est le **déterminant du jacobien** de CWM, calculé à chaque pixel depuis S260 et jamais employé que pour un repli | — |
| **spectre unifié gravité–capillarité** | [ECKV / Elfouhaily et al. 1997](https://www.oceanopticsbook.info/view/surfaces/level-2/wave-variance-spectra-examples) : `k_m = 370 rad/m`, `c_m = 0,23 m/s`, âge de vague `Ω_c`, étalement `Δ(k) = tanh[a₀ + a_p(c/c_p)^2,5 + a_m(c_m/c)^2,5]` | queue `f⁻⁴` continuée à la main ([ADR-157](../adr/ADR-157-queue-d-equilibre-et-vagues-pointues.md)) **plus** calage empirique sur Cox–Munk ([ADR-158](../adr/ADR-158-rugosite-ajustee-a-cox-munk.md)) | ECKV remplacerait **les deux** : la variance de pente en sortirait au lieu d'être ajustée. Notre `mss` est 13,7 % au-dessus de Cox–Munk (S306) |
| **ciel physique** | modèles de ciel à ciel clair (Preetham, Hosek–Wilkie) | ciel **procédural** : deux couleurs interpolées en `sin(élévation)` + bruit de valeur pour les nuages ([ADR-162](../adr/ADR-162-ciel-precalcule-des-reflets.md)) | non mesuré — **A299** |
| **exposition / tone mapping** | — | aucun : sortie linéaire vers sRGB | non mesuré — **A299** |

## 6. L'ordre, et pourquoi

Ordonné par **ce qui se voit**, tel que la §2 et la §4 le mesurent — pas par difficulté.

1. **Rendre les revues avec `--meilleur`.** Fait (§1). Coût nul, et cela invalide trois verdicts
   de suite si on ne le fait pas.
2. **La couleur du corps d'eau, depuis ses sources.** Une constante, une dérivation, une
   provenance. C'est le poste le plus faux (facteur 9) et le moins cher.
3. **Diffusion sous la surface aux crêtes**, pilotée par le jacobien **déjà calculé**. Cher en
   rien, et c'est ce qui donne aux crêtes leur translucidité.
4. **Écume aux crêtes**, couverture de Monahan, réflectance de Koepke. C'est ce qui donne
   l'échelle, et elle est totalement absente.
5. **Ciel et exposition** (A299) : non mesurés, donc à mesurer avant de décider.
6. **Spectre ECKV**, qui retirerait deux ADR empiriques. Le plus profond, le plus cher, et le
   moins visible à lui seul.

**Ce que cet ordre ne dit pas** : que le résultat sera bon. Les points 2 à 4 sont des décisions
**visuelles** ; elles se soumettent, elles ne se décrètent pas.

## 7. Limites de ce document

1. **Le PDF de Bruneton et al. 2010 n'a pas été lu** — portail protégé. Ce qui en est rapporté
   vient de son implémentation de référence publique et du résumé de l'éditeur, ce qui suffit pour
   le mécanisme (variance de pente → BRDF) mais pas pour ses formules exactes.
2. **Le talk Sea of Thieves n'a été lu qu'en résumé** (PDF non convertible ici).
3. **Aucune valeur du guide non reprise ici n'est validée** par le dépôt (I-14).
4. **La référence photographique de l'utilisateur n'est pas dans le dépôt** : toute comparaison
   reste qualitative tant que vent, spectre, focale et exposition ne sont pas connus — le guide le
   dit lui-même (§13.4).
5. **Rien de la côte** (§6–§7 du guide) n'est traité : porte F.

## 8. Reproduire

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --meilleur --revue-mer=<tag>
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --ciel-clair --test-ab=cielclair
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --ciel-clair --reflets-filtres --test-ab=tout
python outils/spectre_image.py "viewer/captures/s306/*_iv_rendu_complet_*.ppm"
python outils/apercu_ppm.py viewer/captures/s306/<image>.ppm
```

Témoin d'intégrité inchangé : `--vagues --modulation --revue-mer=c_asymetries` rend toujours
`0x422a86f52e2bd821`, `0x4e2da43a6e5dec18`, `0xd4a337b4eb1ed006`, `0xe4da5434ba05e1aa`.
