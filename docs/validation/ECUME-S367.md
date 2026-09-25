# Le champ d'écume de B : la référence — S367

2026-09-25. Liste **7.1** (écume et moutons — C14, banc B9, champ d'écume de SPEC-006 §4), *absente* jusqu'ici, conçue
par [ADR-014](../adr/ADR-014-mousse-spray-bulles.md). Session de physique de l'alternance d'ADR-191 D3, choisie parce
que le verdict *« Je valide les rendue sauf ecume »* ([revue](REVUE-VISUELLE.md) §26) refuse l'écume rendue : sans
mémoire, elle ne montrait que des taches instantanées au bord lisse. Machine de référence ; aucune carte graphique.

## Reproduire

- Commit de P6 de S367 ou plus récent, dans `code/`.
- `cargo test -p water-core --release --offline s367 -- --nocapture` — quatre essais, lignes `S367`, instantané :
  décroissance, pire 7,69·10⁻⁷ ; advection, écart du centre 9,39·10⁻⁶ m, sommet 0,8632 ; seuil sur une onde, 0 / 0,5 / 1 ;
  déterminisme, hash `a55bac3be0ee8125`.
- `cargo run -p water-core --release --offline --example ecume_couverture -- sigma` — `σ_a/g` = 0,0836 aux trois vents ;
  `-- 0.45` — couverture nulle (deux minutes) ; `-- kappa 2.0 2.5 3.0` — la table du §2 (trois minutes) ; `-- grand 13
  11`, `-- grand 10 11`, `-- grand 7 11` — rapports 0,94, 1,06, 1,02 à Monahan (cinq minutes chacun).
- `cargo run -p water-core --release --offline --example ecume_trainees -- 0.2287 periodique` — lignes `TRAINEES_S367`,
  trois minutes : au texel de 0,5 m, Ly/Lx 5,33 advecté, 1,81 témoin.

## En une phrase

L'écume a désormais une mémoire : un champ à deux canaux — l'écume vive qui s'éteint en trois secondes et nourrit un
film résiduel de trente — que la vitesse orbitale de B étire en traînées le long du vent, et dont la couverture suit
Monahan à 6 % près de 7 à 13 m/s ; le seuil physique d'ADR-014, lui, ne fait jamais déferler la mer de B.

## 1. Ce qui est construit

`code/water-core/src/ecume.rs`, `ChampEcume` : grille ancrée au monde, deux canaux ; un pas = **advection**
semi-lagrangienne bilinéaire par la vitesse de surface de B au point d'arrivée, **décroissance** intégrée exactement
(`dFa/dt = −λa·Fa`, `dFr/dt = λa·Fa − λr·Fr` : tout ce que l'actif perd devient résiduel), **sources** : `Fa` monte à
l'indicateur de déferlement, montée lisse de l'accélération verticale descendante `−a_z/g` — la dérivée seconde analytique
de B — autour d'un seuil. Pour une onde seule, 0,45 g équivaut à `a·k = 0,45`, la cambrure de Stokes : le second critère
d'ADR-014 §3.1 est contenu dans le premier.

| critère 1, écrit avant | mesure |
|---|---|
| décroissance : le système fermé, 10⁻⁶ | **7,7·10⁻⁷** à 10 s, quarante pas |
| advection d'une gaussienne par une vitesse uniforme, cent pas | centre à **9,4·10⁻⁶ m**, masse à 1,8·10⁻⁶ ; sommet 1 → 0,863 : la diffusion de l'interpolation |
| le seuil sur une onde seule | indicateur **0 / 0,5 / 1** à `a·k` = 0,30 / 0,45 / 0,60 |
| déterminisme | deux passes, même hash |

## 2. La couverture contre Monahan (B9, scénario 1)

Mer de vent pleinement développée (Pierson–Moskowitz, étalement cos^2s, s = 10), 64 composantes, champ d'un mètre ;
couverture = part où l'écume **active** dépasse ½ ; Monahan et O'Muircheartaigh (1980), `W = 3,84·10⁻⁶·U10^3,41`.
**Prédiction, écrite avant** : au seuil physique, l'ordre de grandeur, pas mieux qu'un facteur 3.

**Manquée.** À 0,45 g : **aucun déferlement**, à 7, 10 et 13 m/s. La cause, mesurée : la bande de B est
**autosimilaire** — elle suit le pic, rapport 7,74 entre ses fréquences extrêmes — et l'écart-type de l'accélération
verticale vaut **0,0836 g à tout vent**. 0,45 g est à 5,4 écarts-types ; et **aucun seuil fixe ne peut suivre
`U10^3,41`** : sur cette bande, les crêtes ont les mêmes statistiques à tout vent. Ce qui fait croître le déferlement
avec le vent — la queue non résolue, l'apport du vent — n'est pas dans la bande.

**Décision** : la **place** du déferlement vient de la physique — les crêtes les plus accélérées ; sa **quantité**, de
l'observation — Monahan. Seuil `κ·σ_a` ; couverture active au régime (10 s de mise en régime, 20 s de mesure, champ de
128 m) :

| κ | 7 m/s | 10 m/s | 13 m/s |
|---:|---:|---:|---:|
| 2,0 | 10,0 % | 6,8 % | 10,2 % |
| 2,5 | 2,46 % | 1,30 % | 3,01 % |
| 3,0 | 0,43 % | 0,16 % | 0,57 % |

Même courbe aux trois vents, au bruit de réalisation près ; mise en commun : `C(κ) ≈ 2,26 %·exp(−3,52·(κ − 2,5))`,
inversée par `seuil_pour_couverture(fond, w)`. **Vérifiée sur des mers d'une autre graine** (critère : un facteur
1,5) :

