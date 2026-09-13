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
- [ ] **P2** — instrumenter : horodatage de la trame entière, acquisition et présentation séparées, intervalle réel ; mode sans vsync déclaré.
- [ ] **P3** — mesurer la cadence sur fenêtre ouverte, deux passages, aux deux formats ; publier les distributions.
- [ ] **P4** — chiffrer les exclusions : part de l'eau dans la trame, coût du ciel, des transferts et de la présentation.
- [ ] **P5** — confronter à ADR-125 : 16,67 ms et 2 ms, et dire ce que la scène J1 tient et ne tient pas.
- [ ] **P6** — document de réception (en-tête ADR-131 D3, rang de passage) ; suite complète `code/`.
- [ ] **P7** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*
