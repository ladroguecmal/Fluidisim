# Les sommets de déferlement le long d'un faisceau de rayons — S633 (liste 3.5)

*S633, 2026-10-07, en autonomie (ADR-247 : la physique des partiels).* S632 trouvait le point de déferlement le long d'un rayon ; ici, chaque
point porte ce que SPEC-006 §6 demande à un sommet — le flux dissipé et la direction de crête —, dans l'ordre du faisceau : la polyligne
chaînée d'une côte quelconque.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s633 -- --nocapture` ; suite du cœur : 818 essais listés.

## 1. Ce qui est construit

`deferlement::sommets_sur_rayons` : pour chaque rayon (son voisin : le suivant, ou le précédent pour le dernier), le point de S632, le flux
`ρ·g·H²/8·c_g` en kW/m (`H` = 0,78·h au point), la direction de crête (θ du rayon, interpolé) ; les sommets dans l'ordre du faisceau.

## 2. Mesuré (références calculées au plan par `s633_ref.py`, numpy)

| | référence | mesuré |
|---|---|---|
| faisceau de cinq rayons, côte droite, pas de ½ s | cinq sommets dans l'ordre | idem |
| premier sommet : x ; flux ; direction | 112,600476 m ; 16,973662 kW/m ; (−0,9940883 ; 0,1085748) | à 10⁻¹² |
| contre l'analytique (flux 16,973288 kW/m, direction (−0,9940882 ; 0,1085754)) | à 10⁻⁴ ; à 10⁻⁵ | 2,2·10⁻⁵ ; 6·10⁻⁷ |
| les cinq positions sur la côte droite | à 10⁻⁹ m l'une de l'autre | tenu |
| refus : un seul rayon | | tenu |

Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

La polyligne de déferlement d'une côte quelconque peut désormais naître des rayons : chaque sommet sait où la houle casse, combien
d'énergie elle y perd, et dans quelle direction elle court — à 2·10⁻⁵ de l'analytique là où celle-ci existe.

Manquent : les caustiques (des rayons qui se croisent), un faisceau qui se replie, la largeur de la zone, une polyligne par phase de marée,
la publication.
