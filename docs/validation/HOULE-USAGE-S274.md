# Houle progressive : quelle précision sert l'usage — S274

Consigne de l'utilisateur (2026-09-18) : avant tout raffinement, vérifier que la précision
recherchée sert le résultat final ; rattacher chaque critère à ce qu'il protège ; traduire
l'écart en grandeurs d'usage. Bancs : [S272](RESIDU-TEMPOREL-S272.md), [S273](BANDE-LINEAIRE-S273.md).
Fixture : h = 1 m, λ = 4 m (k = π/2), a = 1 cm (`ak` = 0,0157), 2 s ≈ 1,2 période, domaine
4 m × 1,5 m, fond linéaire traversant, perturbation fermée.

## 1. Ce que protège chaque critère

| critère (déclaré S272) | protège | état S273 |
|---|---|---|
| tous les pas reçus, aucun refus | **fonctionnement** | tenu, 6 000 pas |
| fond uniforme traversant exact, bilan des bandes, transaction, zéro allocation (S270, S273) | **fonctionnement** | tenus |
| oracle 128/256 modes < 0,1 % | **validité de l'instrument** | tenu, < 0,053 % |
| champs / a² ≤ 1 % | **validité de l'instrument** : la référence d'ordre deux s'applique | **manqué** (2,89 %) |
| demi-pas ≤ 0,5 % | **fidélité** : le temps ne domine pas le verdict (réserve d'ADR-120) | manqué (0,668 % à 2 ms) |
| décroissance en dx | **fidélité** : convergence | tenue, lente (9,11 / 6,18 / 5,88 %) |
| écart L2 de `η'` ≤ 2 % contre l'oracle d'ordre deux | **fidélité** de la correction non linéaire (ADR-120, 2 % décidés par l'utilisateur) | manqué (5,88 %) |

Aucun critère ne protège la **qualité visuelle** : le rendu ne consomme aucun domaine δ
(§4). Le quatrième critère échoue, donc le dernier ne juge pas le solveur : ADR-120 exige
lui-même que l'erreur de la référence tienne dans les 2 % (`e_spatial + e_temporel + e_référence`).

## 2. L'écart de 5,88 % en grandeurs d'usage

`η'` est la correction non linéaire : sa valeur rms vaut 99 µm, 1 % de la vague. Maille fine :

| grandeur | écart pas réel − oracle | repère |
|---|---|---|
| hauteur | **5,8 µm** rms, 26 µm max ; 0,058 % de a | `η'` rms 99 µm |
| pente | **1,9·10⁻⁵** rms, 9,1·10⁻⁵ max | pente de `η'` 3,2·10⁻⁴ ; de la houle, ak = 1,6·10⁻² |
| déphasage de la vague (mode k en quadrature), à 2 s | **1,35·10⁻³ rad** = 0,86 mm = 0,36 ms | Stokes ω₂t = 1,13·10⁻³ rad |
| harmonique 2k, t ≥ 1 s | amplitude +1,5 %, phase −0,009 rad | — |
| forme d'ADR-120 (max au dernier instant / max de `η'`) | 18,0 % | le dernier instant porte la dérive maximale |

Transposé par similitude de Froude à une houle de 100 m de longueur d'onde **à la même
cambrure** (×25 en hauteur, pentes et phases inchangées, ×5 en temps) : 0,15 mm rms et
0,66 mm max, sous les 3 mm de hauteur d'image (S201) ; 1,8 ms d'équivalent horloge, sous
les 20 ms d'ADR-003. **Ces nombres ne prouvent pas l'invisibilité** : ils valent pour
1,2 période et `ak` = 0,016, et aucun rendu ne montre δ.

## 3. D'où vient l'écart : l'ordre trois, pas le pas

`D = N(a) − N(a/2)`, `N = η'/a²`, est ce qui distingue le pas réel d'un comportement
d'ordre deux. Projeté sur `sin θ`, il croît avec t à un taux **0,85 / 0,86 / 0,87** fois celui
que prédit la dispersion d'amplitude de Stokes (`ω₂ = 0,611 (ak)² ω` à kh = π/2), aux trois
mailles ; 64 à 67 % de sa norme est dans le mode k. L'écart à une droite vaut 30 % :
transitoire et ondes libres. C'est une **cohérence d'ordre de grandeur indépendante de la
maille**, pas une mesure du décalage de fréquence du solveur.

Le pas réel porte donc la dérive de phase d'une vague non linéaire, que l'oracle d'ordre deux
n'a pas. ADR-122 l'avait établi pour les véhicules : une troncature d'ordre deux ne juge pas la
phase sur plusieurs périodes. Le critère brut de S272 mesure surtout ce manque.

## 4. Usage réel

- **Rendu** : `viewer/` ne consomme aucun domaine δ, et δ est encore en x-z. **Raccord
  manquant** : un chemin qui publie la surface d'un domaine δ au rendu, composée avec B+W
  sous sa bordure d'éponge. Sans lui, aucune comparaison en mouvement aux distances de jeu
  n'est possible, et rien ne se conclut sur la visibilité.
- **Besoin découvert — cohérence de phase entre δ et B.** B est linéaire (ADR-113) ; un δ
  fidèle accumule la dérive de Stokes `η' ≈ a·ω₂·t` par rapport à lui. Ordre de grandeur, par
  la formule seule : `ak` = 0,06, T = 7 s, 60 s → ≈ 0,1 rad, soit ≈ 7 cm de hauteur pour
  a = 0,75 m. Trois effets : `|δ|` n'est plus petit devant B (ADR-001) ; couture de phase à la
  bordure ; surface rendue différente de la surface de jeu B+W (I-04). À trancher avant un
  domaine δ de longue durée sous une houle cambrée (options au §6 après P4).

## 5. Critères de la campagne P3, écrits avant mesure

Mesurer **le coefficient d'ordre deux du pas réel**, `N₂`, pas la houle complète.

- Amplitudes 2 cm, 1 cm, 5 mm ; mailles 0,125 / 0,0625 / 0,03125 ; dt = 1 ms. Contrôle à
  dt = 0,5 ms à la maille fine, amplitudes 1 cm et 5 mm. Même fixture, même oracle.
- `N*(a) = 2N(a/2) − N(a)` vaut `c₂ − c₄a²/2` si `η' = a²(c₂ + c₃a + c₄a²)`. **Extrapolation
  qualifiée** si `‖N*(2 cm) − N*(1 cm)‖ ≤ 0,5 %` de la norme de l'oracle ; `c₂` extrapolé à
  trois amplitudes sert alors de valeur, et l'écart avec `N*(1 cm)` entre dans `e_référence`,
  avec la garde de l'oracle (< 0,053 %).
- `e_temporel` = `‖N*(1 ms) − N*(0,5 ms)‖` à la maille fine.
- **Verdict ADR-120** : `‖N₂ − oracle‖ + e_temporel + e_référence ≤ 2 %` à la maille fine, en
  norme L2 espace-temps (métrique déclarée S272), décroissant de 0,125 à 0,03125 ; le maximum au
  dernier instant est publié à titre d'information.
- Sinon : chiffrer la part en défaut, sans relever de seuil. Un reçu dit « coefficient d'ordre
  deux », jamais « houle progressive reçue ».
