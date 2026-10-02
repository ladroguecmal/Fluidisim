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

Session : S448 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* (sans réponse sur le critère (2) de c2 : c2 reste
plafonné, le raccord conservatif retenu) — la suite déclarée : le lot des registres (ADR-213 D3, dû en S448), puis **c3**.

**c3, la conception.** Le banc `deferlement_en_mer` : la vague de Chen (S410, `apic3d_deferlement` — Stokes d'ordre 3, `ε` = 0,55,
`λ` = 2 m, 1 m d'eau) dans une bande `Apic3` **en eau totale**, avec ses particules et la bascule (S408–S432), posée par le **raccord
conservatif** (S447) dans une mer `Volume3` relative dont B est la composante linéaire de la vague (`a = ε/k`, `λ` = 2 m) ; la même
bande seule, à parois, en témoin (la forme seule). Le raccord passe à la zone des colonnes : la bande garde une couronne de colonnes à
ses bords.

**Critères, écrits avant** (le « reçu si » de C7d-3c, §22.3, réduit à ce que le raccord change) : (1) **le retournement** — l'instant
et l'abscisse du premier retournement dans la fenêtre à 10 % de ceux de la bande seule ; (2) **la part de la fenêtre en particules** au
retournement à 0,1 près de celle de la bande seule ; (3) **la masse au raccord** sous 0,1 % du volume d'une demi-période ; (4) sous une
houle calme (`ε` = 0,1, sans déferlement) : **aucune colonne en particules**. Bancs courts (D4) : une seule résolution, 20 mailles
par longueur d'onde.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le lot des registres (S446–S447).
- [x] **P3** — le banc `deferlement_en_mer` ; mesures ; critères.
- [ ] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — feuille de route, liste, index pour S445–S447 ; `Registres` : dernier lot S448, le prochain au plus tard en S451.
- **P3a** — le raccord porté du banc dans le système : `water_core::band_in_sea::BandInSea` (`configure`, `feed_sea`, `feed_band`,
  `set_conservative` ; tampons réservés ; une colonne de particules de la bande garde la hauteur de la mer, ses vitesses passent) ;
  `Volume3::rest`. Le banc `raccord_bande_mer` passe par lui : **les chiffres de S447 au bit** (9,4218 mm, 1,126·10⁻³ m³, 3,943·10⁻⁶ m³).
- **P3b** — banc `deferlement_en_mer` (houle calme `ε_B` = 0,2 dans une mer de 24 m ; bande de 8 m, groupe de Stokes d'ordre 3 à
  `ε₀` = 0,55 au centre ; 20 mailles par longueur d'onde ; 20 s de calcul) : **dans la mer** — retournement **0,736 s** à 13,65 m, part
  de la fenêtre en particules 0,350, masse au raccord **3,4·10⁻⁷ m³** (≈ 0) ; **bande seule de 8 m** (parois à 4 m du centre) — 0,846 s,
  13,95 m, 0,375 ; **bande seule de 16 m** — 0,576 s, 13,25 m, 0,125. **(1) mal posé** : la forme seule dépend de la largeur de sa bande
  (un groupe modulé n'est pas une solution ; son évolution dépend de ce qui l'entoure) — la mer tombe entre les deux. **(3) tenu.**
  **(4) manqué** : sous une houle calme (`ε` = 0,1), 42 colonnes en particules après 1 s (bande seule : 20) — la bascule à ses défauts,
  non avec les réglages reçus en C7d-1/C7d-2 (la vitesse propre relative à B). Suite **762**. **c3, première session : non reçu.**

