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

Session : S468 — **terminée**. En autonomie : **l'ombre du joueur**.

**Ce que la session fait.** Le soleil direct occulté par la capsule, dans `saut_optique.gdshaderinc` (le rendu final, Godot,
ADR-192 ; l'afficheur, banc, n'en a pas besoin) : `ombre_corps(q)` — 0 si le chemin de la lumière jusqu'à `q` touche le corps, 1
sinon ; au fond, le chemin réfracté (de `q` à la surface le long du soleil réfracté, puis le soleil dans l'air, la surface prise
au niveau moyen) ; à la surface, le soleil dans l'air. Il éteint la part directe : de l'éclairement du fond (et ses caustiques),
du corps d'eau, de l'éclat. Sous le ciel couvert, rien ne change (le soleil direct est déjà éteint).

**Critères, écrits avant.** (1) l'ombre là où la géométrie la met : sur le fond, partant du pied du joueur le long du soleil
réfracté (vers le nord-ouest… à l'opposé du soleil) — vérifié sur une image ; (2) ciel couvert : l'image inchangée au bit hors
du corps (lu dans le code : `soleil_direct` déjà nul) ; (3) la cadence de `--cout` sous 10 % de perte ; (4) les images montrées.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'ombre ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `ombre_corps` dans `saut_optique.gdshaderinc` ; elle éteint la part directe du fond (et ses caustiques), du corps d'eau
  et de l'éclat. **Mesuré** : (1) l'ombre part du pied du joueur **vers le sud-est** — à l'opposé du soleil, au nord-ouest (le plan
  disait « nord-ouest » : une erreur d'écriture, la géométrie est celle-ci) — vers la caméra de R38 ; un masque de contrôle
  (surface, fond) l'a montrée de la forme de la capsule sur la surface à 5,80 s ; (2) ciel couvert (`COUVERT=1`) : les six images
  **identiques au bit** à celles d'avant ; (3) `--cout` : 405 → **397 images/s** (−2 %) ; (4) image `saut_t1.60.png` envoyée.
- **P3** — C10-SCENES-S454 §17 ; journal ; jeton libre ; maillons 1 ; suivant : S469, le direct avec le joueur debout (le lien local vérifié, l'image envoyée).
