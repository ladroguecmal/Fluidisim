# ADR-133 — Le majorant de pente d'un impact suit la dispersion, et le budget de composition avec lui

- Statut : **actée**, S215, 2026-09-13 ; autonomie technique S71.
- Traite **A254** (sévérité 1, S214). Prolonge [ADR-094](ADR-094-ce-que-le-champ-radial-peut-annoncer-de-sa-pente.md)
  et [ADR-128](ADR-128-le-budget-de-pente-borne-les-perturbations.md) ; éclaire **A208** en
  déplaçant sa cause.
- Ne réécrit aucun ADR. Ne change ni `slope_max()`, ni le refus `Steepness`, ni un bit publié de
  hauteur, de vitesse ou de pente.
- Mesures : [BUDGET-PENTE-S215](../validation/BUDGET-PENTE-S215.md).

## Constat

S214 a mesuré le budget de pente de la scène J1 à **84 % de `BREAKING_SLOPE`** pour *une* source
de chaque type, et S215 a exercé la suite : **deux impacts et un sillage refusent l'image**
(`SlopeEnvelope`, budget 0,5902 contre 0,4488). Le budget est une somme sur les sources
(ADR-128, ADR-119 règle 1) ; il ne passe donc pas à l'échelle en nombre de sources.

**La cause n'est pas celle qu'A254 nommait, et ce n'est pas non plus A208.** Mesurée à
échantillonnage fin — le maximum de pente d'un impact radial est atteint en `r = 0,2062 λ`, soit
0,69 m ici, invisible à la grille de 1,3 m de S214 :

| âge (s) | pente réelle de l'impact | majorant publié | rapport |
|---:|---:|---:|---:|
| 0 | 0,212607 | 0,212607 | **1,00** |
| 4 | 0,049488 | 0,212607 | 4,30 |
| 16 | 0,021147 | 0,212607 | 10,05 |
| 39 | 0,010525 | 0,212607 | 20,20 |
| 56 | 0,006986 | 0,212607 | **30,44** |

Le mécanisme est **la dispersion**, et il est général à W. Un majorant bâti comme une **somme de
modules modaux** — une norme L1 — est invariant quand chaque mode ne fait plus que tourner ; le
maximum **spatial**, lui, décroît à mesure que les phases se décohèrent. Le sillage prescrit le
montre par l'autre bout : son enveloppe est serrée tant que la source force (rapport 1,39 à 1,90
jusqu'à 16 s) et se desserre dès qu'elle s'éteint (3,64 à 24 s, 4,77 à 39 s), pendant que son
majorant reste figé à 0,157.

**Ce n'est donc ni un défaut d'unité — I-18 est tenu, les deux termes sommés sont déjà convertis en
pente réelle (S141) — ni l'alignement d'A208 : c'est le temps.**

## Ce que la mesure a établi de plus, et qui rend la décision possible

**La décroissance est universelle dans la famille.** Le rapport ne dépend que de l'âge
**adimensionné** `τ = (t − birth)/√(λ/g)` : il est identique **à trois décimales** pour λ de 0,5 à
8 m et pour E de 0,05 à 4 000 J — deux énergies à λ égal donnent la même colonne, ce que la
linéarité exigeait et que le contrôle dit au lieu de le supposer.

**La profondeur n'y entre pas.** De 20 à 4 m les colonnes sont identiques ; en deçà,
`RadialImpact::new` **refuse le champ lui-même**. Le domaine où la similitude vaut est exactement
celui où le champ existe.

**Le rapport n'est pas monotone** : il vaut 8,7 à `τ = 0,5` puis retombe à 1,06 à `τ = 1`, la
perturbation s'aplatissant avant de se reformer. Une table sûre prend donc le **minimum par
intervalle**, et non une interpolation.

## Décision

