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

Session : S477 — **terminée**. En autonomie, plan de complétion K1 : **le type d'eau dans l'espace** (ADR-217 D1, second temps).

**Ce que la session fait.** Une **carte des constituants** (Chl, a_g(440), MES par texel, sur un rectangle de la carte en coordonnées
de B) lue **par fragment** : une structure `Optique` (R0, kd, c) que `optique_en(xy)` calcule — le modèle de `type_eau.py` porté dans
le nuanceur — et que les fonctions de la lumière de l'eau reçoivent en paramètre (Godot n'a pas de variable globale modifiable :
vérifié). L'eau (surface vue d'en haut et d'en dessous), le fond, le ciel sous l'eau de `mer.tscn` ; `saut.tscn` prêt à la lire. Sans
carte, `optique_en` rend les uniformes de la scène. Une scène d'essai : un panache de rivière qui entre dans la mer
(`TYPE_EAU_CARTE=panache`).

**Critères, écrits avant.** (1) sans carte, les images **au bit** (`mer.tscn` : proche, sous l'eau ; `saut.tscn` : six captures) ;
(2) le modèle du nuanceur égal à `type_eau.py` : une carte uniforme d'un préréglage donne les mêmes images que `TYPE_EAU=<préréglage>`
(à l'arrondi des flottants près : écart ≤ 1/255 par canal) ; (3) la transition montrée (images) ; (4) le surcoût GPU mesuré, ≤ 0,3 ms
par image à 1280 × 720.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `Optique`, `optique_en`, les fonctions qui la reçoivent ; la carte dans `mer.gd` ; le panache ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — (1) sans carte au bit (mer et saut) ; (2) carte uniforme contre préréglage : ≤ 1/255 — après un premier contrôle mal posé
  (la grille de 12 km dépasse la carte de 5 km) ; (3) le panache montré ; (4) surcoût **0,044 ms**. Preuve : TYPE-EAU-S474 §5.
- **P3** — la preuve : TYPE-EAU-S474 §5 ; **le lot des registres** (en retard : dû en S473) pour S471–S477 — LISTE (8.5, 13.1), FEUILLE-DE-ROUTE ; QUESTIONS-OUVERTES inchangée ; journal ; jeton libre ; maillons 1 ; suivant : S478, K2.
