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

Session : S702 — **en cours**. En autonomie, sans arrêt. S700 : la pose par faces de S698, même nourrie des vitesses de la 3D, retarde
le retournement de 0,085 s. **Une pose qui ne dépend pas du porteur.**

**Ce que la session fait.**

- **La pose par la grille** (`feed_left_grid`) :
  - le porteur ne donne que deux choses : la vitesse normale de chaque face du bord et le volume qui la franchit ;
  - chaque quantum naît dans la tranche que le flux a balayée pendant le pas, `x ∈ [0, u·dt)`, à la sous-maille (y, z) la moins occupée
    de la face, décalée par une suite à faible discrépance ;
  - sa vitesse et sa matrice affine sont celles que la grille lui donne là (le G2P d'APIC), comme à toute particule. `w` et le gradient
    viennent ainsi de la 3D elle-même, non d'un profil.
- **Le montage passe à un mode nommé** (ADR-277 D2) : `Large::{Aucun, Colonnes, ProfilSgn, Rejeu(..)}`. Chaque mode dit ce qu'il allume ;
  une combinaison sans sens est refusée par une assertion.
- **R4** : le rejeu de l'enregistrement du tout-3D par la pose par la grille. Les données sont la vitesse des faces et `h` au plan, celles
  que SGN donnerait.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277)

- **témoin** : le tout-3D, et le rejeu exact de S699 (−0,011 s). Le passage qui enregistre doit redonner 2,637 349 s au bit : il vérifie que
  le passage au mode nommé n'a rien changé.
- **instrument** : le premier retournement de R4. Ce que rendrait chaque hypothèse :
  - si la pose par la grille est juste, R4 près de l'exact (−0,011 s) ;
  - si la naissance dans la tranche ou le G2P ne suffit pas, un écart du côté de R3 (+0,074 s) ou de R2 (−0,037 s).
- **calcul** : aucun nombre neuf ; ≈ 13 + 7 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1 : la pose est jugée ici contre la 3D (deux copies du même solveur). Elle ne sera nourrie par SGN qu'en S703 ;
  - ADR-276 D2 : R4 diffère de R3 par la pose seule, non par les données ;
  - ADR-277 D2 : le mode nommé, cette session.
- **pièges** :
  - le `h` de la 3D au plan est lu au quantum de 6 mm ; il fixe la part de la couche de surface ;
  - la grille au moment de la pose est celle de la fin du pas précédent ;
  - une boucle sur toutes les particules par face coûterait cher : une seule passe compte l'occupation.

**Critères, écrits avant.**

1. R4 à moins de **0,02 s** et **0,15 m** du tout-3D (2,637 s, 9,988 m).
2. Le passage qui enregistre au bit de S699 ; la masse à 10⁻¹², la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le mode nommé ; la pose par la grille ; R4 ; (1)–(2).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
