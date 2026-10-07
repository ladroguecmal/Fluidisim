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

Session : S641 — **terminée**. En autonomie (ADR-247). **La trente-deuxième revue de méthode** (ADR-222 D4 : S636–S640), **le lot**
(feuille de route S638–S640), et **l'audit des intentions initiales** demandé par l'utilisateur (2026-10-07,
[AUDIT-INTENTIONS-INITIALES-S640](../docs/registres/AUDIT-INTENTIONS-INITIALES-S640.md)) versé là où une session peut le verser seule
(ADR-218 D2) : les intentions fondatrices absentes deviennent des points de la liste ; les dérives non décidées sont inscrites à leurs
points ; ce qui relève de l'utilisateur lui est soumis, rien n'est tranché à sa place.

**Critères, écrits avant.** (1) ADR-259 relit chaque session S636–S640 et décide ; (2) la ligne S638–S640 de la feuille de route ; (3) les
intentions 1.1–1.11 de l'audit deviennent onze points (*absents*, « ajouté en S641 »), le décompte et les dépendances suivent,
`etat_projet.py --check` à zéro ; (4) 4.11 dit que son seuil n'est pas une règle reçue (ADR-112 D1) ; 13.2 ne dit plus C11 non exécuté ;
7.8 porte le détail d'ADR-016 ; les mineurs de l'audit (1.15) vont aux « manquent » de leurs points ; (5) les questions de l'utilisateur
au registre des questions ouvertes.

### Plan

- [x] **P1** — jeton ; plan ; le lot.
- [x] **P2** — ADR-259 ; la liste, ses dépendances ; les questions.
- [x] **P3** — rituel.

### Notes de reprise
- **P2 fini** — ADR-259 (D1 : un témoin qui supprime une cause avant de nommer un remède ; D2 : une valeur tirée d'un ADR se cite avec ceux
  qui le nomment ; D3 : le bilan global relit la liste contre les documents fondateurs) ; onze points ajoutés (121 → 132, absents 3 → 14),
  leurs dépendances ; 4.11, 13.2, 7.8 et quatre « manquent » corrigés ; les questions de l'audit dans la boussole.
