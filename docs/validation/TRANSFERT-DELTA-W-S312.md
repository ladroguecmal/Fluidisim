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

### 2 bis. La direction non plus ne franchit pas le raccord

`WaveEvent` porte `direction_turns` et `anisotropy`. Soumis aux deux champs, à `λ` = 2 m dans 8 m
d'eau — donc hors de toute question de régime :

| `anisotropy` | `RadialImpact` | `ImpactField` |
|---:|---|---|
| 0 | admis | admis |
| 0,25 | **`Error::Anisotropy`** | **`Error::Anisotropy`** |
| 0,50 | **`Error::Anisotropy`** | **`Error::Anisotropy`** |
| 1,00 | **`Error::Anisotropy`** | **`Error::Anisotropy`** |

**Aucune anisotropie non nulle n'est constructible.** Et `WaveEvent::impact` remet
`direction_turns` à zéro dès qu'`anisotropy` vaut zéro : à anisotropie nulle, la direction est
effacée au niveau du contrat lui-même. Un impact de W est **isotrope**, sans exception.

C'est la deuxième composante non représentable qui n'a rien à voir avec le volume net, et
l'utilisateur l'avait prévue : *« une perturbation de moyenne nulle peut également présenter une
forme, une direction ou un spectre incompatibles avec les primitives existantes »*.

---

## 3. Le second cas contrôlé : un paquet d'ondes en eau profonde

`examples/transfert_paquet.rs`.

Le cas de S311 est refusé par W (§2). Il en faut donc un que W puisse recevoir — et le choix du
**paquet** fait d'une pierre deux coups, parce qu'il répond aussi au point 2 de l'utilisateur.

### 3.1 Volume net et composante de moyenne nulle sont **deux fonctionnelles**, pas deux morceaux

L'utilisateur demande de les séparer, et avertit : *« ne présumez pas qu'en supprimant la moyenne,
le reste devient automatiquement transférable »*. Il a raison, et la raison est plus profonde
qu'un avertissement de prudence : **le flux sortant `q(t)` ne se découpe pas** en « une part nette »
et « un reste ». Son intégrale et sa forme sont deux **fonctionnelles distinctes** du même signal.
Retrancher une constante à `q(t)` pour annuler son intégrale ne produit pas une onde : cela produit
un signal que rien n'a émis.

Le paquet le rend visible parce qu'il permet de faire varier l'une sans l'autre :

```text
η(x) = a · exp(−(x−x₀)²/2σ²) · cos(k(x−x₀))   →   ∫η dx = a·σ√(2π) · exp(−k²σ²/2)
```

**Le volume net vaut `exp(−k²σ²/2)` fois l'échelle du volume absolu.** Deux points du dépôt, sur
la même formule :

| cas | `kσ` | volume net | volume absolu | **net / absolu** |
|---|---:|---:|---:|---:|
| **bosse de S311** | 0 | tout | tout | **≈ 1** |
| **paquet de S312** | 9,42 | 5,96·10⁻⁸ m³ *(plancher `f32`)* | 2,41·10⁻² m³ | **2,47·10⁻⁶** |

L'analytique donne 1,94·10⁻²¹ m³ pour le paquet : le 5,96·10⁻⁸ mesuré **est** le plancher de la
somme en `f32`, pas un volume. Les deux cas transportent autant d'eau d'avant en arrière ; l'un
en déplace en net, l'autre pas du tout. Le paramètre `σ/λ` du banc balaie la transition.

### 3.2 Le montage

Canal long en `x`, invariant en `y`, **fond nul** — donc `band_in` = 0 et aucun double comptage
possible (ADR-179 D4). `λ` = 2 m, `h₀` = 1,25 λ = **2,5 m** : au-dessus du seuil d'admission de
`RadialImpact` (§2) et `k h₀` = 7,85, donc `tanh(k h₀)` = 1 à 10⁻⁶ près — l'eau est profonde pour
**l'onde** autant que pour le champ. Maille 12,5 cm (16 par longueur d'onde), 480 × 2 × 22 mailles,
éponge de `6σ` = 18 m, pas de 10 ms, 5 432 pas pour 54,3 s.

### 3.3 Ce qui traverse la ligne de contrôle

