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

Session : S665 — **terminée**. En autonomie vers la v2 ; 2.7. En S664, le facteur de `Cote2D` manquait (6,6 % au centre, 15 % au bord) ;
trois causes nommées : la normalisation au départ, les parois de la marche, `K_r`.

**Ce que la session fait.** (a) **Les bords périodiques à phase tournée** dans la marche à grand angle : `A(n + W) = A(n)·e^(i·k_n·W)`,
`W = ny·dy`, `k_n = k₀·sin θ` — exacts pour une côte droite, sans parois ; le système tridiagonal devient cyclique (Sherman–Morrison).
`propager_periodique`, à côté de `propager_grand_angle` (S660, S662 inchangés). (b) **La normalisation au départ** : l'amplitude incidente
multipliée par le facteur WKB du bord du large (`transformer`, la référence de S362). `Cote2D` les emploie, sans marge.

**Contrôles du plan** (ADR-266, ADR-267)

- **témoin** : les parois et la normalisation supprimées, `K_r` reste seul. Ce que le facteur rendrait : **à 2 % de la côte 1D** si le
  modèle de S660 porte déjà la réfraction de l'amplitude ; **un écart qui suit `K_r`** (≈ 3 % à mi-profondeur, ≈ 6 % au rivage) s'il lui
  manque ; **un écart qui varie avec `n`** si les bords périodiques sont faux.
- **instrument** : les bords éprouvés d'abord — une onde plane oblique à 30° sur fond plat, en périodique : `|A|` = 1 partout à 10⁻⁶ (avec
  des parois, des franges) ; la même onde, la phase transverse `∂_n arg A` = `k₀·sin θ` à 10⁻⁶ ; le solveur cyclique contre un produit
  matrice-vecteur (le résidu à 10⁻¹²).
- **calcul** : aucun nombre nouveau ; la plage et la houle de S364 et S664.
- **ADR** : ADR-196 (D3, le bord du large), ADR-259 D1, ADR-266, ADR-267.
- **pièges** : la torsion de phase au raccord (le signe : `A_{−1} = A_{ny−1}·e^(−i·k_n·W)`) ; les coefficients `p` au raccord (périodiques
  aussi) ; le périodique ne vaut que pour une côte uniforme le long de ses bords.

**Critères, écrits avant.** (1) L'onde oblique sur fond plat : `|A|` = 1 et `∂_n arg A` = `k₀·sin θ` à 10⁻⁶ ; le résidu du solveur à
10⁻¹². (2) `Cote2D` contre la côte 1D (la plage de S364, 10 s, 1 m, 30°) : **le facteur à 2 %, la phase à 15°, le bord (n = ±100 m) à 1 % du
centre** — et le verdict du témoin. (3) Les essais de S659–S664 passent.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les bords périodiques, la normalisation ; (1)–(3).
- [x] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
- **P2 fini** — (1) 2·10⁻¹³ ; (2) le témoin : l'écart suivait `K_r` exactement — le remède, la levée par le flux oblique lue par l'opérateur
  (k linéaire dans le flux) : le facteur 0,56 %, la phase 3,65°, le bord 0 ; (3) passent ; Berkhoff non linéaire 0,101 ; 0,099 ; 0,094 ;
  0,125. Deux versions divergentes essayées et rejetées (le retard, le k non linéaire dans le flux).
