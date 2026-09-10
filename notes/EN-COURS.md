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

Session : S135 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S134-1 — ADR-086 s'est arrêtée au chemin pression et rien ne coordonne l'admission
entre couches. Établir **ce qui est déjà garanti** avant de supposer qu'il manque une
transaction, puis décider de ce qui manque vraiment.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — inventaire, et il commence par une vérification que S134 avait suggérée : les
      emprunts interdisent-ils déjà le cas problématique ? Puis établir ce que chaque couche
      sait faire, et quel état incohérent reste **observable**.
- [x] **P3** — ADR-091 : pas de coordinateur, l'admissibilité s'annonce. Aucune admission
      n'étant annulable, c'est le seul moyen d'éviter un état partiel.
- [x] **P4** — `would_admit` et `would_confirm`, sans duplication des contrôles.
- [x] **P5** — annonce contre verdict sur cinq cas ; scénario inter-couches joué.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 6f02880 = master, trois copies coïncidentes.

Ce que la lecture a déjà établi, et qui recadre la question :

- **Les deux couches ont chacune leur admission transactionnelle.** Côté pression,
  `bound_pressure::Controller` (ADR-086, 088, 089). Côté impacts, `prepared_water::LiveWater` —
  « paire journal/champs publiée ensemble ; une commande bloquée interdit une vue dite
  courante ». Il ne manque donc pas une transaction *par couche*.
- **Pendant une requête mixte, rien ne peut bouger.** `sample_world_batch` prend deux vues
  immuables ; `LiveWater::admit` et `Controller::admit` exigent `&mut`. Le compilateur interdit
  déjà d'admettre pendant qu'on échantillonne — c'est la même garantie structurelle qu'en S130.
- **La cause est déjà commune aux deux couches** : `wave_journal::Cause { entity, command,
  emission }` est utilisée par le journal W **et** par les métadonnées des sources de pression.
  Deux effets d'un même événement de jeu peuvent donc porter la même cause.

D'où la question réelle, qui n'est pas celle du titre : ce qui reste possible est qu'une cause
soit **partiellement admise** — sa pression acceptée, son impact refusé, ou l'inverse — et que
rien ne permette de le constater. Forcer les deux couches à réussir ensemble les coupleraient ;
permettre de voir qu'une cause est incomplète ne les couple pas.

Piège à éviter : construire une transaction inter-couches parce que le titre de la suite dit
« transaction ». ADR-086 a refusé de faire dépendre le contrôleur des impacts pour une raison
qui vaut toujours — le couplage coûterait plus que ce qu'il résout.

P2-P6 : ADR-091, ADMISSIBILITE-S135, journal, index, README, REPRISE, jeton rendu, ff-only.
266 tests/cinq ignorés, hachages inchangés. Aucun angle ni leçon nouveaux.

Ce que la session a corrigé en cours de route : j'ai d'abord cru que `reject` permettait
d'annuler une confirmation, donc qu'une transaction inter-couches était réalisable par l'hôte en
admettant d'abord la couche annulable. Lecture faite, `insert` rend `Conflict` sur
`(Confirmed, Rejected)` : **rien n'est annulable**, et c'est ce fait qui commande la décision.
Vérifier ce qu'une primitive fait vraiment, avant de bâtir un ordre d'opérations dessus.

Pour S136 sans relire : A200 (sévérité 1) est le plus gros manque ouvert du corpus. ADR-055 dit
« le générateur physique devra établir ce transfert » pour l'énergie ; ADR-083 a posé le contrat
`λ = α·b` avec α à calibrer. Ce qui manque est ce qui relie un événement de gameplay — un objet
de taille b entrant à vitesse v — aux deux nombres que l'événement porte. SPEC-001 §5 bis
(Wagner) donne l'étendue mouillée et la masse ajoutée ; ENVELOPPE-IMPACTS-S123 §4.3 donne le
plafond d'énergie que le candidat peut porter, soit 1e-6 à 1e-2 de l'énergie de référence. C'est
une session de conception, pas de code.
