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

Session : S191 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S190-1 / S189-1, construire et recevoir une projection discrète compatible
avec le véhicule de banc, puis éprouver le profil 14×14×8 / extrapolation 80 ms
sous les 2 % actés. Ne pas confondre projecteur discret et solveur à surface libre.

### Plan

- [x] **P1** — reprise et état réel, jeton/plan seul ; master propre 6684094 et trois copies à jour.
- [x] **P2** — dériver D/G et leurs conditions de bord ; protocole de projection,
  critères indépendants, métriques B4 et portée avant simulation.
- [x] **P3a** — construire le projecteur de banc ; gradient, solénoïdal, référence
  algébrique indépendante, refus/non-convergence ; un support sans mutation du précédent.
- [ ] **P3b** — campagne projetée contre plein/raffiné, axes séparés, composition locale,
  profil reçu S190 et témoins ; qualifier pression, divergence et réserve de référence.
- [ ] **P4** — réception, correction datée des inférences invalidées, ADR si décision,
  angles/leçons et propagation B4/A50/file plurielle ; aucune tolérance redemandée.
- [ ] **P5** — rituel REPRISE §6, journal/index/README/décomptes, vérifications,
  jeton libre et avance rapide des copies propres sans suppression.

### Notes de reprise

REPRISE lue entièrement dans cette conversation ; les changements depuis sont ceux de
S190, relus avec dernier journal/index/file active. Invariants, ADR-001 et METHODE déjà
lus ; aucun changement depuis. B4-TOLERANCE-S190 porte le profil et la réserve mesurée.
Autres lignes de la file active conservées : aucun déplacement de priorité implicite.
P2 : PROJECTION-B4-S191 publié. D central/extension nulle et G=-D^T ; CG
sur DD^T, dimensions intérieures paires. La projection fixe est linéaire ;
la somme des erreurs des axes requiert leur additivité, correction de prémisse S189.

P3a : pressure_projection.rs, 3 tests debug/release reçus : gradient/curl/adjoint/
linéarité/idempotence/L2 ; matrice dense indépendante ; zéro/refus atomiques.
Aucun support historique touché. projected_b4 porte les tests ; campagne à écrire P3b.
