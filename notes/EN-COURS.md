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

Session : S659 — **en cours**. En autonomie vers la v2. **2.7, la bathymétrie 2D** (le point qui en débloque six) : il manque la
bathymétrie 2D et la **diffraction des hauts-fonds isolés**, où les rayons de S583 font des caustiques. Le meilleur chemin au réalisme
visé (S643) : un modèle de houle côtière cuit par rivage. Première pièce : **le modèle parabolique de pente douce** (Radder 1979) —
réfraction et diffraction ensemble, une marche en `x`, Crank–Nicolson, tridiagonal en `y`.

**L'équation** (dérivée au plan, de `∇·(p∇φ) + k²pφ = 0`, `p = C·C_g`, `φ = A·e^(i∫k̄dx)`, `A_xx` négligé) :
`A_x = −(p·k̄)_x/(2p·k̄)·A + i/(2p·k̄)·[(p·A_y)_y + p·(k² − k̄²)·A]`, `k̄(x)` la moyenne de `k` sur `y`. À une dimension, elle redonne
`A ∝ (p·k)^(−½) ∝ C_g^(−½)` : la levée par le flux d'énergie.

**La référence : les mesures de Berkhoff, Booy et Radder (1982)**, le haut-fond elliptique, lues dans l'exemple public de Basilisk
(`basilisk.fr/src/examples/section-2, -3, -5, -7`, le rapport d'amplitude mesuré) : section 2 à x = 3 m, section 3 à x = 5 m, section 5 à
x = 9 m (profils en `y`), section 7 sur `y` = 0 (profil en `x`). La géométrie (vérifiée dans le même exemple) : `h₀` = 0,45 m ; pente
1:50 tournée de 20° (`x′ = x·cos 20° − y·sin 20°`, `y′ = x·sin 20° + y·cos 20°`, montée `(5,82 + x′)/50` pour `x′ ≥ −5,82`) ; le haut-fond
`(x′/3)² + (y′/4)² ≤ 1`, épaisseur `−0,3 + 0,5·√(1 − (x′/3,75)² − (y′/5)²)` ; T = 1 s.

**Contrôles du plan** (ADR-266)

- **témoin** : sans objet au départ ; un écart aux mesures se localise ensuite par les deux cas analytiques (le plat, la levée) et le
  pas de maille.
- **instrument** : le lecteur est `|A|` ; éprouvé sur deux cas de réponse connue — l'onde plane sur fond plat (`|A|` = 1) et la levée à
  incidence normale sur une pente 1:50 de 0,45 à 0,15 m (0.990551, `√(C_g0/C_g)`, ce script).
- **calcul** : `k(h)` par Newton, la levée de référence, la profondeur minimale (0.1336 m > 0, asserté), par ce script.
- **ADR** : ADR-264 (le calcul en grilles locales), ADR-260 (un module `f64` rangé : O, un outil de cuisson), ADR-262, ADR-263 D2.
- **pièges** : **l'axe `y` des mesures est inversé** (Basilisk trace `-$1`) ; la normalisation (les mesures sont des rapports à
  l'amplitude incidente) ; l'erreur de l'approximation parabolique aux angles obliques (la pente est tournée de 20°) ; les parois
  latérales (réfléchissantes, `A_y` = 0) ; la non-linéarité de l'expérience, que le modèle linéaire ne rend pas.

**Critères, écrits avant.** (1) Le plat : `|A|` = 1 à 10⁻⁶ sur 20 m. (2) La levée à incidence normale : à 0,5 % de 0.9906. (3) Berkhoff,
à la maille fine : l'écart quadratique moyen du rapport d'amplitude aux mesures **≤ 0,20 sur chacune des quatre sections**, et le pic de la
section 3 (2,21 mesuré) à 15 % ; deux mailles rapportées.

### Plan

- [x] **P1** — jeton ; plan ; les mesures récupérées.
- [ ] **P2** — `pente_douce.rs` et ses essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 2.7, 3.6 ; rituel.

### Notes de reprise
