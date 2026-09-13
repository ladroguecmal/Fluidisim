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

Session : S214 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **validité avant accélération** (ADR-131 D6) — faire composer l'impact et le sillage par
le cœur (`mixed_water`) sur la scène J1, exercer le budget conjoint, puis instruire A251 (emprise et
durée honnêtes du sillage, reçues par coutures comme ADR-126).

### Entrée

Ligne `Session suivante` de S213, elle-même conforme à ADR-131 D6 et à la §Suite de
[TEMPS-SILLAGE-S213](../docs/validation/TEMPS-SILLAGE-S213.md). Aucune entrée utilisateur nouvelle.
Maillons 0 à l'amorce (S213 a avancé W dans `code/water-core/src` et acté ADR-131) : le chaînage est
licite, et la ligne nomme un **travail nécessaire de J1**, pas un reliquat d'optimisation.

### État réel constaté à l'amorce

`master` et **trois** copies de travail (`fluidisim-viz-navigation-8895b1`,
`project-status-progress-d31d78`, `reprise-projet-2d3506`) toutes propres au même commit `08c2011`.
Jeton `libre`, battement 10:56, aucune session concurrente. Machine : AMD Ryzen AI 7 350,
RTX 5070 Laptop (DX12). Aucune dépendance nouvelle prévue.

### Le défaut que cette session vise, énoncé avant toute mesure

**L'hôte compose de sa propre main, et n'a jamais demandé au cœur si sa composition est
admissible.** `scene::FrameData::references` somme B, l'impact et le sillage terme à terme
(`background.eval` + `RadialImpact::sample` + `bound_pressure::Prepared::sample_batch`), et le
shader fait de même côté GPU. Le cœur possède pourtant le chemin qui **refuse** :
`mixed_water::sample_world_batch`, dont le budget est la **somme** des majorants de pente des
**perturbations** (ADR-128, S205) — `slope_max()` de chaque champ d'impact plus `slope_envelope()`
de la pression — comparée à `max_slope`. Les deux moitiés ont été admises séparément : l'impact sur
cette mer en S205 (zéro refus), le sillage seul en S212 (enveloppe ≤ 0,165 contre 0,449).
**Leur somme n'a jamais été évaluée** (HOTE-GPU-S212 §Admission : « non exercé »).

### Thèse et critères, déclarés avant toute mesure

1. **Accord.** `mixed_water::sample_world_batch` et la somme à la main de l'hôte donnent la même
   hauteur et les mêmes pentes, aux mêmes points et aux mêmes instants, à **1e-6** de l'amplitude
   locale. Un écart au-delà est un défaut de l'un des deux chemins, pas une tolérance.
2. **Budget conjoint.** `mixed_water::slope_floor(impacts, pressure)` publié à chaque âge, contre
   `max_slope = BREAKING_SLOPE` (π/7 ≈ 0,4488). La somme est la seule forme portable (ADR-119
   règle 1) ; aucune loi du maximum n'est employée.
3. **Refus, s'il y en a.** Localisés (point, instant) et **qualifiés** : `Slope` (la pente réelle
   des perturbations dépasse) ou `SlopeEnvelope` (seuls les majorants dépassent) — la distinction
   d'ADR-098/A208 change entièrement le verdict, et le second n'est pas un défaut de physique.
4. **Aucun seuil de réussite présumé.** Le budget de pente est un contrat d'admission, pas le
   budget de 2 ms ; les deux ne se mélangent pas.
5. Chaque mesure publiée avec **techniques présentes, absentes et domaine de validité** (ADR-131 D3).

**Prédiction écrite pour être contredite** : le budget conjoint reste sous π/7 à tous les âges, donc
zéro refus, et l'accord des deux chemins tombe au niveau de l'arrondi f32 (≲1e-7). Si elle tient,
elle ne prouve que la scène présente ; si elle tombe, l'image montre aujourd'hui de l'eau que le
cœur refuserait.

### Plan

- [x] **P1** — jeton, entrée, défaut visé, thèse, critères et plan seuls.
- [x] **P2** — scène : journal d'impact et montage `mixed_water` (`prepared_water::Prepared::build`) ; compilation, test minimal.
- [x] **P3** — accord : composition du cœur contre la somme à la main, aux âges déclarés, sur la grille projetée ; écart publié.
- [x] **P4** — budget conjoint : `slope_floor` impact + sillage à chaque âge contre π/7 ; refus localisés et qualifiés.
- [ ] **P5** — hôte : `--verify` passe par la composition du cœur ; contrôles S212/S213 conservés ; coût publié avec son en-tête.
- [ ] **P6** — A251 : emprise et durée honnêtes déduites de la recette et de la vitesse ; couture spatiale et temporelle mesurées comme ADR-126.
- [ ] **P7** — A251 : réception des coutures et refus nommé ou garde ; publication avec la fixture.
- [ ] **P8** — document de réception (en-tête ADR-131) ; suite complète `code/`.
- [ ] **P9** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

