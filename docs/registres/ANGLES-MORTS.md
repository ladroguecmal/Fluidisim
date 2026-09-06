# Registre des angles morts

Points **absents des documents sources** et susceptibles de coûter cher s'ils sont découverts
tard. Un angle mort n'est pas une question ouverte : une question ouverte est connue et suivie ;
un angle mort ne l'était pas.

Sévérité : **1** = refonte d'architecture si découvert tard, **2** = refonte d'un sous-système,
**3** = travail supplémentaire localisé.

Historique : 28 recensés en S01, 12 ajoutés en S02 (phénomènes secondaires), 9 en S03 (harnais
de validation), 6 en S04 (signatures), 3 en S05 (revue croisée), 6 en S06 (outillage auteur),
6 en S08 (revue croisée des SPEC), 4 en S09 (écriture du chemin poussé), 3 en S10 (persistance),
3 en S11 (audit des points ouverts), 3 en S12 (mécanismes de détail), 3 en S13 (revue croisée des
documents récents), 3 en S14 (audit inverse des invariants), 2 en S15 (audit des registres) — **91 au total**.
**Trente et un ont été trouvés dans nos propres écrits**, pas dans les documents sources : A49, A56,
A57, A58, puis A65 à A91. La proportion
augmente, et c'est attendu — plus le corpus grandit, plus ce qu'il se contredit à lui-même dépasse
ce que les sources avaient omis.

---

## Tableau général

> **Ce que dit la colonne « Traité dans » — précision S15.** Elle indique **où le point est
> discuté**, et non s'il est refermé. Trois situations s'y confondaient ; elles se distinguent
> désormais ainsi :
>
> - **supprimé** — l'angle mort n'existe plus, la décision l'a fait disparaître. Marqué en toutes
>   lettres dans la cellule *(seul cas à ce jour : A16)*.
> - **comblé** — une décision le traite complètement, rien n'est en attente.
> - **en attente d'un tiers** — le point est **décrit, pas réglé** : il attend une équipe, une
>   mesure ou un arbitrage. Marqué **`⏳`** dans la cellule.
>
> La distinction n'était pas faite, et elle coûte : **A59** — le géoïde absent de l'outil de terrain,
> 70 m d'écart à 30 km, sévérité 1 — affichait « SPEC-005 §4 » exactement comme A63, qui est réglé.
> Un lecteur en concluait que le problème des 70 mètres était traité. Il est décrit, et le restera
> tant que l'équipe terrain n'aura pas changé le référentiel de son outil.
>
> Les sévérité 1 en attente sont marquées ci-dessous. Les autres lignes se renseigneront au fil des
> sessions qui les touchent : remplir 89 lignes d'un coup ne serait ni utile ni fiable.

