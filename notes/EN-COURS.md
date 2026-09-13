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

Session : S222 — terminée
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Reprends le projet », conversation neuve — **aucune mémoire de S217 à S221**, tout a été
relu depuis `AGENTS.md`, `REPRISE.md` §4 et les trois ADR 135/136/137. master et trois copies à
c33953a, jeton libre, maillons 0. Copie principale.
Objectif : **A254, la part somme** — ce que coûte une scène à plusieurs sillages dans le budget de
pente, et ce qu'une borne locale conjointe lui rendrait.

### Ce que la relecture établit avant toute mesure

`mixed_water::slope_floor(impacts, pressure, time)` somme **un majorant par impact**
(`slope_max_at`, ADR-133) **plus un seul terme de pression** — la signature ne prend qu'un
`Option<&bound_pressure::Prepared>`. Donc :

- **plusieurs sillages ne peuvent entrer dans le budget qu'en partageant un journal**, une recette
  et une emprise ; c'est cette configuration que la ligne de suite nomme, et c'est la seule que le
  cœur sache composer aujourd'hui ;
- dans cette configuration, les sources partagent les **emplacements** du demi-spectre : leurs
  amplitudes modales s'additionnent **en complexe**, donc le terme de pression est déjà
  sous-additif par construction. La « part somme » d'A254 ne se lit donc pas sur le nombre de
  sillages de la même façon que sur le nombre d'impacts, et il faut le mesurer avant de le dire.

Les bornes locales (ADR-135, 136, 137) et leur partition (S219) **ne sont branchées sur aucune
admission** : elles publient, elles ne refusent pas. La question de la session est de savoir si
elles ont de quoi le faire.

### Thèse et critères, déclarés avant toute mesure

1. **Ce que l'enveloppe globale fait du nombre de sources**, mesuré et non supposé : à deux et
   trois sillages, proches puis éloignés, le rapport de `slope_envelope()` à sa valeur pour une
   seule source. Si la composition complexe la rend franchement sous-additive, A254 n'a pas de
   « part somme » du côté des sillages, et c'est la réponse.
2. **Ce qu'une borne locale conjointe rendrait** : partition spectrale (ADR-137) sur l'emprise, à
   **budget d'évaluations égal** d'une configuration à l'autre, contre le **maximum réel**
   échantillonné finement. Gain, et ce qui reste au-dessus du maximum.
3. **Traduire en part de π/7** : le budget d'admission complet — impacts toujours sommés
   (ADR-133) plus le terme de pression — avec l'enveloppe actuelle puis avec la borne partitionnée.
   Dire combien de sources passent dans chaque cas. C'est la seule forme qui réponde à A254.
4. **Le coût de passe fait partie du verdict.** S219 mesure 18 à 36 s pour 32 767 évaluations : un
   gain réel peut être **inutilisable** comme terme d'admission par image. Le dire, plutôt que
   publier un gain sans son prix.
5. Aucun seuil de réussite présumé ; publication avec techniques, domaine et rang de passage
   (L289). Aucune admission migrée avant P6.

**Prédiction écrite pour être contredite** : éloignés, deux et trois sillages donnent une enveloppe
globale qui croît nettement moins vite que le nombre de sources (facteur ≤ 1,6 à trois), parce que
les phases modales se désalignent ; le maximum réel, lui, reste proche de celui d'une source.
La borne locale partitionnée rend alors l'essentiel de l'écart — moins d'un facteur 1,5 au-dessus
du maximum — mais **à un coût de plusieurs secondes**, donc sans usage possible comme terme
d'admission par image. Autrement dit : je prédis un gain réel et inutilisable en l'état, et c'est
ce résultat-là qu'il faut savoir écrire s'il se produit.

### Plan

