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

## 2. L'oracle, reçu avant le candidat

`examples/delta_mobile.rs oracle`. L = h = 2 m, `ω = 3,918171 rad/s`, `T = 1,603601 s`.

**Ordre deux depuis le repos**, dérivé sur place (sympy) puis écrit en forme fermée : forçages
`K₂ = −gk²·sin 2ωt/(2ω)` et `D₂ = gk(gk·sin²ωt + ω²cos²ωt·sinh 2kh)/(4ω²cosh²kh)`, système
`B₂' − σ₂A₂ = K₂`, `A₂' + gB₂ = D₂`, `σ₂ = 2k·tanh 2kh`, conditions `B₂(0) = A₂(0) = 0`. Numériquement
`b₂ = a²·[−0,7913 cos(5,5515 t) + 0,3986 cos(7,8363 t) + 0,3927]` : une harmonique libre à
`Ω = √(2gk tanh 2kh)`, une liée à `2ω`, et la constante `k/4`. Maximum sur une période
`1,478·a²` — **0,15 / 1,5 / 7,4 / 14,8 % de `a`** à `a = 0,001 / 0,01 / 0,05 / 0,10 m`.

**Véhicule HOS contre cet ordre deux** (écart maximal de `b₂` sur la période, relatif au maximum) :

| a (m) | K = 64 | K = 256, M = 3 | K = 256, M = 2 |
|---|---:|---:|---:|
| 0,01 | 0,308 % | **0,034 %** | 0,025 % |
| 0,05 | — | **0,823 %** | 0,757 % |
| 0,10 | — | **3,283 %** | 3,078 % |

À `K = 64`, le plancher de 0,31 % est celui du **symbole discret** du relèvement (écart 4,8·10⁻³ à
`k tanh kh`) : A242 et L277 le prévoyaient, et `dispersion_error` le publie à côté. À `K = 256` il
tombe à 3·10⁻⁴. Le rapport **3,99** entre 10 et 5 cm est exactement `(ka)²` : l'écart restant est
l'ordre quatre que l'analytique ne contient pas, pas un défaut du véhicule.

**Précision propre** du véhicule retenu (M = 3, Q = 16, K = 256, dt = 1 ms), profil aux centres
d'une grille de 128 colonnes, a = 10 cm : Q = 32 → 1,0·10⁻⁸ `a` ; K = 512 → 5,3·10⁻⁵ `a` ;
dt = 0,5 ms → 1,6·10⁻¹¹ `a`. **L'oracle est quatre ordres de grandeur sous les tolérances du banc.**

## 3. Ce qui a été construit

`src/delta_mobile.rs`, sous-module de `delta_projection` ; API publique
`Volume::set_free_surface(eta, rest)`, `Volume::step_surface_mobile(duration_us, max_iters,
budget_us, jobs, clock)`, `Volume::wet_cells()`, `SURFACE_THETA_MIN`. Stockage : un tableau f32 par
maille (inverse de la diagonale), **+4·nx·nz octets**. Les chemins S199–S233 restent au bit :
empreinte du filtre S232 `0xc5ab1eadb094d058` inchangée, 26 tests δ et 8 d'exécution inchangés.

Trois choses que la construction a apprises.

1. **Le critère 4 était faux avant toute mesure**, pour une raison physique (§1, correction datée).
2. **Un champ à divergence nulle écrit à la main n'éprouve pas une garde.** Son second membre n'est
   que de l'arrondi, et la pression f32 y plafonnait à un résidu relatif de 3,3·10⁻⁶, par CG simple
   (vérifié en forçant M = I) comme préconditionné, à 1 ms comme à 1 µs : le pas refusait par
   `Convergence` avant d'atteindre la garde visée. La garde d'après pas est reçue sur une dynamique
   réelle : crête initiale au sommet admis, dépassement au demi-cycle.
3. **La borne `θ_min` agit** : des `θ` de 4·10⁻⁶ à 5·10⁻⁵ sont rencontrés à 5 et 10 cm (la surface frôle
   des centres de maille). Sa sensibilité est publiée au §5.

## 4. Tests du cœur

| essai | résultat |
|---|---|
| symétrie de l'opérateur, surface ondulée 2×3 m à fantômes latéraux et verticaux | `A_ij = A_ji` **au bit** ; diagonale du préconditionneur au bit |
| projection d'un champ quelconque sur cette surface | convergée, divergence < 10⁻⁴ |
| repos, projection seule, 2,0 / 2,013 / 1,9 m sur fond bosselé | zéro itération, champs **au bit** nuls |
| repos, 50 pas à 1,9 m | surface et vitesses **au bit** |
| onde 10 cm, nx = 16, 400 pas | volume à l'arrondi, topologie changeante, crête redescendue |
| mobile − linéaire, a = 1 mm, nx = 16, 800 pas | < 1 % de `a` |
| refus avant pas (sommet, fond), après pas (dynamique), `Convergence`, g = 0 | état au bit, mode désarmé |
| exécution : **580 points d'expiration**, 5 refus d'entrée, horloge reculante | zéro allocation, reprise identique au bit |

Suite du cœur : **425 réussis, 5 ignorés** (419 avant la session).

## 5. Réception contre l'oracle

