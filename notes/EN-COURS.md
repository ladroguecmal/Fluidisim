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

Session : S355 — **terminée**. **La v1 en scène vivante, 2 : la coque dans la production de δ** — d'abord le pas
linéaire sur la carte.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — *« Continue »*. Liste 6.4 : manque la production GPU. **Constat** : la coque a été reçue dans le **mode
linéaire** de δ — couvercle, faces coupées, couvercle partiel (S330–S337, porte D) ; la production est le pas
**mobile couplé**, qui n'a de solide nulle part, même sur CPU, et une coque qui perce une surface mobile serait une
physique nouvelle. **Chemin retenu**, à écrire en ADR sur les mesures : le domaine δ d'une coque est un domaine
**linéaire**, le mode reçu par la porte D, porté sur la carte ; le pas mobile couplé garde les autres domaines.
Découpage : S355 le pas linéaire à ouvertures, toutes ouvertes ; puis la découpe d'un solide fixe ; puis la coque
qui bouge et perce le couvercle ; puis la scène — une session du lot 5 entre chacune (ADR-184 D1).

Critères, écrits avant le code :
1. **`Linear3` sur la carte** : le pas linéaire de `Volume3` — prédiction égale au courant, éponge, divergence
   pondérée par les ouvertures, terme du couvercle, opérateur pondéré au couvercle à demi-maille, gradient conjugué
   à cycles fixes repartant du pas précédent, correction, flux de colonne, hauteur compensée, rappel de l'éponge.
   Ouvertures réservées à la création, toutes ouvertes (I-06).
2. **Contre la référence** : bosse de 10 cm sur la grille de la porte D (96 × 96 × 8, 25 cm, 2 m), pas de 10 ms,
   200 pas, éponge de 3 m à 2,5 /s : **|Δη| ≤ 10⁻⁴ m** partout, relevé tous les vingt pas ; volume de δ au plancher
   du transport.
3. **Le coût** sur cette grille : p50 et p99 du pas horodaté, au nombre de cycles qui tient le critère 2 ;
   alimentation relevée (A270).
4. ADR-191, preuve, file, feuille de route, liste 6.4.

### Plan

*Amendé en cours de session, sur la demande de l'utilisateur (00 h 20) : sa question sur le réalisme du rendu, puis cinq
réponses — [ADR-191](../docs/adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md). Le pas linéaire sur la carte n'avait pas
commencé (aucun diff) : ses étapes — `Linear3`, le banc, le coût, l'ADR de la coque — passent **telles quelles** à la
prochaine session de physique ; le plan initial est au commit `a2c81dea`.*

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la décision de l'utilisateur : ADR-191, note datée d'ADR-178, file, feuille de route, liste 8.1, index,
  REPRISE §5.
- [x] **P3** — rituel.

### Notes de reprise
- **P2, la question et les réponses.** Référence de l'utilisateur : FluidNinja LIVE-2 (Fab, Unreal, Andras Ketzer) —
  « using 2D sim to drive 3D visualization », simulation attachée au joueur, lointain en motifs passifs ; images :
  un tourbillon, de l'eau turquoise peu profonde avec un fond visible, des traînées d'écume. Réponses : moteur maison,
  à construire ; rendu final dans le moteur ; nous l'écrivons en module ; alterner avec la physique. ADR-191.

