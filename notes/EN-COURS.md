# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S306 — en cours : le guide reçu, confronté au dépôt puis éprouvé.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, carte réelle.
Entrée : un fichier, sans consigne — `guide_topologie_ocean_haute_mer_plage.md`, v1.0 du
2026-09-20, recopié tel quel dans [`docs/sources/`](../docs/sources/guide_topologie_ocean_haute_mer_plage.md).
Il dit analyser « les deux images reçues » (une photo de haute mer, un rendu) : c'est donc, selon
toute vraisemblance, une **réponse indirecte à R12** obtenue ailleurs. Le verdict lui-même n'est
toujours pas donné ; la question sera posée au rituel, pas avant — le travail n'en dépend pas.

Capacité visée : **savoir ce que ce document change, et ce qu'il ne change pas.** Deux moitiés :
1. La **confrontation** au dépôt, section par section — déjà acté, déjà construit, diverge, neuf
   et actionnable, hors périmètre présent. Sans cela, un document de 773 lignes se dissout en
   impressions et reforke la conception par la bande.
2. L'**hypothèse qu'il met au premier rang** (§1, §10.3, §12.3) et que le dépôt n'a jamais
   testée : l'écart entre notre mer et une photo viendrait d'abord des **normales fines** (notre
   queue spectrale, ADR-155) et de l'**environnement lumineux** (ADR-162), et non de la topologie
   ni des asymétries. ADR-176 a répondu par la statistique de la surface ; le guide dit de
   regarder ailleurs **en premier**. C'est une thèse concurrente, et elle se mesure.

Ce que je ne fais pas : réécrire ADR-176 ni rouvrir les cinq arbitrages d'ADR-027 ; construire
quoi que ce soit de la côte, de la plage ou du déferlement (porte F et au-delà — l'ambition
d'ADR-127 les contient, l'ordre des portes les place plus tard) ; prendre pour acquis un chiffre
du guide sans sa source.

Critères, écrits avant la mesure :
1. Chaque section du guide est classée, et **chaque divergence nommée avec le document du dépôt
   qui la tranche** — un désaccord sans référence est une impression.
2. Le test A/B de §12.3 tourne sur **notre** rendu, quatre sorties au même instant, même caméra,
   même mer : hauteur brute, normales géométriques seules, matériau sans queue, rendu complet.
3. La cause dominante des stries est **attribuée par la mesure** — queue spectrale, ciel, ou
   géométrie — et non par préférence. Si la mesure contredit ADR-176, elle est dite telle quelle.
4. Les scènes antérieures restent **au bit** : les sorties de diagnostic sont des modes en plus,
   jamais une modification du chemin de rendu.

### Plan

- [x] **P1** — amorce, jeton, guide recopié dans `docs/sources/`, plan seul.
- [x] **P2** — confrontation section par section, écrite dans un document de lecture.
- [x] **P3** — les deux sorties de diagnostic manquantes (hauteur en fausses couleurs, normales
  géométriques seules) ; `--no-tail` et le rendu complet existent déjà.
- [x] **P4** — le test A/B aux quatre sorties, et la cause dominante attribuée.
- [ ] **P5** — *(découpage déclaré)* balayage de la coupure `λ/Δ` : le seul essai qui dit si les
  stries sont évitables sans toucher à l'énergie du modèle.
- [ ] **P5** — conséquences : file active, angles morts, note datée à ADR-176 si la mesure la
  contredit, et ce qui doit remonter à l'utilisateur comme arbitrage.
- [ ] **P6** — preuve, rituel REPRISE §6.

### Notes de reprise

**P2 — la confrontation est faite** : [LECTURE-GUIDE-OCEAN-S306](../docs/registres/LECTURE-GUIDE-OCEAN-S306.md).
Résultat en une ligne : **aucune divergence réelle**, beaucoup de convergence, et un seul apport
qui change quelque chose maintenant — l'**ordre de diagnostic**.

- **Le guide décrit ce que nous faisons déjà**, parfois mot pour mot. Deux cas frappants : sa
  « couche résiduelle définie » (§8.4) est exactement ADR-175 D7 (δ publie une perturbation, le
  rendu reconstruit `η_B + δ`) ; et son projected grid (§5.2, Johanson) **est** notre maillage
  depuis S211 — `grid_point` lance un rayon par sommet vers `z = 0`, borné à l'horizon. Un
  document écrit sans connaître le dépôt converge sur ses deux choix structurants de rendu.
- **§5.4 (coutures, T-junctions, morphing) est sans objet chez nous** : une seule grille projetée,
  aucun raccord entre niveaux. Un tiers des risques de maillage du guide ne nous concerne pas.
