# Le plancher d'un bilan de masse — dérivé, mesuré, éprouvé sur une fuite — S313

2026-09-20. **Ordre A** du lot 2, sous [ADR-181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md)
D5 et D6, qui révisent D1 d'[ADR-179](../adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md).

S312 avait mesuré un résidu de 2,0·10⁻¹¹ m³ et l'avait attribué au « plancher `f32` »
**sans le démontrer** ([A304](../registres/ANGLES-MORTS.md)). C'est une excuse tant qu'elle n'a pas
de loi. L'utilisateur a retiré le chiffre de T1 et demandé quatre grandeurs distinctes, une borne
d'erreur lorsqu'elle est possible, des balayages en amplitude, pas de temps et résolution, et
**une erreur volontaire pour éprouver l'instrument**.

Machine de référence (ADR-174 D1). `examples/plancher_bilan.rs`, `delta3d_closure.rs`.

---

## 1. La dérivation, écrite avant la mesure

Le résidu vaut `delta − band_in − perturbation_in + sponge_out`, nul en arithmétique exacte. Trois
mécanismes peuvent le peupler, et **ils se départagent par leurs lois** :

| | origine | prédiction |
|---|---|---|
| **H1 — représentation** | `η` est un `f32` de magnitude `h₀`, résolution `ulp(h₀)` | insensible à `a` **et** à `dt` |
| **H2 — incrément** | l'arrondi porte sur ce que le pas **ajoute** | `∝ a` **et** `∝ dt` |
| **H3 — accumulation `f64`** | la somme des `N` colonnes | `∝ a`, insensible à `dt` |

Trois balayages suffisent, un paramètre à la fois (**L354**). Les prédictions sont écrites avant
d'exécuter quoi que ce soit ; c'est ce qui leur donne leur valeur.

---

## 2. Les balayages : deux hypothèses réfutées, une confirmée

### 2.1 Amplitude — H1 tombe

Maille 25 cm, `dt` = 1 ms, cuve fermée, 200 pas.

| `a` | pire résidu | **résidu / volume absolu** |
|---:|---:|---:|
| 2·10⁻⁴ m | 2,398·10⁻¹⁴ m³ | **6,24·10⁻¹¹** |
| 2·10⁻³ | 2,238·10⁻¹³ | **5,82·10⁻¹¹** |
| 2·10⁻² | 1,990·10⁻¹² | **5,18·10⁻¹¹** |
| 2·10⁻¹ | 2,069·10⁻¹¹ | **5,38·10⁻¹¹** |

**Strictement linéaire en `a` sur quatre décades** — H1 est réfutée. Et la colonne de droite est la
découverte utile : le rapport au **volume absolu de perturbation** est **constant**. C'est
l'échelle pertinente que l'utilisateur demandait, et elle n'a pas été choisie, elle a été trouvée.

### 2.2 Pas de temps — H3 tombe

| `dt` | résidu moyen | rapport au précédent |
|---:|---:|---:|
| 250 µs | 9,687·10⁻¹⁴ m³ | — |
| 500 µs | 1,941·10⁻¹³ | **2,00** |
| 1 ms | 4,123·10⁻¹³ | **2,12** |
| 2 ms | 8,537·10⁻¹³ | **2,07** |
| 4 ms | 1,801·10⁻¹² | **2,11** |

**Linéaire en `dt`** — H3 est réfutée. **H2 est la seule qui survit aux deux balayages.**

### 2.3 Résolution — et le mécanisme se précise

À **aire constante** (3 m²), `a` et `dt` fixés :

| `dx` | `N` | résidu moyen |
|---:|---:|---:|
| 0,5 m | 12 | 7,751·10⁻¹³ m³ |
| 0,25 m | 48 | 4,123·10⁻¹³ |
| 0,125 m | 192 | 2,500·10⁻¹³ |

`N` quadruple, le résidu est divisé par **1,88** puis **1,65** — soit ≈ 2, la signature d'une
accumulation en **`√N`** et non en `N`. Le mécanisme est donc plus précis que H2 ne le disait : le
télescopage du transport porte sur des différences de flux **arrondies une fois par colonne**, de
signe indépendant d'une colonne à l'autre.

### 2.4 La borne

```text
plancher_du_pas  =  u₃₂ · activité / √N       u₃₂ = 2⁻²⁴,  activité = Σ|Δ(η−repos)|·dx²
```

**Mesurée à un facteur 3 près, et jamais dépassée**, sur tout le balayage : le rapport
`résidu / plancher` vaut **3,00** en cuve fermée, **3,67** avec réécriture, **5,70** en cas ouvert.

