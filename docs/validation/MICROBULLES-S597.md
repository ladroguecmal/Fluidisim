# Le nuage de microbulles — S597 (liste 7.3)

*S597, 2026-10-07, en autonomie (ADR-247).* 7.3 (les microbulles visuelles) était absent.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s597 -- --nocapture` ; suite du cœur : 772.

## 1. Ce qui est construit

`microbulles.rs` : un **nuage** laissé par un déferlement — des classes de diamètre, `N` bulles par m², réparties sur une profondeur ;
chaque classe remonte à sa vitesse terminale (Tomiyama, S540) ; la fraction restante `max(0, 1 − v·t/D)` ; l'**épaisseur optique**
`τ = Σ N·2·π·r²·fraction` ; l'**opacité** `1 − e^(−τ)`, ce que le rendu blanchit.

## 2. Mesuré (références écrites au plan par son script ; la taille d'ensemble vérifiée, ADR-250 D1)

Trois classes (50, 100, 200 µm), 1 m de nuage ; 10 000 bulles par classe intégrées une à une (`Bulle::pas`, le transitoire et la traînée
implicite), aux instants 10, 30, 60, 120 s.

| | référence | mesuré |
|---|---|---|
| la fraction submergée de chaque classe, 12 mesures | l'analytique, à 0,05 (l'écart-type de l'ensemble ≤ 0,005) | **2·10⁻⁴** au pire |
| les vitesses de remontée (Tomiyama) | — | 1,33 ; 4,98 ; 16,28 mm/s |
| `τ(0)` | `Σ N·2·π·r²` | à 10⁻¹² |
| l'opacité | décroissante | 0,165 → 0,063 à 120 s → 0 à 1 000 s |
| classes vides, profondeur nulle, diamètre nul | refus | tenu |

Critères (écrits avant) : (1)–(3) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

L'eau blanche d'un déferlement s'éclaircit comme ses bulles remontent — les grosses en une minute, les fines en dix — et le rendu reçoit
son opacité. Manquent pour 7.3 : la dissolution des bulles, l'émission par le déferlement (3.5) et les impacts, la turbulence qui retient
les plus fines, le rendu et son niveau de détail (8.4).
