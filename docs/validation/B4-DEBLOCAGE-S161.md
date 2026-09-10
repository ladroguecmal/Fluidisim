# S161 — B4 : ce qui bloque, et ce qui ne bloque pas

2026-09-10. Instruit le déblocage de **B4**, désigné par ADR-109 comme seule voie ouverte et
attendu seul par **A214**. Protocole : [PLAN-BENCHMARK](PLAN-BENCHMARK.md) §B4.

## 1. Ce que B4 demande, volet par volet

> Même scène simulée deux fois : (a) perturbative B+W+δ, (b) **substitutive intégrale de
> référence, à résolution élevée**. Comparaison de la surface, des forces sur la coque, et de la
> perception en double aveugle.

| volet | état |
|---|---|
| comparaison de **surface** | demande la référence intégrale, rien d'autre |
| comparaison des **forces sur la coque** | demande la référence **et** un intégrateur de corps rigide, que le corpus range en construction (BILAN-S69 §5.3) |
| **perception en double aveugle** | demande des personnes — hors de portée d'une session (`REPRISE.md` §5) |
| contrôle du **terme source** (ajout S04, A50) | demande la référence, et un `S` que l'on sait tronquer |

**Un seul obstacle est commun aux quatre : la référence.** C'est bien elle qu'il faut instruire, et
c'est ce que le corpus dit depuis S158 sans l'avoir ouverte.

## 2. Ce que la référence doit être — et ce qu'elle ne peut pas être

Elle doit calculer **le champ total** dans son emprise, sans décomposition, pour qu'on puisse
comparer `simuler(A) + simuler(B)` à `simuler(A et B ensemble)`.

**Un solveur linéaire ne peut pas servir.** `dispersif.rs` — le milieu à dispersion exacte de S39,
instrument qui a rétracté ADR-042 — est **linéaire** : la superposition y est vraie par
construction, et l'écart mesuré serait nul quel que soit le rapport d'amplitude. Il mesurerait la
justesse de son intégration, pas la validité de l'additivité. C'est le piège le plus proche, parce
que ce fichier est le plus « référence » d'apparence du dépôt.

**Le candidat est donc un solveur non linéaire**, et le dépôt en a un : `shallow.rs`, Saint-Venant
1D en `f64`, bien équilibré, flux HLL, second ordre et RK2 optionnels, 1 165 lignes, reçu par le
mode `physics` du harnais sur six montages canoniques et par l'oracle croisé de S37. Sa
non-linéarité est celle qui compte ici : advection et terme `h·u`.

## 3. Ce que le dépôt possède, mis à plat

| pièce | nature | utilisable comme référence B4 ? |
|---|---|---|
| `shallow.rs` | Saint-Venant 1D, **non linéaire**, f64 | **oui**, en 1D |
| `delta.rs` | même modèle, f32, véhicule d'essai | comme témoin croisé, pas comme référence |
| `dispersif.rs` | milieu **linéaire** à dispersion exacte | **non** — additif par construction |
| `background.rs` | houle analytique de Gerstner | c'est le `B` à tester, pas la référence |
| oracle croisé (S37) | confronte `delta` et `shallow` | méthode déjà écrite, réutilisable |

**Le blocage n'est donc pas « il n'y a pas de référence ».** Il est que la référence disponible est
**1D et non dispersive**, quand `B` est une houle dispersive 2D. Ce que cela autorise et interdit
est la question de la section suivante — et c'est une question de conception, pas d'outillage.
