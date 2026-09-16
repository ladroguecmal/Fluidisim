# Surface mobile couplée B/W→δ — S253

Contrat : [ADR-152](../adr/ADR-152-surface-mobile-couplee.md). Suite de
[RACCORDEMENT-DELTA-S250](RACCORDEMENT-DELTA-S250.md) (volume, surface imposée) et de
[SURFACE-MOBILE-S237](SURFACE-MOBILE-S237.md) (surface mobile, sans fond).

## 1. Protocole, écrit avant construction

### 1.1 Oracle

Onde stationnaire de S237 : bassin à murs `L = 2 m`, profondeur `h = 2 m`, `k = π/L`,
`ω² = gk·tanh kh`, départ au repos avec `η = a·cos kx`. Référence **totale** : véhicule HOS
d'ordre `M = 3` de S193/S237 (Q = 16, K = 256, dt = 1 ms), inchangé.

Le **fond** est l'ordre un analytique de profondeur finie :
`ζ_fond = a·cos kx·cos ωt`, `P = ρga·C(z)·cos kx·cos ωt`, `U = (agk/ω)·C·sin kx·sin ωt`,
`W = −(agk/ω)·S·cos kx·sin ωt`, avec `C = cosh k(z+h)/cosh kh` et `S = sinh k(z+h)/cosh kh`,
dérivées exactes et laplacien nul. Il est prolongé **analytiquement** au-dessus du plan moyen,
ce qui le garde incompressible : le prolongement de Taylor d'ordre un ne l'est pas
(`div = z·U_xz`). Ce fond vérifie les murs et `W(−h) = 0`. La perturbation naît à zéro
(`η' = 0`, `v = 0`, `p' = 0`) et doit reconstruire la part non linéaire de HOS.

Contrôles du fournisseur, avant usage : sur une grille de points sous et au-dessus du plan moyen,
`div U = 0` et `U_t + ∇P/ρ = 0` à l'arrondi près, et `ζ_t = W(0)`. Le résidu contracté `S` se
réduit donc au terme `(U·∇)U`, comparé à son expression.

### 1.2 Critères, déclarés avant mesure

1. **Fond nul, identité au bit.** Avec des échantillons nuls et sans éponge, 50 pas de
   `step_perturbation_mobile` rendent u, w, p, `eta` et restes identiques au bit à
   `step_surface_mobile` depuis le même état, `a = 5 cm`, `nx = 32`.
2. **Refus et reprise.** Contexte, forme, non-planarité, `eta` incohérent dans une colonne et
   surface hors gardes : refus sans rien modifier. Expiration à plusieurs points : aucune
   avancée, état intact, mode couplé éteint, reprise identique au bit. Zéro allocation pendant
   le pas.
3. **Profil** (a = 5 et 10 cm) : `max|ζ − η_HOS|/a` sur tous les points et tous les pas d'une
   période ≤ **2 %** à `nx = 128`, et décroissant de 32 à 128. Tolérance de S237, critère 5.
4. **Harmonique `2k`** : écart maximal de `b₂` au véhicule ≤ **20 %** de `max|b₂,HOS|` à
   `nx = 128`, décroissant en raffinant. Tolérance de S237, critère 6.
5. **Témoin discriminant**, à `a = 5 cm` et `nx = 32` sur une période. Le pas couplé et le même
   pas **sans résidus de surface** (valeurs fantômes du fond et bande éteintes par un commutateur
   de test, géométrie totale conservée) sont comparés à l'ordre deux fermé de S237
   (`b₂ = a²·B₂(t)`). Le premier reste ≤ 20 % de `max|a²B₂|`, le second le dépasse.

**Correction du témoin, datée du 2026-09-16 (P6), après sa première exécution et avant toute
mesure des critères 3 et 4.** Le premier témoin mettait à zéro toute la correction du fond sur
les fantômes latéraux. Il retirait ainsi `−P_fond(Γ)` en entier, y compris sa partie d'**ordre
un** `ρg·ζ_fond`, et cassait la dynamique linéaire. Il divergeait (0,70 m/s dès le pas 100, refus
`Domain` au pas 1145), ce qui ne mesure pas les résidus de surface. Dans le témoin corrigé, la
pression du fond à l'interface est **linéarisée** en `ρg·ζ_fond(x_Γ)` (Taylor en x compris) et la
bande est éteinte. Seuls les termes d'élévation d'ordre deux disparaissent. Tolérance inchangée.
Le pas couplé, lui, rendait déjà `b₂` à 2,17 % de l'ordre deux sur cette première exécution.

Chiffres de S237 pour le **solveur total** (profil / `b₂`) : 10 cm 1,714/1,71 % (32),
0,592/0,98 (64), 0,230/0,43 (128) ; 5 cm 0,850/2,44 (32), 0,550/1,41 (64), 0,252/0,71 (128,
S238). Publiés en regard, ils ne servent pas de critères.

Un critère manqué est publié tel quel ; aucune tolérance n'est modifiée après mesure.

### 1.3 Arrêt

Pas couplé construit, critères 1 à 5 mesurés et publiés, coût médian à 128 colonnes relevé avec
techniques présentes et absentes. Si l'oracle refuse, s'arrêter au diagnostic et le publier.
