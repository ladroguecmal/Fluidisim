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

Session : S217 — en cours
Agent : Codex (fichiers, git et cargo disponibles)
Objectif : instruire la part dynamique d'A255, file J1, sans présumer une loi universelle.

### Entrée et état réel

Reprise demandée par l'utilisateur. Master et trois copies propres à b1860c1 ; branche B
archivée conservée. Jeton libre à l'entrée ; maillons 0. Travail dans la copie principale.
Aucune dépendance nouvelle. Lecture d'amorce effectuée ; approfondissement ci-dessous.

### Thèse et critères avant mesure

Après extinction, l'échelle sqrt(sigma/g) peut réduire le temps seulement à géométrie
adimensionnée constante : vitesse/sqrt(g sigma), durée/sqrt(sigma/g), cutoff*sigma et
trajectoire/sigma restent des paramètres indépendants. Une similitude n'est pas une loi
universelle sur des histoires de forçage différentes. Comparer d'abord des homothéties,
puis faire varier vitesse et durée séparément. Pendant le forçage : série distincte.
Les maxima échantillonnés sont des minorants ; aucune table de sûreté ne sera déduite d'un
seul balayage. Raffiner recherche spatiale et quadrature avant d'interpréter un écart.

### Plan

- [x] **P1** — jeton, thèse et plan seuls.
- [x] **P2** — lectures ciblées, dérivation des groupes et protocole de campagne reproductible.
- [x] **P3** — instrument et campagne : similitudes, variations indépendantes, contrôles de résolution et forçage.
- [x] **P4** — publier le verdict et ses limites ; construire seulement ce que les preuves autorisent.
- [ ] **P5** — rituel de fin §6, file plurielle, journal, index, jeton libre et copies synchronisées.

### Notes de reprise

Les reçus S216 restent ceux de la session précédente : 356 tests réussis et cinq ignorés.
P3 : 54 mesures initiales et neuf contrôles. À tau=4, 256×512/pas0,5 :
base1,637275, lent2,272143, long2,005464 ; écart de maximum128→256 <0,002 %.
Homothéties conservées à ~1e-6 du rapport imprimé, charge et découpage aussi.
Reconstruction/cœur max2,383e-6, sous1e-5. Deux tests d'instrument réussis en release.
La réponse libre mélange eta et vitesse : son module eta n'est pas invariant ;
test mono-mode reçu avec énergie conservée. Correction documentaire nécessaire en P4.
La suite workspace est encore en cours (sortie code/target/s217-workspace-tests.log).
P4 : verdict publié, notes correctives ADR-133 et reçus S215/S216. Aucun changement
src ni nouvel ADR : la courbe à un âge est réfutée, une borne locale avec reste spatial
est la suite de construction proposée. Deux tests d'exemple debug/release passent.
Suite workspace toujours en cours ; ne pas recopier356 comme résultat S217 avant sa fin.