| grandeur | valeur | lecture |
|---|---:|---|
| volume **net** traversé | 4,928·10⁻⁵ m³ | ce qui part en net |
| volume **absolu** traversé | 5,195·10⁻² m³ | ce qui fait l'aller-retour |
| **net / absolu** | **9,49·10⁻⁴** | le paquet ne transporte **pas** d'eau en net |
| période dominante à la jauge | 1,1370 s | contre 1,1318 s théorique, **+0,46 %** |
| `λ` déduite par `k = ω²/g` | **2,0183 m** | contre 2 m posés |
| énergie sortante *(état de jauge)* | **2,8676 J** | `ρ g cg ∫η²dt · largeur` |
| **réflexion artificielle, en 3D** | **2,844·10⁻⁷** | seuil 1 %, **quatre ordres sous** |

La période mesurée à **0,46 %** de la théorie est un contrôle de la dispersion autant qu'une mesure :
le pas couplé porte bien `ω² = gk` sur ce cas, et c'est ce qui rend le transfert vers W légitime —
les deux côtés du raccord parlent la même relation de dispersion.

**Le contrôle du double comptage** (point 6) : `band_in` et `perturbation_in` valent **0 exactement**,
cumulés sur les 5 432 pas. Rien n'entre, donc rien de ce qui sort n'est de l'eau qu'on recompte.
L'éponge, elle, a absorbé 9,047·10⁻⁵ m³ en absolu — presque le double du net traversé, et c'est
exactement la distinction qu'ADR-179 D3 impose de ne pas confondre.

---

## 4. Le transfert, et ce qu'il porte réellement

Le transfert est construit **par les interfaces existantes** (ADR-180 D8) : un `WaveEvent::impact`
émis au point de sortie, calibré sur ce que la jauge a mesuré, puis un `RadialImpact<128>` bâti
dessus. Deux champs de l'événement sont **forcés**, et chacun est une perte que le registre porte :

- `anisotropy = 0`, parce que **toute autre valeur est refusée** (§2 bis) ;
- `displaced_l = 0`, parce qu'il est **mesuré inerte** (§1.1) : le remplir donnerait l'apparence
  d'un transfert de volume sans en faire un, ce qu'ADR-180 D1 interdit.

### 4.1 Amplitude — c'est elle que T3 juge

À la naissance, le champ est au repos (`∂η/∂t` = **0 exactement**, mesuré sur 4 096 rayons), donc
toute son énergie est potentielle : `E = ½ρg∫η²dA`, en quadrature radiale exacte en angle.

| | valeur |
|---|---:|
| énergie demandée au transfert | 2,867 599 J |
| **énergie que le champ porte** | **2,867 635 J** |
| **erreur relative** | **1,24·10⁻⁵** |

**T3 est tenue sur le transfert effectivement réalisé** — 1,24·10⁻⁵ pour 5 % admis, quatre ordres
de marge. La grandeur testée est l'amplitude, comme ADR-180 D5 le demande ; ce n'est **pas** le
volume, que ce transfert ne porte pas et que le registre déclare en attente.

### 4.2 Direction — la perte est de moitié, mesurée

L'impact est isotrope. Mesuré par intégration de `½ρgη²` sur le disque, grille centrée sur la
source :

| demi-plan | part de l'énergie |
|---|---:|
| **avant** *(la direction du signal sortant)* | **0,5000** |
| **arrière** *(vers le domaine)* | **0,5000** |

**La moitié de l'énergie confiée à W repart dans la direction opposée.** Ce n'est pas une
imprécision : c'est la forme de la primitive. Un impact est le champ lointain d'une source
**ponctuelle** ; le signal sortant, lui, est un front qui traverse une **ligne**. Le raccord est
physiquement cohérent pour une perturbation localisée, et faux pour un front étendu.

### 4.3 Longueur d'onde et spectre — la troisième perte, et elle n'était pas prévue

| | valeur |
|---|---:|
| `λ` demandée au transfert | 2,0183 m |
| bande d'Hankel du champ, par construction | **1,009 m à 4,037 m** |
| `λ` dominante mesurée à 4 s | 1,386 m |
| `λ` dominante mesurée à 10 s | 1,467 m |
| **écart à la demande, à 10 s** | **27,3 %** |

`wavelength_m` n'est **pas** une longueur d'onde : c'est le centre d'une bande de **deux octaves**,
`k ∈ [k₀/2, 2k₀]`. Le paquet sortant, lui, est à bande étroite — `Δk/k ≈ 1/(kσ)` = **11 %**. Le
champ construit porte donc un train **dispersif** dont la longueur d'onde locale dérive avec le
temps (1,386 m à 4 s, 1,467 m à 10 s), quand le paquet en avait une seule.

