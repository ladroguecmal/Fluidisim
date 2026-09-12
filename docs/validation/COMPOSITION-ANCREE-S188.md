# S188 — La composition, rejouée sur un réseau ancré

2026-09-12. S187-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[COMPOSITION-ERREURS-S186](COMPOSITION-ERREURS-S186.md) a mesuré comment l'erreur spatiale et
l'erreur temporelle se composent, et conclu que la loi est le **maximum** pour les deux modes
causaux. [RESEAU-GRADUE-S187](RESEAU-GRADUE-S187.md) a ensuite trouvé que le réseau sur lequel
cette mesure a été prise posait son dernier nœud **hors** du bloc, ce qui coûtait jusqu'à un
facteur six et **concentrait** toute l'erreur sur une tranche unique (**A231**,
[ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md)).

Cette session rejoue la grille de S186 sur un réseau **ancré**. Comme en S183–S187, les
conditions sont publiées **avant** la première exécution (§1–§6) ; les relevés viennent après
(§7).

## 1. Pourquoi rejouer, et ce qui serait insuffisant

Deux raisons, et il faut les séparer parce qu'elles n'exigent pas la même chose.

**La première est arithmétique et ne demanderait pas une session.** La règle de dimensionnement
de S186 §8.5 — égaliser les erreurs des deux axes pris seuls, puis s'arrêter — dépend des
**magnitudes**. L'axe spatial vient de perdre jusqu'à un facteur six ; le point de parité se
déplace donc, et l'exemple publié par S186 (« à `r = 2`, la cadence ne devient dominante qu'à
`c = 32` ») ne tient plus. On pourrait le recalculer de tête.

**La seconde est la vraie, et elle ne se recalcule pas.** La loi elle-même — `eU(r,c) ≈ max` —
a été mesurée sur une erreur **concentrée** : S186 §8.3 montre que l'erreur globale était
*exactement* celle de la tranche la plus haute, aux trois ratios. Un réseau ancré la
**répartit** : S187 §8.5 relève une erreur de tranche haute nulle et un maximum déplacé vers le
milieu du bloc.

Or une composition de deux erreurs en norme maximum dépend de **où** vivent leurs maxima. Si
les deux erreurs culminent au même endroit, elles s'y rencontrent et la plus grande gagne — ce
qui est exactement le comportement « maximum » observé. Si elles culminent en des endroits
différents, le maximum global peut être celui de l'une **sans interaction du tout**, ou leur
somme locale ailleurs. **Rien ne garantit que la loi survive au déplacement de l'un des deux
maxima**, et c'est ce que le rejeu mesure.

## 2. Ce qui change, et ce qui ne doit surtout pas changer

**Une seule variable change : où le dernier nœud se pose.** Le bloc, le pas, le montage d'eau,
la référence, les métriques, les trois modes de réemploi, les sept cadences, les trois lois et
leur critère de jugement sont ceux de S186, à l'identique. Si l'on changeait aussi la
graduation, on ne saurait pas à quoi attribuer l'écart.

La famille ancrée porte **exactement les nombres de nœuds** de la famille isotrope de S186 :

| famille | S186 — débordante | S188 — ancrée | nœuds |
|---|---|---|---:|
| plein | `axis_indices(r=1)` = 1…14 | 14 nœuds par axe, 1…14 | 2744 |
| `r = 2` | 1, 3, 5, …, 15 | 8 nœuds par axe, 1…14 | 512 |
| `r = 4` | 1, 5, 9, 13, 17 | 5 nœuds par axe, 1…14 | 125 |
| `r = 8` | 1, 9, 17 | 3 nœuds par axe, 1…14 | 27 |

Les indices ancrés sont ceux que S187 a employés comme témoin « ancré uniforme » : ils
couvrent `[1, 14]` avec le nombre de nœuds demandé, arrondi au plus proche. Ils sont sortis
dans `support/` pour que S187 et S188 partagent **le même** réseau — deux copies
divergeraient, et la comparaison entre les deux sessions ne vaudrait plus rien (L137). S187
est rejoué et son empreinte vérifiée.

**Le nom « r » est conservé** pour désigner les lignes, parce que le rejeu doit se lire ligne à
ligne contre S186 ; mais il ne désigne plus un pas uniforme, seulement un nombre de nœuds par
axe. Le tableau ci-dessus est la traduction, et elle est rappelée dans chaque table.