- **§6 et §7 (côte, plage, déferlement, wet/dry) sont quasi absents du dépôt** — et c'est daté :
  A234 (fond uniforme seulement) et porte F. Le guide y apporte de la matière sourcée (TMA [S6],
  bilan d'action SWAN [S5] et son défaut de phase, Celeris [S7], Jeschke–Wojtan [S8], wet/dry
  [S10]) : consignée, rien à construire.

**Trois points actionnables, et une correction.**
1. **Le test A/B de §12.3 n'a jamais été fait.** Nous avons déjà deux de ses quatre sorties
   (`--no-tail` = matériau sans queue ; le défaut = rendu complet). Manquent les deux sorties
   **géométriques** : hauteur brute, normales sans détail. C'est P3.
2. **Le ratio `λ/Δ` : nous sommes à la borne basse de ce que le guide recommande.**
   `spectral_weight` vaut 1 tant que `λ ≥ 4h` et tombe à 0 en `λ = 2h` (Nyquist exact) ; le guide
   propose `λ/Δ ≥ 4–8` comme point de départ. Les composantes entre 2 et 4 sont précisément
   celles qui font le plus de bruit de pente. Jamais éprouvé contre un critère visuel.
3. **`min(J)` n'est jamais publié.** Le déterminant du jacobien est calculé (il sert de garde à
   `det < 0,1` dans `covariance_transport`) mais aucune distribution n'est mesurée.
4. **Correction sourcée d'un chiffre du dépôt** : SPEC-001 §3 donne `H/h ≈ 0,78` (McCowan) comme
   critère de déferlement, et ADR-023 §4 en dérive des sites turbulents. Le guide (§7.1, Coastal
   Engineering Manual [S9]) rappelle que c'est un repère **du cas de la vague solitaire sur fond
   horizontal**, pas une loi universelle. L'attribution est juste, l'emploi comme critère unique
   ne l'est pas → note datée à SPEC-001, rien à reprendre (le mécanisme n'est pas construit).

**Et une prédiction vérifiable, tirée du rapprochement** : S303 a mesuré notre `mss` **14 % trop
haute** contre Cox–Munk, ce qui est le symptôme d'une bande fine trop forte (§4.4 du guide, « ne
pas compter deux fois »). Si les stries viennent de la queue, la `mss` mesurée **sans** queue doit
tomber **sous** Cox–Munk. Si elle reste au-dessus, la queue n'est pas la cause. À vérifier en P4.

**P3 — les sorties manquantes, sans toucher au chemin de rendu.** Trois modes dans
`ocean_fragment`, portés par `p.reflection.w` qui valait un **zéro littéral** : aucune taille
d'uniforme ne change. 1 = hauteur (rampe fixe ±3 m), 2 = normales géométriques seules (bande
corrigée par le jacobien, **sans** la queue), 3 = jacobien. Le ciel devient un gris constant en
diagnostic. Outil d'analyse : `outils/spectre_image.py` (Python standard), qui mesure sur une
capture PPM l'énergie **haute fréquence** — un écart-type de luma ne distingue pas une grande
masse d'un tapis de stries, un passe-haut si.

*Fausse alerte, et ce qu'elle a appris* : le premier contrôle d'identité au bit a **échoué**.
Cause : j'avais rejoué `--revue-mer` avec `--multi --ciel-clair`, que S304 n'employait pas. Avec
la commande exacte de la preuve S304, les quatre empreintes sont **identiques au bit**
(2463a897…, 6592d82c…, bd3d6046…, a2ccc245…). Le témoin ne valait rien tant que la commande
n'était pas celle du document. Deuxième contrôle, plus fort : la sortie `iv_rendu_complet` du
test A/B porte l'empreinte `0x4e2da43a6e5dec18`, **celle de l'image de R12 elle-même**.

**P4 — la cause des stries est attribuée, et ce n'est pas celle qu'ADR-176 visait.**

| sortie (pose `proche`) | `luma_et` | `hf_rms` | `hf_part` | `p99_hf` |
|---|---:|---:|---:|---:|
| (iii) matériau **sans** queue | 51,41 | **1,93** | 0,0375 | 9,14 |
| (iv) rendu complet | 51,29 | **9,82** | 0,191 | 41,19 |

Pose `rasante` : 1,69 → **11,28** (`hf_part` 0,041 → 0,266 ; `p99` 8,14 → 48,13).

- **La queue spectrale porte 80 à 85 % de l'énergie haute fréquence de l'image** (×5,1 en
  `proche`, ×6,7 en `rasante`). C'est elle, les stries.
- **Le contraste global ne le voit pas** : `luma_et` passe de 51,41 à 51,29 — il *baisse*. Le
  défaut n'est pas dans le contraste, il est dans la **fréquence spatiale**. C'est exactement le
  piège que le guide décrit, et c'est pourquoi aucune mesure antérieure ne l'avait attrapé.
