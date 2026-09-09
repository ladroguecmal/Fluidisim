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

Session : S118 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : exercer le contrôleur de publication dans un cycle hôte temporel mixte
(S117-1) — recettes 224×128 et 256×128, plusieurs changements d'instant, identité en
bits avec la voie directe, chemin `Unchanged`, refus, coût mise à jour + requête 64.

### Plan

- [x] **P1** — vérifier la passation, prendre le jeton, déclarer ce plan.
- [ ] **P2** — écrire la campagne `cycle_mixed` : contrôleur piloté par une séquence
      d'instants (avance, retour, répétition), requête mixte B+impact+pression sur la
      publication, comparaison en bits à la préparation directe, refus hors fenêtre et
      vue à une date non publiée.
- [ ] **P3** — mesurer `update`+`current`+requête 64 aux deux recettes, recevoir la
      campagne en release, lire ce que les chiffres disent.
- [ ] **P4** — publier le livrable, rituel de fin (`REPRISE.md` §6) et synchronisation
      `--ff-only` vers master.

### Notes de reprise

Départ c16c308 (= master, quatre worktrees au même commit sauf 5134cd archivé et c107bf
à S44). Copie de travail : `claude/reprise-projet-2d3506`.

Ce que S117 laisse acquis : `Controller::{new,update,published_time,state,current}`,
deux pools disjoints, bascule après succès intégral, `Unchanged` sur même instant,
`Err(Time)` pour une vue à une date non publiée. Reçu à la recette 16×24 seulement —
S118 doit l'exercer aux recettes que S116 a reçues spatialement (224×128, 256×128).

Attendu de la mesure : le contrôleur ne recopie pas les coefficients, donc
`update`+`current` devrait coûter la préparation directe de S116 (48–58 ms) sans surcoût
mesurable ; c'est cela qu'il faut vérifier, pas seulement l'égalité en bits.
