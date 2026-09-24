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

Session : S356 — **en cours**. **Rendu 1 (ADR-191) : les crêtes de B — l'écume et la lumière qui les traverse.**
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — ADR-191 : une session de rendu, une de physique ; la part de l'eau du rendu du moteur maison s'écrit en
module, depuis l'afficheur. **Ce qui est déjà su** : au verdict R14 (S308), l'utilisateur a nommé le travail
optique prioritaire — ciel, exposition, absorption, **diffusion aux crêtes**, **écume**, hautes fréquences du reflet ;
S307 en a fait l'ordre (RENDU-ECART-S307 §6) : la couleur du corps d'eau (faite), **la lumière des crêtes, pilotée
par le jacobien de CWM déjà calculé à chaque pixel**, puis **l'écume, couverture de Monahan & O'Muircheartaigh —
0,42 % à `U₁₀` ≈ 7,8 m/s —, réflectance effective de Koepke 0,22**.

**Le point délicat** : le rendu filtre les ondes courtes avec l'empreinte du pixel ; un seuil fixe sur le jacobien
ferait disparaître l'écume au loin. Le seuil porte donc sur la variable normalisée `s = (J − 1)/σ(h)`, `σ(h)`
l'écart-type du jacobien au filtrage du pixel ; `s_t` se tire de la loi de `s` à pleine résolution, pas d'un choix.

Critères, écrits avant le code :
1. **La couverture** : sur CPU, loi de `s` pour la mer de `--meilleur` (bande et queue, pleine résolution, plusieurs
   instants et lieux) ; `s_t` tel que `P(s < s_t) = W(U₁₀)` ; écart à la gaussienne et incertitude publiés.
2. **L'écume** (`--ecume`), module WGSL séparé : couverture par pixel tirée de `s_t` et de `σ(h)`, bord adouci,
   réflectance 0,22 éclairée comme l'eau. Sans l'option, les images de `--revue-mer` **au bit**. Avec : couverture
   recalculée sur CPU aux empreintes de 0, 0,5 et 2 m, à ± 30 % de `W`.
3. **La lumière des crêtes** (`--cretes`), même module : masque de compression tiré du même jacobien, diffusion vers
   l'avant, couleur de diffusion dérivée du corps d'eau (ADR-177) ; ses deux paramètres **à calibrer par R19**
   (I-14). Sans l'option, au bit.
4. **Images et coût** : quatre états (base, écume, crêtes, les deux) aux quatre poses de R14 ; surcoût GPU mesuré.
5. **R19** préparée, références demandées ; preuve, file, feuille de route, liste 8.4.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la loi de `s` et le seuil ; critère 1.
- [ ] **P3** — l'écume au rendu ; critère 2.
- [ ] **P4** — la lumière des crêtes ; critère 3.
- [ ] **P5** — images et coût ; critère 4.
- [ ] **P6** — R19, preuve, file, feuille de route, liste ; critère 5.
- [ ] **P7** — rituel.

### Notes de reprise
