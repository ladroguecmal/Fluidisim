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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S507 — **en cours**. En autonomie, **A328** — le δ 3D à coque mobile converge lentement en `dt` près de la coque (S505). Et le lot
des registres, dû en S506.

**Ce que la session fait.** Localiser avant de remédier (ADR-226 D1) : la coque en translation sous-critique (0,5 et 2 m/s, départ en
rampe), quatre pas (ADR-230 D1) ; trois normes de l'écart au plus fin : le maximum sur la surface, le maximum hors de la coque (à une
maille près de son empreinte), l'écart quadratique ; puis, si le défaut reste dans le champ, éteindre un à un les termes de la mise à jour
par pas contre un témoin.

**Ordre de grandeur, écrit avant.** Le bord de la coque est une discontinuité qui se déplace d'une fraction de maille par pas : au bord,
l'écart **maximal** est de l'ordre de la hauteur portée par la colonne coupée, et ne décroît en `dt` que si la maille décroît aussi — un
ordre ≈ 0 attendu au bord ; hors de la coque, l'ordre du schéma (1, le pas de la géométrie appliqué d'un coup) ; en norme quadratique, le
bord pèse en `√(colonnes du bord / colonnes)` ≈ 0,1.

**Critères, écrits avant.** (1) les trois normes publiées sur quatre pas aux deux vitesses ; (2) si l'ordre hors de la coque et l'ordre
quadratique sont ≥ 0,8 aux deux vitesses : A328 se réduit au bord de la coque (une propriété d'une paroi qui se déplace sur une grille
fixe, de résolution : A317) — levée, réécrite ; sinon, la suite : éteindre les termes ; (3) le lot des registres.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — les trois normes, quatre pas, deux vitesses (calcul détaché).
- [ ] **P3** — verdict sur A328 ; preuve ; lot des registres ; rituel.

### Notes de reprise
