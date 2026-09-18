# Réflexion du paquet MAC — S269

## Protocole avant mesure

Consommateur : `step_perturbation_mobile` avec fond nul, éponge ADR-164 ; premier
volet des bords ouverts, sans réception de fond traversant. Comparer trois domaines :
24 m avec éponge de 4 m, 24 m sans éponge (mur), 48 m sans éponge (garde).
Profondeur 1 m, air jusqu'à 1,5 m, gravité 9,81, densité 1025 ; paquet d'amplitude
crête initiale 2 mm centré en x=8 m, jauge x=12 m. Somme d'ondes progressives vers
+x, k de 1 à 5,5 rad/m par 0,05, poids gaussien centré π et écart-type 0,6.
`ω²=gk tanh(kh)` ; vitesses initiales depuis le potentiel linéaire incompressible.
Ce spectre et ces dimensions sont des fixtures déclarées, pas des paramètres produit.

Largeur 4 m = deux longueurs d'onde centrales ; taux `10 cg(k0)/4` selon ADR-046,
à vérifier ici sans hériter de son reçu 1D. dx=0,25 m puis 0,125 m, dt=5 ms.
Fenêtre incidente [0,12] s, fenêtre réfléchie [14,36] s. Les arrivées centrales
attendues valent environ 4,5 et 22,5 s ; la garde doit constater les queues réelles.
Mesure énergétique à jauge : R=sqrt(integrale eta² retour / integrale eta² incident).
La crête seule ne mesure pas la réflexion d'un paquet dispersif.

Critères avant campagne :
- pas tous reçus, aucune valeur non finie ; traces publiées, refus conservé ;
- garde : sqrt(E_garde_retour/E_garde_incident) <=0,001 (10 % du seuil R=1 %) ;
- témoin mur : R>=0,5, pour refuser un paquet disparu avant le bord ;
- critère candidat R<=0,01 issu ADR-046, aux deux mailles ; baisse de dx ne doit
  pas changer R de plus de 0,002 absolu. Sinon résultat non convergé, pas reçu ;
- si garde ou solveur échoue : arrêter la réception, diagnostiquer avant nouvelle
  campagne. Un chiffre brut peut être publié, mais pas appelé coefficient de réflexion.

Arrêt : réception bornée ou cause de refus démontrée avec prochaine action concrète.
Aucune modification des seuils après mesure. Pas de promesse de coût temps réel.


## Première garde et diagnostic déclaré avant suite

Garde dx=0,25 : tous les 7 200 pas reçus, mais rapport tardif **0,019668** >0,001.
Énergie incidente 7,1510674e-6 m²s ; retour 2,7662244e-9 m²s. Le paquet linéaire
analytique aux mêmes fenêtres donne **8,8193e-5**, inférieur au seuil. Le signal
numérique tardif n'est donc pas la simple queue du paquet analytique.
Aucune réflexion de l'éponge n'est encore mesurée ni déclarée.

Deux diagnostics distincts : (1) garde dx=0,125, pour mesurer l'effet spatial ;
(2) même garde dx=0,25 en reculant seulement le mur gauche de 24 m, paquet/jauge/bord
droit inchangés en coordonnées physiques. Celui-ci distingue le retour d'une onde
parasite partie à gauche des autres erreurs du paquet. Aucun seuil de réception changé.


## Diagnostic et changement d'instrument, avant mesure de l'éponge

Garde fine dx=0,125 : rapport brut 0,0077458, encore refusé. Garde avec mur gauche
reculé : 0,0137305 au pas 0,25. Les traces montrent deux contributions : queue
numérique à 14–18 s (fortement réduite par le raffinement), puis onde revenue du
mur gauche (supprimée en reculant ce mur). Leurs origines détaillées dans
l'initialisation/discrétisation ne sont pas complètement attribuées.

La mesure brute d'ADR-046 est donc **non recevable pour ce montage**, et le reste.
Nouveau protocole déclaré avant de lancer mur/éponge : mesurer la réponse au bord
par **différence temporelle au témoin long**, même pas et même bord gauche :
`R_diff=sqrt(integrale (eta_cas-eta_garde)^2 retour / E_incident_garde)`.
Ce nombre mesure l'effet du bord droit relativement au domaine long, pas l'énergie
absolue de tout le signal tardif. Il ne reçoit pas la propagation du paquet initial.

Garde supplémentaire obligatoire : comparer deux domaines longs, 48 et 72 m, avec
même bord gauche et mêmes fenêtres. Leur différence normalisée doit rester <=0,001
(seuil initial conservé). Avant 12 s, cas et garde doivent aussi différer de <=0,001
en norme énergétique incidente. Sinon l'instrument refuse. Enregistrer chaque pas
(5 ms), pas les seules traces à 250 ms. Garder R_mur>=0,5, R_eponge<=0,01 et
convergence entre dx=0,25 et 0,125 à 0,002 absolu. Ne pas appeler reçu un résultat
qui manque l'un de ces critères. Cette différence ne change aucun champ du solveur.
