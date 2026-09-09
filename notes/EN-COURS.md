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

Session : S131 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S130-1 — sortir de la saturation. Après un `Full`, le contrôleur conserve sa
publication mais ne peut plus changer d'instant. Le chemin de sortie existe déjà ; il s'agit
de le parcourir en entier, de recevoir ce qu'il garantit, et de mesurer ce qu'il coûte.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [ ] **P2** — inventaire avant de décider (L209), et **chercher ce qui manque** : un
      élargissement qui réussit sans résoudre l'attente est-il possible, et qu'en sait
      l'appelant avant d'essayer ?
- [ ] **P3** — recevoir le cycle complet : élargissement, reprise, reconstruction, et
      identité en bits du champ d'après avec une préparation directe du journal élargi.
- [ ] **P4** — recevoir les refus du chemin : pool insuffisant, pool tout juste suffisant.
- [ ] **P5** — mesurer la fenêtre pendant laquelle l'hôte n'a plus de champ.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

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
