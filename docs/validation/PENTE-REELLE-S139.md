# S139 — La borne de pente, la pente réelle, et ce que le budget additionne

2026-09-10. [ADR-094](../adr/ADR-094-d-ou-vient-la-limite-de-pente.md) actée. A205 traitée.
S138-1. Sonde : `code/water-core/examples/pente_reelle.rs`.

## 1. La question n'était pas celle qu'on croyait

A205 demandait d'où vient `max_slope`, valant 0,1 sans provenance depuis S77 et renvoyé « à
calibrer B2 ». La limite physique, elle, était déjà écrite : SPEC-001 §4, cambrure limite de
Stokes `H/λ ≈ 1/7`, pente `πH/λ = 0,4488`.

Ce qui manquait est **le rapport entre la grandeur comparée et la grandeur bornée**. `max_slope`
est confronté à `slope_bound = Σ|a_k·k·dk|·k`, une borne L1 obtenue en majorant `|J1| ≤ 1` — pas
la pente du champ. ADR-058 §21 et ADR-062 §50 le disaient tous deux (« borne conservative »),
sans dire de combien. **Un qualificatif n'est pas un nombre**, et c'est ce qui a permis au seuil
de rester sans provenance pendant soixante sessions.

## 2. Où la pente est maximale

λ = 4 m, E = 0,01 J, N = 64. Profil de `|∇η|` à `t = birth`, en unités de λ :

| r/λ | 0,05 | 0,10 | 0,15 | 0,20 | **0,2062** | 0,25 | 0,30 | 0,40 | 0,45 | 0,60 |
|---|---|---|---|---|---|---|---|---|---|---|
| `\|∇η\|` ×10⁻³ | 0,439 | 0,812 | 1,064 | 1,163 | **1,1645** | 1,106 | 0,914 | 0,312 | 0,012 | 0,418 |

**Le maximum vaut 1,164464e-3 en `r = 0,2062 λ`**, pour une borne `slope_bound = 2,090296e-3`.

Sur 201 instants répartis sur les 2 s d'horizon, **le maximum temporel est l'instant initial** —
attendu, les phases `cos(ω_k t)` y valant toutes 1, mais non trivial : les `J1(k r)` n'ont pas le
même signe à un `r` donné, si bien que rien n'interdisait *a priori* qu'une combinaison
ultérieure dépasse.

## 3. Le rapport est une constante du modèle

`ρ = slope_bound / max|∇η|`. Il ne dépend d'aucun paramètre fourni par l'appelant :

| balayage | plage | ρ | `r_max/λ` |
|---|---|---|---|
| longueur d'onde | 0,5 → 32 m | **1,795071** partout | 0,2062 partout |
| énergie | 1e-4 → 100 J | 1,795070 à 1,795071 | — |
| nombre de modes | N = 64, 128, 256 | 1,795070 à 1,795071 | — |
| rayon du domaine | 0,5 → 8 λ | 1,795071 partout | — |
| grille de recherche | 200 → 40 000 points | 1,79507 dès 200 points | — |

C'est structurel : la forme spectrale d'ADR-060 est fixe, le champ est exactement homothétique
en λ (S136), et `ρ` est le rapport de deux intégrales de la même forme — un nombre pur.

**Vérification indépendante.** Quadrature f64 hors du dépôt, `J1` par approximation rationnelle,
N de 64 à 4096 : **ρ = 1,795071271**, `r_max = 0,206168 λ`, stable dès N = 256. La sonde en f32
via `sample()` donne 1,795071 : les deux chaînes, qui ne partagent ni le type ni la fonction de
Bessel, coïncident à sept chiffres.

Borne inférieure théorique : `ρ ≥ 1/max|J1| = 1/0,5819 = 1,7185`. La valeur mesurée en est à 4,5 %,
ce qui situe l'essentiel du conservatisme dans la majoration `|J1| ≤ 1` elle-même.

## 4. La pente réelle est-elle sûre comme borne ?

