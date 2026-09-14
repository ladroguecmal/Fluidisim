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

Session : S236 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python et GPU local disponibles)
Entrée : « Continue », master propre à 5744c8d, trois copies au même commit, jeton libre,
secteur. Base du cœur : 413 réussis, 5 ignorés.

**Objectif.** Que le **cœur** puisse composer et admettre la scène représentative S235.
**Ce que la lecture a changé avant le plan.** (1) `mixed::admits` n'accepte un point que dans le
domaine de **tous** les impacts : sur huit disques éloignés, l'intersection est presque vide, et la
scène est refusée par le domaine avant toute pente. L'image, elle, rend « hors emprise, B seul »
(ADR-126 règle 4). (2) Le balayage d'ADR-138 ne couvre que le disque de l'ancre : sûr pour
l'intersection, **faux pour l'union** — deux impacts récents hors de portée de l'ancre y
échapperaient. Donc un budget servi sur l'union demande d'abord une borne valide partout, avant
d'être plus serrée.
**Thèse à éprouver, calculée avant construction.** Sur l'union des domaines, un plancher
**bidimensionnel** — maximum sur des cellules du plan de `Σ F_i(distance minimale)` +
pression, raffiné par séparation — reste une borne sûre sans constante de Lipschitz (termes
décroissants) et peut admettre la scène. Si la pression globale suffit à la faire refuser malgré
une somme d'impacts exacte, le lot bascule vers la localisation de la pression (A261), sans rien
construire d'autre.
**Critères.** Borne ≥ pente réelle sur la scène et sur un contre-exemple construit ; mode
intersection **identique au bit** ; mode union : perturbation nulle hors de son emprise ; garantie
dans les deux sens d'ADR-128 conservée dans chaque mode ; coût d'annonce publié.
**Arrêt.** Scène S235 admise par le cœur et reçue, ou constat chiffré de ce qui manque. Aucun
seuil changé, aucune scène réduite.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [ ] **P2** — diagnostic sur la série S235 : plancher actuel ; ADR-138 étendu à l'union ;
  plancher 2D par séparation (pression globale, puis nulle hors emprise du sillage) ; bornes contre
  la pente réelle S235 ; contre-exemple du trou d'ADR-138 sur l'union.
- [ ] **P3** — décision et ADR-142 si P2 conclut ; sinon bascule déclarée.
- [ ] **P4** — cœur : mode de couverture de la composition, plancher sur l'union ; tests (bit
  intersection, zéro hors emprise, borne ≥ réelle, contre-exemple, deux sens d'ADR-128).
- [ ] **P5** — réception : série S235 admise par le cœur, marges réelles, coût du plancher,
  suite complète du cœur.
- [ ] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S235 — 49 refus / 161 instants, tous par majorant ; pente réelle ≤0,2154 ; plancher aux
naissances = pression 0,175–0,202 + impacts 0,28–0,37. `mixed_water.rs` : `admits` (intersection,
l. 158), `slope_floor_joint` (ancre, 8 cellules sur le rayon de l'ancre, l. 246).