# Le juge des raccords sous un pas plus court — S714 (liste 4.14)

*S714, 2026-10-09, en autonomie ; l'utilisateur dort.* S713 : sur la plage de Synolakis, le pas de 2,5 ms rend juste la crête du
déferlement, que le pas de 10 ms plaçait 0,04 d trop bas. Le juge des raccords (le tout-3D de S690, au pas de 10 ms) en dépend-il ?

## Reproduire

- `python outils/essai.py the_full_3d_judge_with_a_short_step_s714 --ignore` (≈ 29 min). Le mode nommé est `Large::AucunPasCourt`
  (ADR-277 D2).

## Mesuré

| le tout-3D de S690 | retournement | air enfermé | coût |
|---|---|---|---|
| pas ≤ 10 ms (le juge) | 2,637 s, 9,988 m | 2,790 s, 10,325 m | 777 s |
| **pas ≤ 2,5 ms** | **2,595 s, 9,938 m** | 2,750 s, 10,308 m | 1 731 s |

La masse : 1,2·10⁻¹⁶. La dette sous un quantum.

## Ce que cela dit

- **Le juge dépend un peu du pas** : −0,042 s et −0,05 m. C'est dans la tolérance d'ADR-278 D2 (0,1 s, 0,15 m). Les verdicts des
  raccords (S693–S703), tous jugés au même pas de 10 ms, tiennent.
- **Le plafond de 10 ms reste celui des montages de déferlement** : 2,2 fois moins cher. La sensibilité est inscrite. Elle entre dans
  l'incertitude du juge (ADR-280 D2), avec celle du volume (S709–S710, 0,2 à 0,3 s), qui reste la plus grande.
- Sur la plage de Synolakis, le pas court rendait la crête du déferlement à 0,002 d. Pour une séance visuelle où le moment du déferlement
  compte, le pas de 2,5 ms est le plus juste, au prix de ce coût.

## La suite

Le LOD reprend (LOD-ETAPE-2-S705), sur ce que S707–S708 ont appris : la renaissance et la mort de la 3D lisent la surface, non le compte.
**S715** : N2 par la surface. **S716** : la revue.
