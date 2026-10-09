# La projection de densité hybride — S747 (liste 4.1 ; ADR-287 D1, ADR-289)

*S747, 2026-10-09.* SANS-SURFACE-S745 : corriger la surface garde l'onde mais freine la lame ; ne pas la corriger fait l'inverse. **La
question** : une projection hybride, complète sauf dans la lame mince (moins de trois mailles d'eau dans la colonne), tient-elle les trois
essais ?

## Ce qui est fait

`DensityVariant::Hybrid` (`LAME_MINCE_S747` = 3) : comme `Complete` dans une colonne d'au moins trois mailles d'eau, aucune correction en
dessous. Les mailles d'eau de chaque colonne sont comptées à chaque pas, dans un tableau réservé à la configuration.

## Reproduire

- `python outils/essai.py runup_with_hybrid_density_s747 --ignore` (6 min).
- `python outils/essai.py rest_and_channel_with_hybrid_density_s747 --ignore` (11 min).

## Mesuré

| essai | critère | `Complete` (S744) | `WithoutSurface` (S745) | **hybride** |
|---|---|---|---|---|
| le repos, l'escalier 1:30 et 1:12 | 1 cm/s ; 3 mm | tenu | tenu | **tenu** (6,8 mm/s ; 0,01–0,07 mm) |
| l'onde solitaire, canal à 2,5 cm | largeur 80 % ; creux 10 % de `H` | tenu (92 % ; 9 mm) | non tenu (87 % ; 40,5 mm) | **tenu** (92 % ; 9,0 mm ; la crête finale 100,2 mm) |
| la remontée de S645 | 10 % de 0,2295 m | −11,0 % | **−5,5 %** | **−12,2 %, non tenu** |

## Ce que cela dit

- **L'hybride garde l'onde et le repos, mais freine encore la lame.** Le freinage ne vient donc pas de la lame mince : il vient de la
  correction de surface en eau plus profonde, là où le ressaut monte la pente.
- La projection déplace les particules sans changer leur vitesse. Dans le ressaut, ce déplacement peut retirer de l'énergie à l'eau qui
  monte. C'est l'hypothèse à regarder d'abord (ADR-287 D4 : le champ avant le second suspect).

## La suite (ADR-289 D3.1)

Regarder le ressaut avec et sans la correction de surface : le déplacement de la projection, l'énergie de la lame, la vitesse du front.
Puis le remède, ou une autre approche que la projection si aucune variante ne tient les trois essais.
