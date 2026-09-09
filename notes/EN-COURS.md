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

Session : S113 — terminée
Agent : Codex (fichiers, git et cargo disponibles)
Objectif : référence f64 indépendante et réception spatiale du montage multisource S112.

### Plan

- [x] **P1** — vérifier la passation et déclarer le plan.
- [x] **P2** — construire la référence, comparer les résolutions et mesurer le coût reçu.
- [x] **P3** — publier résultats et limites ; rituel final et synchronisation.

### Notes de reprise

Départ31a9e2b, copies actives propres identiques. S112-1.
Oracle f64 à quadrature complète, somme modale avant énergie et puissance.
Seuils spatiaux S103 ; puissance reçue séparément avec seuil déclaré et convergence de référence.
Bibliothèque inchangée prévue ; assertions exécutables dans un exemple de réception.


P2 en cours : premier balayage128/256 refuse toutes les résolutions au seuil puissance1e-7 W.
Écart des références5,8441e-7 W : comparer à128 empêche toute réception honnête à ce seuil.
Références raffinées256/512 en cours.112×80 échoue déjà sur potentiel1,1256e-5 >1e-5.
Les essais et sorties sont dans code/target/s113-reception.log et s113-dense.log.

P2 terminée : références256/512 reçues ; neuf recettes normales et cinq denses, assertions OK.
224×128 et256×128 passent10143 points-temps chacun ; puissance max6,1785e-8/3,5708e-8 W.
Cycles pression préparation+64 points médians48,2895/56,1288 ms. B/codecs exclus.
Rapport RECEPTION-MULTISOURCE-S113 ; bibliothèque inchangée, suite S112 non relancée.

P2 a4917db. Publication, questions et passation synchronisées ; aucun nouvel angle/leçon distincte.
Suite S114 : S113-1, reprise multisource WPJR vers B+pression aux recettes224×128/256×128.
