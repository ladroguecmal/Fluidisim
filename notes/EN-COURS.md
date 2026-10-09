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

Session : S719 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. S718 : de bout en bout, la remontée
dépasse le tout-3D de 3,3 cm, et le large (SGN) en est la cause. ADR-278 D3 : l'onde de départ, un profil de Boussinesq (KdV,
`γ = √(3a/4d³)`), n'est l'équilibre ni de SGN ni de la 3D. **La même onde pour les deux : l'onde solitaire de SGN** (le profil de Rayleigh,
`k = √(3a/(4d²(d+a)))`, plus large de 14 %), avec la même vitesse `c·η/(d+η)`, `c = √(g(d+a))`.

**Les essais.** Le montage de S717–S718 (la même fonction), l'onde de Rayleigh pour SGN, pour Saint-Venant et pour la 3D :

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | le tout-3D jusqu'à 5 s (`Large::AucunRayleigh`) : le nouveau témoin | rapporté (le retournement, l'air, la remontée) |
| **E2** | de bout en bout (`Large::BoutEnBoutRayleigh`) | contre E1 : le retournement à 0,1 s et 0,15 m (ADR-278 D2), l'air après lui ; **la remontée à 1,25 cm** et 0,1 s ; le volume rendu à 0,5 % ; le coût |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-278, ADR-281)

- **témoin** : E1, par la même fonction. Seule l'onde de départ change par rapport à S717–S718 (ADR-276 D2). La forme et la vitesse
  viennent toutes deux de Rayleigh.
- **instrument** : ceux de S718 (le retournement, l'air, la remontée sous la maille, recalculée à chaque pas). Ce que rendrait chaque
  hypothèse :
  - l'onde de départ était la cause : E2 à 1,25 cm de E1 ;
  - le porteur SGN lui-même : l'écart demeure, vers 3 cm.
- **calcul** :
  - `k_R/γ_KdV = √(d/(d+a)) = √(0,5/0,65) = 0,877` ;
  - le volume de l'onde, `2a/k`, passe de 0,316 à 0,361 m² par mètre de large ;
  - le coût : ≈ 20 + 5 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-278 D3 : la question ouverte, mise à l'épreuve ;
  - ADR-276 D1 : l'onde, une seule fonction pour les trois solveurs ;
  - ADR-277 D2 : deux modes nommés, ramenés aux modes de base dès l'entrée de la fonction.
- **pièges** : chaque construction de l'onde dans la fonction (Saint-Venant du large, SGN, la 3D) passe par la même structure.

**Critères de la session.** E2 tenu, ou son échec attribué.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — E1, E2.
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
