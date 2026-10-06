# L'invalidation et la praticabilité par agent — S573 (liste 7.7 ; SPEC-006 §5.4 ; ADR-018 §2)

*S573, 2026-10-06, en autonomie.* Deux manques de [S572](TRAVERSABILITE-TUILES-S572.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core traversabilite -- --nocapture` ; suite du cœur : 747.

## 1. Ce qui est construit

- `invalider(tuile, cellules)` : une commande de V à l'amont fait passer la tuile en cadence `Immediat` et toutes ses prévisions à
  `CrossCause::Aucune` — « je ne sais plus », un résultat — jusqu'à une publication qui reçoit une prévision établie (SPEC-006 §5.4).
- `Agent` — `Humanoide` (sous la nage, `HR` < 1,25), `Vehicule { gue }`, `Bateau { tirant, marge }` ; `praticable(agent, échantillon)` ;
  `prochain_changement(agent, profondeur(t), …)`, par `prochain_franchissement_de`, la recherche de S570 généralisée à des seuils
  quelconques.

## 2. Mesuré (références écrites au plan par son script)

| cas | référence | mesuré |
|---|---|---|
| véhicule (gué 0,6 m), depuis la mi-marée descendante | praticable dans 3 726,000 s, en descendant | **3 725,9999 s**, −1 |
| bateau (0,9 m + 0,2 m de marge), depuis la mi-marée montante | navigable dans 6 034,925 s, en montant | **6 034,9260 s**, +1 |
| humanoïde, marée culminant à 1,2 m | aucun changement | `None` |
| bornes | véhicule à 0,6 m oui, à 0,601 non ; bateau à 1,1 m non, à 1,101 oui ; humanoïde à 0,5 m et 2 m/s non | tenues |
| invalidation | `Immediat`, 256 `Aucune` ; sans prévision, toujours `Aucune` ; avec, rétablies | 128 prévisions `Maree` → 256 `Aucune` (`Immediat`) → 256 → **128 rétablies** |

Critères (écrits avant) : (1), (2), (3), (4) — **tenus**. **En route, un défaut réel trouvé par la borne** : la praticabilité d'un bateau
calculait `profondeur − tirant − marge > 0`, la prévision comparait à `tirant + marge` ; en f32 les deux ne coïncident pas
(1,1 − 0,9 − 0,2 = 4,5·10⁻⁸) : à sa borne, la praticabilité aurait contredit la prévision. Les deux lisent désormais le même seuil.

## 3. Ce que cela dit — et ne dit pas

Un PNJ, un véhicule et un bateau reçoivent chacun leur réponse et leur prochaine échéance ; une vanne ouverte en amont efface les
promesses de la marée au lieu de les laisser mentir. Manquent pour 7.7 : la glace porteuse et sa charge (`P ∝ h²`), la température, la
marée de B (2.2), la source réelle des échantillons (B, W, V répliqués), et le danger d'un humanoïde dans un courant qui varie (la
prévision ne suit que la profondeur).
