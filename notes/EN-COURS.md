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

Session : S197 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo 1.97.0 disponibles)
Objectif : **A242**. S196 a montré que la dérive d'énergie, employée comme critère de
domaine par S193 à S196, ne détecte pas la sous-résolution — contre-exemple faux d'un
facteur cinq passant 65× sous le seuil. Deux devoirs : **auditer** les configurations
déjà publiées, à commencer par celles qui portent **ADR-123, actée** ; et **apparier**
le critère dans les bancs, pour que la faute ne puisse plus se répéter.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — protocole **avant tout code** : ce que « loin du bord » veut dire et se
      mesure, le classement des cibles par enjeu, ce qui compte comme échec — un chiffre
      qui bouge n'est pas une conclusion qui tombe — et ce qu'on fera si une cible échoue.
- [x] **P3a** — apparier le critère : garde partagée qui **refuse** un ordre tiré d'un
      triplet non monotone, et contrôle de raffinement ajouté à chaque banc publié.
      Assertions de continuité sur les valeurs déjà publiées.
- [x] **P3b** — exécuter l'audit : S194/ADR-123 d'abord, puis S195 et S193. Relever les
      marges et, pour ADR-123, l'effet sur ses **seuils** et non sur ses décimales.
- [x] **P4** — verdict par cible. Note corrective datée là où il en faut ; un ADR n'est
      jamais réécrit, et une décision qui changerait demanderait un ADR neuf.
- [>] **P5** — rituel de fin (§6) : journal, angles, leçons, index/README/décomptes,
      jeton `libre`, copies avancées sans suppression non prouvée.

### Notes de reprise

P4 S197 : AUDIT-RESOLUTION-S197 §8–§9 reçus. **A242 close** (traitée, remède en place).
**A241 corrigée** : le repli est **réfuté**, pas « un tiers ». **L278** écrite.
Corrections propagées : avertissement en tête de REPLI-CROISEES-S196, note de
**confirmation** datée sur **ADR-123** (non réécrite), note sur SOURCES-MULTIPLES-S195.
Décomptes à porter en P5 : **123 ADR, 242 angles, 278 leçons, 18 invariants, 6 SPEC,
23 cas** — A242 est close, pas nouvelle, donc le compte d'angles ne bouge pas.
**Aucun ADR** : ADR-123 est confirmée par l'audit.

P3 S197 : `dispersion_error(upto)` et `richardson()` ajoutés au support partagé, deux
tests neufs ; audit ajouté aux trois bancs publiés. Workspace 331 réussis/cinq ignorés,
12 tests au banc S196. Empreintes **changées** par l'ajout de l'audit : coupling
`0x4bc0934d630c2c50`, sources `0xb9b742189219c8ce`, fallback `0xcbb87743073b703d`.

**RÉSULTATS DE L'AUDIT — deux cibles tiennent, une tombe.**
(1) **ADR-123 tient.** La table **mesurée** de S194 §7.8 rejouée : à K=64 elle reproduit
exactement le publié (jamais/19,82/11,70/5,38/0,78/0,75/0,40) ; à K=256 et K=1024, toutes
deux identiques, elle donne jamais/19,38/11,50/5,20/0,78/0,72/0,40. **Déplacement maximal
3,3 %**, aucun énoncé qualitatif ne bouge. Convergée dès K=256. Note : l'ajustement
`α` bouge plus (1,302602 → 1,257, soit 3,7 %) et ses extrapolations sous s=0,0125 bien
davantage — mais ADR-123 ne repose pas dessus, sa table est mesurée.
(2) **A240/S195 tient.** Série A −0,437 → **−0,425** ; série B +0,783 → **+0,774**.
(3) **S196 TOMBE.** L'écart pair/impair de **0,131** devient **0,005** à K=1024 — c'est
la prédiction 2 de S196, celle qui **réfute** A241 : le repli n'explique **rien**. À K=64
l'audit reproduit exactement −0,525/−0,394, donc l'audit est fidèle. L'exposant dense
n=2..16 passe de −0,478 à **−0,445** (dans la tolérance déclarée −0,42…−0,62, donc la
moitié « limite » survit, sa valeur passant de −0,52 à ~−0,45).
**Pourquoi S196 tombe et pas les autres** : S194 et S195 comparent des configurations à
**même bande**, l'erreur de symbole y est commune aux deux côtés et s'annule. S196
comparait deux familles de **modes et bandes différents** — 38 contre 40 — donc
d'exposition différente à l'erreur. C'est la leçon à écrire.

