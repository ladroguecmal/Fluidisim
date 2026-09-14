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

---

## Session en cours

Session : S235 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python et GPU local disponibles)
Entrée : « Continue », master propre à f0159ca, trois copies au même commit, jeton libre,
alimentation secteur (BatteryStatus 2).

**Objectif.** Scène J1 **représentative à plusieurs sources** dans l'hôte GPU : plusieurs
sillages (journal commun, montage S222) et plusieurs impacts à naissances décalées, composés et
admis par le cœur ; **visibilité** de la grille et des impacts, **exactitude au retour dans le
champ** ; coût GPU et CPU publiés (ADR-131 D3, alimentation selon A270).
**Faits de départ.** Plusieurs sillages d'un même journal et d'une même recette publient
4 096 modes au total (S222) : la cuisson GPU ne devrait pas croître avec eux, la préparation CPU
si. Le shader ne dessine qu'un impact. Un impact neuf vaut 47,4 % de π/7 (S222) : des naissances
simultanées feraient refuser le budget conjoint.
**Critères, déclarés avant construction.** Hauteur GPU contre cœur ≤ 3 mm (tolérance S201) sur la
scène multi-sources, intérieurs de grille sous leur borne ; composition du cœur sans refus aux
instants vérifiés, refus d'une variante dense publiés et non masqués ; retour dans le champ :
coefficients publiés et grille **identiques au bit** à un passage continu, sinon écart publié,
expliqué et confronté aux 3 mm ; coût avec et sans visibilité, pose visible et hors champ.
**Arrêt.** Scène vérifiée et mesurée. Si le budget ne tient pas sur la combinaison, le dire selon
ADR-131 D1 avec les techniques absentes, sans réduire la scène pour passer.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [ ] **P2** — lectures ciblées (profil radial, `mixed_compose`, `Timeline` multi-sources,
  S222/S223) ; déclarer la scène (positions, naissances, trajectoires) et prédire son admission
  par le cœur avant de construire.
- [ ] **P3** — impacts multiples : profils et centres en tableau au GPU, références CPU et
  composition du cœur à N impacts ; `--verify` vert.
- [ ] **P4** — sillages multiples dans un journal : grille, bornes, `verify_lattice` sur la scène.
- [ ] **P5** — visibilité : emprise et disques contre le cône de vue, cuisson et préparation CPU
  sautées hors champ ; retour dans le champ comparé à un passage continu.
- [ ] **P6** — coût : banc et cadence, scène mono et multi, visible et hors champ, alimentation
  publiée ; document de validation.
- [ ] **P7** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S234 — grille du sillage 0,426 ms (960×540, secteur), cadence 384 Hz, 9 tests de l'hôte ;
`code/` 413 réussis / 5 ignorés (S233, non modifié depuis).