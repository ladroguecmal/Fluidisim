# Leçons

Enseignements généralisables, à réutiliser hors du contexte où ils sont apparus. Une leçon qui ne
sert que dans son cas d'origine n'a pas sa place ici.

---

## L01 — Une longue liste de questions ouvertes signale une décomposition manquante

*(S01)* Trente questions ouvertes, dont plusieurs qualifiées de « critiques », ont été réduites
par une seule décision structurelle. Quand des questions résistent individuellement, elles
partagent souvent une prémisse fausse.

**Réflexe** : avant de répondre à la troisième question d'une liste, chercher ce qu'elles ont en
commun.

## L02 — Un ordre de grandeur tue une direction plus vite qu'une analyse

*(S01)* `λ = 2πv²/g` — une ligne — a éliminé une approche que les documents tenaient pour acquise
et qui aurait coûté des mois.

**Réflexe** : chiffrer avant d'argumenter. Chercher spécifiquement la formule qui donne un
**minorant de coût** ou un **majorant de qualité**.

## L03 — Distinguer « question ouverte » et « question mal posée »

*(S01)* « Quelle profondeur maximale simuler ? », « quelle erreur de précalcul est acceptable ? »,
« quelle méthode pour la simulation hors caméra ? » n'avaient pas de réponse parce qu'elles
présupposaient un mécanisme qui n'avait pas lieu d'exister.

**Réflexe** : quand une question résiste, interroger sa prémisse avant d'inventer une réponse.
Une question dissoute vaut mieux qu'une question résolue.

## L04 — Les angles morts coûteux sont hors du domaine

*(S01, confirmé en S02)* Sur 40 angles morts recensés, les plus graves relevaient du déterminisme flottant, de la
latence GPU, de la précision aux grandes coordonnées, des référentiels non inertiels et de
dépendances vers d'autres équipes — pas de la physique des fluides.

**Réflexe** : passer systématiquement la liste de la Phase 4 de `METHODE.md`, quel que soit le
domaine.

## L05 — Ne jamais interpoler deux réalisations d'un processus stochastique

*(S01)* Mélanger deux champs de houle indépendants de même énergie fait chuter la hauteur
significative de 29 %. On interpole les **paramètres** qui engendrent le champ, jamais le champ.

Vaut pour le bruit, le terrain procédural, la végétation, la météo, l'audio granulaire — partout
où deux régions voisines sont générées indépendamment.

## L06 — Une bonne décision d'architecture rend gratuites des opérations qui étaient coûteuses

*(S01)* La décomposition en couches a rendu gratuits : la création de domaine, sa destruction, sa
croissance, son rétrécissement, le changement de solveur, la resynchronisation multijoueur,
l'arrivée en cours de partie.

**Réflexe** : évaluer une architecture candidate à la liste des opérations qu'elle rend triviales,
pas seulement à son coût nominal. Si elle n'en rend aucune triviale, ce n'est probablement pas la
bonne.

## L07 — Le petit objet est numériquement plus dur que le grand

*(S01)* La raideur de flottabilité rapportée à la masse croît quand l'objet rétrécit : une balle de
ping-pong diverge là où un porte-conteneurs est stable. Vrai pour tout couplage ressort/masse
issu de la géométrie.

**Réflexe** : dans tout système de forces dérivées de la géométrie, tester d'abord le cas le plus
petit et le plus léger.

## L08 — Concevoir le banc qui peut infirmer sa propre thèse

*(S01)* B4 est explicitement construit pour pouvoir invalider ADR-001. Sans lui, l'architecture
serait défendue au lieu d'être testée, et l'erreur éventuelle se découvrirait en production.

## L09 — Erreur de S01 : l'outil avant le contenu

J'ai tenté d'écrire de longs textes français via des documents en ligne de commande, et perdu un
cycle sur une erreur de citation. Le contenu était prêt, l'outil ne l'était pas.

**Réflexe** : pour un contenu long, non trivialement échappable, ou dans une langue accentuée,
utiliser directement l'outil d'écriture de fichier. Réserver l'interpréteur de commandes à
l'inspection et aux opérations courtes.

## L10 — La transition entre deux régimes est asymétrique par nature

