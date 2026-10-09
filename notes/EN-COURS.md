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

Session : S752 — **terminée**. En autonomie ; session longue. DEUX-REMEDES-S750 : R1 (le déplacement de la projection avec sa vitesse, reprise
de la grille) tient le repos, la remontée (+7,9 %) et la largeur de l'onde ; mais le creux est manqué de 0,8 mm, et l'onde s'atténue de 9 %
(91,3 mm contre 100,2 mm avec `Complete`). **La question** : d'où vient l'atténuation, et une mise à jour de la vitesse sans la grille la
supprime-t-elle ?

**Le bilan propre de la projection** (ADR-290 D1) : à chaque pas, l'énergie cinétique et l'énergie potentielle de toutes les particules,
juste avant et juste après `density_project` ; leur somme cumulée est rendue par le cœur (`density_projection_budget`). C'est une lecture,
sans effet.

**L'hypothèse nommée** : R1 remplace la vitesse de chaque particule déplacée par celle de la grille à sa nouvelle place. Ce passage par la
grille lisse la vitesse, et retire de l'énergie à l'onde. Or une particule d'APIC porte le gradient de vitesse autour d'elle (sa matrice
affine `C`). **R1′** (`set_density_shift_affine`) : `v ← v + C·Δx`, la mise à jour exacte au premier ordre, sans grille, `C` gardée.

**Signature prédite** :
- R1 retire de l'énergie cinétique à chaque pas, R1′ presque rien ;
- R1′ garde la remontée comme R1, car le déplacement porte sa vitesse ;
- R1′ garde l'onde comme `Complete`.

**Les essais, et leurs critères écrits avant** (ADR-290 D3 : le repos d'abord) :
1. **R1′, le repos** sur l'escalier, 1:30 et 1:12 : 1 cm/s ; 3 mm.
2. **Le bilan sur le canal à 2,5 cm**, `Complete`, R1, R1′ (4,25 s) : l'énergie cumulée retirée par la projection, rapportée ; et pour R1′,
   **l'onde** (la largeur 80 % ; le creux 10 mm ; **la crête finale à 5 % de celle de 0,25 s**).
3. **R1′, la remontée de S645** : à 10 % de la loi.

**Les quanta** (ADR-288 D2) : le creux se lit à environ 1 mm près (la surface lissée sur 10 cm) ; au départ, l'onde montre déjà un creux de
5 mm, l'ajustement de l'onde de départ (S739 : −4,8 mm à 0,25 s). Le critère de 10 mm est au-dessus des deux.

**Contrôles du plan** (ADR-276, ADR-287, ADR-288, ADR-290)

- **témoin** :
  - `Complete` et R1 (S750), sur les mêmes essais ;
  - l'onde exacte ;
  - la loi de Synolakis ;
  - le repos exact.
- **instrument** : le bilan propre de la projection (nouveau, éprouvé d'abord : nul au repos, à 10⁻¹⁰ J) ; ceux de S743, S740 et S645.
- **calcul** : le repos (2 min), le canal trois fois (≈ 30 min), la remontée (7 min).
- **ADR** : ADR-276 D2 (R1′ diffère de R1 par la mise à jour seule) ; ADR-290 D1, D3.
- **pièges** :
  - `C·Δx` suppose un déplacement petit devant la maille (borné au quart de maille par la projection) ;
  - l'énergie potentielle dépend de `z` ; seul le changement par la projection compte.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le bilan, R1′ ; (1), (2).
- [x] **P3** — (3).
- [x] **P4** — preuve ; le lot ; fermeture.

### Notes de reprise
- **Fini** :
  - R1′, le repos tenu ;
  - le bilan : `Complete` 0 / +2,11 J ; R1 +0,12 / +3,02 J ; R1′ +2,85 / +3,06 J ;
  - l'onde : R1′ à 80 %, 9,3 mm, la crête +14,7 % ;
  - la remontée : R1′ −11,1 %.

  **L'hypothèse est réfutée.** R1 est retenue : ADR-291.
