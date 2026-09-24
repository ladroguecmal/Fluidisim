# Le coût du pas de δ sur la scène de la porte B — S341

2026-09-24. **Porte C** ([ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.4,
[ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D3) : δ ≤ 2 ms GPU au 99ᵉ centile de la contribution
par image, sur la scène de la porte B, techniques présentes et absentes publiées. Chemin de la v1 (ADR-174 D4).
Cette mesure n'optimise rien : elle dit **où vont les 4,5 ms**, pour que le premier levier se choisisse sur un
chiffre ([ADR-131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md) D3–D4).

## Reproduire

- Commit `5fa59528` ou plus récent ; machine de référence.
- `cargo run --manifest-path viewer/Cargo.toml --release --offline -- --delta3d-horodatage` — l'horodatage ne
  touche pas au pas : 60 pas horodatés contre 60 nus, surface publiée identique au bit.
- `… -- --delta3d-cout-scene` — lignes `DELTA3D_COUT_S341`, une vingtaine de secondes ; `PAS=` (1 000).
- `… -- --delta3d-scene-mesure` — le témoin de S302, ligne `cout`.
- Alimentation (A270) : `Win32_Battery.BatteryStatus` et `BatteryStatus.PowerOnline`, avant et après.

## 1. Le domaine du chiffre (ADR-131 D3)

- **Scène** : `Config::review`, celle des revues R11 et R16 — 120 × 112 × 28 mailles de 25 cm (376 320 mailles,
  1 148 896 faces), mer `--houle` à 64 composantes, éponge de 3 m, **32 cycles**, pas de 16,667 ms, 177 dispatchs.
- **Machine** : RTX 5070 Laptop, Dx12. **Secteur** au début et à la fin des deux passages — `BatteryStatus` 2,
  charge 97 %, `PowerOnline` vrai. **Témoin** : le banc de S302 rejoué, 4,630 ms, pour 4,62 publiés.
- **Grandeur** : le temps GPU entre horodatages, du début de la première passe à la fin de la dernière ; chaque pas
  soumis seul et attendu. **Ne mesure pas** : un rendu concurrent, le recouvrement entre images, la soumission
  CPU, la relecture différée des diagnostics. La contribution par image d'un pas par image s'en approche ; elle ne
  s'y réduit pas.

## 2. Ce qui est mesuré

| | médiane | 99ᵉ centile | max |
|---|---:|---:|---:|
| **pas entier**, 1 000 pas, passage 1 / 2 | 4,452 / 4,477 ms | **4,505 / 4,651** | 4,834 / 4,852 |
| passe 1 — fond, prédiction, couplage | 1,978 / 1,991 | 2,002 / 2,102 | |
| — dont **l'évaluation du fond seule** | **1,533** | 1,554 | 1,594 |
| passe 2 — projection, 32 cycles | 2,056 / 2,057 | 2,068 / 2,071 | |
| passe 3 — correction, transport, diagnostics | 0,391 / 0,393 | 0,394 / 0,445 | |
| copies et intervalles | 0,028 | 0,057 | 0,203 |

| cycles | 0 | 8 | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| dispatchs | 17 | 57 | 97 | 177 | 337 |
| projection, médiane (ms) | 0,093 | 0,582 | 1,073 | 2,068 | 4,056 |
| pas entier, médiane (ms) | 2,527 | 2,940 | 3,449 | 4,487 | 6,614 |

**La projection vaut 0,087 ms plus 0,062 ms par cycle** — linéaire, à 6 µs près au pire. Un cycle fait trois
dispatchs de 376 320 mailles et deux réductions. *Estimation, non mesurée* : il lit et écrit une vingtaine de
mégaoctets, soit de l'ordre de 60 µs à la bande passante de cette carte — le cycle serait borné par la mémoire,
non par le calcul.

## 3. Techniques présentes et absentes (ADR-131 D2–D3)

**Présentes** : pas résident sur la carte, sans relecture ; travail fixe (177 dispatchs) ; départ chaud de la
pression ; préconditionneur de Jacobi ; diagnostics différés (ADR-175 D3) ; fond évalué sur la carte — aucun
échantillon CPU.

**Absentes** :

- **factorisation du fond** — la phase d'une composante ne dépend que de la colonne, son atténuation que de la
  couche ; la carte les recalcule pour chacune des 1,15 million de faces et des 64 composantes ;
- **charge utile réduite** — 26 champs écrits par face ; le pas en lit une dizaine selon l'axe de la face :
  vitesse, une ligne de `grad_u`, `du/dt` et `grad p` sur cet axe, `p`, `η` ; ni `grad η` ni le laplacien ;
- **multigrille** du pas 3D (le mode mobile 2D l'a depuis S274, ADR-167) — moins de cycles à résidu égal ;
- **fusion de noyaux** — cinq dispatchs par cycle, prédiction et divergence séparées ;
- **précision mixte** — tout en f32 ;
- **cadence découplée de l'image** (I-05) — un pas étalé sur deux images ;
- **LOD spectral** du fond de δ — 64 composantes partout ; **réglage des cycles contre l'usage** (ADR-144 non
  relevé : les pas sont déjà déclarés dégradés à 32 cycles, S302).

## 4. Ce que la mesure décide

- **Le 99ᵉ centile suit la médiane** à 1–4 % : le pas est régulier. La porte C est une affaire de moyenne, non de
  pointes.
- **Premier levier : l'évaluation du fond**, 1,53 ms — le plus gros noyau, un tiers du pas. Les deux techniques
  absentes qui la visent, factorisation et charge utile réduite, **se reçoivent au bit** : elles ne changent ni
  les valeurs ni l'ordre des sommes. À publier avec elles : la surface publiée identique au bit, et le coût.
- **Deuxième : la projection**, 2,06 ms, linéaire en cycles — multigrille, ou fusion des dispatchs d'un cycle.
- **Troisième, l'architecture** : la cadence découplée, qui divise la contribution par image — à éprouver contre
  la référence, le pas de temps doublant.
- **Pour 2 ms**, il faut retirer 2,5 ms au 99ᵉ centile. Aucun levier seul n'y suffit (ADR-131 D4) : la porte se
  juge sur la combinaison.

## 5. Ce que ce document ne dit pas

Le coût d'un pas dans l'image rendue, avec le rendu de la mer sur la même carte ; le coût d'une autre scène ;
le coût sur batterie (A270 : facteur 1,65 en S234).

---

## 6. S342 — le fond factorisé par colonne et par couche

2026-09-24. Premier levier du §4. `sample_faces_tiled` : un groupe de 256 fils couvre 16 colonnes × 16 couches
d'une famille de faces ; sinus et cosinus de chaque colonne, atténuation de chaque couche, calculés une fois en
mémoire de groupe ; puis la même accumulation, dans le même ordre, avec les mêmes primitives. Par défaut jusqu'à
64 composantes ; `sample_faces` reste, en repli et en témoin.

**Reproduire** : commit `a880c7f1` ou plus récent ; `… -- --delta3d-fond-tuiles` (identité) ; `… --delta3d-cout-scene`
et `FOND=faces … --delta3d-cout-scene` (les deux noyaux), secteur relevé avant et après.

**Identité, au bit** : les 29 871 296 valeurs du fond — 26 champs × 1 148 896 faces — identiques entre les deux
noyaux aux pas 0, 50 et 500 ; 60 pas de production, surface publiée identique sur 13 440 colonnes.

| même session, secteur (97 %) | fond seul, médiane / q99 | passe 1 | pas entier, médiane / q99 |
|---|---:|---:|---:|
| face par face (`FOND=faces`) | 1,527 / 1,551 ms | 1,981 | 4,456 / 4,501 |
| **par tuiles** | **1,237 / 1,261** | 1,819 | **4,348 / 4,405** |

**Ce que cela dit.** Le fond perd 19 %, le pas 0,11 ms seulement : les sinus, cosinus et exponentielles
n'étaient pas l'essentiel. Ce qui reste est l'accumulation des **26 champs** de chaque face, 64 fois, et leur
écriture — 120 Mo par pas. D'où le levier suivant : **n'écrire que les champs lus**, une dizaine selon l'axe de la
face (§3), ce qui retire à la fois le calcul et l'écriture de seize champs sur vingt-six.

---

## 7. S343 — dix champs par face au lieu de 26

2026-09-24. Suite du §6 : ce qui restait du fond était l'accumulation des 26 champs de chaque face et leur
écriture, dont le pas ne lit que dix. **La disposition compacte** — `override COMPACT` dans les noyaux du fond,
vraie pour le pas seulement : `η`, `u` (3), `du/dt`, la ligne de `grad u` et `grad p` de l'axe de la face, `p`. Le
tampon du pas passe de 120 à 46 Mo. Les bancs de S300 compilent le même fichier sans la constante et rendent
leurs nombres publiés — 1,46 à 2,44·10⁻³ Pa sur `p_dyn`, 6,0425·10⁻⁸ et 3,9462·10⁻⁷ au couplage.

**Reproduire** : commit `46c294d3` ou plus récent ; `… -- --delta3d-empreinte` (surface publiée et vitesses après 60
et 600 pas, empreintes FNV) ; `… --delta3d-cout-scene`, `FOND=faces … --delta3d-cout-scene` ; secteur relevé.

**Identité, au bit** : empreintes relevées avant le changement, rejouées après — 60 pas `0x5efa267462dfa0ad` /
`0xc5c6a85d3d29f44b`, 600 pas `0x9325cf58781f8b74` / `0xea1bebe0ffabc19a`, identiques.

**Trouvé en mesurant — un défaut de S342** : créer le pas prenait **247 s** depuis le noyau par tuiles. Le
compilateur Dx12 déroulait la mise à zéro de ses 12 Ko de mémoire de groupe, que wgpu ajoute par défaut. Le noyau
écrit cette mémoire entière avant de la lire : la mise à zéro est désactivée pour les noyaux du fond du pas —
**4,8 s**, empreintes inchangées, et 0,05 ms gagnées au fond.

| secteur, 97 % | fond seul | passe 1 | projection | passe 3 | pas, médiane / q99 |
|---|---:|---:|---:|---:|---:|
| S341, départ | 1,533 | 1,978 | 2,056 | 0,391 | 4,452 / 4,505 ms |
| S342, par tuiles | 1,237 | 1,819 | 2,055 | 0,443 | 4,348 / 4,405 |
| **S343, compact, par tuiles** | **1,038** | **1,303** | 2,059 | **0,288** | **3,679 / 3,727** |
| S343, compact, face par face | 1,374 | 1,579 | | | 3,933 / 3,981 |

**Ce que cela dit.** La charge utile compte plus que la factorisation : la prédiction, le couplage et la
correction lisent le fond, et lire dix champs au lieu de vingt-six les accélère autant que le fond lui-même. Le
pas perd 0,77 ms depuis S341 (−17 %). **La projection pèse désormais 56 % du pas**, inchangée : c'est le levier
suivant. Il manque 1,73 ms au 99ᵉ centile.

---

## 8. S345 — la cadence de 30 Hz, éprouvée avant d'être étalée

2026-09-24. Troisième levier du §4 : [ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md) §7 fixe le pas de
simulation à **30 Hz**, le rendu interpolant ; un pas de 3,7 ms étalé sur deux images de 60 Hz contribuerait
≈ 1,85 ms par image. Avant de l'étaler, la physique à 33,3 ms au lieu de 16,7. Pourquoi pas la projection d'abord :
ses cycles sont déjà bornés par la mémoire, et fusionner ses réductions au bit obligerait chacun des 5 880 groupes à
relire les 5 880 partiels — plus cher que le gain.

**Reproduire** : commit `b1f72111` ou plus récent ; `… -- --delta3d-cadence-cuve` ; `… --delta3d-cadence-scene` —
`CADENCES=pas_us:cycles,…`, la première en référence ; témoin : `CADENCES=16667:32,16667:64`.

**Critère 1, la cuve de S305** (mode (1, 1), `nx` = 32, 64 cycles, deux périodes) — **tenu** :

| pas | période | écart à la théorie | amplitude au dernier extrême / initiale |
|---|---:|---:|---:|
| 1 ms | 2,150743 s | +0,376 % (spatial) | 1,00005 |
| 16,7 ms | 2,150533 | +0,366 % | 1,00002 |
| **33,3 ms** | **2,149899** | +0,337 % | **0,99987** |

À 33,3 ms, −0,039 % de période et −0,013 % d'amplitude contre 1 ms, pour 1 % permis : sur ce mode, le pas de temps
ne pèse presque pas.

**Critère 2, la scène de B, 30 Hz contre 60 Hz** (12 s, front de S302 et témoins) — **non tenu** :

| écart au pire, chaque seconde | amplitude de l'onde isolée | position de son maximum |
|---|---:|---:|
| 30 Hz, 32 cycles | **6,28 %** (+3,9 à 1 s, +5,7 à 5 s, +6,3 à 8 s) | 0,35 m ; 1,03 à 3 s ; 7,9 à 12 s |
| 30 Hz, 64 cycles | 8,35 % | 6,25 m à 2 s |
| **témoin** : 60 Hz, 64 cycles au lieu de 32 | 1,31 % | 8,05 m à 6 s |

Aucune colonne hors bornes aux deux cadences. **Ce que le témoin tranche** : la partie « position » du critère était
**mal posée** — le maximum d'une onde dispersée saute d'une crête à l'autre même quand seule la projection change ;
la partie « amplitude » mesure un **effet réel** de la cadence, cinq fois celui du témoin, et presque toujours dans
le même sens : **plus d'amplitude à 30 Hz**. Ni la projection (64 cycles n'y changent rien) ni l'éponge (exacte en
temps, `exp(−taux·dt·…)`) ne l'expliquent. **Candidat, non démontré** : la dissipation numérique de l'advection par
la mer, par pas — moins de pas, moins d'amortissement ; la cuve, sans advection, ne montrait rien. Aucune des deux
cadences n'est « la vraie » : sans référence convergée en temps, on sait qu'elles diffèrent, pas laquelle est juste.

