# Le lac au repos sur une pente — S743 (liste 4.1 ; ADR-287 D1)

*S743, 2026-10-09.* BANC-CANONIQUE-S742 : sur le fond lisse en pente, le bassin oscillait. **La question** : la 3D tient-elle un lac au
repos sur une pente, avec ou sans la projection de densité, sur l'escalier ou le fond lisse ?

## Ce qui est fait

`repos_pente_s743` : 1 m plat à 0,30 m d'eau, la pente jusqu'au sec, 0,5 m de sec ; 2,5 cm, deux rangées, 2 s. Deux lectures de la surface
contre le niveau (ADR-286 D2), sur les colonnes de plus de trois mailles d'eau :
- par les particules : la plus haute, plus `dx/4` ;
- par φ.

La plus grande vitesse des particules.

## Reproduire

`python outils/essai.py the_lake_at_rest_on_slopes_s743 --ignore` (6,5 min, les huit cas).

## Mesuré

Le critère : la vitesse sous 1 cm/s, l'écart par les particules sous 3 mm.

| pente | fond | projection | vitesse max | écart, particules | écart, φ | verdict |
|---|---|---|---|---|---|---|
| 1:30 | escalier | non | 6,8 mm/s | 0,01 mm | 0,58 mm | **tenu** |
| 1:30 | escalier | oui | 6,3 cm/s | 4,63 mm | 7,19 mm | **non tenu** |
| 1:30 | lisse | non | **0,68 m/s** | 0,82 mm | **22,7 mm** | **non tenu** |
| 1:30 | lisse | oui | **1,96 m/s** | **52,8 mm** | 35,5 mm | **non tenu** |
| 1:12 | escalier | non | 6,8 mm/s | 0,07 mm | 0,59 mm | **tenu** |
| 1:12 | escalier | oui | 6,3 cm/s | 4,71 mm | 7,31 mm | **non tenu** |
| 1:12 | lisse | non | **0,27 m/s** | 1,36 mm | **22,0 mm** | **non tenu** |
| 1:12 | lisse | oui | **1,87 m/s** | **36,3 mm** | 33,6 mm | **non tenu** |

## Ce que cela dit

- **Seule la 3D sur l'escalier, sans projection, tient le repos.** C'est la configuration de S639, et des témoins S1 et S2.
- **La projection de densité ne sait pas traiter un fond posé dans le domaine.** Sur le canal (S740), le fond était le plancher du domaine,
  et elle gardait l'onde. Contre l'escalier, elle met le lac en mouvement (6 cm/s, 5 mm) ; contre le fond lisse, il diverge. C'est sans
  doute ce qui freinait la remontée de S645 (S742, −12 %). Sa densité, près d'un solide, ne compte pas le solide comme la reconstruction le
  fait (les particules reflétées sous le fond, S639).
- **Le fond lisse ne tient pas le repos** : des particules y prennent 0,3 à 0,7 m/s sans que la surface bouge (0,8 à 1,4 mm), et la lecture
  par φ s'y trompe de 22 mm. C'est ce qui ruinait la levée de S742.

## La suite (S744)

**La projection de densité consciente du fond** : dans sa densité, le volume solide d'une maille coupée ou d'une maille d'escalier compte
comme de l'eau pleine, comme le fait la reconstruction de la surface. Elle sera jugée sur ce banc (le repos, l'escalier), puis sur le canal
(l'onde solitaire) et sur la remontée de S645. Le fond lisse viendra après, son propre défaut à part.
