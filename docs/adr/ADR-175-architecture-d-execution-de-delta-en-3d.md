# ADR-175 — Architecture d'exécution de δ en 3D : pas de production résident sur GPU à travail borné, référence CPU pour la réception

- **Statut : actée**, S294, 2026-09-19 ; autonomie technique S71 et **accord de principe de
  l'utilisateur** ([ADR-174](ADR-174-arbitrages-du-2026-09-19.md) D6 : « oui si cela débloque la
  situation »).
- **Traite A295.** Applique [ADR-007](ADR-007-interface-solveur.md) §2 et §4.1,
  [SPEC-004](../specs/SPEC-004-interfaces.md) §4 et §8.4, [ADR-012](ADR-012-ordonnanceur-budget-degradation.md)
  §3, §4 et §7, ADR-174 D3 (δ ≤ 2 ms GPU), I-04, I-05, I-06, I-12, I-13, I-17.
- **Retire du chemin de production** la délégation d'[ADR-173](ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md),
  qui reste valable dans le cœur (note datée posée). **Précise**
  [ADR-143](ADR-143-la-pression-f32-converge-a-sa-precision-representable.md) et
  [ADR-144](ADR-144-la-tolerance-physique-est-une-condition-d-acceptation.md) : critères de la
  référence et des bancs de réception, pas portes d'exécution à chaque pas.
- Diagnostic d'origine : [BILAN-GLOBAL-S293](../registres/BILAN-GLOBAL-S293.md) T3.

## 1. Constat

- **La porte B demande δ sur les deux dimensions horizontales**, et ADR-173 garde sur CPU, à chaque
  pas, le résidu `b − A·p`, les itérations restantes et les portes d'ADR-143/144 ; advection et
  couplage y sont aussi. Extrapolé des coûts par maille mesurés en S291, un domaine 64×64×32
  coûterait 30 à 120 ms de CPU par pas — estimation, non mesure (A295).
- **Le chemin S289–S291 attend la carte et relit la pression à chaque pas.** SPEC-004 §8.4 : « la
  lecture synchrone n'existe pas ». Ce contrat avait été contourné sans être relevé.
- **Les portes d'ADR-144 buteront d'emblée en 3D** : ADR-144 l'écrit lui-même — franchir 32 768
  mailles « est le point dur du passage à la 3D, qui y arrivera d'emblée ». Un domaine 32³ y est.
- **S291 : « la variance vient entièrement du nombre d'itérations qui restent au processeur ».**
  Le pire pas est commandé par une boucle à longueur non bornée.
- **Les contrats d'origine décrivent déjà l'autre voie** : `IFluidSolver` 3D (ADR-007 §1), GPU
  à lecture différée avec `latency_frames ≥ 1` (§4.1), dégradation « moins d'itérations de
  pression » déclarée (§2 ; SPEC-004 §4 `PressureItersCut`), tick découplé et δ avec au plus une
  image de retard (ADR-012 §7). Cette décision y revient ; elle n'invente pas d'architecture.

## 2. Décision

**D1 — Deux implémentations, deux rôles.**

- La **référence** vit dans le cœur (`water-core`, CPU, sans dépendance, ADR-020). Le schéma y est
  défini et reçu contre des oracles indépendants. Elle **sort de la boucle d'image** : c'est la
  spécification que la production doit reproduire, et l'instrument qui la juge.
- La **production** vit dans l'hôte (`viewer/`, wgpu déjà autorisé, ADR-130). Le pas **entier** —
  advection, bandes de couplage à B/W, projection, surface, éponge — est résident sur GPU, sur des
  tampons réservés à la création du domaine (I-06). Le CPU ordonnance, publie les paramètres
  analytiques de B/W (phases repliées, I-08) et enregistre un nombre borné de dispatchs ; **aucun
  travail en `O(N)` sur CPU, aucune lecture synchrone** (SPEC-004 §8.4).

