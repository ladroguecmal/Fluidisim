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
Session          : S58
État             : terminée
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : Trancher A103 — la masse volumique de l'eau — sur délégation explicite de
                   l'utilisateur, en mesurant d'abord ce que la constante commande réellement.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — recenser tous les emplois de la masse volumique dans le code et dans le corpus ; établir ce qui en dépend et ce qui n'en dépend pas.
- [x] **P3** — balayer la constante et mesurer, pour chaque grandeur publiée, l'écart ET la valeur absolue ; figer le constat par un test.
- [x] **P4** — ADR-048 : trancher, dire ce qu'il faudrait pour inverser, et corriger la justification fautive de body.rs sans réécrire d'ADR.
- [x] **P5** — appliquer la décision au code ; vérifier les 123 tests, la campagne physics et les deux hashs.
- [x] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, passation et jeton.

### Notes de reprise

Délégation : l'utilisateur a demandé en conversation, le 2026-09-07, que les points laissés
« pour lui » soient réalisés par la session. Le premier — fusionner S57 dans master — est fait
(avance rapide vers bd9f087). Le second est A103, ouvert depuis S21 et rappelé en fin de
chaque session depuis. Précédent de forme : ADR-027, cinq arbitrages tranchés sur délégation.

**Thèse à mesurer, pas à supposer.** `body.rs` justifie `RHO_EAU = 1000` par « la valeur avec
laquelle la référence de C10 se referme ». Or les trois références de C10 sont écrites *en
fonction de* `RHO_EAU` (physics.rs : `(cube.rho / RHO_EAU) * cote`, `RHO_EAU * G * aire`,
`TAU * (rho * cote / (RHO_EAU * G)).sqrt()`). Si c'est exact, **C10 se referme pour toute
valeur** et ne contraint rien : la justification serait une instance d'A104, jamais reliée à
A103. À vérifier par balayage avant d'en tirer quoi que ce soit (L75).

Attendu à contrôler aussi : ρ apparaît-il dans l'hydrodynamique ? Saint-Venant s'écrit en
h et u, ρ s'y simplifie ; si c'est le cas ici, la portée de la décision est bornée aux forces
sur les corps. Ne pas le supposer non plus.

P2/P3 : recensement et balayage faits, RHO-EAU-S58. RHO_EAU vit à cinq endroits, aucun dans
un solveur : la portée est bornée aux forces sur les corps. Balayage 1000 vers 1025 par
recompilation : les quatre cas C10 passent identiquement, écart 0,000 pour cent aux deux
valeurs, parce que les trois références sont construites AVEC la constante. Les valeurs
publiées bougent (tirant -2,439, raideur +2,500, période -1,227 pour cent) sans verdict.
Hashs check inchangés. Les seuls contrôles qui échouent sont deux tests unitaires de body.rs
comparant le tirant au littéral 0,25 : A180. Constante restaurée à 1000 avant commit.

P4 : ADR-048 écrit. D1 valeur 1025 (mer ouverte, ADR-001) ; D2 la constante devient une
propriété du milieu, type Milieu avec MER et EAU_DOUCE, motif estuaire ; D3 la référence C10
reste construite avec la constante — pas de littéral inventé — et sa cécité est écrite (A180).
Section 4 : ce qu il faudrait pour inverser chacune des trois. Notes correctives datées dans
CAS-CANONIQUES et 00_INDEX, sans réécriture. Reste P5 : appliquer au code.

P5 : Milieu introduit dans water-core (MER 1025 par defaut, EAU_DOUCE 1000) ; RHO_EAU retiree
de l export. force_verticale, equilibre et tirant prennent le milieu. physics.rs C10 utilise
Milieu::default(). Tests de body.rs transposes : le litteral devient 0,243902439, avec mention
qu il decoule d ADR-048 D1 et non d une mesure ; deux tests ajoutes (sensibilite reelle au
milieu, et un corps a 1010 qui coule en eau douce mais flotte en mer). 125 tests reussis
(40 coeur + 85 harnais), deux ignores. Hashs check inchanges. Campagne physics identique :
seul C04 ordre un echoue comme voulu, C08 sans verdict. C10 vert aux nouvelles valeurs.

P6 : rituel exécuté. Journal S58, A180 (sévérité 2, ouvert par décision), L176, A103 close
dans le tableau et par note corrective datée, actions S58-1 et S58-2 (S58-2 avant S58-1).
Index et REPRISE : ADR portés de 47 à 48, angles de 179 à 180, tests de 123 à 125 ;
A103 retirée des rappels de fin de session. Aucun invariant ne cite la masse volumique.
Jeton libre. **Branche S58 non fusionnée dans master.**
