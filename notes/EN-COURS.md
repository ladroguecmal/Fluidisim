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

Session : S699 — **en cours**. En autonomie, sans arrêt. S698 : le raccord du large par particules, nourri par SGN, laisse −0,047 s au
retournement. Non traversé, il fait −0,017 s ; la traversée coûte −0,030 s. Trois causes y restent mêlées : le porteur, le profil, la
pose sans matrice affine.

**Ce que la session fait : le raccord entre deux copies du même solveur** (ADR-273 D1).

- **L'enregistrement.** Le tout-3D (le montage sans raccord de S697) enregistre au plan x = 5,0 m, à chaque pas :
  - la vitesse normale de chaque face (rangée × couche) ;
  - chaque particule qui franchit le plan vers la droite : l'instant, la position, la vitesse, la matrice affine.
- **Le rejeu.** Le montage raccordé à 5,0 m rejoue l'enregistrement par le bord à particules de S698 :
  - les vitesses des faces, interpolées dans le temps ;
  - les particules enregistrées, posées telles quelles.

  Ni SGN, ni profil.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276)

- **témoin** : le tout-3D, par la même fonction ; le même passage qui enregistre (2,637 s attendu, S697).
- **instrument** : le premier retournement. Ce que rendrait chaque hypothèse :
  - si le bord est transparent quand on le nourrit exactement, le témoin à 0,02 s près. L'écart de S698 revient alors à l'alimentation
    par SGN (le porteur, le profil, la pose) ;
  - si le bord lui-même fausse l'onde (la vitesse imposée à la face, le retrait à gauche), un écart proche de celui de S698.
- **calcul** : aucun nombre neuf. Le coût : ≈ 13,5 min + ≈ 7 min.
- **ADR** : ADR-273 D1, ADR-276 D2.
- **pièges** :
  - les indices des particules bougent quand le rivage en retire une. Une paire avant/après qui saute de plus de 0,1 m est écartée,
    et les particules ajoutées au rivage sont ignorées ;
  - le décalage d'instant entre les deux passages : une particule est posée au milieu du pas qui suit son passage ;
  - la masse comptée par quantum posé.

**Critères, écrits avant.**

1. Le rejeu à moins de **0,02 s** et **0,15 m** du témoin.
2. La masse à 10⁻¹², la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'enregistrement, le rejeu ; l'essai ; (1)–(2).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
