# ADR-023 — Quatre mécanismes restés à spécifier

- **Statut** : proposée
- **Session** : S12
- **Ferme** : ADR-008 §5.3 et §5.4, ADR-013 §7.4, ADR-015 §7.3 — les quatre points remontés par
  [`AUDIT-POINTS-OUVERTS-S11`](../registres/AUDIT-POINTS-OUVERTS-S11.md) §7.4
- **Dépend de** : ADR-008, ADR-010, ADR-013, ADR-014, ADR-015, SPEC-001, SPEC-002, SPEC-006

---

## 1. Pourquoi ces quatre-là, et ce qu'ils ont en commun

L'audit S11 a trouvé quatre points ouverts qui disaient « à spécifier » sans qu'aucune session ne
l'ait jamais pris en charge : ce n'étaient ni des mesures, ni des arbitrages, ni des dépendances
inter-équipes, mais du travail de conception rendu invisible par l'endroit où il était inscrit.

**Leur origine est commune ; leur cause ne l'est qu'à moitié**, et il vaut mieux le dire que de
forcer une unification qui n'existe pas :

- le **terme d'impact** (§2) et le **nageur** (§3) sont les **deux frontières du domaine de validité
  du modèle de flottabilité d'ADR-008** — l'une dans le temps, l'impact étant plus bref que le tick ;
  l'autre dans la nature du corps, un nageur n'étant pas passif ;
- les **sites turbulents permanents** (§4) et la **coalescence des poches d'air** (§5) n'ont de
  rapport ni avec eux ni entre eux.

Deux des quatre se résolvent en **élargissant un mécanisme existant** plutôt qu'en en écrivant un
nouveau, et un troisième en réutilisant un canal déjà spécifié. C'est le résultat qu'on doit
attendre d'un corpus mûr, et le signe qu'il l'est.

| § | Mécanisme | Résolution | Nouveau mécanisme ? |
|---|---|---|---|
| 2 | Terme d'impact (*slamming*) | impulsion de masse ajoutée, sous-tick | **oui**, mais il réutilise le tenseur de masse ajoutée d'ADR-008 §2 |
| 3 | Nageur en surface | seconde condition d'entrée au mode contraint d'ADR-008 §3 | **non** |
| 4 | Sites turbulents permanents | polyligne de déferlement dégénérée, dérivée de `H/h = 0,78` | **non** |
| 5 | Coalescence des poches T2 | addition des moles, niveau commun par `shape_lut` | **non** |

---

## 2. Le terme d'impact — ce qu'on publie n'est pas la pression

### 2.1 Pourquoi l'intégration quasi-statique le manque

Le modèle d'ADR-008 §2 intègre la pression sur des points d'échantillon à chaque tick. Une entrée
dans l'eau est plus brève que le tick, et l'échantillonnage la traverse sans la voir.

Théorie de Wagner : une carène de **relèvement de fond** `β` entrant à la vitesse `v` mouille une
demi-largeur qui croît comme `c(t) = (π/2)·v·t/tan β`. Le temps pour mouiller une demi-largeur `b` :

```
t_impact = 2·b·tan β / (π·v)
```

| Cas | `b` | `β` | `v` | `t_impact` | à 30 Hz |
|---|---|---|---|---|---|
| Étrave de vedette retombant de vague | 1,0 m | 30° | 5 m/s | **73 ms** | 2,2 ticks |
| Corps humain tombant de 3 m | 0,20 m | 45° | 7,7 m/s | **17 ms** | 0,5 tick |
| Coque plate claquant à plat | 2,0 m | 10° | 4 m/s | **56 ms** | 1,7 tick |

**L'impact dure de l'ordre du tick, ou moins.** Un terme de force échantillonné à 30 Hz le rate ou
le double selon la phase — et le défaut est intermittent, donc introuvable.

### 2.2 La pression de pic est ce qu'on ne sait pas

Le coefficient de pression maximal d'un dièdre (Wagner) vaut :

```
C_p = 1 + ( π / (2·tan β) )²          p_max = C_p · ½ ρ v²
```

| `β` | `C_p` | `p_max` à `v = 5 m/s` | à `v = 10 m/s` |
|---|---|---|---|
| 45° | 3,5 | 43 kPa | 173 kPa |
| 30° | 8,4 | 105 kPa | 420 kPa |
| 20° | 19,6 | 245 kPa | 980 kPa |
| 10° | 80,4 | **1,0 MPa** | **4,0 MPa** |
| 5° | 323 | 4,0 MPa | 16 MPa |

