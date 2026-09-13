# D'où vient le pessimisme du majorant de pente d'un sillage — S216, 2026-09-13

Traite la part **statique** d'**A255** : le majorant du sillage pesait 88 % du budget de pente après
ADR-133. Décision : [ADR-134](../adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md).
A255 reste ouverte sur sa part dynamique.

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : aucune technique de rendu. Ce document mesure un **contrat
  d'admission**, pas un coût. Échantillonnage : grille de 0,25 à 0,5 m dans l'emprise puis
  raffinement local à 2 cm autour de l'argmax ; maximum directionnel cherché sur 4 096 directions.
- **Techniques absentes** : sans objet — aucun budget de temps n'est en jeu. Le budget de 2 ms
  (ADR-125) et le budget de pente (ADR-128) restent deux contrats distincts.
- **Domaine de validité** : sillage prescrit S212 et ses variantes — `angular` 64/128/256, `radial`
  32/64/128, `cutoff` 2/3/4, σ 1 et 2 (σ = 4 **refusée à la construction**), 2/8/16 tronçons,
  vitesses 1,5 / 3 / 6 m/s ; emprises de 128 × 104 à 512 × 416 m ; âges 0,5 à 39 s. **Ne dit rien**
  d'un champ de pression qui ne serait pas issu de cette famille gaussienne.
- **Rang de passage** : sans objet (aucune mesure de temps).

## 1. La question a changé avant la première mesure

La suite écrite en S215 demandait de refaire sur le sillage la campagne **mesurée** qui a traité
l'impact. La lecture de `slope_envelope_tight` l'a déplacée :

```rust
for s in self.slots { bound += |k_s| * |eta_s|; }
```

Une somme **scalaire** sur des modes dont les vecteurs d'onde pointent dans des directions
différentes, quand la pente est un **vecteur**. Ce majorant suppose l'alignement en phase **et** en
direction, et la seconde condition ne dépend ni du temps ni du point : elle est fausse par
construction de la recette. ADR-095 avait retiré la direction **au sein** d'un mode et la phase de
sa réponse ; l'étalement **entre** modes n'avait jamais été touché.

Il y a donc deux pessimismes, et les confondre aurait fait calibrer une table là où une inégalité
suffit. Les séparer était la première mesure à faire.

**Contrôle de lecture** : la somme scalaire reconstruite depuis `render_components` reproduit
`slope_envelope()` à **1e-7**. Ce que l'instrument manipule est bien ce que le budget consomme.

## 2. La séparation, et la prédiction contredite

Prédiction écrite avant mesure : part statique ≈ 1,5, expliquant l'essentiel du pessimisme pendant
le forçage.

| âge (s) | phase | scalaire | directionnel | réel | part statique | résidu | total |
|---:|---|---:|---:|---:|---:|---:|---:|
| 0,5 | forçage | 0,038037 | 0,024464 | 0,019385 | **1,5548** | 1,2620 | 1,9621 |
| 2 | forçage | 0,107494 | 0,077957 | 0,064453 | 1,3789 | 1,2095 | 1,6678 |
| 4 | forçage | 0,135072 | 0,107316 | 0,097430 | 1,2586 | **1,1015** | 1,3864 |
| 8 | forçage | 0,144921 | 0,116545 | 0,085966 | 1,2435 | 1,3557 | 1,6858 |
| 16 | forçage | 0,164995 | 0,135757 | 0,086934 | 1,2154 | 1,5616 | 1,8979 |
| 18 | après | 0,156797 | 0,131175 | 0,069110 | 1,1953 | 1,8981 | 2,2688 |
| 24 | après | 0,157044 | 0,131058 | 0,043181 | 1,1983 | 3,0351 | 3,6369 |
| 39 | après | 0,157981 | 0,131885 | 0,033152 | **1,1979** | **3,9782** | 4,7654 |

**Contredite pour moitié.** La part statique vaut bien 1,55 à la naissance, mais elle **retombe à
1,20 et s'y fixe** : elle explique l'essentiel à 4 s (1,26 sur 1,39) et **moins de la moitié** à
16 s (1,22 sur 1,90). Le demi-spectre n'est donc pas étalé uniformément sur un demi-disque — sinon
le rapport vaudrait π/2 ≈ 1,571 ; le sillage de Kelvin concentre son énergie dans un cône, et
c'est cette concentration que le 1,20 mesure.

## 3. Ce que la famille dit de chaque part

**La part statique ne dépend pas de la quadrature** : **1,2586** à quatre décimales pour `angular`
64 / 128 / 256, `radial` 32 / 128 et `cutoff` 2 / 4. C'est une propriété du champ, pas de sa
discrétisation — et c'est ce qui autorise à la **calculer** plutôt qu'à la tabuler. Elle bouge avec
ce qui change le champ : 1,31 à σ = 1 ; 1,40 à 1,52 à 1,5 m/s ; 1,44 à 1,48 à 6 m/s. σ = 4 est
**refusée à la construction**, comme l'était λ = 20 m pour l'impact en S215 : la bibliothèque borne
sa famille elle-même.

**Le demi-spectre ne porte que `angular/2` directions distinctes** — 32 / 64 / 128 mesurées pour
`angular` 64 / 128 / 256. Le coût n'est donc pas l'obstacle à un maximum exact.

**Le résidu suit le temps depuis l'extinction, pas l'âge absolu** : à 30 s, **1,75** pour seize
tronçons (encore en forçage), 3,43 pour huit, **6,29** pour deux (28 s après extinction). Il est
insensible à la recette — 3,43 / 3,46 / 3,45 / 3,46 pour `radial` 128, `cutoff` 2, `cutoff` 4,
`angular` 256 —, sauf `radial` 32 à 2,89, qui est la recette que S212 savait déjà décrocher.

