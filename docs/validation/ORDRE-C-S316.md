# L'ordre C : six propriétés d'un même transfert, chacune attribuée — S316

2026-09-21. **Ordre C** du lot 2 ([ADR-181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md)
D10), sous la décision de l'utilisateur du 2026-09-20
([ADR-183](../adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md) D5) : *« évaluez
conjointement l'amplitude, la direction, le spectre, la phase, la vitesse de propagation et la
réflexion artificielle. Distinguez les résultats propres à la primitive W, ceux du raccord et ceux
de la simulation δ. »*

Machine de référence (ADR-174 D1), référence CPU de δ. `examples/transfert_oriente.rs`, modes
`instrument`, `primitive`, `ordre_c <maille> <pas µs> <amplitude>`, `oblique_maille` ;
`water-harness`, essais `a306_*`.

**En une phrase.** La primitive est exacte à 10⁻⁴ ; le raccord portait deux fuites de l'onde posée,
corrigées, et ne garde qu'une limite de modèle ; tout le reste de l'écart **converge avec la maille
de δ** — sauf une part que δ a raison de porter et que W, linéaire, n'a pas.

---

## 1. Comment un écart s'attribue — la règle, écrite avant la mesure

| attributaire | critère |
|---|---|
| **δ** | l'écart **converge** quand la maille s'affine, 25 → 12,5 → 6,25 cm |
| **raccord** | l'écart est présent **dès la ligne d'émission** et ne converge pas |
| **primitive** | l'écart existe **sans δ ni ligne** : le train contre l'évolution exacte de sa demande |
| **instrument** | l'écart vient de la façon de mesurer ; il se démontre sur un défaut **connu** |

La mesure en a imposé un cinquième, que la règle ne prévoyait pas : **la physique que W ne porte
pas** (§5.3).

## 2. L'instrument : la phase se déroule le long du trajet — A307

S315 comparait deux phases **enroulées**, lues aux deux extrémités, chacune à sa propre pulsation
dominante, puis ramenées à un instant commun : l'erreur de chaque pulsation, multipliée par vingt
secondes, devenait de la phase, et le nombre de tours restait inconnu.

L'instrument de S316 fait autrement. À **fréquence fixe**, la projection `Z(x, ω) = Σ η(x,t)·e^{iωt}·dt`
d'une onde `A cos(ωt − kx + φ)` a pour argument `kx − φ`. Donc :

- la pente de sa phase **en `x`** est le nombre d'onde à cette fréquence ;
- l'écart `D(x) = arg Z_W − arg Z_δ`, pris sur la **même fenêtre**, ne contient **aucun terme de
  temps** ;
- `D` varie lentement d'une colonne à la voisine, donc il **se déroule** sans ambiguïté — le plus
  grand saut est mesuré et publié, jamais supposé ;
- le retard de groupe est la pente **en `ω`** de la phase, et il se lit sans ajuster d'enveloppe.

**Éprouvé sur un défaut connu avant d'être appliqué à δ** (principe d'ADR-181 D6). Deux trains
exacts, l'un sous `g`, l'autre sous `g(1+ε)²` avec ε = −4,36 % — ce que la dispersion de δ fait à
25 cm :

| | attendu | lu |
|---|---:|---:|
| écart de phase à 10 λ, déroulé | −0,85299 tour | **−0,85299** |
| même écart, lu aux seules extrémités | | **+0,14701** |
| retard de groupe | −2,0189 s | **−2,0189 s** |
| décalage d'émission | 0 | **0,0000 s** |

**Une impasse qui vaut pour la suite.** Pris **entre les deux lignes**, le retard ratait de 11 %
(−1,797 s). Les relevés commencent à la naissance du train, donc **coupés au milieu de l'enveloppe**
à la première ligne, et une gaussienne coupée porte son propre retard, `σ_t·√(2/π)` — différent
pour deux paquets de vitesses de groupe différentes. Le retard se lit donc comme une **pente en
`x`** sur les stations au relevé complet (au-delà de 4,5 enveloppes), l'ordonnée ramenée à la ligne
donnant le décalage d'émission. La phase à `ω₀`, elle, n'est pas biaisée par la coupure.

**Et une seconde, dans l'autre direction du déroulement.** Chaque fréquence se déroulait en `x`
depuis sa valeur **à la ligne**, connue modulo un tour ; deux fréquences voisines pouvaient y tomber
de part et d'autre de ±½. Le raccord S315 a lu ainsi **+4,8 s** de retard d'émission, pour −0,06 s
au S316. On déroule désormais d'abord **en `ω`** à la ligne, puis chaque fréquence en `x` depuis
cette ancre : +0,07 s.

