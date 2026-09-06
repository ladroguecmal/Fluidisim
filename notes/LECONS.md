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