| Code | Titre | Sév. | Traité dans |
|---|---|---|---|
| A01 | Référentiels non inertiels | 1 | ADR-002 |
| A02 | Planète sphérique, horizon, géoïde | 1 | ADR-002 |
| A03 | Précision `f32` et origine flottante | 1 | ADR-002 |
| A04 | Le sillage lointain n'est pas simulable | 1 | ADR-001, ADR-011 |
| A05 | Sillage en eau peu profonde | 3 | ADR-011 |
| A06 | Dispersion incohérente entre couches | 1 | ADR-001 |
| A07 | Latence de lecture GPU | 1 | ADR-007, ADR-008 |
| A08 | Non-déterminisme flottant | 1 | ADR-003 |
| A09 | Coût réel du raffinement (`dx⁻⁴`) | 2 | SPEC-001 |
| A10 | Vitesse de groupe = c/2 | 2 | SPEC-001, ADR-005 |
| A11 | Interpolation de champs stochastiques | 2 | ADR-004 |
| A12 | Audio | 2 | **ADR-016** *(S02)* |
| A13 | Mouillage, ruissellement, séchage | 3 | ADR-010 |
| A14 | Le serveur n'a pas de caméra | 2 | ADR-009 |
| A15 | Persistance et propriété des volumes | 2 | ADR-010 |
| A16 | Triche par injection d'énergie | 1 | **ADR-021** — supprimé, plus atténué |
| A17 | Liquides ≠ eau | 2 | ADR-004, ADR-010 |
| A18 | Eau et vide spatial | 3 | **ADR-015 §5** *(S02)* |
| A19 | Glace et états thermiques | 2 | **ADR-017** *(S02)* |
| A20 | Navigation IA et eau | 2 | **ADR-018** *(S02)* |
| A21 | Battement d'allocation | 2 | ADR-006 |
| A22 | Outillage auteur des rivières | 2 | ADR-011 |
| A23 | Coriolis en station rotative | 3 | ADR-002 |
| A24 | Marée et trait de côte mobile | 2 | ADR-011, ADR-018 |
| A25 | Réflexion et résonance de bassin | 3 | ADR-011 |
| A26 | Masse ajoutée | 2 | ADR-008 |
| A27 | Vue sous-marine | 3 | **ADR-019** *(S02)* |
| A28 | Horloge, arrivée en partie, temps accéléré | 2 | ADR-003 |
| **A29** | L'inondation est limitée par l'air, pas par l'eau | 2 | ADR-015 §2 |
| **A30** | Poche d'air d'une coque retournée, et son point de non-retour | 2 | ADR-015 §3 |
| **A31** | L'aération réduit la densité effective | 3 | ADR-014 §5.2 |
| **A32** | L'interface air/eau est un mur acoustique | 2 | ADR-016 §4.1 |
| **A33** | L'écume doit être advectée par la vitesse orbitale | 3 | ADR-014 §2.1 |
| **A34** | La couverture de moutons se compte en pour cent | 3 | ADR-014 §3.1 |
| **A35** | La portance de la glace suit `h²`, pas la flottabilité | 3 | ADR-017 §3 |
| **A36** | La marée est prédictible — capacité offerte, non exploitée | 3 | ADR-018 §4 |
| **A37** | On est emporté bien avant de nager | 2 | ADR-018 §3 |
| **A38** | Réflexion totale interne : la surface est un miroir | 3 | ADR-019 §2 |
| **A39** | Une mer agitée ne gèle pas en plaque | 3 | ADR-017 §2.1 |
| **A40** | Délai de propagation du son | 3 | ADR-016 §3 |
| **A41** | Le harnais est une contrainte d'architecture, pas un outil de test | 1 | ADR-020 |
| **A42** | Le régime D2 : reproductible sur la même machine | 2 | SPEC-003 §2 |
| **A43** | Comparer des solveurs à `dx` égal désigne le mauvais | 1 | SPEC-003 §5.2 |
| **A44** | Sans oracle indépendant, « différent » et « faux » se confondent | 2 | SPEC-003 §5.1 |
| **A45** | La dérive lente est invisible aux seuils par commit | 2 | SPEC-003 §7.1 |
| **A46** | Le chemin de dégradation est le moins testé et le plus exécuté | 2 | SPEC-003 §9.1 |
| **A47** | Sans paire nulle, un jury perceptuel produit du bruit | 3 | SPEC-003 §5.3 |
| **A48** | La version de pilote graphique doit être archivée avec la mesure | 3 | SPEC-003 §7.3 |
| **A49** | Torricelli à charge variable : vidange deux fois plus longue | 3 | ADR-010, cas C12 |
| **A50** | Le champ de fond doit fournir ses dérivées, sinon le terme source est faux | 1 | SPEC-004 §6.1 |
| **A51** | W doit publier un instantané immuable par tick | 2 | SPEC-004 §1.2 |
| **A52** | La pose du solide doit être interpolable dans le tick | 3 | SPEC-004 §7.1 |
| **A53** | Une dégradation silencieuse est une dégradation non mesurable | 2 | SPEC-004 §4.1 |
| **A54** | Un domaine ne doit jamais bloquer sur le streaming | 2 | SPEC-004 §8.3 |
| **A55** | Un champ 2D intégré n'est pas dérivable d'un journal : coût réseau caché | 2 | SPEC-004 §5.1 |
| **A56** | Budget mémoire et budget de temps fixés indépendamment | 1 | ADR-012 §3, écart R04 |
| **A57** | La dégradation la moins chère coûte du travail au pire moment | 2 | ADR-012 §4, écart R06 |
| **A58** | Deux ADR inventant la même parade signalent un concept manquant | 2 | ADR-006 §2, écart R07 |
| **A59** | Le géoïde absent de l'outil de terrain : 70 m d'écart à 30 km | 1 | **⏳** SPEC-005 §4 — attend l'équipe terrain |
| **A60** | Le cycle eau ↔ terrain bloque le pipeline | 2 | SPEC-005 §3 |
| **A61** | Une retouche de bathymétrie invalide la réfraction sur des kilomètres | 2 | SPEC-005 §8 |
| **A62** | L'obsolescence des données cuites est silencieuse | 2 | SPEC-005 §7.3 |
| **A63** | Un état côtier stocké en 3D pèse 197 Mo par plage | 2 | SPEC-005 §6 |
| **A64** | Le `shape_lut` détecte gratuitement les maillages non étanches | 3 | SPEC-005 §9 |
| **A65** | Le chemin poussé n'existe dans aucun document d'interface | **1** | SPEC-004 §10.6, écart E04 |
| **A66** | Une cuisson qui fait tourner δ ne peut pas être reproductible entre machines | **1** | SPEC-005 §7.2, écart E07 |
| **A67** | Un facteur d'économie fondé sur un rapport d'échelles s'effondre là où le rapport change | 2 | SPEC-004 §6.2, écart E08 |
| **A68** | La vitesse de surface mêle orbitale et courant ; un seuil de danger calculé dessus oscille | 2 | SPEC-002 §5, écart E05 |
| **A69** | Une passe de validation logée dans le mode rapide lui fait lire ce que ce mode exclut | 2 | SPEC-005 §7.3, écart E10 |
| **A70** | Un coût de cuisson chiffré en temps simulé se lit comme un temps de calcul | 3 | SPEC-005 §11.5, écart E09 |
| **A71** | Une signature survit à la décision qui la vide de son objet | 2 | SPEC-004 §3, SPEC-006 §3.2 |
| **A72** | Un `half` se choisit sur l'étendue de la grandeur, pas sur la précision voulue | 3 | SPEC-006 §3.1 et §6 |
| **A73** | L'anticipation locale fait jouer deux fois le même impact | 2 | SPEC-006 §3.3 |
| **A74** | Une prédiction publiée sans son hypothèse reste crue après que l'hypothèse a cessé | 2 | SPEC-006 §5.4 |
| **A75** | Un point inscrit dans « ce qui reste ouvert » échappe aux audits | 2 | ADR-022 §2.2 |
| **A76** | Une dégradation plus rapide que la restauration de ce qu'elle détruit fait pomper | 2 | ADR-012 §4 rang 5, ADR-022 §2.6 |
| **A77** | Le serveur charge des données cuites — il n'est pas « sans assets » | 2 | ADR-022 §5.1 |
| **A78** | Une correction s'applique là où vit l'affirmation ; un point ouvert n'affirme rien | 2 | AUDIT-POINTS-OUVERTS-S11 §6.2 |
| **A79** | Le corpus a onze destinataires extérieurs, la liste officielle en portait quatre | 2 | AUDIT-POINTS-OUVERTS-S11 §7.2 |
| **A80** | Quatre points disent « à spécifier » sans qu'aucune session ne l'ait pris en charge | 2 | AUDIT-POINTS-OUVERTS-S11 §7.4 |
| **A81** | L'impact d'entrée est plus bref que le tick : une force échantillonnée le rate ou le double | **1** | ADR-023 §2.1 |
| **A82** | Un critère de bascule de mode ne mesure qu'une chose, et ce n'est pas toujours la bonne | 2 | ADR-023 §3.1 |
| **A83** | Un phénomène permanent modélisé par émission d'événements sature le réseau en régime nominal | 2 | ADR-023 §4.1 |
| **A84** | Un invariant ne s'audite jamais : il n'est ni une affirmation datée ni une absence | **1** | ADR-024 §4, écarts E01 et E07 |
| **A85** | Aucune taille de structure du corpus n'avait jamais été vérifiée | 3 | REVUE-CROISEE-S13, écart E04 |
| **A86** | Deux échelles de rangs de dégradation portent les mêmes numéros | 2 | SPEC-006 §7, écart E02 |
| **A87** | Un invariant qui nomme un mécanisme vieillit avec lui | **1** | ADR-026 §1, audit S14 |
| **A88** | La masse d'un nœud V était transférée à un solveur jamais déterministe | **1** | ADR-025, audit S14 |
| **A89** | Une action décidée qui n'entre dans aucune liste exécutable n'est pas exécutée | 2 | AUDIT-REGISTRES-S15 §R03 |
| **A90** | Un registre dit où un point est discuté, jamais s'il est refermé | 2 | AUDIT-REGISTRES-S15 §R04 |
| **A91** | Une question dissoute peut revenir sous une autre forme, sans que son statut bouge | 2 | AUDIT-REGISTRES-S15 §R06 |

