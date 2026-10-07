# Méthode de travail

Révision S321, 2026-09-22, demandée par l'utilisateur lors de l'analyse complète
([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md), mesures dans
[BILAN-GLOBAL-S321](../docs/registres/BILAN-GLOBAL-S321.md)). Version précédente dans Git à
`9b201099`. Les précautions qui ont trouvé des erreurs restent ; ce qui change, c'est **quand**
elles se chargent, et qu'un outil tient celles qu'on oubliait.

## Protections actives — à charger au moment qu'elles nomment

Trente-quatre, tirées des erreurs qui se sont **répétées** (trois ajoutées par la revue de S486, [ADR-223](../docs/adr/ADR-223-premiere-revue-de-methode.md), trois par celle de S491, [ADR-224](../docs/adr/ADR-224-deuxieme-revue-de-methode.md), trois par celle de S496, [ADR-226](../docs/adr/ADR-226-troisieme-revue-de-methode.md), une par celle de S501, [ADR-228](../docs/adr/ADR-228-quatrieme-revue-de-methode.md), une par celle de S506, [ADR-230](../docs/adr/ADR-230-cinquieme-revue-de-methode.md), une par celle de S511, [ADR-231](../docs/adr/ADR-231-sixieme-revue-de-methode.md), une et une élargie par celle de S516, [ADR-232](../docs/adr/ADR-232-septieme-revue-de-methode.md), une élargie par celle de S521, [ADR-233](../docs/adr/ADR-233-huitieme-revue-de-methode.md), une et une élargie par celle de S526, [ADR-234](../docs/adr/ADR-234-neuvieme-revue-de-methode.md), une par celle de S531, [ADR-235](../docs/adr/ADR-235-dixieme-revue-de-methode.md), une changée par celle de S536, [ADR-236](../docs/adr/ADR-236-onzieme-revue-de-methode.md), une élargie par celle de S541, [ADR-237](../docs/adr/ADR-237-douzieme-revue-de-methode.md), une élargie et un outil par celle de S546, [ADR-238](../docs/adr/ADR-238-treizieme-revue-de-methode.md), une élargie par celle de S551, [ADR-239](../docs/adr/ADR-239-quatorzieme-revue-de-methode.md), deux élargies par celle de S556, [ADR-240](../docs/adr/ADR-240-quinzieme-revue-de-methode.md), deux élargies et un outil par celle de S561, [ADR-242](../docs/adr/ADR-242-seizieme-revue-de-methode.md), une élargie par celle de S566, [ADR-243](../docs/adr/ADR-243-dix-septieme-revue-de-methode.md), une élargie par celle de S571, [ADR-244](../docs/adr/ADR-244-dix-huitieme-revue-de-methode.md), une, une élargie et une corrigée par celle de S576, [ADR-245](../docs/adr/ADR-245-dix-neuvieme-revue-de-methode.md), deux élargies par celle de S586, [ADR-248](../docs/adr/ADR-248-vingt-et-unieme-revue-de-methode.md), une élargie et une rendue exécutoire par celle de S591, [ADR-249](../docs/adr/ADR-249-vingt-deuxieme-revue-de-methode.md), une élargie par celle de S596, [ADR-250](../docs/adr/ADR-250-vingt-troisieme-revue-de-methode.md), une élargie par celle de S601, [ADR-251](../docs/adr/ADR-251-vingt-quatrieme-revue-de-methode.md)) ; chacune renvoie à sa leçon, et au contrôle
qui la tient quand il existe. [LECONS](LECONS.md) est leur archive : on y cherche, on ne la relit
pas. Une leçon nouvelle ne s'écrit que si elle crée ou change une ligne de cette table.

