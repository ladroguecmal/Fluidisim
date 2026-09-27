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
- [x] **P3** — la vorticité de la grille aux centres des mailles ; essai (rotation solide, uniforme) ; critère 2.
- [x] **P4** — `ColumnsSwitch::floor_vorticity` : la colonne requise et le fond sous la maille tourbillonnaire la plus basse ; essai.
- [x] **P5** — le banc du tourbillon enfoui ; les trois montages ; critère 3.
- [x] **P6** — la vague de Chen avec le seuil ; critère 4.
- [>] **P7** — suite entière ; critère 5.
- [ ] **P8** — preuve (BANDE-ETROITE-S413 §6), ADR-212 note (C6c-3), liste, file, feuille de route, index.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — R35 : REVUE-VISUELLE §40 (verdict), BANDE-ETROITE-S413 §5.5, file (décision en tête ; campagne). Réglage retenu pour
  la suite ; défauts du code inchangés jusqu'à C7 (les « Reproduire » de S408–S414 les citent).
- **P3** — `Apic3::vorticity(i, j, k)` : `|∇ × u|` au centre de la maille, vitesses ramenées aux centres, différences centrées
  (décentrées au bord). Essai `_s415` : rotation solide d'axe `y`, Ω = 1,5 s⁻¹ — **3,000** partout (critère 2 tenu, au bit à
  l'arrondi) ; uniforme : 0.
- **P4** — `ColumnsSwitch::floor_vorticity` (défaut `None`, S414 au bit) : (3b) de `decide`, une colonne dont une maille d'eau
  tourbillonne au-delà du seuil est requise ; `place_floor`, le fond à `k` sous la plus basse de ces mailles. Essai `_s415` :
  cisaillement enfoui (rangées 2 à 4, colonnes 6 à 10) — sans seuil tout passe aux colonnes ; à 2 s⁻¹, bande 3 à 12, fond **0**
  sous le cisaillement, 0,3 m à côté ; volume exact. 32 essais d'APIC 3D tenus.
- **P5** — `examples/apic3d_tourbillon.rs` : Lamb–Oseen d'axe `y`, `r_c` 0,1 m, 0,5 m/s au plus, à mi-profondeur d'un bassin de
  2 × 0,2 m et 1 m d'eau, 5 s ; énergie cinétique et vorticité max sur la grille (mêmes lectures pour tous) ; accesseurs
  `velocity_v`, `velocity_w`, `grid_vorticity`. **Et, ajouté** : `floor_speed` (la vitesse, les mots de l'utilisateur) à côté de
  `floor_vorticity`, `cell_speed`. Énergie restante à 5 s / vorticité max / particules :

  | montage | 5 cm | 2,5 cm |
  |---|---|---|
  | APIC seul | 0,750 / 5,24 / 25 600 | 0,806 / 6,98 / 204 800 (511 s) |
  | bande de C6c-2 : tout à la grille | 0,403 / 2,34 / 0 | 0,518 / 3,79 / 0 |
  | vorticité 1 s⁻¹ | 0,625 / 5,15 / 3 700–6 150 | 0,709 / 7,02 / 25 800–47 000 (289 s) |
  | vorticité 0,3 s⁻¹ | 0,705 / 5,23 / 5 150–11 360 | 0,750 / 7,01 / 34 500–56 600 (331 s) |
  | **vorticité 1 + vitesse 0,2 m/s** | **0,765** / 5,20 / 9 070–11 260 | au calcul |
  | vitesse 0,1 m/s | 0,789 / 5,21 / 13 500–18 500 | — |
  | vorticité 0,3 + vitesse 0,2 | 0,773 / 5,21 / 9 250–12 860 | — |

  Critère 3 : **prédiction 1 tenue** (la grille perd 2,4 à 2,5 fois l'énergie qu'APIC perd) ; **prédiction 2** (à 20 % de la perte
  d'APIC) **manquée à 1 s⁻¹** (1,5 fois), tenue à 0,3 s⁻¹ à 5 cm (1,18), manquée à 2,5 cm (1,29) ; **le cœur du tourbillon est gardé**
  à tous les seuils (vorticité max à 2 % d'APIC seul). **Compris** : l'énergie d'un tourbillon est surtout dans son écoulement
  extérieur, **irrotationnel** — la vorticité le laisse à la grille, qui le lisse ; **la vitesse** (l'idée de l'utilisateur à la
  lettre) le prend : avec elle, l'énergie de la bande égale celle d'APIC seul, 2,3 à 2,8 fois moins de particules.
- **P6** — la vague de Chen (maintien 0,3 s, fond 4 ; trois à la fois) : C6c-2 — part 0,50, 12 262 particules, 33 s ; **vorticité
  1 s⁻¹ — part 0,998**, 31 292, 50 s, 13 retours rapides, « retournement » à 0,449 (une maille vide) ; vorticité + vitesse 0,2 m/s
  — part 1,000, 59 544, 82 s. **Le seuil absolu de vorticité n'est pas à l'échelle** : sous une houle raide de 2 m, la
  déformation de la grille (1 à 3 s⁻¹) fausse sa vorticité ; la vitesse prend toute la houle. **Ajouté : la part de rotation**
  (`rotation_share`, le critère Q sans dimension, `|Ω|²/(|Ω|²+|S|²)`, avec un gradient plancher de 0,5 s⁻¹ ; `floor_rotation`) ;
  essai `_s415` (rotation solide 1, déformation pure 0). Rotation 0,6 : vague — part 0,577, 17 024 particules, retournement
  0,7021 (C6c-2 : 0,7021), impact 1,3088, **28 retours rapides** ; tourbillon à 5 cm — énergie 0,51, vorticité max 4,28 (le cœur
  moins bien gardé que par la vorticité simple) ; rotation + vitesse 0,2 — 0,765, 9 000–10 600 particules. Tourbillon à 2,5 cm,
  vorticité 1 + vitesse 0,2 : **0,800** (APIC seul 0,806), vorticité max 7,00, 66 000–84 000 particules, 360 s. **Critère 4** :
  la vorticité absolue manqué (la bande prend tout) ; la rotation tient le retournement mais ajoute 39 % de particules et hésite.
  **Conclusion** : aucun critère ne fait les deux — gratuit sous une houle, complet sur un tourbillon. **La voie** : la vitesse
  **propre de δ**, relative à B (ADR-198) — la houle est à B, un seuil sur l'écart ne coûte rien sous elle et prend courants,
  sillages, jets ; elle demande la production relative à B (C7, C10). Essai du jet corrigé (5 s⁻¹ aux bords, pas 2,5).
