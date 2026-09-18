# ADR-165 — Flux de bande du fond aux frontières latérales

Actée S270, 2026-09-18, autonomie S71. Étend le transport ADR-152 aux frontières
latérales du pas perturbatif mobile ; aucun changement du pas total S237.

## Constat et décision

`transport_coupled` ferme Q_v et la bande du fond aux deux extrémités. Pour un
courant uniforme U et une élévation uniforme a du fond analytique, v=η'=0 doit
rester stationnaire. La bande vaut Ua partout : la fermer au bord crée pourtant
±dt Ua/dx dans les colonnes extrêmes. Ce défaut ne dépend pas de la dispersion.

Conserver v_n=0 aux frontières, mais intégrer la bande du fond sur leurs faces u,
avec les échantillons U et ζ_fond déjà fournis. La surface à la frontière vaut
`repos + η'_colonne_voisine + ζ_fond_face` : prolongement constant de η' (Neumann),
fond évalué à la face. Sa hauteur doit être constante verticalement dans chaque
colonne de faces u extérieure, comme celle des faces w dans ADR-152 ; incohérence
refusée par BackgroundContext avant publication. Le débit est la somme par couche
`U_face * (longueur_mouillée(surface) - longueur_mouillée(repos))`, tronquée par le
fond solide et les limites de la couche. La fermeture de projection `open_u=0`
reste propre à v et ne masque pas ce débit prescrit du fond.

Calculer les deux débits depuis l'état ancien avant de modifier η'. Les flux
intérieurs, l'ordre de sommation et la compensation restent inchangés. Fond nul :
débits nuls, identité S237 conservée. Le témoin sans résidus S253 éteint aussi ces
flux. Contrôle de budget, validation et rollback englobent les nouvelles boucles.

## Portée

Cette correction autorise le transport de la bande prescrite à travers les bords.
Elle ne reçoit pas la condition extérieure universelle de v, ni le fond solide
traversant W(b)≠0, ni la propagation non linéaire d'une houle incidente entière.
L'éponge ADR-164 reste responsable de réduire la perturbation près des bords.
Aucun nouveau paramètre, stockage persistant ou allocation dans le pas.

[Réception](../validation/FOND-TRAVERSANT-S270.md).

**Note S273, 2026-09-18.** Le débit par couche `U_face * (longueur mouillée…)` est remplacé par
la quadrature linéaire d'[ADR-166](ADR-166-quadrature-lineaire-de-la-bande.md), commune aux faces
intérieures et extérieures ; troncature par le fond solide et le reste de la décision inchangés.
