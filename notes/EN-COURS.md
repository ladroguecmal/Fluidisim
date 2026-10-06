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

Session : S537 — **en cours**. En autonomie, **9.3 — un corps quelconque** : le prédicteur balistique (S405) détecte le contact par la
sphère englobante ; une planche qui tourne touche l'eau par un coin, bien après sa sphère.

**Ce que la session fait.** `ballistic::predict_hull(objet, sommets, …)` : le contact quand le **sommet le plus bas** de l'enveloppe
convexe (ses sommets dans le repère du corps, tournés par l'orientation intégrée) atteint la surface à sa propre position horizontale ;
l'instant par la même bisection sur un pas de RK4 que `predict` ; la région utile, la plus grande distance d'un sommet au centre. `predict`
inchangé.

**Ordre de grandeur, calculé.** Lâchée de 10 m : une boîte alignée (demi-hauteur 0,1 m) touche à **1,420686 s** ; une planche de 4 × 0,2 ×
0,2 m tournant à 3 rad/s autour de son axe long… — autour de `x` (demi-longueur 2 m selon `y`) — touche par un coin à **1,315550 s** ; sa
sphère englobante (2,0025 m) la ferait toucher à **1,276902 s**, 38,6 ms trop tôt. Le seuil d'instant (10⁻⁹ s) contre la bisection
(10⁻¹² s) : un rapport de 1 000 (ADR-236 D1).

**Critères, écrits avant.** (1) La boîte alignée en chute libre : l'instant à 10⁻⁹ s de `√(2(z₀ − h)/g)`. (2) La planche tournante :
l'instant à 10⁻⁹ s de la racine de `z₀ − ½gt² − (h_y|sin ωt| + h_z|cos ωt|)` (bisection indépendante, f64) ; la sphère englobante publiée
(38,6 ms d'avance). (3) `predict` au bit (suite).

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — la fonction, les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 9.3 ; rituel.

### Notes de reprise
