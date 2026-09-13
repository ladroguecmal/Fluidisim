# Fluidisim — Système de gestion de l'eau

Système d'eau temps réel pour un jeu de très grande échelle : fond analytique B, ondes W,
volumique local δ et réseau hydraulique V. **L'ambition complète est conservée** ; les versions
construisent progressivement ces capacités ([feuille de route](docs/FEUILLE-DE-ROUTE.md)).

## Reprendre ou consulter

- [AGENTS.md](AGENTS.md) : amorce unique, vérifications Git et règles de travail.
- [REPRISE.md](REPRISE.md) : passation active et rituel de fin, à lire en entier.
- [Index](docs/00_INDEX.md) : décisions, spécifications et preuves.
- [File active](docs/registres/QUESTIONS-OUVERTES.md#file-active) : travaux présents et déclencheurs.
- [Bilan global S227](docs/registres/BILAN-GLOBAL-S227.md) : alignement, freins et corrections.

## Afficher la mer

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml
```

[Commandes et limites de l'afficheur](viewer/README.md). Il montre B/W ; ses dépendances ont été
autorisées et mises en cache en S211. Sur une machine neuve, leur récupération reste à prévoir.

## Vérifier

```powershell
cargo test --workspace --release --offline --manifest-path code/Cargo.toml
python outils/etat_projet.py
```

Le premier vérifie la suite du cœur/harnais. Le second lit l'inventaire et l'activité Git sans
modifier le dépôt ; il ne mesure ni la productivité ni la couverture des capacités.

## Repères

| emplacement | contenu |
|---|---|
| `code/` | cœur et harnais Rust sans dépendance externe |
| `viewer/` | afficheur GPU séparé, cosmétique |
| `docs/adr/`, `docs/specs/` | décisions conservées et contrats |
| `docs/validation/` | preuves, domaines et protocoles de réception |
| `docs/sources/` | intentions initiales non modifiées |
| `notes/EN-COURS.md` | plan et reprise d'une étape interrompue |
| `notes/JOURNAL.md`, `notes/LECONS.md` | histoire des sessions et enseignements |

Les règles vivent dans AGENTS/REPRISE ; l'état courant dans la feuille de route. Cette page
ne recopie plus les comptes rendus de session (refonte S227 ; version antérieure : `dfd1507`).
