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

Session : S414 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Tu peux commit tout, les pousses. Pour le fond
automatique il serait intéréssant que les systèmes de prédictions permettent de jouer sur la position du fond, exemple si un
évènements va aller en profondeur mettre le fond a bonne distance etc.... Mais sans prédictions comme tu le pensais cela me
convient. »* — `main` et `poste` poussés (`617ea1b4..a554f0cb`). Suite : **C6c-2** (ADR-212 §4 et D4). Agent : Claude Code
(Opus 5.5), au poste ; référence CPU.

**Thèse.** Le critère de S408 place aussi le **fond** : dans chaque colonne de la bande, `k` mailles sous la **première maille
non-eau depuis le bas** (la surface, le fond d'une cavité, le dessous d'une lèvre, le corps), avec une hystérésis `h` — descendre
dès que la cible passe sous le fond, remonter seulement au-delà de `h` mailles. **Option, l'idée de l'utilisateur** : dans
l'empreinte prévue du corps (l'horizon de S408), la cible descend sous le point le plus bas qu'il atteindra. Descendre ensemence
la tranche au réseau nominal (exact) ; remonter absorbe les particules de la tranche, l'écart de volume au solde vertical (exact).
Bande → colonne : l'eau sous le fond et le solde vertical comptent dans la masse ; les mailles à la grille sont « occupées ».
Sans `fond` réglé : S408 au bit.

**Critères, écrits avant** (ADR-212 §3). (1) Sans fond : les essais de S398 à S413, B10 et la vague au caractère près. (2) Aller et
retour du fond (descente puis remontée, dix fois, sur un état réel) : volume ≤ 10⁻⁹. (3) **B10** (`Fr` = 2, `D/dx` = 8, quart,
maintien 0,3 s — R34), `fond` = 4 : pincement **à un pas d'APIC seul** (1,5067 √(D/g)) ; particules **÷ 3 au moins** contre S408
(29 120) ; volume ≤ 10⁻⁹ ; mouvements du fond comptés — sans et avec la prédiction. (4) **La vague de Chen** (maintien 0,3 s),
`fond` = 4 : planche R35 (APIC seul | bande pleine | bande étroite) — **jugée par l'utilisateur** ; particules, calcul, retours
rapides publiés. (5) Suite entière, zéro avertissement. **Arrêt** : si le pincement s'écarte de plus d'un pas, publier ce qui le
porte (le fond trop haut au passage de la cavité ?) ; ne rien rendre défaut.

### Plan

- [x] **P1** — jeton, plan seul ; l'idée de l'utilisateur au plan (option mesurée).
- [x] **P2** — bande → colonne avec fond : occupation, masse (fond, solde vertical) ; la bascule n'en refuse plus ; essai.
- [>] **P3** — `apply_band_floor` : descente (ensemencement), remontée (absorption, écart au solde vertical) ; essai d'allers-retours ; critère 2.
- [ ] **P4** — `ColumnsSwitch` : cible du fond (`floor_cells`, `floor_hysteresis`, `floor_prediction`), appliquée après le masque ; essais.
- [ ] **P5** — B10 avec fond, sans et avec prédiction ; critère 3.
- [ ] **P6** — la vague de Chen avec fond ; planche R35 ; critère 4.
- [ ] **P7** — suite entière ; critère 5.
- [ ] **P8** — preuve BANDE-ETROITE-S413 §5 (un fil, une preuve) ; liste, file, feuille de route, index ; revue R35.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — `convertible_height` : une maille à la grille est « occupée » ; `apply_columns_mask` : l'eau sous le fond et le solde
  vertical d'une colonne convertie s'ajoutent à la masse de la voie mixte, le fond s'efface ; le refus de S413 levé. Essai `_s414` :
  32 colonnes à fond → colonnes, volume exact, surface à 0,5 m (décalage 1,1 mm). 28 essais d'APIC 3D tenus.