**Ce que cela décide.** La cadence de 30 Hz n'est **pas reçue** par ce critère. Deux voies, publiées : attribuer
l'écart (l'advection seule, un paquet sur une mer au repos, à 30 et 60 Hz) ; ou le **faire juger** — la même scène
rendue aux deux cadences, côte à côte : un écart de 4 cm sur une onde de 65 cm, dans une mer de 2,5 m, peut être
invisible, et c'est l'utilisateur qui supervise les rendus.

---

## 9. S346 — A318, attribué en partie

2026-09-24. D'où vient l'écart d'amplitude entre 30 et 60 Hz (§8) ?

**Reproduire** : commit `e98fa1ee` ou plus récent ; `MER=repos … --delta3d-cadence-scene` (fond de B d'amplitude
nulle) ; `CADENCES=16667:32,25000:32 … --delta3d-cadence-scene` (40 Hz).

| écart d'amplitude de l'onde isolée, contre 60 Hz à 32 cycles | 1 à 5 s | 6 à 12 s |
|---|---:|---:|
| témoin : 60 Hz, 64 cycles | ≤ 1,3 % | ≤ 1,2 % |
| **40 Hz** | **≤ 0,73 %** | jusqu'à 9,2 % (12 s) |
| **30 Hz**, mer au repos | +1,0 à +1,9 % | jusqu'à +3,9 % |
| **30 Hz**, mer de la porte B | **jusqu'à +5,7 %** | jusqu'à +6,3 % |