*(S01)* Faire entrer une information dans un domaine détaillé est gratuit (c'est un terme de
forçage). L'en faire sortir demande un mécanisme (mesure de flux, agrégation, émission).
Les documents supposaient la symétrie, ce qui doublait inutilement le travail.

**Réflexe** : dans tout couplage grossier/fin, examiner les deux sens séparément. Ils n'ont
presque jamais le même coût.

## L11 — Champ d'abord, particules ensuite

*(S02)* Écume, aération, embruns : la tentation est de tout mettre en particules parce que le
phénomène *est* granulaire. Mais ce qui persiste et dérive se représente mieux par un champ
advecté, et les particules ne servent que le proche-caméra. Le même arbitrage s'est reproduit à
l'identique pour l'audio, où le lit d'ambiance remplace les émetteurs individuels à distance.

**Réflexe** : demander si le phénomène doit *persister et se déplacer*. Si oui, c'est un champ ;
les particules sont sa garniture, jamais son support.

## L12 — Chercher le facteur limitant, pas le facteur évident

*(S02)* Un compartiment ne s'inonde pas à la vitesse de l'eau qui entre, mais à celle de l'air qui
sort. Une plaque de glace ne porte pas par flottabilité mais par résistance en flexion. On n'est
pas emporté par la profondeur mais par le produit `d·(v+0,5)`.

Dans les trois cas, le paramètre auquel on pense en premier n'est pas celui qui gouverne, et
l'erreur va toujours dans le sens dangereux : trop rapide, trop portant, trop franchissable.

**Réflexe** : pour tout phénomène de seuil, identifier explicitement la grandeur limitante avant
d'écrire la formule. Elle est rarement celle qui donne son nom au phénomène.

## L13 — Une donnée analytique offre des capacités qu'une simulation ne peut pas offrir

*(S02)* Parce que B est une fonction fermée du temps, la marée est **prédictible** : le système
peut annoncer qu'un gué se fermera dans quarante minutes. Aucune simulation, même parfaite, ne
donnerait cela à ce prix.

**Réflexe** : après avoir choisi une représentation analytique pour des raisons de coût, faire le
tour de ce qu'elle rend *possible* et pas seulement de ce qu'elle rend *moins cher*. Les bénéfices
secondaires d'un choix d'architecture sont rarement listés, et ils orientent le design.

## L14 — Donner le chiffre juste évite une passe d'ajustement tardive

*(S02)* Une mer force 7 est blanche à 4 %, pas à 40 %. Sans la relation de Monahan, ce réglage
serait poussé dix fois trop haut, puis corrigé en fin de production après discussion.

**Réflexe** : partout où un artiste ou un designer devra choisir une valeur à l'œil, chercher s'il
existe une loi qui la donne. Fournir la loi coûte une ligne et supprime un débat.

## L15 — Écrire le test révèle l'erreur que la relecture ne voit pas

*(S03)* ADR-010 avait été relu plusieurs fois avec son temps de vidange faux d'un facteur deux.
L'erreur est apparue à la seconde où il a fallu écrire l'assertion : formuler la référence force à
intégrer, et l'intégration contredit l'estimation.

**Réflexe** : pour toute valeur dérivée publiée, écrire la vérification avant de la publier. La
relecture confirme la cohérence interne d'un texte, jamais sa justesse.

## L16 — L'instrument de mesure est une contrainte de conception, pas un livrable annexe

*(S03)* Le besoin de milliers d'exécutions rapides impose que le système soit instanciable sans le
moteur. Cette exigence détermine la structure du code et doit être actée avant la première ligne :
un système écrit sans harnais ne se laisse pas instrumenter ensuite.

**Réflexe** : concevoir l'outil de mesure en même temps que l'objet mesuré, et laisser ses
contraintes remonter dans l'architecture. Un raisonnement mené à l'envers — de la mesure vers le
code — produit des contraintes qu'aucune analyse directe n'aurait trouvées.

## L17 — Comparer à qualité égale, jamais à réglage égal

*(S03)* Deux solveurs comparés à `dx` identique se départagent sur le coût par cellule, qui n'est
pas la grandeur utile. Un candidat deux fois plus cher par cellule mais correct à `dx` double fait
seize fois moins de travail.

