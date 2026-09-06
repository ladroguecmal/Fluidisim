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
documents récents), 3 en S14 (audit inverse des invariants), 2 en S15 (audit des registres), 2 en S16 (dossier B2), 2 en S17
(dossier de réunion), 1 en S18 (arbitrages), 1 en S19 (nature du projet), 3 en S20 (première ligne de code), 4 en S21 (cas analytiques), 4 en S22 (C01 et le premier δ), 4 en S23 (C04 et le lit sec), 4 en S24 (C08 et l'oracle), 4 en S25 (C03 et la dissipation), 4 en S26 (les harmoniques), 4 en S27 (le nombre de Courant), 3 en S28 (la paroi mobile) — **131 au
total**. **Soixante et onze ont été trouvés dans nos propres écrits**, pas dans les documents sources :
A49, A56, A57, A58, puis A65 à A131. La proportion
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
| **A92** | La largeur d'éponge borne `λ_cut` par le haut, et c'est le plus petit domaine qui décide | **1** | ADR-005 §5, DOSSIER-B2 §3 |
| **A93** | Un banc produit un couple de valeurs liées, jamais une valeur seule | 2 | DOSSIER-B2 §7 |
| **A94** | Deux demandes bloquent la première ligne de code et n'étaient présentées nulle part comme urgentes | **1** | DOSSIER-REUNIONS §3 |
| **A95** | Les deux demandes les plus bloquantes n'ont aucun destinataire nommé | **1** | DOSSIER-REUNIONS §7.2 |
| **A96** | Un arbitrage qui traîne est souvent un arbitrage mal posé | 2 | ADR-027 §1 |
| **A97** | Reporter à une équipe qui n'existe pas est plus confortable que reporter tout court | **1** | ADR-028 §2.2 |
| **A98** | `sin` n'est pas spécifié bit à bit : le déterminisme de B ne survit pas à un appel de bibliothèque | **1** | ADR-029 §2 |
| **A99** | Le grain d'une réduction ordonnée change son résultat | 2 | ADR-029 §3 |
| **A100** | Un test peut affirmer une propriété vraie avec des données incapables de la révéler | 2 | ADR-029 §3, `host_impl.rs` |
| **A101** | Un champ reproductible peut être reproductiblement faux | **1** | ADR-029, note S21 |
| **A102** | Une mesure statistique a besoin d'une fenêtre de plusieurs fois la plus longue onde | 2 | ADR-029, note S21 |
| **A103** | Le corpus n'a jamais fixé la masse volumique de l'eau, dont dépendent des références | 2 | `body.rs`, C10 |
| **A104** | Une règle énoncée dans un fichier n'empêche pas sa violation dans le même fichier | 2 | `physics.rs`, C10 |
| **A105** | Un cas canonique peut être moins discriminant que son énoncé ne le laisse croire | **1** | ADR-030 §6.1, C01 |
| **A106** | Une tolérance sans provenance traverse vingt-deux sessions sans être questionnée | 2 | ADR-030 §6.3, I-14 |
| **A107** | Une fusion faite par import de contenu ne referme pas le fork qui l'a causée | **1** | `FORK-S08-S15.md`, S22 |
| **A108** | Une propriété exacte vérifiée sur son intérieur seul ne dit rien de ses bords | 2 | ADR-030 §4, `delta.rs` |
| **A109** | Une explication correcte et documentée peut n'expliquer aucune part du défaut observé | 2 | ADR-031 §1.1 |
| **A110** | Un seuil de mesure ne déplace pas seulement un verdict, il peut le renverser | **1** | ADR-031 §3.1, C04 |
| **A111** | Une erreur globale faible masque une erreur locale vingt fois plus grande | **1** | ADR-031 §1, C04 |
| **A112** | Une constante sans effet mesurable a une provenance, et ce n'est pas une dette | 3 | ADR-031 §4, `H_SEC` |
| **A113** | Un ordre de convergence est une propriété du couple (solveur, cas), pas du solveur | **1** | ADR-032 §3, C08 |
| **A114** | Avec un oracle, le triplet le plus fin est le moins fiable — la règle s'inverse | **1** | ADR-032 §4 |
| **A115** | Un critère d'écart local ne distingue pas « a convergé » de « progresse lentement » | 2 | ADR-032 §5 |
| **A116** | Une erreur d'hôte absorbée publie un résultat vide qui a l'air d'un résultat | 2 | ADR-032 §5, `physics.rs` |
| **A117** | Le nombre de Courant commande la dissipation, et le corpus n'en parle nulle part | **1** | ADR-033 §2.3 |
| **A118** | Un mot partagé — « dissipation » — a fait recommander l'inverse de ce qu'il fallait, trois sessions durant | 2 | ADR-033 §1 |
| **A119** | Un cas passe parce que son montage n'est pas dans le régime où le système vivra | **1** | ADR-033 §2.1, C03 |
| **A120** | Une conclusion juste peut ne pas épuiser la question qu'elle ferme | 2 | ADR-033 §3, ADR-030 §5 |
| **A121** | Rien n'a vérifié qu'un spectre se comporte comme la somme de ses modes pris séparément | 2 | ADR-034 §3.2 |
| **A122** | Si δ mange les composantes courtes, personne n'a dit si la transition les réinjecte | **1** | ADR-034 §3.3, ADR-005 |
| **A123** | Le budget de résolution se dimensionne sur la composante la plus courte à conserver, pas sur la dominante | **1** | ADR-034 §2.2 |
| **A124** | Un cas qui vit dans le code et pas dans le corpus est introuvable pour la session suivante | 2 | C22, ADR-032 §6.4 |
| **A125** | `u_max` de la CFL n'a jamais été défini, alors qu'une SPEC impose des parois mobiles | **1** | ADR-035 §2, SPEC-001 §2.1 |
| **A126** | Une marge par défaut peut protéger d'un défaut que personne n'a identifié | **1** | ADR-035 §4.1 |
| **A127** | Une loi mesurée sur un régime est publiée sans son domaine de validité | 2 | ADR-035 §5, ADR-033 §2.2 |
| **A128** | Une source extérieure a rendu en une heure ce que sept sessions n'avaient pas vu | 3 | S27, journal |
| **A129** | Un cas qui exigerait une explosion conclurait que le défaut n'existe pas | **1** | C23, ADR-035 note S28 |
| **A130** | Le gain de portée et la fragilité à une définition fausse croissent ensemble | 2 | ADR-035 note S28 |
| **A131** | Un objet rapide dans l'eau divise le pas de temps par trois, et rien ne le budgétait | 2 | C23, C20 |

Quatre-vingt-onze angles morts recensés, tous traités ou explicitement cadrés. Aucun n'est laissé sans
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

**Ajoutés en S16, en préparant l'exécution du banc B2**

- **A92** *(sévérité 1)* — **La largeur d'éponge borne `λ_cut` par le haut, et c'est le plus petit
  domaine qui décide.** ADR-005 §5 pose `L_s = λ_cut/2` par face et conclut que `λ_cut` « fixe le
  coût minimal d'un domaine » — sans jamais calculer ce que cela laisse. L'intérieur utile vaut
  `W − λ_cut` : sur le domaine d'impact de référence, 6 × 6 m, la valeur proposée de 4 m ne laisse
  que **11 % de surface au sol**, et à 6 m il ne reste **rien**. Personne n'avait rapproché la
  formule d'éponge des emprises de SPEC-001 §2.4 — deux paragraphes distants de deux documents, et
  l'un des deux nombres les plus structurants du projet en dépend.
- **A93** — **Un banc produit un couple de valeurs liées, jamais une valeur seule.** B2 était écrit
  pour trancher `λ_cut` ; il doit trancher `λ_cut` **et** le taux de décimation admissible par classe
  de domaine, parce que les deux sont liés par `dx ≤ λ_cut/N` et qu'aucun des deux ne veut rien dire
  sans l'autre. Un protocole qui ne publie qu'un des deux laisse le second se choisir plus tard, à
  l'œil, par quelqu'un qui n'aura pas les mesures.

**Ajoutés en S17, en rassemblant le dossier de réunion**

- **A94** *(sévérité 1)* — **Deux demandes bloquent la première ligne de code et n'étaient présentées
  nulle part comme urgentes.** Le corpus classait ses dépendances extérieures par **gravité de
  conséquence** ; le critère utile est l'**irréversibilité** — ce que la réponse débloque. Reclassé
  ainsi, « acter ADR-020 » et « qui possède le harnais de validation » passent devant les quatre
  interfaces inter-équipes que l'index mettait en tête. La seconde était rangée depuis S03 parmi des
  questions de format de fichier, alors que H1 doit précéder la première ligne du solveur et que la
  qualité de toutes les décisions à venir est plafonnée par celle du harnais.
- **A95** *(sévérité 1)* — **Les deux demandes les plus bloquantes n'ont aucun destinataire nommé.**
  « Direction technique » et « assurance qualité technique » sont des rôles, pas des personnes.
  Personne n'est aujourd'hui identifié pour acter ADR-020 ni pour arbitrer la propriété du harnais —
  et c'est la condition préalable à la tenue même des réunions. Un dossier de demandes sans
  destinataire identifié est un dossier qui ne part pas.

**Ajouté en S18, en tranchant les arbitrages**

- **A96** — **Un arbitrage qui traîne est souvent un arbitrage mal posé.** Sur les cinq questions
  réputées « en attente d'une réponse humaine » depuis S06, **deux se sont dissoutes** dès qu'on a
  cherché à y répondre : « qui porte le trait de côte mobile » supposait qu'il soit stocké — il est
  dérivé, donc personne ne le porte ; « combien de temps l'eau d'un joueur absent persiste-t-elle »
  supposait que l'eau ait besoin d'une politique propre — elle pèse 4 Ko par joueur, elle suit celle
  de l'objet. Une troisième, la propriété du harnais, cherchait **un** propriétaire là où il en faut
  deux. L'étiquette « attend une décision humaine » avait donc masqué pendant douze sessions le fait
  qu'il n'y avait, dans trois cas sur cinq, rien à trancher — et personne ne relit une question qu'on
  a classée comme n'étant pas la sienne. C'est L03 rencontrée à un endroit nouveau : la liste des
  choses qu'on ne décide pas.

**Ajouté en S19, en apprenant qu'il n'y a pas d'autres équipes**

- **A97** *(sévérité 1)* — **Reporter à une équipe qui n'existe pas est plus confortable que reporter
  tout court.** Quatorze questions ont été classées « attend une réponse d'une autre équipe » entre
  S02 et S17. Aucune n'avait de destinataire. L'étiquette faisait deux choses à la fois : elle
  dispensait de répondre, **et elle désignait un responsable** — ce qui est plus rassurant qu'un
  simple report, et rend la question invisible à tout audit, y compris à celui de S11 qui cherchait
  précisément les points ouverts périmés. C'est A78 et L61 combinés, et amplifiés par le fait qu'un
  report **nominatif** ne se relit jamais : on ne vérifie pas qu'un tiers a répondu si l'on n'attend
  rien de précis de lui.

**Ajoutés en S20, à la première ligne de code**

- **A98** *(sévérité 1)* — **`sin` n'est pas spécifié bit à bit.** IEEE 754 impose l'exactitude des
  quatre opérations et de la racine carrée ; **jamais celle des transcendantes**. `B` étant une somme
  de sinusoïdes, son déterminisme inter-plateforme — I-03 — ne survivait pas à un appel de
  bibliothèque standard, et le défaut se serait présenté comme une divergence de plateforme sans
  cause apparente. ADR-003 §2 énumérait trois disciplines et n'avait pas celle-là. **Dix-neuf
  sessions de conception ne l'ont pas trouvé ; la première ligne de code l'a trouvé en une heure.**
- **A99** — **Le grain d'une réduction ordonnée change son résultat.** L'addition flottante n'est pas
  associative : sur `[1 ; 10¹⁶ ; −10¹⁶ ; 1]`, un grain de 1 donne `1,0` et un grain de 2 donne `0,0`.
  Un système de tâches qui choisirait son grain d'après le nombre de fils — le réglage naturel —
  rendrait faux, silencieusement, le corollaire « changer `worker_count` ne change pas le résultat ».
- **A100** — **Un test peut affirmer une propriété vraie avec des données incapables de la
  révéler.** Le premier test écrit pour A99 employait une suite arithmétique : les deux sommes
  étaient égales, et le test échouait en prétendant que la propriété était fausse. Elle ne l'était
  pas ; c'étaient les données qui étaient trop bien conditionnées. Un test qui passe sur des données
  mal choisies est pire — il aurait affirmé une garantie inexistante.

**Ajoutés en S21, au premier cas analytique**

- **A101** *(sévérité 1)* — **Un champ reproductible peut être reproductiblement faux.** La vitesse
  orbitale de `B` était en quadrature au lieu d'être en phase avec l'élévation : **sous une crête,
  l'eau n'avançait pas**, elle montait. Le hash de conformité était parfaitement stable — parce qu'il
  l'était : le champ était reproductible, et faux. Dix-neuf sessions de conception, six audits du
  corpus et un étage de harnais complet n'ont pas pu faire la différence ; une identité fermée qui ne
  dépend d'aucun paramètre du code — `u = ω·η` — l'a faite en un passage. **Le déterminisme et la
  justesse sont deux propriétés sans rapport, et aucun raffinement de la première n'approche la
  seconde.**
- **A102** — **Une mesure statistique a besoin d'une fenêtre de plusieurs fois la plus longue onde.**
  La restitution de `Hs` par la variance donne 8,5 % d'écart quand la fenêtre ne fait que
  1,7 longueur d'onde de la plus longue composante. Le champ est juste ; c'est la mesure qui est
  trop courte. Vaut pour toute estimation spectrale — et le piège est d'autant plus sûr que la
  fenêtre est dimensionnée sur la longueur d'onde *moyenne*, qui est la valeur qui vient à l'esprit.

**Ajoutés en S21, au premier calcul de force**

- **A103** — **Le corpus n'a jamais fixé la masse volumique de l'eau.** Vingt-et-une sessions, six
  SPEC, vingt-neuf ADR : `ρ_eau` n'apparaît nulle part. Les documents citent `ρ_glace = 917` et le
  `ρ = 500` du cube de C10, mais jamais la valeur à laquelle ils se rapportent. La référence de C10
  ne se referme qu'avec **1000** — l'eau douce — alors que le projet parle de mer ouverte, où la
  valeur usuelle est 1025. L'écart est de 2,5 % sur tout tirant d'eau. **Une constante qu'aucun
  document ne fixe finit par être choisie par le premier code qui en a besoin**, et ce choix ne
  ressemble alors pas à une décision. Elle est posée dans `body.rs` avec sa justification et son
  alternative ; elle reste à arbitrer.
- **A104** — **Une règle énoncée dans un fichier n'empêche pas sa violation dans le même fichier.**
  L'en-tête de `physics.rs` pose que *une référence tirée des paramètres ne prouve rien*. Deux des
  quatre cas de C10, écrits sous cet en-tête, comparent une force construite comme `ρ·g·A·d` à la
  référence `ρ·g·A` : ils ne testent que la différence finie. Ils affichent **0,000 %**, comme les
  deux autres — et c'est précisément ce qui les rend indiscernables dans le rapport. Le défaut a
  été vu à la relecture du résultat, pas à l'écriture. **Un écart nul est un signal à examiner, pas
  un résultat à encaisser** — il signifie souvent que la mesure et la référence partagent une
  ligne de code. Conséquence portée dans le module : chaque cas de C10 y est classé par degré
  d'indépendance.

**Ajoutés en S22, à la première exécution d'un solveur**

- **A105** — **Un cas canonique peut être moins discriminant que son énoncé ne le laisse croire.**
  C01 impose un fond « en pente 1:20 », donc à pente **constante**. Sur un tel fond, la hauteur
  d'eau au repos varie linéairement, le saut de hauteur aux interfaces est le même partout, et sa
  divergence est presque nulle : un schéma non équilibré y est *presque* équilibré **par accident de
  géométrie**. Mesuré : le schéma au premier jet passe `max|u|` avec 0,53 mm/s pour 1 mm/s admis. Un
  fond courbe le ferait tomber d'un ordre de grandeur. Le cas reste utile — il a bien éliminé le
  schéma, par son autre assertion — mais **il est plus faible que sa réputation**, et B3 s'apprête à
  s'en servir pour éliminer des candidats. Un C01-bis à fond courbe est proposé (ADR-030 §6.1).
  Sévérité 1 par ce que le cas commande : le protocole d'un banc.

- **A106** — **Une tolérance sans provenance traverse vingt-deux sessions sans être questionnée.**
  `max|u| < 1 mm/s` et `max|η−η₀| < 1 mm` sont dans `CAS-CANONIQUES` depuis son écriture. I-14 exige
  qu'aucun nombre ne vive sans formule ni banc ; ces deux-là n'ont ni l'une ni l'autre, et ont
  survécu à deux revues croisées, un audit des points ouverts et deux audits d'invariants. **Un
  seuil se lit comme une décision déjà prise**, alors qu'une constante physique se lit comme une
  valeur à justifier — c'est la même dette, et une seule des deux se fait attraper. S22 en donne une
  lecture physique (21,6 mm de surface = 43 cm de trait de côte sur une pente 1:20) ; une lecture
  n'est pas une justification.

- **A107** — **Une fusion faite par import de contenu ne referme pas le fork qui l'a causée.**
  S16 a fusionné les deux lignes du fork S08-S15 en **recopiant les documents** de l'une dans
  l'autre. Le contenu a bien été réuni ; l'historique git, lui, est resté divergent — et la ligne
  source, n'ayant rien reçu, a continué seule pendant **quatre sessions** (S18 à S21, dont ADR-027 à
  ADR-029 et tout le code). Le fork était donc réputé clos et ne l'était pas. **Une réconciliation
  qui ne passe pas par l'outil qui a créé la divergence ne la supprime pas, elle la masque** — et
  elle la masque d'autant mieux qu'elle produit un dépôt qui a l'air complet. `CLAUDE.md` impose
  déjà `git worktree list` et `git branch -a` à l'amorce : c'est ce qui l'a fait voir en S22, à la
  seconde commande. Le dispositif a fonctionné ; c'est la fusion qui était incomplète.

