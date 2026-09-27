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

Session : S412 — **en cours**. Conception, au poste. Réponses de l'utilisateur (2026-09-27) au §8 de
[TRUCAGES-TEMPS-REEL-S411](../docs/registres/TRUCAGES-TEMPS-REEL-S411.md) : *« 1. Je valides ton choix 2. Je valides ton choix 3.
Ok 4. cela dépends une simulation d'un joueur de 15m peut être calculé a l'avance et plus le joueur se rapproche de la
simulation et peux intérargir et simule en temps réel, a réfléchir »*. Agent : Claude Code (Opus 5.5), au poste.

**Livrables.** (1) Un ADR des décisions : la bande étroite en profondeur est C6c, avant C7 ; la surface continue avant C10 ; les
courants après la campagne ; le calcul d'avance au loin, la simulation vivante de près — à réfléchir, première analyse et point
de file. (2) La **conception de C6c** — structure, masse exacte, critères « reçu si », découpage —, lue sur le code de la zone
(S398–S410) avant d'être écrite. Le code de C6c commence à la session suivante.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-211, les décisions du 2026-09-27 ; notes datées (trucages §8, campagne) ; file, feuille de route.
- [x] **P3** — lecture du code de la zone : vitesses eulériennes, transferts aux faces de frontière, `φ`, échange, bascule.
- [x] **P4** — ADR-212, la bande étroite en profondeur : structure, masse, critères, découpage ; index.
- [>] **P5** — rituel.

### Notes de reprise
- **P2** — [ADR-211](../docs/adr/ADR-211-les-trucages-retenus.md) (D1 à D4 ; D4, première analyse : un niveau « cuit » entre le
  factice et δ vivant, passage par transfert d'état, I-17 par la graine) ; notes datées : trucages, campagne, ADR-207 ; file
  (décision en tête, point D4 avec déclencheur) ; feuille de route ; index (ligne B). Lien vers ADR-212 posé d'avance (P4).
- **P3** — lu (`apic3d.rs` `step`, `reconstruct` ; `apic3d_columns.rs` `columns_begin/advect/label/transport/exchange`,
  `virtual_column_sums`, `apply_columns_mask`). Le pas : P2G → **advection des faces de la zone** (semi-lagrangienne, la face
  de frontière lui appartient, S406) → reconstruction (la zone saute, **particules virtuelles** des colonnes voisines, S399) →
  étiquettes de la zone (`φ = z − η`) → gravité, projection unique → **transport de `η`** par les débits mouillés, les faces de
  frontière chargeant un **solde** → G2P, advection → **échange** : absorption (paie le solde de la face la plus proche ; au cœur,
  `η` en `f64`), retrait, pose à la face. **Conséquence pour C6c** : une colonne de la bande est une colonne de la zone dont la
  hauteur eulérienne `β` n'est plus la surface libre mais **le fond de la bande**, des particules au-dessus ; tout le mécanisme se
  réemploie, tourné à la verticale — la face `w` à `β` est une frontière (solde vertical, absorption, pose à la face). Un seul
  paramètre par colonne : `β` = surface (colonne), `β` = 0 (les colonnes entières de S408), entre les deux (la bande étroite).
- **P4** — [ADR-212](../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md) : `β` par colonne (colonne : `β` = `η` ; bande étroite ;
  bande pleine : `β` = 0, S398–S410 au bit) ; sous `β` la machinerie de la zone ; la face à `β`, frontière à solde vertical ; le
  critère place `β` à `k` mailles sous la surface la plus basse, hystérésis `h` (prédiction `k` = 4, `h` = 2) ; six critères ;
  C6c-1 (`β` fixe) puis C6c-2 (`β` placé). Index. `--check` : 0.
