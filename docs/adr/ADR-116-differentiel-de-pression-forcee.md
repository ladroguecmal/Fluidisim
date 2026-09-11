# ADR-116 — La pression imposée entre dans le champ profond

- **Statut : actée**, S180, 2026-09-12, autonomie technique S71.
- **Applique :** ADR-069/071/113/114 ; SPEC-001 potentiel profond, SPEC-004 §6.1.
- **Portée :** champ spectral de pression préparé à un instant ; modèle profond uniforme.

## Dérivation

Pour un mode de vecteur horizontal K, k=|K|, amplitudes complexes H=eta et V=eta_t,
pression imposée P (Pa positive vers le bas), poids w, E=exp(kz), phase K·x :

```text
phi = E V/k ; phi_t = E A, A = -g H - P/rho
V_t = -g k H - k P/rho
u_h = E i K V/k ; u_z = E V
du_dt_h = E i K A ; du_dt_z = E k A
grad_u_hh = -E K Kᵀ V/k ; grad_u_hz = grad_u_zh = E i K V
grad_u_zz = E k V
p_dyn = E (rho g H + P)
grad_p_h = E i K (rho g H + P) ; grad_p_z = E k (rho g H + P)
```

Toutes ces expressions sont pondérées par w et projetées par Re(amplitude exp(iK·x)),
puis sommées dans l'ordre existant. eta et grad_eta sont évalués en surface, indépendants
de z. z est relatif au plan moyen, dans ]-4096,0]. Hydrostatique et gravité compensées
comme ADR-114. Δu=(k²-K_x²-K_y²)u, nul idéalement ; conserver le petit défaut de norme
f32 de représentation au lieu d'en faire un Laplacien de grille.

p_dyn est la pression hydrodynamique relative à l'hydrostatique du plan moyen.
Elle contient la réponse à la pression imposée : àz=0, p_dyn=rho g eta+p_applied.
La décomposition p_wave=p_dyn-E P n'est utile qu'à l'explication, elle ne doit pas
être utilisée seule dans S. La pression appliquée p_applied et son gradient sont
publiés séparément comme grandeurs de surface, indépendantes de la profondeur.
S=U_t+(U·∇)U+grad_p_dyn/rho-nu ΔU (ADR-114) ne reçoit aucun second forçage volumique.
Le forçage agit par la condition de pression à la surface ; ne pas le compter deux fois.

## Contrat et construction

Field::differential(point) retourne un PressureDifferential contenant BackgroundSample,
applied_pressure (Pa), grad_applied_pressure (Pa/m, composante z nulle), density (kg/m³).
L'instant est celui auquel Field est préparé : aucun nouvel argument temporel libre.
La durée active reste semi-ouverte [birth,end), conformément à ModalPressure::pressure.
Aux commutations, les dérivées temporelles sont celles de la branche active àcet instant,
non une différence centrale àtravers la discontinuité. H et V restent continus ; A saute.

Chaque Slot garde K, g et rho d'origine, soit16 octets supplémentaires par nœud.
Cela évite d'inventer le milieu à l'évaluation et de reconstruire K depuis un produit
déjà pondéré. Accumulation directe et incrémentale conservent les métadonnées ; même
chemin FieldState/bind. Les pools existants se dimensionnent avec size_of ; aucune
allocation dans le fournisseur. Valeurs, coefficients et condensats historiques inchangés.

Lot sur scratch hôte : publication du préfixe demandé seulement après succès intégral,
reste de sortie intact, même convention que sample_batch. Domaine hérité de Field,
plus z local ; erreurs de domaine, non-finitude et capacité explicites. Métadonnées
de Slot non valides refusées. Pression et gradients ne sont jamais remplacés par zéro.

## Réception et suite

[DIFFERENTIEL-PRESSION-S180](../validation/DIFFERENTIEL-PRESSION-S180.md) : mode fixe
fermé f64, démarrage/forçage/extinction, directions croisées et dérivées par différences
finies, accord de surface, refus, publication atomique, chemins direct/incrémental/relié.
La contre-épreuve sans pression imposée doit échouer au démarrage, où eta=u=0 mais
la pression et l'accélération ne sont pas nulles.

Restent ouverts : composition différentielle B+impacts+pressions, contexte monde et
cycle contrôleur, coût, surface libre non linéaire et δ3D. A50/B4 partiels ; pas de
nouveau seuil de décimation ni de certification multiplateforme.
