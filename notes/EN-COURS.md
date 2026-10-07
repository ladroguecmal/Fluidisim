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

Session : S615 — **en cours**. En autonomie (ADR-247) : **10.5 — les grandes formes cohérentes entre clients, les détails locaux libres**
(absent). δ n'est jamais le même d'un client à l'autre (SPEC-003 : jamais D1 ; une autre maille, d'autres détails semés localement). La
grande forme passe à W, répliqué, au-delà d'une coupure `λ_cut` (B2, ADR-001) ; le reste demeure local et libre.

**Ce que la session fait.** Un module `coherence_clients.rs` : `filtre_gaussien(profil, dx, σ)` — la coupure passe-bas (noyau tronqué à 4σ,
normalisé) ; `transduire_coupe(domaine, σ)` — la transduction de S612 sur la seule part passe-bas de δ (`η` et `ū` filtrés), le reste
laissé au client. Ne fait pas : le 2D/3D, le choix de `λ_cut` par la physique (ici `σ` = 1 m), l'autorité de l'émission de W (qui émet
l'événement répliqué : 10.1), le transport.

**Références, calculées avant** (`s615_ref.py`, numpy). La même cause — une bosse de 5 cm — chez deux clients : **A** (maille 0,5 m, pas de
détail) et **B** (maille 0,25 m, des rides locales de 1 mm à 1 m de longueur d'onde sur 40 m, semées par le client). À 5 s, la coupure
(`σ` = 1 m) puis la transduction. À **15 s**, les W des deux clients diffèrent de **1.116420231e-04 m** pour une crête de 0.024016 m
(0.465 %) ; leurs restes locaux à 5 s diffèrent de **4.328374884e-04 m** — les détails sont libres ; les rides
fuient dans W de **2.734537962e-05 m** (2.73 % de leur amplitude, par les bords de la zone ridée) ; **sans coupure**, les
deux W diffèrent de **4.335598843e-04 m** (3.9 fois plus).

**Quantum** : f64. **Critères, écrits avant.** (1) l'écart des W à 15 s égal à la référence à 10⁻⁹ m, sous 1 % de la crête ; (2) l'écart des
restes égal à la référence à 10⁻⁹ m, au-dessus de 0,25 mm ; (3) la fuite des rides égale à la référence à 10⁻⁹ m, sous 5 % de leur amplitude ;
(4) sans coupure, l'écart des W égal à la référence à 10⁻⁹ m, plus de trois fois celui avec coupure ; (5) refus : `σ` ou `dx` non positifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `coherence_clients.rs` et ses essais ; (1)–(5).
- [ ] **P3** — preuve ; liste 10.5 ; rituel.

### Notes de reprise
