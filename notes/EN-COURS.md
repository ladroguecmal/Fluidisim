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

Session : S417 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7b puis C7c**
([conception](../docs/validation/APIC-CARTE-S416.md) §1). Agent : Claude Code (Opus 5.5), au poste ; RTX 5070 Laptop.

**Ce que la session fait.** **C7b** : le corps cinématique de S393 sur la carte — mailles solides, faces imposées, images radiales
dans la reconstruction, particules repoussées — puis **B10 nu** (APIC seul, sans bande) sur la carte contre la référence. Ensuite
la **conception de C7c** (la zone des colonnes et le fond : ≈ 1 800 lignes du cœur, découpées), et son premier morceau si le temps
le permet.

**Critères, écrits avant.** (1) Les étages avec le corps, sur un état de B10 chauffé : étiquettes identiques (solides compris),
`φ` à 10⁻⁵ m, vitesses de grille à 10⁻⁴ m/s après projection et après imposition, positions à 10⁻⁵ m après le corps. (2) **B10
nu**, Fr = 2, D/dx = 8, quart de domaine : pincement **au même pas** que la référence (ou à un pas, publié), `φ` à **3 mm** de la
référence dans la bande de l'interface (|φ| < dx) jusqu'au pincement, air enfermé et cavité au pincement publiés des deux côtés.
(3) Coût par étage publié. (4) Zéro avertissement ; suite inchangée. **Arrêt** : un écart au-delà de l'arrondi se publie et
s'explique avant C7c.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la carte : le corps (paramètres, étiquettes solides, faces imposées, images radiales, particules repoussées) ; banc des étages sur B10 ; critère 1.
- [ ] **P3** — B10 nu sur la carte contre la référence : pincement, air enfermé, cavité, `φ` à l'interface, coût ; critères 2 et 3.
- [ ] **P4** — conception de C7c : lecture de `apic3d_columns.rs`, découpage, critères (preuve §7).
- [ ] **P5** — suite entière, zéro avertissement ; preuve ; liste, file, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise
