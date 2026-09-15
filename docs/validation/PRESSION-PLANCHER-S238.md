# Le plancher de la pression f32 de δ — S238, 2026-09-15

Traite **A272** : au-delà des 8 192 mailles reçues par [PRESSION-F32-S231](PRESSION-F32-S231.md),
le résidu relatif de la pression stagne au-dessus de son seuil et le pas refuse
([SURFACE-MOBILE-S237](SURFACE-MOBILE-S237.md) §6).

## 1. Protocole, écrit avant mesure et construction

### 1.1 Ce que le seuil actuel est, et ce qu'il n'est pas

- `project` arrête le gradient conjugué quand le **vrai** résidu recalculé vérifie
  `‖b − Ap‖² ≤ 10⁻¹²·‖b‖²`, soit un résidu relatif de 10⁻⁶. Ce nombre vient du candidat f64 de S199 ;
  S231 l'a conservé en f32 « sans relever le seuil » et l'a tenu jusqu'à 128×64 avec des résidus déjà
  à 8,7·10⁻⁷. Il n'a aucune provenance physique.
- La tolérance **physique** de la projection est celle de S199 §5, critère 4, déclarée avant
  construction : `max|div u|·dx / max|u| ≤ 10⁻⁵`.
- **Identité discrète** : après correction, dans toute maille fluide, `div u = r_i / scale` avec
  `r = b − Ap` et `scale = −ρ/dt`, parce que `scale·k1 = −1` et que la correction emploie les mêmes
  coefficients et les mêmes valeurs imposées que l'opérateur et son second membre. Le critère de S199
  est donc une norme **maximale** du résidu, mise à l'échelle par `|scale|·max|u|/dx`. Il sera vérifié
  numériquement, pas supposé.
- **Borne d'arrondi.** Un résidu relatif est borné inférieurement, en arithmétique finie, par un terme
  de l'ordre de `κ(A)·ε` : il croît avec la taille. L'**erreur inverse composante par composante**
  (Oettli–Prager) `ω = max_i |r_i| / (|b| + |A||p|)_i` ne dépend pas du conditionnement ; l'évaluation
  de `r_i` en virgule flottante a une erreur relative à `(|b| + |A||p|)_i` bornée par `γ_n = n·u/(1−n·u)`,
  `u = 2⁻²⁴` en f32, `n` le nombre d'opérations de la ligne (Higham, *Accuracy and Stability of
  Numerical Algorithms*, §3.1–3.3). `ω` est publiée comme **diagnostic** : elle dit si un résidu est
  indiscernable de son propre calcul. Elle n'est pas un seuil d'acceptation.

### 1.2 Règle proposée

1. **Inchangé** : le critère de résidu relatif 10⁻⁶ reste le premier ; tout pas qui l'atteint suit
   exactement le chemin actuel. Les cas S231, S232, S233 et S237 doivent rester **au bit**.
2. **Stagnation** : le vrai résidu, recalculé à chaque relance, **ne décroît plus strictement** d'une
   relance à la suivante. Aucun facteur arbitraire.
3. **Acceptation au plancher** : à stagnation, le pas est accepté si et seulement si le critère
   physique de S199 est tenu, `max|div u|·dx/max|u| ≤ 10⁻⁵`, calculé sur les mailles fluides après
   correction. Sinon il est dégradé, comme aujourd'hui.
