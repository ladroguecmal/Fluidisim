# ADR-138 — Le budget de pente tient compte de la position relative des impacts

- Statut : **actée**, S223, 2026-09-13 ; autonomie technique S71.
- Traite **A262** (S222). Prolonge [ADR-094](ADR-094-ce-que-le-champ-radial-peut-annoncer-de-sa-pente.md)
  et [ADR-133](ADR-133-le-majorant-de-pente-suit-la-dispersion.md) sans les remplacer :
  `slope_max()` et `slope_max_at()` restent publiées et inchangées **au bit**.
- Même famille qu'[ADR-134](ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) : une
  **inégalité**, pas une table. Elle en diffère sur un point, dit ci-dessous.
- Ne change ni `Steepness`, ni `steepness` publiée, ni un bit de hauteur, de vitesse ou de pente.
- Mesures : [COURONNE-IMPACT-S223](../validation/COURONNE-IMPACT-S223.md).

## Constat

`mixed_water::slope_floor` additionnait le majorant **global** de chaque champ d'impact. Deux
impacts frais à cent mètres l'un de l'autre consommaient donc le même budget que deux impacts
confondus, alors qu'aucun point du domaine ne voit les deux maxima. S222 a mesuré que c'était
devenu **le goulot** : un impact neuf vaut 47,4 % de π/7, deux saturent, et **trois impacts frais
séparés de cinquante mètres dépassent π/7 — refusés** — quand le maximum réel de leur composition
vaut 0,2125, soit 47 % de la limite.

ADR-133 a resserré chaque majorant **dans le temps** ; il n'a jamais touché à leur somme **dans
l'espace**. Et le resserrement temporel est nul à la naissance (ρ = 1, majorant exactement atteint),
c'est-à-dire exactement là où le budget sature.

## Ce que la lecture du champ a donné, et ce qu'elle a corrigé

La pente radiale vaut `Σ_n c_n k_n J₁(k_n r) cos(ω_n t)`, et le majorant publié était construit par
une ligne : `slope += coefficient * k;` avec, en commentaire, « `|J1| <= 1`, borne conservative ».

**Le facteur manquant était le pic de `J₁`**, 0,581865 — et `SLOPE_L1_RATIO = 1,795071`, mesuré en
S141 pour convertir la borne L1 en pente réelle, vaut `1/0,5819 × 1,045`. La constante mesurée du
dépôt retrouvait ce pic sans le nommer.

