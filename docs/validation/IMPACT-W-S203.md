# S203 — Un impact porté par W devient visible

2026-09-13. S202-1, ADR-124 étape 3, ADR-125. Exemple CPU sans dépendance, images locales
seulement. **Aucune ligne de bibliothèque modifiée** : l'hôte déclare scène, emprise et
observateur ; `water-core` compose B+W par son chemin existant (ADR-062/063).
Relevés bruts : [IMPACT-W-S203-MESURES](IMPACT-W-S203-MESURES.md). Décision : [ADR-126](../adr/ADR-126-emprise-d-un-impact-visible.md).

## 1. Reproduction

Depuis `code/` :

```text
cargo test  -p water-core --example render_impact
cargo run   -p water-core --release --example render_impact scene
cargo run   -p water-core --release --example render_impact seams
cargo run   -p water-core --release --example render_impact controls
cargo run   -p water-core --release --example render_impact observer
cargo run   -p water-core --release --example render_impact cost
cargo run   -p water-core --release --example render_impact render ../captures 3
```

`render` écrit `impact-s203-aNNN_N.ppm` et `temoin-s203-aNNN_N.ppm` dans `captures/`
(ignoré par git) et **échoue** si un rayon reste non résolu, si la composition refuse un
point, ou si un pixel diffère du témoin sans avoir échantillonné l'emprise. Onze tests
d'exemple. La caméra, la marche et l'export de S201 vivent désormais dans
`examples/support/ray_view.rs`, une seule implémentation : après extraction, l'image S201
t12 se reproduit **au bit** (`0xa52ff81902b150c3`, 10 021 895 évaluations).

## 2. Constat d'amorce : la mer de S201 n'admet aucune composition

Avant tout impact, `compose` forme un plancher de pente indépendant du point,
`steepness_B·π = Σ aᵢkᵢ`, et le compare à `max_slope = π/7 = 0,4488` (ADR-094).

| Hs recette S201 | plancher L1 | majorant directionnel | pente échantillonnée | marge L1 |
|---:|---:|---:|---:|---:|
| 0,25 | 0,1014 | 0,0955 | — | 0,3474 |
| 0,50 | 0,2027 | 0,1911 | 0,1405 | 0,2461 |
| 1,00 | 0,4055 | 0,3822 | — | 0,0433 |
| **1,50** | **0,6082** | **0,5733** | **0,4215** | **−0,1594** |

**La mer inspectée en S201 fait refuser chaque point de toute composition B+W.** Aucune
session ne l'avait passée par `compose` : l'image S201 évaluait B seul. Le plancher est
linéaire en Hs ; marge nulle à Hs ≈ 1,107 m pour cette recette (Tp 6 s, bande 0,5–4 fp).

Trois grandeurs, qu'il ne faut pas confondre :

- **L1**, `Σ cᵢ` avec `cᵢ = aᵢkᵢ` : ce que le budget consomme ;
- **directionnelle**, `max_u Σ cᵢ|dᵢ·u|` + erreur de discrétisation en `u` majorée : une
  **borne démontrée**, strictement sous L1 dès que les directions ne sont pas colinéaires
  (éventail 0,25 tour ici). Gain de 5,7 % seulement : **elle refuse encore la mer S201** ;
- **échantillonnée**, maximum de `|∇η|` sur 512×512 m au pas 1 m et 121 instants : borne
  **inférieure** du maximum réel. Rapport L1/échantillon 1,4428, identique à 0,5 et 1,5 m
  (champ homothétique en Hs).

