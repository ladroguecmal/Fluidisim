# S193 — Conditions de surface non linéaires et référence de Stokes

2026-09-12. S192-1. **Dérivation et protocole avant code et mesure**.
Véhicule de réception, pas choix technologique B3, pas sélection de solveur δ.
Le seuil B4 de 2 % est fixé (ADR-120) et n'est pas redemandé.

## 1. Le problème exact et sa forme canonique

Liquide incompressible, irrotationnel, inviscide. Domaine périodique `x∈[0,L]`, fond plat
imperméable en `z=−h`, surface libre `z=η(x,t)`, pression atmosphérique nulle en surface.

```
Δφ=0            dans −h<z<η(x,t)
φ_z=0           en z=−h
ψ(x,t)=φ(x,η,t)                        (trace de Dirichlet à la surface)
```

Soit `W=φ_z|_{z=η}` la vitesse verticale **à la surface réelle**. Les deux relations de
surface s'écrivent alors sans approximation :

```
η_t = W(1+η_x²) − η_x ψ_x
ψ_t = −gη − ½ψ_x² + ½W²(1+η_x²)
```

**Dérivation**, parce qu'aucune de ces formes n'est reprise d'un document du dépôt. Par la
règle de dérivation composée, `ψ_x = φ_x|_η + η_x W`, donc `φ_x|_η = ψ_x − η_x W`. La
condition cinématique `η_t = W − η_x φ_x|_η` donne directement
`η_t = W(1+η_x²) − η_x ψ_x`. La condition de Bernoulli en surface
`φ_t|_η + ½(φ_x²+φ_z²)|_η + gη = 0`, avec `ψ_t = φ_t|_η + η_t W`, donne

```
ψ_t = −gη + η_t W − ½(ψ_x−η_x W)² − ½W²
    = −gη − ½ψ_x² + W²(1+η_x²) − ½η_x²W² − ½W²
    = −gη − ½ψ_x² + ½W²(1+η_x²)
```

le terme croisé `η_x ψ_x W` s'annulant exactement entre les deux premières lignes.

**Ce que S192 en gardait.** Le véhicule linéaire de S192 est le cas `W=G_hψ`, `η_t=W`,
`ψ_t=−gη` : les trois produits `η_xψ_x`, `½ψ_x²`, `½W²(1+η_x²)` sont absents, et `W` est
évalué au plan moyen et non à la surface réelle. **C'est exactement ce que cette session
ajoute**, et rien d'autre : même physique, même fond, même profondeur finie, aucun terme
visqueux, aucun déferlement, aucune mousse, aucun air.

**Ce qui reste inchangé et doit rester dit.** Le système ci-dessus suppose l'irrotationnel,
l'incompressible, et une surface **graphe** — une fonction de `x` — donc aucun rouleau,
aucun retournement, aucune inclusion d'air. Ces hypothèses ne sont pas des choix de
discrétisation : elles bornent ce qu'un véhicule de cette famille pourra jamais recevoir.

## 2. Références de Stokes, et leur domaine de validité

L'oracle est **analytique et extérieur au candidat** : il ne partage ni discrétisation, ni
solveur, ni condition aux limites avec lui.

### 2.1 Profil d'ordre deux, profondeur finie

Onde progressive, `θ=kx−ωt`, `k=2π/L`, amplitude de premier ordre `a`, `ω²=gk tanh(kh)` :

```
η = a cos θ + k a² b₂(kh) cos 2θ ,   b₂(kh) = cosh(kh)(2+cosh 2kh) / (4 sinh³(kh))
```

Source : Dean & Dalrymple, *Water Wave Mechanics for Engineers and Scientists*, théorie de
Stokes du second ordre ; le cas de profondeur infinie est déjà dans **SPEC-001 §1 ter** avec
sa source, sous la forme `η=a cos θ+(ka²/2)cos 2θ`.

**Deux limites indépendantes vérifient `b₂`, et elles sont la provenance exigée par I-14.**
En profondeur infinie, `cosh(kh)≈e^{kh}/2`, `cosh 2kh≈e^{2kh}/2`, `sinh³(kh)≈e^{3kh}/8`,
donc `b₂→1/2` : on retombe exactement sur SPEC-001 §1 ter. En faible profondeur,
`b₂→3/(4(kh)³)`, donc

