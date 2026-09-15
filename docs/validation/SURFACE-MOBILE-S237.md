# Surface géométriquement mobile — S237, 2026-09-15

Suite de [SURFACE-LINEARISEE-S233](SURFACE-LINEARISEE-S233.md). Jalon J2 : une frontière libre
qui **se déplace** dans le candidat δ MAC x-z, et non plus un déplacement linéarisé autour d'un
couvercle fixe.

## 1. Protocole, écrit avant construction

### 1.1 Modèle discret

- **Représentation** : fonction hauteur `η_i` par colonne (surface graphe, un seul point par
  verticale). Le fluide d'une colonne est l'ensemble des mailles du fond coupé (`frac > 0`) dont
  le centre `z_c = (k + ½)·dx` est **sous** `η_i`. Les autres mailles sont de l'air : elles sortent
  du système de pression.
- **Pression hydrostatique analytique** (en-tête du module) : `p_hydro = ρg(z_ref − z)`, avec un
  **niveau de référence `z_ref` fourni**, intérieur au domaine, et non plus le sommet `z₀`. Au
  repos `p_dyn ≡ 0` pour tout `z_ref` : le repos exact ne dépend pas de l'alignement de la surface.
- **Condition de surface** `p = 0` à `z = η`, soit `p_dyn(Γ) = ρg(z_Γ − z_ref)`, imposée par
  **fluide fantôme** (Gibou, Fedkiw et al. 2002) : entre une maille fluide et une maille d'air
  voisine, l'interface est à `θ·dx` du centre fluide, `θ` interpolé linéairement sur la fonction
  distance `φ = z_c − η`. Verticale : `θ = (η_i − z_c)/dx`, `z_Γ = η_i`. Horizontale :
  `θ = (η_i − z_c)/(η_i − η_j)`, `z_Γ = z_c`. L'opérateur reçoit `a/θ` sur la diagonale et le second
  membre `a·p_dyn(Γ)/θ` : les termes hors diagonale sont inchangés, **l'opérateur reste
  symétrique**, le gradient conjugué reste valide. Au couvercle S233, `θ = ½` redonne le facteur 2.
- **Conditionnement** : `θ` est borné inférieurement par `θ_min = 10⁻³` (paramètre numérique, pas
  seuil physique), et le gradient conjugué du mode mobile est **préconditionné par la diagonale**
  pour que les lignes à `a/θ` grand ne gouvernent pas le nombre d'itérations. Sensibilité à `θ_min`
  publiée à la réception.
- **Correction** : une face entre deux mailles fluides prend le gradient ordinaire ; une face entre
  une maille fluide et l'air prend `(p_dyn(Γ) − p_c)/(θ·dx)` ; une face entre deux mailles d'air
  n'est pas corrigée.
- **Extrapolation** : dans chaque colonne de faces, les vitesses au-dessus de la dernière face
  corrigée reçoivent sa valeur (extrapolation constante verticale), dans l'ordre des indices.
- **Advection** : l'advection centrée d'ordre deux du chemin `step` est conservée, sur les vitesses
  extrapolées. Aucune stabilité d'advection n'est revendiquée (déjà le statut de `step`).
- **Surface** : `η_i ← η_i − (dt/dx)(Q_{i+1} − Q_i)`, avec `Q` le débit de la face verticale
  **intégré jusqu'à la hauteur mouillée de la face** `η_f = ½(η_{i−1} + η_i)` :
  `Q = Σ_k open_u·u·dx·clamp((η_f − k·dx)/dx, 0, 1)`. Murs : `Q = 0`. Le volume se télescope ;
  somme compensée f32 de S233 conservée. En petite amplitude, `Q` ne diffère de celui de S233 que
  par `u·(η − z₀)`, terme quadratique.
- **Ordre du pas** : gardes, sauvegarde, advection, géométrie à `η^n`, projection, extrapolation,
  `η^{n+1}`, validation, garde sur `η^{n+1}`, publication. Temps en microsecondes entières,
  coefficients selon ADR-141.

### 1.2 Gardes et refus atomiques

- `g > 0`, durée `≥ 1 µs`, garde de pas de S233 `dt²g/dx ≤ 1`.
- **Surface à au moins deux mailles au-dessus du fond coupé** et **à au moins une maille sous le
  sommet** du domaine, avant et après le pas : sans mouillage du fond ni débordement. Sinon
  `Domain`, état intact.
