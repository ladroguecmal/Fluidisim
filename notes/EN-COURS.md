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

Session : S363 — **en cours**. **Rendu 6 : le ciel** — *« Tu as raison sur le ciel, il n'aide pas au reflets et limite la
qualité du rendue final »* (R20) ; alternance d'ADR-191 D3 après S362.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — deux défauts vus en S359 : une **couture verticale au centre du ciel de Godot**, et des nuages en blocs. Et un
fait de S308 : le « ciel clair » de `--meilleur` est **plat** (sommet à 0,81 de l'horizon, la photographie de
référence de l'utilisateur à 0,36) ; S308 en avait tiré un ciel calé sur elle (`ciel_mesure`, extinction par canal), jamais
passé dans `--meilleur`. **La cible d'image** de S308 (`outils/cible_image.py`, valeurs de la photographie dans
`outils/courbe_tonalite.py`) : p05/p50 0,1926 ; dynamique p95/p05 23,70 ; **contraste local 0,4549** ; fraction claire
0,07415. L'afficheur plafonnait à 0,31–0,32 de contraste local, et S308 disait le manque « dans la mer ».

Critères, écrits avant le code :
1. **La mesure d'abord** : les quatre grandeurs comparables sur Godot tel qu'il est (poses de R14, S360–S361) et sur
   l'afficheur de R19, contre la photographie ; **prédiction** — la surface fine de S360 relève le contraste local
   au-dessus de celui de l'afficheur.
2. **La couture** : hypothèse — le hachage `fract(sin(x)·43758)` avec x ≈ 10⁴ est hypersensible à l'arrondi, et Godot
   compile `i + (1, 0)` autrement de part et d'autre de la colonne centrale. Remède : un hachage entier. Mesure : saut de
   luminance du ciel entre les colonnes 639 et 640, rapporté aux sauts voisins ; après, du même ordre qu'eux (≤ 2 fois
   leur médiane).
3. **Le ciel calé** : `ciel_mesure` de S308 porté dans `ciel.gdshaderinc` (une source, ciel et reflets) ; les quatre
   grandeurs remesurées ; ne pas dégrader le rapport mer / ciel sous l'horizon de plus de 15 % sans le dire.
4. **Images R23** ; preuve ouverte par « Reproduire », file, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la mesure contre la photographie ; critère 1.
- [ ] **P3** — la couture : diagnostic, hachage entier ; critère 2.
- [ ] **P4** — le ciel calé sur la photographie ; critère 3.
- [ ] **P5** — images R23, preuve, file, index ; critère 4.
- [ ] **P6** — rituel.

### Notes de reprise
