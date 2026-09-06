# ADR-032 — C08 n'est pas exécutable tel qu'énoncé : un ordre est une propriété du couple (solveur, cas)

- **Statut** : proposée
- **Session** : S24
- **Tranche** : `CAS-CANONIQUES` §C08 — protocole de mesure de la convergence
- **Corrige** : rien n'est réécrit. `CAS-CANONIQUES` §C08 reçoit une note corrective datée, et
  SPEC-003 §5.1 un renvoi sur le coût de l'oracle.
- **Produit** : le contrôle de convergence du mode `physics`, et le montage régulier `c08_regulier`
- **Clôt** : action **S23-1** — par la négative

---

## 1. L'énoncé, et les trois choses qu'il ne dit pas

> **C08.** *Montage : un cas de C02, C04 ou C09, exécuté à `dx`, `dx/2`, `dx/4`. Référence :
> solution analytique si elle existe, sinon l'oracle lent. Mesure : `p` par extrapolation de
> Richardson. Assertion : `p > 0,8`.*

Exécuté, il ne conclut pas. Trois raisons, et chacune est mesurée ci-dessous.

1. **Il ne dit pas sur quelle *grandeur*** du cas porte l'ordre. C04 en produit quatre.
2. **Il ne dit pas que le cas doit être *régulier***. Aucun des trois qu'il désigne ne l'est.
3. **Il ne dit pas que trois grilles ne suffisent pas** à établir qu'on est en régime asymptotique
   — et que sans cela, `p` est un nombre sans signification.

## 2. La mesure : quatre grandeurs de C04, et aucune conclusion

`t = 2 s`, grilles de 200 à 3200 cellules, référence analytique (Ritter).

| Grandeur | erreurs, 200 → 3200 | ordres par triplet | dernier `p` |
|---|---|---|---|
| erreur L1 relative (globale) | 2,24e-2 → 2,86e-3 | +0,595 · +0,686 · +0,732 | 0,73 |
| `h(0)` (ponctuelle) | 3,10e-2 → 3,39e-3 | +0,698 · +0,747 · +0,787 | 0,79 |
| `u(0)` (ponctuelle) | 1,69e-1 → 1,81e-2 | +0,695 · +0,754 · +0,795 | 0,80 |
| **front, ε = 1 mm (locale)** | 2,654 → 1,130 m | **−0,504 · −0,059 · +0,237** | 0,24 |

**Aucune de ces suites n'est stabilisée.** Les ordres montent régulièrement : le dernier n'est donc
pas la limite, et le publier serait publier un artefact du choix des grilles. Le front, lui, donne
des ordres **négatifs** sur les grilles grossières — la signature du régime pré-asymptotique, où
les différences successives grandissent avant de décroître.

> **C08 sur C04 n'est ni passé ni échoué : il n'est pas exécutable.** Le rapport le dit en ces
> termes, avec le décompte des grandeurs sans verdict — un rapport sans échec ne doit jamais se
> lire comme une validation.

**Ceci clôt l'action S23-1 par la négative.** L'exposant du front n'est pas stabilisable sur ces
grilles. Le chiffre de ×3·10⁵ d'[ADR-031](ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) §2
reste donc une extrapolation, comme il le disait — et sa prudence était plus justifiée qu'il n'y
paraissait.

## 3. Décision : un ordre de convergence se mesure sur un cas régulier

Même solveur, montage régulier — bosse gaussienne de 1 cm sur 1 m d'eau, fond plat, régime linéaire
(`a/h₀ = 1 %`), ni front ni séchage, référence par oracle. Ordres observés : **+0,819 · +0,978**.

**Le solveur est bien d'ordre ≈ 1.** L'ordre réduit mesuré sur C04 vient donc de la **solution**, pas
du schéma : un schéma d'ordre 1 sur une solution à dérivée discontinue converge à un ordre réduit,
et c'est un résultat classique.

> **Décision. L'ordre de convergence est une propriété du couple (solveur, cas), jamais du solveur
> seul. C08 se mesure sur un cas *régulier*, et son assertion `p > 0,8` ne s'applique qu'à celui-là.**
>
> Sur un cas singulier — C04, C09 — la mesure reste utile mais change de nature : elle compare des
> candidats **entre eux** sur le même cas, et ne se compare à aucun seuil absolu.

Conséquence pour le protocole de B3 : les deux critères d'entrée d'ADR-030 et ADR-031 se mesurent
sur C01 et C04, qui sont singuliers ; **la convergence, elle, se mesure ailleurs**, sur le montage
régulier. Trois mesures, trois montages, et aucun ne remplace les autres.

### 3.1 Ce que l'énoncé doit gagner

- **nommer la grandeur** sur laquelle `p` est mesuré ;
- **exiger la régularité** du cas pour que l'assertion `p > 0,8` ait un sens ;
- **exiger cinq grilles au moins**, et rapporter la suite des ordres, pas un ordre ;
- **rapporter « non concluant »** comme un état distinct de « passé » et de « échoué ».

