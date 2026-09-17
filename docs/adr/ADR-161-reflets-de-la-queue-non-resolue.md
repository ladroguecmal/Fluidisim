# ADR-161 — Filtrer les reflets de la queue non résolue

Actée S265, 2026-09-17, autonomie technique S71. Réponse au verdict R6 : les zones entre
les pics moyens et petits paraissent trop rugueuses. Complète ADR-155/157/158/160.

## Constat et périmètre

Le shader coupe des pentes, puis évalue un miroir par pixel. La variance retirée disparaît de
l'éclairage. Le calcul non linéaire du reflet n'est pas filtré : une pente proche de Nyquist
peut donc encore produire du grain. Cela se constate dans `ocean_fragment` ; ce n'est pas une
preuve que ce défaut explique à lui seul le verdict. Aucun choix de vent reçu.

## Décision

Variante d'hôte `--reflets-filtres`, sur `--vagues` :

1. Filtrer la queue par `w = spectral_weight(k,h) exp(-(kh)^2/2)`, où `h` est le maximum des
   dérivées écran du **point de Lagrange**. Le gaussien a un écart type d'un pixel : choix de
   reconstruction numérique déclaré, pas une longueur physique ni un réglage de vent. La
   coupure Nyquist existante reste stricte. Les hauteurs et déplacements restent inchangés.
2. Conserver la covariance manquante `C = somme (1-w²) a²/2 · k kᵀ` des pentes linéaires de la
   queue. L'énergie de modulation multiplie C, puis le jacobien résolu la transporte en
   coordonnées eulériennes. C'est une approximation au premier ordre : les fluctuations du
   jacobien non résolu et les moments supérieurs de CWM ne sont pas reconstruits.
3. Intégrer l'éclairage existant sur une distribution gaussienne de pentes, covariance C,
   par quadrature de Gauss–Hermite 3×3. Une variante 5×5 sert de contrôle de convergence.
   Les poids sont positifs et de somme un. Variance nulle : miroir existant exact.
4. C'est une **fermeture du rendu**, inspirée de la transition normales/BRDF de
   [Bruneton, Neyret et Holzschuch (2010)](https://doi.org/10.1111/j.1467-8659.2009.01618.x).
   Ce lot n'implémente pas leur BRDF complète : ni masquage microfacette, ni diffusion multiple,
   ni réception radiométrique du ciel de banc. Le résultat visuel doit être jugé par l'utilisateur.
5. Sans l'option, chemin historique conservé. Ni spectre, ni modulation M, ni requête de jeu
   modifiés. Comparaison à 5 m/s comme témoin médian de R6, sans adoption de ce vent.

## Réception et arrêt

Critères écrits avant construction dans [REFLETS-S265](../validation/REFLETS-S265.md).
La conservation des seconds moments est une propriété de la fermeture linéaire, pas une
nouvelle réception Cox–Munk du champ CWM. Un succès numérique ne reçoit pas le visuel.