Vaut pour toute comparaison d'implémentations : régler chaque candidat jusqu'à une qualité cible
commune, **puis** comparer les coûts. Une campagne menée à réglage fixe désigne le mauvais
candidat, et le résultat est d'autant plus difficile à défaire qu'il est étayé par des chiffres.

## L18 — Mesurer le bruit avant de mesurer le signal

*(S03)* Une paire nulle dans un jury perceptuel, un écart-type d'exécution avant un seuil de
performance : dans les deux cas, la mesure du plancher de bruit précède l'interprétation de tout
résultat.

**Réflexe** : tout protocole de comparaison doit produire son propre plancher de bruit. Sans lui,
on ne sait pas si un écart observé signifie quelque chose — et un jury motivé, comme un tableau de
bord, trouvera toujours des différences.

## L19 — Rendre l'interdit inexprimable plutôt que de l'interdire

*(S04)* Une règle qu'on peut enfreindre finit par l'être. Une opération que l'interface ne permet
pas de nommer ne le sera jamais.

Appliqué en S04 : aucune fonction de lecture GPU synchrone n'existe dans `IGpuBackend` ; aucun type
n'exprime une coordonnée monde ; `seal()` fait *échouer* l'allocation générale au lieu de la
compter ; `accumulate_force` ne mène qu'à la pose de rendu, ce qui rend l'invariant I-04 mécanique.

**Réflexe** : pour chaque règle qu'on s'apprête à écrire dans un document de revue, chercher
d'abord s'il existe une forme d'interface qui la rend impossible à violer. Une règle vaut mieux que
rien, mais c'est le dernier recours, pas le premier.

## L20 — Ce que l'appelant doit fournir n'apparaît qu'en écrivant la signature

*(S04)* Le besoin de dérivées du champ de fond — sans lesquelles l'équation de la perturbation est
fausse et l'architecture paraîtrait défaillante — n'est apparu qu'au moment de décider quels champs
`IBackgroundField::sample_batch` devait renvoyer. Quatre sessions de conception ne l'avaient pas
fait remonter.

Même famille que L15, autre déclencheur : L15 dit qu'écrire le test vérifie la *valeur* ; L20 dit
qu'écrire la signature révèle l'*exigence*. Dans les deux cas c'est la contrainte de formalisation,
et non la relecture, qui produit l'information.

**Réflexe** : écrire les signatures avant de croire une conception terminée. Le passage de la prose
à la liste d'arguments est un filtre que rien d'autre ne remplace.

## L21 — Ressources et capacités dérivées ne cohabitent pas dans un même profil

*(S05)* Un profil de qualité déclarait mémoire, budget CPU, budget GPU **et** un nombre maximal de
domaines. Les deux premières valeurs étaient vérifiées cohérentes entre elles ; la troisième
contredisait la quatrième d'un facteur six. Personne ne l'a vu pendant quatre sessions, parce que
chaque vérification n'avait porté que sur deux termes à la fois.

**Réflexe** : dans toute table de configuration, séparer ce qu'on *alloue* de ce qu'on *en déduit*.
Une capacité dérivée écrite comme une ressource finit toujours par contredire les ressources qui
l'entourent, et le fait silencieusement.

## L22 — Deux parades identiques inventées séparément signalent un concept manquant

*(S05)* Deux ADR écrits à une heure d'intervalle ont jugé la même grille d'adressage trop grossière
et proposé chacun sa subdivision locale. Ce n'était pas deux détails : c'était un mécanisme absent
du document qui aurait dû le porter — et deux implémentations divergentes garanties.

**Réflexe** : quand deux documents contournent le même obstacle par des moyens voisins, ne pas
arbitrer entre les deux moyens. Remonter au document qui aurait dû fournir le mécanisme.

## L23 — Une « exception contrôlée » est presque toujours une grandeur mal classée

*(S05)* Un texte se déclarait « exception contrôlée » à un invariant, tout en décrivant quelque
chose qui n'en était pas une. Le contenu était juste, l'étiquette fausse — et une exception admise
se cite, puis se généralise, jusqu'à ce que l'invariant soit perdu par accident de vocabulaire.

