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

Session : S215 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A254** — le budget de pente est une somme sur les sources et la scène J1 en consomme
84 % avec deux. Mesurer d'abord ce que vaut réellement chaque majorant, puis décider.

### Entrée

Ligne `Session suivante` de S214. Maillons 0 à l'amorce (ADR-132 actée). A254 est une ligne de la
**file active J1** et elle conditionne la mutualisation de J1-bis : le chaînage est licite et la
règle des deux maillons est satisfaite.

### État réel constaté à l'amorce

`master` et trois copies de travail propres au même commit `70d2b36`. Jeton `libre`, battement
11:37. Machine : AMD Ryzen AI 7 350, RTX 5070 Laptop (DX12). Aucune dépendance nouvelle prévue.

### Le doute qui commande le plan, énoncé avant toute mesure

**A254 peut être en partie un artefact d'échantillonnage, et c'est la première chose à trancher.**
S214 a comparé le majorant conjoint (0,3477 à 0,3776) à une pente réelle de 0,0929 à 0,0352, et en
a tiré un pessimisme de 3,7 à 10,5. Mais cette pente réelle est le maximum sur **4 477 sondes d'une
grille de 1,3 m**, et le maximum de pente d'un impact radial est atteint en `r = 0,2062 λ`
(ADR-094), soit **0,69 m** du centre pour λ = 3,35 m. Une grille de 1,3 m ne peut pas le voir :
elle passe à côté du pic par construction. S214 l'a noté en réserve ; S215 doit le mesurer.

Deux issues, et il faut les nommer **avant** :

- **Le pessimisme survit à un échantillonnage fin** → A254 tient, et la question devient *d'où
  vient-il*. Piste nommée : `RadialImpact::slope_max()` est **figée à `t = birth`**
  (`slope_bound / SLOPE_L1_RATIO`), donc indépendante de l'âge, alors que `slope_envelope()` du
  sillage est recalculée à chaque instant — d'où un impact à 0,212607 **constant** sur 39 s. ADR-094
  a mesuré que le maximum ne décroît pas sur **2 s** ; rien n'a été mesuré sur 56 s.
- **Le pessimisme s'effondre** → A254 reçoit une note corrective datée, et le constat change de
  nature : le budget serait à 84 % parce que les pentes y sont réellement, ce qui est une nouvelle
  plus mauvaise mais pas la même, et qui n'appelle pas les mêmes remèdes.

### Thèse et critères, déclarés avant toute mesure

1. **Un majorant est un majorant** : sur tout l'échantillonnage fin et à tous les âges, la pente
   réelle de chaque champ reste **≤** son majorant publié. Un seul dépassement est un défaut de
   sûreté, pas une imprécision — c'est la seconde moitié de l'annonce d'ADR-094, et elle se vérifie
   sur 56 s et pas seulement sur 2 s.
2. **Pessimisme mesuré par champ**, impact et sillage séparément, à échantillonnage fin, à chaque
   âge : `majorant / pente réelle`. C'est lui, et non le rapport conjoint, qui dit où porter un
   remède.
3. **Occupation réelle** du budget recalculée avec ces chiffres, et comparée à celle de S214.
4. **Le refus à deux sources est exercé**, pas déduit : une scène avec une source de plus, et le
   refus rendu par le cœur, avec sa cause.
5. Aucun seuil de réussite présumé ; aucune décision prise avant P6. Chaque mesure publiée avec
   techniques présentes, absentes et domaine (ADR-131 D3), et **rang de passage** (L289).

**Prédiction écrite pour être contredite** : à échantillonnage fin, la pente réelle de l'impact
monte près de `slope_max()` (rapport < 1,5) et celle du sillage reste loin de son enveloppe
(rapport > 3) ; le pessimisme conjoint tombe donc sous 3, et A254 perd la moitié de sa force sans
disparaître — l'occupation de 84 %, elle, ne dépend d'aucun échantillonnage et ne bouge pas.

### Plan

