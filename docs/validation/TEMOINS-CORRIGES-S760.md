# Les témoins du sélecteur refaits avec la 3D d'ADR-294 — S760 (listes 4.1, 4.14 ; ADR-289 D3.3)

*S760, 2026-10-10.* Les témoins tout-3D du sélecteur (SELECTEUR-DOMAINES-S732 §6) avaient été mesurés en S734–S738 avec l'ancienne 3D,
trop haute (S739). **La question** : que deviennent-ils avec la 3D corrigée d'ADR-294 (`Complete` consciente du fond, le compte cumulé
d'énergie) ?

## Ce qui est fait

- `ScenePlage.corrigee` : la 3D d'ADR-294 dans le témoin de S734.
- `ScenePlage.arret_mur` : le témoin s'arrête quand l'onde atteint le mur, à 5 mm du repos (ADR-286 D1, ADR-293 D4).
- `density_energy_removed` : l'énergie que le compte a retirée, cumulée (une lecture).
- `Large::AucunCorrigeeEnergie` : le montage tout-3D de R43 (S755) avec la 3D d'ADR-294.

## Reproduire

- `python outils/essai.py the_selector_witness_s4_corrected_s760 --ignore` (45 min) : le repos 1:3, S4 à 0,1 et à 0,2.
- `python outils/essai.py the_selector_witnesses_s2_s3_corrected_s760 --ignore` (75 min).
- `python outils/essai.py the_r43_witness_with_the_energy_account_s760 --ignore` (20 min).

## Mesuré

Les critères ont été écrits avant le calcul. Le témoin est contrôlé comme l'objet (ADR-285 D1) : aucune particule ne sort, et le front
reste à plus de 1 m du mur, sauf pour S3, arrêté au mur.

**(1) Le repos sur l'escalier de 1:3** : 9,0 mm/s ; 0,12 mm. **Tenu** (1 cm/s ; 3 mm).

**S4, la remontée contre la loi de Synolakis** `2,831·√cot·(H/d)^(5/4)·d`, à 15 % plus le quantum `dx/cot` = 8,3 mm (ADR-293 D3) :

| scène | la loi | **la 3D d'ADR-294** | l'ancienne 3D | le retournement | l'énergie retirée |
|---|---|---|---|---|---|
| (2) `H/d` = 0,1 sur 1:3 | 0,1379 m | **0,1368 m (−0,7 %)**, par φ et par les particules | — | aucun | 0,78 J |
| (3) `H/d` = 0,2 sur 1:3 | 0,3279 m | **0,2835 m (−13,6 %**, la bande ±17,5 %) | +47 % (S734), +37 % au moins (S735) | au reflux, 5,71 s (après le maximum, 3,83 s) | 0,58 J |

**(2) et (3) tenus.** À 0,2, le retournement au reflux est celui que Synolakis annonce (dès 0,141 à 1:3).

**Les autres témoins** :

| scène | **la 3D d'ADR-294** | l'ancienne 3D | l'énergie retirée |
|---|---|---|---|
| (4) S2, `H/d` = 0,3 sur 1:19,85 | le retournement à **4,14 s**, x = 14,09 m (4,3 d avant le rivage, t·√(g/d) = 18,4) ; l'air à 4,35 s | 3,29 s (S734) | 0 |
| (5) S3, `H/d` = 0,5 sur 1:90 | le retournement à **6,84 s**, x = 16,41 m (9,9 m sur la pente) ; l'air à 6,96 s ; l'onde au mur à 9,62 s | 3,49 s, la durée au-delà du mur (S734) | 2,16 J |
| (6) R43, le montage de S755 | **identique à S755** : le retournement 3,0915 s, 11,0875 m ; la remontée 0,3005 m ; la masse 1,2·10⁻¹⁶ | 2,62 s (S730) | 0 |

- **S2** : les mesures de Synolakis placent la crête à 8,38 d du rivage à t·√(g/d) = 15, et à 3,66 d à 20. Interpolée à 18,4, elle est vers 5,2 d. La
  3D retourne à 4,3 d : 0,9 d en avance, l'avance de la crête déjà vue (ADR-292 D3).
- **S3** : la vague retourne. Avec la 3D corrigée, le déferlement de S3 n'est pas seulement glissant : la question de la scène (la 3D est-elle
  nécessaire ?) reçoit une première réponse, oui, à cette résolution. Il reste à la juger contre une référence (les seuils de
  déferlement glissant).
- **R43** : le compte n'a rien retiré, et le calcul est identique à S755. **S755 reste vrai tel quel.**

## Ce que cela dit

- **La 3D corrigée remonte juste** sans déferlement (−0,7 %), là où l'ancienne remontait de 37 à 47 % trop haut. La remontée de S645
  (−11 %, ADR-292 D3) n'est donc pas un défaut de la 3D corrigée : son montage (l'onde posée près du pied) en est la cause probable.
- **Les vagues retournent plus tard** que dans l'ancienne 3D, de 0,47 à 0,85 s (R43, S2), et S3 de 3,3 s. Le calibrage du prédicteur
  (SELECTEUR-DOMAINES-S732 §7, P1) se fera sur ces témoins-ci.
- **Le compte d'énergie joue** dans les scènes sans grand déferlement (S4 : 0,6 à 0,8 J ; S3 : 2,2 J), et pas où le déferlement dissipe
  (S2, R43).

## La suite

- S761 : la cinquante-sixième revue de méthode (ADR-222 D4).
- ADR-289 D3.2, le banc de la 3D : le corps qui flotte (Archimède, exact) ; la rupture de barrage (les mesures de Martin et Moyce, un
  téléchargement à accorder).
- Le calibrage du prédicteur (`x_b`, `t_b`) sur les témoins de S760, puis le raccord du rivage replacé (≈ 12,2 m pour R43, ADR-284).
