# Référence δ tridimensionnelle, surface mobile — S296

2026-09-19. Porte B, lot 2 d'[ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md).
Code : `code/water-core/src/delta3d_mobile.rs` ; banc `examples/delta3d_mobile.rs` ; essais
`src/tests_delta3d.rs` et `tests/delta_runtime.rs`. Référence CPU, f32, sans dépendance.

## 1. Protocole et construction

Les quatre critères de `notes/EN-COURS.md` sont posés dans le commit `7c5180b`, avant le code.
Le schéma de [S237](SURFACE-MOBILE-S237.md) est étendu aux deux dimensions horizontales :
fonction hauteur `η(x,y)`, centres mouillés, Dirichlet fantôme vertical et latéral, advection
centrée MAC, extrapolation constante verticale, transport par débits mouillés dans x et y.
Chaque débit est calculé depuis `ηⁿ` avant toute mise à jour. Fond plat, murs fermés.

Projection : Jacobi, départ depuis la pression publiée et résidu vrai (ADR-169), portes
ADR-143/144 inchangées. La divergence des lignes franches gouverne l'acceptation ; les lignes
fantômes gardent leur diagnostic séparé. Certificat d'arrêt `γ₈` à `ny=1`, `γ₁₀` sinon : quatre
ou six faces par ligne, dérivation S295. Ce choix concerne le mode mobile ; le chemin linéaire
S295 reste inchangé. Une seule réserve nouvelle : inverse de la diagonale, 4 octets par maille,
comptés auprès de l'hôte avant scellement.

API : `set_free_surface(eta, rest)`, `step_surface_mobile(duration_us, max_iters, jobs)`,
`wet_cells()`. Durée entière, coefficients seuls arrondis en f32. La référence est hors boucle
d'image : aucun budget temporel ni production GPU revendiqués. Aucun changement de la 2D.

## 2. Identité, conservation et refus

- `ny=1`, cas S237, 32×36, amplitudes 5 et 10 cm, 1 604 pas de 1 ms : **identité au bit**
  des hauteurs, pressions, u/w et nombres d'itérations avec la 2D forcée sur Jacobi. Le témoin
  multigrille est désactivé uniquement par son contrôle de test existant.
- Opérateur à fantômes x/y/z : symétrie par produits croisés et coefficients de matrice ;
  diagonale de Jacobi confrontée à la matrice ; réduction au bit aux lignes exportées par la 2D.
- Projection d'un champ quelconque : divergence franche sous la tolérance `10⁻⁵` d'ADR-144.
- Transposition x↔y, cuves 12×7×12 et 7×12×12, 300 pas : écart de hauteur maximal
  **2,3841858·10⁻⁷ m** (un ulp autour de 2 m). Les réductions changent d'ordre ; aucune identité
  au bit n'est promise après transposition.
- Repos non aligné à 2,013 m, 50 pas : zéro itération, hauteurs inchangées, vitesses et pression nulles au bit.
- Gardes au fond et au sommet, garde après transport réellement atteinte, plafond de pression :
  refus atomiques, six champs publiés restaurés au bit, y compris le reste de hauteur.
- Allocateur compteur : **zéro allocation** sur 20 pas mobiles 3D et au refus, arène scellée.

## 3. Oracle HOS, cas limite ny=1

`cargo run -p water-core --release --offline --example delta3d_mobile -- hos`

Protocole S237/S238 : L=h=2 m, g=9,81 m/s² fourni, domaine jusqu'à 2,25 m ; HOS M=3,
Q=16, K=256, une période linéaire (1 604 pas de 1 ms), plafond 4 000 itérations. Oracle
indépendant déjà qualifié par S237 §2. Profil : maximum sur tous les points et pas / amplitude.
Harmonique : erreur maximale du coefficient cos(2kx) / maximum de l'oracle.
Tolérances inchangées : profil <2 %, harmonique <20 % à 128 colonnes, décroissance en raffinant.

