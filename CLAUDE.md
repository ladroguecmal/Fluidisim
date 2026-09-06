# Amorce — à exécuter avant toute autre chose

Ce dépôt est la connaissance projet du **système de gestion de l'eau** d'un jeu de très grande
échelle. Il contient d'abord la **conception** ; depuis S20, il contient aussi `code/` — le
harnais de validation et deux δ d'essai, en Rust, sans aucune dépendance.

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
3. **Vérifie l'état réel** avant de croire quoi que ce soit, dans cet ordre :

   ```bash
   git worktree list      # une autre session travaille-t-elle dans un worktree isolé ?
   git branch -a          # existe-t-il une branche parallèle plus avancée ?
   git log --oneline -15
   git status --short
   ```

   **Les deux premières commandes ne sont pas facultatives, et elles passent avant le jeton.** Le
   jeton de `REPRISE.md` est un fichier **versionné** : il est propre à une branche et à une copie
   de travail. Une session travaillant dans un worktree isolé possède son propre jeton, le trouve
   `libre`, et le prend — deux jetons disent alors `occupé` simultanément, chacun dans son univers.

   **Le projet a effectivement forké deux fois par ce mécanisme** — en S07, puis en S21, avec des
   identifiants d'ADR en collision les deux fois. Voir
   [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md). Le second fork s'est produit
   parce que le correctif du premier avait été écrit **dans une seule branche** (**L137**) : ce
   fichier-ci ne l'avait jamais reçu. **Si tu corriges cette procédure, réplique la correction dans
   toutes les branches vivantes le jour même.**

   Ce qui est committé est fait ; ce qui est modifié non committé appartient à l'étape interrompue.

## Les cinq règles qui ne se négocient pas

- **Français**, réponses courtes et factuelles. Le fond va dans les fichiers, pas dans le message.
- **Markdown pour la conception, code pour le code** *(S20)*. La règle d'origine — « Markdown
  uniquement » — visait les artefacts publiés et les pages HTML ; elle reste vraie pour eux. Depuis
  que ADR-020 est acté et que l'utilisateur a autorisé l'ajout de code (S20), le dépôt contient un
  arbre `code/` en Rust. Toujours pas de page HTML, pas d'artefact publié.
- **Le plan se déclare avant le travail** dans `notes/EN-COURS.md`, et se commit seul. Une étape
  par commit, message `S<n> P<k> — …`, aucune étape de plus d'un quart d'heure.
- **Un ADR n'est jamais réécrit.** Une erreur factuelle reçoit une note corrective datée ; une
  décision qui change fait l'objet d'un nouvel ADR.
- **Le rituel de fin (`REPRISE.md` §6) est la dernière étape du plan.** Une session qui ne
  l'exécute pas fait perdre l'essentiel de son apport.

## Ce que tu ne décides pas

**Les cinq arbitrages de design ont été tranchés en S18** ([`ADR-027`](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md)),
sur délégation explicite de l'utilisateur. Ne les rouvre pas sans qu'il le demande ; s'il les rouvre,
ADR-027 dit pour chacun ce qu'il faudrait changer pour l'inverser.

**Ce qui reste hors de ta portée n'est pas de la conception** — ce sont des faits et des actions, et
aucune quantité de raisonnement ne les produit :

1. **Constater l'état réel du projet.** Du terrain a-t-il été sculpté ? Un format réseau
   existe-t-il ? Du code existe-t-il ? L'utilisateur ne le sait pas non plus (S19). L'inférence
   raisonnable — personne d'autre ne travaillant sur le projet, rien n'est figé — est **probable et
   non vérifiée**.
2. **Agir sur l'infrastructure de l'utilisateur** — dépôt distant notamment (`REPRISE.md` §9).
3. ~~**Ajouter du code à ce dépôt.**~~ **Autorisé en S20**, explicitement. Le dépôt contient
   désormais `code/` — Rust, sans aucune dépendance, étages **H1** et **H3** du harnais. La règle
   « Markdown uniquement » reste vraie pour ce qu'elle visait : la **conception** s'écrit en
   Markdown, jamais en page HTML ni en artefact publié. Ne redemande pas cette autorisation.

   *Et une chose que S21 a apprise, qui vaut consigne :* le code n'est pas qu'un livrable, c'est un
   **instrument de mesure de la conception**. Un cas analytique a trouvé en un passage un défaut de
   physique que dix-neuf sessions de conception, six audits et deux revues croisées n'avaient pas
   vu. **Quand une propriété numérique est revendiquée, l'écrire coûte moins cher que la relire.**

*(Ce qui a disparu de cette liste en S19 : « nommer des personnes ». ADR-020 est acté, et il n'y a
personne à nommer pour le reste — ce n'était pas une information manquante, c'était une question
sans objet.)*

**Il n'y a pas d'autres équipes** *(S19)*. Une seule personne travaille sur ce système ; les
développeurs observent. Les quatorze « demandes extérieures » de `docs/DOSSIER-REUNIONS.md` ne sont
donc pas des demandes : ce sont **des décisions à trancher sans interlocuteur**, ou des
spécifications d'un travail dont l'auteur sera l'implémenteur. Le dossier garde sa valeur — il classe
par ce que la réponse débloque — mais **ne reporte rien à une équipe**. Voir
[`ADR-028`](docs/adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) §2.
