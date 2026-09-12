# S183 — Coût complet du consommateur différentiel

2026-09-12. S182-1 / A50. **Mesure locale sur une machine, aucun budget cible certifié.**

Ce document publie **les conditions de mesure avant tout chiffre** (P2), puis les relevés (P3).
C'est délibéré : un chiffre de coût lu sans ses conditions devient un budget dans la session
suivante, et un budget recopié se périme en silence (A185). Ce qui est écrit ici avant la
première exécution reste écrit après, même si les chiffres déplaisent.

## 1. Ce qui est mesuré, et contre quoi

Le montage mixte publie deux consommateurs **strictement parallèles**, mêmes entrées, même
`classify`, même atomicité de lot :

| chemin | fonction | sortie | scalaires par point |
|---|---|---|---:|
| surface | `mixed_water::sample_world_batch` | `WaterSample` | 10 |
| différentiel | `mixed_water::differential_world_batch` | `DifferentialSample` | 31 (+3 par `momentum_residual`) |

La comparaison porte sur **le même montage, les mêmes points, le même instant**. Ce n'est donc
pas « le différentiel contre autre chose » : c'est le surcoût de demander les dérivées plutôt que
la surface, tout le reste étant égal.

Quatre phases sont chronométrées séparément, parce que l'hôte ne les paie pas au même rythme :

1. **Préparation** — une fois par montage ou par renouvellement : `RadialImpact::new`,
   `Prepared::build`, `bake` + `half_into`, `bound_pressure::Context::new`, `Controller::new`.
2. **Actualisation** — une fois par instant publié : `Controller::update`.
3. **Évaluation** — une fois par lot de points, sur les deux chemins.
4. **Refus** — chaque cause de refus indépendante des points, plus un refus par point.

S'y ajoute ce qui ne se chronomètre pas : **empreinte mémoire** des tampons que l'appelant doit
fournir, et **allocations** déclarées à l'hôte.

## 2. Conditions matérielles et logicielles

AMD Ryzen AI 7 350, Windows 11 x86_64 MSVC, rustc 1.97.0 / LLVM 22.1.6,
profil `release` du dépôt (`overflow-checks = true`, ADR-029 §3). Aucune modification de la
bibliothèque : la mesure ajoute un exemple et ne touche pas au code d'exécution.

Machine et chaîne identiques à [COUT-PROFIL-IMPACT-S125](COUT-PROFIL-IMPACT-S125.md) ; les deux
séries sont donc comparables entre elles, et avec rien d'autre.

**Protocole de chronométrage**, repris de S125 parce qu'il a déjà servi :

- une seconde de mise en régime sur **tous** les montages avant la première mesure ;
- quinze blocs, **ordre des montages renversé un bloc sur deux**, pour que la dérive thermique
  et la montée en fréquence ne se rangent pas systématiquement du même côté ;
- `black_box` sur les entrées et les résultats, pour que rien ne soit éliminé comme mort ;
- min / médiane / max **des moyennes de bloc**, jamais les latences extrêmes d'un appel isolé ;
- **deux exécutions indépendantes** du binaire ; les deux sont publiées.

Un ordinateur portable sous Windows n'offre ni cœur isolé ni fréquence fixe. Les médianes de
quinze blocs absorbent le bruit ordinaire ; elles n'absorbent pas une migration de cœur ni un
changement de politique d'alimentation. C'est la raison de la deuxième exécution : deux séries
qui s'accordent valent mieux qu'une série resserrée.

## 3. Montages — lots et recettes

Fixture de référence, celle de S181/S182 : `SeaState { hs 0,1 ; tp 6 ; θ 0,125 tour ; graine 42 }`,
ancre monde à `1e9 m`, impacts `N=64`, `λ=4 m`, `E=0,01 J`, `g=9,81`, `ρ=1025`, `h=20 m`,
pente limite 0,1, rayon 16 m, horizon 4 s ; fenêtre de pression 0 → 8 s, deux sources.
**Paramètres de banc, pas paramètres gameplay.**

Trois axes varient, un à la fois autour de ce point de référence :

