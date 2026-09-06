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
Session          : S19
État             : en cours
Battement        : 2026-09-05
Objectif         : trois réponses de l'utilisateur, dont une qui change la nature du projet
```

### Plan

Trois informations reçues, d'importance très inégale :

1. **ADR-020 est acté.** Le premier ADR à quitter le statut « proposée ». Le blocage n°1 du dossier
   de réunion tombe, et avec lui l'obstacle à l'écriture du harnais.
2. **Il n'y a pas d'autres équipes.** Je suis seul à travailler sur le projet ; les développeurs
   observent. **Les onze destinataires extérieurs n'existent pas.**
3. **« Je ne sais pas »** sur `int64`/`f64` et sur l'état réel du projet.

Le second point est de loin le plus lourd, et il n'est pas de la logistique : il invalide la
catégorie « attend une réponse d'une autre équipe », qui structure le corpus depuis S02.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — `ADR-028` §1–2 : ADR-020 acté · **ce que « il n'y a pas d'autres équipes » change**.
  *Thèse : quatorze « demandes extérieures » ne sont pas des demandes. Ce sont des **décisions
  différées à personne**. C'est L61 à l'échelle du corpus — l'étiquette « attend un tiers » a protégé
  quatorze questions de l'examen qui les aurait tranchées.*
- [ ] **P3** — `ADR-028` §3 : **les positions monde**, tranchées faute d'interlocuteur.
  *Thèse : le choix se dérive au lieu de se choisir. La résolution du point fixe se déduit de l'ulp
  d'un `f32` au rayon de référentiel — 4096 m — et le déterminisme devient structurel au lieu d'être
  disciplinaire.*
- [ ] **P4** — `ADR-028` §4 : **la propriété du harnais, révisée**. ADR-027 §6 confiait les seuils à
  une assurance qualité qui n'existe pas.
  *Thèse : avec un acteur unique, le conflit d'intérêt ne se supprime pas, il se contraint dans le
  temps — les seuils s'écrivent **avant** la mesure, dans un commit qui la précède. C'est l'écriture
  anticipée de S07 appliquée à la mesure.*
- [ ] **P5** — `ADR-028` §5–6 : le dossier de réunion requalifié · ce qui reste ouvert.
- [ ] **P6** — répercussions : `REPRISE.md` §1 et §5, `CLAUDE.md`, `00_INDEX.md`,
  `DOSSIER-REUNIONS.md`, ADR-020, ADR-002 §7.1, SPEC-003 §11.4.
- [ ] **P7** — angles morts, décomptes.
- [ ] **P8** — rituel de fin.

### Notes de reprise

- **Ce que « il n'y a pas d'autres équipes » ne change pas** : les contraintes restent vraies. Le
  géoïde décale toujours une plage de 70,7 m à 30 km, que l'équipe terrain existe ou non. Ce qui
  change est **qui répond** — et la réponse est : moi, sous la délégation déjà donnée en S18.
- **Ce que cela change pour la suite** : le blocage n'est plus « obtenir des réponses ». ADR-020
  étant acté, **H1 est écrivable**. C'est le premier élément du chemin critique, et le seul moyen de
  convertir dix-huit sessions de conception en quelque chose qui s'exécute et se vérifie.
  Une réserve à lever avant : `CLAUDE.md` pose « **Markdown uniquement** ». Écrire H1 ajoute du code
  à un dépôt qui n'en contient pas — c'est un changement de nature, et il se demande.