4. **Plafond d'itérations** : inchangé ; atteint sans stagnation détectée, le pas reste dégradé
   (variable de dégradation d'ADR-007 §2).
5. Le rapport publie la raison de l'arrêt et `ω`.

### 1.3 Critères

1. **Loi du plancher** : résidu relatif minimal atteignable et `ω` à ce minimum, en fonction du nombre
   de mailles (16×8 à 256×128, second membre S231 ; et le cas mobile S237 au quart de période à
   32/64/128 colonnes). Publiée avant de construire la règle.
2. **Au bit** : empreinte du filtre S232 `0xc5ab1eadb094d058`, tests S231 à S237 inchangés.
3. **Cas S237** 5 cm / 128 colonnes : la trajectoire complète est reçue avec les tolérances de S237
   (profil ≤ 2 %, `b₂` ≤ 20 %), sans modification de ces tolérances.
4. **Système sans solution** (Neumann pur, second membre de moyenne non nulle) : stagnation détectée,
   critère physique **non** tenu, pas **dégradé**.
5. **Contre f64** : au plancher, écart de pression et de vitesse corrigée à la solution du même
   système assemblé indépendamment en f64 (méthode S231) ; publié, et confronté à l'estimation
   `δu/u ≲ div·L/(π²·u)` pour un résidu porté par le mode le plus lent.
6. **Identité** `div u = r/scale` vérifiée au bit près de l'arrondi sur les mailles fluides.
7. **Coût** : itérations consommées par le cas S237 avant (64 000 sans issue) et après.

## 2. Loi du plancher, mesurée avant la règle

Tests ignorés `pressure_floor_law_s231_family_s238` et `pressure_floor_law_mobile_quarter_period_s238`,
trace (test seulement) du vrai résidu à chaque relance et de l'erreur inverse `ω`.

**Famille S231** (8×4 m, surface `4 + 0,01·sin`, un pas depuis le repos) :

| grille | mailles | relances | résidu relatif final | `ω` final | divergence S199 |
|---|---:|---:|---:|---:|---:|
| 16×8 | 128 | 1 | 6,84·10⁻⁷ | 8,6·10⁻⁷ | 3,8·10⁻⁶ |
| 32×16 | 512 | 2 | 4,96·10⁻⁷ | 7,5·10⁻⁷ | 2,4·10⁻⁶ |
| 64×32 | 2 048 | 2 | 7,88·10⁻⁷ | 1,3·10⁻⁶ | 5,1·10⁻⁶ |
| 128×64 | 8 192 | 2 | 7,16·10⁻⁷ | 7,1·10⁻⁷ | 8,1·10⁻⁶ |
| 256×128 | 32 768 | 2 | 9,97·10⁻⁷ | 6,6·10⁻⁷ | **1,58·10⁻⁵** |

Cette famille **converge jusqu'à 32 768 mailles** : la taille seule ne fait pas le refus. Mais la
divergence mise à l'échelle de S199 **croît** avec la taille et dépasse 10⁻⁵ à 256×128 alors que le
résidu relatif passe — les deux critères ne mesurent pas la même chose (§3).

**Cas mobile S237** (5 cm, quart de période) : à 32 et 64 colonnes, pire résidu final 9,997·10⁻⁷ et
9,990·10⁻⁷, **`ω` = 8,2·10⁻⁵ et 4,7·10⁻⁴** ; à 128 colonnes, pas 397 : **3 481 relances d'une itération
chacune**, le vrai résidu **alterne exactement** entre 1,0610·10⁻⁶ et 1,0630·10⁻⁶ — un cycle de période
deux — et **`ω` = 5,5·10⁻⁸ à 6,2·10⁻⁸, soit une unité d'arrondi `u`**.

Deux conclusions.

1. **Au refus, la pression est la meilleure solution que f32 représente.** Une erreur inverse d'un `u`
   ne laisse rien à gagner : la relance produit une itération, l'arrondi ramène l'état, et le cycle est
   **déterministe**. Continuer jusqu'au plafond n'a jamais eu d'issue.
2. **`ω` n'est pas un critère d'arrêt** : aux pas qui convergent selon le critère actuel, elle vaut
   jusqu'à 4,7·10⁻⁴ — le gradient conjugué minimise l'énergie de l'erreur, pas l'erreur composante par
   composante. Un seuil sur `ω` aurait refusé des pas que tout le reste accepte. Elle reste diagnostic.

## 3. Règle retenue, après mesure

La non-décroissance stricte du §1.2 est **écartée** : S233 a déjà établi qu'une hausse isolée du vrai
résidu arrondi précède une convergence, et couper là changerait les bits reçus. La mesure fournit un
signe sans nombre et sans ambiguïté :

- **Cycle certifié** : à une relance, le champ de pression `p` est **identique au bit** à celui d'une
  relance précédente (empreinte FNV-1a de 64 bits). La relance est une fonction déterministe de `p`
  seul (`res = b − Ap`, `dir = M⁻¹res`) : un état revenu est une suite périodique, que le plafond
  aurait parcourue sans converger. **S'arrêter là ne change aucun pas qui convergeait.**
- **Détection de Brent** (mémoire constante, aucune allocation) : une empreinte de référence est
  renouvelée quand le nombre de relances depuis la précédente atteint une puissance de deux, et chaque
  empreinte nouvelle lui est comparée. Toute période est détectée.

**Première construction écartée par la mesure.** Une mémoire des deux dernières relances — suffisante
pour le cas de 8×12 mailles, où le cycle est court — **n'a pas vu le cas visé** : relancé, le pas 397 à
128 colonnes refusait toujours. L'historique relevé au pas même (test ignoré
`pressure_state_history_at_the_floor_s238`) : **3 480 relances, 482 états distincts, l'état 482
identique au bit à l'état 62 — une période de 420 relances**, résidu minimal à la relance 60. Le
résidu semblait alterner à deux valeurs ; l'état, lui, parcourait un cycle deux cents fois plus long.
Avec Brent, le cycle est certifié à la **relance 931, en 1 450 itérations** au lieu du plafond de 4 000,
divergence 1,45·10⁻⁷, pas reçu.

