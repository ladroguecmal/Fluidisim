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

Session : S219 — en cours
Agent : Codex (fichiers, git et cargo disponibles)
Entrée : Continue ; master et trois copies propres à4f9841f, jeton libre, maillons0.
Lectures du projet conservées de S218 ; état réel revérifié. Copie principale.

### Thèse et plan

Un tas de rectangles classés par borne permet de raffiner le maximum sans perdre
la couverture. Les enfants héritent aussi de la borne du parent : le maximum ne
peut augmenter. Budget explicite en évaluations, pas promesse de millisecondes.
Pool de l'appelant ; arrêt avant une division si capacité ou budget insuffisants.
Pas de migration d'admission ni de certificat f32 ajouté.

- [x] **P1** — jeton et plan seuls.
- [x] **P2** — construire le parcours borné et recevoir couverture, arrêts et déterminisme.
- [x] **P3** — mesurer gain/coût sur fixtures S218 et publier le contrat et ses limites.
- [ ] **P4** — rituel §6, journal, registres, index, file active, jeton et copies.

### Notes de reprise

Suite S218 : borne locale ADR-135 disponible, réserve non certifiée A258.
Un arrêt retourne la meilleure couverture obtenue ; aucun rectangle ne disparaît.

P2 : tas maximal, division binaire grand côté, borne héritée du parent. Deux tests ciblés reçus debug ; couverture/aire, budget pair, capacité, point, zéro, refus, déterminisme. Reprise inter-appels non construite : chaque appel repart de la racine du champ courant, ce qui évite des bornes périmées. Suite release lancée, reçu à P3.

P3 : 362 tests release passent/5 ignorés. Deux passages isolés base identiques en valeurs ;35,64/35,75s à65535 évaluations. Gain1,48–1,52 sur3 cas recevables ; aucun gain à8191. A259 : bornes grossières plafonnées identiques, priorité spatiale indisponible. Suite proposée S220 : borne de Taylor avec Hessienne signée et reste, phases quantifiées à couvrir. Aucune admission ni reprise inter-appels.
