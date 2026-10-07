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

Session : S677 — **terminée**. En autonomie vers la v2 ; K3, 3.5 (« manquent… la largeur de la zone »). La polyligne de S588–S633 est
tirée de McCowan, un seuil sur une houle unique. La côte 2D porte maintenant une mer qui déferle par Battjes et Janssen, avec sa
fraction de vagues déferlées `Q_b` et sa dissipation `D` en chaque nœud.

**Ce que la session fait.**

- **`Cote2D` garde `Q_b` et `D`** (par `ρ`) à chaque nœud des tables, avec déferlement : 8 octets par nœud, communs aux composantes.
- **`Cote2D::zone_de_deferlement(seuil)`** rend les polylignes où `Q_b` = `seuil`, dans les axes locaux de B. Les carrés de marche de
  S630 sont séparés du calcul de l'écart (`deferlement::contours_du_champ`), au bit.
- **`Cote2D::dissipation_par_metre(ρ)`** rend `∫ D ds`, moyennée le long de la côte, en kW/m.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : les essais de S630 (les contours d'une côte quelconque), aux mêmes sorties.
- **instrument** : l'équilibre d'énergie 1D de S669, sans rétroaction (la côte cuite en une marche), avec son propre `Q_b` (une
  bissection sur `ln Q`). Ce qui départagerait :
  - une dissipation juste rend `∫D ds` égal au flux perdu de la référence ;
  - un `D` sans `g`, ou sans `f̄`, s'en écarte d'un facteur ;
  - un indice de table décalé déplace la ligne de plusieurs pas.
- **calcul** (scratchpad `s677_calc.py`, et ce script qui asserte) :
  - le flux du large vaut 21,7 kW/m ; 20,7 kW/m sont perdus, égaux à `∫D ds` ;
  - la zone commence à `Q_b` = 1 %, à s = 3 717 m (5,66 m de fond), et mesure 233 m de large ;
  - le plancher du début : `Hrms` à 0,4 % donne Δs ≈ 1.2 m, d'où la borne de **3 m** ;
  - l'énergie : le flux au rivage (4,5 % du flux du large) à 0,8 % près, d'où la borne de **2 %**.
- **ADR** : ADR-196, ADR-268 ; SPEC-006 §6.
- **pièges** :
  - `E = ρg·a²/2` : le premier calcul de ce plan comptait `g` deux fois. `Hrms` au rivage, retrouvé à 0,511 m, l'a montré ;
  - la grille des contours (`x` le plus rapide) : `s` en `x`, `n` en `y` ;
  - les axes locaux de B : `x = n̂·(s + origine) + t̂·n`.

**Critères, écrits avant.**

1. Le partage de `contours` : au bit (les essais de S630 inchangés).
2. `∫D ds` de la côte (une marche) à moins de **2 %** du flux perdu de la référence 1D.
3. À `Q_b` = 1 %, une polyligne ouverte sur toute la largeur, chacun de ses sommets à moins de **3 m** du début 1D.
4. Rapportés : la largeur de la zone, le flux dissipé en kW/m, la côte au point fixe (S674) comparée.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `contours_du_champ` ; `Q_b`, `D` dans `Cote2D` ; l'essai ; (1)–(4).
- [x] **P3** — preuve ; liste 3.5 ; rituel.

### Notes de reprise
- **P2 fini** — (1) au bit ; (2) 0,94 % ; (3) une ligne, 97 sommets, à 0,13 m ; (4) début 3 716 m (5,7 m), largeur 234 m, ≈ 21 kW/m.
