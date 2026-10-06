# La coque qui perce la surface, en mouvement sur la carte — S504 (liste 6.4)

*S504, 2026-10-06, en autonomie.* La suite de [MOBILE-CARTE-S503](MOBILE-CARTE-S503.md) (un solide immergé qui bouge sur la carte). Une
coque qui perce le couvercle demande deux choses de plus à la carte, que la référence fait depuis S334 : la pression d'un couvercle en
partie couvert retire le dépôt du pas (le flux de paroi l'emporte pendant ce pas), et l'eau de surface d'une colonne dont le couvercle se
referme passe aux voisines.

## Reproduire

- `MODE=pilonnement|roulis water-viewer --lineaire-coque` (`CYCLES`, `SANS_TRANSFERT` pour le témoin) — lignes `COQUE_S504`, ≈ 10 s.
- `water-viewer --lineaire-carte --solide --cycles=32`, `CLOISON_E=0.05|0.075 water-viewer --lineaire-cloison`,
  `water-viewer --lineaire-mobile` : S358, S493, S503.

## 1. La construction

- **Le cœur** : `Volume3::lid_transfer_weights` — par colonne, le rapport de fermeture `1 − après/avant` et les quatre parts du transfert,
  celles du cœur.
- **La carte** : `lid_partial` retire le dépôt du pas ; `motion_gather` rassemble ce que chaque colonne reçoit de ses voisines (sans
  écriture concurrente), `motion_apply` l'applique en somme compensée ; dépôt et poids effacés au pas suivant sans mouvement.

## 2. Mesuré

La coque de la porte D (4 × 1,6 × 1 m à 500 kg/m³, à son tirant) dans un δ de 12 × 8 × 2 m (48 × 32 × 8 mailles de 25 cm), murs ; 300 pas
de 10 ms.

| cas | carte contre référence | élévation (rapport) | volumes | transferts |
|---|---|---|---|---|
| pilonnement 5 cm, 3,5 rad/s | **1,7·10⁻⁶ m** | 4,5 cm (27 000 ×) | 5,9·10⁻⁷ m³ | aucun (une boîte en pilonnement ne referme pas de couvercle) |
| roulis 0,05 rad, 2,5 rad/s | **2,4·10⁻⁷ m** | 8,8 mm (37 000 ×) | 2,0·10⁻⁷ m³ | 5 814 colonnes·pas |
| **témoin** : roulis, la carte sans transfert | 5,6·10⁻⁵ m, croissant | — | — | — |

S358, les cloisons de S493 et la sphère de S503 : inchangés (S358 identique au binaire d'avant). Coût par pas : recoupage CPU 7,9 à
8,2 ms, carte 0,7 à 0,9 ms.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) S358 et S493 inchangés | identiques | tenu |
| (2) pilonnement puis roulis : la carte à 10⁻⁴ m, élévation ≥ 10 × l'écart | 1,7·10⁻⁶ et 2,4·10⁻⁷ m ; 27 000 et 37 000 × | tenu |
| (3) volumes à 10⁻⁶ m³ | 5,9·10⁻⁷ ; 2,0·10⁻⁷ | tenu |
| (4) le coût, publié | 8 ms + 0,8 ms | publié |

**6.4 reste partielle** : C23 sur le système (le pas borné par la vitesse de la paroi) et le coût du recoupage, que le CPU porte entier.