```
η₂/η₁ = b₂ k a → 3a / (4h (kh)²) = 3 a L² / (16 π² h³)
```

c'est-à-dire **proportionnel au nombre d'Ursell** `U = aL²/h³`. La condition de validité
classique de Stokes, `U ≪ 1`, sort donc de la formule elle-même et non d'une convention
ajoutée. Les deux limites sont calculées ici ; elles ne sont pas citées.

### 2.2 Fréquence d'ordre trois

La dépendance de la fréquence à l'amplitude est un effet du **troisième** ordre. En
profondeur infinie, résultat classique de Stokes :

```
ω = ω₀ (1 + ½ (ka)²) ,   ω₀ = √(gk)
```

**C'est le seul oracle de fréquence adopté par cette session.** En profondeur finie, la
forme usuelle, avec `σ = tanh(kh)`,

```
ω² = gkσ [ 1 + (ka)² (9 − 10σ² + 9σ⁴)/(8σ⁴) ]
```

se réduit bien à la précédente pour `σ=1` — le facteur vaut `(9−10+9)/8 = 1` — mais
**elle n'est pas adoptée comme oracle**, pour une raison qui n'est pas de la prudence
d'écriture : à l'ordre trois, en profondeur finie, la fréquence d'une onde de Stokes
**dépend de la convention de courant moyen** (vitesse eulérienne moyenne nulle sous le
creux, ou flux de masse moyen nul). Les deux conventions diffèrent d'un terme du même ordre
que la correction mesurée. Un candidat périodique à potentiel périodique — le nôtre — ne
peut porter **aucun** courant moyen : sa convention est imposée par sa représentation, elle
n'est pas choisie. Comparer sa fréquence à une formule dont la convention n'est pas
déclarée, c'est comparer deux grandeurs différentes. Les profondeurs finies seront donc
**relevées et publiées**, avec la formule ci-dessus en regard à titre indicatif, sans
verdict.

### 2.3 Domaine de validité, et ce qu'il interdit

La référence de Stokes est bornée des deux côtés, et les deux bornes mordent :

| borne | expression | ce qu'elle exclut |
|---|---|---|
| cambrure | `ka ≲ 0,44` en profondeur infinie ; `H/λ ≈ 1/7` (SPEC-001) | les fortes amplitudes, le déferlement |
| Ursell | `U = aL²/h³ ≪ 1` | **la faible profondeur à amplitude utile** |

La seconde est la plus contraignante ici, et elle a une conséquence que le protocole doit
assumer **avant** de mesurer : à `L=8 m` et `h=0,25 m`, `U ≪ 1` exige `a ≲ 2,4·10⁻⁵ m`,
soit `ka ≲ 1,9·10⁻⁵`. Le signal non linéaire relatif y vaut `b₂ka ≈ 2·10⁻³` — mesurable en
`f64`, mais dans un régime où la théorie de Stokes est à la limite de son propre domaine.
**Le cas peu profond de S192 n'est donc pas recevable contre Stokes à amplitude utile.**
Ce n'est pas un défaut du candidat : c'est l'absence de référence. Une réception non
linéaire en faible profondeur demande une autre famille d'oracles — cnoïdal, ou
Boussinesq — qui n'est pas construite dans ce dépôt.

## 3. Le candidat : troncature en amplitude sur le relèvement discret

### 3.1 Ce qui manque et comment l'obtenir

Le système du §1 est exact, mais `W` n'est pas une fonction locale de `(η,ψ)` : il exige de
résoudre Laplace jusqu'à la surface **déplacée**. Le candidat l'obtient par un développement
en amplitude — méthode spectrale d'ordre élevé, Dommermuth & Yue ; Craig & Sulem : on écrit
`φ = Σ_{m≥1} φ^{(m)}`, chaque `φ^{(m)}` d'ordre `ε^m` en amplitude et **harmonique dans la
tranche fixe** `−h<z<0` avec `φ_z=0` en `z=−h`. Le développement de Taylor de la condition
de Dirichlet en `z=η` autour de `z=0` donne, ordre par ordre,

```
φ^{(1)}|₀ = ψ
φ^{(m)}|₀ = − Σ_{l=1}^{m−1} (η^l/l!) ∂_z^l φ^{(m−l)}|₀     (m ≥ 2)
W = Σ_{m+l ≤ M, m≥1, l≥0} (η^l/l!) ∂_z^{l+1} φ^{(m)}|₀
```

