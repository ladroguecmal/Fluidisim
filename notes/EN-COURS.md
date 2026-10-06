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

Session : S575 — **en cours**. En autonomie : **le lot** (dû ; feuille de route S572–S574 ; la ligne 7.6 de la liste reformulée), puis
**C15 — la croissance de la glace** (non exécuté ; listes 7.6, 13.2) : « lac abrité, `FDD` imposé, 30 jours ; épaisseur à ± 10 % de
`0,035·√FDD` ; aucune plaque tant que `Hs > 0,15 m` ; masse conservée sur un cycle gel/dégel complet ». ADR-203 D6 : le gel des contenants
passe par V.

**Ce que la session fait.** La glace d'un lac est **une couche des liquides de V** (ADR-241), de densité 917, au-dessus de l'eau — sa
flottaison est alors l'hydrostatique des couches. `glace::geler(…)` : l'épaisseur visée par Stefan (S574), le volume de glace visé
`A·h` (l'aire de la surface, fournie), et le gel **par quanta exacts** — 917 ml d'eau deviennent 1 000 ml de glace (`917·1000 = 1000·917` :
la masse à l'entier) ; la fonte, le chemin inverse. Le nœud gagne 83 ml par quantum (la glace prend plus de place). Pas de prise en plaque
si `Hs ≥ 0,15 m` (le mécanisme existe : l'assertion n'est plus vide, note S29 de C15).

**Références, calculées avant** (ce script les écrit). Un lac de 10 × 10 m, 2 m d'eau, 30 jours à 10 K de gel (300 K·jour) : Stefan
**0.610219 m** ; la référence de C15, `0,035·√FDD` = **0.606218 m** (écart +0.66 %) ; **61021 quanta**,
55956257 ml d'eau gelés ; la surface monte de 50.647 mm. Le dégel complet rend toute l'eau.

**Quantum** (ADR-236 D1) : un quantum de glace, 1 000 ml sur 100 m² = 10 µm d'épaisseur. **Critères, écrits avant.** (1) l'épaisseur
à ± 10 % de `0,035·√FDD` (et à un quantum de Stefan) ; (2) la masse `ρ_eau·V_eau + ρ_glace·V_glace` exacte à chaque pas, et l'eau rendue
au millilitre après le dégel complet ; (3) sous `Hs` = 0,2 m, aucune glace en 30 jours ; (4) la composition somme au volume du nœud à chaque
pas (la place comprise : refus si la capacité manque).

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `geler`, `fondre` et C15 ; (1)–(4).
- [ ] **P3** — preuve ; listes 7.6, 13.2 ; C15 ; rituel (`--lot`).

### Notes de reprise
