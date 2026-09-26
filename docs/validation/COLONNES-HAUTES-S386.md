# Les colonnes hautes, mesurées avant d'être construites — S386

2026-09-26. **C2** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
[conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5) ; décision qui en sort : [ADR-208](../adr/ADR-208-la-colonne-graduee.md).
Session cloud, sans carte graphique. Liste **4.1**, **4.3** ; ADR-006 §6.4 (un `dx` plus fin en vertical près de la surface).

## Reproduire

- Commit `ca355677` (P5b de S386) ou plus récent ; Python 3, bibliothèque standard.
- `python outils/colonnes_hautes.py` — 42 s ; lignes `COLONNES_HAUTES_S386` : le contrôle S295, puis, par variante, le
  moins d'inconnues qui tient le critère (valeurs attendues au §2). `--tout` imprime chaque réglage.
- `python -m pytest -q outils/test_colonnes_hautes.py` — six essais, moins d'une seconde.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s386 -- --nocapture` — six essais, trois
  secondes ; lignes `S386 onde oblique` : 0,0046 % et 0,0048 % contre la fréquence du schéma gradué (§3).

## En une phrase

La dispersion d'une colonne **se calcule** ; calculée, une colonne haute unique à pression linéaire ne garde la précision
du schéma fin qu'en gardant dix-sept couches cubiques sur vingt-huit (÷1,47), quand une pression **linéaire par morceaux**
sur des nœuds étirés divise les inconnues par 2,55 — l'estimation de ÷3,1 faite en S384 était fausse.

## 1. L'instrument

`outils/colonnes_hautes.py`. Le pas linéaire de δ, pour un mode horizontal de valeur propre discrète `κ²`, se réduit dans
chaque colonne à un problème vertical — Neumann au fond, Dirichlet à demi-maille au couvercle — dont la réponse `φ` donne
`ω² = g·κ²·Σ φ·dx`, puis `Ω = acos(1 − ω²dt²/2)/dt` : c'est `scheme_frequency` de S295, écrit sous forme `A = D·M⁻¹·Dᵀ`. Une
colonne haute est une **restriction de Galerkin** : pressions dans un sous-espace `P`, vitesses dans `Q`,
`Aᵣ = (PᵀDQ)·M̂⁻¹·(PᵀDQ)ᵀ`, second membre `Pᵀ·d`.

| variante | pression | vitesses |
|---|---|---|
| **G** | linéaire sur les `m` mailles du bas | libres (`Aᵣ = PᵀAP`) |
| **Q** | idem | horizontale linéaire, verticale interne liée à celle du haut de la colonne haute |
| **E** | grille **étirée** en volumes finis, couches de `dx·r^j` sous `K` couches cubiques | — |
| **N** | **linéaire par morceaux**, nœuds aux centres de mailles fines, écarts ×`r` sous `K` couches cubiques | libres |

**Contrôles de l'instrument** (essais) : le schéma fin redonne les deux valeurs imprimées par l'essai de S295
(Ω/ω − 1 = −1,4355·10⁻² et −3,6827·10⁻³) ; `m = 1`, un nœud par maille et des couches uniformes redonnent le schéma fin ;
Galerkin ne baisse jamais `Σ φ·dx` (Ritz) ; opérateurs réduits symétriques à 10⁻¹² près.

## 2. Ce qui est mesuré

**Critère 2** (écrit avant) : sur λ de `4·dx` à `2·h`, 25 longueurs d'onde, l'erreur ajoutée `|Ω_réduit/Ω_fin − 1|` ne
dépasse pas `max(|Ω_fin/ω − 1|, 10⁻⁴)` — pas plus que la maille n'en fait déjà, ou une phase de 10⁻⁴ (≈ 3 mm sur une vague
de 0,5 m après une minute). Le moins d'inconnues par colonne qui le tient :

| variante | porte B : `h` 7 m, `dx` 25 cm, 28 couches | bassin : `h` 3 m, `dx` 10 cm, 30 couches |
|---|---|---|
| G | 17 cubiques + 1 : **19**, ÷1,47 (rapport 0,834) | 20 + 1 : **22**, ÷1,36 |
| Q | 16 + 1 : **18**, ÷1,56 (0,629) | — |
| E | `r` 1,2, 5 cubiques : **14**, ÷2,00 (0,966) | — |
| **N** | **`r` 1,25, 3 cubiques : 11, ÷2,55** (0,923) | **`r` 1,25, 6 cubiques : 14, ÷2,14** (0,918) |

