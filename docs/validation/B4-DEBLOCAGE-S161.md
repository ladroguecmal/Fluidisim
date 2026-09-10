# S161 — B4 : ce qui bloque, et ce qui ne bloque pas

2026-09-10. Instruit le déblocage de **B4**, désigné par ADR-109 comme seule voie ouverte et
attendu seul par **A214**. Protocole : [PLAN-BENCHMARK](PLAN-BENCHMARK.md) §B4.

## 1. Ce que B4 demande, volet par volet

> Même scène simulée deux fois : (a) perturbative B+W+δ, (b) **substitutive intégrale de
> référence, à résolution élevée**. Comparaison de la surface, des forces sur la coque, et de la
> perception en double aveugle.

| volet | état |
|---|---|
| comparaison de **surface** | demande la référence intégrale, rien d'autre |
| comparaison des **forces sur la coque** | demande la référence **et** un intégrateur de corps rigide, que le corpus range en construction (BILAN-S69 §5.3) |
| **perception en double aveugle** | demande des personnes — hors de portée d'une session (`REPRISE.md` §5) |
| contrôle du **terme source** (ajout S04, A50) | demande la référence, et un `S` que l'on sait tronquer |

**Un seul obstacle est commun aux quatre : la référence.** C'est bien elle qu'il faut instruire, et
c'est ce que le corpus dit depuis S158 sans l'avoir ouverte.

## 2. Ce que la référence doit être — et ce qu'elle ne peut pas être

Elle doit calculer **le champ total** dans son emprise, sans décomposition, pour qu'on puisse
comparer `simuler(A) + simuler(B)` à `simuler(A et B ensemble)`.

**Un solveur linéaire ne peut pas servir.** `dispersif.rs` — le milieu à dispersion exacte de S39,
instrument qui a rétracté ADR-042 — est **linéaire** : la superposition y est vraie par
construction, et l'écart mesuré serait nul quel que soit le rapport d'amplitude. Il mesurerait la
justesse de son intégration, pas la validité de l'additivité. C'est le piège le plus proche, parce
que ce fichier est le plus « référence » d'apparence du dépôt.

**Le candidat est donc un solveur non linéaire**, et le dépôt en a un : `shallow.rs`, Saint-Venant
1D en `f64`, bien équilibré, flux HLL, second ordre et RK2 optionnels, 1 165 lignes, reçu par le
mode `physics` du harnais sur six montages canoniques et par l'oracle croisé de S37. Sa
non-linéarité est celle qui compte ici : advection et terme `h·u`.

## 3. Ce que le dépôt possède, mis à plat

| pièce | nature | utilisable comme référence B4 ? |
|---|---|---|
| `shallow.rs` | Saint-Venant 1D, **non linéaire**, f64 | **oui**, en 1D |
| `delta.rs` | même modèle, f32, véhicule d'essai | comme témoin croisé, pas comme référence |
| `dispersif.rs` | milieu **linéaire** à dispersion exacte | **non** — additif par construction |
| `background.rs` | houle analytique de Gerstner | c'est le `B` à tester, pas la référence |
| oracle croisé (S37) | confronte `delta` et `shallow` | méthode déjà écrite, réutilisable |

**Le blocage n'est donc pas « il n'y a pas de référence ».** Il est que la référence disponible est
**1D et non dispersive**, quand `B` est une houle dispersive 2D. Ce que cela autorise et interdit
est la question de la section suivante — et c'est une question de conception, pas d'outillage.

## 4. Ce qu'un B4 en une dimension peut rendre, et ce qu'il ne peut pas

**Ce qu'il peut rendre : une infirmation.** L'additivité est une propriété mathématique ; il suffit
d'**un** contre-exemple pour la borner. Si `simuler(A) + simuler(B)` s'écarte de
`simuler(A et B)` au-delà d'un rapport d'amplitude donné, dans un modèle non linéaire d'eau, alors
la décomposition additive a un domaine de validité et ce rapport en donne une borne — dans ce
régime.