**Réflexe** : devant une exception revendiquée, chercher d'abord la reformulation qui la fait
disparaître. Elle existe presque toujours, et elle renforce la règle au lieu de l'affaiblir.

## L24 — Une contradiction contient souvent une simplification

*(S05)* Deux documents attribuaient deux origines incompatibles à une même onde répliquée. Le
mécanisme de sécurité destiné à réconcilier les deux — plafonner la demande d'un client par une
cause connue du serveur — contenait sa propre réfutation : *si le serveur connaît la cause, il peut
émettre l'événement lui-même*. La résolution a supprimé un chemin de données, un mécanisme de
validation et un angle mort de sécurité d'un seul coup.

**Réflexe** : ne pas chercher à départager les deux branches d'une contradiction. Chercher
l'hypothèse commune qui les rendait toutes deux nécessaires. Une bonne résolution retire du
système ; elle n'y ajoute pas un arbitre.

## L25 — Une donnée dérivée ne se retouche jamais

*(S06)* Un pipeline ne meurt pas de complexité, il meurt d'ambiguïté : dès qu'une donnée a deux
origines — un fichier d'auteur et une passe de génération qui l'écrase — plus personne ne sait
laquelle fait foi, et les corrections manuelles disparaissent au recalcul suivant.

**Réflexe** : une seule source de vérité par donnée. Un besoin de correction manuelle devient une
**entrée supplémentaire**, jamais une retouche du produit. Si la retouche est impossible à exprimer
comme une entrée, c'est le modèle d'entrée qui est incomplet.

## L26 — Le partitionnement d'un recalcul suit les dépendances, pas la géométrie

*(S06)* Retoucher un haut-fond invalide la réfraction jusqu'à l'isobathe où la houle cesse de
sentir le fond — dix kilomètres au large sur un plateau doux. Une grille carrée de partitions
découperait cette dépendance au mauvais endroit et forcerait à recuire des zones sans rapport.

Vaut pour tout système d'invalidation ou de cache : la portée d'une modification est une propriété
du **phénomène**, pas du découpage qu'on a sous la main. Découper d'abord, mesurer la propagation
ensuite, c'est se condamner à recalculer trop ou pas assez.

## L27 — Ce qui n'est pas dans le dépôt n'existe pas pour la session suivante

*(S06)* Le rôle, les règles de travail, l'état du projet et les arbitrages en attente vivaient dans
une mémoire privée de compte. Elle ne voyage pas. Une continuation par un autre compte, une autre
machine ou une autre personne aurait dû tout reconstituer.

**Réflexe** : tout ce dont une reprise a besoin est un fichier du dépôt, pas un souvenir. Et la
passation ne se documente pas seulement, elle se **ritualise** : une liste de fin de session
exécutée à chaque fois, faute de quoi la session suivante repart de la lecture du code source de
la précédente — c'est-à-dire de rien.

## L28 — Ce qui est déclaré avant survit ; ce qui est écrit après ne survit pas

*(S07)* Une session coupée par une limite d'usage n'a aucune occasion d'écrire « je m'arrête ».
Tout dispositif de passation qui suppose une action au moment de l'arrêt — un résumé final, une
mise à jour d'état, un message de clôture — est inutile précisément dans le cas pour lequel on
l'avait conçu.

La seule information exploitable est **antérieure** : un plan déclaré avant le travail, et un
journal de ce qui a été effectivement validé. L'écart entre les deux est exactement ce qui a été
interrompu, et il se lit sans rien deviner.

**Réflexe** : pour tout mécanisme de reprise, se demander *à quel moment il écrit*. S'il écrit à la
fin, il ne protège que les cas où rien n'a mal tourné.

## L29 — Une impasse explorée est un résultat, et c'est celui qu'on perd

*(S07)* Un gestionnaire de version conserve les fichiers, jamais le raisonnement. Ce qui disparaît
d'abord dans une interruption, ce n'est pas le travail produit — il est sur le disque — c'est
*« j'ai essayé X, ça ne marche pas parce que Y »*. Sans cette phrase, la session suivante réexplore
la même impasse intégralement, et peut fort bien s'y arrêter plus longtemps.

**Réflexe** : consigner les chemins écartés avec leur motif, au même titre que les décisions
prises. Un espace prévu pour cela dans le journal de travail coûte trois lignes et évite des
heures.