**L'ancrage est appliqué aux trois axes.** S187 a mesuré que l'ancrage ne rapporte presque rien
horizontalement (−2,5 %, −40 %, **+1,3 %** selon le nombre de nœuds, non monotone). L'appliquer
quand même est le choix d'ADR-118 : la règle est uniforme parce qu'une règle par axe demanderait
de savoir d'avance où vit le maximum, et c'est précisément ce qu'un consommateur ne sait pas.
La conséquence est assumée : une ligne peut être très légèrement moins bonne que sur le réseau
débordant, et si cela se produit il faudra le dire.

## 3. Métriques — inchangées

Identiques à S186 §4, mot pour mot, parce qu'un rejeu qui change sa métrique ne rejoue rien :

- état initial `u' = 0` ;
- **erreur de source** `eS = max |S_utilisée − S_réf|` sur toutes les mailles et tous les pas,
  rapportée à `max |S_réf|` ;
- **erreur de champ** `eU = max |u'(T) − u'_réf(T)|` sur les mailles intérieures, rapportée à
  `max |u'_réf(T)|`.

S'y ajoute, et c'est le seul ajout, **la localisation du maximum** : pour chaque case, l'indice
de la tranche qui porte l'écart maximal. C'est la grandeur qui décide de §1, et elle n'existait
dans aucune des deux sessions précédentes — S186 publiait l'erreur par tranche, jamais où le
maximum se trouvait dans le cas composé.

## 4. Les trois lois, et le même critère

Les trois lois de S186 §5 — additive, quadratique, maximum — et **le même critère déjà
déclaré** : une loi est retenue si son rapport mesuré/prédit reste dans `[0,80 ; 1,25]` sur
toutes les cases au-dessus du plancher de 0,386 %. Le critère n'est pas rediscuté ; il est
repris tel quel, ce qui est la condition pour que les deux verdicts se comparent.

Le verdict est publié **globalement puis par mode**, comme S186, parce que c'est le découpage
par mode qui avait rendu son relevé lisible.

Trois issues possibles, déclarées avant la mesure :

- **la loi tient** — le maximum pour les modes causaux, la quadratique pour l'interpolation.
  Alors S186 §8.5 garde sa conclusion, seules ses magnitudes changent, et un budget conjoint
  reste licite ;
- **la loi change** — une autre loi est retenue, ou aucune. Alors la conclusion de S186 était
  une propriété du réseau et non de la composition, et cela doit être écrit dans les deux
  documents ;
- **la loi tient mais pour une autre raison** — par exemple parce que les deux maxima
  continuent de coïncider malgré l'ancrage. C'est l'issue la plus instructive, et c'est la
  colonne « tranche du maximum » qui permettra de la distinguer de la première.

## 5. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** Deux exécutions, mêmes bits ; une empreinte est publiée. Aucune
   durée n'est mesurée.
2. **La ligne pleine est la référence, littéralement.** À 14 nœuds par axe, l'ancré et le
   débordant **coïncident** — `axis_indices(r = 1)` donne déjà 1…14. La ligne pleine doit donc
   rendre le champ de référence **bit pour bit**, et à `c = 1` les trois modes doivent le rendre
   aussi. Si cette réception échoue, le réseau ancré n'est pas ce que l'on croit.
3. **Les erreurs temporelles pures sont celles de S186.** À réseau plein, les sept `eU` de
   chaque mode ne dépendent pas du réseau : elles doivent redonner **exactement** les valeurs
   de S186 §6.2, donc aussi celles de S185. C'est le contrôle croisé qui autorise à comparer
   les deux grilles.
4. **Les erreurs spatiales pures sont celles de S187.** À `c = 1`, la colonne doit redonner les
   valeurs ancrées de S187 §8.4 — notamment 1,6947 % à huit nœuds par axe pour un réseau
   horizontal de pas 2. Les lignes 5 et 3 nœuds n'ont pas d'équivalent publié à ancrage complet
   sur les trois axes ; elles sont nouvelles et signalées comme telles.
5. **Le support historique est intact.** `cadence_error` (`0x39567a1d4bc2ba4c`), `composed_error`
   (`0x0e743846d4656870`) et `graded_lattice` (`0x6cf13183b4a240df`) rendent leurs empreintes
   publiées après le déplacement des indices ancrés dans `support/`. Vérifié par exécution des
   trois.
