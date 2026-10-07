# Un canal dans V : la loi de Manning — S592 (liste 2.5)

*S592, 2026-10-07, en autonomie (ADR-247).* 2.5 était absent ; c'est aussi la base des rivières (2.4).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s592 -- --nocapture` ; suite du cœur : 766.

## 1. Ce qui est construit

Une loi d'arête de V, **`Flow::Manning { width_mm, length_mm, roughness_e6, outlet_slope_e6 }`** : un bief de canal rectangulaire,
`Q = (1/n)·A·R^(2/3)·√S_f`, la profondeur moyenne des deux côtés au seuil, la pente de frottement entre les deux surfaces ; vers dehors,
la sortie en régime uniforme (la pente du lit donnée). Paramètres entiers (I-10). Ajoutée à la validation du pas, à celle de l'instantané
et à son empreinte ; les liquides de V l'héritent.

## 2. Mesuré (références écrites au plan par son script, ses rapports seuil/quantum vérifiés — ADR-249 D1)

Dix biefs de 100 × 5 m, le lit descendant de 0,1 m de bief en bief (`S` = 10⁻³), `n` = 0,015, 5 m³/s apportés au premier, trois heures au
pas d'une seconde.

| | référence | mesuré |
|---|---|---|
| la profondeur des dix biefs | la hauteur normale `y_n` = 0,706106 m (bissection sur Manning ; Froude 0,54) | **0,70611 m**, les dix |
| le débit de chaque arête, sur la dernière minute | 5 m³/s | **5,00000 m³/s**, les dix |
| le bilan | volume = reçu − sorti | **exact au millilitre** |
| l'instantané de V | la loi acceptée, restaurée au bit, la suite identique | tenu |
| la suite entière | inchangée | 766 essais |

Critères (écrits avant) : (1)–(4) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

V transporte l'eau d'un canal comme un canal la transporte : le régime uniforme s'installe à la hauteur que Manning prédit. Manquent pour
2.5 : les ouvrages (écluses, vannes de canal — la commande existe), le remous (un barrage aval : la courbe de remous contre une
intégration de la ligne d'eau), le régime torrentiel et le ressaut, la surface du canal dans le rendu.