## L30 — Un protocole qu'on n'a pas exécuté est un protocole faux

*(S07)* Le dispositif de reprise a été appliqué à sa propre écriture. Il a produit deux défauts en
deux étapes : la première étape d'un protocole d'écriture anticipée ne peut pas être protégée par
ce protocole, et la case d'avancement doit se cocher juste avant le commit et non avant le travail,
faute de quoi l'historique ment.

Aucun des deux n'était visible à la relecture. Tous deux sont apparus à la première exécution.

**Réflexe** : exécuter un protocole sur lui-même, ou sur un cas réel, avant de le publier. Même
famille que L15 et L20 — c'est la contrainte d'exécution, jamais la relecture, qui produit
l'information.

## L31 — Le chemin tiré et le chemin poussé sont deux interfaces, pas une

*(S08)* Une spécification d'interfaces écrite depuis le consommateur qui *interroge* laisse passer
tout ce que le système **publie sans qu'on le lui demande**. Le défaut est invisible à la relecture
— il n'y a rien à relire — et il se manifeste ailleurs : chaque consommateur décrit alors sa propre
publication dans son propre vocabulaire. Ici trois ADR l'ont fait séparément, et trois interfaces
inter-équipes se sont retrouvées sans document à soumettre.

**Réflexe** : pour tout système, énumérer **séparément** ce qu'on vient lui demander et ce qu'il
annonce de lui-même. Le second n'apparaît jamais en écrivant les signatures du premier. C'est L20
poussé d'un cran — écrire les signatures révèle l'exigence, mais seulement des appels qu'on a pensé
à écrire.

## L32 — Une exigence de déterminisme doit nommer sa portée, ou elle est fausse

*(S08)* « Reproductible bit à bit » sans dire *entre quoi et quoi* est soit trivial, soit
inatteignable. Ici l'exigence portait sur une cuisson qui fait tourner un solveur dont un autre
document dit qu'il n'est jamais déterministe entre machines. Chaque phrase était juste dans son
document ; leur conjonction était impossible.

Et quand une telle exigence est inatteignable, la question n'est pas comment l'atteindre mais **ce
qu'elle protégeait**. Ici : que tous les participants chargent le même octet — obtenu par un
producteur unique, pas par un calcul reproductible partout. La bonne résolution a retiré une
exigence sans ajouter un mécanisme, comme en L24.

**Réflexe** : derrière toute exigence de déterminisme, écrire la propriété observable qu'elle sert.
Elle a souvent une réalisation beaucoup moins chère, et la portée non dite est l'endroit où
l'exigence devient fausse.

## L33 — Un facteur d'économie fondé sur un rapport d'échelles n'est jamais une constante

*(S08)* Le ×64 de l'échantillonnage grossier tenait au rapport `λ_cut/dx`, qui varie d'un facteur
2,5 entre deux régimes écrits dans les mêmes documents — 10 points par longueur d'onde au scénario
nominal, 4 dans la zone de déferlement. L'optimisation s'effondrait donc précisément dans le
domaine le plus gros, celui qu'elle devait rendre abordable.

**Réflexe** : quand une optimisation est justifiée par « X est grand devant Y », écrire le rapport,
chercher dans les documents déjà écrits le régime où il est le **plus petit**, et transformer le
taux constant en **contrainte** (`dx ≤ λ_cut/N`). Un taux se copie ; une contrainte se vérifie.

## L34 — Un contrôle de cohérence entre deux nombres ne voit pas que les deux sont mal fondés

*(S08)* La revue S05 avait classé « cohérence mémoire de SPEC-001 §2.4 avec ADR-012 §3 » parmi les
contrôles passés. Elle l'était : le rapprochement des deux budgets est juste. Mais les comptages de
cellules dont ils dérivent appliquaient trois taux d'occupation différents sans le dire. Un audit
par paires ne peut pas voir cela — il vérifie une relation, jamais une provenance.

**Réflexe** : un audit de cohérence et un audit de provenance sont deux passes distinctes, et la
seconde ne se déduit pas de la première. Même famille que L21 — vérifier deux termes à la fois
laisse structurellement passer ce qui est faux en amont des deux.