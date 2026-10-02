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

Session : S455 — **terminée**. Sans « Continue » (ADR-215 D1) : l'étape 2 d'ADR-215 D4, **le temps réel à 4 m**.

**Le constat (S454).** La scène de 4 m (80 × 80 × 114 mailles, 3,2 m d'eau, 2,5 m d'air) coûte **27 ms de carte par pas** pour un pas
de **4,7 ms simulées** ; avec les relectures, 44 ms au mur ; la fenêtre tient **0,11** du temps réel.

**Ce que la session fait.** (1) **Le profil** : la durée de chaque étage de la carte sur la scène de 4 m (médianes sur le saut entier).
(2) **Les réductions**, par ordre de gain attendu, chacune mesurée : la profondeur (3,2 m d'eau, imposés par l'arrêt de B10 à
`3·Fr·D` ; une scène de jeu n'en demande pas tant), l'air (2,5 m), les étages qui balaient toute la grille ou toutes les faces alors
que la bande en occupe un sixième, les relectures du pas stable (une réduction sur la carte au lieu de 4 n mots relus).

**Critères, écrits avant.** (1) le profil publié, étage par étage ; (2) **simulé / réel ≥ 0,9** dans la fenêtre à 4 m, la masse exacte et
la scène stable jusqu'à t = 16 ; (3) si (2) n'est pas atteint, le gain obtenu et ce qui reste, chiffrés.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le profil des étages.
- [x] **P3** — les réductions, mesurées une à une.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — le profil (scène de 4 m, 3,2 m d'eau, 2,5 m d'air ; médianes, ms) : **projection 13,9** ; reconstruction 1,85 ; décision
  de la bascule 1,64 ; fil de l'échange 0,94 ; séparation et corps 0,84 ; P2G 0,78 ; le reste sous 0,6. Au mur, **46 ms** par pas
  pour 25 ms de carte : les relectures du pas stable (toutes les vitesses de particules et de faces) et de la pression entière.
- **P3** — (a) **la vitesse maximale sur la carte** (`speed_max`, `speed_max_finish` ; un mot relu) et `pressure_stats` (huit mots) :
  le pas stable identique au pas près (S453 : 125 pas, pas moyen 4 856 µs, inchangés) ; mur 46 → **31,5 ms**. (b) **une scène de jeu**
  (`C10_ARRET=0.6` : la sphère s'arrête à 0,6 m, 1,4 m d'eau ; `C10_AIR=1.5`) : 80 × 80 × 58 mailles, pas de carte **12 ms**
  (projection 6,7), l'eau au plus haut à 2,11 m sous un plafond à 2,9 m ; stable jusqu'à t = 16. (c) **les horodatages éteints** dans
  la boucle vivante (`set_timing`). (d) **le nombre de Courant** (`set_courant`, `COURANT=`) : 0,8 et 1,0 stables jusqu'à t = 16,
  masse exacte ; à 1,0, l'image à t = 2 ne change que sur 0,3 % des pixels — **retenu pour la scène vivante** (ADR-215 D2). La fenêtre
  (20 s, 4 m) : **simulé / réel 0,98** (0,83 sur les 3 premières secondes, le saut) ; image 16,7 ms en médiane, 37,6 ms au 99e centile.
- **P4** — C10-SCENES-S454 §4 ; journal ; jeton libre ; maillons 12 (justifiés : S406) ; suivant : S456, la houle (ADR-215 D4 étape 3).
