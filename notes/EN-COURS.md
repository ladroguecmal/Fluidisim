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

Session : S195 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo 1.97.0 disponibles)
Objectif : S194-1, **`n` sources** — A240. ADR-123 chiffre le domaine de la superposition
pour **deux** trains ; le nombre de paires croît comme `n²` et rien ne s'extrapole. C'est
la seule limite d'ADR-123 qui touche l'architecture. **Option tranchée par l'utilisateur**
contre la correction croisée quadratique, qui reste portée par la file active.

### Plan

- [x] **P1** — reprise, état réel (quatre copies au même commit), jeton et plan seuls.
- [x] **P2** — dérivation et protocole **avant tout code** : décomposition du forçage
  croisé à `n` termes, les deux régimes d'addition — cohérent contre dispersé — et leurs
  prédictions **opposées**, les deux séries (cambrure totale fixée, cambrure par train
  fixée), exigence de bande, jeux de phases déterministes, réceptions chiffrées dont la
  **continuité avec S194** et la convergence sous la forme exigée par **L274**.
- [x] **P3a** — banc `nl_sources_2d.rs` réutilisant `support/nl_surface.rs` sans le
  modifier ; tests propres dont `n=1` à écart exactement nul, `M=1` à écart d'arrondi, et
  reproduction du point `n=2` de S194.
- [x] **P3b** — campagne : les deux séries × deux jeux de phases × `n = 2..6`, séparation
  des mécanismes par la **durée**, convergence en `K` (ordre + Richardson), empreinte et
  deux exécutions identiques.
- [x] **P4** — documenter, propager A240/A50/B4 et la file ; ADR seulement si une décision
  de projet est prise ; ne **pas** dériver de seuil de bascule W/δ.
- [x] **P5** — rituel de fin (§6) : journal, angles, leçons, index/README/décomptes,
  jeton `libre`, copies avancées sans suppression non prouvée.

### Notes de reprise

P5 S195 : rituel terminé. **123 ADR, 241 angles, 276 leçons, 18 invariants, 6 SPEC,
23 cas**, vérifiés contre le dépôt. File plurielle relue (A211) : ligne A240 passée à
close, ligne **A241** créée, ligne de tête devenue S195-1, titre redaté. Journal, index,
README et REPRISE portés. Jeton libre. Trois copies isolées avancées sur `master`,
aucune suppression — aucune n'est prouvée morte.

