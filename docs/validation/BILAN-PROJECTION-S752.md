# Le bilan propre de la projection, et R1′ — S752 (liste 4.1 ; ADR-290 D1 ; ADR-291)

*S752, 2026-10-09.* DEUX-REMEDES-S750 : R1 tient presque tout, mais l'onde s'atténue de 9 %. **La question** : d'où vient l'atténuation, et
une mise à jour de la vitesse sans la grille (R1′ : `v + C·Δx`) la supprime-t-elle ?

## Ce qui est fait

- `Apic3::density_projection_budget` : l'énergie cinétique et potentielle changées par la projection, cumulées (ADR-290 D1).
- `set_density_shift_affine` (R1′) : la vitesse corrigée par la matrice affine, `v_a += Σ_b C[a][b]·Δx_b`.

## Reproduire

- `python outils/essai.py affine_shift_rest_and_channel_s752 --ignore` (30 min).
- `python outils/essai.py runup_with_affine_shift_s752 --ignore` (6,5 min).

## Mesuré

| version | le repos | le bilan, cinétique | le bilan, potentielle | l'onde : largeur, creux, crête | la remontée |
|---|---|---|---|---|---|
| `Complete` consciente | tenu | 0 J | +2,11 J | 92 % ; 9,0 mm ; +3,2 % | −11,0 % (S744) |
| R1 | tenu (S750) | +0,12 J | +3,02 J | 92 % ; 10,8 mm ; **−7,5 %** | +7,9 % (S750) |
| **R1′** | **tenu** (6,8 mm/s ; 0,02–0,07 mm) | **+2,85 J** | +3,06 J | 80 % ; 9,3 mm ; **+14,7 %** | **−11,1 %** |

**L'hypothèse est réfutée.** R1′ injecte de l'énergie cinétique (+2,85 J), fait grossir l'onde de 15 %, et ne rend pas la remontée. Le
bénéfice de R1 ne vient donc pas du transport de la vitesse avec le déplacement : il vient de ce que la particule **reprend** la vitesse
cohérente de la grille. **Toutes les versions soulèvent un peu l'eau** (+2 à +3 J d'énergie potentielle en 4,25 s) : c'est la dilatation
de S748, mesurée cette fois directement.

## Ce que cela dit, et la décision

Après huit tentatives (S740–S752), **R1 est retenue comme la 3D corrigée, version 1** ([ADR-291](../adr/ADR-291-la-3d-corrigee-version-1.md)),
avec ses écarts mesurés : la remontée +8 %, l'onde −7,5 % sur 10 m, le creux 10,8 mm. Ses écarts restants seront jugés contre les mesures
de Synolakis, plutôt que par un réglage de plus.