| amplitude | nx | profil (%) | harmonique (%) | dérive moyenne (m) | mailles mouillées min–max | itérations max | arrêts au plancher |
|---|---:|---:|---:|---:|---:|---:|---:|
| 5 cm | 32 | 0,850428 | 2,438478 | 2,980·10⁻⁸ | 1 021–1 029 | 135 | 17 |
| 5 cm | 64 | 0,550022 | 1,409859 | 2,421·10⁻⁸ | 4 085–4 107 | 297 | 17 |
| 5 cm | 128 | **0,252893** | **0,703828** | 1,676·10⁻⁸ | 16 356–16 410 | 592 | 23 |
| 10 cm | 32 | 1,716173 | 1,712045 | 3,353·10⁻⁸ | 1 017–1 031 | 145 | 34 |
| 10 cm | 64 | 0,594157 | 0,982572 | 2,235·10⁻⁸ | 4 080–4 111 | 272 | 34 |
| 10 cm | 128 | **0,223506** | **0,427441** | 1,583·10⁻⁸ | 16 362–16 415 | 594 | 6 |

**Critères tenus** aux deux amplitudes, avec décroissance des deux erreurs à chaque raffinement.
La topologie mouillée change au cours de chaque trajectoire. Le refus historique à 5 cm du
banc S237 n'est pas reproduit : les certificats d'arrêt S238 et la porte physique S239 sont
présents dans cette référence. Aucun seuil n'a été déplacé.
En hauteur absolue au cas fin : **0,126 mm** à 5 cm et **0,224 mm** à 10 cm ; cela ne vaut
pas verdict perceptif sur un rendu.

## 4. Onde oblique

`cargo run -p water-core --release --offline --example delta3d_mobile -- oblique`

Mode (1,1) de la cuve 8×4 m, profondeur 4 m, amplitude **1 mm** (S237 §1.4 corrigé),
1 000 pas de 1 ms. Comparaison au mode linéaire S295 sur les mêmes cellules mouillées,
et à `ω²=g·k·tanh(kh)`. Phase : premier zéro de la projection du mode, interpolé entre
les deux pas voisins ; erreur `ω·t_zéro−π/2` en degrés. Erreurs de profil maximales / amplitude.

| nx × ny | mobile − linéaire (%) | contre ω (%) | erreur de phase (°) | dérive moyenne (m) |
|---|---:|---:|---:|---:|
| 16 × 8 | 0,095367 | 2,506052 | 1,226514 | 5,215·10⁻⁸ |
| 32 × 16 | 0,095367 | 0,618448 | 0,248220 | 4,377·10⁻⁸ |
| 48 × 24 | 0,095367 | **0,253045** | **0,065204** | 4,305·10⁻⁸ |

Critère mobile − linéaire <1 % tenu sur les trois grilles. L'erreur continue et l'erreur de
phase décroissent ; le cas fin tient aussi 1 % contre la solution analytique. L'écart mobile −
linéaire atteint 2 ulp de hauteur autour de 4 m (0,954 µm) ; aucune convergence de cet écart
au-delà de la précision f32 n'est revendiquée.

## 5. Domaine reçu et limites

La capacité reçue est le mouvement non linéaire d'une surface graphe en trois dimensions,
consommé par la référence de réception de la porte B (ADR-175 §4.1). Le lot suivant est le
couplage B/W et les frontières : la porte B complète n'est pas franchie.

Aucune réception de déferlement, retournement, mouillage du fond, cavité, bathymétrie variable,
parois mobiles, viscosité, rotationnel, ni longue durée. Le plancher des lignes fantômes (A274)
reste ouvert ; sa loi selon la taille et θ n'est pas établie par ces cas.

Techniques présentes : MAC 3D, Jacobi, départ chaud, réductions ordonnées f32, somme compensée
sur la hauteur. Absentes : multigrille 3D, parallélisme du calcul de champs, GPU, couplage B/W.
Les durées imprimées servent à reproduire le banc ; des tâches concurrentes ont tourné pendant
la réception, elles ne constituent donc **pas une mesure de coût de production**.

## 6. Vérifications

Suite `cargo test --workspace --release --offline` : 530 réussis (414 cœur, 18 exécution δ,
2 géométrie, 1 table radiale, 95 harnais), 18 ignorés, aucun échec. Le test supplémentaire de
symétrie coefficient par coefficient et de diagonale, ajouté ensuite, passe séparément :
**531 essais reçus au total**. Les deux modes du banc sont exécutés explicitement ; les
assertions portent sur les seuils préexistants et la décroissance, pas sur des valeurs ajustées.
L'afficheur n'est pas modifié ni revendiqué reçu de nouveau.
