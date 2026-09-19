# Fond spectral réel et frontières de la référence 3D — S298

2026-09-19. Consommateur : référence CPU d’ADR-175 et son aperçu ; production GPU distincte.
Aucune tolérance antérieure déplacée. Aucun état δ sérialisé, aucune dépendance ajoutée.

## 1. Entrée réelle du fond

`BackgroundGrid3` réserve avant scellement les trois familles MAC, un jeu en préparation et
un jeu publié. Les coordonnées sont locales à l’ancre de B, axes communs, origine au fond du
domaine ; z est relatif au plan moyen de B. Le fond est recalculé à chaque instant, jamais
sauvegardé. Après refus, champs, gravité et instant publiés restent ceux du dernier succès.
Avant le premier succès, `view()` ne fournit rien ; un instant périmé est refusé par le pas.

Le chemin consomme `Background::differential_grid_extended` (S276, ADR-154), une grille x-z
par rangée y, puis range les faces dans l’ordre MAC. Phase et atténuation ne sont plus
recalculées par point. Scratch, coordonnées et deux jeux MAC sont comptés à la configuration.
Les tests comparent chaque champ au ponctuel prolongé aux trois familles, sur un domaine
non carré, sous et au-dessus du plan moyen, à trois instants dont 9 876 s ; test des refus,
réserve et absence d’allocation de l’échantillonnage puis du vrai pas couplé.

**Portée** : B réel seulement. Le prolongement de W au-dessus du plan moyen reste A286.
Le fond B est profond : il n’impose pas une paroi au fond de la boîte δ. L’atténuation et
la vitesse verticale résiduelle doivent donc être publiées pour chaque scénario consommateur.

## 2. Cas limites des frontières — critères avant mesure

`delta3d_boundaries` compare le pas 3D au pas 2D reçu, avec son préconditionneur ordinaire.
Les coordonnées et fixtures physiques sont identiques ; la direction se transpose de x à y.
Un seul élément dans la direction transverse : cas limite, pas un test d’incidence oblique.

- Houle progressive S272–S274 : h=1 m, λ=4 m, amplitude 1 cm, emprise 4 m,
  2 000 pas de 1 ms ; dx = 0,125 / 0,0625 / 0,03125 m, deux axes.
- Paquet S269 : spectre initial et vitesses du potentiel inchangés, désormais partagés par
  `support/reflection_packet.rs`. Domaine 24 m, profondeur 1 m, amplitude 2 mm, jauge 12 m ;
  7 200 pas de 5 ms, dx=0,25 puis 0,125 m ; mur et éponge, sur chaque axe.
  Éponge 4 m, taux `10 cg(π)/4`, relaxation de hauteur et vitesse.
- Aucun refus, hauteur 3D−2D <3 mm (repère S201). Pour le paquet,
  `sqrt(∫(jauge3−jauge2)² dt / E_incident2)` ≤0,001 : réserve d’instrument S269.
  Maxima de pente et vitesse publiés, aucun seuil nouveau inventé.

## 3. Résultats des cas limites

Six trajectoires progressives et huit de paquet, chacune calculée en 2D et en 3D :
**139 200 pas acceptés**, aucun refus. L’écart maximal de hauteur vaut **1,192093·10⁻⁷ m**
(0,000119 mm), tous cas et axes confondus. Sur mur/éponge grossiers dans x : identité des
hauteurs et vitesses à chaque pas. Le changement de préconditionneur ne déplace pas le reçu.

| cas | dx (m) | max pente 3D−2D, deux axes | max vitesse (m/s), deux axes | erreur énergétique jauge x / y |
|---|---:|---:|---:|---:|
| progressive | 0,125 | 9,537e-7 | 9,095e-10 | — |
| progressive | 0,0625 | 1,907e-6 | 1,384e-9 | — |
| progressive | 0,03125 | 3,815e-6 | 5,675e-9 | — |
| mur | 0,25 | 9,537e-7 | 1,537e-8 | 0 / 8,560e-6 |
| mur | 0,125 | 1,907e-6 | 5,215e-8 | 1,591e-5 / 1,581e-5 |
| éponge | 0,25 | 9,537e-7 | 1,164e-8 | 0 / 8,704e-6 |
| éponge | 0,125 | 9,537e-7 | 2,841e-8 | 1,017e-5 / 1,030e-5 |

Critères d’équivalence tenus. La trace complète reste en mémoire ; seules les mesures sont
imprimées. Le maximum d’itérations 3D est 235 pour la houle fine, 65 pour le paquet.

**Ce que ce reçu ne dit pas.** Les doubles gardes 48/72 m de S269 ne sont pas rejouées ici :
aucun nouveau coefficient de réflexion n’est annoncé. On reçoit la reproduction des trajectoires
2D dans la boîte courte, dans les deux orientations. Ni absorption oblique/aux coins, ni mer
large bande à la frontière, ni coefficient d’ordre deux S274 au budget de 2 % ne deviennent
reçus par cette équivalence. Ces limites physiques restent celles de S269/S274.

## 4. Reproduction

```powershell
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_boundaries -- progressive
cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_boundaries -- packet
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline real_background_
cargo test --manifest-path code/Cargo.toml -p water-core --release --offline --example delta_reflection
```

Le potentiel partagé garde ses tests d’incompressibilité, de cinématique et de direction :
6 tests d’exemple réussis, 1 ignoré. Les temps muraux de calculs concurrents ne reçoivent
aucune performance de production.
