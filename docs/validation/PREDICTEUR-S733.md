# Le prédicteur du sélecteur — S733 (liste 4.14 ; SELECTEUR-DOMAINES-S732, P1)

*S733, 2026-10-09, en autonomie.* **La question** : SGN, calculé en avance depuis l'état de départ, prévoit-il où et quand la vague de R43
se retourne, et où retombe son jet ?

## Ce qui est fait

- **SGN sur un fond doux** (`Serre1D::nouveau_fond`) :
  - la surface reconstruite, la hauteur lue sous elle aux faces ;
  - la source du fond centrée sur les hauteurs des faces, qui équilibre exactement un lac au repos ;
  - le terme dispersif en `g·η_xx` (l'approximation de pente douce).

  Sans fond, au bit (S694 inchangé).
- **Le prédicteur** (`selecteur.rs`, `prevoir`) : SGN avancé sur une copie, trois critères de déclenchement publiés évalués à chaque pas :
  - Kennedy, `η_t > α·√(g·h)` ;
  - la hauteur, `(η − η₀)/(η₀ − z) > γ` ;
  - Froude, `|u|/√(g·h) > φ`.

  Pour chacun, le premier instant et le lieu, `h_b`, `H_b`, `L_jet`. Il rend aussi la trajectoire de la crête.
- `outils/crete_film.py` : la crête d'un film de la 3D (une lecture ponctuelle, rapportée).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s733 -- --nocapture` (≈ 70 s ; quatre essais).
- `python outils/crete_film.py calculs/s730_tout3d_12.bin` (depuis `outils/`).

## Mesuré

**E1 — le fond doux.**

| | mesuré | critère |
|---|---|---|
| (1) le lac au repos sur la plage de R43, 2 s | la vitesse **4,7·10⁻¹⁵ m/s**, la masse 0 | 10⁻¹² m/s |
| (2) la levée, 1:50, de 0,5 à 0,2 m (Saint-Venant) | **1,2292** contre Green 1,2574 (−2,2 %) ; 1,2514 une fois l'usure du schéma retirée (−0,5 %) | 5 % |

Pour (2), on a pris une bosse gaussienne de σ = 2 m à la place de l'onde longue du plan : celle-ci aurait mesuré 31 m, plus que la pente.
Une bosse de σ = 0,5 m était usée de 20 % par l'écrêtage du limiteur. SGN donne 1,134 : la bosse se disperse (rapporté).

**E2 — la prévision de R43**, contre le témoin de S730 E2 (le retournement à 2,620 s et 9,938 m ; l'air à 10,375 m) :

| critère | déclenchement | contre le retournement | le jet prévu, `x_b(témoin) + L_jet` |
|---|---|---|---|
| Kennedy, α = 0,65 | 2,729 s, 10,463 m | 0,11 s trop tard, 0,52 m trop loin | 10,244 m |
| la hauteur, γ = 0,8 | 2,339 s, 9,312 m | 0,28 s trop tôt, 0,63 m en amont | 10,280 m |
| Froude, φ = 0,8 | 2,970 s, 10,788 m | 0,35 s trop tard, 0,85 m trop loin | 10,239 m |

- (4) **manqué** : aucun des seuils publiés ne tombe à 0,3 m du retournement avec une avance de 0 à 0,3 s.
- (5) **tenu pour les trois** : le jet prévu tombe à 0,10–0,14 m de l'air du témoin.
- (6) **manqué** : 277 ms pour 3 s à 2,5 cm.
- Rapporté, hors critère : Kennedy α = 0,35 (2,404 s ; 9,812 m) et la hauteur γ = 1,0 (2,523 s ; 9,738 m) tomberaient dans la fenêtre. Mais
  un seuil choisi sur la scène qui le juge ne juge rien (ADR-248 D2).

**(3) La crête, rapportée** :
- même place que le témoin : 9,113 m contre 9,106 m à 2,3 s ; 9,838 m contre 9,874 m à 2,6 s ;
- **mais bien plus basse au dernier mètre** : 168 mm contre 228 mm à 2,6 s. À 1,5 s, elles étaient à 152 et 147 mm.

**E3 — le prédicteur à 5 cm** (ajouté après E2, critères écrits avant), contre lui-même à 2,5 cm :
- la prévision de 3 s en **68 ms** (critère : 100 ms) ;
- sur neuf critères, le pire écart **0,030 s et 0,062 m** (critère : 0,05 s ; 0,1 m) ;
- la crête à 2,6 s à −2,2 % (critère : 5 %).

**Tenu.**

## Ce que cela dit

- **La place et la cinématique de la vague sont prévues** : la crête de SGN suit celle de la 3D à quelques centimètres jusqu'au
  déferlement. **Le jet aussi**, quel que soit le critère : à 0,1 m près.
- **L'instant du déclenchement ne l'est pas encore.** Les seuils publiés ont été écrits pour des modèles de Boussinesq contre des mesures.
  Ici, SGN sous-estime la levée du dernier mètre (−26 % à 2,6 s), et les seuils tombent donc ailleurs. Deux causes restent à départager :
  - l'approximation de pente douce, à 1:12 ;
  - la 3D trop haute (S713 : +40 % sur Synolakis avant le déferlement).
- **Le calibrage demande d'autres scènes** : les témoins S2 à S4 (P2 du registre). Les critères seront jugés sur toutes ensemble, et le
  seuil de Synolakis (`H/d > 0,052` à 1:12) dira d'abord s'il y aura déferlement.
- **Le coût** est réglé : à 5 cm, 68 ms pour 3 s d'avance, et la prévision ne se refait qu'à l'arrivée d'une vague.

## La suite

- **P2 du registre** : les témoins S2 (Synolakis, `H/d` = 0,3 sur 1:19,85, dont les mesures de laboratoire départagent la 3D), S3 et S4.
- Le terme de pente complet de SGN, si la pente douce est en cause.
