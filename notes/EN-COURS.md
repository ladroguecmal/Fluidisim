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

Session : S349 — **en cours**. **Porte A, un domaine qui se déplace** ; la dernière porte de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S348 : porte C reçue sur le banc ; la v1 ne demande plus que la porte A. Premier critère tenu au banc
(S344) : deux domaines se disputent un budget — mais un domaine y **vit ou meurt**. Deuxième critère : un domaine
qui **se déplace et se redimensionne** au lieu d'être seulement allumé ou éteint. Cette session : **se déplacer**.

**Ce qui le permet.** Dans le pas de production, la position du domaine n'entre que par l'évaluation du fond de B
(un uniforme) ; le reste travaille en indices locaux. Déplacer le domaine de `(di, dj)` mailles, c'est **décaler son
état** — vitesses aux faces, surface et son reste compensé, pression de départ, surface publiée — et **déplacer
l'origine** du fond. Les mailles qui entrent naissent au repos, δ = 0 (I-12) ; celles qui sortent sont perdues, et
leur volume compté.

Critères, écrits avant le code :
1. **Le décalage** (`Step3::shift`) : sur la scène de B, après un décalage de (+3, −2) mailles, chaque tableau
   décalé est **identique au bit** à l'ancien translaté dans le recouvrement, au repos ailleurs ; zéro allocation
   (tampon de travail réservé à la configuration).
2. **Un domaine qui suit la caméra** le long de la côte de S344 : il se décale vers le point regardé, au plus deux
   mailles par image ; aucune colonne hors bornes ; sa part d'écran reste celle d'un domaine vu de face ; **aucune
   naissance ni extinction** pendant le trajet, là où deux domaines fixes en demandaient deux ; coût du décalage
   publié.
3. Preuve, file, feuille de route ; le redimensionnement et le rang 1 restent.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le noyau de décalage, `Step3::shift`, l'origine du fond ; critère 1.
- [x] **P3** — le domaine qui suit la caméra ; critère 2.
- [ ] **P4** — preuve, file, feuille de route ; critère 3.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** `delta3d_shift.wgsl` et `Step3::shift(di, dj)` : sept tableaux — `u`, `v`, `w`, surface,
  reste compensé, pression de départ, surface publiée —, chacun recopié dans un tampon de travail réservé à la
  configuration puis réécrit décalé ; l'origine du fond avance. `--delta3d-decalage`, scène de B après 30 pas,
  décalage (+3, −2) : **0 valeur différente au bit** dans les recouvrements (363 440 faces `u`, 363 636 `v`, 373 230
  `w`, 12 870 colonnes, 360 360 mailles), entrants au repos ; origine (−15 ; 0) → (−14,25 ; −0,5). Empreintes du pas
  de S343 inchangées.
- **P3, critère 2 tenu** (`--delta3d-suivi`, côte de S344, budget 5 ms, 1 200 images). **Un seul domaine** se décale
  vers le point regardé : **480 décalages d'une maille** (120 m), un par image pendant les trajets ; part d'écran
  **0,1629 constante** après 1 s — un domaine vu de face — ; **1 naissance (au départ), 0 extinction**, là où deux
  domaines fixes en demandaient deux de chaque (S344) ; **aucune colonne hors bornes**. Décalage, temps réel de la
  soumission à la fin sur la carte : **médiane 1,463 ms, max 4,872** — sept soumissions séparées ; les grouper en une,
  avec sept uniformes, est la suite évidente. `diagnostics_now` et `wait` ajoutés pour le banc.

