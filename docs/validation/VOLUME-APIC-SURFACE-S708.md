# Le volume d'APIC par sa surface, contre le compte — S708 (liste 4.14, 1.6)

*S708, 2026-10-08, en autonomie ; session longue.* S707 : APIC tasse ses particules sous la crête. Ici, on mesure ce que cela coûte au
volume d'eau qu'APIC simule, celui de sa surface et de ses étiquettes, non celui qu'on compte.

## L'instrument

`V_φ = Σ clamp(½ − φ/dx, 0, 1)·dx³` sur les mailles non solides, avec φ la distance signée reconstruite (`distance()`, l'eau où
φ < 0). C'est la surface qui commande les étiquettes, donc la dynamique. On la compare à `V_n = n · quantum`, ce que comptent les raccords.

## Reproduire

- `python outils/essai.py the_surface_volume_at_rest_s708 --ignore` (E1, ≈ 40 s) ;
- `python outils/essai.py the_flat_wave_by_its_surface_s708 --ignore` (E2, ≈ 2 min) ;
- `python outils/essai.py the_full_3d_volume_by_its_surface_s708 --ignore` (E3, ≈ 13 min).

## Mesuré

**E1 — l'étalon, tenu.** Au repos (4 m, `h` = 0,49 m, 1 s), `V_φ / V_n` vaut 0,99967 au départ comme à 1 s. La reconstruction rend le
compte à 3·10⁻⁴ près.

**E2 — l'onde plate de S707** (10 m, 1,6 s) :

| t | `V_φ / V_φ(0)` | la crête par la surface | la crête par le compte |
|---|---|---|---|
| 0,01 s | 1,0000 | 0,149 m | 0,149 m |
| 0,4 s | 0,9961 | 0,133 m | 0,136 m |
| 0,6 s | 0,9935 | 0,134 m | 0,140 m |
| 1,0 s | 0,9885 | 0,132 m | 0,147 m |
| 1,6 s | **0,9813** | **0,137 m** | 0,178 m |

**E3 — le tout-3D de S690** (le montage sans raccord, 4 s, le déferlement à 2,637 s) :

| t | `V_n / V_n(0)` (le rivage retire après 2,7 s) | `V_φ / V_φ(0)` | `V_φ / V_n` |
|---|---|---|---|
| 0 s | 1 | 1 | 0,9987 |
| 1,0 s | 1 | 0,9879 | 0,9866 |
| 2,0 s | 1 | 0,9768 | 0,9756 |
| 2,5 s | 1 | 0,9680 | **0,9667** |
| 3,0 s | 0,9944 | 0,9532 | 0,9573 |
| 4,0 s | 0,9653 | 0,9094 | **0,9409** |

## Ce que cela dit

- **APIC perd du volume géométrique dès que l'eau bouge** : ≈ 1,2 à 1,3 % par seconde à compte constant, plus vite après le déferlement.
  Le nombre de particules est exact, mais elles se tassent. La séparation existante n'écarte que les paires à moins de 0,4 maille,
  pour un espacement normal de 0,5.
- **La croissance de l'onde plate était un artefact du compte.** Par sa surface, l'onde se pose vers 0,133 m et y reste.
- **Le juge tout-3D déferle avec ≈ 3,3 % d'eau effective en moins**, au moment du retournement.
- **Ce qu'il faut reformuler.**
  - La masse « au bit » des raccords (S682–S703) est vraie **du compte de particules**, ce que les solveurs s'échangent ; le volume que
    la 3D simule, lui, dérive.
  - Les lectures par le compte (S700, S704, S707) mesurent densité × hauteur.
  - S704 est à relire par la surface, après le remède.

## La suite (S709)

**La projection de densité** (Kugelstadt et al., 2019, *Implicit density projection for volume conserving liquids*). À chaque pas :
- la densité de particules par maille ;
- une seconde équation de Poisson sur son excès ;
- les **positions** déplacées par son gradient, les vitesses non.

Elle se fait en option d'abord, jugée sur ces trois essais :
- au repos, rien ne change ;
- `V_φ` tenu à mieux que 0,3 % sur l'onde plate et sur le tout-3D ;
- le coût est mesuré.

Le juge tout-3D se relance ensuite, et son retournement se compare.