**Ce que cela établit.**

- **Tant que l'onde est groupée** (jusqu'à 5 s), l'écart **suit le pas** : 40 Hz au niveau du témoin des cycles,
  30 Hz nettement au-dessus, et **toujours du même signe** — plus d'amplitude quand les pas sont moins nombreux.
  C'est la signature d'un amortissement numérique **par pas**.
- **La mer en rajoute** sans en être la seule cause : sans elle, l'écart à 30 Hz tombe à 1–2 % au début, 3,9 % au
  plus. L'onde elle-même — d'amplitude 65 cm, cambrure 0,26, advectée par ses propres vitesses — y contribue.
- **Une fois l'onde dispersée**, les écarts n'ont plus d'ordre (40 Hz plus loin que 30 Hz à 12 s) : c'est l'horizon
  de prévisibilité de la scène (A297, S298), au-delà duquel une comparaison point par point perd son sens.
- La cuve de S305, linéaire et sans advection, ne dépendait pas du pas (§8). Le candidat reste l'**advection non
  linéaire** du pas couplé ; il n'est pas localisé dans le code.

**Ce que cela décide.** Aucune cadence n'est « la vraie » : à 60 Hz, l'onde est un peu plus amortie qu'à 30 Hz. La
cadence de 30 Hz change l'amplitude d'une onde forte de quelques pour cent, 4 cm sur 65 dans une mer de 2,5 m. La voir
ou non est une question de rendu : **revue R17**, la scène aux deux cadences, côte à côte.

---

## 10. S347 — les deux cadences rendues, pour la revue R17

2026-09-24. `--pas-delta=<µs>` règle le pas de la scène δ 3D ; sans lui, les captures de S302 restent identiques au
bit (16 empreintes, les mêmes qu'en S339 : les leviers de S342–S343 n'ont pas bougé un pixel). **Reproduire** : commit
`45bf56d7` ou plus récent ; `INSTANTS=120,300,480 … --meilleur --eau-physique=2 --delta3d --pas-delta=16667 --captures`
et `INSTANTS=60,150,240 … --pas-delta=33333 …`.

| pose | pixels différents entre cadences, avec δ | dont plus de 4 niveaux |
|---|---:|---:|
| référence | 17–21 % | 3,9–5,8 % |
| proche | 36–41 % | 12–15 % |
| rasante | 11–14 % | 1,3–1,8 % |
| haute | 9–14 % | 1,3–2,8 % |

Les instants tombent à 60 µs près d'une cadence à l'autre (120 × 16,667 contre 60 × 33,333 ms), ce qui décale aussi
les images de B seul. **Verdict** : R17 ([revue](REVUE-VISUELLE.md) §22).
