# Le juge contre le laboratoire : la plage canonique de Synolakis — S712 (liste 4.14, 4.16)

*S712, 2026-10-08, en autonomie ; session longue.* S709–S710 : le déferlement du tout-3D bouge de 0,2 à 0,3 s selon que la 3D garde ou
non son volume. ADR-280 D2 : une référence extérieure tranche.

## La référence

Les mesures de Synolakis (Caltech), publiées par le banc d'essai de la NOAA : l'onde solitaire **H/d = 0,3 qui déferle** sur une pente de
1:19,85. On a les profils de surface aux instants `t·√(g/d)` = 15, 20, 25, 30. Elles sont téléchargées avec l'accord de l'utilisateur, et
leur provenance est dans [references/synolakis](../../references/synolakis/LISEZMOI.md). L'article de Grilli et al. (1997), qui donne le
point de déferlement, est payant, et sa prépublication est refusée (403).

## Le montage

APIC 3D à 2,5 cm, sur deux rangées :
- d = 0,5 m (20 mailles par profondeur, comme le juge), le fond en escalier à 1:19,85 ;
- l'onde `OndeSolitaire`, le profil de Synolakis, centrée en `X₁ = 19,85 + arccosh(√20)/γ` = 24,44 d ;
- 152 088 particules.

L'élévation est lue par la surface reconstruite (φ), lissée sur 10 cm (ADR-280 D1), et comparée aux points mesurés. Le graphique est
produit par `outils/graphique_synolakis.py`, dans `captures/s712_synolakis.png`.

## Reproduire

- `python outils/essai.py the_judge_against_synolakis_without_projection_s712 --ignore` (E1, ≈ 15 min) ;
- `python outils/essai.py the_judge_against_synolakis_weak_projection_s712 --ignore` (E2, ≈ 21 min) ;
- `python outils/graphique_synolakis.py`.

## Mesuré

| t·√(g/d) | écart quadratique, E1 (sans projection) | E2 (κ = 0,05) | crête mesurée | E1 | E2 |
|---|---|---|---|---|---|
| 15 | 0,048 d | 0,051 d | 0,314 d en 8,38 d | **0,433 d** en 8,12 d | **0,478 d** en 7,77 d |
| 20 | 0,066 d | 0,070 d | 0,318 d en 3,66 d | 0,277 d en 3,92 d | 0,359 d en 2,47 d |
| 25 | 0,038 d | 0,036 d | 0,190 d en 0,30 d | 0,234 d en −2,98 d | 0,221 d en −2,53 d |

`V_φ/V_n` à t = 25 : E1 0,958, E2 0,986.

**Le premier passage, arrêté.** Jusqu'à t = 30, la remontée sur la plage sèche faisait tomber le pas à 0,34 ms, puis le calcul se
figeait. Les comparaisons n'étaient lues qu'à la fin : rien n'a été lu. Désormais, chaque photo est comparée dès qu'elle est prise, et le
calcul s'arrête à t = 25.

## Ce que cela dit

- **La référence ne départage pas les deux versions de la 3D.** Leurs écarts diffèrent de 0,003 d au plus, sous la dispersion des
  mesures (≈ 0,01 à 0,02 d entre points voisins). La question de S709–S710 reste ouverte. La projection reste éteinte : elle coûte 40 %
  et ne rapproche pas du laboratoire.
- **Les deux s'écartent du laboratoire avant le déferlement.** À t = 15, la 3D fait une onde trop étroite et trop haute :
  - la crête est à 0,43 et 0,48 d, contre 0,31 d mesurée ;
  - à x = 8 d, la profondeur est de 0,42 d, d'où H/h ≈ 1,0, contre 0,74 au laboratoire ;
  - le dos de l'onde mesurée (x de 9 à 12 d) est 0,1 d plus haut que la 3D.

  À t = 25, la lame monte ≈ 3 d trop loin sur la plage. Les modèles de Navier–Stokes publiés sur ce cas suivent les mesures à
  t = 15–25 : l'écart est celui de notre 3D.
- **Une cause candidate, la résolution.** À x = 8 d, la profondeur n'est que de 8 mailles. S704 a montré que la crête change entre 2,5 et
  1,25 cm.
- **Un artefact de lecture** : des creux de ≈ −0,03 d à chaque marche de l'escalier, la lecture de φ au droit du fond. Ils pèsent sur
  l'écart quadratique, également pour les deux versions.

## La suite (S713)

La même plage à **1,25 cm**, jusqu'à t = 15 : la crête et l'écart contre les mesures. Si la crête descend vers 0,31 d, la résolution est
la cause, et le juge à 2,5 cm l'est aussi. Sinon, on cherche dans la dynamique d'APIC (le transfert, la surface).
