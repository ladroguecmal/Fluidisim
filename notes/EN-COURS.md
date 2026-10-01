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

Session : S426 — **terminée**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §17 : B10 en bande étroite, pas + bascule 2,45 ms au p99 ; les fils de l'absorption et
de l'échange 0,40 + 0,55, ≈ 2 µs par geste).

**Ce que la session fait.** (1) **L'absorption par vagues.** La visite de la référence (en montant, échange avec la dernière) fixe
un ordre de traitement des absorbées ; le fil 0 le calcule d'avance, sans toucher aux données (l'identité traitée à chaque rang est
connue : une place n'est écrite qu'après avoir été traitée). Deux absorbées ne se gênent que si elles partagent une face de mélange
(leurs mailles à moins de deux d'écart sur chaque axe) ou une colonne (solde, volume) : chaque vague traite, en parallèle, les
absorbées dont toutes les devancières en conflit sont faites — chaque face reçoit ses mélanges dans l'ordre de la référence, **au bit**.
Les vingt-quatre mélanges d'une absorbée se répartissent entre les fils ; le retrait final à forme close (S423). Au-delà de 512
absorbées, l'ancien fil. (2) **L'échange** : le tri par insertion des marquées et leur retrait sur le fil 0, en fin de noyau —
par rang et à forme close si l'arrangement de la référence s'y prête ; puis, s'il reste du temps, les vagues pour l'échange. (3) Mesurer.

**Critères, écrits avant.** (1) Issues identiques à S425 au chiffre près (étages, gestes, B10 en bande étroite et nu, bascules
forcées, ballottement, raccord, bande) — le parallélisme ne doit rien changer. (2) Le coût des fils publié avant et après, le nombre de
vagues par pas ; **visé : pas + bascule ≤ 2 ms au p99** (2,45). (3) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'absorption par vagues ; issues, mesure, vagues par pas.
- [x] **P3** — l'échange : les marquées par rang et à forme close ; mesure ; les vagues de l'échange si le temps reste.
- [x] **P4** — non-régression, suite ; preuve §18 ; registres.
- [x] **P5** — rituel.

### Notes de reprise
- **P2** — l'absorption sans fil séquentiel (`absorb_faces`) : le fil 0 calcule d'avance l'ordre de la visite (sans geste : l'identité traitée à chaque rang est connue), puis tout en parallèle, au bit. **Essai 1, des vagues d'absorbées à faces disjointes** : identique, mais 45 vagues pour ≈ 100 absorbées (106 au plus) à ≈ 3,8 µs — les voisines partagent presque toujours une face ; médiane 0,205 → 0,166, p99 inchangé. **Essai 2, un fil par face touchée** : la première absorbée qui touche une face (dans l'ordre de la visite) en est propriétaire et y applique, dans l'ordre, les mélanges de toutes celles qui la touchent — identique, mais **cinq fois plus lent** (le test « touche-t-elle cette face » recalculait huit nœuds en global). **Retenu** : le test rendu immédiat — `n − base ∈ {0,1}³` désigne le nœud, un masque de 24 bits par absorbée (en mémoire de groupe) dit s'il est mélangé ; et les soldes et volumes (entiers) par cible : la première absorbée de chaque cible ajoute toutes celles de la cible d'un coup. Étage de l'échange, raccord 30 s (gestes 5 289 / 109 / 5 385, 4,015 mm), B10 en bande étroite : **identiques**. ≈ 276 faces touchées par pas. **Fil de l'absorption : médiane 0,205 → 0,132 ms, p99 0,40 → 0,29** ; **pas p99 2,11 ms, + bascule 0,23 = 2,35**.
- **P3** — l'échange : les marquées (≈ 140 par pas) étaient triées par insertion sur le fil 0, en mémoire globale ; **triées par rang** en mémoire de groupe (chaque fil compte les indices marqués plus grands ; l'ancien tri au-delà de 512). Étage du fond et B10 en bande étroite identiques. **Fil de l'échange : médiane 0,28 → 0,244 ms, p99 0,55 → 0,494** (24 + 1,7 µs par geste). **Les vagues de l'échange ne sont pas faites** : les gestes sont surtout des poses (B10 : 2 558 posées pour 1 048 retirées au pas 34), et chaque pose choisit l'emplacement le plus libre en comptant les poses précédentes — une dépendance réelle, à traiter à part (les retraits d'une face-maille, eux, sont les K plus petites clés : groupables). **Pas p99 2,07 ms + bascule 0,24 = 2,30** — visé 2 : manqué de 0,3.
- **P4** — non-régression : étages, cycle, bascules forcées, B10 nu et en bande étroite, ballottement 0,447, colonnes 0,002, raccord 4,015 (gestes compris) **identiques** ; suite 753 / 19 / 0. **La bande sur 30 s ne l'est pas** : 1,210 mm, gestes 5 500 / 360 / 5 829 (S425 : 1,318, 5 478 / 354 / 5 795). **Isolé** : l'ancien fil (`AW_REPLI=1`, `AW_MAX` devenu constante de pipeline) rend 1,318 ; les mélanges faits en séquence dans le nouveau noyau avec la fonction d'origine rendent 1,210 comme la version parallèle — ni les soldes ni l'ordre ; comparés au bit après un pas (`DUMP=`), les chemins diffèrent sur 8 / 3 / 0 des 41 120 valeurs (instants 20 / 60 / 40), **d'une unité du dernier chiffre** : FXC arrondit la même formule autrement dans un autre noyau (L345), 30 s de bande l'amplifient. Le critère 1 (« identiques au chiffre près ») est donc tenu partout sauf sur ce cas chaotique, où l'écart est d'arrondi, dans la dispersion connue (1,45 · 1,71 · 1,318 selon les sessions). Preuve §18 ; registres.
- **P5** — journal ; jeton libre ; maillons 14 (justifiés : S406) ; suivant : S427, C7e — les poses de l'échange, la projection.
