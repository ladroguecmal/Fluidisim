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

Session : S274 — en cours
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo et Python.
Entrée : consigne utilisateur du 2026-09-18 — vérifier que la précision recherchée sert le
résultat final avant tout raffinement ; critères rattachés à ce qu'ils protègent ; écart
traduit en hauteur, pente, déphasage ; extrapolation vérifiée (troisième amplitude, pas de
temps) ; usage réel et rendu ; puis blocage suivant de la feuille de route. master 43385bd.
Objectif : établir le niveau nécessaire de la houle progressive, l'atteindre ou le proposer,
puis ouvrir le blocage suivant.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — lecture d'usage sur les traces S273, sans calcul nouveau : ce que protège chaque
  critère ; écart en hauteur absolue, pente, déphasage (mode k en quadrature, harmonique 2k) ;
  signature séculaire de l'ordre trois contre la dispersion de Stokes ; critères de la
  campagne P3 écrits avant mesure (HOULE-USAGE-S274).
- [x] **P3** — campagne unique : a = 2 cm et dt = 0,5 ms admis par l'exemple ; trois amplitudes
  × trois mailles à 1 ms, contrôle 0,5 ms à la fine ; coefficient d'ordre deux jugé au budget
  d'ADR-120, sans le présenter comme la houle complète.
- [x] **P4** — verdict d'usage : garanties de fonctionnement, seuils justifiés par l'usage ou
  proposés à l'utilisateur, raccord δ→rendu manquant, besoins découverts dans liste et file.
- [>] **P5** — blocage suivant, choisi : coût du pas couplé mobile (A276, ADR-147 point 5).
  *Amendement 19:39* : lot découpé en P5–P8, rituel en P9. P5 = ADR-167 et critères écrits
  avant le code (COUT-MOBILE-S274).
- [ ] **P6** — multigrille mobile : niveaux grossiers recalculés par pas depuis les mailles
  mouillées (Dirichlet vers l'air, Neumann vers le solide), mémoire comptée à la configuration,
  chemin ordinaire inchangé au bit ; essais de symétrie et de positivité.
- [ ] **P7** — branchement dans `project` en mode mobile, témoin Jacobi conservé en essai ;
  essais d'accord avec le témoin, refus/expiration/allocation ; suite complète.
- [ ] **P8** — mesure : itérations et ms par pas, S253 128 colonnes et houle fine ; réceptions
  S253 rejouées ; techniques présentes/absentes/domaine (ADR-131).
- [ ] **P9** — rituel §6.

### Notes de reprise

ADR-120 : 2 % décidés par l'utilisateur, budget conjoint spatial + temporel + référence ;
seul un arbitrage explicite les change. Le rendu (`viewer/`) ne consomme aucun domaine δ.
Repères d'usage existants : hauteur d'image 3 mm (S201), horloge 20 ms (ADR-003, Δz ≈ Aω·Δt).
Traces S273 dans TEMP (fluidisim-s273-*.log), à réutiliser en P2.

P2 : écart fin = 5,8 µm rms (0,058 % de a), pente 1,9e-5, déphasage 1,35e-3 rad à 2 s ;
D = N(a)−N(a/2) croît sur sin θ à 0,85–0,87 × Stokes (ω₂ = 0,611(ak)²ω), indépendant de dx.
ADR-122 : une troncature d'ordre deux ne juge pas la phase. Besoin découvert : cohérence
de phase δ/B (B linéaire). Outil outils/usage_houle.py.

P3 : coefficient d'ordre deux 1,16 % (1 ms) / 0,88 % (0,5 ms) ; troisième amplitude non
qualifiante à la fine (1,94 %) : crête 2 cm > centre de maille 1,56 cm ; budget ADR-120
2,26–2,43 % → non reçu, aucun seuil relevé. Traces fluidisim-s274-<dx>-<a>-<dt>.log.

P4 : 2 % d'ADR-120 ≈ 3 mm d'image à Hs 4 m / ak 0,1 ; J1 demande ≈ 13 %. Aucun seuil changé.
Banc arrêté ; réception cambrée différée (déclencheur). A289 et liste 4.21 (120 points), 8.7.
