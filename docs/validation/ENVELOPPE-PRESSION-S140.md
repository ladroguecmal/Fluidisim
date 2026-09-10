# S140 — Le conservatisme de l'enveloppe de pression, et ce qu'il en reste après resserrement

2026-09-10. [ADR-095](../adr/ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) actée.
A206 traitée. Sonde : `code/water-core/examples/enveloppe_pression.rs`.

## 1. Ce que la mesure devait décider

ADR-094 a laissé une question précise : le facteur de conservatisme de `slope_envelope` est-il,
comme `ρ = 1,7950713` pour l'impact radial, une **constante** du modèle ? De la réponse dépendait
la voie de S139-1 — si oui, chaque couche divise sa borne par son facteur et le budget devient
homogène ; si non, cette voie n'existe pas.

L'enveloppe additionne `(|kx_w|+|ky_w|)·(|Re η|+|Im η|)` par case (`spectral_pressure.rs:346`).
La pente réelle vaut `|Σ k_w·(η_re sin φ + η_im cos φ)|`, `φ` dépendant du point.

## 2. Une case : la factorisation, et le témoin qui valide la sonde

Pour une case unique, le rapport se factorise **exactement** en un facteur de forme, calculable,
et un facteur de phase, déduit. Si la sonde mesurait autre chose que ce produit, elle serait
fausse — et le second ne peut pas dépasser √2 = 1,41421.

| direction de `k` | forme `(\|kx\|+\|ky\|)/\|k\|` | facteur mesuré | phase déduite |
|---|---|---|---|
| 0° | 1,000000 | 1,347381 | 1,347381 |
| 22° | 1,306563 | 1,662719 | 1,272590 |
| 45° | 1,414214 | 1,529837 | 1,081758 |
| 68° | 1,306563 | 1,846538 | **1,413279** |
| 90° | 1,000000 | 1,000000 | 1,000000 |

Aucune phase déduite ne dépasse √2, et la plus haute en est à 6e-4. Le facteur de phase seul,
balayé par l'instant à direction fixée, va de 1,000000 à 1,405819.

## 3. Où le maximum se trouve, et pourquoi rétrécir l'emprise ne suffit pas à le manquer

Profil de `|∇η|` le long de `x`, une case à 45°, enveloppe `4,217611e-3` :

| x (m) | 0,000 | 0,667 | 1,333 | 2,000 | 2,667 | 3,333 | 4,000 |
|---|---|---|---|---|---|---|---|
| `\|∇η\|` ×10⁻³ | 2,7468 | 1,8687 | **0,0119** | 1,8511 | 2,7447 | 2,2008 | 0,5044 |

Le maximum sur ±20 m vaut `2,756902e-3`, et **le maximum sur ±0,04 m vaut exactement le même
nombre**. Ce n'est pas une erreur de sonde : la réponse a `|η_re|/|η_im| = 0,0856`, donc son
maximum est atteint à 0,0136 tour de l'origine — et l'origine est dans toute emprise centrée.
Une emprise centrée sur la source rencontre donc son maximum, si petite soit-elle.

| demi-emprise | centrée (0 ; 0) | centrée (10 ; 7) | centrée (37 ; 23) |
|---|---|---|---|
| 5 λ à 0,1 λ | 1,5298 | 1,5298 | 1,5298 |
| 0,05 λ | 1,5298 | 1,5298 | 1,6069 |
| 0,02 λ | 1,5298 | 1,5298 | 1,8261 |
| 0,01 λ | 1,5298 | 1,5305 | 1,9463 |

## 4. Le facteur n'est borné par rien

Le profil du §3 passe par un zéro vers `x = 1,33 m`. Une emprise étroite posée dessus contient un
champ presque plat, quand l'enveloppe, elle, ne dépend pas de l'emprise :