6. **Tout reste fini**, source et champ, sur les 84 cases.

## 6. Ce que le rejeu ne prouvera pas

- **Aucun seuil de justesse.** A50 attend une décision ; ce rejeu ne la prend pas.
- **Un seul montage, un seul couple d'échelles, une seule profondeur de bloc.** Comme S186 et
  S187.
- **Rien sur le réseau gradué.** Le rejeu n'utilise que l'ancrage uniforme, pour isoler une
  variable. La composition sur un réseau **gradué** — dont S187 montre qu'il répartit l'erreur
  bien davantage — reste non mesurée, et c'est le candidat naturel pour la suite si la loi
  s'avère dépendre de la localisation.
- **Aucun coût.** Les nombres de nœuds sont identiques à ceux de S186 par construction, donc le
  coût de S184 s'applique sans changement. Rien n'est remesuré.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur.

## 7. Relevés

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example anchored_composition
```

Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.
Bloc 16³, 2744 mailles intérieures, `dx = 0,25 m`, `dt = 10 ms`, 100 pas, mailles intérieures
à `z ∈ [−4,05 ; −0,80] m`. `max |S| = 1,540547e-4 m/s²`,
`max |u'(T)| = 7,993168e-5 m/s` — les valeurs de S186 et S187.

Les réseaux ancrés obtenus, et il vaut la peine de les regarder : 14 nœuds → `1…14` ;
8 → `[1, 3, 5, 7, 8, 10, 12, 14]` ; 5 → `[1, 4, 8, 11, 14]` ; 3 → `[1, 8, 14]`. L'accrochage
aux centres de mailles produit un pas **irrégulier** — 7 puis 8 puis 10 — parce qu'un réseau
de huit nœuds ne divise pas treize intervalles en parts égales. Ce n'est pas ce qu'on poserait
à la main, et c'est visible plutôt que lissé.

### 7.1 Réceptions — les six passent

1. **Reproductibilité.** Empreinte `0x21bab548c7b9775c`, `diff` identique sur deux exécutions.
   Aucune durée n'est mesurée : la sortie entière est un résultat.
2. **La ligne pleine est la référence, en bits**, pour les trois modes. L'ancré à quatorze
   nœuds *est* `axis_indices(r = 1)` : la coïncidence attendue est vérifiée, pas supposée.
3. **Les erreurs temporelles pures sont celles de S186**, aux vingt-et-une cases : maintien
   `c = 2` 0,7700 % et `c = 64` 33,2115 % ; extrapolation `c = 8` 0,7754 % ; interpolation
   `c = 64` 6,7740 %. Donc aussi celles de S185. C'est le contrôle croisé qui autorise à
   comparer les deux grilles.
4. **L'erreur spatiale à huit nœuds par axe vaut 1,7160 %.** *Correction de protocole, à
   déclarer et non à taire : §5 annonçait 1,6947 %, et c'était la mauvaise valeur de S187.*
   1,6947 % est la ligne « ancrée » du premier tableau de S187 §8.4, où seul l'axe **vertical**
   était ancré et l'horizontal restait débordant ; 1,7160 % est la ligne à huit nœuds du
   **second** tableau, celui de l'ancrage horizontal. C'est cette dernière que S188 doit
   redonner, puisqu'il ancre les **trois** axes — et il la redonne exactement.

   Et le fait que ce soit exactement elle dit quelque chose : avec l'axe vertical ancré, la
   contribution verticale **disparaît entièrement** de la norme maximum, et il ne reste que
   l'erreur horizontale. Les lignes à cinq et trois nœuds sont nouvelles — S187 n'avait pas
   mesuré l'ancrage simultané des trois axes à ces densités — et elles se lisent de la même
   façon : 3,6805 % et 13,1488 % sont, au chiffre près, les valeurs horizontales ancrées de
   S187 §8.4.
5. **Plancher** de la référence à `dt/2` : **0,386 %**, celui de S186, puisque c'est la même
   référence.
6. **Tout est fini**, source et champ, sur les 84 cases.

Et le support historique est intact : après le déplacement des indices ancrés dans `support/`,
`cadence_error` rend `0x39567a1d4bc2ba4c`, `composed_error` rend `0x0e743846d4656870` avec une
sortie entière identique au `diff`, et `graded_lattice` rend `0x6cf13183b4a240df`.

