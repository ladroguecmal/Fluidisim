
# ADR-094 — D'où vient la limite de pente, et ce que le budget additionne

- **Statut : actée**, S139, 2026-09-10, autonomie technique S71.
- **Traite** [A205](../registres/ANGLES-MORTS.md) et la moitié « limite de pente » du renvoi
  corrigé par [ADR-093](ADR-093-ou-se-calibre-la-source-d-impact.md).
- **Prolonge :** ADR-058 (borne de pente), ADR-062 (composition), ADR-080 (budget de pente),
  ADR-081 (séparation limite physique / limite numérique).
- **Mesure :** [PENTE-REELLE-S139](../validation/PENTE-REELLE-S139.md).

## Problème

`max_slope` décide de l'admissibilité de **tout** champ : c'est le refus `Steepness` de
`RadialImpact::new`, et c'est aussi le plafond du budget de composition d'ADR-080. Il vaut `0,1`
dans toutes les fixtures depuis S77, sans provenance, et ADR-058 §21 comme ADR-062 §50 le
renvoient « à calibrer B2 » — un banc qui ne mesure pas cela (ADR-093).

La question paraissait être « quelle valeur mesurer ». **Elle ne l'était pas**, et c'est le
résultat de cette décision. Deux questions se cachaient sous une seule :

1. **Quelle est la limite physique ?** Elle est déjà écrite dans le corpus : SPEC-001 §4 donne la
   cambrure limite de Stokes, `H/λ ≈ 1/7`. Rien à mesurer.
2. **Sur quelle grandeur la comparer ?** `max_slope` est comparé à `slope_bound`, qui est une
   **borne L1** — `Σ|a_k·k·dk|·k`, obtenue en majorant `|J1| ≤ 1` —, pas la pente du champ.
   ADR-058 et ADR-062 le disaient déjà, sans dire de combien.

Le nombre qui manquait n'était donc pas une propriété de l'eau : c'est **le rapport entre la
borne et la pente réelle**, et il se mesure dans le modèle, comme `α` en S136.

## Ce qui a été mesuré

**`ρ = slope_bound / max|∇η| = 1,7950713`**, et c'est une **constante du modèle**, pas un
paramètre : la forme spectrale d'ADR-060 est fixe et le champ est homothétique en `λ` (S136),
donc `ρ` ne dépend de rien de ce que l'appelant fournit. Vérifié invariant à sept chiffres sur
`λ` de 0,5 à 32 m, `E` de 1e-4 à 100 J, `N` de 64 à 256, rayon de domaine de 0,5 à 8 λ ; et
retrouvé à 1,795071271 par une quadrature f64 indépendante du code du dépôt.

Le maximum est atteint en **`r = 0,2062 λ`**, à **`t = birth`** — là où les phases temporelles
valent toutes 1. Aucun instant ultérieur ne le dépasse : sur trois longueurs d'onde et mille
instants, le maximum sur `t > 0` vaut de 0,99966 à 0,999995 fois celui de `t = 0`.

`ρ` ne peut pas descendre sous 1,7185 = 1/0,5819, l'inverse du maximum de `J1` : la valeur
mesurée en est proche, ce qui dit que la majoration `|J1| ≤ 1` est presque tout le conservatisme.

## Décision

**1. `max_slope` est la pente de déferlement, et elle se dérive.** Provenance : SPEC-001 §4,
`H/λ ≈ 1/7`, pente d'une sinusoïde de cette cambrure `πH/λ`.

| lecture | valeur |
|---|---|
| pente linéaire à la cambrure limite, `πH/λ` avec `H/λ = 1/7` | **0,4488** |
| pente réelle de la crête à 120° de la vague limite de Stokes, `tan 30°` | 0,5774 |

**Valeur retenue : 0,4488**, la plus conservatrice, et la seule cohérente avec la conversion
`steepness · π` que le code emploie déjà (`composition.rs`). L'écart entre les deux lectures est
de 29 % : il est borné et dit, comme `α ∈ [3,35 ; 6,11]` l'a été en S136. **`max_slope` n'est plus
« à calibrer » : plus aucun banc ne le doit.**

**2. La grandeur comparée doit être la pente réelle, pas la borne L1.** `RadialImpact::slope_max()`
est publiée : `slope_bound / ρ`, exacte à `t = birth`, majorante ensuite. Elle ne change aucun
comportement — la construction compare toujours la borne L1 — mais elle rend disponible ce que la
limite physique borne réellement.

