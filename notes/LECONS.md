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