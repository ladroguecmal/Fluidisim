# Cuisson du sillage — S267

## Contrat avant construction

Consommateur : cuisson de la grille de l'hôte, avant chaque rendu, mêmes huit bandes
ADR-148 et reconstruction S234. La boucle additionne chaque mode dans un tableau
local indexé dynamiquement. Hypothèse à mesurer : huit accumulateurs nommés avec
sélection explicite évitent un coût d'adressage/spill du compilateur. Ce mécanisme
n'est pas tenu pour acquis sans mesure. Aucun changement de fréquence, de modes,
de pas de grille ni d'ordre des sommes à l'intérieur d'une bande.

Critères : témoins conservés ; sept images S266 identiques au bit ; comparaison des
neuf grilles GPU au bit aux âges 1/3/12/16 s, avec/sans filtre, poses référence et
rasante, et retour à un instant déjà évalué ; tests hôte et réception spectrale S249
(3 mm) conservés. Coût : secteur début/fin, aucune autre charge GPU du projet,
1280×720, vent 5, reflets filtrés, 120 images après dix de chauffe, référence/rasante.
Retenir si réduction médiane de cuisson >=25 % et eau totale >=10 % aux deux poses.
Ces seuils de rentabilité sont des choix de lot, pas des constantes physiques.
Si l'identité échoue : diagnostiquer, ne pas assouplir le seuil. Si le coût échoue :
rejeter cette variante et déclarer la nouvelle piste avant construction.

I-09 : aucun mélange temporel, réalisation toujours évaluée à l'instant demandé.
I-03 : cœur et ordre modal inchangés ; GPU cosmétique local, pas de réception
multiplateforme. I-06 : mêmes buffers réservés, pipeline créé à l'initialisation.

Comparaison à J2 : les bords ouverts couplés débloquent une capacité physique absente.
Le présent lot se borne au poste dominant du rendu accepté encore au-dessus de 2 ms ;
une fois ce poste traité ou la piste rejetée, revenir à J2 avant nouvelle calibration
cosmétique. C'est un changement de poste, pas un troisième affinage des reflets.
