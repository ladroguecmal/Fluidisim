# C22 — référence de la fenêtre fine, S56, 2026-09-07

> **Note corrective — 2026-09-07, S57.** Le couple 76800/153600 a été mesuré
> ([MESURES-C22-S57](MESURES-C22-S57.md)) : **les deux seuils extrapolés ci-dessous sont
> tous deux trop bas**. L'écart réel des oracles vaut 2,709078717e-10, contre 2,3145e-10
> prévu en n⁻² et 2,5912e-10 avec l'exposant empirique 1,72145 ; le seuil ×30 réel est
> **8,127236151e-9**, et la grille 12800 est **refusée** à 0,9556 du seuil. L'exposant local
> a encore baissé, à 1,61233. Le budget, lui, s'est vérifié à +2,05 %. Rien n'est réécrit
> ici : les chiffres de S56 restent ce qu'ils étaient au moment où ils ont été produits.

Suite de [S49](MESURES-C22-S49.md), action S49-1. Objectif : dimensionner la prochaine
référence par une mesure de contamination et un budget, avant toute conclusion de convergence.

## Protocole fixé avant calcul

Mode dédié `c22-shallow-fin 51200`, depuis `code/` après compilation release : quatre fenêtres
100–1600, 200–3200, 400–6400 et **800–12800**, sur huit champs calculés une fois et deux
oracles 51200/102400. Les deux anciens modes conservent leurs grilles, bornes et paramètres.
Le nouveau mode accepte un premier oracle multiple de 12800, de 25600 à 76800 inclus.
Cette borne est une limite de campagne, sans sens physique.

Montage, solveur, projection conservative, filtre ×30, plancher arithmétique et test de stabilité
inchangés. Les deux oracles sont chronométrés séparément. Aucun test lancé pendant la mesure.
Les champs restent en mémoire, jamais sur disque (I-17) ; seuls les résultats scalaires sont
consignés. Réutiliser les champs ne rend pas les fenêtres indépendantes.

## Stratégie de référence

L'écart 25600/51200 était 1,717298105e-9 en S48, celui de 51200/102400
5,207593646e-10 en S49. Leur quotient correspond à un exposant empirique **1,72145**,
pas exactement deux. Il n'est donc pas justifié de promettre que doubler la référence
divisera sa contamination par quatre. Deux extrapolations servent seulement au budget :
une en n⁻², une en n⁻¹·⁷²¹⁴⁵. Aucune ne remplace le filtre mesuré.

À domaine et temps physique fixés, le nombre de pas CFL croît avec n et chaque pas traite n
cellules : estimation du temps en n². Le coût S49 de 382,716 s donne les ordres de grandeur
ci-dessous (les petits champs, inclus dans cette base, ne sont pas une mesure isolée des oracles).

| Premier / second oracle | Estimation de campagne | Usage |
|---|---:|---|
| 51200 / 102400 | 383 s | référence rejouée en S56 avec une grille de plus |
| 64000 / 128000 | 598 s | emboîté, mais gain de contamination probablement insuffisant |
| 76800 / 153600 | 861 s, environ 14 min 21 s | prochain essai borné recommandé |
| 102400 / 204800 | 1531 s, environ 25 min 31 s | hors de cette campagne bornée |

Le choix 76800/153600 est un compromis de coût à éprouver, **sans garantie de retenir 12800**.
Pour ce couple, le seuil ×30 extrapolé vaut 6,94346e-9 en n⁻², ou 7,77366e-9 avec
l'exposant empirique. Leur proximité de l'erreur attendue sur 12800 motive la mesure du filtre
avant toute interprétation d'ordre ; ce ne sont pas des bornes de confiance.
Il conserve la projection exacte par blocs entiers pour les huit grilles. Une extrapolation
de Richardson du champ n'est pas introduite : elle supposerait l'ordre qu'on cherche à établir.
Un cache de champs sur disque est exclu par I-17. Aucun changement de CFL, amplitude, temps
final ou initialisation pour réduire artificiellement le coût de ce même cas.

