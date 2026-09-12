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

Session : S190 — terminée
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : appliquer l'arbitrage explicite de l'utilisateur du 2026-09-12 :
« 2 % d'erreur acceptable, débloque B4 avec ça ». Définir la grandeur et le périmètre,
recevoir les configurations mesurables au seuil, propager la décision et garder visibles
les volets restants. La projection S189-1 est réordonnée derrière cet arbitrage.

### Plan

- [x] **P1** — lire la reprise, vérifier copies/branches/historique/état, prendre le jeton ; plan seul.
- [x] **P2** — confronter B4, SPEC-004, ADR-119 et mesures ; acter les 2 % par ADR,
  définir avant le rejeu la norme, la référence et la décision de réception.
- [x] **P3** — appliquer le critère aux campagnes existantes, produire une réception
  reproductible avec témoin admis et refusé ; fixer un profil recevable si les preuves suffisent.
- [x] **P4** — propager vers B4/SPEC-004/A50 et les points ouverts ; expliciter le reste
  du banc et tenir une liste plurielle des chantiers signalés par l'utilisateur.
- [x] **P5** — rituel de fin REPRISE §6, vérifications, journal, index, décomptes,
  jeton libre et copies existantes synchronisées sans suppression non justifiée.

### Notes de reprise

Amorce : master propre à ac1ffb8, trois copies au même commit ; branche archivée conservée.
Cargo disponible. Aucun fichier modifié à l'entrée. Aucun agent parallèle lancé.
Dernier bilan actif : BILAN-B4-S176, à actualiser avec la réception au seuil de 2 %.
P3 : 126 couples jugés, 12 reçus/114 refusés ; profil 14×14×8, ext c8, budget
1,800653 %, composé+réserve 1,161371 %. Empreinte 0x4b479c21a520cd7b,
deux release et une debug identiques. Workspace debug 331/cinq ignorés reçu.
P4 : propagation préparée pendant les vérifications P3 ; à compléter et committer séparément.

P4 : seuil propagé à SPEC-004 §6.2/point ouvert, PLAN-BENCHMARK B4, état actif
du bilan S176, suivi ADR-119 et A50. A211 reçoit une file plurielle, portée par
REPRISE §6.7. Aucun nouvel angle ni leçon : défaut déjà nommé A211/L228.

P5 : journal, index, README et reprise mis à jour ; liens locaux vérifiés,
diff sans erreur. Décomptes 120 ADR/233 angles/269 leçons/18 invariants/6 SPEC/23 cas.
Aucun nouvel angle/leçon, A211 déjà porteur du défaut signalé. Suite S190-1 reprend
S189-1 avec seuil fixé ; file plurielle portée au rituel §6.7. Jeton libre.
Les trois copies ont été revérifiées propres, au commit initial ac1ffb8 ; avance rapide
sur le commit de clôture immédiatement après ce commit. Aucun répertoire créé ou supprimé.