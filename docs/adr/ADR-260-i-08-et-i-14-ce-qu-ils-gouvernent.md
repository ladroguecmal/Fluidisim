# ADR-260 — I-08 et I-14 : ce qu'ils gouvernent

- **Statut : actée**, S642, 2026-10-07 ; autonomie technique (ADR-222) ; suite de l'audit des intentions initiales
  ([AUDIT-INTENTIONS-INITIALES-S640](../registres/AUDIT-INTENTIONS-INITIALES-S640.md) §2.1, points 2 et 3) et
  d'[ADR-259](ADR-259-trente-deuxieme-revue-de-methode.md). Précise I-08 (après [ADR-139](ADR-139-volume-et-plan-oriente-des-contenants.md),
  [ADR-141](ADR-141-surface-linearisee-et-coefficients-temporels.md)) et I-14.

## 1. Le constat

- **I-08** (« tout calcul d'eau se fait en `f32` local à un référentiel ») n'est plus tenu à la lettre. Dans 43 modules du cœur, `f64`
  domine. ADR-139 n'en exceptait que V, et ADR-141 que les coefficients temporels. Aucun ADR ne rangeait les autres. Le banc B7 (S618)
  a même mesuré Saint-Venant 2D, écrit en `f64`, comme un module de jeu.
- **I-14** (« toute valeur dérivée d'une formule citée dans SPEC-001 ou SPEC-002, ou marquée à calibrer ») non plus. Une douzaine de lois
  employées depuis la v2 ne sont citées dans aucune des deux spécifications. Chacune l'est en revanche dans la preuve qui l'a reçue, avec sa
  référence publiée (vérifié en S642) :

| loi | preuve |
|---|---|
| Green | BATHYMETRIE-S362, GRAND-EVENEMENT-S614 |
| Brooks–Corey | ASSECHEMENT-S535 |
| Tomiyama, Schiller–Naumann | C13-BULLES-S540 |
| Willis, Rayleigh | EXPLOSION-BULLE-S587 |
| Marshall–Palmer | PLUIE-AIR-S380 |
| Synolakis | GRAND-EVENEMENT-S614, CHAINE-TSUNAMI-S624 |
| Keller | HOULE-PLAGE-S625 |
| Thacker | ORDRE-DEUX-S620 |
| Davies–Taylor, Minnaert | POCHES-AIR-S479 |

## 2. Décisions

**D1 — I-08 gouverne les champs de production.** Ce sont l'état et les mises à jour par pas des champs de B, W et δ évalués dans le jeu
(grilles, particules, spectres, et le GPU). Ces champs sont en `f32`, dans un référentiel local. `f64` reste permis dans quatre cas :

- **V** : la couche V elle-même (ADR-139) ;
- **R** : les références et instruments — juges, références CPU, bilans de diagnostic ;
- **O** : les outils hors ligne et les cuissons ;
- **S** : les calculs scalaires (une phase, un événement, un corps, une prédiction, une poche), à condition d'être arrondis en `f32` avant
  d'entrer dans un champ, comme les coefficients d'ADR-141.

**Une référence de champ (P)** écrite en `f64` est permise comme référence. Elle doit sa **version de production en `f32`** avant que son
point de la liste soit validé, comme le δ 3D a eu sa référence CPU puis sa production GPU. Le coût mesuré sur une référence P est un coût
de référence, non de production (B7, S618).

**D2 — L'inventaire est contrôlé.** [`outils/precision_f64.py`](../../outils/precision_f64.py) range chaque module où `f64` domine :

| catégorie | nombre | modules |
|---|---:|---|
| V | 3 | `hydro_network`, `hydro_geometry`, `hydro_charge` |
| R | 9 | `dispersif`, `gaussian_pressure`, `pressure_mode`, `spectrum_reference_s147`, `bathymetrie`, `delta3d_balance`, `delta3d_closure`, `delta3d_transfer`, `modal_pressure` |
| O | 7 | `riviere`, `geoide`, `portee`, `current_field`, `gaussian_spectrum`, `refraction`, `eponge` |
| S | 15 | `background`, `background_cwm`, `ballistic`, `bulle`, `rigid_body`, `maree`, `tsunami`, `deferlement`, `glace`, `regional_level`, `apic3d_poches`, `coule`, `activite`, `body`, `goutte` |
| **P** | **9** | `shallow`, `saint_venant_2d`, `substitutif`, `changement_solveur`, `grand_evenement`, `delta3d_levels`, `graine`, `precalcul`, `impact_field` |

`etat_projet.py --check` échoue si un module nouveau où `f64` domine n'est pas rangé : essayé en retirant un module de la table, le
contrôle a échoué. Le rangement initial vient de la documentation de chaque module ; une session qui ouvre un module mal rangé le
corrige ici.

**D3 — I-14 : la provenance s'étend à la preuve.** Une valeur a une provenance si elle dérive d'une formule citée dans SPEC-001 ou
SPEC-002, **ou dans la preuve de `docs/validation` qui l'a reçue, avec sa référence publiée**, ou si elle est marquée « à calibrer » avec
ce qui la fixera. L'intention reste entière : aucun nombre magique. Un seuil sans provenance n'est pas une règle. C'est le cas du `0,35·Hs`
d'ADR-001 §3.3, rangé en paramètre de l'appelant en S642 (ADR-112 D1, liste 4.11).

## 3. Ce que cela laisse ouvert

Les neuf références P doivent leur production `f32` : la liste porte cette dette à leurs points (4.11, 4.14, 4.20, 9.6, 11.3, 3.1). Rien
n'est réécrit en `f32` dans cette session : sans mesure de l'écart que l'arrondi introduit, une conversion serait une régression
silencieuse.
