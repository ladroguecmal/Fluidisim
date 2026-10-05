# La référence parallèle et le banc de non-régression — S483

*S483, 2026-10-05, en autonomie ; [ADR-222](../adr/ADR-222-la-methode-se-revise-elle-meme.md) D2 et D3.*

## Reproduire

- `PROFIL=1 [FILS=<n>] code/target/release/examples/apic3d_bulle.exe 4 0.01 500` — le temps par étage (le cœur pose les repères,
  `Apic3::step_marked`, l'exemple lit l'horloge) et l'empreinte de l'état final (`BULLE_EMPREINTE`).
- `[FILS=16] PAS_MAX=10 code/target/release/examples/apic3d_b10.exe 2 16` — B10 à 16 mailles, dix pas.
- `python outils/non_regression.py` (`--inscrire` : réinscrire) ; `python outils/rituel.py fin …` le lance.

## 1. Où était le temps

La bulle de S479 (60 800 mailles, 381 824 particules), 20 pas, séquentiel : reconstruction **41 %**, séparation + corps + échange 19 %,
projection 15 %, transfert vers la grille 10 %, advection 10 %, transfert vers les particules 5 %.

## 2. La référence parallèle, au bit

`Apic3::set_jobs` reçoit un système de tâches de l'hôte (`ScopedJobs` aux bancs, `None` par défaut). Seules des **écritures disjointes**
(`parallel_fill_f32`, S243) : chaque élément écrit une fois, depuis des lectures seules — le résultat ne dépend ni du grain ni du nombre
de fils. Les triplets passent par `as_flattened_mut` (bibliothèque standard, sans `unsafe`). Les produits scalaires gardent leur ordre.

| étage | forme | séquentiel → 16 fils (20 pas) |
|---|---|---|
| reconstruction | une valeur par maille (`reconstruct_cell`) | 14,1 → 2,2 s |
| séparation | **collecte** : chaque particule somme ses voisines (mailles en z, y, x ; ordre du tri) — l'ordre de `separate_shift` de la carte | 6,5 → 1,6 s |
| transfert vers la grille | **collecte** : chaque face somme les particules des mailles qui la touchent — l'ordre de `p2g` de la carte ; le poids du seul nœud, axe par axe ; le tri se fait là, la reconstruction le reprend | 3,4 → 2,8 s |
| advection, transfert vers les particules | une écriture par particule | 3,5 → 0,6 s ; 1,8 → 0,7 s |
| projection | `A·d` par maille ; le reste séquentiel | 5,1 → 3,8 s |

**Mesuré** : l'empreinte de la bulle est **la même avec 0, 1, 4, 8, 12 et 16 fils** ; les 43 essais d'APIC 3D passent ; le banc carte |
référence des poches est inchangé (les collectes reprennent l'ordre de la carte). **La bulle : 34,6 → 11,9 s (× 2,9) ; B10 à 16 mailles,
dix pas : 81 → 25 s (× 3,2).**

**Critère (2) manqué** — ×4 était demandé. Ce qui reste : la projection (les produits scalaires ordonnés, les mises à jour ; `ScopedJobs`
recrée ses fils à chaque appel, environ 150 fois par pas). Un pool de fils persistant demanderait d'effacer une durée de vie (du code
`unsafe`, que le harnais n'a pas) : c'est la suite, à peser. **Le séquentiel ralentit** (34,6 → 47,7 s) : les collectes font plus de
travail que la dispersion et l'accumulation par paires — le prix d'un résultat indépendant du nombre de fils ; les bancs prennent `FILS=16`.

## 3. Le banc de non-régression

`outils/non_regression.py`, contre [EMPREINTES](EMPREINTES.md) (versionné) : (1) la bulle, 4 pas, empreinte au bit et égale avec 1 et
16 fils ; (2) `--apic3d-poches CAS=plusieurs PAS=2` : aucun écart de détection, volume et pression à 10⁻⁴ ; (3) `--v1` 5 s : masse
exacte, trajectoire inscrite (pas, colonnes, particules), pas médian sous 1,3 fois l'inscrit. **124 s.** Passe sur l'état présent ;
**échoue** sur une empreinte modifiée (« dc06f28c8a909e04 au lieu de 0000000000000000 »). `rituel.py fin` le lance et s'arrête s'il
échoue (`--sans-banc "raison"` pour le sauter, dit au journal).

| critère (écrit avant) | | |
|---|---|---|
| (1) au bit, 1 contre 16 fils ; essais d'APIC 3D | identique ; 43 passent | tenu |
| (2) la bulle au moins 4 fois plus vite | × 2,9 (B10 : × 3,2) | **manqué** |
| (3) banc en moins de 3 min, échoue sur une empreinte modifiée, appelé par le rituel | 124 s ; oui ; oui | tenu |
