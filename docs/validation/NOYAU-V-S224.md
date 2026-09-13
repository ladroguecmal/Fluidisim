# Le noyau de la couche V — S224, 2026-09-13

**La couche V existe.** Premier module du graphe hydraulique d'ADR-010, reçu par le cas canonique
**C12**. Zéro module en 223 sessions ; un, désormais.

Aucun ADR nouveau : ADR-010 est actée depuis S01 et complète. Cette session la **construit**.

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : arithmétique entière en millilitres, pas fixe de 100 ms, quantification
  à report de reste, normalisation par arrondi cumulatif, limiteur d'arrivée. Un seul passage
  explicite par pas.
- **Techniques absentes** : itérations de Gauss-Seidel (mesurées inutiles ici, §5) ; réseau fermé
  sous pression, pompes, matériaux poreux, pluie, absorption — tous **hors périmètre par décision**
  d'ADR-010 §4 pour le premier ; non construits pour les autres. Aucun couplage à δ, ni à B/W.
- **Domaine de validité** : contenants à forme prismatique ou tabulée (64 entrées), orifices et
  déversoirs rectangulaires, réseaux **ouverts**, `g_eff` injectée constante sur le pas. Chaînes
  jusqu'à trois nœuds mesurées ; au-delà, non éprouvé.
- **Rang de passage** : sans objet — aucune mesure de temps d'exécution dans cette session.

## 1. Ce que le module est

`code/water-core/src/hydro_network.rs`. Un nœud est un **contenant** dont l'état tient dans un
`i64` de millilitres ; une arête est une **ouverture** portant une loi de débit et un reste
fractionnaire.

```rust
HydroNode { volume_ml: i64, capacity_ml: i64, floor_um: i64, shape: u16 }
Opening   { from, to: Option<u16>, flow: Flow, sill_um, discharge, residue_nl }
Flow      { Orifice { area_mm2 }, Weir { width_mm } }
```

Les invariants ont dicté la forme, et pas après coup :

- **I-03** — V est déterministe bit à bit. L'ordre de parcours est celui du tableau fourni, jamais
  une adresse ; la normalisation elle-même est reproductible sans stocker de reste (§4).
- **I-10** — le serveur exécute V, en entier et à 10 Hz. Aucun flottant ne porte jamais l'état.
- **I-06** — aucune allocation : nœuds, arêtes, formes et scratch appartiennent à l'appelant.
- **I-07** — `g_eff` est injectée, pas lue dans une constante.

## 2. C12, reçu

**Montage.** Réservoir de 1 m² de section, 1 m d'eau, orifice de 10 cm² à arête vive au fond.
**Référence analytique**, à charge variable : `t = (A/(C_d·a))·√(2h₀/g) = 728 s`.

> **Mesure : 727,4 s — écart 0,0824 %**, contre les ±3 % que le cas exige.

Masse conservée exactement, volumes dans leurs bornes à chaque pas, réservoir vide à la fin.

La valeur à charge constante vaudrait la moitié : c'est l'erreur d'un facteur deux qu'ADR-010 §3
portait et que C12 a corrigée en S03. Le module la reproduit du bon côté.

## 3. Deux défauts trouvés en construisant, et ce qu'ils enseignent

**L'interpolation de hauteur tronquait, et cela arrêtait la vidange.** À un millilitre dans un
réservoir de 1 m², la hauteur vaut un micromètre ; l'interpolation entière rendait
`999999/1000000 = 0`. Charge nulle, débit nul, contenant qui ne se vide plus — un symptôme qui
ressemble à une erreur de physique et n'en est pas une. **Arrondi au plus proche** : le plancher de
représentation demeure, la hauteur étant entière, mais il vaut une unité au lieu de deux.

**La normalisation en nanolitres empêchait la quantification d'aboutir.** ADR-010 §4 demande que
plusieurs arêtes vidant le même nœud soient réduites « dans la même proportion ». Faite **avant**
la quantification, elle donne à chaque arête une part sous le millilitre, qui s'arrondit à zéro :
un nœud de 2 ml avec trois fuites gardait 2 ml indéfiniment, avec de la charge. La normalisation
est passée **après** quantification et **en millilitres**, par **arrondi cumulatif** — la part de
l'arête `k` est la différence des sommes proportionnelles arrondies jusqu'à `k` et jusqu'à `k−1`.
Les parts somment alors **exactement** au volume disponible, chacune est à moins d'un millilitre de
sa valeur proportionnelle, et l'ordre du tableau suffit à la reproduire : aucun reste à stocker,
donc aucune source de divergence entre plateformes (I-03).

## 4. Le report de reste, et pourquoi ADR-010 l'exige

