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

Session : S226 — terminée
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue avec S226 », même conversation. master et trois copies à e75f6aa, jeton libre,
**maillons 1**. Copie principale.
Objectif : **V — direction de `g_eff`**. La consigne de S224 arrive à échéance ; S225 en a consommé
la session de battement.

### Ce que le module viole aujourd'hui, et il faut le dire ainsi

`hydro_network::step` prend `g_eff: f32` — un **module**. La direction est donc en dur : les
hauteurs sont des scalaires comptés le long de `+Z`, et rien dans le code ne dit lequel.

> **I-07** — « Tout domaine appartient à un référentiel. Il reçoit `g_eff` par injection. Une
> constante `−9,81·Z` écrite en dur dans un solveur est un **défaut bloquant**. »

La constante n'est pas écrite, mais l'axe l'est, ce qui revient au même. S224 l'avait noté en
réserve — « en module seulement, et c'est dit » — et c'était insuffisant : I-07 ne demande pas qu'on
le dise, il demande que ce ne soit pas le cas. **C12 passait quand même**, parce qu'un réservoir posé
à plat ne distingue pas les deux.

Ce qu'ADR-010 §2 exige, mot pour mot : *« Le plan d'eau est perpendiculaire à `g_eff`, pas à Z. La
hauteur d'une ouverture est donc évaluée par sa distance signée au plan de surface orienté selon
`g_eff`. Sans cela, un vaisseau qui accélère ne verrait pas son réservoir fuir par le hublot latéral
qui se retrouve "en bas". »*

### Forme déclarée avant d'écrire

- `g_eff: [f32; 3]`, et `u = −g_eff/‖g_eff‖` la verticale locale.
- `HydroNode.origin_um: [i64; 3]` remplace `floor_um` : le point de référence depuis lequel la table
  de forme compte la hauteur de surface le long de `u`.
- `Opening.position_um: [i64; 3]` remplace `sill_um` : une ouverture est **quelque part**, pas à une
  hauteur.
- Charge à un point `q` pour le nœud `n` : `h_n − (q − c_n)·u`, avec `h_n` la hauteur donnée par la
  table. Projection calculée en `f64` depuis des différences entières — le module emploie déjà `f64`
  pour le débit, et IEEE strict le rend reproductible (I-03).

### Thèse et critères, déclarés avant toute mesure

1. **Réduction exacte.** À `g_eff = [0, 0, −9,81]`, le module rend **exactement** ce qu'il rendait :
   C12 à la même seconde, mêmes volumes pas à pas. Une généralisation qui déplace le cas plat serait
   refusée — c'est le contrôle qui sépare une généralisation d'une réécriture.
2. **La phrase d'ADR-010, éprouvée telle qu'elle est écrite** : un réservoir dont l'ouverture est
   **latérale** ne fuit pas sous gravité verticale, et **fuit** sous accélération latérale. C'est le
   test que le module actuel ne peut pas passer, quelle que soit sa précision.
3. **C16, part V.** Le cas exige « inclinaison de la surface au repos à ±1° de la normale à
   `g_eff` ». La part ballottement relève de δ ; la part V est l'orientation du plan, et elle se
   vérifie sur la charge aux ouvertures.
4. Déterminisme, refus atomiques et absence d'allocation **conservés** : ce sont des acquis de S224,
   pas des objectifs neufs, et une généralisation qui les casserait serait un recul.

**Prédiction écrite pour être contredite** : la réduction au cas vertical sera exacte au bit, et la
fuite latérale apparaîtra. Mais je prédis surtout un **obstacle que la construction va révéler** :
la table de forme d'ADR-010 §2 est cuite « par coupes **horizontales** », donc valable pour **une
seule orientation**. Dès que `g_eff` s'incline, la relation volume → hauteur change, et à 0,3 g
latéral l'inclinaison vaut `atan(0,3) = 16,7°` — ce n'est pas un petit angle. Je prédis donc que la
direction se corrige pour les **ouvertures** — ce qu'ADR-010 nomme — et **pas** pour le volume, et
que l'incohérence est dans l'ADR elle-même, pas dans le module. Si c'est faux, tant mieux ; si c'est
vrai, c'est un angle mort et il vaut d'être nommé.