- **A108** — **Une propriété exacte vérifiée sur son intérieur seul ne dit rien de ses bords.**
  La reconstruction hydrostatique préserve le repos par identité algébrique. Le solveur qui
  l'implémentait perdait pourtant 1,1 % de son volume en 60 s, parce que la condition de mur
  recopiait la hauteur d'eau au lieu de la surface libre. **Le défaut de bord pesait quarante fois
  le défaut de schéma qu'il masquait.** Une propriété démontrée sur l'opérateur intérieur ne
  s'étend pas gratuitement aux conditions aux limites, et c'est le genre d'évidence qu'on n'écrit
  nulle part. Vaut pour l'éponge d'ADR-005, la frontière W/δ et tout raccord de domaines.

**Ajoutés en S23, à la rupture de barrage**

- **A109** — **Une explication correcte et documentée peut n'expliquer aucune part du défaut
  observé.** Le front trop lent sur lit sec a une cause classique : l'estimation des vitesses
  d'onde, qui vaut `u ± c` en régime mouillé et `u + 2c` au contact du sec. L'explication est
  juste, elle est dans la littérature, et elle décrit un mécanisme réellement présent dans le code.
  Corrigée, elle a déplacé le résultat de **0,15 point sur seize**. Le danger n'est pas de se
  tromper : c'est qu'une explication plausible **arrête la recherche**. Elle aurait été écrite dans
  un ADR comme la cause, avec une correction à l'appui, et personne n'aurait mesuré l'avant/après.
  **Réflexe** : mesurer l'effet d'une correction séparément, avant de la présenter comme la cause.

