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

Session : S652 — **terminée**. En autonomie vers la v2 ; le rouleau 3D. **Étape 5 — le rouleau qui agit** (6.7 : « le rouleau plongeant
qui décolle un nageur »).

**L'instrument, construit et éprouvé d'abord (ADR-263 D2).** La force de l'eau sur la sphère d'APIC (S393), qui impose sa vitesse mais
ne recevait rien : la somme, sur les faces entre une maille du corps et une maille d'eau, de la pression de la maille d'eau fois `dx²`,
dirigée vers le corps. Cas connu : 40 × 8 × 20 mailles de 5 cm, eau à 0,3 m, au repos, sphère `r` = 0,1 m centrée en (1,0 ; 0,2 ;
0,15) m, immergée : **32 mailles de corps, `V` = 0.004000 m³, Archimède `ρgV` = 39.240 N** (`ρ` = 1000, `g` = 9,81).

**Le rouleau.** Le relais de S650, élargi à 8 mailles (0,4 m), une sphère fixe `r` = 0,1 m à x = 10,4 m (après le retournement de
S650, 9,825 m), posée sur sa marche (0.40 m) : centre à z = 0.50 m, à demi immergée au repos (le niveau à 0,5 m).

**Critères, écrits avant.** (1) L'instrument au repos : `F_z` à 15 % de `ρgV` (le biais de la pression lue au centre de la maille voisine,
une demi-maille hors de la face, est rapporté) ; `|F_x|`, `|F_y|` ≤ 1 % de `F_z`. (2) Sous le rouleau : le pic de `F_x` vient après le
retournement, dans la seconde qui suit ; le coefficient de traînée effectif `F_x,max / (½ρ·A·u_max²)`, `A` = πr² = 0.0314 m², `u_max` la
plus grande vitesse horizontale de la grille trois mailles avant le corps, à mi-hauteur d'eau au repos, est dans [0,5 ; 3] — l'ordre de la
traînée d'un corps trapu (Morison). (3) La masse, comme S650. (4) Rapportés : l'impulsion `∫F_x dt` ; à l'échelle de la nature ×20 (Froude :
forces ×8 000), la force, et le produit `h·u` au corps contre la règle d'ADR-018 (0,5 m à 2 m/s emporte un adulte : 1 m²/s).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'instrument et son épreuve ; le rouleau sur le corps ; (1)–(4).
- [x] **P3** — preuve ; liste 6.7, 4.14 ; rituel.

### Notes de reprise
- **P2 en cours (interrompu : limite d'usage)** — l'instrument de force construit ; son épreuve (critère 1) a d'abord lu 1,38 × Archimède
  (la pression au centre de la voisine, une demi-maille hors de la face) ; corrigé avant toute mesure du rouleau (la pression extrapolée à
  la face) : **1,1285 × Archimède, `F_x`, `F_y` nuls — critère 1 tenu** (≤ 15 %). Reste : lancer `the_plunging_roller_pushes_a_body_s652`
  (`--ignored`, ≈ 6 min, sur une copie du binaire, ADR-265 D1), puis la preuve, 6.7, le rituel.
- **P2 fini** — (1) tenu après correction (1,38 → 1,13) ; (2) le pic après le retournement (+0,15 s), **le `C_d` manqué** (18 brut ; lissé
  5,1 ou 0,33 selon la vitesse de référence) — le pic est un choc de deux pas (A334) ; (3) tenu ; (4) à ×20, `h·u` 24 m²/s.
