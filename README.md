# Fluidisim — Système de gestion de l'eau

Connaissance projet du système d'eau temps réel. Ce dépôt est la **mémoire** du système : il
existe pour que le travail survive au changement de conversation, de session et de personne.

## Entrer

- **Vous reprenez le projet** (autre session, autre compte, **autre agent**, autre personne) →
  **[`REPRISE.md`](REPRISE.md)**, à lire en entier avant toute action.
- **Vous cherchez une décision ou une donnée** → [`docs/00_INDEX.md`](docs/00_INDEX.md)

## Organisation

```
AGENTS.md              **l'amorce** — le seul texte, quel que soit l'agent qui lit
CLAUDE.md              renvoi d'une ligne vers AGENTS.md (Claude Code lit ce nom-là)
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
  avec quatre états — `libre`, `occupé`, `interrompu`, `archivé`.
- **Une seule amorce, [`AGENTS.md`](AGENTS.md)**, quel que soit l'agent — Claude, ChatGPT, Codex,
  un autre modèle, ou une personne. Les autres noms de fichier d'amorce sont des **renvois d'une
  ligne**, jamais des copies : deux amorces qui se ressemblent divergeront, et chaque agent suivra
  alors la sienne (**L137**).
- **Avant de regarder le jeton : `git worktree list` et `git branch -a`.** Le jeton est un fichier
  **versionné** : il est propre à une branche et à une copie de travail, et ne dit rien de ce qui
  se passe ailleurs. Le dépôt a forké **trois fois** par ce mécanisme — voir
  [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md). Ces deux commandes sont le
  seul dispositif qui ait tenu.
- **Le plan se déclare avant le travail**, dans [`notes/EN-COURS.md`](notes/EN-COURS.md), et se
  commit seul. Une étape par commit, message `S<n> P<k> — …`. Une session coupée par une limite
  d'usage n'a aucune occasion d'écrire qu'elle s'arrête : seule une déclaration antérieure survit.
- **Reprendre après une interruption** : la procédure est en tête de `notes/EN-COURS.md`. Cinq
  minutes, sans relire le dépôt.

## Où en est le projet

État S116 : la construction est actée par ADR-053. Le noyau B+W dispose d'un journal rejouable,
d'impacts radiaux et de requêtes communes en lot. Le renouvellement numérique est testé
jusqu'à 16 secondes sur un scénario borné ; rétention durable et système complet restent à
construire. Le contrôleur à deux pools assure désormais la bascule après succès et signale
l'expiration. LiveWater admet désormais les commandes en publiant ensemble journal et champs ;
une commande bloquée reste visible. La sauvegarde WLIV conserve cette attente et permet une
reprise sur des buffers plus grands. Le scénario hôte complet est testé et mesuré : 3 impacts
sur 64 points en 0,64 ms médiane locale. Une première réponse physique à une pression mobile
est construite hors runtime, puis étendue à un profil gaussien localisé avec bilan global vérifié.
Les trajectoires avec virage conservent désormais les interférences et le bilan de travail.
Une enveloppe refuse les requêtes hors bornes ; le virage est comparé par raffinement sur
[-8,12]² m et 0–8 s, sans garantie continue. La préparation dispose d'un chemin sur pool hôte.
Potentiel, pente et vitesses sont vérifiés. Le candidat modal à phases entières passe la
réception locale : 243 tests réussis, cinq ignorés ; conformité interplateforme encore ouverte.
La superposition sur pool passe le virage et le découpage, pour toutes les grandeurs.
La cuisson gaussienne possède une recette versionnée, sur pool et sans libm.
Le demi-spectre et les coefficients préparés coûtent 19,800 ms pour 64 points, médiane locale.
Les lots publient après succès intégral ; le parcours par tuiles, plus lent, est rejeté.
Sinus/cosinus partagent leur réduction avec identité locale, sans gain global établi.
Une résolution112×80 passe le virage densifié : lot64 à11,305 ms médian local, usage limité à cette fixture.
Puissance candidate reçue : bilans spatial et temporel, extinction, aux deux résolutions (S104).
Le candidat lie désormais ses requêtes au contexte complet et à son instant préparé.
Une préparation refusée conserve la publication précédente (S105).
Une requête monde commune compose désormais B et la pression, avec contrôle de pente et lot atomique (S106).
Cycle B+pression reçu sur le virage : médianes locales29,37 ms (128²) et16,28 ms (112×80), sans budget cible certifié.
Source de pression immuable et codec WPRS V1 construits (ADR-074), virage196 octets.
Journal de pression construit : doublons, conflits, attente en saturation et reprise explicite (ADR-075).
Instantané mémoire WPJR et restauration construits, attente conservée (ADR-076).
Reprise complète reçue :1152 points-temps identiques en bits, cycles locaux28,4/17,0 ms (S111).
Champ multisource construit, interférences et travail total conservés (S112).
Deux sources reçues contre référence f64 raffinée :224×128 et256×128 (S113).
Préparation+requête pression64 :48,29/56,13 ms médianes locales ; B et codecs exclus.
Reprise multisource reçue :2944 points-temps identiques en bits, refus transactionnels (S114).
Cycle depuis WPRS jusqu’à B+pression :47,95/55,55 ms médians locaux, cuisson exclue.
Requête commune B+impacts+pressions construite : B unique, normale issue des pentes totales,
refus transactionnels et réductions aux chemins antérieurs reçus (ADR-077, S115).
Montage mixte reçu sur8670 points-temps du modèle profond, cycles50,05/57,97 ms (S116).
Suite : contrôleur de publication temporelle du champ de pression, S116-1.
Voir [l'index](docs/00_INDEX.md) et [la passation](REPRISE.md) pour l'état détaillé.
