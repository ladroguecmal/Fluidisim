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

Session : S713 — **terminée**. En autonomie, sans arrêt ; session longue. L'utilisateur dort et a dit de ne pas s'arrêter. S712 : contre
le laboratoire, la 3D à 2,5 cm fait l'onde de Synolakis trop étroite et trop haute avant le déferlement (0,43 d contre 0,31 d à t = 15).
**La résolution en est-elle la cause ?**

**L'essai.** Le même montage (`plage_synolakis`, une seule fonction, ADR-276 D1), sans projection, à **1,25 cm** sur deux rangées. Les
photos à t = 15 et 20, lues dès qu'elles sont prises. Seul `dx` change (ADR-276 D2 ; la largeur suit, sans effet sur une onde plane).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-280)

- **témoin** : E1 de S712 (2,5 cm) : à t = 15, la crête 0,433 d et l'écart 0,048 d ; à t = 20, 0,277 d et 0,066 d.
- **instrument** : celui de S712 (la surface φ lissée sur 10 cm, contre les points mesurés). Son plancher est la dispersion des mesures,
  0,01 à 0,02 d. Ce que rendrait chaque hypothèse :
  - la résolution est la cause : à 1,25 cm, la crête descend nettement vers 0,31 d (de plus de 0,05 d), et l'écart baisse ;
  - elle ne l'est pas : la crête reste vers 0,43 d.
- **calcul** : ≈ 600 000 particules, un pas deux fois plus court, soit ≈ 8 fois le coût de S712 : ≈ 45 min jusqu'à t = 15, autant jusqu'à
  t = 20. Le coût est mesuré et montré.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-279 D1, ADR-280 D2 : la convergence du juge, contre une référence extérieure ;
  - ADR-276 D2 : seul `dx` change.
- **pièges** :
  - les marches de l'escalier deviennent deux fois plus fines, et l'artefact de lecture change de pas ;
  - la mémoire : ≈ 600 000 particules × 60 octets, sans difficulté.

**Critères.** Les crêtes et les écarts à t = 15 et 20, et l'attribution selon l'instrument.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai à 1,25 cm (arrêté après t = 15).
- [x] **P2b** — E2 : le pas plafonné à 2,5 ms (2,5 cm).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — à 1,25 cm, t = 15 : la crête 0,421 d (2,5 cm : 0,433 ; la mesure : 0,314), l'écart 0,038 d. **La résolution n'est pas la cause** (−0,012 d, sous les 0,05 d attendus). Le calcul a été arrêté après t = 15 : au déferlement, le pas tombait sous 1 ms (≈ 1 h 10 pour t = 15, des heures pour t = 20). Suspect suivant : le pas, `c·dt/dx` ≈ 1 dans les deux calculs (S714).
- **E2, ajouté à la session (la même question, ADR-279 D3)** : la plage à 2,5 cm, sans projection, le pas plafonné à **2,5 ms** au lieu de 10 ms. `c·dt/dx` passe de ≈ 1 à ≈ 0,25 ; seul le pas change (ADR-276 D2). **Critères, écrits avant** : si le pas est la cause, la crête à t = 15 descend de plus de 0,05 d vers 0,31 d, et l'écart baisse ; sinon, elle reste vers 0,43 d. Le coût : ≈ 4 fois S712 E1, ≈ 45 min jusqu'à t = 25 ; chaque photo est lue dès qu'elle est prise.
- **P2b fini (E2)** — le pas à 2,5 ms : t = 15, 0,424 d (le pas n'est pas la cause) ; t = 20, 0,316 d en 3,77 d contre 0,318 d en 3,66 d mesurés (le pas compte au déferlement) ; t = 25, 0,218 d en −3,03 d.
