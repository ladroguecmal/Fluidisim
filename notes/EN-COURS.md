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

Session : S424 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §15 : B10 en bande étroite 3,26 ms par pas + 0,48 de bascule au p99 ; la projection
1,35 ms, ≈ 14 itérations à ≈ 75 µs, dix-sept dispatchs chacune).

**Ce que la session fait, dans l'ordre du gain attendu.** (1) **Les niveaux grossiers en mémoire de groupe** : S423 a mesuré que le
groupe des niveaux ≥ 2 coûte ≈ 40 µs par cycle (sur ≈ 75) — dix-huit phases à barrière, chacune un aller-retour en mémoire globale.
Sur B10, les niveaux 2 et 3 font 380 mailles : `x`, `t`, `r` et la nature tiennent dans 6 Ko de mémoire de groupe ; la restriction
lit le niveau 1 en global, la prolongation y écrit, tout le reste dans le groupe. Même arithmétique, même ordre ; la version globale
reste quand les niveaux ne tiennent pas (choisie à la création). (2) **Les noyaux fusionnés**, à arithmétique identique : `α` calculé
dans chaque groupe de la mise à jour (le repli des produits est déterministe) et le premier lissage fin dans la même passe ; le
premier lissage du niveau 1 dans la restriction ; `β` dans la direction, le scalaire `r·z` en double tampon selon la parité de
l'itération (deux pipelines par constante) — dix-sept dispatchs → treize. (3) Mesurer ; ce qui reste dit.

**Critères, écrits avant.** (1) Issues inchangées : étages à l'arrondi, symétrie du cycle, itérations comme S423 ; B10 en bande
étroite au pincement de la référence, volume exact ; ballottement, raccord, bande dans leurs témoins. (2) Le coût par itération et la
projection publiés avant et après ; **visé : projection ≤ 1 ms au p99** sur B10 en bande étroite ; δ ≤ 2 ms reste l'objectif de C7e,
la session dit ce qui en reste. (3) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les niveaux grossiers en mémoire de groupe ; symétrie, issues, mesure.
- [x] **P3** — les noyaux fusionnés (`α` et premier lissage ; restriction et premier lissage du niveau 1 ; `β` et direction) ; mesure.
- [x] **P4** — non-régression, suite ; preuve §16 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — `mg_coarse_shared` : les niveaux ≥ 2 en mémoire de groupe quand ils tiennent dans 1 024 mailles (B10 : 380), `MG_GLOBAL=1` rend la version globale. **Incident** : la pipeline mettait **283 s** à se créer — FXC déroule élément par élément la mise à zéro de la mémoire de groupe que wgpu ajoute (`TEMPS_PIPELINES=1`, `PIPELINE_SEULE=<entrée>` pour le voir) ; elle coûtait déjà 15 s à `switch_apply_group`. **Coupée pour toutes les pipelines** (chaque noyau écrit sa mémoire de groupe avant de la lire — audit des 26 variables) : création des 89 pipelines **≈ 80 → 27 s**, `mg_coarse_shared` 3,2 s ; à revérifier par toute la non-régression (P4). Cycle : résidu, symétrie, itérations **identiques au chiffre près** à la version globale (ballottement, raccord, B10). B10 en bande étroite : pincement identique, volume exact ; **projection médiane 1,28 → 1,15 ms, p99 1,35 → 1,22** (≈ 9 µs par itération : moins que les ≈ 40 attendus — les phases à barrière coûtent encore).
- **P3** — les noyaux fusionnés, à arithmétique identique : `α` dans chaque groupe de la mise à jour avec le premier lissage fin (`mg_cg_update_alpha`), le premier lissage du niveau 1 dans la restriction, `β` dans la direction (`mg_cg_beta_direction`, `r·z` en double tampon, deux pipelines par la constante `CG_PAR`) : 17 → 12 dispatchs, mais **4 µs par itération seulement** (médiane 1,15 → 1,09 ms) — le nombre de dispatchs n'est pas le coût. **Profil** (`PROFIL=1`, banc B10 : chaque noyau répété 50 fois, horodaté) : le groupe des niveaux grossiers **34 µs**, `mg_restrict1` 10,9, les autres 1 à 3. Isolé par une constante d'essai (retirée) : la restriction vers le niveau 2, faite par le seul groupe, 13 µs ; les six lissages du plus grossier 3,7 (≈ 0,6 µs par phase) ; la géométrie relue en global, rien (mise en mémoire de groupe quand même). **Sortis du groupe**, en dispatchs parallèles et à expression identique : `A·z` fin par maille (`mg_fine_az`, dans `F_Q`) puis la restriction vers le niveau 1 ; `L₁·x₁` par maille (`mg_l1_ax`, dans `t₁`) puis la restriction vers le niveau 2 ; la prolongation vers le niveau 1 (`mg_prolong1`). Cycle : résidu, symétrie, itérations identiques au chiffre près. **Itération ≈ 52 µs** (69) : groupe 17,9, restriction 1 : 2,6 + 2,1, restriction 2 : 1,9 + 1,4. B10 en bande étroite : pincement identique, volume exact ; **projection médiane 0,84 ms, p99 0,885** (1,35 en S423) ; pas p99 2,77 ms.
- **P4** — non-régression, la mise à zéro coupée partout : **identique à S423 au chiffre près** — étages (ballottement, raccord, bande, B10), cycle (symétrie, résidu, itérations), bascules forcées (10 instants), B10 nu (pas 54) et en bande étroite (pas 55, volume exact), ballottement 0,447 (diagonale 0,454), colonnes 0,002, raccord 4,015, bande 1,318 mm, gestes compris. Suite du cœur 753 / 19 / 0 avertissement. Preuve §16 ; liste 4.19, feuille de route, index, file.
