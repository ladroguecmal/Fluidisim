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

Session : S421 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**, le coût
([preuve](../docs/validation/APIC-CARTE-S416.md) §5, §12 : 28,7 ms sur B10 en bande étroite, dont 23,6 pour la fin du pas et ≈ 5,5 de
bascule, sur un fil).

**Méthode.** Mesurer d'abord, par sous-étage (horodatages), puis réduire **à sémantique exacte** : chaque réduction doit rendre B10 en
bande étroite, le raccord et la bande du ballottement **au bit près** — mêmes pincements, mêmes séries d'écart, mêmes gestes — que
S420. La principale idée : le fil de l'échange parcourt toutes les faces-mailles de frontière (≈ 45 000 sur B10) pour n'en traiter
qu'une poignée ; une **liste ordonnée des faces-mailles actives** (solde au-delà d'une particule), construite en parallèle dans
l'ordre de la référence, lui laisse le même travail utile, dans le même ordre — exact, puisqu'un solde ne change que par ses propres
gestes. De même pour le solde vertical, la réserve (en parallèle, en entiers) et les soldes de la bascule.

**Critères, écrits avant.** (1) Résultats inchangés au bit près sur B10 en bande étroite (pincement, série de `φ`, gestes, `n`,
volume), le raccord et la bande du ballottement (écarts de surface et gestes identiques à S419–S420). (2) Le coût publié par
sous-étage avant et après. (3) **δ ≤ 2 ms au 99ᵉ centile sur B10 en bande étroite** — visé ; s'il n'est pas atteint, ce qui reste
est nommé, chiffré, et la suite le porte. (4) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'instrument : horodatages par sous-étage (fin du pas : séparation, corps, absorption, échange ; bascule : décision,
  application, fond) ; mesure de B10 en bande étroite.
- [ ] **P3** — l'échange : listes ordonnées des faces-mailles et des colonnes actives ; la réserve en parallèle ; mesure ; critère 1.
- [ ] **P4** — la bascule : les boucles séquentielles réduites (soldes des faces changées, …) ; mesure ; critère 1.
- [ ] **P5** — la projection : ce que coûtent les itérations enregistrées vides, et sa réduction (dispatch indirect nul après
  convergence, ou plafond) ; mesure.
- [ ] **P6** — B10 en bande étroite, coût final ; non-régression ; suite ; preuve §13 ; registres.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — horodatages par sous-étage (32 requêtes ; fin du pas en trois passages : séparation et corps 6, absorption 7, échange 8 ; bascule : décision 9, application 10, fond 11). B10 en bande étroite, résultats **inchangés au caractère près** (pincement, série de `φ`, divergence au pas 35). **p99 (ms)** : transfert 0,20, surface 0,74, **projection 5,93**, extrapolation 0,03, retour 0,07, advection 0,004, séparation et corps 0,40, **absorption 2,14**, **échange 21,44** ; bascule 1,93 (décision 0,88, application 0,64, fond 0,42). Total pas 29,0 + bascule 1,9.