**Ce qu'il ne peut pas rendre : une validation.** L'absence de rupture en 1D ne dit rien du 2D
dispersif, ni des forces sur une coque, ni de la visibilité. Un banc qui ne peut qu'infirmer garde
sa valeur — c'est même la forme la plus honnête d'un juge — mais il ne remplace pas B4, qui doit
pouvoir infirmer **l'architecture**, pas seulement l'additivité d'un modèle.

**Le régime testé n'est pas quelconque.** Saint-Venant est non dispersif et vaut en eau peu
profonde ; c'est **exactement** le régime qu'ADR-001 §3.3 nomme pour justifier la bascule
substitutive : « rouleau de déferlement, piscine, coque qui émerge entièrement, cavité
traversante ». Le contre-exemple, s'il existe, tombera donc dans le domaine que la décision vise —
pas à côté.

**Ce qui reste hors d'atteinte, et qu'il faudra dire à chaque fois** : la houle de `B` est
dispersive en eau profonde ; un écart mesuré en eau peu profonde ne se transporte pas tel quel.

### La question posée, et la forme de la réponse

> À partir de quel rapport `A_δ / A_B` l'écart entre l'addition et la simulation conjointe dépasse
> un seuil donné, dans un modèle d'eau non linéaire ?

ADR-001 propose `max|δ| > 0,35·Hs` **comme valeur de départ, à calibrer**. La mesure la confrontera
directement. Et parce que « visiblement fausse » est perceptuel et que nous ne jugeons pas de
visibilité, la réponse sera rendue **à plusieurs seuils** — 1 %, 5 %, 10 %, 25 % — plutôt qu'en un
nombre unique qui cacherait le seuil choisi (**L242**).

*Une réserve de plan d'expérience, posée avant la mesure* : dans Saint-Venant, la non-linéarité
dépend aussi de l'amplitude absolue rapportée à la profondeur, `a/h0`. Le rapport `A_δ/A_B` seul ne
peut donc pas gouverner l'écart ; les deux doivent varier séparément, sans quoi le plan est
dégénéré comme celui de S157 (**L235**).

## 5. La mesure — `additivite_b4.rs`

Saint-Venant 1D non linéaire, 800 cellules de 0,25 m, ordre 2 + RK2, CFL 0,45. `A` est l'onde de
fond (gaussienne σ = 6 m en x = 60), `B` la perturbation locale (σ = 3 m en x = 100). Écart
mesuré : `‖(η_A + η_B) − η_AB‖ / ‖η_AB‖`, en norme L2 sur le domaine, à 4, 8 et 12 s.

### 5.1 Ce que le rapport `|δ|/Hs` gouverne : rien

À `A_δ/h0` égal, l'écart est le même quel que soit le rapport à l'onde de fond — alors que ce
rapport est le critère d'ADR-001 :

| `A_δ/h0` | `A_B/h0` | **`A_δ/A_B`** | écart à 8 s |
|---:|---:|---:|---:|
| 0,050 | 0,10 | **0,50** | 1,21e-2 |
| 0,050 | 0,50 | **0,10** | 1,23e-2 |
| 0,100 | 0,10 | **1,00** | 2,04e-2 |
| 0,100 | 0,50 | **0,20** | 2,44e-2 |
| 0,105 | 0,30 | **0,35** | 2,43e-2 |

Cinq fois moins de perturbation *relative*, le même écart. **Si le critère d'ADR-001 était le bon
paramètre, ces lignes ne se ressembleraient pas.**

### 5.2 Ce qui gouverne : l'amplitude rapportée à la profondeur

L'écart est proportionnel à `A_δ` sur **cinq décades** — vérifié jusqu'à `A_δ/h0 = 8e-6`, où le
rapport `écart/(A_δ/h0)` vaut encore 0,99 : ce n'est donc pas un plancher d'intégration.

| `A_δ/h0` | 0,025 | 0,050 | 0,100 | 0,175 | 0,250 | 0,375 | 0,500 |
|---|---|---|---|---|---|---|---|
| écart à 8 s | 6,2e-3 | 1,2e-2 | 2,4e-2 | 4,2e-2 | 5,9e-2 | 8,3e-2 | **1,0e-1** |

