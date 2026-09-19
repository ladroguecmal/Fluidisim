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

## L35 — Une décision qui supprime un mécanisme doit lister les signatures qu'elle périme

*(S09)* La revue S05 avait supprimé le chemin d'énergie client → serveur. Elle avait corrigé les
paragraphes concernés dans trois ADR. Mais la **fonction** qui servait ce chemin,
`drain_outgoing_events()`, est restée dans la spécification d'interfaces — dont le statut est
« dernier document avant l'écriture de code ». Elle aurait été implémentée, et personne n'aurait su
dire à quoi elle servait.

Une décision se propage naturellement vers la prose, qui l'explique, et pas vers les signatures, qui
n'ont l'air de rien affirmer.

**Réflexe** : après toute décision qui *retire* un mécanisme, parcourir les interfaces à la
recherche de ce qui le servait. Une signature orpheline ne se signale jamais d'elle-même — elle
compile.

## L36 — Un flottant court se choisit sur l'étendue de la grandeur, jamais sur la précision voulue

*(S09)* Deux fois dans un même document : un volume déplacé en millilitres sature un `half` à
65 litres, dépassé par toute claque de coque ; un flux d'énergie en W/m le sature à 65 kW/m,
dépassé dès une mer à `Hs = 4 m`. Dans les deux cas le raisonnement fautif était le même — « deux
octets suffisent, on n'a pas besoin de plus de précision » — et il portait sur la mauvaise
propriété.

L'erreur est invisible en relecture et se manifeste comme une **saturation silencieuse au cas le
plus spectaculaire** : la plus grosse explosion, la plus grosse tempête. C'est-à-dire au moment où
elle se voit le plus et où on la remarquera le plus tard.

**Réflexe** : avant de fixer l'unité d'un champ à format court, calculer le **cas extrême du
projet** et choisir l'unité pour qu'il tienne. La précision se déduit ensuite ; elle est presque
toujours surabondante.

## L37 — Publier des réductions, jamais des champs, à travers une frontière lente

*(S09)* L'audio veut savoir « combien d'écume autour de l'auditeur ». La réponse tient en
45 octets. Le champ qui la contient pèse 4,2 Mo par cascade, et le rapatrier au CPU coûterait
≈500 Mo/s et une à trois frames de latence — pour une question à laquelle une réduction GPU répond
dans la passe qui produit déjà le champ.

Vaut pour toute frontière lente : GPU → CPU, réseau, processus, disque. Et le corollaire est aussi
utile : **un même champ peut avoir plusieurs publications de formes différentes**, une par classe de
consommateur. Le rendu prend une poignée sans copie, l'audio prend une intégrale. Chercher une forme
unique aurait fait payer à l'un ce dont l'autre a besoin.

**Réflexe** : à chaque frontière lente, demander quelle *question* le consommateur pose, et non
quelle *donnée* il croit vouloir. La réponse est souvent de plusieurs ordres de grandeur plus
petite que la donnée.

## L38 — Deux mécanismes corrects séparément produisent un défaut dont personne n'est propriétaire

*(S09)* L'anticipation locale d'un événement est correcte : elle masque la latence réseau. Un bus
d'événements partagé par plusieurs consommateurs est correct : il évite N interrogations. Ensemble,
ils font jouer deux fois le même impact à 100–300 ms d'intervalle — un défaut qu'aucun des deux
documents ne peut voir, parce qu'il n'appartient à aucun des deux.

**Réflexe** : quand un **second consommateur** rejoint un canal existant, réexaminer les garanties
du *canal*, pas la correction du consommateur. Les défauts de conjonction se trouvent là, et
uniquement là — chacun des composants passe sa propre revue.

## L39 — Une donnée prédictive porte sa cause, ou elle ment

*(S09)* « Ce gué se ferme dans quarante minutes » est vrai *si seule la marée agit*. Cette clause
n'était écrite nulle part. Une vanne ouverte en amont laisse la valeur publiée en place, fausse,
jusqu'à la prochaine republication — et un consommateur qui a planifié dessus maintient son plan.

La forme correcte publie la **cause supposée** à côté de la valeur, et sait publier « je ne sais
plus » : c'est un résultat, pas un échec, et c'est la seule façon de dire à un consommateur que sa
planification doit être refaite.

**Réflexe** : toute valeur prédictive publiée s'accompagne de l'hypothèse sous laquelle elle vaut,
et d'un mécanisme qui l'invalide quand l'hypothèse tombe. Sans quoi la prédiction survit à sa
propre validité — et une prédiction périmée est plus nuisible qu'une absence de prédiction.

## L40 — Un point reporté échappe aux audits

*(S10)* Une contradiction interne à une seule session — un ADR réclamant un format pour un
mécanisme qu'un autre ADR de la même session venait de dissoudre — a traversé **neuf sessions, une
revue croisée des vingt ADR et une revue croisée des cinq spécifications**. Elle était visible en
rapprochant deux paragraphes.

Le motif n'est pas l'inattention, il est de forme : le point vivait dans une liste « ce qui reste
ouvert ». Un audit vérifie ce qui est **affirmé** ; un point reporté se lit comme une lacune connue
et suivie, c'est-à-dire comme quelque chose dont on sait déjà qu'il n'est pas résolu. Personne ne va
vérifier qu'une question ouverte **a encore un objet**.

**Réflexe** : auditer les listes de points ouverts comme on audite les décisions. Trois questions
par point — a-t-il encore un objet ? sa formulation tient-elle encore ? quelqu'un attend-il quelque
chose dessus ? Un point reporté qui a perdu son objet est plus coûteux qu'une contradiction
visible : il crée du travail futur pour rien, et il porte l'autorité d'une tâche planifiée.

## L41 — Une dégradation plus rapide que la restauration de ce qu'elle détruit fait pomper

*(S10)* Le mécanisme de dégradation détruisait des domaines en engageant sa décision pour une
seconde, alors que certains de ces domaines demandent quatre à huit secondes pour se rétablir. Le
résultat n'est pas une économie mais un battement, et le battement est **plus visible que la
dégradation qu'on cherchait à éviter** — ce que le même document énonçait déjà pour un autre de ses
mécanismes, sans l'avoir appliqué à celui-ci.

**Réflexe** : dans tout système qui détruit puis recrée pour économiser — domaines, caches, tampons,
connexions, processus — comparer explicitement la **fenêtre d'engagement** au **coût de
restauration**. Ce sont deux nombres, ils vivent en général dans deux documents, et personne ne les
met côte à côte.

## L42 — Quand quatre situations indépendantes ont la même réponse, la réponse est la bonne

*(S10)* Sauvegarde, rechargement, arrivée en cours de partie, reconnexion et redémarrage de serveur
demandent exactement le même objet — un temps, un journal d'événements, des entiers. Ce n'est pas
une coïncidence commode : c'est la conséquence d'une décision antérieure (l'état du monde se réduit
à un temps et à un journal), et le fait que quatre chemins y mènent est ce qui donne confiance dans
la décision.

Le corollaire est la partie actionnable : **si quatre situations voisines demandent quatre
réponses différentes, le mécanisme commun n'a probablement pas encore été trouvé.** C'est le même
signal que L01 — plusieurs questions qui résistent séparément partagent une prémisse — vu depuis
l'autre côté, une fois la décomposition trouvée.

**Réflexe** : après avoir conçu une réponse, compter combien de situations indépendantes elle
couvre. C'est un test de justesse plus fort qu'un argument, et il est gratuit.

## L43 — Une correction s'applique là où vit l'affirmation, jamais dans la liste des absences

*(S11)* Quatre points ouverts réclamaient un mécanisme qui existait déjà, ou une décision déjà
prise. Dans un cas, la correction et le point périmé étaient dans **le même document, à quatre
sections d'écart** — l'hypothèse de la distance ne tient donc pas.

La cause est de forme. Une correction se pose là où vit l'affirmation qu'elle corrige. **Un point
ouvert n'affirme rien : il déclare une absence**, et personne ne relit une liste d'absences en se
demandant si l'une d'elles a été comblée. C'est ce qui rend cette dette invisible aux audits de
cohérence, qui comparent des affirmations entre elles.

Généralisation de L35 (une signature survit à la décision qui la vide) et de L40 (un point reporté
échappe aux audits) : les trois décrivent le même angle mort vu sous trois éclairages — **ce qui
n'affirme rien ne se relit pas**.

**Réflexe** : toute session qui décide ou corrige quelque chose termine par une **recherche de
texte** sur ce quelque chose dans les listes de points ouverts. Pas une relecture — une recherche.
Coût : une minute. En S11, quatre points sur 110 en dépendaient.

## L44 — Un point ouvert qui se justifie par le fait que d'autres le posent ne devrait pas exister

*(S11)* Trois documents posaient la même question, et deux le savaient : « non traitée ici **comme
elle ne l'est pas dans SPEC-004** », « **comme partout**, dépend du langage ». Aucun ne désignait de
porteur. Un doublon ne nuit que le jour où quelqu'un y répond — il tranche alors dans un document et
pas dans l'autre — et c'est précisément ce jour-là qu'on ne le verra pas.

La formulation qui reconnaît la répétition est le symptôme : elle prouve que l'auteur a vu le
doublon et l'a traité comme une excuse au lieu d'un défaut.

**Réflexe** : au moment d'écrire un point ouvert, chercher qui d'autre le pose et **nommer le
porteur** — le document dont la question relève du *domaine*, pas celui qui l'a écrite en premier.
Un renvoi tient en une ligne. Sur les huit points de la spécification la plus chargée du corpus, six
nomment déjà leur porteur : le coût est nul quand on y pense à l'écriture, et il devient un audit
quand on n'y pense pas.

## L45 — « Qui attend quoi » est un livrable qu'aucun audit de cohérence ne produit

*(S11)* Un audit demande d'ordinaire si les documents se contredisent. Ajouter une troisième
question — *qui attend, et quoi ?* — a produit, sur le même passage et sans travail supplémentaire :
sept destinataires extérieurs que la liste officielle des dépendances ne portait pas, un cinquième
arbitrage humain rangé jusque-là parmi des choix de format, quatre travaux de conception que
personne n'avait planifiés, et le classement des bancs de mesure par nombre de points débloqués —
lequel a confirmé le chemin critique par une voie indépendante de celle qui l'avait établi.

Aucun de ces éléments n'est une incohérence. Ils étaient tous **écrits, exacts et invisibles**,
parce qu'ils vivaient chacun dans un point ouvert, c'est-à-dire dans l'endroit où l'on ne cherche
ni une dépendance, ni une tâche, ni une décision à prendre.

**Réflexe** : après tout audit de correction, faire une seconde passe qui ne demande plus « est-ce
juste ? » mais « **qui est bloqué ?** ». Les deux passes lisent le même texte ; elles n'en tirent
pas la même chose, et la seconde produit un document que quelqu'un peut utiliser tel quel.

## L46 — Publier la grandeur qu'on sait, pas celle qu'on veut

*(S12)* Le réflexe, pour un impact, est de publier la pression de pic : c'est ce qu'on voit, ce
qu'on mesure en essai, ce qu'un artiste demande. Mais la théorie qui la donne **diverge** dans le
régime limite — relèvement de carène tendant vers zéro — et ce que le coussin d'air et la
compressibilité y font n'est pas modélisé. Un modèle qui publie cette grandeur publie son
incertitude, et la publie précisément dans le cas le plus spectaculaire.

L'impulsion, elle, est un **bilan de quantité de mouvement**. Elle est bornée par la masse d'eau
réellement accélérée, elle ne peut pas être fausse d'un ordre de grandeur, et elle se trouve être ce
que le consommateur — un intégrateur de corps rigide — sait appliquer exactement.

**Réflexe** : devant une grandeur qui diverge ou s'effondre dans un régime limite, chercher
**l'intégrale ou la quantité conservée dont elle dérive**. Elle est presque toujours plus robuste,
souvent plus directement utilisable, et le passage de l'une à l'autre est un contrôle croisé gratuit
— ici, 1,1 MN moyens sur 73 ms contre 105 kPa de pic sur la surface mouillée.

## L47 — Un critère de bascule ne mesure qu'une chose, et l'on croit qu'il les couvre toutes

*(S12)* Un mode cinématique contraint existait, avec un critère d'entrée fondé sur la **stabilité
numérique** (`ω·dt > 1`). Il était juste. Le nageur, qui a besoin de ce mode exactement, passe le
critère sans difficulté — parce qu'il en relève pour une autre raison : le contrôle et la caméra.

Un mode a des effets ; un critère a un motif. Tant qu'il n'y a qu'un critère, on confond les deux, et
tout cas qui aurait besoin du mode pour un autre motif passe au travers sans que rien ne signale
l'omission.

**Réflexe** : pour tout mécanisme à bascule, écrire séparément **ce que le mode fait** et **ce qui le
déclenche**, puis se demander qui d'autre a besoin de ce que le mode fait. La réponse ajoute une
condition d'entrée, pas un mode — et un mode de moins est toujours un gain.

## L48 — Un phénomène permanent coûte en régime nominal, pas en pic

*(S12)* On dimensionne spontanément un budget sur les pics : la bataille navale, l'explosion, la
tempête. Un décor permanent — deux cents rochers qui brisent — modélisé par émission d'événements
coûterait **dix fois le pic, en continu**, sans qu'aucun scénario de test conçu pour chercher des
pics ne le révèle.

Et le raisonnement mène plus loin que le budget : un phénomène **stationnaire et déterministe** n'a
aucune raison d'être transmis, puisque chaque destinataire le dérive à l'identique. La question
« combien cela coûte-t-il ? » et la question « pourquoi est-ce transmis ? » ont ici la même réponse.

**Réflexe** : pour tout phénomène toujours actif, multiplier par son **nombre** et par sa
**fréquence** avant de choisir sa représentation. Si le produit dépasse le pic qu'on avait budgété,
la représentation est fausse — et c'est en général qu'on transmet ce qu'on pourrait dériver.

## L49 — Un invariant ne s'audite jamais, parce qu'il n'est ni une affirmation ni une absence

*(S13)* Deux invariants sur dix-sept se sont révélés faux à la première tentative : l'un exigeait un
mécanisme aboli huit sessions plus tôt, l'autre était universel là où l'ADR qu'il cite en source le
contredit pour la moitié des cas.

Un audit a deux prises connues — les **affirmations**, qu'on confronte entre elles ; les
**absences**, qu'on relit pour vérifier qu'elles ont encore un objet (L40). Un invariant n'est ni
l'une ni l'autre : il se présente comme le **socle** contre lequel on vérifie le reste, et la
référence qu'il porte se lit comme une provenance et non comme une dépendance à surveiller. On ne
vérifie donc jamais le mètre étalon.

Et l'erreur y coûte double. Un invariant faux ne produit pas une erreur mais deux, en sens
contraires : quelqu'un l'applique, ou quelqu'un constate qu'il n'est pas tenu et « rétablit » ce
qu'il décrit — c'est-à-dire rouvre ce qu'une décision avait fermé, **en obéissant à la règle**.

**Réflexe** : toute session qui écrit une décision relit les règles fondamentales que cette décision
cite, et se demande si l'une d'elles devient fausse. Le corpus des invariants est court par
construction ; c'est une lecture de trois minutes, et c'est la seule occasion où quelqu'un les
regarde autrement que comme un juge.

## L50 — Ce qui n'a jamais été vérifié d'une certaine manière est faux de cette manière-là

*(S13)* Trois structures sur trois portaient une taille fausse, et l'erreur d'origine s'était
propagée dans six documents, chacun recalculant depuis le chiffre faux du précédent. Le corpus avait
pourtant été audité deux fois : S05 a contrôlé les tables numériques dupliquées, S08 a recalculé une
quarantaine de valeurs depuis leurs formules. **Personne n'avait additionné les champs d'un
`struct`** — et le taux d'erreur dans cette classe non contrôlée a été de 100 %.

Ce n'est pas un hasard : une classe de vérification qu'on n'a jamais faite ne bénéficie d'aucune
correction accidentelle. Les valeurs vérifiées se corrigent au fil des relectures ; les autres
dérivent librement, et rien ne signale leur âge.

**Réflexe** : après tout audit, écrire **ce qu'il n'a pas regardé**. La liste des classes de contrôle
non passées vaut mieux qu'un score : elle dit exactement où chercher la prochaine fois, et l'on peut
parier sur son rendement.

## L51 — Une correction crée un précédent, pas un réflexe

*(S13)* La revue S05 avait trouvé que les poches d'air n'avaient pas de ligne dans la table
d'autorité, et fait ajouter deux lignes à cette table. Sept sessions plus tard, une nouvelle force
autoritaire a été spécifiée — et n'a pas eu de ligne. Le mécanisme d'ajout était connu, documenté,
appliqué une fois, et non réappliqué.

Une correction ponctuelle ne se généralise pas d'elle-même. Ce qui la généralise est soit un
**invariant** — qui la rend refusable —, soit une **entrée de rituel** — qui la rend systématique.
Sans l'un ou l'autre, elle reste un événement de l'historique, et l'historique ne se relit pas.

**Réflexe** : en corrigeant un écart, se demander s'il appartient à une **classe**. Si oui, la
correction ne suffit pas : il faut le mécanisme qui la reproduira sans qu'on y pense. Trois écarts de
S13 — E08, E02, E04 — sont des récidives ou des premières d'une classe, et c'est ce qui justifie
qu'ils aient produit des angles morts plutôt que de simples notes.

## L52 — Un invariant qui nomme un mécanisme vieillit avec lui

*(S14)* Dix invariants sur dix-sept ne disaient plus ce que leur source dit, et le partage est net.
Tiennent : ceux qu'une **signature rend mécaniques** — `g_eff` injecté dans la signature de
configuration, aucun type n'exprimant une coordonnée monde — et ceux qui servent à **décider**
plutôt qu'à refuser, invoqués pour trancher *comment* faire. Ont vieilli : ceux qui **nomment un
mécanisme** — une fonction, un plafonnement, une catégorie de valeur.

La raison est simple une fois vue : un mécanisme est remplaçable, une propriété ne l'est pas. Une
règle qui désigne le moyen devient fausse dès qu'on change de moyen, alors même que l'intention
qu'elle servait n'a pas bougé — et elle devient fausse **en silence**, puisque personne ne relit le
socle.

**Réflexe** : en écrivant une règle fondamentale, la formuler comme une **propriété observable** et
non comme un passage obligé. « Aucun code ne reconstruit la surface par ses propres moyens » survit ;
« tout consommateur passe par telle fonction » ne survit pas à l'ajout d'un second chemin. Et quand
la propriété peut être rendue **inexprimable par une signature**, c'est encore mieux : ce sont
exactement les invariants qui ont tenu.

## L53 — Les revues croisées ne confrontent que des documents qui se citent

*(S14)* Le défaut le plus grave du corpus a survécu treize sessions : un ADR transférait la masse
d'un compartiment à un solveur non déterministe, ce qu'un invariant interdit explicitement. Deux
revues croisées et deux audits ne l'avaient pas vu.

La cause est mécanique. Une revue croisée part des renvois : on confronte A et B parce que A cite B.
Or **cet ADR ne cite pas l'invariant, et l'invariant ne cite pas cet ADR** — leur seul point de
contact était un mot, « masse », qui n'apparaît dans aucune des deux listes de dépendances. Le graphe
des citations a des composantes non connexes, et une revue qui le suit ne visite jamais l'arête
manquante.

**Réflexe** : au moins un passage d'audit doit ignorer les renvois et parcourir **chaque règle contre
tout le corpus**, pas contre ses sources déclarées. C'est plus long, cela ne se fait pas à chaque
session — mais c'est le seul passage qui trouve ce que le graphe des citations ne relie pas.

## L54 — Un outil qui échoue à moitié est pire qu'un outil qui échoue

*(S14)* Deux blocs de formules ont été silencieusement amputés parce que des accents graves, passés
à un interpréteur de commandes, y ont été lus comme une substitution de commande. L'outil a **signalé
une erreur de syntaxe et a poursuivi** : le fichier a été écrit, tronqué, et le script a annoncé sa
réussite.

Un échec franc se voit. Un échec partiel produit un artefact plausible, et il ne se voit que si l'on
relit. Ici il a été vu ; il aurait pu ne pas l'être, et la formule manquante aurait été découverte
par quelqu'un qui essaie de l'appliquer.

**Réflexe** : après toute écriture produite par un outil de transformation — script, gabarit,
génération — **relire le résultat, pas le code de retour**. Et pour le contenu qui porte des
caractères que l'outil pourrait interpréter, passer par un fichier plutôt que par une ligne de
commande : c'est L09, dont ce cas est la troisième occurrence.

## L55 — Une action n'est exécutée que si elle entre dans une liste que quelqu'un relit

*(S15)* Sept actions du corpus annonçaient un ajout à un banc ou au harnais ; trois n'ont pas eu
lieu. Ni le document cible ni le délai ne les distinguent : **les quatre exécutées l'ont été par la
session qui les décidait, dans une étape inscrite à son plan**, et les trois perdues avaient été
annoncées dans le corps d'un document.

Cela vaut même à l'intérieur d'une seule session : une action décidée en écrivant un ADR, à l'étape
6, meurt si l'étape 7 ne la prévoit pas. La distance n'y est pour rien ; ce qui compte est
l'existence d'une liste qu'on relit avant de clore.

Le décompte inverse le confirme : les quatre registres d'audit qui portent une table d'actions
affichent **40 sur 40 exécutées**. Le dispositif fonctionne quand il existe.

**Réflexe** : quand une décision en engendre une autre — « à ajouter au banc », « à porter à tel
document » — **l'inscrire immédiatement dans le plan de la session**, ou la clore sur-le-champ. Une
annonce en prose est une intention, pas une tâche. Et le rituel de fin doit relever les actions
décidées en séance, faute de quoi elles ne survivent qu'au hasard.

## L56 — Un registre dit où un point est discuté, jamais s'il est refermé

*(S15)* Un registre d'angles morts a une colonne « traité dans » qui pointe vers un document. Elle
mêlait trois situations : le point **supprimé** par une décision, le point **comblé**, et le point
**décrit mais en attente d'un tiers** — une équipe, une mesure, un arbitrage. Rien ne les
distinguait.

Le coût est concret : le point de sévérité 1 le plus urgent du corpus — un outil qui place une plage
soixante-dix mètres au-dessus du niveau de la mer — affichait exactement la même chose qu'un point
réglé. Un lecteur en concluait qu'il était traité.

Et un registre qui accumule sans jamais se vider cesse d'être lu : quatre-vingt-neuf points en quinze
sessions, un seul portant une fermeture.

**Réflexe** : tout registre a besoin de deux colonnes distinctes — **où** le point est traité, et
**dans quel état** il est. La seconde ne s'invente pas à la fin : elle se remplit à chaque fois qu'une
session touche le point. Et la valeur qui compte le plus n'est ni *ouvert* ni *clos*, c'est
**« en attente de quelqu'un d'autre »** — la seule que personne ne pense à écrire, et la seule qui
dise qu'il n'y a rien à attendre de nous.

## L57 — Deux paragraphes qui ne se citent pas peuvent borner le même nombre

*(S16)* La largeur d'éponge est donnée par une formule dans un ADR ; les emprises des domaines de
référence sont données par un tableau dans une fiche chiffrée. Aucun des deux ne cite l'autre. Mis
côte à côte, ils **bornent par le haut le paramètre le plus connecté du projet** — et la borne tombe
en dessous de la valeur proposée depuis la première session.

Personne ne l'avait vu parce que personne n'avait de raison de rapprocher les deux : l'un parle de
zone d'absorption, l'autre de comptage de cellules. C'est le même angle mort que L53 — les revues
croisées ne confrontent que des documents qui se citent — mais vu depuis un autre côté : ici il ne
s'agit pas d'une contradiction à trouver, mais d'une **contrainte à calculer**, qui n'existe dans
aucun des deux documents et seulement dans leur produit.

**Réflexe** : avant d'exécuter une mesure, chercher ce que le corpus **impose déjà** au résultat.
Un banc qui balaie un paramètre doit d'abord savoir dans quel intervalle la réponse est admissible ;
cet intervalle est souvent calculable, et le calculer coûte une heure quand la mesure coûte des
semaines. Si l'intervalle est vide, on l'apprend avant de monter le banc et non pendant.

## L58 — Un banc produit un couple de valeurs liées, jamais une valeur seule

*(S16)* Le banc central du projet était écrit pour trancher une longueur d'onde de coupure. Il doit
en réalité trancher cette longueur **et** le taux d'échantillonnage du champ de fond, parce que les
deux sont liés par une même inégalité et qu'aucun ne veut rien dire sans l'autre.

Un protocole qui ne publie qu'une des deux valeurs ne laisse pas la seconde indéterminée : il la
laisse **se choisir plus tard, à l'œil, par quelqu'un qui n'aura pas les mesures**. C'est le pire des
trois cas — mieux vaut une valeur mesurée, ou une valeur explicitement ouverte, qu'une valeur qui
paraît acquise parce qu'elle figurait à côté d'une autre.

**Réflexe** : pour tout banc, écrire **ce qu'il publie**, au pluriel, avant d'écrire comment il
mesure. Si deux paramètres apparaissent ensemble dans une contrainte, ils sortent ensemble du banc
ou ils n'en sortent pas.

## L59 — Classer par ce que la réponse débloque, pas par la gravité du sujet

*(S17)* Le corpus listait ses dépendances extérieures par **gravité de conséquence** : d'abord les
interfaces dont un mauvais accord coûterait un recâblage, ensuite le reste. Reclassées par
**irréversibilité** — que débloque la réponse, qu'est-ce qui devient irrattrapable si elle tarde ? —
deux demandes remontent en tête que rien ne présentait comme urgentes, dont une rangée depuis quatorze
sessions parmi des questions de format de fichier alors qu'elle conditionne le premier livrable du
chemin critique.

Les deux classements répondent à des questions différentes : « qu'est-ce qui ferait le plus de dégâts
si c'était mal décidé » et « qu'est-ce qui ferait le plus de dégâts si c'était décidé tard ». Le
second est le seul qui serve à établir un ordre d'action — et c'est presque toujours le premier qu'on
écrit, parce qu'il se déduit du contenu au lieu de demander de penser au calendrier.

**Réflexe** : devant toute liste de choses à obtenir de quelqu'un d'autre, la retrier une fois par
« avant quoi ». L'ordre change, et ce qui remonte est en général ce que personne ne réclamait — parce
qu'un bloqueur silencieux ne se signale pas, il attend.

## L60 — Un dossier de demandes sans destinataire identifié ne part pas

*(S17)* Seize fiches prêtes, chiffrées, tenant chacune seule. Les deux plus bloquantes s'adressent à
« la direction technique » et à « l'assurance qualité technique » — des **rôles, pas des personnes**.
Personne n'est identifié pour les recevoir.

Le corpus pouvait tout produire sauf cela : la liste des demandes se déduit des documents, la liste
des destinataires non. C'est une information d'organisation, et elle n'est ni dans les ADR ni dans les
spécifications — elle n'y sera jamais.

**Réflexe** : quand un livrable est destiné à quelqu'un d'extérieur, vérifier que ce quelqu'un a un
**nom** avant de le déclarer prêt. Un document parfait adressé à une fonction reste dans le dépôt, et
son absence de réponse se lit à tort comme un désaccord ou comme une absence d'urgence.

## L61 — Un arbitrage qui traîne est souvent un arbitrage mal posé

*(S18)* Cinq questions étaient classées « en attente d'une réponse humaine » depuis douze sessions.
Mises en demeure d'être répondues, **trois n'étaient pas des arbitrages** : deux posaient une question
dont la prémisse était fausse — « qui porte le trait de côte » suppose qu'il soit stocké, « combien de
temps l'eau persiste » suppose qu'elle ait une politique propre — et la troisième cherchait **un**
propriétaire là où il en faut deux.

Le mécanisme est celui de L03, rencontré à un endroit nouveau : **la liste des choses qu'on ne décide
pas**. Une question qu'on a classée comme n'étant pas la sienne n'est plus interrogée — ni sur sa
réponse, ce qui est normal, ni sur sa **formulation**, ce qui ne l'est pas. L'étiquette protège la
question de l'examen qui l'aurait dissoute.

**Réflexe** : avant de porter une question à quelqu'un d'autre, l'examiner comme si on devait y
répondre soi-même — au moins jusqu'à savoir de quel type de réponse elle relève. Une question bien
posée qu'on transmet est un service ; une question mal posée qu'on transmet est une charge, et elle
revient.

## L62 — Une décision prise par délégation s'écrit pour être défaite

*(S18)* Quand quelqu'un délègue un arbitrage, il ne délègue pas son autorité : il délègue le travail
de l'instruire. La décision qui en sort doit donc être **plus argumentée** qu'une décision ordinaire,
et surtout **bon marché à inverser** — parce que celui qui a délégué reste le seul à pouvoir juger si
elle sert son projet, et qu'il jugera plus tard, avec des informations que je n'ai pas.

Trois choses le rendent possible, et elles tiennent en une ligne chacune : le **motif**, écrit pour
qu'on puisse le contester sans reconstituer le raisonnement ; le **chiffre** qui rend la décision
vérifiable ; et **ce qu'il faudrait changer si la réponse était l'inverse**. La troisième est celle
qu'on omet, et c'est la seule qui transforme une décision en décision *révisable*.

**Réflexe** : à chaque décision prise à la place de quelqu'un, écrire son chemin de retour. Sans lui,
la délégation devient un fait accompli — ce qui n'était pas ce qu'on demandait.

## L63 — Reporter à quelqu'un est plus confortable que reporter, et bien plus durable

*(S19)* Quatorze questions ont été classées « attend une réponse d'une autre équipe » sur seize
sessions. Aucune n'avait de destinataire : les équipes n'existaient pas.

L'étiquette faisait deux choses, et c'est la seconde qui coûte. Elle dispensait de répondre — c'est le
report ordinaire, et un audit le trouve. **Mais elle désignait aussi un responsable**, et un report
nominatif ne se relit pas : on ne vérifie pas qu'un tiers a répondu si l'on n'attend rien de précis
de lui, et l'absence de réponse se lit comme une absence d'urgence plutôt que comme une absence
d'interlocuteur.

C'est pourquoi l'audit qui cherchait exactement cette dette est passé à côté : il vérifiait qu'un
point ouvert avait encore un **objet**, jamais qu'il avait encore un **destinataire**.

**Réflexe** : pour toute question reportée à un tiers, écrire **qui**, nommément, et **depuis
quand**. Un nom absent ou un délai anormal sont deux signaux visibles ; « l'équipe X » n'en est pas
un. Et préférer la formulation « à trancher, sans interlocuteur » à « attend l'équipe X » : la
première laisse la question dans le champ de travail, la seconde l'en sort.

## L64 — Une propriété garantie par un type vaut mieux qu'une propriété défendue par une règle

*(S19)* Le déterminisme inter-plateforme peut s'obtenir en `f64` : sémantique IEEE stricte, pas de
contraction FMA, pas d'arithmétique étendue, ordre des opérations fixé. Toutes ces conditions sont
tenables, et **chacune se perd par un drapeau de compilation que personne ne relit**.

En entiers, la même propriété ne dépend de rien : une addition d'entiers 64 bits donne le même
résultat partout. Le choix ne portait donc pas sur la précision — les deux en ont largement assez —
mais sur **ce qui garantit la propriété** : un type, ou une discipline.

C'est L19 — rendre l'interdit inexprimable — transposé d'une interface vers une représentation. Et le
critère de décision qui en découle est net : entre deux représentations également capables, choisir
celle dont la propriété critique **survit à la négligence**.

**Réflexe** : devant un choix de représentation, ne pas comparer les capacités mais demander *qu'est-ce
qui casse la propriété que je tiens le plus à garder, et faut-il quelqu'un pour la casser ou suffit-il
d'une distraction ?*

## L65 — Une heure de code trouve ce que dix-neuf sessions de conception n'ont pas trouvé

*(S20)* Le corpus avait été audité six fois : ADR entre eux, spécifications entre elles, points
ouverts, documents récents, invariants dans les deux sens, registres. Cent points d'attention
recensés, trois erreurs arithmétiques débusquées, deux invariants réécrits.

**La première heure de code a trouvé qu'une fonction sinus n'est pas déterministe** — c'est-à-dire
qu'un invariant central du projet, vérifié et re-vérifié, était **inapplicable** sur le premier calcul
qui l'aurait employé.

La raison n'est pas que les audits étaient mauvais. C'est qu'un document affirme, et qu'un programme
**s'exécute**. Un audit compare des affirmations entre elles ; il ne peut pas découvrir qu'une
opération qu'aucun document ne mentionne — l'appel à `sin` — porte une propriété que tous supposaient.
Ce qui n'est écrit nulle part n'est audité nulle part.

**Réflexe** : dès qu'une propriété est revendiquée sur un calcul, écrire le calcul. Pas pour le
livrer — pour voir de quoi il dépend. La liste des dépendances d'un calcul réel est toujours plus
longue que celle de sa description, et l'écart est exactement là où vivent les défauts que la
relecture ne trouve pas.

## L66 — Un test peut affirmer une propriété vraie avec des données incapables de la révéler

*(S20)* Un test écrit pour démontrer que l'addition flottante n'est pas associative employait une
suite arithmétique. Les deux sommes étaient égales, et le test échouait — en prétendant que la
propriété était fausse. Elle ne l'était pas : les données étaient trop bien conditionnées.

Il a échoué, donc il a été corrigé. **Le cas inquiétant est l'autre** : le même test écrit dans
l'autre sens — « les deux sommes sont égales » — serait passé, et aurait affirmé une garantie
inexistante. Un test vert sur des données mal choisies est pire qu'un test absent, parce qu'il ferme
la question.

**Réflexe** : pour tout test qui démontre une propriété numérique, se demander **par quel mécanisme
ces données-là peuvent la révéler**. Si la réponse n'est pas immédiate, les données sont mal choisies.
Le corollaire tient en une ligne : un test de non-associativité a besoin d'ordres de grandeur
hétérogènes, un test de précision a besoin de valeurs proches de l'ulp, un test de saturation a besoin
du cas extrême du projet — et jamais de la valeur moyenne.

## L67 — Le déterminisme et la justesse sont deux propriétés sans rapport

*(S21)* La couche `B` produisait un champ dont le hash de conformité était parfaitement stable d'une
exécution à l'autre, d'une construction à l'autre, au bit près. Il l'était : le champ était
**reproductible**. Il était aussi **faux** — la vitesse orbitale en quadrature au lieu d'en phase,
c'est-à-dire de l'eau qui n'avance pas sous une crête.

Le point n'est pas qu'un test de déterminisme soit insuffisant : c'est qu'**aucun raffinement d'un
test de déterminisme ne s'approche de la justesse**. Plus de points d'échantillonnage, plus de
grandeurs hachées, plus de plateformes comparées : rien de tout cela ne rapproche d'un défaut de
physique, parce que ce n'est pas la même dimension. Un projet peut donc accumuler une couverture
déterministe impressionnante et n'avoir vérifié aucune équation.

**Réflexe** : pour toute batterie de tests, se demander laquelle des deux questions elle répond —
*« le même deux fois ? »* ou *« le bon ? »* — et vérifier qu'il existe des tests de l'autre famille.
Si la réponse est « les tests sont verts », la question n'a pas été comprise.

## L68 — Un écart nul est un signal à examiner, pas un résultat à encaisser

*(S21)* Quatre cas de C10 affichaient tous **0,000 %**. Deux étaient de vrais succès ; les deux
autres comparaient une force construite comme `ρ·g·A·d` à la référence `ρ·g·A` — la mesure et la
référence partageaient une ligne de code. Ils ne pouvaient pas échouer.

Un écart nul parfait, sur une grandeur qui traverse un calcul numérique, est rare et donc suspect :
il signifie le plus souvent que la référence n'est pas indépendante de la mesure. Un vrai succès
laisse une trace d'arrondi. Et le rapport ne distingue pas les deux : **la tautologie et la
démonstration s'y affichent identiques**, ce qui rend le défaut invisible à la lecture du résultat.

**Réflexe** : devant un `0,000 %`, remonter à la référence et se demander de quelle ligne de code
elle sort. Si c'est de la même que la mesure, le cas ne prouve rien — le dire dans le cas lui-même,
pas dans un registre séparé, et classer les cas d'une batterie par degré d'indépendance plutôt que de
les compter comme égaux.

## L69 — Une constante qu'aucun document ne fixe est choisie par le premier code qui en a besoin

*(S21)* Vingt-et-une sessions, six spécifications, vingt-neuf ADR : la masse volumique de l'eau
n'apparaissait nulle part. Le corpus citait `ρ_glace = 917`, le `ρ = 500` d'un cube de test, et les
formules qui s'y rapportent — jamais la valeur à laquelle elles se rapportent. Elle a été fixée par
le premier fichier qui a dû calculer une poussée, à la valeur qui faisait se refermer le cas de test.

C'est le mécanisme qui compte, pas la constante : **une décision jamais posée n'est pas une décision
en attente, c'est une décision qui sera prise par accident**, et par le code le plus tardif, à
l'endroit le moins visible, sur le critère le plus local. Elle ne ressemblera alors pas à un choix, et
personne ne pensera à la rouvrir.

**Réflexe** : quand une référence numérique se referme sur une valeur, vérifier que **chaque**
constante qu'elle emploie est écrite quelque part dans le corpus. Celles qui manquent sont à poser
explicitement, avec leur alternative et la conséquence chiffrée du choix — ici 2,5 % sur tout tirant
d'eau, contre une tolérance de 1 %.

## L70 — Une erreur de test qui révèle une propriété mérite que cette propriété devienne un cas

*(S21)* Un contrôle d'homogénéité échantillonnait par erreur à 5 000 m, au-delà du rayon de
référentiel de 4096 m. Le test échouait. Mais **le code avait raison** : `eval` refusait le point,
conformément à I-08, et c'est le test qui en faisait un NaN.

La réaction naturelle est de corriger la distance et de passer. C'est une perte : l'erreur venait de
révéler qu'une propriété du système — le refus hors référentiel — n'était couverte par **aucun** cas,
et personne ne l'aurait cherchée. Le bogue de test avait fait gratuitement le travail d'une revue.

**Réflexe** : après avoir corrigé un test fautif, se demander *pourquoi il a échoué plutôt que de
mal passer*. Quand la réponse est « parce que le code a correctement refusé », il y a là une
propriété active que rien ne teste : lui donner son propre cas, avant de corriger l'original.

## L71 — Quand un cas porte plusieurs assertions, l'une d'elles travaille et le nom du cas désigne souvent l'autre

*(S22)* C01 s'intitule « repos hydrostatique » et son symptôme annoncé est « les courants
parasites ». Il porte deux seuils : `max|u| < 1 mm/s` et `max|η − η₀| < 1 mm`. Le schéma mis à
l'épreuve **passe le premier** (0,53 mm/s) et **échoue le second** (21,6 mm pour 1 mm).

La raison est physique et se généralise : un déséquilibre **stationnaire** installe un écart
permanent avec un courant presque nul — l'eau s'est déplacée une fois, puis s'est arrêtée dans sa
mauvaise position. Le nom du cas, lui, décrit le régime **transitoire**, celui qu'on imagine en
lisant l'énoncé.

**Réflexe** : exécuter et rapporter **toutes** les assertions d'un cas, même celles qui paraissent
impliquées par une autre, et se méfier particulièrement de celle que le titre met en avant — c'est
statistiquement celle qui a été choisie pour l'intuition, pas pour son pouvoir de discrimination.

## L72 — Une propriété exacte qui donne un résultat faux localise l'erreur hors de son périmètre

*(S22)* La reconstruction hydrostatique préserve l'eau au repos par **identité algébrique** : ce
n'est pas une bonne approximation, c'est une égalité. Le solveur qui l'implémentait perdait pourtant
1,1 % de son volume en 60 s.

Il n'y avait donc rien à chercher dans l'opérateur intérieur — la démonstration ne laissait pas de
place au doute. L'erreur était nécessairement dans ce que la démonstration **ne couvrait pas** : les
conditions aux limites, qui recopiaient la hauteur d'eau au lieu de la surface libre.

C'est le même mécanisme qu'en S21, où l'identité `u = ω·η` a fait tomber la vitesse orbitale (L67) :
**une identité fermée ne sert pas seulement à valider, elle localise**. Sa valeur de diagnostic est
proportionnelle à la rigueur avec laquelle on sait ce qu'elle couvre.

**Réflexe** : devant un résultat faux là où une propriété est démontrée, ne pas relire la
démonstration — énumérer ce qu'elle **exclut**. Bords, cas dégénérés, initialisation, transitions.

## L73 — Un critère qualitatif devient un critère d'élimination dès qu'on mesure son ordre

*(S22)* « Ce solveur est mal équilibré » est une remarque de méthode : elle invite à préférer, pas à
exclure. Deux mesures la transforment en décision.

La première est l'**ordre** du défaut en `dx` — ici 1, exact, constaté sur cinq grilles. La seconde
est le **coût du raffinement** qui l'amènerait sous le seuil : `dx` divisé par 21,9, donc `×10 500`
en 2D une fois comptés les cellules (`dx⁻²`) et les pas de temps (`dx⁻¹`).

Un facteur dix mille n'est plus une préférence. Il autorise à **éliminer un candidat avant de le
mesurer**, ce qu'aucun jugement qualitatif ne permet — et il rend l'élimination défendable devant
quelqu'un qui ne partage pas l'intuition de départ.

**Réflexe** : devant un défaut qu'on s'apprête à qualifier de « connu, on fera avec », mesurer son
ordre, en déduire le raffinement requis, et l'élever au cube. Le résultat range le défaut dans l'une
des deux seules catégories utiles : réglage, ou disqualification.

## L74 — Une réconciliation qui n'emprunte pas l'outil de la divergence la masque au lieu de la fermer

*(S22)* Le fork S08-S15 a été « fusionné » en S16 par **recopie de documents** d'une ligne vers
l'autre. Le contenu a bien été réuni, un registre l'a acté, et le dépôt avait l'air complet. Mais
l'historique git est resté divergent : la ligne source n'a rien reçu et a continué seule **quatre
sessions de plus**, produisant trois ADR et tout le code.

Le fork était donc doublement dangereux : ouvert *et* réputé clos. Une divergence signalée fait
l'objet de vérifications ; une divergence déclarée résolue n'en fait plus l'objet d'aucune.

**Réflexe** : une réconciliation se fait dans le système qui a créé la divergence, ou elle ne se fait
pas. Si la fusion de contenu est le seul moyen disponible, alors **le registre doit dire que
l'historique reste divergent** — et la ligne source doit être marquée, dans son propre jeton, comme
abandonnée au profit de l'autre. Sans quoi la prochaine session lira « fusion close » et croira.

## L75 — Une explication correcte n'est pas une cause tant que son effet n'a pas été mesuré séparément

*(S23)* Le front trop lent sur lit sec a une explication classique et juste : au contact d'une
cellule sèche, l'onde de tête est l'invariant de Riemann `u + 2c` et non `u ± c`, et l'estimer trop
bas borne la vitesse de propagation numérique sous la vitesse physique du front. Le mécanisme est
réellement présent dans le code. Corriger l'estimation a déplacé le résultat de **0,15 point sur
seize**.

Le danger n'est pas de se tromper — l'explication n'est pas fausse. Le danger est qu'une explication
plausible **arrête la recherche**. Écrite sans mesure avant/après, elle serait entrée dans un ADR
comme *la* cause, avec une correction à l'appui et une justification théorique impeccable ; le vrai
défaut serait resté, et l'aurait fait passer pour résolu.

**Réflexe** : mesurer l'effet d'une correction **isolément**, avant de la présenter comme la cause.
Deux exécutions, un chiffre. Le coût est nul et il tranche entre « j'ai compris » et « j'ai une
histoire cohérente ».

## L76 — Une erreur globale faible peut masquer une erreur locale d'un ordre de grandeur

*(S23)* Sur C04, l'erreur L1 du solveur vaut **0,84 %** sur les huit cents cellules du domaine, et
son erreur de position de front **16,24 %**. Un facteur vingt.

La norme globale est la mesure la plus robuste, la plus facile à défendre et celle qui vient en
premier — et c'est exactement ce qui la rend dangereuse : elle **moyenne le défaut sur le domaine où
il n'est pas**. Un front occupe quelques cellules sur huit cents ; son erreur, si grande soit-elle,
disparaît dans une moyenne.

Le corollaire est ce qui rend la leçon utile hors du numérique : **la grandeur qui décide de ce qu'on
voit n'est presque jamais une moyenne.** Une vague qui monte sur une plage *est* un front de
mouillage ; un joueur ne regarde pas la norme L1 du champ.

**Réflexe** : pour toute validation par norme globale, se demander quelle est la **structure fine**
qui porte le phénomène, et lui donner sa propre mesure. Fronts, discontinuités, extrema, bords.

## L77 — Une grandeur mesurée à seuil ne se compare qu'à une référence prise au même seuil

*(S23)* La position d'un front n'existe pas indépendamment d'une convention : la solution tend vers
zéro continûment, et « où l'eau commence » est un choix, pas un fait. Comparer un front mesuré au
seuil `ε` à la position mathématique où `h = 0` mesure donc **la convention avant le solveur** : à
`ε = 1 cm`, l'écart de définition vaut 15 %, cinq fois la tolérance du cas.

La règle vaut pour toute grandeur définie par franchissement : durée d'un impact, largeur d'une
zone, portée d'une onde, temps de montée. Elle est simple et systématiquement oubliée parce que la
référence analytique, elle, s'écrit sans seuil — et paraît donc plus pure.

**Et le seuil peut renverser un verdict, pas seulement le déplacer** : le même solveur, sur la même
grille, échoue de trois fois la tolérance à `ε = 10⁻⁴` et la manque d'un point à `ε = 10⁻²`. Quand
c'est le cas, le seuil doit être **dérivé** du phénomène — le point au-dessus duquel le profil n'est
plus dominé par sa queue — et non conventionné.

**Réflexe** : écrire la référence **avec** le seuil dedans, et conserver l'écart à la version sans
seuil comme témoin. Si les deux verdicts diffèrent, c'est la définition qu'il faut d'abord régler.

## L78 — « Sans provenance » et « sans effet » sont deux dettes différentes, et la seconde n'en est pas une

*(S23)* `H_SEC` avait été relevé en S22 comme un nombre posé au jugé, à justifier au titre d'I-14.
La réponse attendue était une justification physique. La bonne réponse était un balayage : six
décades, **0,25 point d'effet** sur la grandeur la plus sensible du corpus.

`H_SEC` n'est donc pas un paramètre physique et n'a pas à en recevoir la justification — c'est un
garde-fou contre une division par zéro, libre sur au moins six ordres de grandeur. **Sa provenance
est cette mesure.** À comparer à `ρ_eau` (A103), autre constante relevée comme non fixée, qui
déplace 2,5 % de tout tirant d'eau du projet : la même dette apparente, deux natures opposées.

Ce que cela dit d'I-14 : l'invariant n'exige pas qu'un nombre soit *justifié*, il exige qu'on sache
**ce qu'il commande**. « Rien » est une réponse valide, et c'est même la plus économique — elle
ferme la question définitivement, là où une justification physique reste discutable.

**Réflexe** : devant une constante sans provenance, ne pas chercher d'abord sa justification —
**balayer sa valeur sur plusieurs décades et mesurer**. Le résultat range la constante en deux
catégories : celles qui commandent quelque chose, qui méritent un arbitrage, et les autres, qui
méritent une ligne de documentation.

## L79 — Une propriété qu'on attribue à un objet appartient souvent au couple (objet, épreuve)

*(S24)* « L'ordre de convergence du solveur » n'existe pas. Le même solveur, sans une ligne de
changement, donne `p ≈ 0,98` sur une solution régulière, `0,73` sur la norme globale de C04, et
`0,24` sur la position du front de ce même C04. Ces trois nombres ne se contredisent pas : ils
mesurent trois choses.

La formulation « ordre du solveur » est ce qui rend l'erreur invisible — elle nomme une propriété
d'un objet là où il y a une propriété d'un couple, et une assertion absolue (`p > 0,8`) devient
alors naturelle à écrire.

**Le test est linguistique et il est rapide** : quand une propriété est énoncée sans mentionner
l'épreuve qui la révèle, demander si elle changerait avec une autre épreuve. Si oui, l'épreuve fait
partie de la propriété et doit être nommée avec elle. Vaut pour « ce solveur est stable », « ce
format est compact », « cette approche est rapide » — et pour tout seuil du corpus.

## L80 — Améliorer la référence peut dégrader la mesure

*(S24)* Pour mesurer un ordre de convergence sans solution analytique, on prend pour référence une
grille très fine — l'oracle. Le réflexe est de la vouloir la plus fine possible. **Affiner l'oracle
a dégradé le résultat** : de 12 800 à 25 600 cellules, l'ordre observé est passé de 1,09 à **1,56**,
pour un schéma d'ordre 1 où plus de 1 est impossible.

La raison est que l'oracle porte sa propre erreur. Tant que l'erreur mesurée la domine largement, on
mesure le solveur ; quand elles se rapprochent, les deux se soustraient et le résultat n'a plus de
sens — et il n'a pas l'air d'en manquer.

Conséquence contre-intuitive : **avec un oracle, le triplet de grilles le plus fin est le moins
fiable**, l'inverse exact de ce qui vaut avec une solution analytique. La règle de sélection dépend
de la **nature de la référence**, pas de la finesse.

**Réflexe** : devant toute mesure relative à une référence imparfaite, écrire l'erreur de la
référence elle-même, et écarter les points où la mesure n'en est pas séparée par au moins un ordre
de grandeur. Et se méfier du résultat qui *s'améliore* quand on affine : un ordre qui monte au-delà
de ce que la théorie permet ne dit pas que le solveur est meilleur, il dit qu'on a cessé de le
mesurer.

## L81 — Un critère d'écart local ne distingue pas « a convergé » de « progresse lentement »

*(S24)* Le contrôle d'asymptoticité comparait les deux derniers ordres observés : si l'écart était
petit, la suite était déclarée stabilisée. Il a déclaré stabilisée la suite
**0,595 → 0,686 → 0,732**, dont les écarts valent 0,091 puis 0,046 — petits, et **tous de même
signe**. Une suite qui monte régulièrement n'a pas convergé : son dernier terme n'est pas sa limite.

Ce qu'il faut regarder n'est pas la **taille** des écarts mais la **structure** de leur suite :
changent-ils de signe, ou s'éteignent-ils assez vite pour que le reste soit borné ? Le remède écrit
ici est dérivé plutôt que conventionnel — si les écarts décroissent d'un facteur ≥ 4, la somme des
écarts restants est majorée par `|d₁|/3` — ce qui évite d'ajouter un second seuil arbitraire au
premier (A106).

**Réflexe** : tout critère de la forme « la valeur ne bouge presque plus » doit regarder au moins
trois points et le signe des variations. Vaut pour les seuils qui se stabilisent, les budgets qui se
tassent, les métriques qui se répètent — partout où l'on décide qu'une chose a fini de changer.

## L82 — Un dispositif de mesure doit distinguer *indisponible* de *vide*

*(S24)* L'allocation de l'oracle échouait — arène pleine — et l'erreur était absorbée par un
`Err(_) => continue`. Le rapport affichait alors « 0 grille retenue sur 0 » : un résultat vide, qui
se lit comme un cas ayant tourné et n'ayant rien trouvé, et non comme un cas n'ayant pas tourné.

Le coût de cette confusion est asymétrique. Un cas *indisponible* appelle une correction immédiate ;
un cas *vide* appelle une interprétation, et se range dans « pas de signal ». **La panne se déguise
alors en information**, et d'autant mieux que le reste du rapport est vert.

**Réflexe** : dans un harnais, tout `Err` absorbé est un mensonge en puissance. Trois états au
minimum — mesuré, sans signal, indisponible — et le troisième doit dire sa cause. Le corollaire vaut
pour tout tableau de bord : une case vide et une case non alimentée ne se ressemblent que sur écran.

## L83 — Une recommandation transmise gagne en autorité à chaque recopie, sans jamais être réexaminée

*(S25)* Trois sessions consécutives — S22, S23, S24 — ont recommandé « C03, avec la friction de
fond », et une action au registre existait pour ça. C'était l'inverse de ce qu'il fallait faire :
C03 mesure la dissipation **numérique**, une friction **physique** en ajoute une seconde, et la
mesure ne dit alors plus laquelle des deux éteint la vague.

Le raccourci vient d'un mot employé pour deux choses. Il s'est fait tout seul en S22, et les deux
sessions suivantes l'ont recopié — **moi compris, trois fois** — parce qu'une recommandation déjà
écrite se lit comme un acquis, pas comme une hypothèse.

Le pire est qu'elle n'aurait rien fait échouer. Avec la friction, la demi-vie aurait simplement été
plus courte, et je l'aurais attribuée au schéma. **Une erreur de protocole ne se signale pas
elle-même** : elle produit un résultat plausible.

**Réflexe** : quand on s'apprête à exécuter une recommandation héritée, relire l'**énoncé** de ce
qu'on va mesurer avant de relire la recommandation. Ici, deux phrases de C03 suffisaient.

## L84 — Un cas de validation peut passer parce qu'il n'est pas dans le régime où le système vivra

*(S25)* C03 offre **400 points par longueur d'onde**. Le solveur y tient 20,7 périodes de demi-vie
pour 15 exigées : le cas passe, et un lecteur en conclut que le solveur est bon. À 20 points par
longueur d'onde — l'ordre de grandeur d'un domaine réel — la demi-vie vaut **1,3 période** : l'eau
meurt en une oscillation.

Le cas ne ment pas ; il mesure ce qu'il mesure. Mais **son verdict dépend d'un paramètre que son
énoncé ne mentionne pas**, et ce paramètre est justement celui qui sépare le montage du réel.

C'est la deuxième occurrence dans le même corpus : C01 est presque équilibré *par accident de
géométrie* sur son fond à pente constante (A105). **Le point commun est que ces montages ont été
choisis pour être lisibles** — ce qui est une qualité — et que la représentativité n'a jamais été un
critère explicite de leur écriture.

**Réflexe** : pour tout cas de validation, écrire explicitement le régime qu'il place sous test —
résolution, amplitude, rapport d'échelles — et le comparer au régime visé. L'écart est soit
justifié, soit un angle mort.

## L85 — Quand des grandeurs disparaissent d'une formule, ce qui reste est ce qu'il faut mesurer

*(S25)* L'amortissement d'une onde par le schéma s'écrivait avec la célérité, la longueur d'onde, la
période, le pas d'espace et le nombre de Courant. Après substitution :

> `demi-vie (périodes) = ln 2 · N / (2π²(1−ν))`

**`c`, `λ` et `T` ont disparu tous les trois.** Il ne reste que le nombre de points par longueur
d'onde et le nombre de Courant.

Ce n'est pas une simplification cosmétique : c'est la découverte de la **bonne variable**. Tant que
la formule portait `λ` et `c`, la dissipation semblait dépendre du contenu de la mer, donc devoir
être mesurée cas par cas. Une fois `N` isolé, une seule mesure vaut pour toutes les ondes — et la
question « quelle résolution faut-il ? » devient une division.

**Réflexe** : quand une expression se simplifie plus qu'attendu, ne pas passer à la suite. Regarder
**ce qui reste** : c'est la variable dont le système dépend réellement, et c'est celle qu'il faut
instrumenter, budgéter et exiger. Le corollaire pratique : exprimer les exigences dans ces
variables-là — ici « survivre N périodes » plutôt que « survivre 40 secondes ».

## L86 — Une conclusion correcte peut ne pas épuiser la question qu'elle ferme

*(S25)* S22 avait établi que `λ_cut` ne sortirait pas du véhicule Saint-Venant, faute de dispersion.
C'est exact, c'était bien argumenté, et cela a fermé la question pour trois sessions.

Mais `λ_cut` borne « la plus petite longueur d'onde transportée correctement », et une onde peut
être mal transportée de **deux façons indépendantes** : arriver au mauvais moment (dispersion), ou
**ne pas arriver** (dissipation). La seconde était mesurable depuis le début, sur le véhicule
existant, sans rien ajouter.

Ce type d'angle mort est le plus durable qui soit. Une question laissée ouverte attire l'attention ;
**une question fermée par une réponse correcte ne se rouvre plus** — et la qualité de la réponse est
précisément ce qui la protège de l'examen.

**Réflexe** : devant une conclusion qui bloque une question, vérifier qu'elle couvre toute la
**définition** de ce qui était demandé, et pas seulement le chemin qu'on avait choisi pour y
répondre. Relire la définition d'origine, pas le raisonnement qui vient d'être fait.

## L87 — Une loi n'est testée que par une prédiction qui n'a pas servi à l'établir

*(S26)* La loi de dissipation de S25 avait été vérifiée à 0,2 % — sur les deux balayages qui avaient
servi à la construire. C'est un accord d'ajustement, pas une validation : une formule à un paramètre
retrouve toujours les données dont on a tiré ce paramètre.

Elle faisait pourtant une prédiction qu'aucune de ces mesures ne contenait. Le mode `n` d'un bassin
clos a moins de points par longueur d'onde **et** une période plus courte ; les deux effets se
composent en un **`n²`** qui ne se lit pas dans la formule. Mesuré : rapports 3,99 · 8,85 · 15,59
pour 4 · 9 · 16.

**Réflexe** : après avoir établi une relation, chercher ce qu'elle implique **ailleurs que là où on
l'a mesurée** — un autre régime, une autre variable, un exposant composé. Si rien ne vient, la
relation est une interpolation et doit être présentée comme telle. Le bon signe est qu'on puisse se
tromper : une prédiction qui ne peut pas échouer ne teste rien.

## L88 — Un biais qui varie à l'envers de ce que la théorie prédit désigne l'instrument

*(S26)* Les quatre mesures d'harmoniques présentaient un écart systématique de −2 à −5 % : le
solveur dissipait un peu plus que la loi. Un écart de troncature aurait grandi avec `n`, puisque les
modes courts sont moins bien résolus. **Il décroissait.**

Le sens de variation, et non sa taille, désignait la cause : à `n = 1` la demi-vie vaut 48 périodes
et la fenêtre d'observation 20 — l'ajustement exponentiel ne voyait qu'un quart de la décroissance.
Le biais venait de la **mesure**, pas du solveur, et le même mécanisme avait faussé une restitution
de `Hs` de 8,5 % en S21 (A102).

**Réflexe** : devant un biais systématique, regarder d'abord **comment il varie**, pas combien il
vaut. Un biais qui suit la théorie est physique ; un biais qui va à contresens est instrumental — et
il désigne alors la fenêtre, la résolution de mesure ou le protocole, jamais l'objet mesuré.

## L89 — Une frontière et un filtre ne se dimensionnent pas de la même façon

*(S26)* Le corpus parlait d'une « frontière W/δ » et d'une longueur d'onde de coupure `λ_cut` : une
grandeur unique, au-dessus de laquelle l'onde passe et en dessous de laquelle elle est perdue. La
mesure dit autre chose. L'amortissement varie **continûment**, et la composante deux fois plus courte
ne disparaît pas — elle vit **quatre fois moins longtemps**.

La différence n'est pas terminologique. Une frontière se dimensionne par un seuil, qu'on peut poser
une fois. Un filtre se dimensionne par une **exigence de survie** — « telle composante doit tenir
`X` périodes » — dont découle la résolution. Le seuil précède le calcul ; l'exigence en découle.

Et la conséquence pratique est un facteur trente : le budget se pose sur la composante la plus
**courte** qu'on veut conserver, jamais sur la **dominante**, qui est pourtant celle qui vient à
l'esprit et celle qui donne son nom à l'état de mer.

**Réflexe** : quand une grandeur est décrite comme un seuil, vérifier si le phénomène sous-jacent
est vraiment discontinu. S'il est continu, le seuil cache une loi — et cette loi porte des arbitrages
que le seuil rendait invisibles.

## L90 — Le code n'est pas un lieu de publication

*(S26)* Le cas régulier a été écrit en S24 dans `Bassin::c08_regulier`, exécuté, et mentionné dans
l'ADR de la session. Il n'était pas dans `CAS-CANONIQUES`. Une session ouvrant le corpus de
validation — le lieu prévu pour ça — aurait conclu qu'aucun cas régulier n'existait, et l'aurait
réécrit.

Le code est lu par qui travaille **dessus**, pas par qui cherche **ce qui existe**. Un artefact n'est
publié que là où on le cherchera sans savoir qu'il est là. Un ADR ne suffit pas non plus : il est
daté, il raconte une décision, il n'est pas l'index de ce qui est disponible.

**Réflexe** : après avoir créé quelque chose de réutilisable, se demander *par quel chemin la
prochaine session tombera dessus si elle ne lit pas ce que je viens d'écrire*. Si la réponse est
« elle ne tombera pas dessus », l'inscrire dans le document qui fait office d'index — et l'y
inscrire dans la même session, pas dans les actions ouvertes.

## L91 — Un corpus fermé sur lui-même ne produit que les questions qu'il sait déjà poser

*(S27)* Le défaut central de cette session — `u_max` non défini alors qu'une SPEC impose des parois
mobiles — était **entièrement contenu dans le corpus depuis S04**. Les deux moitiés y étaient,
SPEC-001 §2.1 et SPEC-004 §10.1. Une revue croisée systématique avait même confronté cette paire, et
ne l'avait pas vu : son rapprochement portait sur le coût, et **une variable laissée sans définition
ne déclenche aucune contradiction visible** — il n'y a rien à comparer.

Ce qui a manqué pendant sept sessions n'était pas l'information, c'était la **question**. Elle est
venue d'un projet voisin, d'architecture différente, qui avait rencontré le défaut et l'avait
mesuré.

**Réflexe** : traiter les **mesures** d'un travail extérieur comme des faits, et ses **conclusions**
comme ne nous engageant pas. Un projet qui a choisi une autre architecture ne dit rien de la nôtre,
mais ce qu'il a *cassé* est un fait sur le monde. Et le corollaire est plus large : une relecture
interne, si systématique soit-elle, ne trouve pas les angles morts qui viennent d'une question
jamais posée — il faut une source qui pose les questions autrement.

## L92 — Une marge par défaut peut protéger d'un défaut que personne n'a identifié

*(S27)* `CFL = 0,45` a été posé sans justification écrite. La mesure a montré que le schéma est
stable jusqu'à 0,99 **et juste** — l'erreur de période y reste soixante fois sous la tolérance — et
que monter la valeur achèterait ×20 de portée d'onde et un pas de temps double.

**Rien ne s'opposait à la monter.** Sauf que la marge de Courant est une marge sur `u_max` : à
`ν = 0,45` elle absorbe une sous-estimation de ×2,22, et le défaut réel, mesuré ailleurs, valait
×2,2 à ×2,5. La valeur protégeait d'un trou dont elle ignorait l'existence.

Le danger est asymétrique : une marge qu'on retire ne fait rien échouer **tout de suite**. Le coût
apparaît quand le défaut qu'elle couvrait se manifeste, et il est alors attribué à autre chose.

**Réflexe** : avant de resserrer un paramètre de sécurité au motif que « rien n'échoue », chercher
**contre quoi il protégeait**. Si la réponse n'est écrite nulle part, c'est une raison de plus de
ne pas y toucher — pas une raison de moins.

## L93 — L'ordre des questions vaut souvent plus que leur réponse

*(S27)* L'action ouverte demandait « poser le nombre de Courant comme paramètre de conception », et
son motif était le gain de performance. Répondre à cette question-là aurait donné une valeur — et
une valeur posée sur une définition fausse est plus dangereuse qu'une valeur prudente posée sur la
bonne.

Le bon ordre s'est révélé être : **définition** de la grandeur bornée, puis **règle de calcul** de
la borne, puis **valeur**. Chacun des deux premiers termes change le troisième ; l'inverse n'est pas
vrai. Une session qui aurait livré « `ν = 0,9` » aurait clos l'action en laissant le vrai problème
intact, et en supprimant la marge qui le masquait.

**Réflexe** : devant une question qui demande un nombre, vérifier d'abord que la grandeur à laquelle
il s'applique est **définie**, et que la règle qui l'emploie est **écrite**. Si l'un des deux
manque, le nombre n'est pas la réponse — c'est le troisième terme d'une suite dont les deux premiers
sont vides.

## L94 — Un contrôle bien conçu trouve autre chose que ce qu'il cherchait

*(S27)* Le contrôle visait un confondant précis : le balayage de S25 faisait varier `N` et `a/dx`
ensemble, et un piège documenté ailleurs disait ce que cela coûte. **Ce confondant-là n'existait
pas** — l'amplitude relative était fixe, et `a/dx` n'a pas de sens physique propre.

Mais en séparant les variables, le contrôle a exhibé une **dépendance en amplitude** que personne
n'avait cherchée : la loi de dissipation tient à `a/h = 1 %` et est fausse à 5 %. La conclusion de
S25 reste vraie ; c'est son **domaine de validité** qui manquait, et il exclut une part des cas
réels.

Le contrôle n'a donc pas confirmé ni infirmé ce qu'il visait : il a rendu une propriété que la
question d'origine ne mentionnait pas.

**Réflexe** : ne pas juger un contrôle à sa capacité de confirmer le soupçon qui l'a motivé. Un
contrôle qui sépare proprement des variables produit de l'information sur **chacune** — y compris
celles dont on ne se méfiait pas. Le corollaire : quand un contrôle « ne trouve rien », vérifier
qu'on a bien regardé toutes les sorties, et pas seulement celle qu'on attendait.

## L95 — La forme d'une assertion décide de ce qu'un cas peut voir, et « ça marche encore » ne voit rien

*(S28)* C23 mesure une borne de pas de temps fausse : sous la mauvaise définition, `u_max` est
sous-estimé jusqu'à **×5,5**, et le nombre de Courant réellement réalisé atteint **2,48** — plus du
double de la condition de stabilité.

**Et le solveur ne casse pas.** Le schéma est diffusif, il absorbe le dépassement, l'état reste fini
et le volume conservé. Un cas dont l'assertion aurait été « le solveur diverge » serait donc
**passé**, et aurait certifié l'absence d'un défaut parfaitement présent.

Ce qui est perdu au-delà d'une condition de stabilité n'est pas la simulation : c'est la
**garantie**. Le solveur tient jusqu'à ce qu'il ne tienne plus, sur un cas que rien n'a testé — et
le jour où il cassera, ce sera attribué à autre chose.

**Réflexe** : assertir sur la **grandeur** que la propriété gouverne, jamais sur ses conséquences
visibles. Ici, le nombre de Courant réalisé — pas la présence d'un `NaN`. Le corollaire est général :
« aucune erreur observée » n'est une mesure de rien, et une assertion qui ne peut échouer que sur une
catastrophe ne détecte que les catastrophes.

## L96 — Deux décisions sûres séparément peuvent se renforcer en un défaut

*(S28)* Sous une définition d'`u_max` qui ignore les parois mobiles, la vitesse de paroi qui fait
franchir `C = 1` vaut 5,41 m/s à `ν = 0,45`, **1,90 m/s à `ν = 0,70`**, et 0,49 m/s à `ν = 0,90`.

Chacune des deux décisions se défend seule. Monter `ν` est une optimisation légitime, mesurée, qui
achète de la portée d'onde et du pas de temps. Laisser `u_max` sans définition est un oubli qui n'a
rien cassé pendant vingt-sept sessions. **Ensemble, elles rendent atteignable un trou qui ne l'était
pas** — et c'est l'optimisation qui déplace le seuil, pas l'oubli.

Le piège est que le lien ne se voit dans aucune des deux revues : celle du paramètre conclut « rien
n'échoue », celle de la définition conclut « aucun cas ne l'exerce ».

**Réflexe** : quand une optimisation resserre une marge, chercher **ce que cette marge couvrait
d'autre**. Une marge a rarement une seule fonction, et les fonctions non écrites sont celles qui
disparaissent en silence.

## L97 — Un instrument qui change de réglage cesse d'être comparable à lui-même

*(S28)* `ν = 0,70` a été débloqué pour le solveur du projet. Le réflexe suivant est de le poser aussi
dans le véhicule d'essai — c'est la même grandeur, et la nouvelle valeur est meilleure.

**Ce serait une erreur.** Le véhicule sert à *mesurer* : demi-vies de C03, position du front de C04,
ordres de convergence de C08. Toutes ces références ont été publiées à `ν = 0,45`. Changer la
constante les déplacerait d'un coup, sans qu'aucune mesure y gagne — et les comparaisons entre
sessions deviendraient fausses sans que rien ne le signale.

La distinction à tenir est entre une **décision de conception**, qui porte sur le système à
construire, et un **réglage d'instrument**, qui porte sur ce avec quoi on l'observe. Les deux peuvent
concerner le même paramètre et n'ont pas à prendre la même valeur.

**Réflexe** : avant de propager une valeur nouvellement décidée dans le harnais, se demander quelles
mesures publiées en dépendent. Si la réponse n'est pas « aucune », la valeur reste dans la décision
et l'instrument garde la sienne — en disant pourquoi, sans quoi la prochaine session le prendra pour
un oubli.

## L98 — Une assertion peut être verte parce que le mécanisme qu'elle teste n'existe pas encore

*(S29)* « Aucune plaque de glace ne se forme tant que `Hs > 0,15 m` » est une assertion recevable :
elle porte sur une grandeur mesurable, l'épaisseur, et elle a un seuil. **Elle passe pourtant depuis
le jour où elle a été écrite, et passera jusqu'à ce que le modèle de glace existe** — parce qu'aucune
plaque ne se forme quand rien ne fabrique de plaques.

C'est distinct du symptôme : l'assertion n'est pas mal formée, elle est **vide**. Et une batterie
verte dont une part est vide donne exactement la confiance qu'elle ne mérite pas.

Le remède est le **témoin** : un montage qui doit faire échouer l'assertion. S'il ne la fait pas
échouer, le cas ne teste rien. Ce dépôt en employait déjà un — un schéma volontairement fautif,
conservé, dont le harnais signale comme *anomalie* le jour où il cesserait d'échouer — sans avoir vu
que le dispositif répondait à un problème général.

**Réflexe** : pour toute assertion, se demander *« qu'est-ce qui la ferait échouer aujourd'hui ? »*.
Si la réponse est « rien, parce que la chose testée n'existe pas », il faut un témoin, ou l'assertion
est un décor.

## L99 — Une garde de sécurité posée sur un instrument l'empêche de mesurer

*(S29)* Un balayage devait observer un facteur d'amplification supérieur à 1, en demandant un nombre
de Courant de 1,05. La fonction de réglage bornait à `[0,05 ; 0,99]`, par prudence. **La mesure a
rendu le résultat de 0,99 comme s'il était celui de 1,05**, sans rien signaler.

Le bornage était raisonnable là où il avait été écrit — il protège d'un pas de temps nul. Sur un
**instrument de mesure**, il devient un aveuglement : l'appareil ne peut plus atteindre le régime
qu'il est censé caractériser, et il renvoie une valeur plausible.

C'est la même faute qu'une assertion qui ne peut pas échouer, d'un cran plus haut : ce n'est pas le
test qui est incapable de voir, c'est le **montage** qui est incapable d'atteindre l'endroit où il y
aurait quelque chose à voir.

**Réflexe** : avant tout balayage, vérifier que les bornes du montage permettent d'**atteindre le
régime où l'assertion échoue**. Et faire dire à un réglage bridé qu'il a bridé — un `clamp`
silencieux dans un instrument transforme une question en réponse.

## L100 — Une mesure vide peut porter une conclusion sans la rendre fausse, et c'est le cas le plus dur à voir

*(S29)* Une mesure de stabilité classait les exécutions en « stable / diverge / non fini » et
répondait « stable partout ». Elle ne pouvait échouer que sur une catastrophe ; elle n'a rien prouvé.
Un ADR en a pourtant tiré une ligne.

**Et la conclusion de cet ADR tient quand même** — parce qu'une seconde mesure, continue celle-là,
la portait réellement. La ligne fautive était décorative.

C'est la configuration la plus difficile à détecter : **rien n'est faux, donc rien n'alerte**. La
mesure vide reste dans le document, indiscernable des autres, et une session ultérieure la citera
comme un fait établi — d'autant plus volontiers qu'elle est simple et catégorique.

**Réflexe** : pour chaque conclusion publiée, nommer **laquelle** des mesures la porte, et vérifier
que celle-là est recevable. Les autres sont du contexte, et doivent être présentées comme tel. Une
conclusion adossée à trois mesures dont une seule est valide n'est pas trois fois étayée.

## L101 — Un critère se vérifie sur des cas dont on connaît la réponse, et c'est là qu'il se complète

*(S29)* Le critère de tri — *une assertion est recevable s'il existe une grandeur continue dont elle
est le seuil* — a été éprouvé sur trois cas connus avant d'être appliqué en série, par prudence : un
critère qui classe mal un cas évident classera mal les autres en silence.

Les deux premiers ont confirmé le critère. **Le troisième l'a fait éclater** : « zéro allocation »
est recevable au sens du critère, et pourtant satisfaite si rien ne tourne. Une classe entière
manquait, et elle ne se serait pas vue en série — elle aurait produit des « recevable » corrects et
inutiles.

L'épreuve préalable n'a donc pas servi à valider le critère : elle a servi à le **compléter**, et
c'est un usage plus rentable que celui pour lequel elle était prévue.

**Réflexe** : éprouver un critère de classement sur trois cas dont on connaît déjà le verdict, en
choisissant le troisième **le moins ressemblant aux deux autres**. Le coût est de quelques minutes ;
le gain est de ne pas produire un inventaire entier qui range mal.

## L102 — Un manque de seuils est souvent un manque de lecture

*(S30)* Cinq assertions du corpus étaient floues — « nettement supérieure », « sensiblement plus
longue », « aucun tremblement visible ». Le réflexe est d'y voir des seuils manquants, donc un
travail de calibration : choisir des nombres, les justifier, les marquer « à calibrer » à défaut.

**Aucun n'a eu à être inventé.** Chacune des cinq grandeurs était déjà dans le corpus, sous
l'assertion qui ne la nommait pas : un facteur de résonance dans un ADR, une masse ajoutée dans un
angle mort, une projection « exactement stable » dans une décision, un seuil de formation dans une
SPEC.

Le flou n'était pas une lacune de mesure. C'était **une lacune de rapprochement** : la personne qui
écrivait le cas et celle qui écrivait la formule n'étaient pas au même endroit du corpus, et rien ne
les reliait.

**Réflexe** : devant une assertion floue, ne pas commencer par chercher un seuil. Chercher d'abord
**la grandeur**, puis **où le corpus en parle**. La calibration est le dernier recours, pas le
premier geste — et un seuil inventé ferme la question que la lecture aurait ouverte.

## L103 — Un énoncé flou déplace l'exigence vers le bas, jamais vers le haut

*(S30)* « Aucun tremblement **visible** » tolère tout écart sous le seuil de perception. La
conception, elle, garantissait davantage : l'objet est **projeté sur la surface**, `z = η`,
« exactement stable ». La référence est zéro.

L'assertion floue demandait donc **moins** que ce que le système promet — et un écart de `10⁻⁴ m`,
invisible mais révélateur d'une projection non appliquée, l'aurait passée.

Ce n'est pas un hasard de rédaction. Un flou se résout toujours dans le sens du permissif : un seuil
perceptuel, un « raisonnable », un « acceptable » sont plus larges qu'une identité, et celui qui
lira l'assertion plus tard choisira l'interprétation qui passe. **Le flou n'est pas neutre, il est
orienté.**

**Réflexe** : quand une assertion est floue, chercher ce que la conception **garantit** sur la même
grandeur. Il arrive que la garantie soit exacte, et alors l'assertion précise est plus simple *et*
plus forte que celle qu'on allait écrire.

## L104 — L'instrument qui mesure une assertion ne se lit pas dans son énoncé

*(S30)* Deux assertions de forme identique, dans le même tableau : « zéro allocation après
initialisation » et « aucune capacité dérivée n'est lue depuis un profil de qualité ». Un audit les
a séparées — la première recevable, la seconde manquée — puis s'est aperçu qu'elles avaient le même
défaut potentiel et pas le même sort.

Ce qui les distingue n'est **pas dans le texte** : la première a un compteur, lu par le harnais
depuis dix sessions ; la seconde demanderait une analyse statique qui n'existe pas. La première
mesure quelque chose, la seconde est un vœu.

Classer des assertions sur leur formulation revient donc à classer des promesses sur leur ton. Il
faut, pour chacune, **nommer ce qui la mesure** — et vérifier que cela existe aujourd'hui.

**Réflexe** : à côté de chaque assertion, écrire l'instrument. Trois questions, dans cet ordre :
*quelle grandeur ?* — *quel témoin la fait échouer ?* — *qu'est-ce qui la mesure, et est-ce que ça
existe ?* La troisième est celle qu'on oublie, parce que la réponse paraît évidente quand on vient
d'écrire la première.

## L105 — Deux défauts qui se protègent l'un l'autre ne se trouvent que dans l'ordre

*(S30)* Un cas assertait sur un régime — la résonance transcritique — que son propre montage
n'atteignait jamais : ses quatre vitesses sautaient le point critique. Le défaut était là depuis
l'écriture du cas.

Il n'a été visible qu'**après** avoir donné une grandeur à l'assertion. Tant qu'elle disait
« amplitude nettement supérieure », rien n'obligeait à calculer les nombres de Froude du montage :
une phrase qualitative se lit sans vérifier qu'elle est atteignable.

Les deux défauts se couvraient mutuellement. L'assertion floue dispensait d'examiner le montage ; le
montage incapable ne pouvait pas faire échouer une assertion qui ne mesurait rien.

**Réflexe** : quand une correction en révèle une seconde, noter l'**ordre** qui l'a rendue visible —
c'est lui qui se réutilise. Ici : *donner une grandeur, puis vérifier que le montage atteint le
régime où elle varie.* L'inverse n'aurait rien produit.

## L106 — Une question qualifiée de lourde se reporte d'autant plus qu'elle est qualifiée ainsi

*(S31)* Un angle mort a été désigné « la question la plus lourde ouverte » par quatre sessions
consécutives, chacune le recommandant à la suivante. Il s'est dissous en relisant deux documents
**antérieurs à sa formulation** : sa prémisse était fausse, et elle l'était depuis le début.

Le statut n'a pas seulement décrit la difficulté — **il l'a produite**. Une question étiquetée
lourde appelle une session entière, donc un moment où l'on aura du temps ; et comme on ne l'ouvre
pas, on ne découvre pas qu'elle est légère. Chaque report renforce l'étiquette, qui justifie le
report suivant.

**Réflexe** : avant de reporter une question difficile, lui consacrer **dix minutes** — le temps de
relire ce que le corpus dit déjà de sa **prémisse**, pas de sa réponse. C'est le geste le moins cher
du répertoire, et aucune des quatre sessions qui recommandaient celle-ci ne l'avait fait.

## L107 — Ce qu'un système dissipe le plus vite est souvent ce pour quoi il existe

*(S31)* Une couche de simulation a été introduite pour produire les perturbations locales — sillages,
impacts, remous. La loi de dissipation, établie sur des ondes longues, s'y applique à ce qu'elle
porte réellement : ces perturbations. Or elles sont **courtes**, donc mal résolues, donc les
premières effacées.

Le résultat est chiffré et contre-intuitif : la distance sur laquelle un sillage reste visible varie
comme la **puissance quatrième** de la vitesse de l'objet. Un facteur 3 en vitesse fait deux ordres
de grandeur. Les objets lents — les plus nombreux dans une scène — n'en produisent aucun.

La coïncidence n'est pas fortuite. Un système est dimensionné sur son cas nominal, et les phénomènes
qu'il **ajoute** sont par construction plus fins que ceux qu'il transporte déjà. **La finesse est ce
qui justifie la couche, et c'est ce que la discrétisation détruit en premier.**

**Réflexe** : après avoir mesuré une dissipation, une latence ou une perte sur un cas de référence,
la recalculer sur ce que le système **apporte de spécifique**. Les deux réponses diffèrent souvent
d'un ordre de grandeur, et c'est la seconde qui décide de l'intérêt du système.

## L108 — Un solveur peut détruire une propriété que son équation conserve exactement

*(S31)* L'équation résolue est non dispersive : une perturbation s'y scinde en deux trains qui se
propagent **sans déformation**. La forme est un invariant exact du modèle.

Mesuré sur le solveur : un paquet fin **quintuple sa largeur** en vingt secondes, en perdant 90 % de
son amplitude. **Tout cet étalement est numérique** — il n'a aucune contrepartie dans le modèle, et
rien n'en mesurait la quantité.

Le point n'est pas qu'un schéma soit imprécis : c'est que **les invariants exacts d'une équation
sont les meilleures références de validation qui existent**, et qu'ils sont faciles à ne pas voir.
La forme conservée ne figure dans aucune formule ; elle est une propriété de la solution, pas un
terme du système.

**Réflexe** : pour toute équation résolue, dresser la liste de ce qu'elle **conserve exactement** —
masse, énergie, forme, symétries, états d'équilibre. Chaque item est un cas de validation dont la
référence vaut `0` ou `1`, sans aucun seuil à inventer. C'est la famille dont C01 fait déjà partie.

## L109 — Deux documents peuvent se contredire sans qu'aucun ne soit faux, et personne ne le voit

*(S31)* Un ADR place le générateur de sillage dans une couche ; un autre, écrit plus tard, décrit une
dissipation qui ne s'applique qu'à une **autre** couche. Les deux sont corrects dans leur propre
cadre. Ensemble, ils laissent une question sans réponse : **où vit le sillage ?** — et de cette
réponse dépend l'existence même du problème que la session venait de chiffrer.

Ce type de contradiction ne se voit dans aucune revue croisée, parce que les deux textes ne parlent
pas du même objet : l'un parle d'un générateur, l'autre d'un amortissement. Il n'y a pas de phrase à
opposer à une autre. **Ce qui manque est un troisième énoncé, jamais écrit** — celui qui dirait à
quelle couche appartient le phénomène.

**Réflexe** : quand un résultat dépend de la couche à laquelle un phénomène appartient, vérifier que
cette appartenance est **écrite quelque part**. Si elle ne l'est qu'implicitement, dans deux
documents qui n'en parlent pas ensemble, elle n'est pas décidée — et le résultat qui en dépend est
suspendu à une décision que personne n'a prise.

## L110 — Un corpus qui grandit rend son propre socle moins consulté

*(S32)* Deux sessions consécutives ont ouvert une question qualifiée de grave, l'ont reportée, puis
l'ont dissoute en relisant **le même document** — le premier ADR du corpus, celui qui définit les
couches. La réponse y était depuis la première session. La seconde fois, c'est la session
**précédente** qui avait posé la question sans le consulter.

Le mécanisme est mécanique : trente-sept documents se lisent moins qu'un, et celui qu'on saute est
**celui qu'on croit connaître**. Un socle est relu au démarrage d'un projet, puis jamais — alors
qu'il est précisément l'endroit où les définitions vivent, et que les questions difficiles portent
presque toujours sur des définitions.

Le coût mesuré ici : deux angles morts de gravité 1 ouverts pour rien, quatre sessions de report, et
un document entier écrit sur un objet qui n'appartenait pas à la couche qu'il étudiait.

**Réflexe** : rattacher explicitement une **famille de questions** au document qui en décide, et
l'écrire dans le registre. Ici : *toute question sur l'appartenance d'un phénomène à une couche se
règle dans ADR-001 §2, et nulle part ailleurs.* Une règle d'aiguillage coûte une ligne et remplace
une relecture.

## L111 — Le même mécanisme peut réaliser une exigence et en trahir une autre

*(S32)* La dissipation numérique efface les perturbations avec le temps. Le document fondateur exige
par ailleurs que la couche concernée « tende vers 0 en s'éloignant de sa source ».

Pour un phénomène **entretenu** — une source qui réalimente en permanence — les deux se rejoignent :
la dissipation atténue avec le temps de trajet, ce qui produit exactement la décroissance **spatiale**
demandée, et à la bonne échelle. Pour un phénomène **transitoire**, rien ne réalimente : la même
dissipation le tue avant sa fin physique.

**Un défaut et une propriété voulue, produits par le même mécanisme, séparés par la seule présence
d'une source.** Traiter la dissipation comme un défaut uniforme — ce que faisait la session
précédente — conduisait à vouloir la corriger partout, donc à détruire la propriété.

**Réflexe** : avant de corriger un mécanisme jugé nuisible, énumérer **ce qu'il produit d'autre**.
S'il réalise une exigence ailleurs, ce n'est pas le mécanisme qu'il faut changer, c'est le périmètre
où il s'applique — et la partition qui en résulte est souvent la vraie décision de conception.

## L112 — Quand une grandeur disparaît d'un critère, le critère devient transportable

*(S32)* La condition « un transitoire doit survivre à sa durée physique » s'écrivait avec la durée
numérique, la durée de chute gravitaire, la célérité, la profondeur et la résolution. Après
résolution, l'accélération de la pesanteur **disparaît** :

> `dx ≤ K · L^1,5 / √(2h)`

Ce n'est pas une simplification cosmétique. Tant que `g` figurait dans la formule, le critère
paraissait attaché à un contexte gravitaire particulier. Une fois `g` éliminé, il ne reste que la
taille du phénomène, la profondeur et le nombre de Courant — **trois grandeurs qu'un concepteur
choisit**, là où `g` est subi.

C'est la deuxième fois dans ce projet qu'une élimination révèle la bonne variable ; la première
avait fait disparaître la longueur d'onde, la célérité et la période d'une loi de dissipation.

**Réflexe** : après avoir établi un critère, chercher activement ce qui s'y **annule**. Ce qui reste
est ce sur quoi on peut agir, et c'est la forme dans laquelle le critère doit être publié.

## L113 — La conclusion la plus rassurante est celle qu'il faut vérifier en premier

*(S32)* Une session a produit deux résultats : un défaut chiffré et brutal — les petites
perturbations meurent huit à vingt fois trop tôt — et une bonne nouvelle : la décroissance spatiale
exigée par la conception se produit toute seule, à la bonne échelle.

Le défaut est **mesuré**. La bonne nouvelle est **dérivée** : elle suppose qu'une source constante
et une dissipation exponentielle donnent une décroissance exponentielle en espace, ce qui est vrai
en régime linéaire — et le solveur ne l'est pas. Aucune mesure du corpus ne porte sur une source
entretenue ; toutes portent sur des perturbations relâchées.

**La conclusion la moins étayée est donc celle qui rassure**, et c'est un ordre naturel : on cherche
à confirmer ce qui inquiète, et on accepte ce qui soulage. Un raisonnement qui aboutit à « tout va
bien » sollicite moins de vérification que celui qui aboutit à « il faut trente fois plus de
cellules ».

**Réflexe** : dans un rapport qui contient une mauvaise et une bonne nouvelle, vérifier **l'étai de
la bonne** avant de publier. Et l'écrire : si elle est dérivée et non mesurée, le dire à l'endroit
même où elle rassure.

## L114 — Écrire qu'une conclusion est dérivée est ce qui la fera vérifier

*(S33)* Une session a produit une conclusion rassurante — un mécanisme indésirable produisait en fait
une propriété exigée par la conception — et l'a écrite en signalant qu'elle était **dérivée, jamais
mesurée**. La session suivante l'a mesurée : elle tient, à 0,3 % près, avec un `R²` de 1,0000.

**Sans cette mention, personne ne serait allé vérifier.** La conclusion rassurait, elle était
cohérente, elle occupait la même place typographique qu'un résultat — rien ne la distinguait d'un
fait, et c'est exactement ce qui la rendait dangereuse.

Le geste ne coûte rien : une phrase, à l'endroit même où la conclusion apparaît. Il ne s'agit pas de
se dédire par avance, mais de **rendre visible le statut** de ce qu'on affirme — dérivé, mesuré,
estimé, supposé.

**Réflexe** : dans tout document qui mêle mesures et déductions, marquer chaque conclusion par son
statut, à l'endroit où elle est énoncée et non dans une section « limites » qu'on ne lira pas. Une
conclusion dérivée qui se sait dérivée finit mesurée ; les autres finissent citées.

## L115 — Un garde-fou peut porter sur la bonne idée et la mauvaise condition

*(S33)* Une mesure exigeait qu'aucune réflexion ne pollue la fenêtre d'observation, et le contrôle
écrit pour cela vérifiait qu'on mesurait **derrière le front de l'onde**. C'est la bonne idée : la
fenêtre doit être dans la zone atteinte. Mais la condition manquait l'essentiel — elle ne vérifiait
pas que le front **n'avait jamais atteint le mur du fond**. L'onde était revenue ; le contrôle
déclarait la fenêtre saine.

Le défaut est plus dangereux qu'une absence de contrôle : **on fait confiance à ce qui existe**. Un
montage sans garde-fou est examiné à chaque usage ; un montage qui en a un ne l'est plus.

Et il est invisible par construction : une condition fausse ne se manifeste que **le jour où elle
est franchie**, c'est-à-dire dans un régime qu'on n'avait pas prévu — donc jamais pendant l'écriture.

**Réflexe** : pour tout garde-fou, écrire **le cas qu'il doit refuser**, et vérifier qu'il le refuse.
Pas le cas nominal, qu'il accepte de toute façon. Un garde-fou qu'on n'a jamais vu déclencher n'a
pas été testé.

## L116 — Une sonde générique voit des défauts qu'aucun contrôle spécifique n'attend

*(S33)* Le contrôle dédié n'a pas vu la réflexion. Ce qui l'a vue est le **coefficient de
détermination** de l'ajustement — tombé à 0,487 là où les cas sains donnaient 0,999. Il n'était pas
là pour ça : il était là pour dire si la décroissance est bien exponentielle.

C'est précisément ce qui fait sa valeur. Un contrôle spécifique répond à une question qu'on a su
poser ; une **sonde générique** — un `R²`, un résidu, un bilan de conservation — répond à la
question *« la forme que je suppose est-elle celle que j'observe ? »*, et cette question attrape tout
ce qui casse la forme, **y compris ce qu'on n'avait pas imaginé**.

**Réflexe** : accompagner toute régression d'un indicateur d'ajustement **publié**, même quand la
forme n'est pas en doute — surtout quand elle ne l'est pas. Trois lignes de code, et une sonde qui
travaille pour des défauts futurs. De même : un bilan de masse à côté d'une mesure de vitesse, un
résidu à côté d'un ajustement.

## L117 — Un écart qui change de signe n'est pas un biais

*(S33)* Quatre mesures comparées à une prédiction donnaient +5,9 %, +2,7 %, −0,3 %, −2,4 %. La
tentation est de lire « environ 3 % d'erreur » et de passer.

Le **signe** dit davantage que la taille. Un écart systématiquement de même signe est un **biais** —
un terme manquant, une constante fausse, un instrument décalé. Un écart qui **change de signe de
façon monotone avec un paramètre** est la signature d'un terme d'ordre supérieur négligé : la
prédiction est correcte au premier ordre, et l'écart résiduel suit le paramètre.

Le diagnostic est différent, et l'action aussi : un biais se corrige, un terme d'ordre supérieur se
borne. Ici il fixait le domaine de validité de la loi plutôt que de la mettre en doute.

**Réflexe** : devant une série d'écarts, regarder d'abord **la suite des signes**, puis la
progression. C'est le complément de L88, qui disait qu'un biais variant à l'envers de la théorie
désigne l'instrument ; ici, un biais qui change de signe désigne un terme négligé — et l'un comme
l'autre se lisent dans la **structure** des écarts, jamais dans leur moyenne.

## L118 — La non-testabilité prédit la défaillance

*(S34)* Sur dix garde-fous audités, un seul masquait au lieu de refuser. C'était **le seul qui
n'était pas appelable isolément** : il vivait en ligne dans une fonction lançant des simulations
coûteuses, et le vérifier demandait d'en exécuter une.

Ce n'est pas une coïncidence, et la chaîne est mécanique :

> emplacement en ligne → non testable isolément → jamais testé → jamais vu déclencher → défaut
> invisible

Les neuf autres avaient été éprouvés au fil des sessions **sans que ce soit délibéré** — simplement
parce qu'on pouvait les appeler, et qu'on finit par le faire.

**La conséquence de méthode est forte** : la testabilité isolée est un **prédicteur de défaut**, donc
un critère de revue plus efficace que la relecture. Chercher ce qui n'est pas appelable seul coûte
une recherche textuelle ; relire tout le code coûte une session et laisse passer ce qui a l'air
correct.

**Réflexe** : écrire tout contrôle comme une fonction appelable seule, et faire de son **premier
usage** son test de déclenchement. Le coût est de quelques lignes ; le gain est qu'il existera un
endroit où lui poser la question.

## L119 — Le test d'un garde-fou est le cas qu'il refuse, et il lui faut aussi un témoin

*(S34)* Un contrôle qui n'a jamais été vu refuser n'a pas été testé : le cas nominal passe de toute
façon, et son succès ne dit rien de la condition écrite. Le seul test qui informe est **celui qui
déclenche**.

Mais l'audit a fait apparaître la moitié manquante : un garde-fou qui refuserait **tout** passerait
aussi ce test-là. Il lui faut donc un **témoin** — le cas sain qu'il ne doit *pas* refuser.

Les deux ensemble encadrent la condition : l'un montre qu'elle attrape ce qu'elle vise, l'autre
qu'elle ne va pas au-delà. Séparément, chacun se satisfait d'une condition fausse — un contrôle
« toujours vrai » passe le témoin, un contrôle « toujours faux » passe le déclenchement.

**Réflexe** : deux tests par garde-fou, et pas un. Le second coûte trois lignes, et c'est lui qui
distingue un contrôle d'un refus systématique.

## L120 — Borner une estimation aberrante répond à une question qui n'en a plus

*(S34)* Une estimation d'ordre de convergence était bornée entre deux valeurs raisonnables, par
prudence. Le geste paraît anodin — on évite qu'un nombre absurde se propage.

Mais un ordre hors de ces bornes **n'est pas une valeur à corriger** : c'est le signe que les données
d'entrée ne sont pas dans le régime supposé. Le borner revient à répondre à une question dont on
vient d'apprendre qu'elle n'a pas de réponse, et à rendre le résultat **indiscernable** d'un cas
sain.

La correction n'est pas de supprimer le bornage — il faut bien un nombre pour continuer, et un
bornage conservateur est préférable à un nombre absurde. Elle est de **le signaler** : le résultat
reste utilisable, et son statut change.

**Réflexe** : devant tout `clamp`, `max`, `min` ou valeur de repli, se demander si la valeur écartée
était une **erreur de calcul** ou une **information**. Dans le second cas, la borne reste et le
signalement s'ajoute. Un garde-fou qui corrige sans le dire transforme une anomalie en résultat.

## L121 — On écrit un garde-fou pour empêcher, jamais pour mesurer

*(S34)* L'audit a établi que dix garde-fous *peuvent* refuser. Il n'a pas pu dire s'ils refusent
**en usage réel**, ni à quelle fréquence : aucun ne compte ses déclenchements.

Le même manque touche les saturations de modèle — les `max(0)` qui empêchent une hauteur d'eau
négative de se propager. Une saturation qui se déclenche rarement est un filet ; une saturation qui
se déclenche à chaque pas est un solveur qui produit des états impossibles et qu'on maquille.
**Les deux sont indiscernables** tant que personne ne compte.

L'omission a une cause commune, et elle est dans l'intention : **un garde-fou est écrit pour
empêcher quelque chose**, et une fois qu'il empêche, on passe à autre chose. L'idée qu'il puisse
aussi *renseigner* sur la santé du système ne vient pas — il est rangé dans la catégorie « sécurité »
et non « instrumentation ».

**Réflexe** : tout garde-fou et toute saturation portent un compteur. Ce sont les capteurs les moins
chers du système, ils sont déjà placés exactement là où les choses tournent mal, et leur fréquence de
déclenchement est une mesure de santé qu'aucun test ne donne.

---

> **Leçons importées de la lignée B le 2026-09-06 (S35).** Les quinze suivantes ont été écrites
> dans une histoire parallèle du dépôt (sessions **B-S22** à **B-S26**), où elles portaient les
> numéros `L71` à `L85` — déjà pris ici par d'autres leçons. Carte de renumérotation :
> [`FORK-S22-S26`](../docs/registres/FORK-S22-S26.md). **Leur texte n'a pas été modifié** ; seuls
> leurs renvois l'ont été.

## L122 — Certaines propriétés ne s'obtiennent pas en raffinant

*(B-S22)* Le premier schéma de solveur produisait 19,5 mm/s de courant parasite sur une pente au
repos, pour un seuil de 1. En divisant la maille par deux, l'erreur tombait d'un facteur 1,8. La
réaction naturelle est d'en conclure « il faut raffiner » — et de chiffrer : il aurait fallu
`dx ≈ 7 mm`, six mille mailles pour quarante mètres **en une dimension**, le cube de cela en trois.

Le schéma corrigé, lui, ne donne pas une petite erreur : il donne **1,5·10⁻¹⁵ m/s à toutes les
finesses de maille**. Ce n'est pas treize ordres de grandeur de mieux, c'est autre chose — l'arrondi
machine, c'est-à-dire l'absence d'erreur. La différence entre les deux colonnes n'est pas
quantitative.

**Réflexe** : devant une erreur qui décroît avec la résolution, ne pas se demander « à quelle
résolution devient-elle acceptable » mais **« existe-t-il une formulation où elle est exactement
nulle »**. Quand la réponse est oui — et pour tout ce qui relève d'un équilibre, d'une symétrie ou
d'une loi de conservation, elle l'est souvent — chercher cette formulation coûte moins cher que
n'importe quel budget de calcul, et le raffinement devient un choix de qualité au lieu d'un rattrapage.

## L123 — Un cas qui élimine doit garder en vie ce qu'il élimine

*(B-S22)* Deux fois dans la même session, un cas canonique a rejeté un premier jet : C01 a rejeté le
terme de fond centré, C04 a rejeté le flux de Rusanov. Dans les deux cas, la tentation était de
remplacer le fautif et de passer.

Les deux ont été **conservés**, derrière un interrupteur, avec un test qui **verrouille leur
échec**. Le motif n'est pas la nostalgie : `CAS-CANONIQUES` affirmait de C01 qu'il était « celui qui
élimine le plus de candidats », et cette affirmation n'avait jamais été démontrée faute d'un seul
code qui l'exerce. Le jour où le schéma naïf disparaît, C01 redevient un paragraphe qui l'affirme.
Pire : une retouche future pourrait l'équilibrer par accident, et personne ne s'apercevrait que le
cas ne discrimine plus rien.

**Réflexe** : quand un test rejette une implémentation, se demander si le test tire sa valeur de ce
rejet. Si oui, garder l'implémentation rejetée et **tester qu'elle échoue toujours**. Un test
d'élimination sans contre-exemple vivant est un test dont personne ne peut plus vérifier qu'il
mesure quelque chose.

## L124 — Un cas diagnostic ne prouve rien seul, et rend un échec attribuable

*(B-S22)* À côté des deux assertions de C01 — vitesse et surface — une troisième mesurait la
conservation du volume, avec cette mention explicite : *non probant*. Le schéma est conservatif par
construction, la maille perd exactement ce que sa voisine gagne ; ce cas ne pouvait pas échouer pour
une raison de physique.

Il a pourtant décidé de la suite. Quand C01 est tombé, le volume était conservé **exactement**, ce
qui écartait d'un coup la comptabilité du schéma et désignait l'équilibre hydrostatique. Sans lui,
l'échec aurait ouvert deux pistes au lieu d'une, et la mauvaise coûte une demi-journée.

**Réflexe** : à côté d'un cas qui peut échouer, prévoir un cas qui **ne peut pas** échouer pour la
même raison. Il ne compte pas dans la couverture — le dire dans le cas lui-même, sinon il gonfle un
décompte de vérifications (A104) — mais il partitionne les causes. Un échec attribuable vaut
plusieurs échecs constatés.

## L125 — Une thèse fausse écrite avant la mesure vaut mieux qu'une intuition juste écrite après

*(B-S23)* Le plan déclarait, avant d'écrire une ligne : *« la période de C03 passe, la demi-vie
échoue »*. Le raisonnement était solide — C04 venait d'établir que le schéma est d'ordre un, et un
schéma d'ordre un est diffusif. La demi-vie a donné **43 périodes** pour un minorant de 15.

La thèse était fausse, et c'est ce qui l'a rendue utile. En cherchant *pourquoi*, il a fallu
trouver le terme manquant : la diffusion numérique n'est pas une propriété du schéma, c'est une
propriété du schéma **et de la maille rapportée à ce qu'on transporte**. Sans la thèse écrite
d'avance, le résultat aurait été enregistré comme « C03 passe » et rangé — et la loi qui l'explique,
avec son balayage propre en ordre un, n'aurait jamais été cherchée.

**Réflexe** : écrire la prédiction **avant** la mesure, en une phrase, avec sa raison. Une
prédiction qui se réalise coûte une ligne ; une prédiction qui échoue désigne précisément l'endroit
où le modèle mental est faux, et c'est l'information la plus chère du métier. La règle vaut pour
tout ce qui se mesure — un profilage, une migration, une estimation de charge — pas seulement pour
un cas de test.

## L126 — Un paramètre qu'un énoncé ne fixe pas est tranché par le premier qui l'exécute

*(B-S23)* Cinq des sept cas canoniques exécutés portaient un paramètre libre dont le verdict dépend :
une résolution, un seuil de détection, une normalisation, une norme, une constante physique. Aucun
n'était signalé. Chacun a été tranché par la personne qui écrivait le code, **au moment de le
faire**, sur le critère le plus local, et sans que rien dans le résultat n'en garde trace.

Ce n'est pas une négligence d'auteur : c'est mécanique. **Tant qu'un énoncé n'est pas exécuté, son
paramètre manquant n'existe pas** — il n'y a rien pour le révéler, et aucune relecture ne le fera
apparaître, parce que relire consiste à reconstruire le sens, et que reconstruire le sens comble les
trous sans les signaler.

Le coût n'est pas l'imprécision, c'est **l'incomparabilité** : deux implémentations peuvent
satisfaire le même énoncé chacune sous ses propres conditions et n'être jamais comparées. Un critère
qui ne discrimine pas ne sert à rien, même quand il est vrai.

**Réflexe** : pour tout énoncé destiné à être vérifié — cas de test, critère d'acceptation, seuil
d'alerte, clause de contrat — appliquer le test des deux implémenteurs : *deux personnes qui ne se
parlent pas obtiennent-elles le même nombre ?* Si la réponse dépend d'un choix qu'aucune des deux
n'a écrit, l'énoncé est incomplet, **quelle que soit sa précision apparente**.

## L127 — Ce qu'un test ne couvre pas doit être imprimé par le test

*(B-S23)* Un cas canonique a été exécuté amputé de ses deux tiers : ni solide, ni rotation, en une
seule dimension — c'est-à-dire privé des deux raisons d'être que son propre énoncé lui donnait. Le
tiers restant passe confortablement.

Rien dans un rapport ne distingue « ce cas passe » de « le tiers de ce cas que j'ai su écrire
passe ». Et l'écrire dans un document ne suffit pas : le document se lit une fois, le rapport se lit
à chaque exécution, et c'est le rapport qui sera regardé le jour où quelqu'un décidera sur la foi
d'une colonne de verts.

**Réflexe** : la couverture manquante appartient à la **sortie** de l'outil, pas à sa documentation.
Faire imprimer, à chaque exécution, la liste de ce qui n'est pas couvert et de ce qui l'est
partiellement — avec la raison. Le coût est de quelques lignes ; il achète qu'un rapport vert ne
puisse jamais se lire comme une couverture complète.

## L128 — Une simplification algébrique efface le domaine où elle est valide

*(B-S24)* Le terme de fond d'un schéma s'écrit avec les hauteurs reconstruites aux **deux bords** d'une
maille. À l'ordre un, les deux valent la même chose, l'expression se réduit à une forme deux fois
plus courte, et c'est cette forme courte qui a été écrite — correctement. Étendue à l'ordre deux, où
les deux bords diffèrent, elle injectait une force parasite là où la source devait être exactement
nulle : **quinze fois l'erreur du schéma précédent, et croissante sous raffinement**.

Le point n'est pas qu'une simplification soit dangereuse — elle est juste dans son domaine. C'est que
**la forme simplifiée ne porte plus la trace de ce qui l'autorise**. Les deux termes qui s'annulaient
ont disparu, et avec eux la seule chose qui rappelait qu'ils s'annulaient *sous condition*. Le code
suivant hérite d'une expression correcte et d'aucun avertissement.

**Réflexe** : quand une expression se simplifie parce que deux grandeurs sont égales, écrire
l'hypothèse à côté de la forme courte — pas la démonstration, l'hypothèse, en une ligne. C'est le
seul endroit où elle sera lue au moment où quelqu'un la généralisera. La règle vaut hors des
mathématiques : toute abstraction qui « marche parce que, ici, X = Y » se casse au premier contexte
où X ≠ Y, et c'est toujours le contexte suivant.

## L129 — Un scalaire ne classe pas

*(B-S24)* Trois schémas numériques, comparés sur la position d'un front : le classement obtenu est
**inverse** du classement réel. Le schéma qui gagne sur ce chiffre est 2,2 fois pire sur l'erreur
globale, et son front dépasse la référence au raffinement suivant — ce n'est pas de la précision,
c'est une traînée parasite que le critère compte comme un succès.

Réduire un objet riche à un nombre est ce qui rend une comparaison possible ; c'est aussi ce qui
permet à un candidat de bien figurer **sur le nombre** sans être meilleur. Et le défaut est
invisible : le tableau de comparaison est parfaitement lisible, les chiffres sont exacts, la
conclusion est fausse.

**Réflexe** : un critère de classement doit inclure au moins une mesure **globale** — une norme, une
intégrale, une agrégation sur tout l'objet — à côté des mesures ponctuelles. Quand un candidat gagne
sur un point et perd sur la norme, c'est presque toujours qu'il triche sans le savoir. Cela vaut pour
les benchmarks de performance, les métriques produit, et toute note unique attribuée à quelque chose
qui a plusieurs dimensions.

## L130 — Améliorer la constante et améliorer le taux sont deux choses, et on les confond

*(B-S24)* Passer un schéma à l'ordre deux **en espace seulement** a divisé son erreur par 2,2 — un gain
franc, visible dès la première mesure. Et l'ordre observé n'a **pas bougé** : 0,725 contre 0,742. Le
gain était entièrement dans la constante.

Les deux se ressemblent sur une mesure isolée et n'ont rien à voir sur trois. Une constante divisée
par deux est un gain **acquis une fois** ; un exposant amélioré est un gain qui **croît avec la
taille du problème**. Ici, il fallait aussi l'ordre deux en temps — et c'est l'étape intermédiaire,
mesurée séparément *exprès*, qui l'a montré.

**Réflexe** : ne jamais conclure sur un ordre, un exposant ou une complexité à partir d'un seul point
de mesure — il faut au minimum trois tailles, et regarder le **rapport des rapports**. Et lorsqu'une
amélioration est censée changer le régime et non le facteur, exiger cette vérification avant de la
déclarer acquise : en optimisation comme en analyse numérique, un facteur deux est souvent tout ce
qu'on obtient d'un changement annoncé comme structurel.

## L131 — Avant de corriger, vérifier qu'on mesure la bonne chose

*(B-S25)* Un cas de validation était rouge depuis trois sessions. Son obstacle avait été nommé,
chiffré, et la session devait le corriger dans le code. **Aucune ligne de code n'a été touchée, et le
cas est passé au vert** : la référence et la mesure n'étaient pas la même grandeur, et le seuil de
détection choisissait un régime que rien ne pouvait satisfaire.

L'ordre des opérations n'est pas un détail de méthode, c'est ce qui a évité de « corriger » un
solveur qui suivait sa référence à 0,7 %. Une correction faite pour satisfaire une mesure fausse
**dégrade le code et rend le test vert** — le pire des deux mondes, et il ne laisse aucune trace.

**Deux corollaires**, tous deux vérifiés dans la même séance :

- **la mesure est un suspect au même titre que le code.** Trois défauts de mesure ont été
  soupçonnés ; un était réel, un était nul, un était réel mais sans effet sur le verdict. Aucun
  n'était devinable — il a fallu imprimer le profil terme à terme ;
- **une précaution correcte peut être sans effet.** L'un des trois — comparer une moyenne de maille
  à une moyenne de maille plutôt qu'à une valeur ponctuelle — était fondé, et les deux quantités
  coïncidaient à cinq chiffres. Le garder coûte peu ; l'annoncer comme une correction aurait été
  faux.

**Réflexe** : devant un écart persistant entre une mesure et une référence, écrire d'abord la
comparaison **terme à terme**, et se demander de chaque colonne si elle représente bien la même
grandeur que celle d'à côté. Cela prend un quart d'heure et arrive régulièrement avant le débogage.

## L132 — Un critère qu'aucun candidat ne peut satisfaire est aussi inutile qu'un critère que tous satisfont

*(B-S25)* Un seuil de détection avait été fixé à une valeur parfaitement reproductible — et il
désignait un régime où **aucune** implémentation possible ne pouvait passer. Le critère était donc
exact, stable, vérifiable… et incapable de distinguer un bon candidat d'un mauvais, puisqu'il les
recalait tous.

C'est le symétrique du défaut évident. Un critère trop laxiste ne trie pas ; on le sait. **Un critère
trop exigeant ne trie pas non plus**, et cela se voit beaucoup moins, parce qu'un critère sévère a
l'apparence de la rigueur. Dans les deux cas, la question à poser est la même : *si deux candidats
diffèrent, ce critère les sépare-t-il ?*

**Réflexe** : pour tout seuil, tout SLA, toute exigence chiffrée, vérifier qu'il existe **au moins un
comportement réaliste qui le passe et au moins un qui le rate**. Sinon ce n'est pas une exigence,
c'est une décoration — et elle coûtera du temps à quelqu'un qui la prendra au sérieux.

## L133 — Le coût de l'instrument croît sans que personne le regarde

*(B-S25)* La batterie de validation valait quatre centièmes de seconde à sa création. Cinq sessions
plus tard, elle en valait **soixante-quatorze** — chacune n'y ayant ajouté « qu'un balayage » de
plus. Personne ne mesure le temps de l'outil qui mesure, et il n'existe aucun seuil qui déclenche.

La dérive est indolore parce que chaque incrément est justifié : le balayage ajouté répond à une
vraie question. C'est l'accumulation qui ne l'est pas, et elle n'apparaît qu'au moment où l'outil
devient assez pénible pour qu'on cesse de le lancer — c'est-à-dire trop tard, puisque sa valeur
tient à ce qu'on le lance souvent.

**Réflexe** : donner à l'instrument son propre budget, et le lui faire afficher à chaque exécution,
comme il affiche ses résultats. La comparaison à ce budget doit être **dans la sortie**, pas dans la
tête de celui qui l'exécute.

## L134 — Une formule énoncée avec ses constantes se cite ; elle ne se recalcule pas

*(B-S26)* Un document de conception posait une formule, un profil, une constante, et concluait sur une
valeur. La vérification tenait en une multiplication : l'intégrale valait le tiers de ce que la
conclusion supposait, et **la valeur annoncée était fausse d'un facteur sept**. L'erreur a survécu
vingt et une sessions, six audits, deux revues croisées — et un dossier de banc entier qui s'appuyait
dessus pour fixer une borne.

Le mécanisme n'est pas l'inattention. **Un énoncé qui a la forme d'un résultat désactive la
vérification** : on ne recalcule pas un résultat, on le cite. Une formule accompagnée de ses
constantes ressemble à un aboutissement, alors qu'elle est un raisonnement — et un raisonnement se
refait.

**Réflexe** : repérer les formules **dont dépend une décision** — pas toutes, celles-là — et les
refaire une fois, avec leurs constantes, en écrivant le calcul intermédiaire. Le coût est de quelques
minutes par formule. Le signe qu'il faut le faire : la formule est citée ailleurs que là où elle a
été écrite.

## L135 — Un opérateur peut changer de nature sans erreur, et continuer à produire des nombres plausibles

*(B-S26)* Un amortissement s'écrivait `×(1 − σ·dt)`. Au-delà de `σ·dt = 1` le facteur devient négatif ;
une saturation à zéro l'en empêche, et la maille cesse d'être amortie : elle est **écrasée** à l'état
de repos à chaque pas. L'opérateur est devenu autre chose — sans plantage, sans avertissement, et en
rendant des valeurs qui avaient l'air meilleures que les précédentes.

C'est le cas dangereux. Un opérateur qui explose se signale ; un opérateur qui **dégénère** continue
de fonctionner, et ce qu'il mesure n'a plus le sens qu'on lui prête. Les protections écrites pour la
robustesse — saturations, valeurs de repli, bornes de sécurité — sont précisément les endroits où
cela se produit, parce qu'elles sont conçues pour que rien ne se voie.

**Réflexe** : à côté de chaque saturation ou repli, se demander *dans quel régime il s'active*, et
**faire afficher ce régime dans la sortie**. Ici, une colonne `σ·dt` avec un marqueur au-delà de 1 a
suffi : sans elle, trois lignes d'un tableau auraient été lues comme mesurant ce que les autres
mesuraient.

## L136 — Un témoin sépare ce qu'on mesure de ce qui traîne avec

*(B-S26)* Il fallait mesurer ce qu'une frontière absorbante réfléchit. Le train d'ondes parcourt deux
cents mètres avant de revenir à la jauge, et **il perd 10 % de son amplitude en route**, par
dissipation du schéma. Sans précaution, ces 10 % sont comptés comme de l'absorption : le dispositif
est crédité d'une qualité qui appartient à la route.

La parade a coûté un essai de plus — le même montage avec l'absorption **désactivée**, où le bord
redevient un mur parfait. Le rapport des deux élimine tout ce qui est commun aux deux trajets. Et le
témoin lui-même est une mesure utile : s'il s'écarte trop de 1, c'est la résolution qui est en cause,
et la mesure principale ne veut plus rien dire.

**Réflexe** : dès qu'une grandeur se mesure **après un trajet, un délai ou une chaîne de traitement**,
prévoir l'essai identique où le dispositif étudié est neutralisé. C'est la différence entre mesurer
un effet et mesurer un effet **plus tout ce qui l'accompagne** — et le second est toujours plus
flatteur.

---

## L137 — Un correctif de procédure écrit dans une seule branche ne protège que cette branche

*(S35)* Le dépôt a forké deux fois. Le premier fork a été constaté, documenté dans un registre, et
la procédure d'amorce a été corrigée pour qu'il ne se reproduise pas. **Tout cela a été écrit d'un
seul côté du fork.** L'autre lignée, qui n'avait rien constaté, a reforké huit sessions plus tard
par le même mécanisme — sans jamais savoir qu'elle était exposée.

Le défaut a une forme générale, et elle est déplaisante : **le remède au fork est lui-même sujet au
fork.** Il en va de même de tout correctif écrit dans un artefact que la divergence peut dupliquer —
un fichier de configuration, une convention d'équipe, un fichier de règles, une page de wiki par
projet.

**Réflexe** : après avoir écrit un correctif de procédure, se demander *dans combien d'endroits ce
texte doit exister pour faire son travail*, et l'y mettre le jour même. Un correctif qui vit à un
seul endroit protège cet endroit. Le corollaire pratique : ce qui protège vraiment est ce qui
s'exécute au démarrage — dans ce projet, deux commandes de `git` — et non ce qui se lit.

## L138 — Deux implémentations du même modèle valent un oracle, et ne valident rien du modèle

*(S35)* Deux lignées parallèles ont écrit indépendamment le même solveur : mêmes équations, même
flux, même intégration. La tentation immédiate est d'y voir une confirmation croisée.

**Elle n'en est pas une au niveau qui compte.** Les deux partagent le modèle, donc exactement les
mêmes angles morts — une dimension, pas de dispersion. Ce que le modèle ne contient pas, aucune des
deux ne peut le voir, et **deux erreurs identiques ne se corrigent pas en se répétant**.

Ce que la concordance élimine, en revanche, est la **faute d'implémentation** : l'indice décalé, le
signe inversé, la condition de bord mal posée. C'est la classe de faute qui produit les résultats
les plus convaincants et les plus faux, et un projet à une seule implémentation n'a aucun moyen de
la détecter — il n'a que ses propres assertions, écrites par ceux qui ont écrit le code.

**Réflexe** : devant deux réalisations qui concordent, séparer ce qu'elles partagent de ce qu'elles
ne partagent pas. La concordance ne certifie que la couche non partagée. Et le corollaire est
d'action : **ne pas supprimer la seconde implémentation au motif qu'elle fait double emploi** —
c'est précisément ce double emploi qui rend un désaccord informatif.

## L139 — Ce qui résiste à une fusion n'est pas le gros morceau, c'est celui qui touche au reste

*(S35)* Le plan annonçait le code comme la partie lourde : 1070 lignes de solveur d'un côté, 2431 de
harnais modifiées des deux côtés. La prédiction était fausse dans les deux sens.

**Le solveur s'est importé en deux lignes** et n'a demandé aucune retouche — parce qu'il ne dépend
que d'une interface d'hôte qui n'avait pas divergé. **C'est le harnais qui résiste**, alors qu'il
contient moins de code neuf : chaque montage y est écrit contre son propre solveur et porte les
mêmes noms de fonction des deux côtés.

La taille ne prédit rien ; **la surface de contact prédit tout**. Un fichier volumineux à interface
étroite se transporte ; un fichier moyen qui nomme les mêmes choses que son homologue entre en
collision sur chaque nom.

**Réflexe** : avant d'estimer un travail de fusion, de migration ou d'extraction, compter les
**dépendances** de chaque fichier, pas ses lignes. Et quand deux modules se disputent des noms, la
bonne réponse est souvent de ne pas les fusionner du tout : deux modules côte à côte coûtent moins
qu'un module réconcilié, et gardent la comparaison possible.

## L140 — Un document se renumérote, un événement se préfixe

*(S35)* Réconcilier deux histoires demandait de désambiguïser cinq identifiants d'ADR, quinze de
leçons, douze d'angles morts — et cinq numéros de session. Les quatre premiers ont été renumérotés à
la suite. **Les sessions ne l'ont pas été** : elles ont reçu un préfixe.

La raison n'est pas esthétique. Un identifiant de document est une **étiquette** : elle désigne, elle
ne prétend rien, et la changer ne coûte que des renvois à réécrire. Un numéro de session est un
**événement daté** : il dit *ce qui s'est passé, quand, dans quel ordre*. Renuméroter S24 en S39
placerait un travail de l'après-midi après un travail du soir et **mentirait sur la chronologie** —
une falsification, pas une convention.

**Réflexe** : devant une collision d'identifiants, trier d'abord les identifiants en deux tas — ceux
qui *nomment* et ceux qui *datent*. Renuméroter les premiers, préfixer ou qualifier les seconds. La
distinction vaut au-delà d'une fusion de dépôts : numéros de version, de ticket, de build, de
migration.

## L141 — Un test vert ne dit pas qu'un chiffre publié est encore vrai

*(S36)* `p = 1,003` a été publié dans un ADR, puis périmé par une correction faite pour un autre cas
une session plus tard. Le code rend désormais 0,9997. **Personne ne l'a vu, et personne ne pouvait
le voir** : l'assertion qui protège cette grandeur est un **minorant** — `p > 0,8` — et trois
millièmes ne la font pas broncher. Le cas est resté vert de bout en bout.

C'est le défaut des décomptes recopiés — trouvé en S07, retrouvé en S10 — appliqué à un objet qu'on
croyait à l'abri : **une mesure**. Un décompte se périme parce que personne ne le recompte ; un
chiffre mesuré se périme parce que personne ne le remesure, et la suite de tests ne compense rien.
Elle surveille des **seuils**, pas des **valeurs**.

**Réflexe** : distinguer, dans ce qu'un test protège, la **décision** (le cas passe-t-il ?) et la
**valeur** (combien vaut-il ?). Une assertion à seuil ne protège que la première. Pour la seconde,
il faut soit un test qui compare au chiffre publié — c'est ce qui a révélé l'écart ici — soit
l'acceptation explicite que tout chiffre cité dans un document est un instantané daté. Le choix se
fait document par document ; l'omettre revient à choisir le second sans le dire.

## L142 — Un chiffre cité hors du tableau qui le produit perd ce qui le rend vrai

*(S36)* `ADR-040` §5 publie un tableau à **deux colonnes** — ordre un, ordre deux — et le chiffre
« 43,1 périodes » y désigne la colonne de gauche. Ailleurs dans le corpus, dans un autre ADR et dans
`CAS-CANONIQUES`, le même 43,1 circule **seul**. Une session l'a pris pour la valeur du cas, a
mesuré 161,14 sur le montage courant, et a cru un instant que le chiffre ne se reproduisait pas.

La citation n'était pas fausse : elle était **incomplète d'une dimension**. Le tableau d'origine
énonçait la condition ; la citation ne l'a pas emportée avec elle. C'est la forme générale d'**A153**
— *l'ordre d'un schéma est un couple (schéma, solution), pas un nombre* — et elle vaut pour toute
grandeur qui dépend d'un réglage : une demi-vie, un temps de réponse, un taux d'erreur, un débit.

**Réflexe** : citer un chiffre avec le paramètre qui le distingue de ses voisins dans le tableau
d'où il vient, ou ne pas le citer du tout et renvoyer au tableau. La question à se poser est
mécanique : *ce tableau a-t-il plus d'une ligne, ou plus d'une colonne ? Alors la citation doit dire
laquelle.*

## L143 — Reproduire un chiffre publié n'est pas la même opération que faire passer un test

*(S36)* La session a transporté six montages d'une lignée à l'autre. Les faire compiler et passer
n'a rien révélé : tout était vert du premier coup. Ce qui a révélé quelque chose, c'est d'avoir
**confronté quatre grandeurs à ce que les documents en disaient** — dont une qui ne correspondait
plus.

Les deux opérations paraissent voisines et ne mesurent pas la même chose. Faire passer un test
vérifie que le code satisfait ses propres critères. Reproduire un chiffre publié vérifie que **le
code et sa documentation parlent encore du même objet** — et c'est la seule des deux qui puisse
détecter que la documentation a vieilli sans que rien ne casse.

**Réflexe** : à chaque reprise d'un travail écrit par d'autres — ou par soi, six mois plus tôt —
lister d'abord les **chiffres publiés** et les rejouer, avant d'écrire quoi que ce soit de neuf.
C'est court, c'est falsifiable, et l'écart trouvé est presque toujours plus intéressant que la
concordance. Trois chiffres sur quatre se sont reproduits ici ; c'est le quatrième qui valait la
session.

## L144 — Comparer deux mesures de précisions différentes revient à mesurer la moins précise

*(S37)* Deux implémentations du même modèle ont été confrontées pour la première fois. Sur le cas où
la solution exacte est connue, leur écart s'est révélé **exactement égal** à l'erreur de l'une des
deux contre la vérité — au chiffre près, à toutes les durées. Ce n'était pas une coïncidence : l'une
calcule en simple précision, l'autre en double, et **neuf ordres de grandeur** les séparent. La plus
exacte tient lieu de vérité, et la comparaison n'apprend rien de plus que la mesure directe.

**Un instrument de comparaison a un plancher, et sous ce plancher il ne mesure que lui-même.** La
conséquence pratique n'est pas qu'il faille des précisions égales — c'est qu'il faut **mesurer le
plancher avant de lire le résultat**, faute de quoi on ne sait pas si un écart parle du sujet ou de
l'instrument.

**Réflexe** : avant toute comparaison A contre B — deux implémentations, deux versions, deux
environnements, un avant et un après — chercher la grandeur qui borne ce que la comparaison peut
distinguer, et la **mesurer** plutôt que l'estimer. Quand un cas de référence exact existe, il la
donne gratuitement : confronter chaque côté à la vérité avant de les confronter l'un à l'autre. Un
écart rapporté sans son plancher n'est pas un résultat.

## L145 — Deux codes chacun cohérent peuvent être incompatibles sur une convention qu'aucun test ne voit

*(S37)* Deux solveurs écrits séparément se sont révélés porter deux définitions du mot « sec » —
`10⁻⁶ m` d'un côté, `10⁻¹⁰ m` de l'autre, quatre ordres de grandeur. Une cellule entre les deux est
sèche pour l'un et mouillée pour l'autre. Sur le front d'une rupture de barrage, l'écart de vitesse
atteint **98 % de la vitesse maximale du montage**.

**Les deux suites de tests étaient vertes, et aucune ne pouvait voir le problème.** Un test vérifie
qu'un code est cohérent **avec lui-même** : ses assertions sont écrites dans la même convention que
son implémentation. Une convention n'a pas de contraire interne — elle n'a qu'un contraire *chez le
voisin*. Le seul instrument qui la révèle est une seconde implémentation, ou un consommateur qui
attend autre chose.

Le plus instructif est que **les deux lignées avaient identifié la question** — chacune a un
document qui dit que la position d'un front dépend du seuil qui la définit. Elles y ont répondu
différemment sans le savoir. *Savoir qu'un choix existe ne protège pas de diverger sur ce choix.*

**Réflexe** : à toute frontière entre deux composants écrits séparément — deux services, deux
équipes, un producteur et son consommateur — lister les **seuils, unités, arrondis et conventions de
nommage** avant de comparer les comportements. Ce sont eux qui divergent en silence, et ils ne
laissent aucune trace dans les suites de tests de part et d'autre.

## L146 — Aligner avant de comparer, et chiffrer ce que l'alignement déplace

*(S37)* La confrontation de deux solveurs a d'abord donné un écart de 1,4 %, jugé grand. Un réglage
n'était pas le même des deux côtés — le flux numérique, choisi différemment et pour de bonnes
raisons dans chaque lignée. Une fois aligné, l'écart tombe à **0,065 %** : **vingt et une fois
moins**.

Sans l'alignement, la comparaison mesurait la différence entre deux *choix de méthode*, pas entre
deux *implémentations* — deux questions sans rapport, et l'une des deux ne se posait pas.

Mais l'alignement seul n'aurait pas suffi : **c'est le facteur 21, mesuré exprès, qui donne son sens
au 0,065 %**. Sans échelle, un petit nombre ne dit rien — il pourrait être petit parce que les deux
codes se ressemblent, ou parce que la grandeur est intrinsèquement peu sensible.

**Réflexe** : avant une comparaison, énumérer les réglages qui doivent être identiques et le dire
explicitement dans le compte rendu. Puis mesurer **une fois** avec un réglage volontairement
désaligné : ce contre-exemple donne l'échelle, et transforme « ils concordent » en « ils concordent
vingt et une fois mieux que s'ils différaient d'un choix de méthode ».

## L147 — Un rattrapage qui ne se déclenche jamais n'est pas inoffensif : c'est un détecteur muet

*(S38)* Une saturation remettait à zéro les hauteurs d'eau négatives dans deux solveurs. On la
soupçonnait de maquiller un défaut à chaque pas. Mesure : **elle ne se déclenche jamais** en régime
nominal — zéro sur trois cas, deux implémentations, cinquante mille pas. Le soupçon tombe.

Mais le compteur qui l'établit dit aussi autre chose. Elle **mord** dès qu'on sort de la condition de
stabilité du schéma, et là **elle ne rattrape rien** : la masse créée dépasse le volume total de
dix-sept ordres de grandeur. Elle ne convertit pas une divergence en résultat acceptable — elle la
convertit en **suite de nombres finis**, c'est-à-dire en quelque chose qui ressemble à un résultat
et que rien n'arrête.

**Le danger n'est donc pas la fréquence, c'est le silence.** Un rattrapage qui ne se déclenche jamais
est le meilleur détecteur possible de l'événement rare qu'il rattrape : le jour où il mord, c'est
l'information la plus utile que le système puisse produire. Tant qu'il est muet, elle est perdue.

**Réflexe** : devant tout `clamp`, `max`, valeur de repli, `catch` qui avale, retry silencieux —
poser deux questions dans cet ordre. *À quelle fréquence se déclenche-t-il ?* — et si la réponse est
« jamais », ne pas s'arrêter là. *Que vaut le système au moment où il se déclenchera ?* Si la
réponse est « il est déjà perdu », alors ce n'est pas un filet : c'est une alarme, et il faut la
brancher.

## L148 — Un seuil coupe ce qu'il nomme, jamais ce qu'on croit qu'il nomme

*(S38)* Deux solveurs déclarent une cellule « sèche » sous un seuil de hauteur, et lui donnent alors
une vitesse nulle. On en concluait naturellement que rien n'entre plus dans une cellule sèche.
**Faux** : le seuil coupe la **vitesse**, pas le **flux de masse**. Le terme de diffusion du schéma
continue à y déposer de la matière, même quand les deux vitesses voisines sont nulles. Les deux
solveurs traînent un film derrière leur front, de longueur proportionnelle à leur seuil.

L'erreur de lecture est banale et se reproduit partout : un test conditionnel est lu comme s'il
gouvernait **l'objet**, alors qu'il ne gouverne qu'**une expression**. « Sec » nommait une propriété
physique ; le code n'en implémentait qu'une conséquence parmi trois.

**Réflexe** : quand un seuil porte un nom qui décrit un **état** — sec, vide, inactif, expiré,
déconnecté — chercher toutes les conséquences que cet état devrait avoir, et vérifier laquelle le
code applique réellement. Puis renommer le seuil d'après ce qu'il fait, ou compléter le code d'après
ce qu'il nomme. Un nom d'état sur un seuil d'expression est une promesse que personne ne tient.

## L149 — Un inventaire fait à la lecture manque un point ; un changement de signature les trouve tous

*(S38)* Un recensement écrit, relu, mis en tableau, avait manqué un point de saturation sur cinq. Il
n'a été trouvé que parce que la fonction concernée est passée de fonction libre à méthode : le
compilateur a refusé l'appel restant et l'a désigné.

La relecture ne pouvait pas le trouver, et pas par inattention — l'appel manquant était dans une
branche conditionnelle d'un étage intermédiaire d'intégrateur, à trente lignes de ses jumeaux et
d'apparence identique. **La lecture cherche ce qu'elle sait chercher.**

**Réflexe** : pour recenser tous les usages de quelque chose, ne pas lire — **forcer chaque usage à
se déclarer**. Changer une signature, ajouter un paramètre obligatoire, rendre un type non
copiable, retirer une valeur par défaut : le compilateur, le typeur ou l'éditeur de liens énumère
alors ce que la lecture aurait manqué. Cela vaut aussi pour un `grep` : il ne trouve que les
formulations auxquelles on a pensé.

---

> **Leçons importées de la lignée B le 2026-09-07 (S39).** Les trois suivantes ont été écrites en
> **B-S27**, où elles portaient les numéros `L86` à `L88` — déjà pris ici par des leçons de S07-S08.
> Carte : [`FORK-S22-S26`](../docs/registres/FORK-S22-S26.md) §3.2 bis.

## L150 — Une mesure ne peut pas dire de quel cadre elle dépend

*(B-S27)* Une session avait mesuré qu'un dispositif d'absorption réfléchissait `1,6·10⁻³` **quelle que
soit sa largeur**, et en avait tiré une règle de dimensionnement qui déplaçait un paramètre central.
La mesure était exacte, reproductible, corrigée de son biais de trajet, balayée sur deux décades.
**Elle ne décrivait pourtant pas le dispositif : elle décrivait le milieu.** Changé le milieu, le
même dispositif réfléchit **67 %**.

Ce qui rend l'épisode instructif, ce n'est pas l'erreur — c'est qu'**aucune vérification interne à la
mesure ne pouvait la révéler**. Ni un raffinement, ni un témoin, ni un balayage de paramètre, ni un
second estimateur : tous restent dans le cadre où la mesure est faite, et c'est le cadre qui portait
l'hypothèse. Une mesure établit une relation **entre ses variables** ; elle est muette sur ce qu'elle
tient fixe.

**Réflexe** : avant de tirer une règle générale d'une mesure, écrire la liste de ce que le montage
tient fixe **et qui n'apparaît nulle part dans le résultat** — le milieu, le régime, l'échelle, la
linéarité. Chaque entrée de cette liste est une réserve à nommer dans la conclusion, avec la mesure
qui la lèverait. Nommer la réserve ne la lève pas ; mais **c'est ce qui transforme une rétractation
en étape prévue plutôt qu'en démenti**, et c'est la seule protection connue.

## L151 — Tout montage de mesure doit venir avec un essai dont le résultat attendu est zéro

*(B-S27)* Le montage d'une mesure de réflexion a été faux deux fois : la fenêtre d'observation était
contaminée, d'abord par un retour périodique, ensuite par la queue du signal incident. **Le résultat
mesuré valait 0,18 à 0,32 dans les trois montages** — les deux faux et le bon.

Un résultat stable au travers d'erreurs de montage n'est pas rassurant : c'est la situation la plus
dangereuse, parce que la stabilité **ressemble à de la robustesse**. Les deux défauts n'ont été vus
que par un essai construit pour rendre **zéro** : le même montage avec le dispositif étudié
neutralisé, et une fenêtre qui doit ne rien contenir. Il a rendu 5,1 %, puis 3,4 %, enfin `4·10⁻¹⁷`.

**Réflexe** : pour tout dispositif de mesure, écrire l'essai nul **avant** l'essai réel — celui dont
on sait ce qu'il doit rendre, et dont l'écart à zéro n'a aucune interprétation possible sinon un
défaut de montage. Un test qui peut échouer pour une seule raison vaut dix tests qui peuvent échouer
pour dix.

## L152 — Un budget qu'on desserre quand il gêne ne mesure plus rien

*(B-S27)* La batterie de validation, à laquelle chaque session ajoute « un balayage », a dépassé son
budget de temps. Le réflexe naturel — porter le budget de 120 à 180 secondes — aurait fait
disparaître l'avertissement sans rien changer au problème, et aurait garanti qu'il revienne.

C'est la version coûteuse de baisser une tolérance pour faire passer un test. Dans les deux cas, le
seuil cesse d'être une **contrainte** pour devenir une **description** : il ne dit plus ce qu'on veut,
il dit ce qu'on a. Et un seuil qui suit la mesure ne peut plus la contredire, donc ne sert plus.

**Réflexe** : quand un budget est dépassé, les seules réponses sont **réduire la consommation** ou
**assumer le dépassement en le laissant visible**. Desserrer est une troisième option qui ressemble
aux deux premières et n'en est aucune. Le corollaire vaut pour tout seuil qu'on s'impose — délai,
taille, latence, dette : *le moment où il gêne est le seul moment où il travaille.*

## L153 — Une action confiée à « l'utilisateur » n'a pas de porteur

*(S39)* Le dépôt a forké trois fois par le même mécanisme. Après le deuxième, une action a été
relevée qui disait exactement quoi faire pour empêcher le troisième : *répliquer le registre dans
toutes les branches vivantes le jour où l'une d'elles est reprise*. Elle a été inscrite au registre
des actions, avec pour porteur « l'utilisateur ». **Quatre sessions plus tard, le troisième fork
s'est produit, pour exactement la raison que l'action nommait.**

L'action n'était pas mauvaise, et l'utilisateur n'a rien manqué : **elle n'avait simplement pas de
moment d'exécution**. Une action portée par quelqu'un qui n'est pas dans la boucle de travail
n'est jamais en tête de file de personne. Elle est lue à chaque session par quelqu'un qui la
reconnaît comme n'étant pas la sienne, et elle survit intacte jusqu'à ce que le défaut se
reproduise.

**Réflexe** : à toute action relevée, se demander *qui l'exécutera, et à quel moment précis*. Si la
réponse n'est pas « la prochaine session qui rencontre telle condition », alors soit la faire
maintenant, soit la transformer en **règle vérifiée au démarrage** — dans le fichier d'amorce, dans
un test, dans un contrôle automatique. Une action dont le porteur est extérieur au travail doit être
signalée comme telle à chaque fois qu'on la lit, ou elle ne sera jamais faite.

## L154 — Une distinction neuve doit d'abord être passée sur le document qui la pose

*(S39)* Un ADR établit que le corpus appelle du même mot trois objets distincts, et met en garde en
toutes lettres : *tant qu'ils partagent le mot, un résultat sur l'un se lit comme un résultat sur
l'autre*. **Trois paragraphes plus bas, dans sa section de décisions, il commet exactement cette
erreur** — il compte comme desserrant une contrainte un résultat qui porte sur le mauvais des trois
objets.

L'erreur a survécu quatre sessions et n'a été trouvée que parce qu'une rétractation extérieure a
forcé à relire cette décision. Rien dans le document ne signalait la contradiction : les deux
passages sont justes séparément.

Le motif est mécanique. Une distinction s'énonce dans un paragraphe, mais elle **invalide des
raisonnements écrits ailleurs** — y compris ceux qu'on vient d'écrire, dans l'élan de la même
session, avec l'ancienne façon de penser encore active.

**Réflexe** : après avoir posé une distinction — deux sens d'un mot, deux cas d'un type, deux
régimes d'un système — relire immédiatement **les conclusions du document qui la pose**, puis celles
des documents voisins, en se demandant de laquelle des deux choses chacune parle. C'est un balayage
de dix minutes, et c'est le seul moment où l'on a la distinction fraîche et les conclusions sous la
main.

## L155 — Un document qui dit comment l'infirmer vaut mieux qu'un document qui a raison

*(S39)* Un ADR a été rétracté sur deux de ses quatre décisions, quatre sessions après avoir été
écrit. Il ne s'en trouve pas diminué — il s'en trouve **validé**.

Son §6 énonçait sa propre réserve : *le milieu où cette mesure a été faite n'a pas la propriété qui
compte ; la règle que je retire protégeait peut-être exactement de cela ; **c'est la première chose
à mesurer**.* Une session suivante a construit l'instrument, fait la mesure, et la réserve était
fondée. Le résultat a coûté une session — pas une refonte, pas une enquête, pas la découverte
tardive d'un défaut en aval.

**Comparer avec l'alternative** : le même ADR sans sa réserve aurait été juste dans son domaine et
faux hors de lui, sans que rien dans le texte ne dise lequel des deux on lisait. Il aurait fallu que
le défaut se manifeste ailleurs, plus tard, sans étiquette.

**Réflexe** : à chaque conclusion, écrire la phrase *« ceci serait faux si… »* et la rendre
**exécutable** — quelle mesure, quel montage, quel régime. Ce n'est pas une précaution rhétorique :
c'est ce qui transforme une conclusion en instrument. Un document qui a raison rend service une
fois ; un document qui dit comment l'infirmer rend service jusqu'à ce qu'on l'infirme.

## L156 — Une grandeur dont le dénominateur est un réglage n'est pas mesurable

*(S40)* Deux implémentations du même modèle divergeaient de 98 % de la vitesse maximale de leur
montage sur une cellule. La cause : un seuil qui décide à partir de quelle hauteur d'eau on calcule
`u = hu/h`. Sous ce seuil la vitesse est lue nulle, au-dessus elle est lue comme le quotient — et sur
un film d'un dixième de nanomètre, le quotient rend n'importe quoi.

Le balayage l'a établi sans ambiguïté : la quantité **dépasse la borne physique de son propre
montage** — la vitesse maximale que la solution exacte contient — et **varie d'un facteur deux et
demi avec le réglage, sans tendance monotone**. Deux signes qui, ensemble, ne laissent aucune lecture
alternative.

**La conclusion n'est pas qu'il faut mieux régler le seuil**, c'est que cette quantité n'a pas de
valeur à publier. Toutes les grandeurs que le corpus mesure vraiment — position du front, hauteur,
volume — se sont révélées insensibles au même réglage sur sept décades.

**Réflexe** : devant une grandeur dérivée par division, regarder **qui décide du dénominateur**. Si
c'est un seuil, une valeur de repli, un compteur qui peut valoir zéro, un intervalle de temps
configurable — alors la grandeur existe dans la plage où le dénominateur est franc, et **nulle part
ailleurs**. Deux tests le montrent : *dépasse-t-elle une borne physique connue du montage ?* et
*varie-t-elle sans tendance quand on bouge le réglage ?* Une réponse positive à l'une des deux suffit
à retirer la grandeur des rapports.

## L157 — Une raison fausse à l'appui d'une bonne décision la fait reporter indéfiniment

*(S40)* Une session avait refusé de trancher une question, à juste titre, et avait écrit pourquoi :
*« cela déplacerait la position du front, donc le verdict d'un cas, donc un critère d'entrée de
banc »*. Le refus était sage. **La raison était fausse** : mesuré, le front bouge de 0,148 % pour une
tolérance de 3 %.

L'effet n'a pas été de tromper qui que ce soit — personne n'a pris de mauvaise décision. L'effet a
été que **l'action a été reportée quatre fois de suite**. Chaque session la lisait, voyait un coût
annoncé considérable — rouvrir un critère de banc — et choisissait autre chose. La bonne prudence
était devenue un épouvantail.

Le vrai motif de prudence existait pourtant, et il était plus simple : *une des deux valeurs n'avait
aucune provenance*. Écrit ainsi, il désignait le travail à faire — mesurer — au lieu d'un risque à
éviter.

**Réflexe** : quand on décide de **ne pas** faire quelque chose, écrire la raison avec autant de soin
que pour une décision positive, et la formuler comme *ce qui manque* plutôt que comme *ce que ça
casserait*. Un coût surestimé dans un document ne se corrige jamais tout seul : il est cité,
respecté, et il repousse l'action jusqu'à ce que quelqu'un mesure. **La prudence se justifie par une
ignorance, pas par une catastrophe supposée.**

## L158 — Reprendre un travail extérieur prend ses conclusions et laisse ses dispositifs

*(S41)* Deux réconciliations ont importé d'une lignée parallèle ses décisions, ses leçons et ses
angles morts — dont un qui disait : *un paramètre qu'un énoncé ne fixe pas est tranché en silence par
le premier qui mesure*. **Elles ont laissé le remède que cette lignée avait construit contre lui** :
une rubrique de spécification ajoutée à chaque fiche de cas, neuf occurrences là-bas, zéro ici.

Le mécanisme est banal et il n'a rien d'une négligence. Un import se guide sur une liste de
**documents modifiés** ; il voit un fichier changé des deux côtés, en réconcilie le contenu visible —
et ne voit pas qu'une **rubrique** a été ajoutée à l'intérieur, parce qu'une rubrique n'est pas un
document. Les conclusions sont des objets nommés, faciles à énumérer ; les dispositifs sont diffus.

**Réflexe** : en reprenant le travail de quelqu'un d'autre — une branche, un fork, une équipe
dissoute, un projet repris — lister séparément ce qu'il a **conclu** et ce qu'il a **mis en place**.
Pour la seconde liste, la question qui fonctionne est : *contre chacun des problèmes qu'ils ont
nommés, qu'ont-ils construit ?* Un angle mort importé sans son remède est une dette qu'on croit
avoir payée.

## L159 — Un défaut absent mais armé se documente là où il se refermerait

*(S41)* Une lignée voisine avait payé cher une simplification algébrique : correcte tant que le
schéma était d'ordre un, fausse dès l'ordre deux, et **rien dans l'écriture courte ne rappelait
l'hypothèse qui l'autorisait**.

Le même code, ici, portait la **forme générale** — donc le défaut n'existait pas. La tentation était
de classer l'angle mort « sans objet ici » et de passer. Mais le solveur est d'ordre un : la forme
courte y serait **exacte**, et un lecteur qui simplifierait aurait raison sur le moment et armerait
le piège pour plus tard.

Le commentaire écrit ne corrige rien — il n'y a rien à corriger. Il dit **pourquoi la forme longue
est là**, à l'endroit précis où quelqu'un voudrait la raccourcir.

**Réflexe** : quand une vérification conclut « ce défaut n'existe pas chez nous », se demander
*qu'est-ce qui le ferait apparaître ?* Si la réponse est un changement plausible — passer à l'ordre
supérieur, généraliser un cas, lever une contrainte — alors écrire la raison **au point de
modification**, pas dans un registre. Un registre se lit quand on cherche ; un commentaire se lit
quand on touche.

## L160 — Attribuer un effet à une cause quand trois ont changé est faux par construction

*(S41)* Un document notait qu'un cas de validation échouait sur un véhicule et passait sur l'autre,
et donnait l'explication : *l'autre est passé à l'ordre deux.* C'était plausible, c'était la
différence la plus visible, et c'était **la moitié de la vérité**.

Trois choses différaient en réalité : l'ordre du schéma, le seuil de détection, et la référence à
laquelle on comparait. Mesuré à schéma égal, la seule révision de la mesure retirait **la moitié** de
l'écart. Aucun des deux facteurs seul ne franchissait la tolérance.

L'erreur ne vient pas d'un défaut d'attention : elle vient de ce qu'**une seule des trois différences
avait un nom**. « Passer à l'ordre deux » est un événement, il a une date et un ADR ; « la référence
a changé de ponctuelle à moyennée » était une ligne dans une fonction. On attribue à ce qu'on peut
nommer.

**Réflexe** : avant d'écrire *A explique B*, énumérer **tout** ce qui diffère entre les deux
situations comparées — y compris ce qui n'a pas de nom : un seuil, une version de dépendance, une
machine, un jeu de données, une façon de mesurer. Puis neutraliser les facteurs un par un. Si c'est
trop coûteux, écrire *A et le reste expliquent B* plutôt qu'une attribution qu'on n'a pas faite. Une
attribution non démontrée se cite ensuite comme un fait.

## L161 — Une valeur de repli placée après une mesure efface le refus de cette mesure

*(S42)* Une mesure rendait `NaN` sur un montage vide — un refus correct, même s'il venait d'un
accident de calcul plutôt que d'une intention. La ligne suivante saturait la valeur pour l'affichage,
`valeur.min(1e6)`. **En Rust, `f64::min` propage le non-`NaN`** : `NaN.min(1e6)` rend `1e6`.

Le refus s'est donc transformé en **la plus grande valeur du domaine**, comparée à un minorant —
c'est-à-dire en le **meilleur score possible**. Un montage sans rien à mesurer était déclaré
excellent par l'assertion qui portait le résultat publié.

Le motif n'a rien de spécifique à `NaN` ni à Rust. Toute étape placée **après** une mesure et conçue
pour « présenter proprement » — saturation, valeur par défaut, `unwrap_or`, `coalesce`, `try/catch`
qui rend une constante, arrondi qui ramène dans une plage — travaille sur un canal qui transporte
aussi les **refus**, et les traite comme des valeurs.

**Réflexe** : pour toute mesure qui peut refuser, écrire le refus dans un canal que la mise en forme
**ne peut pas** traverser — un type somme, une exception, une valeur qui reste manifestement absente.
Et si le refus voyage dans le même canal que les valeurs, vérifier **chaque** transformation en aval
en lui donnant le refus à manger. La question tient en une ligne : *que devient mon refus s'il passe
là-dedans ?*

## L162 — Corriger une mesure écrite deux fois ne corrige rien tant qu'on n'a pas trouvé la seconde

*(S42)* La même régression était écrite à deux endroits du même fichier : en ligne dans le cas, et
dans une fonction extraite une session plus tôt **précisément pour être testable**. J'ai ajouté le
refus dans la fonction extraite, relancé l'essai — et le cas continuait de déclarer le néant
conforme. L'assertion passait par l'autre copie.

L'extraction de la session précédente était bonne et n'était pas allée au bout : elle avait créé la
fonction et **branché le tableau de diagnostic dessus**, en laissant l'assertion sur le code
d'origine. Rien ne le signalait — les deux copies donnaient les mêmes nombres, ce qui est exactement
la condition pour qu'un duplicata survive.

**Réflexe** : quand on extrait une fonction pour la rendre testable, la seule fin acceptable est
qu'il **ne reste aucun appelant de l'ancien code** — vérifié en le supprimant, pas en le relisant. Et
quand on corrige une mesure, chercher d'abord *combien de fois est-elle écrite ?* avant de chercher
*où est le défaut ?* Une correction validée par un test qui passe par la mauvaise copie est pire
qu'une absence de correction : elle est écrite, elle est relue, et elle ne fait rien.

## L163 — Une valeur de repli prise dans le domaine nominal ne sera jamais suspectée

*(S43)* Un estimateur d'ordre de convergence rendait `1.0` chaque fois qu'il n'avait rien à mesurer :
moins de trois points, valeurs toutes nulles, et surtout **deux mesures successives identiques** —
c'est-à-dire le cas où le solveur ne converge pas, le résultat le plus important que ce contrôle
puisse produire.

`1.0` était **l'ordre nominal du schéma**. Un contrôle en aval signalait tout ordre hors de
`[0,3 ; 3,0]` ; il ne pouvait pas broncher. Le repli n'était pas mal signalé : il était **invisible
par construction**, parce que la valeur choisie pour « ne rien dire » était celle qui dit *tout va
bien*.

Comparer avec le cas voisin, trouvé une session plus tôt : un autre repli valait `10⁶`, hors du
domaine plausible. Il était tout aussi faux, et il **finit par se faire remarquer** — un lecteur voit
un nombre rond et démesuré. Un repli de `1.0` au milieu des ordres attendus ne se fera jamais
remarquer.

**Réflexe** : toute valeur de repli — `unwrap_or`, `else` d'un test de validité, valeur par défaut,
constante de secours — doit être choisie **hors du domaine des valeurs valides**, ou ne pas exister
du tout. La question à poser est : *si cette valeur apparaît dans un rapport, est-ce que quelqu'un
peut la distinguer d'une vraie mesure ?* Si la réponse est non, la seule issue correcte est de mettre
le refus dans le **type** — `Option`, `Result`, une variante d'énumération — pour que l'appelant ne
puisse pas ne pas le voir.

## L164 — Un audit vérifie ce qu'un contrôle fait, rarement ce qu'il fait quand il n'a rien à faire

*(S43)* Un audit de garde-fous avait examiné celui-ci, l'avait trouvé défaillant, et avait corrigé son
bornage — le seul des dix qui masquait au lieu de refuser. **Le défaut suivant était sur la ligne
juste au-dessus**, et il a survécu neuf sessions.

Les deux tests écrits par cet audit donnaient au contrôle une série **absurde** et une série
**saine**. Aucun ne lui donnait une série **vide de l'objet qu'il mesure**. Ce n'est pas un oubli :
un audit se construit à partir de ce que le contrôle est censé attraper, et la liste vient de
l'intention de son auteur. Le cas « il n'y a rien à mesurer » n'est dans l'intention de personne — il
n'est le but d'aucune ligne de code.

**Réflexe** : à tout contrôle, poser **trois** cas et non deux — celui qu'il doit refuser, le témoin
qu'il ne doit pas refuser, et **l'entrée vide de ce qu'il examine** : liste sans élément, mesure sans
signal, série sans variation, fichier sans ligne. Le troisième est celui qu'on n'écrit jamais
spontanément, et c'est souvent celui qui révèle que le contrôle rend une valeur au lieu de refuser.

## L165 — Quand la grandeur mesurée est un écart, zéro est son meilleur point

*(S44)* Trois défauts en trois sessions, trois écritures différentes, une seule cause. Une mesure qui
refusait — `NaN` — traversait une opération de mise en forme et en ressortait comme un résultat :
`min(10⁶)` en a fait le plus grand nombre du domaine, `else { 1.0 }` la valeur nominale, et
`max(0, seuil − mesure)` **zéro**, c'est-à-dire le succès parfait d'un déficit.

**La gravité croît et la visibilité décroît dans le même ordre.** Une valeur hors du domaine
plausible finit par se faire remarquer ; une valeur au milieu du domaine nominal, jamais ; une valeur
au **meilleur point** du domaine est exactement ce qu'on espère lire, et personne ne la
questionnera.

Le motif ne tient pas à l'opération mais à la **grandeur**. Dès qu'on mesure un écart, une erreur,
une dérive, un déficit, un taux de perte — **zéro est le meilleur résultat possible**, et il est dans
le domaine des valeurs valides. Toute opération capable de produire zéro à partir d'un refus
transforme donc ce refus en succès parfait : `unwrap_or(0.0)`, `max(0.0)`, `saturating_sub`, une
différence de deux valeurs par défaut identiques.

**Réflexe** : quand la grandeur est un écart, **aucune valeur de repli n'est acceptable** — le refus
va dans le type. Et pour les autres grandeurs, la question à poser est *où tombe mon repli dans le
domaine des valeurs valides ?* S'il tombe dedans, il est invisible ; s'il tombe sur le meilleur
point, il est pire qu'une absence de contrôle.

## L166 — Un mécanisme de repli ne s'inspecte jamais seul : c'est l'aval qu'il faut suivre

*(S44)* Un inventaire des valeurs de repli a été fait pour cesser de les trouver par hasard. Il en a
recensé quarante-neuf, dont **treize irréprochables** : elles rendent `NaN`, qui échoue toute
comparaison et rend rouge tout contrôle qui le reçoit.

**Ces treize ont produit les trois défauts des trois dernières sessions.** Pas parce qu'elles étaient
mauvaises, mais parce que `min`, `max` et une soustraction suivie d'un `max` **avalent tous le
`NaN`** — la valeur de refus la plus solide du langage disparaît sur la ligne suivante.

L'inventaire aurait donc pu être fait entièrement, correctement, et ne rien trouver : les huit replis
que la thèse visait se sont révélés innocents, et les deux fautifs étaient dans une expression que
personne n'aurait classée comme « valeur de repli » — un `max(0, …)` qui sert à borner un déficit.

**Réflexe** : ne jamais auditer un mécanisme de refus à l'endroit où il est produit. **Partir de la
valeur publiée et remonter** — quelle expression la calcule, qu'est-ce qui entre dedans, et que
devient un refus à chaque étape. C'est plus long qu'un `grep`, et c'est la seule chose qui trouve. Le
`grep` sert à borner le travail, pas à le faire.

## L167 — Vérifier aussi que l'assertion existe quand la mesure échoue

*(S45)* Un cas de dispersion ne construisait ses assertions qu'après avoir trouvé des passages
par zéro. Sans onde, il ne produisait ni mesure fausse ni verdict rouge : il produisait une liste
vide. Un audit des seules valeurs publiées ne pouvait voir cette disparition.

**Réflexe :** vérifier ensemble le nombre et l'identité des assertions attendues, puis leur
verdict sur une entrée sans mesure. Une branche qui saute un échantillon peut aussi sélectionner
silencieusement les seules données valides : tester une fenêtre partiellement invalide, pas
seulement une fenêtre entièrement vide. Le témoin valide reste nécessaire : une eau plate est
sans période et pourtant parfaitement recevable pour une mesure de flottabilité statique.

## L168 — Retirer les refus change l'objet qu'on prétend valider

*(S46)* Un contrôle de stabilité retirait les triplets non mesurables puis comparait les ordres
restants. Ceux-ci pouvaient être stables, mais ce n'était plus la famille de grilles fournie.
Le filtrage présentait une propriété du sous-ensemble comme celle de l'ensemble.

**Réflexe :** conserver les absences jusqu'au verdict. Si un sous-ensemble est recevable, sa
sélection et sa portée doivent être explicites. Au bilan, compter chaque objet attendu une
fois : succès, échec ou absence de verdict. Un affichage « indéterminé » sans incrément de
compteur laisse la mesure visible et son absence de conclusion invisible dans la synthèse.

## L169 — La reproductibilité d'un chiffre ne reproduit pas la portée de son verdict

*(S47)* Un résultat importé se reproduisait exactement et restait déclaré vert. Les deux
branches n'appliquaient pourtant pas le même contrat : support singulier et trois grilles
contre cas régulier et stabilité établie. Rejouer les nombres ne pouvait pas résoudre cet écart.

**Réflexe :** comparer aussi les conditions qui autorisent le verdict. Quand elles manquent,
conserver le diagnostic et ses contrôles utiles, mais retirer la validation qu'ils ne prouvent
pas. Une requalification ne doit ni déplacer la mesure ni neutraliser son détecteur de défaut.

## L170 — Raffiner la référence et raffiner l'objet mesuré répondent à deux questions distinctes

*(S48)* Une suite de cinq grilles donnait des ordres non stabilisés. Doubler deux fois les
oracles a rendu les nombres presque invariants sans changer le verdict. La sensibilité à la
référence avait diminué ; le régime asymptotique des grilles étudiées n'avait pas progressé.

**Réflexe :** identifier quel axe de raffinement traite l'incertitude observée. Une référence
plus coûteuse ne rend pas les points de mesure plus fins. Rapporter séparément l'influence
de l'oracle et l'évolution de l'ordre ; leur confusion peut consommer le budget sans tester
l'hypothèse qui bloque la conclusion.

## L171 — Être proche de la valeur attendue ne prouve pas la stabilisation

*(S49)* Les ordres mesurés finissent à 2,0117, avec une dernière variation inférieure
à la tolérance. Pourtant la dérive ne ralentit pas assez pour satisfaire le critère
préétabli. Arrêter le raisonnement à « proche de deux » aurait transformé une tendance
en validation, sans modifier un seul chiffre.

**Réflexe :** séparer la proximité de la cible et la stabilité de la suite. Rapporter
quelle condition manque et conserver le critère déclaré avant mesure. Des fenêtres
chevauchantes peuvent montrer une tendance ; elles ne multiplient pas les preuves indépendantes.

## L172 — Partager le calcul ne demande pas de partager tout le contrat

*(S51)* Deux estimateurs appliquaient Richardson avec des planchers et des catégories
légitimement différents. La formule copiée avait pourtant laissé diverger le refus des
non-finis : corriger un rapport ne corrigeait pas le filtre utilisant la seconde copie.

**Réflexe :** isoler le calcul commun et laisser les choix de seuil, de fenêtre et de statut
aux appelants. Vérifier leurs usages réels ; une ressemblance de formule ne justifie pas
de fusionner des mesures portant sur des supports ou des unités différents.

## L173 — Les coordonnées d'une mesure font partie de ses conditions de validité

*(S53)* Une suite de valeurs avait la forme d'une convergence parfaite, mais les grilles
associées ne suivaient pas le raffinement supposé par la formule. Le validateur les utilisait
comme étiquettes et déclarait la suite stable. Un retrait en amont pouvait produire le même trou.

**Réflexe :** vérifier le support de la mesure avec ses valeurs : tailles, espacements ou
instants selon la formule. Préserver les absences jusqu'au contrôle ; après suppression,
la régularité apparente des valeurs ne permet pas de reconstruire le support perdu.

## L174 — Une valeur répétée ne dit pas que le signal a perdu son sens de variation

*(S55)* Un détecteur de retournement comparait seulement deux différences consécutives.
La quantification introduisait une différence nulle au sommet, qui effaçait la montée avant
la descente. Raffiner la grille a ainsi rendu une oscillation nominale non mesurable.

**Réflexe :** tester les plateaux quand un événement dépend du signe des variations.
Conserver le dernier sens observé jusqu'au retournement, sans fabriquer un événement sur
un plateau final. Rejouer aussi les témoins déjà acceptés : rétablir des observations
manquantes change leur estimation même lorsque leur verdict reste le même.

## L175 — Une extrapolation n'est incertaine qu'en proportion de ce qu'elle fait franchir

*(S57)* S56 s'était arrêtée sur un partage : deux modèles d'extrapolation encadraient
l'erreur mesurée, si bien que **le choix du modèle, et non la mesure, aurait décidé du sort
de la grille**. La session suivante a mesuré, et le pire des deux modèles s'est encore
dégradé — l'exposant local est tombé de 1,72145 à 1,61233. Pourtant la question a cessé
d'être ouverte : le déficit d'admission n'étant plus que de 4,44 %, les quatre exposants
candidats, de 1,5 à 2,0, s'accordent à **0,8 %** sur la taille de référence requise.

Ce ne sont pas les modèles qui se sont améliorés, c'est la **portée** qui a été raccourcie :
d'un facteur 1,5 en taille à un facteur 1,03. La même incertitude d'exposant devient
indifférente quand la distance extrapolée est courte, et décisive quand elle est longue.

**Réflexe :** quand une extrapolation décide d'un verdict, ne pas chercher un meilleur
modèle — **raccourcir sa portée jusqu'à ce que le choix du modèle devienne indifférent**,
quitte à mesurer une fois de plus. Et vérifier la direction de l'erreur des modèles écartés :
ici les deux sous-estimaient, ce qu'aucun encadrement ne laissait prévoir.

## L176 — Un blocage qui dure se vérifie avant de se trancher : son motif peut ne pas exister

*(S58)* Un arbitrage attendait depuis trente-sept sessions, rappelé à chaque fin de session avec
le même argument : un cas de validation exigeait une valeur, le domaine du jeu en appelait une
autre, et l'écart valait deux fois et demie la tolérance du cas. Trancher demandait donc de
choisir entre une mesure et une intention.

**La mesure n'existait pas.** Les trois références du cas étaient construites *avec* la constante
en litige : elles la suivaient, l'écart était structurellement nul, et le cas restait vert pour
n'importe quelle valeur. La tolérance opposée portait sur cet écart, jamais sur la valeur.
L'argument était faux dès son écriture, et il a été recopié dans trois documents.

Deux choses l'ont rendu invisible. Il était **exact sur ses nombres** — les littéraux cités ne se
retrouvent bien qu'avec l'ancienne valeur — et faux seulement sur leur conséquence. Et il avait
été formulé par la session qui posait la constante, donc au moment où personne ne pouvait encore
le contredire par une mesure.

**Réflexe :** devant un point ouvert de longue date, **rejouer son motif avant de le trancher**,
et le rejouer par une exécution, pas par une relecture — ici, recompiler avec l'autre valeur et
regarder les verdicts a coûté quatre minutes. Un blocage ancien a été formulé une fois, tôt, et
recopié depuis ; sa charge de vérité est celle du jour où il a été écrit. Corollaire de forme :
**si un balayage ne fait tomber que des tests unitaires et aucun cas de validation, ce sont les
tests unitaires qui tenaient la propriété** — et le cas de validation est à requalifier, pas à
créditer.

## L177 — Un protocole écrit d'avance est une hypothèse, y compris dans ses remèdes

*(S59)* Une session avait clos son rapport par une consigne de prudence : au-delà du quart
d'heure, découper le calcul en tranches temporelles gardées en mémoire. La consigne était de
bonne foi, cadrée, et elle respectait tous les interdits du dossier — pas de fichier, pas de
changement de paramètre. **Elle aurait détruit la mesure.** L'intégration tronque son dernier pas
pour atterrir sur le temps demandé, donc découper insère des pas qui n'existaient pas, et le
champ change. Trois campagnes de référence devenaient incomparables.

Ce qui rend le cas instructif n'est pas l'erreur, c'est **qui pouvait la voir**. La session qui a
écrit le remède ne subissait pas encore le problème : elle a prescrit sans exécuter. Celle qui
l'a appliqué avait la mesure sous la main, et quatre minutes ont suffi.

**Réflexe :** un protocole reçu — d'une session précédente, d'un document, de soi-même — se lit
comme une **hypothèse à éprouver**, pas comme une contrainte à satisfaire, et ses **remèdes**
méritent la même défiance que ses conclusions. Le premier geste est de chercher ce que le remède
déplace, avant de l'implémenter. Corollaire : en écrivant une consigne pour la session suivante,
**dire ce qu'on n'a pas vérifié** — le coût d'une prescription non éprouvée est payé plus tard,
par quelqu'un qui la croira mesurée.

## L178 — Avant de recalibrer un critère, demander sur quelle grandeur il porte

*(S60)* Un filtre d'admission était soupçonné d'être trop strict : il refusait des mesures, il
avait retardé un verdict de trois sessions, et un angle mort documentait qu'il se réglait sur une
quantité mal choisie. Tout invitait à le recalibrer.

**Les deux mesures ont donné des réponses opposées, et c'est leur désaccord qui a instruit.** Un
essai de refus a montré que le remplaçant envisagé ne refusait pas une contamination flagrante —
donc il ne pouvait pas prendre la place. Une contre-épreuve rétrospective a montré que le filtre
avait bien bloqué un résultat déjà atteignable, à trois centièmes de millième près. Les deux sont
vraies parce qu'elles ne parlent pas de la même grandeur : le critère est **juste pour les
erreurs** et **sans rapport avec l'ordre**, et le dispositif ne les distinguait pas.

**Réflexe :** devant un critère suspect, ne pas commencer par sa valeur. Demander d'abord **ce
qu'il protège** et **ce qu'il commande** — s'il y a plus d'une grandeur publiée, vérifier qu'un
seul critère ne décide pas des deux. Un critère bien réglé pour une grandeur peut être arbitraire
pour la voisine, et le symptôme est trompeur : il ressemble à un mauvais calibrage.

**Et le corollaire, sur la conduite :** un critère qu'on rouvre juste après en avoir obtenu un
résultat favorable doit être instruit à charge. La sortie honnête est souvent de **ne pas
changer** et de nommer l'expérience qui manque — ici, un régime jamais observé, que sept minutes
de calcul suffiront à atteindre.

## L179 — Avant de mesurer une frontière, chercher si elle se calcule

*(S61)* Quatre campagnes, près d'une heure de calcul et quatre sessions ont cherché quelle taille
d'oracle admettrait une grille. La réponse tenait dans un rapport d'entiers : les deux quantités
comparées — l'erreur d'une grille et l'écart de deux oracles — ont **la même origine**, l'erreur
du schéma, et leur rapport ne dépend que de `oracle/grille`, au carré. Le point de bascule était
calculable dès la première campagne.

Ce qui a caché la réponse est le mot **empirique**. Le filtre était décrit comme « un indicateur
empirique, sans borne prouvée » — ce qui est vrai de ce qu'il **borne**, et faux de ce qu'il
**exige**. La première qualité, honnêtement écrite, a dispensé quatre sessions d'examiner la
seconde.

**Réflexe :** quand un critère fait tâtonner, écrire son inégalité et y substituer la forme
attendue des grandeurs qu'il compare. Si les deux viennent de la même source, leur rapport se
simplifie et la frontière devient géométrique. **Et se méfier d'un critère qu'on qualifie
d'empirique** : la mention décrit souvent sa garantie, pas son contenu.

**Corollaire, payé dans la même session** : le calcul de dimensionnement a d'abord utilisé un
exposant mesuré entre deux tailles voisines, extrapolé sur cinq décades — il annonçait 72 heures
là où la réponse était « impossible ». **Un exposant local n'est pas une loi**, et c'est L175 que
la session précédente venait d'écrire.

## L180 — Un balayage qui ne déplace rien est un résultat, pas un échec de manipulation

*(S62)* Pour savoir de quoi dépendait une mesure statistique, la session a fait varier la taille de
sa fenêtre dans le scénario : **un facteur 16, et l'écart n'a pas bougé d'un chiffre**. La réaction
naturelle est de se croire maladroit. C'était la mesure la plus informative de la séance : le
paramètre du scénario n'atteignait pas le code, et la vraie fenêtre était un littéral dans
`main.rs`. Un balayage sans effet **prouve une absence de lien**, et cette absence est parfois le
défaut cherché.

**Et le corollaire, sur ce qu'un test a le droit de figer.** Le premier test écrit dans la foulée
figeait le rapport mesure/référence à sa valeur nominale, `0,914723`. Il a échoué aussitôt : dans
un montage voisin — autre instant, autre graine — il vaut `0,688`. Le chiffre n'était pas une
propriété du système mais **de la réalisation échantillonnée**. Un test qui l'aurait accepté aurait
figé une mer particulière en croyant figer une loi.

**Réflexe :** devant un chiffre stable, demander *stable sur quoi ?* avant d'en faire une
référence. Et quand un balayage ne déplace rien, ne pas recommencer plus fort — **chercher le
chemin par lequel le paramètre est censé arriver**, et vérifier qu'il existe.

## L181 — « Pas encore fait » et « ne peut pas être fait » n'envoient pas au même endroit

*(S63)* Un dossier de préparation listait cinq préalables au banc décisif du projet. Quatre étaient
levés depuis quarante sessions sans que rien ne l'ait dit ; le cinquième était marqué **« non
exécuté »**. Il était en réalité **inexécutable** : le cas demande la mesure d'une erreur de
célérité en fonction de la longueur d'onde, et les deux véhicules disponibles sont non dispersifs
— cette erreur n'existe pas chez eux. Le milieu à dispersion exacte écrit plus tard ne convient pas
davantage, pour la raison inverse : son erreur est nulle par construction.

Les deux formulations coûtent le même nombre de mots. **« Non exécuté » invite à exécuter, et
promet donc du travail de routine. « Inexécutable » dit qu'il manque une pièce de conception**, et
envoie la session vers un tout autre geste. Une ligne de tableau mal qualifiée peut ainsi cacher,
pendant des années, la seule vraie question d'un dossier.

**Réflexe :** dans un tableau d'état, distinguer *pas encore fait*, *inexécutable en l'état* et
*sans objet*, et écrire pour la seconde **ce qui manque**. Et se méfier des états les plus anciens
d'un dossier : ce sont ceux qu'on relit le moins, parce qu'ils ont l'air acquis.

## L182 — Une raison de ne pas faire quelque chose se vérifie comme un résultat

*(S64)* Une action différait un changement en avançant deux motifs techniques : *il faut d'abord
corriger la sommation*, et *le coût serait multiplié par 64*. Les deux étaient précis, plausibles,
et **faux**. Le `NaN` invoqué ne venait pas de la sommation mais d'une limite de portée du repère
local ; et le facteur 64, exact sur la mesure isolée, valait **+6 %** sur le mode complet, où cette
mesure ne pèse rien.

Le motif d'un refus reçoit moins d'examen qu'une affirmation positive, parce qu'il ne débouche sur
rien qu'on puisse vérifier tout de suite : on ne fait pas la chose, donc on ne voit pas que la
raison était fausse. Une prescription positive finit par être exécutée et se corrige ; **une raison
de s'abstenir peut survivre indéfiniment.**

**Réflexe :** quand une session antérieure explique pourquoi elle n'a pas fait quelque chose,
traiter ses motifs comme des mesures à refaire, et les refaire **avant** d'accepter le report. Les
deux d'ici ont coûté quatre minutes à démentir.

**Corollaire, vérifié trois sessions de suite** : un balayage qui ne déplace rien désigne un
paramètre qui n'atteint pas le code — la fenêtre en S62, la graine en S64. Voir **L180**.


## L183 — Une propriété d’ensemble ne devient pas une égalité entre deux fenêtres

*(S66)* Un contrôle attribuait la variation de variance spatiale à la perte de précision.
Une référence plus précise rendait le même refus. Les termes croisés entre composantes,
qui ne disparaissent pas sur une petite fenêtre finie, expliquaient presque tout l’écart.
La mer statistiquement homogène ne promet pas la même variance sur chaque portion observée.

**Réflexe :** avant de transformer une propriété statistique en assertion locale, nommer le
support sur lequel elle est vraie : ensemble, limite de fenêtre, durée ou réalisation.
Décomposer le résidu et modifier le mécanisme suspecté seul. Un contrôle sensible à un défaut
peut aussi réagir à autre chose ; son nom ne suffit pas à attribuer la cause d’un refus.

## L184 — Le support d’un second moment dépend aussi des différences de fréquences

*(S67)* Le domaine couvrait largement les ondes individuelles, mais la variance gardait 6,6 %
d’écart. Densifier le spectre sans élargir ses bornes rapprochait les composantes : leurs
battements atteignaient 20 km sur une fenêtre de 3 km. Les termes croisés ont été mesurés,
ils expliquaient le résidu ; raffiner les points ne pouvait pas les faire disparaître.

**Réflexe :** pour dimensionner une moyenne quadratique, examiner les fréquences de ses
produits et pas seulement celles du signal. Séparer résolution des oscillations et étendue
nécessaire pour moyenner les battements. Une grille convergée en pas peut rester trop courte.

## L185 — Une dépendance de budget ne détermine pas un ordre de construction

*(S71)* Deux composants consomment une enveloppe commune ; cela impose de mesurer leur coût
ensemble avant réception. Cela ne prouve pas que le premier doive être entièrement calibré
avant que le second existe. Nommer ce qui traverse la dépendance : donnée, signature,
ressource ou preuve. Un préalable qui ne fournit aucune entrée nécessaire peut être un jalon
ultérieur. Vérifier aussi qu’un banc annoncé exécutable ne possède pas de volets encore absents.

## L186 — Une identité commune exige un producteur commun ou une corrélation

*(S72)* Deux producteurs indépendants ne peuvent pas partager un compteur futur par convention.
Avant de promettre une déduplication ou une réconciliation par id, décrire qui crée cet id,
quand chaque participant le connaît, et comment une prédiction se rattache au fait confirmé.
Les octets et leur tri peuvent être parfaits alors que la correspondance reste impossible.

## L187 — Refuser sans corruption ne préserve pas la complétude

*(S73)* Un conteneur plein peut conserver exactement son état et perdre une information requise
pour le résultat global. Tester aussi ce que l’appelant pourra encore prétendre après le refus :
état local cohérent et état complet sont deux propriétés distinctes. La récupération et la
validité publiée doivent porter cette différence, pas seulement le code de retour.

## L188 — Une preuve négative locale ne certifie pas une livraison globale

*(S74)* Ne pas avoir observé de perte dans un pool ne prouve pas que tous les événements lui
sont parvenus. Nommer un indicateur selon ce qu’il mesure effectivement : perte connue,
plutôt que complet. Conserver cette portée dans le format sauvegardé ; persister un booléen
ambigu transforme une observation locale en prétendue garantie pour la session suivante.

## L189 — Deux preuves physiques ne couvrent pas le même mouvement

*(S75)* Une énergie conservée et la bonne fréquence d’un mode peuvent coexister avec une
source répétée artificiellement et un mauvais transport d’enveloppe. Nommer séparément phase,
énergie, déplacement du paquet et validité du support spatial. Leurs contrôles doivent rester
séparés ; la facilité de vérifier les deux premiers ne reçoit pas les deux autres par extension.

## L190 — Une identité intégrée ne fournit pas une densité locale

*(S76)* Deux intégrandes peuvent avoir la même intégrale sans représenter la même répartition.
Une forme de bord du bilan énergétique est exacte pour le total mais peut devenir négative
localement. Avant de pondérer une position ou un rayon, vérifier la positivité et le sens local
du poids. Changer la question de total à localisation impose de réexaminer la mesure.

## L191 — Un domaine calculable n’est pas un domaine reçu

*(S77)* Borner la variation de phase entre deux nœuds évite une sous-résolution évidente.
Cela ne borne pas automatiquement l’erreur totale : amplitude, troncature et oscillations
peuvent encore intervenir. Séparer contrôles d’admission numériques et réception physique
mesurée ; ne pas transformer le premier en certificat du second dans la documentation.

## L192 — Une grandeur conservée peut quitter son domaine d’observation

*(S78)* Le disque de 8 m semblait perdre de l’énergie ; celui de 20 m la récupérait. Avant de
corriger une amplitude ou un bilan, varier séparément l’étendue et le pas de mesure. Un déficit
par transport réclame une frontière de flux, pas une source compensatrice inventée.

## L193 — Une identité vectorielle ne se teste pas sur une seule composante

*(S79)* La correction orbitale horizontale de S21 avait laissé un signe vertical erroné.
Lorsqu’une grandeur est vectorielle, nommer la loi qui contrôle chaque composante. Une
régression sur la composante historiquement fautive ne protège pas les autres directions.

## L194 — Le stockage de préparation n’est pas une publication

*(S80)* Une opération peut laisser un préfixe dans son espace temporaire tout en restant
transactionnelle pour le consommateur. Nommer clairement les deux stockages et leurs droits
permet de refuser sans copie de secours cachée. Le succès est le point de publication ; les
lecteurs ne doivent jamais accéder au temporaire pour gagner une copie.

## L195 — Mesurer le chemin complet avant d’optimiser sa structure

*(S81)* La préparation coûtait des microsecondes, les lots des dizaines de millisecondes.
Une refonte du journal aurait optimisé le mauvais étage. Mesurer séparément préparation,
base et composition puis attaquer les invariants recalculés dans la boucle dominante.
Conserver les sorties à bits identiques permet d’isoler une optimisation de calcul du modèle.

## L196 — La borne d’interpolation ne borne pas à elle seule le programme

*(S82)* Le reste Hermite suppose valeurs et dérivées nodales exactes. Une table quantifiée
et une évaluation flottante ajoutent leurs erreurs. Publier séparément borne analytique du
schéma et erreur mesurée du code ; une grille dense n’est pas une preuve de tous les arguments.

## L197 — Une étiquette ne prouve pas la provenance d’un résultat

*(S83)* Ajouter un instant à un tampon calculé ailleurs ne garantit pas son origine. Quand
c’est possible, produire les grandeurs couplées depuis une même requête plutôt que demander
au consommateur de certifier leur cohérence après coup. Déclarer séparément les associations
hôte que le système ne peut encore vérifier, notamment les transformations géométriques.

## L198 — Une échéance de représentation ne termine pas le phénomène

*(S84)* Une durée de validité borne ce que le calcul sait produire, pas la durée physique
ni la conservation des faits permettant de le reconstruire. Séparer les trois horloges avant
une purge ou un renouvellement. Reconstruire depuis la cause originelle évite le redémarrage
artificiel de phase ; changer de résolution exige en plus de mesurer la discontinuité.

## L199 — Séparer le résultat d'une tentative de l'état du service conservé

*(S85)* Un renouvellement refusé peut laisser une version encore utilisable ; la même version
peut ensuite expirer sans nouvelle tentative. Retourner l'erreur de l'opération et calculer
séparément la validité à l'instant demandé évite aussi bien le faux arrêt que le faux succès.

## L200 — Le retour transactionnel ne doit pas effacer une entrée reçue

*(S86)* Annuler un calcul candidat protège la cohérence des données publiées, mais peut cacher
une commande non appliquée. Conserver cette commande et distinguer version publiée et vue courante.
La sauvegarde doit inclure cette attente : restaurer seulement les données publiées ferait disparaître
la raison pour laquelle le service refusait de répondre.

## L201 — Restaurer un blocage avant de tenter de le résoudre

*(S87)* Une cible plus capable peut accepter une opération autrefois refusée. La restauration
ne doit pourtant pas confondre capacité nouvelle et opération déjà exécutée : rétablir d'abord
les faits publiés et leur attente, puis déclencher explicitement la nouvelle tentative.
Cela conserve un point de comparaison avant reprise et évite un acquittement implicite.

## L202 — Mesurer la reprise sans la confondre avec le stockage

*(S88)* Un parcours complet peut vérifier une restauration depuis des octets tout en laissant
hors champ la durabilité disque. Chronométrer séparément sauvegarde mémoire, reconstruction et
requêtes ; annoncer les exclusions évite de transformer un coût de codec en latence de redémarrage.
Une référence directe des faits vérifie le cycle de vie sans réutiliser son chemin de sauvegarde.

## L203 — Une décomposition linéaire du champ ne rend pas son énergie additive

*(S89)* Les réponses de segments de forçage s'additionnent, mais leur énergie quadratique
contient des interférences. Valider la segmentation sur le champ, puis le travail sur la vitesse
totale. Normaliser chaque segment indépendamment rendrait le résultat dépendant du découpage
de la trajectoire, alors que le mouvement physique n'a pas changé.

## L204 — Un bilan fermé peut rester spatialement imprécis

*(S90)* Travail intégré et énergie concordent sur une quadrature grossière parce qu'ils
partagent la même approximation spectrale. Cette identité reçoit la comptabilité, pas la
résolution. La comparer à un travail spatial indépendant puis raffiner séparément le spectre
évite de confondre conservation et justesse du champ.

## L205 — Une borne d'accès ne certifie pas la précision à l'intérieur

*(S92)* Un rectangle validé peut fermer les requêtes hors domaine sans prouver le calcul
sur ce rectangle. Garder distincts le contrat d'accès, les échantillons de réception et une
éventuelle borne continue. Un constructeur qui vérifie la forme des bornes ne doit pas être
nommé ou présenté comme un certificat numérique.

## L206 — Une erreur absolue faible peut effacer un démarrage

*(S95)* La précision absolue d'une fonction trigonométrique ne garantit pas sa précision
relative près de zéro. Une soustraction dans la réduction d'angle peut altérer le premier
mouvement tout en passant une réception globale. Vérifier séparément le développement aux
petits temps, avec un signal non nul, et exploiter les symétries avant les soustractions.

## L207 — Deux mesures à des positions différentes ne se comparent pas

*(S118)* Un écart de coût de 15 à 28 % entre deux voies du même calcul venait entièrement de
l'ordre des blocs de mesure : le premier bloc d'un processus est lent, les suivants ne le sont
pas. Deux explications plausibles s'offraient d'abord — une empreinte mémoire doublée, un
paramètre qui variait d'un côté seulement — et **toutes deux étaient fausses**. Ce qui a tranché
n'est pas le raisonnement mais un troisième témoin : le même code, mesuré en dernier.

Avant de croire un écart entre deux voies, les replacer symétriquement. Un témoin qui reproduit
la cause soupçonnée coûte quelques lignes ; une explication plausible non testée entre dans un
livrable et y reste. Vaut au-delà du temps de calcul : dès que deux mesures diffèrent par autre
chose que ce qu'on croit comparer.

## L208 — Un prédicat ponctuel ne révèle jamais un ensemble vide

*(S119)* Une fonction qui répond « cet instant est-il servable ? » peut être exacte à chaque
date et ne jamais dire qu'**aucune** date ne l'est. Sur un montage dont la fenêtre utile est
vide, la cause change simplement de côté selon la date demandée : trop tôt d'un côté, trop tard
de l'autre, jamais « impossible ». L'hôte qui n'interroge que des dates ne l'apprend pas.

Quand une propriété globale existe — un domaine vide, une intersection vide, un ensemble sans
solution — elle demande sa propre fonction, à côté du prédicat ponctuel. Les deux ne sont pas
redondantes : l'une répond sur un point, l'autre sur l'ensemble, et la seconde ne se déduit pas
d'un nombre fini d'appels à la première. Vaut pour toute API de validation, pas seulement pour
le temps.

## L209 — Mesurer le coût de l'exactitude avant de choisir une approximation

*(S120)* La session ouvrait sur une commande explicite — produire des *bornes* pour ce qui était
évalué point par point. L'inventaire préalable a montré que deux des trois conditions n'avaient
besoin d'aucune borne : leurs prédicats exacts sont des comparaisons, quatre ordres de grandeur
moins chers que l'évaluation qu'ils précèdent. Une boîte englobante aurait été strictement moins
informative, pour le même prix.

Une approximation se justifie par le coût de l'exactitude, et ce coût se constate — il ne se
suppose pas depuis le vocabulaire de la question posée. Lire ce que fait réellement le code avant
de décider de la forme de la réponse coûte quelques minutes ; s'être engagé sur la mauvaise forme
coûte la session. Vaut aussi quand la commande vient d'une session précédente : elle a nommé un
problème, pas nécessairement la forme de sa solution.

## L210 — Vérifier qu'un défaut est atteignable avant de le corriger

*(S121)* Un angle mort désignait une confusion réelle entre deux causes d'erreur, à un endroit
précis du code. La confusion existait ; **l'endroit était faux**. Une sonde de quelques dizaines
de lignes — 674 000 échantillons sur toute la plage représentable des paramètres — a montré que
le bloc incriminé n'a aucune entrée qui l'atteigne, et a désigné au passage le site voisin où le
même défaut est, lui, atteignable et démontrable.

Corriger le premier aurait produit du code juste, testé par rien, et laissé le vrai défaut en
place — avec la satisfaction d'avoir refermé une entrée du registre. **Avant de corriger, écrire
ce qui atteint le défaut.** Si rien ne l'atteint, ce n'est pas la correction qui est en cause,
c'est la cible. Le corollaire vaut pour ce qu'on écrit ensuite : une sonde qui ne trouve pas de
contre-exemple mesure une marge, elle ne démontre pas une impossibilité.

## L211 — Une taxonomie d'erreurs se teste par surjection

*(S122)* Neuf noms de refus ont remplacé deux fourre-tout. Le test qui compte n'est pas que le
code compile, ni que les refus existants passent : c'est que **chaque nom soit produit par au
moins une entrée**, et que le paramètre nommé soit bien celui qu'il faut changer. Un nom
qu'aucune entrée n'atteint est une promesse vide ; un nom atteignable mais trompeur est pire,
parce qu'il envoie l'appelant modifier la mauvaise grandeur — indéfiniment, s'il boucle.

C'est ce test qui a montré qu'un des nouveaux noms mentait encore : le refus attribué à
l'énergie venait en réalité d'une intégrale qui ne dépend que de la longueur d'onde. La
relecture ne l'avait pas vu, deux fois de suite.

Corollaire pour les conditions composées : quand un refus tient à une relation entre plusieurs
paramètres, le nommer d'après un seul est un mensonge commode. Nommer la relation — portée,
régime, résolution — coûte un mot de plus et reste vrai des deux côtés. Voir [[L210]], qui dit
la même chose de la cible d'une correction : ce qui se vérifie par relecture ne se vérifie pas.

## L212 — Un paramètre non calibré n'interdit pas de conclure : mesurer la sensibilité

*(S123)* La confrontation demandée semblait bloquée par un inconnu : la longueur d'onde d'un
impact n'est reliée à aucune propriété de l'objet, et sans ce lien, dire qu'un cas « entre dans
le domaine » n'a pas de sens. Attendre la calibration aurait reporté la session entière.

La sortie n'est pas de choisir une valeur — ce serait inventer un nombre sans provenance — mais
de **balayer toute la plage plausible du paramètre et de regarder si le verdict change**. Ici,
un facteur 2π : sur onze cas, trois sont refusés pour toute valeur, un seul passe pour toutes,
et la conclusion tient sans que le paramètre soit connu. Le résultat est plus solide qu'avec une
valeur choisie, puisqu'il ne repose sur aucune.

La règle : quand un paramètre manque, chercher d'abord si la question posée en dépend
réellement. Souvent la réponse est non sur toute la plage utile, et l'inconnu devient une note
de bas de page au lieu d'un blocage. Quand la réponse est oui, on a au moins appris que le
paramètre est décisif — ce qui est aussi un résultat, et qui justifie de le calibrer.

## L213 — Un facteur sur une borne n'est un facteur sur le résultat que si elle est seule active

*(S124)* Reculer la portée de Bessel d'un facteur 32 devait multiplier par 32 la portée d'un
champ d'impact. Le gain réel va de zéro à +82 % : dès que la première borne recule, une seconde
— jusque-là masquée — devient active et reprend la main. Trois cas sur onze n'ont rien gagné du
tout, parce que la seconde borne mordait déjà avant.

Avant d'annoncer le bénéfice d'un déblocage, chercher **quelle borne prend le relais**. La
mesure coûte une dichotomie ; l'annonce non vérifiée entre dans une décision et il faut ensuite
la corriger par une note datée. Le corollaire est plus utile encore : une fois la seconde borne
identifiée, on sait si le déblocage vaut la peine — ici, les deux bornes devaient tomber
ensemble, et aucune des deux levées séparément ne donnait presque rien. Voir [[L210]] et
[[L211]] : ce qui se vérifie par relecture ne se vérifie pas.

## L214 — Séparer le coût du réglage et celui de l'usage qu'il débloque

*(S125)* Quadrupler le nombre de modes multiplie le coût par environ quatre sur les mêmes
points. Mais les points rendus accessibles passent davantage par une autre branche du noyau :
le montage étendu coûte environ neuf fois le montage initial. Mesurer seulement à travail
constant aurait correctement mesuré le réglage et sous-estimé son usage.

Comparer les variantes sur une charge commune pour isoler le paramètre, puis mesurer la
charge nouvelle qu'il permet. Publier les deux facteurs avec leurs domaines ; aucun des deux
ne remplace l'autre. Un coût marginal faible devant un ancien scénario dominant n'est pas
une réserve de budget garantie pour tous les usages.

## L215 — Recevoir une portée demande un temps de transport cohérent

*(S126)* Un champ comparé à un oracle à128 mètres passait largement son critère. Mais
l'horizon de4 secondes n'envoyait le groupe le plus rapide qu'à7 mètres : le banc mesurait
les queues du champ, pas un paquet ayant parcouru la distance promise. Les erreurs relatives
locales étaient grandes sur des valeurs infimes, sans invalider l'exactitude absolue reçue.

Avant de déclarer une portée utile, confronter distance, vitesse caractéristique et horizon
numérique. Un banc spatial peut être exact tout en ne contenant jamais le phénomène de
transport qu'on lui attribue. Quand un garde couple rayon et âge, attendre davantage n'est
pas une correction gratuite : il faut dimensionner les deux ensemble.

## L216 — Un paramètre « à calibrer » peut être déjà déterminé par le modèle

*(S136)* `α`, le rapport entre longueur d'onde et taille d'objet, était étiqueté « à calibrer »
depuis ADR-083, et trois sessions l'avaient traité comme un paramètre libre — en balayant [1, 2π]
faute de mieux. Il ne l'était pas : la forme spatiale que le modèle engendre a un rayon
caractéristique proportionnel à λ, et α n'est que le rapport qui le fait coïncider avec l'objet.
Le mesurer a coûté une sonde de quarante lignes et a réduit un intervalle de facteur 6 à un
facteur 1,8 — en déplaçant au passage les conclusions d'une session entière.

L'étiquette « à calibrer » est honnête quand elle recouvre une grandeur **physique** que le
modèle ne contient pas. Elle est trompeuse quand la grandeur est une **conséquence** du modèle
qu'on n'a pas encore calculée. Avant de renvoyer un nombre à un banc, se demander lequel des deux
cas on a : le banc mesure le monde, il ne mesure pas ce que notre propre modèle affirme déjà.

## L217 — Un renvoi se vérifie, surtout quand il se recopie

*(S137)* Trois décisions successives ont renvoyé la calibration de la source d'impact « au banc
B2 ». Aucune n'avait ouvert B2, dont les métriques mesurent tout autre chose — la technologie de
propagation, pas ce qu'un objet émet. Le renvoi était faux à sa naissance, et les deux suivantes
l'ont recopié parce qu'il était déjà écrit.

Un renvoi non vérifié est pire qu'un manque déclaré : il **ferme** la question au lieu de la
laisser ouverte. Personne ne cherche ce qui est déjà attribué. Ici, le trou a survécu soixante
sessions sous une étiquette qui le désignait comme traité.

Deux gestes en découlent. Quand on écrit un renvoi, ouvrir la cible et vérifier qu'elle porte
bien ce qu'on lui confie. Quand on en recopie un, se rappeler qu'il n'a peut-être jamais été
vérifié — l'ancienneté d'une phrase n'est pas une preuve. Voir [[L137]] : le même dépôt sait déjà
qu'une chose écrite deux fois finit par diverger ; une chose renvoyée trois fois n'est pas
davantage garantie.

## L218 — Corriger une occurrence n'est pas corriger l'erreur

*(S138)* S137 a trouvé un renvoi faux, l'a corrigé dans les trois décisions qu'elle avait sous
les yeux, et a conclu. L'audit de la session suivante en a trouvé **six** : deux ADR portaient la
même erreur sans avoir été ouverts, dont un écrit par un autre agent trois sessions plus tôt.

Le défaut de méthode est net et se répare en une commande. Quand une erreur est trouvée par
hasard — dans un document qu'on lisait pour autre chose — la première question n'est pas
« comment la corriger » mais « **combien de fois figure-t-elle** ». Une recherche de texte sur la
formulation fautive coûte quelques secondes et transforme une correction ponctuelle en
correction réelle.

Le corollaire vaut pour ce qu'on écrit ensuite : annoncer « trois ADR portaient ce renvoi » sans
avoir compté, c'est publier un décompte faux — et le dépôt sait déjà ce que valent les décomptes
non vérifiés (S07, S10). Voir [[L217]] : le renvoi non vérifié ferme la question ; le correctif
non cherché la rouvre à moitié.

## L219 — Un qualificatif n'est pas un nombre

*(S139)* « Borne conservative » est écrit deux fois dans le corpus à propos de `slope_bound` —
ADR-058 §22 et ADR-062 §50 — et les deux fois c'est exact. Pendant soixante-deux sessions,
personne n'a mesuré **de combien**. Le facteur vaut 1,7950713, et l'obtenir a demandé une sonde
de deux cents lignes et un quart d'heure.

Le mécanisme est celui de [[L216]] et [[L217]], sous une troisième forme. Une étiquette juste
— « à calibrer », « traité en Sxx », « conservative » — **rend la question présentable**, donc
close : elle a l'air d'une réponse partielle alors qu'elle est une absence de réponse. Les trois
étiquettes ont chacune coûté une soixantaine de sessions dans ce dépôt.

Le test est mécanique et tient en une question : *si ce qualificatif était remplacé par le nombre
qu'il résume, saurais-je l'écrire ?* Si non, la question est ouverte, et le document doit le dire
comme tel plutôt que la qualifier.

## L220 — Un budget qui additionne des natures différentes ne peut pas porter un seuil dérivé

*(S139)* Le budget de pente d'ADR-080 additionne trois termes : la pente **exacte** du fond, la
borne L1 d'un impact (1,795 fois la pente réelle) et l'enveloppe L1 d'une pression (facteur
inconnu, non constant). Le tout est comparé à un seul `max_slope`.

Il n'existe alors **aucune valeur physiquement juste** : le seuil qui rend justice aux impacts
autorise au fond une cambrure de 1,8 fois la limite de déferlement, et l'inverse étrangle les
impacts d'un facteur 65 en énergie. Ce n'est pas un défaut de calibration, c'est une propriété de
la somme — et elle explique pourquoi le nombre est resté sans provenance : *il n'y en avait pas à
trouver*.

Généralisable à toute somme qui sert de garde : **avant de chercher le seuil, vérifier que tous
les termes sont la même grandeur.** Une somme de majorants d'inégale finesse est sûre — c'est
pourquoi le défaut est invisible en essais — mais elle n'est plus interprétable, et un seuil
posé dessus ne se dérive de rien. La réparation n'est pas de choisir mieux : c'est de faire
publier à chaque terme la grandeur réelle qu'il majore.

*Post-scriptum S140.* « Faire publier à chaque terme la grandeur réelle qu'il majore » est trop
fort : pour la pression, cette grandeur ne se calcule pas, elle se cherche, et une recherche qui
manque le maximum rend un majorant faux — pire que conservateur. La règle tenable est **la même
grandeur majorée, le meilleur majorant exact de chacun, et la marge résiduelle mesurée**. La
leçon tient, sa prescription se corrige (ADR-095).

## L221 — Un battement se relève, il ne s'écrit pas de mémoire

*(S140)* Les cinq battements de S139 — de 03:05 à 04:25 — étaient **estimés**. L'heure réelle à la
fin de la session était 03:20 : le jeton rendu portait un battement **une heure dans le futur**,
c'est-à-dire exactement ce qui fait croire à une session active et bloque la suivante pendant deux
heures.

Le dépôt savait déjà : `REPRISE.md` porte depuis S118 une note disant qu'un battement estimé avait
dû être corrigé, avec la phrase « le relever, jamais l'écrire de mémoire ». Je l'ai lue en
démarrant S139, et je l'ai refaite dans la même session.

**Ce qui n'a pas marché est le format, pas le contenu.** Une mise en garde posée dans un
commentaire au milieu d'un bloc de données se lit comme du décor. Elle est désormais une **leçon**
et une ligne de la procédure : `date` avant chaque commit d'étape, une commande, deux secondes.
Voir [[L219]] — c'est le même mécanisme qu'un qualificatif à la place d'un nombre : une phrase qui
a l'air de traiter la question sans l'obliger.

## L222 — Décomposer une majoration avant de chercher à la calibrer

*(S140)* Le conservatisme de l'enveloppe de pression valait 3,03. Chercher « le facteur de la
pression » comme on avait trouvé `ρ = 1,795` pour l'impact aurait échoué — il n'y en a pas un,
parce que la majoration en cache **deux, de natures opposées** :

- un facteur de **forme** — normes L1 au lieu d'euclidiennes — borné par 2, indépendant de tout,
  et **éliminable exactement et gratuitement** : deux `hypot` au lieu de deux `abs` ;
- un facteur d'**alignement** — les termes n'atteignent pas leur maximum au même endroit — que
  **rien ne borne**, parce qu'il dépend du domaine choisi par l'appelant et non du modèle.

Généralisable : devant une majoration trop lâche, **la séparer avant de la mesurer**. Chaque
facteur relève d'un traitement différent — le premier se supprime, le second se mesure et
s'annonce. Les confondre mène soit à calibrer un nombre qui n'existe pas, soit à laisser sur la
table un gain de 60 % qui ne coûtait rien. Voir [[L220]], dont cette leçon corrige la
prescription.

## L223 — Le recalcul parallèle est l'instrument d'une migration, pas une redondance

*(S141)* En migrant le budget de pente, j'ai changé quatre sites de bibliothèque et lancé les
essais : **tout est passé au vert**. Puis `examples/receive_mixed` a échoué — il recalcule le
budget *hors* de la bibliothèque, à partir des mêmes grandeurs publiées, et sa formule sommait
encore l'ancienne. Un cinquième site avait été oublié, et c'est ce recalcul-là qui l'a dit.

Un test d'égalité de bits n'aurait rien appris : il dit *qu'un* hachage a bougé, jamais **lequel
des termes** ni **où**. Un recalcul indépendant de la même quantité, lui, désigne le site.

Généralisable : quand une formule partagée change, ce qui protège n'est pas la comparaison de
sorties gelées mais **une seconde implémentation de la même grandeur, écrite ailleurs et de
façon indépendante**. Elle coûte quelques lignes, elle vieillit avec le modèle, et le jour d'une
migration elle vaut une journée de recherche. Voir [[L49]] et [[L167]].

## L224 — Une constante mesurée contre une frontière devient fausse quand la frontière bouge

*(S141)* `K_ENERGIE` avait été obtenue en S136 par dichotomie sur le candidat : le plus grand `E`
que la construction acceptait. C'est une mesure honnête — et elle mesurait **la frontière**, pas
la physique. Quand S141 a déplacé cette frontière d'un facteur `ρ`, la borne d'énergie annoncée
par le générateur est devenue fausse d'un facteur `ρ² = 3,22`, dans le sens conservateur, donc
sans que rien ne casse côté sûreté.

Ce qui l'a rattrapée est un essai qui vérifiait la borne **des deux côtés** — construction juste
en dessous, refus juste au-dessus. Un essai qui n'aurait vérifié que « la borne est respectée »
serait resté vert en annonçant trois fois moins que le vrai.

Deux prescriptions, et la seconde est celle qui économise le plus :

1. **vérifier une borne des deux côtés**, toujours — une borne qu'on ne peut pas dépasser sans
   refus *et* qu'on peut atteindre sans refus ;
2. **écrire la dépendance dans le code, pas dans la valeur.** `K_ENERGIE = 8,891e-4 ·
   SLOPE_L1_RATIO²` suit désormais toute nouvelle mesure du rapport ; `2,865e-3` aurait recommencé
   à mentir à la prochaine décimale. Voir [[L219]] : un nombre sans sa dépendance est un
   qualificatif déguisé.

## L225 — Une dispense accordée pour absence de conséquence tombe le jour où la conséquence apparaît

*(S142)* ADR-082 avait délibérément laissé `ImpactField` de côté : « ses bornes ont la même maladie,
mais il n'est plus le chemin actif, et le renommer sans consommateur ajouterait du travail sans
lecteur ». C'était juste, et bien argumenté.

Puis S141 a migré l'autre champ, et les deux constructeurs ont cessé de dire la même chose de
`max_slope`. La dispense n'est pas devenue fausse : **son motif a disparu**. Le lecteur qui
manquait existait désormais — quiconque lit le paramètre.

Ce qui est généralisable n'est pas « les dispenses expirent », c'est **où regarder** : une
décision de ne rien faire s'appuie sur un état du reste du système, et ce sont les modifications
de cet état-là qui doivent la rouvrir, pas le calendrier. En pratique, quand une migration change
un vocabulaire partagé, **relire les endroits qu'on avait exemptés de ce vocabulaire** — ils sont
peu nombreux, ils sont écrits, et c'est exactement là que l'incohérence s'installe.

Corollaire pour la rédaction : une dispense doit dire **de quoi elle dépend**, pas seulement
qu'elle est accordée. Celle d'ADR-082 le disait — « il n'est plus le chemin actif » — et c'est ce
qui a permis de constater sa péremption en une lecture. Voir [[L219]] et [[L224]] : un motif écrit
vaut mieux qu'une conclusion écrite.

## L226 — Une garde structurelle ne vaut que par les portes qu'elle laisse ouvertes

*(S143)* Pour empêcher qu'un futur champ compare une borne L1 à `max_slope`, la voie qui paraissait
la meilleure était un type : `RealSlope` au lieu d'un `f32` nu, garde à la compilation, rien à
inscrire, rien à se rappeler.

Elle ne garde rien, et le motif tient en une phrase : **l'hôte doit pouvoir en construire un**,
puisque c'est lui qui fournit le milieu. Le constructeur est donc public, et l'auteur pressé
écrira `RealSlope::new(slope)` — trois mots — pour faire compiler sa comparaison fausse. Le type
ne l'arrête pas ; il lui demande de signer, ce qui n'est pas la même chose.

Généralisable, et pas seulement aux types : **toute garde structurelle qui doit rester ouverte à un
usage légitime laisse exactement la même porte à l'usage fautif.** Avant de choisir une garde,
chercher qui a le droit de la contourner — s'il existe un tel acteur, la garde est une convention
déguisée en contrainte.

Ce qui reste alors : rendre la faute **bruyante** plutôt qu'impossible. Deux gardes exécutables
l'ont fait ici pour un centième du coût, et l'une d'elles donne même le facteur manquant dans son
message d'échec. Voir [[L219]] — et se souvenir qu'on ne mesure une garde qu'en la voyant échouer.

## L227 — Une réparation proposée se périme comme un état, et pour la même raison

*(S144)* A208 proposait deux réparations. La première attribuait une cause que la bibliothèque ne
peut pas établir — défaut de rédaction, repérable à la lecture. **La seconde était juste le jour
où elle a été écrite, et sans objet six jours plus tard** : publier le rapport des deux enveloppes
aurait montré le facteur de forme, et la session suivante l'a retiré du calcul.

Le dépôt sait déjà qu'un **état** recopié se périme et qu'il faut le dater (A185, S63). Une
**réparation proposée** est un état déguisé : elle décrit ce qu'il faudrait faire *étant donné le
système tel qu'il est*. Quand le système bouge, elle vieillit sans que rien ne la marque — et elle
vieillit d'autant plus vite qu'elle est fine.

En pratique : une proposition de réparation portée dans un angle mort se **relit avant d'être
exécutée**, jamais ne s'exécute sur sa seule autorité — surtout après plusieurs sessions. Et
l'écrire en disant *de quoi elle dépend* — ici « tant que l'enveloppe additionne les deux facteurs
» — aurait suffi à la voir tomber. Voir [[L225]] : une dispense aussi dépend d'un état ; c'est le
même mécanisme, du côté de l'action au lieu de l'inaction.

## L228 — Un chaînage local propage la proximité, pas l'importance

*(S145)* Ce dépôt enchaîne ses sessions par une ligne « suite Sxxx » que chacune écrit pour la
suivante. Le mécanisme n'a jamais rompu en 144 sessions, et il est la raison pour laquelle un
projet mené par des sessions sans mémoire tient une trajectoire.

Mais il ne propage **que ce que la dernière session a vu**. Un bilan qui recommande quatre choses
n'a aucun porteur : sa recommandation est écrite dans un registre, et la « suite » de la session
suivante parle d'autre chose — non par désaccord, par proximité. Résultat mesuré : deux des quatre
recommandations de S69 sont restées lettre morte pendant soixante-seize sessions, dont la plus
concrète des quatre.

Généralisable à tout dispositif de passation : **ce qui n'est pas dans le canal que le suivant lit
par obligation n'existe pas.** Ajouter un registre ne répare rien — un registre est précisément ce
que personne ne relit. La réparation consiste à faire porter l'intention par le canal obligatoire
lui-même, si étroit soit-il, et à rendre son contournement **explicite** : écarter par écrit, pas
par oubli. Voir [[L55]] — une annonce en prose est une intention, pas une tâche ; ici, même une
tâche n'est rien sans un canal qui la porte.

## L229 — Une mesure sur un tirage unique ne distingue pas un biais d'une dispersion

*(S146)* A187 vivait depuis S64 : « à 256 composantes, `Hs` s'écarte de 6,6 % ». La mesure était
juste, sa robustesse avait été vérifiée — raffinement de l'échantillonnage, bornes du spectre,
amplitude par composante — et deux sessions l'avaient instruite. Ce qui n'avait pas été varié est
**la graine**.

Douze graines par densité donnent une tout autre lecture : aucun biais — la moyenne des écarts
tient dans ±0,42 % — mais une **dispersion qui double** entre 32 et 256 composantes. Les +6,6 %
étaient une réalisation à 3 σ.

Généralisable, et pas seulement aux graines : **avant de chercher la cause d'un écart, vérifier
qu'il est reproductible sur le paramètre le plus arbitraire du montage.** Un écart qui ne survit
pas au changement de tirage n'est pas un défaut à expliquer, c'est une largeur de distribution à
mesurer — et les deux appellent des travaux opposés.

Le coût de l'erreur se lit dans le corpus : A187 a tenu une conclusion prudente pendant quatre-vingts
sessions — « la tolérance ne peut pas descendre sous 7 % tant que la cause est inconnue » — là où
la vraie réponse était que **la tolérance dépend du nombre de composantes**. Voir [[L219]] : le
qualificatif « inexpliqué » a la même propriété qu'un qualificatif chiffré manquant, il rend la
question présentable.

## L230 — Une normalisation peut masquer une perte que les dérivées amplifient

*(S147)* Renormaliser un spectre tronqué retrouve exactement sa variance totale, même lorsque
la bande supprimée portait une part importante de ses vitesses ou de ses pentes. Le contrôle
portant sur la grandeur normalisée ne peut pas détecter cette erreur : il vérifie le réglage.

Avant de recevoir une représentation normalisée, mesurer **avant normalisation** ce qui a été
retranché, puis contrôler les moments qui gouvernent ses consommateurs. Séparer l'erreur de
troncature de l'erreur de discrétisation : augmenter le nombre de points dans une bande ne
récupère jamais ce qui est hors bande. Vaut pour tout signal filtré dont on utilise les dérivées.
## L231 — Une borne en secondes qui vaut exactement 2^n microsecondes n'est pas une durée

*(S155)* Le noyau de pression refusait au-delà de « 16 secondes » depuis soixante sessions, et
ADR-071 disait lui-même que c'était « un périmètre de travail à calibrer par réception ». Personne
ne l'avait calibré, et B2 — qui mesure à soixante secondes — était bloqué par cette valeur.

16 000 000 µs, c'est 2^24 à un pour cent près, et le commentaire d'une fonction voisine parlait
justement de « 24 bits ». **Une grandeur physique dont la valeur est une puissance de deux
déguisée vient de la représentation, pas du phénomène.** C'est un signal lisible à l'œil nu, et il
suffit à décider d'aller mesurer.

La mesure a donné mieux qu'un déblocage : il n'y avait aucun mur à seize secondes — l'erreur croît
continûment, rien ne distingue 16 de 15 ou 17 — **mais l'âge n'était pas gratuit non plus**, ce que
la lecture du code laissait croire puisque la propagation libre est en arithmétique entière. Les
deux moitiés de la réponse étaient fausses. Voir [[A213]] et ADR-106.

Corollaire de méthode : remplacer la constante par le **budget** qui la justifie. Une borne écrite
seule se transmet sans provenance et personne n'ose y toucher ; une borne accompagnée de son
budget d'erreur dit exactement à quelle condition on peut la déplacer. C'est I-14 appliqué aux
limites de domaine, pas seulement aux coefficients.

## L232 — Un écart absolu ne distingue pas la perte de précision de la croissance du signal

*(S155)* Mesurant l'erreur du noyau modal quand la durée de forçage s'allonge, j'ai lu un écart
multiplié par quinze entre 16 et 60 secondes et j'ai failli conclure à une dégradation numérique.
L'amplitude du champ, elle, était multipliée par presque quatre sur le même intervalle : **la
moitié de la croissance était du signal, pas de l'erreur.** Rapportée à l'amplitude, l'erreur
suivait la même loi que dans le régime où l'amplitude ne bougeait pas.

Les seuils de régression du dépôt sont souvent absolus, parce qu'ils sont nés d'une fixture unique
où l'amplitude ne variait pas. Dès qu'on sort de la fixture, un seuil absolu mélange deux choses
et fait ressembler un signal plus fort à un calcul plus faux.

Le piège a une seconde face, rencontrée dans la même session. Rapporter l'écart à `|eta|`
instantané le faisait exploser d'un facteur cent au voisinage des nœuds de l'oscillation, là où le
dénominateur passe par zéro — une explosion qui n'était pas davantage une perte de précision.
**Le bon dénominateur est une grandeur que la dynamique conserve** : ici l'amplitude invariante
`sqrt(|eta|^2 + |v|^2/omega^2)`, que la propagation libre laisse fixe. Voir [[L218]] pour l'autre
manière de se tromper en lisant ses propres mesures.

## L233 — Une conservation qui découle de la structure ne mesure rien

*(S156)* Le bilan énergétique d'un sillage prolongé se conserve au bit près à 16, 20, 30, 45 et
60 secondes, et la puissance est exactement nulle dès l'extinction. C'est un beau tableau, et il
ne prouve rien : après extinction chaque mode tourne, et la rotation laisse `g|eta|² + |v|²/k`
invariant. **Le bilan ne pouvait pas ne pas se conserver.**

Le piège est d'autant plus efficace que la quantité vérifiée est physiquement importante et que le
résultat est excellent. Un chiffre parfait obtenu d'une identité algébrique ressemble exactement à
un chiffre parfait obtenu d'une physique juste.

La question à se poser avant de publier une conservation : **qu'est-ce qui aurait pu la briser ?**
Si la réponse est « rien, c'est une identité », alors la mesure confirme l'implémentation et il
faut le dire ainsi — jamais la présenter comme un certificat du phénomène. Ici, la validité
spatiale du champ, qui était la vraie question, était mauvaise au moment même où le bilan était
parfait. Voir [[L232]] : la même session, deux façons de lire un bon chiffre pour un mauvais.

## L234 — Deux bornes indépendantes n'ont pas d'ordre stable

*(S156)* Un champ de sillage est borné en rayon par la résolution angulaire et en durée par la
résolution radiale. Les deux mécanismes sont indépendants, et **laquelle des deux mord dépend de
l'instant** : à 8 secondes, une recette 512 radial / 128 angulaire n'est honnête qu'à 45 m tandis
que 128/512 l'est au-delà de 200 m ; à 60 secondes, exactement l'inverse, la seconde se trompant
d'un facteur 75 là où la première reproduit la référence au bit près.

L'habitude est de chercher « le » facteur limitant et de dimensionner dessus. Quand deux bornes
suivent des lois différentes — l'une proportionnelle, l'autre en périodicité — leur croisement se
déplace avec le régime, et une valeur unique choisie dans un régime est fausse dans l'autre.
**Mesurer les deux séparément coûte un essai de plus et évite de dimensionner sur la mauvaise.**

Corollaire pratique : une borne écrite comme un nombre — « le domaine vaut 45 m » — perd
l'information dont le lecteur suivant aura besoin. Ce qui se transmet est la **loi et son
paramètre** : « le rayon est proportionnel à `angular` ; il vaut 45 m pour 128 ». Même remarque
qu'en [[L231]], où une constante en secondes cachait un nombre de bits.

## L235 — Un plan d'expérience où deux variables sont liées ne peut pas les séparer

*(S157)* Pour éprouver la dépendance à la largeur de source, j'ai balayé `sigma` en gardant le
produit réduit `sigma·cutoff` constant — le bon réflexe, puisque c'est ce qui conserve la forme
spectrale. Mais le pas radial vaut `dk = cutoff/radial`, donc à produit constant
`dk = 6/(sigma·radial)` : **`sigma` et `dk` n'étaient plus des variables indépendantes**.

L'ajustement libre `t = C·sigma^p·dk^q` rassemblait alors les sept points à un facteur 1,38, avec
un exposant `q = −1,04` séduisant parce que proportionnel à la période spatiale. Trois paramètres
pour sept points liés ajustent n'importe quoi, et le bon chiffre donnait au résultat toute
l'apparence d'une loi.

La vérification tient en une ligne : **écrire les variables du plan et regarder si l'une se déduit
des autres.** Si oui, aucun ajustement ne séparera leurs effets, quelle que soit la qualité du
résidu. En variables réellement indépendantes — `sigma` et `radial` — l'exposant valait 0,74 pour
une source et 1,20 pour une autre, ce qui dit franchement qu'il n'y a pas de loi de puissance.

Le résidu d'un ajustement mesure la souplesse du modèle autant que la régularité des données.
Voir [[L232]] : encore une façon de lire un bon chiffre pour un mauvais.

## L236 — Un instant de rupture qui dépend du seuil n'est pas un instant de rupture

*(S157)* Chercher « à partir de quand » une résolution cesse d'être valide suppose qu'il existe un
instant à mesurer. Ici la dégradation est **graduelle** : la courbe monte, et l'instant qu'on en
tire est celui de la tolérance qu'on a choisie. Le seuil de 10 % donnait 16 / 36 / 56 secondes,
celui de 100 % donnait 32 / 46 / au-delà de la fenêtre — et l'exposant de la loi supposée passait
de 0,90 à 0,63 au passage.

Le signe à guetter est exactement celui-là : **si le résultat bouge quand la convention bouge, ce
n'est pas le phénomène qu'on mesure, c'est la convention.** Il faut alors soit obtenir la
convention de qui la subira — c'est ce qui manquait ici, personne n'ayant spécifié l'erreur
acceptable —, soit publier la courbe entière plutôt qu'un point d'icelle.

Corollaire pour les garde-fous : un refus à l'exécution gèle une tolérance dans l'API. Quand la
tolérance n'est pas spécifiée, la coder revient à choisir à la place du consommateur la chose la
plus lourde de conséquences. Voir ADR-108.

## L237 — Écrire la valeur avant de lire l'horloge

*(S157)* Deux battements faux en deux sessions : une minute d'avance en S156, dix minutes de
retard en S157. La cause n'est pas l'étourderie, c'est l'**ordre des gestes** — j'écrivais la
valeur dans le script, puis je lançais `date` dans la même commande pour « vérifier ». La valeur
était donc décidée avant d'être connue, et la vérification arrivait trop tard pour servir.

La règle du dépôt dit « le relever, jamais l'écrire de mémoire » (note S118). Elle ne vise pas la
mémoire longue : elle vise ce geste-là, à trente secondes d'intervalle. **Lire, puis écrire ce
qu'on a lu, dans cet ordre et en deux temps.**

**Addendum du 2026-09-10 (S158)** : la leçon a été violée à la session suivante, deux fois, pour
le geste exact qu'elle décrit. Une leçon écrite ne change pas un geste ; seule une procédure le
change. Celle-ci : **lire l'horloge dans un appel séparé, puis copier la valeur lue**. Tant que la
lecture et l'écriture tiennent dans la même commande, la valeur est décidée avant d'être connue,
et la lecture ne sert qu'à donner bonne conscience.

La généralisation dépasse le battement. Chaque fois qu'une valeur mesurable est écrite dans un
script avant d'être mesurée, la mesure ne sert plus qu'à confirmer — et elle confirme rarement
quelque chose qu'on aurait le courage de défaire.

## L238 — Qui lit une borne est immunisé contre une erreur de phase

*(S158)* Un champ de sillage replié se trompe d'un facteur cinquante sur son élévation
échantillonnée. Au même instant, son enveloppe de pente et son énergie se trompent de 0,6 % et
0,04 % — **huit mille fois moins**. La raison est structurelle : le repliement rephase les modes
sans toucher aux amplitudes, et toute grandeur calculée depuis les coefficients ne voit que les
amplitudes.

Le déclencheur d'écume du dépôt lit l'enveloppe, pas un échantillon. Il est donc immunisé, sans
que personne l'ait conçu pour cela.

La règle se généralise et vaut au moment de choisir ce qu'une couche publie : **une borne dérivée
des coefficients traverse intacte les erreurs de phase ; un échantillon les subit en entier.**
Quand un consommateur peut être servi par une borne, la servir coûte moins cher en robustesse
qu'un échantillon plus précis. La réciproque est le piège : remplacer une borne par un échantillon
« plus exact » peut dégrader le consommateur d'un facteur mille sans qu'aucun test d'exactitude ne
s'en aperçoive — c'est pourquoi S158 laisse un témoin qui tombera si cela arrive.

L'immunité ne vaut que pour les erreurs de phase. Une erreur d'**amplitude** traverse les bornes
comme les échantillons. Voir [[L233]] : là aussi, une propriété structurelle expliquait un chiffre
parfait, et il fallait savoir laquelle.

## L239 — Demander la tolérance avant le mode de défaillance, c'est demander dans le mauvais ordre

*(S158)* Trois sessions ont cherché de quoi borner un champ de sillage replié : une loi (S157),
puis une tolérance (S158). La question utile était en amont des deux — **qu'est-ce que cette erreur
casse au juste ?**

La réponse la déclasse. L'erreur est déterministe, identique chez tous les participants, donc elle
ne produit ni désynchronisation, ni divergence de réplique, ni inégalité entre joueurs ; l'autorité
au sens d'I-15 reste acquise **avec l'erreur dedans**. Ce qui souffre est la vraisemblance, et rien
d'autre. Un garde-fou contre une faute protège d'un dommage ; un garde-fou contre une infidélité
arbitre une apparence. Ni la même urgence, ni le même juge, ni le même coût quand on se trompe.

Le mode de défaillance dit aussi **qui tranche**. Une infidélité se juge à l'œil, et le dépôt a
déjà un dispositif pour cela — B4 et sa perception en double aveugle. Chercher une dérivation
numérique était chercher dans la mauvaise catégorie.

D'où l'ordre à tenir : *que casse l'erreur* → *qui en juge* → *quelle tolérance* → *quel
garde-fou*. Commencer par la fin fait mesurer longtemps une quantité dont on ne sait pas encore si
elle mérite d'être bornée. Voir ADR-109 et [[L236]].

## L240 — Une consigne qui nomme une ressource disparue n'instruit plus, elle égare

*(S159)* Le bloc de jeton de `REPRISE.md` avait accumulé, de session en session, des notes du type
« cette copie a été refusionnée, terminez de même ». Elles étaient exactes le jour où elles ont été
écrites. Au moment de S159, l'une annonçait « cinq worktrees » quand il y en avait six, et une
autre prescrivait de refusionner une branche que plus rien ne justifiait de garder.

Le défaut n'est pas l'inexactitude, c'est le **genre**. Un document de passation doit porter la
**procédure** ; l'**inventaire** se constate — ici par `git worktree list`, en une seconde et sans
risque de mentir. Tout inventaire recopié dans un document commence à vieillir à la seconde où il
est écrit, et il vieillit sans prévenir, parce que rien ne relit une note qui a été juste.

Le signe à guetter : une consigne qui **nomme** une ressource — une branche, un fichier, un
chemin. Elle a une date de péremption que son lecteur ne connaîtra pas. Quand la consigne peut être
écrite sans nommer, elle survit ; quand elle doit nommer, elle appartient à un registre daté, pas à
un document qu'on lit pour agir. Voir [[L218]] : là aussi, une écriture juste à sa date induisait
en erreur plus tard.

## L241 — Mettre à jour avant de supprimer, même quand la suppression est l'objectif

*(S159)* Six copies de travail à assainir, dont quatre annonçaient un jeton `libre` avec quatre
« dernière session » différentes — S158, S157, S146 et S44. La tentation est de supprimer d'abord :
c'est ce qu'on est venu faire.

L'ordre inverse vaut mieux, et pour une raison qui n'est pas la prudence. **Une avance rapide
éteint le danger sans rien détruire** : dès qu'elles lisent le même jeton, aucune ne peut plus
égarer une session, qu'on les supprime ensuite ou non. Le geste réversible produit ici l'essentiel
du bénéfice, et le geste irréversible ne fait que ranger.

Le critère se généralise : **quand un geste réversible et un geste irréversible visent le même
danger, faire le réversible d'abord change ce qu'une interruption laisse derrière elle.** Une
session coupée après l'avance rapide laisse un dépôt plus sûr qu'au départ ; coupée au milieu des
suppressions, elle aurait laissé un état intermédiaire que personne n'aurait su lire.

## L242 — Publier un « facteur », c'est publier trois choses

*(S160)* Deux sessions consécutives ont publié « 2,5 » sur le même problème, et la troisième a été
dépensée à demander si c'était un plafond. Ce n'en était pas un, et la moitié de la réponse ne
demandait aucune mesure : **les deux nombres n'étaient pas la même statistique**. S157 publiait une
*étendue* `max/min` ; S158 une *déviation* au rapport idéal. Sur le seul jeu de S158, les deux
valent 3,42 et 2,50 — un « facteur » ne désigne rien tant qu'on ne dit pas laquelle.

L'autre moitié demandait une mesure, et elle a montré deux dépendances que le nombre publié
cachait : **le régime** — S158 écartait deux cases sur un motif inexact, et un seuil uniforme fait
passer sa déviation de 2,47 à 12,30 — et **la famille de montages** : 2,17 à 2,47 pour un cutoff de
6, sur un facteur 4 en sigma, mais 24,08 à cutoff 1,5.

Un facteur publié se lit donc avec trois compléments, et sans eux il décrit le montage de son
auteur en ayant l'air de décrire le problème :

1. **quelle statistique** — étendue, déviation, écart-type, dispersion ;
2. **sur quel régime** — quelles cases sont retenues, et pourquoi, vérifié plutôt qu'affirmé ;
3. **dans quelle famille** — les paramètres qui n'ont pas varié pendant la mesure.

Voir [[L219]] : un qualificatif rend une question présentable sans y répondre ; un facteur nu fait
pire, il donne l'illusion d'un chiffre. Et [[L229]] : là, l'écart ne survivait pas au changement de
graine ; ici, il ne survit pas au changement de bande.

## L243 — Un blocage hérité se vérifie avant d'être contourné

*(S161)* B4 était « bloqué par la référence substitutive intégrale ». La phrase circulait depuis
plusieurs sessions, et elle était vraie — mais elle ne disait pas *ce qui* manquait, et personne ne
l'avait ouverte.

En l'ouvrant : la référence demandée est un solveur qui calcule le champ total sans décomposition,
et le dépôt en a un depuis **S36** — `shallow.rs`, Saint-Venant 1D non linéaire, reçu par le
harnais et par l'oracle croisé. Ce qui manquait n'était pas la référence : c'était **une fonction
de trois lignes** pour poser deux perturbations dans un même domaine, `configure_bosses`. Le
montage minimal d'un test d'additivité était inexprimable, ce qui suffit à expliquer qu'il n'ait
jamais été tenté.

Ce qui rend le motif générique : un blocage énoncé une fois se **recopie** de session en session
comme un fait, et chaque recopie le rend moins ouvrable — il devient une propriété du corpus au
lieu d'un constat daté. C'est [[L217]] appliqué à un empêchement plutôt qu'à un renvoi : *un
blocage non vérifié ferme la question aussi bien qu'un renvoi faux.*

En pratique, deux questions avant d'accepter un blocage hérité : **de quoi exactement a-t-on
besoin**, et **qu'est-ce qui existe déjà qui y ressemble** ? Ici, la seconde a suffi. Voir aussi
[[L240]] : une consigne qui nomme une ressource disparue égare ; un blocage qui ne nomme rien
égare autant.

## L244 — Une décomposition d'état n'est pas une superposition d'évolutions

*(S162)* Un diagnostic comparait deux simulations autonomes additionnées à leur simulation
conjointe. Il révélait une interaction physique réelle. L'architecture visée, elle, prévoyait
une équation du résidu avec termes croisés : ce diagnostic n'en testait pas le calcul.

Pour `U_t=N(U)`, poser `U=Q+d` donne `d_t=N(Q+d)-Q_t`, même si N est non linéaire.
La reconstruction additive reste exacte en principe ; ce sont les approximations de l'équation
résiduelle qu'il faut recevoir. Un écart de superposition ne les mesure pas.

Avant d'interpréter un banc, écrire côte à côte **l'équation prévue et celle exécutée**.
Une réserve de dimension ou de précision ne suffit pas si l'objet mathématique a changé.
Et construire le résidu après coup par soustraction ne teste pas son intégration : ce serait
une identité. Un vrai témoin doit retirer un terme du couplage et faire perdre l'accord.
## L245 — Le résidu du schéma contient des termes absents de l'équation continue

*(S163)* Le flux physique du résidu était exact. Il fallait pourtant lui ajouter un terme de
couplage issu de Rusanov : la vitesse de diffusion numérique change entre fond seul et champ
total. Sans `(a_total-a_fond) saut(fond)`, le couplage se trompe de 2,73e-4 à N240,
contre 5,92e-15 pour le calcul complet. Les deux conservent la masse.

Soustraire les équations continues puis discrétiser n'équivaut donc pas, sans contrôle, à
soustraire leurs versions discrètes. Ce qui paraît artificiel dans la physique peut être
indispensable pour retrouver le schéma effectivement exécuté. Le défaut décroît au raffinement,
mais cela ne le rend pas acceptable à la maille utilisée.

Pour recevoir un couplage, écrire le résidu du **calcul complet**, frontières et intégrateur
compris, et distinguer cette fidélité au schéma de sa précision physique. Ici l'accord à
1e-13 coexiste avec des différences entre grilles au pourcent : les deux verdicts répondent
à des questions différentes. Voir L244 et SPEC-004 §6.1.
## L246 — Un défaut d'identité ne distingue pas une approximation d'un terme manquant

*(S164)* La dérivée continue du fond et l'omission de sa source échouent toutes deux au test
qui exige de retrouver le même RK2 du total. Au raffinement, la première réduit son écart
d'un facteur quatre à chaque division du pas par deux ; la seconde conserve son erreur.
Le même verdict nominal désignait deux comportements différents.

Un critère vérifie la propriété qu'il énonce : ici l'identité discrète. Il ne qualifie pas
à lui seul la cohérence, la stabilité ou l'utilité physique de ce qu'il rejette. Avant de
condamner une approximation parce qu'elle n'est pas identique, mesurer sa convergence et
la confronter à un vrai terme manquant.

Le remède de cette expérience se dérive au niveau des étages : corriger par les incréments
du fond prescrit, plutôt que par ses dérivées continues, retrouve l'identité au pas utilisé.
Cela reçoit le changement de variables discret ; la précision du schéma total reste à part.
Voir L245 et FOND-PRESCRIT-S164.

## L247 — Recevoir une sortie ne reçoit pas l'information entrante

*(S165)* Un bord alimenté par le fond seul laisse sortir une bosse avec un écart de hauteur
de 0,2 % de son amplitude initiale à N240 ; dans le cœur, l'écart est encore vingt fois plus
petit. Le même bord, sur le même total initial décomposé avec un fond variable, donne 44 %
d'écart jusque dans le cœur. Il omet alors le résidu extérieur qui compense ce fond.

Une frontière fait deux choses : laisser sortir ce que le domaine connaît et recevoir ce
qu'il ne connaît pas. Une bonne mesure de la première ne garantit rien sur la seconde.
Déclarer l'information entrante disponible avant de qualifier la fermeture, et tester les
deux sens. Un oracle aux mêmes étages permet de localiser le défaut ; il ne résout pas
l'absence d'information dans le système réel.

Le montage avec compensation extérieure est volontairement non local : il reçoit le
changement de variables, pas la possibilité de retrouver un extérieur arbitraire. Cette
limite appartient à l'énoncé du banc. Voir FRONTIERE-LOCALE-S165 et A221.

## L248 — Un témoin qui partage le schéma ne voit pas ses défauts communs

*(S166)* La fermeture autonome donne un écart au témoin de 0,15 % de l'amplitude,
mais l'écart à l'onde analytique atteint 31 % au même maillage. Vers 12 s, les deux
calculs ont perdu 13 % de hauteur de crête. Le bord est proche du témoin parce qu'ils
partagent le calcul intérieur dissipatif, pas parce que l'onde est fidèlement transportée.

Une référence du même schéma isole utilement un changement de variables ou une frontière.
Elle ne qualifie pas ce que les deux calculs ont en commun. Garder deux comparaisons :
l'une contre le même calcul pour localiser le défaut, l'autre contre une référence
physique indépendante pour mesurer la justesse. Ne pas soustraire leurs maxima : comparer
les champs point à point avant de prendre la norme.

Ici la référence indépendante est une onde simple non linéaire, dérivée des invariants
et bornée avant choc. Elle montre aussi que le fond prescrit peut hériter des défauts du
solveur total alors qu'il était exact à l'entrée. Voir BORD-AUTONOME-S166 et A222.

## L249 — Préserver un champ ponctuel ne préserve pas son intégrale discrète

*(S167)* Le fond analytique reste exact en chaque centre de cellule, avec un résidu nul,
mais le volume calculé comme somme(h dx) ne ferme pas avec les flux physiques du bord.
Le budget résiduel ferme pourtant à l'arrondi. Le défaut restant décroît comme dx² :
les centres ne sont pas des moyennes et la quadrature temporelle n'est pas une intégrale.

Avant d'affirmer qu'une méthode conserve une grandeur, nommer sa représentation discrète
et le flux auquel on la compare. Changer le schéma du fond change aussi son flux : ici
le flux pertinent est delta numérique + fond physique, pas l'ancien Rusanov total.
Même après cette correction, l'exactitude ponctuelle ne donne pas la clôture du volume.

Un bilan nul par symétrie ne tranche pas cette question : le fond figé de S167 a un
flux net nul et une source intégrée nulle. La réception suivante doit casser cette
symétrie. Voir FOND-PRESERVE-S167 et A223.

## L250 — Recevoir un bilan sans le fabriquer

*(S168)* Les moyennes du fond et les flux temporels sont intégrés indépendamment,
respectivement dans l'espace et dans le temps. Leur accord à l'arrondi reçoit la
conservation. Déduire le flux de la variation de volume aurait donné le même zéro,
sans vérifier la physique ni les quadratures.

Les contre-épreuves séparent les causes : avec les centres, intégrer mieux le temps
ne supprime pas le défaut spatial ; avec les moyennes, deux temps RK2 laissent un
défaut temporel qui diminue par quatre au demi-pas. Les deux corrections sont nécessaires.
Enfin le fond figé asymétrique rend son flux net non nul : le bilan ne peut plus passer
par annulation des termes que l'on veut éprouver.

Préserver l'indépendance des chemins de calcul du témoin et du candidat, et vérifier
qu'au moins un cas fait travailler chaque terme du bilan. Voir VOLUME-MOYEN-S168.

## L251 — Transporter un écart demande de changer son point de référence

*(S169)* Copier un invariant total de la cellule intérieure vers le fantôme crée une
perturbation sur un fond spatialement variable. Le fond exact était différent aux deux
positions. Copier l'écart à Q intérieur, puis le réancrer sur Q fantôme, préserve d=0.

Une fermeture correcte pour le total n'est donc pas automatiquement équilibrée pour
un résidu. Tester l'assemblage sur un fond variable sans perturbation avant les scènes
complexes ; conserver l'ancien transfert comme contre-épreuve. Préservation, qualité
et conservation restent indépendantes : même la fermeture fautive ferme le volume.
Voir ASSEMBLAGE-AUTONOME-S169.
## L252 — Raffiner la destination ne répare pas une source figée sur un réseau grossier

*(S170)* Le solveur passe de dx1 m à0,25 m, mais la source interpolée garde H8 m.
L'injection intégrée reste identique, et l'écart au témoin ne disparaît pas. L'intégrale
exacte du même interpolant ne dépend pas du découpage fin qui la reçoit.

Distinguer les deux résolutions dans le protocole. Un ratio fixe mélange leurs effets ;
un seul alignement du réseau cache aussi sa sensibilité de phase. Ici décaler le réseau
inverse le signe de l'injection et peut rendre l'interpolation pire que l'omission.

Une injection presque nulle ne garantit pas une source locale juste : l'omission perd
presque toute l'onde tout en ayant un petit défaut de volume, car l'intégrale nette de
la source exacte est petite. Recevoir structure locale et bilan séparément.
Voir SOURCE-DECIMEE-S170 et A225.
## L253 — Une somme télescopique transporte les erreurs aux bornes

*(S171)* Construire une source comme différence de flux partagés annule les flux
intérieurs dans le bilan global. Il reste le flux à chaque extrémité : s’il est
interpolé avec erreur, le bilan physique garde cette erreur. Un alignement heureux
des nœuds peut la masquer ; varier le décalage fait partie du test.

Ancrer les flux aux valeurs exactes aux bornes ferme ce bilan, mais ne rend pas les
différences locales exactes. Recevoir séparément intégrale, structure locale et champ
évolué ; déclarer aussi la disponibilité des valeurs utilisées aux bornes. Une preuve
de conservation ne remplace ni une preuve de précision ni une interface fournissant
les données nécessaires. Voir SOURCE-FLUX-PARTAGES-S171 et A225.

## L254 — Le résidu dépend de sa référence autant que de la perturbation

*(S172)* Changer la représentation du fond Q sans changer l’eau initiale impose de
changer d=T-Q. Le résidu peut alors porter une grande compensation de représentation
sans nouvel événement physique. Une comparaison qui impose d=0 aux deux fonds
changerait aussi l’état total et confondrait deux expériences.

Même total initial ne suffit pourtant pas : l’évolution conserve le défaut
S-Lnum(Q). Recevoir la cohérence de la source avec la représentation, puis comparer
à une évolution totale indépendante. L’identité discrète élimine cette dépendance
mais ne garantit pas la préservation physique d’un fond déjà solution exacte.

Mesurer représentation, évolution et bilan séparément ; aucune de ces identités ne
constitue à elle seule un critère physique d’activation du résidu. Voir
FOND-RECONSTRUIT-S172 et A50.

## L255 — La cohérence temporelle porte sur les incréments

*(S173)* Soustraire la même sécante ΔQ/dt aux deux étages RK2 rend leur assemblage
compatible avec la variation connue du fond. Cela ne suffit pas si le flux physique
est intégré autrement : le trapèze laisse un défaut de volume que prédit exactement
son écart à la quadrature temporelle indépendante.

Recevoir les incréments complets, pas seulement la justesse ponctuelle des dérivées.
Garder deux références : le solveur total pour l’identité discrète et la solution
physique pour la préservation du fond. Sur Q exact seul, le premier est diffusif alors
que le second doit rester intact. Voir FOND-MOBILE-S173 et A50.

## L256 — Un bilan fermé est relatif aux flux qu’il compte

*(S174)* La source construite depuis les mêmes instantanés que le fond ferme exactement
son budget de flux interpolés. Ceux-ci restent différents des flux analytiques aux
frontières. Le second bilan garde alors un défaut que ni dx ni dt du solveur ne corrigent
àcadence du fond fixée.

Nommer la référence du bilan ; publier séparément conservation de la représentation
et fidélité de ses flux aux données physiques. Prédire leur différence depuis les flux,
sans reconstruire cette prédiction depuis le volume évolué. Une chaîne cohérente peut
transporter exactement des données approximatives. Voir CADENCE-FOND-S174 et A225.

## L257 — Une identité de solveurs exige aussi des frontières identiques

*(S175)* Le résidu àsource discrète retrouve le solveur total uniquement si les deux
voient la même règle de frontière àchaque étage. Lui opposer un total àfantômes
analytiques alors que son propre bord est autonome mesurerait l’effet du bord,
pas une faute dans l’identité algébrique.

Apparier les conditions aux limites pour recevoir l’identité, puis comparer séparément
les champs entre fermetures. Calculer leur différence avant de prendre le maximum :
la différence de deux maxima n’est pas le maximum de la différence. Conserver les
budgets fondés sur les flux de chaque calcul. Voir FRONTIERE-FOND-DECIME-S175.

## L258 — Une réception doit nommer le code qui la consomme

*(S176)* Treize sessions reçoivent le résidu, ses sources et ses frontières sur un fond
analytique1D. Elles ne construisent pas pour autant le fournisseur de dérivées du B
réel : ses consommateurs utilisent encore un autre type d’échantillon.

Relier chaque preuve àson modèle, àson implémentation et àson interface consommatrice.
Quand ce dernier lien manque, le prochain lot est un raccord de construction concret,
pas nécessairement une autre variation du banc. Une matrice de réception doit montrer
cette absence au lieu de transformer le nombre de tests en avancement du runtime.
Voir BILAN-B4-S176.

## L259 — Partager les phases ne dispense pas de qualifier les dérivées

*(S177)* Le fournisseur différentiel reprend les paramètres et phases de B. Son
raccord de surface reste bit àbit identique ; ses dérivées sont pourtant celles du
champ analytique représenté, pas celles de la fonction quantifiée effectivement
évaluée en machine. Une différence finie trop petite mesure cette quantification.

Recevoir séparément raccord des valeurs et dérivées physiques, avec une référence
analytique indépendante et des pas couvrant l’arrondi sans masquer la troncature.
La concordance d’un hash n’est pas une preuve sur un gradient. Voir FOURNISSEUR-B-S177.

## L260 — Contracter après la somme conserve les interactions

*(S178)* Un opérateur linéaire peut s’évaluer composante par composante. Le terme
advectif `(U·∇)U` ne le peut pas : sommer les résidus de modes isolés retire tous les
produits croisés, même si chaque mode est exact et chaque somme reproductible.

Fournir d’abord le champ total et son gradient, puis contracter. Recevoir séparément
la formule totale et une contre-épreuve où l’addition des résultats isolés diffère.
Un calcul modal correct n’établit pas la justesse de l’opérateur non linéaire composé.
Voir SOURCE-B-S178 et ADR-114.

## L261 — Une symétrie qui annule un champ n'annule pas ses dérivées

*(S179)* Au centre d'un impact isotrope, la vitesse horizontale est nulle et la
direction radiale indéfinie. Le gradient horizontal y reste isotrope et non nul.
Réutiliser le traitement de la vitesse pour remplir le gradient de zéros détruirait
les termes de couplage exactement au centre de leur source.

Dériver la limite de chaque grandeur demandée ; recevoir son voisinage dans plusieurs
directions. Une singularité de coordonnées n'est pas une singularité du champ physique.
Voir DIFFERENTIEL-W-S179, ADR-115 et contre-épreuve du gradient isotrope neutralisé.

## L262 — Une condition imposée doit traverser les grandeurs dérivées

*(S180)* Une pression appliquée àla surface intervient àla fois dans l'accélération
et dans la pression profonde. Garder le forçage dans l'évolution tout en l'oubliant
dans le gradient de pression fabrique une source volumique parasite. Le rajouter
ensuite comme une force extérieure peut au contraire le compter deux fois.

Dériver ensemble évolution, champ et source depuis la même condition imposée ;
documenter où celle-ci est déjà incluse. Recevoir dès le démarrage : un état encore
nul peut avoir une accélération et une pression non nulles. Voir ADR-116 et
DIFFERENTIEL-PRESSION-S180, contre-épreuve à56Pa avec eta=u=0.

**Application S181 de L260 et L262 :** le montage B+deux impacts+pressions conserve
les interactions après sommation, et la pression imposée est comptée une fois. La
réception confronte la source totale aux sources isolées et au gradient de Bernoulli.
Voir COMPOSITION-DIFFERENTIELLE-S181 ; aucune nouvelle leçon numérotée.

**Application S182 :** la réception du rejeu s'étend aux nouvelles sorties, source
comprise : l'identité antérieure de la seule surface ne prouve pas celle des dérivées.
Trente-quatre scalaires confrontés en bits après publication et restauration ; voir
CYCLE-DIFFERENTIEL-S182. Application des exigences existantes, pas de nouvelle leçon.

## L263 — Un axe de mesure ne mesure son effet que s'il dépasse ce qu'il transporte

*(S183)* Le protocole de S183 faisait varier la taille du lot pour mesurer l'amortissement des
contrôles de montage, payés une fois par lot. L'axe a bien produit des chiffres, réguliers et
reproductibles sur deux exécutions — et ils ne disaient pas ce qu'on lisait dedans. Les contrôles
coûtent 0,025 à 0,097 µs, soit moins de 0,4 ‰ d'un lot de 256 points : leur amortissement est
sous le bruit. Ce que l'axe montrait, c'était l'effet inverse et cent fois plus grand — le coût
par point **augmente** avec le lot (8,59 → 12,26 µs), parce qu'un lot plus grand visite des points
plus nombreux et plus dispersés et perd en localité. L'axe mesurait la distribution des points.

Ce qui a sauvé la lecture n'est pas la prudence : c'est qu'une **autre** table donnait le coût
des contrôles directement. Les refus indépendants des points sont exactement ces contrôles, sans
les points ; leur chiffre a rendu l'interprétation impossible à tenir.

Avant de faire varier un axe, chiffrer l'effet attendu et le comparer à ce que l'axe déplace en
même temps ; quand le confondant domine, prévoir une seconde voie qui isole l'effet — ici les
chemins de refus. Et écrire dans le document ce que l'axe a réellement mesuré, pas ce qu'il
devait mesurer : le protocole de S183 avait été publié avant l'exécution (§3), ce qui a rendu
l'écart lisible au lieu de le laisser se réécrire. Voir COUT-DIFFERENTIEL-S183 §6.7.

Corollaire immédiat, et coûteux à ignorer : **multiplier un coût par point par un nombre de
points sous-estime un grand lot** — de 13 à 16 % en différentiel et de 39 à 43 % en surface
entre le lot 1 et le lot 256. S125 le disait déjà d'une autre manière, en refusant d'extrapoler
un rapport hors de sa grille.

## L264 — Avant d'optimiser un parcours, mesurer la part qu'il peut atteindre

*(S184)* Le fournisseur différentiel coûte 2100 fois le pas de solveur qu'il alimente. La
réaction naturelle était évidente et elle avait même l'air élégante : sur un réseau régulier,
la phase de chaque mode avance d'un incrément constant, donc une récurrence remplace la
trigonométrie par une rotation, et le coût des sommes modales s'effondre. Le raisonnement est
juste. Il ne sert à rien.

Mesuré : la phase et la trigonométrie ne pèsent que **15 à 18 %** du différentiel de B, et la
récurrence n'en retire que **12 à 15 %**. À 32 composantes, un nœud coûte 140 ns par
composante, dont 21 de phase ; les 119 restants sont l'arithmétique ordinaire qui produit les
26 scalaires de sortie, et aucune traversée ne les supprime. L'optimisation visait le tiers
visible du travail en croyant viser le tout.

Ce qui a trompé n'est pas un calcul faux, c'est une **saillance** : la trigonométrie est la
partie difficile à écrire, celle qui a une table, un type dédié, un ADR. L'addition de
vingt-six flottants n'a rien de tout cela. Le coût, lui, ne suit pas ce qu'il a coûté à écrire.

Mesurer la part **avant** d'optimiser vaut une demi-heure et évite de construire une
optimisation correcte et inutile — et de la défendre ensuite, parce qu'elle marche. Le chiffre
utile n'est pas « combien la récurrence est plus rapide » (5,6×, et c'est vrai), c'est
« combien elle retire au total » (12 %). Toujours rapporter un gain à ce dont il est une
fraction ; un facteur cinq sur un sixième du travail n'est pas un facteur cinq.

Voir CONSOMMATION-S184 §6.4 et A227. Même famille que L263 — un axe ne mesure son effet que
s'il dépasse ce qu'il transporte ; ici, une optimisation ne vaut que la part qu'elle touche.

## L265 — Un contrôle qui relie deux mesures indépendantes attrape ce qu'aucune des deux ne montre

*(S185)* La session mesurait deux grandeurs : l'erreur de **source** introduite par le
réemploi, et l'erreur de **champ** qui en résulte après cent pas. Les deux tables étaient
plausibles — monotones en cadence, ordonnées comme attendu entre les trois modes, du bon
ordre de grandeur. Publiées telles quelles, elles n'auraient éveillé aucun soupçon.

Une troisième colonne les reliait : avec un état initial nul et une advection d'ordre
supérieur, l'écart de champ **doit** être l'intégrale en temps de l'écart de source. Elle
affichait 100 % d'écart sur toutes les lignes. Le défaut n'était pas dans la physique mais
dans le harnais — la fonction qui construisait les instantanés rangeait la source par indice
de bloc, celle qui la chargeait la lisait **compacte**, et chaque maille recevait donc la
source d'une autre. Les erreurs de source restaient justes (elles comparaient deux tableaux
dans le même rangement faux) et les erreurs de champ restaient vraisemblables.

Ce qui a sauvé la session n'est pas d'avoir vérifié chaque mesure, c'est d'avoir vérifié une
**relation entre** elles. Une mesure isolée ne peut être fausse que d'une façon qui se voit ;
deux mesures liées par une identité connue se contredisent dès que l'une dérape.

Chercher, dans tout protocole, la quantité qu'on peut calculer **de deux façons
indépendantes** et publier leur écart. S170 le faisait déjà avec son bilan de volume prédit —
le défaut signé contre l'intégrale de l'erreur de source — et le désignait comme le contrôle
central plutôt que comme un ornement. Une identité qui ferme à l'arrondi ne coûte presque
rien à écrire et ne se contente pas de rassurer : elle est le seul contrôle qui détecte une
erreur de **plomberie**, celle que les valeurs ne trahissent pas.

Corollaire, appris ici aussi : quand l'identité échoue à 100 %, soupçonner l'indexation avant
la physique. Un écart total, et non un écart grand, est la signature d'un appariement rompu.

Voir CADENCE-3D-S185 §6.1 et A228.

## L266 — Deux erreurs mesurées séparément ne se composent pas : leur somme est une enveloppe, jamais une prédiction

Le dépôt avait deux mesures d'approximation portant sur la même source — la décimation
spatiale (S170) et la cadence temporelle (S185) — et rien n'interdisait de les additionner
pour dimensionner les deux à la fois. Mesurées **ensemble**, sur le même véhicule et contre
une seule référence, elles ne s'additionnent sur aucune des 84 cases : l'additive surestime
jusqu'à 1,9 fois, et la loi réelle **change avec le mode de réemploi** — le maximum pour les
deux modes causaux, la quadratique pour l'interpolation. Sur un réseau décimé, dégrader la
cadence peut même **réduire** l'erreur totale de 17 %.

Ce qui généralise, en trois points :

1. **La somme est une enveloppe sûre et rien de plus.** Elle n'a été dépassée nulle part, ce
   qui en fait un outil de dimensionnement légitime — à condition de dire qu'on paie son mou.
   La présenter comme une prédiction, c'est annoncer une erreur qu'on n'a pas.
2. **La loi de composition est une quantité à mesurer, pas à choisir.** Additive, quadratique
   et maximum donnent des conclusions de conception **opposées** : la première dit de réduire
   les deux axes, la troisième dit que l'axe bon marché est gratuit jusqu'à la parité. Déclarer
   les trois avant la mesure, et le critère qui les juge, coûte un paragraphe.
3. **La composition peut dépendre d'un troisième paramètre**, ici le mode de réemploi. Un
   verdict global rejetait les trois lois ; séparé par mode, il en retenait une par mode. Une
   loi rejetée sur l'ensemble n'est pas une loi absente : c'est peut-être deux lois.

Corollaire de méthode, et il vaut au-delà de ce cas : **une mesure de composition exige une
référence unique et un contrôle croisé avec les mesures qu'elle compose.** Ici la ligne
`r = 1` devait redonner, chiffre par chiffre, les quatorze valeurs publiées par S185 — et
elle les redonne. Sans ce contrôle, une différence de montage entre les deux sessions se
serait lue comme une interaction entre les deux erreurs.

Voir COMPOSITION-ERREURS-S186 §8.5 et §8.6, A229. Même famille que L265 — relier deux mesures
indépendantes est ce qui révèle ce qu'aucune ne montre seule ; L265 le faisait dans une
session, L266 le fait entre deux.

## L267 — Une campagne qui balaie une résolution à convention de placement fixée mesure la convention autant que la résolution

S170, S184, S186 ont balayé un ratio de décimation `r` sur des dizaines de configurations,
avec le même réseau, et aucune n'a mis en doute **où** ce réseau posait ses nœuds. S187 a
changé un seul indice — le dernier nœud, déplacé de l'extérieur du domaine vers sa maille
de bord — et l'erreur est passée de **41,2 % à 6,8 %** à nombre de nœuds identique. Soit un
facteur six, gratuit, invisible à quatre sessions de mesure parce que la convention était
constante dans toutes.

Ce qui généralise, en trois points :

1. **Un paramètre balayé cache les paramètres tenus fixes.** Une courbe erreur-contre-`r`
   très propre — et celles de S186 l'étaient, avec des lois d'ordre deux et des constantes
   stables — ne dit rien sur ce qui n'a pas varié. La régularité d'une courbe n'est pas une
   preuve que son ordonnée est la plus basse atteignable.
2. **Quand la métrique est un maximum, `où` compte plus que `combien`.** Un maximum est
   porté par un endroit ; échantillonner **cet** endroit supprime le terme dominant, et
   ajouter des nœuds ailleurs ne le fait pas. C'est l'inverse d'une norme quadratique, où
   la densité seule gouverne. Le dépôt mesure en maximum depuis S170 : la conséquence
   valait d'être tirée plus tôt.
3. **La contre-épreuve est ce qui transforme un effet en règle.** Ancrer horizontalement,
   là où la source ne pique pas, vaut −2,5 %, −40 % et **+1,3 %** — non monotone, et une
   fois défavorable. Sans cette mesure, la conclusion aurait été *« il faut ancrer »*, une
   règle qui se serait trompée ailleurs. Avec elle, elle devient *« il faut poser un nœud
   là où vit le maximum »*, qui est vraie parce qu'elle dit le mécanisme.

Corollaire de méthode : **avant de raffiner un réglage, vérifier qu'aucune convention
n'annule le gain attendu.** La graduation que S187 est venue chercher vaut 1,4 ; la
convention qu'elle a trouvée en chemin valait 6. L'ordre de grandeur de ce qu'on ne
questionne pas est rarement plus petit que celui de ce qu'on optimise.

Voir RESEAU-GRADUE-S187 §8.4, ADR-118, A231. Même famille que L266 — une mesure n'est
valide que dans le montage où elle a été prise, et une convention de montage en fait
partie.

## L268 — Un rejeu qui confirme n'est pas un rejeu inutile : il transforme une coïncidence en condition

S188 a rejoué la grille de S186 sur un réseau corrigé, et le verdict est revenu **identique,
mode par mode**. Un tel résultat ressemble à une session perdue. Il ne l'est pas, et ce qui
fait la différence tient en une ligne de protocole : **la métrique ajoutée avant la mesure**.

S186 publiait l'erreur par tranche, mais jamais **où** vivait le maximum dans le cas composé.
S188 l'a relevé, et la réponse — la même tranche pour les deux erreurs, dans 39 cases jugées
sur 39 — a converti la conclusion de S186 d'un fait mesuré en une **condition de validité** :
la loi du maximum vaut *tant que les deux maxima coïncident*, ce qui est une propriété du
contenu et non de la composition. Le corpus a donc gagné, sans un chiffre nouveau sur la loi,
la connaissance de ce qui la casserait.

Ce qui généralise :

1. **Avant de rejouer, écrire la métrique qui distinguerait les explications possibles.** Un
   rejeu qui ne mesure que ce que mesurait l'original ne peut rendre que deux réponses — pareil,
   ou différent — et « pareil » n'apprend rien. S188 a déclaré trois issues en §4, dont une
   troisième : *la loi tient mais pour une autre raison*. C'est la colonne de tranche qui
   permettait de la distinguer, et elle a été ajoutée pour cela.
2. **Une loi mesurée une fois est une coïncidence tant qu'on ne connaît pas son mécanisme.**
   Le mécanisme est ce qui dit son domaine. Sans lui, on transporte la loi partout ; avec lui,
   on sait où elle cesse — ici, dès que les deux maxima se séparent, ce que le réseau gradué de
   S187 fait déjà.
3. **La dispersion autour d'une loi peut venir du montage, pas de la loi.** Le réseau mal placé
   de S186 donnait 0,826–1,155 ; le réseau ancré donne 0,895–1,060. Corriger le montage a
   **resserré** la loi. Une plage large n'est donc pas nécessairement le bruit de la nature :
   c'est peut-être un défaut du banc, et cela se vérifie en corrigeant le banc.

Corollaire : **ne pas changer deux variables dans un rejeu.** S188 n'a bougé que le placement
des nœuds, en conservant les nombres de nœuds, la référence, les métriques et le critère de
jugement déjà déclarés. C'est ce qui permet d'attribuer l'écart de magnitude — jusqu'à 3,7 fois
— au seul ancrage, et donc de lire le verdict inchangé comme une information.

Voir COMPOSITION-ANCREE-S188 §7.3, §7.4 et A232. Même famille que L266 — une mesure ne vaut
que dans son montage — et que L267, qui en est le cas où la convention du montage était
l'erreur.

## L269 — Une loi mesurée en norme n'est pas une loi : regarder les champs avant de nommer une loi

Trois sessions ont mesuré comment deux erreurs se composent en publiant des **rapports de
normes** : S186 a retenu le maximum pour les modes causaux, S188 l'a confirmé, S189 l'a réfuté
sur un autre réseau. Les trois avaient les **champs d'erreur en mémoire** — il fallait bien les
calculer pour en prendre la norme — et aucune des deux premières ne les a regardés.

Quand S189 l'a fait, la « loi » s'est dissoute en une structure plus simple et plus solide :
les deux champs s'**additionnent maille par maille**, à 10 % près, et exactement dans les cas
dégénérés. Tout le reste — maximum, additive, quadratique, et les rapports de 0,437 à 1,71 —
n'est que la **position relative de leurs pics** vue à travers une norme maximum.

Ce qui généralise :

1. **Une norme est une projection : elle jette l'information qui explique son résultat.**
   Nommer « loi » ce qu'une norme affiche, c'est nommer une ombre. Le coût de regarder le champ
   est nul quand on l'a déjà calculé, et c'est le seul niveau où une explication existe.
2. **Trois lois candidates qui se partagent les cas sont le signe qu'aucune n'est la bonne.**
   S186 retenait le maximum pour deux modes et la quadratique pour le troisième, et
   s'en félicitait comme d'un découpage éclairant. C'était un indice : une structure unique
   plus fine se cachait dessous, et le découpage par mode n'en était qu'une manifestation.
3. **La localisation doit être à la granularité de l'objet, pas à celle qui est commode.**
   S188 a localisé les maxima à la **tranche** — 196 mailles — et conclu qu'ils coïncidaient.
   À la maille, ils ne coïncident jamais. Une granularité trop grossière ne rend pas une
   réponse imprécise : elle rend la **mauvaise** réponse, avec l'apparence d'une confirmation.

Corollaire qui vaut pour la conception, et non seulement pour la méthode : **ce qui se
transporte, c'est le mécanisme, pas la loi.** L'additivité locale est une propriété du pas de
temps — explicite, presque linéaire — et elle dit d'elle-même où elle cesserait : une
projection de pression couple toutes les mailles. La « loi du maximum », elle, ne disait rien
de son domaine, et c'est pourquoi elle a survécu deux sessions de trop.

Voir COMPOSITION-GRADUEE-S189 §7.4, ADR-119, A232 et A233. Même famille que L268 — un rejeu
transforme une coïncidence en condition — dont c'est ici la suite : S188 avait obtenu la
condition, S189 a obtenu le mécanisme, et le mécanisme a réfuté la loi.

## L270 — Un opérateur global n'est pas nécessairement non linéaire ; une borne de norme requiert la bonne décomposition

S189 attendait de la projection une menace sur l'additivité parce qu'elle couple toutes
les mailles. S191 dérive P avant de le programmer : à opérateurs et bords fixes,
P(a+b)=Pa+Pb. Le couplage spatial décrit la portée, pas la linéarité. L'évolution
complète comporte encore l'advection et les erreurs de résolution : son additivité
se mesure séparément de celle de P.

L'autre confusion allait dans le sens rassurant : « la somme borne sans rien supposer ».
La triangulaire borne une somme de champs ; elle ne prouve pas que l'erreur d'une
évolution composée est la somme des erreurs de deux autres évolutions. Écrire le
résidu de cette identité manquante rend la borne exacte : e≤s+t+r. Sur le profil
S191, r ajoute0,002594 point, faible mais conceptuellement indispensable.

Avant de transporter une garantie : écrire l'opérateur et la décomposition exacte
sur lesquels elle repose. Un résultat numérique favorable ne remplace aucune des
deux identités. Voir ADR-121 et PROJECTION-B4-S191 ; limite explicitée : grille,
bords ou opérateurs dépendant de l'état peuvent changer cette conclusion.
## L271 — Une contre-épreuve à signal nul trouve les biais que les lignes principales absorbent

S193 déclarait, parmi ses contre-épreuves, une ligne sans intérêt apparent : mesurer le
décalage de fréquence non linéaire à une amplitude **négligeable**, où il doit valoir zéro.
C'est elle, et elle seule, qui a trouvé le seul défaut de la session — un biais d'estimateur
de `−1,1125·10⁻⁷`, soit 0,14 % de la grandeur à mesurer à la plus petite amplitude, et **de
même nature** qu'elle. Les vingt-quatre lignes principales l'avaient absorbé sans rien
signaler, et leurs verdicts passaient.

Le mécanisme est général : un biais additif constant se cache dans une mesure dont on attend
une valeur non nulle, et il ne se cache nulle part dans une mesure dont on attend **zéro**.
Une ligne à signal nul n'est donc pas une formalité de complétude : c'est le seul point du
protocole où l'erreur systématique n'a aucun endroit où se dissimuler.

Conséquence pratique : dans tout protocole qui mesure un effet, déclarer **avant** la mesure
au moins une configuration où l'effet est nul par construction — amplitude négligeable, état
au repos, ordre dégénéré — et lui donner un seuil serré. S192 en avait déjà l'esprit avec son
« lac immobile », mais l'appliquait à la stabilité et non à l'estimateur. Voir A236 et
SURFACE-LIBRE-NL-S193 §7.5.

## L272 — Sur un modèle semi-discret, la ligne de base d'une mesure est ce que le modèle porte, pas ce que le continu dit

Trois fois dans la même session, la même erreur a failli passer sous trois formes
différentes : comparer la fréquence mesurée à `ω₀` du continu, alors que le véhicule porte
`ω_d = √(g G_h(k))` ; ajuster l'harmonique libre sur `√(2gk tanh 2kh)` au lieu de
`√(g G_h(2k))` ; et construire la condition initiale et la fenêtre d'observation sur `ω₀`.
Les deux premières ont été corrigées par dérivation avant mesure ; la troisième est passée et
a produit un biais, trouvé par la contre-épreuve de L271.

L'ordre de grandeur explique pourquoi c'est un piège et non une maladresse : à `h=8 m` et
`K=64`, l'écart `G_h` contre `k tanh(kh)` vaut `1,2·10⁻³`, soit un écart de fréquence de
`6·10⁻⁴` — **16 % du décalage non linéaire à la plus grande amplitude, et dix fois le
décalage entier à la plus petite**. Une ligne de base continue ne décale donc pas légèrement
le résultat : elle mesure la discrétisation en croyant mesurer la physique.

La règle : identifier, pour chaque grandeur mesurée, la version **que le candidat porte
réellement**, et l'employer comme ligne de base ; garder l'oracle continu pour la grandeur
**relative** qu'on veut recevoir. Le contrôle qui le valide est gratuit : à l'ordre
dégénéré, l'écart à la ligne de base doit rentrer dans l'arrondi. Voir A236 et
SURFACE-LIBRE-NL-S193 §5.3.

## L273 — Le comptage d'ordres majore ; seule la mesure dit si une structure fait mieux

Le protocole de S193 prédisait une dérive de volume d'ordre `a^{M+1}`, par simple comptage
des ordres du système tronqué, et allait jusqu'à écrire qu'annoncer la conservation exacte de
S192 « serait un aveu d'erreur ». La mesure a donné `2,85·10⁻¹⁸` — de l'arrondi — et
**exactement zéro** à l'ordre un. Deux lignes d'algèbre, écrites après coup, en donnent la
raison : au mode nul, `(η Bψ)₀` et `(η_x ψ_x)₀` sont **la même somme**, et les deux termes
d'ordre deux de `η_t` s'annulent identiquement. Le système tronqué conserve `∫η` exactement.

La leçon n'est pas « le comptage d'ordres est inutile » — il donne une borne sûre et il l'a
donnée. Elle est que **le comptage d'ordres majore, parfois de très loin, parce qu'il ignore
les annulations de structure**, et qu'une prédiction pessimiste tenue pour acquise fait
manquer une propriété exacte. Le même schéma s'était déjà produit en sens inverse en S189 et
S191 (L270) : une garantie transportée sans écrire sa décomposition. Ici c'est une limite
transportée sans écrire son annulation.

Conséquence : une prédiction d'ordre se **mesure** avant d'être commentée, et un écart
favorable de plusieurs ordres de grandeur se traite comme un résultat à expliquer, jamais
comme une bonne surprise. La prédiction fausse reste écrite dans le document, avec sa
réfutation datée : c'est la prédiction qui a été prise, pas une rédaction à corriger après
coup. Voir SURFACE-LIBRE-NL-S193 §7.3.

## L274 — Un contrôle de convergence juge un ordre, jamais la taille d'un déplacement

Le protocole de S194 déclarait, pour distinguer le couplage mesuré d'un artefact numérique,
que passer `K` de 32 à 64 ne devait pas déplacer l'écart de plus de 2 %. Le déplacement
mesuré vaut `5,37 %`. Le contrôle est donc non tenu — et il n'aurait jamais pu l'être, car
ce qu'il mesurait était la **convergence** : le rapport des déplacements successifs
`|K32−K64|/|K64−K128|` vaut `3,81`, soit 4, soit exactement l'ordre deux.

Le défaut est logique et se dit en une phrase : sur une grille grossière, un déplacement de
quelques pour cent est **ce que produit** un schéma qui converge. Un contrôle qui juge la
taille du déplacement échoue donc d'autant plus que le schéma est meilleur, et il ne
distingue pas les deux situations qu'il prétend séparer. La forme correcte comporte deux
nombres, et ils demandent **trois** niveaux de raffinement et non deux : l'**ordre** de la
suite des déplacements, et le **résidu de Richardson** au pas effectivement retenu — ici
`1,93` et `0,47 %`, qui répondent à la question posée.

Conséquence pratique : tout contrôle rédigé « le déplacement doit rester sous x % » est à
reformuler en « l'ordre doit valoir y et le résidu rester sous x % », et il faut prévoir
trois niveaux dès le protocole. Le dépôt en compte d'autres sous la première forme (A239).
Voir COUPLAGE-DEUX-TRAINS-S194 §7.9.

## L275 — Une hypothèse sur une cause numérique se teste en retirant le terme soupçonné, pas en raisonnant

Devant une sensibilité de l'écart au raffinement vertical, S194 a formé une hypothèse
solide et précise : la condition initiale emploie le coefficient `b₂` du **continu** sur un
véhicule semi-discret, elle injecte donc une harmonique liée légèrement fausse, donc une
onde libre parasite dont l'amplitude dépend de `K`. C'est exactement le mécanisme d'A236,
déjà payé deux fois dans la session précédente, et le raisonnement était juste dans chacune
de ses étapes.

Elle est fausse. Le terme a été **retiré**, purement, et le résidu de discrétisation est
passé de `0,4698 %` à `0,5026 %`, l'ordre de `1,9295` à `1,8711` : rien n'a bougé. La cause
est ailleurs — dans la dynamique, `G_h` étant le seul objet dépendant de `K`.

Ce que la leçon retient n'est pas « méfie-toi des hypothèses » mais quelque chose de plus
utile : **une hypothèse de cause numérique est presque toujours testable par soustraction,
pour bien moins cher que le raisonnement qui la défend**. Retirer le terme a coûté un
paramètre booléen et trois exécutions. Un mécanisme déjà rencontré est le plus dangereux de
tous, parce qu'il se reconnaît sans être vérifié — et A236 avait précisément été *invoquée*
dans deux sessions avant d'être, ici, **mise à l'épreuve**. La variante sans le terme reste
publiée dans les mesures : c'est la trace de la réfutation, et sans elle la prochaine
session refera le même raisonnement.

Même famille que L273 — une prédiction se mesure avant d'être commentée — dont c'est le
versant causal : L273 portait sur un ordre de grandeur prédit, L275 porte sur un mécanisme
désigné.

## L276 — Une variable de protocole peut être réfutée par le véhicule avant d'être mesurée

*(S195)* Le protocole de S195 désignait le **jeu de phases initial** comme la variable qui
décide du régime d'addition, et l'écrivait en gras : *« une variable du protocole, pas un
détail de mise en œuvre »*. Trois réceptions sur dix en dépendaient, dont la principale. La
dérivation qui la soutenait était juste — les deux bornes, somme des amplitudes et racine de
la somme des carrés, sont les bonnes.

Elle était fausse sur le chemin qui y mène, et le véhicule le disait déjà. Chaque train
avance à **sa propre pulsation** : sur une fenêtre de dix périodes, les phases relatives
balaient toutes leurs valeurs, et une fonctionnelle qui prend un maximum sur l'espace *et sur
le temps* échantillonne les deux régimes quel que soit le départ. L'alignement initial ne
survit pas à la première période. Ce qui sépare réellement les régimes est la **fonctionnelle**
— le maximum tend vers la borne cohérente, la norme L2 vaut la racine de la somme des carrés
par construction.

Le fait décisif n'a demandé ni campagne ni analyse : un seul point à `n=6`, deux jeux de
phases, et le rapport valait `0,774` au lieu du `3,87` prédit. Il a été trouvé en écrivant les
**tests propres** du banc, avant toute mesure — et fixé là, en test, plutôt que découvert dans
une table à la fin.

Écrire les contrôles de vie d'un banc **avant** la campagne, et y inclure au moins un point
qui éprouve la variable dont le protocole fait dépendre sa conclusion principale. Une variable
de protocole n'est pas une hypothèse physique : c'est une affirmation sur ce que le véhicule
va faire, et le véhicule peut la contredire pour quelques lignes de test. Trois réceptions
mal spécifiées coûtent une campagne entière quand on l'apprend à la fin ; elles ne coûtent
qu'une note quand on l'apprend au début.

Corollaire, et il vaut pour la lecture des sessions précédentes : quand une réception échoue,
séparer d'abord **mauvaise spécification** et **défaut de banc**. Ici les sept réceptions
indépendantes de la prémisse de phase passent toutes — cas nul exact, continuité avec S194 à
`10⁻⁶`, convergence, bande, énergie. Le banc n'avait rien. Le protocole, si.

Voir SOURCES-MULTIPLES-S195 §7.1 et §7.3, et A241 pour ce que la réfutation a ouvert.

## L277 — Un invariant conservé ne dit rien de ce qui est résolu

*(S196)* Trois sessions de suite ont employé la dérive relative d'énergie sous `10⁻⁴` comme
critère de domaine, et l'ont traitée comme une attestation : sous le seuil, la configuration
est saine. S196 en a trouvé le contre-exemple, et il n'est pas marginal — à `n = 16` et
`K = 32`, la grandeur mesurée est **fausse d'un facteur cinq**, et l'énergie dérive de
`1,54e-6`, soixante-cinq fois **sous** le seuil.

Rien n'avait l'air cassé : aucun pas refusé, aucune valeur infinie, une courbe lisse. C'est la
nature même du défaut. Un schéma sous-résolu ne viole pas ses invariants — il les conserve
parfaitement sur le champ appauvri qu'il représente. La conservation mesure **ce que le schéma
préserve**, jamais **ce qu'il résout**, et les deux questions sont indépendantes.

Le symptôme, lui, était visible pour qui regardait au bon endroit : le triplet de convergence
donnait des incréments de **signes opposés**, `−1,105e-2` puis `+1,178e-4`. Une suite qui
converge ne fait pas cela. Mais la formule de Richardson, à qui on ne demande rien, aurait
imprimé « ordre 6,552 » — un chiffre d'apparence excellente, tiré d'un niveau hors domaine.

Deux gestes, et ils coûtent quelques lignes :

- **apparier tout critère de conservation à un contrôle de raffinement.** Un invariant borne
  les erreurs qu'il voit ; il faut un second contrôle pour celles qu'il ne voit pas ;
- **refuser d'imprimer un ordre depuis un triplet non monotone.** Incréments de signes opposés,
  ou second incrément plus grand que le premier : le niveau grossier est hors de son domaine,
  et le chiffre qu'on en tirerait n'a pas d'objet. Le banc doit le **dire**, pas le calculer.

C'est la même famille qu'**A238** — ne pas établir un ordre sur un maximum de résidu — et que
**L274** — déclarer la convergence en ordre et résidu sur trois niveaux. Toutes trois disent
qu'un chiffre de convergence est une affirmation sur un régime, et qu'il faut vérifier d'être
dans ce régime avant de l'énoncer.

Corollaire pour la lecture des sessions passées : « énergie conservée à `10⁻⁸` » ne certifie
aucune de leurs valeurs. Il ne faut pas pour autant les suspecter en bloc — les configurations
de S194 et S195 étaient loin du bord de résolution — mais la phrase ne doit plus être lue comme
une garantie qu'elle n'a jamais été. Voir A242 et REPLI-CROISEES-S196 §8.5.

## L278 — Une erreur systématique ne s'annule dans une comparaison que si les deux côtés la portent également

*(S197)* S196 avait bâti un montage soigné : deux familles de modes, même `n`, même cambrure
totale, bande relative bornée et déclarée comme confondant résiduel, échelle absolue éprouvée
par un témoin dédié. Il en a tiré un écart de `0,131` et une conclusion — le repli des
harmoniques croisées pèse un tiers. À résolution convergée, cet écart vaut `0,005`.

Ce qui n'avait pas été contrôlé n'est aucune des variables du montage : c'est **l'exposition
à une erreur de modèle**. Le symbole de dispersion se trompait de plus de 100 % en haut de
bande, et les deux familles avaient des bandes différentes — 38 contre 40 — donc des erreurs
différentes. L'« effet du repli » était, pour l'essentiel, l'écart entre deux défauts.

Deux sessions voisines n'ont pas été touchées, et la raison est instructive. S194 et S195
comparaient des configurations **à même bande** : l'erreur y était commune aux deux côtés et
s'est annulée, laissant 1 à 3 %. S195 portait pourtant une erreur de symbole de 43 %, et sa
conclusion tient. **Ce n'est donc pas l'ampleur de l'erreur qui décide, c'est sa répartition
entre les termes comparés.**

Une comparaison protège d'un défaut commun — c'est ce qui la rend puissante, et ce qui fait
qu'on lui fait confiance. Mais elle ne protège que de ce qui est **commun**. Dès que les deux
bras diffèrent par autre chose que la variable étudiée — une bande, une résolution, une
taille de domaine, un nombre de modes — ils différent aussi par leur part d'erreur, et cette
part entre directement dans le résultat.

Le geste : pour chaque comparaison, lister ce qui **diffère** entre les deux bras, et pour
chaque différence se demander non pas « est-ce physique ? » mais « **cela change-t-il la part
d'erreur de modèle que ce bras porte ?** ». S196 avait bien listé les différences ; il les
avait toutes jugées sur leur physique, aucune sur leur erreur.

Corollaire pratique, et il est bon marché : mesurer l'exposition de chaque bras — ici
`dispersion_error(upto)` — et la publier **à côté** du résultat, comme on publie une dérive
d'énergie. Un écart de 120 % contre 112 % entre deux bras se voit alors avant de conclure,
et non six sessions plus tard. Voir AUDIT-RESOLUTION-S197 §8.4, A242 et L277.

## L279 — Corriger le canal ne sert à rien si l'auteur est en conflit d'intérêt

*(S198)* S145 avait trouvé que deux des quatre recommandations de BILAN-S69 étaient restées
lettre morte pendant soixante-seize sessions, non par désaccord mais parce qu'aucun canal ne
les portait. Le remède choisi fut excellent et il a marché : faire porter la recommandation
par la ligne `Session suivante` du jeton, que toute session lit à l'amorce. Le canal est
devenu fiable — personne n'a plus jamais manqué cette ligne.

Cinquante-trois sessions plus tard, la mesure dit que rien n'a changé : **trente-trois
sessions sur trente-huit** ont pris pour sujet le reliquat de la précédente, et huit sessions
consécutives n'ont produit **aucune ligne de système**. Le canal marchait. Le contenu qui y
passait était le problème.

Parce que cette ligne est écrite **par la session qui vient de finir**, à partir de ce
qu'elle a laissé en plan. Elle a toujours raison localement : elle sait mieux que quiconque
ce qui manque à son propre travail, et ce qui manque est passionnant. Elle n'a simplement
aucune raison de proposer autre chose, et aucun moyen de savoir qu'elle est le trente-troisième
maillon d'une chaîne. **Le conflit n'est pas de mauvaise foi : il est structurel, et la bonne
foi ne le corrige pas.**

Quand un mécanisme de transmission échoue, distinguer trois causes avant de choisir le
remède : le canal n'existe pas, le canal existe et n'est pas lu, ou le canal est lu et
**celui qui le remplit n'est pas en position de bien le remplir**. Les deux premières se
corrigent en construisant ou en imposant une lecture ; la troisième demande de **retirer la
plume**, au moins par intermittence. C'est ce que fait la règle des deux maillons
(`REPRISE.md` §6.8) : elle ne juge pas la proposition, elle borne le nombre de fois où son
auteur peut la faire seul.

Corollaire pour tout garde-fou déjà en place : vérifier qu'il **mesure** son effet et pas
seulement sa présence. A211 était « à éprouver » depuis S145 et personne ne l'avait éprouvée,
faute d'un chiffre à regarder. Il en coûtait une commande `git` et vingt lignes de script.

## L280 — Une scène inspectée n'est pas reçue tant qu'elle n'a pas traversé le chemin qui la consommera

*(S203)* S201 a rendu la mer de référence — JONSWAP, Hs 1,5 m — et l'a inspectée : caméra,
intersections, zéro rayon non résolu, deux instants. Deux sessions plus tard, la première
tentative d'y poser un impact découvre, **avant d'écrire une ligne de rendu**, que la
composition B+W refuse chaque point de cette mer : son plancher de pente L1 vaut 0,608 contre
une limite de 0,449. L'image était juste ; elle n'avait jamais été demandée au chemin qui
compose.

Le défaut n'était pas dans le rendu, ni dans la composition, ni même dans le budget de pente
pris seul : il était dans leur **rencontre**, que personne n'avait provoquée. Chaque couche
avait sa réception, et aucune réception ne portait sur la scène que toutes deux prétendent
servir. Un corpus de réceptions par couche peut ainsi être complet et laisser le premier cas
d'usage impossible.

Le geste, et il coûte une commande : **dès qu'un chemin de consommation existe, y faire
passer la scène de référence**, même sans rien lui ajouter. Ici, un journal vide et un appel à
`compose` sur un point auraient suffi en S201. Et quand un budget indépendant du point est en
jeu, calculer son plancher sur la scène **avant** de choisir les paramètres visibles : c'est
un nombre, pas une campagne. Voir IMPACT-W-S203 §2 et A245.

## L281 — Un domaine tronqué se reçoit à sa frontière, pas à son admission

*(S203)* Le domaine d'un impact radial avait trois gardes — résolution, portée, régime — et
chacune dit ce que le **calcul** tolère. Aucune ne disait ce qui se **voit** quand l'hôte rend
le fond seul au-delà du disque ou après l'horizon. Mesurée, cette couture vaut 78 mm à un
horizon de 2 s, et reste au-dessus de 3 mm jusqu'à 48 s pour tout rayon et tout nombre de
modes : un champ linéaire dispersif s'étale, il ne s'éteint pas. L'admission acceptait
8 m et 4 s ; l'image demande 52 m et 56 s.

Ce qui généralise : **tout champ tronqué dans l'espace ou le temps** — domaine δ, niveau de
détail, emprise, fenêtre de repli — a deux réceptions distinctes. L'admission répond à « le
calcul est-il valide ici ? » ; la couture répond à « l'observateur voit-il où il s'arrête ? ».
La seconde ne se déduit pas de la première, et c'est elle qui dimensionne.

Le geste : pour chaque troncature, mesurer l'amplitude **sur** la frontière pendant toute la
vie du domaine, et l'amplitude **dans** le domaine à l'instant où il disparaît ; déclarer le
seuil avant. Et recevoir l'image contre un **témoin** qui exécute exactement la même marche
avec le même prédicat de domaine : un pixel différent hors domaine est alors une preuve de
fuite, pas une impression. Voir ADR-126, IMPACT-W-S203 §4–5 et A246.

## L282 — Un ordre n'est pas un périmètre, et une glose écrite sous le nom de l'utilisateur hérite de son autorité

*(S204)* L'utilisateur a donné un ordre de construction — rendre l'eau visible, fixer le budget,
puis construire des effets bornés. La décision qui l'a consigné l'a écrit comme un périmètre :
δ « borné par son cas d'usage », V « en attente d'un besoin gameplay ». Deux sessions et deux
agents l'ont ensuite recopié dans un ADR, la file active, le document de reprise et un rapport
de validation. Il a fallu que l'utilisateur rappelle que ses ambitions initiales n'avaient
jamais changé.

Deux mécanismes, et ils se renforcent. **Le glissement** : « d'abord X » devient « seulement X »
dès qu'on écrit ce qui n'est *plus préalable* sans écrire ce qui *reste dû*. « Un solveur général
ne conditionne plus l'affichage » était vrai ; « revenir à un δ général demande une nouvelle
décision » ne découlait de rien. **L'autorité empruntée** : un ADR marqué « direction explicite
de l'utilisateur » ne se relit pas comme une interprétation. Les sessions suivantes ne vérifient
pas la demande ; elles appliquent la décision.

Le geste, pour toute décision de priorité, de budget ou d'ordre : **écrire ce qu'elle ne retire
pas**, en une phrase, et le confronter aux intentions d'origine (`docs/sources/`, ADR-001). Et
quand une décision transcrit une demande : **citer les mots de l'utilisateur à part, et marquer
la glose comme glose**. Une restriction de périmètre qui ne cite aucune phrase de l'utilisateur
qui la demande n'est pas une décision de l'utilisateur. Voir ADR-127, A248.

**Corollaire S205 à L280.** Le même défaut vit dans les **essais** : sept essais de refus de la
composition produisaient leurs verdicts avec des mers de 1 à 10 cm, choisies pour isoler un
mécanisme, et aucun ne pouvait voir qu'une mer de jeu dépassait seule la limite. Un essai qui
isole un mécanisme doit être accompagné d'**un point à paramètres du produit** — pas pour tester
le mécanisme, pour tester que le mécanisme laisse passer le produit. Voir A249.

## L283 — Un coût unitaire ne se confronte à un budget qu'à travers la charge que l'observateur exige

*(S206)* S203 avait mesuré 14 µs par point B+W et écrit « environ 140 points dans 2 ms ». Le
nombre était juste, et il ne disait rien : combien de points l'image demande-t-elle ? S206 l'a
dérivé de l'observateur — deux échantillons par plus courte longueur d'onde visible, soit un
sommet tous les 2,75 px au point d'impact — et la charge est de 36 000 sommets, dont 86 % dans
l'emprise parce qu'une grille projetée est dense là où l'observateur regarde. À cette charge, le
coût unitaire donne 280 ms ; à une charge choisie pour être confortable, 17 ms.

Ce qui généralise : **un budget ne se confronte ni à un coût par point, ni à une charge choisie,
mais à la charge dérivée de ce qui doit rester visible**. Une densité réduite n'est pas une
dégradation neutre si elle passe sous la fréquence de ce qu'on voulait montrer : elle retire la
fonctionnalité en silence, ce qu'ADR-127 interdit.

Le geste : avant toute campagne de coût, écrire la chaîne observateur → plus courte longueur
d'onde à conserver → densité de Nyquist → nombre de points, et mesurer **à** cette densité ; les
densités plus lâches se publient comme bornes basses, jamais comme options. Et vérifier où tombe
la charge : ici, dans l'emprise la plus chère. Voir COUT-IMAGE-S206 §2, A250.

## L284 — Un seuil de réception tiré d'un instant ne vaut pas pour l'horizon

*(S208)* S206 avait mesuré l'erreur d'une table radiale à λ/8 : 0,090 mm, à un instant, +3 s. S208
en a fait un critère de réception « sur toute l'emprise et tout l'horizon », déclaré avant la mesure
comme il se doit — et il a échoué à l'instant zéro, 0,182 mm, parce qu'un impact naît compact : son
pic central a la courbure la plus forte de toute sa vie, et l'erreur d'Hermite croît comme la
dérivée quatrième.

La déclaration préalable a fait son travail : l'échec a été vu, consigné avant retouche, et c'est
le pas qui a changé, pas le seuil. Ce qui a manqué est en amont : **un seuil se tire du pire cas
du domaine qu'il couvre**, pas d'un point de mesure pris pour commode. Pour un champ qui évolue, le
pire cas se cherche dans le temps comme dans l'espace ; pour une interpolation, il est là où la
courbure est maximale, et on sait souvent où c'est avant de mesurer.

Le geste : quand un seuil déclaré vient d'une mesure antérieure, relire **sur quel sous-domaine**
cette mesure portait. S'il est plus petit que celui du critère, mesurer d'abord le pire cas connu
— ici l'instant initial — ou déclarer le critère sur le sous-domaine réellement couvert. Voir
ADR-129 (note S208).

## L285 — Une interface pensée pour la requête ponctuelle se paie par image quand l'image la consomme telle quelle

*(S212)* Deux fois de suite, une couche W a été branchée dans l'image par son interface de
service, et deux fois le coût dominant était un travail **qui ne dépend pas de l'image**. En S203,
l'impact réévaluait par sommet 256 fonctions de Bessel de `k·r`, indépendantes du temps : la table
d'ADR-129 l'a divisé par cent. En S212, le sillage refait à chaque image la préparation modale de
4 096 nœuds × 8 tronçons (10,9 ms), alors que dans la solution de Duhamel un tronçon achevé ne
fait plus que tourner. Dans les deux cas l'interface était juste pour ce qu'elle servait : une
requête à un instant, sur quelques points, où préparer est gratuit rapporté à l'appel.

Ce qui généralise : **une requête ponctuelle amortit sa préparation sur un appel ; l'image
l'appelle à tous les points et à tous les instants**. Ce qui est constant par requête devient
proportionnel à la cadence, et ce qui est constant par point devient proportionnel aux sommets.

Le geste : avant de brancher une couche dans l'image, écrire pour chaque étape de sa requête de
quoi elle dépend — instant, point, ni l'un ni l'autre — et sortir de la boucle d'image tout ce qui
ne dépend pas de ce qu'elle fait varier. Mesurer ensuite à deux formats et deux résolutions de
recette : deux lois linéaires sur des variables différentes s'y séparent, une seule mesure les
confond. Voir HOTE-GPU-S212, ADR-129, L283.

## L286 — Un verdict de coût porte le nom de l'implémentation mesurée, et la liste de ce qui lui manque

*(S213)* S212 a mesuré juste et conclu faux : « coût refusé » pour un sillage dont
l'implémentation n'avait ni LOD, ni visibilité, ni mutualisation, ni repli du temps, puis « si les
deux leviers ne tiennent pas 2 ms, l'arbitrage revient à l'utilisateur ». L'utilisateur a corrigé
le jour même (ADR-131). Le glissement tient à deux mots. **« Refusé »** vient du vocabulaire de
réception, où il juge un critère ; appliqué à un budget, il juge la fonctionnalité. **« Les
leviers »** au pluriel défini laisse croire que les deux premières idées épuisent l'espace, et
transforme leur échec éventuel en fin de partie. C'est A248 — un ordre lu comme un périmètre — par
le chemin du coût.

Ce qui généralise : **une mesure de coût est une coordonnée dans un espace de techniques, pas un
verdict sur ce qu'on voulait construire**. Sans la liste des techniques absentes, un lecteur ne
peut pas distinguer « impossible » de « pas encore optimisé », et l'auteur non plus.

Le geste : ouvrir chaque mesure de coût par trois lignes — techniques présentes, techniques
absentes, domaine de validité — et réserver « reçu / refusé » aux critères déclarés. Avant de
proposer une suite, énumérer l'espace entier (temps, espace, LOD spatial, spectral, temporel,
visibilité, mutualisation, et ce que le cas ajoute) plutôt que les deux idées les plus proches. Ne
jamais écrire la condition sous laquelle on demanderait de réduire l'ambition. Voir ADR-131, A252.

## L287 — Deux couches évaluées au même endroit logique ne le sont pas au même point numérique

*(S214)* La composition du cœur et la somme à la main de l'hôte donnaient la même eau à 1,8e-5 m
près — dix-huit fois le critère déclaré, sur une scène où tout le reste était exact au bit. La
cause n'était dans aucune des deux physiques : l'hôte évaluait **B** au point monde quantifié
(`WorldPos`, 1/2048 m) et les **perturbations** au point `f32` brut. Une même sonde avait deux
positions, distantes d'au plus 244 µm, et le produit par la pente rendait les 18 µm.

Ce qui généralise : **quand deux couches d'un même modèle acceptent des représentations de position
différentes, un appelant les mélange sans le voir**, et l'écart qui en résulte imite exactement une
erreur de physique — il est petit, lisse, et croît avec la pente. Aucun test de couche ne l'attrape,
parce que chaque couche est juste à son point. Le cœur s'en protégeait déjà par construction — une
conversion monde → local servie aux trois couches — et c'est précisément cette protection que
l'hôte contournait en reconstruisant ses points lui-même.

Le geste : quand deux chemins qui doivent dire la même chose divergent au-delà de leur arrondi,
**comparer d'abord au même point, avant de soupçonner le calcul**. Et, en composant, dériver tous
les points d'une conversion unique — même quand l'interface publique de chaque couche accepte
volontiers un point brut.

## L288 — Un majorant additif transforme le nombre de sources en budget, et personne ne le mesure composé

*(S214)* Le budget de pente refuse sur la **somme** des majorants des perturbations, source par
source. Chaque moitié avait été admise seule — l'impact en S205, le sillage en S212 — et chacune
passait largement. Composées, elles occupent **84 %** de π/7, pour une pente réelle dix fois plus
faible : la troisième source refuserait toute l'image, par marge et non par raideur. Le défaut
n'est visible dans aucune mesure de source unique, et il ne se déduit d'aucune d'elles sans faire
l'addition explicitement.

Ce qui généralise : **une borne additive fait du nombre d'éléments une ressource rare, et un banc
qui n'exerce qu'un élément ne peut pas le montrer**. Le pessimisme de chaque majorant, inoffensif
seul, devient le facteur d'échelle du système entier. C'est A208 vue par l'autre bout : là où A208
regardait ce qu'une emprise consomme, ici c'est le **cardinal** qui consomme.

Le geste : pour toute grandeur bornée par une somme de majorants, publier **l'occupation** —
part du budget consommée — et non seulement le verdict d'admission, et mesurer au moins deux
éléments ensemble avant de conclure qu'une couche tient. Voir A254, ADR-119 règle 1, ADR-128.

## L289 — Une mesure de coût sur une machine portable dit son rang de passage

*(S214)* Le même binaire, trois passages : 1,25 ms, puis 1,65, puis 1,58–1,84 — 20 à 40 % d'écart
sur une médiane de 120 images, la valeur basse tombant sur le passage à froid. S213 avait attribué
un écart de cette taille à une différence entre son exemple et son hôte, et l'avait laissé « non
attribué ». Il n'y avait pas de différence à expliquer : il y avait un ordre de passage.

Ce qui généralise : **sur une machine à gestion thermique agressive, la répétition intra-passage ne
mesure pas la dispersion inter-passage**. Cent vingt images consécutives donnent une médiane serrée
et fausse ; c'est entre les exécutions que vit la variance. Un bloc de chauffe (A195) protège du
premier effet, pas de celui-ci.

Le geste : exécuter au moins deux passages séparés avant de publier un coût, publier leur écart, et
dire le rang du passage cité. Ne comparer deux chemins qu'à rang égal.

## L290 — Un majorant en norme L1 est invariant par dispersion ; le champ qu'il borne ne l'est pas

*(S215)* Le budget de pente de la composition refusait une scène à deux sources alors que la pente
réelle y était dix fois plus faible. La cause n'était ni l'emprise, ni l'alignement, ni l'unité :
le majorant est une **somme de modules modaux**, et après extinction de la source chaque mode ne
fait plus que tourner — la somme des modules ne bouge donc plus, tandis que le maximum **spatial**
décroît à mesure que les phases se décohèrent. Mesuré : facteur 1 à la naissance, 4 à quatre
secondes, 30 à cinquante-six.

Ce qui généralise : **toute annonce construite comme une somme de modules se périme avec l'âge de
ce qu'elle décrit**, et elle se périme d'autant plus vite que le milieu est dispersif. L'annonce
reste vraie — c'est bien un majorant — mais elle cesse d'être informative, et un contrat qui la
consomme devient progressivement un contrat sur le passé. Le piège est qu'elle passe tous les tests
de sûreté : on ne vérifie jamais qu'un majorant est *serré*, seulement qu'il n'est pas dépassé.

Le geste : pour toute grandeur annoncée comme borne, mesurer **son rapport à la grandeur bornée en
fonction du temps**, pas seulement à l'instant où elle est établie. Si le rapport dérive, chercher
l'échelle de temps propre du phénomène et l'annoncer en fonction d'elle. Voir ADR-133, A254.

**Note corrective S217 — 2026-09-13 (A257).** L290 suppose des coefficients qui
évoluent chacun par une rotation pure. La hauteur complexe d'un mode de pression
ne satisfait pas cette hypothèse : elle mélange hauteur et vitesse de l'instant
d'extinction. Son module peut varier ; les deux amplitudes propres tournent, et
l'énergie du couple est conservée. Le constat de pessimisme demeure, l'énoncé
« somme des modules ne bouge plus » ne se transporte pas à ce coefficient.

## L291 — Une grille qui n'a pas la résolution du pic attribue le pessimisme au mauvais terme

*(S215)* S214 a relevé la pente réelle d'une scène composée sur une grille de 1,3 m. Le maximum de
pente d'un impact radial est atteint en `r = 0,2062 λ`, soit 0,69 m : la grille passait à côté par
construction. Le **chiffre conjoint** s'en est trouvé à peu près juste — le sillage, mieux résolu,
dominait le relevé — mais l'**attribution** était fausse de bout en bout : le pessimisme venait
presque entièrement de l'impact, que la grille ne voyait pas, et non du terme qu'elle mesurait.

Ce qui généralise : **un maximum sous-échantillonné ne se trompe pas au hasard, il se trompe en
faveur du terme le mieux résolu**. Le total peut rester plausible alors que sa décomposition est
inversée — et c'est la décomposition qui dit où porter le remède. Un remède dirigé par un relevé
trop grossier vise le mauvais champ.

Le geste : avant de relever un maximum, écrire **où il est censé se trouver et à quelle échelle**,
puis vérifier que le pas y met plusieurs points. Quand la structure est connue — ici, un profil
radial dont ADR-094 donne le rayon du pic —, échantillonner le long de cette structure plutôt que
sur une grille uniforme. Et raffiner localement autour de l'argmax plutôt que raffiner partout :
c'est le carré du gain en moins pour le même résultat.

## L292 — Avant d'acter une similitude, faire varier le paramètre qu'on n'a pas fait varier

*(S215)* La décroissance du pessimisme s'effondrait parfaitement sur l'âge adimensionné `t/√(λ/g)`
— identique à trois décimales sur quatre longueurs d'onde et cinq énergies. Tout était prêt pour
l'ADR. Restait une dimension jamais variée : **la profondeur**, dont la relation de dispersion
dépend, et qui valait 20 m dans toutes les mesures. Trois minutes pour la faire varier, et deux
réponses : les colonnes sont identiques de 20 à 4 m, et sous 4 m le constructeur **refuse le champ
lui-même**. Le domaine où la similitude vaut est donc exactement celui où l'objet existe.

Ce qui généralise : **un effondrement parfait sur les paramètres qu'on a variés ne dit rien de
ceux qu'on n'a pas variés**, et la conviction qu'il procure est exactement ce qui dissuade d'aller
voir. La vérification manquante est en général bon marché — c'est le même banc avec une valeur de
plus — et son résultat est utile dans les deux sens : elle élargit le domaine, ou elle le borne.

Le geste : avant d'acter une loi d'échelle, **lister les paramètres du problème et pointer ceux qui
sont restés constants**. Faire varier chacun au moins une fois. Et regarder si l'objet se refuse
lui-même hors du domaine mesuré : une réserve qu'un constructeur applique déjà n'a pas à être
écrite dans l'ADR.

## L293 — Avant de calibrer une loi, chercher la part que l'algèbre retire gratuitement

*(S216)* La suite prescrite était de refaire sur le sillage la campagne mesurée qui avait traité
l'impact : varier les paramètres, chercher une échelle de temps, tabuler un rapport, poser une
garde. La lecture du majorant a montré qu'une partie de son pessimisme ne demandait **aucune
mesure** : il sommait scalairement des contributions vectorielles de directions différentes, et une
inégalité de Cauchy–Schwarz en retire 20 % en un passage, sans table, sans garde, sans domaine de
validité. Calibrer d'abord aurait fait payer une campagne pour un écart dont une partie s'annule
par le calcul — et l'aurait fait payer **deux fois**, puisque la table aurait ensuite dû être
refaite sur le majorant resserré.

Ce qui généralise : **un écart mesuré entre une borne et ce qu'elle borne se décompose avant de se
calibrer**. Certaines parts sont structurelles — une inégalité trop lâche, une norme mal choisie,
une projection oubliée — et se corrigent exactement ; d'autres sont physiques et ne se connaissent
que par la mesure. La tentation est d'attaquer le total, parce que c'est lui qu'on a mesuré.

Le geste : devant un majorant trop large, écrire **ce qu'il suppose** — ici, un alignement
simultané en phase et en direction — puis se demander, pour chaque hypothèse, si elle est fausse
*par construction* ou seulement *en général*. Les premières se retirent gratuitement. Ne mesurer
que ce qui reste. Voir ADR-134, A255.

## L294 — Un test qui fige la cause d'un refus devient un test du pessimisme du majorant

*(S216)* Resserrer une borne a fait passer un verdict de `SlopeEnvelope` à `Slope` : à limite
fixée sous le plancher, la limite tombait désormais sous la **pente réelle au point**, et le refus
devenait attribuable au champ plutôt qu'à la marge. Le test qui exigeait `SlopeEnvelope` a cassé,
alors que ce qu'il éprouvait — un point admis géométriquement que la requête refuse tout de même —
n'avait pas bougé.

Ce qui généralise : **quand deux causes de refus se distinguent par la position d'un seuil, figer
laquelle survient fait du test une mesure de ce seuil**, c'est-à-dire exactement de la quantité
qu'on cherche à améliorer. Le test se met alors à défendre le défaut : toute amélioration le casse,
et la pression est de renoncer à l'amélioration plutôt qu'au test.

Le geste : dans un test dont le sujet est *qu'il y a refus*, assertez l'ensemble des verdicts
acceptables, et réservez l'égalité stricte aux tests dont le sujet **est** la cause — ceux-là
doivent alors construire le cas qui la force, pas l'obtenir par hasard.

## L295 — Un discriminant coûte moins qu'une campagne, et il en supprime une

*(S216)* Deux mécanismes pouvaient expliquer le pessimisme résiduel : la décohérence des phases, ou
une emprise trop petite pour contenir le point où elles s'alignent. Le second relève de l'hôte et
d'aucune loi mesurée ; le premier demande une campagne complète. Les distinguer a coûté une boucle
de trois lignes — agrandir l'emprise à âge fixé, à pas de grille **constant** — et le résultat est
tombé net : maximum identique à six décimales pour seize fois l'aire.

Ce qui généralise : **avant d'ouvrir une campagne, chercher l'expérience qui élimine l'une des
hypothèses**. Elle est presque toujours beaucoup moins chère que la campagne, parce qu'elle n'a pas
besoin d'être précise — seulement discriminante. Et elle protège d'un gaspillage asymétrique :
mesurer finement un phénomène qui n'est pas la cause ne produit rien du tout.

Le geste : écrire les hypothèses concurrentes, puis pour chacune la **prédiction qui la sépare** des
autres, et exécuter d'abord celle qui est la moins chère. Veiller à ce que le protocole ne rende pas
le test vide : ici, faire croître le pas de grille avec l'emprise aurait fait lire un maximum manqué
comme un maximum absent.

## L296 — Une similitude ne supprime pas les paramètres qu'elle conserve

*(S217)* Trois tailles de sillage rendent les mêmes rapports à six décimales quand
longueurs, durées, vitesses, bande et charge sont mises à l'échelle ensemble. Mais
faire varier seulement la vitesse ou la durée déplace le rapport de 38,8 % et 22,5 %
à âge réduit égal, après raffinement. Les courbes coïncidaient parce que les groupes
sans dimension restaient identiques, pas parce qu'ils avaient disparu du problème.

Avant de déclarer une loi universelle, écrire les groupes que la transformation
**conserve**, puis en varier un indépendamment. Comparer à âge absolu égal des
sources de durées différentes confond aussi deux effets : âge depuis extinction
et histoire du forçage. Un temps réduit organise une famille ; il ne la remplace pas.

## L297 — Une borne doit porter la représentation exécutée

*(S218)* Une borne de Hessienne couvre une somme trigonométrique continue. Le code
évalue des produits f32 puis des phases Q32 : la fonction exécutée comporte des sauts
que la seule distance géométrique ne décrit pas. La monotonie des produits arrondis
aux extrémités du domaine permet de borner leur variation sans supposer leur continuité.

Avant de transporter une preuve dans le code, lister les transformations entre l'objet
mathématique et sa représentation. Séparer preuve algébrique, réserve d'arrondi et
contre-épreuves ; une campagne favorable ne convertit pas la troisième en la première.

## L298 — L'adaptation ne crée pas l'information de sa priorité

*(S219)* Le tas raffine la plus grande borne, mais les grandes régions publient toutes
le même plafond global.8191 évaluations ne resserrent rien. Le parcours devient utile
plus tard : il fallait d'abord payer des raffinements essentiellement géométriques.

Avant de changer un ordonnanceur, vérifier que son indicateur distingue effectivement
les objets au niveau où il les choisit. S'il est saturé, améliorer l'indicateur ou sa
résolution peut être nécessaire ; réordonner ses égalités ne révèle aucune information
nouvelle. Publier aussi le coût d'obtention de cette information, pas seulement celui
de la sélection qui la consomme.

## L299 — Un minimum de branches cache celle qui a gagné

*(S220)* La borne publiée est le minimum de plusieurs branches, chacune avec sa réserve. En
cours de campagne, un agrégat semblait placer la borne sous « maximum + réserve d'ordre
deux » — donc une branche fausse. La feuille maximale était en fait plafonnée par l'autre
branche, dont la réserve est deux fois plus petite. Le même relevé a ensuite montré le vrai
plancher (la réserve, pas la géométrie) et la vraie cause du plateau (les modes non résolus).

Quand une grandeur est un minimum, un maximum ou une somme de termes hétérogènes, publier sa
**décomposition sur l'élément qui la fixe** — quelle branche, quel terme, à quelle échelle —
avant d'interpréter l'agrégat ou de choisir le levier suivant. Le calculer hors
chronométrage coûte une évaluation ; le deviner a coûté une campagne arrêtée.
## L300 — Un écart de coût entre variantes peut être du code machine

*(S221)* La passe spectrale, qui fait plus d'opérations que la passe d'ordre deux, coûtait 16 %
de moins, dans les deux ordres d'une micro-mesure alternée. En faisant passer l'ordre deux par la
même version compilée (classes calculées puis jetées), il est tombé au même coût, à bits
identiques. L'écart venait de la génération de code, pas de l'algorithme.

Avant d'attribuer un écart de coût à ce que fait une variante, le mesurer en alterné dans un même
processus, puis **échanger le chemin compilé à sémantique égale**. Si l'écart suit le chemin, c'est
une marge d'implémentation, réelle mais fragile. Il faut la publier comme telle, et ne pas l'adopter
sans en comprendre la cause.

## L301 — Compter n'est pas peser

*(S221)* S220 avait attribué le reste des grosses mailles aux « 1608 modes exclus sur 4096 », le
groupe le plus nombreux. Ils portaient 1,1 à 2,2 % de la masse, et le remède qu'on en tirait, la
coupure garantie à `D ≥ 2`, n'a presque rien apporté. La contribution venait d'une classe moins
nombreuse et beaucoup plus lourde. S220 avait écrit « répartition non mesurée », et c'est ce qui a
permis de ne pas bâtir la décision sur la seule coupure fausse.

Quand un agrégat est une somme pondérée, l'attribuer par la **masse** de chaque groupe, jamais par
son effectif. Et quand la décision doit précéder la mesure, construire une famille qui contient
l'hypothèse et ses voisines, plutôt que la seule hypothèse.
## L302 — Un gain réel peut être structurellement inutilisable ; publier sa loi d'échelle, pas son facteur

*(S222)* La borne locale partitionnée rend exactement ce que l'enveloppe globale perd : 2,32 fois le
budget sur une scène à trois sillages séparés, et elle colle au maximum réel à 1 %. Publier ce
facteur seul aurait annoncé un progrès. Il coûte **25 secondes** contre un budget d'image de 2 ms,
et ce prix n'est pas une constante à optimiser : les mailles utiles doivent être **sous-ondulatoires**
— au-delà d'une demi-longueur d'onde, tous les modes retombent dans la classe non résolue et la
borne redevient l'enveloppe globale —, leur nombre croît donc comme l'aire divisée par `λ_min²`, et
chaque maille est `O(N)`. Quatre ordres de grandeur, défendus par une loi.

Ce qui généralise : **un facteur de gain sans son exposant de coût n'est pas un résultat, c'est une
moitié de résultat** — et c'est la moitié qui donne envie de construire. Quand le prix suit une loi
d'échelle imposée par la physique du problème (une longueur d'onde, un rayon de corrélation, une
échelle de maillage), aucune optimisation d'implémentation ne la franchit, et le dire économise la
session qui aurait essayé.

Le geste : à côté de tout gain, écrire **ce qui fixe son prix** et si ce quelque chose est une
constante ou un exposant. Chercher la taille de maille à laquelle le gain disparaît — elle existe
presque toujours, et elle est la loi. Voir ADR-137, A261, SOMME-SILLAGES-S222 §3.

## L303 — Une borne « locale » ne l'est que sous l'échelle du phénomène qu'elle borne

*(S222)* J'attendais d'un appel sur un petit rectangle qu'il soit à la fois serré et bon marché, et
qu'on puisse donc opposer « requête locale » à « partition coûteuse ». Les deux attentes sont
fausses. **Serré** : au-delà d'un mètre de demi-côté — la plus courte longueur d'onde représentée
valant 2,09 m —, la borne locale **vaut exactement l'enveloppe globale**, à la réserve numérique
près ; elle ne devient informative qu'en dessous. **Bon marché** : l'appel est `O(N)` sur tous les
modes quelle que soit la taille du rectangle, donc 670 µs et non quelques microsecondes — la
localité du rectangle ne réduit pas le travail, elle ne fait que resserrer le résultat.

Ce qui généralise : **le mot « local » recouvre deux propriétés indépendantes** — la borne est-elle
plus serrée sur une petite région, et coûte-t-elle moins cher à y calculer ? Une représentation
spectrale donne la première sous l'échelle de son mode le plus court, et **jamais** la seconde,
puisque chaque mode a un support infini. Supposer les deux ensemble fait espérer un raccourci qui
n'existe pas.

Le geste : devant une annonce « locale », demander séparément **à partir de quelle taille elle
resserre** et **ce que son calcul coûte en fonction de la taille**. Si le coût est indépendant de la
région, la localité est un gain de précision, pas un gain de travail — et elle ne s'oppose pas à une
partition, elle en est la brique.

## L304 — Une inégalité de manuel a un rang, et ce n'est pas toujours celui qu'on emploie

*(S223)* J'ai fondé toute une session sur `|J_ν(x)| ≤ √(2/πx)`, souvenue comme « la » borne des
fonctions de Bessel. Elle est vraie pour `ν = 1/2`, où elle est une **égalité** — `J_{1/2}(x) =
√(2/πx)·sin x` —, et **fausse pour `ν = 1`** : `|J₁|` la dépasse de 3,4 % en `x = 2,166`.
L'asymptote est la limite en `+∞`, pas un majorant, et la mémoire les confond d'autant plus
volontiers que le cas `ν = 1/2` la rend exacte.

Ce qui généralise : **une inégalité classique se souvient sans ses hypothèses**, et l'hypothèse
oubliée est presque toujours le paramètre auquel on ne pensait pas — l'ordre, le signe, le domaine.
Le danger particulier est qu'une borne fausse « de peu » ne se voit pas : elle produit des résultats
plausibles partout sauf sur un intervalle étroit, et le programme y ment sans échouer.

Le geste : **vérifier numériquement toute inégalité empruntée, sur le domaine exact où on l'emploie
et sur la fonction que le programme exécute**, avant de bâtir dessus. Le coût est une boucle ; le
coût de ne pas le faire est une borne qui n'en est pas une. Et si la vérification échoue, relever
la constante qui la rend vraie plutôt que d'abandonner la forme : ici `sup |J₁|·√x = 0,825031` a
sauvé toute la construction. Voir ADR-138, COURONNE-IMPACT-S223 §1.

## L305 — Un majorant qui décroît vaut mieux qu'un majorant plus serré

*(S223)* Deux bornes du même champ radial : `slope_max_at` — calibrée, mesurée, serrée à 4,5 % près
au centre — et l'enveloppe par couronne, une inégalité *plus lâche au centre* mais **décroissante en
`1/√r`**. C'est la seconde qui a débloqué A262, parce que la question n'était pas « quelle est la
pente maximale ? » mais « deux champs distants peuvent-ils atteindre leur maximum ensemble ? ». Une
borne plate ne peut pas répondre non ; une borne décroissante le peut, même mal.

Ce qui généralise : **la qualité d'un majorant ne se mesure pas seulement à sa valeur, mais à ce
qu'il fait varier**. Une borne constante est inexploitable par toute inégalité géométrique, quelle
que soit sa finesse ; une borne monotone en un paramètre autorise l'inégalité triangulaire, la
composition, le balayage — et donc des gains qu'aucun resserrement de la borne plate n'atteindrait.
La comparaison entre S222 et S223 le chiffre : même gain d'environ 2,3, obtenu en 25 s côté
pression (où la borne est plate en espace) et en 13 µs côté impact (où elle décroît).

Le geste : devant une borne à améliorer, demander d'abord **de quoi elle pourrait dépendre** — le
temps (ADR-133), la direction (ADR-134), la distance (ADR-138) — avant de chercher à la resserrer à
paramètre fixé. Et garder les deux : leur **minimum** choisit la meilleure sans mélanger preuve et
calibration.

## L306 — Un état entier a un plancher, et il n'est pas dans l'état mais dans ce qu'on en dérive

*(S224)* Le noyau V porte son état en millilitres entiers, et c'est là qu'on cherche naturellement
les effets de quantification. Le plancher était ailleurs : dans la **hauteur**, dérivée du volume et
exprimée en micromètres entiers. À un millilitre dans un réservoir d'un mètre carré, la hauteur vaut
un micromètre ; l'interpolation la rendait nulle, la charge s'annulait, et le contenant cessait de
se vider tout en contenant de l'eau. Le symptôme — une vidange qui s'arrête — ressemble à une erreur
de physique et n'en est pas une.

Ce qui généralise : **quand une grandeur dérivée rentre dans le calcul du taux qui fait évoluer
l'état, c'est sa résolution à elle qui fixe le point mort**, pas celle de l'état. Le piège est que
la grandeur dérivée est souvent choisie plus fine que l'état — ici micromètres contre millilitres —
ce qui donne l'impression rassurante d'être du bon côté, alors que le rapport des deux dépend de la
**géométrie** : le plancher vaut la surface du contenant multipliée par une unité de hauteur, donc
un millilitre pour un mètre carré et **dix litres pour un hectare**.

Le geste : pour chaque grandeur dérivée qui reboucle sur le taux, écrire **à quelle valeur de l'état
elle devient nulle**, et vérifier que ce point est bien là où on l'accepte. Arrondir au plus proche
plutôt que tronquer recule le plancher d'un facteur deux, jamais davantage. Voir A264, ADR-010 §2.

## L307 — Normaliser avant de quantifier annule ce que la quantification devait produire

*(S224)* ADR-010 demande que plusieurs arêtes vidant le même contenant soient réduites « dans la
même proportion ». Fait avant la quantification — sur les nanolitres —, ce partage donne à chaque
arête une part inférieure au millilitre, qui s'arrondit à zéro : un nœud de deux millilitres avec
trois fuites les gardait indéfiniment, avec de la charge, sans que rien ne signale d'anomalie.
Chaque étape était correcte ; leur ordre ne l'était pas.

Ce qui généralise : **un partage proportionnel et une quantification ne commutent pas**, et
l'ordre qui paraît naturel — répartir finement, puis arrondir — est celui qui perd. Il faut
quantifier d'abord, puis répartir **dans l'unité que l'état porte réellement**, sinon la
répartition distribue des quantités que l'état ne sait pas représenter. Le même piège guette partout
où une ressource entière est partagée : budgets, emplacements, jetons.

Le geste : répartir par **arrondi cumulatif** — la part du `k`-ième est la différence des sommes
proportionnelles arrondies jusqu'à `k` et jusqu'à `k−1`. Les parts somment exactement au total,
chacune est à moins d'une unité de sa valeur proportionnelle, et l'ordre du tableau suffit à la
reproduire : aucun reste à stocker, donc aucune source de divergence. Voir NOYAU-V-S224 §3.

## L308 — Une cadence est un intervalle, et mesurer ses parties les change

*(S225)* Le coût d'une image était connu depuis S211 par deux nombres — un CPU, un GPU — obtenus
hors écran, sur une texture, avec une réserve écrite dans la ligne : *« sky, upload, readback,
presentation excluded »*. La cadence réelle, elle, n'avait jamais été mesurée, et trois obstacles
expliquent pourquoi personne ne s'y était risqué : sous **vsync** l'intervalle mesure l'écran et non
le coût ; **relire un horodatage GPU sérialise** le processeur et la carte, donc détruit le
recouvrement que l'on prétend chronométrer ; et **acquérir une image de la chaîne d'échange bloque**,
de sorte que le temps « CPU » d'une trame réelle est à moitié de l'attente — 2,22 ms sur 4,32 ici.

Ce qui généralise : **une cadence n'est pas la somme de ses passes**, et l'instrumentation qui la
décompose la modifie. Les deux mesures — le rythme et sa répartition — ne peuvent pas être prises
dans le même passage, et présenter l'une sous le nom de l'autre est une erreur de catégorie, pas
d'arrondi. Le symptôme est qu'un banc hors écran donne des chiffres **plus petits et plus stables**
que la réalité, ce qui les rend d'autant plus convaincants.

Le geste : mesurer le rythme **sans instrumentation fine**, présentation comprise et synchronisation
d'écran désactivée ; mesurer la répartition dans un **second passage**, et l'étiqueter comme
sérialisée. Puis vérifier que la somme des parties approche le tout — si elle ne l'approche pas, ce
n'est pas la mesure qui est fausse, c'est le modèle de recouvrement qu'on avait en tête. Voir A265.

## L309 — La pose d'une fixture peut être son pire cas sans que personne ne l'ait choisi

*(S225)* Le coût GPU de l'eau valait 4,16 ms depuis S212, mesuré à la caméra de la scène S201, fixe.
En faisant tourner la caméra sur les mêmes 590 images, la médiane tombe à **2,85 ms** et le maximum
atteint 4,23 : la valeur publiée pendant treize sessions était à peu près le **maximum** d'un
balayage, pas sa médiane. Le facteur au budget passe de 2,08 à 1,42 en médiane. Personne n'avait
choisi cette pose pour sa sévérité — elle venait de S201, où elle servait à regarder une image.

Ce qui généralise : **une fixture héritée porte les intentions de la session qui l'a créée, pas
celles de la session qui la mesure**. Une pose choisie pour montrer un phénomène le cadre au mieux,
donc met le plus de travail à l'écran ; reprise telle quelle comme point de mesure, elle devient un
pire cas silencieux. Le biais est systématique et va toujours dans le même sens.

Le geste : pour toute grandeur mesurée sur une pose, une graine ou un instant hérités, **balayer ce
paramètre au moins une fois** et publier médiane et maximum plutôt qu'une valeur unique. Si la
valeur héritée se révèle extrême, le dire — elle reste utile comme borne, à condition d'être nommée
comme telle.

## L310 — Un cas canonique bien choisi peut être muet sur un invariant

*(S226)* Le premier module de V recevait `g_eff` en **module** : la direction était en dur, ce
qu'I-07 qualifie de « défaut bloquant ». **C12 passait quand même**, à 0,08 % de sa référence
analytique, parce qu'un réservoir posé à plat ne distingue pas une gravité dirigée d'une gravité
scalaire. Le cas était bien conçu, la mesure honnête, la réception réelle — et l'invariant violé.

Ce qui généralise : **un cas de validation éprouve ce qu'il met en jeu, et rien d'autre**. Un
invariant qui porte sur une **généralité** — un référentiel quelconque, une orientation quelconque,
un nombre quelconque de sources — n'est pas éprouvé par un cas qui fixe ce paramètre à sa valeur la
plus simple, même quand ce cas est exigeant par ailleurs. Le danger particulier est que la réussite
du cas donne l'impression d'une couverture qu'elle n'a pas, et d'autant plus qu'elle est serrée.

Le geste : pour chaque invariant, tenir la **liste des cas qui le mettraient en défaut**, séparément
de la liste des cas qui valident la physique. Quand aucun cas n'y figure, l'écrire — c'est une
dette, pas un silence. Ici, C16 était ce cas et il existait depuis S01 ; personne ne l'avait relié à
V. Voir I-07, C12, C16.

## L311 — Deux dispositions d'un même paragraphe peuvent se contredire là où chacune sert

*(S226)* ADR-010 §2 demande deux choses : que la table `volume → hauteur` soit cuite hors ligne « par
coupes **horizontales** », et que le plan d'eau soit « perpendiculaire à `g_eff`, pas à `Z` ». Les
deux sont justes, écrites à trois paragraphes d'écart, et **incompatibles dès que `g_eff` penche**.
Mesuré : pour un prisme la table reste exacte au milieu de sa course — le coin gagné d'un côté vaut
celui perdu de l'autre —, mais pour une **coque en V** elle se trompe de **9,89 %**, et c'est
exactement le cas que l'ADR invoque pour justifier la table.

Ce qui généralise : **une contradiction entre deux dispositions d'un même document ne se voit pas à
la lecture**, parce que chacune est lue dans son propre contexte et paraît raisonnable. Elle
n'apparaît qu'à la construction, et seulement si l'on construit les deux **ensemble** — ici, vingt-
cinq sessions après l'ADR, et seulement parce qu'une session a implémenté la seconde en ayant déjà
la première. Le cas le plus traître est celui où les deux coïncident dans le cas trivial : le prisme
cache l'incohérence, la cale la révèle.

Le geste : quand une décision pose une **représentation** (une table, un format, un repère) et,
ailleurs, une **généralité** (un référentiel quelconque, une orientation quelconque), vérifier
explicitement que la représentation survit à la généralité — et mesurer l'écart sur le cas qui a
justifié la représentation, pas sur le cas facile. Voir A266.

*Correction factuelle S227, 2026-09-13.* Les mesures ci-dessus imposaient une cote centrale
verticale. Le module la consomme comme une distance normale : son prisme incliné est donc faux
aussi. Le « 9,89 % » de la cale est limité au domaine non tronqué du montage S226. Voir la note
corrective de [GRAVITE-DIRIGEE-S226](../docs/validation/GRAVITE-DIRIGEE-S226.md) et le suivi A266.
La leçon rejoint L258/L280 : éprouver la représentation **sur le chemin qui la consomme**.

## L312 — Borner chaque accès ne réserve pas une ressource partagée

*(S227)* Le limiteur de V comparait chaque arrivée à la place libre du receveur. Trois arêtes
voyaient chacune 1 ml libre et y plaçaient ensemble 3 ml. La conservation de masse passait :
elle ne contraint pas la répartition. La normalisation des sorties protégeait les sources,
pas le contenant qu'elles partageaient.

Le geste : lorsqu'une limite porte sur une ressource commune, vérifier **la somme des accès**
après leurs limites individuelles. Éprouver aussi un nœud qui reçoit et émet dans le même pas,
la saturation et un témoin non saturé. La réduction cumulative entière réserve ici exactement
la place disponible ; son ordre déterministe ne constitue pas une preuve multiplateforme.
Ce principe vaut pour un volume, un pool de mémoire ou un budget consommé par plusieurs tâches.

## L313 — Une densité partagée obéit à la couche la plus exigeante, le coût à la plus chère

*(S234)* Le projet cherchait depuis S212 un LOD « de la grille » : moins de sommets, moins de
coût. Le calcul fait avant de construire a montré que, sous la tolérance de l'image, la densité
du maillage était dictée par **B** — hessienne 0,349 sur 32 composantes —, alors que le coût
venait du **sillage** — 4 096 modes par sommet. Alléger le maillage retirait au mieux 35 %, et rien
en vue haute. Évaluer le sillage sur sa propre grille, au pas que **son** contenu autorise, puis le
reconstruire à chaque sommet, a divisé la passe par dix.

Ce qui généralise : **quand plusieurs couches partagent un échantillonnage, chercher d'abord
laquelle impose la densité et laquelle impose le coût.** Si ce ne sont pas les mêmes, aucun LOD du
support commun ne touche le produit qui coûte : il faut découpler les densités d'évaluation. Et
l'ordre de reconstruction compte autant que la grille : en linéaire, le même critère demandait
183 825 nœuds ; en bicubique avec dérivées exactes, 9 701. Voir L283 (la charge dérivée de
l'observateur), LOD-SILLAGE-S234 §1.

## L314 — Un verdict d'admission pris sur une scène âgée ne dit rien d'une scène qui se renouvelle

*(S235)* S222 avait écrit « trois sillages et huit impacts passent », et la file en avait tiré que
le budget de pente ne conditionnait plus la mutualisation. La phrase était juste à l'instant mesuré,
où tous les impacts étaient âgés. La scène S235, déclarée avant mesure avec des naissances toutes
les 4 s, est refusée sur 49 instants sur 161 : à chaque naissance, un impact neuf pèse 47 % du
budget, et les majorants des anciens n'ont pas le temps de décroître. Un premier essai de prédiction
avait lui aussi faussé l'instant, dans l'autre sens, en inscrivant les huit impacts dès 0 s.

Ce qui généralise : **un budget se juge sur la série temporelle de l'usage, renouvellements
compris**, et un « passe » publié porte l'instant et l'état des sources qui le rendent vrai.
Quand ensuite le verdict tombe, **séparer la grandeur réelle du majorant avant de conclure** : ici
aucune des 49 pentes réelles n'approchait le seuil, et c'est ce qui transforme un refus de scène en
travail de bornes au lieu d'une réduction d'ambition. Voir L283, A208, SCENE-MULTI-S235 §2.

## L315 — Une borne prouvée « pour tout point » l'est sur le domaine qu'on servait

*(S236)* ADR-138 écrivait que son inégalité valait « pour tout point ». Elle valait pour tous les
points que la requête **acceptait** : l'intersection des disques, donc le disque de l'ancre, que son
balayage parcourait. Personne ne l'avait écrit, parce que personne n'imaginait servir autre chose. Le
jour où le mode union est envisagé, la même borne devient fausse d'un tiers sur un contre-exemple de
trois impacts. Symétriquement, la requête et l'image décrivaient deux domaines différents depuis
trente-trois sessions (A271), sans qu'aucun test ne compare les deux sur une scène où ils diffèrent.

Ce qui généralise : **toute garantie porte le domaine qui la rend vraie, écrit à côté d'elle** — et
changer le domaine servi, même par ajout d'un mode, rouvre chaque preuve qui en dépendait avant la
première ligne de code. Le geste qui a suffi ici : chercher, pour chaque borne réemployée, le point du
nouveau domaine qu'elle ne regarde pas, et construire l'essai qui s'y place. Voir L310, ADR-138 note
S236, ADR-142.

## L316 — Avant de fixer une tolérance d'accord entre deux modèles, calculer l'écart physique qui les sépare

*(S237)* Le protocole de la surface mobile demandait qu'à `a = 1 cm` le mode mobile reste à 1 % du
mode linéaire, « la petite amplitude étant linéaire ». L'ordre deux, calculé une heure plus tard pour
l'oracle, donnait une harmonique de **1,5 % de `a`** à cette amplitude : un écart que le mode mobile
*doit* produire et que le mode linéaire ne peut pas produire. Le critère aurait échoué pour une raison
juste — ou, pire, un candidat qui l'aurait tenu aurait été suspect. Corrigé avant toute mesure,
à `a = 1 mm` (0,15 %) : mesuré 0,167 %.

Ce qui généralise : **une tolérance d'accord entre deux modèles différents doit être fixée au-dessus
de l'écart que la physique met entre eux, et le calcul de cet écart précède le chiffre.** « Petit »
n'est pas une amplitude : c'est un rapport, ici `ka·(1,48/k)`, qu'il faut évaluer. Même famille que
L283 : la charge et la tolérance se dérivent de ce qui doit être distingué, jamais d'une intuition
d'échelle. Voir SURFACE-MOBILE-S237 §1 (correction datée).

## L317 — Un plancher d'arrondi se certifie par une borne, pas par la répétition

*(S238)* Au pas que la pression f32 refusait, le résidu semblait **alterner entre deux valeurs**. Une
détection de cycle à deux relances l'aurait certifié dans un montage ; au pas visé, l'état parcourait
une période de **420 relances**, et dans le banc, sous un autre ordre de réduction, il fallait
**7 386 itérations** puis plus de 16 000 avant le retour au bit. La répétition existait, exacte et
déterministe ; sa longueur n'était bornée par rien. Ce qui était constant, en revanche, était l'erreur
inverse composante par composante : **0,9 à 1,3 unité d'arrondi** à chaque pas au plancher.

Ce qui généralise : **« l'itération ne peut plus rien gagner » se prouve en comparant le résidu à la
borne d'arrondi de son propre calcul** (`γ_n`, dérivée des opérations de la ligne), pas en attendant
que l'état revienne — le retour prouve la même chose, à un coût que personne ne peut promettre. Une
grandeur scalaire qui se répète (le résidu) ne dit pas la période de l'état. Et le prix est à écrire
avant de promettre « au bit » : un certificat qui arrête plus tôt change aussi des pas que l'ancien
critère aurait fini par accepter — ici par un tirage d'arrondi sous 10⁻⁶. Voir PRESSION-PLANCHER-S238
§3–4, ADR-143 ; même famille que L316 : le nombre vient de ce qu'on doit distinguer.

## L318 — Un seuil d'acceptation et une tolérance physique de normes différentes ne se bornent pas l'un l'autre

*(S239)* Le gradient conjugué de δ arrêtait sur un résidu **relatif en norme 2** (`ρ ≤ 10⁻⁶`) et le
candidat déclarait une tolérance en **norme maximale normalisée** (`D = max|div u|·dx/max|u| ≤ 10⁻⁵`).
Rien ne les reliait : `D = ρ·θ·Λ` exactement, et le critère d'arrêt ne borne que `ρ`. Mesuré de 128 à
32 768 mailles, `Λ` — la forme du second membre — **double à chaque raffinement**, et le produit
`θ·Λ` croît d'environ 30 % : la **taille seule** faisait franchir la tolérance, sur des pas que tout
le reste annonçait reçus.

Trois choses généralisent.

1. **Décomposer avant de mesurer.** Une identité discrète déjà vérifiée (`div u = r/scale`) a réduit
   l'écart à trois nombres sans dimension, dont deux étaient mesurables en une campagne. La campagne
   a alors répondu à une question posée, pas à une curiosité.
2. **Un seuil peut porter, dans son propre tableau de réception, la mention qui en limite la portée.**
   S199 avait écrit « configuration unitaire testée ; **pas borne universelle** » en face de son
   `< 10⁻⁵`, et S237 avait desserré une de ses propres assertions à 10⁻⁴ sans dire pourquoi. Trois
   sessions ont traité ce nombre comme une garantie. **Relire la colonne de portée avant de faire
   d'un seuil une condition.**
3. **Toutes les lignes d'un système ne disent pas la même chose.** Une ligne à fluide fantôme est une
   condition de bord de raideur `1/θ`, pas une conservation : son résidu plafonne à **un ulp de sa
   propre magnitude** — mesuré ici en puissances de deux exactes — pendant que les lignes franches
   descendent de deux ordres de grandeur. Appliquer une tolérance physique à la maximale sur toutes
   les lignes revient à mesurer le conditionnement du bord, pas la qualité de la projection.

Voir TOLERANCE-PRESSION-S239 §1.2 et §3, ADR-144. Famille de L316 et L317 : le nombre vient de ce
qu'on doit distinguer, et sa portée doit être écrite avec lui.

## L319 — Un invariant jamais mesuré sur une couche n'y est ni tenu ni violé : il est inconnu

*(S240)* I-06 — « aucune allocation à l'exécution » — était reçue pour le cœur par un allocateur
compteur, et **n'avait jamais été mesurée pour l'hôte graphique**. Quinze sessions de rendu l'ont
citée comme un reliquat sans jamais la compter. Un allocateur compteur de quatre-vingts lignes, sans
dépendance nouvelle, a rendu le verdict en une exécution : **134 allocations et 30 541 octets par
image**, dont **une seule à nous** — 12 032 octets, 39 % des octets de l'image, un `Vec` de contour
construit et jeté à chaque image.

Deux choses généralisent.

1. **Ce qui n'est pas compté n'est pas tenu.** Tant qu'un invariant n'a pas d'instrument sur une
   couche, l'écrire dans la liste n'y change rien. L'instrument coûte moins cher que le débat : ici,
   une session, et il reste dans le binaire. Le corollaire vaut aussi dans l'autre sens — la mesure
   a montré que **rien d'autre** que ce `Vec` n'était à nous, et que la boucle d'événements de winit
   n'alloue rien du tout : l'inquiétude diffuse était mal placée.
2. **La constance d'une grandeur informe souvent mieux que sa valeur.** Le protocole nommait
   l'allocation comme suspect direct de la gigue CPU — quatre fois la médiane au maximum. Médiane,
   p95 et maximum du compte d'allocations se sont révélés **égaux**, à l'octet près, sur 590 images
   et trois exécutions : une image lente alloue exactement comme une image rapide. **Le suspect est
   disculpé par sa constance, pas par sa valeur**, et A265 perd une hypothèse au lieu de la garder
   en réserve. Une mesure qui ne trouve rien à corriger reste un résultat, à condition qu'elle ait
   été capable de trouver.

Voir ALLOCATIONS-HOTE-S240, ADR-145, amendement daté sous I-06. Parent de L318 : la portée d'une
règle s'écrit avec elle, et une couche qu'elle n'a jamais visitée n'est pas une couche conforme.

## L320 — Ordonner les obstacles avant d'en lever un, et lire un comparable sur les questions qu'il ne pose pas

*(S241)* La file portait A275 — « f32 ne tient plus la tolérance à 32 768 mailles » — comme prérequis
du passage à la 3D. Confrontés à nos propres mesures de coût, déjà publiées et jamais rapprochées de
celle-là : **à 2 048 mailles, δ seul vaut 2,8 fois les 2 ms qu'ADR-125 donne à toute l'eau**, et à
8 192 mailles une image en demande ≈ 296 ms. À la taille où A275 mord, le coût est **deux à trois
ordres de grandeur** au-dessus du budget. Un lot qui n'aurait corrigé que la précision n'aurait rien
débloqué.

Deux choses généralisent.

1. **Un obstacle mesuré ne devient un prérequis qu'après comparaison aux autres obstacles du même
   chemin.** Les deux chiffres étaient dans le dépôt depuis des sessions ; aucun ne manquait. Ce qui
   manquait était le rapprochement — et il ne coûte pas une campagne, il coûte de mettre deux
   tableaux côte à côte. **Avant d'écrire « prérequis » dans une file, vérifier ce qui bloque
   *avant*.**
2. **Un comparable externe s'exploite sur ce qu'il ne fait pas.** Un moteur du commerce règle sa
   pression par un nombre d'itérations et un facteur de relaxation, et n'expose **aucun critère de
   convergence** : il ne pose pas la question à laquelle deux de nos sessions viennent de répondre.
   Cela ne dit pas que notre critère est faux — cela dit que **notre classe de fidélité est un choix**,
   payé en coût, et que les itérations fixes sont le levier bon marché que nos propres ADR interdisent
   aujourd'hui. Un comparable qui ne mesure pas son erreur ne peut rien nous apprendre sur la
   précision ; il nous apprend beaucoup sur les compromis que l'on peut acheter.

Et une règle de tenue, corollaire de la 1 : **ce qu'un fournisseur publie est une affirmation, pas une
mesure de ce projet.** Chaque ligne de `docs/COMPARABLES-EXTERNES.md` porte son URL et son statut —
documenté, déduit, non trouvé — et aucun nombre lu ailleurs n'entre ici comme seuil (I-14, REPRISE §2).
Parente de L318 : la portée d'un nombre s'écrit avec lui.

## L321 — Le meilleur témoin du bruit d'un banc est un changement sémantiquement neutre

*(S242)* Deux restructurations de la même boucle ont été écrites : l'une change ce que la machine
fait, l'autre non. La seconde — trier sur la rangée 0 en gardant l'imbrication d'origine — s'est
révélée sans gain, et c'est ce qui l'a rendue précieuse : **sémantiquement neutre, elle a mesuré
+2,6 à +7,7 %** sur la fenêtre de forçage du banc. Le vrai changement y mesurait +0,5 à +2,4 %. Sans
ce témoin, on aurait publié « le changement coûte 2 % en forçage » — une phrase fausse, et du mauvais
côté.

Trois choses généralisent.

1. **Un banc ne dit pas son propre bruit ; il faut le lui faire dire.** Répéter la même exécution
   mesure une partie de la dispersion ; **rejouer une variante qui ne peut rien changer** la mesure
   entièrement, arrondi du compilateur et placement mémoire compris. Une variante écartée n'est donc
   pas du travail perdu : c'est l'étalon.
2. **Ne pas attribuer un écart plus petit que le témoin.** La règle se dit en une ligne et elle
   coûte une exécution.
3. **Un coût qui croît avec l'histoire n'est pas une constante à gagner.** Ici, la préparation payait
   3,7 µs par tronçon **achevé** et par image — invisible à 24 tronçons, 0,74 ms à 200, et sans
   borne. Le supprimer ne fait gagner presque rien aujourd'hui et change la forme du coût pour
   toujours. **Chercher les termes qui grandissent avant les constantes qui pèsent** : les seconds se
   voient sur un banc, les premiers seulement dans une partie longue que personne ne joue en mesurant.

Et une mise en garde de même famille que L318 : le raccourci « `phase(−p)` est le conjugué de
`phase(p)` » est vrai **en valeur** et faux **au bit** — l'angle est reconstruit par quadrant depuis
l'entier, et le zéro signé diffère à l'origine. Une identité mathématique n'est pas une identité
flottante ; la vérifier coûte moins cher que d'expliquer un hash qui bouge.

## L322 — Un critère de réception qui ne peut pas mordre ne sert à rien ; ceux-là ont mordu deux fois

*(S243)* Le protocole du lot exigeait, avant toute écriture de code, que **le chemin à un fil ne soit
pas ralenti** (4) et qu'**aucune allocation ne soit ajoutée au chemin d'image** (5). Les deux ont
attrapé un défaut que la mesure « le parallélisme va-t-il plus vite ? » aurait laissé passer :

1. l'hôte parallèle construisait un tampon de tranches à chaque appel — `update` passait de **0 à 50
   allocations par image**, effaçant ce que S240 venait de recevoir ;
2. lancer des fils là où il n'y avait que 0,30 ms de travail faisait **monter** ce travail à 0,76 ms.
   Le banc principal, lui, allait bien : le gain de ×2,6 sur la fenêtre chargée aurait masqué la
   perte sur la fenêtre légère.

Trois choses généralisent.

1. **Écrire les critères qui peuvent faire échouer, pas ceux qui vont réussir.** Un critère qui ne
   peut que confirmer la thèse ne mesure rien. Ceux-ci portaient sur ce que le changement risquait
   d'abîmer ailleurs — et c'est exactement là qu'il a mordu.
2. **Un paramètre de découpage encode le travail par élément, et seul l'appelant le connaît.** Le
   remède au second défaut n'est pas un seuil réglé après coup mais la conséquence d'un rapport
   mesuré : 76 ns par nœud sans travail, 211 ns de plus par tronçon actif, contre ≈ 67 µs pour créer
   un fil. Découper sans regarder son propre travail est un ralentissement, pas une optimisation.
3. **Une garantie de déterminisme se démontre, elle ne se recopie pas de sa voisine.** Pour une
   somme, le découpage change le résultat — l'addition flottante n'est pas associative — et le
   contrat doit fixer le grain. Pour une écriture disjointe, rien ne s'accumule entre tâches : la
   garantie est **inconditionnelle**, et plus forte. Deux primitives voisines, deux énoncés
   différents ; les confondre aurait fait porter au projet une contrainte inutile, ou pire, une
   promesse fausse.

Voir PARALLELISME-S243, ADR-146. Famille de L321 : ce qu'on ne peut pas voir échouer, on ne l'a pas
mesuré.

## L323 — Mesurer le prix d'un outil avant de construire avec lui, et ne jamais confondre inféré et mesuré

*(S244)* Le lot devait paralléliser les passes de δ avec la primitive construite en S243. Avant
d'écrire le moindre refactor, une mesure de dix minutes a chiffré **ce qu'un appel parallèle coûte à
vide** : ≈ **125 µs par fil**, presque indépendamment du travail. Les passes visées valent **21,7 µs**
(`apply`), 5,5 (axpy), 3,8 (`dir`). Un appel à deux fils coûte **quatorze fois** la pass qu'il
découperait. La route était fermée — et elle l'a été avant, pas après.

Trois choses généralisent.

1. **Le prix d'un outil se mesure séparément de son usage.** Un banc « avec et sans » aurait montré
   un ralentissement sans dire pourquoi, et aurait laissé croire à un réglage à trouver. Mesurer
   l'outil **à vide**, sur une charge triviale, sépare son coût fixe de tout le reste — et rend la
   conclusion transférable : le même chiffre explique *aussi* pourquoi le lot précédent, lui,
   gagnait (3 270 µs de travail contre 950 de fils).
2. **Un chiffre inféré d'une différence n'est pas un chiffre mesuré.** S243 avait écrit « ≈ 67 µs par
   fil », déduit de l'écart entre deux fenêtres d'un banc chargé. La mesure directe donne **125 µs**.
   L'inférence n'était pas absurde, elle était contaminée par tout ce que la différence contenait
   d'autre. **Quand un nombre commence à servir de règle, il faut aller le mesurer pour lui-même** —
   et le corriger par note datée là où il a été publié, sans réécrire ce qui reste vrai.
3. **Annuler ce que la mesure ne soutient pas.** L'ordre de parcours d'`apply` était manifestement à
   contresens de la mémoire ; l'échanger est exact au bit ; il ne gagne **rien**, à aucune des cinq
   tailles. Il a donc été annulé et le résultat publié. Un changement gardé « parce qu'il devrait
   aider » est une dette : la prochaine session le lira comme une optimisation reçue.

Et un corollaire sur ce que vaut une session : **celle-ci n'a gagné aucun facteur**. Elle a rendu la
carte — où va le temps (écritures 67–73 %, réductions 12–13 %, itérations doublant par raffinement),
ce que chaque technique restante achèterait, et laquelle est fermée. Une carte fausse coûte des
sessions ; une carte mesurée en économise. Voir COUT-DELTA-S244, famille de L321 et L322.

## L324 — Mesurer plus large que sa cible : un lot qui rate la sienne peut en atteindre une autre

*(S245)* Le lot visait la **vitesse** : la multigrille devait faire tomber les 425 itérations de la
pression à 32 768 mailles, et avec elles les 286 ms du pas. Elle les a fait tomber à 134 — et le pas
est passé à 729 ms, parce qu'un cycle coûte cinq produits fins par itération. **Cible manquée.**

Le banc relevait aussi, sans que ce fût le sujet, la **divergence** et le drapeau `degraded` de
chaque pas. C'est là que le vrai résultat était : 1,339·10⁻⁵ **refusé** devient 8,512·10⁻⁶ **reçu**.
A275 — « f32 ne tient pas la tolérance à cette taille » — tombe, et sa cause apparaît du même coup :
ce n'était pas la taille, c'était **l'arrondi accumulé sur 425 itérations**. Moins d'itérations, moins
d'arrondi, et le test passe sans qu'on y touche.

Trois choses généralisent.

1. **Relever plus que la grandeur visée.** Le coût d'ajouter deux colonnes à un banc est nul ; le
   coût de ne pas les avoir est une découverte manquée. Ici, la colonne qui a tout décidé —
   `degraded` — n'était même pas dans le protocole : elle y était parce que le banc de la session
   précédente la portait déjà.
2. **Une technique a plusieurs effets, et l'on n'en cherche souvent qu'un.** Un solveur qui converge
   en moins d'itérations est plus rapide *et* plus précis. Le second effet ne se voit que si on le
   mesure, et il peut valoir plus que le premier — ici il ferme une anomalie ouverte depuis six
   sessions.
3. **Une contre-épreuve avant de conclure, même quand la conclusion arrange.** Avant d'écrire « la
   multigrille ne paie pas en vitesse », une variante — hiérarchie plus profonde, cycle allégé — a
   été mesurée : **pire**, avec un compte d'itérations plat mais haut. Ce profil accuse le transfert
   et disculpe le niveau grossier et le lisseur. Sans cette mesure, la suite du lot aurait été
   cherchée au mauvais endroit.

Et un corollaire de tenue : le branchement qui en sort — **repli et non remplacement** — ne coûte
rien là où l'ancien chemin suffit, et son verdict reste celui d'ADR-144, inchangé. Une technique qui
ne paie que dans un régime s'installe dans ce régime, pas partout. Famille de L321 et L323 : ce qu'on
ne mesure pas, on ne peut ni le gagner ni le perdre sciemment.

## L325 — Une provenance ne voyage pas : ce qui est dérivé ailleurs n'est pas dérivé ici

*(S246)* Le lisseur de la multigrille portait un facteur `2/3` accompagné de sa justification :
« il se dérive — il minimise le facteur de lissage du stencil à cinq points ». La phrase avait la
forme d'une provenance, elle citait le bon stencil, et elle était **fausse** : `2/3` est l'optimum à
**une** dimension. En deux dimensions, le minimax des modes de haute fréquence donne `4/5`, pour un
facteur de lissage de `3/5` au lieu de `2/3`.

Elle avait été écrite trois sessions plus tôt, dans le même dépôt, par le même dispositif qui exige
qu'aucune valeur n'entre sans provenance (I-14). Elle a passé une relecture, un ADR et une réception.
Ce qui l'a trouvée n'est pas une relecture de plus : c'est d'avoir **cherché ailleurs**, éliminé
quatre suspects par la mesure, et fini par revenir sur le seul nombre que personne ne soupçonnait.

Trois choses généralisent.

1. **Une provenance ne voyage pas entre dimensions, géométries ou régimes.** Un résultat classique
   reste vrai dans le cadre où il a été établi. Quand on le transporte, la dérivation doit être
   **refaite dans le nouveau cadre**, pas citée. Le symptôme est toujours le même : une justification
   correcte dans sa phrase et inapplicable à son usage.
2. **Une provenance fausse est plus dangereuse qu'un nombre nu.** Un nombre sans justification
   attire la vérification ; un nombre avec une justification plausible l'écarte. `2/3` a survécu
   précisément parce qu'il *paraissait* dérivé.
3. **Éliminer des suspects est un résultat, et c'est ce qui finit par désigner le vrai.** Ce lot a
   écarté par la mesure le niveau grossier, le lissage, l'ordre de la prolongation et la géométrie
   des mailles coupées. Aucun de ces quatre n'a rien rapporté — et c'est en n'ayant plus où chercher
   qu'on relit ce qui semblait acquis.

Corollaire de tenue, de la famille de L323 : la prolongation bilinéaire construite pour ce lot **ne
gagnait rien** et a été **annulée**. Un opérateur plus coûteux gardé « parce qu'il est meilleur en
théorie » est une dette, que la session suivante lirait comme une amélioration reçue.

## L326 — Étalonner un instrument sur un chiffre déjà publié, avant de s'en servir

*(S247)* Le protocole du lot exigeait, **avant toute mesure nouvelle**, que l'instrument
d'échantillonnage soit passé à la pose de référence, où il devait retrouver les **7,5 %** publiés par
S234 deux sessions plus tôt. Il a rendu **7,54 %**. L'ordre — étalonner, puis employer — n'a rien
coûté, et il a rendu tout ce qui suit défendable : un instrument neuf qui annonce un chiffre neuf
n'est pas une mesure, c'est une affirmation.

Trois choses généralisent.

1. **Un dépôt qui publie ses chiffres se fournit ses propres étalons.** La valeur de S234 n'avait pas
   été conservée pour cela ; elle a servi de témoin parce qu'elle était écrite avec son domaine. Un
   chiffre publié sans son domaine n'aurait rien étalonné du tout.
2. **Une part peut ne pas bouger pendant que sa queue triple.** À incidence rasante, la fraction de
   sommets sous Nyquist est celle de la pose de référence — 7,59 % contre 7,54 % — mais le **pire
   écart passe de 2,589 à 8,243 m**. Un banc qui n'aurait relevé que la part aurait conclu « rien ne
   change ». **Relever une part et un extrême coûte la même boucle** ; n'en relever qu'un est un
   choix, et il faut le faire exprès.
3. **Un chiffre bien posé élimine aussi le mauvais remède.** 8,2 m au loin, ce sont exactement les
   deux pixels de la grille projetée : à cette distance, une onde de 2 m est **plus petite qu'un
   pixel**. Densifier le maillage — le réflexe — dépenserait des sommets pour dessiner ce que l'écran
   ne peut pas montrer. La mesure ne dit donc pas seulement qu'il y a un défaut : elle dit **par
   quelle voie il ne se corrige pas**, ce qui vaut souvent davantage.

Famille de L323 et L325 : ce qui n'a pas d'étalon n'a pas de valeur mesurée, et ce qu'on ne relève
pas ne peut pas surprendre.

## L327 — Une image de banc est un instrument : on l'étalonne, et son désaccord est une mesure

*(S248)* Les cartes du maillage devaient retrouver, avant qu'on les regarde, les extrema que S247
avait publiés. Le premier jet annonçait **567,9 m** et **1 228,8 m** là où S247 publie **2,589** et
**8,243**. Il aurait été facile de conclure « la carte est fausse » — elle ne l'était pas. Elle
mesurait **autre chose** : S247 comptait dans l'emprise du sillage, la carte comptait jusqu'à
l'horizon. Une fois les deux quantités séparées, l'étalon passe exactement, **et le désaccord est
devenu un résultat** : sur toute l'eau visible, l'écart entre sommets monte à plus d'un kilomètre,
ce que personne n'avait relevé.

Trois choses généralisent.

1. **Une image produite par un programme est un instrument, pas une illustration.** Elle se soumet
   aux mêmes exigences : une empreinte reproductible, un domaine écrit, et un étalon passé **avant**
   usage. Sans cela elle ne prouve rien, et elle est d'autant plus dangereuse qu'elle convainc.
2. **Quand un instrument neuf contredit une mesure ancienne, la première hypothèse n'est pas qu'il
   se trompe : c'est qu'il ne mesure pas la même chose.** Chercher la différence de définition avant
   de chercher le bogue est plus rapide, et c'est souvent là que se trouve le fait nouveau.
3. **Une carte montre la forme qu'un nombre cache.** « 7,59 % des sommets sous Nyquist » et « une
   bande étroite à l'horizon, le reste sain » sont le même nombre et deux conclusions différentes —
   la seconde dit *où couper*, la première ne le dit pas. De même, séparer les couches d'ADR-001
   dans deux images a rendu lisible en un coup d'œil ce que l'architecture affirme depuis S01 : `B`
   partout et lisse, `W` et `δ` locaux et structurés.

Famille de L326 : ce qui n'a pas d'étalon n'a pas de valeur mesurée — et l'étalon vaut aussi pour ce
qu'on regarde.

## L328 — Un composant numérique s'éprouve dans le solveur qui l'emploie, contre le vrai résidu

*(S252)* Le cycle multigrille de S245 avait ses essais : symétrie au bit, opérateur grossier contre
opérateur fin, réduction par cycle. Tous passaient, et à juste titre. Le défaut était ailleurs, dans
les trois lignes du gradient conjugué qui l'emploie : β y était formé avec la mauvaise quantité.
Sept sessions ont mesuré ce solveur. Elles en ont tiré une conclusion de coût (« la multigrille ne
gagne pas de vitesse ») et une explication de précision (A275 tenue par « moins d'itérations »),
puis ont construit sur les deux : ADR-147, S246, A281.

Trois choses généralisent.

1. **L'essai d'un composant ne reçoit pas son assemblage.** L'essai qui manquait tenait en une
   ligne : à l'arrêt de la récurrence, comparer le résidu qu'elle croit avoir au vrai résidu
   recalculé. Toute méthode à récurrence (gradient conjugué, somme compensée, accumulation) offre
   ce contrôle gratuit.
2. **Une porte qui recalcule la vérité protège le résultat et cache le défaut.** Aucun pas faux n'a
   été publié, et c'est pour cela que rien n'a alerté : le défaut ne se voyait que dans le coût.
   Un coût anormal est un symptôme à attribuer, pas une propriété du régime. S251 l'avait d'abord
   rattaché au plancher.
3. **Un succès accidentel trouve toujours son explication après coup.** Les relances du gradient
   fautif faisaient un raffinement itératif involontaire. Corriger le défaut a fait perdre la
   tolérance à 32 768 mailles, et prouvé que l'explication publiée était fausse. Avant de
   revendiquer une cause, se demander ce qui changerait si elle l'était.

Famille de L322 (un critère qui mord) et de L326 (étalonner avant d'employer).

## L329 — Un témoin s'éprouve à l'ordre qu'il prétend garder

*(S253)* Le témoin « sans résidus de surface » devait éteindre les termes d'ordre deux. Il
éteignait aussi `ρg·ζ_fond` sur les fantômes latéraux, un terme d'**ordre un**. Il a divergé, et
une divergence donne un écart infini, supérieur à la tolérance de 20 % : le critère « passait »
pour une raison fausse. Corrigé en conservant la pression linéarisée, il rend 99,56 % sans
diverger.

1. **Un témoin retire ce qu'il nomme, et rien d'autre.** Avant de mesurer, vérifier qu'il reste
   exact à l'ordre inférieur, ici la dynamique linéaire. Sinon, il mesure autre chose que ce qu'il
   annonce.
2. **Un témoin qui échoue trop fort est aussi suspect qu'un candidat qui réussit trop bien.** Une
   divergence n'est pas un écart mesuré : c'est le signe que l'instrument lui-même est faux.

Famille de L322 (un critère qui mord pour la bonne raison) et de L328.

## L330 — Un oracle tronqué borne ce qu'il peut recevoir

*(S273)* Le résidu progressif S272 était refusé à 8,68 % contre un oracle d'ordre deux. Corriger
la quadrature fautive l'a ramené à 5,88 %, et le raffinement ne le faisait presque plus baisser.
L'écart restant venait de la référence : la solution contient un ordre trois que l'oracle n'a pas,
environ 5,6 % de `η'` à cette amplitude. Aucun solveur ne pouvait passer 2 % contre lui.

1. **Avant d'attribuer un écart au solveur, mesurer celui de la référence.** Un oracle tronqué en
   un paramètre physique se qualifie en faisant varier ce paramètre. Si l'écart normalisé ne
   dépend pas de la maille, il n'est pas numérique.
2. **L'extrapolation dans le paramètre sépare les deux parts.** Ici `2N(a/2) − N(a)` retire le
   terme d'ordre trois. La part qui reste converge avec la maille, et c'est elle que le solveur
   doit à la référence.
3. **Ce contrôle s'écrit avec le seuil, pas après le refus.** S272 l'avait déclaré comme contrôle
   (« a² ≤ 1 % ») sans en tirer la conséquence : un contrôle qui échoue invalide la métrique
   avant de juger le candidat.

Famille de L329 (un témoin s'éprouve à l'ordre qu'il prétend garder) et de L322.

## L331 — Une précision se juge dans l'unité de l'usage, et une extrapolation dans son régime

*(S274)* Trois sessions poursuivaient 2 % sur la correction non linéaire d'une houle. Traduits,
les 5,88 % restants valaient 5,8 µm sur une vague de 1 cm. Et les 2 % valaient à peu près 3 mm
d'image pour la mer la plus cambrée : ni absurdes, ni requis pour la mer de référence.

1. **Avant de raffiner, traduire le seuil en grandeur d'usage au régime le plus exigeant servi**
   (hauteur, pente, phase contre les tolérances d'image et d'horloge). Un seuil relatif sur une
   petite correction ne dit rien de ce qui se voit, dans un sens comme dans l'autre.
2. **Une extrapolation dans un paramètre se vérifie par un point de plus, et ce point doit rester
   dans le même régime discret.** Ici, la troisième amplitude franchissait le centre des mailles de
   surface : l'accord manqué venait du changement de topologie, pas de la physique.
3. **Le point d'arrêt se déclare avec l'usage** : une réception différée garde son déclencheur
   dans la file, elle n'est pas abandonnée.

Famille de L330 (un oracle tronqué borne ce qu'il peut recevoir).

## L332 — Une optimisation qui garde les bits se prouve par l'égalité

*(S276)* L'échantillonnage du fond coûtait 33 ms par pas, et chaque face recalculait des phases et
des exponentielles qui ne dépendaient que de sa colonne ou de sa couche. Passer la boucle des
composantes à l'extérieur, sans changer la suite d'opérations reçue par chaque face, a donné
×6,4 **au bit**.

1. **Réordonner les boucles sans changer la suite d'opérations de chaque sortie garde les bits.**
   On peut alors hisser tout ce qui ne dépend pas de l'indice intérieur, et l'essai est une égalité
   exacte, pas une tolérance.
2. **Une égalité au bit dispense de toute re-réception** : le rejeu S275 et le direct S276 sont
   restés identiques, et aucune mesure physique n'a été refaite. Une optimisation à tolérance, elle,
   oblige à rejouer chaque réception qui en dépend (ADR-167, ADR-169).
3. **Chercher d'abord ce qui garde les bits**, puis seulement ce qui les change.

Famille de L195 (coût complet) et de L328.

## L333 — Une demande de verdict doit dire ce que la scène ne contient pas

*(S277)* R10 a montré à l'utilisateur la houle la plus pauvre du dépôt — étalement nul, aucune
vague sous 40 m, surface rigoureusement invariante le long des crêtes — en lui demandant si δ était
convaincant. Il a répondu, exactement et légitimement, que le motif était trop répétitif et que les
vagues devaient interagir avec l'onde. **Il a jugé la scène, pas la couche**, et il a eu raison :
rien dans la demande ne disait que la scène était appauvrie exprès, ni pourquoi.

1. **Une scène de mesure n'est pas une scène de démonstration.** L'étalement nul était la condition
   pour que le fond soit exactement plan dans une tranche 2D. C'est un choix juste pour mesurer, et
   trompeur pour montrer. Les deux usages demandent deux scènes, ou une mise en garde explicite.
2. **Ce qu'on retire d'une scène se déclare dans la demande de verdict**, avec sa raison, et pas
   dans la réponse à l'objection. Sinon le verdict porte sur l'absence, et il est perdu pour la
   question posée — ici, « le pas de 16 ms suffit-il ? » n'a pas reçu de réponse.
3. **Un retour qui semble à côté de la question désigne souvent une limite structurelle.** Les deux
   remarques de l'utilisateur pointaient la même chose : δ est une tranche 2D sous une houle
   idéalisée, et les vaguelettes ne sont qu'un habillage hors du domaine simulé. Mesurer avant de
   répondre (`bandes_s277.rs`) a transformé une impression en quatre nombres.

## L334 — Un banc qui ne peut pas échouer ne prouve rien

*(S278)* Le banc de l'ordonnanceur devait montrer que le budget n'est jamais dépassé. Sa première
version faisait longer au bateau trois îlots espacés de 150 m : il ne rencontrait qu'un domaine à
la fois, le budget maximal distribué valait 0,800 ms pour 2,0 déclarés, et les quatre assertions
passaient. **Le sac à dos n'avait jamais été sollicité.** Le banc ne mesurait pas la propriété
annoncée — il mesurait qu'elle n'avait pas eu l'occasion d'être fausse.

1. **Une assertion qui passe ne dit pas que la propriété tient** : elle dit que le cas exécuté ne
   l'a pas contredite. Tant que la contrainte n'a pas mordu, une borne jamais approchée et une
   borne jamais dépassée rendent la même mesure.
2. **Le remède est une assertion sur la difficulté du cas, pas sur son résultat.** Ici, une ligne
   qui refuse de passer si la demande simultanée tenait dans le budget. Elle a immédiatement
   échoué, et c'est ce qui a rendu le banc utile.
3. Cela vaut pour tout banc dont le scénario est **choisi par l'auteur de la propriété** : la
   tentation n'est pas de tricher, elle est de prendre un cas propre. Écrire ce qui rend le cas
   difficile force à vérifier que la difficulté est là.

## L335 — Deux documents qui ne se raccordent pas ne se voient qu'au moment où quelqu'un calcule

*(S278)* ADR-012 §2 définit la priorité comme un produit de trois poids dont aucun n'est borné, et
dont l'un — `W_urgence = 1/temps` — diverge. ADR-013 §5 définit le score d'activation comme
« `s ∈ [0,1]`, composé selon ADR-012 §2, avec hystérésis 0,60 / 0,40 ». Les deux sont cohérents
séparément, lisibles, et **incompatibles**. Ils ont vécu ainsi de S01 à S278, à travers six audits
et deux revues croisées.

1. **Une incohérence entre deux documents ne se lit pas, elle se calcule.** Personne ne compare
   les domaines de définition en relisant ; le compilateur, lui, demande quelle valeur mettre.
2. **C'est le second argument d'écrire le code** (S21 : le code est un instrument de mesure de la
   conception). Le premier est de trouver les erreurs de physique ; le second, plus discret, est de
   trouver les endroits où deux textes justes ne se raccordent pas.
3. La réparation appartient à un **nouvel** ADR (ADR-170), jamais à une retouche des deux anciens :
   ce qui a été décidé reste lisible, et ce qui manquait devient une décision datée.

## L336 — Exclure sur une mesure que l'exclusion empêche de produire est absorbant

*(S279)* L'ordonnanceur estime le coût d'un domaine par celui de son dernier pas — ADR-012 §3 le
demande, et pour une bonne raison : « un ordonnanceur qui planifie sur des coûts théoriques dérive
dès la première optimisation ». Mais un domaine que le budget écarte **n'exécute plus de pas**. Il
ne produit donc plus de mesure, conserve le coût qui l'a fait écarter, et reste écarté. La bande δ
s'est éteinte au bout de 0,352 s et n'est jamais revenue, alors que rien à l'écran n'avait changé.

1. **Une boucle de rétroaction dont l'entrée est produite par l'action qu'elle commande a un état
   absorbant.** Le motif ne tient ni au budget ni au coût : il apparaît partout où l'on décide de
   *ne pas faire* sur la foi d'une mesure que seul *faire* produit — repli d'un cache, dégradation
   d'un service, seuil de réessai.
2. **Un pic suffit.** Il n'est pas besoin que le coût moyen dépasse le budget : le pire pas mesuré
   valait 45,8 ms pour une médiane de 22. La décision se prend sur la dernière valeur, et la
   dernière valeur d'un domaine écarté est celle de son pire pas.
3. **Élargir le budget referme le cas, pas le défaut.** C'est ce qui a été fait ici, et il faut le
   dire ainsi : le mécanisme reste absorbant, et il reviendra au premier budget serré. Sortir
   demande soit une estimation qui décroît tant qu'on ne tourne pas — un droit de retenter — soit
   une dégradation qui **rétrécit** au lieu d'exclure (ADR-012 §4 rang 1).

**Levée en S280** ([mesure](../docs/validation/COUT-ROBUSTE-S280.md)) : l'estimation est devenue la
**médiane des huit derniers pas payés**, et le compte d'échantillons décroît d'une unité par pas non
payé — vidé, le domaine retombe sur son nominal et retente. Au budget qui le condamnait, la bande
paie désormais tous ses pas ; à un budget réellement insuffisant, elle est **affamée et non
absorbée** — vivante, estimation honnête, servable dès que le budget le permettrait. La quatrième
leçon est celle-là : **l'antidote d'une boucle absorbante est l'oubli**. Une mesure qu'on ne peut
plus rafraîchir doit se périmer, faute de quoi elle devient un jugement définitif rendu sur une
seule observation.

## L337 — Déplacer un calcul sur un accélérateur, jamais le critère qui le reçoit

*(S289)* Pour faire calculer la pression de δ sur GPU, la pente naturelle était d'y mettre le
solveur **entier**, critère d'arrêt compris. C'était impossible sans perdre l'essentiel : les
trois portes du cœur — tolérance physique sur le champ corrigé, plancher d'arrondi composante
par composante, empreinte au bit pour détecter un cycle — demandent chacune un rapatriement
**par test**, soit exactement ce qu'un solveur résident existe pour supprimer. Les porter sur la
carte aurait coûté ce que le déplacement faisait gagner, **et** aurait mis la réception physique
du système sous la dépendance d'un pilote graphique.

Le bon découpage se lit dans les ordres de grandeur, et il est presque toujours le même :
l'approximation coûte `O(n)` **par itération**, la vérification coûte `O(n)` **une fois**. Le
calcul part ; le critère reste. L'accélérateur propose, l'autorité dispose.

La propriété qui compte n'est cependant pas la vitesse. C'est que **le résultat faux devient
inutile au lieu d'être dangereux** : une mauvaise proposition ne peut coûter que des itérations.
C'est ce qui a permis d'activer un chemin GPU dans le pas réel sans avoir reçu physiquement le
GPU, sans exiger d'identité entre cartes, et en gardant le refus atomique — départ restauré au
bit sur une valeur non finie.

**Réflexe** : devant un calcul à déporter — accélérateur, cache, approximation, modèle appris,
service externe —, séparer d'abord *ce qui cherche* de *ce qui constate*, et compter le coût de
constater. S'il est d'un ordre inférieur, il ne part pas. Même famille que L328 : un composant
numérique s'éprouve dans le solveur qui l'emploie, contre le vrai résidu — ici c'est le solveur
qui **reste** l'épreuve, à demeure et en production.

## L338 — Le poste dominant d'un appel accéléré peut n'être aucun des deux qu'on soupçonne

*(S289)* Les deux suspects d'un appel GPU sont le calcul et les transferts. Mesuré, l'appel du
cycle résident coûte 4,1989 ms à 6 656 mailles : 0,2606 d'empaquetage, 1,150 de calcul réel sur
la carte — et **3,9333 d'encodage, soumission et attente**. Ni l'un ni l'autre des suspects :
27 % de temps utile, et le reste dans la *description* du travail, parce que le cycle émettait
sept commandes par itération, 896 en tout. La même cause produisait les allocations qui
interdisent le chemin d'image — sept par itération, dans l'encodage de la pile graphique, pas
dans notre code.

Optimiser le noyau aurait été le geste évident et n'aurait presque rien rendu : le plafond est
27 %. Le lot utile est de dire le travail en moins de commandes.

**Réflexe** : décomposer le temps d'un appel déporté en *préparer / décrire / exécuter / attendre*
**avant** de toucher au noyau, et publier les quatre. Un banc qui ne rend qu'un total ne peut pas
désigner le lot suivant — et il laisse optimiser la part qu'on voit au lieu de celle qui coûte.

## L339 — Un instrument doit être dans les deux bras, et sa fenêtre ne doit contenir que ce qu'on mesure

*(S290)* Deux mesures fautives le même jour, de la même famille, et toutes deux invisibles à la
relecture du code qui les produisait.

1. **L'instrument n'était que dans un bras.** La comparaison de six variantes d'encodage laissait
   celles à un seul tampon de commandes payer un aller-retour de lecture d'horodatage, que les
   variantes par tranches ne payaient pas — l'horodatage étant tu dès qu'il y a plusieurs
   soumissions. Le gain des tranches était surestimé d'environ 0,5 ms. Le signe qui a trahi le
   défaut : **deux configurations identiques par construction affichaient 2,2454 et 1,6761 ms.**
   Une paire qui devait être égale ne l'était pas ; c'est le seul test qui pouvait le voir.
2. **La fenêtre contenait le code d'autrui.** La mesure d'I-06 sur notre empaquetage englobait les
   `write_buffer` de la bibliothèque graphique et comptait 19 allocations — attribuées à notre
   code, alors qu'ADR-145 §1 ne lit cet invariant que là, et §2 permet explicitement les autres.
   Resserrée sur le seul remplissage de nos réserves, elle rend zéro.

Les deux erreurs vont dans le sens qui flatte le travail en cours, et c'est la raison de la
leçon : une mesure fautive se remarque quand elle déçoit, jamais quand elle confirme.

**Réflexes.** *(a)* Mettre dans le protocole une paire qui **doit** rendre le même nombre, et la
publier ; un écart y est un défaut d'instrument, pas un résultat. *(b)* Avant de publier un compte
attaché à un invariant, écrire à qui appartient chaque ligne dans la fenêtre — un invariant qui
porte sur « notre code » ne se mesure pas sur une fenêtre qui appelle une dépendance. *(c)* Dix
passages ne suffisent pas à une médiane sur une machine à quelques dixièmes de ms de variance ;
trente, et aucune différence de moins de 5 % lue comme un effet. Même famille que L338 : un total
ne désigne rien, et un total mal borné désigne faux.

## L340 — Refaire un petit calcul coûte souvent moins que de le synchroniser

*(S290)* Le cycle de pression avait, par itération, deux dispatchs entiers dont le seul travail
était de sommer quelques centaines de valeurs pour produire un scalaire. Mesuré, un dispatch coûte
**1,86 µs d'enregistrement CPU quel que soit son contenu** : ces deux-là payaient le prix plein
pour un travail négligeable. Les deux échappatoires envisagées étaient des synchronisations —
compteur atomique désignant le dernier groupe, ou reformulation supprimant la dépendance — et
toutes deux coûtaient de la portabilité ou de la précision.

La sortie était de **ne pas synchroniser du tout** : chaque groupe refait la somme pour lui-même,
au début du noyau suivant, sur des valeurs écrites par le dispatch précédent — donc visibles par
la seule frontière de dispatch, sans rien demander de plus. Redondance pure : la somme est faite
autant de fois qu'il y a de groupes. Résultat identique **au bit**, un tiers de dispatchs en
moins, et la carte plus rapide aussi tant que la redondance reste petite.

Et elle ne reste pas petite indéfiniment : le coût redondant est en `groups²`, mesuré, avec un
croisement déjà visible entre 104 et 512 groupes. C'est la seconde moitié de la leçon.

**Réflexe** : quand une valeur partagée coûte une synchronisation, calculer ce que coûterait de la
**refaire** partout où elle sert. Le rapport à comparer n'est pas « une fois contre N fois », c'est
« N fois le petit calcul » contre « le prix de la coordination », et ce prix inclut ce qu'on cesse
de pouvoir garantir. Publier aussi l'exposant de la redondance, parce qu'il fixe le domaine où la
réponse reste vraie.

## L341 — Un travail préparé « au cas où » se paie à chaque tour, et la relecture ne le voit pas

*(S291)* Le solveur de pression appliquait son préconditionneur — un cycle multigrille complet —
**avant** d'entrer dans la boucle qui l'emploie. Le code était juste, et il l'avait toujours été :
tant que la boucle tournait, la préparation servait. Puis S289 a mis un candidat GPU devant, la
boucle a cessé de tourner à presque tous les pas, et la préparation est devenue du travail jeté :
**0,83 ms sur 5,79, le plus gros poste hors GPU.** Trois sessions avaient lu ce code de près sans
le voir, parce qu'il n'y a rien à voir — la faute n'est pas dans la ligne, elle est dans l'ordre.

Deux choses en sortent, et la seconde est la vraie.

1. **Ce qu'une boucle conditionnelle emploie se calcule dans la boucle.** Différer l'amorçage ne
   change rien numériquement quand une itération tourne, et supprime tout quand aucune ne tourne.
2. **Une optimisation en amont périme les préparations en aval, silencieusement.** Rendre une
   étape inutile ne la supprime pas : elle continue de coûter, et son coût devient d'autant plus
   visible que le reste a maigri. Aucun test ne se plaint — le résultat est identique.

**Réflexe** : après avoir rendu une étape beaucoup plus rapide ou beaucoup plus rare, **remesurer
par poste** ce qui l'entourait, et chercher ce qui se prépare encore pour elle. Ne pas relire :
mesurer. Une relecture voit ce qui est faux, pas ce qui est devenu inutile. Même famille que
L338 — un total ne désigne rien.

## L342 — Un chemin rapide qui refroidit n'est pas un chemin rapide

*(S291)* Le pas de δ passait de 17,93 à 8,27 ms de médiane en déportant sa pression sur le GPU.
Puis un pas a coûté **467 ms**, et 132 à la reproduction — toujours au premier appel venant après
quelques secondes sans GPU. Un envoi de préchauffage au démarrage **ne le supprime pas** : ce qui
le supprime est de ne pas laisser la carte inactive entre deux appels. Ce n'est pas un chemin
neuf qui se prépare, c'est un chemin **refroidi** qui se rallume.

Pour ce qui est rendu à l'écran, **le nombre qui décide est le pire pas, pas la médiane**. Un gel
de 130 ms annule le bénéfice de milliers de pas à 8 ms, et il suffit que la bande ait été éteinte
un instant — ce qu'un ordonnanceur fait exprès, régulièrement, pour économiser du budget.

Le même écart entre médiane et pic s'est retrouvé dans un réglage : la longueur de cycle qui
minimise la médiane (3,80 ms, pic 25,79) n'est pas celle qui minimise le pic (4,81 ms, pic 8,10).
Optimiser la moyenne et optimiser la latence sont deux objectifs, et ils divergent.

**Réflexes.** *(a)* Publier le maximum à côté de la médiane, toujours, et le dire quand les deux
ne désignent pas le même réglage. *(b)* Mesurer un chemin accéléré **après une pause**, pas
seulement en rafale : une rafale est le régime le plus favorable et le moins représentatif.
*(c) *Ce qui garde une ressource tiède est un arbitrage — qui paie, quand, contre quel budget —
et pas un réglage qu'on pose en passant.

*Note datée S293, 2026-09-19.* Le mécanisme « refroidi » n'est **pas établi** : S292 a mesuré
l'appel isolé après des pauses de 0 à 8 s, endormies ou à processeur saturé — ×1,4 à ×2, pas
×200. Le pic de S291 reste à localiser dans le pas complet (A294). La leçon sur le **pire pas**
tient ; celle sur la cause, non.

## L343 — Un ordre sans dépendance nommée devient un verrou

*(S293)* « A276/A281 restent préalables mesurés de la 3D », écrit en S249, est devenu en S276 le
déclencheur « avant la 3D ». Aucune dépendance ne le fondait : la porte C se reçoit **sur la scène
de la porte B**, et le livrable de J2 — cavité, gerbe, proche-coque — est tridimensionnel. La
dépendance allait dans l'autre sens. Pendant seize sessions, le travail a glissé vers ce qui
restait déverrouillé : le coût d'une tranche 2D dont les frais fixes ne représentent pas le
régime à tenir.

Un déclencheur d'ordre (« X avant Y ») doit **nommer la dépendance qu'il protège** et le critère
qui le lève. Sans elle, il ne protège rien ; il décide seulement où ira le travail suivant — et
ADR-127 §6 exige justement une dépendance, pas une préférence, pour ordonner les jalons.

## L344 — L'oracle d'un schéma porte aussi sa structure en temps

*(S295)* Pour séparer une faute du solveur de l'erreur de discrétisation, la fréquence exacte du
schéma a été calculée hors du code : Laplacien discret, structure verticale discrète, Euler
symplectique. L'écart restait à **0,29 %** — trop pour de l'arrondi, trop peu pour une faute
franche. La cause était dans l'oracle : Euler symplectique décale la hauteur et le flux d'un
demi-pas, et un départ au repos cinématique donne `ηⁿ = A·(cos(nΩdt) + β·sin(nΩdt))`,
`β = −ω_s²·dt²/(2·sin(Ω·dt))`, pas `A·cos(nΩdt)`. Avec ce terme : 0,005 %.

Un oracle « du schéma » n'est exact que s'il reproduit **toute** la récurrence, conditions
initiales discrètes comprises. Faute de quoi il mesure un écart qui n'appartient ni au code ni
au modèle — et l'on corrigerait le solveur pour la faute de l'instrument.

