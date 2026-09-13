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

Session : S225 — en cours
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue avec S225 », même conversation. master et trois copies à 59116c8, jeton libre,
maillons 0. Copie principale.
Objectif : **cadence complète de l'hôte** — travail nécessaire de J1 (ADR-131 D6), nommé depuis
S213, jamais mesuré, reporté explicitement par S224. La promesse se tient ici.

### Ce que l'hôte mesure aujourd'hui, et ce qui manque

`Gpu::benchmark` dessine **hors écran**, sur une texture créée pour l'occasion, et publie deux
nombres : `CPU_prepare_upload_submit_ms` et `GPU_water_ms`, ce dernier avec une réserve écrite dans
la ligne elle-même — *« sky, upload, readback, presentation excluded »*. La paire d'horodatages
n'entoure que la passe « water only » ; le ciel est dessiné et **non chronométré**.

Il manque donc trois choses, et ce sont exactement celles qu'une cadence exige :

1. **La chaîne d'échange.** Acquérir une image, présenter, et le coût de ces deux gestes.
2. **Le coût de ce qui a été exclu.** Ciel, transferts, présentation : la réserve est écrite depuis
   S211 et n'a **jamais été chiffrée**. Une exclusion annoncée n'est pas une exclusion mesurée.
3. **La cadence elle-même** — l'intervalle réel entre deux images, et non la somme de passes
   isolées, dont rien ne dit qu'elle est atteinte.

**Le piège est dans la fenêtre.** Le mode de présentation est `AutoVsync` : l'intervalle entre deux
images y mesure **l'écran**, pas le coût. Mesurer la cadence sous vsync donnerait 60 ou 120 Hz quel
que soit le travail, et ce serait un chiffre vide. La mesure se fera donc **sans vsync**, et le
dira.

### Thèse et critères, déclarés avant toute mesure

1. **Une cadence est un intervalle, pas une somme.** Publier la distribution des intervalles réels
   entre présentations — médiane, p95, maximum — sur une fenêtre ouverte, sans vsync.
2. **Chiffrer les exclusions.** Horodater la **trame entière** (ciel + eau) en plus de la passe
   d'eau seule, et mesurer séparément l'acquisition d'image et la présentation. La part de l'eau
   dans la trame devient alors un fait, pas une hypothèse.
3. **Confronter à ADR-125, dans ses propres termes** : 60 images/s, soit **16,67 ms** par image, et
   **2 ms** pour l'eau. Dire les deux — l'hôte tient-il 60 Hz, et l'eau tient-elle sa part ?
4. **Rang de passage** (L289) : au moins deux passages séparés, écart publié. Une cadence est une
   mesure de temps, et cette machine varie de 20 % entre passages.
5. Publication avec en-tête ADR-131 D3 : techniques présentes, absentes, domaine.

**Prédiction écrite pour être contredite** : la trame complète à 960×540 coûtera 6 à 7 ms de GPU —
eau 4,2 plus ciel et présentation — et 2,5 ms de CPU, donc une cadence sans vsync autour de
**150 Hz**, largement au-dessus des 60 Hz d'ADR-125. Et les exclusions de S211–S213 se révéleront
**petites devant l'eau** : je prédis que ciel, transferts et présentation pèsent ensemble moins que
la passe d'eau, donc que les mesures antérieures n'ont rien caché de matériel. Si c'est faux — si
l'exclu pèse autant que le mesuré — alors tous les verdicts de coût depuis S211 portent sur une
fraction du problème, et c'est la découverte de la session.

### Plan

- [x] **P1** — jeton, ce qui manque, thèse, critères, prédiction, plan seuls.
- [x] **P2** — instrumenter : horodatage de la trame entière, acquisition et présentation séparées, intervalle réel ; mode sans vsync déclaré.
- [x] **P3** — mesurer la cadence sur fenêtre ouverte, deux passages, aux deux formats ; publier les distributions.
- [x] **P4** — chiffrer les exclusions : part de l'eau dans la trame, coût du ciel, des transferts et de la présentation.
- [x] **P5** — confronter à ADR-125 : 16,67 ms et 2 ms, et dire ce que la scène J1 tient et ne tient pas.
- [ ] **P6** — document de réception (en-tête ADR-131 D3, rang de passage) ; suite complète `code/`.
- [ ] **P7** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2+P3+P4 : un seul programme instrumenté porte les trois étapes. `Gpu` passe à **quatre**
horodatages — eau en 0/1, ciel en 2/3 — et `gpu_breakdown` rend la passe d'eau **et** la trame
complète, du début du ciel à la fin de l'eau. Le mode `--cadence` ouvre une vraie fenêtre en
**`AutoNoVsync`**, et il le dit dans sa ligne : sous vsync, l'intervalle mesure l'écran et non le
coût.

