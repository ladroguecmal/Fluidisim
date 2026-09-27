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

Session : S413 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue »* — la suite déclarée : **C6c-1**
([ADR-212](../docs/adr/ADR-212-la-bande-etroite-en-profondeur.md) §4). Agent : Claude Code (Opus 5.5), au poste ; référence CPU.

**Thèse** (ADR-212 D1–D3). Une hauteur `β` (« fond de la bande ») par colonne de la bande ; `β` = 0, la bande pleine de
S398–S410, **au bit**. Une maille dont le centre est sous `β` est **à la grille** (comme une maille de la zone) : `φ = z − β`, eau,
faces advectées ; la face `w` au-dessus de la dernière lui appartient. **Les soldes** : les débits entre parts eulériennes
(mouillés jusqu'au plus bas des deux `β`, ou de `β` et `η`) transportent `β` en `f64` ; la part d'une face entre une maille à la
grille et une maille de particules charge le **solde latéral** de la face-maille (la frontière de S399, généralisée maille par
maille : « à la grille » d'un côté, particules de l'autre) ; la face `w` au-dessus de `β` charge un **solde vertical** par colonne.
**L'échange** : une particule sous `β` est absorbée et paie le solde vertical ; un solde vertical dû retire la particule la plus
proche au-dessus de `β`, reçu en pose une à `β + dx/16`. Particules virtuelles de la part eulérienne (la sienne, les voisines)
dans la reconstruction. La bascule (S408) refuse, dans cette part, une colonne dont `β` > 0 (C6c-2 la placera).

**Critères, écrits avant** (ADR-212 §3). (1) `β` = 0 : les essais S398–S410 tels quels ; B10 à bande dynamique (S408, défauts)
et la vague de S410 (défauts) au caractère près. (2) **Repos** : un bassin tout en bande, `β` à quatre mailles sous la surface,
et un bassin mi-zone mi-bande : vitesses sous les seuils de l'essai de repos de S398–S399, volume ≤ 10⁻⁹, densité au-dessus de
`β` dans 8 ± 0,4 particules par maille. (3) **Ballottement** (le banc du raccord, S399) avec `β` : période et amortissement à un
point d'APIC seul sur 30 s, volume exact ; particules comptées contre la bande pleine. (4) Suite entière, zéro avertissement.
**Arrêt** : si la densité au-dessus de `β` dérive (le risque d'A316), la publier et ne rien rendre défaut.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `β` : état (`floor`, reste, `solde_w`), `set_band_floor`, volume total, finitude ; essai de refus et de volume ; critère 1 (essais).
- [x] **P3** — étiquettes et reconstruction : mailles sous `β` à la grille, particules virtuelles jusqu'à `β`.
- [x] **P4** — advection : les faces d'une maille à la grille, la face au-dessus de `β`.
- [>] **P5** — transport : `β` par les débits ; soldes latéraux maille par maille ; solde vertical.
- [ ] **P6** — échange : absorption sous `β`, règlement du solde vertical, règlement latéral généralisé ; la bascule refuse `β` > 0.
- [ ] **P7** — essais du repos et du volume ; critère 2.
- [ ] **P8** — le banc du raccord avec `β` ; critère 3 ; bancs B10 et vague au caractère près (critère 1).
- [ ] **P9** — suite entière ; critère 4.
- [ ] **P10** — preuve `BANDE-ETROITE-S413` ; liste (4.16), file, feuille de route, index.
- [ ] **P11** — rituel.

### Notes de reprise
- **P2** — `Columns3` : `floor`, `floor_roundoff`, `solde_w`, `floors` (réservés : 16 octets de plus par colonne) ;
  `set_band_floor` (refus `Domain`/`Shape`/`NotFinite`, particule sous le fond refusée ; zone remise à zéro), `band_floor`,
  `band_floor_volume` ; `total_volume` + fond + soldes verticaux (rien sans fond) ; `floor_of`, `grid_cell` (utilisés en P3).
  Essai `_s413` ; les 23 essais d'APIC 3D tenus (24 avec lui).
- **P3** — `reconstruct` saute les mailles à la grille (`grid_cell`) ; `columns_label` : sous le fond, `φ = z − fond`, eau ;
  `virtual_column_sums` : une colonne de la bande à fond compte ses particules virtuelles jusqu'à son fond, la sienne comprise.
  Essai `_s413` : une seule traversée de `φ` par colonne ; hauteur lue **identique au bit** à la bande pleine (0,3989 m ; les
  virtuelles tombent sur le réseau nominal).
- **P4** — `columns_advect` : une face-maille `u`, `v` est à la grille si l'une de ses deux mailles l'est (la zone, ou sous le
  fond) — la règle de S406 maille par maille ; une face `w` si la maille au-dessous est sous le fond (la face au-dessus de la
  dernière comprise). Sans fond, au bit : 25 essais d'APIC 3D tenus.
