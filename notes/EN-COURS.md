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

Session : S452 — **terminée**. Verdict de l'utilisateur (2026-10-02) : *« Je valide le render »* — **R37 reçu** (REVUE-VISUELLE §42).

**Ce que la session fait — le rendu en direct, sur la carte.** Le module `surface_carte` de l'afficheur, sur le device de la carte de
la bande (`ApicCarte`), lit `φ` (`cellf`) et le masque des colonnes (`cmask`) sans retour au CPU : (1) **une passe de calcul** fond `φ`
au raccord bande | colonnes — le `champ_rendu` de S451, porté (deux passes d'une moyenne 3 × 3 sur les colonnes à moins de deux mailles
d'une frontière) ; (2) **une passe de fragments** — un rayon par pixel dans le champ fondu (le quart reflété en entier, échantillonnage
trilinéaire aux centres des mailles), le premier zéro par pas d'une demi-maille puis bissection, la normale au gradient, l'ombrage de
R37 (Lambert, Fresnel de Schlick, reflet du ciel, reflet du soleil), la sphère en gris (intersection analytique). Le banc
`--surface-carte` mène B10 en bande étroite sur la carte (la référence donne le pas) et capture une image aux instants de R37.

**Critères, écrits avant.** (1) **le fondu porté** : le champ rendu de la carte égale le `champ_rendu` du CPU sur le même `φ` à 10⁻⁵
près ; (2) **le coût** : le rendu d'une image de 960 × 600 en **1 ms au plus** (médiane, sur la carte) ; (3) quatre images de la
carte, aux instants de R37, **comparables à R37** — montrées à l'utilisateur.

### Plan

- [x] **P1** — R37 inscrit ; jeton, plan seul.
- [x] **P2** — `surface_carte` (le fondu, le lancer de rayons) ; le banc `--surface-carte` ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `viewer/src/surface_carte.{rs,wgsl}` : le fondu (deux passes de calcul, `cellf` → `tmp` → `champ`), le lancer de rayons
  (une passe de fragments : pas d'une demi-maille, bissection, normale lissée, ombrage de R37, sphère analytique) ; accès de la carte
  (`gpu`, `phi_buffer`, `mask_buffer`, `grid`) ; le banc `--surface-carte` (B10 en bande étroite, 68 s). **Mesuré** (RTX 5070, Dx12) :
  (1) écart du fondu **7,2·10⁻⁷** — tenu (10⁻⁵) ; (2) **0,23 ms** par image, horodatée (médiane de 200 ; mur 0,69 ms) — tenu (1 ms) ;
  (3) `captures/s452/carte_t{0.5,1.0,2.0,3.0}.png` — à l'œil, celles de R37 (le cratère, le jet), un peu plus lisses — envoyées.
- **P3** — SURFACE-CONTINUE-S450 (§ S452) ; journal ; jeton libre ; maillons 9 (justifiés : S406) ; suivant : S453, le rendu dans la boucle vivante de l'afficheur, puis C10.
