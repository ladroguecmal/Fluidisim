# ADR-114 — Le résidu continu de B est une accélération à soustraire

- **Statut : actée**, S178, 2026-09-11, autonomie technique S71.
- **Applique :** SPEC-004 §6.1, SPEC-001 §1, ADR-113 et ADR-048.
- **Portée :** fond B linéaire profond uniforme ; aucun schéma δ choisi.

## Décision et dérivation

BackgroundSample ajoute grad_p_dyn (Pa/m) et laplacian_u (1/(m s)).
Sa méthode momentum_residual(rho,nu) forme S en m/s² :

```text
S_i = du_dt_i + Σ_j u_j grad_u[i][j] + grad_p_dyn_i/rho - nu laplacian_u_i
```

rho est la même densité uniforme que celle fournie à differential ; nu est la viscosité
cinématique uniforme, finie et positive ou nulle, en m²/s. Aucune valeur par défaut.
La viscosité dynamique devrait être divisée par rho avant cet appel. Le solveur
perturbatif reçoit **-S**, pas S. Le résultat n'est pas une force ni un résidu discret.
Le type est une donnée publique : la cohérence de rho reste un contrat de l'appelant.

La pression totale vaut P_h+p_dyn, avec ∇P_h/rho=g_eff. En soustrayant l'équilibre
hydrostatique, gravité et P_h disparaissent ensemble. Ajouter encore g à cette méthode
compterait la gravité deux fois. Référentiel local d'ADR-113, gravité uniforme de B ;
aucun terme de référentiel accéléré ou de viscosité variable n'est implicitement reçu.

Avec les notations d'ADR-113, pour une composante :

```text
grad_p_dyn = rho g a k exp(kz) (d_x cosθ, d_y cosθ, sinθ)
Δu = k²(1-d_x²-d_y²) u
```

Le potentiel harmonique donne Δu=0 pour une direction exactement unitaire. Les
directions stockées en f32 ne le sont qu'à l'arrondi près : le fournisseur conserve
le petit défaut de norme dans cette expression, sans changer les paramètres de B.
Les termes du Laplacien ne sont pas les secondes différences d'une grille.
Le défaut ω²-gk de représentation reste lui aussi dans S ; on ne remplace pas
du_dt+grad_p/rho par un zéro présumé.

Même pour un mode Airy idéal, S n'est pas nul : l'advection vaut
(0,0,k(aω exp(kz))²). Pour plusieurs modes, les vitesses et gradients se somment
**avant** contraction : S contient les interactions entre modes. Additionner les
résidus de modes isolés perd ces termes croisés.

## Contrat et réception

Les chemins ponctuel et par lot existants fournissent les deux nouveaux champs,
sans allocation ; leur validation des sorties couvre ces champs. momentum_residual
est pur, valide rho, nu, l'échantillon et son résultat ; aucun résultat partiel publié.
Nu invalide est distingué par Viscosity. Les valeurs historiques d'eval sont conservées.

Réception prévue : gradient de pression par différences finies ; Laplacien par
divergence du gradient et direction légèrement non unitaire admise ; advection
contre ∇(|u|²/2) et différence temporelle indépendante ; mode seul non nul, interactions
croisées, densité, viscosité, refus et non-finis. Voir
[SOURCE-B-S178](../validation/SOURCE-B-S178.md).

## Ce qui reste ouvert

W différentiel et composition B+W ; conditions de surface libre non linéaires,
projection de pression, frontières et δ3D ; viscosité variable et milieu non uniforme.
Un résidu volumique reçu ne ferme pas les équations de surface. A50 reste partielle.
I-02/03/06/07/08/09 inchangés ; conformité sur plusieurs plateformes non certifiée.
