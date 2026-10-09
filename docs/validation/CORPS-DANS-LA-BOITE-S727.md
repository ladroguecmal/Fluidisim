# Un corps dans la boîte — S727 (liste 4.14 ; LOD, étape 3, B4a)

*S727, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-3-S722, B4a. La boîte de 3D du raccord de S725, fixe. Un corps la
traverse. Le corps fait-il, dans la boîte entourée de Saint-Venant, ce qu'il ferait dans la 3D entière ?

## L'essai

Une sphère de rayon 8 cm, son centre à la surface (à demi immergée), tirée à 0,3 m/s selon x pendant 1,5 s :
- **le témoin** : un APIC entier de 2 m × 2 m (80 × 80 × 24, ≈ 820 000 particules), aux murs fermés ;
- **la boîte** : un APIC de 1 m × 1 m (≈ 205 000 particules) au milieu, dans un Saint-Venant de 2 m × 2 m troué au même endroit, aux mêmes
  murs. Saint-Venant part du niveau que lit la boîte (ADR-283 D1).

Le même corps, le même réseau de particules, les mêmes murs : seul le dehors de la boîte change (la 3D contre Saint-Venant). Les vagues d'un
corps sont courtes, et Saint-Venant les porte mal (ADR-283 D1). Les critères jugent donc ce qui compte en jeu, la 3D près du corps.

## Reproduire

- `python outils/essai.py a_body_in_the_box_s727 --ignore` (≈ 13 min, le témoin compris).

## Mesuré

| critère | mesuré | seuil |
|---|---|---|
| (1) la force sur la sphère : l'écart moyen au témoin, de 0,2 à 1,5 s | **2,0 %** de sa moyenne | 10 % |
| (2) la surface dans la boîte à 1,0 s : l'écart quadratique au témoin | **1,3 mm, 1,7 %** de la plus haute vague (7,8 cm, l'eau soulevée par le corps) | 20 % |
| (3) la masse | **3,9·10⁻¹⁵** | 10⁻¹² |

**Critères : tenus.**

## Ce que cela dit

- **Un corps dans une boîte de 3D entourée de Saint-Venant se comporte comme dans la 3D entière** : sa force à 2 %, l'eau autour de lui à
  2 % de la plus haute vague, avec quatre fois moins de particules.
- Les vagues courtes qui sortent de la boîte, que Saint-Venant porte mal, ne reviennent pas troubler le corps, du moins sur une boîte de
  1 m autour d'un corps de 16 cm.
- C'est la pièce qui compte pour le jeu : un joueur, un objet, dans une eau en 2D, avec sa bulle de 3D.

## La suite

**B4b** : la boîte qui suit le corps. Elle se déplace avec lui : des colonnes naissent devant (depuis Saint-Venant, par la surface) et
meurent derrière (vers Saint-Venant, par la surface). On la juge contre le témoin de B4a, un corps qui sort de la boîte fixe.
