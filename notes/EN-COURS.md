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

Session : S205 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A245**, bloquant J1 (FEUILLE-DE-ROUTE) — la composition B+W refuse toute mer
au-delà de Hs ≈ 1,1 m, dont la mer de référence S201. Lot bibliothèque : faire composer B+W
sur une mer réelle sans affaiblir ce que le budget garantit pour les perturbations.

### État réel à l'amorce

master = copies isolées = 990e6ae (S204 P5), propres ; jeton libre depuis S204 (même agent).
Arbitrage « hôte interactif de J1 » posé à l'utilisateur en S204, **non répondu** ; A245 n'en
dépend pas.

### Décision de conception, prise avant le plan (lecture du code)

Quatre sites mettent `steepness_B·π` (borne L1 de B, 0,6082 à Hs 1,5) dans le budget de refus :
`composition::compose`, `prepared_water::mixed::sample_world_batch`, `mixed_differential`,
`bound_pressure::Prepared::sample_world_batch`. Aucune SPEC ne consomme une garantie « surface
sous π/7 » : `steepness` sert à l'écume et au déferlement (SPEC-004 §2, SPEC-001 §3).
Remèdes écartés : terme directionnel (refuse encore, 0,5733) ; refus sur la pente réelle au
point (refus dispersés et causés par la mer, lot atomique perdu) ; borne statistique (ne
garantit rien). **Retenu** : le budget de refus ne somme que les **perturbations** (impacts
`slope_max`, pression `slope_envelope`) ; la raideur de B **reste publiée** dans `steepness`
= (B + perturbations)/π, même ordre de somme ⇒ **bits inchangés pour tout lot déjà admis** ;
`Slope`/`SlopeEnvelope` jugés sur la pente réelle **des perturbations** au point ;
`slope_floor` devient exact dans les deux sens. Rugosité de la mer = fait d'environnement
publié, pas une erreur de requête (ADR-127 D7 : pas de fonctionnalité retirée en silence).

### Plan

- [x] **P1** — état réel, décision de conception, plan seul.
- [x] **P2** — code : quatre sites, budget = perturbations seules, nommage sur leur pente
 réelle, publication inchangée ; documentation de `slope_floor` (exact) ; compilation.
- [x] **P3** — essais : rejouer water-core, trier **chaque** échec (attente liée à B dans le
 budget → réécrite avec cas W et motif écrit ; autre → défaut à corriger) ; nouveaux essais :
 mer S201 Hs 1,5 composable (journal vide et avec impact) par `compose` et `mixed` ;
 `slope_floor` exact des deux côtés ; verdicts `Slope`/`SlopeEnvelope` atteints par W seul.
- [x] **P4** — gardes du contrat de pente (I-18) relues et mises à jour ; workspace debug
 complet ; essais touchés en release.
- [x] **P5** — bout en bout : image S203 +3 s rejouée **au bit** (contrôle « admis inchangé ») ;
 impact sur la mer S201 Hs 1,5 contre témoin, zéro pixel hors emprise ; budget d'impact π/7.
- [x] **P6** — ADR-128 ; notes datées ADR-080/095/098/126 ; A245 close ; COMPOSITION-MER-S205.
- [x] **P7** — rituel §6 : journal, angles, leçons, décomptes, feuille de route (état J1), file
 active, index/README/REPRISE, compteur, jeton libre, copies.

### Notes de reprise

Critère de non-régression déclaré **avant** le code : pour tout lot admis par l'ancienne règle,
`eta`, `deta_dt`, `u_total`, `normal`, `steepness`, `aeration` identiques au bit ; seuls des
refus disparaissent. Si un hachage de campagne ou d'essai change, c'est un défaut du lot, pas
une attente à réécrire.

P2 : quatre sites modifiés (`budget` = perturbations, `perturbation` = leur pente au point,
`bound`/`envelope` publiés inchangés). `composition` contrôle désormais `steepness` fini à la
sortie (le refus ne le garantissait plus). `Background::differential_slope_envelope` retirée :
seul usage = budget différentiel ; son assertion d'essai remplacée par l'équivalence des deux
chemins au plancher exact (`slope_floor` et son prédécesseur flottant). Compilation sans
avertissement. Essais non encore rejoués.

