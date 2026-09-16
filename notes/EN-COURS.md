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

Session : S250 — en cours
Agent : Codex GPT-6 ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée : « Continue ». Master propre f2dd759, une seule copie, archive B conservée.

Objectif : premier raccordement volumique B/W→δ sur le candidat MAC x-z existant.
Le code confirme que les fournisseurs différentiels ne sont pas consommés par Volume.
Lot borné : géométrie et surface imposées, advection perturbative avec termes croisés,
source continue -S et éponge de vitesse avant projection ; durée/budget entiers,
refus atomiques et aucune allocation. La surface mobile nécessite ses résidus de bord :
ne pas lui greffer un résidu volumique en prétendant recevoir le problème complet.

Réception avant code : fond nul retrouve le pas existant à l'arrondi ; force manufacturée
avec signe connu ; advection croisée contre dérivée analytique ; fournisseur réel B et W
planaires traversant Volume ; source contractée après somme ; refus non planaires et
non finis atomiques ; expiration/reprise ; amortissement décroissant, projection reçue.
Référence indépendante sur les accélérations, puis source dense contre source échantillonnée
identique (pas de décimation ni de seuil B4 réinventé). Aucun taux de réflexion promis.
Arrêt : API consommée par le pas MAC et exemple reproductible, tests et limites transmis.

### Plan

- [x] **P1** — amorce, lectures et plan seuls.
- [x] **P2** — ADR/protocole, contrat des faces et limites de la coupe 2D.
- [x] **P3** — pas perturbatif atomique, source et éponge, tests ciblés.
- [>] **P4** — exemple B/W réel, contre-épreuves, suite de tests et mesure.
- [ ] **P5** — rituel §6 : file, trajectoire, preuves, journal, jeton.

### Notes de reprise

Les champs BackgroundSample sont 3D. Une coupe de l'impact radial n'est pas un fond 2D :
sur y=0, uy peut être nul mais d(uy)/dy ne l'est pas. Refuser les échantillons non planaires.
SPEC-004 §6.1 et ADR-114 imposent -S continu, pas l'annulation du pas numérique du fond.
L'équilibre δ=0 n'est exact que si S=0 ; un fond linéaire porte normalement un résidu
quadratique non nul. Ce résidu doit engendrer une correction, pas être effacé.
