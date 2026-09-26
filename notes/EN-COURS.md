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

Session : S391 — **en cours**. **A321** : à 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit le solveur de
pression ; à 60 Hz la minute tient ([angle mort](../docs/registres/ANGLES-MORTS.md), [preuve](../docs/validation/MULTIGRILLE-3D-S385.md)
§5). Demande de l'utilisateur (2026-09-26) : *« Corrige A321 d'abord »* — avant la pluie, pièce 5, et C3b. Agent : Claude
Opus 5.5, Claude Code (application de bureau) au poste — fichiers, git, cargo, Python, RTX 5070 Laptop, Godot 4.6.3.

**Hypothèse, écrite avant la mesure.** La prédiction advecte δ par Euler explicite et différences centrées — `advect` (`u'·∇u'`)
et les termes croisés d'`extra` (`U·∇u'`, `u'·∇U`) ; le cœur fait de même (`advect_mobile3`, `extra3`). Ce schéma (FTCS) est
**instable pour tout pas** : il porte une anti-diffusion `(dt/2)·U_a·U_b·∂_a∂_b`, de taux ≈ `|U|²·dt/(2·dx²)` au plus fort, sur
les modes de deux à quatre mailles. **Prédictions** : (1) le temps d'explosion varie à peu près comme `1/dt` ; (2) le mode qui
croît est de l'échelle de la maille, dans les vitesses ; (3) éteindre `U·∇u'` (et `u'·∇u'`) supprime l'explosion à 30 Hz. Second
suspect : le transport de la hauteur par des flux centrés (`fluxes`, la bande de B).

**Critères.** (1) **Instrument** : le banc `--delta3d-a321`, commutateurs éteints, rend la production au bit (empreintes de
`--delta3d-empreinte`). (2) **Attribution** : temps d'explosion à 16,7, 25 et 33,3 ms ; échelle du mode qui croît ; un témoin
par terme éteint (L136) — le terme dont l'absence retire l'explosion est nommé, sinon l'hypothèse est dite fausse. (3) **Le
correctif**, choisi sur (2) et déclaré ici **avant** son code, dans la référence et la production, même formule. Reçu si : la
scène tient **deux minutes** à 30 et à 60 Hz ; les trois cas de cuve restent à 3 mm de la référence ; les réceptions de
dispersion de la référence tiennent leurs tolérances ; empreintes nouvelles expliquées champ par champ. (4) Suites, zéro
avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'instrument : commutateurs de banc dans le pas (éteints : au bit), banc `--delta3d-a321` (croissance par
  seconde, échelle du mode, premier pas non fini) ; critère 1.
- [ ] **P3** — l'attribution : pas de temps, témoins terme par terme ; critère 2 ; le correctif déclaré ici.
- [ ] **P4** — le correctif dans la référence (cœur), ses essais.
- [ ] **P5** — le même dans la production ; critère 3 (deux minutes, cuves, dispersion, empreintes).
- [ ] **P6** — critère 4 ; preuve ; A321, file, feuille de route.
- [ ] **P7** — rituel.

### Notes de reprise

**P2 — critère 1 tenu**, au second essai. Commutateurs dans `Step::switches` (l'ancien mot de remplissage) ; banc
`viewer/src/delta3d_a321.rs`, `--delta3d-a321`. **Vu en chemin** : des branches `if (switches…)` compilées dans les
pipelines de production changeaient les empreintes (60 pas `0xf1d768aa…` au lieu de `0x6e90a7ae…`), commutateurs à zéro — le
compilateur réarrange l'arithmétique voisine (L345). Remède : constante `override BENCH_SWITCHES`, vraie seulement dans une
seconde série de pipelines (`step_bench`), prise quand un commutateur est allumé ; empreintes d'avant retrouvées au bit.
Le bit 32 prend les pipelines de banc sans rien éteindre : témoin du bruit d'arrondi, la scène amplifiant tout écart.