Soixante-quatre angles morts recensés, tous traités ou explicitement cadrés. Aucun n'est laissé sans
propriétaire.

**Quatre ont été trouvés dans nos propres écrits** — A49, A56, A57, A58 — et non dans les documents
sources. A49 est apparu en écrivant le test qui devait le vérifier ; les trois autres en confrontant
les ADR les uns aux autres (`REVUE-CROISEE-S05.md`). Aucun n'était visible en relisant le document
qui le contenait : il fallait un second document pour les révéler.

**A56 mérite d'être retenu au-delà de ce projet.** ADR-012 fixait `memoire_blocs = 384 Mo` et
`domaines_max = 24`, chiffres vérifiés cohérents entre eux — mais le budget de temps du même profil
n'autorisait qu'environ un sixième de ce nombre de domaines. Deux ressources vérifiées, une
capacité dérivée déclarée comme si elle était libre, et la contradiction est restée invisible
pendant quatre sessions.

---

## Les points à retenir, en une ligne chacun

**Sévérité 1 — auraient imposé une refonte d'architecture**

- **A01** — Rien n'est immobile : la surface s'oriente sur la gravité apparente. Un solveur qui
  code `−9,81·Z` en dur est disqualifié.
- **A02** — Depuis une passerelle à 30 m, l'horizon est à 19,6 km. Un océan plan fausse la
  détection autant que le rendu.