- **La prédiction de P2 est vérifiée, et chiffrée.** Instrument S260, 10⁶ points : `mss` de Cox–Munk
  **0,0437** ; notre modèle **0,0497** (+13,7 %) ; **sans queue 0,0200** (−54 %). La queue porte donc
  **0,0297 des 0,0497 — 60 % de la variance de pente**, et tout l'excès. Pour tomber exactement sur
  Cox–Munk il faudrait qu'elle en porte 0,0237, soit **−20 % en variance, −10,7 % en amplitude**.
- **`replis = 0` sur tous les modèles** : le jacobien ne se retourne **jamais** dans notre mer.
  L'indicateur que le guide met au premier rang (§4.3) est publié, et il écarte une hypothèse.

**La conclusion honnête, qui n'est pas « la queue est fautive ».** La queue n'est que 20 % trop
forte en variance ; la réduire de 20 % ne diviserait l'énergie haute fréquence que par ≈ 1,1,
quand elle vaut ×5 à ×6 le reste. **L'essentiel de ces stries est donc légitime** — une vraie mer
porte cette variance de pente. Ce qui est en cause, c'est **la bande de longueurs d'onde où on la
rend** : `spectral_weight` garde tout son poids jusqu'à `λ = 4·empreinte` et ne tombe à zéro qu'à
`λ = 2·empreinte`, le Nyquist du pixel. Les composantes entre 2 et 4 empreintes sont exactement
celles qui produisent un motif de deux pixels. C'est le point §4.2 de la lecture, et c'est ce que
P5 mesure.

**Non retenu, et pourquoi** : rien du guide ne rouvre ADR-027 ni ne réduit ADR-127 ; aucun de ses
chiffres non repris ici n'est validé par le dépôt (I-14).

**Ce qui est déjà su au départ.** Le rendu porte déjà : mer multimodale à étalement (ADR-156),
queue d'équilibre et vagues pointues (ADR-157), rugosité calée sur Cox–Munk (ADR-158), queue
spectrale en pentes par pixel (ADR-155), reflets de la queue non résolue (ADR-161), ciel
précalculé (ADR-162), sommes d'ondes filtrées par la distance (ADR-163), et depuis S304 les deux
asymétries d'ADR-176. `--no-tail` éteint déjà la queue ; `--sans-asym` éteint ADR-176. Ce qui
manque pour le test A/B, ce sont les deux sorties **géométriques** : hauteur et normales seules.

---

## Archive — notes de S304 (lot de la mer, en attente du verdict R12)

**Construit** : `tayfun()` et `lagged_eps()` dans `water.wgsl`, accumulations par système dans la
boucle existante de `band_cwm` (branches explicites — FXC refuse l'indexation dynamique en
écriture, L345) ; douzième `vec4` de l'uniforme `(split, k̄₁, k̄₂, retard)` ; mêmes deux termes dans
la référence CPU `cwm_reference` ; activation par `--vagues --modulation`, témoin `--sans-asym`,
balayage `--retard=`. Le fragment n'a pas changé : le sommet lui transmet la déformation **retardée**.

**Réception d'ADR-176 §3, cinq critères tenus** :

| critère | exigé | obtenu |
|---|---|---|
| 1. statistiques | `Sk` ≥ 0,06 ; `c₀₃` ∈ [−0,18 ; −0,13] ; `c₂₁` −0,058 ± 0,02 ; `c₄₀` ∈ [0,26 ; 0,45] ; `mss` ± 2 % | **0,0656** ; **−0,155** ; **−0,057** ; **0,340** ; +0,2 % |
| 2. GPU contre CPU | 3 mm ; 5·10⁻⁴ | 1,59·10⁻⁶ m ; 2,69·10⁻⁴, aucun repli |
| 3. scènes au bit | défaut, `--houle`, `--vagues`, `--sans-asym` | **identiques** au binaire de S301 |
| 4. écart au jeu | publié | 0,3651 → **0,3999 m** (+3,5 cm ; CWM en porte 36,5) |
| 5. coût | publié, secteur relevé | 1,0138 → **1,0163 ms** (+0,25 %) |

`k̄` par système : **0,1742** (mer de vent) et **0,0317** (houle) — l'instrument et l'hôte donnent
les mêmes, donc les deux implémentations portent bien le même modèle.

**Images R12** : `--revue-mer=<état>`, quatre poses de R11, trois états nommés dans le fichier
(`a_houle_seule`, `b_vagues_modulation`, `c_asymetries`). Regardées ici : l'écart `a` → `c` est
franc (la mer de R11 était lisse et striée) ; l'écart `b` → `c` est plus fin — crêtes plus marquées,
contraste crête/creux plus net. C'est à l'utilisateur de juger, R12 pose quatre questions.

