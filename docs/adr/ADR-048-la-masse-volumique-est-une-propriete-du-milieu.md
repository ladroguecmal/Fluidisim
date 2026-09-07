# ADR-048 — La masse volumique de l'eau est une propriété du milieu, et le cas qui devait l'arbitrer est aveugle

- **Statut** : proposée
- **Session** : S58
- **Tranche** : angle mort **A103** *(sévérité 2)*, ouvert depuis S21, rappelé en fin de **trente-sept**
  sessions consécutives — sur **délégation explicite de l'utilisateur**, 2026-09-07
- **Corrige** : rien n'est réécrit. [`CAS-CANONIQUES`](../validation/CAS-CANONIQUES.md) note S21,
  [`00_INDEX`](../00_INDEX.md) et l'en-tête de `body.rs` reçoivent une note corrective datée : leur
  argument commun est faux, et il l'était dès son écriture.
- **Applique** : [`ADR-001`](ADR-001-les-quatre-couches.md) §2 — *toute question d'appartenance
  d'un phénomène à une couche se règle là*
- **Produit** : le balayage de [`RHO-EAU-S58`](../validation/RHO-EAU-S58.md) ; `Milieu` dans
  `water-core` ; deux valeurs nommées au lieu d'une constante
- **Clôt** : **A103**. **Ouvre** : **A180**

---

## 1. La question qu'on posait depuis trente-sept sessions

*« L'eau du projet est-elle douce (1000 kg/m³) ou de mer (1025) ? »* — posée en S21, jamais
tranchée, et présentée partout avec le même argument : le cas canonique **C10** exigerait 1000,
tandis que le domaine du jeu — la mer ouverte — appellerait 1025 ; l'écart, 2,5 % sur tout tirant
d'eau, vaudrait **deux fois et demie la tolérance de ±1 %** de ce cas.

L'argument rendait la question urgente et bloquée à la fois : une mesure existante semblait
contredire la physique du domaine, et aucune session ne pouvait lever la contradiction sans
trancher pour l'humain.

**L'argument est faux.** Balayage complet dans [`RHO-EAU-S58`](../validation/RHO-EAU-S58.md).

## 2. Ce que la mesure dit

Constante portée à 1025, recompilation, campagne `physics` relancée :

| Grandeur | ρ = 1000 | ρ = 1025 | déplacement | écart à la référence |
|---|---:|---:|---:|:--:|
| `C10-tirant` | 0,250000 m | 0,243902 m | −2,439 % | **0,000 % aux deux** |
| `C10-raideur` | 2452,500000 N/m | 2513,812500 N/m | +2,500 % | **0,000 % aux deux** |
| `C10-période` | 1,003033 s | 0,990726 s | −1,227 % | **0,000 % aux deux** |

Les trois références sont écrites *en fonction de* la constante — `(ρ_corps/ρ_eau)·H`, `ρ_eau·g·A`,
`2π√(ρ_corps·H/(ρ_eau·g))`. Elles suivent la mesure. **L'écart est structurellement nul, et la
tolérance de ±1 % porte sur cet écart, pas sur la valeur.** C10 ne peut pas voir ce dont il était
présenté comme le juge : c'est **A104** — *une référence tirée des paramètres ne prouve rien* —
énoncée dans l'en-tête du fichier même qui a commis la faute, et jamais reliée à A103.

Les deux hashs `check` sont inchangés. `RHO_EAU` n'apparaît dans aucun solveur : Saint-Venant
s'écrit en `h` et `u`, où `ρ` se simplifie. **La portée de cette décision est bornée aux forces sur
les corps** — elle ne touche ni B, ni W, ni δ, ni V, ni aucun cas de C01 à C09 et C22.

## 3. La décision

**Il n'existe aucune mesure qui départage 1000 et 1025, et il n'en existe pas de candidate.**
Le choix est conventionnel ; il se décide sur le domaine, et le domaine est écrit depuis ADR-001.

### D1 — La valeur du projet est celle de l'eau de mer : `1025 kg/m³`