| axe | valeurs | ce qu'il déplace |
|---|---|---|
| **lot** | 1, 8, 64, 256 points | amortissement des contrôles de montage, indépendants des points |
| **recette** de pression | (radial 8 × angulaire 12), **(16 × 24)**, (24 × 32) → 48 / 192 / 384 créneaux | coût modal par point et empreinte des deux pools |
| **composantes de B** | **16**, 64 | coût du fond par point, payé identiquement par les deux chemins |
| **impacts confirmés** | 0, **1**, 4 | coût radial par point et taille du pool |

Les valeurs en gras forment le montage de référence. Les points d'un lot sont tirés sur des
rayons irréguliers dans tous les quadrants, centre inclus, à l'intérieur du domaine — la même
distribution pour les deux chemins, et la même d'un bloc à l'autre.

## 4. Ce que la mesure ne prouvera pas

À écrire avant de mesurer, faute de quoi la conclusion s'élargit toute seule :

- **Ce n'est pas un budget.** Aucune enveloppe d'image, aucun nombre de requêtes par seconde,
  aucune cible. Les ~49 ms de cycle mixte de S118 sont un **contexte historique**, pas une
  enveloppe disponible ; S125 le disait déjà.
- **Ce n'est pas une mesure du cycle.** On mesure des appels de bibliothèque, pas le service
  vivant, pas la pression du cache d'un jeu réel, pas la concurrence.
- **Ce n'est pas multiplateforme.** Une machine, une chaîne, un système. I-03 porte sur les
  **valeurs**, pas sur les durées ; rien ici ne dit ce que coûte ce code ailleurs.
- **Un rapport différentiel/surface n'est pas un facteur de conception.** Il vaut pour ces
  montages ; S125 a montré qu'un facteur attendu de 4 sur les modes donnait 8,85 à 9,38 en usage
  étendu. Extrapoler un rapport hors de sa grille est le défaut que cette session cherche à ne
  pas commettre.
- **Multiplier le coût d'un point par un nombre de points n'est pas une réception.** Les
  contrôles de montage sont payés une fois par lot ; c'est précisément ce que l'axe « lot »
  mesure, et c'est pourquoi il est mesuré plutôt que supposé.

## 5. Allocations — ce qui est reçu et ce qui ne l'est pas

L'allocation se mesure sur **le canal que le système déclare** : `Allocator` de SPEC-004 §8.1,
avec `seal()`. Après scellement, toute allocation générale doit échouer plutôt qu'être comptée —
c'est ce qui rend I-06 mécanique (`host.rs`).

Le protocole : allocateur compteur, `Background::configure`, **`seal()`**, puis toute la suite —
préparation des impacts et de la pression, actualisation, évaluations des deux chemins, chemins
de refus. La réception attendue est `refused_after_seal == 0` **et** `persistent_calls` inchangé
après le scellement.

**Ce que ce contrôle ne voit pas**, et qui doit être dit : un `Vec` créé dans le chemin
d'exécution sans passer par `alloc_persistent` serait invisible à l'allocateur d'hôte. La
bibliothèque interdit `unsafe` (`#![forbid(unsafe_code)]`), et le dépôt n'a aucun allocateur
global instrumenté ; en introduire un pour cette mesure serait une décision d'architecture, pas
un détail de banc. Le complément est donc une **inspection de source** publiée avec les chiffres,
et elle est annoncée comme telle : une inspection, pas une mesure.

