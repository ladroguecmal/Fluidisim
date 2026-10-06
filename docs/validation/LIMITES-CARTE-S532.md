# La carte refuse ses limites matérielles avec un nom — S532 (ADR-235 D1)

*S532, 2026-10-06, en autonomie.* La carte linéaire de δ (`Linear3`) s'est arrêtée deux fois au premier grand domaine : 71 504 groupes de
dispatch pour 65 535 (S520), une liaison de 230 Mo pour 128 Mo (S529). ADR-235 D1 : une limite atteinte se refuse avec un nom.

## Reproduire

- `water-viewer --lineaire-limites` — lignes `LIMITES_S532`.

## 1. La construction (`viewer/src/delta3d_linear.rs`)

`Linear3::new`, après l'obtention de l'adaptateur et avant toute création : les groupes des noyaux de mailles (un par 64 mailles, en une
dimension) contre `max_compute_workgroups_per_dimension` ; six tampons (vitesses, géométrie, mouvement, paires de la dispersion, état,
colonnes) contre `max_storage_buffer_binding_size` et `max_buffer_size`. Un dépassement rend `Err` en nommant la limite et la grandeur.
Les noyaux par face passent en deux dimensions depuis S520 ; l'assertion du pas reste, en défense.

## 2. Mesuré

| domaine | résultat |
|---|---|
| 512 × 512 × 17 (4 456 448 mailles) | **refusé** : « 4456448 mailles demandent 69632 groupes, au-delà de 65535 par dimension (limite de l'adaptateur) » |
| 448 × 320 × 24 (3 440 640 mailles, le domaine fin de S529) | accepté |
| bancs de la carte | sillage `e41630abd739b189`, S503 2,205·10⁻⁶ m, S504 1,669·10⁻⁶ m : inchangés |

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) au-delà refusé par une erreur nommée, sans arrêt ; en deçà accepté | les deux | tenu |
| (2) les bancs au bit | les trois | tenu |

La limite des liaisons ne se franchit pas sur cette carte avant celle des groupes (les tampons croissent comme les mailles, et les limites
de l'adaptateur sont relevées depuis S529) : son refus est écrit, pas éprouvé. Au-delà de 4,19 M mailles, il faudra des noyaux de mailles
en deux dimensions — les réductions par groupe en dépendent.
