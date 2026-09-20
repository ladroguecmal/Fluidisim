# Le premier transfert δ → W — S312

2026-09-20. **Lot 2 d'[ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7**,
sous [ADR-179](../adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) et la décision
de l'utilisateur du 2026-09-20, actée en
[ADR-180](../adr/ADR-180-retour-delta-w-et-conservation-du-volume.md).

S311 a identifié ce qui sort d'un domaine δ ([preuve](SORTIE-DELTA-S311.md)) et a buté sur une
question : le **volume net** sortant n'a pas de receveur dans W. L'utilisateur a tranché — option 1,
limitée aux tests, le non-transféré **registré** et jamais dit restitué — et a ajouté une
instruction qui commande l'ordre de cette session : *« Il ne faut pas créer une nouvelle primitive
dans W avant d'avoir vérifié si cette responsabilité relève déjà d'une autre couche. »*

Machine de référence (ADR-174 D1). Aucune revendication d'énergie ni de quantité de mouvement
comme bilan (ADR-179 D7).

---

## 1. Le receveur du volume net : ce n'est pas l'impact, c'est la couche

`examples/receveur_volume_net.rs`.

S311 avait mesuré **une** primitive. Cela ne suffisait pas à décider : si porter de l'énergie et
pas de volume est une propriété de *l'impact*, on ajoute une primitive ; si c'est une propriété de
*W*, la chercher là est une erreur d'architecture. La différence se mesure, et elle a été mesurée.

### 1.1 `displaced_l` du contrat n'est honoré par aucun champ

`WaveEvent` porte un `displaced_l` — un volume déplacé, en litres. Deux impacts identiques à ce
seul champ près, **0 contre 1000 L**, donnent :

| champ construit | écart maximal de `η` | identiques au bit |
|---|---:|---|
| `RadialImpact<128>` (ADR-060) | **0** | **oui** |
| `ImpactField` (ADR-058) | **0** | **oui** |

768 points, trois âges. **Le volume du contrat est inerte** : il traverse l'encodage et n'entre
dans aucun champ. C'est exactement ce qu'ADR-180 D3 demandait de vérifier plutôt que de lire.

### 1.2 Le volume net des trois productions de W

La grandeur publiée est le **net rapporté à l'absolu** — `|∫η dA| / ∫|η| dA`. Un net petit ne dit
rien si le champ l'est aussi ; c'est le rapport qui distingue « anneau de moyenne nulle » de
« champ négligeable ».

**Impact radial**, quadrature radiale exacte en angle, **rayon balayé** — parce qu'un disque
tronque toujours un champ non nul à son bord, et que chaque mode y contribue `2πR·J₁(kR)/k`, qui
**oscille** avec `R` au lieu de converger :

| âge | `R` = 4 m | 8 m | 12 m | 15,9 m |
|---|---:|---:|---:|---:|
| 0 s | +1,86·10⁻⁴ | +1,10·10⁻⁵ | +2,29·10⁻⁵ | **−2,01·10⁻⁶** |
| 1 s | +4,90·10⁻⁴ | +6,17·10⁻⁵ | −5,31·10⁻⁶ | +1,05·10⁻⁵ |
| 4 s | +1,35·10⁻³ | −5,13·10⁻⁵ | +7,71·10⁻⁵ | −3,38·10⁻⁵ |

*(net en m³ ; l'absolu vaut 3,7 à 12,8·10⁻³ m³ selon l'âge.)*

**Le net change de signe avec le rayon et reste sous la borne de troncature à chacun des douze
points** — l'anneau d'une longueur d'onde à l'amplitude du bord. Il ne converge vers rien. Un
champ qui porterait un volume donnerait l'inverse : une valeur stable dès que le disque contient
la perturbation. **C'est de la troncature, et S311 ne pouvait pas le savoir avec un seul rayon.**