- **A03** — À 100 km de l'origine, l'ulp d'un `f32` vaut 12 mm.
- **A04** — `λ = 2πv²/g` : un sillage à 10 m/s a des vagues de 64 m.
- **A06** — Une vague qui traverse un domaine discret en ressort déphasée.
- **A07** — Une force lue depuis un solveur GPU arrive 1 à 3 frames en retard.
- **A08** — Sans sémantique IEEE stricte, deux clients ne voient pas la même mer.
- **A16** — Un client modifié pouvait fabriquer un tsunami par la transduction. ADR-021 §3
  supprime le chemin d'énergie client → serveur : l'angle mort n'est plus atténué, il n'existe plus.
- **A59** — Un outil de terrain en plan tangent place une plage située à 30 km de l'ancre de région
  **70 m** au-dessus ou au-dessous du niveau de la mer. À 3 km, l'erreur dépasse déjà le marnage.
  Le « zéro » d'une scène est une distance au centre de la planète, pas une altitude.

**Ajoutés en S06 — pipeline**

- **A60** — Le graphe eau ↔ terrain contient un cycle : la rivière impose sa pente au terrain, le
  trait de côte impose le fetch, la bathymétrie impose la zone de déferlement. Non brisé, il bloque
  le pipeline. L'eau doit être déclarée **en amont** du terrain — l'inverse de la pratique courante.
- **A61** — Une houle ne sent le fond qu'en deçà de `h = λ/2`. Retoucher un haut-fond invalide donc
  la réfraction jusqu'à l'isobathe 50 m pour une houle de 100 m, soit **10 km au large** sur un
  plateau à 1:200. Les partitions de cuisson suivent les isobathes, pas une grille carrée.
- **A62** — Une donnée cuite ne se périme pas bruyamment : un rocher déplacé rend une polyligne de
  déferlement fausse pendant des mois. Chaque artefact porte l'empreinte de ses entrées, et la
  construction échoue s'il est périmé.
- **A63** — Seize états côtiers stockés en volumes 3D pèsent 197 Mo par plage. Stockés en conditions
  initiales 2D : 1,2 Mo, et le volume se ré-établit en 2 à 3 s au lieu de 40.
- **A64** — La courbe volume → hauteur d'un contenant est monotone par construction physique. Si le
  calcul ne l'est pas, le maillage fuit : la cuisson est un test d'étanchéité gratuit.

**Sévérité 2 — refonte d'un sous-système**

- **A09** — Halver `dx` coûte ×16, pas ×8.
- **A10** — L'énergie voyage à `c/2`. Toute éponge, tout horizon, tout établissement s'en déduit.
- **A11** — Mélanger deux champs de houle indépendants fait chuter `Hs` de 29 %.
- **A12** — L'audio consomme les mêmes champs que le rendu ; câblé après coup, il aura son propre
  LOD incohérent.
- **A14** — Le serveur n'a pas de caméra : sa hiérarchie est l'intérêt, pas la visibilité.
- **A17** — Ce n'est pas un système d'eau, c'est un système de liquides.
- **A21** — Le battement qui tue n'est pas logique, il est allocatoire.
- **A22** — Une rivière descend de 1,6 ‰ ; on ne la pose pas à plat sur un terrain existant.
- **A24** — 3 m de marnage sur une pente 1:50 déplacent le rivage de 150 m.
- **A26** — La masse ajoutée d'une coque vaut environ la masse déplacée.
- **A28** — Un temps en `f32` a un ulp de 62 ms après 12 jours de session.
- **A29** — Un compartiment fermé s'inonde à la vitesse à laquelle l'air sort, pas à celle de
  Torricelli. Sans arête d'évent, tous les temps d'avarie sont trop rapides.
- **A30** — Une coque retournée flotte grâce à sa poche d'air, qui perd la moitié de son volume à
  10 m : il existe un point de non-retour.
- **A32** — 0,11 % de l'énergie sonore traverse l'interface. Le mixage immergé n'est pas un
  passe-bas, c'est une coupure et un lit distinct.
- **A37** — 50 cm d'eau à 2 m/s emportent un adulte. Un critère fondé sur la seule profondeur fera
  traverser des torrents aux PNJ.

**Sévérité 3 — travail localisé**

- **A05** — Au-delà de `Fr_h = 1`, le sillage devient un cône de Mach ; 19,47° codé en dur est
  faux dans toutes les zones côtières.
- **A13** — Le mouillage de surface est le puits universel de tout ce qui est trop petit pour être
  un volume.
