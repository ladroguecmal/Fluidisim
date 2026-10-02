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

Session : S454 — **terminée**. *« Continue »* — le lot des registres (ADR-213 D3, dû en S454), puis **C10, les scènes**
(campagne §6 : le joueur qui saute à 5 cm, la gerbe d'étrave, la lame du déversoir ; le critère d'arrêt du §3.4).

**Ce que la session fait.** (1) Le lot des registres pour S451–S453. (2) **La conception de C10** : son découpage en lots, chacun avec
son critère (`docs/validation/C10-SCENES-S454.md`). (3) **C10-1, le saut du joueur** — jusqu'ici B10 n'existe qu'en quart (deux plans de
symétrie, 0,8 m de côté) : B10 sur **un domaine entier**, la sphère au centre, mené par la carte seule et rendu par `surface_carte`
(sans réflexion) ; d'abord le quart déplié (1,6 m), puis **une scène de 4 m × 4 m**.

**Critères, écrits avant.** (1) **le quart déplié** (domaine entier de 1,6 m, sphère au centre) reproduit le quart : la profondeur de
la cavité à t = 1 et la hauteur du jet à t = 2 à une maille près (5 cm) de celles du quart ; (2) **la scène de 4 m** : masse en quanta
exacte, `φ` fini jusqu'à `t·√(g/D)` = 4, le coût d'un pas de la carte publié (médiane) et quatre images ; (3) la fenêtre
`--surface-direct` sur la scène de 4 m (`COTE=4`).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le lot des registres.
- [x] **P3** — la conception de C10.
- [x] **P4** — C10-1 : B10 en domaine entier ; mesures (1) à (3) ; images.
- [x] **P5** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — feuille de route, liste, file active pour S451–S453 (la surface continue reçue, R37, et en direct ; « A322 avant C10 » retiré : levée en S442) ; `Registres` : dernier lot S454, le prochain au plus tard en S457.
- **P3** — `docs/validation/C10-SCENES-S454.md` : C10 en cinq lots — C10-1 le saut sur un domaine entier ; C10-2 le saut dans la mer (le raccord sur la carte, ses marges gardées en colonnes) ; C10-3 la coque et la gerbe d'étrave ; C10-4 la lame du déversoir ; C10-5 la scène du §3.4.
- **Décision de l'utilisateur** (2026-10-03) : *« que tu ne t'arrêtes pas de travailler jusqu'à une v1 solide visuellement, et
  physiquement, prends les décisions »* — [ADR-215](../docs/adr/ADR-215-autonomie-jusqu-a-une-v1-solide.md) (la v1 solide définie, D3 ; l'ordre, D4).
- **P4** — `B10::entier`, le rendu d'un domaine entier, le banc `--c10-saut` ([preuve](../docs/validation/C10-SCENES-S454.md) §3) :
  (1) cavité tenue (0,75 maille), jet sur l'axe 2,7 mailles plus haut que le quart (témoin 0,06) — tranché, ADR-215 D2 ; (2) la scène de
  4 m, masse exacte jusqu'à t = 16 **après correction du plafond** (la nappe plaquée divergeait ; 2,5 m d'air) ; 27 ms par pas ;
  (3) la fenêtre à 4 m : simulé / réel 0,11. Images envoyées.
- **P5** — C10-SCENES-S454 §3 ; journal ; jeton libre ; maillons 11 (justifiés : S406) ; suivant : S455, le temps réel à 4 m (ADR-215 D4).
