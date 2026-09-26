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

Session : S400 — **en cours**. Deux parts. Demande de l'utilisateur (2026-09-27) : *« https://scottiefox.github.io/caustic-volume/ Il
s'agit d'un projet sur tree.js mais peut être il serait intéressant à analyser du point de vue des rendus ou autres, il faut avoir
en tête que les objectifs de cette référence ne sont pas les mêmes que mon projet. Continue »*. Agent : Claude Opus 5.5, session
cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Le site est bloqué par le réseau ; le dépôt public
(`scottiefox/caustic-volume`, MIT) est lu par git.

**Première part — la référence, rangée comme comparable** (`docs/COMPARABLES-EXTERNES.md`, une section datée) : ce que c'est, ce
que le code fait (lu, non exécuté), ce que nous faisons déjà, ce qui manque chez nous et qu'elle montre (8.5 : rayons de lumière
dans l'eau, particules), et ce qu'elle n'autorise pas — ses buts ne sont pas les nôtres : un jouet interactif réglé à l'œil, pas
une référence validée ; **aucun de ses nombres n'entre comme seuil**. Les pistes vont à la file, avec leur déclencheur.

**Seconde part — recevoir le raccord (C5b)** : les deux remèdes attribués en S399, critères de S399 **inchangés**. (E) **la zone
lit sa surface comme la bande** : `φ = z − (η + e(η))`, `e` le biais de lecture d'un réseau nominal au même niveau
(`lattice_read_error`, table calculée à la configuration) — la masse (`η`) reste exacte, la pression voit ce que la bande verrait ;
(F) **la séparation tenue du côté de la bande** : une particule que la séparation pousserait dans une colonne de la zone reste
dans sa colonne. **Prédictions** : (E) — repos ≤ 1 cm/s, et à 5 cm plus de migration ni de courant de surface ; (F) — la densité de
la dernière colonne de la bande à 8 ± 0,4 à 2,5 cm.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la référence : section des comparables ; pistes à la file (rendu, au poste).
- [x] **P3** — (E) et (F) dans `apic3d_columns.rs` ; le repos (critère 2 de S399) ; sans zone et toutes colonnes au bit.
- [ ] **P4** — `apic3d_raccord`, 30 s, 5 et 2,5 cm ; critère 4 de S399 ; attribution si manqué.
- [ ] **P5** — suite ; preuve (§6 de RACCORD-3D-S398) ; liste, file, A316.
- [ ] **P6** — rituel.

### Notes de reprise

- **P2** : section « CAUSTIC//VOLUME » des comparables (lu au dépôt, non exécuté). Même méthode que nos caustiques de S361 (Wyman,
  rapport d'aires). Ce qui manque chez nous : rayons de lumière et caustiques **dans** l'eau et sur les objets par un volume de
  tranches (8.5, 8.6) — à juger par un contrôle de conservation (moyenne d'une tranche = `E₀·exp(−K_d·z)`) et une photographie,
  **sans le gain artistique** `godRays` ; dispersion seulement si une photographie montre des franges ; occultation des photons par
  la coque. Pointeurs : file (rendu), liste 8.5.
- **P3** : (E) `columns_read` — table de 32 lectures du réseau nominal (`lattice_read_error`, noyau et rayon courants), faite à
  `enable_columns` et quand la mesure change le noyau ou le rayon ; appliquée **seulement s'il y a une bande** (une colonne hors du
  masque) : toutes colonnes, la zone lit `η` exactement. (F) dans `separate` : une particule poussée d'une colonne de la bande dans
  une colonne de la zone garde `x, y`. **Repos (critère 2 de S399) tenu : 1,970·10⁻⁵ m/s** (S399 : 1,007·10⁻² m/s) ; vu échouer :
  table coupée, 1,007·10⁻² m/s. Sans zone : ligne de S389 au chiffre près (1,9964 s, +1,01 %, +7,04 %, +0,21 %, 93,5) ; toutes
  colonnes : ligne de S398 (+0,02 %, +0,49 %, volume +1,39·10⁻¹⁰). Garde du test ramenée au critère (0,01).