- **A18** — Une brèche vers le vide évapore 14 % de l'eau et gèle les 86 % restants, ce qui obture
  la brèche.
- **A23** — Coriolis à 3 tr/min vaut 6,4 % de `g` ; un bassin de 20 m dans un anneau de 100 m a une
  flèche de 50 cm.
- **A25** — Un port a un mode propre de bassin ; une seiche de lac se compte en minutes.
- **A31** — 10 % d'aération, c'est 10 % de portance en moins : on ne flotte pas dans l'eau blanche.
- **A33** — Advectée par le seul courant moyen, l'écume reste uniforme et lit comme une texture.
- **A34** — Une mer force 7 est blanche à 4 %, pas à 40 %.
- **A35** — Doubler l'épaisseur de la glace quadruple sa charge admissible.
- **A36** — B étant analytique, la marée est prédictible : « ce gué se ferme dans 40 minutes » est
  une capacité offerte gratuitement, qu'il serait dommage de ne pas exploiter.
- **A38** — Au-delà de 48,6°, la surface vue de dessous est un miroir. C'est la caractéristique la
  plus reconnaissable d'une vue sous-marine et la plus souvent omise.
- **A39** — La glace en plaque exige `Hs < 0,15 m` : elle apparaît d'abord dans les eaux abritées.
- **A40** — Une explosion à 500 m s'entend 1,46 s après avoir été vue.
- **A47** — Un jury motivé trouve des différences partout ; sans paire nulle on ne sait pas si
  le résultat dépasse le bruit.
- **A48** — La version de pilote explique la moitié des sauts de performance inexpliqués.
- **A49** — Un temps de vidange calculé à charge constante est faux d'un facteur deux.

**Ajoutés en S03, sévérité 1 et 2 — tous méthodologiques**

- **A41** — Le harnais échoue non parce qu'il est mal écrit, mais parce que le système testé ne se
  laisse pas instancier seul. Cela se décide au premier jour, pas au moment d'écrire les tests.
- **A42** — Sans reproductibilité sur une même machine, un bug qui survient une fois sur cinquante
  n'est jamais reproduit, donc jamais corrigé.
- **A43** — Comparer deux solveurs à `dx` égal mesure leur coût par cellule, pas leur rendement.
  Un candidat deux fois plus cher par cellule mais correct à `dx` double fait seize fois moins de
  travail. Une campagne à `dx` fixe désignera le mauvais, et l'erreur sera défendue par des
  chiffres.
- **A44** — Comparer une version à la précédente ne dit jamais laquelle est juste. L'oracle est un
  solveur délibérément lent, pas le build d'hier.
- **A45** — 1 % de dégradation par semaine : invisible à chaque commit, **+14 % par trimestre**,
  **+68 % sur un an**. C'est ainsi que les budgets se perdent.
- **A46** — Le chemin de dégradation tourne sur la configuration minimale, donc chez la majorité
  des joueurs, et n'est presque jamais testé.

**Ajoutés en S04, trouvés en écrivant les signatures**

- **A50** *(sévérité 1)* — En régime perturbatif, l'équation de la perturbation contient un terme
  source `S = ∂U/∂t + (U·∇)U + ∇P/ρ − ν∇²U` qui n'est pas nul, parce que le fond résout les
  équations d'ondes linéaires et non les équations discrètes du solveur. Un solveur qui ne reçoit
  que hauteur et vitesse ne peut pas former `S` : le domaine dérive lentement par rapport au fond,
  la frontière redevient visible, et **on conclut à tort qu'ADR-001 ne fonctionne pas**. Première
  hypothèse à écarter si le banc B4 échoue.
- **A51** — `EvalWater` est lue simultanément par les fils physique, audio et IA pendant que la
  simulation avance. Sans instantané immuable publié par tick, un lecteur voit un jeu de paquets à
  moitié mis à jour : défaut intermittent, rare, introuvable.
- **A52** — Un solveur qui sous-cycle quatre fois en utilisant la pose de début de tick fait
  avancer une coque à 10 m/s par sauts de 33 cm, soit plusieurs cellules à `dx = 0,10 m`.
- **A53** — Un solveur qui coupe ses itérations de pression sans le dire rend l'ordonnanceur aveugle
  et le banc de dégradation muet.
- **A54** — Sans test d'admission sur la résidence des données, un domaine bloque sur le streaming
  ou simule au-dessus d'un trou de bathymétrie.
- **A55** — Un champ de hauteur 2D intégré n'est pas une fonction pure du journal d'événements. Le
  déterminisme exige alors des points de reprise à stocker, répliquer et transmettre — le coût
  réseau qu'ADR-009 avait précisément supprimé. À intégrer au protocole de B2, faute de quoi la
  comparaison se fera sur la seule qualité visuelle.

