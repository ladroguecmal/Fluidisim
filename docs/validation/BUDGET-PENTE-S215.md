# Le budget de pente à plusieurs sources, et ce qui le rendait pessimiste — S215, 2026-09-13

Traite **A254** (sévérité 1, S214) : le budget de pente est une somme sur les sources, et la scène
J1 en consommait 84 % avec deux. Décision : [ADR-133](../adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md).

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : aucune technique de rendu. Ce document mesure un **contrat
  d'admission**, pas un coût. Échantillonnage fin des champs : 20 001 points de rayon pour l'impact
  (pas 2,6 mm = λ/1288), grille de 0,25 m pour le sillage puis raffinement local à 2 cm.
- **Techniques absentes** : sans objet — aucun budget de temps n'est en jeu ici. Le budget de 2 ms
  (ADR-125) et le budget de pente (ADR-128) sont deux contrats distincts et ne se mélangent pas.
- **Domaine de validité** : scène J1 (S201/S203/S205/S212) — mer JONSWAP Hs 1,5 m, un impact
  λ 3,35 m / E 164 J, un sillage prescrit 64×128 ; famille d'impacts λ ∈ {0,5 ; 0,75 ; 1 ; 2 ;
  3,35 ; 5 ; 8} m et E ∈ [0,05 ; 4 000] J, domaine ADR-126 (`R = 15,5 λ`, `A = 96 √(λ/g)`),
  profondeurs 4 à 20 m. Impacts **isotropes** seulement — `RadialImpact::new` refuse les autres.
  **Ne dit rien** de la famille du sillage, ni d'un autre profil radial qu'ADR-094.
- **Rang de passage** : sans objet (aucune mesure de temps ; L289 ne s'applique pas).

## 1. Le doute qui commandait la session, et son issue

S214 comparait le majorant conjoint à une pente réelle relevée sur une **grille de 1,3 m**, quand
le maximum de pente d'un impact radial est atteint en `r = 0,2062 λ` — 0,69 m ici. La grille passait
à côté du pic par construction, et S214 l'avait noté en réserve sans le mesurer.

**Issue : le pessimisme survit à l'échantillonnage fin, et sa cause change.** Le chiffre conjoint de
S214 (3,7 à 10,5) tenait, mais son attribution était fausse — il n'était pas partagé entre les deux
champs, il venait presque entièrement de l'impact.

## 2. Ce que valent réellement les deux majorants

Prédiction écrite avant mesure : impact serré (< 1,5), sillage lâche (> 3). **Exactement inversée.**

| âge (s) | impact réel | majorant | pessimisme | sillage réel | majorant | pessimisme |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 0,212607 | 0,212607 | **1,00** | 0 | 0 | — |
| 2 | 0,105857 | 0,212607 | 2,01 | 0,064453 | 0,107494 | 1,67 |
| 4 | 0,049488 | 0,212607 | 4,30 | 0,097430 | 0,135072 | **1,39** |
| 8 | 0,033053 | 0,212607 | 6,43 | 0,085966 | 0,144921 | 1,69 |
| 16 | 0,021147 | 0,212607 | 10,05 | 0,086934 | 0,164995 | 1,90 |
| 24 | 0,015711 | 0,212607 | 13,53 | 0,043181 | 0,157044 | 3,64 |
| 39 | 0,010525 | 0,212607 | **20,20** | 0,033152 | 0,157981 | 4,77 |
| 56 | 0,006986 | 0,212607 | **30,44** | — | — | — |

**Sûreté d'abord** : `max(pente réelle / majorant)` vaut **0,999998** pour l'impact et **0,718847**
pour le sillage. Aucun majorant n'est dépassé — sur 56 s, là où ADR-094 n'avait vérifié que 2 s.
L'annonce d'ADR-094 est **exactement atteinte à la naissance**, et à cet instant seulement.

**Le mécanisme est un seul, et il est général à W : la dispersion.** Un majorant bâti comme une
**somme de modules modaux** — une norme L1 — est invariant quand chaque mode ne fait plus que
tourner. Le maximum **spatial**, lui, décroît à mesure que les phases se décohèrent. Le sillage le
montre par l'autre bout : serré tant que sa source force (1,39 à 1,90 jusqu'à 16 s), il se desserre
dès qu'elle s'éteint (3,64 à 24 s, 4,77 à 39 s) pendant que son majorant reste figé à 0,157.

Ce n'est donc **ni** un défaut d'unité — **I-18 est tenu**, les deux termes sommés sont déjà
convertis en pente réelle (S141) — **ni** l'alignement d'A208 : c'est le temps.

## 3. La décroissance est universelle, et c'est ce qui rend une loi possible

Le rapport ne dépend que de l'âge **adimensionné** `τ = (t − birth)/√(λ/g)`. Relevé à onze valeurs
de τ, il est **identique à trois décimales** pour λ = 0,5 · 1 · 3,35 · 8 m :

```
τ        0     0,5      1      2      4      8     16    27,4     48    66,8     96
ρ(τ)  1,000  8,713  1,063  1,273  2,409  4,823  7,071  10,066  15,228  20,227  30,243
```

- **Indépendant de l'amplitude** : deux énergies à λ = 3,35 (164 J et 16,4 J) donnent la même
  colonne. La linéarité l'exigeait ; le contrôle le dit au lieu de le supposer.