## 3. La primitive seule ne porte que 10⁻⁴

Le train est une somme **discrète** de 128 modes, bande tronquée à ±4 écarts-types, phase en
virgule fixe, amplitudes en `f32`. On le compare à l'évolution exacte de **sa propre demande**,
calculée par quadrature continue en `f64` sur ±8 écarts-types — une autre écriture, pas le train
relu :

| instant | erreur de forme | écart maximal |
|---|---:|---:|
| naissance | 1,02·10⁻⁴ | 1,8 µm |
| après 40 s, dix longueurs d'onde | 1,04·10⁻⁴ | 1,7 µm |

C'est la troncature de la bande, `√erfc(4)` ≈ 1,2·10⁻⁴, et elle **ne croît pas** en se propageant.
Sur le banc de δ, le module du train à fréquence fixe est conservé à **1,000002–1,000005** entre le
premier relevé complet et la dernière station. En usage : 1,8 µm pour 3 mm de tolérance d'image. **La
primitive ne porte aucune part mesurable des écarts du transfert.**

## 4. Le raccord : deux fuites de l'onde posée, et une fenêtre de banc

Lus dans le code avant la mesure, puis mesurés sur le même relevé de δ, raccord de S315 contre
raccord corrigé :

1. **La largeur temporelle devenait spatiale par la vitesse de groupe de l'onde *posée*** — une
   information qu'une scène réelle ne donne pas. Le raccord S316 prend celle de l'onde **émise**,
   `g/(2ω)`.
2. **Le train naissait sur la face de la ligne, la jauge lisait le centre de la colonne**, une
   demi-maille plus loin. Mesuré : l'écart de phase à l'émission du S315 dépasse celui du S316 de
   **0,059, 0,032 et 0,016 tour** aux trois mailles — `k·dx/2` (0,057 ; 0,031 ; 0,016) —, et son
   retard d'émission de **0,134, 0,070 et 0,035 s** — `dx/(2c_g)` exactement.

*Portée pour le passé* : la régression de phase de S314 à l'émission (0,9659 à 25 cm) portait ce
décalage d'une demi-maille.

**Et une fenêtre de banc.** Le protocole de retour de S311 cale sa fenêtre sur la vitesse posée. À
25 cm, δ va **25 % moins vite** : la fenêtre de retour lisait la traîne du passage direct. Le banc
décide désormais sa durée sur la vitesse **mesurée** entre la première et la dernière station.

## 5. La phase et la vitesse à dix longueurs d'onde

### 5.1 Déroulée, la phase n'est plus indéterminée

Montage de S315 — deux lignes séparées de 20 m, paquet `λ` = 2 m, σ = 3 m —, prolongé de 10 m de
stations avant l'éponge, relevé sur **toutes** les colonnes. Trois mailles, deux amplitudes. Écart
`D(x₂) − D(x₁)` à `ω₀`, en tours, raccord S316 :

| maille | 2 cm | 1 cm | part linéaire (`a → 0`) | part non linéaire, 2 cm | saut maximal |
|---:|---:|---:|---:|---:|---:|
| 25 cm | **−0,9142** | −0,9160 | −0,9166 | +0,0024 | 0,097 |
| 12,5 cm | **−0,2372** | −0,2422 | −0,2438 | +0,0067 | 0,012 |
| 6,25 cm | **−0,0452** | −0,0524 | −0,0548 | +0,0096 | 0,0014 |

À 25 cm, lue aux extrémités, la même phase aurait rendu **+0,086** : c'est l'indétermination de
S315, levée. La **part linéaire converge à l'ordre deux** (rapports 3,76 puis 4,45 ; ordres 1,91 et
2,15) : c'est la dispersion numérique de δ. Le pas de temps, fixé à 10 ms aux trois mailles, est
éprouvé à part, à 6,25 cm sous 5 ms : **en calcul à la publication de cette preuve** ; son
résultat s'ajoute ici au commit P4c.

### 5.2 Les prédictions de S315 manquées, et la raison est physique

