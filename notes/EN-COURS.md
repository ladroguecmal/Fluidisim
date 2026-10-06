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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S552 — **terminée**. En autonomie, **6.6 — S548 et S549 ensemble** : une citerne latérale s'envahit par une brèche, la coque gîte
vers elle et s'enfonce — l'eau entre sous une pesanteur que la gîte incline, et pèse là où elle se tient.

**Ce que la session fait.** Un essai, sans code neuf : la barge de S548 (20 × 8 × 4 m, 246 t) ; une citerne latérale de V à tribord
(20 × 0,5 × 4 m, `y` de 3,5 à 4 m), forme volumique, une brèche de 0,1 m² à son fond ; la mer, vue du navire, une forme volumique de
400 × 400 × 20 m centrée sur la verticale de la brèche, replacée à chaque pas sur la surface du monde ; la pesanteur du navire
`Rᵀ·(0, 0, −g)` pour V ; l'eau de la citerne pèse en son centre mouillé (S549).

**Ordre de grandeur, calculé — la référence indépendante** (ADR-239 D1) : la flottabilité perdue en section, intégrée numériquement (la
section intacte `y` ∈ [−4 ; 3,5] sous la flottaison inclinée, la force et le moment résolus par bisection) : **gîte 8,088°, tirant au
centre 1,6355 m, 21,68 m³ dans la citerne** ; tribord immergé à 2,20 m, bâbord à 1,07 m (pont sec, bouchain noyé). Une citerne de 2 m
n'aurait aucun équilibre avant 40° (calculé) : elle n'est pas prise.

**Critères, écrits avant.** (1) La gîte finale à 3 % de 8,088°. (2) Le tirant au centre à 1 % de 1,6355 m ; l'eau à 2 % de 21,68 m³.
(3) La masse de V exacte. Quantum : 1 ml sur 10 m² de citerne (0,1 µm), rapport 10⁷.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; liste 6.6 ; rituel.

### Notes de reprise
- **P2 fini** — essai `s552` : gîte 8,101° (8,088), tirant 1,6192 m (1,6355 le long de l'axe ; `cos θ` : 1,6193 vertical), eau 21,689 m³
  (21,68) ; masse exacte. Suite 711.
- **P3** — preuve BRECHE-LATERALE-S552 ; liste 6.6 ; index ; journal.

