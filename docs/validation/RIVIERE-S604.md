# L'éditeur de rivières, son cœur — S604 (liste 12.2 ; SPEC-005 §5)

*S604, 2026-10-07, en autonomie (ADR-247).* 12.2 était absent : « éditeur de rivières : dessin, validation bloquante, gravure ». Construit :
le cœur de l'éditeur, sans son interface — ce que l'outil affiche en continu (§5.2), ce qu'il refuse (§5.3), ce qu'il grave.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s604 -- --nocapture` ; suite du cœur : 795.

## 1. Ce qui est construit

Un module `riviere.rs` : un réseau de biefs (la ligne d'eau par sommets, une largeur, un débit, un `n` de Manning ; des nœuds source,
confluence, lac, mer) ; `Bief::profil` — par segment, la pente de la ligne d'eau, la hauteur normale de Manning (section rectangulaire),
`v`, `Fr` ; `ressauts` ; `valider` — la ligne d'eau descend strictement (aussi à travers un nœud), `ΣQ` se conserve à chaque confluence, `v`
dans une plage plausible (0,1–3 m/s par défaut, réglable), un lac a un exutoire ; `Bief::graver` — le fond au milieu de chaque segment et les
conflits au-delà d'un seuil de creusement.

## 2. Mesuré (références calculées au plan par une bissection indépendante)

| | référence | mesuré |
|---|---|---|
| bief A (30 m³/s, 20 m, n = 0,035, pente 5·10⁻⁴) : `h` | 1,781932256 m | 1,78193225550 m |
| le réseau juste (A, B → confluence → C → lac → D → mer) | aucun défaut | aucun |
| un sommet tapé 69,5 au lieu de 20,0 (chute ×100) | vitesse, 3,519132 m/s | `Vitesse` (A, 0), 3,5191320786 |
| un sommet qui remonte ; un sommet plat | remonte (A, 0) | idem, seul défaut |
| C parti de 19,1 m | le nœud remonte | `NoeudRemonte` (C, nœud 2) |
| B à 11 m³/s | confluence, écart 1 m³/s | `Confluence` (2, −1) |
| le lac sans exutoire | un défaut | `LacSansExutoire` (3) |
| la faute ÷100 (chute de 5 mm) | aucun défaut | aucun |
| bief E, raide puis doux : `Fr` | 1,176452 ; 0,145674 ; un ressaut | idem ; ressaut après le segment 0 |
| gravure de C, terrain à 19,5 / 17,7 m | creusements 2,582 et 3,082 / 0,782 et 1,282 m ; conflits {0, 1} / {1} | au 10⁻⁹ m ; idem |
| refus | largeur, débit, `n`, sommets, nœud | tenu |

Critères (écrits avant) : (1)–(6) — **tenus**.

## 3. Ce que cela dit — et ne dit pas

Chaque faute de saisie isolée produit un défaut et un seul, au bon endroit : l'auteur sait quoi corriger. **La règle de vitesse
n'attrape qu'un sens** : `v ∝ S^0,3`, une chute cent fois trop forte quadruple la vitesse (bloquée), une chute cent fois trop faible la
divise par cinq (0,18 m/s, dans la plage) — la règle de SPEC-005 §5.3 ne suffit pas à elle seule contre l'erreur d'unité qu'elle vise ;
une alerte sur une pente hors d'une plage, ou sur `Fr` très faible, la compléterait (proposé, non construit).

Manquent : l'interface et la colorisation, la spline, `largeur(s)`, `section_type(s)`, `debit(t)`, la cinquième règle (les régions de
niveau marin pavent la planète), la gravure dans une carte de hauteurs.
