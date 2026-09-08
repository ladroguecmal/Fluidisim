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