---

## Archive — notes de S303 (pour le lot)

**Ce que la recherche donne** (sources dans la preuve) : l'asymétrie verticale d'une mer profonde
vient des **harmoniques liées du second ordre** (`Sk = 3k̄σ` en bande étroite, Longuet-Higgins 1963,
Tayfun 1980) ; les pentes se mesurent au miroitement depuis Cox & Munk 1954, révisées par IASI ; et
— le plus visible — les **rides ne sont pas uniformes** : elles se raccourcissent et se redressent
sur les crêtes des vagues longues, s'aplatissent dans les creux (JFM 2024 : pente modulée de 20 %
à `ε_L` = 0,1, doublée à 0,4), avec un **retard** qui place leur maximum en avant de la crête.

**Ce que la mesure donne** (instrument de S260 étendu, 10⁶ points, même réalisation que le rendu) :

| | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `Sk` |
|---|---:|---:|---:|---:|---:|
| observations (Cox–Munk 7,95 m/s ; 3k̄σ) | 0,0437 | −0,058 | −0,222 | 0,40 | 0,156 |
| `--houle` — **la scène montrée en R11** | 0,0198 | −0,001 | 0,002 | −0,026 | −0,0001 |
| `--vagues --modulation` (le meilleur construit) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,0030 |
| **S303 retenu** (+ Tayfun + retard −0,20) | 0,0497 | **−0,057** | **−0,155** | 0,340 | **0,066** |

**Trois choses apprises, dans l'ordre d'importance.**
1. **Faute de protocole de ma part** : la scène soumise à R11 était `--houle` **seule**, c'est-à-dire
   sans la queue d'équilibre ni les vagues pointues construites en S260–S261. C'est la
   configuration que S260 avait déjà mesurée « pentes quasi gaussiennes ». L'utilisateur a jugé la
   plus pauvre des trois mers du dépôt.
2. **Même la meilleure n'a aucune asymétrie** : `Sk` 0,003 contre 0,156 ; crêtes et creux aussi
   arrondis les uns que les autres — le « concave plutôt que convexe » du verdict, chiffré.
3. **Le second ordre par composante ne peut pas la produire** (mesuré : 0,0022) : ce sont les
   termes **croisés** qui la portent. D'où Tayfun, qui les contient tous pour une somme de plus.

**Le retard est le seul paramètre libre**, calé sur `c₀₃` ; son **signe** ne l'est pas — seul un
retard négatif (rides en avant de la crête) donne l'asymétrie du signe observé, et `c₂₁` tombe sur
Cox–Munk sans avoir été visé. Reste ouvert : le noyau exact du second ordre pour deux systèmes
(la vérité est entre 0,063 et 0,150), les capillaires parasites, l'asymétrie horizontale.

---

## Archive — notes de S302 (pour le lot)

**P2 — la scène tient** (`--delta3d-scene-mesure`, mer `--houle` à 64 composantes, domaine
96×128×28 à 25 cm = 344 064 mailles, repos 3,5 m, éponge 3 m, 32 cycles, 60 Hz, 12 s) :
- **aucune colonne hors bornes**, jamais, avec et sans paquet ;
- le paquet garde ses 25 cm et traverse de `y = 26` à `y ≈ 7` en 11 s, soit **1,7 m/s** — la
  vitesse de groupe théorique d'une onde de 8 m vaut 1,77 m/s. L'onde isolée (paquet − témoin)
  reste à 0,18–0,26 m ;
- δ **sans paquet** porte déjà 9 à 19 cm : la correction couplée de B à Hs 2,5 m (R10 : 10,6 cm
  mesurés en 2D à Hs 2 m). À dire dans la revue : une partie de ce que l'utilisateur verra n'est
  pas l'onde mais cette correction ;
- coût **4,23 ms** par pas (médiane, 177 dispatchs) : au-delà des 2 ms d'ADR-174 D3, ce qui est un
  point de la porte C, pas de la revue ; tenable à 60 Hz avec le rendu (≈ 2 ms) ;
- **tous les pas reçus sont déclarés dégradés** au sens d'ADR-144 (divergence franche 2·10⁻²).
  Les diagnostics ne reviennent que 60 fois sur 720 : l'anneau est plein tant que la carte
  travaille — perdus, jamais retardants, comme D3 le prévoit.

`Step3::on_device` : le pas se construit sur un device **fourni**, celui du rendu ; la surface
publiée devient un tampon que le rendu lie (D7), sans passage par le CPU. `published_buffer()` est
le seul tampon exposé.

