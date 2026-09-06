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
| `water-core::body` | flottabilité — `FloatingBox`, cas C10 | ADR-013 |
| `water-core::delta` | **`δ` véhicule d'essai** — Saint-Venant 1D, ordre un | ADR-030, ADR-031 |
| `water-core::shallow` | **seconde implémentation du même modèle**, ordre deux disponible | ADR-038 → ADR-042, ADR-043 |
| `water-core::hash` | hash de conformité, FNV-1a | I-03, cas C18 |
| `water-harness` | l'instrument de mesure — étage **H1** | SPEC-003 §10 |

## Deux implémentations du même modèle, et c'est voulu

`delta.rs` et `shallow.rs` résolvent les **mêmes** équations — Saint-Venant 1D, volumes finis, flux
de Rusanov — et ont été écrites **indépendamment**, dans deux histoires parallèles du dépôt qui
s'ignoraient (registre [`FORK-S22-S26`](../docs/registres/FORK-S22-S26.md)). Aucune des deux n'est
le solveur `δ` du projet : ce choix appartient au banc B3.

**Ne pas en supprimer une.** Leur désaccord sur un cas sans solution analytique désigne une faute
d'implémentation dans l'une des deux — c'est le seul oracle de cette classe dont le projet dispose
(ADR-043 §3). `shallow.rs` porte en outre l'**ordre deux** (MUSCL + RK2), que `delta.rs` n'a pas.

## Ce qu'il n'y a pas

Ni `W`, ni `V`, et pas de `δ` **de production** — seulement deux véhicules d'essai. H1 est le
premier étage du harnais, pas le système. Ce que H1 débloque est
**la CI par commit, dès le premier jour de code** — et sa définition de fin est écrite : la batterie
déterministe tourne en moins de 60 secondes, sans GPU.

## Bénir une référence

```bash
./target/release/water-harness bless scenarios/C18-invariants.toml
```

`bless` **imprime** le hash, il ne l'écrit pas. Une référence s'inscrit à la main, dans un commit
qui ne contient rien d'autre — ADR-029 §4. Le confort d'une réécriture automatique coûterait
exactement la propriété qu'on cherche : qu'un déplacement de référence soit un acte visible.