Deux enseignements, et ils vont dans des directions opposées :

- **c'est l'angle qui domine, pas la vitesse.** Passer de 30° à 10° multiplie la pression par
  **dix** ; doubler la vitesse ne la multiplie que par quatre. Un fond plat est le vrai danger, et
  c'est la raison d'être du relèvement de carène sur toute coque rapide ;
- **la formule diverge quand `β → 0`.** À relèvement nul elle prédit une pression infinie, ce qui
  est faux : le coussin d'air piégé et la compressibilité de l'eau l'écrêtent. **On ne connaît donc
  pas la pression de pic**, et un modèle qui la publie publie son incertitude.

### 2.3 Décision : publier l'impulsion, pas la pression

> **Le terme d'impact est une impulsion de masse ajoutée, appliquée sur le tick, et non une force
> de pression échantillonnée.**

Quand la carène mouille une demi-largeur `c`, elle entraîne l'eau que la théorie de la masse ajoutée
d'une plaque plane de demi-largeur `c` décrit : `m_a = ½·π·ρ·c²` par mètre de longueur. La quantité
de mouvement verticale transmise à l'eau entre le début et la fin de la pénétration vaut donc :

```
J = Δ(m_a) · v_rel              par mètre de longueur mouillée
```

| Cas | `c` final | `m_a` (par m) | `v_rel` | `J` (par m) | Section de 10 m |
|---|---|---|---|---|---|
| Étrave de vedette | 1,0 m | 1 571 kg/m | 5 m/s | 7,85 kN·s/m | 78,5 kN·s |
| Corps humain | 0,20 m | 62,8 kg/m | 7,7 m/s | 0,48 kN·s/m | *(≈0,3 kN·s au total)* |

Sur les 73 ms de l'étrave, cela fait une force moyenne de l'ordre de **1,1 MN** — cohérente avec la
pression de pic de 105 kPa appliquée sur la surface mouillée, ce qui est le contrôle qu'il fallait
faire.

**Trois raisons de préférer l'impulsion :**

1. **c'est un bilan de quantité de mouvement, il ne peut pas être faux.** La pression de pic dépend
   d'une théorie qui diverge ; l'impulsion, non — elle est bornée par la masse d'eau réellement
   accélérée ;
2. **c'est ce que l'intégrateur du solide veut recevoir.** Un moteur de corps rigides applique une
   impulsion en une opération exacte ; une force de 1,1 MN appliquée pendant un tick de 33 ms donne
   un résultat qui dépend du schéma d'intégration ;
3. **elle réutilise une grandeur déjà là.** ADR-008 §2 impose déjà un tenseur de masse ajoutée par
   archétype. Le terme d'impact n'ajoute pas de modèle physique, il exploite en régime transitoire
   celui qui servait en régime établi.

### 2.4 Le terme d'impact est autoritaire

Ses entrées sont l'état du solide — répliqué par la physique et le réseau — et `η`, `v_eau` issus de
**B + W** (ADR-008 §2). Aucune ne vient de δ. Il satisfait donc **I-15** et relève de la ligne
« poussée d'Archimède » de la table d'autorité d'ADR-008 §1, sans exception ni aménagement.

C'est ce qui permet qu'un claquement de coque **casse quelque chose** : dégât de structure, chute
d'un personnage, largage de cargaison. Un terme d'impact calculé depuis δ n'aurait pas pu, et
l'ensemble aurait dû être refait.

### 2.5 Sous-cyclage et déclenchement

L'impact est détecté quand la vitesse normale relative d'un point d'échantillon franchit la surface
vers le bas au-delà d'un seuil. Valeur de départ **`v_rel·n > 2 m/s`**, à calibrer : en deçà,
l'impulsion est inférieure au bruit de la flottabilité quasi-statique elle-même.

L'impulsion est intégrée **analytiquement sur le tick** à partir de `t_impact` (§2.1) et de la
vitesse d'entrée, sans sous-cyclage du solveur : les deux grandeurs sont connues en forme fermée,
et sous-cycler ne ferait qu'échantillonner plus finement une courbe qu'on sait intégrer.

Le relèvement `β` est une propriété d'archétype de coque, au même titre que le tenseur de masse
ajoutée — pas une mesure faite à l'exécution sur la géométrie.

### 2.6 Point de rupture, et le banc qui l'attrape

