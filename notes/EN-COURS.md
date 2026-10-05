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

Session : S486 — **terminée**. En autonomie, **la première revue de méthode** ([ADR-222](../docs/adr/ADR-222-la-methode-se-revise-elle-meme.md)
D4, toutes les cinq sessions) **et le lot des registres** (dû en S486, ADR-213 D3).

**Ce que la session fait.** Relire au journal les frictions de S481 à S485 — ce qui a coûté du temps ou fait refaire — et corriger
METHODE (ses protections), la boussole (ses pièges), les outils, par un ADR. Puis le lot : la feuille de route, les angles morts nouveaux
(sévérité 2 ou plus, ou bloquants), l'index.

**Critères, écrits avant.** (1) chaque friction relue reçoit une suite écrite — une protection, un piège, un contrôle outillé, ou la raison
de ne rien changer ; (2) au moins un contrôle outillé nouveau, qui aurait vu une des erreurs de S481–S485 ; (3) le lot fait, `--check` à 0.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la revue : ADR-223 ; METHODE, boussole ; le contrôle nouveau.
- [x] **P3** — le lot des registres ; preuve ; rituel.

### Notes de reprise
- **P2** — ADR-223 : neuf frictions relues, chacune sa suite (trois faites avant, une sans changement, trois protections, deux pièges,
  un essai) ; METHODE (21 protections), LECONS L374–L376, boussole (pièges, la ligne « revue »). **Contrôle nouveau** :
  `air_pocket_centroid_is_the_bubble_centre_s486` (il aurait vu le centre faux de S479 : 0,16 m au lieu de 0,20). Le script qui
  écrivait ces lignes est tombé dans le piège qu'il consignait (un `U` échappé dans un heredoc) : refait depuis un fichier.
- **P3** — le lot : feuille de route (S484–S486) ; A325 (les parois ne sont pas des symétries — B10 en quart concerné) et A326 (la
  carte bornée à 65 535 groupes par passe) ; index (ADR-223, preuves de S484 et S485 déjà). Pas de preuve à part : la revue est ADR-223.
