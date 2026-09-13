# ADR-131 — Un dépassement de budget qualifie une implémentation ; le budget s'éprouve sur la combinaison des optimisations

- **Statut : actée**, S213, 2026-09-13, **clarification explicite de l'utilisateur** (réponse au
  compte rendu de S212).
- **Précise [ADR-127](ADR-127-ambition-complete-construction-progressive.md) D7** (note datée
  ajoutée, ADR-127 non réécrit). Laisse [ADR-125](ADR-125-budget-image-60hz-deux-ms.md) entier :
  60 images/s et eau 2 ms restent le profil.
- **Corrige le cadrage** de [HOTE-GPU-S212](../validation/HOTE-GPU-S212.md) et de ses
  propagations (note corrective datée, texte d'origine conservé). Voir A252.
- Trajectoire tenue dans [FEUILLE-DE-ROUTE](../FEUILLE-DE-ROUTE.md) §2 (J1) et §3.

## 1. Ce que l'utilisateur a demandé

> *Continue avec la piste CPU en S213, mais corrige le cadrage : c'est l'implémentation actuelle
> qui dépasse le budget, pas le sillage ni l'objectif final qui sont refusés.*

Et, dans le même message : les deux pistes de S212 sont pertinentes mais ne définissent pas les
possibilités d'optimisation — intégrer à la trajectoire **les LOD spatiaux, spectraux et
temporels, la visibilité et la mutualisation des calculs** ; *il ne faut pas attendre l'échec de
deux optimisations pour me demander de réduire l'ambition* ; chaque mesure précise **les
techniques présentes, celles encore absentes et le domaine de validité** ; le budget de 2 ms reste
**un objectif à éprouver sur leur combinaison, sans présumer qu'elle réussira ou échouera** ;
A251 et la vérification de la composition impact+sillage restent dans les travaux nécessaires —
*accélérer le rendu ne doit pas masquer ses limites de validité.*

## 2. Ce qui était faux dans S212

HOTE-GPU-S212 écrivait « exactitude reçue, **coût refusé** », « le chemin … est refusé en coût »,
et bornait la suite à deux leviers : « si les deux leviers mesurés ne tiennent pas 2 ms,
l'arbitrage revient à l'utilisateur ». Le premier énoncé attribue à la fonctionnalité un verdict
qui porte sur une implémentation dépourvue de presque toute optimisation ; le second transforme
deux essais en compte à rebours vers une demande de réduction. C'est le mécanisme d'A248 — un
ordre ou un budget lu comme un périmètre — sous une autre forme. Les mesures, elles, restent
justes.

## 3. Décision

**D1 — Un dépassement qualifie l'implémentation mesurée.** Un coût au-dessus du budget se dit de
la combinaison de techniques mesurée, sur son domaine : « *l'implémentation X, avec telles
techniques et sans telles autres, dépasse le budget sur tel domaine* ». Il ne se dit jamais d'une
fonctionnalité, d'une couche ou d'un objectif du produit. « Reçu » et « refusé » restent réservés
aux critères déclarés (tolérances, exactitude), pas au budget.

**D2 — L'espace d'optimisation du rendu est nommé dans la trajectoire, et il est ouvert.** Au
2026-09-13 il comprend au moins :

| technique | ce qu'elle vise dans les lois mesurées en S212 | ce qu'elle doit publier avec elle |
|---|---|---|
| **temps** — ne plus préparer par image (tronçons achevés repliés, modes préconstruits) | CPU ∝ nœuds × tronçons | écart au chemin préparé ; pic aux bornes de tronçon |
| **espace** — grille locale et transformée | GPU ∝ sommets × nœuds | quadrature d'image contre le cœur ; coutures et période de grille |
| **LOD spatial** — densité et emprise selon la distance et la taille à l'écran | sommets ou texels évalués | densité de Nyquist par distance (L283) ; coutures entre niveaux |
| **LOD spectral** — nœuds par source selon distance et visibilité | nœuds (CPU et GPU) | écart à la recette pleine ; **durée et rayon honnêtes réduits** (ADR-107) |
| **LOD temporel** — cadence de mise à jour selon distance, vitesse, régime | CPU par image | erreur de phase entre mises à jour ; I-09 (on avance des paramètres, on n'interpole pas des réalisations) |
| **visibilité** — frustum, occlusion, hors écran, projection de l'emprise | tout calcul d'une source invisible | exactitude au retour dans le champ |
| **mutualisation** — calculs communs aux sources et aux couches (nœuds partagés par sources de même recette, phases communes, passe et grille communes B/W) | coût par source ajoutée | exactitude de la superposition dans le domaine linéaire (ADR-123 hors de ce domaine) |

La liste n'est pas close : une technique découverte s'y ajoute. Aucune n'est présumée suffire ni
échouer.

**D3 — Chaque mesure de coût publie trois choses** : les **techniques présentes**, les
**techniques absentes** (au regard de D2), et le **domaine de validité** du chiffre (scène,
recette, nombre de sources, formats, instants, machine, grandeur exacte mesurée). Une mesure qui
ne les donne pas ne se confronte pas au budget.

**D4 — 2 ms est un objectif à éprouver sur la combinaison.** Le budget d'ADR-125 se confronte à la
combinaison des techniques de D2 sur des scènes représentatives, sans présumer du résultat. Un
levier seul ne « tient » ni n'« échoue » le budget : il déplace une loi, et se mesure comme tel.

**D5 — Aucune demande de réduction d'ambition ne se fonde sur l'échec d'optimisations prises
isolément.** Le moment d'un arbitrage n'est pas fixé par un nombre de leviers essayés. Si une
incompatibilité persiste sur une combinaison mesurée, elle se présente selon ADR-127 D7 (scène,
mesure, options, ce que chacune dégrade), avec ce qui était absent de la combinaison ; réduire le
périmètre reste une décision de l'utilisateur qui nomme ce qui est retiré (ADR-127 §6). Un LOD
est une dégradation au sens d'ADR-012 et I-05 — un mécanisme, pas une suppression — et publie son
erreur.

**D6 — Accélérer ne masque pas les limites de validité.** Tout chemin accéléré se reçoit contre
le chemin de référence du cœur, et conserve ses contrôles de validité. Restent **travaux
nécessaires de J1**, indépendants du coût : **A251** (durée honnête et coutures d'un sillage
visible) et la **composition impact + sillage** par le cœur (budget conjoint, ADR-119).

## 4. Ce que cette décision ne fait pas

Elle ne change ni le profil (ADR-125), ni le seuil de 2 % (ADR-120), ni aucun invariant. Elle ne
choisit aucune technique, ni leur ordre au-delà de la suite demandée (temps en S213). Elle ne
déclare pas que la combinaison tiendra 2 ms. Elle ne rouvre aucune mesure : les chiffres de S206 à
S212 sont exacts ; seuls leurs énoncés de verdict changent de sujet.

Invariants relus : I-05 (dégradation), I-06 (allocations), I-08 (précision, phases repliées),
I-09 (interpolation de paramètres), I-12 ; aucun amendé.
