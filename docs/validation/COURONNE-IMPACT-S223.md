# Une inégalité conjointe pour la somme spatiale des impacts — S223, 2026-09-13

Traite **A262** (S222) : la somme des majorants d'impact ignorait la distance entre champs, et
c'était le goulot du budget de pente. Décision : [ADR-138](../adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md).

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : majorant d'impact à dispersion (ADR-133) ; enveloppe par couronne et
  inégalité de position relative (ADR-138, construites ici). Maximum réel relevé au pas de 0,25 m
  sur **l'intersection des disques**, seule région que la composition admet (ADR-077, ADR-080).
- **Techniques absentes** : aucune technique de rendu — GPU, LOD, visibilité, mutualisation ; un
  seul fil. Ce document mesure un **contrat d'admission**, pas une image.
- **Domaine de validité** : `bessel` exécutée sur `[0 ; 2048]` ; impacts de la scène J1
  (λ 3,35 m, E 164 J, rayon 52 m), deux et trois champs, séparations 0 / 5 / 20 / 50 / 90 m, âges
  0 / 2 / 8 s ; famille de réception de l'enveloppe : λ ∈ {0,75 ; 2 ; 3,35 ; 5} m, six instants
  adimensionnés, 25 couronnes. Impacts **isotropes** — `RadialImpact::new` refuse les autres.
  **Ne dit rien** du terme de pression, ni d'un autre profil radial qu'ADR-094.
- **Rang de passage** : deuxième et troisième passages pour les coûts ; premier écarté (machine
  froide, L289).

## 1. L'inégalité que j'ai supposée est fausse, et c'est le premier résultat

La thèse déclarée en P1 s'appuyait sur `|J_ν(x)| ≤ √(2/πx)`. **Elle ne vaut que pour `ν = 1/2`**,
où `J_{1/2}(x) = √(2/πx)·sin x` — une égalité. Pour `ν = 1`, mesuré sur `bessel` telle que le
programme la calcule, 4 millions d'échantillons par régime :

| régime | pire rapport à l'asymptote | en | dépassement |
|---|---:|---:|---:|
| table de Hermite (x ≤ 64) | **1,034023** | x = 2,1656 | **3,4 %** |
| asymptotique (x > 64) | 1,000044 | x = 65,18 | 44 ppm |

Les 3,4 % sont **la fonction elle-même** : l'asymptote est la limite en `+∞`, pas un majorant. Les
44 ppm viennent des termes correctifs du développement (A&S 9.2.1) — donc de l'approximation.

**La constante qu'il faut** : `sup_x |J₁(x)|·√x = 0,825031` en `x = 2,165952`, contre l'asymptote
`0,797885`. Et le palier : la table rend `0,581865191` en `x = 1,840658` contre le maximum théorique
`0,581865013`, soit **0,36 ppm**.

**Cela éclaire une constante du dépôt.** `SLOPE_L1_RATIO = 1,795071`, mesuré en S141, vaut
`1/0,5819 × 1,045` : **c'était le pic de `J₁`** que la mesure retrouvait. Le code posait `|J₁| ≤ 1`
en commentaire — « borne conservative » — et la calibration rattrapait le facteur derrière.

## 2. L'enveloppe par couronne

```text
|dη/dr| ≤ (1 + 1e-4) · Σ_n |c_n| k_n · min( 0,5818650 ; 0,8250310 / √(k_n r) )
```

et `slope_max_beyond(t, r)` rend le **minimum** de cette borne et de `slope_max_at(t)` — deux
majorants du même champ, l'un calibré et reçu (ADR-133), l'autre une inégalité à constante mesurée.

**Réception** (`slope_max_beyond_bounds_the_annulus_and_tightens_s223`) : quatre longueurs d'onde,
six instants, **25 couronnes** par instant et 1 501 points de rayon chacune — jamais dépassée ;
jamais au-dessus de `slope_max_at` ; **égalité au bit** à `radius = 0` ; resserrement **supérieur à
3** à la naissance sur `r ≥ 15,5 λ`, là où ADR-133 ne donne rien (ρ = 1).

## 3. L'inégalité conjointe, et sa sûreté entre les échantillons

Soit `c₁` le centre du champ d'ancrage et `r₁ = |p − c₁|`. L'inégalité triangulaire donne
`r_i ≥ |d_i − r₁|`, et `slope_max_beyond` décroît, donc pour **tout** point `p` :

```text
Σ_i F_i(r_i)  ≤  F₁(r₁) + Σ_{i≠1} F_i(|d_i − r₁|).
```

