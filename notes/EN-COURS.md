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

Session : S224 — terminée
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue avec S224 », même conversation. master et trois copies à 71555d0, jeton libre,
maillons 0. Copie principale.
Objectif : **ouvrir la couche V**. Premier module du graphe hydraulique (ADR-010), reçu par **C12**.

### Le choix, et pourquoi il n'est pas le plus confortable

La ligne `Session suivante` de S223 en offrait deux : **cadence complète de l'hôte** — travail
nécessaire de J1, adjacent à tout ce que je viens de faire — et **V-noyau**. Je prends V, et il faut
dire pourquoi, parce que c'est précisément le genre de décision que le dépôt a mesurée et jugée.

- **V a zéro module en 223 sessions.** `outils/velocite.sh` : `V modules=0 derniere avancee=jamais`.
  B date de S211, W de S223, δ de S202.
- **ADR-127 la rend obligatoire** et la feuille de route écrit, pour V-noyau : *« Rien ne l'empêche
  de commencer en parallèle de J2. »* Ce n'est donc pas un blocage technique qui l'a retardée.
- **BILAN-VELOCITE-S198 a mesuré ce mécanisme exact** : 33 sessions sur 38 prenaient le reliquat de
  la précédente, et la ligne `Session suivante` « a toujours raison localement ». La cadence de
  l'hôte est adjacente, chaude, et me tend les bras ; c'est exactement ce qui la rend suspecte.
- **J'ai moi-même écrit en S223** : « ne pas laisser V glisser d'une session de plus ». Une consigne
  qu'on s'écrit à soi-même ne vaut que si on l'applique quand elle coûte.

La cadence de l'hôte reste due et n'est pas abandonnée : elle est reportée **explicitement**, pas
oubliée, et la ligne de fin de session la reprendra.

### Ce que la conception donne déjà, et qui fait que ceci est une construction et non un dessin

ADR-010 est complète et **actée depuis S01** : nœud (`volume_ml` entier, `capacity_ml`, `shape_lut`
volume → hauteur), arête (Torricelli `Q = C_d·A·√(2·g_eff·Δh)`, déversoir en `H^{3/2}`), pas fixe de
**100 ms**, débits calculés en flottant puis **quantifiés en millilitres avec report de reste**,
limiteur `transfert ≤ min(volume_amont, capacité_libre_aval)` après **normalisation** quand
plusieurs arêtes vident le même nœud, 2 à 4 itérations de Gauss-Seidel.