- Pression non convergée : `Convergence`, état intact ; expiration : zéro avancée, état intact.
- Zéro allocation dans le pas.

### 1.3 Oracle indépendant

**Onde stationnaire d'amplitude finie**, bassin à murs `L = 2 m`, profondeur `h = 2 m`, mode
`η₀ = a·cos(kx)`, `k = π/L`, fluide au repos. Profondeur `kh = π`.

- **Référence non linéaire** : véhicule potentiel HOS d'ordre `M = 3` de S193
  (`examples/support/nl_surface.rs`, ADR-122), périodique de longueur `2L`, condition initiale
  paire `η̂₁ = a/2`, `ψ = 0`. Il ne partage avec le candidat ni équations (potentiel contre Euler),
  ni discrétisation (spectrale contre MAC), ni pas de temps (RK4).
- **Domaine de l'oracle** (ADR-122) : Ursell `U = aλ²/h³ = 2a`, cambrure `ka`. Amplitudes
  `a = 0,01 / 0,05 / 0,10 m` : `ka = 0,016 / 0,079 / 0,157`, `U = 0,02 / 0,10 / 0,20`. S193 a reçu
  le véhicule à `U = 0,05` et mesuré sa falaise à partir de `U ≈ 65` ; `U ≤ 0,2` est dans le domaine.
- **Recoupement** : solution d'ordre deux dérivée sur place (sympy), onde stationnaire depuis le
  repos, contre le véhicule à `M = 3` à la plus petite amplitude non linéaire.

### 1.4 Critères et tolérances de banc

Grilles `nx = 32 / 64 / 128` (`nz` jusqu'à `z = 2,25 m`), durée **une période** linéaire, pas de
`1 ms`. Grandeurs, écrites avant mesure :

1. **Repos exact au bit** à `z_ref = 2 m` et à un niveau non aligné sur les faces.
2. **Opérateur symétrique au bit** (coefficients `A_ij = A_ji` lus sur vecteurs unitaires).
3. **Volume** : dérive de `Σ η_i` à l'arrondi f32, comme S233.
4. **Petite amplitude** (`a = 0,01 m`, `nx = 64`) : écart mobile − linéaire S233 ≤ **1 %** de `a`
   sur la période. Motif : critère fin de S233.
5. **Profil** (`a = 0,05` et `0,10 m`) : `max|η_MAC − η_HOS|/a` sur tous les points et pas ≤
   **2 %** à `nx = 128`, et décroissant de `32` à `128`. Motif : échelle de réception d'ADR-120,
   employée ici comme tolérance de banc, jamais comme seuil de bascule.
6. **Harmonique `2k`** : `b₂(t) = (2/L)∫(η − z_ref)cos(2kx)dx`. Écart maximal au véhicule ≤ **20 %**
   de `max|b₂,HOS|` à `nx = 128`, décroissant en raffinant. Motif : `b₂ ≈ 1–4 %` de `a`, 20 % de `b₂`
   restant sous l'échelle de 2 % du critère 5.
7. **Témoin discriminant** : le mode linéaire S233 sur la même grille rend `max|b₂| ≤ 1 %` de
   `max|b₂,HOS|` — il ne peut produire l'harmonique.

Un critère manqué est publié tel quel ; aucune tolérance n'est modifiée après mesure.

**Correction du protocole avant toute mesure (P4b, 2026-09-15).** Le critère 4 plaçait la
comparaison au mode linéaire à `a = 0,01 m` en supposant l'effet non linéaire négligeable. L'ordre
deux calculé en P3 le contredit : `max|b₂| = 1,478·a²`, soit **1,5 % de `a`** à `a = 0,01 m` — un
écart physique que le mode mobile *doit* produire et que le mode linéaire ne peut pas produire. Le
critère 4 aurait échoué pour une raison juste. Il est porté à **`a = 0,001 m`** (effet non linéaire
0,15 % de `a`), tolérance **1 %** inchangée ; `a = 0,01 m` rejoint les amplitudes comparées au
véhicule HOS (critères 5 et 6). Aucune mesure du candidat n'avait eu lieu.

**Coefficient d'advection.** L'advection existante multiplie par `dt` ; le mode mobile le lui passe
comme coefficient construit en f64 depuis la durée entière puis arrondi en f32, au même titre que
`ρ/dt`, `dt/ρ` et `dt/dx` (ADR-141) : aucune durée ni horloge n'est portée en f32.
