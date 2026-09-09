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

Session : S126 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S125-1 / A203 — recevoir sept composantes du champ étendu contre un oracle
f64 indépendant, en contrôlant séparément convergence spectrale et angulaire.

### Plan

- [x] **P1** — état réel, jeton et plan seuls ; continuité de S125 sur master propre.
- [x] **P2** — construire une campagne release indépendante, spectre physique ADR-060,
      Bessel par intégrale angulaire f64 sans table ni PhaseQ32, raffinements séparés.
      Prévoir centre, rayons irréguliers, raccord Bessel, bord spatial et âge limite.
- [ ] **P3** — exécuter et interpréter ; conserver tout refus. Rapport avec domaine exact,
      provenance des critères, contrôles de l'oracle et contre-épreuve du verdict.
      Corriger le candidat seulement si une erreur est établie (sinon bibliothèque inchangée).
- [ ] **P4** — vérifications adaptées, journal, angles/leçons/actions, index et passation,
      décomptes, jeton libre et commit de fin.

### Notes de reprise

Départ c7cb140, master seul avancé ; anciennes copies propres en retard, aucune copie créée.
REPRISE et règles déjà lus dans cette conversation, continuité immédiate. S125 complet.
Fixtures à recevoir : λ4, E0,01 J, g9,81/rho1025/h20, durée4 s ; N128/R64 et N256/R128.
Critère annoncé AVANT mesure : erreur absolue de chaque composante divisée par son
échelle naturelle (intégrale des poids positifs correspondants) <=1e-4. Reprend le seuil
normalisé d'élévation ADR-060, étendu aux autres unités comme critère de banc, à calibrer B2.
Oracle : raffinements spectraux 512/1024/2048 ; angulaire 1024/2048 ; écarts normalisés
<=1e-6 (1 % du seuil de réception). Pas de division par la valeur locale près des zéros.
Référence centre initial analytique ; zéro initial des vitesses ; défaut de signe volontaire
sur vitesse pour vérifier que le verdict peut refuser. Aucune promesse de borne continue.

P2 : receive_extended_impact.rs construit, compilation release réussie. Intégrales
angulaires indépendantes et trois maillages spectraux. Centre analytique reçu à 4,64e-14.
Première exécution lancée ; recevoir les résultats en P3 (processus 68746).
