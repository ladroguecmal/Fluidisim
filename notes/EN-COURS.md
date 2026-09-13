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

Session : S220 — terminée
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Reprends le projet » ; master et trois copies propres à 8ef0c64, jeton libre,
maillons 0. AGENTS, REPRISE (jeton, file active, §4 S219–S217, §5–§9), EN-COURS, journal
S218–S219, PARTITION-S219, ADR-135, code local_bound/partition/phase lus. Copie principale.

### Thèse et plan

A259 : la borne ADR-135 somme des **modules** de variation, `c_k·min(2,D_k)` ; aux mailles
moyennes cette somme ne voit aucune annulation entre modes. Développer chaque mode à l'ordre
deux autour de la phase **exécutée** au centre : `a sin(ψ+δ) = a sin ψ + a cos ψ·δ + R`,
`|R| ≤ a·δ²/2`. Le terme `a cos ψ` est exactement `η_k(c)` : la Hessienne de la pente vaut
`M = −Σ w_k ⊗ (2π t_k) η_k(c)`, **signée**, et `|S(c) + M u|` est convexe en `u`, donc son
maximum sur le rectangle est à un coin. Écart exécuté/linéaire `e_k` (arrondi des produits
`t·x`, fraction, Q32, arrondi des coins) majoré par `E_k` et payé `c_k E_k`. Choix par mode :
dans `M` si `E_k + D_k²/2 < min(2, D_k)`, sinon terme ADR-135. Retenir le minimum des branches
ordre un, ordre deux et globale, chacune avec sa réserve : jamais pire qu'ADR-135.
Au maximum de `|S|`, `S·Mu = 0` : l'excès devient quadratique, ce qu'ADR-135 ne peut pas.
Critères déclarés avant mesure : sondes sous la borne (pic manqué, multidirectionnel, 4000 m) ;
borne ordre deux ≤ ordre un sur tout rectangle ; gain strict près d'un maximum ; S219 inchangée
en ordre un au bit. Gain de partition non promis. Aucune admission migrée, A258 reste ouverte.

- [x] **P1** — jeton et plan seuls.
- [x] **P2** — ADR-136 : dérivation, écart de phase quantifiée, choix par mode, réserve, limites.
- [x] **P3** — construire la branche ordre deux (Field, Prepared) sans changer ADR-135 ni S219.
- [x] **P4** — tests : couverture, domination, gain près du maximum, 4000 m, refus, identité S219.
- [x] **P5** — exemple S220 : même rectangles S218 et partition S219 par ordre ; campagne isolée.
- [x] **P6** — publier la réception S220 et ses relevés bruts.
- [x] **P7** — rituel §6, journal, registres, index, file active, jeton et copies.

### Notes de reprise

Suite S219 : partition adaptative disponible, A259 (plateau des grandes mailles) ouverte.

P3 : une passe O(N) ; `Slot::accumulate` rend (sin, cos) sans changer ses opérations. Branche ADR-135 recalculée dans la même passe (identité au bit à recevoir P4). Partition : `partition_slope_envelope_order`, l'appel S219 délègue en `First`. Compilation debug sans erreur.

P4 : quatre tests S220 debug réussis. Deux attentes du test « quadratique » étaient fausses et ont été corrigées **avant** toute mesure de campagne, sans toucher au code : (1) avec un seul mode, la borne globale est exacte et plafonne les deux branches, donc on compare les branches sans elle ; (2) le demi-côté `h` donne un excès ADR-135 ≈ `h` (et non `2h`), et un excès ADR-136 ≈ `h²/2` plus la réserve (≈ 7e-5) plus un plancher `E` ≈ 6e-6, la marge `8ε` de la phase quantifiée. Mesuré : h = 0,05 → 1,3263e-3 au total. Interruption utilisateur au milieu de P4 pour régler le niveau d'effort (xhigh constaté), puis reprise. Suite release complète : 366 réussis (268 cœur + 4 + 1 + 93), 5 ignorés, zéro échec.