**Champ périodique**, et c'est la mesure décisive, parce qu'elle **n'a pas de troncature** :
l'intégrale porte sur exactement **une cellule** du champ, qui est périodique de côté `4λ`.

| âge | net | absolu | **net / absolu** |
|---|---:|---:|---:|
| 0 s | −4,65·10⁻¹¹ m³ | 1,27·10⁻² m³ | **3,67·10⁻⁹** |
| 1 s | −2,74·10⁻¹¹ | 9,10·10⁻³ | **3,01·10⁻⁹** |
| 4 s | +1,87·10⁻¹¹ | 1,42·10⁻² | **1,32·10⁻⁹** |

**Neuf ordres de grandeur sous l'absolu** : le volume net est nul au plancher de la somme.

**Source de pression mobile** — le sillage (ADR-071), assemblé par son spectre gaussien —, boîte
grandie à **maille constante** de 25 cm :

| demi-boîte | 20 m | 40 m | 80 m |
|---|---:|---:|---:|
| net | +1,62·10⁻¹ m³ | −2,78·10⁻¹ | −4,27·10⁻² |
| **net / absolu** | **6,28·10⁻²** | **2,67·10⁻²** | **1,03·10⁻³** |

Même signature : le rapport **tombe de 61 fois** quand la boîte quadruple, et le net change de
signe. La compensation d'une source de pression vit aux **petits nombres d'onde**, donc très
étalée ; une boîte finie en coupe toujours une part.

### 1.3 La raison, et elle est structurelle : W n'a pas de mode `k = 0`

L'intégrale d'un champ sur le plan **est** l'amplitude de son mode de nombre d'onde nul. Mesuré
sur les trois productions :

| production | plus petit `k` | d'où il vient |
|---|---:|---|
| impact radial | **0,785 rad/m** | `lo = k₀/2`, borne basse de la bande d'Hankel |
| champ périodique | **0,393 rad/m** | `(n_x, n_y) = (0, 0)` **sauté** dans la boucle des modes |
| sillage | **0,187 rad/m** | quadrature polaire à `k = (i + ½)·dk`, l'origine exclue |
| `ModalPressure::new([0, 0], …)` | — | **refusé, `Error::Domain`** |

**Aucune production de W ne possède le mode `k = 0`, et le constructeur modal le refuse
explicitement.** Ce n'est donc pas une propriété de la primitive impact : **c'est une propriété de
la couche**. W est une somme de perturbations de moyenne nulle autour d'un plan de repos — et une
« primitive de W portant un volume net » serait, par définition, un déplacement de ce plan,
c'est-à-dire **B sous un autre nom**.

### 1.4 Ce que portent les autres couches

Ces deux faits se **lisent dans les types**, et un type se lit — ils ne sont pas donnés comme des
mesures :

- **V** — `hydro_network::HydroNode { volume_ml: i64, … }`. En V, le volume **est** la variable
  d'état, en millilitres entiers, et un transfert entre nœuds est exact par construction
  (ADR-010 §4, I-10). V est un receveur **exact** de volume net, disponible aujourd'hui.
- **B** — `background::SeaState { hs, tp, theta_turns, components, graine }`. **Aucun champ de
  niveau moyen.** B est une somme de composantes de nombre d'onde non nul autour de `η = 0` : le
  plan de repos est la référence implicite, et **rien ne le porte**. « Modifier le niveau moyen de
  B », l'option que l'utilisateur nomme, n'existe donc pas encore — c'est un **scalaire à ajouter**,
  pas une primitive, et c'est une décision qui lui revient (ADR-180 §3).
- **δ** — `Balance3::volume` porte le volume de perturbation du domaine, exactement (S310).

### 1.5 Réponse au point 3 de l'utilisateur

**W n'est pas le receveur, et aucune primitive nouvelle ne l'y rendrait apte sans en faire du B.**
La responsabilité relève d'une autre couche, comme l'utilisateur le supposait, et la réponse se
sépare selon le type d'environnement, comme il le demandait :

