# ADR-142 — La composition mixte sert l'union des emprises, sous un plancher certifié

- Statut : **actée**, S236, 2026-09-15 ; autonomie technique S71.
- **Ajoute** un mode à la requête mixte ([ADR-077](ADR-077-requete-mixte-impacts-et-pressions.md),
  [ADR-080](ADR-080-annonce-des-points-du-montage-mixte.md)) sans le remplacer : les fonctions
  existantes — `admits`, `slope_floor`, `slope_floor_joint`, `sample_world_batch` — restent
  identiques **au bit**.
- Prolonge [ADR-128](ADR-128-le-budget-de-pente-borne-les-perturbations.md) (budget des
  perturbations, exact dans les deux sens) et [ADR-138](ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md)
  (position relative). Emploie [ADR-137](ADR-137-coupure-spectrale-de-la-borne-locale.md) dans
  l'admission pour la première fois.
- Aligne la requête sur [ADR-126](ADR-126-emprise-d-un-impact-visible.md) règle 4 : « hors
  emprise, B seul ».
- Mesures : [SCENE-MULTI-S235](../validation/SCENE-MULTI-S235.md) §2 et `--bornes-union` (S236 P2,
  notes de session) ; réception : [ADMISSION-UNION-S236](../validation/ADMISSION-UNION-S236.md).

## Constat

**La requête du cœur ne compose que l'intersection des emprises.** `mixed::admits` refuse un point
qui sort du disque d'un seul impact ou de l'emprise de la pression. C'était sans conséquence à un
impact ; sur la scène représentative S235 — huit impacts de 52 m répartis sur 70 m, trois sillages —
l'intersection est presque vide, et la scène est refusée **par le domaine** avant toute pente.
L'image, elle, rend « hors emprise, B seul » depuis ADR-126. Le chemin gameplay et le chemin
d'image ne décrivaient donc pas la même eau.

**Le budget refuse la scène sans raison physique.** S235 : 49 instants refusés sur 161, **tous par
majorant seul** ; pente réelle ≤ 0,2154, soit 48 % de π/7.

**Le balayage d'ADR-138 n'est sûr que sur l'intersection.** Il parcourt `r₁ ∈ [0 ; R₁]`, le disque de
l'ancre. Tout point admis par l'intersection y est, et la borne tient. Servi sur l'union, il laisse
hors balayage les points éloignés de l'ancre : deux impacts nés avec l'ancre, confondus à 100 m
d'elle, y additionnent leurs maxima sans qu'aucune cellule ne les compte ensemble.

**Ce que S236 a calculé avant de décider** (série S235, CPU) :

| plancher, sur l'union | instants refusés / 161 | pire / π/7 |
|---|---:|---:|
| actuel (intersection, ADR-138) | 49 | 1,251 |
| ADR-138 étendu à l'union | 40 | 1,190 |
| séparation 2D, pression globale | 32 | 1,153 |
| valeur atteinte par la fonction bornée | 31 | 1,152 |
| **séparation 2D + pression locale ADR-137 sur cellules critiques** | **0** | ≤ 0,9994 |

La quatrième ligne tranche : une somme d'impacts **exacte en position** refuse encore. Les termes
eux-mêmes sont trop larges — pression globale 0,19 contre ≈ 0,07 réelle —, et seul un terme de
pression **local** là où les impacts sont forts rend la scène admissible.

## Décision

**1. Un mode union, à côté du mode intersection.** `mixed::admits_union` n'exige que le domaine de
B. `mixed::sample_world_batch_union` compose, en chaque point, B, **les impacts dont le disque le
couvre** et **la pression si son emprise le couvre** ; une perturbation qui ne couvre pas le point
n'y contribue pas. La raideur publiée (`steepness`) somme les mêmes termes couvrants. Contrôles de
montage (`classify`), ordre de somme, normalisation et erreurs non géométriques : ceux du mode
intersection.

