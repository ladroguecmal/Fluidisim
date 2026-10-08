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

Session : S712 — **terminée**. En autonomie, sans arrêt ; session longue. S709–S710 : le déferlement du tout-3D bouge de 0,2 à 0,3 s selon
que la 3D garde ou non son volume. ADR-280 D2 : **une référence extérieure tranche.**

**La référence.** Les mesures de Synolakis (Caltech), publiées par la NOAA : l'onde solitaire **H/d = 0,3 qui déferle** sur une pente de
1:19,85, aux instants `t·√(g/d)` = 15, 20, 25, 30. Elles sont téléchargées avec l'accord de l'utilisateur, dans
[references/synolakis](../references/synolakis/LISEZMOI.md). L'article de Grilli (1997) est payant, et sa prépublication est refusée.

**Le montage, construit dans l'essai par une seule fonction (ADR-276 D1).**
- APIC 3D à 2,5 cm, d = 0,5 m (20 mailles par profondeur, comme le juge), deux rangées (une onde plane : la largeur ne change rien) ;
- un fond en escalier, pente de 1:19,85, pied à `X₀ = 19,85 d` du rivage au repos ;
- l'onde `OndeSolitaire` (le profil de Synolakis), centrée en `X₁ = X₀ + arccosh(√20)/γ` ;
- un mur au large à `X₁ + 8 d`, la plage sèche jusqu'à `x = −8 d` ;
- le temps de 0 à `30·√(d/g)` = 6,77 s.

**Les essais.**

| essai | ce qui change seul | la lecture |
|---|---|---|
| **E1** | sans projection de densité (le juge de S690–S703) | aux quatre instants, l'élévation par la surface (φ), lissée sur 10 cm, à chaque point mesuré : l'écart quadratique moyen (`η/d`), et la crête mesurée contre la calculée |
| **E2** | la projection faible, κ = 0,05 (S710) | la même |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-279, ADR-280)

- **témoin** : la mesure elle-même. E1 et E2 ne diffèrent que par la projection.
- **instrument** : l'élévation par la surface reconstruite. ADR-280 D1 : c'est la grandeur que voit le solveur, étalonnée en S708 E1.
  Son plancher : la dispersion des mesures elles-mêmes, ≈ 0,01 à 0,02 en `η/d` entre points voisins, à t = 25. Ce que rendrait chaque
  hypothèse :
  - la version qui garde son volume est la plus juste : E2 a l'écart le plus faible à t = 20 et 25 (le déferlement) ;
  - la perte de volume aide par hasard : E1 l'emporte ;
  - un écart entre E1 et E2 sous la dispersion des mesures ne tranche pas.
- **calcul** :
  - `γ = √(3·0,3/4)` = 0,474 ; `L = arccosh(4,472)/γ` = 4,59 ; `X₁` = 24,44 d ;
  - ≈ 145 000 particules à deux rangées, 6,77 s ;
  - le coût, mesuré au premier passage ; on attend ≈ 15 min par essai.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-280 D2 : la référence extérieure ;
  - ADR-279 D1 : aucune tolérance nouvelle avant cette mesure ;
  - ADR-276 D1 : le montage, une seule fonction.
- **pièges** :
  - le sens des x : la mesure compte depuis le rivage, positive vers le large ; le domaine, depuis le mur du large ;
  - η sur la plage sèche n'est pas comparé (aucune mesure là où il n'y a pas d'eau) ;
  - les instants tombent exactement (pas bornés).

**Critères de la session.** E1 et E2 mesurés contre le laboratoire, et le verdict selon l'instrument. S'il tranche, une proposition sur le
défaut de la projection ; sinon, la dispersion nommée.

### Plan

- [x] **P1** — jeton ; plan ; la référence téléchargée.
- [x] **P2** — le montage ; E1.
- [x] **P3** — E2.
- [x] **P4** — preuve ; rituel.

### Notes de reprise
- **E1, premier passage arrêté** : à t ≈ 26,6, la remontée sur la plage sèche a fait tomber le pas à 0,34 ms, puis le calcul s'est figé. Les photos n'étaient comparées qu'à la fin : rien n'a été lu. Désormais, chaque photo est comparée dès qu'elle est prise, et le calcul s'arrête à t = 25 (le déferlement est vers t = 20). t = 30, la remontée, relève de la lame mince (S690).
- **P2–P3 finis** — E1 et E2 se valent contre le laboratoire (écarts 0,048/0,051, 0,066/0,070, 0,038/0,036 d). Toutes deux font l'onde trop haute à t = 15 (0,43–0,48 d contre 0,31). Le graphique est dans captures/s712_synolakis.png. Suite : S713, la même plage à 1,25 cm.
