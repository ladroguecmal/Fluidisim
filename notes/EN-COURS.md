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

Session : S643 — **terminée**. Sur la demande de l'utilisateur ([ADR-261](../docs/adr/ADR-261-reponses-du-2026-10-07.md) D3 : *« toutes
les anciennes intentions doivent être jugées dépassées ou non, de meilleure solution […] l'objectif reste le même d'une performance et
réalisme insane »*). **La réévaluation des intentions fondatrices** : chaque section des deux documents sources
(`systeme_eau_architecture_globale`, §1–§20 ; `zones_ouvertes`, §2–§32) jugée — **gardée**, **dépassée**, ou **remplacée par une meilleure
solution** —, avec la trace actuelle et la conséquence.

**Critères, écrits avant.** (1) Le registre `REEVALUATION-INTENTIONS-S643` couvre chaque section des deux sources (aucune omise : compté) ;
(2) chaque verdict « dépassée » ou « meilleure solution » nomme ce qui remplace et où cela vit (point, ADR) ; (3) ce qui en découle est
versé : points ajoutés ou reformulés, `etat_projet --check` à zéro ; (4) ce qui change un critère de jugement du projet passe par un ADR.

### Plan

- [x] **P1** — jeton ; plan ; les deux sources relues en entier.
- [x] **P2** — le registre ; l'ADR ; la liste.
- [x] **P3** — rituel.

### Notes de reprise
- **P2 fini** — (1) le registre couvre les 20 + 32 sections (34 + 31 lignes, recomptées par script) : garder 28 + 23, meilleure solution
  6 + 4, dépassée 0 + 4 ; (2) chaque remplacement nommé ; (3) 2.11 ajouté (134 points), 1.5, 2.6, 2.7, 4.4, 9.1, 9.11 précisés ; (4)
  ADR-262 — indiscernable du réel, au budget ; le budget, un plafond ; l'activation sur l'erreur visible ; aucun plafond arbitraire.
