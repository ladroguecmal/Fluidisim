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

Session : S524 — **en cours**. En autonomie : **le lot des registres** (dû ; feuille de route S522–S523), puis **A331** — W ne dit pas
quand un point échantillonné sort du domaine honnête de son chemin (S523 : 23–62 % d'erreur silencieuse). Le rayon d'ADR-132
(`2π·angulaire/(3·coupure)`) a été écrit pour la distance à la source ; S523 suggère qu'il vaut pour **la distance du chemin émetteur aux
points**. Le calibrer, puis le vérifier.

**Ce que la session fait.** (a) **La calibration** : le montage de S523 (5 m de fond, 10 m/s, σ 2 m, recette 512 × 256 à coupure 3 :
rayon 179 m), la zone de 80 à 100 m derrière la source, `|y|` ≤ 100 m ; la durée varie, et avec elle `D`, la plus grande distance d'un point
du chemin à un point de la zone. W contre la référence (écart quadratique). (b) **La garde** : `spectral_pressure::farthest_emission(chemin,
min, max)` — la plus grande distance d'une extrémité de segment à un coin de la boîte d'échantillonnage ; l'hôte la compare au rayon
honnête et l'annonce (comme il annonce déjà la durée, ADR-132).

**Ordre de grandeur, calculé.** `D` = 128 / 189 / 260 / 335 m à 16 / 24 / 32 / 40 s, soit `D/R` = 0,72 / 1,05 / 1,45 / 1,87. Les montages
de S523 : le premier `D/R` = 2,96 (62 %), le retenu 0,94 et 0,53 (< 0,4 %).

**Critères, écrits avant.** (1) **La loi** : écart ≤ 5 % pour `D/R` ≤ 1 ; > 10 % pour `D/R` ≥ 1,4. (2) **La garde** : elle signale chaque
montage mesuré à plus de 10 % et aucun à moins de 5 % (les quatre de la calibration, les trois de S523, ceux de S519 et S522). (3) La suite
du cœur, le banc de non-régression inchangés.

### Plan

- [x] **P1** — jeton ; le lot (feuille de route S522–S523) ; plan.
- [ ] **P2** — la calibration ; (1).
- [ ] **P3** — la garde ; (2), (3).
- [ ] **P4** — preuve ; A331 ; rituel (`--lot`).

### Notes de reprise