`examples/delta_mobile.rs reception`, une période, pas de 1 ms, secteur. Profil : `max|η − η_HOS|/a`
sur tous les points et tous les pas. `b₂` : écart maximal au véhicule, relatif à `max|b₂,HOS|`.

**Critère 4** — mobile − linéaire, a = 1 mm, nx = 64 : **0,167 % de `a`** (≤ 1 %) ; l'ordre deux
prédit 0,15 %.

| a | nx | profil mobile | `b₂` mobile | profil linéaire | `max|b₂|` linéaire / `max|b₂,HOS|` |
|---:|---:|---:|---:|---:|---:|
| 5 cm | 32 | 0,850 % | 2,44 % | 7,69 % | 1,7·10⁻⁵ |
| 5 cm | 64 | 0,550 % | 1,41 % | 7,72 % | 1,2·10⁻⁵ |
| 5 cm | 128 | **refus `Convergence` au pas 397** | — | 7,73 % | 7,7·10⁻⁶ |
| 10 cm | 32 | 1,714 % | 1,71 % | 16,13 % | 4,7·10⁻⁶ |
| 10 cm | 64 | 0,592 % | 0,98 % | 16,21 % | 2,7·10⁻⁶ |
| 10 cm | **128** | **0,230 %** | **0,43 %** | 16,23 % | 1,7·10⁻⁶ |
| 1 cm | 64 | 0,190 % | 1,61 % | — | — |

- **Critères 5 et 6 tenus à 10 cm** : 0,23 % ≤ 2 %, 0,43 % ≤ 20 %, décroissance 32 → 64 → 128 sur les
  deux grandeurs (profil ×2,9 puis ×2,6).
- **À 5 cm, non reçus à nx = 128** : refus au pas 397 (§6). Tenus et décroissants de 32 à 64.
- **Critère 7 tenu** : le mode linéaire produit `max|b₂| ≤ 1,7·10⁻⁵` fois l'harmonique de référence
  (≤ 1 %), et son erreur de profil **est** l'effet non linéaire : 7,7 % à 5 cm, 16,2 % à 10 cm, ∝ `a`.
- Volume : dérive ≤ 7,5·10⁻⁹ m. Topologie : 16 362 à 16 415 mailles fluides à 10 cm / nx = 128 ;
  aucune variation à 1 cm / nx = 64 (la surface ne franchit aucun centre).

**Sensibilité à `θ_min`** (a = 5 cm, nx = 32) : `10⁻³` → profil 0,850 %, `b₂` 2,44 % ; `10⁻²` → 1,147 %,
3,29 %. La borne dégrade la précision quand elle grossit ; `10⁻³` est conservé. Elle n'est pas en cause
dans le refus du §6 (même refus, même pas, à `10⁻²`).

## 6. Le refus du plus fin maillage à 5 cm

Rejoué au pas 397 (état atomique restauré) : **résidu relatif 1,0492·10⁻⁶**, pour un seuil de 10⁻⁶, à
4 000 comme à 16 000 et 64 000 itérations — une stagnation, pas un plafond. Divergence après correction
1,45·10⁻⁷. Le pas 397 tombe au quart de période : 16 384 mailles fluides, surface plate entre deux
rangées. **Même `θ` d'ordre ½ partout, donc pas de mauvais conditionnement géométrique.**

C'est le plancher d'arrondi de la pression f32 de S231, qui n'avait reçu que jusqu'à 128×64
(8 192 mailles) avec des résidus jusqu'à 8,75·10⁻⁷ et écrivait ne pas recevoir « toutes les échelles
numériques possibles ». Le banc 128×144 en sort, et le plancher y franchit le seuil d'un vingtième.
Le contrat du solveur n'est **pas** modifié dans ce lot : le refus est le comportement promis.
Consigné en **A272**.

## 7. Coût

CPU séquentiel Windows x86_64, secteur, un pas de 1 ms, médiane / maximum sur la période :

| nx × nz | mobile (ms) | itérations max | linéaire (ms), nz = nx | itérations max |
|---|---:|---:|---:|---:|
| 32 × 36 | 4,57 / 10,86 | 143 | 1,41 / 2,81 | 107 |
| 64 × 72 | 33,3 / 65,4 | 273 | 10,9 / 60,3 | 198 |
| 128 × 144 | 259,9 / 340,0 | 525 | 82,7 / 168,5 | 396 |

Le mode mobile coûte ≈3 fois le linéaire : 12 % de mailles de plus (bande d'air), 30 % d'itérations de
plus, l'advection, l'extrapolation et le test `wet` évalué dans chaque application de l'opérateur.
Aucun domaine n'y tient 2 ms ; aucune technique de coût n'est présente (multigrille, parallélisme,
cache de géométrie) ; ce n'est pas un verdict sur δ (ADR-131).

## 8. Limites

Surface graphe (un point par verticale) : ni déferlement ni retournement. Pas de mouillage du fond,
de bord ouvert, d'air, de cavité, de 3D, de parois mobiles ni de couplage B/W. Advection centrée sans
stabilité revendiquée, reçue sur une période seulement. Fond plat dans la réception ; le fond coupé
n'est éprouvé qu'au repos. Oracle potentiel : aucun effet visqueux ni rotationnel n'est reçu. Pression
f32 au-delà du domaine S231 : refus possibles (A272).