## 4. Le prix caché de l'oracle

C'est le résultat que la session n'attendait pas, et il porte au-delà de C08.

**L'oracle n'est pas la solution : il porte sa propre erreur.** Quand l'erreur d'une grille testée
s'en approche, les deux se soustraient partiellement et l'ordre observé s'envole. Mesuré, en
affinant l'oracle :

| Oracle | ordres observés |
|---|---|
| `nx = 12 800` | +0,889 · +0,945 · +1,087 |
| `nx = 25 600` | +0,890 · +1,058 · **+1,559** |
| `nx = 51 200`, filtre ×10 | +0,819 · +0,978 · +1,313 |
| `nx = 51 200`, filtre ×30 | +0,819 — *trois grilles saines* |

**Un ordre observé de 1,56 pour un schéma d'ordre 1 est impossible**, et c'est ce qui rend la
contamination reconnaissable. Noter que **affiner l'oracle a dégradé le résultat** : les grilles
fines, elles, ne bougeaient pas.

> **Avec un oracle, le triplet le plus fin est le *moins* fiable.** Avec une solution analytique,
> c'est le plus fiable. **La règle s'inverse selon la nature de la référence**, et rien dans
> SPEC-003 §5.1 ne le laissait prévoir.

**Le filtre.** Une grille est retenue si son erreur vaut au moins **trente fois** l'erreur estimée
de l'oracle, elle-même obtenue par la loi mesurée sur les grilles saines. Seuil dérivé, non
conventionnel : à un rapport de 30, la contamination de l'ordre est majorée par
`log₂(1 + 2/30) ≈ 0,09`, moins que la tolérance d'asymptoticité.

### 4.1 Les deux exigences se contredisent, et cela chiffre le banc

C08 a besoin de grilles **assez fines** pour être en régime asymptotique et **assez grossières**
pour ne pas être contaminées. Pour cinq grilles saines :

```
nx_max ≥ 16 · nx_min        (cinq grilles par doublement)
nx_oracle ≥ 30 · nx_max     (filtre de contamination)
⇒ nx_oracle ≥ 480 · nx_min
```

**L'oracle doit être quatre cent quatre-vingts fois plus fin que la grille la plus grossière.** En
1D son coût va comme `nx²` (cellules × pas de temps), en 2D comme `nx³` : l'oracle **est** le banc,
et tout le reste est du bruit budgétaire à côté.

C'est une contrainte à porter au plan de benchmark, où « l'oracle lent » est cité comme une
référence disponible et non comme le poste dominant.

## 5. Deux défauts de l'outil, trouvés par les données et non par relecture

1. **Le critère d'asymptoticité comparait les deux derniers ordres.** Il déclarait stabilisée la
   suite 0,595 → 0,686 → 0,732, dont les écarts sont petits **mais tous de même signe**. Un critère
   d'écart local ne distingue pas « a convergé » de « progresse lentement ». Remplacé par un critère
   dérivé : la progression est éteinte si les écarts changent de signe, ou décroissent d'un facteur
   ≥ 4 — auquel cas la somme des écarts restants est majorée par `|d₁|/3`.
2. **Les ordres négatifs étaient classés « indéterminé ».** Or des différences successives qui
   *grandissent* sont exactement la signature du pré-asymptotique. Les masquer retirait la seule
   chose que le contrôle devait constater.

Et un troisième, d'une autre nature : **l'échec d'allocation de l'oracle était absorbé** par un
`Err(_) => continue`. Le rapport affichait « 0 grille retenue sur 0 » — un résultat vide qui a l'air
d'un résultat. L'erreur remonte désormais au `Sink`, et le cas se déclare *indisponible* plutôt que
vide.

## 6. Ce qui reste ouvert

1. **Le montage régulier n'a que trois grilles saines** avec l'oracle actuel, donc un seul ordre et
   aucun verdict d'asymptoticité. Le compléter demande un oracle à `nx ≈ 100 000`, dont le coût est
   à mesurer avant d'être engagé. Angle mort **A116**.
2. **Le seuil `ε` du front reste conventionnel** (action S23-2, reportée). S24 a montré que ce
   n'était pas le point bloquant — l'ordre est réduit sur *toutes* les grandeurs, pas seulement sur
   le front — mais la dette demeure, et A110 avec elle.
3. **`ordre_final()` prend le triplet le plus fin.** C'est juste avec une solution analytique, faux
   avec un oracle (§4). La fonction devrait recevoir la nature de la référence ; elle ne l'a pas
   encore, et le filtre de contamination masque le problème sans le résoudre.
4. **Aucun cas régulier n'existe dans `CAS-CANONIQUES`.** Le montage de S24 est écrit dans le code
   et non dans le corpus de validation. Il devrait y entrer sous son propre numéro — proposition
   **C22**, « convergence sur solution régulière ».
