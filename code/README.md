# `code/` — le cœur, le harnais et les bancs

Ajouté en S20 ([ADR-020](../docs/adr/ADR-020-bibliotheque-sans-dependance-moteur.md)) : le système d'eau
est une bibliothèque instanciable sans le jeu, **sans aucune dépendance externe**, construite et
testée hors réseau. L'afficheur GPU, qui a ses propres dépendances verrouillées, vit à part dans
[`viewer/`](../viewer/README.md).

## Construire et vérifier

```bash
cd code
cargo test --offline --release --workspace    # cœur et harnais, ≈ 1 min ; décompte au journal
cargo build --offline --release
./target/release/water-harness check scenarios/*.toml
```

La construction se tient à **zéro avertissement** : un avertissement neuf doit se voir
([METHODE](../notes/METHODE.md), protections actives).

## Ce qu'il y a

| emplacement | rôle |
|---|---|
| `water-core/src/` | la bibliothèque — B, W, δ (véhicules 1D d'essai, tranche 2D, référence 3D), V, ordonnanceur ; `#![forbid(unsafe_code)]` |
| `water-core/src/tests_*.rs`, `water-core/tests/` | essais du cœur, nommés par la session qui les a posés |
| `water-core/examples/` | **bancs** — des instruments de mesure, pas le système (voir ci-dessous) |
| `water-core/examples/support/` | modules partagés par plusieurs bancs, inclus par `#[path]` |
| `water-harness/` | harnais H1 et H3 : `check`, `physics`, `bless`, campagnes C22 ([SPEC-003](../docs/validation/SPEC-003-harnais-de-validation.md)) |
| `scenarios/` | scénarios du harnais, C02 et C18 |

## Bancs et système

Un banc est **compilé** par `cargo test` et **exécuté par aucune suite**. Ce qu'il reçoit n'est
donc protégé que par sa preuve : toute preuve ouverte depuis S321 commence par « Reproduire » —
commit, commande, valeurs attendues, durée. Une capacité qui passe d'un banc au système y entre
**avec ses essais** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md) D6). APIC, la seconde
représentation retenue ([ADR-186](../docs/adr/ADR-186-apic-seconde-representation.md)), est encore
dans le banc `lot5_comparaison.rs`.

Lancer un banc :

```bash
cargo run -p water-core --release --offline --example <banc> -- <arguments>
```

`cargo run` recompile et **s'arrête sur une erreur** ; ne jamais exécuter directement un binaire de
`target/` après une compilation dont on n'a pas lu la sortie (L362). Chaque banc dit dans son
en-tête ce qu'il mesure et comment on l'appelle ; chaque preuve dit lequel lancer.

## Deux implémentations du même modèle, et c'est voulu

`delta.rs` et `shallow.rs` résolvent les **mêmes** équations — Saint-Venant 1D, volumes finis, flux
de Rusanov — et ont été écrites **indépendamment**, dans deux lignées du dépôt qui s'ignoraient
([FORK-S22-S26](../docs/registres/FORK-S22-S26.md)). Ce sont des véhicules d'essai, pas le δ du
projet. **Ne pas en supprimer une** : leur désaccord sur un cas sans solution analytique désigne une
faute dans l'une des deux ([ADR-043](../docs/adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §3).

## Bénir une référence

```bash
./target/release/water-harness bless scenarios/C18-invariants.toml
```

`bless` **imprime** le hash, il ne l'écrit pas. Une référence s'inscrit à la main, dans un commit
qui ne contient rien d'autre (ADR-029 §4) : un déplacement de référence doit être un acte visible.

*Les notes par session que ce fichier accumulait de S48 à S83 sont dans Git à `e6371650` ;
l'histoire vit au [journal](../notes/JOURNAL.md).*
