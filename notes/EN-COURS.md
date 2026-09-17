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

Session : S265 — en cours
Agent : Codex, GPT-6 ; fichiers, git, cargo, Python, accès web ; GPU à vérifier.
Entrée : verdict R6, « la surface a l'air trop rugueuse, entre les pic moyen et pic même plus petit
la surface doit etre plus lisse ». Aucun choix de vent explicite. Master propre 34dcf7d, copie unique.

Objectif : identifier et traiter la rugosité entre les crêtes sur le chemin de rendu,
sans assimiler le retour à un choix de vent ou déformer sans preuve le spectre physique.
Comparaison à vent fixé pour isoler le changement ; aucun vent représentatif adopté par défaut.

### Plan

- [x] **P1** — amorce et plan seuls.
- [x] **P2** — consigner le verdict ; examiner queue, modulation et réflexion ; définir le
  remède et les critères avant code (ADR si décision nouvelle).
- [x] **P3** — construire le remède borné dans l'hôte avec ses contrôles numériques ciblés.
- [>] **P4** — recevoir le chemin GPU, comparer avant/après aux mêmes poses, fournir R7.
- [ ] **P5** — rituel §6 : preuves, file, feuille de route, journal, jeton.

### Notes de reprise

Le travail visuel est explicitement demandé. Les bords ouverts J2 restent un lot de capacité
indépendant ; cette session vise une correction consommée par l'image, pas une nouvelle calibration
indéfinie. Si les preuves infirment le remède, publier ce résultat et replanifier avant construction.

P2 : miroir par pixel après coupure de pente confirmé dans le shader ; ADR-161 et protocole
REFLETS-S265 écrits. Fermeture gaussienne de seconds moments, limites CWM déclarées.

P3 : variante `--reflets-filtres`, covariance transférée, quadrature 3/5, oracle f64 et sondes GPU
construits. Tests hôte 18 réussis / 1 ignoré / 0 échec. Premier contrôle 5 m/s : pente 5,307e-5,
covariance 2,154e-7, déterminant min 0,530496. Matériel RTX 5070 Laptop DX12 confirmé.
P4 : campagne locale lancée (3 vents, témoin puis quadratures 3/5) dans `viewer/captures/s265`.