La théorie de Wagner suppose une entrée **rapide devant la période de la houle** et une surface
localement plane. Elle cesse de valoir quand la vitesse d'entrée devient comparable à la vitesse
orbitale de la surface — cas d'une coque qui accompagne la vague plutôt que de la percuter. Le
critère de déclenchement de §2.5 porte donc sur la vitesse **relative**, et non absolue, ce qui
traite le cas sans mécanisme supplémentaire.

À vérifier au banc **B6** (flottabilité), avec deux références analytiques : la conservation de la
quantité de mouvement totale eau + solide, et la décroissance en `1/tan β` de la durée d'impact.
Cas canonique **C20**, ajouté à `CAS-CANONIQUES.md`.

---

## 3. Le nageur — il n'y a pas de modèle à écrire

ADR-008 §5.4 disait : « Nageur / joueur en surface : modèle distinct, **probablement cinématique
contraint** plutôt que dynamique. À traiter avec l'équipe personnage. »

L'intuition était juste et la conclusion était trop large : le mode cinématique contraint **existe
déjà**, c'est celui d'ADR-008 §3, et il ne lui manque qu'une seconde condition d'entrée.

### 3.1 Le critère existant est bon, il est seulement le seul

ADR-008 §3 bascule un corps en mode contraint quand `ω·dt > 1,0`, avec `ω = √(k/(m + m_a))` et
`k = ρ·g·A_flottaison`. Appliqué à un humain :

```
A_flottaison ≈ 0,25 m²   →   k = ρ g A ≈ 2 450 N/m
m ≈ 75 kg,  m_a ≈ 70 kg  →   ω = √(2450/145) ≈ 4,1 rad/s
ω·dt à 30 Hz ≈ 0,14      →   « intégration normale »
```

**Un nageur passe le critère de stabilité sans difficulté.** Le mode contraint ne se déclenche donc
jamais pour lui, alors que c'est exactement le mode qu'il lui faut. Le critère n'est pas faux : il
mesure la **stabilité numérique**, et la stabilité n'est pas le problème du nageur.

### 3.2 Le vrai motif : un corps contrôlé n'est pas un corps passif

Le modèle de flottabilité d'ADR-008 §2 décrit un solide **passif** soumis à une pression. Un joueur
en surface n'en est pas un : il a une intention, et il a une caméra. Deux conséquences que la
physique ne rattrape pas :

- **une flottabilité dynamique se bat contre les commandes.** Le pilonnement d'un corps humain a une
  période propre `T = 2π/ω ≈ 1,5 s`. Un joueur qui veut avancer subit un mouvement vertical du même
  ordre de grandeur que son intention, et le contrôle devient mou sans que rien ne soit faux ;
- **c'est le poste le plus exposé au mal des transports.** Une caméra attachée à un corps qui pilonne
  et roule librement sur la houle est le cas d'école. Ce n'est pas un argument de confort : c'est une
  contrainte de conception qui n'a pas de solution en aval.

> **Décision.** Le mode contraint d'ADR-008 §3 reçoit une **seconde condition d'entrée** :
> *un corps contrôlé par un joueur ou par une IA, en surface, y bascule quel que soit son `ω·dt`.*
> Le mode lui-même est inchangé — projection sur la surface `z = η`, orientation alignée sur la
> normale, vitesse horizontale amortie vers la vitesse orbitale — et la commande du joueur s'ajoute
> dans le repère de la surface, non dans le repère du monde.

### 3.3 Ce que le nageur perd, et ce qu'il ne perd pas

Le mode contraint supprime la force de flottabilité, pas le rapport à l'eau. Tout ce qui compte pour
le jeu passe par d'autres chemins, tous déjà spécifiés :

| Effet | D'où il vient | Statut |
|---|---|---|
| Être emporté par un courant | `HR = d·(v+0,5)`, SPEC-002 §5 et ADR-018 §3 | déjà spécifié |
| Ne plus flotter dans l'eau blanche | `ρ_eff = (1−α)·ρ`, ADR-014 §5.2 | déjà spécifié |
| Être poussé par une déferlante | événement W, table d'autorité d'ADR-008 §1 | déjà spécifié |
| Hypothermie, gel | `temp` du `TraversabilitySample`, SPEC-006 §5.1 | déjà spécifié |
| Seuils de progression (0,15 / 0,50 / 1,00 / 1,30 m) | ADR-018 §2 | déjà spécifié |

