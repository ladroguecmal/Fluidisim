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

Session : S419 — **terminée**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7c-3**, le fond de la
bande sur la carte ([conception](../docs/validation/APIC-CARTE-S416.md) §8). Agent : Claude Code (Opus 5.5), au poste.

**Ce que la référence fait avec un fond** (S413, ADR-212) : sous le fond d'une colonne de la bande, l'eau est **à la grille** —
`φ = z − fond`, mailles d'eau, faces advectées comme la zone, particules virtuelles jusqu'au fond pour la reconstruction ; le
transport charge le **solde vertical** de la colonne (rangées sous le fond) ; l'échange absorbe une particule passée sous le fond
(solde vertical, mélange aux faces à la grille) et règle le solde vertical (retrait de la plus basse, pose à `dx/16` au-dessus du
fond) ; la frontière latérale se lit **maille par maille**. **Le déplacement du fond** (`move_band_floor`) est piloté par la bascule :
il part avec elle en C7c-4 (déclaré ici, pas un oubli).

**Critères, écrits avant.** (1) Étages de la cuve en bande étroite (`APIC3D_FOND=4`, tout en bande, S413) à l'arrondi : `φ` et
étiquettes, advection, projection, soldes verticaux après le transport (à l'écart que fixe la vitesse admise, S418) ; un pas entier :
gestes, `n` identiques, positions à 10⁻⁵ m indice pour indice. (2) **Le ballottement en bande étroite**, 5 cm, 30 s : surface à
3 mm de la référence **ou au plus au niveau du témoin** (±10⁻⁶ m/s, même instrument) ; volume de la carte constant exactement ;
période publiée. (3) Raccord, B10, colonnes, ballottement inchangés ; suite ; zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — cœur : accès au solde vertical. Carte : le fond (tampon, paramètre), étiquettes et `φ` sous le fond, virtuelles
  jusqu'au fond, advection sous le fond ; banc `CAS=fond` : étages jusqu'à l'advection.
- [x] **P3** — le transport sous le fond : soldes verticaux et latéraux des rangées à la grille ; banc.
- [x] **P4** — l'échange avec un fond : absorption sous le fond, frontière maille par maille, solde vertical (retrait, pose) ; un pas
  entier.
- [x] **P5** — le ballottement en bande étroite, 30 s, et le témoin ; critère 2.
- [x] **P6** — non-régression, suite ; preuve §11 ; liste, file, feuille de route, index.
- [x] **P7** — rituel.

### Notes de reprise
- **P2** — cœur `columns_solde_w`. Carte : `Params` à 176 octets (`floors`), le fond dans `cols[2C + 32 + col]`, `floor_of`, `below_floor`, `grid_at` ; sous le fond `φ = z − fond`, eau ; virtuelles jusqu'au fond ; advection des faces à la grille (une `w` au-dessus d'une maille sous le fond). `band_state` (`CAS=fond`, fond à 4 mailles sous le creux, 5 116 particules) : **`φ` 8,4·10⁻⁷ m, étiquettes identiques**, advection 1,8·10⁻⁷ m/s, projection 94/94, positions 1,2·10⁻⁷ m après advection. L'échange diffère (68 absorbées sous le fond, 32 posées côté référence) : P4.
- **P3** — le transport sous le fond : à une frontière, les rangées sous le fond de la bande vont à son solde vertical ; entre deux colonnes de la bande à fond, les rangées à la grille (mailles pleines, sans mouillage) aux soldes verticaux et, sous le fond d'une seule, au solde latéral. Chaque face garde ses deux contributions (`to_low`, `to_high`) ; `floor_update` les rassemble par colonne — sans atomique. Bande étroite : **soldes à 8,2·10⁻¹⁰ m³** au pire sur 1,6·10⁻⁵ (882 quanta), sous la borne de l'écart de vitesse admis (≈ 3·10⁻⁹) ; raccord inchangé (retraits 32/32, 6·10⁻⁸ m).
- **P4** — l'échange avec un fond : `absorb_mark` prend aussi les particules passées sous le fond ; `absorb_one` sous le fond (mélange
  aux faces à la grille, `floor_face`, solde vertical) ; frontière latérale lue maille par maille (`grid_at`) ; (4) le solde vertical
  (retrait de la plus basse au-dessus du fond, pose à `dx/16` au-dessus, sous-réseau le plus libre). Bande étroite, un pas après
  5, 40, 70 pas : **gestes, `n` identiques, positions à 1,2·10⁻⁷ m indice pour indice** (absorbées 12 et 2, posées 8 et 4) ; raccord
  inchangé. **Après 20 pas** (68 absorbées, 32 posées, identiques) : **3 posées sur 32** à l'emplacement miroir en `y` (même maille,
  même hauteur, sous-réseau 0,25 ↔ 0,75) — critère 1 manqué pour elles. Cause : le cas est **invariant en `y`** (le mode (1, 0), un
  réseau symétrique) ; les emplacements miroirs sont à des distances quasi égales, et les entrées diffèrent déjà de 1,2·10⁻⁷ m
  (l'advection à l'arrondi). Essais : une tolérance « le premier gagne à 10⁻⁶ près » **aggrave** (7 sur 32 : la référence les
  ordonne vraiment, à l'arrondi) — retirée ; les carrés évalués sans contraction (`square_sum`, barrière `bitcast`) : sans effet ici,
  gardés (plus fidèles). Masse identique ; le jugement se fera sur la durée, contre le témoin (P5).
- **P5** — `CAS=fond --apic3d-carte-ballottement` (le même instrument que le raccord ; `TEMOIN=`) : 30 s, 1 500 pas — **surface à
  1,45 mm** au pire (t = 18,7 s ; maximum courant 0,02 mm à 4 s, 0,57 à 8 s, 1,02 à 12 s, 1,45 dès 20 s), **critère 2 tenu** (3 mm) ;
  période 1,9969 s contre 1,9972 (−0,015 %) ; **volume de la carte constant à 0 quantum** (la référence : 0) ; gestes cumulés 5 617 /
  5 541 absorbées, 360 / 352 retirées, 5 950 / 5 862 posées, `n` 5 093 / 5 089 ; l'ordre diverge au pas 14. **Témoins** : ±10⁻⁶ —
  0,92 mm (−0,005 %) ; ±10⁻⁴ — 1,18 mm (0,000 %). La carte finit au-dessus des deux témoins, sous les 3 mm.
- **P6** — non-régression : ballottement (0,456 mm), B10 (pas 54, 9,195 mm), colonnes (0,003 mm) identiques ; le raccord déplacé par `square_sum` dans sa dispersion (4,41 → 4,48 mm, 5 205 absorbées au lieu de 5 204). Suite **753**, zéro avertissement. Preuve §11 ; liste, file, feuille de route, index.
- **P7** — journal ; jeton libre ; maillons 7 (justifiés : S406) ; suivant : S420, C7c-4.
