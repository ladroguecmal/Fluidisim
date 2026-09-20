# ADR-176 — Les asymétries de la surface rendue : second ordre en bande étroite et modulation retardée

- **Statut : actée**, S303, 2026-09-20 ; autonomie technique S71. Répond au **verdict R11**
  ([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md#verdict-r11--reçu-s303-2026-09-20)) :
  « la topologie est à revoir […] les micro vaguelettes ou pics doivent être convexes plutôt que
  concaves ».
- **Complète** [ADR-155](ADR-155-queue-spectrale-en-pentes-par-pixel.md) (queue en pentes),
  [ADR-156](ADR-156-mer-multimodale-et-etalement.md) (mer multimodale),
  [ADR-157](ADR-157-queue-d-equilibre-et-vagues-pointues.md) (queue d'équilibre et CWM) et
  [ADR-158](ADR-158-rugosite-ajustee-a-cox-munk.md) (modulation de la queue). Reprend le point
  qu'ADR-157 avait explicitement laissé hors de portée : **l'asymétrie de l'élévation**, dont le
  déclencheur écrit dans la file active était « si l'asymétrie est jugée visible ». Elle vient de
  l'être.
- **Mesures qui la fondent** : [ANATOMIE-SURFACE-S303](../validation/ANATOMIE-SURFACE-S303.md).

## 1. Constat

Mesuré sur la réalisation même du rendu, 10⁶ points, élévation et pentes analytiques en f64 :

| | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `Sk` |
|---|---:|---:|---:|---:|---:|
| observations (Cox–Munk à 7,95 m/s ; `3k̄σ`) | 0,0437 | −0,058 | −0,222 | 0,40 | 0,156 |
| meilleure variante construite (`--vagues --modulation`) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,0030 |

La rugosité et l'aplatissement des pentes sont atteints depuis S260–S261 ; **aucune asymétrie ne
l'est**. Deux faits mesurés commandent la décision :

1. **Les termes croisés entre composantes portent l'asymétrie verticale.** Le second ordre
   appliqué composante par composante donne `Sk` = 0,0022 — il est quartique en amplitude, donc
   négligeable pour une mer à 64 composantes. Une correction qui ne couple pas les composantes ne
   peut pas produire de crêtes pointues.
2. **L'asymétrie des pentes vient du retard des rides sur la crête.** Les vagues courtes se
   raccourcissent et se redressent sur les crêtes des vagues longues et s'aplatissent dans les
   creux, avec un temps de réponse : leur maximum est **en avant** de la crête. Le balayage montre
   que seul un retard négatif donne le signe observé de `c₀₃`.

## 2. Décision

**D1 — Second ordre en bande étroite (Tayfun 1980), par système.** Le rendu ajoute à l'élévation
de la bande `η₂ = ½·k̄_s·(η_s² − η̂_s²)` pour chaque système du spectre (mer de vent, houle), où
`η̂` est la quadrature de ce système et `k̄_s` son nombre d'onde moyen pondéré par l'énergie. La
forme contient **tous les termes croisés** du système et coûte une seule somme supplémentaire :
le nuanceur somme déjà les composantes, il accumule aussi leur quadrature. Les pentes suivent par
dérivation du produit.

**Pourquoi par système et non sur la bande entière.** La formule suppose une bande étroite ; chaque
système l'est, leur réunion ne l'est pas. Par système on mesure `Sk` = 0,063, bande entière 0,150,
et la valeur vraie — qui demande le noyau exact du second ordre pour deux systèmes — est entre les
deux. **On prend la borne prudente** et on nomme l'écart plutôt que de le combler par un choix.

**D2 — Modulation retardée de la queue.** La modulation d'ADR-158 (`énergie × max(0, 1 + M·ε)`,
`M = 2`) est déphasée : `ε_δ = cos δ·ε + sin δ·ε̂`, avec `ε̂` la quadrature de la déformation.
`δ = −0,20 tour`. **C'est le seul paramètre libre de cette décision**, calé sur `c₀₃` de Cox–Munk,
et il est dit comme tel ; son **signe**, lui, est imposé par le mécanisme et confirmé par le
balayage. À `δ = 0` le rendu est celui d'ADR-158, au bit.

**D3 — Où cela vit : le rendu, pas le modèle de B.** Comme CWM (ADR-157 §4) et la queue
(ADR-155), ces deux corrections sont **cosmétiques** : `EvalWater` et les requêtes de jeu restent
eulériennes et linéaires, et l'écart entre la surface rendue et la surface interrogée est mesuré et
publié. La couche B du cœur, ses empreintes, ses réceptions et le couplage de δ **ne changent pas**.
La conséquence — une image qui n'est pas ce que le jeu interroge — est celle d'A288, déjà ouverte,
et ce lot l'aggrave d'un terme de l'ordre de `½k̄σ²` (≈ 6 cm ici) contre 0,365 m pour CWM.

**D4 — Portée des variantes.** Les deux corrections s'activent avec `--vagues --modulation`. Le
rendu par défaut et `--houle` restent **identiques au bit** ; aucune scène de réception antérieure
ne bouge sans être rejouée et expliquée.

**D5 — Ce que la prochaine revue montre.** Toute demande de revue de mer déclare désormais **quelles
options de topologie sont actives** — R11 a jugé la mer la plus pauvre du dépôt parce que la demande
ne le disait pas.

## 3. Réception, écrite avant construction

1. **Statistiques** sur la même réalisation, 10⁶ points : `Sk` ≥ 0,06 (contre 0,003), `c₀₃` entre
   −0,13 et −0,18, `c₂₁` dans ±0,02 de −0,058, `c₄₀` entre 0,26 et 0,45, `mss` inchangée à 2 %.
2. **GPU contre CPU** : élévation et pente du nuanceur contre la référence f64, aux sondes, aux
   tolérances de S260 (3 mm en déplacement, 5·10⁻⁴ en pente).
3. **Scènes antérieures au bit** : défaut, `--houle`, `--vagues` sans modulation, et toutes les
   vérifications existantes.
4. **Écart au jeu** publié : écart vertical maximal entre surface rendue et requête linéaire, avant
   et après ce lot.
5. **Coût** : GPU eau à 1280×720, deux poses, secteur relevé aux deux bornes (ADR-131, A270).
6. **Revue R12** : mêmes poses qu'R11, options déclarées, avec et sans les asymétries.

## 4. Ce que cette décision ne fait pas

- Elle **ne corrige pas** le noyau exact du second ordre pour un spectre à deux systèmes : borne
  prudente retenue, écart nommé.
- Elle n'ajoute **ni capillaires parasites**, ni écume, ni micro-déferlement — mécanismes
  documentés, absents du modèle.
- Elle ne touche **ni à `mss`** (14 % au-dessus de Cox–Munk avant comme après), **ni à l'asymétrie
  horizontale `As`** (nulle en eau profonde chez nous), **ni au couplage de la queue à δ** (la
  queue reste un habillage non couplé, comme R10 l'avait déjà relevé).
- Elle ne change **aucune grandeur de jeu** : I-04 et I-15 tiennent, δ et B gardent leurs rôles.

Invariants relus : I-01, I-04, I-13, I-14, I-15 ; aucun amendé.
