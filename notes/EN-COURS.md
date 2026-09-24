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

Session : S341 — **en cours**. **Porte C, la mesure qui manque** ; chemin de la v1
([ADR-174](../docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4), porte en cours de §3 bis depuis S340.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — l'utilisateur : *« Continue »*, après la porte B reçue (S340). Le terme de D2 (lot 5) reste sans réponse :
le lot 5 reste suspendu.

**Ce que la porte C demande** ([ADR-175](../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4.4) :
le pas de δ tient **2 ms GPU au 99ᵉ centile de la contribution par image**, sur la scène de la porte B, machine de
référence, **techniques présentes et absentes publiées** (ADR-131 D3). **Ce qu'on sait** : 4,62 ms par pas en
médiane de banc, 30 pas, 32 cycles, 376 320 mailles, 177 dispatchs (S302). **Ce qu'on ne sait pas** : le 99ᵉ
centile, et où vont les 4,6 ms — fond, prédiction, couplage, projection, correction, transport. Choisir un levier
sans cette décomposition serait deviner.

Critères, écrits avant le code :
1. **L'horodatage par passe** : début et fin de chacune des trois passes du pas, les copies entre elles comprises
   dans l'écart ; le pas de production **inchangé au bit** — la surface publiée après 60 pas identique avec et
   sans horodatage.
2. **Le banc** (`--delta3d-cout-scene`) : la scène de la porte B (`Config::review`, 32 cycles), 1 000 pas horodatés
   après 30 de chauffe : médiane, 99ᵉ centile et maximum du pas entier et de chaque passe ; la projection à 0, 8,
   16, 32 et 64 cycles — coût par cycle et part fixe.
3. **Le domaine** (ADR-131 D3, A270) : alimentation relevée au début et à la fin ; témoin — le coût de S302,
   `--delta3d-scene-mesure`, rejoué dans le même état ; ce que la grandeur mesure et ne mesure pas.
4. **La preuve** : techniques présentes, absentes, domaine ; la part de chaque étage ; **le premier levier**
   choisi sur la mesure, avec ce qu'il doit publier.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'horodatage par passe ; critère 1.
- [x] **P3** — le banc, le témoin, l'alimentation ; critères 2 et 3.
- [x] **P4** — la preuve, le premier levier ; file, feuille de route ; critère 4.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** Six horodatages — début et fin des trois passes — et `timed_step_passes` ; `timed_step`
  garde son sens (début de la première passe, fin de la dernière). `--delta3d-horodatage` : scène de la porte B,
  60 pas horodatés contre 60 nus depuis le même état — **0 colonne différente au bit** sur 13 440, 60 horodatages.
- **P3, critères 2 et 3 tenus** (`--delta3d-cout-scene`, deux passages ; secteur au début et à la fin des deux,
  `BatteryStatus` 2, charge 97 %, `PowerOnline` vrai ; témoin S302 `--delta3d-scene-mesure` : 4,630 ms, pour 4,62
  publiés). **Pas entier**, 1 000 pas : médiane 4,452 / 4,477 ms, **99ᵉ centile 4,505 / 4,651**, max 4,834 / 4,852.
  **Par passe**, médianes : fond et prédiction 1,98–1,99 ms, dont **l'évaluation du fond seule 1,533** (q99 1,554) ;
  projection 2,06 ; correction et transport 0,39 ; copies 0,03. **Projection = 0,087 ms + 0,062 ms par cycle**
  (0 → 0,093 ; 8 → 0,582 ; 16 → 1,073 ; 32 → 2,068 ; 64 → 4,056). Pas entier à 0 cycle : 2,53 ms.

