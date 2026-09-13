# S209 — Lot de résolution de l'hôte GPU

2026-09-13. Suite de [HOTE-GPU-S208](HOTE-GPU-S208.md), application
d'[ADR-130](../adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md).
**Préparation locale seulement : aucun registre contacté, aucune source téléchargée.**

## Manifeste prêt pour la résolution

À créer dans `viewer/Cargo.toml` après accord de résolution. Versions exactes héritées de
S208 ; leur disponibilité sera constatée par le registre, sans substitution silencieuse.
Les fonctionnalités par défaut restent actives pour cette première résolution portable.

```toml
[workspace]
resolver = "2"
members = ["."]

[package]
name = "water-viewer"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
water-core = { path = "../code/water-core" }
wgpu = "=30.0.1"
winit = "=0.30.13"
pollster = "=1.0.1"

[profile.release]
overflow-checks = true
```

Créer aussi `viewer/src/main.rs`, provisoirement `fn main() {}`, uniquement pour que Cargo
dispose d'une cible lors de la résolution. Ce fichier ne sera pas présenté comme un hôte
construit. Ignorer `viewer/target/`, conserver `viewer/Cargo.lock`. Ne pas lancer ce binaire
vide ; le remplacer par l'initialisation GPU après réception des sources.

## Opération soumise à accord

Depuis la racine du dépôt :

```powershell
cargo generate-lockfile --manifest-path viewer/Cargo.toml --config 'registries.crates-io.protocol="sparse"'
```

Accès à l'index public de crates.io pour résoudre les trois versions ci-dessus et leurs
dépendances transitives. Aucun `fetch`, `build`, `run`, `vendor` ou installation de composant
Rust dans ce lot. Taille de l'index à lire inconnue avant résolution ; les tailles des trois
archives annoncées en S208 ne sont pas la taille de cette opération.

Le `Cargo.lock` peut porter des paquets destinés à plusieurs plateformes. L'inventaire doit
distinguer cet arbre portable des sources effectivement nécessaires à la cible Windows locale.

## Sortie attendue avant l'accord sur les sources

Lire le verrou sans commande qui télécharge les sources et produire un tableau comprenant
**nom, version, source, licence, taille d'archive et checksum** pour chaque paquet du registre.
La licence et la taille ne figurent pas dans le verrou : les lire dans les métadonnées publiques
crates.io, dans le même périmètre de lecture, sans prendre les archives. Toute information
absente reste inconnue. Vérifier la cible locale avec `rustc -vV`.

Présenter la somme des tailles compressées et la liste exacte ; ne pas confondre cette somme
avec l'espace après extraction ou compilation. Toute source Git ou registre autre que crates.io
doit être signalée avant récupération. Demander alors l'accord nommé sur les sources, et le
choix de leur conservation dans le dépôt (vendoring), prévu par S208. Le choix reste ouvert.

## Vérifications locales effectuées

- `cargo 1.97.0` disponible ; aide locale de `generate-lockfile` consultée.
- `cargo metadata --offline --no-deps --format-version 1 --manifest-path code/Cargo.toml`
  réussit : deux membres, `water-core` sans dépendance, harnais dépendant seulement du cœur local.
- Aucun manifeste de `code/` modifié. Aucun test numérique rejoué pour ce dossier documentaire ;
  348 tests réussis et cinq ignorés restent le reçu de S208, pas une nouvelle mesure.

## Lot de construction qui suit les accords

Fenêtre et adaptateur GPU identifiés, puis mer S201 alimentée par les composantes réelles du
cœur et phases repliées (I-08), puis impact avec `RadialTable` au pas λ/16. Recevoir la caméra,
les refus de domaine et la comparaison CPU/GPU avant de mesurer le temps GPU de l'eau.
L'absence d'horodatage GPU doit être annoncée, jamais remplacée par un temps CPU étiqueté GPU.
Le sillage reste dû pour J1 ; cette première tranche ne clôt pas le jalon.

A247/A250 restent ouvertes ; aucun budget, invariant ou périmètre changé. La prochaine
session avance J1 dès l'accord de résolution, puis poursuit sur l'accord des sources.
