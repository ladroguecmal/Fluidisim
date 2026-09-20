# Le transfert orienté δ → W — S314

2026-09-20. **Ordre B** du lot 2, sous [ADR-182](../adr/ADR-182-criteres-de-conservation-actes-et-ordre-b.md)
D7 à D9, qui reprend les cinq essais d'[ADR-181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md) D9.

S312 avait construit un transfert qui tenait l'amplitude (1,24·10⁻⁵) et la réflexion (2,84·10⁻⁷)
et **ratait tout le reste** : la moitié de l'énergie repartait à contresens, le spectre s'étalait
sur deux octaves, la phase était impossible. ADR-182 D8 interdit de compter deux critères tenus
pour une réception ; les cinq essais existent pour que le verdict porte sur les **six** propriétés.

Machine de référence (ADR-174 D1). `src/wave_train.rs`, `examples/etalement_paquet.rs`,
`examples/transfert_oriente.rs`.

---

## 1. Pourquoi une autre famille, et pas un impact avec un paramètre de plus

Les deux champs d'impact de W sont **isotropes par construction** : `RadialImpact` est une somme de
`J₀(k r)`, qui ne dépend que du rayon ; `ImpactField` est un motif périodique symétrique. Les deux
**refusent** toute anisotropie non nulle (S312 §2 bis). Ce n'est pas un réglage manquant, c'est la
forme de la fonction.

La famille qui porte direction, spectre et phase existait déjà dans le dépôt — mais dans **B** :
`background::Component` est une onde plane avec amplitude, nombre d'onde, direction et phase.

```text
η(x,t) = Σ a_m · cos( k_m·(x − origine) − ω_m·(t − naissance) + φ )      ω_m = √(g |k_m|)
```

Chaque mode est une solution **exacte** de la houle linéaire en eau profonde. Direction, spectre et
phase ne sont donc pas approchés : ils sont **portés**. Ce qui est approché est l'échantillonnage
de la bande, et il a son refus.

---

## 2. La question posée avant d'écrire une ligne : l'enveloppe survit-elle ?

Une onde plane ne meurt pas, et tout ce que W produit décroît et s'éteint. Un **paquet**, lui,
meurt — il s'étale. S'il s'étale vite, il devient de la mer en quelques secondes et n'a pas sa
place dans W. `examples/etalement_paquet.rs` le mesure, sur la seule arithmétique de la dispersion,
contre la loi écrite d'avance `σ(t) = σ₀√(1 + (ω''t/σ₀²)²)` avec `ω'' = −¼√g k₀^{−3/2}` :

| `λ₀` = 2 m, `σ₀` = 3 m | élargissement | écart au modèle |
|---:|---:|---:|
| 5 s | ×1,0031 | 0,01 % |
| **10 s** | **×1,0125** | **0,03 %** |
| 20 s | ×1,0490 | 0,13 % |
| 40 s | ×1,1839 | 0,40 % |

`τ = σ₀²/|ω''|` vaut **64 s** pour cette porteuse, 90 s à `λ₀` = 4 m, 128 s à 8 m, et croît comme
`σ₀²`. **Un transfert dure dix secondes ; l'enveloppe en tient soixante.** Deux contrôles
indépendants tombent juste au passage : le centre avance exactement à `cg`, et le produit
`crête × σ` reste à 1 — l'énergie ne se perd pas dans l'étalement, elle s'étale.

**Conséquence de conception, et c'est elle qui fait la primitive** : l'horizon et le rayon d'un
train **se calculent** au lieu d'être choisis, et deviennent deux **refus** — `Horizon` et
`Radius` — qu'aucune autre production de W ne porte.

---

## 3. La primitive

`WaveTrain<N>`, `src/wave_train.rs`. Onze essais unitaires, **un par propriété**, aucune déduite
d'une autre (ADR-182 D7) :