**Deux phases, et la séparation n'est pas un confort.** Relire un horodatage appelle
`poll(wait_indefinitely)`, qui **sérialise** CPU et GPU : mesurer la décomposition à chaque image
détruit le recouvrement, donc la cadence qu'on prétend mesurer. Phase 1 — 590 images, aucune
relecture : l'intervalle réel. Phase 2 — 200 images avec relecture : la décomposition, publiée sous
le nom `DECOMPOSITION_serialisee`, qui n'est **pas** une cadence.

| grandeur | médiane | p95 | max |
|---|---:|---:|---:|
| intervalle | **5,0450 ms** (198,2 Hz) | 5,8988 | 6,5087 |
| CPU de trame | 4,3160 | 5,1743 | 5,8655 |
| — dont acquisition d'image | **2,2212** | — | 3,9164 |
| — dont sillage (cœur) | 1,2453 | — | 2,2971 |
| — dont transfert | 0,2076 | — | 0,4883 |
| — reste (B, profil, soumission) | 0,6419 | — | — |
| présentation | 0,5989 | — | 1,8279 |
| GPU eau | 4,1585 | — | 4,8583 |
| GPU trame entière | 4,2455 | — | 4,9569 |

**Rang de passage** (L289) : quatre passages, intervalle 4,9187 / 4,9560 / 4,9433 / 5,0450 ms —
**écart 2,6 %**, et 0,4 % sur le GPU d'eau. C'est bien plus reproductible que les 20 à 40 % que S213
avait relevés sur son CPU : une mesure prise **sous charge soutenue** ne varie pas comme une mesure
prise en rafales courtes. À verser à L289.

**La prédiction est confirmée sur une moitié et contredite sur l'autre.**

- **Confirmée, et plus fortement que prédit** : les exclusions de S211–S213 — ciel, transferts,
  relecture, présentation — ne pèsent **rien** sur le GPU. La part de l'eau dans la trame vaut
  **0,9795 à 0,9806** sur quatre passages : tout le reste du GPU fait **0,087 ms**. La réserve
  écrite dans la ligne depuis S211 était honnête et, maintenant, chiffrée : elle ne cachait rien.
- **Contredite** : j'annonçais 6 à 7 ms de GPU pour la trame complète et ~150 Hz. La trame coûte
  **4,25 ms** et la cadence atteint **198 à 203 Hz**.
- **Non prédit, et c'est le fait neuf** : le CPU d'une vraie trame est dominé par **l'attente**.
  L'acquisition d'image vaut **2,22 ms sur 4,32**, soit **51 %** — et ce n'est pas du travail, c'est
  la contre-pression du GPU. Le travail réel du CPU fait 2,10 ms. Aucun banc hors écran ne pouvait
  le voir : `benchmark` dessine sur une texture et n'acquiert jamais rien.

Contrôles : `VERIFY` inchangé (7,2271e-5 m à 16 s), `BENCH` inchangé (GPU eau 4,2037 ms),
`--smoke` 120 images — les quatre horodatages n'ont pas déplacé les chemins existants.

P5 : **ADR-125 pose deux nombres, et la cadence en éclaire un troisième.**

| grandeur | mesurée | part d'une trame de 60 Hz (16,67 ms) | budget ADR-125 |
|---|---:|---:|---|
| GPU eau | **4,1585 ms** | **24,9 %** | 2 ms, soit 12 % — **dépassé 2,08 ×** |
| GPU trame (eau + ciel) | 4,2455 | 25,5 % | — |
| trame complète (intervalle) | 5,0450 | **30,3 %** | — |
| travail réel du CPU | 2,10 | 12,6 % | — |

**La question « l'hôte tient-il 60 Hz » a une réponse, et ce n'est pas la bonne question.** Il les
tient trois fois — 198 Hz. Mais ADR-125 ne demande pas que l'eau tourne seule à 60 Hz : elle lui
accorde **2 ms d'une trame de 16,67** qui doit aussi porter un jeu. L'eau en prend 4,16, et la scène
entière 5,05 : il resterait **11,6 ms** pour tout le reste au lieu des 14,67 prévus.

Le verdict de coût ne bouge donc pas — il est **confirmé par un autre chemin**, et le rapport 2,08
recoupe les 4,18 ms de S213 à 0,5 % près.

**Ce que la cadence ajoute, et que les passes isolées ne pouvaient pas dire : où optimiser.** Le
travail réel du CPU tient dans **2,10 ms**, entièrement recouvert par les 4,16 ms du GPU — la trame
est **bornée par le GPU**, et l'acquisition d'image (2,22 ms) en est la contre-pression, pas un
coût. Toute seconde gagnée sur le CPU serait donc **invisible** tant que le GPU domine. Les quatre
techniques qui restent à J1-bis — espace, LOD, visibilité, mutualisation — sont toutes du côté GPU :
la mesure confirme la trajectoire au lieu de la contredire, et c'est la première fois qu'elle est
confirmée **par une cadence** et non par une passe.

Réserve à ne pas franchir : l'hôte ne dessine **que** de l'eau et un ciel, sur une caméra fixe, sans
interface, sans ombres, sans autre géométrie. Les 198 Hz ne disent rien d'un jeu ; ils disent que
l'eau, seule, laisse 11,6 ms.
