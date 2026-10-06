# L'échantillon de traversabilité et le prochain franchissement — S570 (liste 7.7)

*S570, 2026-10-06, en autonomie.* 7.7 était absent ; ADR-018 et SPEC-006 §5 le spécifient : le système d'eau ne touche pas au maillage
de navigation, il publie un signal par tuiles.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s570 -- --nocapture` ; suite du cœur : 743.

## 1. Ce qui est construit

`traversabilite.rs` : `echantillon(profondeur, courant)` — **le produit de danger `HR = d·(v + 0,5)`** (ADR-018 §3) et sa classe, la classe
de profondeur d'un humanoïde (§2), bornes basses incluses (« 50 cm à 2 m/s, HR = 1,25, emporte déjà un adulte ») ;
`prochain_franchissement(profondeur(t), t₀, horizon, pas, cause)` — le délai avant que la profondeur franchisse un seuil, le seuil, le sens,
et la cause supposée (SPEC-006 §5.4), par un balayage puis une bissection à la milliseconde. Le courant est celui de surface, fourni par
l'appelant (SPEC-006 §5.5).

## 2. Mesuré (références écrites au plan par son script)

| cas | référence | mesuré |
|---|---|---|
| 0,5 m à 2 m/s | HR 1,25, pour la plupart | 1,25, pour la plupart |
| 0,3 m à 0,5 m/s | 0,30, faible | 0,30, faible |
| 1,0 m à 1,0 m/s | 1,50, pour la plupart | 1,50, pour la plupart |
| 1,2 m à 2,0 m/s | 3,00, pour tous | 3,00, pour tous |
| 1,5 m immobile (la borne) | 0,75, pour certains | pour certains |
| marée M2 (`0,8 + 0,4·sin`, 12,42 h), depuis la mi-marée montante | 1,0 m dans 3 726,000 s, en montant | **3 725,9999 s**, +1 |
| depuis la pleine mer | 1,0 m dans 7 452,000 s, en descendant | **7 452,0003 s**, −1 |
| depuis la mi-marée descendante | 0,5 m dans 6 034,925 s, en descendant | **6 034,9251 s**, −1 |
| une eau étale | aucun franchissement | `None` |

Critères (écrits avant) : (1) `HR` à 10⁻⁶ et les classes, aux bornes — **tenus** ; (2) les délais à 10 ms, le seuil, le sens — **tenus**
(0,3 ms au plus) ; (3) les refus — **tenus**. **En route** : une assertion ajoutée hors du plan dans l'essai était fausse (la borne de
1,5 m attribuée à la mauvaise classe) — corrigée ; le module avait raison.

## 3. Ce que cela dit — et ne dit pas

Le signal de traversabilité existe en un point : ce qu'un PNJ doit savoir (puis-je passer, est-ce dangereux, quand ce gué se ferme-t-il)
se calcule et se prédit. Manquent pour 7.7 : les tuiles et leur publication (SPEC-006 §5.2 : 16 × 16 cellules, cadence et séquence par
tuile, sous-cellule d'ADR-006), les événements de franchissement et leur invalidation par une commande de V (§5.4), la glace porteuse (sa
charge `P ∝ h²`), la température, le gué d'un véhicule et le tirant d'eau d'un bateau (ADR-018 §2), et la marée de B elle-même (2.2).
