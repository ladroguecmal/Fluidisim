# Amorce — à exécuter avant toute autre chose

Ce dépôt est la connaissance projet du **système de gestion de l'eau** d'un jeu de très grande
échelle. Il ne contient pas de code : il contient la conception.

## Ce que tu fais en premier, sans exception

1. **Lis [`REPRISE.md`](REPRISE.md) en entier.** Il contient ton rôle, les règles de travail, la
   carte de la connaissance, l'état du projet, les arbitrages en attente et le rituel de fin de
   session. Il fait foi sur toute mémoire privée et sur tout souvenir de conversation.
2. **Regarde le jeton** en tête de `REPRISE.md`.
   - `libre` → tu le prends, démarrage à froid : suis l'ordre de lecture de `REPRISE.md` §3.
   - `occupé`, battement de moins de deux heures → une autre session travaille. **Ne reprends
     pas.** Signale-le et arrête-toi.
   - `occupé` avec un battement ancien, ou `interrompu` → reprise à chaud : la procédure est en
     tête de [`notes/EN-COURS.md`](notes/EN-COURS.md). Cinq minutes, sans relire le dépôt.
3. **Vérifie l'état réel** avant de croire quoi que ce soit : `git log --oneline -15` et
   `git status --short`. Ce qui est committé est fait ; ce qui est modifié non committé appartient
   à l'étape interrompue.

## Les cinq règles qui ne se négocient pas

- **Français**, réponses courtes et factuelles. Le fond va dans les fichiers, pas dans le message.
- **Markdown uniquement.** Pas de page HTML, pas d'artefact publié.
- **Le plan se déclare avant le travail** dans `notes/EN-COURS.md`, et se commit seul. Une étape
  par commit, message `S<n> P<k> — …`, aucune étape de plus d'un quart d'heure.
- **Un ADR n'est jamais réécrit.** Une erreur factuelle reçoit une note corrective datée ; une
  décision qui change fait l'objet d'un nouvel ADR.
- **Le rituel de fin (`REPRISE.md` §6) est la dernière étape du plan.** Une session qui ne
  l'exécute pas fait perdre l'essentiel de son apport.

## Ce que tu ne décides pas

**Cinq** arbitrages de design, quatre interfaces inter-équipes et sept autres destinataires
extérieurs attendent une réponse humaine. Ils sont listés dans `docs/00_INDEX.md`, section « Ce qui
attend une réponse humaine », et détaillés dans `docs/registres/AUDIT-POINTS-OUVERTS-S11.md` §7.
Les rappeler, ne pas les trancher, ne pas les contourner par une hypothèse implicite.
