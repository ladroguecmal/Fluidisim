# Le relais au rivage, brique 2 : la sortie à droite d'APIC 3D — S682 (liste 4.14)

*S682, 2026-10-08, en autonomie, vers la v2.* La deuxième brique du relais au rivage
([conception](../registres/RELAIS-RIVAGE-S679.md) et sa note de S680, [ADR-271](../adr/ADR-271-le-film-du-rivage-a-saint-venant.md)).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s682 -- --nocapture` (≈ 6 s).

## Ce qui a été fait

`Apic3::enable_right_outlet` s'appuie sur les bords ouverts de S446. Une particule qui franchit le bord droit n'est plus retenue par le
domaine : elle est retirée. Son volume (`dx³/8`) est compté par rangée et au total (`right_outlet`), et Saint-Venant 2D le recevra.

La sortie est éteinte par défaut. Elle est refusée avec les gouttes et la zone des colonnes, dont les tableaux par particule ne suivent
pas le retrait.

## Mesuré

**Le montage** : un bassin de 2 m × 0,1 m, l'eau à 0,4 m, maille de 5 cm ; le bord droit ouvert à 0,1 m/s pendant 1 s.

**Ce qui départage** :

- une sortie juste rend le compte exact et le flux à mieux que 10 % ;
- une particule retenue s'entasse au bord ;
- un volume compté sans retrait casse le compte.

| | critère | mesuré |
|---|---|---|
| (1) sans la sortie | au bit | les 57 essais d'APIC passent ; le banc de non-régression aussi |
| (2) le compte | exact | 5 120 = 4 864 + 256 ; le volume, 256 quanta |
| (3) le volume sorti contre le flux de la face intégré | < 10 % | **4,000 L contre 4,000 L** (le plan : 4,00 L, 256 particules) |
| (4) la dernière colonne | ≤ 8 par maille mouillée | **8,00** ; le témoin sans sortie, **20,0** (l'entassement) |

## Suite

- La brique 3 : poser des particules au bord droit pour un volume donné (le reflux qui revient de Saint-Venant), avec le réservoir du
  reste d'un quantum.
- Puis le raccord au repos sur les six plages de S678.