C'est exactement la composante que l'utilisateur avait nommée sans qu'on sache encore la chiffrer :
*« une perturbation de moyenne nulle peut également présenter une forme, une direction ou un
**spectre** incompatibles avec les primitives existantes »*.

### 4.4 Propagation

| | valeur |
|---|---:|
| crête de l'anneau à 4 s | 3,267 m |
| crête de l'anneau à 10 s | 7,912 m |
| vitesse déduite | **0,774 m/s** |
| `cg` du paquet côté δ | **0,884 m/s** |
| **écart** | **12,4 %** |

L'anneau avance **12 % trop lentement**, et la cause est celle de §4.3 : la crête d'un train
dispersif suit la vitesse de groupe de sa composante **dominante**, et celle-ci est plus courte
que la longueur d'onde demandée. La propagation n'est donc pas un défaut indépendant — c'est le
spectre, vu par une autre mesure. Les deux écarts sont cohérents : `cg ∝ λ^½`, et
`√(1,467/2,018)` = 0,853 contre 0,876 mesuré.

### 4.5 Les trois catégories, publiées

**Registre du volume** — 5 432 pas :

| | valeur |
|---|---:|
| sorti | 4,928·10⁻⁵ m³ |
| **transféré** | **0** |
| **en attente de restitution** | **4,928·10⁻⁵ m³** |
| créé | **0** |
| résidu numérique cumulé | 1,258·10⁻⁸ m³ |
| **conservation globale revendicable** | **non** |

**Registre de la grandeur propagative** — l'énergie de jauge :

| | valeur |
|---|---:|
| sorti | 2,8676 J |
| **transféré** *(part avant du champ)* | **1,4338 J** |
| **non délivré** *(part arrière)* | **1,4338 J** |
| créé | **0** |
| part non délivrée | **50,0 %** |

Le second registre demande un mot de précaution. Son `en attente` n'est **pas** de l'eau qui
attend : c'est de l'énergie envoyée dans la mauvaise direction. Les deux registres partagent
l'arithmétique et l'interdit — rien ne peut effacer une attente, et `created()` vaut zéro dans les
deux — mais pas la signification. Un futur receveur de volume fera baisser le premier ; seule une
primitive orientée fera baisser le second.

---

## 5. Ce que ce banc a trouvé sans le chercher : **T1 n'est pas mesurable sur un cas de moyenne nulle**

ADR-179 D1 définit T1 comme *« résidu ≤ 10⁻⁶ de l'échelle du pas »*, l'échelle étant
`max(|delta|, |band_in|, |sponge_out|)` (S310). Sur ce cas, le rapport **dépend entièrement du
plancher d'activité** qu'on met sous le pas :

| pas retenus *(échelle ≥ …)* | 5 432 / 5 432 *(tous)* | 5 308 *(≥ 10⁻³ du max)* | 5 127 *(≥ 10⁻²)* | 2 093 *(≥ 10⁻¹)* | 318 *(≥ ½)* |
|---|---:|---:|---:|---:|---:|
| **pire rapport** | 1,95 | 7,03·10⁻² | 1,09·10⁻² | 1,31·10⁻³ | **1,79·10⁻⁴** |

**Même sur les 318 pas les plus actifs, le rapport vaut 1,8·10⁻⁴ — 180 fois au-dessus du seuil.**
Ce n'est donc pas l'artefact d'une division par un pas mort, et il ne se corrige pas par un
plancher.

La cause n'est pas le schéma, c'est la **normalisation** :

- l'**échelle du pas** rétrécit avec `dt` et avec l'activité — ici l'échelle maximale de tout le
  banc vaut **1,15·10⁻⁷ m³**, un dixième de millilitre ;
- le **résidu** ne rétrécit pas : c'est le plancher d'une somme en `f32` sur 21 120 colonnes,
  2,0·10⁻¹¹ m³ au pire pas ;
- et normaliser par le volume de perturbation du domaine ne sauve rien : `Balance3::volume` est
  une somme **signée**, donc **nulle par construction** pour un paquet. Mesuré : 2,78·10⁻⁴.

Rapporté à une grandeur **absolue**, le même résidu est minuscule — division des deux nombres
publiés ci-dessus : 2,008·10⁻¹¹ / 2,409·10⁻² = **8,3·10⁻¹⁰**, et le résidu cumulé rapporté au
transit absolu vaut 1,258·10⁻⁸ / 5,195·10⁻² = **2,4·10⁻⁷**. Les deux passent le seuil avec trois
ordres de marge.