`slope_max() = slope_bound/ρ` n'est exacte qu'à `t = birth`. S'en servir comme majorant exige
qu'aucun instant ne la dépasse — une propriété de **sûreté**, pas de précision. Balayage dense
(1 000 instants × 2 000 rayons) :

| λ | max à t = 0 | max sur t > 0 | rapport |
|---|---|---|---|
| 0,5 | 7,452573e-2 | 7,450039e-2 | 0,999660 |
| 4 | 1,164465e-3 | 1,164415e-3 | 0,999957 |
| 32 | 1,819476e-5 | 1,819466e-5 | 0,999995 |

Jamais dépassée, et de peu — le rapport tend vers 1 par en dessous parce que le premier instant
échantillonné est proche de zéro, pas parce qu'une marge s'évanouit. **Mesuré, non démontré** :
aucune preuve analytique n'est produite ici, et c'est la limite de ce résultat.

## 5. Le budget additionne trois grandeurs de natures différentes

`composition.rs` : `bound = steepness_B·π + Σ slope_bound (+ slope_envelope)`, comparé à
`max_slope`. Les trois termes ne sont pas la même chose :

| terme | nature | facteur au-dessus de la pente réelle |
|---|---|---|
| `steepness_B · π` | pente **exacte** du fond | 1 |
| `slope_bound` (impact radial) | borne L1 | **1,7950713**, constante |
| `slope_envelope` (pression) | borne L1, `Σ(|kx|+|ky|)(|Re η|+|Im η|)` | non mesuré ; 1 à 2 pour une case seule, davantage au-delà |

Le facteur de la pression n'est pas une constante et ne peut pas l'être : `(|kx|+|ky|)` majore
`|k|` d'un facteur 1 à √2 selon la direction, `(|Re| + |Im|)` majore `|η|` d'un facteur 1 à √2
selon la phase, et la somme sur les cases perd en plus toute compensation entre elles.

**Un même nombre de budget ne dit donc pas le même état physique selon qui le consomme.** À
budget 0,4488 : le fond B seul est exactement à la limite de déferlement ; un impact radial seul
est à 0,2500, soit 55,7 % de cette limite ; une pression seule est quelque part entre les deux
et le double.

C'est la raison pour laquelle `max_slope` ne se dérivait pas : **le seuil est physiquement juste
pour au plus un des trois termes à la fois.**

## 6. Ce que le seuil actuel achète

| | valeur |
|---|---|
| pente de déferlement (Stokes, `πH/λ`) | 0,448799 |
| `max_slope` employé depuis S77 | 0,1 |
| pente réelle admise à 0,1, impact seul | **0,055708** — 12,4 % de la limite |
| cambrure admise à 0,1, fond B seul | `H/λ = 0,0318` — 1/31,4 au lieu de 1/7 |
| seuil qui serait juste **si le budget ne contenait que des impacts** | 0,4488·ρ = 0,805626 |
| facteur sur l'énergie admissible d'un impact | ×64,9 (`E ∝ pente²`) |

## 7. Ce qui est décidé, et ce qui ne l'est pas

**Décidé** (ADR-094) : `max_slope` est la pente de déferlement, dérivée de SPEC-001 §4, valeur
retenue 0,4488, lecture alternative 0,5774 (crête à 120° de la vague limite) — écart borné de
29 %, dit et non tranché arbitrairement. Plus aucun banc n'a à le calibrer.

**Publié** : `RadialImpact::slope_max()`, la pente réelle du champ, sans changer aucun
comportement.

**Non fait, et c'est une action** : migrer le refus `Steepness` et le budget d'ADR-080 vers les
pentes réelles. Cela déplace la frontière d'admission de tous les champs et change les hachages
de campagne — S139-1.

**Non mesuré** : le facteur de conservatisme de la pression (A206). **Non démontré** :
l'applicabilité du critère de Stokes à un paquet transitoire, employé ici comme majorant
géométrique (A207).
