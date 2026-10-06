# C15 : la glace d'un lac dans V — S575 (listes 7.6, 13.2)

*S575, 2026-10-06, en autonomie.* C15 (non exécuté) : « lac abrité, `FDD` imposé, 30 jours ; épaisseur à ± 10 % de `0,035·√FDD` ;
aucune plaque tant que `Hs > 0,15 m` ; masse conservée sur un cycle gel/dégel complet ». ADR-203 D6 : le gel des contenants passe par V.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s575 -- --nocapture` ; suite du cœur : 750.

## 1. Ce qui est construit

La glace d'un plan d'eau est **une couche des liquides de V** (ADR-241), de densité 917, au-dessus de l'eau : sa flottaison est
l'hydrostatique des couches. `glace::ajuster_glace(nœud, composition, eau, glace, aire, h_visée)` gèle ou fond **par quanta exacts** —
917 ml d'eau deviennent 1 000 ml de glace (`917·1000 = 1000·917` : la masse à l'entier), le nœud gagnant 83 ml par quantum ; refus si la
capacité manque. L'épaisseur visée vient de Stefan (S574), calculée depuis **le gel cumulé**.

## 2. Mesuré (références écrites au plan par son script)

Un lac de 10 × 10 m, 2 m d'eau, 30 jours à 10 K de gel (300 K·jour), au pas d'une heure.

| | référence | mesuré |
|---|---|---|
| l'épaisseur après 30 jours | Stefan 0,610219 m ; C15 `0,035·√300` = 0,606218 m ± 10 % | **0,610210 m** (un quantum = 10 µm ; +0,66 % de C15) |
| les quanta gelés | 61 021 | **61 021** ; l'eau 144 043 743 ml, le nœud 205 064 743 ml |
| la masse `1000·V_eau + 917·V_glace` | constante à chaque pas | **exacte à chaque pas** |
| le dégel sur dix jours | l'eau rendue au millilitre | **200 000 000 ml, 0 de glace** |
| sous `Hs` = 0,2 m, 30 jours | aucune glace | **aucune** (le mécanisme existe : l'assertion n'est plus vide, note S29 de C15) |

Critères (écrits avant) : (1) ± 10 % et à un quantum de Stefan — **tenus après correction** ; (2), (3), (4) — **tenus**.

**La première mesure a manqué « à un quantum de Stefan » de 2,4 mm** : le pilote de l'essai repartait chaque heure de l'épaisseur
**quantifiée**, et la troncature (jusqu'à un quantum par pas) s'accumulait sur 720 pas. L'état exact est le gel cumulé, comme le report de
reste des arêtes de V (ADR-010 §4) ; corrigé dans le pilote, le critère inchangé.

## 3. Ce que cela dit — et ne dit pas

Un lac gèle sans créer ni perdre une goutte, et sa glace flotte par la même hydrostatique que l'huile de S559. **C15 passe** sur V. Ne
disent rien : un dégel physique (ici l'épaisseur visée décroît à la main ; le bilan d'énergie de la fonte reste à écrire), la glace dans
B (une mer qui gèle) et dans δ, le gel borné aux plans d'eau gelables, le rendu.
