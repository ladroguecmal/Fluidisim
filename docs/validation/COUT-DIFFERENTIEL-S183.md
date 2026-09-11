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

*À recevoir en P3. Aucun chiffre n'est écrit avant l'exécution.*

## 7. Décision et suite

*À recevoir en P4.*
