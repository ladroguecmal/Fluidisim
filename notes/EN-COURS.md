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

Session : S334 — **en cours**. **A317** : une coque qui perce le couvercle de δ rayonne selon la position de sa
paroi dans la maille ; chemin de la porte D ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue, pour la référence je n'ai pas trouvé »* (2026-09-24, 08:00) — aucune référence réelle,
aucun verdict formulé sur les images de S333 : la porte D reste en attente de son verdict (R15). Suite
déclarée par S333 sans verdict : A317.

**Ce que la session doit rendre possible.** Un rayonnement de δ qui ne dépende pas de la position de la paroi
sous la maille — sans quoi une coque qui se déplace dans la grille verra ses anneaux changer à chaque maille
franchie. **Diagnostic, écrit avant le code** : δ porte par colonne une hauteur de *remplissage*, l'excès
d'eau rapporté à la section entière ; sous une coque qui perce le couvercle, cet excès monte dans la seule
part libre `a` du couvercle, où la surface vaut `(η − z₀)/a`. δ y impose pourtant `ρg(η − z₀)` : la surface
d'une colonne en lamelle est `1/a` fois trop molle — 12,5 fois à 8 %. **Correctif proposé** : la pression du
couvercle d'une colonne en partie couverte est `ρg(η − z₀)/a`, ouverture bornée par `dt²·g/dx` pour la
stabilité ; ni l'état ni le transport ne changent, donc ni la conservation ni les colonnes pleines ou fermées.

Critères, écrits avant le code :
1. **Reproduction hors du jeu** : pilonnement imposé — 5 cm, 4,48 rad/s — de la coque 4 × 1,6 × 1 m dans un δ
   de 16 × 16 m, cinq placements sous la maille (φ = 5, 8, 30, 55, 80 % d'eau dans la maille de bord −y) ;
   amplitude rayonnée à 3 m de chaque flanc long : la dispersion et la dissymétrie d'A317, mesurées.
2. **Après correctif** : les dix amplitudes (cinq placements, deux flancs) à ± 5 % de leur moyenne, et chaque
   rapport de flancs à ± 5 % de 1.
3. **Rien d'autre ne change** : essais S324–S333 verts, valeurs imprimées inchangées — les cubes de S332 ont
   leurs parois sur des faces, leurs couvercles sont pleins ou fermés.
4. **Stabilité** : la scène de la porte D, 800 pas, reste stable ; volume au plancher du transport.
5. **La scène de la porte D rejouée** : placements 8/52 et 30/30, amplitudes des deux flancs à ± 5 % l'une de
   l'autre ; images refaites.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — banc `a317_lamelle` : pilonnement imposé, cinq placements ; critère 1 (avant correctif).
- [ ] **P3** — correctif : la pression du couvercle en partie couvert, commutable pour la mesure ; critère 3.
- [ ] **P4** — banc après correctif ; critère 2.
- [ ] **P5** — scène de la porte D rejouée, deux placements, images ; critères 4 et 5 ; masse ajoutée de S332
  re-mesurée.
- [ ] **P6** — preuve (PORTE-D-S333 §6, S334) ; A317 note datée ; R15 au registre des revues ; file, feuille
  de route si un état change.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2, critère 1 — A317 reproduit hors du jeu** (pilonnement imposé 5 cm, 4,484 rad/s ; bandes à 2,5–3,5 m ;
  fenêtre 3–6 s). Amplitude quadratique moyenne des flancs −y / +y, mm : φ 0,05/0,55 → 16,08 / 8,89 (1,81) ;
  0,08/0,52 → 15,81 / 8,28 (1,91) ; 0,30/0,30 → 10,89 / 10,91 ; 0,55/0,05 → 8,86 / 16,11 ; 0,80/0,80 → 15,99 /
  15,99. Moyenne 12,78 mm, **écart 35 %**. **Le diagnostic écrit ne suffit pas** : l'amplitude n'est pas
  monotone en l'ouverture du couvercle — 0,80 rayonne comme 0,05 ; minimum vers 0,5, paroi au centre de la
  maille. Un second mécanisme est probable.
