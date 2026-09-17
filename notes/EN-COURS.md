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

Session : S263 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-17 08:09), verdict R5 : « trop rugueuse, trop de petits pics, je ne connais pas le
niveau de vent ». Master propre 7a88546, jeton libre, maillons 0.

Constat S261 : Cox–Munk borne la rugosité **à un vent donné**, et la scène est une mer pleinement
développée à environ 8,4 m/s. Tout ce qui fait « trop rugueux » dépend du vent : `Hs` et `Tp` par
Pierson–Moskowitz, `mss` par Cox–Munk (SPEC-001 §1 sexies). Le vent de la référence est inconnu.

Objectif : **le vent devient un paramètre de la scène** (`--vent=U`), dont se déduisent la mer de vent
(`Hs = 0,21·U²/g`, `ωp = 0,877·g/U`) et la coupure de la queue. Celle-ci est choisie pour que la `mss`
totale égale Cox–Munk au même vent, sans jamais dépasser la limite gravité-capillarité (1,7 cm).
Houle, modulation `M` = 2 et CWM sont inchangés. **Calibration perceptive** : l'utilisateur choisit
parmi des rendus à 3, 5 et 8,4 m/s, tous conformes à Cox–Munk à leur vent. Sans `--vent`, toutes
les scènes restent au bit.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — verdict R5 consigné ; ADR-160 (vent de scène) et protocole, avant code.
- [ ] **P3** — hôte : recette de vent, queue coupée à la `mss` de Cox–Munk ; instrument statistique
  par vent (`mss`, pointe, replis) ; scènes existantes au bit.
- [ ] **P4** — rendus de calibration R6 (trois vents × poses) envoyés, consignés.
- [ ] **P5** — rituel §6.

### Notes de reprise