**Ajoutés en S08, tous trouvés dans nos propres écrits en les croisant**

- **A65** *(sévérité 1)* — Une spécification d'interfaces écrite du point de vue du consommateur qui
  *interroge* laisse passer tout ce que le système **publie** sans qu'on le lui demande. Trois ADR
  ont alors décrit chacun sa propre publication — champ d'écume, signal de traversabilité, bus audio
  — dans son propre vocabulaire, et trois interfaces inter-équipes se sont retrouvées sans aucun
  document à soumettre. Le défaut ne se voit pas en relisant le document : il n'y a rien à y voir.
- **A66** *(sévérité 1)* — Une chaîne d'outils entière peut être bâtie sur une exigence de
  déterminisme **inatteignable par construction**. Ici : exiger une cuisson reproductible bit à bit
  d'un pipeline qui fait tourner un solveur δ, dont un autre document dit qu'il n'est jamais D1.
  Chaque phrase est juste dans son document ; leur conjonction est impossible.
- **A67** — Un facteur d'économie justifié par « X est grand devant Y » s'effondre dans le régime,
  déjà écrit ailleurs, où le rapport est le plus petit. L'échantillonnage grossier du champ de fond
  passe de 10 à 4 points par longueur d'onde entre le scénario nominal et la zone de déferlement —
  et c'est la zone de déferlement qui a 384 k cellules.
- **A68** — La vitesse de surface publiée mêle **orbitale et courant**. Tout seuil calculé dessus —
  danger d'emportement, traversabilité, IA — oscille à la période de la houle, avec une amplitude
  (`πHs/T` = 0,63 m/s à Hs = 1 m, T = 5 s) du même ordre que la grandeur mesurée.
- **A69** — Une passe de validation logée par commodité dans le mode le plus rapide de la CI lui
  fait lire exactement ce que ce mode exclut par conception. L'intention — un seul système de
  validation — était juste ; c'est la cadence qui était mal choisie.
- **A70** — Un coût de cuisson chiffré en **temps simulé** se lit comme un temps de calcul, et
  contredit silencieusement une autre phrase du même document.

**Ajoutés en S09, tous apparus en écrivant les signatures du chemin poussé**

- **A71** — Une **signature survit à la décision qui la vide de son objet**.
  `drain_outgoing_events()` servait le chemin δ→serveur qu'ADR-021 §3 a supprimé en S05 ; l'écart
  R03 avait retiré le chemin de données, pas la fonction. Elle est restée neuf mois de conception
  dans un document dont le statut est « dernier avant l'écriture de code », et elle aurait été
  implémentée. Une décision qui supprime un mécanisme doit lister les **signatures** qu'elle
  périme, pas seulement les paragraphes.
- **A72** — Un `half` se choisit sur l'**étendue** de la grandeur, jamais sur la précision voulue.
  Deux cas dans un seul document : `displaced_ml` sature à 65 litres, dépassé par toute claque de
  coque ; un flux dissipé en W/m sature à 65 kW/m, dépassé dès `Hs = 4 m`. Dans les deux cas
  l'erreur est invisible en relecture et se manifeste comme une saturation silencieuse au cas le
  plus spectaculaire — celui qu'on remarquera le plus tard et qui compte le plus.
- **A73** — L'**anticipation locale** d'ADR-009 §7.2 et le bus d'événements partagé, pris ensemble,
  font jouer deux fois le même impact à 100–300 ms d'intervalle. Chacun des deux mécanismes est
  correct seul. C'est leur conjonction qui produit le défaut, et personne n'en est propriétaire.
- **A74** — Une **prédiction publiée sans son hypothèse** reste crue après que l'hypothèse a cessé
  d'être vraie. « Ce gué se ferme dans quarante minutes » suppose que seule la marée agit ; une
  vanne ouverte en amont laisse la valeur en place, fausse, jusqu'à la republication de la tuile.
  Vaut pour toute donnée prédictive publiée : elle porte sa cause, ou elle ment.

**Ajoutés en S10, en écrivant la persistance**

- **A75** — **Un point inscrit dans une liste « ce qui reste ouvert » échappe aux audits.** La
  contradiction entre ADR-007 §3 (« persistance hors caméra ») et ADR-013 §6 (« il n'existe pas de
  simulation ralentie hors caméra ») est interne à S01, visible en rapprochant deux paragraphes, et
  a traversé la revue croisée des vingt ADR **et** celle des cinq SPEC. Motif : un audit vérifie ce
  qui est *affirmé*, et un point reporté se lit comme une lacune connue et suivie. Personne ne va
  vérifier qu'une question ouverte a encore un objet.
