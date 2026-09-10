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

Session : S164 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S163-1/A219, recevoir la clôture temporelle du résidu sur un fond analytique prescrit.

### Plan

- [x] **P1** — état réel, jeton et plan committé seul.
- [x] **P2** — dériver les sources aux étages RK2 et déclarer le protocole avant mesure ;
      synchroniser les copies propres au jeton occupé.
- [x] **P3** — implémenter le fond prescrit dans le véhicule S163, comparer incréments discrets,
      dérivées continues et source temporelle omise ; raffinement et témoins nuls.
- [x] **P4** — réception, limites et suivi A219/A50, tests S163 conservés ; décision si nécessaire.
- [ ] **P5** — rituel de fin, journal/index/décomptes/suite, jeton rendu et copies synchronisées.

### Notes de reprise

Départ master 6a295cb propre ; quatre copies au même commit, aucune branche vivante avancée.
REPRISE et corpus lus dans cette conversation ; S163 achevée, aucune étape à reconstituer.
112 ADR,219 angles,245 leçons,18 invariants,6 SPEC,23 cas ; 299 tests/cinq ignorés,
plus exemples S162 et S163. Aucun worktree créé. BILAN-S145 porté par la poursuite B4.

Instrument : fond Q réévalué aux temps RK, résidu seul intégré. Ne pas reconstruire le résidu
par soustraction d'une référence avancée ; seules les différences du fond prescrit sont permises.
Une dérivée continue peut converger sans reproduire exactement la référence au pas donné.
P2 : FOND-PRESCRIT-S164 déclare incréments RK2, dérivées continues et omission.
Fond onde debout linéaire, même état total initial gaussien, seuil S163 conservé.

P3 : step_prescribed construit dans le support S163 ; trois sources temporelles reçues.
72 montages N240 en release : incréments <=1,60e-13 hauteur ; dérivée continue converge
à l'ordre deux, omission ne converge pas. 4 tests propres S164 +3 host et 5 tests propres
S163 +3 host reçus. Deux avertissements d'import partagé corrigés par annotation locale.

P4 : 78 exécutions release reçues (72 N240 +6 raffinements), 7 tests exemple S164 et
8 S163 reçus. A219 traitée dans le véhicule ; A50 partielle, A220/S164-1 suit la frontière
locale. Aucun ADR nouveau. Suite workspace inchangée, reçue S163 et non relancée.