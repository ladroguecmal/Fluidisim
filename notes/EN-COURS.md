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

Session : S435 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3a**, A320
([preuve](../docs/validation/MER-S369.md) §6), les pistes restantes.

**Ce que la session trouve en entrant.** La forme de Bernoulli ne freine A320 que de 15 à 20 % (S434) ; la bisection réfute un
défaut de forme. S369 avait noté : taux ≈ `a²`, ≈ 4,5 fois Benjamin-Feir (`ω(ak)²/2`), **plus lent à maille fine** (0,052 → 0,033
de 25 à 12,5 cm, sous 5 cm), convectif (×e tous les ≈ 18 m). Une extrapolation linéaire en `dx` de ces deux points donne
≈ 0,014 s⁻¹ — l'ordre de Benjamin-Feir (0,012). **La question physique décide donc de la suite** : si A320 est, au fond, la
modulation de Benjamin-Feir de la houle portée par δ, plus un excès de discrétisation, le critère « < 0,01 s⁻¹ » de C7d-3a est mal
posé à 7,5 cm (Benjamin-Feir y vaut 0,027) et l'advection antisymétrique ne viserait que l'excès ; si elle est à l'échelle de la
maille, elle est numérique et l'advection antisymétrique est la piste.

**Ce que la session fait.** (1) `MER_SPECTRE=1` au banc `mer` : en fin de calcul, le spectre de l'élévation de δ hors des éponges
(transformée discrète, fenêtre de Hann) — l'énergie par bande de nombre d'onde, rapportée à celui de la houle `K`. (2) Le germe de
1 mm sous 7,5 cm de houle, masque 7, à **50, 37,5, 25 et 12,5 cm** (la dernière en arrière-plan, ≈ 2 h) : le taux de 35 à 59 s et
sa limite quand la maille s'affine. (3) La conclusion, puis, selon elle, l'advection antisymétrique ou la refonte du critère.

**Critères, écrits avant.** **Benjamin-Feir** si les deux tiennent : (a) à 25 cm, plus de la moitié de l'énergie de δ en fin de
calcul dans la bande instable de Benjamin-Feir autour de la houle, `|k − K| ≤ 2√2·ak·K` (ak = 0,118 : 0,52 rad/m autour de
`K` = 1,57) ; (b) le taux extrapolé à maille nulle (ajustement en `dx` sur les trois mailles les plus fines) entre la moitié et le
double de `ω(ak)²/2` = 0,027 s⁻¹. **Numérique** si l'énergie est d'abord aux longueurs d'onde de moins de `4·dx`, ou si la limite
est sous 0,01 s⁻¹ (la croissance disparaît avec la maille). Autrement : indécis, écrit tel quel. Rien du pas couplé n'est changé
dans cette session sans un critère écrit avant.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `MER_SPECTRE` au banc.
- [ ] **P3** — mesures : le spectre à 25 cm ; le taux à quatre mailles ; la conclusion selon les critères.
- [ ] **P4** — preuve (MER-S369 §7) ; A320 ; registres ; suite.
- [ ] **P5** — rituel.

### Notes de reprise
