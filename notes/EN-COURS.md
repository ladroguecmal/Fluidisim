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

Session : S224 — en cours
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
- [ ] **P2** — lire SPEC-004/006 sur ce que V publie, et fixer la forme du module : types, pools, refus. Déclarer avant d'écrire.
- [ ] **P3** — construire le noyau : nœuds, arêtes d'orifice, pas à 100 ms, quantification à report de reste, limiteur avec normalisation.
- [ ] **P4** — recevoir C12 : temps de vidange contre 728 s, conservation, non-négativité, capacité.
- [ ] **P5** — déterminisme et refus atomiques ; aucune allocation dans le pas.
- [ ] **P6** — déversoir de débordement et chaîne de nœuds (Gauss-Seidel), si P4 et P5 tiennent ; sinon dire ce qui manque.
- [ ] **P7** — document de réception ; suite complète `code/`.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*