Soit `écart ≈ 0,24 · A_δ/h0` pour `A_B/h0 ≥ 0,10`, avec une légère sous-linéarité au-delà de 0,25.

**Le coefficient dépend de l'onde de fond, et pas comme on l'attendrait** : il vaut 0,24 pour
`A_B/h0 ≥ 0,10` mais **0,95** à `A_B/h0 = 0,02`. Le régime de très faible amplitude de fond n'est
donc pas décrit par la même constante, et cette session ne l'explique pas.

### 5.3 Contrôle : la part numérique

Le pas CFL dépend de la hauteur maximale, donc de la présence de `B` : les trois simulations d'un
même point n'avancent pas par la même suite de pas. À pas imposé, l'écart change de **0 à 2 %**.
La part numérique est donc négligeable, et le résultat n'est pas un artefact de pas variable.

## 6. Ce que cela dit d'ADR-001

Le critère proposé est `max|δ| > 0,35·Hs`. Appliqué à la mesure ci-dessus, **il autorise des
écarts qui varient d'un facteur dix selon l'état de mer** :

| état de mer | `Hs/h0` | seuil `0,35·Hs` en `A_δ/h0` | écart d'additivité au seuil |
|---|---:|---:|---:|
| houle faible sur fond de 10 m | 0,10 | 0,035 | **0,8 %** |
| mer formée en petit fond | 0,50 | 0,175 | **4,2 %** |
| mer démontée, `Hs` proche du fond | 1,00 | 0,350 | **8,4 %** |

Un critère de bascule est censé dire *quand la décomposition cesse d'être valide*. Celui-ci laisse
la validité dépendre de `Hs`, alors que la mesure montre qu'elle n'en dépend pas.

**Ce n'est pas une infirmation d'ADR-001** : la décomposition additive tient très bien — moins de
1 % d'écart tant que `A_δ/h0 ≤ 0,04`, et 10 % seulement quand la perturbation atteint la moitié de
la profondeur. **C'est une infirmation de son paramétrage.**

## 7. Portée, et ce que cela ne dit pas

- **1D, non dispersif, deux gaussiennes.** Le régime est celui qu'ADR-001 §3.3 nomme — petit fond,
  fortes amplitudes — mais `B` est en eau profonde et dispersive ; l'écart mesuré ne s'y transporte
  pas. En eau profonde, la profondeur ne joue plus : la variable y serait la cambrure, et cela
  reste à établir.
- **Ni forces sur coque, ni perception.** B4 juge l'architecture ; ce volet juge une propriété
  mathématique de l'addition. Il peut infirmer, il ne peut pas valider.
- **Le régime `A_B/h0 = 0,02` n'est pas expliqué**, seulement mesuré.

## 8. Décision

[ADR-111](../adr/ADR-111-le-critere-de-bascule-s-exprime-en-profondeur.md) : le critère de bascule
s'exprime en `max|δ|/h`, aucun seuil n'est gelé (ADR-108), et la décomposition n'est **pas**
infirmée — son paramétrage l'est. Note corrective datée portée à ADR-001 §3.3.

**B4 reste bloqué pour ses trois autres volets** : forces sur la coque (intégrateur de corps
rigide), perception en double aveugle (personnes), contrôle du terme source (ajout S04, A50). Ce
qui est débloqué est le premier, et il suffisait pour infirmer un paramétrage vieux de cent
soixante sessions.

## Note de portée du 2026-09-10 (S162)

Les mesures ci-dessus restent celles du montage S161. **Leur interprétation comme choix du
paramètre de bascule est remplacée par ADR-112** : ce montage additionne des évolutions
indépendantes, sans le résidu couplé ni les termes croisés de SPEC-004 §6.1. Il ne reçoit donc
pas le premier volet architectural de B4. Le contrôle du terme source A50 reste préalable.
Voir [ADR-112](../adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md) et
[ADDITIVITE-PROFONDE-S162](ADDITIVITE-PROFONDE-S162.md). Aucun chiffre historique n'est effacé.