## 6. Relevés

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example differential_cost`

Deux exécutions du même binaire, notées **E1 / E2**. Médianes des quinze blocs sauf mention.
Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release,
C18 et C02 inchangés.

### 6.1 Ce que les deux chemins refusent, et ce qu'ils préservent

Aux mêmes entrées, les deux chemins rendent **la même cause** : `Time`, `Context`, `MaxSlope`,
`Capacity`, `Slope`. Sortie inchangée sur refus des deux côtés. Sans cette égalité, comparer
leurs durées comparerait deux contrats différents ; elle est donc reçue avant toute mesure.

### 6.2 Évaluation — le chiffre principal

µs par point, lot 64, `E1 / E2` :

| montage | créneaux | surface | différentiel | rapport |
|---|---:|---:|---:|---:|
| référence (B16, 1 impact) | 192 | 11,35 / 10,42 | 39,29 / 35,74 | 3,46 / 3,43 |
| recette 8×12 | 48 | 4,53 / 4,40 | 15,76 / 14,88 | 3,48 / 3,38 |
| recette 24×32 | 384 | 18,55 / 18,50 | 65,24 / 63,20 | 3,52 / 3,42 |
| B à 64 composantes | 192 | 13,69 / 12,15 | 45,58 / 42,41 | 3,33 / 3,49 |
| 0 impact | 192 | 8,92 / 8,73 | 30,95 / 30,14 | 3,47 / 3,45 |
| 4 impacts | 192 | 16,43 / 16,04 | 59,99 / 52,68 | 3,65 / 3,28 |

Axe des lots, montage de référence (`E1 / E2`) :

| lot | surface µs/pt | différentiel µs/pt | rapport |
|---:|---:|---:|---:|
| 1 | 8,59 / 8,10 | 33,84 / 31,93 | 3,94 / 3,94 |
| 8 | 8,89 / 8,62 | 35,53 / 33,19 | 4,00 / 3,85 |
| 64 | 11,35 / 10,42 | 39,29 / 35,74 | 3,46 / 3,43 |
| 256 | 12,26 / 11,27 | 39,31 / 35,98 | 3,21 / 3,19 |

Sur toute la grille, le rapport tient dans **3,0 à 4,3**, médiane voisine de **3,4**.

### 6.3 Attribution par couche

Le même lot 64, pression retirée, donne le coût de chaque couche par différence (µs/point,
`E1 / E2`) :

| couche | surface | différentiel | rapport |
|---|---:|---:|---:|
| B seul, 16 composantes | 0,646 / 0,664 | 2,097 / 2,177 | 3,25 / 3,28 |
| B seul, 64 composantes | 2,32 / 2,28 | 8,12 / 8,27 | 3,50 / 3,63 |
| un impact radial N64 | 1,708 / 1,551 | 6,274 / 5,531 | 3,67 / 3,57 |
| un impact, moyenne sur quatre | 1,710 / 1,582 | 6,076 / 5,693 | 3,55 / 3,60 |
| pression, 48 créneaux | 2,32 / 2,22 | 7,42 / 7,03 | 3,20 / 3,17 |
| pression, 192 créneaux | 9,00 / 8,21 | 30,92 / 28,03 | 3,44 / 3,42 |
| pression, 384 créneaux | 16,36 / 16,31 | 57,03 / 55,23 | 3,49 / 3,39 |

**Le rapport est le même couche par couche**, à 3,2–3,7, alors que les trois couches ne font
pas du tout le même calcul. Il ne suit donc pas la nature du travail dérivé : il suit **ce qui
est publié** — 31 scalaires contre 10, soit 3,1. La pression coûte 0,043–0,048 µs par créneau
en surface et 0,148–0,161 en différentiel, linéairement en créneaux.

### 6.4 Préparation et actualisation

µs par opération, `E1 / E2` :

| opération | 48 créneaux | 192 créneaux | 384 créneaux |
|---|---:|---:|---:|
| `bake` + `half_into` | 3,45 / 3,47 | 13,77 / 13,53 | 28,68 / 27,15 |
| `bound_pressure::Context::new` | 0,022 | 0,022 | 0,022 |
| `Controller::new` | 43,6 / 40,8 | 178,2 / 167,9 | 355,3 / 343,9 |
| `Controller::update` | 40,1 / 38,4 | 171,2 / 163,0 | 346,7 / 330,2 |

`RadialImpact::<64>::new` : **0,60 / 0,62 µs**, indépendant du montage — cohérent avec les
0,5723 / 0,5631 µs relevés par S125 pour N64. `Prepared::build` : 0,017 µs à zéro impact,
0,64 / 0,61 à un, 2,67 / 2,49 à quatre — soit **~0,62 µs par impact**, coût fixe négligeable.

**Ces quatre lignes sont identiques pour les deux chemins.** Le consommateur différentiel
n'ajoute rien à la préparation ni à l'actualisation ; tout son surcoût est dans l'évaluation
par point. En contrepartie, `update` coûte **0,85 à 0,90 µs par créneau**, une fois par instant
publié et quel que soit le nombre de points demandés : au montage de référence, une
republication vaut environ **4,4 points différentiels** ou **16 points de surface**.

### 6.5 Refus

µs par appel refusé, lot de 64 demandé, `E1 / E2` :

| montage | `Time` | `Context` | `Capacity` | hors domaine au point 63 | au point 0 |
|---|---:|---:|---:|---:|---:|
| référence | 0,047 / 0,048 | 0,025 | 0,050 / 0,051 | **2399 / 2232** | 2,11 / 2,00 |
| recette 8×12 | 0,047 / 0,048 | 0,025 | 0,050 / 0,051 | 1013 / 941 | 2,03 / 2,00 |
| recette 24×32 | 0,047 / 0,048 | 0,026 | 0,050 / 0,051 | 4354 / 4002 | 2,09 / 2,02 |
| B à 64 composantes | 0,047 / 0,048 | 0,026 | 0,050 / 0,051 | 2882 / 2653 | 8,37 / 7,57 |
| 0 impact | 0,031 | 0,025 | 0,031 | 2017 / 1873 | 2,14 / 1,99 |
| 4 impacts | 0,090 / 0,089 | 0,026 | 0,097 / 0,091 | 3661 / 3373 | 2,15 / 2,13 |

Les refus **indépendants des points** sont gratuits : 0,025 à 0,097 µs, contre 2514 µs pour le
lot de 64 qu'ils remplacent. Ils refusent bien avant de toucher un point, et c'est reçu.

Les refus **portés par un point** ne le sont pas. Un lot de 64 dont le dernier point sort du
domaine coûte **2399 / 2232 µs** — soit **95 %** du prix du même lot réussi (39,29 × 64 =
2514 µs) — et ne publie rien : le lot est atomique (ADR-063). Le même point placé en tête coûte
**2,0 µs**, un rapport de **1150**.

Ces 2,0 µs ne sont pas nuls non plus, et le montage à 64 composantes dit pourquoi : ils y valent
**8,37 / 7,57 µs**, soit exactement le différentiel de B à 64 composantes. Un point hors du
rayon d'un impact **paie d'abord B en entier**, parce que le test géométrique du domaine
d'impact vient après `differential_local`. Voir **A226**.

### 6.6 Empreinte et allocations

`WaterSample` 40 o · `DifferentialSample` 124 o · `Slot` 64 o · `Node` 16 o ·
`Option<RadialImpact<64>>` 1640 o. Tampons que l'appelant doit fournir, en octets :

| montage | cuisson | pools pression | champs impacts | tampons surface | tampons différentiels | total surface | total différentiel |
|---|---:|---:|---:|---:|---:|---:|---:|
| référence, lot 64 | 9216 | 24576 | 1640 | 5120 | 15872 | 40552 | 51304 |
| recette 8×12, lot 64 | 2304 | 6144 | 1640 | 5120 | 15872 | 15208 | 25960 |
| recette 24×32, lot 64 | 18432 | 49152 | 1640 | 5120 | 15872 | 74344 | 85096 |
| 4 impacts, lot 64 | 9216 | 24576 | 6560 | 5120 | 15872 | 45472 | 56224 |
| référence, lot 256 | 9216 | 24576 | 1640 | 20480 | 63488 | 55912 | 98920 |

Passer au différentiel coûte **+10,8 ko** sur un lot de 64 et **+43,0 ko** sur un lot de 256 :
ce sont les seuls tampons qui changent, `scratch` et `output`, à 124 octets par point au lieu
de 40. Les pools de pression dominent l'empreinte et ne dépendent pas du chemin choisi.

**Allocations d'hôte**, `Background::configure` puis `seal()`, sur les six montages :

| montage | octets | appels | refusées après `seal` |
|---|---:|---:|---:|
| B à 16 composantes (cinq montages) | 512 | 1 | **0** |
| B à 64 composantes | 2048 | 1 | **0** |

Une seule allocation, à la configuration, avant scellement. **Zéro demande après `seal()`** —
donc zéro dans toute la préparation des impacts et de la pression, l'actualisation, les
évaluations des deux chemins et les cinq chemins de refus. I-06 est tenu, mécaniquement.

**Inspection de source** complétant ce que l'allocateur d'hôte ne voit pas (§5) : sur les
dix-neuf modules du chemin d'exécution, hors modules `#[cfg(test)]`, le seul emploi du tas est
`Background.components: Vec<Component>` — alloué dans `configure` et `from_spectrum`, et
**déclaré à l'hôte** juste avant (`background.rs:144` et `:191`). C'est une inspection, pas une
mesure.

