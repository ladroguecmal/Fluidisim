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

Session : S563 — **en cours**. En autonomie : **le lot des registres** (dû ; feuille de route S561–S562), puis **5.7 — l'air scellé avec
plusieurs liquides** : `step_liquids` ignore l'air des compartiments ; un compartiment étanche qui contient du carburant et que la mer
envahit par le fond doit comprimer son air comme en S538.

**Ce que la session fait.** `step_liquids_air(…, air, pressions_pa)` : la pression de jauge de chaque poche scellée (Boyle isotherme,
`Air::Sealed`) ajoutée des deux côtés du seuil d'un orifice ou d'une vanne ; un seuil au-dessus de la surface amont ne laisse passer aucun
liquide ; les évents comme `step_air` (le même calcul, partagé). Les déversoirs et les pompes ne voient pas la pression des poches (comme
dans `step_air`). Tout ouvert : `step_liquids` au bit.

**Références, calculées avant** (une bissection indépendante du code, l'équilibre des pressions au seuil et Boyle résolus ensemble, la mer
de 100 × 100 m qui baisse de ce qui entre). Un compartiment de 1 × 1 × 2 m contenant 0,5 m³ d'huile (ρ = 850) et 1,5 m³ d'air à la pression
atmosphérique ; la mer à 1,5 m au-dessus de la brèche du fond (deux orifices de 1 000 mm², `C_d` = 0,62). **Scellé : 0,126197 m d'eau
entrent** (l'air à +9 307,6 Pa) ; **ouvert (le témoin) : 1,074893 m**. Constantes de temps (ADR-240 D2) : 96 s scellé (la raideur de
Boyle, ×7,9), 755 s ouvert ; l'essai dure 3 000 s. Le dépassement du pas explicite près de l'équilibre scellé : 6·10⁻⁷ m.

**Quantum** (ADR-236 D1) : 1 ml sur 1 m², 1 µm. **Critères, écrits avant.** (1) Scellé : l'eau entrée à 10⁻⁴ m de 0,126197 m ; l'huile
entière dans le compartiment, au millilitre. (2) Ouvert : à 10⁻⁴ m de 1,074893 m. (3) Tout ouvert : `step_liquids_air` identique au bit
à `step_liquids` sur le manomètre de S560. (4) `step_air` inchangé (la suite entière).

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `step_liquids_air` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.7 ; rituel (`--lot`).

### Notes de reprise
