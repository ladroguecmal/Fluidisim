# Coût B+W S81 — 2026-09-08

Commande : `cargo run --release --manifest-path code/Cargo.toml -p water-core --example bench_water`.
Poste local Windows, AMD Ryzen AI 7 350, rustc 1.97.0. Trois échauffements puis 21 mesures,
p50/p90/max par tri ; aucune promesse de percentile de production ni budget matériel cible.
Préparation séparée ; lot chronométré incluant Background::eval réel (32 composantes),
composition et copie finale. Points sur grille 0,25 m ; sources identiques superposées,
0,01 J chacune, lambda 4 m, N64, profondeur 20 m, t=1 s. Limite de pente d’essai 1.
Le test de charge multisource ne constitue pas une nouvelle réception physique de ce profil.

## Mesures avant/après

Latence médiane du lot en millisecondes. Runs successifs sur machine non isolée : bruit possible.

| Sources | Points | Avant | Après | Réduction |
|---|---|---|---|---|
| 1 | 16 | 4,2232 | 2,3146 | 45,2 % |
| 1 | 64 | 17,3492 | 9,9559 | 42,6 % |
| 4 | 64 | 68,8204 | 40,2427 | 41,5 % |
| 16 | 64 | 277,8773 | 159,1579 | 42,7 % |

Après optimisation, p90/max pour 1×64 : 10,4954/10,8724 ms ; pour 16×64 :
163,7309/206,9217 ms. B seul : environ 57 µs pour 64 points. Préparation après :
0,6 µs pour une source, 2,5 pour quatre, 9,8 pour seize (granularité horloge significative).
Microbanc 1024 Bessel : médiane 4743,4→2949,4 µs. L’évaluation W domine, pas la préparation.
Même optimisé, le chemin n’est pas reçu pour une charge AAA.

Mémoire utile des pools : 1632 octets par Option<RadialImpact<64>> ; 26112 pour seize.
Bases/sortie/temporaire/points/coordonnées monde : 9728 octets pour 64 points. Ces chiffres
excluent journal, B, capacités Vec de mesure, pile et tables statiques. Table ajoutée : 512 octets.
Les allocations du banc sont faites hors chronométrage ; ce n’est pas une mesure exhaustive
par allocateur global. Les chemins de production restent sans allocation explicite.

## Optimisation exacte

Les 128 valeurs PhaseQ32(i<<25).cos() étaient recalculées pour chaque nœud et chaque point.
Elles sont désormais stockées en bits f32 explicites. Générateur : bench_water --angles.
Un test compare chaque entrée aux bits du calcul original ; ordre de sommation inchangé.
Aucun seuil, spectre ou nombre de directions modifié. Une nouvelle ADR physique serait inutile :
la représentation calculée reste identique, il s’agit d’une optimisation de son évaluation.

Empreintes de tous les champs WaterSample des sorties identiques avant/après :
1×16 : 9a5f18b79619d78b ; 1×64 : dbe2d45f541a3948 ;
4×64 : 34c2667e33731c3f ; 16×64 : 5468cade51d40647.
Les hashs sont des contrôles locaux de régression, pas une preuve interplateforme universelle.

## Suite

S80-1 réalisée. Prochain poste : le calcul Bessel à 128 directions par nœud. S82 doit construire
un candidat d’approximation accélérée avec erreur explicitement bornée et références existantes,
le comparer sur ce même banc, sans remplacer le chemin exact avant réception. Ne pas déduire
une capacité maximale de production des seules médianes locales. Budget et profils restent ouverts.
