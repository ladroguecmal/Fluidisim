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
Session          : S24
État             : en cours
Battement        : 2026-09-06
Objectif         : C08 — la convergence sous raffinement, et l'ordre du front
```

### Plan

`CAS-CANONIQUES` §C08 : *« Un solveur qui ne converge pas ne résout pas l'équation qu'on croit : il
est **faux**, pas imprécis. »* C'est le cas le plus sévère du corpus, et deux actions le réclament —
**S23-1** (l'exposant du front n'est pas stabilisé) et **S23-3** (C08 n'est pas écrit).

**Ce que S23 laisse et qui commande cette session.** L'ordre apparent du front vaut 0,13 · 0,27 ·
0,36 · 0,41 sur quatre raffinements successifs : **il monte encore**. Un ordre qui n'a pas convergé
n'est pas un ordre — c'est un chiffre qui dépend des grilles choisies. ADR-031 §2 s'appuie dessus en
le disant, et cette session doit soit le stabiliser, soit établir qu'il ne l'est pas et jusqu'où.

*Thèse déclarée avant l'exécution : `p` dépendra de la grandeur mesurée plus que du solveur.*
L'erreur L1 devrait donner `p ≈ 1`, le front bien moins. Si c'est le cas, l'énoncé de C08 a un trou :
il dit « un cas de C02, C04 ou C09 » et jamais **sur quelle grandeur de ce cas**. Or un cas en
produit plusieurs, et l'assertion `p > 0,8` n'a pas le même sens selon celle qu'on prend.

- [x] **P1** — plan, jeton.
- [x] **P2** — le mode `convergence` : Richardson à trois grilles, sur une grandeur quelconque,
      avec la référence exacte quand elle existe. Traiter honnêtement les deux cas dégénérés — une
      erreur au bruit d'arrondi (C01 équilibré) et un `p` calculé hors régime asymptotique.
- [x] **P3** — appliquer à C04 sur quatre grandeurs : erreur L1, `h(0)`, `u(0)`, front.
- [x] **P4** — exécuter, constater, et **balayer les triplets de grilles** : si `p` dépend du
      triplet, le rapporter comme tel plutôt que d'en publier un.
- [x] **P5** — *(remplacée en séance)* **le cas régulier avec oracle**. S23-2 (dériver le seuil ε) est reportée : la mesure ci-dessous a montré qu'elle n'était pas le point bloquant. ~~**S23-2**~~ : dériver le seuil `ε` du front au lieu de le conventionner. Piste : le
      bon seuil est celui pour lequel l'ordre observé du front rejoint celui de la norme globale —
      en dessous, la mesure est dominée par la queue du profil. À vérifier, pas à supposer.
- [x] **P6** — ADR-032 si la conclusion engage le protocole des bancs, note datée sinon.
- [x] **P7** — répercussions : `CAS-CANONIQUES` §C08, index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S23 laisse et qui vaut pour ici.**

- **La batterie `physics` sort en code 1** : C04 échoue par décision d'ADR-031. Ne pas « réparer ».
- **Deux pistes sont mortes** sur le retard du front — vitesses d'onde au lit sec (0,15 point),
  seuil de séchage (0,25 point sur six décades). Ne pas les refaire.
- **Le témoin `C01-jet` doit rester rouge**, et l'agrégation des témoins est par identifiant.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107). Vérifié à l'ouverture de S24 : rien
n'a bougé ailleurs.

#### P3-P4 — la thèse est fausse, et ce qui la remplace est plus embêtant

**Thèse déclarée : « `p` dépendra de la grandeur ». Réfutée.** Les trois grandeurs non locales
convergent au même rythme, et ce rythme n'est pas 1 :

| Grandeur | erreurs, nx = 200 → 3200 | ordres par triplet | dernier `p` |
|---|---|---|---|
| erreur L1 relative (globale) | 2,24e-2 → 2,86e-3 | +0,595 · +0,686 · +0,732 | **0,73** |
| `h(0)` (ponctuelle) | 3,10e-2 → 3,39e-3 | +0,698 · +0,747 · +0,787 | **0,79** |
| `u(0)` (ponctuelle) | 1,69e-1 → 1,81e-2 | +0,695 · +0,754 · +0,795 | **0,80** |
| **front, ε = 1 mm (locale)** | 2,654 → 1,130 m | **−0,504 · −0,059 · +0,237** | **0,24** |

Le front reste à part — il l'était déjà — mais **les trois autres sont groupées autour de 0,75, et
non de 1**. Un schéma d'ordre 1 sur une solution dont la dérivée est discontinue *ne converge pas à
l'ordre 1* : c'est un résultat classique, et personne dans le corpus ne l'avait écrit.

**Le vrai résultat : aucune des quatre n'est en régime asymptotique.** Les ordres montent
régulièrement — +0,595 → +0,686 → +0,732 — et le dernier n'est donc pas la limite. Sur cinq
grilles, de 200 à 3200 cellules, **C08 ne peut rien conclure**. L'énoncé, lui, en demande trois.

> **C08 n'est ni passé ni échoué sur ce montage : il n'est pas *exécutable*.** Et le rapport le dit
> en ces termes, avec un décompte — un rapport sans échec ne doit pas se lire comme une validation.

**Deux défauts de mon propre outil, trouvés par les données.**

1. **Le critère d'asymptoticité comparait les deux derniers ordres.** Il déclarait stabilisée la
   suite 0,595 → 0,686 → 0,732, dont les écarts sont petits **mais tous de même signe**. Un critère
   d'écart local ne distingue pas « a convergé » de « progresse lentement ». Remplacé par un critère
   **dérivé** : la progression est éteinte si les écarts changent de signe, ou décroissent d'un
   facteur ≥ 4 — auquel cas la somme des écarts restants est majorée par `|d₁|/3`.
2. **Le premier jet refusait les ordres négatifs** en les classant « indéterminé ». Or des
   différences successives qui *grandissent* sont exactement la signature du pré-asymptotique, et
   c'est ce que le front donne : −0,504 puis −0,059 puis +0,237. Les masquer retirait la seule
   chose que le contrôle devait constater.

**Conséquence pour ADR-031.** Sa prudence était justifiée, et plus qu'il n'y paraissait : l'exposant
du front n'est pas seulement non stabilisé, il est **négatif** sur les grilles grossières. Le chiffre
de ×3·10⁵ reste une extrapolation, et l'action S23-1 se conclut par un « non » : cinq grilles ne
suffisent pas.

#### P5 — l'ordre réduit vient de la solution, pas du schéma. Et l'oracle a un prix caché.

**L'étape déclarée était « dériver le seuil ε ». Elle a été remplacée en séance** par une mesure plus
urgente : P4 venait de montrer que *toutes* les grandeurs de C04 convergent à ~0,75, y compris la
norme globale. La question n'était donc plus le seuil du front, mais **si le solveur converge tout
court**. S23-2 est reportée, et la raison est écrite ici plutôt que perdue.

**Le montage.** Une bosse gaussienne de 1 cm sur 1 m d'eau, fond plat, domaine de 40 m, 1 s. Lisse
partout, régime linéaire (`a/h₀ = 1 %`), ni front ni séchage. Pas de solution analytique : la
référence est l'**oracle**, la grille la plus fine, comparée par **moyenne conservative** — chaque
cellule grossière contre la moyenne des `k` cellules fines qu'elle contient, exacte puisque les
grilles sont emboîtées.

**Résultat.** Ordres observés : **+0,819 · +0,978** avant contamination. Le solveur est bien d'ordre
≈ 1 sur une solution régulière.

> **La conclusion tombe : l'ordre réduit mesuré sur C04 — 0,73 à 0,80, et 0,24 au front — vient de
> la *solution*, pas du schéma.** Un schéma d'ordre 1 sur une solution à dérivée discontinue
> converge à un ordre réduit ; c'est un résultat classique, et c'est une propriété du **couple**
> (solveur, cas), pas du candidat. Aucun des trois cas que C08 désigne — C02, C04, C09 — n'est
> régulier.

**Le prix caché de l'oracle, et il est plus intéressant que le résultat.**

L'oracle n'est pas la solution : il porte sa propre erreur. Quand l'erreur d'une grille testée s'en
approche, les deux se soustraient et l'ordre observé **s'envole**. Mesuré, en affinant l'oracle :

```
oracle nx=12800  → ordres +0,889 · +0,945 · +1,087
oracle nx=25600  → ordres +0,890 · +1,058 · +1,559   ← 1,56 pour un schéma d'ordre 1
oracle nx=51200, filtre ×10  → +0,819 · +0,978 · +1,313
oracle nx=51200, filtre ×30  → +0,819                 ← trois grilles saines seulement
```

**Affiner l'oracle a rendu le résultat *pire*** — parce que les grilles fines, elles, ne bougeaient
pas. Le filtre écarte les grilles dont l'erreur vaut moins de trente fois l'erreur estimée de
l'oracle (seuil dérivé : à ×30, la contamination de l'ordre est majorée par 0,09).

> **Avec un oracle, le triplet le plus fin est le *moins* fiable** — alors qu'avec une solution
> analytique c'est le plus fiable. Ma fonction `ordre_final()` prend le triplet le plus fin : c'est
> juste pour C04, faux pour le cas régulier. **La règle dépend de la nature de la référence**, et
> rien dans SPEC-003 §5.1 ne le dit.

**Et les deux exigences se contredisent.** Pour conclure, C08 a besoin de grilles assez fines pour
être asymptotiques **et** assez grossières pour ne pas être contaminées. La fenêtre est étroite : il
faut `nx_oracle ≥ 30·nx_max` et `nx_max ≥ 16·nx_min` pour cinq grilles, soit un oracle à
**480 fois** la grille la plus grossière. **L'oracle est le poste dominant du banc**, et « l'oracle
lent » de SPEC-003 §5.1 ne dit pas à quel point.

**Deux défauts trouvés au passage.**

1. **L'arène du mode `physics` était dimensionnée à 1 Mo** et l'oracle à 51 200 cellules l'épuisait.
2. **L'échec d'allocation était absorbé par un `Err(_) => continue`.** Le rapport affichait
   « 0 grille retenue sur 0 » — un résultat vide qui a l'air d'un résultat. Corrigé : l'erreur
   remonte au `Sink` et le cas se déclare indisponible au lieu de se déclarer vide.
