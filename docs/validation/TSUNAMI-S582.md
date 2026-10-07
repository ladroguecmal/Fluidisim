# La propagation macroscopique d'un tsunami — S582 (liste 3.4)

*S582, 2026-10-07, en autonomie.* 3.4 était absent : « propagation macroscopique, puis raffinement à la côte ». ADR-001 §3.1 : un tsunami
est un objet de W — dérivé d'un événement horodaté, déterministe —, pas un très grand domaine δ.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s582 -- --nocapture` ; suite du cœur : 758.

## 1. Ce qui est construit

`tsunami.rs` : une onde longue le long d'un **rayon** (un profil de profondeur linéaire par morceaux) — le **temps de parcours**
`τ(s) = ∫ ds/√(g·h)`, exact par segment (`2L/(√g·(√h_a + √h_b))`) ; la **levée de Green** `A(s) = A₀·(h₀/h)^(1/4)` ; le **niveau**
`η(s, t) = A(s)·(1 − u²)²`, `u = (t − t₀ − τ(s))/T` — un polynôme : aucune transcendante, le même niveau sur toute plateforme (I-03).

## 2. Mesuré (références écrites au plan par son script ; la formule du segment éprouvée par Simpson)

Un rayon de 1 000 km sur 4 000 m de fond, puis une pente jusqu'à 10 m sur 100 km.

| | référence | mesuré |
|---|---|---|
| `τ` au bout du plateau | 5 048,187773 s | **5 048,187773 s** |
| `τ` à la côte | 6 009,747349 s (la pente : 961,559576 s ; Simpson, 10⁶ intervalles : identique) | **6 009,747349 s** |
| Green à 10 m | ×4,472136 | à 10⁻⁹ |
| le flux `A²·√h` en cinq points | constant | à 10⁻¹² |
| une impulsion (A₀ = 0,5 m, `T` = 600 s) à 1 050 km | le pic à `t₀ + τ` = 5 443,751 s ; 0,5942325 m | **5 444 s** (à la seconde) ; **0,5942325 m** à l'instant exact ; nul avant l'onde |

Critères (écrits avant) : (1)–(4) — **tenus**. **Avant la mesure**, une faute du plan relevée et écrite aux notes : l'amplitude d'un pic
échantillonné à la seconde porte un quantum de 1,4·10⁻⁶, au-dessus du seuil de 10⁻⁶ (ADR-236) — l'amplitude a été mesurée à l'instant
exact, le seuil inchangé.

## 3. Ce que cela dit — et ne dit pas

Un tsunami traverse un océan à la vitesse d'une onde longue et grandit en arrivant à la côte, avec l'heure d'arrivée et la hauteur que la
théorie prédit — sans solveur, au bit. Manquent : l'étalement d'une source ponctuelle et les rayons qui se courbent (la réfraction, 3.6),
la dispersion au large pour les sources courtes, le déferlement et le raffinement à la côte (δ, la plage 4.14), l'événement de W qui le
porte et sa réplication.
