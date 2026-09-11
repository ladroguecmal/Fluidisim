# Source par flux partagés — S171

## Protocole déclaré

Suite S170-1/A225, véhicule Saint-Venant1D mouillé, sans choix du runtime.
Même onde, Q figé, fenêtres, fantômes analytiques, moyenne de cellule et budget physique
que SOURCE-DECIMEE-S170. La sonde `source_decimee` conserve ses témoins Exact,
Omitted et Linear et ajoute FluxRaw et FluxAnchored.

FluxRaw interpole linéairement F(Q) sur les nœuds grossiers. Chaque face fine est
évaluée une fois ; S_i=(Fhat_i-Fhat_{i+1})/dx. La somme télescope :
Σ(S_i-Sexact_i)dx=Fhat(30)-Fhat(90)-[F(Q(30))-F(Q(90))].
La télescopie seule n'annule donc pas l'erreur du budget physique aux bornes.

FluxAnchored remplace les segments de bord par une interpolation passant par les
flux physiques exacts à30 et90m ; les nœuds strictement intérieurs sont conservés.
Aucune correction uniforme de source. Le coût supplémentaire est l'accès aux flux
exacts aux bornes ; leur disponibilité n'est pas établie pour le runtime.
Le compteur nodes est le nombre de nœuds retenus, pas un coût de calcul : le constructeur
expérimental évalue aussi des nœuds extérieurs avant de les filtrer.

H=1,2,4,8,16m, phase0/H/2 ; N=120,240,480 et pas de temps divisé par deux àN240.
128 évolutions. Comparer erreur locale de source, hauteur/débit face à la référence,
écart de champ au témoin Exact et défaut de volume prédit par l'injection nette.
La source issue du flux linéaire est constante par segment grossier : elle peut
conserver son intégrale tout en étant localement moins précise que Linear.