**P3 — les à-coups d'A297 se voient dans les chiffres** (`--delta3d-scene-acoups`, dérivée seconde
temporelle par colonne, 2,2 à 4,4 millions d'échantillons) :

| | d2 RMS | q99,99 | max | localité | rugosité de maille RMS | max |
|---|---:|---:|---:|---:|---:|---:|
| témoin, 32 cycles | 9,2·10⁻⁵ m | 1,8·10⁻³ | 7,2·10⁻³ | **7,4** | 2,17·10⁻³ m | 5,9·10⁻² |
| témoin, 128 | 9,2·10⁻⁵ | 1,8·10⁻³ | 4,8·10⁻³ | 7,3 | 1,99·10⁻³ | 2,1·10⁻² |
| témoin, 512 | 9,2·10⁻⁵ | 1,7·10⁻³ | 4,8·10⁻³ | 7,3 | 2,03·10⁻³ | 2,2·10⁻² |
| paquet, 32 | 1,6·10⁻⁴ | 2,0·10⁻³ | 6,7·10⁻³ | 7,3 | 2,68·10⁻³ | 4,7·10⁻² |

« Localité 7,4 » : au pire à-coup, la colonne fautive vaut 7,4 fois la moyenne de ses huit
voisines — la signature d'une bascule, pas d'une onde. **Le balayage de cycles tranche** : de 32 à
512 cycles la rugosité ne bouge pas (2,17 → 1,99 → 2,03 mm) ; ce n'est **pas** une pression
sous-convergée, c'est le schéma. À comparer à la signature du paquet lui-même à l'échelle de la
maille, `a·k²·dx²` = 9,6·10⁻³ m : le bruit vaut environ **un cinquième** du signal utile en
hauteur, et jusqu'à 5 fois au pire point. Conséquence pour la revue : le bruit de maille est à
montrer et à nommer, pas à cacher ; 32 cycles suffisent visuellement (le maximum seul gagne à
128), donc le choix de cycles est une question de coût.

**P4 et P5 — la scène est rendue et soumise.** `Step3::on_device` fait naître le domaine sur le
device du rendu ; le nuanceur lie **la seule surface publiée** (groupe 3), l'interpole en
Catmull-Rom bicubique et l'ajoute à la somme des couches avec un fondu de 3 m. Témoin : les sept
images de la revue R9 gardent leurs empreintes **au bit** (0x8ae42dfb1fe7d1b3…) — le rendu existant
n'a pas bougé. Fenêtre : `--houle --delta3d`, **D** bascule, **R** relance l'onde ; un pas fixe de
16,667 ms par image, donc le fond que δ consomme et celui que l'image montre sont au même instant.
Cadence **197 Hz** (5,06 ms), essai de 120 images passé.

**Dimensionnement, deux allers-retours mesurés** : d'abord 24×32 m avec une onde de 25 cm à 8 m —
l'onde se noyait dans une mer de 2,5 m (les deux images se ressemblaient). Puis 32×32 m : **refusé
par le device**, le tampon des faces (26 flottants par face, S300) franchissant les 128 Mio d'une
liaison de stockage. Retenu : **30 × 28 m** (120×112×28) et un **front** de 65 cm sur 16 m, crête
longue de 12 m — visible sans être hors du régime perturbatif (`ak` 0,26 ; le refus non diagnostiqué
de 2D est à 0,335). Une vue plongeante a été essayée et écartée : rien ne s'y lit (déjà mesuré S275).

Captures : quatre poses × avec/sans × quatre instants, plus deux images de différence ×6.
La couche change **13,5 à 15,7 %** des octets à la pose de référence (8,5 à 11,2 % de plus de
quatre niveaux). Preuve : [SCENE-DELTA3D-S302](../docs/validation/SCENE-DELTA3D-S302.md) ; demande
de revue : REVUE-VISUELLE §16 (R11), avec **quatre questions explicites** — R10 avait échoué faute
de dire ce qu'on attendait.

*Tenue du plan* : le battement du commit P2+P3 a été écrit **sans lire l'horloge** (23:18 au lieu
de 23:06, L237) ; corrigé au commit suivant. Une fenêtre interactive s'est ouverte une fois parce
que l'ancien binaire ignorait un drapeau neuf après un échec de compilation — vérifier que la
compilation a réussi avant de lancer.

---

## Archive — notes de S301 (pour le lot)

