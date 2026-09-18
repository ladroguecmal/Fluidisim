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

---

## Session en cours

Session : S279 — en cours
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : « branche l'ordonnanceur sur la bande δ ». master 57fb182, maillons 1 — S278 a livré de
quoi décider et personne ne s'en sert.
Objectif : **la bande δ cesse d'être câblée**. À chaque image, l'afficheur publie ses trois poids,
l'ordonnanceur décide, et δ ne vit que s'il est retenu. Le coût annoncé est celui **mesuré** au pas
précédent (ADR-012 §3), pas une constante.

**Ce qui doit rester vrai** : dans la pose de la revue R10, la bande est visible en permanence,
donc elle doit vivre en permanence et **la scène doit être identique au bit** à celle d'avant le
branchement. Un branchement qui change l'image quand rien ne devait changer est un branchement
faux.

### Plan

- [ ] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — `W_perception` pour de vrai : fraction d'écran de l'emprise de la bande, projetée
  depuis la caméra, coupée au plan proche puis au cadre. Essais : de face, de dos, hors champ.
- [x] **P3** — les seuils passent au profil, et la première calibration (ADR à écrire). *Découpage
  déclaré à 23:05, imposé par une mesure : voir les notes.*
- [ ] **P3b** — le branchement : soumission, décision et allocation à chaque image ; δ n'avance et
  ne s'affiche que retenu ; coût réinjecté depuis la mesure du pas précédent.
- [ ] **P4** — réception : scène R10 identique au bit quand la bande reste visible ; relevé de
  l'extinction et de la renaissance quand la caméra se détourne ; coût réinjecté vérifié.
- [ ] **P5** — rituel §6.

### Notes de reprise

**Mesure qui impose un découpage** (relevé `part_de_cadre_des_poses_de_r10_s279`) : la part de
cadre de la bande vaut **0,5774 / 0,5456 / 0,5571 / 0,3185 / 0,5089** pour les poses défaut, le
long des crêtes, face à la houle, haute et rasante. **Aucune n'atteint le seuil d'allumage de
0,60.** Or `P = g · p · u` est un produit de trois fractions : il est **toujours ≤ la plus petite**,
donc un domaine maximal au gameplay et à l'urgence mais occupant 55 % du cadre ne s'allume jamais.
Brancher tel quel éteindrait la bande dans toutes les poses de R10.

**Ce qu'il ne faut pas faire** : remonter `W_gameplay` ou `W_urgence` pour que ça passe — ce serait
régler les poids sur le résultat voulu (L334). Les seuils 0,60 / 0,40 sont des **valeurs de départ
d'ADR-013 §5, explicitement « toutes à calibrer »**, et c'est leur première mise à l'épreuve.

**Critère de calibration, indépendant du résultat cherché** : S275 a mesuré que δ change 10 à 24 %
des pixels à hauteur d'œil et en incidence rasante, et **0 % vue d'en haut**. Un domaine mérite donc
de vivre quand sa présence change l'image — pas quand il occupe le cadre. Cette frontière tombe
entre la pose haute (0,3185, δ invisible) et la rasante (0,5089, δ visible). Les seuils doivent
appartenir au **profil**, pas au cœur : chaque hôte calibre les siens et les écrit.

**P2 : deux attentes fausses, corrigées par la mesure.** (1) Dans la pose de R10 la caméra est
**à l'intérieur** de l'emprise (256 × 200 m, œil au milieu) : la bande remplit le cadre quel que
soit le regard, et c'est pour cela qu'elle y vivra en permanence. (2) **La part d'écran n'est pas
monotone en lacet** — mesurée à 0,0308 de face et 0,0323 à 0,3 rad depuis l'extérieur : l'aire
projetée d'un rectangle ne décroît pas avec l'angle. Ce dont l'hystérésis a besoin n'est pas la
monotonie mais l'absence de saut ; mesuré : **moins de 0,01 par pas de 0,05 rad** sur un demi-tour.

`W_gameplay` et `W_urgence` n'ont **pas** de source dans un afficheur : il n'y a ni acteur ni
objectif. Ils seront déclarés, avec leur raison écrite — c'est exactement ce que l'ordonnanceur
attend d'un hôte (ADR-012 §2), et c'est honnête tant qu'on ne prétend pas les avoir mesurés.

La caméra du viewer donne `eye`, `yaw`, `pitch` et un champ vertical de 50° ; `lod::Projection`
porte déjà `forward/right/up/tan_half/aspect`. L'emprise de la bande est le rectangle
`x ∈ [X0, X0 + NX·DX]`, `y ∈ [±HALF_WIDTH]` au niveau de l'eau.

Attention : en direct, δ **renaît au repos** quand on le rallume (I-12). Une extinction n'est donc
pas gratuite pour l'onde injectée — elle repart de zéro. C'est le comportement voulu, pas un défaut,
mais il faut le dire dans la réception.
