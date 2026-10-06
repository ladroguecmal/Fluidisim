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

Session : S499 — **en cours**. En autonomie, **6.1, B6 — le nombre de points du proxy par archétype** (ADR-008 §5.1 : « 20 à 60 »). La
porte D a pris 16 × 8 × 4 points sans mesure de ce qu'il fallait ; S494–S495 ont vu une bouée de 4 × 4 × 4 points rouler et chavirer.

**Ce que la session fait.** Un banc dans le cœur : pour chaque archétype en pavé (navire, barque, caisse ; la balle est contrainte), et des
grilles `nx × ny × nz`, la raideur de pilonnement, la hauteur métacentrique en roulis et en tangage — mesurées par le moment de rappel
d'une inclinaison de 10⁻⁴ rad en eau calme — contre l'analytique `GM = KB + BM − KG`, et la houle vue par la flottaison contre le continu.

**Ordre de grandeur, écrit avant (ADR-226 D3).** L'inertie de flottaison d'une grille de `n` centres : `(L²/12)·(1 − 1/n²)` — l'erreur
sur `BM` est `1/n²`, sur `GM` `(BM/GM)/n²` : le terme concurrent est `GM`, petit devant `BM` pour un navire (roulis : `BM` = 4,27 m,
`GM` = 1,25 m → `n` ≥ 9 pour 5 %). La couche partielle pousse en son milieu et non au centre de sa part immergée : `KB` faux de
`(1 − f)·f·e²/(2d)` au plus `e²/(8d)`. La houle : `sin(kL/2)/(kL/2)` contre la moyenne de `cos(k·x)` sur `n` centres.

**Critères, écrits avant.** (1) la raideur de pilonnement `ρgA` à 10⁻⁹ quelle que soit la grille ; (2) l'erreur mesurée sur `BM` suit
`1/n²` à 10⁻³ près (la formule calculée dans l'essai), celle sur `KB` la borne de la couche partielle ; (3) par archétype, le plus petit
proxy dont `GM` (roulis et tangage) tient 5 % et la houle 1 % de son onde de projet — publié, comparé aux 20 à 60 points d'ADR-008.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le banc ; (1)–(3).
- [ ] **P3** — preuve ; liste 6.1 ; rituel.

### Notes de reprise
