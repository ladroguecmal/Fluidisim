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

Session : S409 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Reprends le projet »* ; au poste. Suite désignée
au poste par S408 : **C3b** (conception S384 §5, C3 : « δ ≤ 2 ms au 99ᵉ centile sur la scène de la porte B, **puis à 10 cm**
sur une scène de même surface ; A298 remesurée »). Agent : Claude Code (Opus 5.5), application de bureau, **au poste** — fichiers,
git, cargo, Python, **RTX 5070 Laptop** ; Godot non utilisé. Branche `poste` (= `main` = `claude/eager-volta-lf0kw3`).

**Ce que « même surface » ne peut pas vouloir dire** (calcul, avant toute mesure) : la scène de la porte B (30 × 28 m, boîte de
7 m) à 10 cm, c'est 300 × 280 × 70 = **5,9 M mailles**, 17,8 M faces — le tampon des faces (10 flottants) passe la liaison de
128 Mio vers 3,4 M faces (≈ 1,1 M mailles), et le coût par maille de S350 (3,57 ms pour 376 320) donnerait ≈ 56 ms par pas.
Le critère de S384 reposait sur les colonnes hautes (14 m de côté à 10 cm), que S386–S387 ont réservées à l'eau calme. **Lu
ici** : la même mer, la même boîte verticale, une perturbation à la même pente, sur l'**emprise que le budget permet** ; la
mesure dit laquelle.

**Thèse.** À 10 cm, Jacobi rampe (son taux par itération se dégrade avec `N`) et la multigrille, indépendante de la maille (C1),
doit payer davantage qu'à 25 cm (projection ÷ 1,9 en S390). Une scène à 10 cm tient-elle deux minutes (L369), à 30 Hz, avec le
terme de second ordre d'ADR-209 — dont le nombre de Courant, lui, croît de 2,5 ?

**Critères, écrits avant.** (1) Sans les variables nouvelles, au bit : `--delta3d-empreinte` inchangé (60 et 600 pas). (2) À
10 cm, boîte de 7,2 m (`nz` = 72 : trois niveaux grossiers), deux emprises au moins : la multigrille atteint le **résidu médian
de Jacobi-32 à 25 cm** (7,1·10⁻⁵) en **≤ 8 cycles** ; *prédiction* : Jacobi-32 y est ≥ 5 fois moins bon qu'à 25 cm, la
multigrille aux mêmes cycles qu'à 25 cm. (3) À résidu égal, pas multigrille ≤ Jacobi, gain de projection **> 1,9** (*prédit*).
(4) Durée d'usage : la scène à 10 cm tient **deux minutes** à 30 Hz avec la multigrille retenue — sinon à 60 Hz, et l'on
nomme ce qui casse (témoin, termes éteints un à un, L136). (5) Budget : l'**emprise carrée la plus grande** à 10 cm dont les
deux parts de 30 Hz tiennent **≤ 2 ms au 99ᵉ centile**, publiée avec la loi coût/emprise ; *prédiction* ≈ 0,45 M mailles,
≈ 8 m de côté. (6) **A298** sur le pas retenu (multigrille, terme d'ADR-209 ; cuve fermée) : l'écart carte/référence au pas
d'usage (33,333 ms) sur **deux minutes**, et au pas de S305 (1 ms, 5 s) pour comparaison ; **close** si l'écart à deux
minutes ≤ 3 mm **et** la pente extrapolée franchit 3 mm après plus d'une heure ; sinon ouverte, pente publiée. (7) Suite de
l'afficheur, zéro avertissement. **Arrêt** : si la scène à 10 cm explose aux deux cadences, publier et ne rien changer aux
défauts ; la multigrille ne devient défaut que pour la scène à 10 cm, si elle y est nécessaire.

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — la scène à une maille et une emprise données (`MAILLE=`, `EMPRISE=`) : boîte de 7,2 m, éponge en mailles, un
  impact à la pente de celui de R16 et à l'échelle de l'emprise ; sans variable, la scène de S390 au bit ; critère 1.
- [ ] **P3** — qualité à 10 cm : Jacobi 32/64/128 contre multigrille 4/6/8, deux emprises (300 pas) ; critère 2.
- [ ] **P4** — coût à 10 cm, pas entier et deux parts ; l'emprise la plus grande sous 2 ms ; critères 3 et 5.
- [ ] **P5** — deux minutes à 10 cm, 30 Hz puis 60 Hz si besoin ; critère 4.
- [ ] **P6** — A298 : `longue_cuve` paramétrée (`PAS_US`, `PAS`, `MULTIGRILLE`) ; 1 ms × 5 000 et 33,333 ms × 3 600 ; critère 6.
- [ ] **P7** — suite de l'afficheur (et du cœur si touché), zéro avertissement ; critère 7.
- [ ] **P8** — preuve : MULTIGRILLE-3D-S385 §6 (un fil, une preuve) ; liste 4.19, file, feuille de route, index ; A298.
- [ ] **P9** — rituel.

### Notes de reprise
