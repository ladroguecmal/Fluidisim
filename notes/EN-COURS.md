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

Session : S131 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S130-1 — sortir de la saturation. Après un `Full`, le contrôleur conserve sa
publication mais ne peut plus changer d'instant. Le chemin de sortie existe déjà ; il s'agit
de le parcourir en entier, de recevoir ce qu'il garantit, et de mesurer ce qu'il coûte.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — oui, il est possible : `copy_into` ne refuse que si le stockage est plus
      petit que la publication. ADR-087 et `required_capacity`.
- [x] **P3** — cycle reçu, champ d'après identique en bits à la voie directe et différent
      de l'ancien.
- [x] **P4** — balayage sur les tailles, chaque étape distinguée ; service maintenu à chaque échec.
- [x] **P5** — élargissement 0,1 µs service maintenu ; reconstruction 12,21/13,41 ms.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 57d0a3b = master, trois copies coïncidentes.

Ce que la lecture a déjà établi, et qui change la forme attendue de la réponse :

- **`copy_into` prend `&self`**, pas `&mut self`. Or le contrôleur expose `journal()` en
  lecture seule depuis S130. L'élargissement peut donc se faire **pendant que le contrôleur
  sert encore**, et la reprise aussi : seule la reconstruction impose de le libérer.
  La fenêtre sans champ se réduit donc à une préparation, et non à tout le cycle.
- **`copy_into` copie l'attente** avec la publication, sans admission implicite.
- **`copy_into` refuse seulement si `slots.len() < count`.** Un pool de taille exactement
  `count` passe la copie et laisse l'attente irrésolue : `retry` y rendra `Full` à nouveau.
  Un élargissement peut donc « réussir » sans sortir de la saturation, et rien ne le dit à
  l'appelant avant qu'il essaie. C'est le candidat le plus sérieux pour un ajout d'API.

Piège à éviter : reconstruire d'abord et élargir ensuite. L'ordre importe, et il est mesurable.

P2-P6 : ADR-087, SORTIE-SATURATION-S131, journal, index, README, REPRISE, jeton rendu, ff-only.
260 tests/cinq ignorés, ciblé aussi en release, hachages inchangés. Aucun angle ni leçon nouveaux.

Une mesure corrigée avant publication : placée d'abord avant le bloc de mise en régime, elle
donnait une médiane tenable mais un maximum à 35 ms. Déplacée après (A195).

Pour S132 sans relire : S131-1 est un chemin incrémental. La superposition modale est linéaire
et S112 l'a reçue (champ multisource = somme des contributions, interférences conservées). L'idée
est d'ajouter au champ publié la contribution de la seule source admise, au lieu de recalculer
toutes les sources. Attention : `prepare_segments` accumule sur tous les segments de toutes les
sources publiées, dans l'ordre canonique ; ajouter après coup change l'ordre de sommation, donc
**l'identité en bits avec la voie directe n'est pas acquise** — c'est le point à mesurer d'abord,
avant toute décision. Si elle tombe, il faudra choisir entre le gain et l'identité, et ce choix
touche les hachages de campagne.
