# S68 — Contrats séparés, 2026-09-08

Application de [ADR-052](../adr/ADR-052-separer-phase-et-statistique.md), action S66-1.
La décision et la borne ont été écrites avant lecture des résultats de campagne.

## Mesures et bilan

| Scénario | Score phase (maximum erreur/borne, seuil 1) | Ratio statistique conservé |
|---|---:|---:|
| C02-dispersion | 0,227536 | 0,979548732 |
| C18-invariants | 0,650281 | 1,397506641 |

49 positions et toutes les composantes par scénario, primitive spatiale commune à eval.
Les deux assertions de phase passent. C02 rapporte 12 assertions et 1 diagnostic ; C18,
7 assertions et 1 diagnostic. Les diagnostics ont zéro refus et **aucun verdict statistique**.
Le ratio C18 ne devient pas « homogénéité validée ». Son ancien échec est retiré parce que
son attribution est réfutée, par une décision explicite, sans déplacement de sa mesure.
Le contrôle de précision ne certifie que les arrondis spatiaux sur ces échantillons.

Les hashs restent 0x9babd7e12935c263 et 0xd57d81f47d9f8611 : extraction de la primitive
sans changement d'ordre des opérations. Les références inscrites restent intactes.
Hs reste à +1,388 % au nominal et sous la tolérance existante de 10 %, sans calibration nouvelle.

## Vérification

Test de la borne sur 256 composantes ; défaut injecté gardant seulement quatre bits de phase
spatiale rejeté par la même comparaison. Test de refus : positions vides, NaN, hors rayon,
coefficient non fini. Le rapport teste une statistique finie hors ancien seuil et une mesure
NaN : la première reste diagnostique, la seconde est un refus compté, pas une omission.
Le témoin de précision du scénario C18 passe aussi par le montage complet du harnais.

Reproduction depuis code/ : `cargo test --offline`, puis `cargo build --offline --release`,
`./target/release/water-harness check scenarios/C02-dispersion.toml scenarios/C18-invariants.toml`
et `./target/release/water-harness physics scenarios/C02-dispersion.toml scenarios/C18-invariants.toml`.

La séparation ne fournit aucune barre d'erreur à la statistique. S64-2 doit définir population,
réalisations et règle de validation avant une éventuelle nouvelle tolérance. S63-1 reste le
blocage du banc B2. Aucun modèle 3D ni déterminisme interplateforme validé ici.

Validation finale : **137 tests réussis** (44 cœur + 93 harnais), **cinq ignorés**. Compilation
release réussie, deux scénarios check verts. Physics terminé avec sortie 1 pour le seul échec
C04 ordre un conservé ; tous les autres verdicts physiques inchangés hors séparation décrite.
