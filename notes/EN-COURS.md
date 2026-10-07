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

Session : S612 — **terminée**. En autonomie (ADR-247) : **4.20 — le changement de solveur pendant une simulation** (ADR-007 ; absent,
conçu). ADR-007 §3 : pas de transfert d'état entre solveurs — `transduction δ → W → destruction → création à δ = 0 → nouveau solveur` ;
l'énergie est partie dans W, B + W est inchangé, et des solveurs voisins ne communiquent que par W.

**Ce que la session fait.** Un module `changement_solveur.rs` : `TrainW1D` — deux trains d'ondes longues analytiques qui voyagent à
`±c` (d'Alembert), `eta(x, t)`, `u(x, t)`, `energie` ; `transduire(domaine)` — les invariants de Riemann aux centres, `R = (η + √(h/g)·ū)/2`,
`L = (η − √(h/g)·ū)/2` (`ū` la moyenne des faces), le domaine ensuite détruit ; `energie_delta(domaine)`. Le nouveau solveur : un
`Domaine1D` (S609) d'une autre maille, né sur le champ de W et alimenté par W à ses bords. Ne fait pas : la transduction 2D/3D (celle de
S312–S316 existe sur la référence, en exemple), un solveur d'une autre famille, W non linéaire, la décision de changer (l'ordonnanceur).

**Références, calculées avant** (`s612_ref.py`, numpy indépendant). Le domaine de S609 en perturbatif (l'extérieur nul), une bosse de 5 cm
(5 m) lâchée au repos au milieu ; la bascule à **5 s**. Continuité à la bascule : **3.5e-18 m** (par construction, `R + L = η` :
assemblage, ADR-248). Énergie : δ **76.881476004 J/m**, W **76.785520448 J/m** — **-0.1248 %** (`u` au demi-pas). À **15 s**,
contre la solution exacte (deux demi-bosses à `±c`) : le solveur continué **3.150019308e-04 m**, W **2.382494298e-04 m** — la bascule n'ajoute
rien. **Le nouveau solveur** (0,25 m, 0,025 s, sur [150 ; 250] m, né sur W à 5 s, alimenté par W seul) : à 15 s, le train de droite y est
(crête 0.024930 m), écart à W **3.429847911e-05 m**.

**Quantum** : f64. **Critères, écrits avant.** (1) la continuité sous 10⁻¹⁵ m (assemblage) ; (2) l'énergie de W à -0.1248 % de celle
de δ, à 10⁻⁹ relatif de la référence, et sous 0,5 % ; (3) à 15 s, les deux écarts à l'exact égaux aux références à 10⁻⁹ m, celui de W au plus
celui du solveur continué ; (4) le nouveau solveur : l'écart à W égal à la référence à 10⁻⁹ m et sous 1 % de la demi-bosse (2,5·10⁻⁴ m) ;
(5) refus : un train évalué hors de son domaine de définition rend 0 (comme l'interpolation de la référence).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `changement_solveur.rs` et ses essais ; (1)–(5).
- [x] **P3** — preuve ; liste 4.20 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(5) tenus ; accesseurs `u`, `maille`, `profondeur`, `gravite` ajoutés à `Domaine1D`. Suite : 801 essais listés.
- **P3** — preuve CHANGEMENT-SOLVEUR-S612 ; liste 4.20 (absent → partiel) et décompte ; index ; journal.