ADR-094 et ADR-095 tiennent `steepness_B·π` pour la pente **exacte** du fond, marge 1. C'est
faux pour un B à plusieurs directions : sa marge sur le maximum réel est d'au moins 1,061
(borne directionnelle) et, sur la fenêtre échantillonnée, d'au plus 1,443. Notes correctives
datées dans les deux ADR ; angle mort **A245**. Aucune borne indépendante du point n'admet
la mer S201 : seule une grandeur **locale** (la pente réelle au point, connue à
l'échantillonnage depuis S144) ou statistique le ferait. Ce lot ne migre rien : il change des
bits et des frontières d'admission.

**Conséquence pour ce lot** : la scène passe à **Hs = 0,5 m**, recette S201 inchangée par
ailleurs (`0xae7cc08b64111fda`). C'est un choix de banc, dit, pas une préférence visuelle.

## 3. La scène

- Fond : JONSWAP V1 N32, Tp 6 s, graine 201, éventail 0,25 tour, Hs 0,5 m ; naissance de
  l'impact à t = 12 s (instant de l'image S201), au point local (0 ; 10) m.
- Milieu de l'impact : g 9,81, ρ 1025, profondeur 20 m (régime profond : > λ).
- **Budget de pente alloué** : `Medium::max_slope = π/7 − plancher_B = 0,246068`. L'hôte
  le déclare, pour que la **construction** refuse un impact trop raide au lieu que la
  requête refuse chaque point.
- Entrée de banc par `impact_generator` : b = 1 m, v = 8 m/s, fraction transférée
  **0,005** (à calibrer B2 ; borne admise par le budget 0,006698). D'où λ = 3,35 m,
  E = 164 J sur E_ref = 32 800 J.
- Champ : `slope_max` 0,212607 (plancher + pente = 0,415338 ≤ π/7), borne L1 0,381646,
  **η centre à la naissance 0,15482 m** — borne de hauteur exacte, tous les coefficients
  étant positifs ; la bibliothèque ne la publie pas, l'échantillon au centre la rend.

## 4. L'emprise : ce qui se voit à sa frontière

`RadialImpact` refuse hors de son disque et après son horizon. Hors emprise, l'hôte rend
B seul : **toute amplitude non nulle à la frontière est une couture visible**, dans l'espace
(r = R) ou dans le temps (t = A). L'admission numérique (Resolution, Reach) ne dit rien de ces
coutures.

Critère déclaré **avant** la campagne : max|η_W| ≤ min(3 mm, 2 % de η centre) = 3,00 mm sur
les deux coutures. 3 mm est la tolérance verticale de la marche de rayon ; 2 % est le seuil
d'ADR-120 **emprunté** comme choix de banc — il n'a pas été dérivé pour une couture.

**Couture temporelle, indépendante de N** : 78,4 mm à 2 s · 20,0 à 4 s · 13,0 à 8 s ·
10,1 à 12 s · 8,3 à 16 s · 6,2 à 24 s · 4,8 à 32 s · 3,25 à 48 s · 2,80 à 56 s · 2,44 à 64 s.
**Aucun horizon ≤ 32 s ne passe**, quelles que soient N et R : un impact linéaire ne
s'éteint pas, il s'étale.

**Couture spatiale** : l'amplitude de l'anneau quand il franchit R, indépendante de A tant
que A > R/c_g — 5,05 mm à 30 m · 3,84 à 40 · 3,06 à 50 · 2,77 à 55, soit ≈ 1/R.

Les deux coutures se croisent : il faut R assez grand pour que l'anneau y arrive déjà
atténué, **et** A assez long pour qu'il ait quitté R ou soit atténué. Le premier candidat
qui passe selon la règle déclarée (plus petit N, puis plus petit R) :

| N | A | R | couture spatiale | couture temporelle |
|---:|---:|---:|---:|---:|
| **256** | **56 s** | **52 m** | 2,937 mm | 2,255 mm |
| 256 | 48 s | 65 m | 0,281 mm | 3,246 mm ✗ |
| 256 | 64 s | 39 m | 3,935 mm ✗ | 0,010 mm |
| 512 | 56–72 s | ≥ 55 m | ≤ 2,77 mm | ≤ 2,80 mm |

N256 ne passe que dans une **fenêtre étroite** ; N512 passe largement. Deux contrôles avant
de retenir N256 :

- **accord N256/N512** à A 56 s, R 52 m : max|Δη| = 0,0000 mm à +1, +3, +6, +30 s et
  0,0001 mm à +56 s ;
