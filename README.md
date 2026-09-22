# Fluidisim — Système de gestion de l'eau

Système d'eau temps réel pour un jeu de très grande échelle : fond analytique B, ondes W,
volumique local δ et réseau hydraulique V. **L'ambition complète est conservée** ; les versions
construisent progressivement ces capacités ([feuille de route](docs/FEUILLE-DE-ROUTE.md)).

## Reprendre ou consulter

- [AGENTS.md](AGENTS.md) : amorce unique, vérifications Git et règles de travail.
- [REPRISE.md](REPRISE.md) : passation active et rituel de fin, à lire en entier.
- [Index](docs/00_INDEX.md) : décisions, spécifications et preuves.
- [File active](docs/registres/QUESTIONS-OUVERTES.md#file-active) : travaux présents et déclencheurs.
- [Bilan global S321](docs/registres/BILAN-GLOBAL-S321.md) : code, documents et méthode ; la méthode refondue ([ADR-187](docs/adr/ADR-187-methode-refondue-s321.md)).

## Afficher la mer

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml
```

[Commandes et limites de l'afficheur](viewer/README.md). Il montre B/W, et δ avec `--delta3d` ; ses dépendances ont été
autorisées et mises en cache en S211. Sur une machine neuve, leur récupération reste à prévoir.

## Vérifier

```powershell
cargo test --workspace --release --offline --manifest-path code/Cargo.toml
cargo test --release --offline --locked --manifest-path viewer/Cargo.toml
python -m pytest -q outils
python outils/etat_projet.py --check
```

Les deux premiers vérifient le cœur, le harnais et l'afficheur ; le troisième, les outils Python.
Le dernier lit l'inventaire et l'activité Git sans modifier le dépôt, et tient les contrôles de la
méthode — navigation, plafonds, battement, `EN-COURS`, encodage, fichiers produits, décompte de la
liste, preuves reproductibles. Il ne mesure ni la productivité ni la couverture des capacités.

## Repères

| emplacement | contenu |
|---|---|
| `code/` | cœur et harnais Rust sans dépendance externe |
| `viewer/` | afficheur GPU séparé, cosmétique |
| `docs/adr/`, `docs/specs/` | décisions conservées et contrats |
| `docs/validation/` | preuves, domaines et protocoles de réception |
| `docs/sources/` | intentions initiales non modifiées |
| `notes/EN-COURS.md` | plan et notes de la session en cours ; reprise d'une étape interrompue |
| `notes/METHODE.md` | méthode, et ses protections actives |
| `notes/JOURNAL.md`, `notes/LECONS.md` | histoire des sessions ; archive des leçons |

Les règles vivent dans AGENTS/REPRISE ; l'état courant dans la feuille de route. Cette page
ne recopie plus les comptes rendus de session (refonte S227 ; version antérieure : `dfd1507`).