P2 S197 : protocole dans AUDIT-RESOLUTION-S197. **Découverte structurante faite avant
toute mesure, et par arithmétique pure** : `K` n'entre dans le véhicule que par le
symbole de dispersion `dn[q]`, précalculé — le pas de temps ne le voit jamais. Donc
(a) le défaut se calcule en forme fermée sans simuler, (b) monter `K` ne coûte qu'à la
construction, l'audit est bon marché.
Écart `|G_h − k tanh kh|/(k tanh kh)` à **K=64**, la valeur de S193 à S196 : q=2 **0,48 %**,
q=3 **1,08 %**, q=9 **9,32 %**, q=16 27 %, q=24 55 %, q=38 112 %. Sur la bande peuplée :
**S194/ADR-123 9,3 %**, S195 n=6 **43,6 %**, S196 n=16 **112 %**. Pour tenir 1 % il
faudrait K=256 / 512 / 1024. Le symbole converge à l'ordre deux.
**Ne pas surinterpréter** : les deux évolutions comparées partagent le même symbole, et
l'amplitude est aux modes porteurs (0,5–1,1 %). Mais S194 dit que le couplage est
gouverné par le **désaccord de triade**, une différence de fréquences — donc exactement
ce qu'un symbole biaisé déplace. D'où les deux prédictions opposées déclarées.
Critères posés sur les **énoncés** et non sur les décimales : `α=1,302602` va bouger,
ce n'est pas le sujet ; le seuil de cambrure doit rester dans 0,0077–0,0104.

S197 : master 21763ab propre, quatre copies alignées ; 123 ADR/242 angles/277 leçons.
**Ce que l'audit vise, par enjeu décroissant.** (1) **ADR-123, actée** : ses seuils
(`0,009` par train pour 2 %, 19,8 périodes) et son ajustement `α=1,302602`,
`β=5,898728` sortent de `nl_coupling_2d` à **Q=16, K=64**, couple (2,3), 20 périodes,
`dt=T₁/400`. (2) S195/A240 : l'exposant des `n` sources, `Q≤24, K=64`. (3) S193/ADR-122.
**Repère de coût déjà connu** : le cas qui a cassé en S196 était `Q=38, K=32`, soit
`k_max h = 239`. S194 tourne à `Q=16` → `k_max h = 100`, bien moins exigeant ; S195 à
`Q≤24`. L'hypothèse de travail est donc que S194 est loin du bord — **à vérifier, pas à
supposer**, c'est tout l'objet de la session.
**Piège de lecture à tenir** : ADR-123 publie `α` à sept chiffres. Ce nombre **va**
bouger avec `K` ; ce qui compte est si ses **seuils** bougent. Ne pas confondre « le
chiffre se déplace » et « la conclusion tombe ».

P5 S196 : rituel terminé. **123 ADR, 242 angles, 277 leçons, 18 invariants, 6 SPEC,
23 cas**, vérifiés contre le dépôt. Workspace 331 réussis/cinq ignorés en debug et
release. File plurielle relue (A211) : A241 requalifiée, ligne **A242** créée, tête
devenue S196-1, titre redaté. **A242 propagée** par une note datée dans
SOURCES-MULTIPLES-S195 et COUPLAGE-DEUX-TRAINS-S194 : leurs valeurs ne sont pas remises
en cause — leurs configurations sont loin du bord — mais la phrase « énergie sous 1e-4 »
ne doit plus se lire comme une garantie de justesse. Jeton libre ; trois copies avancées
sur `master`, aucune suppression.

P4 S196 : REPLI-CROISEES-S196 §8–§9 reçus. **A241 requalifiée** — moitié « limite »
close (saturation à −0,52), moitié « cause » partielle (le repli pèse un tiers, la thèse
de S195 était trop forte). **A242** ouverte : le critère de conservation ne détecte pas
la sous-résolution — contre-exemple à un facteur cinq avec une énergie 65× sous le seuil.
**L277** écrite. **Aucun ADR.** Décomptes à porter en P5 : **123 ADR, 242 angles,
277 leçons, 18 invariants, 6 SPEC, 23 cas**.

