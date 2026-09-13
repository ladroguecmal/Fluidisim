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

Session : S221 — en cours
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue » juste après S220, même conversation ; master et trois copies à 32e7afd,
jeton libre, maillons 0. Lectures S220 conservées, état réel revérifié. Copie principale.

### Thèse et plan

A260 : un mode à grande largeur de phase paie `2 c_k` (exclu) ou `D²/2` presque autant (inclu),
**mode par mode**. Toute partition des modes en `R ∪ U` donne `|S(p)| ≤ |S_R(p)| + |S_U(p)|`,
et `|S_U(p)| ≤ G(U)`, l'enveloppe directionnelle ADR-134 du seul sous-ensemble, valable en tout
point. Famille de coupures `U_j = {D_k ≥ D*_j}`, `D* ∈ {2, 1, 1/2}`, accumulée par classes
dans la même passe O(N) ; minimum avec ADR-135 et ADR-136, qui restent **au bit**. Pour la coupure
`D* = 2`, gain garanti non négatif sur les exclus à `2 c_k` : `G(U) + |S_U(c)| ≤ 2 C_U`.

**Limite annoncée avant mesure** : `G(U)` ne dépend pas du point. Une grande cellule loin du
sillage garde `G(U)` si `U` porte l'essentiel de la masse — l'enveloppe ne voit pas la
localisation spatiale du paquet, qui vit dans la cohérence des phases. Hypothèse testée, sans
chiffre : gain à 8191 évaluations seulement si la masse `C_U` de `D ≥ 2` porte la majorité
de `C` sur les feuilles 2 × 1,5 m ; aucun gain attendu à 2047. Critères : sondes sous la borne,
domination au bit sur ADR-136, gain strict sur un rectangle à spectre étalé, S220 inchangée au
bit, décomposition `C_U`/`G(U)` publiée par classe.

- [x] **P1** — jeton et plan seuls.
- [ ] **P2** — ADR-137 : coupure spectrale, famille de seuils, domination, réserve, limite spatiale.
- [ ] **P3** — construire la passe générique (ordre deux inchangé au bit), ordre `Spectral`, Prepared.
- [ ] **P4** — tests : couverture, domination, gain strict, identité S220, refus, partition.
- [ ] **P5** — exemple S221 : décomposition par taille de maille, partitions 2047→32767 ; campagne isolée.
- [ ] **P6** — publier la réception S221 et ses relevés bruts.
- [ ] **P7** — rituel §6, journal, registres, index, file active, jeton et copies.

### Notes de reprise

Suite S220 : ordre deux à 1,006–1,012 × maximum à 32767 évaluations ; plateau à 8191 (A260).
Campagne détachée : signaler la fin par un fichier marqueur, **pas** par `tail -f` d'un journal
écrit par un autre processus (verrou Windows constaté S220).
