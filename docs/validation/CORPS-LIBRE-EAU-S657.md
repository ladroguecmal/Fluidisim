# Le corps libre contre l'eau autour de lui — S657 (liste 6.7) ; A334 levée

*S657, 2026-10-07, en autonomie, vers la v2.* C'est le premier plan écrit avec le bloc « Contrôles du plan » (ADR-266). En S655, le corps
libre semblait dépasser l'eau, mais il était comparé à une sonde restée à sa position de départ.

## Reproduire

- Le lecteur, dans la suite : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core the_water_around_reader -- --nocapture`.
- Le corps sous le rouleau : `… the_free_body_against_the_water_around -- --ignored --nocapture`. Les pas de 10 et 5 ms ont tourné en
  parallèle, sur deux copies du binaire (≈ 8 et 18 min).

## 1. Le lecteur, éprouvé d'abord (ADR-263 D2)

**La vitesse de l'eau autour du corps** : le plus grand module de la vitesse, aux centres des mailles d'eau voisines d'une maille du corps.
Il n'est lu que si quatre mailles d'eau au moins touchent le corps.

Cas connu : une sphère imposée à 0,5 m/s en eau au repos. Le lecteur lit **0,466 m/s** sur 48 mailles, dans la plage de 0,3 à 0,7 m/s
écrite avant.

## 2. Mesuré

| pas | le corps au plus vite | l'eau autour, au même instant | rapport | rapport au plus, sur toute la course | masse |
|---|---|---|---|---|---|
| 10 ms | 1,870 m/s à 3,631 s | 2,078 m/s | **0,90** | 0,97 | au bit |
| 5 ms | 2,317 m/s à 3,523 s | 2,536 m/s | **0,91** | 0,98 | au bit |

**Critères, écrits avant.** (1) à (3) **tenus**. Le corps ne va jamais plus vite que l'eau qui le porte.

## 3. Le verdict du témoin

Deux causes étaient candidates à l'excès de S655 : (a) le comparant mal placé, (b) un reste du couplage. Le comparant local supprime (a),
et l'excès disparaît : **c'était (a)**. Le couplage à masse ajoutée implicite de S655 est juste.

**A334 est levée**, avec trois résultats :

- la force lissée ne dépend pas du pas (S654) ;
- le couplage du corps libre, stabilisé par la masse ajoutée, porte le corps à la vitesse de l'eau, sans la dépasser ;
- seul le pic instantané d'une force sur un corps fixe reste une grandeur à ne pas lire seule : il ne se rapporte que lissé.

## 4. Ce que cela dit

Le rouleau d'APIC 3D **emporte un corps libre** qui flotte (densité 500). Il le soulève et le porte de deux mètres vers la plage, à la
vitesse de l'eau qui l'entoure, sans la dépasser. La masse d'eau est exacte, et le résultat se répète d'un pas de temps à l'autre (1,87 et
2,32 m/s, chacun à 0,9 fois l'eau). C'est ce que 6.7 demandait du rouleau : un nageur emporté.

Manquent : la rotation du corps, un corps plus léger que sa masse ajoutée, la poche d'air qui pousse un acteur.