S315 prédisait **−0,853 ; −0,165 ; −0,0072** en supposant que δ, à la pulsation que lit le raccord,
garde le nombre d'onde de sa condition initiale. Mesuré : `k_δ(ω₀)` le dépasse de **+1,0 %** et
**+0,64 %**. Le raccord lit le maximum du spectre **temporel** ; la condition initiale pose celui
du spectre **spatial** ; les deux diffèrent du jacobien `dk/dω = 1/c_g`, qui croît avec `k`. Le
décalage attendu, `σ_k²/(2k)`, vaut +0,56 % pour σ = 3 m. **Ce n'est pas un défaut du transfert** :
c'est la différence entre deux définitions de « la » longueur d'onde d'un paquet (L365).

### 5.3 Ce qui converge est δ ; ce qui reste est de la physique que W n'a pas

Le paquet a `a·k` = 0,063 ; sa pulsation porte la correction de Stokes `(ak)²/2`, que W — linéaire —
ne porte pas. La session l'a d'abord supposée **indépendante de la maille**, et la demi-amplitude l'a
contredit : la part non linéaire de δ **croît** avec la résolution — 6 %, 17 % puis 24 % de la
correction de Stokes pleine (0,040 tour, un majorant : le paquet s'étale de 25 % et son amplitude
effective baisse). δ résout de mieux en mieux une physique réelle ; W ne l'a pas.

C'est une **cinquième colonne** d'attribution : ni le raccord, ni l'erreur de δ, mais la frontière
du modèle de W — et elle se voit dès dix longueurs d'onde.

### 5.4 La vitesse de groupe

Retard de groupe à 10 λ, lu comme la pente en `x` de `∂D/∂ω` sur les relevés complets :

| maille | retard du train sur δ | `c_g` de δ contre l'onde émise | `c_g` de δ par transit |
|---:|---:|---:|---:|
| 25 cm | −7,37 s | −25,3 % | 0,692 m/s |
| 12,5 cm | −1,75 s | −7,2 % | 0,829 m/s |
| 6,25 cm | −0,38 s | −1,65 % | 0,871 m/s |

Rapports 4,2 et 4,6. Deux prédictions concurrentes avaient été écrites pour 12,5 cm : 1,66 s (lu
par S315 sur les enveloppes) et 0,93 s (sous une erreur de dispersion en `(k·dx)²`). **La seconde
est réfutée.** Attribution : **δ**.

