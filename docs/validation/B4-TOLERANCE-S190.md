# B4 — Réception au seuil de 2 %, S190

2026-09-12. Arbitrage utilisateur acté par [ADR-120](../adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).
**Protocole déclaré avant exécution.** Le résultat sera ajouté après le rejeu.

## Conditions et décision

Véhicule et fournisseur partagés de S185–S189 : bloc 16³, 2744 cellules intérieures,
dx = 0,25 m, z de −4,05 à −0,80 m ; B16, impact et pressions du montage existant.
Repos initial, 100 pas de 10 ms, T0 = 1,5 s, T final = 2,5 s ; advection/viscosité
et soustraction de S, sans projection ni surface libre. Aucun changement du support.

Référence R : source pleine à chaque pas, chargée directement. Référence R2 : même
montage, 200 pas de 5 ms. Norme maximum composante par composante, normalisation
commune M = max|R(T)|. Réserve mesurée q = max|R−R2|/M ; ce n'est pas une borne
rigoureuse de l'erreur au continu. Le seuil de 2 % ne reçoit ici que ce montage.

Réseaux : plein ; ancrés isotropes 8³ et 5³ (témoins S188) ; gradués verticaux de
S189 à horizontale pleine, Nz = 3, 4, 5, 8 ; deux candidats mixtes ancrés horizontalement
à 8 nœuds par axe et gradués verticalement à Nz = 6, 8. Graduation issue du profil
de courbure au premier instant, comme S189. Cadences 1, 2, 4, 8, 16, 32, 64 ;
maintien et extrapolation causale, démarrage en maintien inclus. Pas d'instantané futur.

Pour chaque couple, mesurer s (spatial seul), t (temporel seul), e (composé), e2
(contre R2), tous divisés par M. **Reçu si s+t+q ≤ 0,02 ET e+q ≤ 0,02 ET e2 ≤ 0,02**,
avec toutes les valeurs finies et M > 0. Publier séparément ces nombres : un refus
par budget peut coexister avec un e inférieur à 2 % ; c'est le refus de financer une
compensation, pas une mesure cachée. La comparaison e2 ≤ e+q contrôle la triangulaire.

Choisir parmi les couples reçus celui qui minimise **le nombre exact d'évaluations
ponctuelles de source** `nœuds × ceil(100/c)` ; égalité départagée par e+q puis ordre
du tableau. C'est un décompte sur cette fenêtre, pas une mesure de temps CPU ni un
optimum sur tous les réseaux. Afficher aussi le rapport au plein (274400 évaluations).

## Contrôles requis

1. Plein/cadence 1 identique en bits à R dans les deux modes.
2. Réserve q reproduisant le contrôle S185 (environ 0,386 %) ; s des témoins ancrés
   reproduisant 1,7160 % et 3,6805 % à l'arrondi publié.
3. Au moins un couple dégradé reçu et un refusé ; omission totale de S refusée
   (erreur 100 % puisque le véhicule démarre au repos).
4. Refus des non-finis et du dénominateur nul, seuil inclusif exercé à 2 % et juste
   au-dessus. Toutes les cellules et sources contrôlées avant les maxima.
5. Exécutions répétées, sorties numériques identiques ; comparaison debug/release.
   Les résultats sont ceux de ce programme, aucun historique remplacé.

## Limites maintenues

Ce test ne mesure ni forces, ni perception, ni hauteur de surface. Le contrôle temporel
ne qualifie pas l'erreur spatiale du solveur et ne fournit pas une référence Navier–Stokes.
Un réseau/cadence reçu ici est un **profil de banc**, pas un défaut universel du runtime.
Les 2 % sont décidés indépendamment de l'issue de cette réception.