- **A110** — **Un seuil de mesure ne déplace pas seulement un verdict, il peut le renverser.** Le
  même solveur, sur la même grille, affiche **−3,46 %** au seuil `10⁻²` et **−10,83 %** au seuil
  `10⁻⁴` : le premier passe presque la tolérance de 3 %, le second échoue de trois fois. Rien dans
  l'énoncé de C04 ne dit lequel prendre. On savait qu'un seuil déplace une mesure ; on n'avait pas
  vu qu'il peut décider de l'issue. Sévérité 1 parce que C04 sert à **éliminer des candidats** : un
  banc dont le verdict dépend d'une convention non écrite élimine au hasard.

- **A111** — **Une erreur globale faible masque une erreur locale vingt fois plus grande.** L'erreur
  L1 du solveur sur C04 vaut 0,84 % sur tout le domaine ; son erreur de front vaut 16 %. Une
  validation qui n'aurait retenu que la norme globale — le réflexe naturel, et la mesure la plus
  robuste — aurait déclaré le solveur excellent. **Les mesures globales et les mesures locales ne
  se remplacent pas**, et c'est le front, pas la moyenne, qui décide de ce qu'on voit à l'écran :
  une vague qui monte sur une plage *est* un front de mouillage.

- **A112** — **Une constante sans effet mesurable a une provenance, et ce n'est pas une dette.**
  `H_SEC` a été relevé en S22 comme un nombre posé au jugé, à justifier au titre d'I-14. Balayé sur
  six décades, il déplace la grandeur la plus sensible du corpus de 0,25 point sur seize. La bonne
  réponse n'était donc pas de lui trouver une justification physique — il n'en a pas — mais de
  **mesurer ce qu'il commande**, et de constater qu'il ne commande rien. I-14 n'exige pas qu'un
  nombre soit justifié : il exige qu'on sache ce qu'il commande, et « rien » est une réponse
  valide. À distinguer de `ρ_eau` (A103), qui déplace 2,5 % de tout tirant d'eau.

