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

Session : S207 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **archiver la réponse de l'utilisateur** à l'arbitrage posé en fin de S206 — « GPU,
hôte séparé » — dans une décision (ADR-130), et la propager. Session courte, **aucun code**, aucune
dépendance téléchargée : le téléchargement d'une bibliothèque se demandera nommément quand le lot
de l'hôte commencera.

### État réel à l'amorce

master = copies = 94dd705 (S206 P6), propres ; S206 terminée à 08:49, jeton libre.
Réponse reçue par la question structurée, après le commit de S206 : option (A), recommandée.

### Plan

- [x] **P1** — état réel, plan seul.
- [x] **P2** — ADR-130 : décision de l'utilisateur (chemin de rendu J1 sur GPU, hôte séparé) ;
 conséquences techniques déléguées (hôte hors du workspace sans dépendance, `water-core` inchangé
 sous ADR-020, données publiées vers le GPU selon I-08) ; ce qui reste ouvert (budget GPU de
 l'eau, pile exacte, permission de téléchargement). Notes datées ADR-125 et ADR-020 si touchés.
- [x] **P3** — propagation : feuille de route (§4, J1), file active, REPRISE, index, README.
- [x] **P4** — rituel §6 : journal, décomptes, compteur, jeton libre, copies.

### Notes de reprise

Formulation de la question et des options : S206, FEUILLE-DE-ROUTE §4 avant cette session.

P2 : ADR-130 actée (choix de l'utilisateur : GPU, hôte séparé ; `water-core` sans dépendance
publie ce que le GPU consomme ; hôte hors du workspace sans réseau ; cosmétique ; aucune
dépendance sans autorisation nommée ; budget GPU et pile ouverts). Note datée ADR-125. ADR-020
non touché : il prévoyait déjà `IGpuBackend` fourni par l'hôte.
P3 : feuille de route (J1, §4 tranché + lignes dépendances et budget GPU), file active (chemin
de rendu tranché, S208 = ADR-129 puis hôte GPU), REPRISE, index, README.
P4 : journal S207, suivi A250 ; aucun angle ni leçon nouveaux ; décomptes 130/250/283/18/6/23.
Jeton libre, copies à avancer.
