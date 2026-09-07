# C22 shallow — déplacement des fenêtres, S49, 2026-09-07

Suite de [S48](MESURES-C22-S48.md), action S48-1. Montage physique, schéma,
initialisation aux centres et seuils inchangés. Aucune modification du solveur.

## Protocole

Trois fenêtres déclarées avant les calculs : 100–1600, 200–3200, 400–6400 cellules,
cinq grilles par doublement chacune. Les sept champs sont calculés une fois ; deux oracles
restent en mémoire et servent à toutes les comparaisons. Aucun champ enregistré sur disque
(I-17). Les fenêtres partagent leurs grilles : elles ne sont pas trois expériences indépendantes.

Commande depuis code/ : `cargo run --offline --release -- c22-shallow-fenetres 51200`.
Le mode historique `c22-shallow` conserve sa fenêtre et ses paramètres. Le nouveau mode
accepte un premier oracle multiple de 6400, strictement supérieur à 6400, au plus 51200 ;
sans argument il utilise 12800. Le second oracle double le premier.

Chaque fenêtre retient son préfixe dont l'erreur dépasse 30 fois l'écart entre oracles.
Une grille refusée coupe la fenêtre ; aucun triplet reconstruit au-delà du trou.
Le filtre reste empirique : l'écart de deux oracles du même schéma ne borne pas leur erreur
commune. Plancher n_fin·epsilon_f64. Critère de stabilité inchangé, tolérance 0,10 et
réduction des variations successives ; trois ordres exploitables requis. Le seuil p>0,8
n'est appliqué qu'après stabilité, sur le triplet grossier selon le contrat Oracle.

## Mesures

Oracles 51200/102400 : écart L1 = 5,207593646e-10, seuil empirique ×30 = 1,562278094e-8.
Les sept grilles passent ce filtre ; le plancher arithmétique est 2,273736754e-11.

| Cellules | Erreur contre 51200 | Erreur contre 102400 |
|---|---:|---:|
| 100 | 7,514852894e-5 | 7,514862127e-5 |
| 200 | 2,380667708e-5 | 2,380687750e-5 |
| 400 | 7,306411789e-6 | 7,306682339e-6 |
| 800 | 1,981772242e-6 | 1,982071672e-6 |
| 1600 | 5,045954241e-7 | 5,049004674e-7 |
| 3200 | 1,250945568e-7 | 1,253917967e-7 |
| 6400 | 3,097872190e-8 | 3,127869550e-8 |

| Fenêtre | Grilles retenues | Ordres des trois triplets | Verdict |
|---|---:|---|---|
| 100–1600 | 5/5 | 1,637649 ; 1,631735 ; 1,849839 | non concluant |
| 200–3200 | 5/5 | 1,631735 ; 1,849839 ; 1,960632 | non concluant |
| 400–6400 | 5/5 | 1,849839 ; 1,960632 ; 2,011665 | non concluant |

**Pourquoi le dernier verdict reste ouvert.** Les deux dernières variations sont
d0=0,110793296 et d1=0,051033081. La variation finale passe la tolérance 0,10,
mais ne change pas de signe et ne décroît pas d'un facteur quatre :
4·d1=0,204132325 > d0. Le critère existant refuse donc la stabilité.
La proximité de 2 du dernier ordre ne suffit pas ; aucun seuil n'a été adapté au résultat.

La référence fine change l'erreur de la grille 6400 d'environ 0,96 % ; le filtre ×30
ne garde qu'une marge d'environ deux à cette résolution. Les cinq premières erreurs contre
51200 reproduisent S48 aux chiffres imprimés. Les observations sont compatibles avec
une approche de l'ordre deux, sans établir le régime asymptotique selon le contrat courant.

## Coût, validation et suite

Coût release local : **382,716 s**, deux oracles et sept champs, trois fenêtres.
Aucune suite de tests lancée en parallèle de cette mesure. Ce coût dépend de la machine
et de sa charge extérieure ; il ne constitue pas une garantie temps réel.
La réutilisation évite de répéter les deux grands calculs pour les trois fenêtres.

Le bilan donne zéro succès, zéro échec physique, trois familles sans verdict. La commande
sort avec zéro parce qu'aucun échec n'est établi, pas parce que C22 est validé.
Les fenêtres chevauchantes ne constituent pas trois validations indépendantes.

S48-1 est exécutée. Une fenêtre encore plus fine nécessitera de contrôler à nouveau
l'influence de l'oracle : garder automatiquement la référence actuelle serait injustifié.
Cette extension coûteuse reste ouverte (S49-1), sans bloquer les corrections de refus
déjà identifiées dans le harnais. Aucun solveur 3D ni banc B3 n'est validé ici.

Vérifications : **105 tests réussis** (38 cœur + 67 harnais), deux ignorés.
Le nouveau test vérifie une grille rejetée suivie de valeurs admissibles : la fenêtre
reste coupée. Refus de 9600 et 6400 pour le mode fenêtres ; taille 9600 testée aussi
par la commande (sortie 1). Mode historique rejoué à 3200/6400 : trois grilles retenues,
stabilité indisponible, aucun faux succès. Compilation release réussie, diff sans erreur.
Les tests de mesures historiques passent ; aucun changement des solveurs ou scénarios.

**Suivi S56 — 2026-09-07.** S49-1 close pour sa stratégie et son budget ; la fenêtre
800–12800 a été mesurée avec les mêmes oracles : grille finale rejetée, 4/5 seulement.
Les sept anciennes grilles reproduisent ce rapport. Couple suivant 76800/153600 suivi en
S56-1 ; [REFERENCE-C22-S56](REFERENCE-C22-S56.md) donne protocole et coût estimé.