| propriété | mesure |
|---|---|
| amplitude demandée = crête à la naissance | 10⁻⁷ |
| **volume net nul** — W ne porte pas de moyenne (D9) | 10⁻⁵ de l'absolu |
| invariance en travers, secteur nul | **au bit** |
| énergie vers l'avant | **> 10×** l'arrière |
| un quart de tour de phase échange crête et zéro | **exact** |
| vitesse de groupe contre `½√(g/k)` | 1 % |
| **borne de pente atteinte** | rapport 0,9 à 1,0 |

La dernière ligne mérite un mot : dans les champs d'impact, la borne L1 majore la pente réelle d'un
facteur constant qu'il a fallu **mesurer** (1,795 et 1,702, S141–S142). Pour un train à bande
étroite elle est **atteinte** — tous les sinus s'alignent un quart de longueur d'onde après la
crête —, donc aucune constante de conversion à calibrer, et l'essai le vérifie au lieu de le croire.

**Cinq refus, dont trois que la dispersion impose** : `Horizon`, `Radius`, `Resolution`. Le dernier
a rendu un service immédiat : à `N` fixé, **ouvrir le secteur coûte des modes de bande**, et un
secteur de cinq directions est refusé à `N` = 64 — il faut `N` = 256. Le compromis n'est pas
commenté, il est **appliqué**.

---

## 4. Le transfert, en trois gestes

1. **Lire** `η(t)` sur la ligne de contrôle intérieure de S311 — le signal, pas son volume.
2. **L'identifier** par cinq nombres issus du **même** signal : instant d'arrivée et largeur
   d'enveloppe (moments de `η²`), pulsation dominante (**maximum du périodogramme** — §7.1 dit
   pourquoi, et pourquoi pas les passages par zéro), puis **amplitude et phase ensemble**, par
   projection sur les deux quadratures. Aucun n'est ajusté pour qu'un autre tombe juste.
3. **Émettre** un `WaveTrain` qui les porte. Direction, spectre et phase sont des paramètres de
   **construction** ; rien n'est corrigé après coup, et surtout pas l'amplitude (ADR-182 D8).

---

## 5. Essai 2 — le paquet spectral, et les trois limitations de S312

Paquet de S312 : `λ` = 2 m, `h₀` = 2,5 m, `σ` = 3 m, maille 12,5 cm, 5 432 pas.

| propriété | S312, impact | **S314, train** | |
|---|---:|---:|---|
| **direction** — part vers l'avant | 0,500 | **1,00000** | *recul 3,97 σ* |
| **spectre** — bande relative | deux octaves pour 11 % | 0,0980 contre 0,1061 | écart **7,6 %** |
| **phase** — erreur de forme RMS | impossible | **7,5 %** | régression **0,9938** |
| **amplitude** | calibrée en énergie | écart **1,7 %** | |
| **vitesse de groupe** | 12,4 % | **0,83 %** | |
| **réflexion** *(mesurée à part)* | 2,84·10⁻⁷ | **2,84·10⁻⁷** | seuil 1 % |
| **volume net** | 100 % en attente | **100 % en attente** | ADR-182 D9 |

**Les trois limitations que l'utilisateur avait nommées sont corrigées**, et la réflexion n'a pas
bougé — elle est une propriété du raccord côté δ, pas du transfert, et il fallait que le changement
de primitive ne la dégrade pas.

L'erreur de forme de 7,5 % est une mesure **point à point** du signal reconstruit contre le signal
mesuré, sur 1 263 instants. Elle n'est ni une erreur d'amplitude (1,7 %) ni une erreur de phase —
le coefficient de régression du modèle sur le signal vaut 0,9938, et une phase fausse l'effondrerait.
C'est ce que le modèle à enveloppe **gaussienne** ne capture pas de la forme réelle : la traîne
dispersive que le domaine a produite.

*Ce coefficient est une **régression**, `⟨modèle·signal⟩/⟨signal²⟩`, et non une corrélation
normalisée : il vaut 1 quand le modèle a la bonne amplitude **et** la bonne phase, et il peut
dépasser 1 si l'amplitude est légèrement forte. C'est dit ici pour qu'on ne le lise pas comme un
coefficient borné.*