**Et l'inégalité classique qu'on serait tenté d'employer est fausse ici.** `|J_ν(x)| ≤ √(2/πx)` vaut
pour `ν = 1/2`, où elle est une **égalité** ; pour `ν = 1` elle est **dépassée de 3,4 %** en
`x = 2,166`. L'asymptote est la limite en `+∞`, pas un majorant. Mesuré sur `bessel` exécutée,
4 millions d'échantillons par régime : 3,4 % de dépassement sur la table de Hermite, 44 ppm sur le
développement asymptotique (termes correctifs d'A&S 9.2.1).

## Décision

**1. `RadialImpact::slope_max_beyond(time, radius)`** — majorant de la pente réelle **sur la
couronne `r ≥ radius`** :

```text
|dη/dr| ≤ (1 + garde) · Σ_n |c_n| k_n · min( J1_PEAK_BOUND , J1_DECAY_BOUND / √(k_n r) )
```

Constantes : `J1_PEAK_BOUND = 0,5818650` (maximum de `J₁`), `J1_DECAY_BOUND = 0,8250310`
(`sup_x |J₁(x)|·√x` relevé sur `bessel` **exécutée**), garde `1e-4` — 275 fois le plus grand
dépassement mesuré du palier (0,36 ppm), et couvrant l'égalité par construction sur la branche en
`1/√x`. La valeur rendue est le **minimum** de cette borne et de `slope_max_at(time)` : deux
majorants du même champ, l'un calibré et reçu, l'autre une inégalité à constante mesurée ; leur
minimum choisit le meilleur sans mélanger preuve et calibration.

**2. `mixed_water::slope_floor_joint`** — plancher conscient de la position relative. Soit `c₁` le
centre du champ d'ancrage (le plus grand majorant global, index le plus petit à égalité) et
`r₁ = |p − c₁|`. L'inégalité triangulaire donne `r_i ≥ |d_i − r₁|`, et `slope_max_beyond` décroît,
donc pour **tout** point `p` :

```text
Σ_i F_i(r_i)  ≤  F₁(r₁) + Σ_{i≠1} F_i(|d_i − r₁|).
```

**Le balayage est sûr entre ses échantillons, pas seulement dessus** : sur une cellule `[a, b]` de
`r₁`, `F₁` est majorée par `F₁(a)` — elle décroît — et `F_i(|d_i − r₁|)` par `F_i(δ)`, avec `δ` la
plus petite distance atteignable sur la cellule, **nulle si `d_i ∈ [a, b]`**. Aucune constante de
Lipschitz n'est nécessaire : les deux termes sont monotones du bon côté.

**3. L'annonce et le refus lisent la même quantité.** `slope_floor` passe par cette inégalité dès
**deux** champs, et `sample_world_batch` calcule son budget **une fois**, hors de la boucle des
points, par `slope_floor`. Sans cela la garantie d'ADR-128 dans les deux sens — `max_slope ≥ floor
⟹ aucun refus` — cesserait de tenir. Le résultat est pris en **minimum** avec la somme d'origine :
ce chemin n'est jamais plus lâche.

**4. Huit intervalles, et c'est une mesure.** `JOINT_SLOPE_SAMPLES = 8` rend **96,5 %** du gain de
soixante-quatre pour **11 %** de son coût : 13,3 µs contre 121,3 µs à deux champs, soit 0,7 % du
budget d'image de 2 ms. Le balayage est linéaire en intervalles, la qualité ne l'est pas.
Provenance (I-14) : `examples/couronne_impact_s223.rs`.

**5. `steepness` publiée ne bouge pas.** La boucle de la requête continue de sommer `slope_max()`
pour l'enveloppe publiée ; seul le **budget de refus** emprunte l'inégalité. Aucun bit publié ne
change, comme ADR-134 l'avait tenu.

## Ce que cette décision obtient, et ce qu'elle ne fait pas

**Obtient.** À la naissance — là où ADR-133 ne donne rien — le gain vaut **1,76** pour deux impacts
séparés de 50 m et **2,37** pour trois. En admission : trois impacts frais à 50 m passent de
**142,1 % de π/7, refusés**, à **60,0 %, admis**. À séparation nulle le gain vaut **exactement
1,0000** : la borne ne gagne que là où la géométrie le permet, et c'est ce qui la rend crédible.

**Ne fait pas.**

- **Elle n'est pas une inégalité pure**, et c'est sa différence avec ADR-134 : sa constante de
  décroissance est **mesurée** sur la fonction que le programme exécute, parce que l'inégalité
  mathématique disponible est fausse au rang qui nous intéresse. Elle a donc un **domaine** — celui
  de `bessel`, `[0 ; 2048]` — et un banc qui la fixe. Statut d'ADR-133, pas d'ADR-134.
- **Elle ne touche pas la pression.** Le terme modal reste celui d'ADR-134 ; A261 l'explique et
  S222 a montré que le rendre spatial coûte des secondes.
- **Elle ne rend rien à séparation nulle**, ce qui est correct : deux impacts au même point
  s'additionnent vraiment.
- **Elle n'ordonne pas les champs par autre chose que leur majorant global.** Un autre ancrage
  pourrait resserrer davantage ; rien ne dit lequel, et le balayage d'ancrages coûterait le carré.
- **Elle ne change pas `Steepness`** ni la frontière d'admission d'un champ seul : à un champ, le
  chemin est celui d'avant, au bit.

## Réversibilité

Une méthode, une fonction, une constante d'échantillonnage, et un `slope_floor` qui aiguille au-delà
de deux champs. Revenir consiste à retirer l'aiguillage : la somme d'origine est toujours calculée,
puisque le résultat est son minimum avec l'inégalité.

## Note corrective datée du 2026-09-15 (S236) — la garantie vaut sur l'intersection, pas « pour tout point »

La décision écrit que l'inégalité vaut « pour **tout** point `p` ». L'inégalité, oui ; **le balayage,
non** : il parcourt `r₁ ∈ [0 ; R₁]`, le disque de l'ancre, et ne dit rien des points au-delà. Le
plancher reste juste pour la requête d'ADR-080, qui refuse tout point hors du disque d'un seul impact :
les points admis sont tous dans le disque de l'ancre. Il est **faux** dès qu'on sert l'union des
emprises : test `adr138_sweep_misses_the_union_and_the_union_floor_does_not` — ancre plus énergique en
(0, 0), deux impacts confondus à 100 m nés au même instant, pente réelle **0,4252** contre plancher
**0,2837**. Aucun refus existant n'est changé ; le mode union reçoit son propre plancher
([ADR-142](ADR-142-composition-sur-l-union-des-emprises.md)). Second fait relevé au même passage :
les termes d'un champ dont la distance minimale dépasse son domaine sont ajoutés, alors que ce champ
est nul là — pessimisme sans faute, mesuré sur S235 (49 refus, 40 sans ces termes).
