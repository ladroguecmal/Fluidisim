# Composition impact + sillage par le cœur, et domaine d'image du sillage — S214, 2026-09-13

**Validité avant accélération** ([ADR-131](../adr/ADR-131-un-depassement-qualifie-une-implementation.md) D6).
Deux travaux nécessaires de J1, indépendants du coût : faire composer la scène **par le cœur**
(`mixed_water`, budget conjoint) et donner au sillage visible un domaine reçu (**A251**). Aucune
technique d'optimisation n'est ajoutée ici, et aucun verdict de budget n'est tiré.

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : phases repliées de B au GPU (S211) ; table de Bessel de l'impact à
  λ/16 (ADR-129) ; repli temporel du sillage (S213) ; publication `[A, B, kx, ky]` rebasée ; somme
  modale par sommet sur GPU dans l'emprise ; grille projetée à 2 px.
- **Techniques absentes** : grille locale et transformée ; LOD spatial, spectral, temporel ;
  visibilité ; mutualisation ; parallélisme CPU (un fil) ; SIMD explicite.
- **Domaine de validité** : scène J1 S201/S203/S205 — mer JONSWAP Hs 1,5 m / Tp 6 s / 32
  composantes / graine 201 ; **un** impact (λ 3,35 m, rayon 52 m, ttl 56 s) ; **une** source de
  sillage prescrite (8 tronçons de 2 s à 3 m/s sous 19 620 N, σ 2 m, cutoff 3, contexte 40 s,
  emprise `[-64, -48] × [64, 56]`) ; recettes 64×128 et 128×256 ; 6 988 sondes, caméra S201 ;
  AMD Ryzen AI 7 350, RTX 5070 Laptop, DX12, Windows, release. **Ne dit rien** d'un autre nombre
  de sources, d'un autre σ, d'une autre mer, d'une autre machine.
- **Rang de passage** : les coûts ci-dessous sont ceux des deuxième et troisième passages ; le
  premier, sur machine froide, donne 20 % de moins (voir §5).

## 1. Ce que l'hôte faisait, et ce que le cœur en dit

L'hôte sommait B, l'impact et le sillage **de sa propre main** — `FrameData::references` côté CPU,
le shader côté GPU — et n'avait jamais demandé au cœur si sa composition était admissible
([HOTE-GPU-S212](HOTE-GPU-S212.md) §Admission : « non exercé »). `mixed_water::sample_world_batch`
est le chemin qui compose **et refuse**, avec le budget de pente des perturbations (ADR-128) :
`slope_max()` de chaque champ d'impact plus `slope_envelope()` de la pression, comparé à
`max_slope`. Les deux moitiés avaient été admises séparément — l'impact sur cette mer en S205, le
sillage seul en S212 — jamais leur somme.

**Le chemin mixte du cœur ne peut pas être le chemin de rendu, et c'est une décision, pas un
défaut.** Il ne compose que sur l'**intersection** des domaines (ADR-077 : « leur intersection
détermine » ; ADR-080 : lot atomique) : **4 477 sondes sur 6 988**, les autres tombant hors du
disque de 52 m de l'impact (2 392) ou hors de l'emprise du sillage (1 589). L'image, elle, dessine
les 6 988. Un service qui interroge une bouée veut ce refus ; une image veut une valeur partout.

## 2. La composition est exacte, le point d'évaluation ne l'était pas

Critère déclaré avant mesure : accord à 1e-6 de l'amplitude. **Il échoue d'abord, et la prédiction
écrite pour être contredite l'est.**

| âge (s) | amplitude (m) | écart hôte / cœur avant (m) | après (m) |
|---:|---:|---:|---:|
| 4 | 0,9675 | **1,782e-5** | **0** |
| 8 | 0,9978 | 1,252e-5 | 0 |
| 16 | 0,7827 | 1,335e-5 | 0 |
| 24 | 0,7749 | 9,49e-6 | 0 |
| 39 | 1,2162 | 6,91e-6 | 0 |

La cause a été isolée dans le même passage, en comparant le cœur à **la même somme à la main, au
point local que le cœur emploie** : écart **0,000000000 m** sur η aux cinq âges, et 1,5e-8 sur les
pentes (aller-retour pente → normale → pente, un ulp). **La composition du cœur est donc exacte au
bit ; tout l'écart venait du point.**

