# Le raccord de la boîte : APIC à quatre bords dans Saint-Venant troué — S725 (liste 4.14 ; LOD, étape 3, B3)

*S725, 2026-10-09, en autonomie ; l'utilisateur dort.* LOD-ETAPE-3-S722, B3. Saint-Venant troué (B1, S723) et APIC à quatre bords (B2, S724)
sont réunis : une boîte de 3D au milieu d'une eau en 2D.

## Ce qui est fait

**`relais_boite.rs`, `RelaisBoite`.** À chaque pas, pour chaque face du trou :
1. l'état de la colonne 3D voisine : le niveau par la surface reconstruite, les vitesses moyennes des particules ;
2. le **flux complet** de Rusanov contre la maille active de Saint-Venant (`flux_rusanov`, rendu public), donné à Saint-Venant ;
3. `F₀/h`, borné par la célérité, devient la vitesse du bord d'APIC, et l'eau qui entre est posée :
   - par la grille, à gauche et en y ;
   - par le réseau, à droite ;
4. le pas d'APIC ;
5. le bilan de chaque face (ce que Saint-Venant a cédé, contre ce qu'APIC a reçu et rendu) devient une **dette par face**. Elle est rendue
   par fraction (20 % par pas), étalée sur six mailles voisines, et comptée dans la masse.

Les sorties en y sont comptées par face (`y_outlet_step`).

**Trois fautes trouvées en route, chacune nommée avant d'être corrigée :**
- **le niveau de départ** : la surface reconstruite de la boîte se lit à 0,39931 m pour 0,4 m posés. Saint-Venant, à 0,4 m, poussait son
  eau dans la boîte, où elle n'entre que par quanta, et le couplage s'emballait (1,2 m/s au repos). Comme en S684 (ADR-273 D2, une seule
  source), Saint-Venant part désormais du niveau que lit la 3D ;
- **les pics** : rendre le bilan d'une face d'un coup dans une seule maille y fait un pic de 3 mm par quantum. D'où la dette étalée ;
- **la fuite de masse** (2,4·10⁻¹¹) : le pas d'APIC est un `f32` (0,025000000373), celui de Saint-Venant un `f64` (0,025). Tout ce qui
  touche Saint-Venant se compte désormais avec son pas. Le diagnostic pas à pas est passé de « 12 pas écartés » à 0.

## Reproduire

- E1 : `python outils/essai.py the_box_at_rest_in_saint_venant_s725 --ignore` (≈ 2 min) ;
- E2 : `python outils/essai.py a_hump_crosses_the_box_s725 --ignore` (≈ 2,5 min) ;
- E2′ : `python outils/essai.py a_long_wave_crosses_the_box_s725 --ignore` (≈ 3,5 min) ;
- le diagnostic de la masse : `cargo test --release -p water-core --lib box_mass_diagnostic -- --ignored --nocapture`.

## Mesuré

La boîte : 1 m × 1 m, 0,4 m d'eau, `dx` = 2,5 cm (≈ 205 000 particules), au milieu de Saint-Venant.

| essai | la masse | le résultat |
|---|---|---|
| **E1**, le repos, 1 s (Saint-Venant 3 m × 3 m) | **6,5·10⁻¹⁴** | APIC 4,2·10⁻⁶ m/s, Saint-Venant 2,4·10⁻⁶ m/s, `|η|` 2,3·10⁻⁷ m : **tenu** |
| E2, une bosse de 2 cm, rayon 0,4 m (`kd` ≈ 1,6), 1,2 s | 1,4·10⁻¹⁴ | la réflexion **18 %**, l'aval 18 % : échoue |
| **E2′, une onde longue de 2 cm, rayon 1,2 m (`kd` ≈ 0,5), 1,6 s** (Saint-Venant 6 m × 6 m) | **2,3·10⁻¹⁴** | **la réflexion 9,0 %** (critère 10 %), l'aval 13,2 % : **tenu** |

## Ce que cela dit

- **La boîte de 3D vit au milieu de Saint-Venant** : au repos au micron, la masse au bit, une onde longue qui la traverse en ne réfléchissant
  que 9 % de son amplitude.
- **La réflexion dépend de l'onde.** Une bosse dispersive (E2) réfléchit deux fois plus. La 3D la porte autrement que Saint-Venant (la
  dispersion), et ce désaccord physique réfléchit aussi : un raccord entre deux modèles différents ne peut pas être transparent pour une
  onde que l'un des deux porte mal. Le jugement du raccord seul est l'onde longue (E2′).
- **L'aval (13 %)** mêle le raccord et la différence des modèles (la 3D garde la dispersion que Saint-Venant n'a pas).
- **Le niveau que lit la 3D est la source** : un écart de 0,7 mm entre les deux niveaux suffit à faire s'emballer le couplage.

## La suite

**B4** : la boîte qui suit un corps. Elle naît autour de lui (par la surface), se déplace avec lui (des colonnes naissent devant, meurent
derrière), et meurt quand il sort.
