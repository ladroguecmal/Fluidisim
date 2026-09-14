# Surface linéarisée — S233, 2026-09-14

## Protocole avant construction

ADR-141 : pression puis hauteur, géométrie fixe et modèle linéaire sans advection.
Q_i = somme_k ouverture_u(i,k) u(i,k) dx ; η_i nouveau = η_i − dt(Q_{i+1}−Q_i)/dx.
Faces latérales fermées : somme des déplacements nulle en arithmétique exacte.
La somme se télescope indépendamment du résidu de projection ; vérifier sa dérive d'arrondi.
Les flux sont ceux du candidat, pas d'un second solveur de surface.

Oracle indépendant : onde stationnaire linéaire en bassin rectangulaire, η−z₀ =
A cos(kx) cos(ωt), k=π/L, ω²=gk tanh(kh), dispersion de SPEC-001 §1.
L8m, h4m, fond0, A0,01m, g9,81 et1,62 ; vitesse initiale nulle. Comparer sur1s,
résolutions16/32/64 et pas2ms/1ms. Publier erreurs maximales normalisées par A,
réduction de l'erreur et masse ; critère de qualité1% sur le cas fin, tolérance de banc
déclarée ici (pas seuil de bascule B4 ni admissibilité universelle).
Témoin discriminant : hauteur imposée inchangée ne peut reproduire cos(ωt).

Recevoir repos exact, évolution/retour de signe et pression consommant η modifiée ;
non-convergence et tout point d'expiration doivent préserver les quatre champs puis
autoriser une reprise identique, sans allocation. Le mode exige un couvercle entièrement
ouvert, g positif et une durée satisfaisant dt²g/dx≤1, garde conservatrice du schéma linéaire
sur grille fixe ; aucune stabilité non linéaire revendiquée. Temps en microsecondes entières.
Le stockage supplémentaire, le coût complet et les limites sont publiés à la réception.
