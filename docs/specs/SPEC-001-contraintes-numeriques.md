# SPEC-001 — Contraintes numériques et fiche de référence chiffrée

- **Statut** : référence — les formules sont fermes, les valeurs dérivées sont des ordres de grandeur à confirmer par mesure
- **Session** : S01

Document de référence unique. Toute discussion de dimensionnement doit citer une ligne d'ici
plutôt que réinventer un chiffre.

---

## 1. Dispersion des vagues de gravité

**Eau profonde** (h > λ/2) : `ω = √(g·k)`, `c = √(gλ/2π)`, **`c_groupe = c/2`**.

| λ (m) | T (s) | c (m/s) | c_g (m/s) | « profond » si h > |
|---|---|---|---|---|
| 0,5 | 0,57 | 0,88 | 0,44 | 0,25 m |
| 1 | 0,80 | 1,25 | 0,62 | 0,5 m |
| 2 | 1,13 | 1,77 | 0,88 | 1 m |
| 5 | 1,79 | 2,79 | 1,40 | 2,5 m |
| 10 | 2,53 | 3,95 | 1,98 | 5 m |
| 30 | 4,38 | 6,84 | 3,42 | 15 m |
| 50 | 5,66 | 8,83 | 4,42 | 25 m |
| 100 | 8,00 | 12,5 | 6,25 | 50 m |
| 200 | 11,3 | 17,7 | 8,84 | 100 m |
| 500 | 17,9 | 27,9 | 14,0 | 250 m |

**Eau peu profonde** (h < λ/20) : `c = √(g·h)`, non dispersif, `c_g = c`.

| h (m) | c (m/s) | h (m) | c (m/s) |
|---|---|---|---|
| 1 | 3,13 | 20 | 14,0 |
| 2 | 4,43 | 50 | 22,1 |
| 5 | 7,00 | 100 | 31,3 |
| 10 | 9,90 | 200 | 44,3 |

> **L'énergie voyage à `c_g`, pas à `c`.** Toute largeur d'éponge, tout horizon de propagation,
> tout délai d'établissement se calcule avec `c_g`. En eau profonde, c'est deux fois plus lent
> qu'on ne le croit en regardant les crêtes.

## 2. Stabilité et coût d'un solveur volumétrique

### 2.1 CFL

`dt ≤ C·dx / u_max`, avec `C ≈ 1` pour une advection explicite, `C ≈ 3–5` pour une advection
semi-lagrangienne (au prix de diffusion).

> **Note corrective S27 — `u_max` n'est pas défini, et le manque n'est pas cosmétique.**
>
> Cette formule est écrite depuis S02 sans que `u_max` soit qualifié. Or **SPEC-004 §10.1** pose
> comme exigence *non négociable* d'accepter « une frontière en mouvement **avec sa vitesse** ».
> Les deux documents se contredisent en silence : l'un impose des parois mobiles, l'autre calcule
> le pas de temps comme si elles n'existaient pas.
>
> **Sur une face au contact d'une paroi mobile, la vitesse qui transporte l'information à travers la
> face est celle du fluide *relative à la paroi*.** C'est elle que le pas doit borner. Ailleurs —
> face pleine, loin de tout solide — la vitesse absolue reste la bonne, faute de paroi par rapport à
> laquelle se déplacer.
>
> **Ce défaut a été mesuré**, sur un autre projet et une autre architecture *(source citée dans le
> journal S27)* : eau au repos, solide mobile, **borne de pas de temps nulle** pendant que le nombre
> de Courant réel valait **0,943**. Rapport `C_relatif / C_absolu` mesuré entre **2,2 et 2,5**, et
> **zéro violation déclarée** — le contrôle de stabilité comparait la vitesse absolue au budget.
> Un solveur peut donc violer sa condition de stabilité d'un facteur deux **en restant vert**.
>
> **Définition retenue** (ADR-035 §2) : `u_max` est le maximum, sur toutes les faces portant une
> inconnue, de la **vitesse gouvernante** — relative à la paroi sur une face coupée, absolue
> ailleurs. Le mot « max » ne se lit pas sur le champ de vitesse seul.
>
> **Et la revue croisée S08 avait examiné cette paire** (écart E08, `004 §6.2 ↔ 001 §2.4`) sans la
> voir : le rapprochement portait sur le coût, pas sur la définition d'une variable non définie.
> Angle mort **A125**.

| dx (m) | u_max = 5 m/s | u_max = 15 m/s | sous-pas à 30 Hz (u=15, C=3) |
|---|---|---|---|
| 0,50 | 100 ms | 33 ms | 1 |
| 0,25 | 50 ms | 17 ms | 1 |
| 0,10 | 20 ms | 6,7 ms | 2 |
| 0,05 | 10 ms | 3,3 ms | 4 |
| 0,02 | 4 ms | 1,3 ms | 9 |

### 2.2 Loi de coût — la règle à retenir

Halver `dx` multiplie le nombre de cellules par 8 **et** le nombre de sous-pas par 2.

```
coût ∝ dx⁻⁴
```

Coût relatif, base `dx = 0,25 m` :