**D2 — Travail borné par pas (I-05, ADR-007 §2).** Le pas de production fait une quantité de
travail **fixée par le profil** : un nombre donné de cycles multigrilles, ou d'itérations
préconditionnées, pour la pression ; jamais une boucle jusqu'à convergence dans la boucle d'image.
Son coût GPU est donc borné et mesurable, contre la part de δ d'ADR-174 D3.

**D3 — Qualité mesurée et publiée, dégradation déclarée (SPEC-004 §4).** Chaque pas calcule sur
GPU, par réduction, la divergence projetée des lignes franches (la grandeur d'ADR-144) et la dérive
de masse. Ces diagnostics sont **relus en différé**, avec leur âge. Au-dessus de la tolérance
d'ADR-144 (`10⁻⁵`, aucun nombre nouveau), le pas est déclaré dégradé (`PressureItersCut`) ; il n'est
ni refusé ni refait dans la boucle. L'ordonnanceur répond selon ADR-012 §4 — rétrécir d'abord,
puis la résolution — ou accorde des cycles dans le budget.

**D4 — Classe de fidélité par couche.**

| couche | à l'exécution | à la réception |
|---|---|---|
| B, W répliqué, V | déterminisme au bit entre plateformes (I-03) | inchangée |
| W local, habillage de rendu | cosmétique | écart au cœur publié |
| δ, référence CPU | hors de la boucle d'image | oracles indépendants ; ADR-143/144 comme critères d'arrêt et d'acceptation |
| δ, production GPU | budget borné, diagnostics publiés, dégradation déclarée | écart à la référence sur les cas de réception, en hauteur, pente et phase contre les tolérances d'image (METHODE, 3 mm S201) ; **aucune identité au bit**, ni avec le CPU ni entre cartes |

δ reste cosmétique et non répliqué : aucune grandeur de jeu n'en sort (I-04, I-15), aucun état
n'est sérialisé (I-17). La réception physique ne dépend donc d'aucun pilote : elle est portée par la
référence, et la production est jugée par comparaison — à refaire sur toute nouvelle carte (A98).

**D5 — Représentation du régime perturbatif en 3D (B3 préliminaire).** Grille MAC x-y-z ; surface
**fonction hauteur** `η(x, y)` à fluide fantôme ; pression scindée `p_hydro + p_dyn` ; bandes de
couplage à B/W sur les quatre côtés et éponge. C'est l'extension directe du candidat 2D reçu —
S237, S253, S254, S268–S274 — et elle en garde les réceptions comme cas limites. Les phénomènes
**non graphes** — cavité, jet, déferlement : scénarios 2 et 3 de B3 — ne sont **pas** demandés à
cette représentation ; ils iront à une seconde, particules ou surface implicite, choisie avec le
premier domaine d'impact (B10), comme ADR-007 §3 l'autorise. La coque en mouvement (scénario 1)
relève de la porte D, par faces coupées (S232) étendues à la 3D.

**D6 — Forme, maille et cadence de départ.** Une boîte de blocs 8³ (ADR-006 §3) ; les ensembles
épars viennent avec B5. La référence est écrite pour tout `dx` ; celui de la production se choisit
sur la scène de la porte B et se consigne — la bande actuelle, à 2 m, est **hors** des niveaux
d'ADR-006 §3.2, ce qui se dira là où elle sera remplacée. Un pas par image d'abord (S275) ; le tick
découplé d'ADR-012 §7 reste la cible, avec l'ordonnanceur.

**D7 — Sortie publiée (I-13).** À la fin du pas, la production écrit une surface **publiée** et
immuable pour l'image — hauteur, et les champs qu'ADR-007 §4 énumère au besoin. Le rendu ne lie
jamais les tampons internes de δ.

## 3. Ce que la décision garde, et ce qu'elle déplace

- **ADR-173** reste vrai dans le cœur : un candidat externe n'y fournit qu'un départ, jamais
  dangereux. Il cesse d'être le chemin de production.
- **ADR-172** : l'export de l'opérateur par le cœur ne sert plus qu'aux essais ; la production
  assemble son opérateur sur la carte.
- **La bande 2D de l'afficheur** reste en l'état jusqu'à ce qu'un domaine 3D la remplace ; rien
  n'est supprimé.
- **A294** se remesure sur le nouveau chemin : sans attente synchrone, un calage de la carte se
  lit dans le temps d'image, au 99ᵉ centile (ADR-012 §3), et non plus dans le pas.

## 4. Réception de la porte B — critères posés avant la construction

1. **Référence 3D.** À `ny = 1`, elle reproduit les réceptions 2D contre HOS (S253 : à 128
   colonnes, 5 cm à 0,162 % / 0,34 %, 10 cm à 0,213 % / 0,53 %) à leurs tolérances ; sous une
   houle à crêtes longues alignée sur `x`, sa solution est invariante en `y` à l'arrondi près ; et
   une onde stationnaire **oblique** — mode `(m, n)` d'une cuve, `ω² = g·k·tanh(k·h)`,
   `k = π·√((m/Lx)² + (n/Ly)²)` — tient sa relation de dispersion, erreur de phase publiée et
   décroissante en raffinant.
2. **Production contre référence**, sur les mêmes cas : écart de hauteur sous 3 mm, pente et phase
   publiées, sur la durée déclarée.
3. **La scène** : une onde qui traverse une mer étalée (`--houle`, S259) dans un domaine 3D, rendue
   ; **revue de l'utilisateur** : « une onde traverse une mer étalée et s'y déforme ».
4. **Le coût relève de la porte C**, mesuré sur cette scène, machine de référence, 99ᵉ centile de
   la contribution par image, contre δ ≤ 2 ms GPU ; techniques présentes et absentes publiées.

## 5. Ce que la décision ne fait pas

Elle ne choisit ni la représentation non graphe, ni le `dx` de production, ni le nombre de cycles
— réglages du profil, à calibrer. Elle ne supprime pas la 2D, ne déplace pas les 2 % d'ADR-120,
ne donne à δ aucune autorité de jeu, et n'ajoute aucune dépendance.

Invariants relus : I-04, I-05, I-06, I-08, I-12, I-13, I-17 ; aucun amendé.

## Note datée du 2026-09-24 (S340) — la porte B reçue

Les critères 1 à 3 du §4 sont tenus. **1** : la référence, S295–S298. **2** : la production contre la référence
sur les trois cas de cuve — le 3 en S305, les 1 et 2 en S340, sous 10⁻⁴ m pour 3 mm
([CUVE-GPU-S305](../validation/CUVE-GPU-S305.md) §9). **3** : la revue R16, *« Tout parrait bon visuellement »*
([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §21). Deux écarts au texte du §4, publiés : le cas 1 de la
production tourne sur **le fond de B** — deux composantes opposées —, non sur l'onde analytique en profondeur finie,
que la production ne peut pas évaluer ; le cas 2 reste sous le seuil d'A297, comme la cuve de S305. Le critère 4
relève de la porte C, désormais en cours.

## Note datée du 2026-09-24 (S348) — le critère 4, tenu sur le banc

Le critère 4 du §4 — le coût, mesuré sur la scène de la porte B, 99ᵉ centile de la contribution par image, contre
δ ≤ 2 ms GPU — est tenu sur le banc : **1,92 ms**, à la cadence de 30 Hz d'[ADR-012](ADR-012-ordonnanceur-budget-degradation.md)
§7, un pas coupé en deux parts, une par image ([COUT-DELTA3D-S341](../validation/COUT-DELTA3D-S341.md) §11). La
cadence est validée à l'œil (R17, S348) ; l'interpolation du rendu qu'ADR-012 §7 demande reste à faire, et le tick
découplé de D6 s'y réalise sans l'ordonnanceur, qui ne commande pas encore la cadence.
