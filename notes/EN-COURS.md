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

Session : S408 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« continue »*. Suite proposée par S407 : **C6**, le critère
de bascule (conception S384 §4.2, étape 6 : « bascule colonnes ↔ particules, à masse exacte (S323), selon le critère »). Agent :
Claude, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Branche
`claude/eager-volta-lf0kw3`, la plus avancée.

**Où en est le raccord** (RACCORD-3D-S398 §5–8) : dans `Apic3`, une zone de colonnes et une bande de particules sous une même
projection, l'échange à masse exacte ; le critère de S399 tenu aux deux mailles — mais sur un **masque fixe**, posé à la
configuration. **C6 demande qu'il bouge** : des particules seulement là où la surface n'est pas un graphe (pli, cavité, jet, objet
qui entre), des colonnes ailleurs ; aucune bascule qui oscille (hystérésis) ; le coût compté — reçu sur B10 et sur une vague qui
déferle. **Cette session, C6a** : la bascule elle-même en 3D et un premier critère, éprouvés sur **B10** ; la vague qui déferle,
C6b.

**Thèse.** Les deux gestes de S323 (2D), portés en 3D sur la zone de S398 : **colonne → particules** — ensemencer sous `η` sur le
réseau nominal (2 × 2 × 2 par maille), la maille du haut au plus près de son volume, le reste à une **réserve** de volume (`f64`)
que l'échange règle aux faces de frontière mouillées ; **particules → colonne** — seulement si la surface reconstruite y forme un
seul segment d'eau posé sur le fond, sans corps ; la hauteur par la **voie mixte** de S323 (la forme par `φ`, le niveau par la
masse, un décalage uniforme sur l'ensemble converti) ; les soldes d'une face qui cesse d'être frontière vont à la réserve. **Le
critère** : une colonne est **requise** en particules si elle n'est pas convertible, si le corps l'atteint (rayon, marge, et sa
vitesse sur un horizon — l'objet qui entre), ou si la pente de sa surface dépasse un seuil (le pli prédit) ; la bande est la
dilatation de ce qui est requis ; une colonne ne repasse aux colonnes qu'après une durée sans être requise (hystérésis).

**Critères, écrits avant.** (1) Sans bascule appelée, au bit : les essais de S398–S407, le banc du raccord au chiffre près (S407).
(2) Allers-retours sur des états réels (le ballottement de S399 après 2 s ; la zone entière passée en particules puis rendue), dix
de suite : volume total exact (≤ 10⁻⁹) ; hauteur de chaque colonne à 0,2 maille du départ (S323) ; une colonne à poche d'air
refusée. (3) B10, `Fr` = 2, `D/dx` = 8, quart : la bande dynamique contre APIC seul — **pincement à un pas près** (1,5067 √(D/g),
pas 0,027) ; volume exact ; **au plus deux bascules par colonne** (aucune oscillation) ; part moyenne des colonnes en particules
publiée (prédiction : ≤ 50 %, le coût). (4) Suite entière, zéro avertissement. **Arrêt** : si le pincement s'écarte de plus d'un
pas, publier l'écart et ce qui le porte, ne rien rendre défaut qui ne soit éprouvé.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `set_columns_mask` : colonne → particules et particules → colonne à masse exacte, réserve, soldes ; critère 1.
- [x] **P3** — essais des allers-retours et du refus ; critère 2.
- [x] **P4** — le critère : `ColumnsSwitch` (requis, dilatation, hystérésis), sur `Apic3` et un corps.
- [ ] **P5** — le banc B10 à bande dynamique (mesures qui lisent l'eau des colonnes) ; lancé.
- [ ] **P6** — les calculs ; critère 3.
- [ ] **P7** — suite entière, zéro avertissement ; critère 4.
- [ ] **P8** — preuve `BASCULE-S408` ; liste (4.16, 4.10), file, feuille de route, index ; A316.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — `apic3d_columns.rs` : `set_columns_mask` (→ `ColumnsChange` : colonnes et particules passées dans chaque sens,
  refusées, décalage de la voie mixte), `convertible_height` (un seul segment d'eau posé sur le fond, sans maille solide ;
  l'iso-zéro de `φ`), `columns_settle_reserve` (la réserve à parts égales sur les faces-mailles de frontière mouillées, au début
  de l'échange), `Columns3::reserve` (dans `total_volume`) ; tampons réservés avec la zone (deux masques, une hauteur `f64` par
  colonne ; `columns_reserved_bytes` suit) — aucune allocation. Ensemencement : sous-couches pleines, la dernière au plus près
  (0 à 4 particules, en diagonale d'abord), vitesse et matrice affine de la grille. Les 16 essais d'APIC 3D tenus tels quels.
- **P3** — deux essais `_s408`. **Allers-retours** sur le ballottement de S406 mené 2 s, dix de chaque sens : **volume exact**
  (3,2·10⁻¹³) ; **hauteur : manqué au critère** — 0,096 maille après un et deux tours, puis l'écart croît d'environ 0,02 par tour
  (0,118, 0,140, … 0,204 au huitième) jusqu'à **0,309** au dixième, sans point fixe : l'ensemencement quantifie la hauteur au
  huitième de maille, et la lecture d'une sous-couche partielle n'est pas sa masse ; la voie mixte corrige le total, pas chaque
  colonne. L'essai protège le volume sur dix tours et **0,2 maille après deux tours** (l'usage : au plus deux bascules par colonne,
  critère 3). Décalage de la voie mixte ≤ 3,3 mm. **La poche d'air** : le premier jet la laissait passer — `φ`, au noyau de deux
  mailles, comble une poche d'une colonne sur trois mailles, et la voie mixte baissait deux colonnes de **7,4 cm** ; corrigé — une
  colonne n'est convertible que si ses mailles occupées se suivent depuis le fond ; refusée, et sa voisine passe (décalage 1,1 mm).
- **P4** — `ColumnsSwitch` (`apic3d_columns.rs`) : `switch(now_us, &mut Apic3)` reconstruit la surface **une fois**
  (`refresh_surface`), décide, applique par `apply_columns_mask` (le cœur de `set_columns_mask`, sans seconde reconstruction) et
  compte les bascules **effectives** par colonne (une conversion refusée n'en est pas une). Requis : non convertible (la même
  lecture que la bascule, `convertible_height`) ; le corps — empreinte du segment parcouru pendant l'horizon, élargie de la marge,
  dès que son bas descend à la marge de la surface ; la pente (différences centrées, décentrées à côté d'une hauteur inconnue).
  Dilatation de Chebyshev séparable ; hystérésis par l'instant de la dernière demande. Défauts : pente 1, marge 2 mailles, horizon
  0,2 s, dilatation 2, maintien 0,5 s — **non calibrés**. `clear_counts` après la bascule qui pose la zone initiale. Deux essais :
  le corps (132 colonnes, l'empreinte dilatée exacte ; tenue 0,5 s ; deux bascules), la marche et la poche (requises, dilatées).
  20 essais d'APIC 3D tenus.