| dx | ×cellules | ×sous-pas | **×coût total** |
|---|---|---|---|
| 0,50 | 0,125 | 0,5 | **0,06** |
| 0,25 | 1 | 1 | **1** |
| 0,10 | 15,6 | 2,5 | **39** |
| 0,05 | 125 | 5 | **625** |
| 0,02 | 1953 | 12,5 | **24 400** |

C'est la raison pour laquelle « augmenter la résolution » n'est jamais une réponse acceptable à un
problème de qualité, et pourquoi la décomposition en couches (ADR-001) est une nécessité et non un
raffinement.

### 2.3 Comptage de cellules — domaine bateau de référence

Emprise 24 × 12 × 6 m (1 728 m³).

| dx | plein | **épars** (bande de 1,5 m sous la surface + coque) |
|---|---|---|
| 0,50 | 13,8 k | 3,5 k |
| 0,25 | 111 k | 28 k |
| 0,10 | 1,73 M | 432 k |
| 0,05 | 13,8 M | 3,46 M |

L'allocation éparse (ADR-006 §3.1) divise le comptage par ≈4 sur ce cas — davantage sur une
colonne verticale ou une traînée.

### 2.4 Mémoire

Cellule eulérienne typique : vitesse (3×f32) + pression (f32) + fonction de niveau (f32)
+ drapeaux ≈ **24–32 octets**, hors particules.

| Domaine | cellules éparses | mémoire |
|---|---|---|
| Bateau, dx = 0,10 | 432 k | ≈ 13,8 Mo |
| Impact, dx = 0,05, 6×6×4 m | 1,15 M | ≈ 37 Mo |
| Déferlement, dx = 0,25, 120×20×5 m | 384 k | ≈ 12 Mo |

> **Note corrective (S08, écart E01).** La colonne « cellules éparses » ci-dessus n'applique pas
> un taux unique, contrairement à ce que le « ≈4 » de §2.3 laisse croire : bateau **÷4**
> (1,73 M → 432 k), impact **÷1** (le comptage donné *est* le comptage plein — une cavité d'impact
> est presque pleine), déferlement **÷2** (768 k → 384 k, un rouleau occupe environ la moitié de sa
> boîte). Les trois taux sont défendables, aucun n'était justifié. Le taux d'occupation est une
> propriété du **phénomène**, pas de l'allocateur : il se déclare par cas, et le « ≈4 » de §2.3 ne
> vaut que pour le domaine bateau qui l'accompagne.

Le budget de 384 Mo d'ADR-012 §3 correspond donc à ≈24 domaines de type bateau, ou 10 domaines
d'impact fin. **Cohérence vérifiée** — le budget n'a pas été choisi arbitrairement.

## 3. Énergie et amplitude

- Densité d'énergie d'une houle irrégulière : `E = (1/16)·ρ·g·Hs²` [J/m²].
  `Hs = 1 m → 613 J/m²` ; `Hs = 2 m → 2 452 J/m²` ; `Hs = 4 m → 9 810 J/m²`.
- Flux d'énergie (puissance par mètre de crête) : `P = E · c_g` [W/m].
  `Hs = 2 m`, `T = 8 s` → `P ≈ 2 452 × 6,25 ≈ 15,3 kW/m`.
- Déferlement en eau peu profonde : `H/h ≈ 0,78` (McCowan). Voir ADR-005 §4.1 pour la largeur de
  zone de surf en fonction de la pente.
- Cambrure limite en eau profonde (Stokes) : `H/λ ≈ 1/7`. Au-delà, la crête déferle.
  Utilisable comme **déclencheur d'écume** directement sur B, sans aucune simulation.

## 4. Houle limitée par le fetch

`Hs ≈ 0,0016 · U10 · √(F/g)` (F en m, U10 en m/s).

| U10 | F = 1 km | 3 km | 10 km | 100 km |
|---|---|---|---|---|
| 5 m/s | 0,08 m | 0,14 m | 0,26 m | 0,81 m |
| 10 m/s | 0,16 m | 0,28 m | 0,51 m | 1,62 m |
| 20 m/s | 0,32 m | 0,56 m | 1,02 m | 3,23 m |

Sert à borner automatiquement `Hs` dans les lacs, les baies et les eaux abritées : le paramètre
n'est pas laissé à l'appréciation d'un auteur.

## 5. Sillages

- Demi-angle de Kelvin en eau profonde : **19,47°**, indépendant de la vitesse.
- Longueur d'onde des vagues transverses : `λ = 2πv²/g`.

| v | 5 m/s | 8 m/s | 10 m/s | 15 m/s | 20 m/s |
|---|---|---|---|---|---|
| λ | 16 m | 41 m | 64 m | 144 m | 256 m |

- Nombre de Froude de profondeur : `Fr_h = v/√(gh)`. Au-delà de 1, le sillage devient un cône de
  demi-angle `arcsin(1/Fr_h)` et les vagues transverses disparaissent. Voir ADR-011 §4.

## 5 bis. Impact d'entrée dans l'eau