- [x] **P1** — jeton, entrée, doute, thèse, critères et plan seuls.
- [x] **P2** — pente réelle de l'impact seul, échantillonnage fin (rayon et temps), contre `slope_max()` ; sûreté du majorant sur 56 s.
- [x] **P3** — pente réelle du sillage seul, échantillonnage fin dans l'emprise, contre `slope_envelope()` à chaque âge. *(Absorbée dans P2 : un seul programme mesure les deux champs ; les deux critères déclarés sont tenus.)*
- [x] **P3-bis** — *ajouté après P2, parce que le résultat l'exige* : la décroissance est-elle **universelle** dans la famille ? Mesurer plusieurs λ et E, et voir si le pessimisme s'effondre sur l'âge adimensionné `t/√(λ/g)` (échelle déjà employée par ADR-126).
- [x] **P4** — table `ρ(τ)` mesurée et **sûre** (minimum par intervalle, la fonction n'est pas monotone), vérifiée sur deux λ ; occupation recomposée, note corrective datée sur A254.
- [x] **P5** — scène à deux sources : exercer le refus tel qu'il est aujourd'hui, et le requalifier avec le majorant resserré.
- [x] **P6** — décider **et construire** : ADR, puis `slope_max_at(t)` dans `RadialImpact` — méthode neuve, `slope_max()` inchangée (ADR-094 : migrer le refus `Steepness` est une autre décision) — consommée par le budget de composition ; tests.
  *(P4 à P6 réécrits après P3-bis : la similitude étant établie, la décision n'est plus « laquelle des trois voies » mais « resserrer, et voici la loi ». Le plan d'origine reste lisible ci-dessus.)*
- [x] **P6-bis** — *scindé de P6 au constat de sa taille, avant de le commencer* : câbler le budget de composition sur `slope_max_at`, instant porté par `slope_floor` ; rattraper les attentes de tests que la frontière déplace.
- [x] **P7** — document de réception (en-tête ADR-131, rang de passage) ; suite complète `code/`.
- [x] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2+P3 : `code/water-core/examples/budget_pente_s215.rs`, release. Impact échantillonné sur 20 001
points de rayon (pas 2,6 mm = λ/1288, le pic d'ADR-094 à 0,691 m est traversé par 266 points) ;
sillage sur une grille de 0,25 m dans l'emprise (213 921 points, λ_min/8,4).
Fixture reproduite : λ 3,350000 m, E 164,000000 J — identiques à `viewer/src/scene.rs`.

**Critère 1 tenu, et c'était la question de sûreté** : `max(pente_réelle / majorant)` vaut
**0,999998** pour l'impact et **0,718847** pour le sillage. Aucun majorant n'est dépassé, sur 56 s
et non plus sur 2 s. L'annonce d'ADR-094 est **exactement atteinte à la naissance** (1,0000) : le
majorant de l'impact est serré à `t = birth` et à cet instant seulement.

**Prédiction contredite, et exactement inversée.** J'avais écrit : impact serré (< 1,5), sillage
lâche (> 3). C'est le contraire.

| âge (s) | impact réel | pessimisme | sillage réel | pessimisme |
|---:|---:|---:|---:|---:|
| 0 | 0,212607 | **1,0000** | 0 | — |
| 2 | 0,105857 | 2,0084 | 0,064453 | 1,6678 |
| 4 | 0,049488 | 4,2962 | 0,097096 | **1,3911** |
| 8 | 0,033053 | 6,4324 | 0,085892 | 1,6872 |
| 16 | 0,021147 | 10,0537 | 0,086914 | 1,8984 |
| 24 | 0,015711 | 13,5322 | 0,043166 | 3,6382 |
| 39 | 0,010525 | **20,1998** | 0,033093 | 4,7739 |
| 56 | 0,006986 | **30,4354** | — | — |

**Le mécanisme est un seul, et il est général.** Un majorant bâti comme une **somme de modules
modaux** — une norme L1 — est **invariant par dispersion** : après extinction de la source, chaque
mode ne fait que tourner, donc la somme des modules ne bouge plus. Le maximum **spatial**, lui,
décroît à mesure que les phases se décohèrent. Le pessimisme croît donc avec l'âge de la
perturbation, mécaniquement, pour **tout** champ de W. On le voit sur les deux : l'impact part de 1
et monte à 30 en 56 s ; le sillage reste sous 2 tant que la source force (16 s) puis monte à 4,8
une fois son enveloppe figée (majorant 0,1570 / 0,1568 / 0,1580 à 24 / 30 / 39 s, constant, pendant
que le champ décroît).

Ce n'est donc **ni** l'alignement d'A208, **ni** un choix d'emprise, **ni** l'additivité d'A254 :
c'est la **dispersion**. A254 avait raison sur le chiffre et se trompait de cause ; la note
corrective vient en P4.

**Conséquence immédiate sur le remède** : la voie « resserrer le majorant » vise `RadialImpact`
d'abord — sa `slope_max()` est figée à la naissance et se trompe d'un facteur 4 à 30 — et le
sillage ensuite, après extinction. `SLOPE_L1_RATIO` est déjà un **rapport mesuré** entre borne L1
et pente réelle (ADR-094, S141) : un `ratio(t)` est le même objet, une dimension plus riche. Reste
à savoir s'il est universel dans la famille — c'est P3-bis, et sans cela il n'y a pas d'ADR.

P3-bis : **la décroissance est universelle dans la famille.** Domaine ADR-126 pour chaque membre
(`R = 15,5 λ`, `A = 96 √(λ/g)`) ; le pessimisme relevé à onze âges adimensionnés `τ = t/√(λ/g)`
est **identique à trois décimales** pour λ = 0,5 · 1 · 3,35 · 8 m et pour E = 0,05 · 0,5 · 16,4 ·
164 · 4 000 J :

```
τ        0     0,5      1      2      4      8     16    27,4     48    66,8     96
ρ(τ)  1,000  8,713  1,063  1,273  2,409  4,823  7,071  10,066  15,228  20,227  30,243
```

Deux énergies à λ = 3,35 donnent la même colonne : le rapport est **indépendant de l'amplitude**,
comme la linéarité l'exige, et c'est le contrôle qui le dit plutôt qu'une supposition.
λ = 20 m à 100 kJ est **refusée à la construction** (`Steepness`) : la borne L1 y dépasse π/7 —
comportement attendu, pas un échec de mesure.
Recoupement : ρ(27,4) = 10,066 contre 10,0537 mesuré directement à 16 s ; ρ(66,8) = 20,227 contre
20,1998 à 39 s ; ρ(96) = 30,243 contre 30,4354 à 56 s. Les deux chemins se rejoignent.

**ρ n'est pas monotone** : elle vaut 8,713 à τ = 0,5 puis retombe à 1,063 à τ = 1 — la perturbation
s'aplatit puis se reforme. Une table sûre doit donc prendre le **minimum par intervalle**, pas
interpoler : sur `[0 ; 1]` le gain est nul, et c'est correct.

Raffinement local du maximum du sillage (±1 m au pas de 2 cm autour de l'argmax grossier) : gain
≤ **1,0034**, donc le balayage à 0,25 m avait déjà convergé. Le balayage global à 0,125 m lancé
d'abord a été **abandonné** — il coûte le carré du gain pour regarder partout ailleurs que le
maximum ; le raffinement local le remplace et le mesure. Impasse consignée pour ne pas la refaire.
L'argmax du sillage suit la source : [-23 ; 2,25] à 0,5 s, [-2,5 ; 4] à 8 s, [21,5 ; 4] à 16 s
(fin du forçage), puis se détache vers [47,5 ; 2,25] à 39 s.

P4 : **table `ρ(τ)` sûre construite et vérifiée hors de sa famille génératrice.**
Un intervalle par unité de `τ`, 0 à 96 (la borne d'âge d'ADR-126) ; dans chaque intervalle, le
**minimum** de ρ sur 21 sous-échantillons **et sur quatre λ génératrices** (0,5 · 1 · 3,35 · 8 m).
Minimum, parce que ρ n'est pas monotone et qu'un ρ trop grand ferait cesser le majorant d'en être
un ; sur quatre λ, parce que l'effondrement en τ est exact à trois décimales mais que l'âge transite
en **microsecondes entières**, et que cette quantification déplace le rapport de quelques ppm d'un λ
à l'autre.

**Garde `1e-4`, et elle a une provenance** (I-14) : avec la table brute, le majorant resserré était
dépassé de **3,0e-6** au pire sur trois λ hors famille (λ = 2 · 5 · 0,75) ; la garde vaut trente-trois
fois ce dépassement mesuré, et le banc qui la fixe est `budget_pente_s215 --table`.

**Contrôle final, quatre λ hors famille génératrice, 961 valeurs de τ chacune** :
`max(pente réelle / majorant resserré) = 0,999983`, atteint à `τ = 0` — **exactement le pire cas du
majorant d'origine** (0,999983). Le resserrement n'ajoute donc aucun risque : il hérite de la
précision de l'annonce d'ADR-094, et ne la dégrade pas.

Table retenue (96 entrées, f32) — 1,0000 / 1,0464 / 1,1979 / 1,4944 / 2,0209 / 2,9092 / 3,8726 /
4,4430 pour τ = 0 à 7, puis croissante jusqu'à **28,7706** à τ = 95. Fichier complet dans la sortie
de l'exemple.

**Occupation recomposée pour la scène J1** (impact né à t = 0, τ = âge / 0,5844 s) :

| âge (s) | τ | ρ(τ) | majorant impact resserré | + sillage | budget | part de π/7 |
|---:|---:|---:|---:|---:|---:|---:|
| 4 | 6,8 | 3,8726 | 0,054902 | 0,135072 | 0,189974 | **42,3 %** *(84 → 42)* |
| 16 | 27,4 | 9,8680 | 0,021546 | 0,164995 | 0,186541 | **41,6 %** *(84 → 42)* |
| 24 | 41,1 | 13,5293 | 0,015715 | 0,157044 | 0,172759 | **38,5 %** |
| 39 | 66,7 | 20,0288 | 0,010615 | 0,157981 | 0,168596 | **37,6 %** |

L'occupation tombe de **84 % à 38–42 %**, et la marge passe de 0,0712 à **0,259–0,262** : de quoi
admettre **un second sillage** de la même recette (0,165) là où il était refusé. Le sillage devient
alors le terme dominant — c'est lui qu'un resserrement ultérieur devra viser, et il faudra pour cela
la même mesure de similitude sur sa propre famille.

**A254 : la cause change, le chiffre tient.** La note corrective vient en P7 avec la réception.

P5 : **le refus est exercé, pas déduit.** Scène J1 à 16 s, un sillage prescrit et n impacts voisins
(positions [0;10], [3;10], [-3;10] — l'intersection des disques de 52 m reste large, donc c'est le
budget qui décide et non la géométrie), composée par `mixed_water::sample_world_batch` au point
[0;10] avec `max_slope = π/7`.

| impacts | budget | part de π/7 | verdict du cœur | budget resserré | part | admis |
|---:|---:|---:|---|---:|---:|---|
| 1 | 0,377603 | 84,1 % | `Ok(())` | 0,186540 | 41,6 % | oui |
| **2** | **0,590210** | **131,5 %** | **`Err(SlopeEnvelope)`** | 0,208085 | 46,4 % | oui |
| 3 | 0,802818 | 178,9 % | `Err(SlopeEnvelope)` | 0,229631 | 51,2 % | oui |

A254 est confirmée dans les termes exacts où elle avait été écrite : **la deuxième source refuse
l'image**, et le verdict rendu est bien `SlopeEnvelope` — le majorant, pas la raideur. Avec le
majorant resserré par la table, trois impacts et un sillage n'occupent que 51 % : la marge redevient
celle d'une scène, pas d'un cas limite.

P6 : **ADR-133 actée**, et la moitié bibliothèque construite.
Avant d'acter, une vérification qui pouvait tout invalider : **la profondeur**. La relation de
dispersion en dépend, et toutes les mesures étaient à 20 m. Résultat : colonnes **identiques** de
20 m à 4 m, et à 2 m et moins `RadialImpact::new` **refuse le champ lui-même** (`depth ≤ π/lo`).
Le domaine où la similitude vaut est exactement celui où le champ existe — il n'y a donc pas de
réserve de profondeur à porter dans l'ADR. Anisotropie : refusée à la construction elle aussi.

Code : `radial_impact::RHO_DISPERSION` (96 f32, provenance et domaine en doc) et
`RadialImpact::slope_max_at(t)`. Hors du domaine mesuré — avant la naissance, au-delà de τ = 96 —
la méthode rend `slope_max()` **telle quelle** : pas d'extrapolation, donc pas de majorant qui
cesse d'en être un. `slope_max()` et le refus `Steepness` sont inchangés (ADR-094 : les migrer est
une autre décision).
Test `slope_max_at_is_a_tighter_bound_at_every_instant_s215`, sur quatre λ dont **trois hors famille
génératrice** (0,75 · 2 · 5 m) : égalité au bit à la naissance, égalité au bit hors domaine dans les
deux sens, **jamais dépassé** sur 481 instants × 2 001 rayons, et resserrement > 4 au-delà de τ = 8.
Passe en 44 s.

P6-bis : **le budget est câblé, et le refus a disparu.**
Quatre sites : `composition::compose`, `mixed_water::sample_world_batch`, `mixed_water::slope_floor`
(qui prend désormais l'instant) et `mixed_differential`. Dans les trois premiers, seul `budget`
change : `bound` / `envelope` restent sur `slope_max()`, parce qu'ils alimentent `steepness`, qui
est un **bit publié** — et ADR-133 n'en change aucun. Vérifié : `d_eta_m = 0,000000000` dans l'hôte,
inchangé depuis S214.

**Trois attentes de tests seulement ont bougé, toutes portant sur le budget lui-même** — pas de
cascade : le plancher à l'instant (`slope_floor_refuses_every_batch_below_it`), le plancher sans
pression (S205), et la limite « juste sous le budget » du test de la mer S201, qui à 3 s devait
descendre avec lui : prendre `slope_max()` n'y éprouvait plus rien, puisqu'il est désormais
**au-dessus** du budget.

Démonstration finale, même scène qu'en P5 :

| impacts | budget avant | verdict avant | budget après | part | verdict après |
|---:|---:|---|---:|---:|---|
| 1 | 0,377603 | `Ok(())` | 0,186540 | 41,6 % | `Ok(())` |
| **2** | **0,590210** | **`Err(SlopeEnvelope)`** | **0,208086** | **46,4 %** | **`Ok(())`** |
| 3 | 0,802818 | `Err(SlopeEnvelope)` | 0,229631 | 51,2 % | `Ok(())` |

Le budget rendu par la bibliothèque et celui calculé à la main depuis la table coïncident à 1e-6 :
le câblage fait bien ce que la table dit.
Hôte : occupation 42,3 / 39,8 / 41,6 / 38,5 / 37,6 % aux cinq âges (contre 77,5 à 84,1 en S214) ;
`VERIFY` **inchangé** (7,2271e-5 m à 16 s, 7,4625e-5 à 39 s) ; GPU eau 1,904 / 4,187 ms inchangé ;
`--smoke` 120 images. Suite `code/` : **355 réussis (257+4+1+93), 5 ignorés** — un de plus qu'en
S214, celui d'ADR-133.

P7 : [BUDGET-PENTE-S215](../docs/validation/BUDGET-PENTE-S215.md) — en-tête ADR-131 D3 (et la
mention que le rang de passage ne s'applique pas : aucune mesure de temps ici) ; §1 l'issue du
doute d'échantillonnage ; §2 les deux majorants et le mécanisme ; §3 la similitude et ses trois
indépendances ; §4 la table et sa sûreté ; §5 le refus exercé puis levé ; §6 les contrôles.
Suite nommée : **la famille du sillage**, qui domine désormais le budget.

P8 : rituel §6 exécuté. Journal S215 ; **A255** (sévérité 2) ; suivis **A254** (traitée pour moitié,
cause corrigée : dispersion et non emprise) et **A208** (non close, non dominante) ; **L290, L291,
L292**. Index, README, REPRISE (§3 « 133 décisions », §4, file active, jeton), feuille de route
(J1, J1-bis, travaux nécessaires, ligne mutualisation désormais conditionnée par A255), file
plurielle de QUESTIONS-OUVERTES.
**Invariants relus** — **I-18** en premier, puisque c'est lui que le sujet touche : ce qui est
comparé à `max_slope` doit être une pente réelle, et ADR-133 le sert **mieux** qu'avant, le majorant
étant désormais proche de la pente réelle à chaque instant au lieu de celle de la naissance ; I-14
(la table cite son banc, la garde cite le dépassement qu'elle couvre) ; I-06 (aucune allocation :
`RHO_DISPERSION` est une constante) ; I-04 et I-08 inchangés. Aucun devenu faux, aucun amendé, aucun
ADR réécrit.
**Règle des deux maillons** : compteur **0**, et cette fois par le code — `outils/velocite.sh`
relancé donne **W = S215** (contre S213 avant la session), B = S211, δ = S202, V = jamais.
**Recommandation portée** : la suite S216 nomme A255, ligne de la file J1, et la couche W.
Décomptes vérifiés : 133 fichiers dans `docs/adr`, 292 leçons, 255 angles.
Jeton libre, battement 13:09. Copies de travail avancées sur master après ce commit.
