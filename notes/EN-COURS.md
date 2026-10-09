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

Session : S723 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-3-S722, **B1 : Saint-Venant
troué**.

**Ce que la session fait.** Dans `SaintVenant2D`, à l'ordre deux :
- **`regler_trou(i0, i1, j0, j1)`** : un rectangle de mailles gelées. Leurs faces avec l'eau active deviennent des parois (la pression de
  la maille active, aucun flux calculé), et la reconstruction des mailles voisines n'y lit rien (les pentes y sont nulles, comme aux
  bords du domaine) ;
- **`pas_avec_flux_trou(dt, flux)`** : sur chaque face du trou, la masse `F` entre dans la maille active (positive) ou en sort. Elle
  emporte `F·u` et `F·v` de la maille active, comme le flux imposé au bord droit (S687). Les faces sont rangées : la gauche, la droite (par
  `j`), puis le bas, le haut (par `i`) ;
- **`pas_avec_flux_bords4(dt, flux)`** : le même flux imposé sur les quatre bords du domaine (la gauche, la droite par `j` ; le bas, le
  haut par `i`), positif vers l'intérieur.

Sans trou ni flux, le code ne change pas (le banc au bit).

**L'essai, entre deux copies du même solveur (ADR-273 D1).** Une bosse d'eau (2 cm, rayon 0,3 m) sur un fond plat de 3 m × 3 m
(`dx` = 5 cm) s'étale en rond ; un trou de 0,75 m × 1,25 m sur son chemin. Le Saint-Venant troué et un second Saint-Venant, qui remplit le
trou, échangent à chaque pas le flux de masse de Rusanov calculé entre leurs mailles voisines. Le même flux, de signe opposé, va aux deux.

**Critères, écrits avant.**
1. La masse des deux ensemble, au bit (10⁻¹²).
2. Contre le Saint-Venant entier, après 1,5 s : l'écart maximal de `h` sous **5 %** de l'amplitude de la bosse (1 mm). Le flux d'interface
   est gelé sur le pas de Heun, et d'ordre un : une erreur d'ordre `dt` est attendue.
3. Le banc de non-régression au bit (le défaut inchangé).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-277, ADR-281)

- **témoin** : le Saint-Venant entier, le même pas, la même bosse.
- **instrument** : l'écart maximal de `h` et la masse, recalculés à la fin (ADR-281 D2 : ils peuvent échouer).
- **calcul** : la bosse de 2 cm sur 0,5 m d'eau, `c` = 2,2 m/s : en 1,5 s, l'onde parcourt 3,3 m et traverse le trou. Le pas : Courant
  0,4.
- **ADR**, et comment chacun est tenu (ADR-277 D1) : ADR-273 D1 (deux copies) ; ADR-282 (la fermeture par l'outil).
- **pièges** :
  - les quatre côtés ont des signes opposés ; on vérifie que la masse se tient pour chacun ;
  - les coins du trou n'ont pas de face diagonale (rien à faire) ;
  - le chemin d'ordre un n'est pas touché (le trou demande l'ordre deux).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le trou, les flux ; l'essai ; (1)–(3).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
