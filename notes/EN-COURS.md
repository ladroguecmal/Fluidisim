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

Session : S286 — en cours
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : continuer ; priorité A276 après diagnostic spatial borné.
Objectif : découpler expérimentalement le pas δ et la cadence image, et supprimer la dépendance
au nombre d'appels dans le vieillissement des coûts avant cette intégration.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — reproduire puis corriger le vieillissement par appels refusés ; pauses et retours
  de temps éprouvés, historique des vrais pas conservé.
- [x] **P3** — pas Live configurable 16/32/48 ms, comparaison à instants communs et par image,
  coût complet et refus publiés ; ne pas activer par défaut une cadence non reçue.
- [ ] **P4** — tests et réception, intégration expérimentale seulement si critères tenus,
  diagnostic explicite sinon ; découper avant quinze minutes.
- [ ] **P5** — rituel §6 : preuves, journal, file/feuille/index, jeton libre.

### Notes de reprise

Critères : le nombre d'appels au même temps ne change pas l'oubli ; les nouveaux pas réussis
seuls alimentent les mesures. Cadence : compter les vrais pas, zéro allocation, pauses et sauts,
écarts hauteur/pente au témoin 16 ms sur houle et onde 0,6 m, coût moyen par image ET pire pas.
Repère hauteur 3 mm S201 ; ni baisse de fréquence ni coût moyen ne reçoivent I-05 si un pas
bloque encore plus de 2 ms. Évaluer le maintien du dernier profil entre pas avant d'introduire
une interpolation qui pourrait cacher un retard. Fond B toujours au temps de scène (ADR-003).
A290 différée, pas de nouvelle réduction spatiale ; priorité au coût avant 3D/deuxième domaine.

P2 : régression rouge (8 mesures devenaient 7 au même temps), puis verte : oubli par
16 ms non financées, reste conservé, pause et retour sans vieillissement.

P3 : 32/48 ms réduisent la moyenne mais échouent sur onde 0,6 m : 4,445/8,916 mm
aux instants calculés. Moyennes 13,150/9,108 ms, pires pas 32,319/32,708 ms. Secteur
aux deux bornes, zéro allocation. Pas de refus représentatif observé, coût des échecs ouvert.