`FrameData::references` évaluait **B** au point monde quantifié — `WorldPos`,
`WORLD_UNITS_PER_METRE = 2048`, pas de 488 µm, erreur ≤ 244 µm — et **l'impact et le sillage** au
point `f32` brut. Une même sonde avait deux positions. Le cœur, lui, convertit **une fois**
monde → local et sert les trois couches au même point (`eval_local` : « chemin interne après
conversion commune B/W »). Le produit de l'écart de position par la pente des perturbations donne
les 18 µm mesurés.

**Correction** (S214) : la référence construit un `WorldPos` par sonde, en tire une fois le point
local, et sert les trois couches avec celui-là. Le point du réseau est le seul que l'interface
publique de B sache servir — `eval_local` est `pub(crate)`. Après correction, la référence de
l'hôte **est** la composition du cœur, au bit sur η.

Portée : 0,6 % de la tolérance de 3 mm de `--verify`, donc jamais visible. C'était néanmoins la
**référence** qui était fausse, pas le GPU, et un LOD spatial creusant la pente l'aggraverait.
Consigné en **A253**.

## 3. Le budget conjoint : 84 % de π/7 pour deux sources, et un refus par majorant

`max_slope = BREAKING_SLOPE = π/7 = 0,448799`. Le budget est **point-indépendant** (ADR-128), donc
`slope_floor` **est** le seuil de bascule, dans les deux sens.

| âge (s) | budget conjoint | impact | sillage | part de π/7 | pente réelle max | majorant / réel |
|---:|---:|---:|---:|---:|---:|---:|
| 4 | 0,347679 | 0,212607 | 0,135072 | 77,5 % | 0,092876 | **3,74** |
| 8 | 0,357528 | 0,212607 | 0,144921 | 79,7 % | 0,089615 | 3,99 |
| 16 | **0,377603** | 0,212607 | 0,164995 | **84,1 %** | 0,073979 | 5,10 |
| 24 | 0,369652 | 0,212607 | 0,157044 | 82,4 % | 0,051954 | 7,11 |
| 39 | 0,370588 | 0,212607 | 0,157981 | 82,6 % | 0,035240 | **10,52** |

**Zéro refus aux cinq âges** : la scène passe. Les deux causes de refus d'ADR-098 ont été séparées
par deux seuils déduits des mesures :

- `max_slope` entre la pente réelle et le budget (0,2028–0,2258 selon l'âge) : **4 477 points sur
  4 477 refusés, tous `SlopeEnvelope`, zéro `Slope`** — le refus vient entièrement de la marge ;
- `max_slope` à la moitié de la pente réelle : 16 à 182 points passent en `Slope` selon l'âge, le
  reste reste `SlopeEnvelope`. Le lot rend `SlopeEnvelope` dans les deux cas — c'est le premier
  point qui parle.

**Ce que cela dit.** Le majorant conjoint vaut **3,7 à 10,5 fois** la pente réelle, et la scène J1 —
*une* source de chaque type — occupe déjà **84 %** de π/7. Marge restante : **0,0712**. Un second
sillage de la même recette (0,165) ou un second impact (0,213) **refuserait toute l'image**, et par
majorant, pas par raideur : la physique garde un facteur dix. A208 nommait ce mécanisme sur un champ
seul et par le choix d'emprise ; ici c'est l'**additivité sur le nombre de sources** (ADR-128,
ADR-119 règle 1) qui borne la scène, et elle n'avait jamais été mesurée composée. Consigné en
**A254**.

## 4. Domaine d'image du sillage — A251, ADR-132

Deux lois déduites de la recette, et reçues par trois critères indépendants. Elles sont les deux
mécanismes d'ADR-107, écrits en formule et recalibrés ; le détail est dans
[ADR-132](../adr/ADR-132-domaine-d-image-d-un-sillage.md).

```
rayon_honnête = 2π · angular / (3 · cutoff)        → 89,36 m   (coin d'emprise : 102,22 m)
durée_honnête = 4π / √(g · cutoff/radial)          → 18,53 s   (contexte déclaré : 40 s)
```

