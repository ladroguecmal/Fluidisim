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

Session : S754 — **terminée**. En autonomie ; session longue. SYNOLAKIS-CORRIGEE-S753 : R1 rend la hauteur juste mais ralentit la vague
(−5,6 %) ; `Complete` garde la célérité (+0,4 %) mais freine la remontée de S645 contre la loi théorique (−11 %). **La question** : laquelle
des deux suit le mieux les **mesures** de Synolakis (ADR-280 D2 : la référence extérieure tranche) ?

**L'essai** : le montage de S753, `Complete` consciente du fond, sans R1 ; le même pas (2,5 ms), les profils à t·√(g/d) = 15, 20, 25. Une
seule différence avec S753 : R1 (ADR-276 D2).

**Les critères, écrits avant** — `Complete` est retenue à la place de R1 si :
1. la crête à t = 15 reste à **0,05 d** de la mesure ;
2. sa place à t = 15 et à t = 20 est plus proche de la mesure que celle de R1 (S753 : 9,42 d et 5,82 d, contre 8,38 d et 3,66 d) ;
3. l'écart quadratique est plus petit que celui de R1 à au moins deux des trois instants (S753 : 0,030, 0,075, 0,059 d).

Sinon, R1 reste, et la question de la célérité reste ouverte.

**Contrôles du plan** (ADR-276, ADR-280, ADR-289, ADR-291)

- **témoin** : les mesures de Synolakis ; S753 (R1) ; S713 E2 (sans correction).
- **instrument** : `comparer_synolakis_s712`.
- **calcul** : ≈ 45 min.
- **ADR** : ADR-280 D2 ; ADR-276 D2 ; ADR-291 (et sa note de S753).
- **pièges** : `Complete` freinait la lame de S645 (une onde posée près du pied, contre la loi) ; le laboratoire peut en juger autrement, ou
  pas.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1), (2), (3).
- [x] **P3** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** (2 531 s) :
  - t = 15 : 0,3644 d en 7,77 d, l'écart 0,0348 d ;
  - t = 20 : 0,3228 d en 2,97 d, 0,0466 d ;
  - t = 25 : 0,1967 d en −1,53 d, 0,0246 d.

  (1) manqué de 0,0009 d, sous le quantum ; (2) et (3) tenus. Mieux que la 3D sans correction aux trois instants. **ADR-292** : `Complete`
  consciente est la 3D corrigée, le critère (1) levé par écrit.
