# La phase à dix longueurs d'onde, et l'oblique à la frontière — S315

2026-09-20. Les deux vérifications que l'ordre B laissait dues, sous
[ADR-183](../adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md) D1 et D2.

S314 a construit `WaveTrain` et mesuré, **sur la ligne d'émission**, une régression de phase de
0,9938. L'utilisateur a tranché : *« la régression de phase mesurée sur la ligne d'émission est
encourageante, mais elle ne suffit pas à valider la propagation »*. Et *« la primitive accepte déjà
une direction oblique, mais il reste à démontrer que **le raccord l'exploite correctement** »*.

Machine de référence (ADR-174 D1). `src/wave_train.rs`, `examples/transfert_oriente.rs`.

---

## 1. Un oracle de phase ne peut pas être le train lui-même

Le train est une **somme de modes exacts** : chaque mode est une solution de la houle linéaire en
eau profonde, donc le champ propage exactement **par construction**. Le comparer à sa propre
formule ne mesurerait rien — ce serait vérifier qu'une addition est une addition.

ADR-183 D2 exige une **autre physique**. Le dépôt n'en a qu'une qui produise des ondes de gravité
indépendamment de W : **δ lui-même**. D'où le montage — **deux lignes de contrôle**, séparées de
dix longueurs d'onde. Le train est émis depuis la première ; δ continue de vivre jusqu'à la
seconde. **Le train prédit, δ constate.**

Trois grandeurs se comparent à la seconde ligne, et **aucune n'est ajustée** — ADR-183 D3 interdit
de rattraper une phase par une amplitude ou par un décalage, et le banc n'offre ni l'un ni l'autre :
l'**instant d'arrivée** de l'enveloppe, l'**amplitude**, et la **phase** de la porteuse.

## 2. Le désaccord attendu, calculé avant la mesure

Le train et δ portent la **même fréquence** — le transfert la lit sur la jauge et la lui donne. Ce
qu'ils ne partagent pas est le **nombre d'onde** : le train pose `k = ω²/g`, l'eau profonde exacte,
tandis que l'onde de δ garde le `k` de sa condition initiale et n'a que la fréquence que le schéma
**discret** lui donne. À fréquence égale et nombre d'onde différent, la phase se sépare
**linéairement avec la distance**.

Calculé depuis les seules mesures de S314, avant que le banc existe, sur `D` = 10 λ = 20 m :

| maille | `ω` lue | `k_train = ω²/g` | `Δk` | **`Δφ` prédit** |
|---:|---:|---:|---:|---:|
| 25 cm | 5,3094 | 2,8736 | −0,2680 | **−0,853 tour** |
| 12,5 cm | 5,5056 | 3,0899 | −0,0517 | **−0,165 tour** |
| 6,25 cm | 5,5495 | 3,1393 | −0,0023 | **−0,0072 tour** |

**Deux ordres de grandeur entre les extrêmes** : la prédiction est donc un vrai test. Et elle dit
d'avance que **la phase à dix longueurs d'onde sera fausse de soixante degrés à 12,5 cm** — sans
que le transfert y soit pour rien.

Le banc ne s'en contente pas : `k` posé n'est pas `k_δ`, et sur une scène quelconque la condition
initiale est inconnue. Il mesure donc `k_δ` **directement**, par périodogramme **spatial** sur un
instantané du domaine pris entre les deux lignes.

## 3. Le contrat de la primitive a refusé la distance demandée

Garder le train jusqu'à la seconde ligne demande une cinquantaine de secondes, où l'enveloppe
s'élargit de **11 %** — au-delà des 10 % que `SPREAD_LIMIT` admet. **Le constructeur a refusé.**

Deux issues, et elles ne se valent pas : **relever la constante**, ce qui l'aurait fait en silence
pour **tous** les appelants ; ou **déclarer** ce qu'on accepte, pour celui-ci seulement.
`TrainSpec::spread_limit` est donc devenu un champ, `SPREAD_LIMIT` en reste le défaut, et le banc
**publie l'élargissement obtenu** à côté de celui qu'il a déclaré. Le module l'avait écrit en
toutes lettres dès S314 — *« un appelant qui veut plus long le demande explicitement et publie
l'écart »* — ; il a suffi de le rendre exécutable.

