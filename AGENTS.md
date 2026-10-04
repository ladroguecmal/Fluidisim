# Amorce — à exécuter avant toute autre chose

**Ce fichier est l'amorce du dépôt, quel que soit l'agent qui le lit** — Claude, ChatGPT, Codex, un
autre modèle, ou une personne. Il n'existe qu'ici : `CLAUDE.md` n'est qu'un renvoi, et tout autre
fichier d'amorce à venir doit l'être aussi.

> **Pourquoi un seul texte, et pourquoi c'est une règle et non une préférence.** Le dépôt a forké
> **trois fois** parce qu'un correctif de procédure avait été écrit dans une seule branche et pas
> dans l'autre (**L137**). Deux fichiers d'amorce qui se ressemblent aujourd'hui divergeront, et le
> jour où ils divergeront, chaque agent suivra le sien. **Un renvoi d'une ligne, jamais une copie.**

Ce dépôt est la connaissance projet du **système de gestion de l'eau** d'un jeu de très grande
échelle. Il contient d'abord la **conception** ; depuis S20, il contient aussi `code/` — le cœur, le harnais
de validation et des candidats δ, en Rust, sans aucune dépendance.

## Ce que tu fais en premier, sans exception

1. **Lis [`BOUSSOLE.md`](BOUSSOLE.md)** (deux pages, depuis S480) — pourquoi le projet existe, vers quoi il va, les
   décisions en vigueur, ce qui attend l'utilisateur — **puis [`REPRISE.md`](REPRISE.md) en entier.** REPRISE contient ton rôle,
   les règles de travail, la carte de la connaissance, le jeton et le rituel de fin de session. Les deux font foi sur toute
   mémoire privée et sur tout souvenir de conversation.

2. **Vérifie l'état réel** avant de croire quoi que ce soit, dans cet ordre :

   ```bash
   git worktree list      # une autre session travaille-t-elle dans une copie isolée ?
   git branch -a          # existe-t-il une branche parallèle plus avancée ?
   git log --oneline -15
   git status --short
   ```

   `python outils/rituel.py debut` fait ces quatre commandes, lit le jeton et dit ce qu'il commande (S480) ; sans Python, les
   commandes ci-dessus suffisent.

   **Les deux premières commandes ne sont pas facultatives, et elles passent avant le jeton.** Le
   jeton de `REPRISE.md` est un fichier **versionné** : il est propre à une branche et à une copie
   de travail. Une session travaillant dans une copie isolée possède son propre jeton, le trouve
   `libre`, et le prend — deux jetons disent alors `occupé` simultanément, chacun dans son univers.

   **Le projet a forké trois fois par ce mécanisme** — S07, S21, et B-S27 le 2026-09-07, avec des
   identifiants en collision à chaque fois. Voir
   [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md).

   Ce qui est committé est fait ; ce qui est modifié non committé appartient à l'étape interrompue.

3. **Regarde le jeton** en tête de `REPRISE.md`.
   - `libre` → tu le prends, démarrage à froid : suis l'ordre de lecture de `REPRISE.md` §3.
   - `occupé`, battement de moins de deux heures → une autre session travaille. **Ne reprends
     pas.** Signale-le et arrête-toi.
   - `occupé` avec un battement ancien, ou `interrompu` → reprise à chaud : la procédure est en
     tête de [`notes/EN-COURS.md`](notes/EN-COURS.md). Cinq minutes, sans relire le dépôt.
   - `archivé` → **cette branche est une archive**. Ne travaille pas dessus ; rejoins la branche que
     le jeton désigne.

   **En prenant le jeton, écris qui tu es** sur la ligne `Agent`. Ce n'est pas une formalité : la
   session suivante doit savoir à quoi s'attendre — quels outils étaient disponibles, et où aller
   chercher si quelque chose manque.

## Ce que tu fais en dernier, si tu travailles dans une copie isolée

**Une copie de travail se ferme.** ADR-110, sur demande de l'utilisateur, après que S159 eut trouvé
**six** copies ouvertes dont quatre annonçaient un jeton `libre` avec quatre « dernière session »
différentes — S158, S157, S146 et **S44**. Une session ouvrant la dernière aurait commencé S45.

Après le rituel de fin, et depuis la copie où tu as travaillé :

```bash
git -C <racine> merge --ff-only <ta-branche>   # ce que tu fais déjà
git worktree remove <ton-répertoire>           # ce que les sessions oubliaient
git branch -d <ta-branche>                     # -d, jamais -D
```

`-d` refuse de supprimer une branche portant un commit unique. **C'est le garde-fou, ne le
contourne pas** : si `-d` refuse, ta branche contient quelque chose que `master` n'a pas, et c'est
une fusion qui manque, pas une suppression qui résiste.

Deux exceptions, et deux seulement.

- **Une branche portant des commits uniques se conserve** — mais pas sa copie de travail. Ce qui
  invite à travailler dans une lignée morte est le répertoire, pas la référence. C'est le cas de
  `claude/reprise-projet-5134cd`, lignée B, 44 commits, jeton `archivé`.
- **Une copie que tu ne peux pas prouver morte se met à jour, elle ne se supprime pas.** Une
  avance rapide éteint son jeton périmé sans rien détruire. C'est le premier geste à faire, avant
  toute suppression, et il suffit à lui seul à supprimer le danger.

## Si tu n'es pas Claude Code

**Rien de ce qui précède ne change.** Le dispositif ne repose sur aucune fonctionnalité propre à un
fournisseur : il repose sur des fichiers versionnés et sur `git`. Trois précisions, tout de même.

### Ce qu'il te faut