### 6.7 Ce que l'axe « lot » a réellement mesuré

Il devait mesurer l'amortissement des contrôles de montage. Il ne l'a pas fait, et c'est le
relevé §6.5 qui le dit : ces contrôles coûtent **0,025 à 0,097 µs**, soit moins de **0,4 ‰** du
lot de 256 points. Leur amortissement est sous le bruit.

Ce que l'axe montre à la place va dans l'autre sens : le coût **par point** *augmente* avec le
lot — 8,59 → 12,26 µs en surface, 33,84 → 39,31 en différentiel. Un lot plus grand visite des
points plus nombreux et plus dispersés ; la localité se dégrade, et cet effet vaut cent fois
celui qu'on cherchait. L'axe a donc mesuré la distribution des points, pas l'amortissement.
Voir **L263**.

Conséquence pratique, et elle n'est pas mince : **multiplier un coût par point par un nombre de
points sous-estime un grand lot**. Du lot 1 au lot 256 sur ce montage, le coût par point monte
de **13 à 16 %** en différentiel et de **39 à 43 %** en surface selon l'exécution.

## 7. Ce qui est reçu, et ce qui ne l'est pas

**Reçu.**

1. Le consommateur différentiel coûte **3,0 à 4,3 fois** le chemin de surface aux mêmes entrées,
   médiane ~3,4, et le facteur est **le même pour chacune des trois couches**. Il suit le nombre
   de scalaires publiés, pas la nature du calcul dérivé.