**1. `RadialImpact` publie `slope_max_at(t)`** — la pente réelle maximale du champ **à l'instant
demandé** —, égale à `slope_max() / RHO_DISPERSION[⌊τ⌋]`. La table `RHO_DISPERSION` couvre
`τ ∈ [0, 96[`, la borne d'âge d'ADR-126.

**2. `slope_max()` ne change pas, et le refus `Steepness` non plus.** ADR-094 a posé que migrer ce
refus déplace la frontière d'admission de tous les champs et que c'est une décision distincte ;
elle le reste, et cet ADR ne la prend pas.

**3. Le budget de composition consomme `slope_max_at(time)`.** `composition::compose`,
`mixed_water::sample_world_batch` et `mixed_differential` disposent déjà de l'instant.
`mixed_water::slope_floor` le reçoit désormais en paramètre : sans lui, l'annonce et le refus
seraient calculés à deux instants différents, et la garantie **dans les deux sens** d'ADR-128 —
`max_slope ≥ slope_floor ⟹ aucun refus` — cesserait de tenir. C'est le seul changement
d'interface, et il est exigé par la garantie, pas par le confort.

**4. La loi est mesurée, pas dérivée, et c'est assumé.** `SLOPE_L1_RATIO` est déjà un rapport
mesuré entre borne L1 et pente réelle (ADR-094, S141) ; `RHO_DISPERSION` est le même objet, une
dimension plus riche. Sa provenance (I-14) est `examples/budget_pente_s215.rs --table` : minimum sur
21 sous-échantillons et quatre λ génératrices, puis garde de `1e-4`, soit **trente-trois fois** le
dépassement maximal mesuré (3,0e-6) sur trois λ hors famille.

**5. Le resserrement n'ajoute aucun risque, et c'est vérifié et non argumenté.** Sur quatre λ hors
famille génératrice et 961 valeurs de τ chacune, `max(pente réelle / majorant resserré) = 0,999983`
— **exactement** le pire cas du majorant d'origine. Hors du domaine mesuré — avant la naissance,
au-delà de `τ = 96` — la méthode rend `slope_max()` telle quelle : on ne resserre pas ce qu'on n'a
pas mesuré.

## Ce que cette décision obtient, et ce qu'elle ne fait pas

**Obtient.** Sur la scène J1 à 16 s, l'occupation du budget passe de **84 % à 42 %**, et la scène
qui refusait à deux sources en admet **trois** à 51 %. La marge redevient celle d'une scène.

**Ne fait pas.**

- **Elle ne resserre pas le sillage.** Le mécanisme y est le même et la mesure existe (rapport 1,4
  à 4,8), mais sa famille est paramétrée autrement — σ, cutoff, radial, angular, tronçons — et
  aucune similitude n'y a été établie. Après le resserrement de l'impact, **c'est le sillage qui
  domine le budget** : c'est donc le travail suivant, et il demande sa propre campagne.
- **Elle ne borne pas le nombre de sources.** Le budget reste une somme ; le resserrement recule
  la limite, il ne la supprime pas. Une scène à dix sillages la retrouvera.
- **Elle ne dit rien des impacts anisotropes** — `RadialImpact::new` les refuse — ni d'un autre
  profil radial que celui d'ADR-094.
- **Elle ne change aucun bit publié** : `sample` est intact, et seuls le budget et son annonce
  bougent.
- **Elle ne ferme pas A208.** Le pessimisme d'emprise et d'alignement qu'A208 décrit existe
  toujours ; S215 montre seulement qu'il n'était pas le terme dominant ici.

## Réversibilité

Une table, une méthode, et un paramètre d'instant sur `slope_floor`. Revenir en arrière consiste à
faire lire `slope_max()` aux quatre sites du budget ; rien d'autre n'en dépend. Remplacer la table
par une loi dérivée — une analyse de phase stationnaire donnerait l'exposant plutôt que les
valeurs — est le chemin attendu, et demande alors un ADR qui remplace celui-ci.