**Les bornes naïves sont fausses de cinq ordres**, et il faut le dire : `N·ulp(h₀)/2·dx²` donne
3,576·10⁻⁷ m³ et sa variante en `√N` 5,162·10⁻⁸, quand la mesure donne 2·10⁻¹². **La somme
compensée de S233 retire `ulp(h₀)` du problème** — sans elle le plancher serait cinq ordres plus
haut, et c'est la première fois que sa valeur est chiffrée.

---

## 3. Le critère : quatre grandeurs, aucun verdict

`delta3d_closure.rs`, `Closure3`. Les quatre grandeurs qu'ADR-181 D5 demande, chacune avec son
unité, et **aucun seuil dans le module** — il publie, l'appelant compare.

| grandeur | ce qu'elle attrape |
|---|---|
| **résidu absolu** (m³), pire et moyen | l'ordre de grandeur, sans interprétation |
| **résidu relatif** au volume absolu | la seule normalisation dont le rapport soit stable (§2.1) |
| **rapport au plancher** dérivé | si ce qui ne se referme pas est encore de l'arrondi |
| **forme du cumulé**, `\|cumulé\|/(moyen·√pas)` | un **bruit** (≈ 1) ou une **fuite lente** (`√pas`) |

Deux garde-fous que S312 a rendus obligatoires : l'échelle est le volume **absolu**, jamais le
signé — nul par construction sur un paquet — ni l'incrément du pas, qui rétrécit avec `dt` ; et un
pas **sans activité** ne fabrique pas de rapport, son plancher valant zéro. C'est exactement la
faute d'A304, et elle ne peut plus se produire.

---

## 4. L'erreur volontaire — et une portée que personne n'avait délimitée

### 4.1 Fuite de bilan : détectée à 10⁻¹³ m³ par pas

Un écart d'un seul signe ajouté au résidu — ce qu'un défaut de solveur ferait en retirant de l'eau
sans la déclarer.

| fuite / pas | rapport au plancher | forme du cumulé |
|---:|---:|---:|
| 0 *(témoin)* | **3,00** | 0,741 |
| 10⁻¹⁵ | 3,00 | 0,775 |
| 10⁻¹⁴ | 3,01 | 1,08 |
| **10⁻¹³** | **20,6** | 4,03 |
| 10⁻¹² | 202 | **13,7** |
| 10⁻¹¹ | 2 021 | **14,14 = √200** |

**Sensibilité : 10⁻¹³ m³ par pas**, soit **0,11 fois le plancher** et **3·10⁻¹²** du volume absolu.
Elle est portée par le **rapport au plancher** : celui-ci prend le pire *rapport*, donc il attrape
une fuite dès qu'elle dépasse le plancher du pas le plus **calme**, pas du plus actif.

La forme du cumulé **sature à `√pas`** quand la fuite domine, comme la dérivation le prédit. Mais
sa dispersion sur le témoin (0,06 à 2,93 sur vingt-deux passages) l'empêche de trancher seule à
10⁻¹³ : elle devient décisive à **10⁻¹²**.

### 4.2 Fuite d'état : **invisible**, et c'est la portée de T1

Du volume réellement retiré au champ, entre deux pas.

| fuite / pas | pire résidu | rapport au plancher | **dérive / volume absolu** |
|---:|---:|---:|---:|
| 0 | 1,82·10⁻¹² m³ | 3,67 | 1,86·10⁻⁵ |
| 10⁻⁹ à 10⁻⁷ | **identique au bit** | 3,67 | 1,86·10⁻⁵ |
| 10⁻⁶ | 2,73·10⁻¹² | 3,78 | **4,6·10⁻³** |
| 10⁻⁵ | 1,65·10⁻¹² | 2,41 | **5,2·10⁻²** |
| 10⁻⁴ | 2,44·10⁻¹² | 2,47 | **0,518** |

**Le résidu ne quitte jamais son plancher pendant que le domaine perd 52 % de son volume de
perturbation.** Ce n'est pas un défaut de l'instrument : c'est **sa portée**. Le résidu ferme *un
pas* ; une fuite qui a lieu **entre** deux pas est hors de son champ, et seule la **dérive** la
voit.

**T1 et T2 ne sont donc pas redondantes.** Chacune attrape exactement ce que l'autre laisse passer,
et **aucune seule ne suffit à parler de conservation**. Aucun document du dépôt ne le disait.

*Fait mesuré en passant* : de 10⁻⁹ à 10⁻⁷ les sorties sont **identiques au bit**. L'offset demandé
est sous `ulp(2,0)` = 2,4·10⁻⁷ m : **la fuite n'a pas lieu**, elle n'est pas représentable. C'est
l'autre face de la somme compensée — elle sauve les incréments du pas, elle ne peut rien pour une
écriture extérieure.

---

## 5. T2, sur la durée demandée

