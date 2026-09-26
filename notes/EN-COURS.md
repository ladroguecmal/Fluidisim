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

Session : S382 — **en cours**. Verdict R30 : *« Je valide, ajoute l'occultation du ciel puis continue, et ensuite le plus
important le solveur 3D »*. Cette session : **l'occultation du ciel**. Agent : Claude Opus 5.5, application desktop ;
fichiers, git, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Une surface reçoit le ciel qu'elle **voit** : `E = ∫ L(ω)·V(ω)·max(n·ω, 0) dω`. Le pied d'un mur, un angle, le
fond d'un bassin voient moins de ciel — par temps couvert, c'est le seul modelé des volumes (R30 : le bloc de la piscine se
confondait avec le sol). `V` se calcule **sur les objets eux-mêmes**, des occultants analytiques (des boîtes, pour
commencer), et non en espace écran : les nuanceurs de la scène sont non éclairés par Godot (son SSAO ne s'y applique pas)
et l'espace écran oublie ce qui est hors champ. Par azimut (32), l'intervalle d'élévation que masque chaque boîte (un test
de dalles en 2D), réuni dans un masque de 32 bandes **d'égal angle solide** (uniformes en `sin h`) ; la part masquée
pondérée par `max(n·ω, 0)` — et par la luminance de la CIE sous le ciel couvert —, rapportée au total **exact** (forme
close du ciel couvert incliné : `E(β)/Lz = [π(1 + cos β)/2 + (4/3)((π − β)·cos β + sin β)]/3`). Loin des occultants, 1
exactement. Le même calcul vers une direction, le soleil : **les ombres portées** par ciel clair. La même liste
d'occultants servira l'exposition de V à la pluie (ADR-205, pièce 10).

**Critères, écrits avant.** (1) Sans occultant (`OCCULTATION=0`, et la mer, qui n'en a pas) : les 12 images **identiques
au bit**. (2) La part du ciel reçue — CIE et uniforme — égale à une intégration numérique indépendante (numpy, tests
rayon-boîte exacts, grille fine) **à ±0,01**, mesurée sur l'image rendue en au moins huit points d'essai (sol au pied du mur
ouest de 5 cm à 4 m, mur, coin du fond du bassin, surface de l'eau près d'un mur, dessus de la margelle) ; loin des
occultants, **1 exactement**. (3) La forme close `E(β)` égale à l'intégrale exacte à 10⁻⁴ ; contrôles 2 à 4 de S381
tenus. (4) L'ombre d'une arête tombe à `H/tan h_soleil` suivant l'azimut du soleil, **à un pixel près** ; hors de l'ombre,
rien ne change. (5) Coût mesuré ; photographies réelles ; jugement de l'utilisateur (R31).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-206 : la visibilité du ciel par des occultants analytiques, pour le rendu et pour V.
- [x] **P3** — l'orientation exacte du ciel couvert (forme close, remplace l'interpolation de S381) ; critère 3 ; ciel
  clair au bit.
- [ ] **P4** — `occultation.gdshaderinc` : la part du ciel vue (32 azimuts × 32 bandes), totaux exacts ; `eclairage_vu`.
- [ ] **P5** — la scène : `boite()` inscrit ses occultants ; parois et bassin ; `OCCULTATION=0` ; critère 1.
- [ ] **P6** — `--controle-occultation` contre `outils/occultation_ciel.py` (intégration indépendante) ; critère 2 ; coût.
- [ ] **P7** — le soleil occulté : les ombres portées (une direction, quatre sous-échantillons) ; critère 4.
- [ ] **P8** — photographies de temps couvert (pied des murs) ; images de R31 ; REVUE-VISUELLE §36.
- [ ] **P9** — preuve `OCCULTATION-CIEL-S382` ; liste, file, feuille de route, index.
- [ ] **P10** — rituel.

### Notes de reprise

**P3 — la forme close.** `E(β)/Lz = [π(1 + cos β)/2 + (4/3)((π − β)cos β + sin β)]/3` (dérivée par le fuseau entre
l'horizon et le plan de la face, axe commun horizontal : `∫ sin³θ = 4/3`, `∫₀^{π−β} sin ψ sin(ψ + β) dψ = ((π − β)cos β +
sin β)/2`) ; contre l'intégrale numérique (3 000 × 6 000) de 0 à 180° par 15° : **4·10⁻⁸**. Verticale 0,396177.
`ciel_uniforme_incline`, `ciel_couvert_incline` dans `ciel.gdshaderinc` ; `eclairage()` les emploie. Critère 1 : 12 / 12
au bit. Contrôles de S381 : critères 2 et 3 inchangés ; 4 — sol 1,00000, mur ouest 0,6057 pour 0,6055 (attendu corrigé à
la verticale exacte).
