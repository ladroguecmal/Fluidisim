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

Session : S648 — **terminée**. En autonomie vers la v2 ; le rouleau 3D. **La forme et la vie du rouleau** : après le retournement de S647,
le jet retombe devant la crête et **enferme de l'air** — la signature d'un déferlement plongeant.

**Le lecteur (ADR-263 D2), éprouvé d'abord.** L'air enfermé : les mailles d'air que l'air libre (la rangée du haut) n'atteint pas, en
remplissage par voisins (six), dans tout le domaine — leur nombre, et l'abscisse de leur centre. Éprouvé sur une couche d'eau à 0,6 m
(rien) et la même avec une cavité posée — x ∈ [0,9 ; 1,1] m, z ∈ [0,25 ; 0,40] m, toute la largeur : **48 mailles** posées ;
le lecteur doit la trouver, au plus 48 mailles (la reconstruction épaissit l'eau), centrée à 0,1 m près de 1,0 m.

**Critères, écrits avant.** (1) le lecteur ; (2) sur l'onde de S647, **à 2,5 cm, de l'air enfermé apparaît dans les 0,5 s qui suivent le
premier retournement, en avant de lui** (le jet a retombé) ; (3) rapportés : son volume le plus grand, sa durée de vie (de l'apparition à la
disparition), à 5 et 2,5 cm ; (4) la masse ; (5) l'onde de S644 n'enferme pas d'air (elle ne déferle pas). Ne juge pas : la quantité d'air
contre une mesure publiée (aucune n'a pu être relue).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le lecteur et son épreuve ; le rouleau ; (1)–(5).
- [ ] **P3** — preuve ; liste 4.14, 4.16 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(5) tenus : le lecteur (48 mailles posées, 48 lues) ; à 2,5 cm l'air enfermé 0,18 s après le retournement, 0,39 m en
  avant, 210 mailles (3,3 L), vie 1,18 s ; 5 cm : +0,30 s, 18 mailles, 1,12 s ; 1:3 rien ; masse exacte. En attendant le calcul :
  l'étude du découpage de la planète mesurée (HEALPix contre cube-sphère), son ADR en brouillon pour S649.
