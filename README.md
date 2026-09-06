# Fluidisim — Système de gestion de l'eau

Connaissance projet du système d'eau temps réel. Ce dépôt est la **mémoire** du système : il
existe pour que le travail survive au changement de conversation, de session et de personne.

## Entrer

- **Vous reprenez le projet** (autre session, autre compte, autre personne) →
  **[`REPRISE.md`](REPRISE.md)**, à lire en entier avant toute action.
- **Vous cherchez une décision ou une donnée** → [`docs/00_INDEX.md`](docs/00_INDEX.md)

## Organisation

```
CLAUDE.md              amorce : dirige toute session vers REPRISE.md
REPRISE.md             passation : rôle, état, rituel de fin de session, jeton
docs/
  00_INDEX.md          point d'entrée, état d'avancement, arbitrages en attente
  01_INVARIANTS.md     les 17 règles non négociables
  adr/                 décisions d'architecture, numérotées, jamais réécrites
  specs/               références chiffrées et signatures d'interfaces
  registres/           angles morts, traçabilité des questions sources
  validation/          harnais, cas canoniques, plan de benchmark
  sources/             documents d'intention d'origine, non modifiés
notes/
  METHODE.md           protocole de conception
  LECONS.md            enseignements généralisables
  JOURNAL.md           historique des sessions et points de reprise
  EN-COURS.md          plan de la session en cours, déclaré avant le travail
code/                  le harnais et deux δ d'essai — Rust, sans dépendance moteur
```

## Règles de tenue

- **Un ADR n'est jamais réécrit.** Une décision qui change fait l'objet d'un nouvel ADR qui
  remplace explicitement le précédent. L'historique du raisonnement a autant de valeur que la
  conclusion.
- **Aucun nombre sans provenance** : formule citée dans `SPEC-001`, ou étiquette « à calibrer »
  avec le banc qui le fixera.
- **Les documents de `docs/sources/` ne sont pas modifiés.** Leur relecture critique vit dans
  `registres/`.
- Chaque session ajoute une entrée à `notes/JOURNAL.md` avant de se clore, et exécute le rituel de
  fin décrit dans [`REPRISE.md`](REPRISE.md) §6.
- **Une seule session travaille à la fois** : le jeton en tête de `REPRISE.md` en tient le compte,
  avec trois états — `libre`, `occupé`, `interrompu`.
- **Avant de regarder le jeton : `git worktree list` et `git branch -a`.** Le jeton est un fichier
  **versionné** : il est propre à une branche et à une copie de travail, et ne dit rien de ce qui
  se passe ailleurs. Le dépôt a forké **deux fois** par ce mécanisme — voir
  [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md). Ces deux commandes sont le
  seul dispositif qui ait tenu.
- **Le plan se déclare avant le travail**, dans [`notes/EN-COURS.md`](notes/EN-COURS.md), et se
  commit seul. Une étape par commit, message `S<n> P<k> — …`. Une session coupée par une limite
  d'usage n'a aucune occasion d'écrire qu'elle s'arrête : seule une déclaration antérieure survit.
- **Reprendre après une interruption** : la procédure est en tête de `notes/EN-COURS.md`. Cinq
  minutes, sans relire le dépôt.

## Où en est le projet

Voir la section « État d'avancement » de l'index. En une phrase : la conception conceptuelle est
faite, les décisions restantes sont expérimentales, et le chemin critique passe par le harnais de
validation.