| champ | 7 m/s | 10 m/s | 13 m/s |
|---|---:|---:|---:|
| 128 m, 20 s | 1,06 | 1,50 | **0,51** |
| **384 m, 40 s** | **1,02** | **1,06** | **0,94** |

Le 0,51 venait du champ : à 13 m/s, 128 m ne tiennent qu'une longueur d'onde de pic.

## 3. Les traînées (B9)

ADR-014 §2.1 : advectée par la vitesse orbitale **complète**, l'écume forme d'elle-même des traînées alignées au vent.
**Critère** : le rapport des longueurs de corrélation du résiduel, le long du vent et en travers, contre le témoin sans
advection (B n'a ni courant ni vent). U10 = 10 m/s, seuil calé, 90 s.

Premier passage : 3,8 contre 1,8 — mais trois artefacts possibles : un **bord d'entrée vide** (le champ fixe perd son
écume vers l'aval sans en recevoir), un résiduel presque **saturé**, la **diffusion** de l'interpolation dans le sens de
l'oscillation. Refait en **domaine périodique** (pour la mesure seulement) et à deux résolutions :

| texel | advection | Lx (en travers) | Ly (le long) | Ly/Lx |
|---|---|---:|---:|---:|
| 1 m | orbitale | 16,9 m | ≥ 64 m (butée) | ≥ 3,8 |
| 1 m | aucune | 2,9 m | 5,1 m | 1,8 |
| 0,5 m | orbitale | **10,9 m** | **58,3 m** | **5,3** |
| 0,5 m | aucune | 2,8 m | 5,1 m | 1,8 |

L'allongement **croît** quand le texel diminue : ce n'est pas la diffusion ; il demeure sans bord d'entrée. **Tenu** :
la vitesse orbitale seule étire l'écume résiduelle le long des vagues. Le mécanisme — dérive de Stokes, dispersion
relative des particules, déferlement qui suit les groupes — n'est pas décomposé.

## 4. Limites

- Référence en f32 sur CPU ; la **production** — cascades RG16F sur la carte, SPEC-006 §4 — et le **rendu** (8.4) restent
  à faire.
- Sources de B seules : ni W (déferlement côtier, sillages), ni δ, ni le vent (embruns) ; la dépendance au vent passe par
  Monahan, pas par la physique.
- Le transfert 1 : 1 de l'actif au résiduel fait un résiduel dense (0,40 en moyenne) : demi-vies et transfert à caler
  (B9, ADR-014 §7.2).
- Couverture vérifiée sur une mer pleinement développée seulement, 64 composantes ; une autre bande changera `C(κ)`.
- Ly bute sur la demi-largeur du domaine : le rapport est une borne basse.