Le système est celui d'un jeu de très grande échelle en **mer ouverte**. Une constante physique se
choisit sur le milieu qu'elle décrit, et rien d'autre ne s'y opposait : ce qui semblait s'y opposer
était un artefact de mesure.

### D2 — Elle cesse d'être une constante globale : c'est une propriété du **milieu**

Un monde de cette échelle contient de l'eau douce — rivières, lacs — et de l'eau de mer. **L'estuaire
est le lieu où les deux se rencontrent**, et il n'est pas une exception à traiter plus tard : c'est
un endroit où un même corps flottant change de tirant en avançant. Une constante globale rend ce
phénomène inexprimable, et le rend inexprimable *silencieusement* — la valeur unique ne se signale
jamais comme un choix.

`water-core` porte donc un type `Milieu`, avec deux valeurs nommées — `Milieu::MER` (1025) et
`Milieu::EAU_DOUCE` (1000) — et le défaut du projet est la mer. `RHO_EAU` disparaît en tant que
constante globale ; les corps reçoivent le milieu dans lequel ils flottent.

**Ce n'est pas une extension de portée** : aucun mélange, aucune stratification, aucun transport de
salinité n'est introduit ici. Le milieu est un paramètre, pas un champ. Le jour où la salinité
devient un champ transporté, `Milieu` est l'endroit où elle arrive, et ce sera un autre ADR.

### D3 — La référence de C10 reste construite avec la constante, et sa cécité est **écrite**

On ne corrige pas la référence en y mettant un littéral : le tirant du cube n'a **aucune source
indépendante** dans le corpus, et en inventer une serait fabriquer la mesure qui manque. Ce qui est
corrigé est l'**annonce** : le rapport ne doit plus laisser croire que C10 arbitre `ρ`. La limite
est portée par **A180** et par la note corrective de `CAS-CANONIQUES`.

Le seul contrôle réel — les deux assertions de `body.rs` qui comparent le tirant au littéral `0,25`
— est conservé et **transposé**, avec son nouveau littéral et la mention explicite qu'il découle de
D1 et non d'une mesure.

## 4. Ce qu'il faudrait pour inverser cette décision

Comme pour les cinq arbitrages d'[`ADR-027`](ADR-027-les-cinq-arbitrages-tranches.md), la décision
dit ce qui la renverserait :

| Inverser | Ce qu'il faudrait |
|---|---|
| **D1** (revenir à 1000 par défaut) | que le jeu se déroule en eaux intérieures, ou que les références d'un autre système — véhicules, personnage — soient déjà calibrées sur l'eau douce. **C'est un fait, pas un raisonnement** : cela relève de l'état réel du projet, hors de portée d'une session (`REPRISE.md` §5). |
| **D2** (revenir à une constante) | que le projet renonce aux eaux intérieures, ou qu'un coût de passage de paramètre soit mesuré et jugé rédhibitoire. Aucun n'est établi ; le coût actuel est de trois appels. |
| **D3** (mettre un littéral dans C10) | une source de tirant indépendante de la formule — une mesure, une table, un document d'intention. Il n'en existe aucune. |

## 5. Ce que cette décision ne dit pas

- **Elle ne valide pas C10.** Le cas reste vert, et il l'était déjà pour de mauvaises raisons sur
  trois de ses quatre assertions (**A104**, S21). D3 ne les rend pas discriminantes.
- **Elle ne déplace aucun résultat publié d'hydrodynamique.** Les 123 tests, les deux hashs, les
  cas C01 à C09 et C22 ne dépendent pas de `ρ`.
- **Elle ne dit rien de la masse ajoutée** (A26), qui reste la variante non exécutée de C10.
- **Elle ne clôt pas la question du domaine réel du jeu.** D1 choisit sur ce que le corpus déclare
  — mer ouverte, ADR-001 et les documents d'intention. Si le projet réel dit autre chose,
  c'est D1 qui cède, et le tableau du §4 dit comment.
