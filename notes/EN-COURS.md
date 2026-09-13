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

Session : S226 — en cours
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
- [ ] **P2** — généraliser : `g_eff` vectoriel, points de référence et positions d'ouverture ; réduction au cas vertical vérifiée **au bit**.
- [ ] **P3** — éprouver la phrase d'ADR-010 : hublot latéral, sous gravité verticale puis sous accélération latérale ; part V de C16.
- [ ] **P4** — mesurer ce que la table de forme perd quand `g_eff` s'incline, et le nommer.
- [ ] **P5** — document de réception ; suite complète `code/`.
- [ ] **P6** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*