| centre `x` | 1,20 | 1,30 | **1,34** | 1,35 | 1,40 |
|---|---|---|---|---|---|
| facteur, demi-emprise 0,05 λ | 2,72 | 3,28 | 3,54 | 3,46 | 3,11 |
| facteur, demi-emprise 0,01 λ | 6,40 | 11,78 | **16,66** | 14,87 | 9,69 |

Et il continue de croître à mesure que l'emprise se resserre. **Ce n'est pas une constante du
modèle** : c'est une propriété de l'emprise que l'hôte choisit. La réponse à la question du §1
est donc non, et la voie que S139 recommandait pour S139-1 n'existe pas telle quelle.

## 5. Sur un champ réaliste

Spectre gaussien cuit, `σ = 1`, `cutoff = 6`, emprise ±12 m, même montage que `receive_power` :

| résolution | cases | enveloppe | resserrée | pente réelle | facteur | après resserrement |
|---|---|---|---|---|---|---|
| 16 × 16 | 128 | 3,845232e-3 | 2,328236e-3 | 1,256375e-3 | 3,0606 | 1,8531 |
| 32 × 32 | 512 | 3,811911e-3 | 2,327360e-3 | 1,256428e-3 | 3,0339 | 1,8524 |
| 64 × 64 | 2048 | 3,803833e-3 | 2,327199e-3 | 1,256431e-3 | 3,0275 | 1,8522 |
| 112 × 80 | 4480 | 3,802787e-3 | 2,327182e-3 | 1,256435e-3 | **3,0266** | **1,8522** |

Le facteur est **stable en résolution** — il caractérise le spectre publié, pas la discrétisation.
Un champ de pression réaliste consomme donc trois fois le budget que sa pente réelle justifierait.

## 6. Ce que la borne resserrée récupère, et ce qu'elle ne récupère pas

`slope_envelope_tight() = Σ |k_w|·|η|`, en normes euclidiennes : majorant tout aussi rigoureux,
même coût, et il retire **exactement** le facteur de forme.

| cas | enveloppe / resserrée | resserrée / pente réelle |
|---|---|---|
| une case, emprise 5 λ | 1,5298 | **1,0000** |
| une case, emprise 0,01 λ centrée | 1,5298 | **1,0000** |
| une case, 0,01 λ sur un zéro | 1,5298 | 10,8916 |
| spectre gaussien 4480 cases | 1,6340 | 1,8522 |

**Elle est exacte — atteinte, pas approchée — quand l'emprise contient le maximum d'une case
unique.** Ce qu'elle ne touche pas est le conservatisme d'alignement, et c'est précisément celui
qui n'a pas de borne.

## 7. Ce qui est décidé, et ce qui ne l'est pas

**Décidé** (ADR-095) : la borne à consommer est la resserrée ; le budget devient homogène par la
**nature** de ses termes — chacun majore la pente réelle et publie le meilleur majorant qu'il sait
calculer exactement — et non par leur exactitude, qui est hors d'atteinte pour la pression.
`max_slope` peut alors être la limite physique 0,4488, et la garantie s'énonce en une phrase :
aucun point ne dépasse la cambrure limite de Stokes.

**Publié** : `Field::slope_envelope_tight()`, sans changer aucun comportement.

**Non fait, et c'est S139-1** : substituer la borne resserrée dans `mixed_water::slope_floor` et
dans le budget de composition, poser `max_slope = 0,4488`, et recevoir le déplacement des refus
avec ses témoins de hachage. Le préalable A206 est levé.

**Ce que S140 corrige de S139** : L220 concluait qu'il fallait faire publier à chaque terme *la
grandeur réelle* qu'il majore. C'est vrai pour le fond B et pour l'impact radial ; c'est
irréalisable pour la pression, dont la pente maximale se **cherche** au lieu de se calculer — et
une recherche qui manque le maximum produit un majorant faux, donc un refus qui n'en est pas un.
La formulation juste est : **la même grandeur majorée, le meilleur majorant exact de chacun, et la
marge résiduelle mesurée plutôt qu'ignorée.**