*(Section ajoutée en S14. Ces trois relations vivaient dans ADR-023 §2, c'est-à-dire hors des deux
documents qu'I-14 désigne comme seules sources de formules. Migrées ici plutôt que d'élargir
l'invariant : le prix d'I-14 est qu'il n'existe qu'un seul endroit où chercher un nombre.)*

Théorie de Wagner, pour une carène de **relèvement de fond** `β` entrant à la vitesse `v`.

**Demi-largeur mouillée** : `c(t) = (π/2)·v·t / tan β`, d'où la **durée d'impact** pour mouiller une
demi-largeur `b` :

```
t_impact = 2·b·tan β / (π·v)
```

| `b` | `β` | `v` | `t_impact` | à 30 Hz |
|---|---|---|---|---|
| 1,0 m | 30° | 5 m/s | 73 ms | 2,2 ticks |
| 0,20 m | 45° | 7,7 m/s | 17 ms | 0,5 tick |
| 2,0 m | 10° | 4 m/s | 56 ms | 1,7 tick |

**Coefficient de pression maximal** : `C_p = 1 + (π / (2·tan β))²`, `p_max = C_p · ½ρv²`.

| `β` | 45° | 30° | 20° | 10° | 5° |
|---|---|---|---|---|---|
| `C_p` | 3,5 | 8,4 | 19,6 | 80,4 | 323 |
| `p_max` à 5 m/s | 43 kPa | 105 kPa | 245 kPa | 1,0 MPa | 4,0 MPa |

**C'est l'angle qui domine, pas la vitesse** : 30° → 10° multiplie la pression par dix, doubler la
vitesse ne la multiplie que par quatre. Et **la formule diverge quand `β → 0`** — le coussin d'air
piégé et la compressibilité l'écrêtent. La pression de pic n'est donc pas une grandeur connue.

**Masse ajoutée d'une plaque plane** de demi-largeur `c`, par mètre de longueur :

```
m_a = ½·π·ρ·c²            impulsion d'impact :  J = Δ(m_a) · v_rel
```

C'est cette dernière que le système publie (ADR-023 §2.3) : un bilan de quantité de mouvement, borné
par la masse d'eau réellement accélérée, là où la pression de pic publierait son incertitude.

---

## 6. Hydraulique (couche V)

- Orifice : `Q = C_d·A·√(2·g·Δh)`, `C_d ≈ 0,62` (arête vive), `0,82` (ajutage).
- Déversoir : `Q = (2/3)·C_d·b·√(2g)·H^(3/2)`, `C_d ≈ 0,60`.
- Manning : `v = (1/n)·R^(2/3)·S^(1/2)` ; `n ≈ 0,035` cours d'eau naturel, `0,013` béton lisse.
- Froude fluvial : `Fr = v/√(gh)` ; ressaut hydraulique au passage de 1.
- Ballottement, premier mode d'un réservoir de longueur L et profondeur h :
  `T = 2π / √( g·(π/L)·tanh(π·h/L) )`.
- Seiche d'un bassin fermé : `T = 2L/√(g·h)`.

## 7. Précision et temps

- ulp d'un `f32` à distance `d` : `d·2⁻²³`. → origine flottante obligatoire au-delà de 4 096 m
  (ADR-002 §2.3).
- ulp d'un `f32` de temps à `t = 10⁶ s` : **62,5 ms**. → le temps ne transite jamais en `f32`
  (ADR-003 §2.2).
- Erreur de hauteur induite par une erreur d'horloge : `Δz ≈ A·ω·Δt`.

## 8. Sources des formules

Théorie linéaire des vagues (Airy), McCowan pour le déferlement, Stokes pour la cambrure limite,
Wagner et von Karman pour l'impact d'entrée et la masse ajoutée,
relations JONSWAP/SMB pour le fetch, Kelvin pour le sillage, Manning et Torricelli pour
l'hydraulique. Toutes classiques, aucune n'est spécifique au projet ; leur intérêt ici est d'être
**réunies avec les ordres de grandeur du projet**.

## Complément S75 — énergie du premier impact spectral linéaire

ADR-058 : en eau profonde uniforme, eta=a cos(k·r) cos(omega t),
psi=-a omega/k cos(k·r) sin(omega t), omega²=gk. L’énergie intégrée vaut
rho/2 intégrale(g eta² + psi deta_dt) dS. Sur un carré périodique de côté L,
les modes cosinus distincts et non opposés sont orthogonaux ; un mode porte rho*g*L²*a²/4.
La normalisation du champ Impact utilise cette identité, vérifiée par quadrature spatiale.
Ce complément ne fixe ni le spectre de source ni les seuils de qualité B2.

## Complément S77 — normalisation du candidat radial

Convention eta(r)=intégrale A(k)J0(kr)k dk. Parseval donne
E_initial=pi*rho*g intégrale A(k)²k dk. Pour A=Cx²(1-x)², x=(k-a)/(b-a),
le facteur intégral est C²(b-a)(a+(b-a)/2)/630. Voir ADR-060 pour sa dérivation et
la quadrature spatiale de contrôle ; la bande [k0/2,2k0] reste à calibrer B2.