**Architecture retenue (P2).** `Step3` crée **un** device et y compile les trois sources telles
quelles — `delta3d_background.wgsl` (S300), `delta3d_cg.wgsl` (S299), `delta3d_step.wgsl`
(nouveau) — chacune avec sa propre disposition de liaisons, sur des tampons communs. Rien de
S299/S300 n'est réécrit (L137). Rangements : `vel = [u|v|w courants | u|v|w prédits]`, l'indice
d'une face étant aussi celui de son échantillon de fond ; `cells_in = [eta | divergence |
eta_roundoff]` — **eta vit là**, c'est l'entrée de `couple_columns` ; `cells_out` = celui de S300 ;
`work = [flux_x | bande_x | flux_y | bande_y | surface publiée]`. Les passages vers la projection
(surface totale → `heights`, second membre → tranche B, préconditionneur → tranche M) seront des
copies de tampon **dans l'encodeur**, sans retour CPU.

**P2 reçu** (`--delta3d-prediction`, 15×11×14, 7 459 faces, fond S300 à 64 composantes, vitesses
d'ordre 0,2 m/s, éponge (1 ; 0,75 ; 2 s⁻¹), dt 5 ms, trois instants) : pire écart **6,0·10⁻⁸ m/s**,
soit **≤ 9,0·10⁻⁶ de l'incrément** du pas, sur les trois familles ; 70 à 85 % des faces au bit ;
zéro face fautive. Refus : durée nulle, éponge trop large, `dt²g/dx > 1`, longueur.

**Défaut trouvé par le banc, corrigé avant commit** : le noyau couplait les faces `i = nx` (u) et
`j = ny` (v), que le cœur saute — borne comparée à `n + 1` au lieu de `n`. Écart de **60 % de
l'incrément**, sur ces seules faces. Le compteur `faces_fautives` (> 5 % de l'incrément) reste
dans le banc : c'est lui qui voit une règle de bord portée autrement, l'arrondi ne le peut pas.
Rapporter l'écart à l'incrément et non à la vitesse était nécessaire : rapporté à la vitesse, le
même défaut ne pesait que 1 %.

**Incident d'outillage, corrigé par un commit séparé** : le battement de P2 a été écrit par
`Get-Content -Raw | Set-Content -Encoding utf8` de Windows PowerShell 5.1, qui **lit en ANSI** :
`REPRISE.md` est parti ré-encodé (mojibake) dans `fff03d5`. Restauré depuis `7ebeeb5`, battement
réécrit à l'outil d'édition. **Ne jamais réécrire un fichier du dépôt par `Get-Content` /
`Set-Content`** ; `[IO.File]::ReadAllText/WriteAllText` (UTF-8 par défaut) ou l'outil d'édition.

**P3a reçu** (`--delta3d-pression`, même fixture, 1 510 mailles mouillées sur 2 310, départ
`p = 0` des deux côtés). Le cœur converge en 65 et 68 itérations, sans affinage. La carte :

| cycles | dispatchs | écart p (t=0) | relatif | écart p (t=1,23 s) | relatif |
|---|---|---|---|---|---|
| 8 | 51 | 2 757 Pa | 0,17 | 2 992 Pa | 0,18 |
| 32 | 171 | 218 Pa | 1,3·10⁻² | 196 Pa | 1,2·10⁻² |
| 64 | 331 | **0,27 Pa** | 1,6·10⁻⁵ | **0,20 Pa** | 1,2·10⁻⁵ |
| 128 | 651 | 0,38 Pa | 2,3·10⁻⁵ | 0,077 Pa | 4,7·10⁻⁶ |

Échelle 16 400 Pa. Plateau dès 128 cycles : plancher f32. En usage : 0,27 Pa ≈ **0,03 mm d'eau**.
Aucune maille sèche non nulle. Résidu vrai de la carte 3,6·10⁻⁷ à 64 cycles, celui du cœur
2,3·10⁻⁷. À 8 cycles l'écart vaut 27 cm d'eau **depuis p = 0** : en trajectoire le départ chaud
part de la pression du pas précédent, et c'est P5 qui dira combien de cycles il faut alors.
`couple_rhs` lit désormais `eta_roundoff` (`cells_in` après la divergence) ; banc S300 rejoué,
chiffres identiques (1,06·10⁻⁶ ; 6,0·10⁻⁸ ; 3,9·10⁻⁷). `init_warm` ajouté à `delta3d_cg.wgsl`.

**P3b reçu** (`--delta3d-correction`, même fixture, vitesses de fin de pas contre celles du cœur
après `step_perturbation_mobile`, extrapolées comprises) : incrément du pas ≈ 0,5 à 0,76 m/s (le
champ de départ est fortement divergent, la projection le redresse). Écart **≤ 1,4·10⁻⁵ m/s**,
**≤ 2,6·10⁻⁵ de l'incrément** à 128 cycles, ≤ 1,7·10⁻⁵ à 64 ; **zéro face fautive** sur les trois
familles et trois instants. Fantômes latéraux avec le fond de la face (`p_dyn`, `grad_p_dyn`),
fantôme du haut avec `eta_roundoff` et `ghost_up` de S300 : aucune règle de bord divergente.

**P4 reçu** (`--delta3d-pas`, pas complet à 128 cycles, **655 dispatchs**, trois instants) :
- hauteur : incrément du pas 4,2 à 5,1 mm ; écart **2,4·10⁻⁷ m** (un ulp de η à 2,25 m),
  159 à 162 colonnes sur 165 au bit ; surface publiée au même écart ;
- **hauteur vraie** `η − reste` : écart **3 à 4·10⁻⁸ m**, six fois sous l'ulp — la compensation
  est portée ;
- vitesses ≤ 1,4·10⁻⁵ m/s (2·10⁻⁵ de l'incrément), pression ≤ 0,38 Pa (2,3·10⁻⁵).
Surface publiée = `(η − repos) − reste` par colonne, **tampon à part** (liaison 7) : le rendu ne
liera jamais `cells_in` ni `vel` (D7, I-13). Accès d'essai `surface_roundoff_for_trials` ajouté au
cœur.

**Défaut trouvé, corrigé avant commit — la somme compensée détruite par le compilateur.** Premier
passage : reste nul sur les 165 colonnes. Expérience : sur la carte `(η + inc) − η` rendait `inc`
**au bit** pour 165/165 colonnes, quand la même addition est inexacte sur CPU pour 164 à 165/165.
Le compilateur de la carte (DX12) simplifie `(a + b) − a → b` ; la compensation de S233 disparaît
sans bruit — hauteur au bit près dans 130/165 colonnes seulement, et une perte systématique de
l'ordre de l'ulp de η par pas. Remède : `exact_difference(s, a)`, soustraction **en entiers sur
les bits IEEE**, exacte par Sterbenz pour deux hauteurs à moins d'un facteur deux. Même famille
que L345 (fraction de phase) : **une identité flottante du source n'est pas une identité du
binaire compilé** — généralisation à écrire en leçon. *(Écrite : L346.)*

**P5 — trajectoire S298, premier passage** (`--delta3d-trajectoire`, 32×24×36, 64 composantes,
1 200 pas, cycles 8/16/32/64 en parallèle, 4 min 39) : écart de hauteur **≤ 5·10⁻⁶ m jusqu'à
t = 1 s** pour 32 et 64 cycles (1·10⁻⁴ à 8 cycles, 5·10⁻⁵ à 16), puis saut au millimètre vers
t ≈ 1,1 s et 1 à 3,5 cm ensuite, **quel que soit le nombre de cycles**. Donc pas la projection.
Localisation (`PAS`, `CYCLES`, `SONDE_PAS` en variables d'environnement) : le saut naît à
**n = 215** dans la maille **(19,10,32)** — **sèche pour le cœur (p = 0), mouillée pour la
carte (p = 37 Pa)** : une surface passée à moins de 10⁻⁶ m d'un centre de maille, classée des deux
côtés opposés. Une seule bascule donne 0,25 m/s d'écart sur une face `v`, 0,08 sur `u`, puis des
centimètres de hauteur. Avant la bascule, les faces adjacentes aux mailles de surface à petit θ
diffèrent déjà de 2·10⁻³ m/s (coefficient 1/θ jusqu'à 1 000) sans effet visible sur η.
**Fausses pistes écartées** : B n'est pas en cause (élévation carte/cœur à 9·10⁻⁸ m sur ce fond,
`--delta3d-fond-s298`) ; l'éponge non plus (colonne intérieure). Une première sonde décalée d'un
pas (lue après le pas du cœur) avait fait croire à un écart de B — artefact de l'instrument.
**Question ouverte à trancher par la mesure** : sensibilité **propre** du schéma de référence
(deux cœurs à ±10⁻⁶ m au départ, `--delta3d-sensibilite`) — si deux cœurs se séparent pareil,
l'écart est une propriété de la référence et le critère ponctuel de §4.2 n'est tenable que sur
une durée déclarée avant la première bascule.
Artefact de banc corrigé : la surface publiée n'était écrite qu'au premier pas (écart 0,16 m à
t = 0) ; `set_state` la publie désormais.

**P5 — tranché par la sensibilité de la référence** (`--delta3d-sensibilite`, deux cœurs, le
second à ±10⁻⁶ m au départ, motif haché, 1 200 pas) : **ils se séparent exactement comme la carte
et le cœur** — 4,6 à 7·10⁻⁵ m jusqu'à t = 1 s, premier dépassement du millimètre au **pas 260**
(1,3 s), 1 à 4,5 cm ensuite, écart quadratique 2,3 à 3,1 mm de t = 3 à 6 s. Carte contre cœur à
64 cycles : quadratique 1,9 à 2,4 mm sur la même période, premier millimètre vers le pas 216.
**L'écart de P5 est une propriété du schéma de référence, pas de la production.** Avant son
horizon, la carte suit le cœur **mieux** (≤ 5,5·10⁻⁶ m à 32–64 cycles) qu'un cœur perturbé d'un
ulp ne suit le cœur (≤ 7·10⁻⁵ m).
**Mécanisme** (sonde) : le transport de la hauteur lit la vitesse de la couche **partiellement
mouillée** (fraction `(surface − k·dx)/dx`). Quand la surface passe le centre de cette couche, sa
face bascule de « projetée » (correction à 1/θ, θ petit) à « extrapolée » depuis la couche
inférieure : 0,25 m/s d'écart sur une face, soit ~6·10⁻⁴ m de hauteur par pas
(`dt/dx · Δu · dx · ½`), observé 5,8·10⁻⁴ puis 2,8·10⁻⁴ m par pas. La hauteur est donc
**discontinue** en la position de la surface par rapport aux centres de maille : deux états
distants de 10⁻⁶ m divergent de centimètres en une demi-seconde. Nouvel angle mort (A297).
**Lecture du critère 2** (ADR-175 §4.2, « sur la durée déclarée ») : la durée ponctuelle ne
peut excéder l'horizon de prévisibilité **de la référence elle-même**, mesuré ; au-delà, la
production se compare à l'enveloppe que la référence a contre elle-même. Aucun seuil relevé.

**P6 mesuré** (`--delta3d-cout-pas`, pas entier horodaté de la première passe à la dernière,
copies comprises, fond S298 à 64 composantes, 30 passages, premier écarté ; secteur aux deux
bornes, BatteryStatus = 2, 98 %) — médianes de banc :

| domaine | mailles | 8 cycles | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| 32×24×36 (cas S298) | 27 648 | 0,305 ms | 0,382 | 0,532 | 0,838 |
| 32³ | 32 768 | 0,329 | 0,410 | 0,561 | 0,876 |
| 64×64×32 | 131 072 | 1,106 | 1,299 | 1,697 | 2,504 |

Dispatchs : 57, 97, 177, 337. Un maximum isolé à 7,34 ms (64×64×32, 8 cycles, premier passage
8,39 ms) : pic d'amorçage de la famille A294, non attribué. **Ce n'est pas la porte C** (pas de
scène, pas de 99ᵉ centile) : 64×64×32 tient sous 2 ms jusqu'à 32 cycles en médiane de banc.

**P5, horizon mesuré** (trajectoire relancée avec la métrique) : premier millimètre au pas
260 (8 et 16 cycles) et 220 (32 et 64), contre 260 cœur contre cœur ; pire écart avant l'horizon
9,1·10⁻⁴ (8), 5,8·10⁻⁵ (16), 1,0·10⁻⁴ (32), 2,4·10⁻⁵ m (64) ; pire sur 6 s 3,4 à 4,3 cm, contre
4,5 cm pour la référence contre elle-même.

*Tenue du plan* — **fusion déclarée P5+P6+P7 en un commit** : leur code a été écrit pendant les
calculs de P5 (4 à 8 min chacun) dans le même fichier, et leurs mesures sont consignées ici.
Le banc de localisation `--delta3d-fond-s298` est gardé : il a écarté B.

**P7 reçu** (`--delta3d-diagnostics`) — justesse, depuis l'état de P4 : divergence des lignes
franches carte **1,1 à 1,8·10⁻⁶** à 128 cycles contre **2,7 à 7,3·10⁻⁶** pour le cœur (sa
projection s'arrête à ses critères, la carte fait 128 cycles) ; toutes lignes 2,5·10⁻⁶ à
8,4·10⁻⁵. À 16 cycles depuis p = 0 : 1,6 à 2,7·10⁻² → **déclaré dégradé**. Différé, 100 pas du
cas S298 à 16 cycles par le seul `step` : 97 diagnostics rendus, **âge 1 pas** pour 96 d'entre
eux, 3 appels sans retour (premiers pas), aucune attente, 68 ms au mur pour les 100 pas.
**84 pas sur 100 déclarés dégradés à 16 cycles** (franche jusqu'à 1,2·10⁻³) alors que la
trajectoire à 16 cycles suit le cœur à 5·10⁻⁵ m jusqu'à t = 1 s : la tolérance d'ADR-144 est
bien plus stricte que l'usage en hauteur — à documenter, sans la relever.
Anneau de 3 emplacements ; si aucun n'est libre, le diagnostic du pas est perdu, jamais le pas
retardé.

---
