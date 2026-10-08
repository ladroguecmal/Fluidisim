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

Session : S714 — **terminée**. En autonomie, sans arrêt ; session longue. S713 : sur la plage de Synolakis, le pas de 2,5 ms rend juste la
crête du déferlement, que le pas de 10 ms plaçait 0,04 d trop bas et 0,26 d en arrière. **Le juge des raccords (S690–S703), au pas de
10 ms, en dépend-il ?**

**L'essai.** Le tout-3D de S690 (le montage sans raccord), avec le pas plafonné à **2,5 ms** : `Large::AucunPasCourt`, un mode nommé
(ADR-277 D2). Seul le plafond change (ADR-276 D2).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-278, ADR-280)

- **témoin** : le juge de S690 au pas de 10 ms, par la même fonction : retournement à 2,637 s et 9,988 m ; air à 2,790 s.
- **instrument** : le lecteur de retournement de S647, et l'air enfermé. Ce que rendrait chaque hypothèse :
  - le juge ne dépend pas du pas : le retournement à moins de 0,1 s et 0,15 m (ADR-278 D2) ;
  - il en dépend : au-delà. Il faudra alors abaisser le plafond des montages de déferlement, et rejuger le raccord retenu (S703) au même
    plafond.
- **calcul** : le coût, au plus 4 fois celui de S690 (13 min) ; au déferlement le pas est déjà sous 2,5 ms, d'où ≈ 35 à 45 min. Mesuré et
  montré.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-280 D2 : le juge sous une option numérique ;
  - ADR-278 D2 : la tolérance ;
  - ADR-276 D2 : seul le plafond change.
- **pièges** : `pas_stable_us` du relais prend le plafond ; le relais de Saint-Venant au rivage en dépend aussi (la même borne).

**Critères.** Le retournement et l'air mesurés, attribués selon l'instrument.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai.
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — au pas de 2,5 ms : le retournement à 2,595 s, 9,938 m (−0,042 s), l'air à 2,750 s ; dans la tolérance d'ADR-278 D2. Le plafond de 10 ms reste.