**Ajoutés en S24, à la mesure de la convergence**

- **A113** — **Un ordre de convergence est une propriété du couple (solveur, cas), pas du solveur.**
  Le même solveur donne `p ≈ 0,98` sur une solution régulière, `0,73` à `0,80` sur C04, et `0,24`
  sur la position du front de C04. Aucun de ces nombres n'est « l'ordre du solveur » : ils mesurent
  trois choses différentes. L'énoncé de C08 pose une assertion absolue — `p > 0,8` — et désigne
  trois cas dont **aucun n'est régulier**, donc aucun ne permet de l'appliquer. Sévérité 1 : C08 est
  le test qui décide si un candidat « résout l'équation qu'on croit », et B3 s'apprête à s'en servir.

- **A114** — **Avec un oracle, le triplet de grilles le plus fin est le *moins* fiable.** L'inverse
  exact de ce qui vaut avec une solution analytique, où l'on prend toujours le plus fin. L'oracle
  porte sa propre erreur ; dès qu'une grille testée s'en approche, les deux se soustraient et
  l'ordre observé s'envole — mesuré : **1,56 pour un schéma d'ordre 1**. Affiner l'oracle a *dégradé*
  le résultat, ce qui est le signe le plus contre-intuitif de la session. Conséquence chiffrée : il
  faut un oracle **480 fois** plus fin que la grille la plus grossière pour cinq grilles saines,
  donc l'oracle **est** le banc. SPEC-003 §5.1 le cite comme une référence disponible.