*Un essai nouveau en est sorti, et il est contre-intuitif* : un rayon **trop large** est un refus
`Resolution`, pas une précaution. La réplique du spectre discret entre dans le disque dès qu'on
déclare un domaine plus grand que nécessaire.

## 4. Deux fautes de méthode, avant le premier chiffre utile

**Comparer deux phases référencées à deux instants différents.** L'identification rend une phase
relative à **l'arrivée de son propre signal**. Or la prédiction arrive **1,66 s avant** δ — leurs
origines sont distantes de 1,45 tour. La soustraction directe rendait 0,38 tour d'écart
« inexpliqué » qui ne mesurait que ce décalage. Corrigé : les deux phases se lisent à un **instant
absolu commun**, `ω·(t − t_c)/2π − φ`.

**Un binaire périmé qui rend des chiffres plausibles.** Deux exécutions précédentes tournaient
encore et **verrouillaient l'exécutable** ; `cargo build` échouait, son erreur était avalée par le
filtre de la ligne de commande, et **l'ancien binaire s'exécutait**. Une mesure de phase calculée
par la version d'avant a failli être publiée. Un échec de compilation filtré est pire qu'une
erreur : il rend la sortie d'un **autre programme**, et elle a l'air normale.

---

## 5. Ce que l'oracle mesure, et ce qu'il ne conclut pas encore

Montage : deux lignes de contrôle, maille 12,5 cm, paquet de S312. La **séparation se balaye**,
tout le reste fixé — c'est le seul moyen de séparer un décalage de comparaison d'un écart qui
s'accumule.

| séparation | `Δφ` espace | `Δφ` attendu | `Δφ` mesuré | **résidu** | rapport d'amplitude |
|---:|---:|---:|---:|---:|---:|
| 1 λ | +0,0729 | +0,0807 | +0,0227 | **−0,058** | 0,985 |
| 3 λ | −0,0057 | +0,0109 | +0,0746 | **+0,064** | 1,021 |
| **10 λ** | **−0,1655** | −0,1096 | +0,2463 | **+0,356** | 1,063 |

### 5.1 Ce qui marche, et qui n'est pas rien

- **L'oracle sait mesurer ce qu'il doit mesurer.** Le périodogramme **spatial** retrouve
  `k_δ` = 3,1395 pour 3,1416 posé — **0,07 %** — sans rien supposer de la condition initiale.
- **Le terme d'espace tombe exactement sur la prédiction de §2** : −0,1655 mesuré contre −0,165
  annoncé avant que le banc existe.
- **L'amplitude et l'arrivée sont des comparaisons utiles, et elles disent quelque chose de δ** :
  le train sur-estime de **6,3 %** à dix longueurs d'onde — c'est la **dissipation numérique** de δ
  sur ce trajet — et arrive **1,66 s trop tôt** sur 24 s, soit une vitesse de groupe supérieure de
  7,5 % à celle que le schéma réalise.

### 5.2 Ce qui ne conclut pas

**Le résidu de phase croît avec la distance** — 0,058 ; 0,064 ; 0,356 —, donc ce **n'est pas** un
décalage fixe de comparaison : le balayage l'établit, et c'est pour cela qu'il a été fait.

Mais le modèle à deux termes ne le prédit pas. Et la raison est structurelle : **une phase n'est
connue que modulo un tour**. Dès que les vitesses de groupe diffèrent, le train et δ n'arrivent
plus ensemble — 1,66 s d'écart à dix longueurs d'onde, soit **1,46 tour** de porteuse —, et la
comparaison de deux phases enroulées devient ambiguë. Les trois résidus sont compatibles avec
plusieurs enroulements, et rien dans ce banc ne tranche lequel.

**La phase à dix longueurs d'onde n'est donc pas validée**, et elle n'est pas non plus invalidée :
elle est **indéterminée par ce montage**. Ce qu'il faudrait est un instrument qui **suit la
porteuse en continu** entre les deux lignes plutôt que de comparer deux phases enroulées aux
extrémités — un déroulement de phase le long du trajet, que ce banc ne fait pas.

