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

Session : S609 — **en cours**. En autonomie (ADR-247) : **4.11 — le régime substitutif quand δ n'est plus petit, restauré depuis graine
(I-17)** (absent). ADR-001 §3.3 : la bascule à `max|δ| > 0,35·Hs_local` (ou par nature) ; le domaine devient **propriétaire du champ
total** dans son emprise, et B+W ne l'alimentent plus que par ses frontières (générateur en entrée, absorbeur en sortie). La restauration
depuis une graine (ADR-022 §3) est la session suivante.

**Ce que la session fait.** Un module `substitutif.rs` : `Mode`, `mode_requis(max|δ|, Hs, par_nature)` ; `Domaine1D` — l'eau peu profonde
linéaire du champ **total** sur grille décalée, schéma avant-arrière, et aux deux bords **Flather contre B** (`u = u_B ± √(g/h)·(η − η_B)`, B centré comme le schéma : `u_B` à la face et au demi-pas, `η_B`
au centre de la maille voisine) :
B y entre, ce qui sort du domaine en sort. Ne fait pas : un solveur substitutif non linéaire (le rouleau, la cavité — l'APIC 3D en serait
un), W aux frontières, le 2D/3D, la graine.

**Références, calculées avant** (ce script, par une implémentation numpy indépendante). Profondeur 2 m (`c` = 4.429447 m/s), 200 m en 400
mailles, pas de 0,05 s ; B : une onde longue progressive de 0,1 m et 40 m. **B seul**, le domaine initialisé sur B : après 60 s, l'écart
au champ de B **6.285449e-04 m** (la dispersion du schéma). **Une bosse** de 5 cm (gaussienne de 5 m au milieu) ajoutée au champ total : à
10 s, deux moitiés de **0.025003 m** ; à 60 s, une fois sorties, il en reste **3.256827e-04 m** — 1.303 % d'une moitié.

**Quantum** : f64. **Critères, écrits avant.** (1) l'écart à B seul à 60 s, égal à la référence à 10⁻⁹ m près, et sous 1 mm ; (2) la moitié
à 10 s et le reste à 60 s égaux aux références à 10⁻⁹ m ; le reste sous 2 % d'une moitié (les frontières laissent sortir ; la réflexion de Flather discret, 1,3 %, mesurée au plan
— une première version, B pris au pas entier et au bord, laissait 4,3 mm d'écart à B seul) ; (3) la bascule :
perturbatif à `max|δ|` = 0,35·Hs, substitutif au-delà ou par nature ; (4) refus : profondeur, `dx`, pas non positifs, nombre de Courant
`c·dt/dx` ≥ 1.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `substitutif.rs` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 4.11 ; rituel.

### Notes de reprise