- **A76** — Une dégradation dont la **fenêtre d'engagement** est plus courte que le **coût de
  restauration** de ce qu'elle détruit est un générateur de pompage. Ici : 1 s d'engagement
  (ADR-012 §5) contre 4,4 à 8 s de rétablissement d'un domaine substitutif. Le pompage est plus
  visible que la dégradation qu'on cherchait à éviter — ADR-012 §5 le dit lui-même pour la manette
  de qualité, sans avoir appliqué le raisonnement à ses propres rangs.
- **A77** — **Le serveur charge des données cuites.** I-10 lui interdit d'exécuter W et δ, ce qui
  fait facilement croire qu'il n'a besoin d'aucun actif. Mais il exécute V, et V a besoin des
  `shape_lut` pour convertir un volume en hauteur — sans quoi il ne peut ni décider d'un
  débordement ni évaluer une ligne de flottaison. Un serveur « sans assets » n'est pas une option,
  et c'est une contrainte de déploiement à annoncer tôt.

**Ajoutés en S11, en auditant les listes « ce qui reste ouvert »**

- **A78** — **Une correction s'applique là où vit l'affirmation qu'elle corrige.** Un point ouvert
  n'affirme rien : il déclare une absence, et personne ne relit une liste d'absences en se demandant
  si l'une d'elles a été comblée. Quatre instances trouvées — ADR-007 §5.3, ADR-006 §7.3,
  ADR-017 §7.2, SPEC-004 §10.3 — dont la dernière a la correction et le point périmé **dans le même
  document, à quatre sections d'écart** : ce n'est donc pas une affaire de distance.
- **A79** — Le corpus adresse une demande à **onze destinataires extérieurs**, là où la liste
  officielle en portait quatre. Les sept manquants — véhicules, personnage, gameplay spatial,
  gameplay survie, réseau/physique solide, gameplay, assurance qualité — étaient chacun cités dans
  un point ouvert, c'est-à-dire à l'endroit où personne ne va chercher une dépendance.
- **A80** — Quatre points disent « à spécifier » et **aucune session ne l'a jamais pris en charge** :
  le terme d'impact de flottabilité, le modèle du nageur en surface, le comportement des rochers
  turbulents permanents, la coalescence de deux poches d'air. Ce n'est ni une mesure, ni un
  arbitrage, ni une dépendance : c'est du travail de conception rendu invisible par l'endroit où il
  est inscrit.

**Ajoutés en S12, en spécifiant les quatre mécanismes**

- **A81** *(sévérité 1)* — **L'impact d'entrée dans l'eau est plus bref que le tick de simulation.**
  73 ms pour une étrave de vedette, **17 ms pour un corps humain tombant de trois mètres**, contre
  33 ms de tick. Une force échantillonnée à 30 Hz le rate ou le double selon la phase, et la
  dispersion qui en résulte sur des entrées identiques est un défaut intermittent, donc introuvable.
  Et le phénomène est **autoritaire** — c'est lui qui casse une coque ou fait tomber un personnage.
- **A82** — Un critère de bascule de mode ne mesure **qu'une seule chose**, et l'on croit ensuite
  qu'il les couvre toutes. Le mode contraint d'ADR-008 §3 se déclenche sur la **stabilité
  numérique** ; un nageur le passe largement (`ω·dt ≈ 0,14`) alors que c'est ce mode qu'il lui faut,
  pour le contrôle et la caméra. Le critère n'était pas faux, il était seul.
- **A83** — Un phénomène **permanent** modélisé par émission d'événements sature le réseau en régime
  nominal, pas en pic. Deux cents rochers émettant un événement par seconde font 9 000 o/s par
  joueur intéressé, soit dix fois le débit d'une bataille navale — pour du décor, et en permanence.
  Un phénomène stationnaire et déterministe se **dérive**, il ne s'émet pas.

**Ajoutés en S13, en confrontant les documents récents au corpus**

- **A84** *(sévérité 1)* — **Un invariant ne s'audite jamais.** Il n'est ni une affirmation datée
  qu'on confronte à d'autres — c'est le rôle des revues croisées — ni une absence qu'on relit pour
  vérifier qu'elle a encore un objet — c'est l'audit des points ouverts. Il se présente comme un
  **socle**, c'est-à-dire comme ce contre quoi on vérifie le reste, et la flèche `→ ADR-xxx` qu'il
  porte se lit comme une provenance et non comme une dépendance à surveiller. Deux des dix-sept se
  sont révélés faux à la première tentative : I-11 exigeait un mécanisme aboli en S05, I-12 était
  universel là où ADR-013 §4 — qu'il cite en source — le contredit pour la moitié des cas. Et un
  invariant faux ne produit pas une erreur mais deux, en sens contraires : on l'applique, ou on
  « rétablit » ce qu'il décrit.