- **homothétie** à b = 2 m (λ = 6,70 m, E = 1312 J) : η centre ×√2 exactement, coutures
  relatives inchangées à 0,019 point près (N256 R 104 m, A 79,2 s ; N512 R 110 m, A 90,5 s).

D'où la forme sans dimension retenue par **ADR-126**, au critère relatif de 2 % :
**R ≥ 15,5 λ, A ≥ 96·√(λ/g), N ≥ 256** — valeurs mesurées suffisantes : R = 14,9 λ
(50 m) et A = 82·√(λ/g) (48 s) échouent. Le seuil absolu de 3 mm devient liant dès que
η centre dépasse 0,15 m (η ∝ √E/λ). Rapprochement : ADR-105 exigeait déjà N512 pour
λ 2–4 m à R 80 m / 60 s ; même garde, autre emprise.

## 5. L'image : l'effet est localisé au pixel

Observateur S201 inchangé : caméra (0 ; −18 ; 7) m vers (0 ; 35 ; 0), 640×360, champ
vertical 50°, deux rayons par pixel. Dans R, `Prepared::sample_world_batch` (journal
confirmé d'un événement, pool N256) ; hors R, `Background::eval`. Bornes de marche B+W :
hauteur Σ|a| + η centre, pente L1 de B + borne L1 de W.

Le **témoin** exécute la même marche avec les mêmes bornes et le même prédicat
`admits`, mais rend B partout. Hors emprise, les deux rendus exécutent donc exactement les
mêmes opérations, et un pixel qui diffère sans avoir échantillonné R trahirait une fuite.

| âge | t | évals B+W | px différents | dont hors R | impact ms | témoin ms | FNV impact |
|---:|---:|---:|---:|---:|---:|---:|---|
| 1 s | 13 s | 5 075 829 | 6 780 | **0** | 55 191 | 10 508 | `ae92143729673447` |
| 3 s | 15 s | 5 062 984 | 8 640 | **0** | 56 739 | 10 583 | `3dba0d3acf15447a` |
| 6 s | 18 s | 5 064 027 | 15 737 | **0** | 58 172 | 10 621 | `7e057b5f0ccac53e` |

Zéro rayon non résolu, zéro refus de composition, résidu ≤ 3 mm ; 125 979 pixels (27,3 %)
touchent l'emprise. Témoins : `db298dee39248f0d`, `14271a7145740ba1`, `590a5964b0a93cf0`.
Inspection des zooms : anneaux discrets à +1 s, nets à +3 s (écart maximal 18 niveaux de
luminance, 1 902 px ≥ 3 niveaux), étendus et déformant le reflet solaire à +6 s. Coûts
uniques, pas des médianes ; hors ligne, comme S201.

## 6. L'observateur

Au point d'impact (pixel 320,0 ; 224,1, à 28,86 m) : λ occupe **11,05 px le long de la
visée** et 45,10 px en travers ; λ/2, la plus courte composante, 5,51 et 22,55 px. Même
azimut, en s'éloignant : λ passe sous 2 px le long de la visée **dès 67 m**, λ/2 dès 47,5 m ;
en travers, λ/2 dès 325,5 m et λ jamais avant 600 m. Le raccourcissement de la visée
rasante, pas la distance seule, décide de ce que l'observateur peut résoudre.

**L'observateur est dans l'emprise en plan** (28 m du centre, R = 52 m) : la projection de
l'emprise n'est pas bornée, et une vue proche d'un impact paie W sur une grande part de
l'image. Une première boîte englobante, calculée sur des points derrière le plan image,
était fausse ; elle a été retirée avant publication.

## 7. Le coût face à ADR-125 (60 images/s, eau 2 ms)

4096 points déterministes dans R, trois chauffes, onze mesures, médianes, un fil :

| âge | B seul | W seul | B+W par lot | par point B+W | points dans 2 ms |
|---:|---:|---:|---:|---:|---:|
| 1 s | 6,55 ms | 50,0 ms | 57,1 ms | 13,95 µs | 143 |
| 3 s | 6,70 ms | 51,8 ms | 59,0 ms | 14,41 µs | 139 |
| 6 s | 6,63 ms | 51,8 ms | 59,3 ms | 14,49 µs | 138 |
| 30 s | 6,50 ms | 56,2 ms | 58,6 ms | 14,30 µs | 140 |

**B seul coûte 1,6 µs par point** : environ 1 250 points dans 2 ms sur ce CPU. L'évaluation
CPU par point ne tient pas 60 Hz, même sans W ; ADR-125 le disait du renderer, c'est
désormais mesuré pour le champ.

**Piste hôte mesurée, non adoptée** : le champ étant radial, une table η(r), η'(r) au pas
λ/16 = 0,209 m (249 rayons) puis Hermite cubique par point. Erreur maximale 0,0014–0,0062 mm
contre l'échantillon direct sur 20 000 rayons — recevable pour le rendu. Évaluation
15–20 ns par point. **Mais la construction coûte 2,66–2,76 ms par image, pour un seul
impact** : N×M échantillons, au-delà du budget eau entier. Pas élargi, parallélisme, N
réduit hors couture ou évaluation GPU des phases (I-08) sont des leviers non mesurés.

## 8. La part que W ne porte pas

Ce qu'un effet δ borné aurait à représenter pour cette entrée, sans rien construire :

- **l'énergie** : 32 636 J sur 32 800 ne partent pas en ondes (fraction à calibrer B2) —
  gerbe, cavité, turbulence, chaleur ;
- **la cavité** : Froude d'entrée v/√(g·2b) = 1,806, sous le seuil ≈ 5 de cavité franche
  (SPEC-002 §3) ; demi-largeur mouillée b = 1 m = 0,2985 λ ;
- **la gerbe** : toute nappe projetée au-delà de ≈ 1 m/s se fragmente (SPEC-002 §1) ;
  majorant balistique d'une projection à v : **3,26 m de haut, 1,63 s de vol** ;
- **le volume déplacé** : W est à volume nul par construction ; `displaced_l` n'y entre pas.

SPEC-001 §2.4 dimensionne déjà un domaine δ d'impact de 6×6×4 m à dx 0,05 m : 1,15 M
cellules, ≈ 37 Mo. C'est 2 246 fois les 512 cellules x–z mesurées en S202 à 0,59 ms.
**Aucun coût n'est transposé** : ni la dimension, ni la charge, ni le noyau ne sont les
mêmes, et aucun noyau 3D ou axisymétrique n'existe dans le dépôt. Ce qui est établi : une
emprise δ d'impact est de l'ordre de 2b × 2b × v²/2g et vit moins de 2v/g ; ce qui ne l'est
pas : qu'un volume eulérien soit le bon outil pour une gerbe qui se fragmente (particules,
SPEC-002 §1), plutôt que pour la seule cavité.

## 9. Ce qui n'est pas reçu

- aucune perception : 3 mm et 2 % sont des seuils de banc, pas des seuils de visibilité ;
- une énergie, une recette, un observateur ; homothétie vérifiée sur λ×2 seulement ;
- **renouvellement non examiné** : `RenewalController` pourrait repousser l'horizon, mais
  le garde de résolution croît avec R + c_g·A, et ce lot ne l'a pas mesuré ;
- un seul impact ; plusieurs impacts additionnent leur coût et leur pente ;
- B4/A50 partiels, A98 intacte, A244 inchangée, I-05 non reçu.

## 10. Suite

Deux lignes nouvelles dans la file active : **A245**, rendre la composition possible sur
une mer de Hs > 1,1 m (lot bibliothèque, change des bits) ; **A247**, le coût par image d'un
impact visible (table 2,7 ms, point 14 µs) contre 2 ms. L'effet δ borné d'impact reste à
choisir entre cavité eulérienne et gerbe particulaire, après A245.