- [x] **P1** — jeton, entrée, ce que la relecture établit, thèse, critères, prédiction, plan seuls.
- [x] **P2** — fixture multi-sillages : deux et trois sources dans un même journal, proches puis éloignées ; témoin que la composition est bien partagée par emplacement, et refus éventuels de la bibliothèque.
- [x] **P3** — enveloppe globale et maximum réel par configuration ; sur-additivité mesurée.
- [x] **P4** — borne locale conjointe : partition spectrale à budget d'évaluations égal ; gain, reste au-dessus du maximum, **coût de passe**.
- [x] **P5** — part de π/7 : budget d'admission complet avec les impacts sommés ; combien de sources passent, avant et après.
- [x] **P6** — décider : ADR si quelque chose est rendu **et** utilisable ; sinon constat motivé, et retour à la file (cadence complète de l'hôte, V-noyau).
- [x] **P7** — document de réception (en-tête ADR-131 D3) ; suite complète `code/`.
- [x] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2+P3 : `code/water-core/examples/somme_sillages_s222.rs` — un seul programme porte les deux
étapes, la fixture n'ayant d'intérêt que mesurée. Recette et emprise de S219–S221, instant de la
fixture « base » (8 s de forçage puis τ = 4), sources sur des trajectoires parallèles écartées de
4 m (« proches », deux fois σ) ou 30 m (« éloignées »).

**Témoin de composition partagée** : `modes=4096` pour une, deux et trois sources. Les emplacements
du demi-spectre sont bien communs, donc les amplitudes modales s'additionnent **en complexe** — ce
que la relecture annonçait est vérifié plutôt que supposé. Aucun refus, ni à la construction, ni au
journal, ni à la préparation. Préparation 6,3 / 13,4 / 18,9 ms : elle, en revanche, est **linéaire**
en nombre de sources.

| config | sources | enveloppe globale | × une source | maximum réel | pessimisme |
|---|---:|---:|---:|---:|---:|
| proches (4 m) | 1 | 0,115171 | 1,0000 | 0,070316 | 1,6379 |
| proches | 2 | 0,165659 | 1,4384 | 0,111800 | 1,4818 |
| proches | 3 | 0,192466 | **1,6711** | 0,130667 | 1,4730 |
| éloignées (30 m) | 2 | 0,146879 | 1,2753 | 0,070390 | 2,0867 |
| éloignées | 3 | 0,165357 | **1,4358** | 0,070463 | **2,3467** |

**A254 n'a pas de « part somme » du côté des sillages, et c'est la première réponse.** Trois sources
coûtent 1,67 fois une seule quand elles sont proches, **1,44** quand elles sont éloignées — très
loin du facteur 3 que subissent les impacts. La composition modale complexe absorbe l'essentiel.

**Mais le pessimisme, lui, empire avec la séparation, et c'est le fait neuf.** Éloignées, le
maximum réel **ne bouge pas** — 0,070316 / 0,070390 / 0,070463 pour une, deux et trois sources :
chaque sillage a sa région, et le maximum reste celui d'un seul. L'enveloppe, elle, croît de 44 %.
Le pessimisme passe donc de 1,64 à **2,35**. **L'enveloppe pénalise la séparation**, exactement
parce qu'aucune enveloppe de modules ne voit la localisation spatiale — c'est A261, mesurée ici
pour la première fois sur une scène et non sur une maille.

P4 : **la borne locale rend tout, et son prix est structurel.**

**Partition sur l'emprise**, budget d'évaluations égal d'une configuration à l'autre :

| config | sources | 2 047 éval. | 8 191 éval. | 32 767 éval. | gain final | coût |
|---|---:|---:|---:|---:|---:|---:|
| proches | 3 | 1,0097 | 1,2206 | **borne/max 1,0053** | 1,4652 | 25,0 s |
| éloignées | 2 | 1,0031 | 1,1310 | **1,0088** | **2,0683** | 24,2 s |
| éloignées | 3 | 1,0023 | 1,1319 | **1,0099** | **2,3237** | 25,1 s |

À 32 767 évaluations la borne colle au maximum à **0,5–1,0 %** dans *toutes* les configurations, et
le gain sur l'enveloppe suit exactement le pessimisme mesuré en P3 : **2,32 quand les sources sont
éloignées**. La borne locale voit donc précisément ce que l'enveloppe ne voit pas.

**Mais j'avais supposé qu'une requête locale serait bon marché, et c'est faux sur les deux plans.**
Mesure ajoutée en P4 : un seul appel `local_slope_envelope_spectral` sur un rectangle.

| demi-côté | borne / enveloppe globale | borne / maximum local | µs par appel |
|---:|---:|---:|---:|
| 1 m (au pire point) | 1,03 à **1,09** | 1,41 à 2,15 | ~670 |
| 4 m | **0,9977** | 1,48 à 2,09 | ~650 |
| 16 m | **0,9977** | 1,48 à 2,09 | ~640 |

- **Au-delà d'environ un mètre de demi-côté, la borne locale *est* l'enveloppe globale** — 0,9977,
  c'est-à-dire l'enveloppe **plus sa réserve numérique**. La raison est dans ADR-137 : au-delà d'une
  demi-longueur d'onde, tous les modes tombent dans la classe non résolue `D ≥ 2`, la coupure les
  renvoie à `G(U)` — l'enveloppe ADR-134 —, et il ne reste rien à gagner. La plus courte longueur
  d'onde représentée vaut `2π/cutoff = 2,09 m` : **la maille doit être sous-ondulatoire ou elle ne
  sert à rien**.
- **Un appel coûte 640 à 700 µs**, pas quelques microsecondes : il est `O(N)` à 4 096 modes, soit
  ~160 ns par mode. Il n'y a donc pas de « requête locale bon marché » à opposer à la partition.

**Le prix est une loi d'échelle, pas un défaut d'implémentation.** Les mailles utiles font ≈ 2 m ;
couvrir l'emprise de 128 × 96 m en demande ~3 000, et atteindre 1,006 en demande 16 384. À
640 µs l'unité, cela fait 2 s et 25 s — exactement ce que la partition mesure. **Aucune optimisation
de constante ne franchit quatre ordres de grandeur** jusqu'au budget de 2 ms.

**Et A261 se chiffre au passage.** Loin de la source, à 2 × 2 m, la meilleure borne disponible vaut
**213 à 757 fois** le maximum local (0,0967 contre 0,000147 pour une source). Le terme des modes non
résolus, `G(U)`, ne dépend ni du point ni des phases : il est spatialement aveugle par construction,
et c'est lui qui plafonne tout. Une borne qui décroîtrait avec la distance au support de la source
n'existe pas dans cette représentation — le champ, lui, est bien localisé.

P5 : **A254, la part somme, traduite dans la monnaie qui décide.** Budget d'admission complet à
l'instant de la fixture : `π/7 = 0,448799`, terme d'impact unitaire `slope_max_at` = **0,030069**
(ADR-133 à τ ≈ 16,8), impacts toujours sommés.

| config | sillages | enveloppe actuelle | part | impacts admis | borne partitionnée | part | impacts | maximum réel | impacts |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| proches | 1 | 0,115171 | 25,7 % | **11** | 0,070740 | 15,8 % | 12 | 0,070316 | 12 |
| proches | 2 | 0,165659 | 36,9 % | **9** | 0,112379 | 25,0 % | 11 | 0,111800 | 11 |
| proches | 3 | 0,192466 | **42,9 %** | **8** | 0,131362 | 29,3 % | 10 | 0,130667 | 10 |
| éloignées | 2 | 0,146879 | 32,7 % | **10** | 0,071014 | 15,8 % | 12 | 0,070390 | 12 |
| éloignées | 3 | 0,165357 | 36,8 % | **9** | 0,071160 | **15,9 %** | **12** | 0,070463 | 12 |

**La part somme d'A254 n'est plus contraignante sur cette scène, et c'est la réponse.** En S214,
*une* source de chaque type consommait 84 % de π/7 et la deuxième refusait l'image. Après ADR-133 et
ADR-134, **trois sillages et huit impacts passent** — et neuf s'ils sont éloignés. Le budget a cessé
d'être le goulot ; ce n'est plus lui qui décide de ce qu'une scène peut porter.

**Ce que la borne partitionnée rendrait en plus : deux à trois impacts.** De 8 à 10 (proches), de 9
à 12 (éloignées) — 18 à 33 % de sources en plus. Et elle est **à un impact près du maximum réel dans
toutes les configurations** : il n'y a pas de troisième chemin à chercher, l'instrument est aussi
bon qu'il peut l'être. C'est son **prix** qui le disqualifie, pas sa qualité.

Le gain est donc réel, borné, et connu : **+2 à +3 impacts pour 25 secondes de calcul**, contre un
budget d'image de 2 ms. Quatre ordres de grandeur, et une loi d'échelle qui les défend (P4).

P6 : **avant de décider, j'ai vérifié le domaine de la conclusion de P5 — et il la corrige.**

Le terme d'impact dépend fortement de l'âge (ADR-133). Balayé contre le terme de pression de trois
sillages éloignés (0,165357) :

| âge de l'impact (s) | terme unitaire | part de π/7 | impacts admis |
|---:|---:|---:|---:|
| 0 à 0,5 | 0,212607 | **47,4 %** | **1** |
| 1 | 0,203180 | 45,3 % | 1 |
| 2 | 0,142269 | 31,7 % | 1 |
| 4 | 0,054900 | 12,2 % | 5 |
| 8 | 0,033892 | 7,6 % | 8 |
| 16 | 0,021545 | 4,8 % | 13 |
| 56 | 0,007390 | 1,7 % | 38 |

**Correction de P5.** J'y ai écrit « la part somme n'est plus contraignante ». C'est vrai à
l'instant mesuré, où les impacts ont une dizaine de secondes — et **faux pour des impacts jeunes** :
sous deux secondes, **un seul** impact passe à côté de trois sillages. Une scène à deux
éclaboussures simultanées est toujours refusée.

**Et la borne partitionnée n'y change rien** : avec la pression à 0,071160 au lieu de 0,165357, un
impact neuf (0,212607) laisse 0,166 — soit **un** impact encore. Le terme qui sature n'est pas la
pression, c'est le **majorant de naissance de l'impact**, et ADR-133 ne peut rien pour lui : à
τ = 0 il est **exactement atteint** (S215 : 0,999998). Ce n'est pas du pessimisme, c'est la physique
— une éclaboussure fraîche a vraiment une pente de 47 % de la cambrure de déferlement.

**La part somme d'A254 a donc changé de côté, et c'est le résultat de la session.**
- **Côté sillages** : elle est absorbée par la composition modale (1,44 à 1,67 pour trois sources),
  et ce qui reste est **spatial** (A261) — la borne locale le rend en entier, à un prix
  structurellement interdit.
- **Côté impacts** : elle est **littérale**. `slope_floor` somme `slope_max_at` par champ, **sans
  aucune conscience de la distance** : deux impacts frais à cent mètres l'un de l'autre s'ajoutent
  exactement comme s'ils étaient au même point. Rien dans S215–S222 ne l'a touchée — ADR-133 a
  resserré chaque majorant **dans le temps**, jamais leur **somme dans l'espace**.

### Décision

**Aucun ADR, aucune migration d'admission.** Trois raisons, dans cet ordre :

1. **Le prix disqualifie la borne partitionnée** : +2 à +3 impacts pour 25 s, contre un budget
   d'image de 2 ms. Quatre ordres de grandeur, et P4 montre que c'est une **loi d'échelle** — la
   maille utile est sous-ondulatoire, leur nombre croît comme l'aire divisée par λ_min², et chaque
   maille est `O(N)`. Aucune constante ne franchit cela.
2. **Le terme de pression n'est plus le goulot** : 16 à 43 % de π/7 pour un à trois sillages, et la
   borne le porterait à 16 %. Migrer coûterait des secondes pour desserrer ce qui ne serre plus.
3. **Le goulot est ailleurs, et il est nommé** : la somme spatiale sur les impacts. C'est là que la
   mesure envoie la suite, et non « retour à la file » — la file elle-même désignait A254, et A254
   n'est pas close : elle a changé de terme.

Ce que cette décision ne fait pas : elle ne retire rien à ADR-135/136/137, qui restent publiées et
mesurées ; elle ne dit pas que la borne locale est inutile — elle dit qu'elle n'est pas un **terme
d'admission par image**. Un usage hors image (validation, outillage auteur, banc) reste ouvert.

P7 : [SOMME-SILLAGES-S222](../docs/validation/SOMME-SILLAGES-S222.md) — en-tête ADR-131 D3 avec
rang de passage ; §1 le témoin de composition partagée ; §2 la sous-additivité et la pénalité de
séparation ; §3 la borne locale, sa requête locale écartée et sa loi d'échelle ; §4 la réponse en
part de π/7 **avec son domaine en âge**, qui corrige l'annonce ; §5 la décision de ne rien migrer.
Suite nommée : la somme spatiale sur les impacts.
Suite complète `code/` hors réseau : **370 réussis (272+4+1+93), 5 ignorés**, aucun échec —
identique à S221 : cette session n'a ajouté aucune ligne à `code/*/src`, seulement un exemple.

P8 : rituel §6 exécuté. Journal S222 ; **A262** (sévérité 2) ; suivis **A254** (change de côté) et
**A261** (chiffré sur une scène, 213 à 757 fois le maximum local loin de la source) ; **L302, L303**.
Index, README, REPRISE (§4, file active, jeton), feuille de route (bloc J1, ligne A254, ligne
mutualisation — elle n'est plus conditionnée par le budget de pente mais par le seul coût de passe),
file plurielle de QUESTIONS-OUVERTES.
**Invariants relus** — I-18 (rien n'a été substitué au terme comparé à `max_slope` : la mesure n'a
migré aucune admission) ; I-06 (aucune allocation ajoutée ; la partition emprunte le pool de
l'appelant) ; I-14 (aucun nombre calibré — la session ne pose aucune constante). Aucun devenu faux,
aucun amendé, aucun ADR réécrit ni acté.
**Règle des deux maillons : compteur 1.** Ni code d'exécution dans `code/*/src`, ni décision actée ;
`outils/velocite.sh` le confirme — **W = S221**, inchangé. Le compteur monte, et c'est correct : une
session qui mesure et conclut de ne rien changer n'avance aucune couche, même quand sa conclusion
est utile. La suite nomme **A262**, née de la mesure mais rattachée à **A254**, ligne de la file
active ; à 2, la session suivante devra choisir dans la file et dire la couche qu'elle avance.
**Recommandation portée** : la ligne `Session suivante` nomme A262 et la couche W.
Décomptes vérifiés : 137 fichiers dans `docs/adr` (inchangé), 303 leçons, 262 angles.
Jeton libre, battement 17:59. Copies de travail avancées sur master après ce commit.
