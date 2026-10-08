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

Session : S700 — **en cours**. En autonomie, sans arrêt. S699 : nourri exactement par la 3D, le bord par particules est transparent
(−0,011 s). L'écart de S698 (−0,047 s) vient donc de l'alimentation par SGN. Ici, ses trois causes, une à la fois (ADR-276 D2).

**Ce que la session fait.** Un enregistrement du tout-3D au plan x = 5,0 m (S699), et pendant ce passage, au même plan :
- le porteur SGN : `h`, `ū` ;
- la 3D : `h`, la plus haute particule de la tranche ± dx, plus dx/4 ;
- la 3D : `ū`, la moyenne des vitesses des faces mouillées.

Puis deux rejeux, chacun ne changeant qu'une chose par rapport à S699 :

| rejeu | ce qui change seul par rapport au rejeu exact (S699, −0,011 s) |
|---|---|
| R2 | les particules posées sans leur matrice affine (`C = 0`) |
| R3 | la pose par faces de S698 : des quanta posés dans la maille de la face, avec la vitesse de la 3D à cette face, `w = 0`, `C = 0` |

S698 (SGN, profil, pose par faces) diffère de R3 par une seule chose : les vitesses (le porteur et le profil). La comparaison au plan sépare
ces deux dernières causes :
- le porteur : l'écart de `h` et de `ū` entre SGN et la 3D, et celui de l'instant de leur crête ;
- le profil : ce qui reste.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276)

- **témoin** : le rejeu exact de S699 (−0,011 s) ; le tout-3D (2,637 s) par la même fonction.
- **instrument** : le premier retournement de chaque rejeu ; au plan, la crête de `h` et de `ū`. Ce que rendrait chaque hypothèse :
  - l'affine en cause : R2 loin de −0,011 s ;
  - la pose en cause : R3 loin de R2 ;
  - les vitesses en cause : R3 près de −0,011 s, S698 à −0,047 s, et la crête de SGN en avance ou plus haute que celle de la 3D.
- **calcul** : aucun nombre neuf ; ≈ 13 + 2 × 7 min.
- **ADR** : ADR-273 D1, ADR-276 D1, D2.
- **pièges** :
  - les drapeaux du montage. Le mode de rejeu est un champ de l'enregistrement. La zone de colonnes ne s'allume ni avec le rejeu ni avec
    les particules (S699) ;
  - le `h` de la 3D au plan est lu au quantum (dx/4) ;
  - SGN tourne dans le passage qui enregistre (`sgn = true`) sans y agir (sans raccord).

**Critères, écrits avant.**

1. R2 et R3 mesurés, et chaque écart attribué à sa seule cause.
2. La masse à 10⁻¹², la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — les deux rejeux, la comparaison au plan ; l'essai ; (1)–(2).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