- **A85** — **Aucune taille de structure du corpus n'avait jamais été vérifiée.** S05 a contrôlé les
  tables numériques dupliquées, S08 a recalculé une quarantaine de valeurs depuis leurs formules ;
  personne n'a additionné les champs d'un `struct`. Trois structures sur trois étaient fausses, et
  l'erreur d'origine — 40 octets annoncés pour 45 — s'est propagée dans six documents, chacun
  recalculant depuis le chiffre faux du précédent. Sans conséquence ici ; c'est la classe de contrôle
  absente qui compte.
- **A86** — Deux **échelles de rangs de dégradation** portent les mêmes numéros dans deux documents
  qui se citent, et une règle écrite pour l'une se lit sans difficulté dans l'autre — avec un
  contresens complet. Vaut pour toute numérotation ordinale reprise d'un document à l'autre :
  préfixer coûte un caractère.

**Ajoutés en S14, en auditant les invariants contre leurs ADR sources**

- **A87** *(sévérité 1)* — **Un invariant qui nomme un mécanisme vieillit avec lui.** Dix invariants
  sur dix-sept ne disaient plus ce que leur source dit, et le partage est net : ceux qui tiennent sont
  ceux qu'une **signature rend mécaniques** (I-07, `g_eff` injecté dans `configure` ; I-08, aucun type
  n'exprimant une coordonnée monde) ou ceux qui servent à **décider** plutôt qu'à refuser (I-09,
  I-15). Ceux qui ont vieilli **nomment un mécanisme** : I-01 nommait une fonction, I-11 un
  plafonnement, I-16 une catégorie de valeur. C'est une règle d'écriture, pas seulement un constat —
  et elle est de sévérité 1 parce qu'un invariant faux fait refuser ce qu'il fallait accepter, ou
  rétablir ce qu'une décision avait supprimé.
- **A88** *(sévérité 1)* — **La masse d'un nœud V était transférée à un solveur qui n'est jamais
  déterministe.** ADR-010 §6 « remettait » la masse d'un compartiment au domaine δ pendant tout un
  épisode d'inondation : le volume qui décide d'un chavirement était donc produit par δ, que I-04
  interdit comme source d'issue de jeu. Et le serveur, qui exécute V et jamais δ, ne pouvait ni geler
  le nœud ni recevoir la masse en retour — il n'avait aucune histoire. Le défaut a survécu treize
  sessions parce qu'ADR-010 ne cite pas I-04 et qu'I-04 ne cite pas ADR-010 : **les revues croisées
  confrontent des documents qui se citent.**
- **A89** — Une action décidée **qui n'entre dans aucune liste exécutable n'est pas exécutée.**
  *(Reformulé en S15 : l'énoncé d'origine attribuait le défaut aux tables « Suite » des registres
  d'audit. C'était faux dans sa prémisse — le registre concerné, `AUDIT-POINTS-OUVERTS-S11`, n'avait
  pas de table, et les quatre registres qui en portent une affichent **40 actions sur 40 exécutées**.)*
  Le partage observé est net : **sept actions du corpus visaient un banc ou le harnais ; les quatre
  exécutées l'ont été par la session qui les décidait, dans une étape inscrite à son plan
  (`notes/EN-COURS.md`), et les trois perdues avaient été annoncées dans le corps d'un document.**
  Une annonce faite en prose est une intention, pas une tâche — et cela vaut même à l'intérieur
  d'une seule session : le cas canonique annoncé par ADR-025 §4 en S14 P6a n'a pas été posé, le plan
  de S14 ne prévoyant aucun cas canonique.

**Ajoutés en S15, en auditant les registres**

- **A90** — **Un registre dit où un point est discuté, jamais s'il est refermé.** La colonne
  « Traité dans » d'`ANGLES-MORTS` portait trois situations que rien ne distinguait : *supprimé*,
  *comblé*, et *en attente d'un tiers*. Conséquence mesurable : **A59**, sévérité 1 — le géoïde
  absent de l'outil de terrain, 70 m d'écart à 30 km — affichait exactement la même chose qu'un point
  réglé. Un lecteur en concluait que le problème était traité ; il est **décrit**, et le restera tant
  que l'équipe terrain n'aura pas changé le référentiel de son outil. Vaut pour tout registre qui
  accumule sans jamais se vider : 89 points en quinze sessions, un seul portant une fermeture.
- **A91** — **Une question dissoute peut revenir sous une autre forme, sans que son statut bouge.**
  « Quelle erreur de précalcul est acceptable ? » avait été dissoute à juste titre : un domaine
  préparé ne contient aucune information physique. Cinq sessions plus tard, une **seconde forme de
  précalcul** — la graine cuite — en contenait, et avec elle un seuil de tolérance réel, à calibrer
  au banc. Le registre annonçait toujours qu'il n'y avait rien à calibrer. Une dissolution est
  valide *pour un mécanisme donné* ; elle ne l'est pas pour la question.