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

Session : S155 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S154-1. B2 mesure des bilans à 60 s ; le noyau de pression refuse au-delà de 16 s.
Prendre la seconde branche annoncée par S154 — **isoler par mesure le blocage numérique** —
parce que la première (quantifier le domaine d'un sillage prolongé) est inaccessible tant que le
noyau refuse la durée à laquelle B2 mesure. Question : le 16 s d'ADR-071 est-il une limite
numérique ou un périmètre déclaré ? ADR-071 dit lui-même « à calibrer par réception » ; personne
ne l'a fait.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [ ] **P2** — sonde : erreur du noyau modal contre l'oracle f64 `PressureMode` pour des âges de
      0 à 64 s, durée active inchangée. La constante d'horizon est relevée **localement et non
      committée** — c'est ce qui rend la mesure possible, et rien d'autre ne change.
- [ ] **P3** — séparer ce que « 16 s » recouvre : l'**âge** auquel on échantillonne et la **durée
      active** du forçage n'empruntent pas le même chemin numérique. Mesurer la seconde seule.
- [ ] **P4** — décider d'après les chiffres : nouvel ADR si les deux bornes se séparent, ou
      provenance mesurée écrite pour la borne conservée. Un ADR n'est jamais réécrit.
- [ ] **P5** — appliquer la décision dans le code, avec un test **témoin** : désactiver le
      mécanisme doit faire échouer le test, sinon le test ne prouve rien.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 1643232 = master (S154, Codex). Worktree remis en avance rapide, rien d'unique.
296 tests/cinq ignorés, 105 ADR, 212 angles, 230 leçons, 18 invariants.

Ce que la lecture du noyau donne **avant** toute mesure, et qui oriente la sonde :
- après extinction, `sample` calcule la rotation libre par `phase(frequency, age - active, 1e6)`,
  arithmétique **entière** i128 réduite modulo un tour ; aucun flottant ne porte le temps ;
- la seule accumulation f32 dépendant du temps est `scale_integer(sinc/1e6, us)` dans la branche
  proche de zéro de J, et son argument est `active = min(age, duration)`, **borné par la durée** ;
- le commentaire de `scale_integer` dit « au plus 24 bits pour une durée <=16 millions de µs » :
  24 bits, c'est 2^24 = 16 777 216 µs. Le 16 s a donc l'air d'être un **nombre de bits**, pas une
  seconde physique.

Prédiction écrite avant la mesure, pour qu'elle puisse être démentie : l'erreur sera à peu près
**plate** en âge et croissante en **durée active**. Si elle croît aussi en âge, la prédiction est
fausse et c'est le résultat le plus intéressant de la session.

Piège : mesurer contre un oracle qui partagerait la quantification f32 de ω masquerait justement
ce qu'on cherche. L'oracle S95 convertit le **même** f32 en f64 — il isole l'erreur
d'implémentation, pas celle de l'entrée. Garder cette convention et le dire.