**Aucun de ces effets ne passait par la flottabilité dynamique.** Le mode contraint ne coûte donc
rien au gameplay, et il enlève le seul mécanisme qui gênait.

### 3.4 Deux seuils dérivés, que le design n'aura pas à choisir

**Quand un nageur cesse de contrôler sa trajectoire.** La vitesse orbitale de surface vaut `πH/T`
(SPEC-001 §1). Un adulte nage en soutenu à ≈0,7 m/s. L'égalité donne la mer où il ne fait plus
route :

| `T` | `H` telle que `πH/T = 0,7 m/s` |
|---|---|
| 4 s | 0,89 m |
| 5 s | 1,11 m |
| 8 s | **1,78 m** |

Autrement dit : **par mer de 1 à 2 m, un nageur ne va plus où il veut.** Ce n'est pas un réglage,
c'est une conséquence de deux nombres déjà écrits, et c'est le seuil qui rend une traversée à la
nage dramatique au bon moment.

**Quand un nageur décolle de la surface.** L'accélération verticale de la surface vaut `(H/2)·ω²`.
Elle atteint `g` — le corps décolle — pour `H ≈ 12 m` à `T = 5 s`, `H ≈ 32 m` à `T = 8 s` : jamais,
dans une houle ordinaire.

**L'exception est déjà écrite.** SPEC-002 §1 donne l'accélération descendante d'une crête qui
déferle : `0,45 g` en déferlement glissant, **`g` en déferlement plongeant**. Un nageur ne décolle
donc de la surface que sous un **rouleau plongeant** — ce qui est précisément la scène qu'on veut,
et dont la condition de déclenchement existait déjà sans avoir été écrite pour cela.

La contrainte cinématique est donc valide sur tout le domaine ordinaire, et sa sortie a un critère
physique, pas un seuil d'auteur.

### 3.5 Ce qui reste à l'équipe personnage

La décision ci-dessus est interne au système d'eau : elle dit *quel mode* s'applique et *quelles
grandeurs* sont fournies. Ce qui appartient à l'équipe personnage, et qu'il faut lui porter :

- l'animation et la machine à états de la nage — le système d'eau ne fournit que `η`, la normale, la
  vitesse orbitale et les seuils d'ADR-018 §2 ;
- le point d'attache de la caméra, sachant que le corps suit désormais la surface et non une
  dynamique propre ;
- la valeur de la vitesse de nage soutenue, dont dépend le seuil de §3.4 — 0,7 m/s est une valeur de
  départ, pas une exigence.

C'est la sixième entrée du tableau « ce que d'autres équipes doivent fournir »
(`00_INDEX.md`), et elle est désormais **exécutable** : il y a quelque chose à soumettre.

---

## 4. Les sites turbulents permanents — dérivés, pas émis

ADR-013 §7.4 proposait : « rochers turbulents permanents, traités comme **émetteurs W
stationnaires** dépendant de la houle locale, sans coût quand personne n'est présent. À spécifier. »

L'intention est juste — sans coût quand personne n'est là, et fonction de la houle locale. Le mot
*émetteur*, en revanche, ne survit pas à l'écriture de la spécification.

### 4.1 Pourquoi un émetteur d'événements est le mauvais mécanisme

Un `WaveEvent` pèse 45 octets et il est **répliqué** (SPEC-006 §3.1). ADR-009 §2 chiffre le trafic
d'une zone chargée — une bataille navale à 20 événements/s — à 900 o/s par joueur intéressé. Une
côte rocheuse porte facilement deux cents sites actifs ; à un événement par seconde chacun :

```
200 sites × 1 év/s × 45 o  =  9 000 o/s par joueur intéressé
                              soit dix fois la bataille navale, en permanence, pour du décor
```

Trois autres objections, chacune suffisante : les événements **s'accumulent** et devraient être
élagués en permanence, ce qu'ADR-021 §4 interdit pour les paquets `W_rep` au-dessus du seuil de
pertinence ; un phénomène **stationnaire et déterministe** n'a aucune raison d'être répliqué,
puisque chaque client le dérive à l'identique (I-15) ; et cela contredirait l'esprit d'I-02 — ce qui
est déterministe n'a pas à être mémorisé.

> **Décision.** Un site turbulent permanent n'est pas un émetteur. C'est un **terme stationnaire
> dérivé**, re-calculé à la demande depuis B, W et la bathymétrie, jamais stocké ni répliqué.