2. Préparation et actualisation sont **identiques** sur les deux chemins : aucun surcoût
   différentiel hors de l'évaluation par point.
3. `+124` octets par point de tampon au lieu de `+40` ; rien d'autre ne change dans l'empreinte.
4. **Zéro allocation d'hôte après `seal()`** dans tout le cycle mesuré, refus compris.
5. Les deux chemins refusent **pour la même cause** aux mêmes entrées, et préservent leur sortie.

**Non reçu, et à ne pas déduire de ce qui précède.**

- **Aucun budget.** Rien ici ne dit combien de points par image le système peut servir. Les
  ~49 ms de cycle mixte et ~35 ms de requête 64 de S118 décrivent un autre montage à une autre
  époque ; ils ne sont ni un témoin ni une enveloppe, et cette session ne les a pas rejoués.
- **Une seule machine**, une seule chaîne, un seul système. I-03 porte sur les valeurs, jamais
  sur les durées.
- **Pas de cycle vivant** : des appels de bibliothèque, pas un service concurrent, pas la
  pression de cache d'un jeu réel.
- **Pas de δ3D ni de solveur.** Le consommateur nourrirait un solveur perturbatif qui n'existe
  pas ; le coût de le *consommer* reste hors mesure.

## 8. Suite

**A50 reste partielle**, et elle le restera tant que le solveur perturbatif qui consomme
`momentum_residual` n'existe pas : ce qui vient d'être mesuré est le coût de **produire** la
source, pas celui de s'en servir.

Deux suites, dans cet ordre.

1. **S183-1 — la consommation perturbative.** Le chiffre qui manque pour fermer la boucle A50
   est celui d'un pas de solveur alimenté par cette source, contre le même pas sans elle. C'est
   aussi la première occasion de voir si le facteur 3,4 se retrouve, s'efface, ou se paie
   ailleurs.
2. **Le classement des points avant le lot**, que A226 réclame et que §6.5 chiffre.

Aucun ADR : rien n'a changé de contrat. Aucun arbitrage humain nouveau.
