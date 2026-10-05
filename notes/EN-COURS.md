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

Session : S488 — **en cours**. En autonomie, **K2-4 — la nappe et sa rupture** ([conception](../docs/registres/CAMPAGNE-K2-S478.md),
ADR-014 §4, **A312**), dans la référence (ADR-213 : la référence d'abord, la carte en K2-5).

**Ce que la session fait.** `Apic3::enable_droplets` : une particule d'eau dont la maille est étiquetée **air** (une nappe plus mince
qu'une maille, que la reconstruction ne voit plus) et dont la vitesse donne `We = ρ·v²·d/σ > 12` (d : le diamètre de la goutte de son
volume, `dx/2·(6/π)^(1/3)`) **devient une goutte** : hors de la grille (ni transfert vers la grille, ni reconstruction, ni séparation),
**balistique** — `g_eff` et la traînée de l'air (`C_d` = 0,47, ρ_air = 1,2) ; elle **redevient de l'eau** en entrant dans une maille
d'eau (sa vitesse transmise). Aucune particule ne naît ni ne meurt : la masse est exacte par construction. Sans `enable_droplets`, le pas
d'avant, au bit.

**Entrées, et comment elles se vérifient.** B10 (`apic3d_b10`, le quart — A325 dit le prix du quart ; la mesure ici est la dépendance à la
maille, au même montage) à `D/dx` = 8, 12, 16, avec et sans gouttes ; la couronne (la particule la plus haute avant le pincement, gouttes
comprises) ; le banc de non-régression au bit (sans gouttes).

**Critères, écrits avant.** (1) sans gouttes, au bit (non-régression, essais d'APIC 3D) ; (2) masse exacte avec gouttes ; une goutte isolée
suit une trajectoire balistique (essai : la portée d'un jet vertical à 1 %) ; (3) **A312** : la hauteur de la couronne de B10 à trois
mailles à 15 % entre elles avec gouttes (contre 40 à 60 % sans) — sinon, l'écart dit et attribué.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les gouttes dans la référence ; essais (1), (2).
- [>] **P3** — B10 à trois mailles, avec et sans gouttes ; (3).
- [ ] **P4** — preuve ; rituel.

### Notes de reprise
- **P2** — `apic3d_gouttes.rs` (`enable_droplets`, `droplets_classify` après les étiquettes, `ballistic_step`) ; les gouttes hors du
  transfert vers la grille, de la reconstruction, de la séparation ; leur vitesse gardée au transfert vers les particules ; balistiques à
  l'advection (bornées au domaine). Refusées avec une zone de colonnes. Essais : `droplet_ballistic_apex_s488` (l'apogée à 1 %, avec et
  sans traînée), `droplets_from_a_jet_keep_the_mass_s488` (des gouttes naissent et retombent, masse exacte) ; 46 essais d'APIC 3D ;
  non-régression au bit. `apic3d_b10` : `APIC3D_GOUTTES=1`.
