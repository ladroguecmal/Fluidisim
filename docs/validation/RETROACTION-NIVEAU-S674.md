# Le niveau moyen rétroagit sur le déferlement — S674 (liste 2.7)

*S674, 2026-10-07, en autonomie, vers la v2.* `Cote2D` (S673) relevait le niveau de 13 cm au rivage. Sa mer déferlait pourtant encore
sur la profondeur au repos, alors que la profondeur réelle y est plus grande de 13 %.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s674 -- --nocapture` (≈ 15 s).

## Ce qui a été fait

`cuire_deferlante` cherche maintenant un **point fixe**. La mer marche sur `h + η̄(s)` ; `η̄` est recalculé de sa contrainte de radiation,
et la marche recommence, jusqu'à `|Δη̄|` < 1 mm (au plus 8 marches). Les tables (`k`, `coth`) sont cuites sur la profondeur totale de
la dernière marche. Le courant est calculé une fois, à la fin. Les écarts de chaque marche sont gardés (`ecarts_du_niveau`).

`cuire_interne` garde le nombre de marches en paramètre. Les essais de S670 et S673, écrits pour une seule marche, le restent : leurs
valeurs sont inchangées. La première marche du point fixe est donc celle de S673.

## Mesuré

**L'instrument** : le même point fixe en 1D dans l'essai. L'équilibre d'énergie de S669 marche sur `h + η̄_ref`, et `η̄_ref` vient de
`houle_moyenne`.

**Ce qui départage** :

- une rétroaction juste suit la référence au plancher de S673 ;
- une profondeur totale oubliée dans les tables ou dans le déferlement laisse `Hrms` au rivage à 0,512 m au lieu de 0,552 m ;
- un raccord faux ne converge pas en trois marches.

| | critère | mesuré |
|---|---|---|
| (1) sans déferlement ; la première marche | au bit ; S670 et S673 inchangés | au bit ; inchangés |
| (2) le facteur de chaque composante | < 2 % du point fixe 1D | **0,43 %** |
| (2) `η̄`, à chaque rangée | < 3 % du pic | **0,21 %** |
| (2) `V`, à chaque rangée | < 5 % du pic | **0,65 %** |
| (3) la convergence | sous 1 mm en au plus 4 marches | **3 marches** : 129,6 ; 4,0 ; 0,1 mm (1D : 129,4 ; 4,0 ; 0,1) |
| (4) `Hrms` au rivage (1 m) | rapporté | 0,513 m sans rétroaction → **0,553 m** avec (1D 0,552 ; le plan : 0,552) |
| (4) `η̄` au rivage | rapporté | 12,96 cm → **12,57 cm** (1D 12,56 ; le plan : 12,56) |
| (4) la cuisson | rapportée | 7,3 s (5,4 s pour une marche) |

## Ce que cela dit

Le niveau qui monte laisse passer plus de houle : au rivage, `Hrms` gagne 8 %, et la remontée baisse d'un demi-centimètre. Le point
fixe converge vite, d'un facteur trente par marche. Les deux effets sont couplés comme dans Battjes et Janssen (1978), qui mettaient
le niveau dans la profondeur.

**Ne fait pas** : le niveau dans la profondeur des marches de W et de δ ; le jet de rive, au-delà de 1 m.
