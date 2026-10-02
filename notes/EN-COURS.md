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

Session : S432 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« continue »* — la suite déclarée : **C7d-2**
([preuve](../docs/validation/APIC-CARTE-S416.md) §21.3 : C7d-1 reçu en référence — la vitesse propre de δ, entrée 0,3 / relâche 0,15
m/s ; la décision de la bascule sur la carte ne porte encore aucun critère d'écoulement).

**Ce que la session fait.** (1) La carte reçoit le critère de C7d-1 : huit flottants de plus dans les paramètres (seuil, relâche, fond
B : `a`, `k`, `ω`, `φ`, niveau moyen, présence) ; un noyau **`switch_flow`** entre la pente et la dilatation — comme l'étape (3b) de
`ColumnsSwitch::decide` : une colonne dont une maille d'eau dépasse le seuil (vitesse propre, relative à B s'il y en a un) est
requise ; sinon, en bande, au-delà de la relâche, gardée — ; **`floor_place`** descend aussi sous ces mailles, comme `place_floor`.
`SwitchSettings::of` les recopie (la vorticité, la part de rotation, la déformation ne sont pas portées : refusées si posées). (2) Les
clés `VITESSE`, `RELACHE`, `FOND_B` dans l'état B10 commun aux bancs (un fond B arbitraire : la houle de `a` 2 cm, `λ` 2 m, au niveau
de l'eau — B10 n'en a pas, la décision se compare quand même).

**Critères, écrits avant.** (1) Le banc de décision (`--apic3d-carte-decision`, cinq instants, la bascule initiale et les bascules
forcées) avec **`VITESSE=0.3 RELACHE=0.15 FOND_B=1`** : masque demandé **identique** à la référence, fonds identiques, positions et
volume comme S420 ; et sans les clés, comme S427. (2) **C7e tenu** : B10 en bande étroite sans les clés, pas + bascule ≤ 2,1 ms au p99,
identique au bit à S428 (`DUMP_B10`) sauf ce que le nouveau noyau change dans la compilation (isolé s'il y en a) ; avec les clés, le coût
publié. (3) Suites, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les paramètres, `switch_flow`, `floor_place`, `SwitchSettings` ; les clés des bancs.
- [x] **P3** — le banc de décision avec et sans les clés ; B10 en bande étroite (coût, identité) ; C7d-2 reçu ou non.
- [ ] **P4** — non-régression, suites ; preuve §21.4 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2** — paramètres (256 octets : seuil, relâche, fond B), `own_speed` (les moyennes de faces de `cell_speed`, moins B), **`switch_flow`** entre la pente et la dilatation (requise au-delà du seuil ; sinon, en bande, gardée au-delà de la relâche), **`floor_place`** sous ces mailles ; `SwitchSettings` recopie seuil, relâche et fond B, et **refuse** vorticité, part de rotation et déformation (non portées). Clés `VITESSE`, `RELACHE`, `FOND_B` dans `b10_band_state_from` (la houle de 2 cm, λ 2 m, au niveau de l'eau). Compilé sans avertissement.
- **P3** — **le banc de décision, identique à la référence dans les quatre séries** (masque demandé, fonds, positions, réserve, volume à 0 quantum) : avec les clés — bascule initiale, bascules forcées (14 887 à 44 070 particules : le critère travaille ; sans lui 5 998 à 8 686), réglage par défaut — et sans elles (S427 au chiffre près). **B10 en bande étroite sans les clés : identique au bit à S428 sur 74 pas**, pas p99 1,83 + bascule 0,24 = **2,06 ms** (C7e tenu). **Avec les clés** : la carte suit la référence au chiffre près — **les deux pincent au pas 53** (le critère change la physique de la référence aussi), 44 104 particules de part et d'autre ; pas p99 1,58 + bascule 0,40 = 1,98 ms (la décision 0,29 : le parcours des mailles ; les fils de l'échange presque vides, le fond étant bas). **C7d-2 reçu.**