| | pourquoi |
|---|---|
| **lire et écrire les fichiers** du dépôt | le travail *est* les fichiers |
| **exécuter `git`** | l'amorce, les commits d'étape, la détection de fork |
| **exécuter `cargo`** *(souhaitable)* | vérifier que `code/` compile et que la suite de tests passe *(décompte au journal, pas ici : il se périme — S203)* |

**Si l'un des trois te manque, dis-le dans ton premier message et n'ouvre pas de session.** Une
session qui ne peut pas committer ses étapes ne laisse aucune trace d'intention, et c'est
précisément ce contre quoi `notes/EN-COURS.md` existe. Tu peux en revanche **lire** le dépôt et
répondre à des questions : cela ne demande que la première ligne du tableau.

### Le nom du fichier d'amorce

Chaque outil lit automatiquement un fichier différent — `CLAUDE.md`, `AGENTS.md`, parfois un autre.
**Le texte vit ici, dans `AGENTS.md`.** Les autres sont des renvois d'une ligne.

Si ton outil en lit un troisième qui n'existe pas encore, **crée-le comme un renvoi**, jamais comme
une copie, et dis-le dans le journal. Le jour où deux amorces divergent, chaque agent suit la
sienne, et le dépôt reforke — il l'a déjà fait trois fois pour cette raison exacte.

### Ce que le changement d'agent ne protège pas

**Deux agents différents se marchent dessus exactement comme deux sessions du même agent.** Le jeton
ne distingue pas les fournisseurs, et il n'a pas à le faire : il distingue les **copies de travail**.
Un agent qui ouvre une copie isolée reforke, quel que soit son nom.

## Les cinq règles qui ne se négocient pas

- **Français**, réponses courtes et factuelles. Le fond va dans les fichiers, pas dans le message.
- **Markdown pour la conception, code pour le code** *(S20)*. La règle d'origine — « Markdown
  uniquement » — visait les artefacts publiés et les pages HTML ; elle reste vraie pour eux. Depuis
  que ADR-020 est acté et que l'utilisateur a autorisé l'ajout de code (S20), le dépôt contient un
  arbre `code/` en Rust. Toujours pas de page HTML, pas d'artefact publié.
  **Exception explicitement autorisée S201 (2026-09-13)** : un exemple peut écrire des
  images locales de banc (PPM et preview), sans publication ; voir
  [ADR-124](docs/adr/ADR-124-image-budget-et-effets-bornes.md). Ne pas redemander cet accord.
- **Le plan se déclare avant le travail** dans `notes/EN-COURS.md`, et se commit seul. Une étape
  par commit, message `S<n> P<k> — …`, aucune étape de plus d'un quart d'heure.
- **Un ADR n'est jamais réécrit.** Une erreur factuelle reçoit une note corrective datée ; une
  décision qui change fait l'objet d'un nouvel ADR.
- **Le rituel de fin (`REPRISE.md` §6) est la dernière étape du plan.** Une session qui ne
  l'exécute pas fait perdre l'essentiel de son apport.

## Ce que tu ne décides pas

**L'ambition finale est complète, et une session ne la réduit pas** *(S204,
[`ADR-127`](docs/adr/ADR-127-ambition-complete-construction-progressive.md))*. δ général, V,
inondations complexes et grande échelle sont obligatoires ; la construction progresse par
versions de plus en plus capables, selon [`FEUILLE-DE-ROUTE`](docs/FEUILLE-DE-ROUTE.md). Un
ordre, un budget ou une priorité ne retirent rien : ADR-124 a été lu ainsi pendant trois
sessions, et l'utilisateur a dû le corriger. Une incompatibilité mesurée se remonte comme
arbitrage explicite.

**Les cinq arbitrages de design ont été tranchés en S18** ([`ADR-027`](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md)),
sur délégation explicite de l'utilisateur. Ne les rouvre pas sans qu'il le demande ; s'il les rouvre,
ADR-027 dit pour chacun ce qu'il faudrait changer pour l'inverser.

**Ce qui reste hors de ta portée n'est pas de la conception** — ce sont des faits et des actions, et
aucune quantité de raisonnement ne les produit :

1. **Constater l'état réel du projet.** Du terrain a-t-il été sculpté ? Un format réseau
   existe-t-il ? L'utilisateur ne le sait pas non plus (S19). L'inférence raisonnable — personne
   d'autre ne travaillant sur le projet, rien n'est figé — est **probable et non vérifiée**.
2. **Agir sur l'infrastructure de l'utilisateur** — dépôt distant notamment (`REPRISE.md` §9).
3. ~~**Ajouter du code à ce dépôt.**~~ **Autorisé en S20**, explicitement. Le dépôt contient
   désormais `code/` — Rust, sans aucune dépendance, étages **H1** et **H3** du harnais. La règle
   « Markdown uniquement » reste vraie pour ce qu'elle visait : la **conception** s'écrit en
   Markdown, jamais en page HTML ni en artefact publié. Ne redemande pas cette autorisation.

   *Et une chose que S21 a apprise, qui vaut consigne :* le code n'est pas qu'un livrable, c'est un
   **instrument de mesure de la conception**. Un cas analytique a trouvé en un passage un défaut de
   physique que dix-neuf sessions de conception, six audits et deux revues croisées n'avaient pas
   vu. **Quand une propriété numérique est revendiquée, l'écrire coûte moins cher que la relire.**

**Il n'y a pas d'autres équipes** *(S19)*. Une seule personne travaille sur ce système ; les
développeurs observent. Les quatorze « demandes extérieures » de `docs/DOSSIER-REUNIONS.md` ne sont
donc pas des demandes : ce sont **des décisions à trancher sans interlocuteur**, ou des
spécifications d'un travail dont l'auteur sera l'implémenteur. Le dossier garde sa valeur — il classe
par ce que la réponse débloque — mais **ne reporte rien à une équipe**. Voir
[`ADR-028`](docs/adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) §2.