**3. Le budget de pente d'ADR-080 est hétérogène, et c'est ce qui interdisait la dérivation.**
`composition.rs` calcule `bound = steepness_B·π + Σ slope_bound (+ slope_envelope)` et le compare
à `max_slope`. Ces termes ne sont pas la même grandeur :

| terme | nature | facteur au-dessus de la pente réelle |
|---|---|---|
| `steepness_B · π` | pente **exacte** du fond | 1 |
| `slope_bound` d'un impact radial | borne L1 | **1,7950713**, constante |
| `slope_envelope` d'une pression | borne L1 | non mesuré ; entre 1 et 2 pour une seule case, davantage au-delà |

**Conséquence : aucun seuil unique ne peut être physiquement juste pour les trois.** Écrire
`max_slope = 0,4488·ρ = 0,806` pour rendre justice aux impacts autoriserait au fond B une cambrure
de `0,806/π = 0,256`, soit **1,8 fois la limite de déferlement**. Le seuil ne devient dérivable
qu'une fois le budget homogène — c'est-à-dire quand chaque couche publie sa **pente réelle**.

## Ce que cette décision ne fait pas

**Elle ne migre pas le refus `Steepness`.** Comparer `slope_max()` au lieu de `slope_bound`
déplace la frontière d'admission de tous les champs, change les bits publiés et donc les hachages
de campagne. C'est un lot à part entière, avec ses témoins : **action S139-1**.

**Elle ne mesure pas le facteur de la pression** (`spectral_pressure.rs:346`), qui n'est
probablement pas une constante — il dépend de la répartition des cases et des phases, quand `ρ`
ne dépend de rien. C'est **A206**.

**Elle ne prétend pas que le critère de Stokes s'applique tel quel à un paquet transitoire.**
La cambrure limite est établie pour une onde progressive monochromatique et permanente en eau
profonde ; le champ d'impact est un paquet dispersif dont la crête vit une fraction de période.
Le critère est employé comme **majorant géométrique** — au-delà, la surface n'est plus une
fonction de la position — et non comme prédiction de déferlement. C'est **A207**.

**Elle ne change pas `0,1` dans les fixtures existantes.** Tant que le budget reste hétérogène,
`0,1` reste un choix de test défendable, et le changer sans migrer le budget déplacerait des
frontières sans les rendre justes.

## Ce que cela coûte aujourd'hui, en chiffres

À `max_slope = 0,1` sur la borne L1 :

- un impact radial seul est admis jusqu'à une **pente réelle de 0,0557** — **12,4 %** de la limite
  physique ;
- le fond B seul est admis jusqu'à `H/λ = 0,0318`, soit **1/31,4** au lieu de 1/7 ;
- l'énergie admissible d'un impact est divisée par **64,9** par rapport au seuil dérivé
  (`E ∝ pente²`).

Ces trois nombres ne sont pas des défauts à corriger d'urgence : ils disent ce que le seuil actuel
achète en marge, et ils permettent enfin de le dire.

## Réception

270 tests, cinq ignorés — un de plus qu'en S138.
`slope_max_is_attained_and_never_exceeded_s139` vérifie les deux moitiés de l'annonce : la pente
échantillonnée **atteint** `slope_max()` à mieux que 1e-3 à l'instant initial, et **ne la dépasse**
ni ailleurs sur le disque ni à vingt instants ultérieurs. La seconde moitié est une propriété de
sûreté : si un instant la dépassait, la grandeur ne pourrait pas servir de borne.

Aucun comportement modifié, aucun hachage de campagne touché : `slope_max()` est additive.

## Note corrective du 2026-09-10 (S140)

La décision 2 — « la grandeur comparée doit être la pente réelle » — **n'est pas applicable à la
pression**, et A206 l'a montré dès la session suivante. La pente maximale d'un champ de pression
ne se calcule pas : elle se cherche sur l'emprise, et une recherche qui manque le maximum produit
un majorant faux. Le facteur de conservatisme de `slope_envelope` n'est d'ailleurs **pas une
constante** — il vaut 3,03 sur un spectre gaussien réaliste et croît sans borne quand l'emprise se
resserre sur un zéro du champ (16,7 mesuré).

La formulation juste est celle d'[ADR-095](ADR-095-ce-que-la-pression-peut-annoncer-de-sa-pente.md) :
**la même grandeur majorée par tous les termes, le meilleur majorant exact de chacun, la marge
résiduelle mesurée.** Le reste de cette décision — `max_slope = 0,4488` avec provenance,
`ρ = 1,7950713`, l'hétérogénéité du budget comme cause de l'absence de provenance — est inchangé.
