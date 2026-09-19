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

Session : S294 — **les arbitrages du 2026-09-19 consignés, l'architecture de δ en 3D décidée**
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : réponses de l'utilisateur aux questions de BILAN-GLOBAL-S293 §6, 2026-09-19 14:47,
citées telles qu'écrites dans ADR-174 §1 (résumé : cible = ce poste ; les 2 ms peuvent changer
tant que l'objectif est atteint ; v1 acceptée ; pas de distant ; ordre et architecture acceptés
« si cela débloque » ; onde de S277 : ne sait pas).
Objectif : que ces réponses deviennent des décisions écrites, que la porte B soit ouverte sans
contredire la trajectoire, et que la 3D s'écrive une fois, au bon endroit. **Aucun code de
solveur dans cette session** : elle prépare la construction de S295, qui visera une capacité.

### Plan

- [x] **P1** — état réel, jeton, plan seuls.
- [x] **P2** — **ADR-174**, arbitrages de l'utilisateur : machine de référence ; budget de l'eau
  au service de l'objectif, profil de travail avec une part pour δ ; v1 = portes A à D ; pas de
  dépôt distant ; porte B avant la suite du coût en 2D ; onde de S277 sans verdict attendu.
  Notes datées sur ADR-125 ; feuille §3 bis, bandeau de la file, REVUE-VISUELLE ; A296 close.
- [x] **P3** — lire les contrats que la 3D touche (ADR-006, 007, 012, 143, 144, SPEC-004 δ,
  structure de `delta_projection`) ; notes de conception ici.
- [x] **P4** — **ADR-175**, architecture d'exécution de δ en 3D : pas de production résident sur
  GPU à travail borné, erreur publiée et état dégradé déclaré, cœur CPU référence de réception ;
  classe de fidélité par couche ; représentation 3D du régime perturbatif et voie non graphe (B3
  préliminaire). Note datée sur ADR-173 ; A295 décidée.
- [ ] **P5** — pilotage : déclencheur d'A276 remplacé (L343) ; REPRISE §6 et METHODE — lot pris
  dans les portes, maillons liés aux colonnes « reçu si » et aux points de la liste ; plafonds des
  documents d'état, contrôlés par `outils/etat_projet.py`.
- [ ] **P6** — file active ramenée aux plafonds (état, déclencheur, lien).
- [ ] **P7** — états des jalons de la feuille de route ramenés aux plafonds.
- [ ] **P8** — rituel §6.

### Notes de reprise

Réponses citées telles qu'écrites dans ADR-174 §1 ; elles font foi. Q3 : aucun dépôt distant, tout reste sur ce PC.

**P3 — ce que les contrats disent de la 3D (14:50–14:51).**
- **ADR-007** : `IFluidSolver` est défini **3D** dès S01 ; `step()` respecte son budget « quitte à
  sous-résoudre — moins d'itérations de pression » ; §4.1 prévoit δ sur GPU avec
  `latency_frames ≥ 1`, et la flottabilité ne lit jamais δ GPU de façon synchrone.
- **SPEC-004 §4** : `StepResult.degraded` avec `PressureItersCut` — la coupe d'itérations
  déclarée est **déjà** le contrat. **§8.4** : « la lecture synchrone n'existe pas » dans
  `IGpuBackend`. Or le chemin S289–S291 **attend** la carte et relit la pression à chaque pas
  (postes « attente » et « lecture ») : contradiction avec SPEC-004 §8.4, jamais relevée.
- **ADR-012 §7** : tick de simulation fixe (30 Hz proposé), δ sur un fil avec au plus une image
  de retard, rendu interpolé. §3 : cible de mesure = 99ᵉ centile, pas la moyenne.
- **ADR-006 §3** : domaine = ensemble épars de blocs 8³, `dx` ∈ {0,02 … 1,00 m}. La bande de
  l'afficheur est à `dx` = 2 m, **hors de ces niveaux** — à dire dans l'ADR, pas à corriger ici.
- **ADR-143/144** : écrits pour l'opérateur 2D à quatre faces ; en 3D, `γ₁₀` est à dériver, et
  ADR-144 note que franchir 32 768 mailles « est le point dur du passage à la 3D, qui y arrivera
  d'emblée » — un domaine 32³ y est déjà. Les portes à chaque pas buteront immédiatement.
- **Cœur 2D** (`delta_projection` et sous-modules, ≈ 3 300 lignes hors tests) : MAC, pression
  scindée `p_hydro + p_dyn`, `g_eff` par la surface, fonction hauteur et fluide fantôme, bandes
  de couplage B/W et éponge, multigrille, f32 avec coefficients construits en f64. Le GPU
  (`pressure_cg.wgsl`) est écrit pour quatre faces.
**Conséquence pour P4** : la décision n'invente pas une architecture, elle **revient à celle de
S01/S04** (δ 3D, GPU, lecture différée, dégradation déclarée) qu'ADR-173 avait contournée.