**Seconde construction écartée par la mesure : le cycle seul n'a pas de coût borné.** Dans le banc de
S237 (système de tâches de l'hôte et horloge réelle, donc un ordre de réduction différent du test), le
même pas 397 exige **7 386 itérations** avant certification : refus sous le plafond de 4 000 du banc.
Sous un plafond de 16 000, les pas 397 à 403 passent au plancher (719 à 7 386 itérations chacun) et
**le pas 404 refuse encore** : au quart de période, plusieurs pas consécutifs sont au plancher, et la
longueur d'un cycle exact n'est bornée par rien. À chacun de ces pas, en revanche, `ω` valait 0,9 à
1,3 `u`.

### Règle finale

La thèse du plan revient, corrigée par ce que la mesure a appris de `ω` :

1. **Critère premier inchangé** : résidu relatif ≤ 10⁻⁶.
2. **Arrêt au plancher** (`Report.floor`) si le vrai résidu, à une relance, est **indiscernable de
   l'arrondi de son propre calcul** — `ω ≤ γ₈ = 8u/(1 − 8u) ≈ 4,77·10⁻⁷` —, ou si l'état revient au bit
   (Brent). La constante n'est pas choisie : dans le modèle standard (Higham §3.1–3.4), une ligne à quatre
   faces évalue chaque terme avec `γ₂`, les somme avec `γ₃`, multiplie par `1/dx²` et soustrait de `b`,
   soit `γ₇` ; la représentation f32 de la solution ajoute `u`.
3. **Acceptation** au plancher si et seulement si la divergence de S199 est ≤ 10⁻⁵ ; sinon dégradé.
4. Plafond sans plancher : dégradé, inchangé.

`ω` n'est **jamais** critère d'acceptation : sur le système incohérent, elle tombe à 2·10⁻⁸ par dérive du
noyau, et c'est la divergence (1,0) qui dégrade le pas.

Construit dans `delta_projection.rs` : `ROUNDOFF_BACKWARD_ERROR`, `PROJECTION_DIVERGENCE_TOLERANCE`,
`Report { floor, backward_error }`, empreinte FNV-1a de `p` et détection de Brent en mémoire constante.
Aucune allocation, chaque passe interrogeant `Control` comme les autres réductions de la pression.

## 4. Réception — 2026-09-15, secteur au début et à la fin

| critère | résultat |
|---|---|
| 1. loi du plancher | §2 |
| 2. au bit | empreinte S232 `delta_filters` **`0xc5ab1eadb094d058`** ; suite **429 réussis, 9 ignorés** (425/5 en S237, quatre tests et quatre mesures S238), aucune tolérance modifiée. **Promesse du §1.2.1 non tenue pour la trajectoire du banc S237** — voir ci-dessous |
| 2 bis. `delta_precision` (S231) | dix cas convergés par le critère premier, **aucun** au plancher, vrais résidus 4,96·10⁻⁷ à 8,70·10⁻⁷ comme publiés en S231 |
| 3. 5 cm, 128 colonnes | **reçu** : profil 0,252 %, `b₂` 0,71 % |
| 4. système sans solution | arrêt au plancher (42 itérations, `ω` = 2,2·10⁻⁸), divergence 1,0, **dégradé** ; cycle seul (certificat retiré) : 280 itérations, **dégradé** |
| 5. contre f64 | pression 3,0·10⁻⁵, vitesse **5,5·10⁻⁸** en écart relatif maximal |
| 6. identité `div u = r/scale` | écart 0,18 pour un second membre de 2,48·10⁶ : **7,2·10⁻⁸** ≤ 64 ε |
| 7. coût | pire pas de la période à 128 colonnes : **526 itérations** ; S237 : refus à 4 000, 16 000 et 64 000 |

### 4.1 La trajectoire de S237, rejouée

`examples/delta_mobile.rs plancher` : 5 cm, une période, pas de 1 ms, plafond de S237 (4 000).

| nx | profil / `a` | `b₂` relatif | pas au plancher | `ω/u` au plancher | résidu relatif max au plancher | divergence max au plancher | itérations max | volume (m) | ms/pas médiane / max |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32 | 0,850 % | 2,44 % | 12 | 2,2–7,4 | 1,31·10⁻⁶ | 1,46·10⁻⁷ | 142 | 1,5·10⁻⁸ | 4,6 / 10,4 |
| 64 | 0,550 % | 1,41 % | 39 | 2,6–7,8 | 1,95·10⁻⁶ | 1,50·10⁻⁷ | 269 | 5,6·10⁻⁹ | 34,6 / 118 |
| 128 | **0,252 %** | **0,71 %** | 38 | 2,7–7,7 | 3,01·10⁻⁶ | 1,50·10⁻⁷ | 526 | 9,3·10⁻¹⁰ | 256 / 366 |

Critères 5 et 6 de S237 **tenus à 128 colonnes et décroissants en raffinant** (2 % et 20 %). Les pas au
plancher se groupent autour des quarts de période (pas 387–415 et 1196–1215), où la surface est plate.
Tous s'arrêtent au bout du **même nombre d'itérations que leurs voisins convergés**, et tous par le
certificat d'arrondi — `ω` final ≤ `γ₈`, testé avant le cycle sur le même `p` —, jamais par la détection
de cycle.

**Ce que la règle a changé.** À 32 et 64 colonnes, S237 recevait tous les pas sous 10⁻⁶ ; la même
trajectoire sous la seule détection de cycle reproduisait ses bits (`pas_au_plancher = 0`, dérive de
volume 3,725·10⁻⁹ m, `b₂` 8,9578·10⁻⁵). Sous la règle finale, 12 et 39 pas s'arrêtent une ou deux
itérations plus tôt : **ces trajectoires ne sont plus au bit.** Profil et `b₂` relatif sont identiques aux
chiffres publiés en S237 ; l'écart de `b₂` bouge au cinquième chiffre (8,9571·10⁻⁵) ; la dérive de volume
passe à 1,5·10⁻⁸ m à 32 colonnes, soit le double du maximum publié par S237 (7,5·10⁻⁹) et un
seizième de l'unité d'arrondi d'une hauteur de 2 m — toujours « à l'arrondi f32 » au sens du critère 3 de
S237. Le §1.2.1 promettait les cas S237 au bit ; la promesse tient pour les tests et l'empreinte, **pas pour
cette trajectoire**. Elle était écrite pour un arrêt sur stagnation, qui ne coupe qu'après l'échec du
critère premier ; un certificat qui coupe plus tôt ne peut pas la tenir. Seule la détection de cycle la
tenait, à un coût non borné (§3).

**Ce que le pas reçu garantit désormais.** Le résidu relatif des pas acceptés au plancher monte à
**3,0·10⁻⁶** à 128 colonnes (6,3·10⁻⁶ dans le montage de test à 96 mailles). Le pas garantit `ω ≤ γ₈` et une
divergence ≤ 10⁻⁵ — mesurée partout ≤ 1,5·10⁻⁷ —, et non plus un résidu relatif de 10⁻⁶ que f32 ne peut
pas décider à ces tailles. Le critère 5 dit ce que cela coûte en exactitude.

### 4.2 Contre la solution f64 du même système

Test ignoré `floor_pressure_matches_independent_f64_solve_s238` : pas 397, 128 colonnes, système
réassemblé en f64 depuis la géométrie (fantômes et coefficients recalculés), gradient conjugué f64
jusqu'à 10⁻¹³.

| | f32, arrêt au plancher | f64 |
|---|---:|---:|
| itérations | 521 | 789 |
| résidu relatif | 2,55·10⁻⁶ (`ω` = 4,3 `u`) | 1,07·10⁻¹³ |
| divergence S199 | 1,45·10⁻⁷ | — |
| écart de pression `max|Δp|/max|p|` | **2,98·10⁻⁵** (max `p` 21,5 Pa) | |
| écart de vitesse corrigée `max|Δu|/max|u|` | **5,5·10⁻⁸** (max `u` 0,194 m/s) | |

L'estimation d'avant mesure bornait `δu/u` à 1,3·10⁻⁴ pour un résidu porté par le mode le plus lent ;
la mesure est **2 400 fois plus petite** : le résidu au plancher n'est pas lent, c'est de l'arrondi réparti.
La pression dynamique porte 3·10⁻⁵ d'écart relatif, soit 0,6 mPa : la vitesse, seule consommée par le
transport, est exacte à l'arrondi de ses propres f32.

### 4.3 Test du chemin de repli

Le certificat d'arrondi devance la détection de cycle dans tous les pas mesurés. Le test
`exact_pressure_cycle_alone_stops_and_is_judged_by_divergence_s238` le retire (commutateur de test seulement)
: le montage du cycle est certifié par **retour au bit de l'état de la relance 6 à la relance 8**
(9 relances, 51 itérations, divergence 5,2·10⁻⁸, reçu) ; le système sans solution s'arrête sur cycle à
280 itérations, divergence 1,0, **dégradé**.

### 4.4 Ce qui n'est pas reçu

- **La divergence S199 des pas convergés selon le critère premier** : 1,58·10⁻⁵ à 256×128 dans la famille
  S231 (§2), et déjà **1,02·10⁻⁵ à 128×64 sur la bosse** dans `delta_precision`, **dans le domaine reçu par
  S231** — qui n'appliquait ce critère qu'à 32×16. Le résidu relatif y passe. Le chemin existant ne tient
  donc pas partout la tolérance physique déclarée par S199. Publié (A273), non corrigé ici.
- Une seule forme de second membre au-delà de 16 384 mailles ; la 3D (six faces, `γ₁₀`) ; la loi de coût du
  certificat sur un pas qui cyclerait sans que `ω` descende sous `γ₈` (aucun rencontré).
- **Coût** (ADR-131) : présentes — préconditionnement diagonal, f32, une passe d'erreur inverse et une
  empreinte par relance ; absente — tout préconditionneur plus fort (multigrille, factorisation
  incomplète) ; domaine — une machine sur secteur, 128 colonnes, une période. 256 ms par pas médian
  à 128 colonnes, contre 260 ms à 10 cm en S237 : la règle ne change pas visiblement le coût du pas, et
  ce coût n'est confronté à aucun budget ici.