## 4. Le discriminant d'emprise, et il tranche

Si le résidu venait de ce que l'emprise est trop petite pour contenir le point où les phases
s'alignent, aucune loi mesurée ne le corrigerait : ce serait A208, et le remède serait côté hôte.

À âge fixé et **à pas de grille constant** — 0,5 m ; le faire croître avec l'emprise aurait rendu
le test vide, un maximum manqué par grossièreté se lisant comme un maximum absent —, l'emprise a
été agrandie d'un facteur 2 puis 4 en côté, soit **seize fois en aire** (512 × 416 m) :

| âge (s) | 128 × 104 | 256 × 208 | 512 × 416 |
|---:|---:|---:|---:|
| 4 | 0,097428 | 0,097428 | 0,097428 |
| 30 | 0,038229 | 0,038229 | 0,038229 |

**Identique à six décimales.** Le maximum est intérieur et saturé : le résidu est bien la
décohérence de phase (L290), et il demande une loi mesurée comme ADR-133 — donc sa propre campagne.

## 5. Ce qui a été construit

`Field::slope_envelope_directional()` — Cauchy–Schwarz sur `Σ c_i |cos(θ − θ_i)|` :
`√(C · (C + R)/2)` avec `C = Σ c_i` et `R = |Σ c_i e^{2iθ_i}|`. Un seul passage, deux
accumulateurs, **aucune arc-tangente**, `O(N)` sans grille de directions, sans table, sans garde et
sans domaine de validité. `bound_pressure::Prepared` la retient aux trois sites de préparation ;
`slope_envelope_tight()` reste publiée et inchangée, et sert de témoin.

Le **maximum exact** était possible — `O(A²)` avec `A = angular/2` — et n'a pas été retenu :
Cauchy–Schwarz ne suppose rien sur la disposition des emplacements et n'a besoin d'aucun tableau
intermédiaire. L'écart est celui des deux bouts analytiques : `0,707 C` contre `0,637 C` à
directions équiréparties. Il est **mesuré** au §6 et non supposé.

Test `directional_envelope_brackets_the_real_slope_s216` : jamais sous la pente réelle sur 40 401
points, jamais au-dessus de la resserrée, gain réel, et le bout analytique — directions toutes
alignées ⟹ la borne vaut exactement la somme scalaire, à 1e-5.

## 6. Réception sur la scène J1

| âge (s) | enveloppe avant | après | gain obtenu | gain disponible | part captée |
|---:|---:|---:|---:|---:|---:|
| 4 | 0,135072 | **0,112538** | 1,2002 | 1,2586 | 87 % |
| 16 | 0,164995 | **0,141372** | 1,1671 | 1,2154 | 81 % |
| 39 | 0,157981 | **0,136760** | 1,1552 | 1,1979 | 79 % |

**Budget de la scène**, impact déjà resserré par ADR-133 : 0,186540 → **0,162917** à 16 s, soit une
occupation de π/7 qui passe de 41,6 % à **36,3 %** ; 37,6 % → **32,8 %** à 39 s. Le chemin parcouru
sur cette scène : **84,1 % en S214, 41,6 % après ADR-133, 36,3 % après ADR-134.**

Contrôles : `d_eta_m = 0,000000000` aux cinq âges — **aucun bit publié n'a changé** ; `VERIFY`
inchangé (7,2271e-5 m à 16 s, 7,4625e-5 à 39 s), tolérance de 3 mm tenue ; GPU eau 4,185 ms
inchangé ; `--smoke` 120 images, code 0. Suite complète `code/` hors réseau : **356 réussis
(258 + 4 + 1 + 93), 5 ignorés**.

**Une seule attente de test a bougé, et pour une bonne raison** : à `max_slope = plancher/2`, le
verdict passe de `SlopeEnvelope` à **`Slope`**. À majorant plus serré, une limite fixée sous le
plancher tombe désormais sous la **pente réelle au point**, et le refus devient attribuable au champ
plutôt qu'à la marge (ADR-098). Figer la cause dans ce test en aurait fait un test du pessimisme du
majorant.

## Suite

**A255 reste ouverte sur sa part dynamique**, et c'est désormais tout ce qu'elle contient : le
résidu vaut 1,10 à 4 s et **3,98 à 39 s**, il est gouverné par le temps écoulé **depuis
l'extinction** de la source, il n'est pas une limite d'emprise, et il n'est pas touché par ADR-134.
La campagne qui le traiterait est celle d'ADR-133 transposée — chercher une échelle de temps propre,
vérifier l'effondrement sur la famille, traiter la phase de forçage à part puisque le majorant y
**croît**. Rien ne dit qu'une telle échelle existe : la découverte de S216 est précisément qu'une
partie du pessimisme n'en demandait pas.

Restent, inchangés : le budget est toujours une **somme** (A254) ; **A253** côté interface ; la
cadence complète, l'interaction manuelle et les poses de caméra ; les allocations de la pile
graphique (I-06) ; la seconde cible (B7). Puis la loi GPU — espace, LOD, visibilité, mutualisation.

## Suivi S217 — 2026-09-13 : l'âge après extinction ne suffit pas

Les chiffres de S216 sont conservés. Comparer à âge absolu égal des durées différentes
ne séparait pas l'effet de l'âge après extinction de celui de l'histoire du forçage.
S217 les sépare : à même âge réduit depuis extinction, vitesse et durée font varier
le rapport majorant publié/maximum de 38,8 % et 22,5 %, après raffinement. La similitude
ne vaut qu'à groupes adimensionnés constants ; aucune table universelle ajoutée.
Une source active peut également voir son majorant diminuer (fixture lente S217).
ADR-134 reste valide ; A255 reste ouverte. Voir [S217](DECOHERENCE-SILLAGE-S217.md).