Mesures S214, 64×128 contre 128×256, mêmes 6 988 sondes, neuf âges :

| âge (s) | écart / amplitude | couture au bord (mm) | rayon d'accord à 10 % (m) |
|---:|---:|---:|---:|
| 4 | 0,37 % | 0,53 | ≥ 105 |
| 8 | 1,16 % | 1,37 | ≥ 105 |
| 12 | 1,40 % | 1,73 | ≥ 105 |
| 16 | 1,64 % | 2,28 | ≥ 105 |
| 18 | **3,55 %** | 2,54 | ≥ 105 |
| 20 | 3,63 % | **3,23** | ≥ 105 |
| 24 | 9,53 % | 5,62 | ≥ 105 |
| 30 | 20,2 % | 7,57 | **40** |
| 39 | 27,9 % | 12,70 | 30 |

- **2 %** (ADR-120) franchi entre **16 et 18 s** : la loi est optimiste d'au moins 3 %.
- **Couture de 3 mm** (tolérance S201) franchie entre **18 et 20 s** : la loi est juste.
- **Rayon d'accord à 10 %** (critère S156) effondré entre **24 et 30 s** : la loi est conservatrice
  de 20 à 40 %.

**Une durée honnête porte donc le critère qui l'a calibrée**, et une garde en porterait la marge.
**Le rayon ne borne jamais cette fixture** : l'accord tient sur toute l'emprise (≥ 105 m, au-delà du
coin à 102,22 m) jusqu'à 24 s, quand la loi annonce 89,36 m. C'est la durée qui mord — ADR-107
avait prévenu que laquelle des deux mord dépend de l'instant.

Le contexte déclaré de 40 s vaut **2,2 fois** la durée honnête. L'hôte le dit désormais : `WAKE_LOI`
publie le domaine avec la fixture, `WAKE_HORS_DOMAINE` s'imprime une fois au premier instant qui
dépasse (24 s dans le témoin). Il n'y a **pas** de refus : le chemin est cosmétique (ADR-129 §3,
I-04) et une garde dans la bibliothèque refuserait la fixture qui a servi à recevoir S211–S214.

## 5. Contrôles conservés, et coût

- `VERIFY` : max 8,03e-5 m à 0 s, 7,29e-5 à 4 s, 7,27e-5 à 16 s, 8,94e-5 à 40,01 s ; variation
  ≤ 3 µm par rapport à S213, tolérance de 3 mm tenue. Lignes `WAKE` inchangées aux cinq âges
  communs. `--smoke` 120 images, code 0.
- Coût, 4 096 nœuds : GPU eau **1,897 / 4,183 ms** (640×360 et 960×540), inchangé depuis S213 ;
  CPU sillage 1,58–1,84 ms. Aucune technique nouvelle, aucun verdict.
- Suite complète `code/` hors réseau : **354 réussis (256 + 4 + 1 + 93), 5 ignorés**, aucun échec.
  La bibliothèque n'a pas été modifiée cette session.

**Le coût varie de 20 % entre deux passages du même binaire.** Trois passages : 1,2555 / 1,2458 ms
(premier, machine froide), 1,6477 / 1,7760, 1,5820 / 1,8398. L'écart « hôte 1,7 contre exemple
1,26 ms » que S213 n'avait pas su attribuer **se reproduit entre deux passages du même programme**,
la valeur basse tombant sur le passage à froid : il n'y a pas lieu d'invoquer une différence entre
l'exemple et l'hôte. Ce n'est **pas** une attribution nommée — aucun compteur de fréquence ni de
température n'a été lu. Conséquence de méthode : une mesure de coût sur cette machine dit son rang
de passage (**L287**).

## Suite

La validité de J1 a avancé sur ses deux travaux nécessaires ; il en reste. **A254** est la plus
lourde : le budget de pente ne passe pas à l'échelle en nombre de sources, et une scène à deux
sources en consomme déjà 84 %. Elle se traite avant toute scène à plusieurs sources, donc avant la
mutualisation de J1-bis. Restent aussi la cadence complète mesurée, l'interaction manuelle et les
poses de caméra, les allocations de la pile graphique (I-06), la seconde cible (B7). Puis la loi
GPU — espace, LOD, visibilité, mutualisation — chacune avec son en-tête.