**Ce qui n'a pas été fait pour y arriver, et ne le sera pas ainsi** : ADR-183 D3 interdit de
rattraper la phase par un décalage temporel. Ajuster le train de 1,66 s aurait rendu les deux
signaux superposables et aurait mesuré **exactement rien**. Le banc n'offre pas ce réglage, et le
résultat reste ce qu'il est : indéterminé, et dit comme tel.

---

## 6. L'oblique à la frontière — essai 3

`examples/transfert_oriente.rs -- oblique`. Un paquet part à l'angle `θ` du domaine et traverse une
ligne normale à `x`. **Rien dans l'extraction ne connaît `θ`** : la direction se mesure, par
périodogramme à **deux dimensions** sur `η(y, t)` le long de la ligne. Le signe de `k_y` vient du
couplage espace-temps — une onde qui monte et une onde qui descend ont le même spectre spatial, et
seule leur marche dans le temps les sépare. Éponges sur les **quatre** bords : un paquet oblique
sort par deux côtés.

Maille 25 cm, `λ` = 2 m, trois angles :

| `θ` posé | `θ` **lu** | écart | `k_y` posé | `k_y` **lu** | écart | **miroir transverse** |
|---:|---:|---:|---:|---:|---:|---:|
| 0° | **0,00°** | **0,000°** | 0,0000 | 0,0000 | — | *(sans objet)* |
| 20° | 22,57° | 2,57° | 1,0745 | 1,1318 | 5,3 % | **1,28·10⁻⁶** |
| 40° | 45,10° | 5,11° | 2,0194 | 2,0994 | 4,0 % | **2,09·10⁻⁷** |

### 6.1 Aucune composante transverse artificielle — c'est le résultat le plus net

Le **miroir** est l'énergie de la direction `−k_y`, celle qu'un raccord fabriquerait s'il créait une
composante parasite. Elle vaut **10⁻⁶ à 10⁻⁷** de la composante utile. À 0°, le miroir se confond
avec la composante elle-même et la mesure est sans objet — dit plutôt que masqué par un chiffre.

**Le raccord ne crée pas de direction qui n'existait pas.** C'est le troisième point de la
décision, et il est tenu.

### 6.2 La direction est lue, et l'écart est attribuable

À 0°, la lecture rend **exactement** zéro : le montage ne fabrique pas d'angle. Aux deux autres, la
direction lue dépasse la posée de 2,6° et 5,1°, et **l'écart s'attribue** :

- `k_y` est mesuré **directement** et rend +4 à 5 % — la largeur spectrale d'un paquet de deux
  longueurs d'onde suffit à l'expliquer ;
- `k` vient de `ω²/g`, et `ω` **lue** vaut 5,37 pour 5,55 posée : c'est la **dispersion numérique
  de δ à huit mailles par longueur d'onde**, la même que S314 a mesurée et vue disparaître au
  raffinement (§7 de la preuve de S314 : 4,36 % → 0,83 % → 0,036 %).

Comme `θ = asin(k_y/k)`, un `k` trop petit et un `k_y` trop grand s'additionnent dans le même sens.
**L'écart d'angle n'est donc pas un défaut du raccord** ; il hérite de la maille, et il devrait
suivre la même convergence. *Cela n'a pas été vérifié* : le cas oblique à 12,5 cm coûte huit fois
celui-ci, et n'a pas été exécuté.

### 6.3 Le contrat a refusé le train, et il avait raison

Au premier passage, le banc n'a **pas pu construire** son train : `Error::Envelope`. Son enveloppe
valait une longueur d'onde **posée**, 2,00 m, pour une longueur d'onde **lue** de 2,14 m — la
dispersion de δ allonge l'onde, et le paquet cesse d'être plus large qu'une longueur d'onde. **Le
contrat s'en est aperçu avant nous.**

La chaîne est fermée par l'essai `the_direction_read_at_the_boundary_is_carried_s315`, qui
construit le train sur les `(k_x, k_y)` **mesurés** ci-dessus et vérifie que la direction portée
est la direction **lue**, à 0,05° près, aux trois angles. Le banc a été corrigé pour dimensionner
son enveloppe sur la longueur d'onde lue ; la correction n'a pas été rejouée sur δ.

### 6.4 Ce que l'essai 3 ne couvre pas

