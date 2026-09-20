# Anatomie d'une surface de mer, et ce qui manque à la nôtre — S303

2026-09-20. Origine : **verdict R11** de l'utilisateur — « la mer ne fait pas réaliste […] la
topologie est à revoir […] selon moi déjà les micro vaguelettes ou pics doivent être convexes
plutôt que concaves », avec consigne de rechercher. Recherche documentaire, puis mesure sur la
**même réalisation** que le rendu. Aucune valeur physique sans provenance (I-14) : les valeurs de
la littérature portent leur source, les nôtres portent leur instrument.

## 1. De quoi une mer du large est faite

Trois échelles coexistent, et leur **interaction** est ce qui fait l'aspect :

1. **La houle** — longue, régulière, née d'un vent lointain, peu cambrée.
2. **La mer de vent** — plus courte, plus cambrée, étalée en direction, portée par le vent local.
3. **Les rides et capillaires** — centimétriques, portées par les vagues plus longues. Sous
   l'action conjointe de la gravité et de la tension superficielle, des rides de faible longueur
   d'onde se forment **sur la face avant** des vagues courtes (5 à 50 cm), engendrées par la crête
   quand celle-ci devient raide ([Longuet-Higgins 1963](https://www.cambridge.org/core/journals/journal-of-fluid-mechanics/article/abs/generation-of-capillary-waves-by-steep-gravity-waves/588B79E6D7B32CB9283F8E190A9B5E03),
   [revue moderne](https://www.mdpi.com/2077-1312/9/11/1217)).

Aucune de ces trois échelles n'est symétrique, et c'est le fond du retour de l'utilisateur.

## 2. Les quatre asymétries, et comment on les mesure

| grandeur | définition | ce qu'elle décrit |
|---|---|---|
| **Sk** (asymétrie verticale) | `⟨η³⟩/⟨η²⟩^{3/2}` | crêtes pointues, creux plats — « convexe plutôt que concave » |
| **As** (asymétrie horizontale) | `⟨H(η)³⟩/⟨η²⟩^{3/2}`, `H` transformée de Hilbert | vague penchée vers l'avant |
| **c₀₃, c₂₁** (asymétrie des pentes) | cumulants de Gram-Charlier de la pente, le long du vent | face avant plus raide, glitter décentré |
| **c₄₀, c₂₂, c₀₄** (aplatissement des pentes) | cumulants d'ordre 4 | rareté des fortes pentes : « pics » plutôt que bosses |

Définitions d'Elgar (1987), reprises telles quelles ; valeurs typiques observées : `Sk` positive
entre 0 et 1, `As` négative entre −1,5 et 0
([synthèse](https://en.wikipedia.org/wiki/Wave_nonlinearity)).

**En eau profonde**, l'asymétrie verticale vient des **harmoniques liées du second ordre** : les
crêtes deviennent plus pointues et les creux plus plats à mesure que la cambrure augmente
([synthèse 2025](https://arxiv.org/pdf/2508.05651)). Pour une bande **étroite**, la théorie donne
`Sk = 3·k̄·σ` ([Longuet-Higgins 1963](https://www.cambridge.org/core/journals/journal-of-fluid-mechanics/article/abs/effect-of-nonlinearities-on-statistical-distributions-in-the-theory-of-sea-waves/B8580FCE9F7A8264417EAC31F803077D),
[rappel](https://arxiv.org/pdf/nlin/0503071)) — pour **notre** mer, `k̄` = 0,083 m⁻¹ et
`σ` = 0,625 m, donc **0,156**.

**Les pentes** se mesurent depuis 1954 par le miroitement du soleil : Cox & Munk donnent
`mss = 0,003 + 5,12·10⁻³·U`, l'asymétrie `c₀₃ = 0,04 − 0,033·U`, `c₂₁ = 0,01 − 0,0086·U`, et les
aplatissements `c₄₀ = 0,40`, `c₂₂ = 0,12`, `c₀₄ = 0,23`
([article original](https://userpages.umbc.edu/~martins/phys650/Cox%20and%20Munk%20Glint%20paper.pdf)).
La révision par 150 millions d'observations IASI confirme la `mss` et **révise les aplatissements**
— `c₄₀` maximal à 0,32 vers 6–9 m/s, et un **changement de signe** de l'asymétrie vers 5–6 m/s
([Bréon & Henriot, révision IASI](https://arxiv.org/html/2210.05456)).

## 3. Le mécanisme que l'œil reconnaît : les rides ne sont pas uniformes

C'est le point le plus visible, et le plus absent de notre rendu. Les vagues courtes chevauchant
une vague longue subissent trois effets simultanés — convergence des vitesses orbitales, gravité
effective modulée, conservation de l'action — dont la somme est sans ambiguïté :

> les vagues courtes deviennent **plus courtes et plus hautes de préférence sur les crêtes** des
> vagues longues, et **s'allongent et s'aplatissent dans les creux**
> ([JFM 2024, révision de la modulation hydrodynamique](https://arxiv.org/html/2410.12960v1)).

La décomposition de la modulation de pente y est donnée : nombre d'onde 5/8, convergence d'action
1/4, gravité effective 1/8. Pour une cambrure de vague longue `ε_L` = 0,1, la pente des rides est
modulée de **≈ 20 %** ; à `ε_L` = 0,4 elle double. Une mer réelle a donc des **crêtes rugueuses et
des creux lisses** — et, parce que les rides mettent un temps à répondre, leur maximum n'est pas
exactement sur la crête mais **en avant** d'elle. C'est ce décalage qui produit l'asymétrie des
pentes mesurée au glitter ; un modèle à deux échelles, rides amorties chevauchant les vagues
longues, est la lecture classique de `c₀₃` ([révision IASI](https://arxiv.org/html/2210.05456)).

## 4. Ce que notre mer fait aujourd'hui — mesuré

Instrument : `code/water-core/examples/statistiques_surface.rs` (S260, étendu en S303), 10⁶ points
tirés dans l'espace et le temps, élévation et pentes analytiques en f64, sur la réalisation de la
scène `--houle` (mer de vent `Hs` 1,5 m / `Tp` 6 s et houle `Hs` 2 m / `Tp` 12 s, 64 composantes,
queue de 64 lignes). Vent équivalent de la recette : 7,95 m/s.

| modèle | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `c₂₂` | `c₀₄` | **`Sk`** |
|---|---:|---:|---:|---:|---:|---:|---:|
| **observations** (Cox–Munk à 7,95 m/s ; `Sk` = 3k̄σ) | 0,0437 | −0,058 | **−0,222** | 0,40 | 0,12 | 0,23 | **0,156** |
| `--houle` (ce que la revue R11 a vu) | 0,0198 | −0,001 | 0,002 | −0,026 | −0,007 | −0,022 | **−0,0001** |
| `--vagues` (queue f⁻⁴ + CWM) | 0,0495 | −0,000 | 0,001 | 0,208 | 0,072 | 0,208 | 0,0029 |
| `--vagues --modulation` (M = 2) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,137 | 0,408 | 0,0030 |

**Trois constats.**

1. **La scène montrée en R11 était la plus pauvre des trois.** `--houle` seule a des pentes
   quasi gaussiennes (`c₄₀` = −0,026 contre 0,40 observé) et une rugosité deux fois trop faible.
   Les deux variantes construites en S260 et S261 — queue d'équilibre, vagues pointues, modulation
   — n'étaient pas activées. C'est une faute de protocole de ma part, pas un défaut du modèle.
2. **Même la meilleure variante n'a aucune asymétrie** : `Sk` = 0,003 contre 0,156 attendus, et
   `c₀₃` = 0,001 contre −0,222. Les crêtes et les creux y sont aussi arrondis les uns que les
   autres — exactement ce que l'utilisateur décrit par « concave plutôt que convexe ».
3. **Le second ordre par composante ne suffit pas** : mesuré, il donne `Sk` = 0,0022. Les termes
   **croisés** entre composantes portent l'essentiel de l'asymétrie ; une somme de corrections
   indépendantes ne peut pas la produire. (ADR-157 l'avait écarté sur les pentes ; ici c'est
   quantifié sur l'élévation.)

## 5. Deux modèles éprouvés, mesurés sur la même réalisation

**(a) Second ordre en bande étroite** (Tayfun 1980, [référence](https://www.researchgate.net/publication/23614433_Narrow-band_nonlinear_sea_waves)) :
`η = η₁ + ½·k̄·(η₁² − η̂₁²)`, où `η̂₁` est la quadrature (transformée de Hilbert) de la bande. La
forme contient **tous les termes croisés** et coûte une seule somme supplémentaire — le rendu
somme déjà les composantes, il lui suffit d'accumuler aussi leur quadrature.

**(b) Modulation retardée** : la modulation de la queue d'ADR-158, mais déphasée de `δ` par
rapport à la crête — les rides maximales **en avant** de la crête, comme le décrit §3.

| modèle | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `c₂₂` | `c₀₄` | `Sk` |
|---|---:|---:|---:|---:|---:|---:|---:|
| observations | 0,0437 | −0,058 | −0,222 | 0,40 | 0,12 | 0,23 | 0,156 |
| Tayfun par système, seul | 0,0199 | −0,001 | 0,001 | −0,022 | −0,003 | 0,002 | **0,063** |
| Tayfun bande entière, seul | 0,0199 | −0,001 | 0,002 | −0,022 | −0,005 | −0,010 | **0,150** |
| retenu : queue f⁻⁴ + M2 + CWM + Tayfun par système + retard −0,20 tour | 0,0497 | **−0,057** | **−0,155** | 0,340 | 0,120 | 0,379 | 0,066 |
| même chose, retard −0,25 tour | 0,0497 | −0,060 | −0,163 | 0,310 | 0,108 | 0,332 | 0,066 |

**Ce que cela règle** : l'asymétrie des pentes passe de 0,001 à −0,155, soit **70 %** de la valeur
observée, et `c₂₁` tombe sur la valeur de Cox–Munk (−0,057 contre −0,058) **sans avoir été visé**.
Les aplatissements restent dans les barres de Cox–Munk et rejoignent la révision IASI
(`c₄₀` 0,34 contre 0,26–0,32 à 5–10 m/s). L'asymétrie verticale passe de 0,003 à 0,063 — vingt
fois plus, mais **deux fois moins** que la bande étroite.

**Le seul paramètre libre est le retard** `δ`, calé sur `c₀₃`, et il est dit comme tel. Son signe,
lui, n'est pas libre : le balayage montre que seul un retard **négatif** — rides en avant de la
crête — donne une asymétrie du signe observé. C'est le mécanisme de §3 qui décide, la mesure
confirme.

## 6. Ce qui reste ouvert, nommé

- **Le noyau exact du second ordre** (Sharma & Dean, Longuet-Higgins 1963) pour un spectre à deux
  systèmes : la valeur vraie de `Sk` est entre 0,063 (par système) et 0,150 (bande entière). Tant
  qu'il n'est pas écrit, aucune des deux variantes n'est « la bonne » ; la borne basse est la plus
  prudente.
- **Les capillaires parasites** sur la face avant des vagues courtes : mécanisme documenté (§1),
  absent du modèle et de la queue. Ils portent une partie du grain visible d'une mer réelle.
- **L'asymétrie horizontale `As`** (vague penchée) : nulle chez nous, faible en eau profonde,
  forte au rivage — à reprendre avec la bathymétrie.
- **Écume, micro-déferlement, rafales** : hors modèle, déjà notés par ADR-157.
- **La `mss` reste 14 % au-dessus de Cox–Munk** — écart d'avant ce lot, inchangé par lui.

## 7. Reproduction

```powershell
cargo run --manifest-path code/Cargo.toml --release --offline --example statistiques_surface
```

L'instrument liste tous les candidats, dont ceux de S260 ; les lignes `S303_…` portent les modèles
de ce lot. Aucun rendu n'est modifié par cette mesure.
