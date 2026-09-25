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

Session : S366 — **terminée**. **Rendu 8 : les références sous l'eau** — verdict R24 : *« Pour les références trouve les
sinon rien a redire cela me paraît good, continue »*. La demande de l'utilisateur prime sur l'alternance d'ADR-191 D3
(la physique reprend à la session suivante). S365 a laissé **`f(ω)` à calibrer** (B11) — la lumière de l'eau selon la
direction de visée — et aucune référence réelle.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Deux sortes de références.** (a) **Mesurées** : la distribution de radiance sous l'eau publiée (Tyler 1960, lac Pend
Oreille, ou d'autres) — ce que vaut la radiance horizontale devant la radiance montante, et la fenêtre devant le miroir ;
c'est ce qui calibre `f(ω)`. (b) **Photographies** libres (Wikimedia Commons) d'eau claire peu profonde, de jour — fenêtre
de Snell, fond de sable —, chiffrées sans téléchargement, dans le navigateur, sur des rapports que la balance des blancs
et l'exposition ne changent pas. Sources et licences consignées ; aucune image n'entre dans le dépôt.

Critères, écrits avant la mesure :
1. **Au moins une** distribution de radiance mesurée, conditions dites, et **au moins deux** photographies libres.
2. `f(ω)` contre la mesure : rapport horizontale / nadir montant ; au-delà de **30 %** d'écart, recalibrer, avant / après
   publiés. **Prédiction** : la mesure donne 2 à 5 (le modèle, 3).
3. Les photographies contre le rendu, sur des rapports sans unité (fenêtre / miroir voisin, gradient de l'horizontale) —
   à titre indicatif : prise de vue inconnue.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le verdict R24 consigné ; les références trouvées, sources et licences.
- [x] **P3** — les chiffrer : la distribution mesurée contre `f(ω)` et le rendu ; les photographies, rapports sans unité.
- [x] **P4** — calibrer ce que la mesure désigne ; avant / après ; images.
- [x] **P5** — preuve (SOUS-MARIN-S365, section datée S366), liste 8.6, file, index.
- [x] **P6** — rituel.

### Notes de reprise
- **P2.** Verdict R24 consigné (REVUE §29, décisions de la file). **Références trouvées**, rien téléchargé, lues dans le
  navigateur : (a) **mesurée** — Tyler (1960), lac Pend Oreille, radiance dans le plan du soleil à 4,2, 29 et 66 m,
  reproduite par Mobley (*Ocean Optics Web Book*, « The Asymptotic Radiance Distribution », fig. 7 ; normalisée à 1 au
  nadir à 4,2 m), numérisée ici par ses pixels : axe log, 32,75 px par décade ; (b) **photographies** Wikimedia Commons :
  *Dharavandhoo Thila – Hanifaru Bay Sharks* (Shiyam ElkCloner, CC BY-SA 3.0, Maldives, contre-plongée en eau claire),
  *Looking up (6158466637)* (Derek Keats, CC BY 2.0, récif vu d'en dessous), *Snell's window* (petebw, CC BY-SA 2.0,
  piscine : la fenêtre, et le fond **dans le miroir**). Pose `sous_eau_oblique` (4 m, 30°) ajoutée pour le cadrage.
- **P3, critère 2 : écart > 30 %, recalibrer.** Tyler à 4,2 m, numérisé par composantes connexes (18 points, deux
  azimuts, 10° à 90° du nadir) : horizontale **2,46** à l'opposé du soleil, **8,71** côté soleil (moyenne des deux 5,6) ;
  60° : 1,52 et 2,46. Le modèle de S365 (3 et 2) : +22 % à l'opposé, −66 % côté soleil ; résidu logarithmique **0,362**.
  `outils/tyler_radiance.py` : lobe avant de Henyey-Greenstein autour du soleil réfracté, **β = 1,0031, K = 213,35,
  g = 0,855** (a0 = 0,2045) : résidu **0,120** ; pires écarts, côté soleil près du nadir (+25 %) et à l'horizontale
  (−21 %). Prédiction (2 à 5) : tenue à l'opposé, dépassée côté soleil. **Critère 3, indicatif**, photographie de Hanifaru
  (tiers bas / tiers haut, luminance sRGB décodée) : eau B/G **4,07**, rendu 4,72 ; dynamique p90 haut / p50 bas **9,7**
  (la photographie écrête sa fenêtre : borne basse), rendu **3,36** — notre fenêtre est terne devant l'eau.
- **P4.** Le lobe dans `optique_eau.gdshaderinc` (`eau_infinie` ; `indice` y passe ; soleil réfracté par Snell) ;
  `mer.gd` : `LOBE_TYLER`, `LOBE=0` rend S365 ; poses face au soleil et dos au soleil (lacet). **Contrôle** : tonalité
  linéaire, juste sous l'horizon, rapport face / dos **2,23 (vert), 2,18 (bleu)** pour **2,215** prédits (notre soleil à
  23,3° du zénith dans l'eau). **Défaut trouvé** : une bande sombre juste au-dessus de l'horizon, vue d'en dessous — la
  **brume** de Godot (perspective aérienne) sur la surface lointaine ; éteinte quand l'œil est dans l'eau. Au-dessus de
  l'eau, proche et rasante **identiques au bit**. Photographie, après : eau B/G 4,10 (4,07), dynamique 2,72 (≥ 9,7) — la
  fenêtre reste terne : le ciel ne porte pas l'éclairement du soleil que reçoit l'eau (`gain_eau` = 2), à suivre.
- **P5.** Preuve SOUS-MARIN-S365 §6 datée S366 (références, Tyler contre `f(ω)`, contrôle, photographie, brume) et
  « Reproduire » ; REVUE-VISUELLE §30, R25 (une question, sans réponse la lueur reste) ; liste 8.6 (reste *partiel*),
  file (ligne du rendu Godot : échelle radiométrique ciel / soleil ajoutée au déclencheur), feuille de route, index.
