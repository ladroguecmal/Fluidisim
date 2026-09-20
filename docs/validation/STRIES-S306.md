# D'où viennent les stries de notre mer — S306

2026-09-20. Réponse mesurée au test A/B que demande le
[guide reçu](../sources/guide_topologie_ocean_haute_mer_plage.md) §12.3, et que le dépôt n'avait
jamais fait. Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1) :
NVIDIA GeForce RTX 5070 Laptop GPU, backend Dx12, secteur (relevé en §7). Aucune dépendance
ajoutée, aucune page publiée. **Le chemin de rendu n'est pas modifié** : tout ce qui suit est
soit une sortie en plus, soit un paramètre dont la valeur par défaut est l'ancienne au bit.

Lecture du guide et classement de ses treize sections :
[LECTURE-GUIDE-OCEAN-S306](../registres/LECTURE-GUIDE-OCEAN-S306.md).

## 1. La question, et pourquoi elle n'était pas posée

Après le verdict R11 (« la mer n'est pas réaliste, les pics doivent être convexes »), S303 a
mesuré la **statistique de la surface** et S304 a construit
[ADR-176](../adr/ADR-176-asymetries-de-la-surface-rendue.md) : deux asymétries, reçues sur cinq
critères. C'est une réponse. Le guide en propose une autre, et il la met **en premier** : dans un
rendu qui ne ressemble pas à une photo, regarder d'abord les **normales fines** et
l'**environnement lumineux**, la topologie ensuite.

Le dépôt n'avait aucun moyen de départager les deux. Il mesurait des statistiques de surface
(S260, S303, S304) et des empreintes d'image, mais **rien qui décrive la structure spatiale de
l'image rendue**. C'est ce qui manquait.

## 2. L'instrument — pourquoi un écart-type ne suffit pas

Une image de grandes masses d'eau et une image de stries fines peuvent avoir **le même
écart-type de luma**. Ce qui les sépare est la répartition en fréquence spatiale. D'où
`outils/spectre_image.py` (Python standard, sans dépendance), qui publie par capture :

| grandeur | définition | ce qu'elle dit |
|---|---|---|
| `luma_et` | écart-type de luma sur les pixels d'eau | le contraste, celui que l'œil juge d'abord |
| `hf_rms` | RMS de la différence à **un pixel** (passe-haut au Nyquist de l'image) | l'énergie des stries |
| `hf_part` | `hf_rms / luma_et` | la **part** de la structure qui est haute fréquence ; insensible à l'exposition |
| `anisotropie` | `RMS(∂y) / RMS(∂x)` | des stries parallèles donnent un rapport nettement > 1 |
| `p99_hf` | 99ᵉ centile de \|gradient\| | les stries les plus vives, sans les noyer dans la moyenne |

Les pixels de ciel sont exclus en diagnostic, par **lecture** de la couleur du coin — pas par
supposition (L348).

Côté rendu, trois sorties de diagnostic ont été ajoutées à `ocean_fragment`, portées par
`p.reflection.w` **qui valait un zéro littéral** : aucune taille d'uniforme ne change pour elles.
1 = hauteur (rampe fixe ±3 m), 2 = normales de la bande résolue **sans la queue**, 3 = jacobien.
Les deux autres sorties du guide existaient déjà : `--no-tail` et le rendu par défaut.

## 3. Le test A/B — la queue porte les stries, et le contraste ne le voit pas

`--vagues --modulation --test-ab=c_asymetries` : cinq sorties, **même instant, même caméra, même
mer**, aux deux poses que l'utilisateur a regardées en R12.

| sortie | pose `proche` | | | | pose `rasante` | | | |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| | `luma_et` | `hf_rms` | `hf_part` | `p99` | `luma_et` | `hf_rms` | `hf_part` | `p99` |
| (i) hauteur seule | 21,80 | 6,36 | 0,292 | 27,1 | 18,79 | 5,92 | 0,315 | 20,6 |
| (ii) normales géométriques | 3,52 | 0,44 | 0,124 | 1,7 | 4,03 | 0,41 | 0,102 | 1,6 |
| **(iii) matériau sans queue** | 51,41 | **1,93** | 0,038 | 9,1 | 40,76 | **1,69** | 0,041 | 8,1 |
| **(iv) rendu complet** | 51,29 | **9,82** | 0,191 | 41,2 | 42,45 | **11,28** | 0,266 | 48,1 |
| (v) jacobien | 36,80 | 12,44 | 0,338 | 68,3 | 38,61 | 11,85 | 0,307 | 65,7 |

**Trois lectures, dans l'ordre d'importance.**

1. **La queue spectrale porte 80 à 85 % de l'énergie haute fréquence de l'image** : `hf_rms` est
   multipliée par **5,1** en pose `proche` et par **6,7** en `rasante` quand on l'allume. Le 99ᵉ
   centile du gradient suit (×4,5 et ×5,9). **Ce sont les stries.**
2. **Le contraste global ne le voit pas.** `luma_et` passe de 51,41 à 51,29 en `proche` — il
   *baisse*. C'est exactement le piège que le guide décrit, et c'est pourquoi aucune mesure
   antérieure du dépôt ne pouvait l'attraper : toutes portaient sur la surface ou sur des
   empreintes, aucune sur la structure spatiale de l'image.
3. **La géométrie, elle, porte peu de haute fréquence.** La sortie (ii), qui est la forme sans
   aucun détail fin, a `hf_rms` = 0,44 — vingt fois moins que le rendu complet. Si les grandes
   masses manquaient, elles manqueraient là, et le guide dit qu'aucun réglage de détail ne les
   ferait revenir. C'est à l'utilisateur de juger l'image (i), qui lui est soumise.

**Témoin au bit, et il vaut d'être dit.** La sortie (iv) du test A/B porte l'empreinte
`0x4e2da43a6e5dec18` (pose `proche`) — **celle de l'image de R12 elle-même**. Le test ne porte
pas sur une mer voisine : il porte sur celle que l'utilisateur a regardée.

## 4. Combien de cette énergie est légitime — la statistique de pente tranche

Instrument de S260 (10⁶ points, même réalisation que le rendu, `--houle` d'ADR-156) :

| | `mss` | écart à l'observation |
|---|---:|---:|
| **Cox–Munk**, `W` = 7,95 m/s (observation) | **0,0437** | — |
| notre modèle retenu (bande + queue + CWM + modulation + ADR-176) | **0,0497** | **+13,7 %** |
| le même **sans queue** | **0,0200** | −54,2 % |

Donc la queue porte **0,0297 des 0,0497 — 60 % de la variance de pente**, et la totalité de
l'excès. Pour tomber exactement sur Cox–Munk elle devrait en porter 0,0237 : **−20 % en variance,
−10,7 % en amplitude**. C'était la prédiction écrite avant la mesure, à partir du §4.4 du guide
(« ne pas compter deux fois ») et du « `mss` 14 % haute » de S303 ; elle est vérifiée.

**Mais −20 % ne réglerait pas les stries.** Réduire la queue d'un cinquième en variance diviserait
`hf_rms` par ≈ 1,1, quand elle vaut ×5 à ×6 le reste de l'image. **L'essentiel de cette énergie
est donc légitime** : une vraie mer porte cette variance de pente. Le problème n'est pas
*combien*, c'est **dans quelle bande de longueurs d'onde on la rend**.

**Et le jacobien ne se retourne jamais.** L'instrument publie `replis = 0` sur les treize modèles,
dont le nôtre. L'indicateur que le guide met au premier rang (§4.3, §12.2) est désormais mesuré,
et il **écarte** l'hypothèse d'un repli du paramétrage. Sa carte (sortie v) est disponible.

## 5. La bande de rendu — un paramètre, et il ne coûte rien

`spectral_weight` garde **tout son poids jusqu'à `λ = 4·empreinte`** et ne tombe à zéro qu'à
`λ = 2·empreinte`, le Nyquist du pixel. Le guide (§5.3) rappelle que Nyquist est une condition
**minimale d'échantillonnage**, pas un critère de qualité, et propose `λ/Δ ≥ 4–8` comme point de
départ : **nous sommes à la borne basse de sa fourchette**. Les composantes entre 2 et 4 empreintes
sont précisément celles qui produisent un motif de deux pixels.

`--coupure=<f>` (treizième `vec4` d'uniforme) déplace les deux bornes à `4f` et `2f`. **`f` = 1
rend ADR-148 au bit** — multiplier par 1,0 est exact, et les quatre empreintes de R12 sont
inchangées après la modification.

| `f` | poids plein / zéro | `hf_rms` (proche) | `hf_part` | `p99` | `luma_et` | `hf_rms` (rasante) | GPU eau |
|---|---|---:|---:|---:|---:|---:|---:|
| **1** — ADR-148 | 4 / 2 empreintes | **9,82** | 0,191 | 41,2 | 51,29 | **11,28** | 1,0212 ms |
| 1,5 | 6 / 3 | 6,42 | 0,125 | 26,7 | 51,23 | 7,50 | — |
| **2** — haut de la fourchette du guide | 8 / 4 | **4,51** | 0,088 | 18,8 | 51,25 | **5,37** | **0,9574 ms** |
| 3 | 12 / 6 | 2,74 | 0,053 | 11,6 | 51,35 | 3,19 | — |
| *sans queue* | — | *1,93* | *0,038* | *9,1* | *51,41* | *1,69* | — |

- **L'énergie haute fréquence se divise par 2,2 à `f` = 2, par 3,6 à `f` = 3**, monotone aux deux
  poses.
- **Le contraste ne bouge pas** : `luma_et` reste entre 51,23 et 51,41 (pose `proche`), la luma
  moyenne à 0,3 % près. On enlève les stries sans toucher à ce que l'œil juge en premier.
- **C'est moins cher** : GPU eau **1,0212 → 0,9574 ms**, −6,2 %, parce que `tail_cwm` s'arrête à
  la première composante de poids nul et qu'elle arrive plus tôt. Un gain, pas un coût.

## 6. Ce que ce lot ne tranche pas

1. **Le choix de `f` est un arbitrage visuel, pas un réglage.** Élargir la coupure retire aussi
   du micro-détail **réel** : l'image devient plus lisse qu'une mer observée, même si le modèle
   (et donc `mss`) ne bouge pas. Le compensateur correct — transférer ces pentes vers le **reflet**
   au lieu de les supprimer — est la voie d'[ADR-161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md)
   et du guide §5.3 ; **il n'est pas mesuré ici**. La décision revient à l'utilisateur
   (**R13**, [REVUE-VISUELLE §18](REVUE-VISUELLE.md#18-r13--doù-viennent-les-stries-et-faut-il-en-enlever-s306)),
   et un ADR l'actera.
2. **Aucun ADR n'est pris.** `--coupure` est une capacité dont le défaut est l'ancien
   comportement au bit ; une capacité n'est pas une décision.
3. **ADR-176 n'est ni contredite ni confirmée par ce lot.** Elle porte sur la forme des crêtes,
   mesurée en statistique de surface ; ce lot porte sur la fréquence spatiale de l'image. Les deux
   questions sont distinctes, et **R12 reste demandée**.
4. **L'anisotropie n'est pas interprétée.** Elle vaut 4 à 9 selon les images, mais une caméra
   presque rasante étire géométriquement les structures à l'horizontale : la mesure est confondue
   avec la perspective, et rien n'en est conclu.
5. **L'environnement lumineux n'est pas testé.** Le guide met le ciel et l'exposition au même rang
   que les normales fines (§10.2, §10.3 rang A). Le dépôt a un habillage « ciel clair » relevé sur
   la photo de référence (S261) mais **aucune comparaison contrôlée** ciel par ciel. Reste ouvert.

## 7. Reproduire

```powershell
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --test-ab=c_asymetries
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --coupure=2 --test-ab=coupure2
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --coupure=2 --cadence
cargo run --manifest-path viewer/Cargo.toml --release --offline -- --vagues --modulation --revue-mer=c_asymetries
cargo run --manifest-path code/Cargo.toml --release --offline --example statistiques_surface
python outils/spectre_image.py "viewer/captures/s306/*.ppm"
python outils/apercu_ppm.py viewer/captures/s306/<image>.ppm
```

La quatrième ligne est le **témoin d'intégrité** : elle doit rendre `0x422a86f52e2bd821`,
`0x4e2da43a6e5dec18`, `0xd4a337b4eb1ed006`, `0xe4da5434ba05e1aa`, les empreintes de S304.
La commande est celle de la preuve S304 — ni `--multi` ni `--ciel-clair` ; les ajouter change les
images et fait mentir le témoin (L348).

**Alimentation (A270)** : secteur, `PowerOnline = True`, `BatteryStatus = 2`, charge 98 %, relevé
au début de la campagne (S305 §8) et inchangé. La seule mesure de coût de ce document est la
cadence de §5, prise deux fois de suite dans le même état.
