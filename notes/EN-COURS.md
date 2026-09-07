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
Session          : S61
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S60-1 — atteindre le régime où l'erreur d'une grille passe sous l'écart des
                   oracles, ou établir qu'il est hors d'atteinte et dire ce qui le remplace.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — chiffrer l'expérience prescrite avant de la lancer, et établir si le régime visé est atteignable dans ce dispositif.
- [x] **P3** — si non : dériver ce que le filtre ×30 exige réellement, et **le vérifier contre les cinq campagnes historiques**, dont les refus sont connus.
- [x] **P4** — conclure : note corrective datée sur ADR-049 D4, et ce qui remplace l'expérience impossible.
- [x] **P5** — appliquer au code ce qui doit l'être, avec essais de refus et témoins.
- [ ] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, jeton, **fusion dans master**.

### Notes de reprise

Départ 9f63f2c. **Le premier geste est celui que S59 a appris (L177) : éprouver la prescription
avant de l'exécuter.** S60 a nommé une expérience — « une grille dont l'erreur passe sous l'écart
des oracles, 25600 contre 51200/102400, moins de sept minutes » — sans la chiffrer.

**Premier calcul, avant toute exécution.** La grille la plus fine possible vaut `o1/2`. Si le
schéma est d'ordre 2, `e(o1/2) ≈ 4C/o1²` tandis que `écart(o1, 2o1) ≈ C/o1² − C/4o1² = 0,75 C/o1²`
— les deux quantités ont **la même origine**, l'erreur du schéma, et leur rapport vaut **16/3 ≈ 5,3
indépendamment de la taille de l'oracle**. Mesuré : 4,97 à o1 = 25600, 3,73 attendu à 51200,
3,00 à 89600 — la lente décroissance vient de l'exposant 1,596 de l'écart, pas de l'ordre.

Extrapolé, `ratio < 1` demande **o1 ≈ 1 355 000** et **72,3 h** de calcul. **L'expérience prescrite
est hors budget de trois ordres de grandeur**, et la grille 25600 nommée dans l'action n'atteint
que 3,73 — plus mal que le 4,97 que S48 avait déjà obtenu sans le voir.

À vérifier avant d'en tirer quoi que ce soit : ces chiffres sont extrapolés d'un calage sur S59.
La mesure `fine 25600` coûte environ 95 s et donne le point réel à `o1/2`.

Piste ouverte par le même calcul : si le rapport `e/écart` ne dépend que du **rapport
oracle/grille**, alors le filtre ×30 équivaut à une condition **géométrique**, connue d'avance et
sans aucune mesure — et les quatre campagnes de S48 à S57 auraient pu savoir leur résultat avant
de calculer. À vérifier contre les refus réellement observés, qui sont tous consignés.

P2/P3 : GEOMETRIE-DU-FILTRE-S61. Treize points de cinq campagnes, oracles de 3200 a 89600 :
ratio = 2,011 k^1,902 o^-0,058, ecart max 23,5 pour cent. La taille d oracle ne compte presque
pas. Identite sous-jacente : ratio = k^p / (1 - 2^-p). Le filtre x30 equivaut a k >= 5,6 a 6,0,
soit un oracle six fois plus fin que la grille la plus fine ; l historique 2, 4, 6, 7 s y range
sans exception. **S60-1 est dissoute** : ratio < 1 demande k = 1, l emboitement exige k >= 2.
Mon propre chiffrage de P1 (72 h) etait faux — exposant local 1,596 extrapole sur cinq decades,
alors que le global vaut 1,9 ; la conclusion se durcit, impossible et non couteux. A183 :
le seuil d admission d une mesure d ordre est fonction de l ordre. L uniformite du biais, elle,
est etablie sans regime extreme : colonne variation plate a 2 pour cent sur un facteur 256.

P4 : ADR-050. D1 le filtre x30 equivaut a k >= 6 environ, condition geometrique lisible avant
tout calcul ; D2 le seuil d admission d une mesure d ordre est fonction de l ordre — A183 ;
D3 filtre conserve, description completee sans reecriture. S60-1 dissoute. Note corrective datee
sur ADR-049 D4, dont le reste tient sans changement.

P5 : annonce d admissibilite affichee avant chaque campagne, et mode --annonce qui n en affiche
que la prevision sans rien calculer. A oracle 51200 elle prevoit la grille 12800 refusee avec un
ratio de 15,0 ; S56 avait mesure 14,81. Trois classes seulement — admise, refusee, et **a la
frontiere** entre k=5 et k=7, ou le modele avoue qu il ne tranche pas : a k=6 il annoncerait 31,6
contre 28,7 mesures, du mauvais cote du seuil. Test qui retrouve l historique 2, 4, 6, 7. Le
mode sec ne contourne aucun refus de taille. 129 tests reussis, deux ignores ; hashs inchanges.

Note d honnetete : deux campagnes ont ete lancees par distraction pendant cette etape, faute
d avoir ce mode sec — l une de 14 min interrompue, l autre de 6 min. Aucun resultat n en depend.
C est exactement le besoin auquel --annonce repond, rencontre en le construisant.