## 6. Essai 1 — l'onde progressive quasi monochromatique

Enveloppe large (`σ` = 6 m, soit `kσ` = 18,8 et une bande de 5,3 %), maille 25 cm.

| propriété | mesure | |
|---|---:|---|
| **amplitude** | **1,15·10⁻³** | quasi exacte |
| **phase** — erreur de forme | 7,6 % | régression **0,9734** |
| **direction** — part vers l'avant | 0,99420 | **recul 1,80 σ seulement** |
| **spectre** — bande relative | 0,0452 contre 0,0531 | écart 14,7 % |
| **vitesse de groupe** | 4,7 % | |
| **réflexion** | 6,19·10⁻⁶ | seuil 1 % |

**L'amplitude à 0,1 % est le résultat de cet essai** : sur une porteuse quasi pure, le transfert
rend exactement ce que la ligne a vu. Les deux estimateurs de fréquence s'accordent ici à 0,12 %
(5,3011 contre 5,3077), ce qui est attendu — à maille grossière le schéma amortit les courtes qui
biaisent le comptage de zéros (§7.1).

**Deux réserves, et elles portent sur le montage, pas sur le transfert.**

1. **La part vers l'avant est un minorant.** La mesure exige que le paquet ait reculé de plus de
   trois écarts-types de son point d'émission ; ici il n'en a reculé que de **1,80**, l'enveloppe
   étant deux fois plus large que celle des autres cas. La valeur vraie est supérieure à 0,994 ;
   les cas qui satisfont la condition (3,4 à 4,2 σ) donnent **1,00000**. *La condition est publiée
   à côté du nombre, comme ADR-182 D5 l'exige.*
2. **Les écarts de spectre (14,7 %) et de vitesse (4,7 %) viennent de la maille**, pas du raccord.
   À 25 cm — huit mailles par longueur d'onde — le schéma de δ produit une onde **réellement** plus
   longue et plus lente que celle qu'on a posée, et §7 le démontre en la raffinant : les mêmes
   écarts tombent à 3,5 % et 0,035 % à 6,25 cm.

## 7. Essai 5 — plusieurs résolutions, et un instrument qui se dégradait

Même paquet, trois mailles. Les mesures sont celles du **transfert**, mais ce qu'elles montrent est
la convergence du **domaine**.

| maille | `ω` lue *(5,5515 posée)* | écart `cg` | erreur de forme | régression | écart de bande | part avant |
|---:|---:|---:|---:|---:|---:|---:|
| 25 cm | 5,3094 *(−4,36 %)* | 4,56 % | 13,3 % | 0,9659 | 19,4 % | 1,00000 |
| 12,5 cm | 5,5056 *(−0,83 %)* | 0,83 % | 7,5 % | 0,9938 | 7,6 % | 1,00000 |
| **6,25 cm** | **5,5495** *(−0,036 %)* | **0,035 %** | **5,7 %** | **1,0038** | **3,5 %** | 1,00000 |

**Les six grandeurs convergent, et de façon monotone.** L'écart de célérité tombe d'un facteur 5,5
puis **24** ; l'écart de bande de 2,5 puis 2,2 ; l'amplitude de 3,2 % à 1,7 % puis **1,1 %**. Ce que
le transfert reproduit est donc **ce que le domaine a réellement produit**, et l'écart résiduel à
l'onde *posée* est la **dispersion numérique de δ** — qui s'efface quand on raffine.

### 7.1 Il a fallu deux passages, et le premier accusait le mauvais coupable

**Au premier passage, le maillage le plus fin était le pire.** La pulsation lue passait de −4,1 % à
−0,45 % puis **+4,7 %** — un **changement de signe** —, l'erreur de forme sautait à 60 % et la
régression tombait à 0,66. Deux points suggéraient une convergence nette ; le troisième la
détruisait.

