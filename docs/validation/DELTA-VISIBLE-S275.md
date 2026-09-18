# δ visible dans l'afficheur — S275

Réception d'[ADR-168](../adr/ADR-168-premier-rendu-de-delta.md). Protocole écrit avant le code.

## Critères de fonctionnement

1. **Précalcul** : tous les pas reçus aux deux pas de temps, sans refus ; échantillons plans ;
   `η'` fini. Publier `max|η'|`, `η'` rms et leur rapport à `Hs`.
2. **Écart entre pas de temps** : `η'(16 ms) − η'(4 ms)` en rms et en maximum, en mm, et
   rapporté à `η'` rms. Comparé aux 3 mm de hauteur d'image (S201) : **mesure**, pas verdict
   visuel.
3. **Couche GPU contre CPU** : hauteur et pentes de la couche δ (fondus compris) sur des sondes,
   GPU contre évaluation CPU f64 de la même lecture Hermite : écart ≤ 0,1 mm et ≤ 10⁻⁵ en pente.
4. **Rien ne change hors `--delta`** : scène S201 et ses vérifications au bit (`--verify`).

## Revue demandée à l'utilisateur

Poses de jeu à 1280 × 720, trois variantes par pose (B seul, B+δ 4 ms, B+δ 16 ms), mêmes instants.
Questions, fixées avant de voir les images :

- la bande B+δ se distingue-t-elle de B seul, et comment (crêtes, creux, reflets) ?
- la limite de la bande est-elle visible ?
- les deux pas de temps se distinguent-ils ?

Références utiles : houle longue sans mer de vent marquée, vue d'un pont ou d'une côte à 5–20 m
de hauteur, crêtes vues de travers.

## Résultats de fonctionnement

Rejeux calculés une fois (258 s, échantillonnage de B réparti sur six fils) et relus depuis
`viewer/captures/s275/rejeu_<dt>.bin`, en-tête des constantes vérifié.

| critère | résultat |
|---|---|
| 1. précalcul | **7 500 pas à 4 ms et 1 875 à 16 ms, tous reçus**, 25 et 24 itérations au pire ; `η'` rms **15,9 mm**, max 98,7 et 105,8 mm (4,9 et 5,3 % de Hs) ; échantillons plans (essai) |
| 2. pas d'image contre 4 ms | hors éponge : **0,25 mm rms, 1,33 mm au maximum** (1,6 % de `η'` rms) ; sur tout le domaine 0,37 mm rms, **11,1 mm dans l'éponge amont** (x = −123 m, t = 30 s), que le fondu de rendu ramène sous 1 mm |
| 3. couche GPU contre CPU | 19 630 sondes dans la bande, deux poses et deux âges : **7,4·10⁻⁸ m** en hauteur, **1,2·10⁻⁸** en pente |
| 4. rien ne change hors `--delta` | sept images de la revue S254 identiques au bit (empreintes avant/après), `--verify` reçu ; essais de l'afficheur 21/21 |

Le critère 2 est une mesure : sous les 3 mm de hauteur d'image hors éponge, sans verdict visuel.
Le bord amont réagit au pas de temps dix fois plus que l'intérieur : limite de la frontière
(liste 4.7), non corrigée ici.

**Contrainte rencontrée** : un tampon de stockage de plus dépassait la limite de huit par étage ;
la bande δ suit donc les impacts dans le même tampon, sa position passée par `reflection.z`
(nulle hors `--delta`).

## Captures envoyées pour revue (R10)

`cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta --revue-delta`
(depuis `viewer/`) : 1280 × 720, champ vertical 50°, âge 20 s, rejeux relus du cache.

| pose | œil (m) | lacet, tangage | empreintes B seul / B+δ 4 ms / B+δ 16 ms |
|---|---|---|---|
| le long des crêtes | (0, −60, 6) | 0, −0,08 | `2c2c7112d88de170` / `6655e844fb685e94` / `ab9ab268355170ef` |
| face à la houle | (60, 0, 8) | −π/2, −0,1 | `a78b0282f39d0f4a` / `1d04366680e37613` / `0291b2d7d0986a73` |
| haute | (0, −200, 60) | 0, −0,35 | `184d2c9d2cfab515` / `3c28db078621434b` / `ed8f64818ea6fb07` |
| rasante | (−30, −20, 3) | 0,9, −0,03 | `ab416e1f0c7240e3` / `fc448ff856113dee` / `f82b079ef7ed1ce9` |

Couches : B (houle à crêtes longues de la scène `--delta`), queue spectrale S256 (habillage de
rugosité, directionnel, non couplé à δ), δ selon la variante. Ni impact, ni sillage, ni W.
Habillage de banc : ciel et brume S211, couleur de l'eau, pas d'écume ni de réfraction.

**Écarts de pixels mesurés avant la revue** (canal maximal, 0–255) :

| pose | B contre B+δ : pixels > 4 niveaux, maximum | 4 ms contre 16 ms : pixels > 4 niveaux, maximum |
|---|---|---|
| le long des crêtes | 16,2 %, 100 | 0 %, 2 |
| face à la houle | 10,0 %, 53 | 0 %, 2 |
| haute | 0 %, 1 | 0 %, 1 |
| rasante | 24,1 %, 92 | 0 %, 22 (pixels isolés) |

δ change l'image à hauteur d'œil et en incidence rasante, par ses pentes sur les reflets ; il
ne change presque rien vue d'en haut à travers la brume. Le pas d'image ne se distingue du pas de
4 ms en aucun pixel au-delà de 4 niveaux. Images `…_diagnostic_ecart_x10` : |B+δ − B| × 10, pour
situer l'effet ; **ce ne sont pas des rendus**. `η'` n'est pas uniforme : à 20 s, un groupe de
grosses vagues vers x = 0–32 m porte `η'` = −47 mm (ordre de la correction liée d'une crête de
1,5 m) ; d'où l'asymétrie gauche/droite des images de différence. Le bord amont grossit en fin de
rejeu (43,8 mm rms dans l'éponge), masqué par le fondu : limite de frontière (liste 4.7).

**En mouvement** : `cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta`,
touche **D** pour B seul → B+δ 4 ms → B+δ 16 ms, **Début** pour revenir à 3 s ; le rejeu couvre
0–30 s, au-delà B seul. La revue attend le verdict de l'utilisateur.