`M` est **l'ordre en amplitude retenu**, et c'est la grandeur que la tâche S192-1 demandait
d'expliciter. Le développement est en `ε` — la cambrure `ka` — et non en une quantité de
maillage : sa troncature est une limite **de modèle**, pas de résolution.

### 3.2 Les dérivées verticales sont des symboles, et c'est là que vit la profondeur

Pour un mode horizontal de nombre d'onde `k_q`, un champ harmonique dans la tranche à fond
imperméable est entièrement déterminé par sa trace en `z=0`. Deux opérateurs suffisent :

```
A : ∂_z   en z=0  →  symbole G_h(k_q), opérateur de Dirichlet-Neumann discret
B : ∂_z²  en z=0  →  symbole k_q², par l'équation de Laplace elle-même
```

et toutes les dérivées supérieures s'en déduisent **algébriquement**, sans inconnue
nouvelle : `∂_z^{2p} = B^p`, `∂_z^{2p+1} = B^p A`. C'est une identité, pas une
approximation : `φ_zz = −φ_xx` est l'équation résolue.

`G_h` est **celui de S192** : relèvement tridiagonal de `K` inconnues dans la profondeur,
`(1+μ/2)φ_0−φ_1=0` au fond (image `φ_{−1}=φ_1`, Neumann nul, pas `φ_0=0`),
`−φ_{j−1}+(2+μ)φ_j−φ_{j+1}=0` à l'intérieur, `φ_K=ψ`, puis
`G_h ψ = (φ_K−φ_{K−1})/dz + dz k_q² ψ/2` — le demi-terme horizontal du demi-volume
supérieur, sans lequel l'erreur d'ordre un ne s'annule pas. Sa forme fermée de vérification
reste `G_h = sinh(γ) tanh(Kγ)/dz`, `γ = acosh(1+μ/2)`, `μ = k_q² dz²`.

**Un changement délibéré par rapport à S192, et sa raison.** S192 prenait
`μ = 4 sin²(πq/N) dz²/dx²`, symbole du Laplacien **entièrement discret**. Ici
`μ = k_q² dz²`, symbole **exact en horizontal**. Motif : la couche non linéaire n'introduit
aucune inconnue verticale — toute la profondeur passe par `G_h` — et le candidat travaille
sur une bande de modes exactement représentée (§4). Avec un horizontal exact, il ne reste
**qu'un seul axe de convergence spatiale**, `K`, et un ordre mesuré en `K` ne peut plus être
confondu avec un effet de maillage horizontal. Conséquence assumée :
`G_h(k_q) → k_q tanh(k_q h)` à l'ordre `dz²`, et le véhicule S193 à `M=1` n'est **pas**
bit-à-bit celui de S192 ; il est son équivalent semi-discret, et c'est cette équivalence-là
qui sera reçue (§5.1).

### 3.3 Les trois systèmes tronqués

En posant `W_n` la part d'ordre `ε^n` de `W`, le développement du §3.1 donne

```
W₁ = Aψ
W₂ = η Bψ − A(η Aψ)
W₃ = ½η² BAψ − η B(η Aψ) + A(η A(η Aψ)) − ½A(η² Bψ)
```

et la troncature **cohérente** du membre de droite — on garde tout produit dont l'ordre
total en `ε` ne dépasse pas `M`, en comptant `η`, `η_x`, `ψ_x` et chaque `W_n` :

```
M=1   η_t = W₁
      ψ_t = −gη
M=2   η_t = W₁ + W₂ − η_x ψ_x
      ψ_t = −gη − ½ψ_x² + ½W₁²
M=3   η_t = W₁ + W₂ + W₃ − η_x ψ_x + W₁ η_x²
      ψ_t = −gη − ½ψ_x² + ½W₁² + W₁W₂
```

Le terme `½W₁²η_x²` de `ψ_t` est d'ordre quatre et tombe ; `−η_xψ_x` est d'ordre deux
exactement et ne se tronque pas au-delà.

### 3.4 L'échelle des ordres est le dispositif de réception

`M=1`, `M=2`, `M=3` ne sont pas trois variantes d'implémentation : ce sont **trois modèles
physiques distincts, aux signatures prédites différentes**, et c'est l'échelle — pas un
chiffre isolé — qui reçoit le véhicule. Les prédictions sont posées **avant** la mesure.

