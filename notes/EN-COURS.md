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

Session : S359 — **en cours**. **Rendu 3 (ADR-191 D3) : l'eau a une épaisseur** — la colonne d'eau dans Godot.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — **verdict R19** de l'utilisateur : *« selon moi la mer n'est pas du tout crédible, mais c'est pas grave on
continue, car je pense qu'il manque plein de chose avec la trnasparence en fonction de la prfondeur etc... »*. Relu sur
les images : l'eau est un aplat opaque — ni lumière qui y entre, ni fond, ni réfraction — et **l'horizon s'assombrit**
au lieu de refléter le ciel clair. Liste **8.5** (transparence, réfraction, caustiques) : *absent*.

Critères, écrits avant le code :
1. **R19 consigné** tel quel (REVUE-VISUELLE §24, décisions de la file).
2. **Le fond** : bathymétrie de démonstration — pente de sable de 6 m sous la caméra à 40 m vers 400 m, jamais sous
   2·Hs (Hs ≈ 2,5 m), puis le large ; texture procédurale, albédo 0,3 *à calibrer* ; aucun téléchargement. **B ne voit
   pas le fond** (2.7 absent) : déclaré, pas caché. Poses côtières ajoutées ; la scène du large reste.
3. **La profondeur reconstruite** depuis le tampon de profondeur, en mode contrôle (mer plate, tonalité linéaire),
   contre la bathymétrie connue en cinq points : écart ≤ 2 % + 5 cm.
4. **L'optique** : Maritorena, Morel et Gentili (1994) sur le trajet oblique — fond × `exp(−Kd·(H + L))` + corps d'eau
   × `(1 − exp(−Kd·(H + L)))`, `Kd = (a + b_b)/μ̄_d` des coefficients d'ADR-177, `μ̄_d` = 0,8 *à calibrer* ; réfraction
   de Snell (n = 1,34) en espace écran ; transmission `1 − F` de Fresnel exact. Contrôle au nadir, mer plate : la
   transmission rendue suit `exp(−2·Kd·H)` à trois profondeurs, à 0,01 près (8 bits). Sans fond, la mer du large ne
   change pas de couleur (écart moyen < 1/255 sur les poses de R19).
5. **L'horizon, diagnostic** : hypothèse — la rugosité confiée à Godot éteint sa réflexion rasante (approximation de
   l'environnement) ; témoin : rugosité bornée ; mesure : luminance de la mer sous l'horizon rapportée au ciel dessus.
   Selon le résultat, correction dans la session ou point de file.
6. **Images R20** : poses côtières et du large, contre R19 ; questions et références demandées.
7. Preuve, file, liste 8.5, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — verdict R19 consigné ; diagnostic de l'horizon ; critères 1 et 5 (mesure).
- [x] **P3** — le fond et ses poses ; critère 2.
- [x] **P4** — la profondeur reconstruite, le mode contrôle ; critère 3.
- [ ] **P5** — l'optique de la colonne ; critère 4.
- [ ] **P6** — l'horizon, selon P2 ; critère 5.
- [ ] **P7** — les images de R20 ; critère 6.
- [ ] **P8** — preuve, file, liste, index ; critère 7.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2.** R19 consigné (REVUE-VISUELLE §24, file). `outils/horizon_mer.py` : horizon = plus forte chute de luminance
  d'une rangée à la suivante ; bandes de 20 rangées à 3 de la ligne ; sRGB → linéaire, Rec. 709. Rapport mer/ciel —
  Godot (R19) rasante **0,153**, proche **0,182** ; afficheur rasante **0,713**, proche **0,728**. Témoin
  `RUGOSITE_MAX=0.05` (uniforme `rugosite_max`) : **0,414** et **0,482**. Reste attendu : `SPECULAR` 0,25 donne à Godot
  F0 = 0,16·0,25² = 0,01 (l'eau : 0,020) ; et son approximation de l'environnement. L'afficheur, lui, calcule sa
  réflexion (ciel clair, nuages, Fresnel, pente non résolue par quadrature de Gauss-Hermite, ADR-161) : c'est ce qui
  se porte. **Critère de P6, écrit ici avant le code** : rapport mer/ciel de Godot à ± 15 % de celui de l'afficheur,
  aux poses proche et rasante.
- **P3.** `godot/sol.gdshader` (sable procédural, albédo (0,34 ; 0,30 ; 0,24), rides de 0,7 m) ; `mer.gd` :
  `--cote` ajoute le fond, `profondeur(x, y)` en coordonnées de B (6 m à y = −30, 40 m à y = 370, puis 300 m ; bancs
  d'un mètre ; jamais sous 5 m), grille de 4 m sur 2 400 × 1 100 m ; pose `plongeante` (12 m, −0,75 rad), touche 5 ;
  captures suffixées `_cote` ; `SANS_EAU=1` masque la mer (instrument). Fond vu seul : rides visibles, gris sous le ciel.
- **P4, critère 3 tenu.** `scene_monde` : NDC `(uv·2 − 1, d)`, profondeur inversée de Godot 4.3+, puis
  `INV_PROJECTION_MATRIX`, `INV_VIEW_MATRIX` ; la mer lit `hint_screen_texture` et `hint_depth_texture`. `mer.gd
  --controle-fond` : mer plate, émission seule, lumières, ambiance, reflets, brume, halo éteints, tonalité linéaire ;
  caméra à 15 m, −0,9 rad ; la profondeur attendue par le rayon du pixel contre `profondeur()` (pas de 5 cm, puis
  dichotomie). Cinq pixels : 8,413 / 8,443 ; 8,894 / 8,948 ; 9,560 / 9,592 ; 7,498 / 7,543 ; 7,796 / 7,824 m — écart
  −2,7 à −5,4 cm, pire 0,23 de la tolérance. Biais de signe constant : le pixel entier contre son centre, non poursuivi.
  *Battements de P2 et P3 écrits en avance d'une et deux minutes — corrigé ici.*
