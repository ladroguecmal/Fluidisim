# Le rouleau 3D : le jet qui retombe et enferme l'air — S648 (listes 4.14, 4.16)

*S648, 2026-10-07, en autonomie, vers la v2.* La campagne du rouleau 3D. L'onde de S647 se retourne sur la pente 1:12. Ici, la suite : le
jet retombe devant la crête et **enferme de l'air**, la signature d'un déferlement plongeant.

## Reproduire

- Le lecteur, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_enclosed_air_reader -- --nocapture`.
- Le rouleau : `… the_plunging_jet_encloses_air -- --ignored --nocapture --test-threads 1`. Compter 6 min à 5 cm et 39 min à 2,5 cm.

## 1. Le lecteur, éprouvé avant de juger (ADR-263 D2)

**L'air enfermé** : les mailles d'air que l'air libre, depuis la rangée du haut, n'atteint pas par voisins, dans tout le domaine. On en tire
leur nombre et l'abscisse de leur centre.

| cas posé | attendu | lu |
|---|---|---|
| une couche d'eau à 0,6 m | 0 maille | 0 |
| la même, avec une cavité x ∈ [0,9 ; 1,1] m, z ∈ [0,25 ; 0,40] m | au plus 48 mailles, centrée à 0,1 m de 1,0 m | **48 mailles, centre 1,000 m** |

## 2. Mesuré

| | 5 cm | 2,5 cm |
|---|---|---|
| 1:12 — premier retournement | 2,571 s, 9,675 m | 2,642 s, 9,988 m |
| 1:12 — premier air enfermé | 2,872 s (+0,30 s), 10,525 m (+0,85 m) | **2,817 s (+0,18 s), 10,375 m (+0,39 m)** |
| 1:12 — air enfermé au plus | 18 mailles = 2,3 L, environ `1,0·H²` par largeur | 210 mailles = 3,3 L, environ `1,5·H²` par largeur |
| 1:12 — vie de l'air enfermé | 1,12 s | 1,18 s |
| 1:3 (S644) — retournement, air enfermé | aucun | aucun |
| particules gardées, aucune sous le fond | tenu | tenu |

**Critères, écrits avant.**

- (1) Le lecteur : tenu.
- (2) À 2,5 cm, de l'air enfermé apparaît 0,18 s après le retournement, en avant de lui : tenu. Le jet a retombé.
- (3) Rapportés : la poche la plus grande et sa vie. Avec la maille, le volume croît (×1,5) et la vie se tient (1,12 → 1,18 s).
- (4) La masse : tenu.
- (5) L'onde de S644 n'enferme pas d'air : tenu.

## 3. Ce que cela dit — et ne dit pas

Sur la pente où Grilli et al. (1997) l'annoncent plongeante, la vague d'APIC 3D fait ce que fait un rouleau plongeant :

1. la crête se retourne ;
2. le jet retombe en avant ;
3. une poche d'air reste prise sous lui ;
4. la poche vit environ une seconde, puis l'air s'échappe.

Sur la pente raide, rien de cela. La masse est exacte tout du long.

Ce n'est pas encore jugé :

- **la quantité d'air contre une mesure publiée** : aucune n'a pu être relue ; le volume n'a pas convergé (+50 % de 5 à 2,5 cm) ;
- **la pression dans la poche** : les poches d'air de K2 (S479) ne sont pas actives ici, l'air enfermé reste à la pression atmosphérique ;
- **l'écume** qui suit, suspendue ;
- **le coût** : 39 min pour 4 s simulées à 2,5 cm (4.19).

Manquent : la quantité d'air jugée, les poches de K2 actives sous le rouleau, le relais 2D → 3D (étape 4), le rouleau qui agit (étape 5).