- **A115** — **Un critère d'écart local ne distingue pas « a convergé » de « progresse lentement ».**
  Le contrôle d'asymptoticité comparait les deux derniers ordres observés et déclarait stabilisée la
  suite 0,595 → 0,686 → 0,732 : écarts petits, **mais tous de même signe**. Une suite qui monte
  régulièrement n'a pas convergé, et son dernier terme n'est pas sa limite. Vaut pour tout critère
  de stationnarité — seuils qui se stabilisent, budgets qui se tassent, mesures qui se répètent.
  Le remède est de regarder la **structure** de la suite des écarts, pas leur taille.

- **A116** — **Une erreur d'hôte absorbée publie un résultat vide qui a l'air d'un résultat.** Un
  `Err(_) => continue` sur l'allocation de l'oracle produisait la ligne « 0 grille retenue sur 0 »,
  sans mention de la cause — l'arène était pleine. Le lecteur y voit un cas qui n'a rien trouvé, pas
  un cas qui n'a pas tourné. **Un harnais doit distinguer *indisponible* de *vide***, et l'erreur
  d'hôte existe précisément pour ça : elle était disponible et jetée.

**Ajoutés en S25, à la mesure de la dissipation**

- **A117** — **Le nombre de Courant commande la dissipation, et le corpus n'en parle nulle part.**
  La loi mesurée en S25 — `demi-vie = ln2·N / (2π²(1−ν))` — fait de `1−ν` le facteur qui décide de
  tout : passer de `ν = 0,45` à `ν = 0,9` multiplie la demi-vie par **4,5**, *et* double le pas de
  temps. Moins de dissipation et moins de calcul, sur le même schéma et la même grille. Or `ν`
  n'apparaît dans aucun ADR, aucune SPEC, aucun banc : il était implicitement rangé parmi les
  réglages de stabilité. **C'est un paramètre de conception**, et il porte un arbitrage — marge de
  stabilité contre portée des ondes — que personne n'a posé. Sévérité 1 : il commande le
  dimensionnement de tout domaine δ.

