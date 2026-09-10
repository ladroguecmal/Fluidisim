
# ADR-095 — Ce que la pression peut annoncer de sa pente

- **Statut : actée**, S140, 2026-09-10, autonomie technique S71.
- **Traite** [A206](../registres/ANGLES-MORTS.md), ouverte la veille par ADR-094.
- **Prolonge :** ADR-080 (budget de pente), [ADR-094](ADR-094-d-ou-vient-la-limite-de-pente.md).
- **Corrige** la voie que S139 recommandait pour S139-1, et qui ne peut pas fonctionner telle
  quelle.
- **Mesure :** [ENVELOPPE-PRESSION-S140](../validation/ENVELOPPE-PRESSION-S140.md).

## Problème

ADR-094 a établi que le budget de pente n'est pas dérivable tant qu'il additionne des grandeurs de
natures différentes, et a laissé un trou nommé : **le facteur de conservatisme de
`slope_envelope` n'était pas mesuré.** La réparation qu'il esquissait — que chaque terme publie
la pente réelle qu'il majore — supposait sans le dire que ce facteur soit, comme `ρ` pour
l'impact radial, une constante connaissable.

**Il ne l'est pas, et la mesure le montre sans ambiguïté.**

## Ce qui a été mesuré

L'enveloppe additionne `(|kx_w|+|ky_w|)·(|Re η|+|Im η|)` par case, quand la pente vaut
`|Σ k_w·(η_re sin φ + η_im cos φ)|`. Le rapport se **décompose en deux facteurs de natures
opposées** :

| facteur | ce qu'il majore | borne | éliminable ? |
|---|---|---|---|
| **forme** — `(\|kx\|+\|ky\|)/\|k\|` puis `(\|Re\|+\|Im\|)/\|η\|` | direction du vecteur d'onde, phase de la réponse | **2** (deux fois √2), atteint | **oui, exactement et sans coût** |
| **alignement** — les phases n'atteignent pas toutes leur maximum au même point de l'emprise | interférence sur l'emprise publiée | **aucune** | non |

Le premier a été vérifié sur une case unique : la factorisation est exacte et le facteur de phase
déduit ne dépasse jamais √2 (mesuré jusqu'à 1,4133 pour 1,41421). Le second **n'est borné par
rien** : une emprise étroite posée sur un zéro du champ donne un facteur de 16,7, et il croît sans
limite à mesure que l'emprise se resserre — l'enveloppe, elle, ne dépend pas de l'emprise.

Sur un champ réaliste — spectre gaussien cuit, `σ = 1`, `cutoff = 6`, de 128 à 4480 cases — le
facteur total vaut **3,027** et ne bouge plus avec la résolution.

## Décision

**1. La borne à consommer est `Σ |k_w|·|η|`, en normes euclidiennes.** `slope_envelope_tight()`
est publiée. C'est un majorant tout aussi rigoureux, de même coût — deux `hypot` au lieu de deux
`abs` — et il retire **exactement** le facteur de forme. Gain mesuré : **1,530** sur une case,
**1,634** sur le spectre gaussien. Il est **exact** — atteint, pas approché — pour une case unique
sur une emprise qui contient son maximum.

**2. Le budget devient homogène par la nature de ses termes, pas par leur exactitude.** C'est le
point où S140 corrige S139 : exiger que chaque couche publie sa pente réelle est irréalisable
pour la pression, dont la pente maximale ne se calcule pas — elle se cherche sur l'emprise, et une
recherche qui rate le maximum produit un majorant faux, c'est-à-dire un refus qui n'en est pas un.

**Chaque terme publie le meilleur majorant de sa pente réelle qu'il sait calculer exactement :**

| terme | ce qu'il publie | marge résiduelle |
|---|---|---|
| fond B | `steepness_B · π`, sa pente exacte | 1 |
| impact radial | `slope_max() = slope_bound/ρ`, exacte à `t = birth` (ADR-094) | 1 |
| pression | `slope_envelope_tight()` | **non bornée** ; 1,85 sur le spectre gaussien mesuré |

Tous majorent alors **la même grandeur** — la pente réelle du champ — et `max_slope` peut être la
limite physique, **0,4488** (SPEC-001 §4). La garantie du budget devient énonçable en une phrase :
*aucun point de la surface ne dépasse la cambrure limite de Stokes*.

**3. La marge de la pression est un coût nommé, pas un défaut à corriger.** Elle rend le budget
plus strict pour la pression que pour les deux autres couches : une pression peut être refusée
alors que son champ tiendrait. C'est le sens sûr du refus, et c'est désormais **mesurable par
l'appelant** — le rapport entre les deux enveloppes publiées dit ce que la forme coûtait ; ce que
l'alignement coûte se mesure sur l'emprise, avec la sonde `enveloppe_pression`.

## Ce que cette décision ne fait pas

**Elle ne migre pas l'admission.** `slope_envelope()` reste la grandeur consommée par
`mixed_water::slope_floor` et par le budget de composition ; substituer la borne resserrée déplace
des refus — dans le sens permissif — et change les hachages de campagne. C'est **S139-1**, dont le
préalable A206 est maintenant levé.

**Elle ne cherche pas à borner le facteur d'alignement.** Il n'est pas bornable : c'est une
propriété de l'emprise que l'hôte choisit, pas du modèle. Une emprise qui se réduit à un point
rend le rapport infini, et rien dans la bibliothèque ne peut l'interdire — ni ne le devrait.

**Elle ne touche pas au fond B.** `steepness_B · π` reste la conversion d'ADR-062 ; elle est
exacte pour le B minimal actuel et cette décision ne la généralise pas à un autre fournisseur de B.

## Réception

271 tests, cinq ignorés — un de plus.
`tight_envelope_brackets_the_real_slope_s140` vérifie l'encadrement aux deux bouts : la borne
resserrée ne passe jamais sous la pente échantillonnée (**sûreté** — un majorant qui passe sous le
champ n'en est plus un), et l'enveloppe L1 ne dépasse jamais son double, c'est-à-dire que le gain
reste dans le facteur 2 que la théorie autorise.

Aucun comportement modifié, aucun hachage touché : la méthode est additive.