Prédiction écrite avant mesure : le report serait nécessaire, et le pas de 100 ms mordrait près de
la fin. **Confirmé, avec son chiffre** : le test `without_the_residue_carry_the_drain_stalls_s224`
jette le reste à chaque pas, et **la vidange s'arrête à 13 ml**.

C'est exactement le seuil que la loi donne : le transfert d'un pas passe sous le millilitre quand
`Q·dt < 10⁻⁶ m³`, soit `h < 13,3 µm`, soit les derniers 13 ml d'un réservoir de 1 m².

**La moitié de ma prédiction était fausse, et P2 l'avait déjà corrigée** : sans report, la masse
reste conservée. Un transfert entier retiré d'un nœud et ajouté à l'autre ne peut rien perdre. Ce
que la troncature dégrade est le **débit**, donc le temps — jusqu'à l'arrêt complet quand le débit
d'un pas tombe sous l'unité.

## 5. Le déversoir, et la question Gauss-Seidel

ADR-010 §3 donne **deux** lois. Le déversoir applique `Q = (2/3)·C_d·b·√(2g)·H^{3/2}`, et le test
les sépare **par leur exposant** plutôt qu'en relisant la formule — c'est ce qui attrape une loi
recopiée dans la mauvaise branche :

| loi | `Q(2 m)/Q(1 m)` mesuré | attendu |
|---|---:|---:|
| déversoir | **2,8284** | `2^{3/2}` = 2,8284 |
| orifice | 1,4151 | `√2` = 1,4142 |

**ADR-010 §4 écrit que « 2 à 4 itérations de Gauss-Seidel par pas suffisent pour un réseau
ouvert ». Ce module n'en fait aucune** — un seul passage explicite depuis l'état du début de pas.
Plutôt que de supposer que c'est assez, la chaîne de trois contenants est intégrée à 100 ms puis à
**1 ms**, sur 60 s :

| | nœud 0 | nœud 1 | nœud 2 |
|---|---:|---:|---:|
| 100 ms, 600 pas | 532 351 | 300 688 | 166 961 |
| 1 ms, 60 000 pas | 532 392 | 300 630 | 166 978 |

**Écart maximal 0,0058 % de la capacité.** Un seul passage explicite suffit donc à 10 Hz sur cette
configuration. **Ce n'est pas une preuve générale** : un réseau plus raide — grandes sections,
faibles volumes — n'est pas couvert, et le réseau **fermé sous pression** reste hors de portée par
décision d'ADR-010 §4.

## 6. Ce qui est reçu, et sous quelle forme

Neuf tests, tous dans `tests_hydro_network.rs` :

- **C12** — temps de vidange contre la référence analytique ;
- **conservation exacte** sur 20 000 pas d'un réseau fermé, volumes dans leurs bornes à chaque pas ;
- **non-négativité** d'un nœud presque vide alimentant trois fuites concurrentes ;
- **capacité aval** jamais dépassée, receveur exactement plein ;
- **déterminisme** (I-03) : deux exécutions de 500 pas, traces identiques, résidus compris ;
- **refus atomiques** nommés — `Domain` (pas ou gravité inutilisables), `Capacity` (indice hors
  tranche, scratch trop court), `Shape` (table non croissante ou mal dimensionnée) — sans qu'aucun
  volume bouge ;
- **report de reste** : sa nécessité, mesurée par son absence ;
- **déversoir** : son exposant, contre celui de l'orifice ;
- **chaîne ouverte** : le pas de 100 ms contre le pas de 1 ms.

Suite complète `code/` hors réseau : **379 réussis, 5 ignorés**, aucun échec, aucun avertissement
neuf — huit de plus qu'en S223, tous de la couche V.

## Suite

**Ce que V n'a pas encore, et qui vient d'ADR-010 elle-même** : `liquid_id` (plusieurs liquides,
angle mort A17), `sky_exposure` et `absorb_rate` (pluie et infiltration), les vannes et pompes, et
le **réseau fermé sous pression**, que l'ADR reporte explicitement en v2 et demande de ne pas
improviser. La **surface libre en référentiel accéléré** — la hauteur d'une ouverture évaluée par sa
distance signée au plan perpendiculaire à `g_eff` — est spécifiée au §2 et **non construite** : le
module prend `g_eff` en module, pas en direction, ce qui suffit à un vaisseau immobile et pas à un
vaisseau qui accélère.

**Ce que la couche V ouvre ailleurs** : C21 (masse d'un compartiment avec et sans δ) et la branche V
de C19 (aller-retour de persistance) deviennent instruisables ; ADR-022 §5.1 attend un état
répliqué et restauré, qui n'existe pas encore.

Restent, inchangés : la cadence complète de l'hôte (J1, reportée explicitement en S224), A261, A258,
A263, la loi GPU, J2/δ général (ADR-127).