### 7.2 L'axe spatial : les magnitudes bougent beaucoup

À nombre de nœuds **identique**, `c = 1` :

| ligne | nœuds/axe | nœuds | eS % | eU ancré % | eU débordant (S186) % | facteur |
|---|---:|---:|---:|---:|---:|---:|
| `r = 1` | 14 | 2744 | 0 | 0 | 0 | — |
| `r = 2` | 8 | 512 | 2,0782 | **1,7160** | 2,5401 | 1,48 |
| `r = 4` | 5 | 125 | 3,7502 | **3,6805** | 13,6043 | **3,70** |
| `r = 8` | 3 | 27 | 13,8404 | **13,1488** | 32,9593 | 2,51 |

Une conversion **mesurée**, sans interpolation : **27 nœuds ancrés (13,1488 %) valent 125
nœuds débordants (13,6043 %)** — la même erreur pour **4,6 fois moins de nœuds**. C'est le
gain d'ADR-118 exprimé dans la monnaie de S184, où le coût suit exactement le nombre de nœuds.

**Et le point de parité se déplace.** La règle de dimensionnement de S186 §8.5 — égaliser les
erreurs des deux axes pris seuls, puis s'arrêter — tient, mais son point d'application change.
À 125 nœuds, l'erreur spatiale débordante de 13,60 % égalait le maintien vers `c ≈ 20` ;
ancrée à 3,68 %, elle l'égale vers `c ≈ 6`. **Un facteur ~3 sur la cadence admissible**, et la
conséquence de conception est directe : sur un réseau ancré, l'optimum se déplace vers **plus**
de décimation spatiale et **moins** de réduction de cadence. L'exemple publié par S186 — « à
`r = 2` la cadence ne devient dominante qu'à `c = 32` » — ne tient donc plus ; il devient
`c ≈ 8` à 512 nœuds ancrés.

### 7.3 Le verdict : la loi ne change pas, mode par mode

Verdict global, cases jugées, avec le critère **déjà déclaré** en S186 §5 et repris tel quel :

| loi | S188 (ancré) | S186 (débordant) | verdict |
|---|---|---|---|
| additive | 0,468 – 0,984 | 0,529 – 0,988 | **rejetée** dans les deux |
| quadratique | 0,659 – 1,245 | 0,749 – 1,209 | **rejetée** dans les deux |
| maximum | 0,869 – 1,707 | 0,826 – 1,489 | **rejetée** dans les deux |

Par mode — le découpage qui avait rendu S186 lisible :

| mode | additive | quadratique | maximum | retenue S188 | retenue S186 |
|---|---|---|---|---|---|
| maintien | 0,500–0,951 | 0,702–0,999 | **0,895–1,060** | **maximum** | maximum |
| extrapolation | 0,468–0,941 | 0,659–0,998 | **0,869–1,000** | **maximum** | maximum |
| interpolation | **0,845–0,984** | **1,017–1,245** | 1,018–1,707 | **additive, quadratique** | additive, quadratique |

**Le verdict est identique, mode par mode.** Et il est **mieux satisfait** : la plage du
maximum se resserre de 0,826–1,155 à **0,895–1,060** pour le maintien, et de 0,860–1,034 à
**0,869–1,000** pour l'extrapolation. L'erreur concentrée du réseau débordant rendait donc la
composition **plus bruyante**, pas plus propre — c'est l'inverse de ce qu'on pourrait craindre
en corrigeant un montage après coup.

Aux cadences hautes la loi est **exacte** : à `c = 64`, le maintien rend 33,2115 % et
l'extrapolation 27,3202 % **aux trois lignes de réseau** — c'est-à-dire l'erreur temporelle
pure, rapport 1,000. L'axe dominant emporte tout, littéralement.

### 7.4 Pourquoi elle tient : les deux maxima n'ont pas bougé l'un par rapport à l'autre

C'est la métrique ajoutée en §3 qui tranche, et elle tranche sans ambiguïté.

> **La tranche qui porte le maximum est la 14 — la plus haute — dans tous les cas :** axe
> spatial seul aux trois réseaux, axe temporel seul aux vingt-et-une cadences, et les 84 cases
> composées. Le compte publié par le programme : **39 cases jugées sur 39 où les deux maxima
> vivent sur la même tranche.**