ADR-179 D2 veut la dérive d'un domaine fermé sous **10⁻⁶ de l'amplitude de référence sur 10 s**.
S310 ne l'avait mesurée que sur 5 s, et S311 l'avait inscrite comme une dette.

| durée | dérive / amplitude, `dx` = 0,25 m | `dx` = 0,125 m |
|---:|---:|---:|
| 1 s | 6,91·10⁻¹⁰ | 3,38·10⁻¹⁰ |
| 5 s | 1,54·10⁻⁹ | 5,77·10⁻¹⁰ |
| **10 s** | **2,99·10⁻⁹** | **5,77·10⁻¹⁰** |
| 20 s | 3,52·10⁻⁹ | 7,92·10⁻¹⁰ |

**Tenue, avec 340 fois de marge à la maille grossière et 1 700 à la fine.** De 1 s à 20 s la dérive
croît de **5,1 fois** pour `√20` = 4,5 attendus : c'est une **marche aléatoire**, pas une fuite. Et
elle **diminue quand la maille se raffine**, conforme à la loi en `A/√N` de §2.4 — un troisième
contrôle de la borne, sur un banc qui n'avait pas servi à l'écrire.

**La dette de S311 est levée par la mesure, pas par un changement de seuil.**

---

## 6. La tolérance proposée — à l'utilisateur, avec ce qui la justifie

ADR-181 D5 exige qu'une tolérance soit justifiée par des mesures et, lorsque possible, par une
borne. Voici les trois, et **ce qu'elles déclarent non conforme**.

| | proposition | ce qui la justifie |
|---|---|---|
| **C1** | `rapport au plancher ≤ 10` | témoin mesuré **3,0 à 5,7** sur trois montages ; plus petite fuite détectée **20,6**. Dix est entre les deux, à la moyenne géométrique près — ni le bruit, ni un chiffre rond choisi seul |
| **C2** | `forme du cumulé ≤ 5`, sur **au moins 200 pas** | témoin **0,06 à 2,93** sur vingt-deux passages ; fuite d'un seul signe à 10⁻¹² → **13,7**, saturation à `√pas`. Attrape ce que C1 ne peut pas : un résidu biaisé **sous** le plancher du pas |
| **C3** | T2 **inchangée**, ≤ 10⁻⁶ sur 10 s | mesurée **2,99·10⁻⁹** et **5,77·10⁻¹⁰** (§5). Le seuil n'a pas besoin d'être touché |

**Sensibilité combinée : 10⁻¹³ m³ par pas**, portée par C1. Aucune tolérance plus fine que cela
n'aurait de sens : l'instrument ne saurait pas la distinguer de son propre plancher.

**Et une exigence de portée, qui n'est pas un chiffre** : C1 et C2 ferment *un pas*, C3 surveille
*une durée*. §4.2 montre qu'une fuite peut passer la première et pas la seconde. **Les trois sont
requises ensemble** ; deux sur trois ne constituent pas une conservation.

### Ce que cette proposition déclare non conforme, aujourd'hui

**Le cas ouvert échoue C2** : sa forme du cumulé vaut **13,67** pour `√200` = 14,14 — son résidu
est **d'un seul signe**. En valeur absolue c'est minuscule (2,88·10⁻¹⁰ m³ sur 200 pas, soit
5·10⁻⁸ de la dérive physique), mais c'est **systématique**, et un biais systématique n'est pas du
bruit. La cuve fermée, elle, passe les trois.

C'est le premier service rendu par le critère : il montre du doigt quelque chose que six sessions
de bilans n'avaient pas vu. Suspect nommé, **non démontré** : la bande ou l'éponge, qui n'existent
que dans ce cas. Enregistré en **A305**.

---

## 7. Limites

Tout est mesuré sur la **référence CPU**, une machine (A98), sur une **cuve fermée** et un cas
ouvert à fond uniforme — ni scène réelle, ni carte. La borne de §2.4 est vérifiée sur un facteur
16 en `dt`, quatre décades en amplitude et un facteur 16 en `N` ; elle n'est pas démontrée
analytiquement au-delà du raisonnement de §2.3, et sa constante (3) est **mesurée**, pas prouvée.
L'erreur volontaire de §4.1 est **un seul signe et une seule forme** : une fuite alternée ou
périodique n'a pas été essayée, et la forme du cumulé, par construction, ne la verrait pas. La
sensibilité de 10⁻¹³ m³ vaut pour **ce montage** — elle suit le plancher, donc l'activité et `√N`.
Enfin, les trois seuils de §6 sont **proposés**, non actés : ADR-181 D5 en fait une décision de
l'utilisateur, et aucun banc du dépôt ne les applique tant qu'elle n'est pas prise.
