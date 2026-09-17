# Défauts restants réparés — S262

Demande de l'utilisateur (2026-09-17) : « répare d'abord les défauts restants avant de peaufiner le
visuel ». Trois défauts publiés en S260–S261.

## 1. Ligne d'horizon

La grille projetée s'arrêtait à 1 500 m, et l'air clair de l'habillage le révélait par une ligne de
tirets sombres à l'horizon.

**Réparation.** La distance lointaine devient un paramètre : `impact.y` dans l'uniforme, `far` dans
`lod::Projection`. Sous la brume, elle reste à 1 500 m. Sous le ciel clair, elle vaut l'horizon
géométrique `√(2·R·h)`, avec R = 6 371 km : 9,44 km à 7 m. L'eau s'y fond dans la couleur d'horizon
sur le dernier tiers, là où la courbure la cacherait.

**Preuves.** R2, R3, R4 et `--spectral-verify` sont identiques au bit après le changement ; afficheur
16 / 1 / 0. Sur le rendu clair, les tirets ont disparu.

## 2. Coût GPU au-dessus de 2 ms

Décomposition avant réparation, `--vagues --modulation --ciel-clair`, 1280×720, vue de référence :

| poste | ms |
|---|---:|
| total GPU eau | 2,284 |
| cuisson de la grille du sillage | 1,272 |
| queue par pixel | ≈ 0,49 |
| CWM des sommets et queue f⁻⁴ | ≈ 0,33 |
| nuages dans les reflets | ≈ 0,05 |

**Réparations**, toutes sans effet au-delà de l'arrondi :

- **(a)** la bande spectrale de chaque mode de sillage est calculée une fois par l'hôte, et non plus
  par nœud × mode ;
- **(b)** `k` et la direction de chaque composante de queue sont calculés une fois ;
- **(c)** le sommet CWM parcourt la bande une seule fois ;
- **(d)** le total de la grille est la somme de ses huit bandes, et non une seconde accumulation ;
- **(e)** les nuages des reflets sont réduits à deux octaves. C'est un habillage, et le ciel en garde
  quatre.

**Après.** Secteur, 99 % au début et à la fin, deux passages, GPU eau médian :

| format | référence | rasante | cuisson |
|---|---:|---:|---:|
| 960×540 | 1,54 ms | 1,56–1,60 ms | 1,07–1,10 ms |
| 1280×720 | 1,98–2,01 ms | 1,98–1,99 ms | 1,06–1,10 ms |
| 1280×720 avant | 2,28 ms | 2,26 ms | 1,23–1,27 ms |

**Sous 2 ms au format de référence de J1 (960×540), avec 0,4 ms de marge ; à la limite en 1280×720.**
- **Présentes** : précalculs (a) et (b), boucle unique (c), total par bandes (d), filtres d'ADR-148 et
  d'ADR-155.
- **Absentes** : LOD temporel de la cuisson, qui pèse encore 1,07 ms, et demi-résolution de la queue.
- **Domaine** : scène S235, deux poses, une machine.

**Effet sur l'image, mesuré octet par octet.** R2 (brume, sans CWM), après (a) à (c) : 0 à 3 octets
différents sur 2 764 800, d'un niveau chacun ; les deux vues sans perturbation sont identiques. La
scène complète : 9 à 27 octets, au plus 8 niveaux, sur des pixels de reflet où le reflet amplifie
l'arrondi. L'ordre des sommes change avec le compilateur de shader et avec (d) : c'est de l'arrondi.
Les empreintes de revue changent donc, et les anciennes restent reproductibles à leurs commits.

**Preuves d'exactitude.**
- `--tail-verify` et `--cwm-verify` : lignes identiques à S256 et S261.
- `--spectral-verify` : le chemin par grille ne change qu'à la 7e décimale (0,00021516 devient
  0,00021508) ; le reste est identique au bit, retour caméra compris.
- `--multi --verify` : 46 contrôles, hauteur max 0,368 mm comme en S235 ; grille intérieure ≤ 0,39 mm
  pour une borne de 2,5 à 3 mm ; coutures 4 µm.

**Incident.** Un rendu de contrôle a réécrit les images locales de R5 (`captures/s261`, non versionnées).
Elles restent reproductibles au commit 1e89da7, empreintes publiées. Chaque revue a désormais son
dossier.

## 3. A288 : la requête de jeu sous CWM

Contrat : [ADR-159](../adr/ADR-159-requete-de-jeu-sous-cwm.md).

### Protocole, écrit avant construction

1. **Un mode** : pour une onde de Gerstner seule, la requête en `x = α + a·d·cos θ(α)` rend `α` à 0,1 mm
   près, et `η = a·sin θ(α)`.
2. **Mer de la scène** (64 composantes, `--vagues`) : pour 10⁴ points de Lagrange `α` tirés, la requête
   en `x = α + D_B(α)` (référence f64) rend `α` à 1 mm près et l'élévation à 1 mm près ; itérations
   maximales publiées.
3. **Refus** : repli synthétique (`det J ≤ 0`), point hors domaine, non-convergence ; aucune sortie.
4. **Contre l'image** : aux sondes, `x = q + D_B^GPU(q)` et `h = water^GPU(q)`. La requête du cœur en `x`,
   composée avec W au point rendu `α`, redonne `q` et `h` à 3 mm près, tolérance de `verify`.
   L'écart au jeu passe de 0,365 m à moins de 3 mm.
5. Aucune allocation ; les requêtes linéaires existantes restent inchangées au bit.
