# S65 — Graines effectives, 2026-09-08

S64-1 : `scenario.graine` atteint désormais `SeaState.graine`, puis les phases initiales.
Pour chaque indice i, calcul entier modulo 2^64 : z = graine + (i+1) ×
0x9E3779B97F4A7C15, mélange Mix13 (xor/décalages 30,27,31 ; multiplicateurs
0xBF58476D1CE4E5B9 et 0x94D049BB133111EB), puis 32 bits hauts vers PhaseQ32.
Provenance : [OpenJDK, SplittableRandom, mix64 et GOLDEN_GAMMA](https://github.com/openjdk/jdk/blob/master/src/java.base/share/classes/java/util/SplittableRandom.java).
Accès direct, sans état séquentiel, horloge ni allocation dans eval : ADR-003 §2.3.
Aucune graine réservée à un comportement historique. Amplitudes, fréquences et directions inchangées.

## Mesures

Scénario C18, instant nominal, Hs=1,2 m, Tp=6 s, fenêtre 3072 m, pas 3 m.
Écart signé en pourcentage ; six graines fixées avant mesure, pas une calibration de tolérance.

| Graine | 32 composantes | 256 composantes |
|---|---:|---:|
| 0 | −1,284502 | −3,961006 |
| 1 | −0,097098 | −5,905626 |
| 2 | −0,655389 | −0,222290 |
| 3 | −2,434535 | −2,097508 |
| 20260905 | +1,388330 | +5,843568 |
| 18446744073709551615 | −0,078335 | −7,269533 |

Reproduction : `cargo test --offline --release mesurer_hs_sur_six_graines -- --ignored --nocapture`
depuis code/. Diagnostic terminé en 111,12 s, avec une autre campagne en concurrence : aucun
budget déduit de cette durée. L'indépendance statistique des réalisations n'est pas démontrée
par six graines distinctes. La possibilité d'ensembles est acquise ; leurs barres d'erreur et
la tolérance restent à instruire. A187 reste ouvert : les écarts à 256 composantes persistent.

## Références déplacées et échec conservé

Hash C02 : 0x3e2c06a7b00e73e3 → **0x9babd7e12935c263**.
Hash C18 : 0x1a8b0629a9f51b6e → **0xd57d81f47d9f8611**.
Inscrits dans 7723ab8 avant le check qui les juge ; check réussi sur les deux scénarios.
La reproductibilité locale est vérifiée, pas la conformité sur plusieurs plateformes.

C18 Hs : 1,203386 → **1,216660 m**, écart 0,282 → **1,388 %** (tolérance 10 %).
Homogénéité : ratio 1,087389 → **1,397507**, donc **nouvel échec** face à 15 %.
Ce cas compare des variances spatiales d'une réalisation ; sa cause doit être mesurée avant
toute correction. Pas de sélection d'une graine qui passe, ni assouplissement du seuil.
C02 Hs : 1,993891 → 1,991779 ; homogénéité : 1,006044 → 0,979549, toujours réussis.
Les libellés C10 suivent la surface libre modifiée ; les grandeurs hydrostatiques ne changent pas.
Les autres mesures nominales restent identiques, hors temps machine. C04 ordre un reste en échec.

Tests : vecteurs entiers connus, accès dans un ordre différent, bits hauts et extrêmes des graines ;
raccord complet fichier→champ sur six graines, deux constructions identiques par graine,
mer plate identique malgré des graines différentes. Le diagnostic d'ensemble est ignoré par
défaut et a été exécuté explicitement ici. Aucun seuil de validation changé.

Validation finale : 133 tests réussis (41 cœur + 92 harnais), trois ignorés par défaut ; diagnostic des douze réalisations exécuté séparément avec succès.
