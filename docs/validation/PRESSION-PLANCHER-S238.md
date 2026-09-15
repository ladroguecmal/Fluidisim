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
  relance précédente (empreinte de 64 bits des deux dernières relances). La relance est une fonction
  déterministe de `p` seul (`res = b − Ap`, `dir = res`) : un état revenu est une suite périodique, que
  le plafond aurait parcourue sans converger. **S'arrêter là ne change aucun pas qui convergeait.**
- **À cycle certifié**, le pas est accepté si et seulement si le critère physique de S199 est tenu
  (divergence ≤ 10⁻⁵) ; sinon dégradé. Plafond sans cycle : dégradé, inchangé.
- Le rapport publie `ω` et l'arrêt sur cycle.
