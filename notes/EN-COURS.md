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

Session : S590 — **terminée**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S587–S589), puis **2.3 — les lacs** (absent ;
« niveau moyen, apports, courants faibles »). Première pièce : **le niveau moyen d'un lac par son bilan d'eau**, dans V — un lac est un
contenant (ADR-010) : ses apports (la pluie sur son bassin versant, ADR-204), son évaporation, son exutoire (un déversoir).

**Ce que la session fait.** Un essai de V, sans code neuf de loi : un lac de 1 000 × 1 000 m (forme volumique, 12 m de haut), plein
jusqu'à la crête de son exutoire (10 m) ; la pluie de 10 mm/h sur un bassin versant de 3,6 km² ; une évaporation de 5 mm/jour ; un
déversoir de 20 m (`C_d` = 0,62) vers dehors. Au pas de 10 s.

**Références, calculées avant** (ce script les écrit). Apports **10.0000 m³/s**, évaporation 0.05787 m³/s ; l'équilibre du déversoir
`Q = (2/3)·C_d·b·√(2g)·H^(3/2)` : **H = 0.419308 m** au-dessus de la crête ; la constante de temps près de l'équilibre
`τ = A/(dQ/dH)` = **28117 s** (7.8 h, ADR-240 D2) ; le temps pour arriver à 1 mm de l'équilibre (RK4 fin) :
**179210 s** (49.8 h). L'essai dure 72 h.

**Quantum** (ADR-236 D1) : 1 ml sur 10⁶ m² — 10⁻⁹ m ; le millimètre de surface est le quantum utile (5.6). **Critères, écrits avant.** (1) le
niveau final à 1 mm de `10 + H` ; (2) le bilan exact au millilitre : le volume final moins l'initial égale la pluie reçue moins le
déversé moins l'évaporé (les transferts de chaque pas) ; (3) le niveau à 49.8 h à 2 mm de l'équilibre (la référence RK4 au même pas).

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; liste 2.3 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — 0,419246 m pour 0,419304 ; 0,418305 m à 49,8 h ; le bilan exact. L'évaporation entière (58 nm/s, le plan 57,87) : référence
  recalculée dans l'essai. Suite 765.
- **P3** — preuve LAC-BILAN-S590 ; liste 2.3 (absent → partiel) et décompte ; index ; journal ; le lot.