L'ancrage a changé la **magnitude** de l'erreur spatiale — jusqu'à 3,7 fois — mais pas
**l'endroit** de son maximum. La raison est que cet endroit n'est pas une propriété du réseau :
`|S|` culmine en haut du bloc parce qu'un mode profond décroît en `exp(k z)`, donc `|u'(T)|`
culmine en haut, donc tout écart relatif à la référence y culmine aussi. Aucun réseau n'y
change rien.

C'est la **première** des trois issues déclarées en §4 — *la loi tient* — et la colonne de
tranche montre qu'elle tient pour **la même** raison qu'en S186, pas par accident. Ce qui
transforme le résultat de S186 d'une observation en une **condition** :

> **La loi du maximum vaut tant que les maxima des deux erreurs coïncident.** Ils coïncident
> ici parce que l'amplitude du champ est maximale sur une frontière du domaine, et que les deux
> erreurs sont relatives à ce champ. Un contenu dont la source culminerait au **milieu** du
> domaine, ou dont l'erreur temporelle culminerait ailleurs que l'erreur spatiale, n'est pas
> couvert — et rien dans le corpus ne le disait (**A232**).

## 8. Ce que la session conclut, et ce qu'elle laisse ouvert

**Conclu.**

1. **La loi de composition de S186 survit à l'ancrage**, mode par mode, sans changement :
   maximum pour les deux modes causaux, additive et quadratique pour l'interpolation. Un
   budget conjoint reste licite.
2. **Elle est mieux satisfaite qu'en S186** — 0,895–1,060 contre 0,826–1,155 pour le maintien.
   Le réseau mal placé ajoutait de la dispersion à la loi.
3. **Elle tient parce que les deux maxima coïncident**, et ils coïncident sur la tranche haute
   dans 39 cases jugées sur 39. C'est désormais une **condition écrite**, pas une coïncidence
   tacite.
4. **Les magnitudes se déplacent jusqu'à 3,7 fois**, donc le point de parité aussi : `c ≈ 20`
   devient `c ≈ 6` à 125 nœuds. L'optimum va vers plus de décimation spatiale et moins de
   réduction de cadence.
5. **27 nœuds ancrés valent 125 nœuds débordants** à erreur égale — mesuré, sans interpolation.
6. **La limite déclarée par ADR-118 est levée** : sa réception disait que la loi de S186
   n'était pas rejouée sur un réseau ancré. Elle l'est.

**Non conclu, et pas contourné.**

- **Aucun seuil de justesse.** A50 attend une décision ; ce rejeu ne la prend pas, et `N` de
  SPEC-004 §6.2 reste le seul des trois paramètres de B4 que personne n'a fixé.
- **Rien sur le réseau gradué.** Le rejeu n'a bougé qu'une variable, volontairement. Or c'est
  le réseau **gradué** qui répartit vraiment l'erreur — S187 §8.5 y relevait une tranche haute
  à **zéro** — et c'est donc la seule configuration connue où les deux maxima pourraient cesser
  de coïncider. La loi y reste à éprouver.
- **Un seul montage, une seule profondeur de bloc.** La coïncidence des maxima est une
  propriété de ce contenu ; §7.4 dit ce qu'il faudrait pour la rompre, la session ne le
  construit pas.
- **Aucun coût remesuré.** Les nombres de nœuds sont ceux de S186 par construction, donc S184
  s'applique sans changement. Le surcoût par maille d'un réseau à poids irréguliers n'est
  toujours pas chiffré.
- **Le véhicule ne projette pas** et l'advection y reste d'ordre supérieur.

**Suite recommandée — S189 : S188-1.** La composition sur réseau **gradué**. C'est le seul
endroit où la condition de §7.4 peut être mise à l'épreuve plutôt que constatée : la
graduation déplace le maximum de l'erreur spatiale vers le milieu du bloc, tandis que l'erreur
temporelle reste accrochée au maximum du champ, en haut. Si la loi du maximum survit à cette
séparation, elle est robuste ; si elle tombe, **A232** est confirmée et la règle de
dimensionnement d'ADR-118 devra dire sur quel réseau elle s'applique. Les deux issues
instruisent, et c'est ce qui fait de cette mesure la bonne suivante.