P5 : exemple `ordre_deux_s220` (grilles S218 2/1/0,5 m en ordre deux avec ordre un publié dans la même passe ; partitions S219 ordre un puis deux à 2047/8191/32767/65535). Base, passage 1 isolé (15:43) : ordre un redonne S218 et S219 **au bit** (0,112293623 à 0,5 m ; 0,115437612/0,098479681/0,077374868). Grille ordre deux : 2 m aucun gain (branche 0,2093 au pire rectangle, 1332/3072 plus serrés) ; 1 m 0,109988 (×1,0495) ; 0,5 m **0,079501** (×1,4125 sur ordre un) en 47,3 s contre 27,5 s S218. Coût par évaluation ≈ ×1,72. Partition ordre deux : 2047 et 8191 au plafond (A259 persiste sous ≈3 m² par feuille) ; **32767 : 0,070740 (1,0059 × référence) en 33,8 s**, meilleur que l'ordre un à 65535 (0,077375, 37,2 s) ; 65535 : 0,070666 (1,0049) en 63,5 s — saturation. Hypothèse à vérifier : plancher = réserve numérique (ordre un 2,67e-4, ordre deux ≥ 5,4e-4, donc la branche un gagne sur les feuilles minuscules). Fausse alerte de sûreté examinée : aucune contradiction, la feuille maximale peut être plafonnée par la branche un. Campagne détachée arrêtée pendant `lent` pour ajouter le diagnostic de feuille maximale (hors chronométrage), relancée 15:47:55 : lent, long, base_tard, base passage 2. Lent partiel conservé : 0,5 m ordre deux 0,017091 contre 0,026653 (×1,5595). Les campagnes détachées passent par `scratchpad/campagne_s220.ps1` (le délai d'un appel d'outil est de 10 min).

P5 fin : campagne relancée terminée à 16:02:29 (lent 15:51, long 15:55, base_tard 15:58, base passage 2 16:02). Journal de campagne resté à « fin lent » : `Add-Content` a échoué pendant que `tail -f` tenait le fichier, et la surveillance a attendu en vain jusqu'à 16:25 (impasse de procédure, pas de mesure). Base passage 2 : bornes identiques au bit au passage 1, 30,8/59,8 s contre 33,8/63,5 s. **Hypothèse du plancher confirmée** par la feuille maximale : base 32767, branche un 0,070740171 = centre 0,070322238 + reste 1,51e-4 + réserve 2,67e-4 ; branche deux 0,070861 (réserve 5,37e-4). Lent/long/base_tard à 65535 : ordre deux 1,0101/1,0063/1,0111 × référence, écart ≈ réserve. Grosses mailles 2×1,5 m : reste d'ordre deux 0,024 (lent) à 0,116 (long), dominé par les modes exclus (2488 dans M sur 4096).

P6 : ORDRE-DEUX-S220 et relevés bruts publiés. Corrections avant publication, relues contre les relevés : branche 2 m « 1,5–1,8 × globale » (et non 0,17–0,23, faux pour lent), coût « 1,6–2,0 × » (et non 1,69–1,78), rapport reste/réserve « 160–360 × » sur les trois feuilles d'ordre deux (la base est plafonnée par l'ordre un). Suite proposée : A260 (enveloppe spectrale des non résolus), A258 (borne d'erreur courante), coût et validité temporelle. À porter au rituel : A260, suivi A255/A258/A259, L299.

P7 : journal, A260 et suivis A255/A258/A259, L299, index, README, REPRISE (§3, §4, file active, jeton), feuille de route et file active plurielle actualisés. Une attribution non mesurée (reste des grosses mailles « fait des modes exclus ») corrigée avant commit dans la réception, A260 et le journal. Maillons 0. Jeton libre ; copies à avancer après le commit de clôture.