La cause n'était pas le schéma. Elle était dans l'**estimateur de fréquence** : les deux premiers
passages comptaient les **passages par zéro** du signal de jauge. Cet estimateur compte *toutes*
les traversées — une traîne courte, une ride résiduelle — et rend une période trop brève. Or une
maille fine **amortit moins** les courtes : l'estimateur se dégrade **exactement quand le domaine
s'améliore**.

Mesuré, les deux estimateurs côte à côte sur le même signal :

| maille | passages par zéro | **maximum du périodogramme** | écart entre les deux |
|---:|---:|---:|---:|
| 25 cm | 5,3227 | 5,3094 | 0,25 % |
| 12,5 cm | 5,5263 | 5,5056 | 0,38 % |
| **6,25 cm** | **5,8110** *(+4,7 %)* | **5,5495** *(−0,036 %)* | **4,7 %** |

Les deux s'accordent tant que le signal est amorti, et **divergent là où il ne l'est plus**. Le
périodogramme ne compte rien : il cherche la fréquence qui explique le plus d'énergie, et il est
insensible à ce qui traverse zéro sans porter d'énergie.

**Portée pour le dépôt.** `examples/transfert_paquet.rs` (S312) emploie l'estimateur par passages
par zéro. À sa maille — 12,5 cm — l'écart entre les deux estimateurs vaut **0,38 %**, donc ses
conclusions tiennent ; mais son `λ_mesure` = 2,0183 m doit être lu comme 2,034 m au périodogramme.
Aucun autre banc du dépôt n'en dépend pour un verdict.

### 7.2 La réflexion

5,67·10⁻⁶, 2,84·10⁻⁷ et 7,95·10⁻⁷ aux trois mailles — non monotone, et **quatre à six ordres sous
le seuil de 1 %** partout. Elle est une propriété de l'éponge, mesurée séparément depuis S311, et
le changement de primitive ne l'a pas touchée.

## 8. Ce qui n'est pas fait, et ne doit pas être compté comme fait

- **Essai 3, la propagation oblique, n'a pas été réalisé.** Il demande un domaine large en `y`,
  donc de dix à vingt fois le coût des cas de cette session, et la primitive le **supporte** —
  `spread_turns`, `directions`, et un essai unitaire qui vérifie qu'un secteur ouvert pointe encore
  vers l'avant — mais **supporter n'est pas éprouver**. L'essai reste dû.
- **Essai 4, la cohérence de phase, est mesurée sur la ligne d'émission** (corrélation 0,9907 et
  0,9733), pas sur un point distant : ce qui est établi est que le train **part** avec la phase du
  signal sortant, pas qu'il la conserve à dix longueurs d'onde. Le second demande un oracle que
  cette session n'a pas construit.
- **Le champ de W n'est pas rebouclé dans δ.** Le couplage reste à un sens (A302).
- **Le volume net reste entièrement en attente**, et rien dans la primitive ne porte de moyenne :
  c'est ADR-182 D9, et c'est vérifié par un essai unitaire, pas supposé.
- **Le transfert n'est pas déclaré validé.** Quatre essais sur cinq, et deux réserves de montage
  sur l'essai 1. ADR-182 D8 : deux critères tenus ne font pas une réception, et quatre non plus
  tant que le cinquième manque.

## 9. Limites

Référence CPU, une machine (A98). Cas **unidirectionnels et invariants en `y`** : rien n'est dit
d'un front oblique ni d'une frontière courbe. L'identification suppose une enveloppe **gaussienne**
et une **porteuse unique** — un signal à deux trains superposés serait lu comme un seul, et le banc
ne le détecterait pas. Le train est émis **une fois**, à l'instant d'arrivée : ni cadence, ni
recouvrement, ni émission continue. `SPREAD_LIMIT` = 10 % est **choisi**, et dit comme tel dans le
module ; ce n'est pas une limite physique mais la frontière du domaine où le champ est celui qu'on
a demandé.
