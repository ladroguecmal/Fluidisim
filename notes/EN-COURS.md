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

Session : S342 — **en cours**. **Porte C, premier levier : le fond de δ factorisé** ; chemin de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S341 ([preuve](../docs/validation/COUT-DELTA3D-S341.md)) : l'évaluation du fond de B coûte **1,53 ms** des
4,45 du pas. Le noyau `sample_faces` calcule, pour chacune des 1 148 896 faces et des 64 composantes, une phase,
un sinus, un cosinus et une atténuation. Or la phase ne dépend que de la **colonne** de la face, et l'atténuation
que de sa **couche** : 64 × 1,15 million de calculs pour ce qui en demande 64 × 40 500 et 64 × 29 × 3.

**Ce que la session doit rendre possible.** Un fond de δ évalué moins cher, **au bit** du précédent : un second
noyau, `sample_faces_tiled`, par tuiles de 16 colonnes × 16 couches — sinus et cosinus de chaque colonne et
atténuation de chaque couche calculés une fois dans la mémoire du groupe, puis la même accumulation, dans le même
ordre, avec les mêmes primitives. L'ancien noyau reste : témoin, et repli au-delà de 64 composantes.

Critères, écrits avant le code :
1. **Identité** : sur la scène de la porte B, les 26 champs des 1 148 896 faces identiques au bit entre les deux
   noyaux, à trois instants ; puis 60 pas de production, surface publiée identique au bit. Si le compilateur
   contracte autrement une expression et qu'un bit bouge, l'écart est publié et attribué, pas masqué.
2. **Coût** : le fond seul et le pas entier, médiane et 99ᵉ centile (banc de S341), secteur relevé, témoin.
3. Le nouveau noyau par défaut si ≤ 64 composantes ; preuve (§6 de COUT-DELTA3D-S341) ; file.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le noyau par tuiles et son branchement ; relecture des faces pour le banc.
- [x] **P3** — l'identité au bit ; critère 1.
- [x] **P4** — le coût ; critère 2 ; preuve, file ; critère 3.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, fait.** `sample_faces_tiled` (`delta3d_background.wgsl`) : groupes de 256 fils, tuiles de 16 colonnes × 16
  couches d'une famille ; sinus et cosinus par colonne, atténuation par couche, en mémoire de groupe (12 Ko) ;
  mêmes primitives, même expression de la position, même ordre d'accumulation. Branché par défaut jusqu'à 64
  composantes (`TILE_COMPONENTS`), `sample_faces` sinon ; 5 070 groupes sur la scène de B. Bascule et relecture
  des faces pour le banc.
- **P3, critère 1 tenu** (`--delta3d-fond-tuiles`, scène de B, 64 composantes). Les **29 871 296** valeurs du fond —
  26 champs × 1 148 896 faces — **identiques au bit** entre les deux noyaux, aux pas 0, 50 et 500 ; 60 pas de
  production, l'un par tuiles, l'autre face par face : surface publiée **identique au bit** sur 13 440 colonnes.
- **P4, critères 2 et 3 tenus** (secteur 97 % avant et après). Même session : face par face, fond 1,527 ms
  (q99 1,551), pas 4,456 (q99 4,501) ; **par tuiles, fond 1,237 (q99 1,261), pas 4,348 (q99 4,405)**. −19 % au fond,
  −0,11 ms au pas : le calcul transcendant n'était pas l'essentiel ; reste l'accumulation de 26 champs et leur
  écriture. Preuve, §6 de COUT-DELTA3D-S341 ; file.