P3 : premier passage **7 échecs / 246**, tous triés, **aucun hachage ni valeur publiée** en
cause. (1) garde I-18 : noms `bound`/`envelope` → `budget` dans les trois budgets, liste
`SITES_CONNUS` mise à jour avec motif. (2) `composition::each_slope_verdict…` : cas 2/3
tenaient à B seul → reconstruits avec un champ d'impact (r = 0,2062 λ à la naissance pour
`Slope`, centre pour `SlopeEnvelope`), et une pente de B de 0,5 vérifiée sans effet.
(3) pression `world_refusals…` : `Slope` → `SlopeEnvelope` — la pente de B (0,00628) faisait
passer la somme au-dessus ; la pression seule tient. (4) `the_same_envelope…` : balayage de la
pente mesuré sur fond d'amplitude nulle, refus posés sur le fond de 0,01. (5)
`normal_matches…cancellation` : plafond entre pente de la pression seule et son majorant.
(6) `mixed_rejects…` : limite sans terme de B ; nom accepté `Slope|SlopeEnvelope`, atomicité
conservée. (7) `slope_floor_refuses…` : « au-dessus c'est B qui décide » remplacé par
« au plancher et au-dessus tout lot passe » (1 à 3 points, trois limites). Plus
`differential_slope_envelope` retirée (P2) et son assertion remplacée par l'équivalence des
deux chemins au plancher exact. **Neuf** : `reference_sea_s201_composes_with_an_impact_s205`.
Après réécriture : **246 réussis + 1 neuf**, deux ignorés (lib). Commentaires de doc des
erreurs `Slope` mis à jour dans les trois modules.

P4 : garde `every_field_places_its_limit_at_stokes_steepness_s143` inchangée et verte ; garde
des sites mise à jour en P3. Workspace debug **344 réussis / cinq ignorés** (247+4+93), un de
plus que S203 (l'essai neuf). Release : 28 essais `composition|mixed|pressure_world|
contrat_pente` verts. Harnais release : **C18 0x85c8bc610f551d11, C02 0x0a3a3bcc945db263**,
identiques aux reçus S179–S182 ; zéro échec. Avertissements anciens du harnais inchangés.

P5 : image S203 +3 s rejouée **au bit** avec la bibliothèque modifiée (impact
0x3dba0d3acf15447a, témoin 0x14271a7145740ba1, 9 495 898 / 9 495 848 évals, 8 640 px, 0 hors R).
Banc : `SlopeRule` (S203 conservée pour reproduction ; ADR-128 = π/7), mode `render-s205 <dir>
<âge> [hs]`, essai `adr128_rule_composes_the_reference_sea_s205` (12 essais exemple). Mer S201
Hs 1,5, plancher B 0,608192, budget impact 0,448799, même W (E 164 J, λ 3,35 m, N256 R52 A56).
**+3 s** : impact 289 462 eau, **0 non résolu, 0 refus**, 15 841 764 évals dont 9 165 220 B+W,
131 483 px touchés, 102 218 ms, 0x0b13a4c1e39a2a3e ; témoin 17 921 ms, 0x0d8495b6adf64de3 ;
**7 342 px différents, 0 hors emprise** ; écart max 17 niveaux, 1 676 px ≥ 3. **+6 s** : 289 456
eau, 0/0, 15 571 336 évals dont 8 879 321 B+W, 100 389 ms, 0x935223a0f507aac7 ; témoin 17 881 ms,
0xb25f06a61b0cc543 ; **16 251 px, 0 hors emprise** ; max 29 niveaux, 2 513 px ≥ 3. Anneaux nets à
+3 s sur la mer raide, discrets et déformant le reflet à +6 s (zooms inspectés). Marche ×1,67
plus d'évaluations qu'à Hs 0,5 : borne de pente B+W 0,99 contre 0,58.

P6 : ADR-128 actée ; notes datées ADR-062, 080, 095, 098, 126 ; COMPOSITION-MER-S205 (défaut,
remèdes pesés, changements, tri des sept essais, réception, image, non reçu) ; clôture A245 au
registre. Décompte attendu : 128 ADR.

P7 : journal S205, A245 close, A249 ouverte, corollaire L280 ; feuille de route (J1 : A245 levé),
file active (A245 close, A247 prochain lot S206, A249), index, README, REPRISE (§3, §4, table
velocite.sh : B et W S205). Décomptes 128/249/282/18/6/23 vérifiés. Jeton libre, copies à avancer.