### Plan

- [x] **P1** — jeton, la violation d'I-07, la forme, la thèse, la prédiction, le plan seuls.
- [x] **P2** — généraliser : `g_eff` vectoriel, points de référence et positions d'ouverture ; réduction au cas vertical vérifiée **au bit**.
- [x] **P3** — éprouver la phrase d'ADR-010 : hublot latéral, sous gravité verticale puis sous accélération latérale ; part V de C16.
- [x] **P4** — mesurer ce que la table de forme perd quand `g_eff` s'incline, et le nommer.
- [x] **P5** — document de réception ; suite complète `code/`.
- [x] **P6** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*

P2 : `g_eff` devient `[f32; 3]`, `HydroNode.floor_um` devient `origin_um: [i64; 3]`, et
`Opening.sill_um` devient `position_um: [i64; 3]` — **une ouverture est quelque part, pas à une
hauteur**. La charge se calcule comme ADR-010 §2 l'écrit : cote de l'ouverture **le long de la
verticale locale** `u = −g_eff/‖g_eff‖`, retranchée à la hauteur de surface donnée par la table.

La projection se fait en `f64` depuis des **différences entières** : sous `u = (0, 0, 1)` elle rend
exactement `q_z − c_z`, donc la soustraction d'altitudes d'avant. Le module employait déjà `f64`
pour le débit ; IEEE strict le garde reproductible (I-03).

**Critère 1 tenu, et c'est le garde-fou qui sépare une généralisation d'une réécriture** : sous
`g_eff = [0, 0, −9,81]`, **tous les nombres de S224 sont identiques** — C12 à 727,4 s et 0,0824 %,
l'arrêt sans report à 13 ml, la chaîne à [532351, 300688, 166961], les exposants du déversoir et de
l'orifice à 2,8284 et 1,4151. Neuf tests, rien n'a bougé.

Les refus gagnent une cause : `g_eff` de norme nulle ou non finie rend `Domain`, comme un pas nul.

P3 : **la phrase d'ADR-010 §2 est éprouvée telle qu'elle est écrite.** Cuve de 4 m² de section et
2 m de haut, remplie à 1 m, hublot **latéral** à `x = +2 m`, `z = 1,2 m` — au-dessus de la surface
au repos. Sous gravité verticale : **0 ml**. Sous 0,3 g latéral — le montage de C16 : **1 829 ml en
dix pas**. C'est exactement le test que le module de S224 ne pouvait pas passer, quelle que soit sa
précision, puisqu'il ne recevait que le module de `g_eff`.

**C16, part V** : l'inclinaison n'est pas lue dans le code, elle est **déduite du comportement**. À
deux abscisses, la cote à laquelle une ouverture se met à débiter est encadrée par dichotomie ; la
frontière entre « débite » et « ne débite pas » **est** le plan de surface. Seuils 444 045 µm à
`x = −2 m` et 1 644 026 µm à `x = +2 m`, soit **16,6990°** contre **16,6992°** attendus — à
**0,0002°**, très loin du degré qu'exige C16.

**Erreur de ma part, et elle était dans le test, pas dans le code.** Le premier attendu posait
`atan(−a/g)` et se trompait de **signe** : l'eau s'accumule du côté où le « bas » penche, donc la
surface y **monte**. Le code rendait la bonne valeur ; c'est l'attendu qui a dû être corrigé, et la
raison est écrite dans le test — la pente d'un plan perpendiculaire à `g` vaut `−g_x/g_z`.

Onze tests passent.

P4 : **ma prédiction est contredite pour le prisme, et confirmée là où la table existe justement.**

J'annonçais que la table de forme deviendrait fausse dès que `g_eff` s'incline. Intégration
numérique de l'aire sous un plan incliné de 0,3 g, deux sections, 200 001 tranches :

