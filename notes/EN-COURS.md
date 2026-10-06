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

Session : S500 — **en cours**. En autonomie, **6.1 — pousser au centre de la part immergée** : S499 a montré que tout le surcoût des
couches du proxy vient de la couche partielle, qui pousse en son milieu (`z_F` faux de `(1 − f)·f·e²/(2d)`).

**Ce que la session fait.** Dans `RigidBody::forces`, la poussée d'un point partiellement immergé s'applique au centre de sa part immergée
— son milieu abaissé de `(1 − f)·e/2` le long de l'axe du corps ; les forces (et donc la translation) ne changent pas, le moment seul.
ADR-227. B6 refait avec ce modèle.

**Ordre de grandeur, écrit avant.** Sur la coque de la porte D (16 × 8 × 4 points, `e` = 0,25 m, tirant 0,49 m) : l'erreur de `KB` qui
disparaît, au plus `e²/(8d)` ≈ 1,6 cm, contre `GM` de tangage ≈ 2,7 m — 0,6 % : les essais du corps (S331–S499) doivent tenir à leurs
tolérances ; le pilonnement droit ne change pas au bit (moment nul par symétrie).

**Critères, écrits avant.** (1) à l'équilibre droit, `z_F = KB − KG` à 10⁻⁹ pour toute grille ; (2) B6 refait : la prédiction
`BM·(1 − 1/n²) + z_F` à 10⁻³ ; les plus petits proxys sans compensation = avec, attendus 7 × 10 × 1 (navire), 7 × 7 × 1 (barque, caisse) ;
(3) toute la suite du cœur tient ; les valeurs publiées des essais du corps qui changent, relevées.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la poussée au centre de la part immergée ; B6 refait ; la suite.
- [ ] **P3** — ADR-227 ; preuve ; liste 6.1 ; rituel.

### Notes de reprise