- La **composante tangentielle** est vérifiée comme *lue conforme* (`k_y` à 4–5 %), pas comme
  *conservée à travers une interface* : le montage n'a qu'un milieu, donc rien ne peut réfracter.
  La conservation de `k_y` à une **discontinuité** reste à éprouver.
- Trois angles, une seule maille. La convergence de l'écart d'angle avec la maille est **prévue par
  l'attribution de §6.2 et non mesurée**.
- Aucune réflexion n'est mesurée dans ce montage oblique : la jauge de S311 suppose un aller-retour
  normal à la ligne.

---

## 7. Où en est le transfert, propriété par propriété

ADR-183 D5 demande d'**attribuer** chaque écart. Voici l'état à la fin de S315 — c'est l'entrée de
l'ordre C, pas sa conclusion.

| propriété | mesure | **à qui l'écart revient** |
|---|---:|---|
| **amplitude** | 1,7 % *(S314)* ; 6,3 % à 10 λ | raccord ; puis **dissipation de δ** |
| **direction** | part avant **1,00000** ; angle lu à 2,6° et 5,1° | primitive : exacte ; angle : **maille de δ** (§6.2) |
| **composante transverse parasite** | **10⁻⁶ à 10⁻⁷** | **aucun** — le raccord n'en crée pas |
| **spectre** | 3,5 % à 6,25 cm *(S314)* | **maille de δ**, convergent |
| **vitesse de groupe** | 0,035 % à 6,25 cm *(S314)* ; 7,5 % à 25 cm | **maille de δ**, convergent |
| **phase, à l'émission** | régression 0,9938 *(S314)* | raccord : tenue |
| **phase, à dix longueurs d'onde** | **indéterminée** | **instrument** — A307 |
| **volume net** | **100 % en attente** | architecture — receveur B/V non construit |
| **réflexion** | 2,84·10⁻⁷ *(S314)* | éponge de δ |

**Le transfert reste partiel** (ADR-183 D6), et deux lignes disent pourquoi : la phase à distance
n'est pas mesurable par ce montage, et le volume net n'a toujours pas de receveur.

## 8. Limites

Référence CPU, une machine (A98). L'oblique n'a qu'une **maille** (25 cm, huit points par longueur
d'onde) et **trois angles** ; la convergence de l'écart d'angle est attribuée, non mesurée. Le
montage oblique n'a **qu'un milieu** : rien n'y réfracte, donc la conservation de la composante
tangentielle à une **discontinuité** n'est pas éprouvée. Aucune **réflexion** n'est mesurée en
oblique. L'oracle de phase suppose une **enveloppe gaussienne** et une **porteuse unique** des deux
côtés. Le train est émis **une fois** : ni cadence, ni recouvrement. Et rien ici ne reboucle W dans
δ — le couplage reste à un sens (A302).

## Note corrective du 2026-09-21 (S316)

L'ordre C ([ORDRE-C-S316](ORDRE-C-S316.md)), qui mesure à fréquence fixe et déroule la phase, corrige
trois affirmations de cette preuve :

1. **« 6,3 % de dissipation sur dix longueurs d'onde » est faux.** À fréquence fixe, le module de δ
   ne perd rien entre les deux lignes (1,0000 à 25 cm) ; il **gagne** même 0,65 % à 12,5 cm sous
   2 cm, par transfert non linéaire. Le rapport d'amplitudes de §5 comparait les **crêtes** de deux
   paquets de dispersions différentes : une différence d'étalement, pas une perte.
2. **Les prédictions de §2 supposaient `k_δ(ω₀) = k₀`.** Le raccord lit le maximum du spectre
   temporel, la condition initiale pose celui du spectre spatial ; le jacobien `1/c_g` les sépare de
   +0,6 à +1,0 %, d'où 0,04 à 0,07 tour d'écart avec la phase déroulée (L365).
3. **La phase à dix longueurs d'onde n'est plus indéterminée** : déroulée, −0,914 ; −0,237 ;
   −0,045 tour aux trois mailles. Et l'écart d'angle de §6.2 n'est dû à la maille de δ que pour
   moitié : la part de `k_y`, +4 à +5 %, ne converge pas — c'est la lecture de la direction.

La vitesse de groupe de §5.1 (7,5 % sous le train à 12,5 cm) est **confirmée** : 7,2 %.