- **A118** — **Un mot partagé a fait recommander l'inverse de ce qu'il fallait, trois sessions
  durant.** S22, S23 et S24 ont toutes recommandé « C03, avec la friction de fond ». C03 mesure la
  dissipation **numérique** ; la friction est de la dissipation **physique** ; le raccourci s'est
  fait tout seul, et personne — moi compris, trois fois — ne l'a rouvert. Une friction ajoutée
  aurait donné deux sources d'amortissement et une mesure ininterprétable, **sans qu'aucun test
  n'échoue** : la demi-vie aurait simplement été plus courte, et on l'aurait attribuée au schéma.
  **Le danger d'une recommandation transmise est qu'elle se recopie sans être réexaminée** — et
  qu'elle gagne en autorité à chaque recopie.

- **A119** — **Un cas passe parce que son montage n'est pas dans le régime où le système vivra.**
  C03 offre **400 points par longueur d'onde**. La demi-vie y vaut 20,7 périodes et le cas passe ;
  à 20 points par longueur d'onde — l'ordre de grandeur d'un domaine réel — elle vaut **1,3
  période**, et l'eau meurt en une oscillation. Le cas ne ment pas : il mesure ce qu'il mesure. Mais
  **son verdict dépend d'un paramètre que son énoncé ne mentionne pas**, et un lecteur en conclut
  que le solveur est bon. Même famille qu'A105 (C01 sur pente constante) : deux cas sur trois du
  corpus δ sont plus faibles que leur réputation.

