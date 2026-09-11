# Fournisseur différentiel B — S177

## Protocole déclaré

S176-1 ; ADR-113 fixe conventions, dérivation, portée et refus. Nouveau module enfant
de background.rs, réexport du type et de son erreur ; eval historique non modifié.

Réception prévue : mono-composante indépendante, directions croisées et dérivées par
différences finies, raccord surface bit àbit, état nul, pression/profondeur, atomicité
sur erreur tardive, paramètres invalides, approximation exponentielle sur son domaine.
Les tolérances numériques doivent être distinguées des seuils physiques. Réexécuter
workspace et conformité historique pertinente ; aucune nouvelle dépendance.

## Implémentation reçue

`background_differential.rs`, enfant de background.rs : BackgroundSample et
DifferentialError réexportés sous `water_core::background`. Méthodes de Background :

```text
differential_local([x,y,z], SimTime, rho) -> Result<BackgroundSample, DifferentialError>
differential(WorldPos, SimTime, rho) -> Result<BackgroundSample, DifferentialError>
differential_batch(points, SimTime, rho, output, scratch) -> Result<(), DifferentialError>
```

Domaine, densité, paramètres de B, capacité et résultat non fini sont des refus distincts.
output doit avoir exactement la taille de points ; scratch peut être plus grand.
Le lot prévalide les paramètres, calcule dans le scratch, puis copie vers output
seulement si tous les points sont reçus. Scratch peut changer lors d’un refus.
Les fonctions d’évaluation n’utilisent ni Vec, ni allocation, ni stockage persistant :
inspection du chemin complet (boucles, tableaux locaux, calcul de phase). Cette
propriété n’est pas déduite du compteur alloc_post_seal du harnais historique.

L’admission des directions autorise32 epsilon f32 autour de la norme carrée unité,
budget numérique de configuration, pas tolérance de direction physique. Les constructeurs
historiques restent inchangés ; une configuration non finie est refusée par le nouveau
chemin. L’hôte doit sélectionner un fond valable dans l’approximation profonde uniforme.

## Réception indépendante

Huit nouveaux tests de bibliothèque :

- exp(-x) comparée àexp f64 sur10401 valeurs x=0..104 par pas0,01 ; borne relative
  2e-6 plus une unité subnormale f32. Àx=0, facteur exactement1 ; sous-flux assumé.
- mono-composante : direction(0,6;0,8), crête, profondeur2m ; vitesse, accélération,
  termes croisés du gradient et pression vérifiés par formules f64 indépendantes.
- directions croisées : gradients spatiaux et temporels contre différences finies,
  divergence et symétrie du gradient. Pas spatial0,002m, temporel0,001s : budgets
  absolus5e-5 à4e-4 selon grandeur, couvrant arrondi/quantification et troncature
  centrale sur ce champ. Ces valeurs ne sont pas des seuils is_smooth_at.
- équation linéaire du mouvement : du_dt=-grad(p_dyn)/rho, gradient de pression
  évalué indépendamment par différences finies sur un spectre satisfaisant ω²=gk.
- raccord àz=0 : eta et les trois vitesses identiques bit àbit àeval_local ; temps0,
  123456789µs et u64::MAX, deux positions. deta_dt ancien égale nouvelle vitesse verticale.
- état nul, domaines invalides, densité non valide et paramètre B non fini refusés.
- lot égal aux appels ponctuels ; point tardif invalide et scratch trop petit laissent
  les sorties sentinelles intactes, lot vide valide.
- résultat non fini par débordement : aucune publication.

Le premier essai d’exponentielle a échoué àla borne relative2e-6 pour une très faible
valeur. Cause : produit n*ln2 arrondi avant soustraction. Réduction corrigée avec ln2
scindé en0,693145751953125 et le reste1,4286067653e-6 ; tolérance inchangée. Les tests
ont été relancés après ce correctif. Les polynômes et la réduction ne reçoivent pas
une précision uniforme des dérivées sur toutes les positions possibles par ces seuls essais.

Les scénarios check historiques sont reçus sans changement de leurs attentes :
C18-invariants hash0x85c8bc610f551d11 ; C02-dispersion hash0x0a3a3bcc945db263.
Leur alloc_post_seal=1 est le compteur du scénario historique, pas une allocation du
nouveau fournisseur. Aucune certification sur une autre plateforme n’est déduite.

Commandes depuis code/ :

```text
cargo test -p water-core background::differential
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Portée et suite

S176-1 construit B seul avec la portée d’ADR-113. Ni W, ni couplage solide ni δ3D ne
sont reçus ; B4 complet reste non reçu et A50 partielle. Le fournisseur ne remplit pas
la source entière : son p_dyn scalaire doit encore fournir son gradient au consommateur,
et le terme visqueux doit être qualifié. Aucun is_smooth_at permissif ajouté.

**Suite S178 : S177-1/A50**, étendre et recevoir le gradient de pression du B et la
formation de son résidu physique continu (termes et unités explicites, viscosité et
Laplacien qualifiés), avec les mêmes composantes et le même contrat de refus. Ce lot
complète l’entrée B du couplage avant son extension àW ; ne pas confondre cette source
continue avec un résidu discret du futur solveur. Porteur : construction de bibliothèque.

**Réception finale S177 :** workspace307 réussis,5 ignorés (water-core214/2 ignorés,
water-harness93/3 ignorés), aucun échec ; deux scénarios check reçus. Les huit nouveaux
tests font partie de ce total. Les avertissements existants du harnais et de wake_plafond
ne sont pas modifiés. Les hashs attendus des scénarios ne changent pas.
