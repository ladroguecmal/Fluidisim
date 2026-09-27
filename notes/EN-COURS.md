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

Session : S415 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Je valides R35, continue avec ta recomandation »* — la
recommandation : **C6c-3**, le fond qui suit l'écoulement (*« le mesh du fond malaxable en fonction du courant, les particules
peuvent naitres et disparaitre en fonction de leurs vitesse »*), conception d'abord. Agent : Claude Code (Opus 5.5), au poste ;
référence CPU.

**La conception, en une idée.** Ce que la grille perd n'est pas la vitesse — un courant uniforme, même rapide, elle le porte sans
perte —, c'est **ce qui varie** : tourbillons et cisaillements, que son advection semi-lagrangienne lisse, et que les particules
d'APIC gardent. Le critère suit donc la **vorticité** `|ω| = |∇ × u|`, lue sur la grille après le pas : une colonne dont l'eau
tourbillonne au-delà d'un seuil passe en bande (dilatée, avec le maintien), et son fond descend à `k` mailles **sous la maille
tourbillonnaire la plus basse** — les particules naissent là où l'eau tourne, disparaissent (le fond remonte) là où elle redevient
régulière. Sous une houle — irrotationnelle —, rien ne change. C'est l'esprit de l'Extended Narrow Band FLIP (Sato et al. 2018 : le
passage particules ↔ grille « en n'importe quel endroit »), dont le critère exact n'est pas lu.

**Critères, écrits avant.** (1) Sans seuil (`None`, le défaut), au bit : essais S398–S414, B10 et la vague. (2) La vorticité de la
grille : une rotation solide rend `2Ω` à 10⁻⁵ près au cœur, un écoulement uniforme 0. (3) **Le tourbillon enfoui** (nouveau banc) :
un tourbillon de Lamb–Oseen d'axe horizontal, à 0,5 m sous une surface calme, 5 s — APIC seul, la bande de C6c-2 (tout passe aux
colonnes : le tourbillon à la grille), la bande à vorticité ; **prédiction** : l'énergie cinétique perdue à la grille est au moins
**deux fois** celle d'APIC seul, celle de la bande à vorticité à **20 %** d'APIC seul ; particules comptées. Si la grille garde le
tourbillon aussi bien qu'APIC, le publier : le critère ne sert pas ici. (4) **La vague de Chen** avec le seuil : particules et
temps publiés, pas plus d'un pas d'écart au retournement de C6c-2 ; la planche si la forme change. (5) Suite entière, zéro
avertissement. **Arrêt** : ne rien rendre défaut.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — verdict R35 consigné (revue, file, preuve) ; le réglage retenu (maintien 0,3 s, fond 4, prédiction à horizon court) inscrit, défauts inchangés jusqu'à C7.
- [>] **P3** — la vorticité de la grille aux centres des mailles ; essai (rotation solide, uniforme) ; critère 2.
- [ ] **P4** — `ColumnsSwitch::floor_vorticity` : la colonne requise et le fond sous la maille tourbillonnaire la plus basse ; essai.
- [ ] **P5** — le banc du tourbillon enfoui ; les trois montages ; critère 3.
- [ ] **P6** — la vague de Chen avec le seuil ; critère 4.
- [ ] **P7** — suite entière ; critère 5.
- [ ] **P8** — preuve (BANDE-ETROITE-S413 §6), ADR-212 note (C6c-3), liste, file, feuille de route, index.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — R35 : REVUE-VISUELLE §40 (verdict), BANDE-ETROITE-S413 §5.5, file (décision en tête ; campagne). Réglage retenu pour
  la suite ; défauts du code inchangés jusqu'à C7 (les « Reproduire » de S408–S414 les citent).