P3b S196 : campagne exécutée, **empreinte 0xbcf2911362458c13**, deux exécutions
identiques ligne pour ligne. **Verdict : ni la prédiction 1 ni la 2.** Exposants sur
n=2..8 : dense −0,451, **impaire −0,525**, **paire −0,394**. Écart pair/impair =
**0,131** — au-dessus du seuil de réfutation (0,10), sous le seuil de confirmation
(0,20). Le repli agit **dans le sens prédit** mais n'explique que **32 %** de l'écart
à la loi dispersée (−0,805 sur cette plage) : les 68 % restants ont une autre cause.
**Prédiction 3 (limite) : confirmée** — fenêtres glissantes −0,438 / −0,520 / −0,520,
l'exposant sature vers **−0,52** dès n≈4, et `n ≤ 6` sous-estimait donc la pente
(−0,44 contre −0,52). **Prédiction 4 : confirmée** — dense contre paire, écart 0,057
sous le seuil de 0,10 : l'échelle absolue ne compte pas.
Réceptions : 1,2,3,4,5,6,8,9 passent (continuité S195 à **5,03e-8**, bande 0,001 à
0,032 %, phases 0,747 et 0,885). **La 7 échoue** : à n=6, ordre 1,268 (<1,5) et résidu
2,27 % (>2 %). Biais mesuré directement K=64→K=128 aux deux bouts : **+0,010** sur
l'exposant — négligeable devant 0,131, donc la conclusion tient.
**À retenir pour P4** : à n=16, K=32 donne une L2 fausse d'un **facteur 5** et la dérive
d'énergie ne l'a pas signalé (1,54e-6, sous le seuil de 1e-4). Le critère de domaine a
laissé passer une configuration cassée ; le triplet de Richardson y est inutilisable et
le banc le **dit** au lieu d'en tirer un ordre fictif.

P3a S196 : `support/nl_fleet.rs` extrait de `nl_sources_2d.rs` — flottille, `Spread`,
`sources`, plus `fleet_from_modes` général. **L'empreinte de S195 se reproduit à
l'identique (`0x5eb378f6ffe26c9f`)** : la refactorisation n'a pas déplacé d'arithmétique,
et c'était le contrôle. `nl_fallback_2d.rs` écrit ; **dix tests passent**, dont le
décompte de repli par construction pour les trois familles, le repli nul chez les
impairs à tout `n`, `paire == dense doublée`, le cas nul, `M=1`, et la continuité S195
à `10⁻⁶`. Workspace 331 réussis/cinq ignorés.

P2 S196 : protocole dans REPLI-CROISEES-S196. Le montage de parité est vérifié par
arithmétique **avant** d'être codé : repli 0,000 chez les impairs à tout `n` ; la famille
paire est **exactement** la dense aux modes doublés, donc même fraction de repli *et* même
bande relative, seule l'échelle change. Trois comparaisons : dense/paire (échelle),
paire/impaire (repli), dense seule (limite). Confondant résiduel déclaré : la bande
relative diffère de 24 % entre pair et impair, et c'est irréductible — deux parités ne
portent pas les mêmes nombres d'onde. Piège déclaré aussi : les termes **triples** de trois
impairs sont impairs et retombent, donc la parité n'éteint le repli que sur les paires.
**Quatre prédictions chiffrées d'avance**, dont la clé : la fraction de repli sature vers
**0,74** (arithmétique pure, calculée sans mesure), donc si le repli gouverne l'exposant,
l'exposant doit saturer aussi. Seuils : écart > 0,20 entre exposants pair/impair confirme ;
< 0,10 réfute A241. Continuité S195 à viser : L2 dense n=6 = **4,507029e-3**.

S196 : master 25206a8 propre, quatre copies alignées ; 123 ADR/241 angles/276 leçons.
**Acquis à ne pas refaire.** S195 : série A (cambrure totale fixée) décroît en `n^-0,46`,
série B (par train) croît en `n^0,75` ; les deux bornes dérivées donnaient −1 et 0 pour A.
Le jeu de phases **ne tranche pas** (facteur 1,05 à 1,39) — une seule série de phases
suffit donc ici, avec un point de contrôle à grand `n`. La bande ne déplace rien
(`Q=32` contre `Q=24` : 0,0000 %), donc elle peut être **serrée** : c'est ce qui rend `n`
grand abordable, le coût allant comme `Q²·(n+1)`.
**Idée du montage, à éprouver en P2** : la parité sépare le repli du reste. Trains tous
**impairs** → somme et différence de deux impairs sont paires, donc aucun terme croisé de
paire ne retombe sur un mode de train : **repli nul**. Trains tous **pairs** → tout y
retombe : **repli plein**. Même `n`, même cambrure totale, indices décalés de 1 seulement.
**Piège déjà repéré** : les termes **triples** `k_i±k_j±k_k` de trois impairs sont impairs
et retombent, eux. La parité ne supprime donc le repli que pour les termes de **paire**,
qui dominent — à dire dans le protocole, pas à découvrir après.
Stabilité vérifiée d'avance : `dt = T₁/400` reste sous `2,5/√(g k_Q)` tant que
`k_Q/k_1 < 25300` ; aucune contrainte pratique.

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