- **Indépendant de la profondeur, dans le domaine où le champ existe** : colonnes identiques de
  20 m à 4 m ; à 2 m et moins `RadialImpact::new` **refuse le champ lui-même**. Il n'y a donc pas
  de réserve de profondeur à porter — le domaine de la similitude est celui de la construction.
- **Non monotone** : 8,713 à τ = 0,5 puis 1,063 à τ = 1 — la perturbation s'aplatit avant de se
  reformer. Une table sûre prend le **minimum par intervalle**, jamais une interpolation.
- **Recoupement** : ρ(27,4) = 10,066 contre 10,054 mesuré directement à 16 s ; ρ(66,8) = 20,227
  contre 20,200 à 39 s ; ρ(96) = 30,243 contre 30,435 à 56 s.

## 4. La table, et pourquoi elle est sûre

`RHO_DISPERSION`, 96 entrées, une par unité de τ sur `[0, 96[` — la borne d'âge d'ADR-126.
Chaque entrée est le **minimum** du rapport sur 21 sous-échantillons **et sur quatre λ
génératrices** (0,5 · 1 · 3,35 · 8 m). Quatre λ, parce que l'effondrement en τ est exact mais que
l'âge transite en **microsecondes entières**, et que cette quantification déplace le rapport de
quelques ppm d'un λ à l'autre.

**Garde `1e-4`, avec sa provenance** (I-14) : sans elle, le majorant resserré était dépassé de
**3,0e-6** au pire sur trois λ hors famille ; la garde vaut **trente-trois fois** ce dépassement
mesuré, et le banc qui la fixe est `examples/budget_pente_s215.rs --table`.

**Contrôle final**, quatre λ hors famille génératrice, 961 valeurs de τ chacune :
`max(pente réelle / majorant resserré) = 0,999983` — **exactement** le pire cas du majorant
d'origine. Le resserrement hérite de la précision d'ADR-094 et ne la dégrade pas.

Hors du domaine mesuré — avant la naissance, au-delà de `τ = 96` — `slope_max_at` rend
`slope_max()` telle quelle. On ne resserre pas ce qu'on n'a pas mesuré.

## 5. Le refus, exercé puis levé

Scène J1 à 16 s, un sillage prescrit et *n* impacts voisins (positions [0;10], [3;10], [-3;10] —
l'intersection des disques de 52 m reste large, donc c'est le budget qui décide et non la
géométrie), composée par `mixed_water::sample_world_batch` à `max_slope = π/7`.

| impacts | budget avant | part | verdict avant | budget après | part | verdict après |
|---:|---:|---:|---|---:|---:|---|
| 1 | 0,377603 | 84,1 % | `Ok(())` | 0,186540 | 41,6 % | `Ok(())` |
| **2** | **0,590210** | **131,5 %** | **`Err(SlopeEnvelope)`** | **0,208086** | **46,4 %** | **`Ok(())`** |
| 3 | 0,802818 | 178,9 % | `Err(SlopeEnvelope)` | 0,229631 | 51,2 % | `Ok(())` |

A254 est confirmée dans les termes exacts où elle avait été écrite — **la deuxième source refusait
l'image**, et par `SlopeEnvelope`, le majorant et non la raideur — puis levée pour la part qui
revenait à l'impact. Le budget rendu par la bibliothèque et celui calculé à la main depuis la table
coïncident à 1e-6.

## 6. Contrôles conservés

- Hôte : occupation 42,3 / 39,8 / 41,6 / 38,5 / 37,6 % aux cinq âges, contre 77,5 à 84,1 en S214.
  `d_eta_m = 0,000000000` — la composition reste exacte au bit contre la somme de l'hôte, donc
  **aucun bit publié n'a changé**. `VERIFY` inchangé (7,2271e-5 m à 16 s ; 7,4625e-5 à 39 s),
  tolérance de 3 mm tenue ; GPU eau 1,904 / 4,187 ms inchangé ; `--smoke` 120 images, code 0.
- Suite complète `code/` hors réseau : **355 réussis (257 + 4 + 1 + 93), 5 ignorés**, aucun échec —
  un de plus qu'en S214, le test de sûreté d'ADR-133 (quatre λ dont trois hors famille génératrice,
  481 instants × 2 001 rayons chacun).
- Trois attentes de tests ont bougé, **toutes portant sur le budget lui-même**. Aucune cascade.

## Suite

**Le sillage domine désormais le budget** : 0,165 contre 0,0215 pour l'impact à 16 s. Le mécanisme
y est le même et le pessimisme est mesuré (1,4 à 4,8), mais sa famille est paramétrée autrement —
σ, cutoff, radial, angular, tronçons — et aucune similitude n'y a été établie. C'est la campagne
suivante, et elle décide de ce que coûte une scène à plusieurs sillages.

Restent, inchangés : le budget est toujours une **somme**, et le resserrement recule la limite sans
la supprimer ; **A253** côté interface (`eval_local` est `pub(crate)`) ; la cadence complète,
l'interaction manuelle et les poses de caméra ; les allocations de la pile graphique (I-06) ; la
seconde cible (B7). Puis la loi GPU — espace, LOD, visibilité, mutualisation.

## Note corrective S217 — 2026-09-13

« Le majorant reste figé » ne vaut pas comme invariant du sillage : la réponse libre
mélange hauteur et vitesse complexes, et |eta| peut varier. L'énergie conservée ne
rend pas constante l'enveloppe des hauteurs. ADR-133 reçoit une note datée ; les
mesures et la décision d'impact sont conservées. Voir [S217](DECOHERENCE-SILLAGE-S217.md).