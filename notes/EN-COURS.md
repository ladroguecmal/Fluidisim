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

Session : S202 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S201-1, profil de budget image explicite et coût par bloc mesuré,
conformément à ADR-124. Pas de réparation générale du solveur δ.

### Plan

- [x] **P1** — état réel, quatre copiesf0fea77 propres, jeton/plan seuls ; préférence
 60Hz/2ms ou30Hz/4ms demandée, mesure indépendante de la réponse.
- [x] **P2** — horloge monotone injectée et coût du dernier pas exposé dans Caps ;
 unité déclarée (domaine du candidat = bloc de banc), inconnu avant mesure ; tests.
- [>] **P3** — coût local sur16/32/64 et plafonds1/64/512, pas1/60, trois chauffes
 et11 mesures par configuration ; médiane/max, résidu/dégradation, refus explicites.
 Comparer au profil retenu, pas transposer le coût source S183 en coût solveur.
- [ ] **P4** — acter budget de travail avec origine, préciser absence de budget GPU
 mesuré et d'ordonnanceur ; résultats, suite effets bornés puis rituel complet.

### Notes de reprise

Reprise entière/invariants déjà lus dans cette conversation. Seuil numérique2 % acquis.
Rendu CPU S201≈12s pour640×360, référence hors ligne, sans budget temps réel reçu.
ADR-007 demande cost_per_block_ms mesuré ; Volume ne gère aucun add/remove_blocks.
On doit nommer le domaine/charge de la mesure, jamais diviser par un nombre fictif
ni présenter un coût de projection seule comme coût d'un pas complet.

Utilisateur :60 images/s, eau2ms/image, réponse reçue pendant P2.
P2 : MonotonicClock injectée ; step_measured expose coût du domaine entier via Caps,
None si inconnu/horloge invalide/refus ; pas de seuil appliqué au solveur.
Quatre tests intégration passent, coût factice1,5ms vérifié, zéro allocation
et sorties identiques au pas non chronométré. Pas de garantie temporelle inventée.

P3 :9 configurations, médianes/max reçus,32x16≈0,59ms vs64x32≈4,79ms
convergés ; début compilation refusé (trait Allocator absent), import corrigé.
Workspace343/cinq ignorés ; quatre tests intégration debug/release. Mesures
archivées, profil user60Hz/2ms appliqué aux comparaisons sans admission physique.
