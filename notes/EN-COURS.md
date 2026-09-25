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

Session : S365 — **en cours**. **Rendu 7 : sous la surface** — *« Continue »*, sans verdict R21 à R23 ; alternance
d'ADR-191 D3 après S364 (physique). **À deux maillons**, un lot qui fait avancer une case : **8.6**, *absente* (vue
sous-marine et passage de la surface, [ADR-019](../docs/adr/ADR-019-vue-sous-marine.md), banc B11), qui dépend de 8.5,
partielle dans Godot depuis S359.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Ce qui se construit.** Depuis l'eau : la **surface vue d'en dessous** — la fenêtre de Snell (le ciel réfracté,
radiance × n²) et, au-delà de l'angle critique, la **réflexion totale** ; le **milieu** entre l'œil et ce qu'il voit —
atténuation par canal `exp(−c·d)`, `c = a + b` de Pope & Fry et Morel (les constantes de S359, `b = 2·b_b`), et la
radiance de l'eau elle-même ; le fond éclairé par `exp(−Kd·H)`, ses caustiques. **Hors session, nommé** : la caméra à
demi immergée (ADR-019 §6), les bulles, l'écume vue d'en dessous, les rayons dans l'eau.

Critères, écrits avant le code :
1. **La fenêtre de Snell** : mer plate, caméra à 5 m sous la surface, visée au zénith, champ de 120° : le bord de la
   fenêtre à **48,27°** (`arcsin(1/1,34)`) à **0,25°** près (deux pixels) ; au-delà, la réflexion totale.
2. **Le milieu** : le fond vu de l'eau, transmission relue par un mode de contrôle contre `exp(−c·d)`, `d` recalculé par
   le script sur la bathymétrie analytique, à **1 %** près par canal. Prédiction : le rouge meurt en 13 m (1 %), le vert
   en 78 m, le bleu en 290 m.
3. **R24** : images depuis l'eau (fenêtre sous la houle, visée horizontale, fond et caustiques) ; références réelles
   demandées à l'utilisateur.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la surface vue d'en dessous (`eau.gdshader`, faces arrière) ; le mode immergé dans `mer.gd`, poses
  sous l'eau, mer plate (`MER_PLATE=1`) ; critère 1.
- [>] **P3** — le milieu : le fond et le fond du ciel vus de l'eau (`sol.gdshader`, `ciel.gdshader`) ; contrôle de la
  transmission ; critère 2.
- [ ] **P4** — images R24, REVUE-VISUELLE §29.
- [ ] **P5** — preuve SOUS-MARIN-S365 ; liste 8.6, file, dépendances, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** `optique_eau.gdshaderinc` (R0, gain, Kd, `c = a + 2·b_b`, `sous_eau`, `eau_infinie`,
  `a_travers_l_eau`) ; `eau.gdshader` : `lumiere_dessous` (Fresnel eau → air, ciel réfracté × n², réflexion totale),
  quadrature commune `integree`, ligne de visée jusqu'à l'œil ; `mer.gd` : `immersion()` (bande de B sous la caméra), poses
  `sous_eau_zenith`, `sous_eau`, `sous_eau_fond`, `FOV`, `MER_PLATE`, `CONTROLE_EAU`. **Impasse** : `FRONT_FACING` — la grille
  polaire présente sa face avant **par en dessous** ; premier rendu, le ciel réfléchi d'en haut, gris uniforme. Remède : le
  mode `sous_eau`, d'un bloc. **Mesure** (`outils/fenetre_snell.py`, mer plate, 5 m, champ 120°) : la plus forte chute se
  trompe sur les bords de nuages tassés près de l'horizon réfracté (46,0 à 48,3°) ; deux directions sans nuage, 48,266° et
  48,245°. **Contrôle de Fresnel** (`--fresnel`, premier pixel à R = 1), 16 directions : **48,254° en moyenne, 48,220 à
  48,311, pire écart 0,048°** pour 48,268° (pixel 0,122°). Soleil dans la fenêtre à 88,8 px du centre, 89,5 attendus.