| section | surface au centre | aire à plat | aire inclinée | écart |
|---|---:|---:|---:|---:|
| prisme | 0,4 m | 1,600000 | 1,666667 | 4,17 % |
| prisme | **1,0 m** | 4,000000 | 4,000000 | **0,0000 %** |
| prisme | 1,6 m | 6,400000 | 6,333333 | 1,04 % |
| coque en V | 0,4 m | 0,160000 | 0,175824 | **9,89 %** |
| coque en V | **1,0 m** | 1,000000 | 1,098901 | **9,89 %** |
| coque en V | 1,6 m | 2,560000 | 2,717949 | 6,17 % |

**Pour un prisme à parois verticales, la table reste exacte** tant que le plan incliné ne touche ni
le fond ni le plafond : le coin gagné d'un côté vaut exactement celui perdu de l'autre. Elle ne
dérape qu'aux extrêmes — 4,17 % près du fond, 1,04 % près du plafond, là où un coin est tronqué.

**Pour une coque en V, elle se trompe de 9,89 % partout, y compris en plein milieu.** Et c'est
exactement le cas pour lequel `shape_lut` existe : ADR-010 §2 la justifie en écrivant *« Un
compartiment n'est pas un prisme : la relation volume → hauteur d'une cale, d'un fond de citerne
bombé ou d'une dépression de terrain est non linéaire. »*

**Les deux dispositions d'ADR-010 §2 sont donc incohérentes là où l'une comme l'autre servent** :
la table est cuite par coupes **horizontales**, et le plan d'eau est perpendiculaire à `g_eff`.
Tant que le contenant est un prisme et que l'eau n'est ni au fond ni au plafond, les deux
coïncident ; dès que le contenant est une cale, non. Sur la charge, 9,9 % d'erreur de hauteur
donnent environ 5 % sur le débit, qui va comme `√h`. **A266**, et ce n'est pas un défaut du module :
c'est une incohérence de la conception, que la construction a rendue visible.

P5 : [GRAVITE-DIRIGEE-S226](../docs/validation/GRAVITE-DIRIGEE-S226.md) — en-tête ADR-131 D3 ;
§1 ce que le module violait, et le fait qu'un cas canonique bien choisi puisse être **muet** sur un
invariant ; §2 la généralisation et sa réduction exacte ; §3 la phrase d'ADR-010 et C16, avec
l'erreur de signe qui était dans le test et non dans le code ; §4 **l'incohérence d'ADR-010 §2**,
mesurée ; §5 les contrôles. Suite : A266 à trancher avant tout contenant non prismatique, puis
l'état répliqué.
Suite complète `code/` : **383 réussis, 5 ignorés**, aucun avertissement neuf.

P6 : rituel §6 exécuté. Journal S226 ; **A266** (gravité 1 — ADR-010 §2 se contredit : table par
coupes horizontales contre plan perpendiculaire à `g_eff`, 9,89 % d'écart sur une cale) ; suivi
**I-07** (violation levée, et vérifiée par le comportement) ; **L310, L311**. Index, README, REPRISE
(§4, file active, jeton), feuille de route (V-noyau), file plurielle.
**Invariants relus** — **I-07** en premier, puisque c'est lui que la session répare : il ne suffit
pas de dire qu'on ne prend que le module, il faut ne pas le faire ; **I-03** (projection en `f64`
depuis des différences entières, IEEE strict, réduction **au bit** sous gravité verticale) ;
**I-10** (l'état demeure entier) ; **I-06** (aucune allocation ajoutée). Aucun n'est devenu faux ;
I-07 **cesse d'être violé**, ce qui est un changement d'état et non un amendement, et le suivi le
consigne comme tel.
**Règle des deux maillons : compteur remis à 0**, et par le code — `outils/velocite.sh` donne
**`V ... derniere avancee=S226`**. Le compteur était à 1 en entrant.
**Recommandation portée** : la consigne de S224, reprise par S225, est tenue — V a avancé. La ligne
`Session suivante` nomme A266, qui est la conséquence directe de ce qui vient d'être construit et
une ligne de gravité 1, avant de reprendre l'état répliqué.
Décomptes vérifiés : 138 fichiers dans `docs/adr` (inchangé), 311 leçons, 266 angles.
Jeton libre, battement 19:19. Copies de travail avancées sur master après ce commit.
