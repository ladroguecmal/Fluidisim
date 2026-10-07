# La côte 2D porte le niveau moyen et le courant de dérive — S673 (listes 2.7, 12.3)

*S673, 2026-10-07, en autonomie, vers la v2.* `houle_moyenne.rs` (S672) calcule le niveau moyen et le courant de dérive d'une côte
droite. `Cote2D` (S670) cuit la mer qui déferle. Les deux sont maintenant reliés.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s673 -- --nocapture` (≈ 11 s).

## Ce qui a été fait

- **`Cote2D::cuire_deferlante` reçoit `c_f`**, le frottement au fond (≈ 0,01 sur le sable).
  - Par rangée de la marche, l'énergie de chaque composante est moyennée le long de la côte ; son vecteur d'onde vient de Snell.
  - La rangée donne `S_ss` et `S_sn`, puis `η̄(s)` (rapporté au bord du large) et `V(s)`.
  - Ils sont stockés par rangée des tables : 8 octets par rangée, rien par composante.
- **`eval`** ajoute `η̄` à `η` et `V·t̂` à `u_total`, linéaires en `s`. Sans déferlement, rien ne change : les sorties de S664–S668 sont
  identiques.
- **`houle_moyenne`** :
  - les vitesses au fond sont échantillonnées une fois par rangée ;
  - la racine du frottement est trouvée par fausse position (Illinois) au lieu de la bissection ;
  - le courant n'est calculé qu'aux rangées des tables.

  Les valeurs de S672 sont inchangées. La cuisson est passée de 8,3 s (la première version) à 5,3 s.

## Mesuré

**L'instrument** : l'équilibre d'énergie 1D de S669 donne l'amplitude de chaque composante ; ces amplitudes passent par
`houle_moyenne`.

**Ce qui départage** :

- une moyenne le long de la côte juste suit la référence au plancher (0,8 % sur `S`) ;
- un `t̂` retourné inverse le courant ;
- une rangée décalée déplace le pic de `V`.

| | critère | mesuré |
|---|---|---|
| (1) sans déferlement | au bit ; S672 inchangé | les sorties de S664–S668 identiques ; S672 aux mêmes valeurs |
| (2) `η̄` des tables, à chaque rangée | < 3 % du pic de la référence | **0,21 %** |
| (2) `V` des tables, à chaque rangée | < 5 % du pic | **0,64 %** |
| (3) `eval` : `η` relevé de `η̄`, `u_total` de `V·t̂` | à 10⁻⁵ | **6·10⁻⁸** |
| (3) le sens du courant | celui de `S_sn` au large | tenu |
| (4) la remontée au rivage (1 m) | rapportée | **13,0 cm** (1D 12,9) |
| (4) le creux le plus bas | rapporté | −2,2 cm, au bord de la bande |
| (4) le pic du courant | rapporté | **0,113 m/s** à 3,2 m de fond |
| (4) la cuisson | rapportée | 5,3 s pour 8 composantes (3,95 km × 192 m) ; le niveau et la dérive, ≈ 3,6 s |

La mer de S667 vient de −20° à +20°, presque de face en moyenne. Le courant reste donc modeste ; une houle plus oblique le rendrait plus
fort (S672 : 27 cm/s pour 5° à 2 m).

## Ce que cela dit

La surface de B près du rivage porte maintenant ce que la mer fait à l'eau en moyenne : le niveau remonte de 13 cm au rivage, et l'eau
court le long de la plage. Un objet flottant dérivera ; la ligne d'eau montera avec la houle.

**Ne fait pas** :

- la rétroaction du niveau sur le déferlement : 13 cm sur 1 m de fond, c'est 13 % de profondeur en plus au rivage ;
- le mélange latéral, la circulation d'une côte non uniforme, le reflux sous la surface ;
- un `c_f` propre au matériau du fond.

Le coût du niveau et de la dérive (≈ 3,6 s) peut baisser en sautant les rangées sans force, au large de la bande.