| | harmonique liée `b₂` | décalage de fréquence en `(ka)²` |
|---|---|---|
| **M=1** | **exactement nulle** — le système est linéaire et découplé mode à mode | **exactement nul** |
| **M=2** | **correcte** à `O((ka)²)` près | **non nul et non conforme** : voir ci-dessous |
| **M=3** | **correcte** à `O((ka)²)` près | **conforme** à `½(ka)²` à `O((ka)²)` près |

La ligne `M=2` mérite son raisonnement, parce qu'elle est la seule prédiction qui puisse
surprendre. Le décalage de fréquence en `(ka)²` a **deux** sources : la contribution directe
des termes quartiques de l'énergie, et la contribution au second ordre de perturbation des
termes cubiques. Une troncature à `M=2` correspond à une énergie cubique : elle perd la
première source et garde la seconde. Elle produit donc un décalage **non nul et
généralement faux** — et non pas zéro, ce qui serait la conclusion naïve.

**Cette prédiction est falsifiable et sera publiée telle quelle.** Si `M=2` rendait le
décalage de Stokes à la tolérance déclarée, le raisonnement ci-dessus serait faux, l'ordre
`M=3` serait inutile pour la fréquence, et il faudrait le dire. Aucune valeur numérique
particulière n'est prédite pour `M=2` : la prédiction porte sur *non nul* et *non conforme*.

## 4. Représentation spectrale bornée, produits et intégration

**État.** `η` et `ψ` sont portés par leurs modes de Fourier `q = −Q..Q` d'un domaine
périodique de longueur `L`, `k_q = 2πq/L`, avec la symétrie hermitienne des champs réels.
Aucun maillage horizontal n'existe dans le candidat : `Q` est une **bande**, pas une grille.

**Produits.** Chaque produit est une convolution tronquée à la bande :
`(fg)_q = Σ_p f_p g_{q−p}` pour `|p|≤Q` et `|q−p|≤Q`. La troncature est une **projection
exacte** `P_Q`, appliquée après chaque produit — il n'y a donc **aucun repliement** à
discuter, ni filtre à calibrer : ce que la bande ne contient pas n'est pas replié, il est
absent. C'est le motif du choix de représentation, et il remplace la question
d'anti-repliement qu'une méthode sur grille aurait dû trancher. `Q` devient un **axe de
vérification** — augmenter `Q` ne doit pas déplacer le résultat — et non un axe de
convergence spatiale.

**Dérivées horizontales.** Symbole `i k_q`, exact sur la bande.

**Intégration en temps.** Runge-Kutta 4 classique sur le couple `(η,ψ)`. Le découpage de
Verlet de S192 exploitait la structure linéaire et ne s'applique plus. Pas prescrit validé
avant emploi par `dt √(g G_h(k_Q)) < 2,5` — borne de stabilité de RK4 appliquée à la
fréquence linéaire la plus rapide de la bande. Entrées et états non finis refusés ; aucun
état modifié lors d'un refus ; publication seulement après calcul fini.

**Énergie et volume.** L'énergie par unité de densité et de largeur du système **tronqué**
s'écrit avec sa propre dérivée :

```
E = ½ ∫ (gη² + ψ η_t) dx        (car ψ G(η)ψ = ψ η_t dans le système exact)
V = ∫ η dx = L η̂₀
```

**Deux avertissements déclarés avant mesure, et ils distinguent S193 de S192.** Le système
tronqué **n'est pas hamiltonien** : son énergie n'est conservée qu'à l'ordre de la
troncature, et la dérive attendue est `O(a^{M+1})` en plus du `O(dt⁴)` de RK4. Et le volume
**n'est plus exactement conservé** : dans le système exact `∫G(η)ψ dx = 0` identiquement,
mais le mode nul de `η Bψ` ne s'annule pas, si bien que la dérive de volume est elle-même un
**diagnostic de troncature** d'ordre `a^{M+1}`. S192 conservait le volume à `1,46·10⁻¹⁶` ;
prétendre le même chiffre ici serait un aveu d'erreur, pas une qualité.

`f64` et allocations de banc ; aucune prétention I-03/I-06/I-08 sur le runtime. Bibliothèque
`water-core` inchangée : le véhicule vit dans `examples/`, comme celui de S192.

