# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S293 — **audit global demandé par l'utilisateur**
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : « Reprends le projet, ton objectif est d'analyser le projet et voir où cela bloque dans
l'avancement, regarde l'intégralité de ce projet. » — 2026-09-19 14:19, S292 venant d'être
interrompue par l'utilisateur après P2/P3 (jeton `interrompu`, arbre propre, copie unique).
Objectif : un diagnostic **vérifié** de ce qui empêche l'avancement — technique, décisionnel,
méthodologique, outillage — classé par effet sur les capacités, avec pour chaque blocage sa
preuve, ce qui le lèverait et ce qui relève de l'utilisateur. Livrable :
`docs/registres/BILAN-GLOBAL-S293.md`. **Aucun code modifié, aucune porte ni seuil déplacé,
aucune réduction d'ambition** (ADR-127) : l'audit constate et propose, il ne tranche pas à la
place de l'utilisateur.

### Plan

- [x] **P1** — état réel, jeton, plan seuls.
- [x] **P2** — clore S292 : entrée de journal courte (interrompue, ce qu'elle a établi, ce qui
  reste), A294 actualisée dans la file avec le geste écrit et jamais exécuté comme suite.
- [ ] **P3** — état réel du code : compilation et suites (cœur/harnais, viewer) hors réseau,
  avertissements ; ce que `code/` et `viewer/` contiennent par couche, confronté aux documents.
- [ ] **P4** — intentions → projet fini → jalons : sources, LISTE-PROJET-FINI, portes §3 bis ;
  recompter reçu/partiel/absent si le décompte affiché est périmé.
- [ ] **P5** — trajectoire mesurée : sessions et commits par sujet et par période (Git et
  journal), capacités reçues, maillons, part conception/code ; fils longs et ce qu'ils ont
  débloqué.
- [ ] **P6** — blocages classés (technique mesuré, décision en attente de l'utilisateur,
  méthode/procédure, outillage/infrastructure), avec preuve, effet et levier.
- [ ] **P7** — rédiger `docs/registres/BILAN-GLOBAL-S293.md` et l'indexer.
- [ ] **P8** — rituel §6.

### Notes de reprise

Notes complètes de S292 (tableau froid/chaud, trois mécanismes, commande non exécutée) :
`git show c629ebb:notes/EN-COURS.md`. P2 les reporte dans le journal et dans A294.