C'est exactement le mécanisme qu'ADR-014 §2.3 avait déjà retenu pour l'écume permanente : « ligne de
déferlement d'une plage, remous d'un rocher : **re-dérivée** du modèle de déferlement de W, jamais
stockée ». Le mécanisme existait donc, sous un autre nom, dans un autre document — et ADR-013 §7.4 en
proposait un second sans le savoir.

### 4.2 Un site turbulent est une polyligne de déferlement dégénérée

SPEC-006 §6 publie déjà la **polyligne de déferlement** : une suite de `BreakerVertex` portant
position, direction de crête, flux dissipé en kW/m et largeur de zone de déferlement. Un rocher qui
brise est le même objet avec une largeur de quelques mètres au lieu de quelques centaines.

**Aucun type nouveau, aucun canal nouveau.** Le site est un `BreakerVertex` de `surf_width_m` petit,
publié sur le canal existant, avec les mêmes consommateurs et pour les mêmes usages : l'audio y
prend son lit de rivage (ADR-016 §2), le rendu son écume, l'ordonnanceur son critère d'activation.

### 4.3 La liste des sites se dérive, elle ne s'écrit pas

Une houle brise quand `H/h ≈ 0,78` (McCowan, SPEC-001 §3). Un haut-fond est donc un site actif
lorsque la tranche d'eau au-dessus de lui satisfait :

```
h  <  H_local / 0,78  ≈  1,28 · H_local
```

| `Hs` local | Profondeur au-dessus du haut-fond en deçà de laquelle il brise |
|---|---|
| 0,5 m | 0,64 m |
| 1 m | 1,28 m |
| 2 m | **2,6 m** |
| 4 m | 5,1 m |

La passe de cuisson balaie donc la bathymétrie à la recherche des minima locaux et retient ceux dont
le dégagement est inférieur à `1,28 · Hs_max` de la région. C'est la ligne « sites turbulents
permanents — dérivé + validation auteur » de la table des données de SPEC-005 §2, et cela en donne
enfin le critère.

**Bénéfice non demandé : la marée allume et éteint les sites.** La tranche d'eau `h` varie du
marnage. Un rocher à 3 m sous le niveau moyen, avec 4 m de marnage, voit `h` passer de 1 à 5 m ; par
mer de 2 m — seuil à 2,6 m — **il brise à basse mer et pas à haute mer**. Un récif qui gronde deux
fois par jour, à heure prévisible (ADR-018 §4), sort d'une inégalité et d'aucun réglage.

### 4.4 Ce qu'un site produit, et à quel coût

Le flux dissipé se calcule comme pour une ligne de déferlement : `P = E·c_g` avec `E = ρgH²/16`
(SPEC-001 §3). Par mer de `Hs = 2 m`, `T = 8 s` : 15,3 kW/m, soit **≈77 kW** pour un rocher de 5 m
de large. Cette valeur pilote directement l'intensité audio et le taux de dépôt d'écume
(ADR-014 §3.2), sans réglage d'auteur.

| Consommateur | Ce qu'il en fait | Coût quand personne n'est là |
|---|---|---|
| Audio | émetteur du lit de rivage, intensité `∝ P` | nul — pas d'auditeur enregistré (SPEC-006 §4.2) |
| Rendu | source du champ d'écume, écume permanente re-dérivée | nul — hors cascade (ADR-014 §2.3) |
| Ordonnanceur | candidat d'activation, priorité par surface écran | nul — `W_perception = 0` (ADR-012 §2) |

**Le coût nul quand personne n'est présent n'est pas une optimisation à écrire : c'est une
conséquence de n'avoir rien à faire tourner.** Un site est une entrée de table plus une formule ;
sans consommateur, la formule n'est pas évaluée.

### 4.5 Invalidation

Un site dérive de la bathymétrie. Retoucher un haut-fond invalide donc la liste des sites
localement — et le partitionnement de cuisson suit déjà les **isobathes** et non une grille carrée
(SPEC-005 §8), ce qui est exactement le découpage dont cette dérivation a besoin. Rien à ajouter.

La validation d'auteur mentionnée par SPEC-005 §2 garde son rôle : la dérivation peut retenir des
centaines de sites dont beaucoup sont sans intérêt scénique. L'auteur en promeut ou en écarte ; il
ne les déplace pas et n'en crée pas — ce serait une retouche de donnée dérivée, ce que la règle de
source de vérité unique interdit (SPEC-005 §1, L25).
