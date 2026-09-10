# ADR-100 — Un spectre de fond avec une bande explicite

- **Statut : actée**, S147, 2026-09-10, délégation technique S71.
- **Applique** ADR-004 §2.2 (`gamma` était déjà prévu) ; **A212 partiellement traitée**.
- **Complète** ADR-099 : ses résultats restent ceux du fond historique ; ils ne reçoivent
  pas une autre répartition spectrale. Aucun résultat historique n'est réécrit.
- Réception de conception : [SPECTRE-FOND-S147](../validation/SPECTRE-FOND-S147.md).

## Problème

Le constructeur actuel distribue des amplitudes égales à des fréquences géométriques.
Il représente une énergie uniforme **en log-fréquence**, soit une densité limite `1/f`.
Son `tp` centre une bande, sans définir de pic. Hs exact ne reçoit donc pas son spectre.
ADR-004 prévoyait déjà un pic JONSWAP ; son absence dans `SeaState` est un écart de construction.

Remplacer les seules amplitudes serait insuffisant : dans la bande historique `[fp/2,2fp]`,
le modèle JONSWAP γ=3,3 perd **4,93 % de m0 mais 24,14 % de m2** avant renormalisation.
Rétablir Hs masque la première perte, sans réparer la seconde.

## Décision

1. Le premier candidat spectral de mer de vent suit **JONSWAP**, forme et unités définies
   dans SPEC-001 §1 bis ; γ est un paramètre explicite, γ=1 inclut la forme PM modifiée.
   Ce modèle unimodal n'est pas une description universelle des houles croisées ou de l'eau
   peu profonde. Ces régimes ne deviennent pas valides en changeant γ.
2. La bande finie fait partie du descripteur du candidat. **Hs est celui de la bande
   représentée**, et les fractions retranchées de m0 et m2 sont publiées comme diagnostics.
   Aucun repli silencieux d'une bande ou d'un paramètre invalide. Le premier montage à
   recevoir sera `[0,5fp,4fp]`, γ=1/3,3/7 et N=32/64/128/256 : c'est une **fixture**,
   pas une calibration des mers du jeu. La valeur 4 est motivée par la mesure S147 ; elle
   laisse encore 6,18 % de m2 hors bande à γ=3,3.
3. Les poids viennent de l'**intégrale de densité par cellule logarithmique**, puis de
   `a_i=√(2e_i)`. La fréquence représentative initiale est le centre géométrique. Les
   erreurs de discrétisation et de troncature sont reçues séparément ; augmenter N ne
   récupère pas une fréquence située hors bande.
4. La voie uniforme existante reste le véhicule H1 historique pendant la construction
   d'un **constructeur spectral explicite**. Pas de migration implicite des scénarios ou
   des sauvegardes. La cuisson doit livrer des coefficients reproductibles avant de
   revendiquer I-03 : les `exp`/`powf` f64 de l'instrument ne sont pas un runtime autoritaire.
5. Les 32 composantes d'ADR-099 sont un **point de départ** pour ce nouveau candidat.
   Ses moments, statistiques sur plusieurs graines et budgets de pente doivent être reçus
   avant d'y transposer les conclusions de B1. Une intégrale correcte ne reçoit ni la
   corrélation spatiale, ni la réponse d'un corps, ni le LOD.

## Ce qui est construit et ce qui pourrait infirmer le choix

S147 construit un instrument f64 hors runtime, reçu contre les intégrales fermées PM,
avec raffinements et contre-épreuve du Jacobien. Les moments m1/m2/m4 du prototype à
32 cellules diffèrent de moins de 0,186 % de leur référence **dans la bande** sur les
six fixtures. Cela autorise la construction du candidat, sans le déclarer déjà intégré.

Une recette dont les moments ou la densité autour du pic ne convergent pas à N croissant
réfute la discrétisation. Une erreur de pente acceptable dans la bande mais inacceptable
après élargissement réfute la bande pour cet usage. Un hachage différent selon la plateforme
refuse la cuisson, sans tolérance de remplacement. I-01 à I-18 sont conservés.

## Ce qui reste ouvert

**S147-1, prochaine session :** construire la configuration spectrale explicite, valider
entrées et coefficients avant publication, recevoir la fixture ci-dessus contre l'instrument
S147, puis B+W, pente et rejeu. Garder la voie historique pour les témoins de migration.
Étudier la cuisson déterministe avant d'utiliser des fonctions transcendantes supplémentaires.

**A212 reste partielle** jusqu'à cette construction et sa réception. Le choix des bandes
du jeu et la distribution directionnelle ne sont pas reçus. La relation fréquence/direction
du constructeur historique (une direction par fréquence, éventail de 30°) n'est pas un
spectre directionnel JONSWAP reçu. Le sillage W4 et B2 restent la trajectoire système,
distincte de ce lot B ; S63-1 est close par son constat propre.