- **A120** — **Une conclusion juste peut ne pas épuiser la question qu'elle ferme.** ADR-030 §5
  établissait que `λ_cut` ne sortirait pas du véhicule Saint-Venant, faute de dispersion. C'est
  exact. Mais `λ_cut` borne *« la plus petite longueur d'onde transportée correctement »*, et une
  onde peut être mal transportée de deux façons indépendantes : arriver au mauvais moment
  (dispersion) ou **ne pas arriver** (dissipation). La seconde moitié était mesurable depuis le
  début. **Une question fermée par une réponse correcte ne se rouvre plus**, ce qui rend ce type
  d'angle mort particulièrement durable.

**Ajoutés en S26, à la mise à l'épreuve de la loi de dissipation**

- **A121** — **Rien n'a vérifié qu'un spectre se comporte comme la somme de ses modes pris
  séparément.** Toutes les mesures de S25 et S26 portent sur un **mode unique**, ou sur une rampe
  dont on connaît la décomposition. Or le solveur n'est pas linéaire : les termes convectifs
  couplent les modes, et rien ne dit que la loi `n²` survive à un spectre réel. Le tableau
  d'ADR-034 §2.1, qui applique la loi composante par composante à une mer de houle, **suppose donc
  ce qui n'est pas mesuré**. Il reste utile comme ordre de grandeur ; il n'est pas une prédiction.

- **A122** — **Si δ mange les composantes courtes, personne n'a dit si la transition les
  réinjecte.** ADR-005 pose une zone de transition W→δ pensée comme un raccord *spatial* : ce qui
  entre dans δ y continue sa vie. ADR-034 montre que δ **filtre** — une onde de 3 s meurt en trois
  secondes à `dx = 1 m`. La question est donc neuve : la transition doit-elle réinjecter
  continûment depuis W ce que δ efface, ou l'effacement est-il le comportement voulu ? Les deux
  réponses sont défendables et elles n'ont pas le même coût. Sévérité 1 : elle touche la couture
  entre deux couches, c'est-à-dire l'endroit où une erreur est la plus chère à défaire.

- **A123** — **Le budget de résolution se dimensionne sur la composante la plus courte à conserver,
  pas sur la dominante.** C'est la grandeur dominante qui vient à l'esprit — une mer de `Tp = 8 s`
  « fait » 100 m de longueur d'onde. Mais son **aspect** vit dans le clapot de 2 à 4 s. À
  `dx = 1 m`, la houle dominante survit 6,4 périodes et le clapot **moins d'une seconde**.
  Conserver ce dernier sur 5 périodes demande `dx = 0,18 m`, soit **trente fois plus de cellules**.
  L'arbitrage n'existait pas avant d'être chiffré, et il porte un facteur trente.

- **A124** — **Un cas qui vit dans le code et pas dans le corpus est introuvable pour la session
  suivante.** Le montage régulier a été écrit en S24 dans `Bassin::c08_regulier`, exécuté, et
  mentionné dans ADR-032 — mais absent de `CAS-CANONIQUES`. Une session qui aurait lu le corpus de
  validation sans lire cet ADR aurait conclu qu'aucun cas régulier n'existait, et l'aurait réécrit.
  **Le code n'est pas un lieu de publication** : il est lu par qui travaille dessus, pas par qui
  cherche ce qui existe. Formalisé en **C22** (S26).

**Ajoutés en S27, en posant le nombre de Courant**

- **A125** — **`u_max` de la CFL n'a jamais été défini, alors qu'une SPEC impose des parois
  mobiles.** SPEC-001 §2.1 écrit `dt ≤ C·dx/u_max` depuis S02 sans qualifier `u_max` ;
  SPEC-004 §10.1 pose comme exigence *non négociable* d'accepter « une frontière en mouvement avec
  sa vitesse ». Les deux se contredisent **en silence**. Le défaut correspondant a été mesuré sur un
  autre projet : eau au repos, solide mobile, **borne nulle** pendant que le Courant réel valait
  0,943 — rapport 2,2 à 2,5, **zéro violation déclarée**. Un solveur peut violer sa condition de
  stabilité d'un facteur deux en restant vert. **La revue croisée S08 avait examiné cette paire**
  (E08) : son rapprochement portait sur le coût, et une variable non définie ne déclenche aucune
  contradiction visible. Sévérité 1.