## 5. Réceptions et campagne déclarées

### 5.1 Tests propres, avant toute campagne

1. **Relèvement** — `G_h` du système tridiagonal contre sa forme fermée
   `sinh(γ)tanh(Kγ)/dz` ; résidu de la récurrence intérieure ; condition de fond
   `(1+μ/2)φ_0=φ_1` ; `G_h(0)=0` traité exactement ; `G_h>0` ailleurs.
2. **Réduction à `M=1`** — à `M=1` le système est linéaire et découplé mode à mode, et son
   évolution doit être **exacte à l'arrondi près** contre un oracle indépendant, sur un
   millier de pas, pour plusieurs modes de phases distinctes simultanément. C'est
   l'équivalence semi-discrète avec S192.

   > **Précision P3a, 2026-09-12.** La formulation initiale de ce test disait « contre la
   > solution analytique de l'oscillateur `η̈=−gG_hη` ». Elle était fausse : RK4 n'intègre
   > pas exactement un oscillateur, son écart à la solution continue est `O(dt⁴)` et non
   > de l'ordre de l'arrondi. Ce qui *est* exact, c'est la **puissance de l'amplification
   > RK4** elle-même. Pour `J=[[0,G_h],[−g,0]]`, on a `J²=−ω²I` avec `ω²=gG_h`, donc
   > `R = cI + sJ` avec `c = 1−(ωdt)²/2+(ωdt)⁴/24` et `s = dt(1−(ωdt)²/6)`, puis
   > `Rⁿ = ρⁿ(cos nθ·I + sin nθ·J/ω)`, `ρ=√(c²+s²ω²)`, `θ=atan2(sω,c)`. C'est cette forme
   > fermée — indépendante du code de pas — qui sert d'oracle. L'écart à l'oscillateur
   > continu reste mesuré, mais par l'axe de convergence en `dt` du §5.2, à sa place.
3. **Convolution tronquée** — un produit de deux champs à bande étroite doit égaler la
   convolution analytique de leurs modes ; un couple choisi pour déborder la bande doit être
   **projeté et non replié**.
4. **Invariances** — état au repos strictement immobile ; lac plat non nul immobile et de
   volume constant ; une gravité multipliée par quatre doit accélérer l'évolution comme `√g`
   à `M=1`.
5. **Refus atomiques** — dimensions, `Q`, `K`, `h`, `g`, `dt` invalides, plafond de
   stabilité, entrées et états non finis : refus sans aucune modification d'état, vérifié
   par comparaison de l'état avant et après.

Tests exécutés en `debug` **et** `release`.

### 5.2 Campagne

`L=8 m`, `g=9,81 m/s²` injecté, `k=2π/8=0,785398 rad/m`, `Q=8` nominal, `K=64` nominal,
`dt=T/400` nominal avec `T=2π/ω₀`, fenêtre de **20 périodes** linéaires.

| axe | valeurs | ce qu'il mesure |
|---|---|---|
| amplitude | `ka = 0,0125 / 0,025 / 0,05 / 0,1` | la **pente en amplitude**, cœur de la réception |
| ordre | `M = 1 / 2 / 3` | l'échelle du §3.4 |
| profondeur | `kh = 6,2832` (`h=8`), `1,5708` (`h=2`), `0,19635` (`h=0,25`) | dispersion ; verdict d'Ursell pour le troisième |
| profondeur discrète | `K = 16 / 32 / 64` | convergence spatiale, ordre 2 attendu |
| temps | `dt = T/100 / 200 / 400 / 800` | convergence temporelle, ordre 4 attendu |
| bande | `Q = 8 / 12 / 16` | vérification de représentation, pas convergence |

**Condition initiale.** Profil de Stokes d'ordre deux du §2.1 pour `η` ; pour `ψ`, sa trace
cohérente au même ordre. Elle n'est **pas** l'oracle : c'est un choix d'expérience. Un écart
résiduel d'ordre deux sur `ψ` excite une onde libre de second harmonique, à la fréquence
`√(g·2k·tanh 2kh) ≠ 2ω`, qui **oscille et ne biaise pas la moyenne** de l'harmonique liée.
La mesure de `b₂` est donc robuste à ce choix, et l'amplitude de cette oscillation sera
publiée comme mesure de la contamination.