Sur une cellule `[a, b]` de `r₁`, `F₁` est majorée par `F₁(a)` — elle décroît — et
`F_i(|d_i − r₁|)` par `F_i(δ)` avec `δ` la plus petite distance atteignable, **nulle si
`d_i ∈ [a, b]`**. **Aucune constante de Lipschitz** : les deux termes sont monotones du bon côté.

**Sûreté vérifiée** : `conjointe ≥ maximum réel` sur les 30 configurations mesurées.

## 4. Ce que l'inégalité rend

| impacts | d (m) | âge | somme | conjointe | gain | maximum réel |
|---:|---:|---:|---:|---:|---:|---:|
| 2 | 0 | 0 s | 0,425215 | 0,425215 | **1,0000** | 0,424938 |
| 2 | 20 | 0 s | 0,425215 | 0,258735 | 1,6434 | 0,212491 |
| 2 | 50 | 0 s | 0,425215 | 0,241167 | **1,7632** | 0,212470 |
| 3 | 20 | 0 s | 0,637822 | 0,302775 | 2,1066 | 0,212500 |
| 3 | 50 | 0 s | 0,637822 | 0,269244 | **2,3689** | 0,212471 |
| 3 | 90 | 0 s | 0,637822 | 0,248618 | **2,5655** | ~0 |

**À séparation nulle le gain vaut exactement 1,0000** : la borne ne gagne que là où la géométrie le
permet, et c'est ce qui la rend crédible. **Le gain est maximal à la naissance** — exactement là où
ADR-133 ne donne rien et où le budget saturait.

**Admission, et c'est A262 dans ses propres termes** :

| impacts | d (m) | âge | part de π/7 avec la somme | avec l'inégalité |
|---:|---:|---:|---:|---:|
| 2 | 50 | 0 s | 94,8 % | **53,7 %** |
| 3 | 50 | 0 s | **142,1 % — refusé** | **60,0 % — admis** |
| 3 | 90 | 0 s | 142,1 % — refusé | **55,4 % — admis** |

**Trois impacts frais séparés de cinquante mètres dépassaient π/7 et passent.**

## 5. Le coût, et le choix d'échantillonnage

| intervalles | borne (2 impacts, d = 20, âge 0) | coût |
|---:|---:|---:|
| 4 | 0,287975 | 6,3 µs |
| **8** | **0,267912** | **13,3 µs** |
| 16 | 0,262073 | 28,2 µs |
| 64 | 0,258735 | 121,3 µs |
| 128 | 0,258241 | 262,3 µs |

**Huit intervalles rendent 96,5 % du gain de soixante-quatre pour 11 % de son coût** — 0,7 % du
budget d'image de 2 ms. Le balayage est linéaire en intervalles, la qualité ne l'est pas ; monter à
64 porterait le coût à 6 % du budget pour 3,5 % de borne en plus. `JOINT_SLOPE_SAMPLES = 8`.

À comparer avec S222 : la borne locale de **pression** rendait 2,32 pour **25 s**, et son prix était
une loi d'échelle. Ici, 2,37 pour **13 µs** — six ordres de grandeur d'écart, parce qu'un champ
radial a un support compact et une décroissance, quand une somme de modules n'a ni l'un ni l'autre.

## 6. Contrôles

- **Zéro attente de test modifiée**, et la raison est structurelle : à un seul champ le chemin est
  celui d'avant **au bit** (l'aiguillage ne se déclenche qu'à deux), et le résultat est le minimum
  avec la somme d'origine, donc jamais plus lâche.
- Suite complète `code/` hors réseau : **371 réussis (273 + 4 + 1 + 93), 5 ignorés** — un de plus
  qu'en S222.
- Hôte : la scène J1 ne porte qu'un impact, donc l'aiguillage n'y joue pas — `floor` 0,167438 et
  0,162917, `d_eta_m = 0,000000000`, `VERIFY` 7,2271e-5 m, `--smoke` 120 images : **identiques à
  S222**. La voie neuve ne perturbe pas la voie existante.

## Suite

**A262 est traitée ; ce qui reste d'A254 est le terme de pression**, et S222 a montré que le rendre
spatial coûte des secondes (A261). Les deux termes du budget ont maintenant chacun leur limite
connue, et elles ne sont pas de même nature : l'impact est borné par une **inégalité** à 13 µs, la
pression par une **loi d'échelle** à 25 s.

Ce qui n'a pas été instruit ici : l'**ancrage** du balayage — le champ au plus grand majorant
global est un choix, pas un optimum, et rien ne dit lequel resserre le plus ; le cas de **plus de
trois champs**, où le balayage reste linéaire mais la perte de l'inégalité triangulaire pourrait
croître ; et l'usage de la borne pour autre chose que l'admission.

Restent, inchangés : A261, A258, le coût de passe, la cadence complète de l'hôte, la loi GPU,
J2/δ général et V-noyau (ADR-127).
