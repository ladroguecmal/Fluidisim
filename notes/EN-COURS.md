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

Session : S750 — **terminée**. En autonomie ; session longue. CIBLE-SURFACE-S749 : aucune des six variantes de la projection ne tient à la fois
le repos, l'onde solitaire et la remontée. Les variantes qui corrigent la surface freinent la lame (−11 à −12 %). **La question** : l'un de
deux remèdes rend-il la remontée à `Complete` (consciente du fond), qui tient déjà le repos et l'onde ?

**Les deux remèdes, chacun une différence avec `Complete` (ADR-276 D2)** :
- **R1, le déplacement avec sa vitesse** (`set_density_shift_resample`) : chaque particule déplacée par la projection reprend la vitesse et
  la matrice affine de la grille à sa nouvelle place (`grid_affine`). Dans APIC, la vitesse d'une particule est déjà celle de la grille à sa
  place (le G2P) : la déplacer sans la mettre à jour place une vitesse au mauvais endroit dans un champ cisaillé.
- **R2, la surface relâchée** (`set_density_surface_relaxation`) : aux mailles de surface, la correction est faite à κ = 0,1 par pas ;
  à l'intérieur, à 1.

**Les essais, et leurs critères écrits avant** :
- **la remontée de S645 d'abord**, pour R1 et R2 (6 min chacun) : à **10 %** de la loi ;
- pour un remède qui la tient, **le repos** (l'escalier : 1 cm/s ; 3 mm) et **l'onde solitaire** (le canal à 2,5 cm : la largeur 80 % ; le
  creux 10 % de `H`).

Si aucun ne la tient, la question reste ouverte, avec les nombres.

**Contrôles du plan** (ADR-276, ADR-287, ADR-288)

- **témoin** : `Complete` consciente (S744 : le repos, l'onde, −11 %) ; S645 sans projection (+0,5 %) ; la loi.
- **instrument** : ceux de S645, S743 et S740.
- **calcul** : 2 × 6 min, puis 15 min pour le remède retenu.
- **ADR** : ADR-276 D2 ; ADR-287 D1 ; ADR-288 D1 (le repos aussi pour le remède retenu).
- **pièges** :
  - R1 reprend la vitesse de la grille : c'est déjà la règle d'APIC au G2P, mais à un moment du pas où la grille a été projetée. Le repos
    dira s'il dérange ;
  - R2 ralentit la correction de surface : la dilatation de S748 peut revenir, plus lentement.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — R1 et R2 ; la remontée pour chacun.
- [x] **P3** — le repos et l'onde pour le remède qui tient.
- [x] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** (801 s) — la remontée de S645, les deux à 10 % :
  - **R1, le déplacement avec sa vitesse : 0,2475 m, +7,9 %** ; le freinage disparaît, le mécanisme est confirmé ;
  - **R2, la surface relâchée : 0,2071 m, −9,8 %** ;
  - les étiquettes : 0,175 et 0,200 m.

  Les deux tiennent ; le repos et l'onde pour les deux (P3), R1 d'abord.
- **P3 fini** (1 355 s) :
  - R1 : le repos tenu (6,8 mm/s ; 0,02–0,07 mm) ; le canal à 92 %, le creux 10,8 mm (manqué de 0,8 mm), la crête finale 91,3 mm ;
  - R2 : le repos tenu ; le canal à 84 %, le creux 13,4 mm ;
  - le niveau : R1 +2,57 → −0,41 mm ; R2 +2,51 → −1,94 mm.