### 5.3 Grandeurs mesurées et seuils

1. **`b₂` mesuré** — moyenne temporelle, sur la fenêtre, de l'amplitude du second harmonique
   ramenée dans le repère de l'onde, divisée par `k a²`. Comparée à `b₂(kh)` du §2.1.
   **Reçu si** le rapport tend vers 1 quand `ka→0` avec un écart décroissant, et si l'écart
   à la plus petite amplitude est **sous 2 %** (ADR-120, seuil non redemandé).
   **Pente en amplitude déclarée : 2.** Un écart décroissant comme `(ka)¹` signerait un
   modèle linéaire ; c'est la contre-épreuve `M=1`, qui doit la produire.
2. **Décalage de fréquence** — `ω` ajusté par moindres carrés sur la phase déroulée du mode
   fondamental `η̂₁` sur la fenêtre. En profondeur infinie, `(ω−ω₀)/ω₀` divisé par `½(ka)²`
   doit tendre vers 1 ; **reçu si** l'écart à ce rapport est sous 2 % à la plus grande
   amplitude utile, pour `M=3`. Pour `M=1`, la valeur doit être **exactement nulle**. Pour
   `M=2`, la valeur est **publiée sans verdict** et la prédiction du §3.4 est confirmée ou
   réfutée par écrit.
3. **Convergence** — ordre en `K` et en `dt` estimé aux deux derniers raffinements, sur
   l'écart à la configuration la plus fine ; ordre `>1,5` en `K`, `>3` en `dt`.
4. **Conservation** — dérive relative d'énergie sous `10⁻⁴` ; dérive de volume relevée et
   **comparée à `a^{M+1}`** au lieu d'être bornée par un seuil.
5. **Bande** — passer `Q` de 8 à 16 ne doit pas déplacer `b₂` ni le décalage de fréquence de
   plus de 2 %.

### 5.4 Contre-épreuves

- **`M=1` sur les mêmes données** : `b₂=0`, décalage nul. C'est la contre-épreuve
  principale, et c'est elle qui distingue « mon véhicule est non linéaire » de
  « mon véhicule est précis ».
- **Gravité de signe inversé** : divergence, pas une onde.
- **Oracle peu profond** `ω=√(gh)k` appliqué au cas profond : doit rester grossièrement
  faux, pour montrer que le véhicule ne converge pas vers Saint-Venant.
- **Amplitude négligeable** : le décalage de fréquence mesuré doit rentrer dans l'arrondi,
  sinon la mesure de fréquence a un biais propre et le chiffre non linéaire ne vaut rien.

Deux exécutions `release` identiques, empreinte des indicateurs publiée. Les chiffres ne
seront ajoutés qu'après exécution.

## 6. Limites de ce qui sera fermé

**Ce qui sera fermé si tout passe.** Le dépôt possédera un véhicule **à la fois non linéaire
et dispersif** en profondeur finie, avec son ordre en amplitude explicite et son domaine de
validité borné par la cambrure et par Ursell. A217 perdra son manque structurel : la
comparaison perturbatif/total cessera d'être interdite faute de véhicule.

**Ce qui ne sera pas fermé, et ne doit pas être annoncé comme tel.**

1. **A217 elle-même** reste ouverte : disposer du véhicule n'est pas avoir mesuré le
   couplage. La comparaison de deux trains superposés contre leur évolution commune est le
   lot **suivant**, et c'est lui qui touche ADR-112.
2. **La source B+W de S190/S191 n'est pas branchée.** Aucun montage couplé complet, aucune
   addition de ces erreurs au budget projeté `1,374540 %` : références et champs différents.
3. **A216** reste inexpliquée ; aucun seuil de bascule, aucune valeur `0,02`/`0,24` ne se
   dérive d'ici.
4. **Forces et perception** de B4 restent non reçues.
5. **Aucun solveur δ n'est choisi.** Ceci est un véhicule de banc ; le 2 % ne choisit pas
   une technologie.
6. **La faible profondeur non linéaire n'a pas d'oracle** dans ce dépôt (§2.3), et la
   bathymétrie variable reste entière : le fond est plat.
7. **La surface reste un graphe** : ni déferlement, ni rouleau, ni air.
8. **Aucune seconde cible** (A98), aucune mesure de coût CPU.