P4 S195 : SOURCES-MULTIPLES-S195 §7–§9 reçus. **A240 close**, **A241** ouverte (le repli
des harmoniques croisées sur les modes de train gouverne la loi à grand n), **L276**
écrite (une variable de protocole peut être réfutée par le véhicule avant d'être mesurée).
**Aucun ADR** : ADR-123 se transporte dans le sens favorable, rien ne change de contrat.
Décomptes à porter en P5 : **123 ADR, 241 angles, 276 leçons, 18 invariants, 6 SPEC,
23 cas**. Les trois corrections de protocole de P3a sont publiées au §7.1, non réécrites.

P3b S195 : campagne exécutée, **empreinte 0x5eb378f6ffe26c9f**, deux exécutions
identiques ligne pour ligne. **Sept réceptions sur dix passent** : 1, 2, 3 (tests),
7 (dilution séculaire, décroissance nette et saturation à 1,0000), 8 (dérive d'énergie
1,5e-9 à 6,3e-9, aucune configuration hors domaine), 9 (ordre **1,756**, résidu de
Richardson **0,892 %** à K=64), 10 (bande Q=32 déplace de **0,0000 %**).
**Trois échouent — 4, 5, 6 — et par mauvaise spécification, pas par défaut de banc** :
elles reposaient toutes sur la dichotomie de phases que P3a avait déjà réfutée.
Série A, rapport n=6/n=2 : max 0,786 alignées / 1,091 dispersées ; L2 0,586 / 0,614.
Attendu 1,667 et 0,431 ; les deux jeux de phases ne diffèrent que d'un facteur 1,39,
pas >2. Série B : max 3,014 / 3,751 ; L2 2,205 / 2,354 ; attendu 5,0 et 1,29.
**Ce que ça donne vraiment** : sur L2, série A décroît en ~n^-0,49 et série B croît en
~n^0,72 — entre les deux lois, jamais l'une d'elles. Le maximum, lui, sature : il ne
décroît pas avec n. **Mécanisme que la dérivation a manqué** : l'amplitude produite par
le couplage tombe en partie sur les **modes de train**, et cette part décroît beaucoup
moins vite — série A alignées, `croise` chute de 6,8× de n=2 à n=6 quand `train` ne
chute que de 1,4×. Réponse à A240 : à cambrure par train fixée la croissance est
**sous-linéaire** (~n^0,72), loin du n² craint et sous le n du régime cohérent.

**Reprise du 2026-09-12 21:32 — la session a changé de main.** La session ouverte à 21:15
a été coupée par une limite d'usage sur un autre compte ; l'utilisateur l'a dit, sans quoi
le battement récent aurait interdit la reprise. `nl_sources_2d.rs` était sur le disque,
non committé : c'est l'étape P3a. Diff lu, exemple compilé, **neuf tests passent**, dont
`single_source_gap_is_exactly_zero` (cas nul, L271), `linear_order_superposes_for_six_sources`
(la réception `M=1`) et `two_sources_reproduce_s194` (la continuité déclarée au protocole).
Étape donc **complétée**, pas annulée. Workspace 331 réussis/cinq ignorés. Rien d'autre
n'était en suspens. Reste P3b, P4, P5.

Hérité : véhicule `NlSurface` de S193 (bande spectrale, convolution tronquée, `M=3` par
ADR-122) et banc de couplage de S194 (`nl_coupling_2d.rs`, trois évolutions en parallèle,
contre-épreuves à écart nul calibrées). ADR-123 : superposition sous 2 % en dessous d'une
cambrure de `0,009` par train en eau profonde, `5,4` périodes à `0,0125`, moins d'une à
`0,014` ; `α = 1,302602`, `β = 5,898728`. Seuil 2 % d'ADR-120 fixé, jamais redemandé.

Thèse de la session, déclarée avant mesure : à cambrure **totale** fixée, les deux régimes
d'addition des `n(n−1)/2` harmoniques croisées donnent des prédictions **opposées** —
**constant** en `n` si les phases s'alignent, **décroissant comme `1/n`** si elles se
dispersent. Le jeu de phases initial est donc une variable du protocole, pas un détail, et
c'est lui qui décide si répartir une même mer sur plus de composantes aide ou non.

Leçons de S194 à appliquer : **L274** — déclarer la convergence en ordre et résidu de
Richardson sur **trois** niveaux, jamais en taille de déplacement ; **L271** — déclarer au
moins une configuration à effet nul par construction ; **A238** — ne pas établir un ordre
de convergence sur un maximum de résidu, employer une fonctionnelle lisse.

P2 : protocole dans SOURCES-MULTIPLES-S195. Derivation complete, sans coefficient
libre : trois familles de termes croises, dont les **triples** C(ai,aj,ak) qui
n'existent pas pour deux trains (apparaissent a n=3, nombres d'onde ki+-kj+-kk).
Serie A, cambrure totale fixee : ecart/A = S(n-1)/n en coherent, 2S√(n(n-1)/2)/n²
en disperse. Rapports n=6/n=2 : **1,667 contre 0,431**, facteur 3,87 et sens
opposes. Serie B, cambrure par train fixee : s(n-1) contre s√(2(n-1)/n), rapports
**5,0 contre 1,29**. A240 avait raison sur le comptage et tort sur la consequence :
rapporte a l'amplitude totale le facteur est n, pas n².
Troisieme prediction, sans ajustement : la part seculaire par train se dilue en
(n-1)/n², donc ecart(N=10)/ecart(N=1) doit **decroitre** avec n en serie A.
Choix de modes 2..7 : place n=2 sur le couple (2,3) de S194, donc continuite
**attendue bit pour bit** avec son point Q=24 (3,612474e-1). Bande portee a Q=24
car les produits cubiques atteignent 21.
Phases : deux jeux deterministes, alignees et suite d'or a faible discrepance.
Separation des mecanismes par la **duree** et non par les modes : a n trains les
sommes ki+kj tombent sur des modes de train, il n'y a plus de mode exclusivement
croise. Le banc en publiera le decompte.
Convergence declaree des le protocole sous la forme de L274 : ordre et residu de
Richardson sur trois niveaux de K, sur la moyenne quadratique (A238).

