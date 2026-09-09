# S111 — Reprise du journal vers la requête B+pression

2026-09-09. Exécutable `restart_pressure`, release avec assertions. Bibliothèque inchangée.

## Parcours reçu

Même source de virage que S103/S107 : σ1 m,coupure6/m,10 Pa, deux segments de2 s,
vitesse(2,0) puis(0,2), départ(0,0), observation0–8 s. Recettes128² et112×80.
Fond B16 composantes, Hs0,1 m,Tp6 s,direction0,125 tour,graine42 ; ancre(10⁹,-10⁹,0)m,
frame7/cell9, g9,81f32, densité1025, emprise[-8,12]². Plafond de pente0,1 de la fixture.
64 premiers points de la grille11², pas2 m, comme S107 ; pas un carré8×8.

Source directe → encodage WPRS196 octets → décodage dans un autre stockage → admission
sur journal sans place → Full et attente → instantané WPJR228 octets → restauration
sur pool à une place → current toujours refusé → retry explicite → préparation depuis
la recette et les segments restaurés → requête monde B+pression. Une seule source,
pas de composition multisource déguisée. Le journal initial à capacité zéro est un
cas de saturation déterministe, pas un dimensionnement de production.

Neuf instants :0,1,1,999999,2,2,000001,3,4,6,8 s. À chaque date et résolution, les dix
champs de WaterSample sont comparés en bits aux sorties construites directement depuis
la source initiale, sans codec ni journal ; énergie et puissance de pression également.
576 points-temps par résolution,1152 au total. Cette identité reçoit la reprise, pas
le modèle physique : les deux parcours emploient la même bibliothèque numérique.
Réception physique indépendante conservée dans S103/S104/S107.

L'instantané avant retry est identique à celui du journal bloqué. Source restaurée
identique en contenu. Dernier point invalide : sortie entière conservée puis requête
valide identique. Instantané tronqué refusé sur des pools candidats séparés : journal
et champ actifs restent utilisables. Hash à3 s :128² `52c1b810097014e8`,112×80
`70561fb44b0a4add`, identiques à S107. Aucun acquittement implicite dans la restauration.

## Mesures locales

Windows, AMD Ryzen AI7 350, rustc1.97.0, release ; trois échauffements et21 mesures.
Codec et instantané mesurés par lots de1000 opérations, valeurs divisées par1000 :
médianes de moyennes de lots, pas quantiles de latence individuelle. Premier essai
unitaire trop proche de la résolution du chronomètre écarté. Données chaudes répétées,
transport et disque absents. Mesures successives, pas de comparaison entre plateformes.

| Opération, médiane |128²|112×80|
|---|---:|---:|
|Encodage WPRS µs|0,0143|0,0130|
|Décodage WPRS µs|0,2132|0,2169|
|Encodage WPJR µs|0,0321|0,0292|
|Restauration WPJR µs|0,3999|0,4016|
|Préparation seule ms|6,7236|3,7799|
|Restauration + retry + préparation + requête64 ms|28,4023|17,0401|

Cycle complet mesuré directement : min/max26,4949/31,2692 ms et15,3402/17,9238 ms.
Les médianes des opérations ne sont pas additionnées. La cuisson, configuration de B,
création des pools et comparaison directe sont hors chronométrie. La restauration
mesurée inclut initialisation des petits tableaux de descripteurs/segments sur pile.
Elle ne recuit pas le spectre, déjà disponible dans ce scénario ; une reprise depuis
zéro incluant le chargement des assets ou du moteur aurait un coût supplémentaire.

236 tests réussis/cinq ignorés inchangés, suite non relancée pour ce seul instrument.
Campagne exécutée avec toutes assertions, formatage/diff vérifiés. Aucun nouvel appel
allocation dans le chemin bibliothèque par inspection ; compteur des services hôte
sans refus après scellement, pas compteur global Rust. Instrumentation allouée hors
mesures.76 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.

## Suite

**S110-1 réalisée sur ce scénario. S111-1, S112 :** construire la superposition de
plusieurs sources de pression compatibles admises au journal, ordre déterministe,
interférences conservées dans énergie/puissance, refus de contexte incompatible et de
journal bloqué avant publication. Recevoir deux sources distinctes et leur travail
total ; ne pas sommer leurs énergies isolées. L'intégration des impacts et le stockage
durable restent ouverts. Le coût élevé de la requête reste mesuré, sans budget certifié.

**Mise à jour S112, 2026-09-09 :** S111-1 réalisée comme construction du champ commun,
voir [MULTISOURCE-S112](MULTISOURCE-S112.md). Suite S112-1 : réception spatiale et coût.
