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

Session : S146 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **lancer B1** — « champ de fond : nombre de composantes et coût d'évaluation ».
Recommandé par [BILAN-S69](../docs/registres/BILAN-S69.md) puis par
[BILAN-S145](../docs/registres/BILAN-S145.md), jamais fait. **Zéro banc sur onze exécuté** depuis
le premier jour du projet ; celui-ci ne demande aucune couche manquante.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — lire le protocole §B1 **en entier** et dire ce qui est exécutable aujourd'hui et ce
      qui ne l'est pas. Le protocole demande une évaluation **subjective en double aveugle** et
      une distance de perception : hors de portée d'une session (`REPRISE.md` §5). Et vérifier si
      des morceaux de B1 ont déjà été mesurés sans être reconnus comme tels — les `bench_*`
      existent, et S145 vient de montrer que le dépôt fait des choses sans les déclarer.
- [x] **P3** — volet **coût** : coût par échantillon pour N ∈ {32, 64, 128, 256}, à ordre de
      sommation fixé, et ce que la troncature coûte.
- [x] **P4** — volet **justesse**, celui qui rend le banc urgent : `Hs` mesuré contre la cible en
      fonction de N. A187 dit 6,6 % d'écart à 256 composantes par battements ; A188 dit que la
      calibration statistique reste à faire. C'est ici qu'on tranche.
- [x] **P5** — la **courbe demandée** : combien de composantes effectives pour qu'un objet de
      taille L voie la même hauteur. La partie « taille d'objet » est mesurable ; la partie
      « distance » ne l'est pas sans caméra — le dire au lieu de la contourner.
- [ ] **P6** — **décision** : N retenu, ce que la décision porte et ce qu'elle ne porte pas. ADR,
      rapport de banc, rituel de fin (§6, **dont le point 7 ajouté hier**), jeton, `--ff-only`.

### Notes de reprise

Départ 533c7ed = master ; worktree `886155`. 275 tests/cinq ignorés, 98 ADR, 211 angles.

Ce qui est établi avant de commencer :
- `background.rs` (587 lignes) est une somme de composantes de **Gerstner**, à ordre de sommation
  fixé, sans allocation à l'exécution (I-06) ; `SeaState { hs, components, … }` et
  `Background::configure` répartissent les composantes géométriquement autour de la période de
  pic, amplitude `a = Hs/(2√(2n))` ;
- **il n'y a pas de LOD spectral** dans le code. Le protocole demande « le coût avec LOD actif et
  inactif » : ce volet-là n'est pas exécutable, et pas parce qu'on manque de temps ;
- A187 (écart de `Hs` par battements) et A188 (calibration statistique) sont la raison pour
  laquelle S69 disait ce banc **urgent**.

Piège nommé par S145, à ne pas retomber dedans : **écrire une sonde de plus au lieu de lancer le
banc**. La distinction n'est pas la forme du code — un banc s'exécute aussi par du code — mais ce
qui est mesuré : une sonde compare le modèle à lui-même, B1 compare `B` à une **cible statistique
extérieure**, `Hs = 4√m0`.

Second piège : annoncer « B1 exécuté » alors que deux de ses volets sont perceptuels. Le banc
rendra un **verdict partiel**, et il faudra que le rapport dise exactement lequel — sinon la
prochaine session lira « B1 fait » et le corpus portera un renvoi faux de plus (L217).
