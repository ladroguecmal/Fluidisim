# L'angle de bande d'une barge instable — S550 (liste 6.6)

*S550, 2026-10-06, en autonomie.* La stabilité aux grands angles : une barge dont la hauteur métacentrique devient négative sous une
charge haute ne reste pas droite ; elle gîte jusqu'à l'angle de bande, `tan θ = √(−2·GM/BM)` (flancs droits, pont sec, bouchain noyé) — la
frontière du chavirement.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core s550 -- --nocapture` ; suite du cœur : 710.

## 1. Le montage (aucun code neuf)

La barge de S548 (20 × 8 × 4 m, 246 t, proxy 4 × 32 × 16 : l'erreur `BM/n²` du proxy, S499, vaut 2,5 mm), une charge de 100 t à 8 m
au-dessus de la quille (une charge ponctuelle, S549), lâchée à ± 1°, amortie 300 s. **La formule d'analyse vérifiée avant la mesure** : une
intégration numérique de la carène inclinée d'une boîte redonne `GZ = sin θ·(GM + BM·tan²θ/2)` au 10⁻⁵ près, nul à 19,084° (la leçon de
S549).

## 2. Mesuré

| | mesuré | attendu |
|---|---|---|
| `GM` (346 t, `T` 2,110 m, `KB` 1,055, `BM` 2,528, `KG` 3,734) | — | −0,151 m |
| gîte d'équilibre, lâchée à +1° / −1° | **+19,310° / −19,310°** | ± 19,084° (écart 1,3 % en tangente) |
| sans la charge | 0,0000° | 0 |

L'écart de 1,3 % : l'erreur propre du proxy sur la hauteur métacentrique (2,5 mm sur 0,151 m, soit 0,8 % en tangente, prévu au plan) et sa
non-linéarité aux grands angles.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la gîte à 3 % en tangente, des deux côtés | 1,3 % | tenu |
| (2) sans charge, droite à 0,1° | 0,0000° | tenu |

## 4. Ce qui manque

Le corps rigide trouve l'équilibre d'une coque instable aux grands angles. **6.6 avance.** Manquent le chavirement au-delà du pont mouillé
(le proxy le porte, la formule non : une référence de la courbe GZ entière), l'eau d'un compartiment pendant la gîte (S548 et S549
ensemble), la poche d'air porteuse, et la dynamique d'un chavirement sous une vague.