Pire cas, partout : **la plus longue vague** (λ = 2·h). Avec huit couches cubiques — l'estimation de S384 —, la variante G
ajoute 1,4·10⁻² d'erreur de fréquence à 14 m, environ seize fois le permis. Galerkin relève la fréquence (Ritz), la
variante Q l'abaisse : le schéma fin étant trop lent, G et N rapprochent en partie de ω continue — un fait, pas un
principe de conception.

## 3. La colonne graduée dans la référence 3D — pas linéaire

`code/water-core/src/delta3d_graded.rs` ([ADR-208](../adr/ADR-208-la-colonne-graduee.md) D1). `Volume3::enable_graded(hôte,
nœuds)` réserve, avant `seal()`, cinq tampons de `colonnes × nœuds` flottants et les nœuds (I-06). La projection du pas
linéaire devient un gradient conjugué sur `Pᵀ·A·P·p̂ = Pᵀ·b` : `P` prolonge les valeurs nodales aux mailles fines
(linéaire entre deux nœuds), `Pᵀ` en est la transposée exacte — mêmes coefficients, mêmes arrondis —, et `A` est
l'opérateur fin reçu (S295), couvercle compris. Les vitesses restent fines ; la correction est celle du schéma reçu. Arrêt :
`‖r̂‖² ≤ 10⁻¹²·‖b̂‖²` sur le vrai résidu, puis la tolérance d'ADR-144 sur la **divergence restreinte** `Pᵀ·div u`, rapportée
au poids de chaque nœud — le champ n'est à divergence nulle qu'en moyenne pondérée par segment dans la partie graduée.
Refus `Domain` sur fond coupé et au pas mobile (C2b).

| critère (écrit avant) | résultat |
|---|---|
| **1** — le calcul redonne S295 | **tenu** (§1) |
| **2** — l'erreur ajoutée ≤ max(erreur du schéma fin, 10⁻⁴), λ de 1 à 14 m, porte B | **tenu** par chaque variante à son réglage (§2) ; **la colonne haute unique d'ADR-207 D2 ne le tient qu'à ÷1,47** ; ADR-208 retient la colonne graduée, ÷2,55 |
| **3** — opérateur réduit symétrique et positif ; repos exact ; volume conservé | **tenu** : cinq essais `s386` — symétrie à 10⁻⁵ et `Pᵀ` transposée de `P` (**vu échouer** sur une transposée faussée) ; repos au bit ; volume à 10⁻⁹ m³ sur 50 pas ; tous les nœuds = schéma fin à 10⁻⁶ m ; comptage exact ; refus du pas mobile, surface rendue au bit |
| **4** — l'onde oblique suit la fréquence de son propre schéma à 10⁻³ | **tenu** : **0,0046 %** (5 nœuds sur 8 couches) et **0,0048 %** (7 sur 16), comme la grille fine de S295 (0,0046 / 0,0047 %) ; Ω_gradué/Ω_fin − 1 = 1,808·10⁻³ et 2,020·10⁻³, identiques en Rust et en Python |
| **5** — sans colonne graduée, suite inchangée ; zéro avertissement | **tenu** : 673 réussis (667 + 6), 18 ignorés |

## 4. Ce que ce document ne dit pas

- **Aucun gain de coût mesuré** : les vitesses et l'opérateur restent sur la grille fine ; seules les **inconnues de
  pression** diminuent (11 sur 28 à la porte B). Le stockage compact exact (ADR-208 D2 : vitesse horizontale aux nœuds,
  verticale par segment) est C2b, comme la multigrille graduée.
- **Le pas mobile n'est pas éprouvé** : la surface y bouge, et les couches cubiques doivent contenir toute sa course
  (ADR-208 D4) ; le gain y sera plus petit que ÷2,55.
- **Une configuration par critère** : porte B et un bassin ; le critère 2 est à recalculer pour chaque domaine (l'outil le
  fait en une minute).
- **Rien sur la carte** : la production garde ses 28 couches (C3).
