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

Session : S229 — en cours
Agent : Codex, GPT-6 (fichiers, git, cargo et Python disponibles)
Entrée : « Continue », après S228 ; quatre copies à c2e9a56, master propre, jeton libre.

**Objectif.** Sauvegarder puis restaurer V avec ses nœuds, ses restes et l'identité de sa configuration géométrique ; recevoir la continuation dans le chemin réel.
**Critère d'arrêt.** Une simulation interrompue puis restaurée reprend à l'identique ; une configuration incompatible ou une sauvegarde invalide est refusée sans mutation. Portée et exclusions explicites. Comparer ensuite la priorité de J2 aux reliquats de V.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — lectures prescrites puis contrat de restauration et réutilisation des mécanismes existants.
- [x] **P3** — construire sauvegarde/restauration de V et refus atomiques.
- [>] **P4** — recevoir la continuation et les données invalides ; publier la preuve.
- [ ] **P5** — rituel §6, file entière, priorité suivante, journal, jeton et synchronisation.

Chaque étape reste sous quinze minutes ; découpage déclaré si nécessaire.

### Notes de reprise

Base S228 : 398 tests réussis, 5 ignorés ; A266 corrigée dans le domaine reçu. Aucun nouveau test général à l'amorce. Aucun travail dans une nouvelle copie.

P2 : ADR-140 précise ADR-022 : écarts à la base auteur, restes non nuls, empreinte de toute la géométrie et contexte de reprise. Mécanisme FNV existant réutilisé, sans garantie cryptographique ; aucun asset ni champ δ écrit. Base immutable empruntée, reconstruction totale des sorties depuis la base après validation. Priorité V bornée justifiée en S228, ensuite J2.

P3 : hydro_network::snapshot construit, WVST V1 (88 + 12 par écart), base empruntée et FNV calculé une fois, capture/restauration atomiques. Cinq tests ciblés passent : continuation tabulée 1000 pas avec témoin sans reste divergent, reconstruction depuis pools sales et bits de contexte, configurations incompatibles, troncatures et altérations de chaque octet, erreurs sémantiques avec contrôle recalculé. Seul avertissement neuf (mut inutile du test) retiré. Reste réception orientée/allocations puis suite complète.
