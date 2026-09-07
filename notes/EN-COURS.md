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
Session          : S63
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S59-1 — recenser les prescriptions non éprouvées, celles qu'une session a
                   écrites pour une situation qu'elle ne subissait pas encore.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [ ] **P2** — séparer les genres : une condition de réversibilité n'est pas une recette, et seules les recettes se vérifient.
- [ ] **P3** — recenser les recettes du corpus et établir, pour chacune, si elle a été exécutée et ce qu'elle a donné.
- [ ] **P4** — éprouver celles qui sont vérifiables à bas coût, en commençant par les plus engageantes.
- [ ] **P5** — rapport, marquage des prescriptions non éprouvées, et la règle d'écriture qui en découle.
- [ ] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, jeton, **fusion dans master**.

### Notes de reprise

Départ 5aac200. **Deux instances connues, et elles sont toutes deux fautives.**
**A181** (S59) : `REFERENCE-C22-S56` §5 prescrivait un découpage temporel au-delà du quart
d'heure — il changeait le champ bit à bit et aurait invalidé la campagne. **ADR-049 D4** (S60) :
l'expérience nommée en S60-1 était impossible, pas coûteuse ; S61 l'a dissoute.

**La thèse à éprouver, et elle est inconfortable** : *les deux seules prescriptions du corpus
qui aient été mises à l'épreuve se sont révélées fautives*. Si c'est exact, le taux n'est pas
anecdotique — il dit que la prescription non exécutée est un genre de texte dont la fiabilité
n'a jamais été établie, et le corpus en contient beaucoup.

**Premier tri, fait à l'ouverture.** Le grep sépare trois genres, et un seul est en cause :

1. **Condition de réversibilité** — *« si la réponse était l'inverse, il faudrait rouvrir X »*.
   ADR-027 en a cinq, ADR-048 une. **Ce n'est pas une recette** : rien à exécuter, rien à
   éprouver, et c'est un dispositif voulu. Hors sujet.
2. **Anticipation de conception** — *« prévoir un raffinement côtier »*, *« probablement 2 à 4 m »*.
   Non éprouvées par nature, mais elles ne se donnent pas pour vérifiées.
3. **Recette procédurale** — *« si X, faire Y »*, avec une action technique. **C'est là que les
   deux fautes se logent**, et c'est ce qu'il faut recenser.

**Où chercher.** Les rapports de mesure finissent tous par une section « suite ». Et surtout
`DOSSIER-B2` et `PLAN-BENCHMARK` sont **entièrement** faits de recettes écrites pour un travail
qui n'a jamais eu lieu — c'est le gisement principal, et personne ne l'a confronté à l'exécution
parce que l'exécution n'a pas commencé.
