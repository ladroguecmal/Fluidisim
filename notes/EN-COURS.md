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

```
Session          : S60
État             : terminée
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S57-2 — éprouver le critère d'admission de C22, et le décider par ADR.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — poser ce que le filtre protège, et instrumenter : calculer les ordres contre **chacun** des deux oracles, sans toucher à aucun critère.
- [x] **P3** — contre-épreuve rétrospective sur 51200/102400, la campagne que le filtre a fait refuser en S56 ; et essai de refus sur un oracle franchement trop grossier.
- [x] **P4** — ADR-049 : trancher au vu des deux mesures, ou refuser de trancher en disant ce qui manque.
- [x] **P5** — appliquer la décision au code, avec ses essais de refus et ses témoins.
- [x] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, jeton, **fusion dans master**.

### Notes de reprise

Départ 283e82a, master et worktree confondus.

**Le piège de cette session est nommé d'avance.** Un critère d'admission qu'on rouvre après
avoir obtenu un succès, c'est le geste que le dépôt refuse ailleurs — *« la session qui rendra
C04 vert devra changer de schéma, pas de seuil »*. **Rien ne sera assoupli pour obtenir un
résultat.** Si la mesure ne tranche pas, la sortie légitime est de ne pas trancher et de dire
ce qui manque.

**Ce que le filtre veut protéger** : que l'erreur mesurée d'une grille soit dominée par sa
propre erreur de discrétisation, et non par celle de l'oracle. Le filtre actuel en est un
**proxy global** — `e(n) ≥ 30 × ‖o1 − o2‖` — et A179 a mesuré qu'il est piloté par le biais du
plus **grossier** des deux oracles, dont aucune erreur publiée ne dépend.

**La piste, et sa contre-épreuve.** L'ordre est estimé sur des **différences successives**, où
une contamination additive uniforme s'annule : mesuré en S59, les ordres calculés contre les
deux oracles coïncident à 1e-5 sur les six triplets. Un critère direct serait donc l'**invariance
de la grandeur publiée au choix de l'oracle**, qui porte sur ce qu'on publie et se lit sans
modèle. Mais il faut d'abord répondre à deux questions, et par la mesure :

1. **Aurait-il conclu en S56**, avec 51200/102400 — la campagne que le filtre a refusée ? Si oui
   avec la même valeur d'ordre, le filtre a coûté 32 minutes pour rien. Si avec une valeur
   différente, il protégeait, et la piste tombe.
2. **Refuse-t-il ce qu'il doit refuser ?** Un oracle franchement trop grossier doit faire
   diverger les deux ordres. Un critère qu'on n'a jamais vu refuser n'a pas été testé (S34).

Réserve à garder au chaud : deux oracles du même schéma partagent leur erreur de modèle. Le
critère proposé n'y remédie pas — il ne la voit pas davantage que le filtre actuel (A114,
ADR-043 §3). Ce serait à écrire dans la décision, pas à passer sous silence.

P2 : invariance a l'oracle mesuree et affichee, y compris hors filtre. Aucun critere touche.
Test synthetique : un biais uniforme mille fois l'erreur la plus fine ne deplace pas l'ordre ;
un biais heterogene cent fois plus petit le deplace. Le critere ne voit que l'heterogeneite.

P3, deux mesures, et elles vont en sens contraire.
 (a) Essai de refus, oracle 3200/6400 : contamination flagrante — e(1600) sous-estimee de 5,2 %,
     deux grilles refusees — et l'invariance ne vaut que 5,1e-3. **Elle ne refuse pas ce qu'elle
     devrait refuser** : deux oracles voisins partagent leur erreur (A114). Elle ne peut pas
     remplacer le filtre. Mais elle **majore** l'erreur reelle sur l'ordre d'un facteur 31,5 ;
     a 51200 d'un facteur 3,1. Majorant conservateur, dans l'unite de la grandeur publiee.
 (b) Contre-epreuve retrospective, 51200/102400 en 371,972 s : le triplet du verdict rend
     **1,997566515**, contre 1,997599436 en S59 — **ecart 3,29e-5**. L'ordre publie par S59
     etait deja mesurable en S56. Entre les deux, 1987,7 s de calcul (33 min 08 s) et deux
     sessions, pour deplacer un ordre de trois centiemes de millieme.
 88 tests harnais, 40 coeur, deux ignores.

P4 : ADR-049. D1 filtre conserve sans aucune modification ; D2 A179 requalifie — mal attribue
et non mal calibre, C22 publie deux grandeurs de nature differente sous un seul critere (A182) ;
D3 invariance publiee comme diagnostic, y compris hors filtre, et disqualifiee comme critere ;
D4 remplacement ouvert par decision, avec l experience manquante nommee (S60-1 : une grille dont
l erreur passe sous l ecart des oracles). Aucun verdict ne change, aucun resultat n est rouvert.

P5 : sortie renommee en diagnostic sans verdict, avec renvoi a ADR-049 D3 sur la ligne de
synthese — une valeur publiee sans etre admissible doit le dire sur la ligne qui la porte.
128 tests reussis (40 coeur + 88 harnais), deux ignores ; hashs check inchanges. Aucun verdict
deplace : le filtre, les seuils et les familles sont ceux de S59.

P6 : rituel exécuté. Journal S60, A182 (sévérité 2, ouvert par décision), L178, S57-2 close,
S60-1 ouverte et prioritaire. Index et REPRISE : ADR 48 vers 49, angles 181 vers 182, tests
127 vers 128. Aucun invariant invalidé, aucun ADR réécrit, aucun verdict déplacé.
Jeton libre. Reste la fusion dans master.
