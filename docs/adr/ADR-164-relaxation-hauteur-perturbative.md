# ADR-164 — Relaxer la hauteur perturbative dans l'éponge mobile

Actée S268, 2026-09-18, autonomie S71. Complète ADR-149/152 pour la hauteur mobile ;
remplace leur limitation « éponge sur la vitesse seule » uniquement pour
`step_perturbation_mobile`. Le pas à hauteur imposée reste inchangé.

Après projection et transport de hauteur, appliquer à chaque centre de colonne :
`η' ← exp(-σ(x) dt) η'`, avec le même taux quadratique et la même largeur que Sponge.
C'est la solution exacte de l'étape locale `∂t η' = -σ η'` ; le découpage avec le
transport est d'ordre un, comme le prédicteur existant. Le fond analytique ne se
relaxe pas : l'état stocke `eta = repos + η'`, jamais la surface totale.

La hauteur compensée représente `eta - reste`. Pour un facteur f32 `f`, l'incrément
est `(f-1)*(eta-repos) - f*reste`, ajouté par la somme compensée existante. Le reste
ancien est amorti lui aussi. Si `f=1`, ne rien écrire (identité de l'intérieur et du
chemin désactivé). Champs et incréments restent f32 ; seul le coefficient temporel
est construit en f64 selon ADR-141. Aucune mémoire supplémentaire.

L'étape est dans la transaction et le budget, avant les contrôles de fin : refus
ou expiration restaure aussi la hauteur et son reste. Le fond fourni reste immuable.
La projection demeure avant transport et relaxation ; la divergence publiée est
celle du champ projeté sur la géométrie du début du pas, comme ADR-152.

Il s'agit d'une couche absorbante à fermeture extérieure réfléchissante, **pas encore
d'une frontière transparente reçue**. Cette dissipation retire de l'énergie et du
volume perturbatif ; aucune conservation de ce volume ni transduction vers W n'est
revendiquée, aucun volume autoritaire V touché. Largeur/taux restent des entrées du
scénario. Les règles dispersives ADR-046 ne fournissent pas une réception automatique
du MAC : réflexion d'un paquet et fond traversant restent à éprouver séparément.

[Critères et preuve](../validation/RELAXATION-SURFACE-S268.md).