| moment | protection | leçons | contrôle |
|---|---|---|---|
| **avant de conclure** | Une grandeur conservée — masse, hash, invariant — ne prouve ni précision, ni volume, ni résolution : mesurer la grandeur d'usage elle-même | L277, L366, L370 | — |
| | Attribuer un effet demande un témoin qui en est privé ; éteindre un à un les termes absents du témoin | L136, L354 | — |
| | Une convergence se lit sur trois points au moins, et juge un ordre, pas un déplacement | L274, L361 | — |
| | Un comportement s'éprouve sur sa durée d'usage : six secondes ne disent rien d'une minute ; **un essai qui attend un équilibre en calcule la constante de temps au plan, ou lit sa trace avant de conclure** (ADR-240 D2) | L369, L398 | — |
| | L'incertitude vraie est la sensibilité à une perturbation minime, pas l'accord de deux calculs | L371 | — |
| | Au-delà de leur temps de divergence (mesuré), deux calculs d'un écoulement qui se déstabilise se comparent en statistiques, pas en trajectoire ; **une propriété d'ensemble se juge sur un ensemble dont la taille est calculée au plan, jamais sur un seul tirage** (ADR-250 D1) | L376, L409 | — |
| | Un montage simplifié (symétrie, quart, paroi prise pour un plan de symétrie) s'éprouve une fois contre le montage entier avant de servir ; un modèle neuf n'est tenu qu'après une scène de jeu | L374 | — |
| | Une erreur systématique ne s'annule que si les deux côtés la portent également | L278 | — |
| | Critère et prédiction s'écrivent avant la mesure ; un seuil ne se relève jamais pour faire passer ; un fait mesuré et son explication se publient séparément | L177, L395 | le rituel refuse sans commit « Snnn P1 » (ADR-238 D1) |
| | Un écart à une référence se localise avant tout remède : séparer les chaînes (le modèle contre sa propre équation, la référence contre la sienne, l'état de départ) par le montage le moins cher qui les isole | L381 | — |
| | Deux trajectoires se comparent depuis le même état, positions **et** vitesses : un corps lâché dans une eau qui bouge part à sa vitesse, toutes ses parts composées | L382 | — |
| **en construisant l'instrument** | Un instrument s'éprouve sur un cas de réponse connue **de la même famille que l'objet** (même forme de source, même régime), en vérifiant sur ce qu'il lit qu'il prend le trait voulu — sa stabilité ne le prouve pas (ADR-233 D1) ; il se réépreuve quand ce qu'il mesure s'améliore ; deux représentations se comparent à frontière et point de fonctionnement égaux | L360, L368, L389 | — |
| | La référence d'un instrument porte le bruit de l'objet mesuré : un instrument qui lit un extremum ou un seuil s'éprouve sur une référence bruitée au plancher de l'objet (f32, échantillonnage), ou ignore par un seuil déclaré avant ce qui est sous ce plancher (ADR-234 D2) ; **le plan écrit, à côté de chaque seuil, le quantum ou le plancher de l'objet et leur rapport — sous 10, le seuil est disqualifié** (ADR-236 D1) ; **le script du plan refuse (`assert`) un rapport sous 10 avant d'écrire** (ADR-249 D1) | L391, L393, L408 | le script du plan |
| | Une grandeur de diagnostic (centre, volume, débit) s'éprouve à sa naissance par un essai du cœur sur un cas de réponse connue ; **une grandeur intégrale d'un schéma (énergie, masse, norme) se mesure avec les poids de son produit scalaire (ouvertures, demi-mailles), lus dans le code, et s'éprouve d'abord sur un invariant connu** (ADR-242 D1) | L375, L400 | essai `air_pocket_centroid_is_the_bubble_centre_s486` |
| | Une référence numérique (pas fin, maille fine) s'éprouve convergée sur trois points avant qu'on juge contre elle ; un départ impulsif ou un régime singulier du modèle n'a pas de référence | L385 | — |
| | Un corps d'essai se choisit loin de ses limites : sa marge (stabilité de forme, `ω·dt`, rampe d'immersion) se calcule avec le proxy qu'il porte avant de mesurer ; une marge de l'ordre de l'erreur du proxy le disqualifie ; **et il n'a que les degrés de liberté que la référence décrit** (nommer ses hypothèses, puis un corps qui les tient, ou borner l'essai) ; **une paire de trajectoires comparée appartient à la famille que la référence décrit — le même invariant, le même front** (ADR-248 D1) ; **tout le montage** se place dans le domaine de validité de ses outils, calculé avant (domaine honnête d'un champ sur la distance qui compte, résolution, régime ; ADR-234 D1) | L384, L387, L390 | — |
| | Une valeur attendue se calcule dans l'essai (la formule, puis la tolérance), jamais en dur depuis un calcul de tête ; **une formule d'analyse nouvelle s'éprouve par un calcul indépendant avant la mesure, et se relit la première devant un écart** (ADR-239 D1) ; **un essai n'affirme que ce que le plan a écrit — une vérification trouvée en route passe d'abord aux notes, avec sa valeur calculée** (ADR-244 D1) ; **avant d'écrire l'essai, le plan se relit contre ses critères : chaque nombre exigé calculé, chaque assertion un critère** (ADR-251 D1) ; **un critère qui découle de la formule implémentée ne juge rien : remplacé par une référence indépendante, ou marqué « assemblage »** (ADR-248 D2) | L378, L397, L403, L406, L407, L410 | — |
| | Un garde-fou nomme ce qu'il autorise (une liste explicite), il ne le déduit pas d'une propriété voisine | L379 | — |
| | Le compilateur est dans la boucle : une identité flottante du source n'est pas celle du binaire, carte graphique comprise — la vérifier sur la cible | L345, L346 | — |
| | Un état quantifié (millilitres, quanta, valeur publiée en f32) ne sert jamais de départ au pas suivant : l'état exact (un cumul, un reste) se garde à part, le quantifié se dérive de lui (ADR-245 D1) | L404 | — |
| **en lançant** | Un binaire ne s'exécute qu'après une compilation **réussie** et lue : `cargo run`, ou code de sortie vérifié, jamais une sortie filtrée | L362 | zéro avertissement de construction : un avertissement neuf se voit |
| | Une revue visuelle part avec ses options explicites (`--meilleur`) ; deux rendus se comparent à horizon forcé | L349, A301 | — |
| | Un calcul long en arrière-plan **écrit sa progression**, et au double de sa durée annoncée **se diagnostique au lieu de se reporter** ; vérifier ensemble processus, journal et cible ; recompter les processus avant toute relance — l'absence de journal ne prouve pas l'absence de calcul | *simufluid*, L372 | — |
| **en écrivant** | Une règle ou un fait vit à un seul endroit ; ailleurs, un renvoi ; **un seuil calculé aussi : une seule fonction, lue par tous ses usages** (ADR-245 D2) | L137, L405 | — |
| | L'horloge se lit dans un appel séparé, à chaque commit d'étape ; le battement, lu par le script qui écrit le jeton, jamais tapé (ADR-228 D2) | L237 | battement |
| | Jamais `Get-Content` ni `Set-Content` de Windows PowerShell sur un fichier du dépôt — il lit en ANSI ; l'outil d'édition, ou Python en UTF-8 | S301 | encodage |
| | Une note à un ADR s'**ajoute** ; écrire le fichier entier avec la seule note l'efface (ADR-005, de S35 à S401) | L373 | un ADR commence par son titre |
| | Une liste qu'il faut penser à tenir se confie à un outil : décomptes, plafonds, fichiers produits | L349 | décompte, plafonds, fichiers produits |
| | Une édition par script vérifie toutes ses ancres avant d'écrire, et s'ancre sur une ligne courte ; **le script est un fichier — écrit par l'outil d'écriture ou par un heredoc entre apostrophes droites (`<<'EOF'`) —, jamais une chaîne entre apostrophes en ligne de commande** : l'apostrophe du français la ferme (ADR-240 D1, corrigé par ADR-245 D3) | L380, L399 | — |
| | Un enchaînement de commandes s'arrête au premier échec (`&&`, jamais une ligne nouvelle) ; un correctif de script qui échoue ne laisse pas partir le script corrigé — corriger le fichier visé à la main ; **le commit de clôture se garde par le code de sortie du rituel, jamais `;` après lui** (ADR-242 D2) | L386, L401 | le rituel sort en erreur sur une anomalie ou un lot dû (code 3) |
| **en choisissant la suite** | Un blocage — hérité ou supposé — se vérifie dans le code ou par un calcul avant d'être contourné, tranché **ou écrit** (ADR-238 D2) ; un ordre nomme la dépendance qu'il protège | L176, L243, L343, L396 | — |
| | Avant de déclarer un remède physique, son ordre de grandeur à l'échelle de la scène : un remède qui n'y pèse pas n'est pas déclaré | L377 | — |
| | Un ordre de grandeur s'écrit après l'avoir calculé (une ligne de script), jamais de tête ; **et se recalcule à chaque changement d'un de ses paramètres — jamais ajusté à vue** (ADR-237 D1) ; **les paramètres d'un montage s'écrivent dans les unités et le type de la loi qui les reçoit** (ADR-249 D2) ; **les nombres d'un plan sont écrits par le script qui les calcule, jamais recopiés ni tapés** (ADR-243 D1) | L388, L394, L402 | — |
| | Avant d'agrandir un domaine d'un ordre de grandeur, ses limites matérielles se calculent (groupes de dispatch, liaisons, tampons, mémoire) ; une limite atteinte se refuse avec un nom, jamais par un arrêt (ADR-235 D1) | L392 | — |
| | Un ordre de grandeur se compare au terme concurrent, sur la durée de la mesure : un effet qui s'accumule se compte à la fin ; « négligeable » se dit contre ce qu'il concurrence, pas contre ce qui le produit | L383 | — |

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

**Accélérer** *(décision de l'utilisateur, 2026-10-02, [ADR-213](../docs/adr/ADR-213-accelerer-tolerance-plafond-rituel-bancs.md))*.
Un critère manqué de moins de 5 %, ou sur un champ négligeable, est accepté sans consulter, et la preuve le dit (D1) — jamais une
garantie de fonctionnement. Un problème sans cause après deux sessions s'arrête sur sa limite mesurée, et rien en aval ne l'attend
(D2). Les registres se mettent à jour par lots de trois sessions (D3). Le banc le plus court qui tranche ; plus de trente minutes de
calcul se justifient dans le plan (D4).

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
valeurs attendues, durée — et l'outil le vérifie. « Ou plus récent » suppose qu'aucun comportement
par défaut ne change sur ce chemin : **une session qui en change un corrige les « Reproduire » qui le
citent** (S326). Une capacité qui passe d'un banc au système, cœur
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
