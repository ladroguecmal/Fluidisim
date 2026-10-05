# Anomalies ouvertes

*Généré par `python outils/anomalies.py --ecrire` (S480) — ne pas modifier à la main ; on écrit dans le
[registre des angles morts](ANGLES-MORTS.md), seul.* Lu : les entrées dont l'en-tête porte un statut (depuis ≈ S280) et leurs
suites datées ; l'état est lu par mots sur la dernière suite qui tranche (voir l'outil). Les entrées plus anciennes, au tableau
général du registre, ne sont pas lues ici.

**42 entrées lues — 24 ouvertes, 18 closes.** Sévérité : 1 = refonte d'architecture si découvert
tard, 2 = refonte d'un sous-système, 3 = travail localisé.

## Ouvertes

| anomalie | sév. | ouverte | titre | dernière suite |
|---|---:|---|---|---|
| A295 | 1 | S293 | L'architecture d'exécution de δ ne peut pas porter un domaine 3D dans le budget | — |
| A298 | 1 | S305 | L'écart entre le pas de production et la référence croît avec le temps, à pente mesurée | — |
| A299 | 1 | S306 | L'environnement lumineux de notre rendu n'a jamais été comparé de façon contrôlée | — |
| A304 | 1 | S312 | Un seuil dont le dénominateur peut s'annuler | S313 : Remède mesuré, décision en attente |
| A310 | 1 | S318 | Le SPH du banc du lot 5 garde une erreur que je n'ai pas isolée | — |
| A283 | 2 | S250 | Le premier raccordement B/W réel au MAC refuse le démarrage à perturbation et hauteur imposée nulles, à seulement 16×8 et 32×16 | — |
| A284 | 2 | S251 | Le démarrage plat reçu coûte | — |
| A286 | 2 | S253 | Le pas couplé mobile (ADR-152) exige un fond prolongé au-dessus du plan moyen | — |
| A287 | 2 | S256 | La rugosité de B est incomplète, et ses directions sont liées à la fréquence | — |
| A288 | 2 | S260 | Sous `--vagues`, la surface rendue n'est pas celle que le jeu interroge | — |
| A290 | 2 | S283 | Rétrécir un domaine perturbatif n'est pas gratuit à un instant arbitraire | — |
| A296 | 2 | S293 | Le profil de 2 ms ne donne aucune part à δ ni de cible matérielle | — |
| A297 | 2 | S301 | La hauteur de la référence δ 3D est discontinue en la position de la surface par rapport aux centres de maille | — |
| A300 | 2 | S307 | Les constantes du nuanceur n'ont pas de provenance, et personne ne les avait auditées | — |
| A301 | 2 | S307 | Aucune mesure du dépôt ne regardait l'image | S308 : L'angle reste ouvert et son déclencheur inchangé |
| A303 bis | 2 | S312 | Personne n'avait demandé à W ce qu'il sait faire | — |
| A305 | 2 | S313 | Le résidu du cas ouvert est d'un seul signe | — |
| A309 | 2 | S316 | La direction que lit le raccord est biaisée, et le biais ne converge pas | — |
| A312 | 2 | S320 | La couronne et le jet d'un impact sont des grandeurs de la maille | S488 : la couronne de B10, avec la référence d'aujourd'hui : 0,205 / 0,204 / 0,305 / 0,386 D à 8, 12, 16, 24 mailles ; la rupture en gouttes n'y change rien — la … |
| A314 | 2 | S323 | La surface d'APIC dépend de l'arrangement de ses particules | — |
| A319 | 2 | S351 | « Rétrécir ou détruire un domaine perturbatif est visuellement gratuit » n'a jamais été mesuré | — |
| A323 | 2 | S409 | Au pas long, la cuve fermée gagne de l'énergie | — |
| A325 | 2 | S485 | Les parois d'APIC ne sont pas des plans de symétrie pour l'écoulement qui les longe | — |
| A320 | 3 | S369 | Sous une houle raide, une perturbation de δ croît | S443–S445 : la bascule des défauts (S443) fait du mode relatif la production ; A320 l'y accompagne, plafonnée (ADR-213 D2), sa limite écrite (MER-S369 §9–11) ; rien en … |

## Closes

| anomalie | sév. | ouverte | close | titre |
|---|---:|---|---|---|
| A285 | 2 | S252 | S252 | Le gradient conjugué préconditionné par la multigrille formait β = ‖r_{n+1}‖²/⟨r_n, z_n⟩ avant le cycle, depuis S245 P5 (6dc0bfa), alors … |
| A289 | 2 | S274 | S369 | Un domaine δ fidèle dérive en phase de B |
| A302 | 2 | S308 | S310 | Aucun bilan de conservation n'a jamais été mesuré à l'interface δ ↔ B/W |
| A303 | 1 | S309 | S309 | Le battement du jeton n'était relu par personne |
| A306 | 2 | S314 | S316 | Aucun estimateur de fréquence du dépôt n'a été vérifié |
| A307 | 2 | S315 | S316 | Aucun banc du dépôt ne déroule une phase |
| A308 | 2 | S316 | S316 | Les bancs du transfert tiraient de l'onde posée ce qu'une scène ne leur donnerait pas |
| A311 | 2 | S320 | S482 | L'air n'est pas modélisé : une bulle enfermée est à pression nulle |
| A313 | 1 | S320 | S323 | Le volume géométrique d'APIC n'a pas de mesure propre en écoulement agité |
| A315 | 2 | S324 | S326 | Les petites cellules d'un fond coupé 3D font ramper le gradient conjugué du mode linéaire |
| A316 | 2 | S325 | S407 | La frontière du raccord dynamique décale la surface et dissipe |
| A317 | 2 | S333 | S335 | Une coque qui perce le couvercle de δ rayonne selon la position de sa paroi dans la maille |
| A318 | 2 | S345 | S348 | À 30 Hz, l'onde de δ sur une vraie mer garde jusqu'à 6 % de plus d'amplitude qu'à 60 Hz |
| A321 | 3 | S390 | S390 | À 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit le solveur de pression |
| A322 | 2 | S409 | S480 | À 10 cm, 30 Hz n'est pas stable sur la minute |
| A324 | 3 | S435 | S436 | En mode relatif, la surface de B qui franchit un centre de maille amplifie δ |
| A326 | 2 | S485 | S487 | La carte ne lance pas plus de 65 535 groupes par passe |
| A327 | 2 | S490 | S492 | Un mur de décor aligné sur la grille fait échouer la projection linéaire de δ |
