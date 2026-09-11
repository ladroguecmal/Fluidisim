# ADR-115 — Le différentiel radial conserve sa limite au centre

- **Statut : actée**, S179, 2026-09-12, autonomie technique S71.
- **Applique :** ADR-060/062/113/114, SPEC-001 §1, SPEC-004 §6.1.
- **Portée :** candidat RadialImpact profond, puis composition ponctuelle B+un impact.

## Champ et dérivation

Pour un nœud de coefficient c, q=kr, E=exp(kz), τ=ω(t-birth), potentiel
φ=-(cω/k) E J0(q) sinτ. Le z local est relatif au plan moyen commun à B et W,
dans ]-4096,0], et non relatif à l'altitude de la cause. Le rayon horizontal est
mesuré depuis la cause. eta reste une grandeur de surface indépendante de z.

```text
eta = c J0 cosτ ; grad_eta = -c k J1 cosτ n
u_h = c ω E J1 sinτ n ; u_z = -c ω E J0 sinτ
du_dt_h = c ω² E J1 cosτ n ; du_dt_z = -c ω² E J0 cosτ
grad_u_hh = c ω k E sinτ [(J0-2R) n nᵀ + R I], R=J1(q)/q
grad_u_hz = grad_u_zh = c ω k E J1 sinτ n
grad_u_zz = -c ω k E J0 sinτ
p_dyn = rho g c E J0 cosτ
grad_p_h = -rho g c k E J1 cosτ n ; grad_p_z = rho g c k E J0 cosτ
```

À r=0, R=1/2 et J0-2R=0 : gradient horizontal isotrope, u_h=grad_p_h=0.
Près de zéro, calculer R et J0-2R par séries en q² évite la division singulière
et la soustraction de deux nombres voisins. Jusqu'à q=1/16, termes jusqu'à q⁶ ;
premier terme omis de R, q⁸/1474560, inférieur à 1,6e-16. Au-delà, J0/J1 du
candidat existant. Les dérivées désignent les fonctions analytiques représentées,
pas la dérivée du polynôme d'interpolation Bessel ni de la phase quantifiée (L259).
Laplacien nul analytiquement : Δ_h J0(kr)=-k²J0, Δ_z E=k²E. Ce zéro est démontré,
pas un repli pour une donnée manquante. Fréquence et omega historiques conservés.

## Contrat

RadialImpact conserve g et rho de sa construction (deux f32 supplémentaires, padding
éventuel) pour publier une pression cohérente avec son amplitude et la dispersion.
Pas de changement aux coefficients ni à sample ; anciens condensats conservés.
Son différentiel rend BackgroundSample, avec les mêmes unités que B et une erreur
distinguant refus radial, refus B, et incompatibilité de gravité. Géométrie puis
temps : domaine refusé même avant naissance ; avant naissance zéro, à naissance
valeur initiale, dernier instant inclus puis Time. La dérivée à naissance est celle
de l'évolution à droite : l'impulsion d'initialisation n'est pas une source continue.

La composition locale exige le même repère et le même plan moyen pour B et W,
déclarés par l'hôte : les coordonnées point sont locales à l'ancre B. Le fournisseur
contrôle frame/cell de W et l'égalité de g. B ne possède pas FrameId/cell, leur
correspondance avec son ancre n'est donc pas certifiable par cette API.
Rho du champ W est fourni à B. Une seule contribution B, puis un W ; ordre fixe.
Tous les champs différentiels s'additionnent avant momentum_residual(rho,nu),
ce qui conserve (B·∇)W+(W·∇)B. Sortie par lot publiée seulement après calcul complet
dans le scratch de l'appelant, sans allocation ; scratch peut changer sur refus.

## Réception et limites

[DIFFERENTIEL-W-S179](../validation/DIFFERENTIEL-W-S179.md) : origine, voisinage,
axes croisés et profondeur ; quadrature angulaire f64 indépendante, différences
finies, anciennes valeurs de surface, contre-épreuve des interactions omises,
refus temporels/géométriques et atomicité.

Restent ouverts : sources W multiples, pression forcée, réseau et cycle LiveWater
pour ce nouveau type, coût, forces et solveur δ3D. A50/B4 restent partiels. Aucun
is_smooth_at adopté ; aucune certification multiplateforme par un test local.
