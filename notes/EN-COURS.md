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

Session : S403 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue »*. Suite proposée par S402 : **C8d**, la décision
du rang 4 et la famine. Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni
Godot. Branche `claude/eager-volta-lf0kw3` (S402), la plus avancée.

**Thèse.** L'ordonnanceur (`scheduler.rs`) connaît le rang 1 (S351) : quand le budget ne tient pas tous les vivants, le focal est
servi entier et les autres rétrécissent ; ce que le rang 1 ne sauve pas reste **affamé sans le dire**. Le rang 4 d'ADR-012 §4 vient
après : un non-focal qui l'a **déclaré** — son coût un niveau plus bas, et ce que son image y perdrait (l'écart d'un aller-retour
de son contenu, ADR-210 D2) — descend d'un niveau (le transfert d'ADR-210) ; l'ordonnanceur descend d'abord celui qui **perd le
moins par milliseconde rendue**, un à la fois, jusqu'à ce que la famine cesse. Ce qui reste affamé est **déclaré** : l'issue est
le rang 5, le repli sur W, à l'hôte. Descente immédiate ; remontée engagée une seconde, un domaine par seconde, celui qui perd le
plus d'abord (ADR-012 §5). Les rangs 2 (pas d'embruns dans δ) et 3 (la fréquence) n'existent pas : le rang 4 suit le rang 1.
**L'API** : `Bid` ne change pas — l'hôte `viewer/` le construit en quatre endroits, et ce conteneur ne peut pas le compiler ; la
déclaration passe par `declare_coarsen`, et `Grant` dit le niveau accordé.

**Critères, écrits avant.** (1) Sans déclaration de rang 4, l'ordonnanceur de S351 **au bit** : ses essais, et l'empreinte
`6aebff024c734fc9` de l'exemple S278. (2) Le rang 4 ne sert qu'après le rang 1 à son minimum : un cas que le rang 1 résout ne
descend personne. (3) Il descend le non-focal qui perd le moins par milliseconde rendue ; jamais le focal ; budget jamais dépassé.
(4) La famine a une **issue déclarée** : ce que ni le rang 1 ni le rang 4 ne servent est rendu comme affamé. (5) Descente
immédiate ; remontée au plus un domaine par seconde, une seconde au moins après sa descente, le plus coûteux en perte d'abord.
(6) Banc : trois domaines δ réels — un focal, une bosse, une source — sous un budget qui se resserre puis revient ; budget tenu à
chaque pas ; l'ordonnanceur descend la bosse, pas la source ; le prix visuel publié contre un témoin qui ignore le contenu
(prédiction, d'après S402 : ≈ 2 mm contre ≈ 15 mm) ; sauts ≤ 3 mm aux passages. (7) Suite entière, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le rang 4 dans `scheduler.rs` : `Coarsen`, `declare_coarsen`, niveau des vivants, choix par la perte, remontée,
  affamés déclarés ; essais des critères 1 à 5.
- [x] **P3** — le banc `delta3d_famine` : trois domaines, budget en quatre phases, transferts d'ADR-210 aux passages ; témoin sans
  pertes déclarées.
- [x] **P4** — les calculs du banc ; critère 6.
- [x] **P5** — suite entière, zéro avertissement ; critère 7.
- [x] **P6** — preuve `FAMINE-S403` ; liste (9.8, 9.9), file, feuille de route, index ; note datée d'ADR-012.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — `scheduler.rs` : `Coarsen { cost_ms, loss_m }`, `declare_coarsen` (refus `Error::Coarsen`), `Grant::level`, `starved()` ;
  vivants avec niveau et instant de descente ; `update_levels` avant le rang 1 : un descendu qui ne déclare plus remonte, le focal
  jamais descendu ; **descente** tant que le rang 1 à son minimum affame, le déclaré de moindre perte par ms rendue d'abord (en
  croix, égalité par identité) ; **remontée** une par seconde, une seconde au moins après la descente, la plus forte perte d'abord,
  seulement si personne n'est affamé. Coûts au niveau : la loi du rang 1 sur le coût déclaré (`scaled_cost` = `cost_for` au bit à
  son niveau). `Bid` inchangé (l'hôte `viewer/` le construit, et ne se compile pas ici). **Critère 1 tenu** : les 28 essais d'avant
  (un littéral de `Grant` complété de `level: 0`), empreinte S278 **`6aebff024c734fc9`**. Critères 2 à 5 : six essais `_s403`
  (le rang 1 suffit : personne ne descend ; la bosse descend, pertes échangées la source ; famine à 0,6 ms : un affamé déclaré ;
  remontée à 1 s puis 2 s, la plus forte perte d'abord ; descente immédiate ; qui ne déclare plus remonte ; refus). 34 tenus.
- **P3** — `examples/delta3d_famine.rs` : trois sites de 16 × 12 m (64 × 48 × 12 à 25 cm, jumeau 32 × 24 × 6 à 50 cm), F (0) focal
  source, B (1) source, A (2) bosse ; référence à 25 cm sans famine ; coûts par la loi de production ramenée à la maille (0,440 /
  0,134 ms) ; budget 2 / 1,05 / 0,6 / 2 ms (0–2, 2–4, 4–5,5, 5,5–8 s) ; perte déclarée = écart d'un aller-retour de la surface
  présente ; l'hôte : transfert d'ADR-210 aux changements de niveau, rang 5 (image effacée en 0,5 s, renaissance au repos) pour un
  affamé ; `TEMOIN=1`, pertes nulles. ≈ 34 s de calcul par seconde simulée, deux calculs en parallèle.
- **P4** — résultats (écart max de l'image à la référence par phase : large / famine 1,05 / sévère 0,6 / retour) : **ordonnanceur**
  — A, la bosse, **descendue en famine : 1,36 mm**, puis affamée (rang 5, 75 pas) 17,8 mm, renaît au repos 20,5 mm ; B, la source :
  0 / 0 / 13,2 (descendue en sévère) / 15,0 mm (remontée) ; F : 0 partout. **Témoin** (pertes nulles, l'identité départage) — B
  **descendue en famine : 14,0 mm**, 16,4 / 17,0 ; A : 0 / 0 / 17,8 / 20,5. **Critère 6 tenu** : le contenu divise par dix l'écart de
  la phase où le rang 4 décide (prédiction ≈ 2 contre ≈ 15 mm) ; sauts ≤ 2,48 mm ; accordé ≤ 0,965 du budget ; focal intact.
  **Constats** : (1) en famine sévère, la victime du rang 5 suit l'ordre du sac à dos (`P/C`, puis l'identité) — ici A, la bosse —,
  pas le contenu ; (2) la descente de B y est inutile (A et B descendus dépassent 0,6 ms) et change seulement qui est nourri ; (3) un
  domaine détruit ne retrouve pas son contenu (A : 20,5 mm après le retour) — le prix du rang 5, publié.
- **P5** — suite entière : **718 réussis**, 19 ignorés, zéro avertissement (712 + 6 essais `_s403`).
- **P6** — preuve `docs/validation/FAMINE-S403.md` (Reproduire au commit `342bdac1`) ; liste 9.8 et 9.9 complétées (aucun point ne
  change de case) ; file (campagne, 84 mots : C8 rang 4 et famine ; reste le pool sur la carte et la victime du rang 5 par le
  contenu) ; feuille de route §3 ter (S403 ; suivante dans le cloud : **C8e**, l'épars et les niveaux sous le pas couplé — C6 attend
  le raccord) ; index (carte B, liste des preuves) ; note datée d'ADR-012 (quatre lectures).
