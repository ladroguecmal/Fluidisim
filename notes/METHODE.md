# Méthode de travail

Révision S321, 2026-09-22, demandée par l'utilisateur lors de l'analyse complète
([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md), mesures dans
[BILAN-GLOBAL-S321](../docs/registres/BILAN-GLOBAL-S321.md)). Version précédente dans Git à
`9b201099`. Les précautions qui ont trouvé des erreurs restent ; ce qui change, c'est **quand**
elles se chargent, et qu'un outil tient celles qu'on oubliait.

## Protections actives — à charger au moment qu'elles nomment

Dix-sept, tirées des erreurs qui se sont **répétées** ; chacune renvoie à sa leçon, et au contrôle
qui la tient quand il existe. [LECONS](LECONS.md) est leur archive : on y cherche, on ne la relit
pas. Une leçon nouvelle ne s'écrit que si elle crée ou change une ligne de cette table.

| moment | protection | leçons | contrôle |
|---|---|---|---|
| **avant de conclure** | Une grandeur conservée — masse, hash, invariant — ne prouve ni précision, ni volume, ni résolution : mesurer la grandeur d'usage elle-même | L277, L366, L370 | — |
| | Attribuer un effet demande un témoin qui en est privé ; éteindre un à un les termes absents du témoin | L136, L354 | — |
| | Une convergence se lit sur trois points au moins, et juge un ordre, pas un déplacement | L274, L361 | — |
| | Un comportement s'éprouve sur sa durée d'usage : six secondes ne disent rien d'une minute | L369 | — |
| | L'incertitude vraie est la sensibilité à une perturbation minime, pas l'accord de deux calculs | L371 | — |
| | Une erreur systématique ne s'annule que si les deux côtés la portent également | L278 | — |
| | Critère et prédiction s'écrivent avant la mesure ; un seuil ne se relève jamais pour faire passer ; un fait mesuré et son explication se publient séparément | L177 | — |
| **en construisant l'instrument** | Un instrument s'éprouve sur un cas de réponse connue, et se réépreuve quand ce qu'il mesure s'améliore ; deux représentations se comparent à frontière et point de fonctionnement égaux | L360, L368 | — |
| | Le compilateur est dans la boucle : une identité flottante du source n'est pas celle du binaire, carte graphique comprise — la vérifier sur la cible | L345, L346 | — |
| **en lançant** | Un binaire ne s'exécute qu'après une compilation **réussie** et lue : `cargo run`, ou code de sortie vérifié, jamais une sortie filtrée | L362 | zéro avertissement de construction : un avertissement neuf se voit |
| | Une revue visuelle part avec ses options explicites (`--meilleur`) ; deux rendus se comparent à horizon forcé | L349, A301 | — |
| | Un calcul long en arrière-plan : vérifier ensemble processus, journal et cible ; recompter les processus avant toute relance — l'absence de journal ne prouve pas l'absence de calcul | *simufluid* | — |
| **en écrivant** | Une règle ou un fait vit à un seul endroit ; ailleurs, un renvoi | L137 | — |
| | L'horloge se lit dans un appel séparé, à chaque commit d'étape | L237 | battement |
| | Jamais `Get-Content` ni `Set-Content` de Windows PowerShell sur un fichier du dépôt — il lit en ANSI ; l'outil d'édition, ou Python en UTF-8 | S301 | encodage |
| | Une liste qu'il faut penser à tenir se confie à un outil : décomptes, plafonds, fichiers produits | L349 | décompte, plafonds, fichiers produits |
| **en choisissant la suite** | Un blocage hérité se vérifie dans le code avant d'être contourné ou tranché ; un ordre nomme la dépendance qu'il protège | L176, L243, L343 | — |

Les contrôles sont ceux de `python outils/etat_projet.py --check`, sauf les avertissements, qui se
lisent à la construction.

## Choisir un lot utile

1. Partir de la demande actuelle, puis de la **porte en cours** que désigne
   [FEUILLE-DE-ROUTE §3 bis](../docs/FEUILLE-DE-ROUTE.md), puis de la file active ; vérifier
   le blocage dans le **code présent** et son contrat. La suite déclarée par la session précédente
   est une proposition, pas un ordre.
2. Nommer la capacité visée et le **consommateur** : joueur, hôte, serveur V ou prochaine brique.
   Une réception n'est reçue qu'après avoir traversé le chemin qui la consomme (L258, L280).
3. Écrire le critère de réception et d'arrêt avant le code ou la campagne. Si la réponse ne
   change aucune décision ou intégration, différer cette recherche avec son déclencheur.
4. Déclarer le plan dans EN-COURS selon REPRISE. Le plan doit être assez court pour montrer ce
   qui sera fini ; pas un traité préalable à une correction.

**Précision rapportée à l'usage** *(consigne de l'utilisateur, 2026-09-18)*. Avant de poursuivre
un raffinement numérique, traduire la précision visée en grandeurs d'usage — hauteur, pente, phase
— contre les tolérances d'image (3 mm, S201) et d'horloge (ADR-003), au régime le plus exigeant
servi, et dire ce que chaque critère protège : fonctionnement, fidélité physique ou qualité
visuelle. Les garanties de fonctionnement ne se négocient pas. Un seuil de précision se justifie
par l'usage ; s'il paraît inutilement strict, le documenter et proposer ce qu'un autre critère
garantirait, sans le relever pour faire passer un test. Au niveau nécessaire, passer au blocage
suivant et garder le perfectionnement dans la file avec son déclencheur. Ne pas conclure à
l'invisibilité sur des chiffres seuls : un rendu se juge par l'utilisateur (REVUE-VISUELLE).
Continuer les étapes autorisées sans attendre de relance, et solliciter l'utilisateur pour un
jugement visuel ou une vraie décision.

