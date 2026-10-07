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

Session : S599 — **terminée**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S597–S598), puis **12.3 — le précalcul côtier
stocké** (absent ; SPEC-005 §6) et, avec lui, la part côtière de **2.8** (absent ; la météo à la fin).

**Ce que la session fait.** `cotier.rs` : la **cuisson** d'une plage — pour 4 états de mer × 4 phases de marée, un `CoastalState` :
un champ 2D au demi-mètre (hauteur de houle, `u`, `v`, intensité du rouleau) en **`f16`** et la polyligne de déferlement (S588) ; la
hauteur hors de la zone de déferlement par la référence de B (levée), dedans saturée à `0,78·h` ; le rouleau, la dissipation
`−d(E·c_g)/dx` ; `u`, `v` **nuls** tant que le courant de dérive littorale n'est pas calculé (écrit tel quel). **L'empreinte** (FNV-1a) des
entrées : bathymétrie, états, phases — une plage dont le fond change est **obsolète**. **La recherche par paramètres** (I-09) : l'état le
plus proche en `(Hs, phase)`, la phase repliée sur un tour — jamais un mélange de champs.

**Références, calculées avant** (ce script les écrit). Une plage de pente 0,04 (120 × 20 m, la grille de SPEC-005 : 240 × 40 texels), une
houle de 8 s en incidence normale, Hs ∈ {0,5 ; 1 ; 1,5 ; 2} m, une marée de 1 m (η = 0 ; +1 ; 0 ; −1 m aux phases 0, ¼, ½, ¾). La profondeur
de déferlement, par la levée de mes formules : **`h_b`** = 0.9343, 1.6414, 2.2892, 2.9043 m ; la ligne de déferlement à
`x_b = (h_b − η)/0,04` — à Hs = 2 m, **47.6081 m** à marée haute et **97.6081 m** à marée basse (le déplacement
`2 m/0,04` = 50 m). La taille : **76800 octets** par état (240 × 40 × 4 × 2), **1228800 octets** pour les seize (SPEC-005 : « 77 Ko »,
« 1,2 Mo »).

**Quantum** (ADR-236, ADR-249) : la ligne interpolée sur 0,5 m (de l'ordre de 0,1 mm, S588 au pas de 5 m : 5 mm) ; le `f16`, 2⁻¹¹ relatif.
**Critères, écrits avant.** (1) la taille exacte ; (2) la ligne de déferlement de chaque état à 1 cm de `x_b`, son déplacement avec la marée ;
(3) les hauteurs relues du `f16` à 2⁻¹⁰ relatif de leur valeur cuite, la hauteur saturée dans la zone de déferlement, le rouleau nul au
large et positif dedans ; (4) la recherche : `(1,1 m ; 0,97)` → l'état `(1 m ; 0)` (la phase repliée), `(1,8 m ; 0,6)` → `(2 m ; ½)` ;
(5) l'empreinte : deux cuissons identiques, la même ; un centimètre de fond changé en un nœud, une autre.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — `cotier.rs` et ses essais ; (1)–(5).
- [x] **P3** — preuve ; listes 12.3, 2.8 ; rituel (`--lot`).

### Notes de reprise
- **En route** (ADR-244 D1, avant de corriger l'essai) : l'essai exigeait une ligne par rangée pour **tous** les états — une attente hors du
  plan. La référence du plan elle-même place la ligne de l'état `(0,5 m ; ¼)` à `x_b = (0,9343 − 1)/0,04` = −1,64 m, **hors de la grille**
  (qui commence à 0,25 m) : à marée haute, la petite houle atteint le bord sans déferler. États hors de la grille (calculés) : [(0.5, 1.0)]. L'essai
  attend aucun sommet pour eux ; le critère (2), 1 cm, inchangé pour les autres.
- **P2 fini** — 1 228 800 octets ; les lignes à 0,22 mm ; 50,0000 m ; le champ ; la recherche ; l'empreinte. Suite 775.
- **P3** — preuve COTIER-S599 ; listes 12.3 et 2.8 (absent → partiel) et décompte ; index ; journal ; le lot.