## Critères du prochain essai

1. Mesurer l'écart réel 76800/153600, les huit erreurs et les deux temps d'oracle.
2. Retenir uniquement le préfixe satisfaisant erreur ≥ 30 × écart réel ; conserver les refus.
3. Exiger cinq grilles et trois ordres exploitables dans la fenêtre 800–12800, puis appliquer
   le test de stabilité existant avant p > 0,8. Un dernier ordre proche de deux ne suffit pas.
4. Comparer aussi les sept anciennes grilles : raffiner l'oracle peut déplacer leurs ordres.
5. Si 12800 reste contaminée, annoncer « sans verdict » et réviser le budget ; ne pas lancer
   automatiquement un doublement supplémentaire. Prévoir une étape distincte pour la mesure
   puis la rédaction. Si le coût dépasse quinze minutes, déclarer un découpage de calcul en
   tranches temporelles gardées en mémoire avant une campagne plus grande.

Les estimations temporelles dépendent de la machine et de sa charge ; elles ne constituent
ni un délai garanti ni un budget temps réel du jeu. L'écart de deux oracles du même schéma
reste un indicateur empirique, sans borne prouvée de leur erreur commune.

## Résultat mesuré

Campagne release locale : **371,116 s**, dont oracle 51200 **72,727 s**, oracle 102400
**292,354 s** ; reste 6,035 s. Rapport des deux temps : environ 4,02, compatible avec
le coût quadratique sur ces deux tailles. Aucune suite de tests en concurrence.

Écart des oracles identique à S49 : **5,207593646e-10**, seuil ×30 **1,562278094e-8**.
Les sept premières grilles reproduisent les deux erreurs et les ordres de S49 aux chiffres
imprimés. La grille **12800** donne **7,410359445e-9** contre 51200 et
**7,710700097e-9** contre 102400 : variation **3,003407e-10**, environ **3,90 %** de
la seconde erreur. Son erreur ne vaut que **0,494 fois** le seuil requis.

La fenêtre 800–12800 conserve donc **4/5 grilles**, deux ordres (1,960632292 et
2,011665373), et reste **sans verdict : stabilité non établie**. Aucun ordre du triplet
3200–12800 publié après rejet. Les trois anciennes fenêtres restent non concluantes.
Bilan : zéro succès, zéro échec physique, **quatre familles sans verdict** ; sortie 0.

En recalant seulement les deux oracles mesurés et en conservant 6,035 s pour le reste,
le budget 76800/153600 devient **827,467 s**, environ **13 min 47 s** ;
102400/204800 demanderait environ **1466,359 s**, soit **24 min 26 s**. Ces estimations
restent dépendantes de la charge ; la borne 76800 limite la prochaine campagne autour du
quart d’heure sans garantir son délai. La mémoire de calcul croît linéairement, le temps
quadratiquement ; la réutilisation en mémoire évite de payer les oracles par fenêtre.

Les deux seuils extrapolés pour 76800 encadrent presque la nouvelle erreur :
6,94346e-9 < 7,71070e-9 < 7,77366e-9. **Choisir le modèle changerait la prévision
admissible/refusée.** Le prochain essai sert donc à mesurer cette admission, sans annoncer
une validation ni commander automatiquement une campagne plus grande. L’erreur contre le
nouvel oracle changera elle aussi : cette comparaison sert seulement au dimensionnement.

S49-1 close pour la stratégie et le budget ; exécution suivante suivie en **S56-1**.
Pas de résultat sur la convergence du modèle 3D, ni sur le banc B3.

## Vérification

123 tests réussis (38 cœur, 85 harnais), deux ignorés ; trois tests C22 ciblés réussis
avant campagne. Le test de refus couvre aussi les tailles du nouveau mode : zéro, oracle
égal à la grille fine, non emboîté, au-dessus de la borne et usize::MAX. Compilation release
réussie ; campagne numérique ci-dessus exécutée. Les deux scénarios check restent verts,
hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e. La campagne physics générale, inchangée
par ce mode séparé, n’est pas répétée.
