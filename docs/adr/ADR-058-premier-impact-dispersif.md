# ADR-058 — Première expansion dispersive d’un impact, à domaine explicite

- **Statut : ACTÉE**, S75, 2026-09-08, délégation technique.
- **Produit** `impact_field.rs`, première tranche W3, sans allocation.
- **Précise** ADR-054 : premier candidat analytique fini et périodique avant paquet régional.

## Représentation et domaine

Un Impact isotrope validé devient 40 modes de surface dans un carré périodique de côté
L = 4 lambda. Indices : nx=0, ny=1..4 puis nx=1..4, ny=-4..4 ; aucune composante moyenne,
aucune paire de cosinus redondante. Ce support et la pondération
`w = 1/(1+(sqrt(nx²+ny²)-4)^4)` sont **des paramètres de source à calibrer par B2**, pas une
loi d’impact dérivée. Ils permettent une première expansion exécutable à énergie connue.
Le maximum initial se situe à la position horizontale de la source, avec vitesse initiale nulle.
Le volume déplacé audio ne détermine pas cette forme ; seul energy_j normalise son énergie.
La coordonnée verticale décrit la cause et ne déplace pas le plan de surface du milieu.

Gravité g, densité rho, profondeur et limite de pente sont injectées. Eau profonde seulement :
h > L/2, donc h > lambda_mode/2 pour tous les modes (SPEC-001 §1). L < 4096 m ; coordonnées
et écarts locaux bornés comme I-08. Anisotropie refusée. La borne de pente est la somme
`sum |a_k| k` ; limite fournie par le profil, **à calibrer B2**, jamais valeur de production
implicite. Cette borne conservative peut refuser un champ dont les pentes réelles se compensent.

Le domaine périodique répète la source : **ce n’est pas encore un impact isolé régional**, ni
une preuve de C07, C19 ou B2. La troncature carrée ne garantit pas une isotropie angulaire exacte.
Cette représentation sert de première expansion analytique contrôlable ; elle ne remplace pas
le paquet régional sans répétition prévu par ADR-054. Aucun candidat B2 n’est sélectionné.

## Propagation et énergie

SPEC-001 §1 donne omega²=gk en eau profonde. Pour chaque mode :

- eta = a cos(k·r) cos(omega t) ;
- dérivée temporelle = -a omega cos(k·r) sin(omega t) ;
- potentiel de surface psi = -a omega/k cos(k·r) sin(omega t).

Les phases temporelles utilisent SimTime entier et PhaseQ32 ; jamais de temps f32. La fréquence
est quantifiée en Q32, tandis que omega nominal sert aux amplitudes des dérivées : erreur de
quantification à borner avant réception interplateforme/longue durée. Pas de transcendantes
système pendant sample ; racines f32 à la construction. Déterminisme entre machines non testé.

Par orthogonalité des modes distincts sur le carré, leur énergie vaut
`E = rho*g*L²/4 * sum a_k²`. La normalisation choisit donc
`a_k = w_k sqrt(4 E/(rho g L² sum w_k²))`.
La validation indépendante intègre sur une grille la densité
`rho/2 * (g eta² + psi * deta_dt)` ; la seconde partie représente l’énergie cinétique de
l’écoulement potentiel profond. Le volume signé intégré est nul à tous les instants testés.
La formule est celle du modèle linéaire ; elle ne revendique ni impact non linéaire ni déferlement.

Avant naissance, le champ est nul. Jusqu’à ttl inclus, il est évalué directement à l’instant
demandé, sans historique. Au-delà, le module **refuse** : il ne prétend pas que l’énergie a
physiquement disparu. Aucune purge du journal n’en découle. L’injection à naissance est une
condition initiale imposée, pas la résolution du contact solide/eau.

## Vérification et limites de réception

Trois tests : énergie intégrée et volume nul sur 32×32 points à 0, 1, 7 et 40 secondes ;
projection spatiale indépendante du mode (4,0) et fréquence analytique f64 ; refus hors domaine,
profondeur, pente et temps. Tolérances numériques de ces essais : 2e-5 relatif énergie,
1e-5 projection et somme de hauteur ; contrôles de calcul, pas seuils de qualité B2.

Le scénario initial de 100 J dépassait la borne de pente 0,1 choisie pour l’essai. Il est ramené
à 1 J ; le contrôle reste inchangé. Le test de refus emploie une borne 1e-10. Ces valeurs sont
des paramètres de tests, pas des profils approuvés du jeu.

**Non acquis :** transport de l’enveloppe à la vitesse de groupe, isotropie convergée, séparation
source/répliques périodiques, vitesse orbitale raccordée à WaterSample, interrogation B+W,
référentiels mobiles, interface de journal vers le champ et budget. La fréquence d’un mode et
la conservation d’énergie ne prouvent pas ces propriétés. S74-1 reste partielle. Prochaine
session : mesurer le transport radial et les retours périodiques avant de choisir le support
régional. Aucun invariant amendé ; I-07/I-08, I-14 et I-17 relus.

> **Actualisation S76 — 2026-09-08.** ADR-059 refuse ce support comme impact régional isolé :
> copie exacte dès naissance à L, mesurée à 16 m. Énergie radiale mesurée avec la densité
> cinétique intégrée en profondeur ; les contrôles d’énergie totale ci-dessus restent valides.
> Prochaine construction : candidat radial à définition continue non périodique.
