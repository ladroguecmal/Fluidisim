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

Session : S236 — terminée
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
- [x] **P2** — diagnostic sur la série S235 : plancher actuel ; ADR-138 étendu à l'union ;
  plancher 2D par séparation (pression globale, puis nulle hors emprise du sillage) ; bornes contre
  la pente réelle S235 ; contre-exemple du trou d'ADR-138 sur l'union.
- [x] **P3** — décision et ADR-142 si P2 conclut ; sinon bascule déclarée.
- [x] **P4a** — cœur : `admits_union`, `slope_floor_union` (séparation, pool hôte, pression
  locale ADR-137 sur cellules critiques), tests de la borne (≥ réelle, contre-exemple du trou
  d'ADR-138 sur l'union, pool épuisé, chemin rapide).
- [x] **P4b** — cœur : `sample_world_batch_union` ; tests (zéro hors emprise contre somme à la
  main, intersection inchangée, deux sens d'ADR-128 au seuil) ; suite complète.
- [x] **P5** — réception : série S235 admise par le cœur, marges réelles, coût du plancher,
  suite complète du cœur.
- [x] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S235 — 49 refus / 161 instants, tous par majorant ; pente réelle ≤0,2154 ; plancher aux
naissances = pression 0,175–0,202 + impacts 0,28–0,37. `mixed_water.rs` : `admits` (intersection,
l. 158), `slope_floor_joint` (ancre, 8 cellules sur le rayon de l'ancre, l. 246).

P2 (`--bornes-union`, logs `bornes_s236.log` puis `bornes_s236b.log`, CPU, viewer seul) :
champs reconstruits comme `Prepared::build` (écart mono 0). Refus sur 161 instants et pire/π/7 :
**actuel 49 (1,2510)** ; ADR-138 étendu à l'union 40 (1,1904) — plus serré que le cœur car il
annule les termes au-delà de leur domaine, que `slope_floor_joint` ajoute ; **séparation 2D,
pression globale : 32 (1,1528)**, 64–244 cellules, ≤2,7 ms ; pression nulle hors emprise du
sillage : identique (tous les impacts sont dans l'emprise) ; **valeur atteinte par G : 31
(1,1515)** → même une somme d'impacts exacte en position refuse : les termes eux-mêmes sont trop
larges (pression 0,19 contre ≈0,07 réelle ; impacts anciens ≈0,11 au centre du neuf).
**Certification** (séparation arrêtée dès que la plus grande cellule ≤ π/7 ; sur cellule ≤1 m de
demi-côté au-dessus du seuil, pression = min(globale, ADR-137 locale)) : **32 / 32 instants
certifiés**, majorants finaux 0,9835–0,9994 π/7, 92–128 cellules, 3–15 appels locaux, **3,0–
13,4 ms**. Donc scène S235 admissible aux 161 instants : 129 par séparation à pression globale,
32 avec pression locale sur cellules critiques. Réserve : ADR-137 = réception numérique, pas
certificat f32 (**A258** entre dans le lot).
Contre-exemple mal construit : ancre (0,0) neuve, deux impacts d'1 s confondus à 100 m → réelle
0,1225 < plancher ADR-138 0,2539 (globaux 0,2126 / 0,2032 / 0,2032, bornes lâches à 1 s). Le trou
de l'union demande deux impacts **nés au même instant** que l'ancre, ancre plus énergique — à
construire en test du cœur (P4).

P3 : ADR-142 (mode union ajouté, plancher certifié par séparation, pression locale ADR-137 sur
cellules critiques ≤1 m, arrêt à 1 cm ou pool plein, annonce = refus au même `max_slope`).
P4a : `code/water-core/src/mixed_union.rs` (rattaché à `mixed_water` comme `mixed_differential`),
`admits_union`, `slope_floor_union`, `sample_world_batch_union`, `FloorCell`, `UnionFloor`,
constantes `FLOOR_LOCAL_HALF` 1 m / `FLOOR_MIN_HALF` 1 cm / `FLOOR_INITIAL_SPLIT` 8. Quatre tests
dans `tests_mixed_water.rs` : **trou d'ADR-138 sur l'union reproduit** — ancre ×1,3 d'énergie en
(0,0), paire confondue à 100 m née au même instant : pente réelle **0,4252**, plancher ADR-138
**0,2837** ; plancher union à 0,99·réelle non certifié (0,4252, 144 cellules), à 1,05·paire
certifié (0,4252, 64 cellules). Échelle de seuils sur quatre impacts N64 recouvrants (réelle
0,00118, somme 0,00461, maximum de G 0,00225) : certifié ⟹ réelle ≤ plancher ≤ seuil ; un
certificat par séparation exigé (seuils 2 et 3 × réelle ajoutés après un premier passage où seul
le chemin rapide certifiait). Pool de 64 cellules : non certifié, borne ≥ réelle ; pool vide :
refus sans cellule. Chemin rapide identique au bit à `slope_floor` (un impact).
**Garde de contrat S143 échouée, puis satisfaite** : la sélection des cellules critiques était une
comparaison non déclarée à `max_slope`. Les trois sites du fichier réécrits sous la forme
`budget > max_slope` et inscrits dans `SITES_CONNUS` (11 sites). Suite complète du cœur :
**417 réussis, 5 ignorés**.

P4b : deux tests de la requête sur le montage `fixture` (B, un impact N64 de 16 m, pression
d'emprise [−8 ; 12]²). (1) Quatre points — disque et emprise, disque seul, emprise seule, aucun :
hauteur de l'union **au bit** d'une somme à la main des seules perturbations couvrantes ; la requête
intersection sert le premier et refuse les trois autres. Premier passage en échec **du test** :
point (11,9 ; 11,9) non représentable au 1/2048 m, somme à la main au point brut au lieu du point
local quantifié (piège S214) ; corrigé en sommant au point local et en prenant 11,875.
(2) Échelle de 25 seuils de 0,9·réelle à 1,01·somme : plancher certifié ⟹ requête `Ok` et
`UnionFloor` rendu identique à l'annonce, réelle ≤ plancher ≤ seuil ; non certifié ⟹ `Slope` ou
`SlopeEnvelope` ; les deux issues présentes, **pression locale sollicitée**. Suite complète du
cœur : **419 réussis, 5 ignorés** ; hôte construit.

P5 (`--multi --union-coeur`, log `union_s236.log`, secteur 00:26–00:34) : **161 / 161 planchers
certifiés, 0 requête refusée** sur 6 988 sondes ; pire plancher 0,9995 π/7 ; ≤128 cellules ; 355
appels locaux ; plancher médiane 0,001 ms, max **12,0 ms** ; requête 6 988 points médiane
**1 421 ms**, max 1 640 ms (≈0,2 ms/point : pression 4 096 modes sur CPU, pas le mode union) ;
**écart à `FrameData::references` : η < 1e-9 m, pente 3e-8**. Document :
[ADMISSION-UNION-S236](../docs/validation/ADMISSION-UNION-S236.md).