Avant de poursuivre un même sujet une troisième session, comparer sa suite à au moins une
capacité de la file encore absente — **à commencer par la voie de la v1** (porte D). Une grande
amélioration locale peut ne plus être prioritaire une fois son usage débloqué.

## Construire et éprouver

| nature du travail | vérification proportionnée |
|---|---|
| Correction de code | cas reproduisant le défaut, témoin nominal, assertions du contrat affecté |
| Propriété numérique ou modèle nouveau | référence indépendante ou identité justifiée ; raffinement des paramètres qui peuvent biaiser le verdict |
| Optimisation | même charge utile, même qualité, coût du chemin consommé ; techniques présentes/absentes et domaine (ADR-131) |
| Intégration | scénario traversant les composants réels, avec refus et reprise si le contrat l'exige |
| Rendu jugé à l'œil | revue de l'utilisateur contre références réelles ([REVUE-VISUELLE](../docs/validation/REVUE-VISUELLE.md)) ; un verdict déclenche une mesure, il n'en tient pas lieu |
| Documentation/procédure | liens, cohérence des états actifs, possibilité réelle d'exécuter la consigne ; `etat_projet.py --check` |

Chercher d'abord si la réponse se **calcule** avant de lancer une campagne. Distinguer le modèle,
son implémentation, l'instrument et la métrique. Un échec utile reste visible. Le seuil physique
vient d'une formule ou d'un banc identifié, jamais d'un chiffre choisi après coup.

**Bancs et système** (ADR-187 D6). Un banc de `examples/` est un **instrument** : compilé par
`cargo test`, exécuté par aucune suite. Ce qu'il reçoit n'est donc protégé que par sa preuve. Toute
preuve ouverte depuis S321 commence par une section **« Reproduire »** — commit, commandes exactes,
valeurs attendues, durée — et l'outil le vérifie. Une capacité qui passe d'un banc au système, cœur
ou afficheur, **y entre avec ses essais** : les chiffres de sa réception deviennent des assertions,
ignorées par défaut si elles sont lentes. APIC y entrera au raccord particules ↔ colonnes.

Une contre-épreuve se justifie par le défaut qu'elle distingue ; pas de variante automatique à
chaque étape. Rejouer les tests affectés, puis la suite requise avant livraison du code ; ne pas
rejouer toute la suite après une retouche documentaire. Les bancs qui portent la propriété étudiée
se lancent explicitement.

Un changement de hash n'interdit pas un correctif : conserver l'ancien reçu, expliquer les champs
qui bougent et recevoir la nouvelle référence. Un changement de protocole réseau ou de décision
architecturale reste une migration explicitement documentée, pas une mise à jour silencieuse.

## Ce qu'il faut lire

La lecture à froid est bornée par REPRISE §3 (ADR-187 D2) ; cette méthode en fait partie, à
commencer par ses protections. Pour le lot : contrats, ADR cités et leurs notes datées,
consommateurs et **preuves existantes, lues avant de modifier**. Les registres — journal, leçons,
angles morts, traçabilité des questions — se consultent par recherche.

Distinguer exigence, proposition et question. Les sources sont des intentions et des propositions,
pas un ensemble de contraintes déjà validées. Les décisions ultérieures de l'utilisateur et les
ADR les arbitrent. Les rôles des couches et les invariants ne se réduisent pas en cours de lot.

## Ce qui s'écrit, et où

| information | porteur unique |
|---|---|
| règle opérationnelle de reprise/clôture | REPRISE ; amorce/fermeture des copies dans AGENTS |
| objectif, étapes et notes de la session **en cours** | EN-COURS, purgé à la clôture |
| décision d'architecture ou de méthode | nouvel ADR ; note datée pour correction factuelle |
| preuve utile à une décision | document de validation, ouvert par « Reproduire » ; **un fil, une preuve** (D7) : un fil de plusieurs sessions enrichit une preuve par sections datées |
| capacité présente et limite | FEUILLE-DE-ROUTE, état remplacé et daté |
| ce que le projet fini doit avoir, coché | LISTE-PROJET-FINI ; pointe, ne recopie pas ; son décompte est vérifié par l'outil |
| travail à faire, motif et déclencheur | file active de QUESTIONS-OUVERTES |
| protection contre une erreur qui revient | table ci-dessus ; la leçon qui la fonde, dans LECONS |
| histoire et résultat de session | JOURNAL, une entrée de vingt lignes au plus |
| navigation | index, carte par système en tête ; liens plutôt que récits copiés |

**Aucune obligation de nouvelle anomalie, leçon, ADR ou document de mesure.** Une vérification
sans anomalie est un résultat recevable. Clore un lot à son critère d'arrêt ; les questions
supplémentaires passent par la file.

Les inconnues externes restent inconnues. Les « autres équipes » ne sont pas des interlocuteurs
présents (ADR-028). Les règles d'autorisation acquises sont dans REPRISE §5 ; ne pas les recréer
à partir d'une réserve historique.