| environnement | receveur | état |
|---|---|---|
| **contenant, piscine, région bornée** | **V**, nœud hydraulique | **existe**, exact au millilitre |
| **masse d'eau ouverte** | **niveau moyen de B**, scalaire par région | **n'existe pas** : `SeaState` n'a pas ce champ |
| *en attendant, pour le prototype* | **registre de déficit** (ADR-180 D2) | construit en §2 |

Ce que cette section ne tranche pas : ni l'articulation δ ↔ V, qui est le lot 6 d'ADR-178 D7 et
n'est pas ouverte, ni la forme qu'aurait un niveau moyen dans B — une constante par région, un
champ lent, ou une entrée de l'ordonnanceur. **Les deux sont des décisions de l'utilisateur**, et
ce document lui apporte de quoi les prendre, pas leur résultat.

---

## 2. W est une couche d'**eau profonde**, et le cas de S311 n'y entre pas

`examples/admission_w.rs`.

Avant de construire un transfert, une question qu'aucune session n'avait posée : les champs de W
**admettent-ils** ce qu'un domaine δ leur enverrait ? Le cas contrôlé de S311 est une onde longue
dans **un mètre** d'eau, `λ` ≈ 12 m. Soumis tel quel :

| champ | verdict |
|---|---|
| `RadialImpact` (ADR-060) | **refusé — `Error::Regime`** |
| `ImpactField` (ADR-058) | **refusé — `Error::Medium`** |

Ce n'est pas une borne à desserrer. Le seuil, balayé au centimètre à trois longueurs d'onde :

| `λ` | profondeur minimale, `RadialImpact` | en `λ` | profondeur minimale, `ImpactField` | en `λ` |
|---:|---:|---:|---:|---:|
| 2 m | 2,01 m | **1,00** | 4,01 m | **2,00** |
| 4 m | 4,01 m | **1,00** | 8,00 m | **2,00** |
| 12 m | 12,00 m | **1,00** | 24,00 m | **2,00** |

Et la troisième production, le **sillage**, n'a pas de paramètre de profondeur **du tout** :
`ModalPressure::new` prend `k`, `g`, `ρ` et un segment, et pose `ω = √(g|k|)`. Toute la couche W
porte la dispersion de l'eau profonde.

### Ce que coûterait de passer outre

Un refus qu'on contourne ne disparaît pas : il devient une **erreur de vitesse**. Célérité que W
donnerait à l'onde, contre celle qu'elle a réellement, `c = √(g/k·tanh(kh))` :

| cas | `λ` | profondeur | `kh` | `c` vraie | `c` de W | **écart** |
|---|---:|---:|---:|---:|---:|---:|
| **canal de S311** | 12 m | 1 m | 0,52 | 3,000 m/s | 4,329 m/s | **+44,3 %** |
| scène δ 3D de S302 | 8 m | 3,5 m | 2,75 | 3,520 m/s | 3,534 m/s | +0,41 % |
| candidat eau profonde | 2 m | 8 m | 25,1 | 1,7671 m/s | 1,7671 m/s | **0,0000** |

**Trois lectures.**

1. **Le cas contrôlé de S311 ne peut pas servir au transfert.** Il a fait son travail — identifier
   et mesurer ce qui sort — et il ne peut pas faire celui-ci. Un second cas contrôlé est
   nécessaire, en eau profonde ; c'est §3.
2. **Ce n'est pas une restriction artificielle.** La scène δ 3D réelle de S302 est à 0,41 % de la
   dispersion de W : l'eau y est déjà assez profonde. C'est le **canal** de S311 qui était
   peu profond, choisi pour d'autres raisons.
3. **Le régime est une composante non représentable**, au sens d'ADR-180 D3, et elle n'a rien à
   voir avec le volume net. Un domaine δ en eau peu profonde — un rivage, un haut-fond, une
   piscine — ne peut **rien** transmettre à W en l'état : ni son volume, ni son onde.
