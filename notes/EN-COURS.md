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

Session : S227 — en cours
Agent : Codex, GPT-6 (fichiers, git, cargo et Python disponibles)
Entrée : audit global demandé par l’utilisateur : intentions, dérives, procédure, documentation,
zones d’ombre, améliorations et correctifs du code. Copie principale, master à dfd1507, propre ;
trois copies propres au même commit, lignée B archivée. Aucun distant. Maillons 0 à l’entrée.

**Objectif.** Produire un diagnostic fondé sur le dépôt et appliquer les corrections bornées qui
rendent la reprise et le prochain lot de construction plus fiables. L’audit demandé prend la
place de la suite automatique A266 ; celle-ci reste prioritaire pour V, sans réduction d’ambition.

**Hypothèses à éprouver.** L’histoire est recopiée dans les points d’entrée ; le compteur de
maillons mesure des fichiers touchés plutôt qu’une capacité utilisable ; des blocages hérités
peuvent être périmés. Le code sera examiné sur ses chemins exécutés, et un défaut ne sera corrigé
qu’avec un cas qui le reproduit. Aucun objectif physique ni seuil ne sera réduit pour faciliter
l’audit. Pas de réécriture d’ADR ou des sources initiales.

### Plan

- [x] **P1** — amorce, état réel, jeton et plan seuls.
- [x] **P2** — intentions initiales, feuille de route, file active, métriques documentaires et code ; consigner le diagnostic factuel.
- [x] **P3a** — points d’entrée et file active réécrits au présent ; méthode proportionnée et critère de capacité reçue.
- [>] **P3b** — remplacer l’indicateur de vélocité par un inventaire portable, sans faux score de productivité.
- [ ] **P4** — reproduire et corriger les défauts de code bornés issus de l’audit ; sinon documenter les correctifs prioritaires avec critères de réception.
- [ ] **P5** — vérifier les changements, achever le bilan global et ordonner les prochains lots selon leur effet sur le système.
- [ ] **P6** — rituel de fin §6 : journal, angles/leçons utiles, file active, index, jeton libre et copies synchronisées.

Chaque étape reste sous quinze minutes ; découpage déclaré ici si nécessaire.

### Notes de reprise

REPRISE lu intégralement malgré sa taille. Git/cargo/Python disponibles ; Git Bash disponible hors
PATH. Aucune campagne historique n’a été relancée.


P2 : bilan préliminaire dans docs/registres/BILAN-GLOBAL-S227.md ; base release 383 réussis,
5 ignorés. Confluence V et soustraction i64 ciblées ; A266 a aussi un écart de convention à vérifier.
P3 découpée en P3a (documentation/procédure), P3b (indicateur portable corrigé).

P3a : historique conservé dans Git dfd1507 et le journal ; ADR/sources intacts. Feuille de route
J1 réconciliée avec S223/S225 ; autorisations et périmètre conservés.
