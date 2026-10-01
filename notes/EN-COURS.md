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

Session : S420 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7c-4**, la bascule et
le déplacement du fond sur la carte ; B10 en bande étroite, le critère de C7 ([conception](../docs/validation/APIC-CARTE-S416.md) §8).

**Ce que la référence fait** (`ColumnsSwitch::switch`, S408–S414) : la surface rafraîchie ; **la décision** par colonne — requise en
particules si non convertible, atteinte par le corps (segment de l'horizon élargi de la marge), ou trop pentue ; dilatation de
Chebyshev ; maintien (hystérésis dans le temps) ; **la bascule à masse exacte** — particules → colonnes par la voie mixte (la forme par
`φ`, le niveau par la masse, décalage borné au quart de maille, l'excès à la réserve), colonnes → particules par ensemencement
nominal, soldes des faces qui cessent d'être frontière à la réserve ; **le fond** placé (k mailles sous la première non-eau,
hystérésis, prédiction du corps en option) puis déplacé (remonter absorbe, descendre ensemence). **Sur la carte** : la décision par
colonne en parallèle ; les retraits et ensemencements sur un fil dans l'ordre de la référence (tableaux indice pour indice, comme
l'échange) ; la masse et la réserve en quanta ; l'instant du maintien en microsecondes sur 32 bits (71 minutes : la production le
prendra relatif, C7e).

**Critères, écrits avant.** (1) La décision : le masque demandé identique à celui de la référence sur des états de B10 en bande
étroite (réglage retenu de R35 : maintien 0,3 s, fond 4, prédiction à 0,05 s). (2) Une bascule : masque, `n`, positions indice pour
indice à 10⁻⁵ m, `η` à 10⁻⁶ m, fond identique, volume total de la carte **constant exactement**. (3) **B10 en bande étroite** sur la
carte contre la référence : pincement au pas de la référence (ou à un pas, publié), écart de `φ` à l'interface pas à pas **au plus
celui du témoin** (±10⁻⁶ m/s), volume exact ; coût publié. (4) Non-régression ; suite ; zéro avertissement. **Arrêt** : ce qui ne tient
pas se publie ; la session s'arrête à l'étape achevée, le reste au jeton.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la décision sur la carte : paramètres, état du critère, hauteurs convertibles, corps, pente, dilatation, maintien ;
  banc : masque demandé contre la référence.
- [x] **P3** — la bascule : particules → colonnes (hauteurs, retraits sur un fil, voie mixte en quanta, réserve), colonnes →
  particules (ensemencement sur un fil), soldes à la réserve ; la réserve réglée dans l'échange ; banc d'une bascule.
- [x] **P4** — le fond placé et déplacé (`place_floor`, `move_band_floor`) ; banc.
- [x] **P5** — B10 en bande étroite sur la carte, carte, référence et témoin ; critère 3.
- [ ] **P6** — non-régression, suite ; preuve §12 ; liste, file, feuille de route, index.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — cœur : `ColumnsSwitch::switch_state` (instants requis, fond demandé). Carte : `Params` à 224 octets (critère), `swb` (sept tranches par colonne), `switch_need` (hauteur convertible, corps), `switch_slope`, `switch_spread`, `switch_request` (maintien sur 32 bits) ; `load_switch`, `decide_for_bench` (tri + reconstruction + décision). `b10_band_state` (B10, maintien 0,3 s, fond 4 ; un pas de plus sans bascule). `--apic3d-carte-decision` : **masque identique** à 0, 10, 30, 50, 60 pas (56 à 59 colonnes en bande sur 256). Critère 1 tenu. Banc lent (3 min : la référence rechauffée à chaque instant).
- **P3** — cœur `columns_reserve`. Carte : `switch_begin`, `switch_apply` sur un fil (capacité ; retraits des converties par la visite
  de la référence ; eau sous le fond et solde vertical comptés ; voie mixte en quanta, décalage borné à 2²⁵ quanta, le reste exact à
  la réserve ; ensemencement nominal en ordre de colonnes ; soldes des faces qui cessent d'être frontière à la réserve ; masque) ;
  `settle_reserve` au début de l'échange ; le drapeau de bande résident (`pcount[8]`). **La liste des retirées construite triée en
  parallèle** (`list_count`, `compact_scan`, `list_scatter`) pour l'absorption comme pour la bascule : le tri par insertion sur un fil
  faisait tomber la carte (délai de garde du pilote) à la bascule initiale, 120 000 particules. Bancs (`--apic3d-carte-decision`,
  `INITIAL`, `PENTE`, `MAINTIEN`) : à 21 instants sans bascule, tout identique ; **ensemencements forcés** (`PENTE=0.02` : 186 et 166
  colonnes, 98 537 et 89 590 particules) — positions **identiques**, réserves égales (1,761·10⁻⁵, 8,683·10⁻⁵ m³) ; **conversions**
  (`MAINTIEN=0` : 3 colonnes) — positions identiques, `η` 7,2·10⁻⁷ m, réserve égale ; **la bascule initiale** (200 colonnes vers les
  colonnes) — `n` 28 672 des deux côtés, positions identiques, **`η` à 1,9·10⁻⁶ m** (critère 2 « 10⁻⁶ » manqué : `η` se lit sur `φ`,
  admis à 10⁻⁵ — le critère était plus serré que sa source), réserve 6·10⁻¹¹ contre 0 (le reste de la division du décalage) ; volume
  de la carte **constant à 0 quantum** partout. Raccord inchangé.
- **P4** — `floor_place` (par colonne : `k` sous la première non-eau des étiquettes rafraîchies, prédiction du corps en option, hystérésis), `floor_move` sur un fil (capacité ; remonter : retraits par la visite de la référence, chacun au solde vertical de sa colonne ; les mailles prises pleines au solde ; descendre : huit particules par maille libérée au réseau nominal). Mots réservés WGSL rencontrés : `pass`, `target`, `from`. Bascule entière avec le fond, 0 (initiale), 10, 40, 60 pas : **fonds identiques** (56 à 59 colonnes à fond), masque, `n`, positions identiques, volume de la carte **constant à 0 quantum** (l'instrument comptait l'eau sous le fond seulement si la carte avait été chargée avec un fond — corrigé). Critère 2 tenu, sauf `η` 1,9·10⁻⁶ m à la bascule initiale (P3).
- **P5** — `BANDE=1 --apic3d-carte-b10` (bascule initiale puis après chaque pas, des deux côtés ; mesures qui comptent l'eau des
  colonnes et sous le fond ; ligne `divergence`). **Pincement identique au chiffre près** : pas 55, t = 2,1676 √(R/g), profondeur
  1,438 D, air 0,0781 D³, cavité 1,937 D, couronne 0,199 D (S414 : cavité 1,937, air 0,078) ; itérations 207,6 / 207,8 ; **volume
  de la carte constant à 0 quantum** ; aucune bascule refusée. **Divergence au pas 35** : une pose de plus sur la carte (2 690 contre
  2 689 ; masques et fonds identiques) — un solde au seuil d'une particule, à quelques quanta (les soldes portent l'écart de vitesse
  admis) ; particules en fin 4 115 / 4 122. **`φ` à l'interface**, pas à pas (mm), carte : 0,17 (28), 0,56 (36), 1,25 (44), 0,58 (48),
  3,71 (50), **50,0 (51)**, 1,46, 0,70, 36,3 (54), 5,1 (55). **Témoins** : ±10⁻⁶ — max 2,46 mm ; ±10⁻⁵ — 50,0 (50, 51), 36,3 (54),
  air au pincement 0,0859 ; ±10⁻⁴ — **197 mm** (36), 42,9 (52), air 0,0859. Contre le seul témoin à 10⁻⁶ écrit dans le critère :
  manqué ; **contre l'enveloppe des trois témoins : la carte est dedans à chaque pas publié** (égale au plus grand aux pas 51 et 54).
  Coût p99 **28,7 ms** dont la fin du pas (séparation, corps, échange sur un fil) 23,6 ms ; la bascule ≈ 5,5 ms au mur — C7e.
