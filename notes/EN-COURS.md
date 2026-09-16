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

Session : S257 — terminée
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-16 22:45), verdict R2 de l'utilisateur : « le résultat se raffine, mais la
topologie d'un océan fluctue selon plusieurs paramètres ; le rendu paraît un grand lac soumis à
beaucoup de vent ; la haute mer semble plus déchaînée, de manière chaotique, et la houle se forme
petit à petit vers les terres ». Et une question : « avait-on réfléchi à une surface lisse avec
effet de normales, déplacement ou autre technologie quand on regarde la mer de dessus, le rayon de
vision perpendiculaire à l'eau, puisqu'une surface plane devrait suffire ? ». Master propre
d6a4253, jeton libre.

Objectif : consigner et classer R2 par ce qui se **calcule** (contenu du spectre, cambrure,
absence de houle, fond uniforme, crêtes linéaires), sans campagne ; répondre à la question depuis
le dépôt (ADR-004, S234/L313, ADR-155) et par un critère chiffré de parallaxe aux poses R1/R2.
Pas de code dans cette session : le prochain lot se choisit sur ce classement. Arrêt : verdict
consigné, réponse écrite, prochain lot recommandé dans la file.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — verdict R2 consigné et classé, faits calculés ; réponse à la question (parallaxe du
  déplacement contre empreinte du pixel, ce que le dépôt prévoyait) dans REVUE-VISUELLE.
- [x] **P3** — rituel §6 : file (houle longue, crêtes non linéaires, LOD déplacement/normales),
  liste du projet fini, journal, jeton.

### Notes de reprise

P2 : R2 classé par construction (mer de vent PM ≈ 8,4 m/s, rien au-delà de 12 s, λp 56 m contre
225 m pour une houle de 12 s, cambrure 0,027, un éventail, aucun déplacement horizontal — grep vide
dans le cœur —, fond uniforme, pas d'écume). Parallaxe `h·sin2θ/(2Hα)` : 1,2–3,4 px en plongeante à
90 m, 3–10 px en haute, 6–16 px en référence/rasante, < 0,1 px pour la queue. Ordre recommandé :
B multimodal, crêtes non linéaires, écume, levée bathymétrique (J5). Deux heredocs bash imbriqués
échouent à l'analyse sur ce poste : écrire les scripts dans le scratchpad.
