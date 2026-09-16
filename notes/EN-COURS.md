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

Session : S249 — en cours
Agent : Codex, GPT-6 (fichiers, git, cargo, accès web ; GPU à vérifier)
Entrée : « Reprends le projet », puis « Continue ». Master propre à 12bd90f, une seule
copie ; branche B archivée conservée. Jeton pris le 2026-09-16 à 17:23 +02:00.

Objectif : A282, filtrage spectral cosmétique selon le pas projeté, consommé par l'afficheur.
Priorité : après deux maillons d'instrumentation, construire la capacité désormais spécifiée.
La multigrille A281 reste un approfondissement au gain incertain ; V et δ gardent leurs
lots et déclencheurs. Aucun changement d'autorité, de recette répliquée ou de seuil physique.

Réception : poids continus dans [0,1], nuls à Nyquist et au-delà ; coefficients proches
inchangés ; GPU comparé à une somme CPU filtrée indépendante sous les 3 mm de S201,
écart volontaire au cœur publié séparément ; poses référence/rasante/haute et deux formats,
retour de caméra, coût avec/sans et allocations. Les impacts tabulés n'ont pas de modes
exposés : leur filtrage est hors de ce premier lot B/sillage et sera porté explicitement en file.
Arrêt : chemin activé dans l'hôte, critères éprouvés, limites et coûts transmis.

### Plan

- [x] **P1** — amorce, lectures ciblées, jeton et plan seuls.
- [>] **P2** — décision et protocole : empreinte de projection, poids et bandes du sillage.
- [ ] **P3** — filtre CPU/GPU et intégration au rendu, tests du contrat.
- [ ] **P4** — réception GPU, comparaison au cœur et au témoin, coût et allocations.
- [ ] **P5** — rituel §6 : preuves, file active, trajectoire, journal et jeton.

### Notes de reprise

A282 touche l'image seulement (ADR-130). La grille locale S234 contient une réalisation
sommée : on ne peut donc pas lui retirer un mode après coup. Prévoir des bandes modales
séparées dès sa cuisson, à phases identiques ; la caméra pondère les amplitudes des bandes.
Le fond (32 modes) se filtre directement. La recette et ses domaines ADR-132 restent intacts.