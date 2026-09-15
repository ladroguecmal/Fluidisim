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