**En usage** (précision rapportée à l'usage) : à 6,25 cm, −0,045 tour sur 20 m, c'est 9 cm de
décalage de crête (0,45 %) et, pour une vague de 2 cm, 5,7 mm d'écart de hauteur au bout de dix
longueurs d'onde — au-dessus des 3 mm d'image. Mais c'est l'écart de **δ** contre la physique
exacte, sur sa propre longueur : au-delà de la ligne, **W porte la dispersion exacte**. Le
transfert ne fait pas perdre la phase ; il **arrête** de la perdre.

## 6. Amplitude : ce que S315 appelait dissipation

À fréquence fixe, un milieu sans perte conserve `|Z|` le long du trajet, quelle que soit la
dispersion — l'étalement change la phase, pas le module. Mesuré sur δ entre les deux lignes :

| maille | 2 cm | 1 cm |
|---:|---:|---:|
| 25 cm | 1,0000 | 1,0000 |
| 12,5 cm | 1,0065 | 1,0017 |
| 6,25 cm | 1,0109 | 1,0030 |

**δ ne dissipe rien de mesurable à la fréquence centrale**, et il y **gagne** en `a²` — un transfert
non linéaire vers le centre du spectre, que W ne porte pas. Les « 6,3 % de dissipation » de S315
comparaient les **crêtes** de deux paquets de dispersions différentes : une différence
d'étalement, pas une perte (note corrective ajoutée à S315).

Amplitude **à l'émission**, train contre δ à `ω₀` : 1,0276 ; 1,0102 ; 1,0057 sous 2 cm, 1,0273 ;
1,0119 ; 1,0078 sous 1 cm — converge vers ≈ 1,003. Train le long du trajet : 1,000004 ; 1,000002 ;
0,999985 — la primitive, exacte.

## 7. Spectre

Spectre du train (relevé complet, sans perte) contre celui de δ à la ligne, sur ±10 % autour de
`ω₀` :

| maille | écart, 2 cm | écart, 1 cm | centroïde | largeur |
|---:|---:|---:|---:|---:|
| 25 cm | 7,3 % | 8,4 % | +0,29 % | −7,5 % |
| 12,5 cm | 4,0 % | 4,3 % | +0,24 % | −2,6 % |
| 6,25 cm | 2,8 % | 4,2 % | +0,19 % | −1,5 % |

L'écart converge vers **2 à 4 %**, et le centroïde reste décalé de +0,2 % : c'est la part du
**raccord**. Sa nature : une porteuse sous enveloppe gaussienne, identifiée par les moments de
`η²`, ne décrit ni le **chirp** qu'un paquet acquiert en se dispersant sur les 12 m qui précèdent la
ligne, ni l'asymétrie du jacobien. Le spectre de δ à la ligne dépend lui-même de l'amplitude à
6,25 cm (2,8 % contre 4,2 %) : le transfert non linéaire de §6 a commencé avant la ligne.

*La première fuite, mesurée* : la vitesse posée **aidait par accident** la largeur aux mailles
grossières (−3,5 % contre −7,5 % à 25 cm), en compensant l'erreur de δ par une information qu'une
scène n'a pas ; à 6,25 cm, les deux raccords coïncident (−1,47 contre −1,51 %).

## 8. Réflexion, par sens de propagation

Le protocole de S311 lisait la réflexion comme l'énergie qui passe à **une** jauge après le retour
attendu. Sur ce banc, cette lecture rend **1,1·10⁻⁴** à 25 cm et **1,4·10⁻⁴** à 12,5 cm — et elle
varie comme `a²` (×3,95 entre 1 et 2 cm) : ce n'est pas de la réflexion. Ce sont les ondes courtes
de δ, si lentes qu'elles passent **vers l'avant** pendant la fenêtre de retour, et des composantes
d'ordre deux (L364).

Séparée par sens sur les 10 m de stations du prolongement — `Z(x) = R·e^{ikx} + L·e^{−ikx}` à chaque
fréquence, `k` étant celui de δ, mesuré :

| maille | réfléchi pendant le passage | réfléchi au retour, 2 cm | au retour, 1 cm |
|---:|---:|---:|---:|
| 25 cm | 1,4·10⁻⁸ | 9,6·10⁻⁸ | 1,0·10⁻⁷ |
| 12,5 cm | 7,4·10⁻⁹ | 8,1·10⁻⁷ | 8,2·10⁻⁷ |
| 6,25 cm | 1,3·10⁻⁸ | 1,5·10⁻⁶ | 1,5·10⁻⁶ |

Indépendante de l'amplitude, donc linéaire ; **quatre ordres sous 1 %** ; elle **croît** avec la
maille. Attribution : **éponge de δ**. La réflexion de S314 à 25 cm (5,7·10⁻⁶), lue avec une fenêtre
calée sur la vitesse posée, n'est pas fiable (note corrective ajoutée à S314) ; ses conclusions
tiennent.

## 9. Volume net

Ramené au **mètre de crête** — la largeur du domaine vaut `2·dx` et change avec la maille — :
2,57 ; 1,96 ; 1,68·10⁻⁴ m³/m sous 2 cm, **×4,0 exactement** entre 1 et 2 cm aux trois mailles.
Grandeur d'**ordre deux** en amplitude, limite extrapolée ≈ 1,5·10⁻⁴ m³/m. W n'a pas de moyenne : le
volume reste **intégralement dans `pending`** (ADR-183 D7), et son receveur, B ou V (ADR-181 D1),
est l'ordre D.

## 10. Direction — l'oblique à 12,5 cm

Le banc oblique de S315, inchangé, à maille moitié. Rien dans l'extraction ne connaît l'angle ; la
direction se lit par périodogramme à deux dimensions sur `η(y, t)` le long de la ligne.

| `θ` posé | lu à 25 cm | lu à 12,5 cm | `ω` lue à 12,5 cm | `k_y`, 25 / 12,5 cm | miroir transverse, 25 / 12,5 cm |
|---:|---:|---:|---:|---:|---:|
| 20° | 22,57° | **21,03°** | +0,12 % | +5,3 % / **+5,2 %** | 1,3·10⁻⁶ / **1,2·10⁻⁷** |
| 40° | 45,10° | **41,85°** | +0,25 % | +4,0 % / **+4,3 %** | 2,1·10⁻⁷ / **2,2·10⁻⁸** |

**L'écart d'angle converge pour moitié.** Sa part `ω` suit δ — et, pour ce paquet court (σ = 1 λ),
le décalage du jacobien de §5.2, qui porte `ω` lue au-dessus de la posée. Sa part `k_y`, +4 à +5 %,
**ne bouge pas avec la maille** : c'est la lecture de la direction par le maximum du périodogramme
sur un paquet de largeur angulaire finie. Le train est construit sur la direction lue et la porte
exactement (41,855°) : **le raccord transmet fidèlement une direction qu'il lit biaisée d'environ
+1 à +2°**. S315 attribuait tout l'écart à la maille de δ ; c'était juste pour moitié.

La composante transverse parasite **décroît** avec la maille : le raccord n'en crée aucune.

Prédictions écrites avant le lancement : 20,9–21,3° à 20°, **tenue** ; 42,0–43,1° à 40°, **manquée
de 0,15°** — elles supposaient l'erreur de `ω` du cas 1D, alors qu'ici `ω` passe au-dessus de la
posée.

## 11. A306 — le harnais relu par un second estimateur

Critère écrit avant : une réception a pu être affectée si l'écart entre les deux estimateurs de
période **dépasse sa marge**. Trois essais permanents, `a306_*`, sur le **même signal** que chaque
réception :

| emploi | réception | écart entre estimateurs | marge |
|---|---|---:|---:|
| `physics_shallow.rs`, C03 sur `Shallow1D` | C03-T, C03-T-mur | 0,038 %, 0,039 % | 1,00 % |
| `physics.rs`, `mesurer_seiche` sur `Delta1D` | C03, C03-mode | 0,035 %, 0,037 % | 1,00 % |
| `physics_dispersif.rs`, milieu vérifié | disp-λ32, λ16, λ8 | 0,23 % | 1,00 % |

**Aucune réception affectée.** Et c'est le **périodogramme** qui se trompe ici : sur une sinusoïde
pure de 8 périodes, la fuite de sa fenêtre le biaise de −0,236 % (−0,038 % à 20 périodes),
reproduit à part. Le mécanisme d'A306 exige une composante courte qui traverse zéro sans porter
d'énergie ; les signaux de **mode** du harnais n'en ont pas. Le cinquième emploi, C02 sur B, lit une
composante unique analytique. **A306 est close pour le harnais** ; le remède de L360 reste vrai,
avec son corollaire : le second estimateur a son propre biais.

## 12. Le tableau d'attribution

| propriété | valeur à 6,25 cm | comportement | **attributaire** |
|---|---:|---|---|
| amplitude, primitive | 0,999985 | exacte | **primitive : rien** |
| amplitude à l'émission | 1,0057 | converge vers ≈ 1,003 | δ ; **raccord ≤ 0,3 %** |
| amplitude le long du trajet | +1,1 % | en `a²`, part linéaire ≈ 0 | **physique que W n'a pas** |
| spectre | 2,8 % ; centroïde +0,19 % | limite 2 à 4 % | δ ; **raccord** |
| phase à l'émission | 0,0027 tour (1°) | ne converge plus | **raccord** |
| phase à 10 λ, linéaire | −0,055 tour | ordre deux | **δ** |
| phase à 10 λ, non linéaire | +0,010 tour | croît avec la maille | **physique que W n'a pas** |
| retard à l'émission | −0,03 s | ne converge plus | **raccord** |
| vitesse de groupe | −1,65 % | ordre ≈ 2,1 | **δ** |
| direction, lecture | +1,0° à 20°, +1,9° à 40° *(12,5 cm)* | part `k_y` constante, +4 à +5 % | **raccord** (lecture) ; δ pour la part `ω` |
| composante transverse | 10⁻⁶–10⁻⁷ *(S315)* | | **aucun** |
| réflexion | 1,5·10⁻⁶ | indépendante de `a` | **éponge de δ** |
| volume net | 100 % en attente | `a²`, ≈ 1,5·10⁻⁴ m³/m | **architecture — ordre D** |

**Ce qui revient au raccord, et c'est peu** : un degré de phase, trois centièmes de seconde, trois
millièmes d'amplitude, quelques pour cent de spectre. **Ce qui revient à la primitive : rien.**
**Tout le reste converge avec la maille de δ**, sauf la part non linéaire, que δ a raison de porter.

## 13. Limites

Référence CPU, une machine (A98). Un seul paquet — `λ` = 2 m, σ = 1,5 λ, eau profonde, `a·k` ≤ 0,063 —,
une seule ligne, émission **unique** : ni cadence, ni recouvrement de trains successifs. Trois
mailles, deux amplitudes : les limites extrapolées reposent sur trois points (L361). Le train est
relevé **depuis sa naissance** : les stations à moins de 4,5 enveloppes de la ligne n'entrent pas
dans les pentes. Le couplage reste à **un sens** (A302) ; rien ici ne reboucle W dans δ. La
profondeur finie n'est pas touchée (ADR-181 D9). A305 (bande, éponge) reste ouverte.