**2. Le plancher du mode union est un certificat par séparation.** Soit, en tout point `p`,
`G(p) = Σ_i [r_i ≤ R_i] F_i(t, r_i) + [p ∈ emprise] P`, où `F_i = slope_max_beyond` (décroissante
en `r`, ADR-138) et `P = slope_envelope` (ADR-134). Sur une cellule rectangulaire, chaque terme est
majoré à la **distance minimale** de la cellule au centre, et `P` n'est compté que si la cellule
touche l'emprise. **Aucune constante de Lipschitz** : les termes sont monotones du bon côté,
exactement l'argument d'ADR-138 porté en deux dimensions. La cellule au plus grand majorant est
coupée en quatre. Arrêts :

- **certifié** dès que le plus grand majorant ouvert est ≤ `max_slope` — il majore alors `G`
  partout ;
- **non certifié** si une cellule de demi-côté ≤ 1 cm reste au-dessus, ou si le pool de cellules
  fourni par l'hôte est épuisé (I-06) : le plancher rendu est ce plus grand majorant, > `max_slope`.

Chemin rapide : si la somme d'origine `Σ_i slope_max_at + P` est ≤ `max_slope`, elle est rendue
sans séparation.

**3. Pression locale sur les seules cellules critiques.** Sur une cellule de demi-côté ≤ 1 m, dont le
majorant à pression globale dépasse `max_slope`, `P` est remplacé par
`min(P, local_slope_envelope_spectral(cellule ∩ emprise))` (ADR-137). Les 1 m et 1 cm sont des
paramètres d'arrêt, pas des seuils physiques : ils ne décident que du moment où l'on renonce, jamais
d'une admission que la borne ne prouve pas. Provenance : S236 P2, où ils certifient les 32 instants
restants de S235 en 92 à 128 cellules et 3 à 15 appels locaux.

**4. L'annonce et le refus lisent la même quantité.** `mixed::slope_floor_union(impacts, pressure,
time, max_slope, pool)` est l'unique implémentation ; la requête l'appelle **une fois** par lot, avec
le même `max_slope`. La garantie d'ADR-128 tient dans les deux sens, **pour ce `max_slope`** :
plancher ≤ `max_slope` ⟹ aucun refus de pente ; plancher > `max_slope` ⟹ tout lot non vide est
refusé (`SlopeEnvelope` si la pente réelle au premier point tient, `Slope` sinon — A208). Le plancher
dépend du seuil par l'arrêt anticipé ; c'est pourquoi il le prend en entrée.

**5. Le mode intersection ne bouge pas.** Aucun bit, aucun refus, aucun coût : un hôte qui
n'appelle pas les fonctions `_union` ne voit aucune différence.

## Ce que cette décision obtient, et ce qu'elle ne fait pas

**Obtient.** Le cœur compose la scène représentative sur l'union de ses emprises, et S236 P2 la
calcule admissible aux 161 instants — dont 129 sans pression locale.

**Ne fait pas.**

- **Elle n'est pas un certificat f32 sur les cellules critiques.** ADR-137 est une « réception
  numérique, pas certificat f32 » : là où elle intervient, la garantie hérite de ce statut (**A258**).
  Ailleurs, le statut est celui d'ADR-138 : constante de décroissance de `J₁` mesurée sur `bessel`
  exécutée, garde de 1e-4.
- **Elle ne résout pas A261** : la pression globale reste le terme ailleurs que près des impacts
  forts, et une scène où la pression seule sature ne gagne rien.
- **Elle ne resserre pas `F_i`** : les impacts anciens gardent leur majorant temporel calibré
  (ADR-133) et leur décroissance en `1/√(kr)` (ADR-138), sans dispersion spatiale du paquet.
- **Elle coûte** : jusqu'à ≈ 13 ms par lot sur S235 quand la pression locale intervient, contre
  0,02 ms pour ADR-138 ; aucun cache entre lots du même instant. Chiffré dans la réception.
- **Elle ne change aucune autorité** : W reste cosmétique pour le jeu (ADR-001 §2) ; le mode union
  aligne la requête sur l'image, il ne donne pas à W un rôle qu'il n'a pas.

## Réversibilité

Fonctions ajoutées, aucune modifiée. Revenir consiste à ne plus les appeler ; les retirer ne touche
aucun chemin existant.
