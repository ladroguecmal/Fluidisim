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
