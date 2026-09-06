# `code/` — le système d'eau et son harnais

Ajouté en S20. Le dépôt a contenu dix-neuf sessions de conception avant la première ligne de code :
c'est délibéré, et c'est ADR-020 — **un système écrit sans harnais ne se laisse pas instrumenter
ensuite**, donc le harnais vient en premier.

## Construire et vérifier

```bash
cd code
cargo test --offline                      # 18 tests
cargo build --offline --release
./target/release/water-harness check scenarios/*.toml
```

Aucune dépendance externe, ni dans le cœur ni dans le harnais. La construction fonctionne sans
réseau, et c'est une condition de la batterie à chaque commit.

## Ce qu'il y a

| Module | Rôle | Document |
|---|---|---|
| `water-core` | la bibliothèque, **sans dépendance moteur** | ADR-020 |
| `water-core::types` | `SimTime`, `WorldPos` en virgule fixe, `WaterSample` | SPEC-004 §1.1, ADR-028 §3 |
| `water-core::phase` | phases repliées et **sinus déterministe** | ADR-003 §2.2, ADR-029 §2 |
| `water-core::background` | `B` minimal — somme de Gerstner, ordre fixé | ADR-004 *(B1 tranchera)* |
| `water-core::hash` | hash de conformité, FNV-1a | I-03, cas C18 |
| `water-harness` | l'instrument de mesure — étage **H1** | SPEC-003 §10 |

## Ce qu'il n'y a pas

Ni `W`, ni `δ`, ni `V`. H1 est le premier étage du harnais, pas le système. Ce que H1 débloque est
**la CI par commit, dès le premier jour de code** — et sa définition de fin est écrite : la batterie
déterministe tourne en moins de 60 secondes, sans GPU.

## Bénir une référence

```bash
./target/release/water-harness bless scenarios/C18-invariants.toml
```

`bless` **imprime** le hash, il ne l'écrit pas. Une référence s'inscrit à la main, dans un commit
qui ne contient rien d'autre — ADR-029 §4. Le confort d'une réécriture automatique coûterait
exactement la propriété qu'on cherche : qu'un déplacement de référence soit un acte visible.