**Ce qu'il faut en conclure, et ce qu'il ne faut pas.** Il ne faut pas relever T1 : le schéma est
bon, le résidu est au plancher `f32` attendu. Il faut **préciser sa normalisation** — une échelle
**absolue**, jamais signée, jamais un incrément qui tend vers zéro avec le pas. C'est une
précision à ADR-179 D1, que son statut *« provisoire, révisable si une mesure donne un autre
plancher »* prévoit explicitement. **Décision de l'utilisateur** ; en attendant, aucun banc de
cette session ne revendique T1.

---

## 6. Réponse aux six points de l'utilisateur

| point | état |
|---|---|
| 1. extraire la perturbation sortante à la surface de contrôle intérieure | **fait** — S311, rejoué ici sur un second cas (§3.3) |
| 2. séparer le volume net de la composante de moyenne nulle | **fait**, et la séparation n'est pas un découpage : ce sont deux fonctionnelles, et le paquet les fait varier indépendamment (§3.1) |
| 3. déterminer quelle part du signal est réellement représentable par W | **fait** — amplitude **oui** ; volume net **non** (§1) ; direction **non** (§2 bis) ; régime **non** hors eau profonde (§2) ; spectre et phase **non** (§4.3, §4.1) |
| 4. construire le transfert et vérifier amplitude, phase, direction, propagation | **fait** — amplitude 1,24·10⁻⁵ ; phase **impossible** (le champ naît au repos) ; direction **50 % à contresens** ; propagation **12,4 % trop lente** (§4) |
| 5. publier transféré, en attente, pertes numériques | **fait**, séparément et à chaque pas, par un registre où l'attente est une différence que rien n'écrit (§4.5) |
| 6. vérifier le bilan global sans double comptage B / W / δ | **fait sur ce cas** — `band_in` et `perturbation_in` valent **0 exactement** sur 5 432 pas (§3.3). Sur une scène à fond réel, la séparation entrant/sortant demande une décomposition en caractéristiques, **non écrite** |

## 7. Ce qui manque véritablement à W — la question de l'utilisateur, chiffrée

Le premier couplage est fait, et l'utilisateur voulait en tirer *« ce qui manque véritablement à
W »*. Trois manques, dans l'ordre de ce qu'ils coûtent au raccord :

1. **Une primitive orientée.** L'anisotropie est refusée par les deux champs ; **50 %** de
   l'énergie transférée repart vers le domaine. C'est le plus gros poste, et le seul qui soit une
   perte *à chaque transfert*.
2. **Un niveau moyen** — et il n'appartient pas à W (§1) : c'est un scalaire de B, ou un nœud de
   V. Tant qu'il n'existe pas, **100 %** du volume net reste en attente.
3. **La profondeur finie.** Hors eau profonde, W refuse (§2) ; forcé, il se trompe de **44 %** de
   célérité sur un cas comme celui de S311. Un rivage, un haut-fond ou une piscine ne peuvent rien
   transmettre à W aujourd'hui.

Un quatrième point, plus fin : **`wavelength_m` n'est pas une longueur d'onde mais le centre d'une
bande de deux octaves**. Une source à bande étroite ne se transfère pas fidèlement, et l'écart se
lit en spectre (27 %) comme en vitesse (12 %).

## 8. Limites

Tout est mesuré sur la **référence CPU**, une machine (A98), une maille (12,5 cm) — **la
convergence en maille de ce second cas n'est pas faite**, contrairement à celle de S311. Le cas est
**unidirectionnel et invariant en `y`** : il ne dit rien d'un front oblique ni d'une frontière
courbe, et la séparation entrant/sortant y est triviale **parce que rien n'entre**. Le transfert
est **ponctuel et unique** — un événement pour un paquet ; une émission continue, sa cadence et son
recouvrement ne sont pas construits. Le champ de W n'est **pas** rebouclé dans δ : le couplage
reste à un sens, et A302 reste ouverte pour l'autre. Aucune revendication d'énergie ni de quantité
de mouvement comme bilan (ADR-179 D7) : les deux registres publient des **états** et des flux de
jauge. **Le prototype n'est pas conforme à la conservation globale**, et ADR-180 D1 interdit de
l'écrire autrement tant qu'un receveur de volume n'existe pas.