- **A126** — **Une marge par défaut peut protéger d'un défaut que personne n'a identifié.**
  `CFL = 0,45` a été posé sans justification écrite. La mesure montre que le schéma est stable
  jusqu'à 0,99 et juste — l'erreur de période reste soixante fois sous la tolérance. **Rien ne
  s'opposait donc à monter la valeur**, sauf ceci : la marge de Courant est une marge sur `u_max`,
  et `1/0,45 = 2,22` couvre presque exactement le facteur du défaut d'A125. La valeur protégeait
  d'un trou qu'elle ne connaissait pas. **Une marge dont on ignore la fonction se retire sans
  bruit**, et le coût n'apparaît que plus tard. Réflexe : avant de resserrer un paramètre de
  sécurité au motif que « rien n'échoue », chercher ce contre quoi il protégeait.

- **A127** — **Une loi mesurée sur un régime est publiée sans son domaine de validité.** La loi de
  dissipation d'ADR-033 §2.2 tient à `a/h = 1 %` et **est fausse à 5 %** — la pente `k` s'y effondre
  de 39 % avec `N`. Le balayage d'origine était à amplitude relative fixe : il n'était pas confondu,
  et sa conclusion tient. Mais **la condition n'était écrite nulle part**, et `a/h = 5 %` est
  ordinaire en eau peu profonde. Une loi sans domaine déclaré est appliquée partout par défaut.

- **A128** — **Une source extérieure a rendu en une heure ce que sept sessions n'avaient pas vu.**
  Le défaut d'A125 a été trouvé parce que l'utilisateur a fourni un projet voisin, écrit ailleurs,
  qui l'avait **mesuré**. Le corpus contenait pourtant les deux moitiés — SPEC-001 §2.1 et
  SPEC-004 §10.1 — depuis S04. Ce qui manquait n'était pas l'information mais **la question**, et
  une architecture différente la pose autrement. Sévérité 3 parce qu'il n'y a pas de correction à
  appliquer, mais la conséquence de méthode est réelle : **un corpus fermé sur lui-même ne produit
  que les questions qu'il sait déjà poser.**

**Ajoutés en S28, en exerçant la définition d'`u_max`**

- **A129** — **Un cas qui exigerait une explosion conclurait que le défaut n'existe pas.** C23
  mesure une borne de pas de temps fausse : sous la définition absolue, `u_max` est sous-estimé
  jusqu'à **×5,5** et le Courant réalisé atteint **2,48**. **Et le solveur ne diverge pas** — Rusanov
  reste diffusif et absorbe le dépassement. Un cas dont l'assertion aurait été « le solveur casse »
  serait donc **passé**, et aurait certifié l'absence d'un défaut présent. Ce qui est perdu au-delà
  de la condition de stabilité n'est pas la simulation, c'est la **garantie** : le solveur tient
  jusqu'à ce qu'il ne tienne plus, sur un cas que rien n'a testé. Sévérité 1 — la forme d'assertion
  décide de ce que le cas peut voir, et « ça marche encore » est la plus trompeuse de toutes.

- **A130** — **Le gain de portée et la fragilité à une définition fausse croissent ensemble.** Sous
  une définition d'`u_max` qui ignore les parois, la vitesse de paroi qui fait franchir `C = 1` vaut
  5,41 m/s à `ν = 0,45`, **1,90 m/s à `ν = 0,70`**, et 0,49 m/s à `ν = 0,90`. Serrer le pas de temps
  pour gagner en portée d'onde **rapproche du trou** au lieu de s'en éloigner. Les deux effets vont
  dans le même sens et se renforcent : c'est la configuration où une optimisation légitime rend un
  défaut latent soudainement atteignable, sans qu'aucune des deux décisions ne paraisse risquée
  isolément.

- **A131** — **Un objet rapide dans l'eau divise le pas de temps par trois, et rien ne le
  budgétait.** Avec la définition correcte, `u_max = u_p + c` : à `u_p = 10 m/s` et `h = 2 m`, le pas
  est divisé par **3,26**. C'est le régime de **C20**, l'impact d'entrée dans l'eau — un cas que le
  corpus décrit depuis S12 sans jamais mentionner son coût temporel. Le budget d'un domaine δ n'est
  donc pas une constante : il dépend de ce qui **tombe dedans**, et un ordonnanceur qui ne le sait
  pas se fera surprendre au moment le plus visible du jeu.

