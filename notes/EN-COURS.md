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

Session : S367 — **en cours**. **Physique : le champ d'écume de B** (7.1) — verdict : *« Je valide les rendus sauf
ecume »*, lu sur toutes les revues en attente (R18, R21, R22, R23, R25), l'écume de R21 refusée. Alternance d'ADR-191 D3
après deux sessions de rendu : la physique ; et 7.1, *absent*, conçu par [ADR-014](../docs/adr/ADR-014-mousse-spray-bulles.md),
est la physique de l'écume refusée.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Ce qui manque à l'écume rendue** (S356, S360) : elle n'a **pas de mémoire** — une tache instantanée là où la crête
dépasse un seuil, au bord lisse. ADR-014 a décidé un **champ** `F` à deux canaux — actif, demi-vie ≈ 3 s ; résiduel,
≈ 30 s, nourri par l'actif —, **advecté par la vitesse orbitale complète** : c'est elle qui rassemble l'écume dans les
zones de convergence en traînées alignées au vent. Banc B9 : la couverture contre Monahan, la persistance, les traînées.
Ici, **la référence** dans le cœur ; la production sur la carte de Godot viendra au rendu (SPEC-006 §4, RG16F).

Critères, écrits avant le code :
1. **Décroissance** : un champ uniforme sans source suit la solution exacte du système à deux canaux (actif → résiduel)
   à 10⁻⁶ près ; **advection** : un motif translaté par une vitesse uniforme revient à sa place à l'erreur
   d'interpolation près, bornée et publiée ; déterminisme au bit.
2. **Couverture (B9, scénario 1)** : mer pleinement développée à `U10` = 7, 10, 13 m/s, sources d'ADR-014 §3.1 ; la
   part de surface blanche au régime établi contre Monahan (`3,84·10⁻⁶·U10^3,41`). **Prédiction** : avec les seuils
   physiques d'ADR-014 (cambrure 0,6/7, accélération 0,45 g) et une bande résolue, l'ordre de grandeur, pas mieux qu'un
   facteur 3 — le seuil se cale alors sur Monahan, **dit**, jamais relevé en silence.
3. **Traînées (B9)** : l'écume résiduelle advectée par la vitesse orbitale s'allonge le long du vent — rapport des
   longueurs de corrélation le long / en travers, contre le témoin advecté par le seul courant moyen.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les verdicts consignés : revue (§23, §26–28, §30), décisions de la file, liste (8.4 à 8.10), dépendances.
- [x] **P3** — la référence du champ (`ecume.rs`) : grille ancrée, deux canaux, décroissance, advection semi-lagrangienne
  par la vitesse de surface de B, sources de B ; critère 1.
- [>] **P4** — la couverture contre Monahan, trois vents ; critère 2.
- [ ] **P5** — les traînées de convergence ; critère 3.
- [ ] **P6** — preuve ECUME-S367 ; liste 7.1, file, dépendances, feuille de route, index.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2.** Verdict consigné, lu sur la demande de S366 (R21, R22, R23, R25) : forme fine, caustiques, ciel, lueur face
  au soleil **validés** ; **l'écume refusée** ; tonalité sans choix — AgX reste le défaut ; **R18** (en direct, non
  rappelé) toujours attendu. Liste : 8.4 (reste absent), 8.5, 8.9, 8.10 retouchés sans changer de case.
- **P3, critère 1 tenu.** `ecume.rs` : `ChampEcume` (deux canaux, `advecter` semi-lagrangien bilinéaire, `decroitre`
  exacte, `deferlement` — montée lisse de `−a_z/g` autour de 0,45 —, `pas_de_temps`, `couverture`). Décroissance :
  **7,7e-7** du fermé à 10 s ; advection d'une gaussienne, 100 pas : centre à **9,4e-6 m**, masse à 1,8e-6, sommet
  1 → 0,863 (diffusion de l'interpolation, publiée) ; onde seule : indicateur **0 / 0,5 / 1** à ak = 0,30 / 0,45 / 0,60
  (signe et échelle de l'accélération) ; déterminisme, même hash.
