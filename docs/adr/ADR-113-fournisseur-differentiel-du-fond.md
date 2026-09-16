# ADR-113 — Le fournisseur différentiel de B explicite sa profondeur

- **Statut : actée**, S177, 2026-09-11, autonomie technique S71.
- **Applique :** SPEC-004 §2.1/§6, SPEC-001 §1 (Airy profond), ADR-048 (rho du milieu).
- **Portée :** B linéaire profond uniforme seulement ; aucun solveur δ choisi.

## Décision

Le nouveau BackgroundSample est distinct du WaterSample consommateur. Le fournisseur
partage les composantes et la phase de Background sans modifier eval ni ses hashs.
Les points sont dans les axes locaux du fond ; z est l’altitude relative au plan moyen
passant par son ancre, et doit appartenir à]-4096,0]. x et y restent dans]-4096,4096[.
Ce domaine mathématique n’est pas un détecteur de mouillage sous la surface instantanée.
L’hôte fournit rho uniforme positif en kg/m³ ; g est celui ayant construit B.

eta et grad_eta décrivent la surface au même (x,y), indépendamment de z.
u et ses dérivées décrivent le champ Eulerien àz. grad_u[i][j]=∂u_i/∂x_j.
p_dyn est la pression de vague en Pa, relative àl’hydrostatique du plan moyen,
ni pression absolue ni pression cinétique rho|u|²/2. Pas de courant inventé.

Pour chaque composante, phase θ=k d·x-ωt+θ0, E=exp(kz), A=aωE :

```text
eta = a sinθ ; grad_eta = (ak cosθ d_x, ak cosθ d_y, 0)
u = (A sinθ d_x, A sinθ d_y, -A cosθ)
du_dt = (-Aω cosθ d_x, -Aω cosθ d_y, -Aω sinθ)
grad_u[i][j] = Ak cosθ d_i d_j       (i,j horizontaux)
grad_u[i][z] = Ak sinθ d_i          (i horizontal)
grad_u[z][j] = Ak sinθ d_j          (j horizontal)
grad_u[z][z] = -Ak cosθ
p_dyn = rho g a E sinθ
```

Ces formules viennent du potentiel φ=-(aω/k)exp(kz)cosθ, rotation de phase de
SPEC-001 §1 ; p=-rho∂tφ=rho(aω²/k)E sinθ, égal àrho g a E sinθ sous ω²=gk.
La bibliothèque conserve ses k et ω quantifiés ; les égalités différentielles sont
reçues àla précision de cette représentation, pas rendues exactes par une modification
du spectre. Les dérivées sont celles du champ analytique représenté ; ni la phase
quantifiée ni les polynômes sin/cos ne sont différentiés comme des fonctions par morceaux.

Le facteur profond utilise une approximation àarithmétique fixe (réduction par ln2,
Taylor ordre10, échelle binaire), sans appel libm exp dans le chemin B. Àz=0, E=1
exactement. Réception contre exp f64 indépendante ; pas de certification multiplateforme
sans exécution sur plusieurs cibles. Toutes les opérations du champ restent f32,
hors conversion ω déjà utilisée par B historique.

## Interface et refus

Méthodes ponctuelle locale/monde et lot monde. Aucun état par cellule stocké ; les
sorties et le scratch appartiennent àl’appelant. Le lot vérifie les tailles puis
calcule dans le scratch et publie seulement après succès complet. Scratch peut changer
sur refus, sorties non. Entrées, paramètres et résultats non finis sont refusés.
Aucun changement silencieux des constructeurs historiques permissifs.

Pas de is_smooth_at permissif, de décimation ou de cadence introduits. B+W, gradient
de pression et Laplacien visqueux restent àconstruire avant de prétendre fournir toute
la source SPEC-004. I-02/03/06/07/08/09 restent inchangés. Réception :
[FOURNISSEUR-B-S177](../validation/FOURNISSEUR-B-S177.md).

**Note S254, 2026-09-16.** `differential_local` refuse toujours `z > 0`. Une entrée distincte,
`differential_local_extended` ([ADR-154](ADR-154-prolongement-borne-du-fond.md)), rend au bit ce
fournisseur pour `z ≤ 0`, et le prolongement borné au-dessus du plan moyen.
