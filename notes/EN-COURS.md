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

Session : S759 — **en cours**. En autonomie ; session longue. BALLOTTEMENT-S757 : la 3D corrigée a la période juste, mais l'oscillation
grandit de 2,8 % par période. S758 a réfuté la correction particule par particule. **La question** : d'où vient l'énergie gagnée, et quelle
correction globale l'arrête sans perdre ce que la 3D corrigée tient ?

**Ce qui manque avant de corriger** : le bilan d'énergie du pas lui-même. Sans projection, le même ballottement s'amortit de 35 % par
période : le pas perd donc de l'énergie (sans doute l'eau qui se tasse et descend). Si la projection rend surtout cette perte, lui retirer
tout son gain laisserait l'amortissement du pas : la correction serait fausse.

**Les deux hypothèses nommées** (ADR-290 D1), départagées par P2 :
- **H1** : le pas conserve presque l'énergie ; le gain vient de la projection. Le remède : retirer à l'énergie cinétique, uniformément,
  tout le gain de la projection.
- **H2** : le pas perd de l'énergie (le tassement) ; la projection en rend plus qu'il n'en a perdu. Le remède : la projection ne rend que
  ce que le pas a perdu ; l'excédent est retiré à l'énergie cinétique, uniformément.

**L'instrument** (ADR-286 D2, une seconde lecture indépendante de φ) : l'énergie des particules, `Σ ½mv² + mgz`, à chaque pas, avant le pas,
avant la projection, après. Le bilan propre de la projection (S752) donne l'écart entre les deux dernières.

**Les critères, écrits avant** (ADR-290 D3, le repos d'abord ; ADR-293 D3, la bande de quantum) :
1. **le repos** sur l'escalier (1 cm/s ; 3 mm) ;
2. **le ballottement** : la période à 1 % ; l'amortissement entre **−0,5 % et +1 %** par période ;
3. **l'onde solitaire** sur le canal à 2,5 cm : la largeur 80 %, le creux 10 mm, la célérité à 1 % (ADR-293 D2).

Synolakis (45 min) vient ensuite, si les trois tiennent.

**Contrôles du plan** (ADR-276, ADR-287, ADR-290, ADR-293)

- **témoin** : la 3D corrigée sans correction (S757) ; la 3D sans projection (S757) ; la dispersion exacte.
- **instrument** : l'énergie des particules (nouvelle : sa seconde lecture est φ, S757) ; le bilan propre (S752).
- **calcul** : le bilan du ballottement 2 × 5 min ; le repos 2 min ; le ballottement 5 min ; le canal 10 min.
- **ADR** : ADR-276 D2 (une différence) ; ADR-286 D2 ; ADR-290 D1 ; ADR-293 D2, D3.
- **pièges** :
  - la correction met à l'échelle les vitesses **et** la matrice affine (APIC) : sinon le transfert vers la grille rend l'énergie ;
  - au repos, l'énergie cinétique est nulle : la correction ne peut rien retirer et ne doit rien diviser par zéro ;
  - l'énergie potentielle de l'eau prise à g = 9,81, la même que le bilan propre ;
  - les gouttes balistiques restent hors de la correction, comme hors de la projection.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le bilan d'énergie du ballottement : le pas et la projection, séparés, avec et sans projection ; H1 ou H2.
- [x] **P3** — la correction choisie par P2 ; (1), (2), (3).
- [x] **P3b** — le compte cumulé (la perte du pas reste due d'un pas à l'autre) ; (1), (2), (3).
- [ ] **P3c** — le compte cumulé contre Synolakis (ADR-293 D1 : la référence extérieure avant la décision).
- [ ] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** (579 s) — **H2** :
  - sans projection, le pas perd 4,32 J en 10 s (dix fois l'énergie du mode, 0,392 J) : le tassement ;
  - la 3D corrigée : le pas perd 1,20 J, la projection rend 1,50 J ; **l'excédent, +0,30 J**, est presque l'énergie du mode ;
  - la perte du pas et le gain de la projection sont réguliers (≈ 0,12 et 0,15 J/s).

  Le remède : la projection ne rend que ce que le pas a perdu (`BeyondStepLoss`).
- **P3 fini** (993 s), H2 pas à pas (`BeyondStepLoss`) :
  - (1) le repos **tenu** (6,8 mm/s ; 0,02 et 0,07 mm) ;
  - (2) le ballottement **NON TENU** : 2,0102 s (**+1,70 %**), l'amortissement **+2,74 %** par période ; l'énergie −0,20 J en 10 s ;
  - (3) le canal **tenu** : 93 %, 8,8 mm, −0,3 %.

  L'injection est arrêtée, mais le ballottement s'amortit. **L'hypothèse (P3b)** : un cliquet pas à pas. Quand la projection rend moins
  que la perte du pas, le reste est oublié ; quand elle rend plus, elle est rognée. **Le remède** : un compte cumulé, borné en bas par zéro
  (un pas qui gagne de l'énergie n'ouvre aucun droit). **La signature** : l'amortissement près de zéro, la période revenue vers +0,3 %.
- **P3b fini** (1 011 s), le compte cumulé (`CumulativeStepLoss`) — **les trois tenus** :
  - (1) le repos : 6,8 mm/s ; 0,02 et 0,07 mm ;
  - (2) le ballottement : 1,9678 s (**−0,44 %**), l'amortissement **0,16 %** par période ; l'énergie à ±0,005 J en 10 s (1 % du mode) ;
  - (3) le canal : 92 %, 9,0 mm, la célérité +0,3 %.

  Le cliquet était la cause. Synolakis ensuite (P3c).
