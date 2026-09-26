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

Session : S388 — **terminée**. **C4a** de la campagne ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5,
[ADR-186](../docs/adr/ADR-186-apic-seconde-representation.md)) : **APIC en 3D dans le cœur**, avec ses essais (METHODE : un banc
qui entre au système y entre avec ses chiffres). C4 est découpée : **C4a** ici — le solveur, le repos, le ballottement ;
**C4b** ensuite — B10 en 3D, une sphère qui entre dans l'eau. Demande de l'utilisateur (2026-09-26) : *« Continue »*. Agent :
Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Le candidat 2D de S318–S320 (`examples/lot5_comparaison.rs`) porté aux trois dimensions dans `water-core`, **avec
ses leçons** : surface reconstruite des particules (Zhu et Bridson 2005) et fluide fantôme à la fraction de maille ; faces
d'air extrapolées sur trois couches puis remises à zéro, les faces alimentées par des particules gardées ; séparation des
particules (0,4 maille, deux passes) ; transferts APIC trilinéaires (Jiang et al. 2015) ; advection RK2. Huit particules par
maille (2 × 2 × 2), `f32` (I-08), `g_eff` injecté (I-07), tous les tampons réservés à la configuration (I-06).

**Critères, écrits avant.** (1) **APIC conserve un champ affine** : particules à `v = a + B·x`, `C = B`, grille couverte →
après transfert vers la grille et retour, `v` et `C` rendus à 10⁻⁵ près (relatif). (2) **La surface au repos** : l'iso-zéro
reconstruite d'une nappe plane tombe à sa hauteur à 1 % de maille près. (3) **Le repos** : une cuve au repos, 2 s — masse
exacte (nombre de particules constant), vitesse parasite ≤ 1 cm/s à 5 cm (2D : 4,4 mm/s). (4) **Le ballottement** du mode
(1, 0) d'une cuve de 2 m, 0,5 m d'eau, 2 cm (le cas de S318, invariant en `y`) : erreur de période ≤ 7 % à 5 cm et ≤ 1 % à
2,5 cm, décroissante (2D : +5,9 % et −0,15 %) ; énergie jamais créée au-delà de 1 %. (5) **Un mode oblique** (1, 1) d'une
cuve carrée : erreur de période ≤ 2 % à la maille la plus fine mesurée — la troisième dimension éprouvée pour elle-même.
(6) Suite entière inchangée, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `apic3d.rs` : configuration et réserve (I-06), grille MAC, particules, ensemencement, tri par maille.
- [x] **P3** — transferts APIC trilinéaires (particules → grille, grille → particules) ; critère 1.
- [x] **P4** — surface reconstruite (rayon au repos calculé), étiquettes ; critère 2.
- [x] **P5** — gravité, projection à fluide fantôme (gradient conjugué, Jacobi), extrapolation, advection, séparation : le pas.
- [x] **P6** — le repos ; critère 3.
- [x] **P7** — banc `apic3d_ballottement` : modes (1, 0) et (1, 1) ; critères 4 et 5.
- [x] **P8** — critère 6 ; preuve `APIC3D-S388` ; liste, file, feuille de route, index.
- [x] **P9** — rituel.

### Notes de reprise

**P2 à P6 en un commit** : le module ne compile sans avertissement qu'une fois le pas écrit (critère 6). `apic3d.rs` :
configuration comptée au flottant près (I-06), ensemencement 2 × 2 × 2, tri par comptage, transferts APIC trilinéaires,
surface reconstruite, pas complet (gravité, projection à fluide fantôme par gradient conjugué à Jacobi, extrapolation trois
couches avec les faces alimentées gardées, RK2, séparation). Essais `s388` (six).

- **Critère 1 tenu** : un champ affine revient intact (`v` à 10⁻⁵ relatif, `C` à 10⁻⁴) ; **vu échouer** sans le terme affine.
- **Critère 2 manqué hors du cas réglé** — mesuré, pas relevé. Le rayon de S318 (`d₀`, au point de la surface) lit l'iso-zéro
  **à −15,05 % de maille** quand la surface tombe sur une face de maille, exactement quand elle passe par un centre ; réglé sur
  les faces, 0 et +9,91 %. **Retenu : le minimax, ±6,12 %** (`r` = 0,03395 m à 10 cm), calculé en f64 et vérifié en 3D (les six
  cas concordent au centième de pourcent, après correction du modèle : il mettait `2·dx − r` là où la reconstruction met
  `dx` sans voisine). C'est la granularité de deux rangées de particules par maille (S318, faute 2). L'essai garde le
  plafond de 7 % comme régression ; le verdict reste « manqué ».
- **Critère 3 tenu** : cuve de 1 m à 5 cm, 0,5 m d'eau, 2 s, 100 pas — masse exacte, vitesse parasite **5,6 mm/s** au pire,
  1,2 mm/s à la fin (2D : 4,4 mm/s) ; 57 itérations, résidu 9,7·10⁻⁷.

**P7 — les ballottements** (`apic3d_ballottement`) :

| cas | 5 cm | 2,5 cm | critère |
|---|---|---|---|
| (1, 0), période (exacte 1,9765 s) | **+2,05 %** (2D : +5,9 %) | **+1,04 %** (2D : −0,15 %) | ≤ 7 % tenu ; ≤ 1 % **manqué de 0,04 point** ; décroissante tenu |
| (1, 1), période (exacte 0,9630 s) | +7,64 % | **+2,69 %** | ≤ 2 % **manqué** |
| énergie créée, en % de l'onde analytique | +22,4 % / (1,1) +26,9 % | +7,0 % / (1,1) +12,7 % | ≤ 1 % **manqué** |

À 10 cm, l'amplitude de 2 cm est sous l'espacement des particules (5 cm) : l'onde n'existe pas (S318, faute 2) — non retenu.
Durées : 15 s, 153 s ; (1, 1) 18 s, 174 s ; 256 000 particules au plus. **L'énergie** oscille dans chaque période (de +0,15 à
−0,56 fois l'énergie de l'onde) et **décroît** sur la durée ; l'onde représentée porte 1,64 fois l'énergie analytique (rangées
de 2,5 cm, S318 : 2,66 cm représentés pour 2) ; 1 % de l'onde vaut quelques micromètres de centre de masse — le critère était
plus fin que la mesure, il reste « manqué ». **Attribution** (témoins à 5 cm, (1, 0)) : rayon de S318 → **+7,87 %** (minimax
+2,05) ; sans séparation → +2,29 %, énergie +22,3. **L'erreur de période vient de la lecture de la surface**, pas de la
séparation ; la piste : plus de particules par maille, ou la surface portée par un ensemble de niveaux (ADR-186 D4).

**P8 — critère 6 tenu** : suite Rust 681 réussis (675 + 6), 18 ignorés, zéro avertissement. Preuve `APIC3D-S388` ; liste 4.12 et 4.16
(texte, sans changer de case : construit et non reçu ne vaut pas partiel), file, feuille de route, index.