P2 : `scene::MixedStore` / `MixedOutcome` / `mixed_compose` — journal d'événements (`confirm`,
Origin::Server), `prepared_water::Prepared::<256>::build`, `bound_pressure::Prepared::from_journal`,
puis `mixed::admits` point par point, `mixed::sample_world_batch` par point (pour localiser) **et**
en lot (verdict que l'hôte recevrait). `Scene` porte désormais `event`, `medium`, `domain`.
Ligne `MIXED` dans `--verify`, aux cinq âges du témoin S212.
**Premiers chiffres** : floor 0,347679 (4 s) · 0,357528 (8 s) · **0,377603 (16 s)** · 0,369652 (24 s)
· 0,370588 (39 s) contre max_slope 0,448799 ; impact seul **0,212607** (constant), sillage 0,135072
à 0,164995. **Zéro refus**, lot admis aux cinq âges — la prédiction tient.
**Mais deux constats non prédits, à instruire en P3/P4** :
1. **84 % du budget conjoint est consommé à 16 s** par *une* source de chaque type. La marge est
   0,0712 ; un second sillage de la même recette (0,165) ou un second impact (0,213) refuserait
   **tout** lot non vide. Le budget est une somme (ADR-119 règle 1, ADR-128) : il ne passe pas à
   l'échelle en nombre de sources, et la scène J1 est déjà à la limite avec deux.
2. **`admitted = 4477 / 6988`.** `mixed::admits` exige le point dans **l'intersection** de tous les
   domaines (ADR-077 « leur intersection détermine », ADR-080 : lot atomique). Hors du disque de
   52 m de l'impact, le cœur ne compose pas — alors que l'hôte y dessine B + sillage. Le chemin
   mixte du cœur **ne peut donc pas être** le chemin de rendu tel quel, et c'est précisément
   pourquoi l'hôte sommait à la main. À qualifier : décision de service (ADR-077) contre besoin
   d'image (ADR-129 §3), pas un défaut d'implémentation.

P3 : accord mesuré aux cinq âges, sur les 4 477 points admis. **Le critère déclaré (1e-6 de
l'amplitude) échoue, et la prédiction est contredite** : `d_eta` max **1,78e-5 m** à 4 s pour une
amplitude de 0,968 m, soit **1,8e-5** relatif — 18× le critère ; `d_slope` max 2,44e-5. Aux autres
âges 1,25e-5 / 1,34e-5 / 9,5e-6 / 6,9e-6 m.
**Cause isolée et prouvée dans le même passage** : `MIXED_POINT` compare le cœur à la **même somme
à la main, au point local que le cœur emploie** — `d_eta_local = 0,000000000` aux cinq âges (bit à
bit sur η) et `d_slope_local = 1,5e-8` (aller-retour pente → normale → pente, 1 ulp). **La
composition du cœur est donc exacte ; tout l'écart vient du point d'évaluation.**
Mécanisme : `FrameData::references` évalue **B** au point monde quantifié (`WorldPos::from_metres`,
`WORLD_UNITS_PER_METRE = 2048`, pas de 488 µm, erreur ≤ 244 µm) et **l'impact et le sillage** au
point `f32` brut. Le cœur, lui, convertit **une seule fois** monde → local et sert les trois couches
au même point (`eval_local` : « chemin interne après conversion commune B/W »). Une même sonde a
donc deux positions dans la référence de l'hôte, distantes de ≤ 244 µm ; le produit par la pente des
perturbations donne les 18 µm mesurés.
Portée : 0,6 % de la tolérance de 3 mm de `--verify`, donc jamais visible — mais c'est la
**référence** qui est fausse, pas le GPU, et un LOD spatial (J1-bis) qui creuserait la pente
augmenterait l'écart. À ouvrir en angle mort (A253) et à corriger en P5.

P4 : budget conjoint mesuré aux cinq âges, et **les deux causes de refus séparées** par deux seuils
déduits des mesures (le budget étant point-indépendant, `floor` **est** le seuil de bascule).

| âge (s) | floor | impact | sillage | part de π/7 | pente réelle max | majorant / réel |
|---:|---:|---:|---:|---:|---:|---:|
| 4 | 0,347679 | 0,212607 | 0,135072 | 77,5 % | 0,092876 | **3,74** |
| 8 | 0,357528 | 0,212607 | 0,144921 | 79,7 % | 0,089615 | 3,99 |
| 16 | **0,377603** | 0,212607 | 0,164995 | **84,1 %** | 0,073979 | 5,10 |
| 24 | 0,369652 | 0,212607 | 0,157044 | 82,4 % | 0,051954 | 7,11 |
| 39 | 0,370588 | 0,212607 | 0,157981 | 82,6 % | 0,035240 | **10,52** |

Seuils : à `max_slope` entre la pente réelle et `floor` (0,2028–0,2258 selon l'âge), **4 477 points
sur 4 477 refusés, tous `SlopeEnvelope`, zéro `Slope`** — le refus vient entièrement de la marge.
À la moitié de la pente réelle, 16 à 182 points passent en `Slope` selon l'âge, le reste reste
`SlopeEnvelope`. Le lot rend `SlopeEnvelope` dans les deux cas : c'est le premier point qui parle.

**Ce que cela dit, et qui n'était pas connu.** Le majorant conjoint est **3,7 à 10,5 fois** la pente
réelle, et la scène J1 — *une* source de chaque type — occupe déjà **84 %** de π/7. Marge restante
0,0712 : un second sillage de la même recette (0,165) ou un second impact (0,213) **refuserait toute
l'image**, et par majorant, pas par raideur — la physique garde un facteur dix. A208 nommait le
mécanisme sur un champ seul et par le choix d'emprise ; ici c'est **l'additivité sur le nombre de
sources** (ADR-128, ADR-119 règle 1) qui borne la scène, et elle n'a jamais été mesurée composée.
Angle mort à ouvrir (A254, sévérité 1 : c'est un refus, pas un défaut cosmétique).