Les invariants qui la contraignent sont connus : **I-03** (déterminisme bit à bit, V nommément),
**I-10** (le serveur exécute V, arithmétique entière, 10 Hz), **I-06** (pools de l'hôte, aucune
allocation à l'exécution), **I-14** (aucun nombre sans provenance).

### Thèse et critères, déclarés avant toute ligne

1. **Réception par C12, qui est analytique et sévère.** Réservoir de 1 m², hauteur 1 m, orifice de
   10 cm² à arête vive : `t_vidange = (A/(C_d·a))·√(2h₀/g) = 728 s`. Assertion du cas : **±3 %**, et
   **masse conservée à la milli-fraction près**. Le second critère est le plus dur : il interdit
   toute perte d'arrondi, et c'est lui qui force le report de reste.
2. **Conservation exacte, pas approchée.** Somme des `volume_ml` constante au millilitre près sur
   toute la vidange quand le système est fermé ; aucun volume négatif, aucun dépassement de
   capacité, à aucun pas.
3. **Déterminisme (I-03)** : deux exécutions du même réseau donnent la **même** suite d'états, au
   bit ; l'ordre de parcours des arêtes est fixé et ne dépend d'aucune adresse.
4. **Aucune allocation** dans le pas (I-06) : nœuds et arêtes viennent de tranches fournies par
   l'appelant.
5. **Refus atomiques** : un pas refusé ne modifie aucun volume — même exigence que `δ` en S200.
6. Publication avec en-tête ADR-131 D3 et rang de passage si un temps est mesuré.

**Prédiction écrite pour être contredite** : le piège ne sera pas la loi de Torricelli mais la
**quantification**. J'attends que le report de reste soit nécessaire dès le premier essai — sans
lui, une vidange de 7 280 pas perd assez de millilitres pour sortir des 3 %, et surtout la masse ne
sera pas conservée. J'attends aussi que le pas de 100 ms soit **trop grossier près de la fin** : le
limiteur devra mordre sur les derniers pas, et c'est là que se joue le respect des ±3 %.

### Plan

- [x] **P1** — jeton, choix motivé, ce que la conception donne, thèse, critères, prédiction, plan seuls.
- [x] **P2** — lire SPEC-004/006 sur ce que V publie, et fixer la forme du module : types, pools, refus. Déclarer avant d'écrire.
- [x] **P3** — construire le noyau : nœuds, arêtes d'orifice, pas à 100 ms, quantification à report de reste, limiteur avec normalisation.
- [x] **P4** — recevoir C12 : temps de vidange contre 728 s, conservation, non-négativité, capacité.
- [x] **P5** — déterminisme et refus atomiques ; aucune allocation dans le pas.
- [x] **P6** — déversoir de débordement et chaîne de nœuds (Gauss-Seidel), si P4 et P5 tiennent ; sinon dire ce qui manque.
- [x] **P7** — document de réception ; suite complète `code/`.
- [x] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2 : SPEC-004 §unités ne donne qu'une ligne sur V — `Volume | i64 | millilitres | couche V
uniquement (ADR-010)` — et SPEC-006 confirme que le millilitre est « la convention de la couche V,
qui compte des volumes de **contenants** ». Aucune forme d'interface n'est prescrite ailleurs :
ADR-010 et les invariants sont donc tout le cahier des charges, et il est complet.

**Forme déclarée avant d'écrire** — `code/water-core/src/hydro_network.rs` :

- `HydroNode { volume_ml: i64, capacity_ml: i64, shape: u16 }` — l'état est **entier**, la forme est
  un indice dans une table de l'hôte.
- `ShapeTable` : hauteurs en **micromètres**, 64 entrées par forme, croissantes, fournies par
  l'hôte (ADR-010 : `shape_lut` est cuite hors ligne). Hauteur par interpolation linéaire en
  volume ; un prisme y est donc **exact**, ce dont C12 a besoin.
- `Orifice { from: u16, to: Option<u16>, area_mm2: i32, sill_um: i32, discharge: f32, residue: i64 }`
  — `to = None` est le rejet hors réseau, dont C12 a besoin ; `residue` porte le report de reste
  d'ADR-010 §4.
- `step(nodes, edges, shapes, g_eff, dt_us) -> Result<(), Error>` : pas fixe de 100 ms, tranches
  fournies par l'appelant — **aucune allocation** (I-06) — et parcours des arêtes dans l'ordre du
  tableau, fixé, sans adresse (I-03).
- Refus : `Capacity`, `Shape` (table non croissante ou mal dimensionnée), `Domain` (`g_eff` ou pas
  non finis, aires négatives), `NonFinite`. **Atomiques** : un pas refusé ne modifie aucun volume.

**Une correction à ma propre prédiction, avant de mesurer.** J'ai écrit en P1 que sans report de
reste « la masse ne sera pas conservée ». C'est faux par construction : un transfert **entier**
retiré d'un nœud et ajouté à l'autre conserve la masse exactement, quelle que soit la troncature.
Ce que la troncature dégrade, c'est le **débit** — donc le temps de vidange, donc les ±3 %. Le
report de reste sert à la précision du débit et au déterminisme, pas à la conservation. Je mesurerai
les deux variantes pour le dire avec un chiffre plutôt qu'avec un raisonnement.

P3+P4+P5 : `code/water-core/src/hydro_network.rs` — **la couche V existe**. Un seul programme porte
les trois étapes, la construction n'ayant de sens que reçue.

**C12 : vidange en 727,4 s contre 728 s de référence analytique — écart 0,0824 %**, très loin des
±3 % du cas. Masse conservée exactement, volumes dans leurs bornes à chaque pas, réservoir vide à la
fin. Sept tests passent ; suite complète **377 réussis (279+4+1+93), 5 ignorés** — six de plus que
S223, aucun avertissement neuf.

**Deux défauts trouvés en construisant, et ils valaient le détour.**

1. **L'interpolation de hauteur tronquait, et cela arrêtait la vidange.** À un millilitre dans un
   réservoir de 1 m², la hauteur vaut un micromètre ; l'interpolation entière rendait
   `999999/1000000 = 0`. Charge nulle, débit nul, contenant qui ne se vide plus. **Arrondi au plus
   proche** au lieu de troncature : le plancher de représentation demeure — la hauteur est
   entière — mais il vaut une unité et non deux.
2. **La normalisation en nanolitres empêchait la quantification d'aboutir.** ADR-010 §4 demande que
   plusieurs arêtes vidant le même nœud soient réduites « dans la même proportion ». Faite **avant**
   la quantification, elle donne à chaque arête une part sous le millilitre, qui s'arrondit à zéro :
   un nœud de 2 ml avec trois fuites gardait 2 ml indéfiniment. La normalisation est passée **après**
   quantification et **en millilitres**, par **arrondi cumulatif** — les parts somment alors
   exactement au volume disponible, chacune est à moins d'un millilitre de sa valeur
   proportionnelle, et l'ordre du tableau suffit à la reproduire (I-03), sans reste à stocker.

**Prédiction confirmée, avec son chiffre.** J'annonçais que le report de reste serait nécessaire et
que le pas de 100 ms mordrait près de la fin. Le test `without_the_residue_carry_the_drain_stalls`
le mesure : sans report, **la vidange s'arrête à 13 ml** — et c'est exactement le seuil dérivé, le
débit d'un pas passant sous le millilitre quand la charge descend sous ≈13 µm. La moitié de la
prédiction que P2 avait déjà corrigée — « la masse ne sera pas conservée » — reste fausse : la masse
l'est par construction.

Reçus : déterminisme (deux exécutions, 500 pas, traces identiques), refus **atomiques** nommés
(`Domain`, `Capacity`, `Shape`) sans qu'aucun volume bouge, capacité aval jamais dépassée,
non-négativité sous trois fuites concurrentes. Aucune allocation dans le pas : nœuds, arêtes,
formes et scratch viennent de l'appelant (I-06).

P6 : **déversoir construit, et la question Gauss-Seidel mesurée plutôt que supposée.**

L'arête devient `Opening { flow: Flow::Orifice { area_mm2 } | Flow::Weir { width_mm }, … }` — le
nom suit ADR-010, qui écrit « Edge = ouverture » et donne **deux** lois. Le déversoir applique
`Q = (2/3)·C_d·b·√(2g)·H^{3/2}` avec `WEIR_DISCHARGE = 0,60`.

**Les deux lois se distinguent par leur exposant, et le test le mesure** plutôt que de relire la
formule : doubler la charge multiplie le débit par **2,8284** au déversoir — `2^{3/2} = 2,8284` — et
par **1,4151** à l'orifice — `√2 = 1,4142`. C'est le contrôle qui attrape une loi recopiée dans la
mauvaise branche.

**Gauss-Seidel : ADR-010 §4 dit « 2 à 4 itérations par pas suffisent pour un réseau ouvert ». Ce
module n'en fait aucune** — un seul passage explicite depuis l'état du début de pas. Plutôt que de
supposer que c'est assez, le test compare le pas de 100 ms à une intégration **cent fois plus
fine** : chaîne de trois contenants, 60 s, 600 pas contre 60 000. Écart maximal **0,0058 % de la
capacité** — [532351, 300688, 166961] contre [532392, 300630, 166978] ml.
**Un seul passage explicite suffit donc à 10 Hz sur cette configuration.** Ce n'est pas une preuve
générale : un réseau plus raide — grandes sections, faibles volumes — n'est pas couvert, et le
réseau **fermé sous pression** reste hors de portée par décision d'ADR-010 §4.

Neuf tests ; suite complète **379 réussis, 5 ignorés**.

P7 : [NOYAU-V-S224](../docs/validation/NOYAU-V-S224.md) — en-tête ADR-131 D3 ; §1 ce que le module
est et quels invariants ont dicté sa forme ; §2 C12 reçu à 0,0824 % ; §3 les deux défauts trouvés en
construisant ; §4 le report de reste et le seuil de 13 ml ; §5 le déversoir par son exposant et la
question Gauss-Seidel **mesurée** ; §6 les neuf réceptions ; suite : ce qu'ADR-010 contient encore
et n'est pas construit — dont la **surface libre en référentiel accéléré**, spécifiée et absente.
Suite complète `code/` : **379 réussis, 5 ignorés**, aucun avertissement neuf.

P8 : rituel §6 exécuté. Journal S224 ; **A264** (sévérité 2 — le plancher de vidange croît avec la
surface : 1 ml pour 1 m², 10 L pour un hectare) ; suivi **A17** (`liquid_id` est désormais une
structure à étendre, plus un paragraphe) ; **L306, L307**. Index, README, REPRISE (§4, file active,
jeton), feuille de route (**V-noyau passe de « aucun module » à sa liste de restes**), file plurielle
de QUESTIONS-OUVERTES.
**Invariants relus — et ils ont dicté la forme, pas été relus après** : **I-03** (ordre du tableau,
arrondi cumulatif sans reste stocké ; deux exécutions identiques au bit), **I-10** (état entier,
10 Hz), **I-06** (aucune allocation : nœuds, arêtes, formes et scratch viennent de l'appelant),
**I-07** (`g_eff` injectée — **en module seulement**, et c'est dit dans la réception comme dans la
feuille de route). Aucun devenu faux, aucun amendé, aucun ADR réécrit ni acté : ADR-010 était
complète.
**Règle des deux maillons : compteur 0**, et par le code — `outils/velocite.sh` donne **`V
modules=1 derniere avancee=S224`** là où il affichait `jamais` depuis 223 sessions, et W = S224.
**Recommandation portée** : la ligne `Session suivante` tient la promesse faite en P1 — la cadence
de l'hôte, reportée explicitement et non oubliée — **et** nomme dès maintenant les deux briques
suivantes de V, avec la consigne de ne pas laisser passer plus d'une session.
Décomptes vérifiés : 138 fichiers dans `docs/adr` (inchangé), 307 leçons, 264 angles.
Jeton libre, battement 18:51. Copies de travail avancées sur master après ce commit.
