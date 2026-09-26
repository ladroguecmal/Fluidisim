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
(dossier de réunion), 1 en S18 (arbitrages), 1 en S19 (nature du projet), 3 en S20 (première ligne de code), 4 en S21 (cas analytiques), 4 en S22 (C01 et le premier δ), 4 en S23 (C04 et le lit sec), 4 en S24 (C08 et l'oracle), 4 en S25 (C03 et la dissipation), 4 en S26 (les harmoniques), 4 en S27 (le nombre de Courant), 3 en S28 (la paroi mobile), 3 en S29 (l'audit des assertions), 3 en S30 (la réécriture), 3 en S31 (la dissolution d'A122), 3 en S32 (celle d'A139), 2 en S33 (le train entretenu), 3 en S34 (l'audit des garde-fous) — **148 au
total**. *Ce décompte cumulé s'arrête à S34 et n'est plus tenu ; le registre compte **195
entrées au 2026-09-09** (S118). Un état sans date se lit au présent — A185.* **Quatre-vingt-huit ont été trouvés dans nos propres écrits**, pas dans les documents sources :
A49, A56, A57, A58, puis A65 à A148. La proportion
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
| **A103** *(clos S58)* | Le corpus n'a jamais fixé la masse volumique de l'eau, dont dépendent des références | 2 | `body.rs`, C10 → **ADR-048** |
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
| **A132** | Une assertion peut être satisfaite parce que le mécanisme testé est absent | **1** | AUDIT-ASSERTIONS-S29 §1 |
| **A133** | Un montage peut être incapable d'atteindre le régime où l'assertion échoue | **1** | AUDIT-ASSERTIONS-S29 §5 |
| **A134** | Une mesure fautive peut porter une conclusion sans que la conclusion soit fausse | 2 | AUDIT-ASSERTIONS-S29 §4 |
| **A135** | L'instrument qui mesure une assertion ne se lit pas dans son énoncé | **1** | AUDIT-ASSERTIONS-S29 §5 bis, C18 |
| **A136** | Une assertion floue peut être plus faible que ce que la conception garantit déjà | 2 | C11, ADR-008 §3 |
| **A137** | Un montage du corpus saute le régime que son assertion vise | **1** | C07, ADR-011 §4 |
| **A138** | Un solveur ne conserve pas la forme d'un paquet que son équation conserve exactement | **1** | ADR-036 §5 |
| **A139** | Le sillage est peut-être un objet de W et non de δ, et rien ne le dit | **1** | ADR-036 §6.2, ADR-011 §4 |
| **A140** | Une question ouverte peut rester lourde des sessions durant sans que sa prémisse soit relue | 2 | ADR-036 §1, A122 |
| **A141** | La durée « attendue » d'un phénomène dépend de ce qu'on regarde, et rien ne le fixe | 2 | ADR-037 §4.1 |
| **A142** | L'équilibre spatial d'un phénomène entretenu est dérivé, jamais mesuré | 2 | ADR-037 §4.3 |
| **A143** | Le même document fondateur a répondu deux fois de suite à une question dite ouverte | **1** | ADR-037 §1, A122 et A139 |
| **A144** | Un garde-fou peut porter sur la bonne idée et la mauvaise condition | **1** | S33, contrôle de réflexion |
| **A145** | Une mesure de qualité d'ajustement rattrape des défauts qu'elle n'était pas censée voir | 3 | S33, le `R²` à 0,487 |
| **A146** | Les saturations de modèle ne sont comptées nulle part, et une saturation fréquente est un défaut | 2 | AUDIT-GARDE-FOUS-S34 §5.1 |
| **A147** | Un garde-fou non testable isolément est celui qui défaille | **1** | AUDIT-GARDE-FOUS-S34 §3.2 |
| **A148** | Aucun garde-fou ne compte ses déclenchements : on ignore lesquels travaillent | 2 | AUDIT-GARDE-FOUS-S34 §5.2 |
| **A149** | Un seuil absolu affiché en pourcentage se lit comme un écart relatif | 3 | `main.rs`, C01 |
| **A150** | La position d'un front numérique dépend du seuil qui la définit | 2 | ADR-038 §5, C04 |
| **A151** | Le corpus ne nomme nulle part le modèle dont C01 est le test canonique | 3 | ADR-038 §6 |
| **A152** | Un paramètre qu'un énoncé ne fixe pas est tranché en silence par le premier qui mesure | **1** | ADR-039 §1 |
| **A153** | L'ordre d'un schéma est un couple (schéma, solution), pas un nombre | 2 | ADR-039 §4.1 |
| **A154** | Un cas partiel qui s'affiche vert ne se distingue pas d'un cas complet | 2 | ADR-039 §5 |
| **A155** | Une simplification algébrique efface le domaine où elle est valide | **1** | ADR-040 §2 |
| **A156** | Un cas qui réduit une solution à un scalaire peut classer deux candidats à l'envers | **1** | ADR-040 §8 |
| **A157** | Un seuil choisi pour la reproductibilité seule peut être reproductible et dénué de sens | **1** | ADR-041 §4 |
| **A158** | Le coût d'un instrument de mesure croît sans que personne le regarde | 3 | ADR-041 §5 |
| **A159** | Une formule énoncée avec ses constantes n'invite pas à être recalculée | **1** | ADR-042 §2 |
| **A160** | Un coefficient d'amortissement n'a de sens que tant que `σ·dt < 1` | 2 | ADR-042 §4.1 |
| **A161** | Trois fonctions distinctes partagent le mot « éponge », et un résultat sur l'une se lit comme un résultat sur les autres | 2 | ADR-043 §5 |
| **A162** | Corriger la référence d'une mesure périme en silence tous les chiffres qui en dérivent | 2 | ADR-040, note S36 |
| **A163** | Deux implémentations du même modèle portent deux seuils de sec incompatibles, `10⁻⁶` et `10⁻¹⁰` | **1** | ADR-044 §5 |
| **A164** | La précision arithmétique des véhicules n'est écrite nulle part : `f32` d'un côté, `f64` de l'autre | 2 | ADR-044 §2 |
| **A165** | Un seuil de sec coupe la vitesse mais pas le flux de masse : le film derrière le front n'est jamais vide | 2 | ADR-045 §6 |
| **A166** | Une mesure peut être juste et sans portée, et rien dans la mesure ne le dit | **1** | ADR-046 §4.1 |
| **A167** | Un défaut de montage peut ne pas se voir dans le résultat qu'il menace | **1** | ADR-046 §3 |
| **A168** | Écrire une distinction ne suffit pas à s'en servir : `ADR-043` §6 ignore le §5 du même document | 2 | ADR-043, note S39 |
| **A169** | Une prudence justifiée par une mauvaise raison se défend mal et se reporte longtemps | 2 | ADR-047 §6 |
| **A170** | Une saturation d'affichage transforme un refus en la meilleure mesure possible | **1** | S42, `c03_seiche` |
| **A171** | Une valeur de repli qui coïncide avec la valeur nominale est invisible à tout contrôle | **1** | S43, `ordre_grossier_estime` |
| **A172** | Quand la grandeur est un écart, zéro est son meilleur point — aucun repli n'y est acceptable | **1** | AUDIT-REPLIS-S44 §4 |
| **A173** | Un refus peut supprimer son assertion ou être ignoré par une sélection — corrigé S45 | **1** | AUDIT-REPLIS-S44 §7 |
| **A174** | Filtrer les refus change la famille mesurée ; omettre les indéterminés fausse le bilan — corrigé S46 | **1** | AUDIT-REPLIS-S44 §8 |
| **A175** | Un chiffre reproductible peut porter un verdict hors contrat — corrigé S47 | **1** | AUDIT-REPLIS-S44 §9 |

Cent huit angles morts recensés, tous traités ou explicitement cadrés. Aucun n'est laissé sans
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

  > **Note corrective — 2026-09-07, S58. Clos par ADR-048, et l'énoncé ci-dessus était faux sur
  > un point.** *« La référence de C10 ne se referme qu'avec 1000 »* : les nombres cités sont
  > exacts, la conséquence ne l'est pas. Les trois références du cas sont construites **avec** la
  > constante, donc C10 passe à écart 0,000 % pour toute valeur — mesuré à 1025 dans RHO-EAU-S58.
  > **Ce n'est pas C10 qui contraignait la valeur, mais deux assertions de test unitaire** ; voir
  > **A180**. La décision : **1025**, et une propriété du milieu plutôt qu'une constante globale.
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

- **A122** — ~~**Si δ mange les composantes courtes, personne n'a dit si la transition les
  réinjecte.**~~ **DISSOUS en S31** — voir [`ADR-036`](../adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) §1 : δ est additif, il ne porte pas la houle mais l'écart, et il n'y a rien à réinjecter. *Énoncé d'origine conservé ci-dessous.* ADR-005 pose une zone de transition W→δ pensée comme un raccord *spatial* : ce qui
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

**Ajoutés en S29, à l'audit des assertions**

- **A132** — **Une assertion peut être satisfaite parce que le mécanisme testé est absent.** C'est
  la **vacuité**, et elle est distincte du symptôme d'A129 : l'assertion peut porter sur une
  grandeur parfaitement mesurable et rester verte parce que rien ne la produit. « Aucune plaque de
  glace ne se forme tant que `Hs > 0,15 m` » (C15) passe avant que le modèle de glace existe ;
  « zéro allocation » passe si la boucle ne tourne pas ; « l'hôte serveur compile et tourne » (C18)
  passe tant qu'il n'y a pas d'hôte serveur. **Ces assertions sont vertes depuis leur écriture et le
  resteront jusqu'au jour où elles devraient enfin dire quelque chose.** Le remède est le **témoin**
  — un montage qui doit faire échouer l'assertion — et il existait déjà ici sans être nommé
  (`C01-jet`). Sévérité 1 : une batterie verte dont une part est vide donne exactement la confiance
  qu'elle ne mérite pas.

- **A133** — **Un montage peut être incapable d'atteindre le régime où l'assertion échoue.** Trouvé
  en se trompant : le balayage d'amplification demandait `ν = 1,05` et recevait **silencieusement**
  `0,99`, `avec_cfl` bornant à `[0,05 ; 0,99]`. La mesure a rendu le résultat de 0,99 comme s'il
  était celui de 1,05. Ce n'est pas une assertion qui ne peut pas échouer — c'est un **réglage qui
  ne peut pas atteindre le régime testé**, et l'effet est le même : une assertion recevable retombe
  en vacuité. **Une garde de sécurité posée sur un instrument de mesure l'empêche de mesurer**, et
  c'est le genre de bornage qu'on écrit par prudence sans voir qu'il aveugle. Sévérité 1.

- **A134** — **Une mesure fautive peut porter une conclusion sans que la conclusion soit fausse.**
  `stabilite_par_courant` était de classe B et n'a rien prouvé ; ADR-035 §4 en a pourtant tiré une
  ligne. **La conclusion de cet ADR tient quand même**, parce qu'une seconde mesure — l'erreur de
  période, continue — la portait réellement. La ligne de stabilité était décorative.
  **C'est la configuration la plus difficile à détecter** : rien n'est faux, donc rien n'alerte, et
  la mesure vide reste dans le document où une session ultérieure la citera comme un fait établi.
  Réflexe : pour chaque conclusion, identifier **laquelle** des mesures la porte, et vérifier que
  celle-là est de classe A.

**Ajoutés en S30, à la réécriture des assertions**

- **A135** — **L'instrument qui mesure une assertion ne se lit pas dans son énoncé.** C18 porte deux
  assertions négatives de forme identique : « zéro allocation après initialisation » et « aucune
  capacité dérivée n'est lue depuis un profil de qualité ». L'audit S29 a classé la première
  recevable et **manqué la seconde**, par ressemblance. Ce qui les sépare n'est pas la formulation
  mais l'existence d'un **instrument** : la première a son compteur, lu par le harnais depuis S20 ;
  la seconde demanderait une analyse statique qui n'existe pas, et elle est donc **vide**.
  **Conséquence de méthode** : classer une assertion sur son énoncé seul ne suffit pas ; il faut,
  pour chacune, **nommer ce qui la mesure** et vérifier que cela existe. C'est un troisième contrôle,
  après la grandeur et le témoin. Sévérité 1 — un audit qui se trompe déclare sain ce qui ne l'est
  pas, et il le fait avec autorité.

- **A136** — **Une assertion floue peut être plus faible que ce que la conception garantit déjà.**
  « Aucun tremblement **visible** en mode contraint » (C11) tolère tout écart sous le seuil de
  perception. Or ADR-008 §3 pose qu'en mode contraint l'objet est **projeté sur la surface**,
  `z = η`, et qualifie le résultat d'« exactement stable » : la référence est **zéro**, pas un seuil.
  L'énoncé demandait donc **moins** que ce que la conception promet, et un écart de `10⁻⁴ m` —
  invisible, mais signalant que la projection n'est pas appliquée — l'aurait passé. **Le flou n'est
  pas seulement imprécis : il déplace l'exigence vers le bas**, et toujours dans ce sens, parce
  qu'un seuil perceptuel est plus permissif qu'une identité.

- **A137** — **Un montage du corpus saute le régime que son assertion vise.** C07 assertit sur
  `Fr_h ≈ 1` ; ses quatre vitesses donnent `0,71 · 1,14 · 1,43 · 2,14` par 5 m de fond. **Le point
  critique n'est jamais atteint.** C'est A133 — trouvé en S29 dans le harnais — cette fois dans le
  corpus, et il y était depuis l'écriture du cas. Il n'a été visible qu'**après** avoir donné une
  grandeur à l'assertion : tant qu'elle disait « nettement supérieure », rien n'obligeait à vérifier
  que le montage produisait le régime. **Une assertion sans grandeur masque un montage incapable**,
  et les deux défauts se protègent l'un l'autre.

**Ajoutés en S31, à la dissolution d'A122**

- **A138** — **Un solveur ne conserve pas la forme d'un paquet que son équation conserve
  exactement.** Saint-Venant est **non dispersif** : une perturbation initiale s'y scinde en deux
  trains qui se propagent à `±c` **sans déformation**. Mesuré sur le véhicule, un paquet gaussien de
  `σ = 1 m` **quintuple sa largeur** en 20 s et perd 90 % de son amplitude ; à `σ = 4 m`, +74 % et
  −68 %. **Tout cet étalement est un artefact numérique**, et rien n'en mesurait la quantité — la
  forme se dégrade avant l'amplitude, ce qui est la signature d'un filtre passe-bas appliqué à un
  spectre. Un cas canonique manque : **C24, conservation de forme d'un paquet**, dont la référence
  est l'invariance, donc `1` exactement, sans aucun seuil à inventer. Sévérité 1 — un sillage, un
  remous et une éclaboussure sont tous des paquets, et c'est ce que δ existe pour produire.

- **A139** — ~~**Le sillage est peut-être un objet de W et non de δ, et rien ne le dit.**~~ **DISSOUS en S32** — ADR-001 §2 range les sillages dans **W** depuis S01, en toutes lettres. La mention « rien ne le dit » était fausse : je n'avais lu qu'ADR-011 §4. Voir [`ADR-037`](../adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) §1. *Énoncé d'origine conservé ci-dessous.* ADR-036 §3
  chiffre la mort d'un sillage porté par δ : deux mètres derrière une barque à 3 m/s, la distance
  visible variant comme `v⁴/dx`. Mais **ADR-011 §4 place le générateur de sillage dans la couche
  W** — *« le générateur de sillage de la couche W doit prendre `h` en entrée »*. Si le sillage est
  un objet de W, il n'est pas discrétisé, il ne se dissipe pas, et tout le §3 tombe. **Aucun
  document ne tranche**, et les deux lectures sont défendables : un sillage est une onde
  (donc W) mais il est créé par un objet local en mouvement (donc δ). Sévérité 1 : la réponse
  décide si le résultat le plus visible de S31 est un problème majeur ou sans objet.

- **A140** — **Une question ouverte peut rester lourde des sessions durant sans que sa prémisse soit
  relue.** A122 a été qualifiée de « question la plus lourde ouverte » en S26, S28, S29 et S30 —
  quatre sessions, quatre recommandations. Elle s'est dissoute en une lecture d'ADR-001 §2 et
  d'ADR-005 §1, deux documents antérieurs à sa formulation. **Ce qui manquait n'était pas un
  travail, c'était une relecture** ; et le statut « ouverte, lourde » l'a rendue chaque fois plus
  intimidante, donc chaque fois plus reportée. Réflexe : avant d'ouvrir une question difficile,
  relire ce que le corpus dit déjà de sa **prémisse** — c'est le geste le moins cher et il n'était
  fait par aucune des quatre sessions qui l'avaient recommandée.

**Ajoutés en S32, à la dissolution d'A139**

- **A141** — **La durée « attendue » d'un phénomène dépend de ce qu'on regarde, et rien ne le
  fixe.** Le critère de dimensionnement d'ADR-037 §3 compare la durée numérique d'une éclaboussure à
  sa durée physique, estimée par le temps de chute gravitaire `√(2L/g)`. Mais **la durée *perçue*
  d'une éclaboussure inclut l'écume et le spray**, qui relèvent d'ADR-014 et survivent bien plus
  longtemps que la déformation de surface. Selon ce qu'on décide de voir, le critère est
  conservateur ou optimiste — et l'écart est d'un facteur qui n'a pas été estimé. **Une durée
  attendue n'est pas une propriété du phénomène, c'est une propriété de ce qu'on en montre.**

- **A142** — ~~**L'équilibre spatial d'un phénomène entretenu est dérivé, jamais mesuré.**~~ **LEVÉ en S33**, dans le régime linéaire : `L½ = K·λ²/dx` vérifiée sur quatre longueurs d'onde, écart de −0,3 % au meilleur point, `R²` jusqu'à **1,0000**. Le régime non linéaire reste non mesuré. Voir [`ADR-037`](../adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) §2.1, note S33. *Énoncé d'origine conservé ci-dessous.* ADR-037
  §2.1 conclut que le proche-coque s'éteint naturellement à 25,6 m, dans la portée qu'ADR-001 donne
  à un domaine. Le raisonnement suppose qu'une source constante et une dissipation exponentielle
  produisent une décroissance en `exp(−x/L_d)` — vrai en régime linéaire, **non vérifié ici**, et le
  solveur n'est pas linéaire. Toutes les mesures de S25 à S32 portent sur des perturbations
  **relâchées**, aucune sur une source **entretenue**. La conclusion la plus rassurante du corpus
  récent est donc la moins étayée.

- **A143** — **Le même document fondateur a répondu deux fois de suite à une question dite
  ouverte.** A122 s'est dissoute en S31 en relisant ADR-001 ; A139 s'est dissoute en S32 en relisant
  ADR-001. Dans les deux cas la réponse y était depuis S01, et dans le second c'est la session
  précédente qui avait posé la question sans consulter le document. **Un corpus qui grandit rend son
  propre socle moins consulté** : trente-six ADR se lisent moins qu'un, et le premier est celui
  qu'on croit connaître. Sévérité 1 par ce que cela coûte — deux angles morts de gravité 1 ouverts,
  quatre sessions de report, un ADR entier (036 §3) écrit sur un objet qui n'existait pas.
  **Réflexe : toute question sur *l'appartenance d'un phénomène à une couche* se règle dans ADR-001
  §2, et nulle part ailleurs.**

**Ajoutés en S33, à la mesure du train entretenu**

- **A144** — **Un garde-fou peut porter sur la bonne idée et la mauvaise condition.** La mesure de
  décroissance spatiale exigeait qu'aucune réflexion ne pollue la fenêtre, et le contrôle écrit pour
  cela vérifiait qu'on mesurait **derrière le front** — pas que le front **n'avait jamais atteint le
  mur**. À `λ = 20 m` sur 200 m, le front était à 280 m : l'onde était revenue, et le contrôle
  déclarait la fenêtre saine. **C'est A133 commis dans la fonction écrite pour l'éviter**, et dans
  la session qui l'invoquait au plan. L'idée était juste, la condition ne l'était pas — et une
  condition fausse est invisible tant qu'elle n'est pas franchie. Sévérité 1 : un garde-fou en qui
  l'on a confiance est plus dangereux qu'aucun garde-fou.

- **A145** — **Une mesure de qualité d'ajustement rattrape des défauts qu'elle n'était pas censée
  voir.** Le `R²` de l'ajustement exponentiel est tombé à **0,487** sur le cas pollué, là où les cas
  sains donnaient 0,999 à 1,000. Il n'était pas là pour détecter une réflexion : il était là pour
  dire si la décroissance est bien exponentielle. **Il a signalé que la forme supposée était fausse,
  sans avoir à connaître la raison** — ce qu'aucun garde-fou spécifique ne sait faire.
  **Réflexe** : accompagner toute régression d'un `R²` publié, même quand la forme n'est pas en
  doute. C'est une sonde générique, et elle coûte trois lignes.

**Ajoutés en S34, à l'audit des garde-fous**

- **A146** — **Les saturations de modèle ne sont comptées nulle part, et une saturation fréquente
  est un défaut.** `delta.rs` en contient une douzaine — `h.max(0.0)` après un pas, la
  reconstruction hydrostatique, l'état initial. Elles relèvent d'une autre famille que les contrôles
  de validité de montage : ce sont des **rattrapages de physique**, qui empêchent une hauteur d'eau
  négative de se propager. **Aucune n'est comptée.** Or une saturation qui se déclenche rarement est
  un filet ; une saturation qui se déclenche à chaque pas est un solveur qui produit des états
  impossibles et qu'on maquille. **Les deux sont indiscernables aujourd'hui**, et le second cas est
  exactement ce que C04 pourrait cacher sur son lit sec.

  > **Note S38 — mesuré, et requalifié.** L'inquiétude est levée **dans la forme où elle est
  > posée ci-dessus** : en régime nominal, la saturation d'état ne se déclenche **jamais** — zéro
  > sur C01, C03 et C04, sur les deux véhicules, y compris sur le lit sec nommément suspecté. Le
  > flux de Rusanov préserve la positivité sous sa condition de Courant. **Il n'y a pas de
  > maquillage.**
  >
  > Mais le danger existe, ailleurs : la saturation **mord** au-delà de la condition de Courant —
  > la frontière mesurée tombe exactement dessus — et là elle ne rattrape rien. Elle transforme une
  > divergence franche, qui aurait produit des `NaN` visibles, en une suite de nombres finis. **Ce
  > n'est pas un filet, c'est un détecteur de divergence, et il était muet.** Voir
  > [`ADR-045`](../adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) et
  > [`AUDIT-SATURATIONS-S38`](AUDIT-SATURATIONS-S38.md).

- **A147** — **Un garde-fou non testable isolément est celui qui défaille.** Sur dix garde-fous
  audités, un seul masquait au lieu de refuser — et c'était **le seul qui n'était pas appelable
  seul** : il vivait en ligne dans une fonction lançant des simulations avec un oracle à 51 200
  cellules. La chaîne est mécanique : *emplacement en ligne → non testable isolément → jamais testé
  → jamais vu refuser → défaut invisible*. Les neuf autres, appelables directement, avaient été
  éprouvés au fil des sessions sans que ce soit délibéré. Sévérité 1 : la non-testabilité **prédit**
  la défaillance, ce qui en fait un critère de revue plus efficace que la relecture du code — G10 a
  survécu à plusieurs relectures parce qu'il *a l'air correct*.

- **A148** — **Aucun garde-fou ne compte ses déclenchements.** L'audit S34 établit que les dix
  *peuvent* refuser ; il ne dit pas s'ils refusent **en usage réel**, ni à quelle fréquence. Un
  garde-fou qui ne se déclenche jamais en production est soit inutile, soit mal conditionné — et
  rien ne permet de distinguer les deux. Un compteur par garde-fou coûterait quelques octets et
  dirait lesquels travaillent. C'est le même manque qu'A146 sur les saturations, et il a la même
  cause : **on écrit un garde-fou pour empêcher, jamais pour mesurer**.

- **A161** — **Trois fonctions distinctes partagent le mot « éponge ».** La même bande de bord
  d'un domaine δ doit **absorber** (ne pas réfléchir, ADR-005 §2), **faire décroître** (ne pas laisser
  δ persister jusqu'au bord, ADR-001) et **transduire** (rendre l'énergie à `W` au lieu de la
  détruire, ADR-005 §3). Leurs critères de dimensionnement n'ont rien en commun — un taux de
  réflexion, une longueur de décroissance, un bilan d'énergie — et **une seule des trois a été
  mesurée**. Le risque est concret et a failli se produire à la fusion : ADR-037 conclut que la
  dissipation rend le **masque** inutile pour les phénomènes entretenus, ce qui se lirait sans
  effort comme « un domaine peut se passer d'**absorbeur** » — conclusion fausse que rien dans le
  vocabulaire n'empêche. ADR-043 D2 impose de nommer les fonctions séparément.

- **A162** — **Corriger la référence d'une mesure périme en silence tous les chiffres qui en
  dérivent.** En B-S25, la référence de l'erreur `L¹` de Ritter est passée du point à la moyenne de
  cellule — une correction juste, faite pour C04. Elle alimentait aussi le `p` de C08, publié une
  session plus tôt à **1,003** ; il vaut désormais **0,9997**, et rien ne l'a signalé. **Le cas
  continuait de passer**, ce qui est exactement ce qui rend l'écart invisible : une assertion à
  minorant (`p > 0,8`) ne bronche pas pour trois millièmes. Le défaut est de la même famille que
  les décomptes recopiés de S07 et S10, sur un objet qu'on croyait à l'abri : **une mesure**.
  *Toute correction d'une référence, d'un seuil ou d'une norme oblige à rejouer les chiffres
  publiés qui en dépendent, ou à les marquer périmés le jour même.*

- **A163** *(sévérité 1)* — **Deux seuils de sec incompatibles, et un seul a une provenance.**
  `delta.rs` déclare une cellule sèche sous `H_SEC = 10⁻⁶ m`, dont `ADR-031` §5 donne l'origine ;
  `shallow.rs` sous `10⁻¹⁰ m`, écrit **en dur** dans deux endroits, sans justification. **Quatre
  ordres de grandeur.** Une cellule entre les deux est sèche pour l'un et mouillée pour l'autre, et
  `hu/h` sur un film pareil rend n'importe quoi : la confrontation de S37 mesure **6,16 m/s**
  d'écart sur une cellule, soit **98 % de la vitesse du front de Ritter**. Sévérité 1 parce que ce
  seuil **déplace la position du front**, donc le verdict de C04, donc le critère d'entrée au banc
  B3. Les deux lignées avaient identifié la question — `ADR-031`, et **A150** — et y ont répondu
  différemment sans le savoir. **Ne pas aligner les deux valeurs par une retouche de constante** :
  c'est une décision de conception (ADR-044 §7).

  > **Note S40 — requalifié : l'incompatibilité n'existe pas, l'ignorance existait.** Le seuil a
  > été balayé sur **sept décades** et sur les **deux** véhicules : le front bouge de **0,148 %** au
  > pire pour une tolérance de 3 %, `h(0)` de deux pour cent mille, le volume pas du tout. **Aucune
  > grandeur publiée ne dépend de ce seuil.** Le désaccord de 6,16 m/s trouvé en S37 portait sur
  > `max|u|`, qui dépasse la vitesse du front de Ritter et varie d'un facteur 2,5 sans tendance :
  > ce n'est pas une grandeur (**A166**).
  >
  > **Les deux valeurs ne sont pas alignées** — une différence sans conséquence se documente au lieu
  > de se corriger, et aligner coûterait la reproductibilité des chiffres de la lignée B. Ce qui
  > justifiait la sévérité 1 demeure et est levé : **une des deux valeurs n'avait aucune
  > provenance**, écrite en dur à huit endroits. Elle en a une maintenant, et elle est réglable.
  > Voir [`ADR-047`](../adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md).

- **A164** — **La précision arithmétique n'est écrite nulle part.** `delta.rs` calcule en `f32`,
  `shallow.rs` en `f64`, et aucun document du corpus ne le mentionnait avant S37 — ni SPEC-001, ni
  les ADR de solveur, ni `CAS-CANONIQUES`. Ce n'est pas un détail d'implémentation : c'est ce qui
  décide de ce qu'une assertion peut affirmer. En `f32`, « bien équilibré » vaut **4,4 µm/s après
  une minute**, pas l'arrondi machine — et l'erreur **croît** avec le temps simulé. Le seuil de C01
  étant à 1 mm/s, la marge est de **×227**, ce qui est confortable et n'était pas connu. Pour un
  jeu de très grande échelle, `f32` sera vraisemblablement imposé par la mémoire : **c'est ce
  chiffre-là le plancher réel de la couche `δ`**.

- **A165** — **Un seuil de sec coupe la vitesse, pas le flux de masse.** `u = 0` sous `h_sec`
  empêche la cellule de **bouger** ; il n'empêche pas la diffusion de Rusanov, `α·(h_R − h_L)`, d'y
  **déposer** de la matière — même quand les deux vitesses sont nulles. Derrière le front de C04,
  `delta.rs` porte **3** cellules à `0 < h < h_sec` et `shallow.rs` **18**, dans le rapport qu'on
  attend de seuils séparés par quatre ordres de grandeur. Conséquence : *un seuil de sec n'assèche
  pas une cellule*, et toute mesure qui suppose un domaine proprement partitionné en sec et mouillé
  — une position de front, un volume de zone humide, un bilan par région — travaille sur une
  frontière floue dont la largeur dépend du seuil. Recoupe **A163**.

---

### Angles morts importés de la lignée B (2026-09-06, S35)

> Les douze suivants ont été ouverts dans une histoire parallèle du dépôt (sessions **B-S22**
> à **B-S26**), où ils portaient les numéros `A105` à `A116` — déjà pris ici. Carte de
> renumérotation : [`FORK-S22-S26`](FORK-S22-S26.md). **Cinq sont de sévérité 1**, et aucun n'a
> été relu par cette lignée : ils sont reportés tels quels, sévérités comprises.

- **A149** — **Un seuil absolu affiché en pourcentage se lit comme un écart relatif.** Quand la
  référence d'un cas est **zéro** — `u ≡ 0` pour C01 — l'écart relatif n'a pas de sens et le
  rapport affichait la mesure elle-même suivie d'un signe `%`. Un seuil de **1 mm** y apparaissait
  comme « 0,1 % », et une mesure de 19,5 mm/s comme « 1,951 % » : deux nombres qui ont l'air
  rassurants et qui décrivent un échec d'un facteur vingt. Le rapport distingue désormais les deux
  formats. **Une unité qui disparaît de l'affichage ne disparaît pas de la grandeur** — elle
  disparaît seulement de ce que le lecteur peut vérifier.
- **A150** — **La position d'un front numérique dépend du seuil qui la définit.** Un front n'a pas
  de bord net : la hauteur décroît continûment vers zéro. Mesurée à 1 mm, la position du front de
  Ritter est 10,84 m ; à 1 µm, 11,39 m — **5 % d'écart pour trois ordres de grandeur de seuil**,
  quand la tolérance du cas est de 3 %. Le seuil n'est donc pas un détail d'implémentation : il
  fait partie de la définition de la grandeur, et un cas qui ne le fixe pas laisse **le candidat
  choisir sa propre note**. La batterie en mesure deux et rapporte les deux.
- **A151** — **Le corpus ne nomme nulle part le modèle dont C01 est le test canonique.** Ni
  Saint-Venant, ni « shallow water » : six spécifications, vingt-neuf ADR — tout le corpus
  antérieur à ADR-038 — et la seule occurrence
  d'« eau peu profonde » sert à un critère de déferlement. Le projet a hérité du test sans hériter
  du cadre qui lui donne son sens. Le test reste valide — l'équilibre hydrostatique n'est pas une
  propriété de la dimension — mais **un énoncé privé de son cadre est plus difficile à interpréter
  qu'il n'aurait dû l'être**, et c'est ainsi qu'une erreur d'unités comme celle de C04 survit à
  vingt-et-une sessions de relecture.

**Ajoutés en B-S23, en exécutant trois cas de plus**
- **A152** *(sévérité 1)* — **Un paramètre qu'un énoncé ne fixe pas est tranché en silence par le
  premier qui mesure.** Cinq des sept cas canoniques exécutés à ce jour portent un tel paramètre, et
  chacun déplace le verdict **au-delà de sa propre tolérance** : la maille de C03 (6,0 ou 43,1
  périodes de demi-vie — échec ou succès), le seuil de mouillage de C04 (5 % pour une tolérance de
  3 %), la normalisation du « 2 % » de C06 (facteur 20), le support et la norme de C08 (`p` de 0,95
  à 0), la masse volumique de C10 (2,5 % pour une tolérance de 1 %). **Ce n'est pas une série
  d'inattentions** : tant que personne n'exécute un cas, le paramètre manquant n'existe pas — rien
  ne peut le révéler. Le coût est précis : **deux candidats peuvent passer le même cas chacun sous
  ses propres conditions, et n'être jamais comparés.** Un banc bâti sur des cas sous-spécifiés ne
  classe personne. Voir ADR-039.
- **A153** — **L'ordre d'un schéma est un couple (schéma, solution), pas un nombre.** Le même code
  donne 0,95 sur une seiche bien résolue et 0,66 sur un front discontinu — et 0 si on change de
  norme. Aucune des trois mesures n'est fausse. Conséquence directe : un énoncé de la forme
  « ce solveur est d'ordre un » n'est pas vérifiable, et un cas qui compare deux candidats doit
  fixer le support avant de comparer les nombres.
- **A154** — **Un cas partiel qui s'affiche vert ne se distingue pas d'un cas complet.** C06 a été
  exécuté sans solide, sans rotation et en une dimension — c'est-à-dire privé des deux raisons
  d'être que son propre énoncé lui donne. Le tiers exécuté passe à 0,76 %. Rien dans un rapport de
  cas ne distingue « ce cas passe » de « le tiers de ce cas que j'ai su écrire passe », et c'est le
  rapport qui doit le dire — à chaque exécution, pas une fois dans un document. Le harnais imprime
  désormais `C06*` et `C10*` comme partiels.

**Ajoutés en B-S24, en passant à l'ordre deux**
- **A155** *(sévérité 1)* — **Une simplification algébrique efface le domaine où elle est valide.**
  Le terme de fond d'Audusse s'écrit avec les hauteurs reconstruites aux **deux bords** de la maille.
  À l'ordre un les deux valent `h_i`, l'expression se réduit à une forme plus courte, et c'est cette
  forme courte qui a été écrite en B-S22 — correctement. Étendue à l'ordre deux, où les deux bords
  diffèrent, elle injectait une force `−g·h·σ` **sur fond plat**, là où la source doit être
  exactement nulle : quinze fois l'erreur du schéma d'ordre un, et croissante sous raffinement.
  **Rien dans l'écriture de la forme courte ne rappelait l'hypothèse qui l'autorise.** C'est A78/L43
  transposé au code : une simplification est une perte d'information sur son propre domaine de
  validité, et elle se paie au premier changement de contexte. *Trouvé par le cas diagnostic de C08
  (L124), pas par la relecture.*
- **A156** *(sévérité 1)* — **Un cas qui réduit une solution à un scalaire peut classer deux
  candidats à l'envers.** Mesuré sur la seule position du front de Ritter, MUSCL + Euler paraît le
  meilleur des trois schémas — 0,87 % contre 6,11 % — alors qu'il est **2,2 fois pire en `L¹`** et
  que son front **dépasse** la référence à la maille suivante. Ce n'est pas de la précision, c'est
  une traîne parasite qu'un seuil de détection compte comme du front. **Un classement fondé sur un
  point d'une solution n'est pas un classement** — et c'est précisément ce à quoi servent les cas
  canoniques. Conséquence portée dans C04 : toute assertion ponctuelle doit être accompagnée d'une
  norme sur la solution entière.

**Ajoutés en B-S25, en disculpant le solveur**
- **A157** *(sévérité 1)* — **Un seuil choisi pour la reproductibilité seule peut être reproductible
  et dénué de sens.** ADR-039 avait posé le test des deux implémenteurs : *deux personnes qui ne se
  parlent pas obtiennent-elles le même nombre ?* Le seuil de mouillage de 10⁻⁶ m le satisfaisait
  parfaitement — et il désignait un **micron d'eau**, épaisseur à laquelle ni le modèle moyenné sur
  la hauteur, ni la rugosité d'un fond, ni le rendu du jeu n'ont de sens. Pire : il choisissait
  exactement le régime où **aucun** schéma de volumes finis ne peut suivre, donc un critère
  qu'aucun candidat ne satisfait, donc qui n'élimine personne. Le solveur a été suspecté trois
  sessions durant alors qu'il suivait le front à 0,7 %. **Le test des deux implémenteurs est
  nécessaire et non suffisant : une condition de mesure doit être reproductible *et* physiquement
  interprétable.** Et le cas est aggravé par le fait que celui qui fixe le seuil est celui dont le
  code est jugé par lui — **la révision demande une confirmation extérieure**.

  > **Note S41 — relu, et l'énoncé est incomplet.** Le fait est exact, et le remède — *une
  > condition de mesure doit être reproductible **et** physiquement interprétable* — tient. Mais la
  > fiche ne dit pas que **deux mesures reproductibles peuvent être incomparables entre elles**, et
  > c'est le cas ici : les deux véhicules mesurent le front à des seuils **et des références**
  > différents — `10⁻³` contre front ponctuel, `10⁻²·h₀` contre front moyenné sur la maille — et
  > `CAS-CANONIQUES` les met côte à côte depuis S36.
  >
  > **À seuil égal, les deux fronts sont identiques à la quatrième décimale.** L'écart de verdicts
  > vient pour **52 %** de la mesure et pour 48 % du schéma. Voir
  > [`AUDIT-ANGLES-IMPORTES-S41`](AUDIT-ANGLES-IMPORTES-S41.md) §6.
- **A158** — **Le coût d'un instrument de mesure croît sans que personne le regarde.** La batterie
  analytique valait 0,04 s en S20 ; elle a atteint **74 secondes** en B-S25, chaque session n'y ayant
  ajouté « qu'un balayage ». Personne ne mesure le temps de l'outil qui mesure. *(Le remède ici a
  été trouvé en passant : le mode release rend 11,7 s, et le hash de conformité y est **identique**
  — ce qui vérifie enfin, plutôt qu'affirme, la propriété pour laquelle Rust avait été retenu.)*

**Ajoutés en B-S26, en mesurant l'éponge**
- **A159** *(sévérité 1)* — **Une formule énoncée avec ses constantes n'invite pas à être
  recalculée.** ADR-005 §2 pose `R ≈ exp(−2∫σ/c ds)`, un profil quadratique, et conclut que
  `σ_max ≈ 4·c/L_s` donne `R < 1 %`. L'intégrale vaut `σ_max·L_s/3`, donc `R = exp(−8/3) = 6,95 %` —
  **un facteur sept**. La vérification tient en une multiplication, et l'erreur a survécu **depuis
  S05** : six audits, deux revues croisées, et un dossier de banc qui s'appuie dessus pour fixer la
  borne haute du paramètre le plus connecté du corpus. **Un énoncé qui a la forme d'un résultat se
  cite ; il ne se recalcule pas.** Le remède n'est pas de tout revérifier mais de repérer les
  formules **dont dépend une décision** et de les refaire, une fois, avec leurs constantes.
- **A160** — **Un coefficient d'amortissement n'a de sens que tant que `σ·dt < 1`.** Écrit
  `×(1 − σ·dt)`, il devient négatif au-delà, et une saturation à zéro le transforme en **remise à
  l'état de repos** : la maille n'est plus amortie, elle est écrasée. L'opérateur change de nature
  **sans erreur, sans avertissement, et en continuant à produire des nombres plausibles** — les
  éponges d'une à trois mailles réfléchissent peu, et ne mesurent pourtant plus une éponge. C'est
  l'analogue, pour un terme source, de la condition CFL sur le transport ; le corpus posait la
  seconde et ignorait la première.

- **A168** — **Écrire une distinction ne suffit pas à s'en servir.** `ADR-043` §5 établit que le
  corpus appelle « éponge » trois fonctions distinctes, et met en garde : *tant qu'elles partagent
  le mot, un résultat sur l'une se lit comme un résultat sur l'autre*. **Trois paragraphes plus
  bas, son propre §6 compte comme un desserrage de la borne de `λ_cut` un résultat qui porte sur
  le masque, alors que la borne vient de l'absorbeur.** L'erreur exacte contre laquelle le §5
  mettait en garde, commise par le document qui l'écrit, dans la même session.
  *Une distinction neuve ne devient opérante qu'après avoir été passée sur les conclusions
  déjà écrites, y compris celles du document qui la pose.* Relevé en S39, quatre sessions plus
  tard, et seulement parce qu'une rétractation extérieure a forcé à relire D1.

- **A169** — **Une prudence justifiée par une mauvaise raison se défend mal et se reporte
  longtemps.** `ADR-044` §7 refusait d'aligner les deux seuils de sec, et donnait pour raison qu'un
  alignement *déplacerait la position du front, donc le verdict de C04, donc le critère d'entrée au
  banc B3*. **La prudence était bonne ; sa justification était fausse** — le front bouge de 0,148 %
  sur sept décades, pour une tolérance de 3 %. La vraie raison de ne pas trancher à la légère était
  qu'une des deux valeurs n'avait **aucune provenance**, ce qui est un défaut d'ignorance et non
  d'écart. Conséquence pratique : **l'action a été reportée quatre fois**, parce que le coût
  annoncé — rouvrir un critère de banc — la faisait paraître plus lourde qu'elle n'était.
  *Une raison fausse donnée à l'appui d'une bonne décision la rend indéfendable au moment de
  l'exécuter : personne ne peut estimer ce qu'elle coûte.*

- **A170** *(sévérité 1)* — **Une saturation d'affichage transforme un refus en la meilleure
  mesure possible.** `C03-demi-vie` saturait sa valeur à `10⁶` périodes, pour afficher proprement
  une demi-vie infinie. Sur un bassin **sans seiche**, la régression ne trouve aucun point et rend
  `NaN` — mais **`f64::min` avale les `NaN`** : `NaN.min(10⁶)` rend `10⁶`. Le refus devenait donc
  la plus grande valeur du domaine, c'est-à-dire le **meilleur score** face à un minorant de 15.
  Le cas échouait par ailleurs — la tolérance conditionnelle testait `NaN >= 15`, faux — mais
  **l'assertion qui porte le résultat publié déclarait le néant excellent**, et le rapport affichait
  `1000000` là où la mesure valait `NaN` (**A149**). *Toute valeur de repli appliquée **après** une
  mesure doit préserver ce que la mesure a refusé : un refus qui traverse une saturation devient un
  résultat.* Trouvé par l'essai à zéro de S42, pas par la relecture.

- **A171** *(sévérité 1)* — **Une valeur de repli qui coïncide avec la valeur nominale est
  invisible à tout contrôle.** `ordre_grossier_estime` rendait **`1.0`** dans les trois cas où il
  n'y a aucun ordre à mesurer : moins de trois grilles, erreurs toutes nulles, et surtout **deux
  grilles successives de même erreur** — c'est-à-dire un solveur qui **ne converge pas**.

  Or `1,0` est **l'ordre nominal du schéma d'essai**, la valeur qu'on espère lire. Et le garde-fou
  **G10**, qui signale tout ordre hors de `[0,3 ; 3,0]`, ne pouvait pas broncher : `1,0` est
  dedans. *Le repli était silencieux par construction* — non par oubli de signalement, mais parce
  que **la valeur choisie pour « ne rien dire » était celle qui dit tout va bien**.

  **Ce qu'A170 décrit en général, celui-ci le porte au pire** : une valeur de repli distincte du
  domaine nominal — `10⁶` périodes pour C03 — finit par paraître suspecte à un lecteur. Une valeur
  de repli **dans** le domaine nominal ne paraîtra jamais suspecte à personne.

  *Une valeur de repli doit être choisie hors du domaine des valeurs valides, ou ne pas exister :
  le refus va dans le type.* Et l'audit de S34, qui a examiné G10 et corrigé son bornage, n'a pas
  vu ce repli — **il était sur la ligne juste au-dessus du `clamp`**.

- **A172** *(sévérité 1)* — **Quand la grandeur mesurée est un écart, zéro est son meilleur point,
  et aucune valeur de repli n'y est acceptable.** Trois défauts en trois sessions, trois formes
  différentes, une seule cause :

  | | forme | valeur rendue | position dans le domaine |
  |---|---|---|---|
  | S42 | `NaN.min(10⁶)` | `10⁶` | **hors** du plausible |
  | S43 | `else { 1.0 }` | `1,0` | **dans** le nominal (**A171**) |
  | S44 | `(15 − NaN).max(0)` | `0` | **le meilleur point** |

  **La gravité croît et la visibilité décroît dans le même ordre.** Un `10⁶` finit par se faire
  remarquer ; un `1,0` au milieu des ordres attendus, jamais ; un `0` sur un déficit **est le
  résultat qu'on espère**. Et le motif ne tient pas au repli mais à la **grandeur** : dès qu'une
  mesure est un écart, une erreur ou un déficit, toute opération capable de produire zéro à partir
  d'un refus le transforme en succès parfait — `unwrap_or(0.0)`, `max(0.0)`, une soustraction de
  deux valeurs égales par défaut. *Le refus doit aller dans le **type**.*

  **Corollaire, et c'est ce que l'audit a coûté à trouver** : un repli ne s'inspecte jamais seul.
  `unwrap_or(NaN)` est irréprochable, et il a produit les trois défauts — parce que `min`, `max`
  et une soustraction suivie d'un `max` avalent tous le `NaN`. **C'est l'aval qu'il faut suivre.**

---

### Angles morts importés de la lignée B — B-S27 (2026-09-07, S39)

> Les deux suivants ont été ouverts en **B-S27**, où ils portaient les numéros `A117` et `A118`.
> **Tous deux de sévérité 1**, et aucun n'a été relu par cette lignée.

- **A166** *(sévérité 1)* — **Une mesure peut être juste et sans portée, et rien dans la mesure ne le
  dit.** B-S26 avait mesuré qu'une éponge réfléchit `1,6·10⁻³` quelle que soit sa largeur, et en avait
  tiré que la largeur ne dépend pas de la longueur d'onde. La mesure était **exacte**. Elle ne
  décrivait pourtant pas l'éponge mais **le milieu** : en eau peu profonde linéaire, une onde droite
  vérifie `u = c·η/h₀`, et multiplier `η` et `u` par un même facteur préserve cette relation —
  l'amortissement ponctuel y est sans réflexion **par accident algébrique**. En milieu dispersif, la
  relation est non locale, elle n'est pas préservée, et `R` atteint **67 %**.
  **Aucune vérification interne à la mesure n'aurait révélé cela** : ni un raffinement, ni un témoin,
  ni un balayage de paramètre. Il fallait **changer de milieu**, c'est-à-dire sortir du cadre où la
  mesure était faite. C'est la limite de la méthode que ces sessions emploient, et elle mérite d'être
  écrite : *une mesure ne peut pas dire de quel cadre elle dépend.* Le seul remède connu est de
  **nommer la réserve avant de conclure**, ce que B-S26 avait fait — et c'est ce qui a rendu la
  rétractation prévisible au lieu d'être un démenti.
- **A167** *(sévérité 1)* — **Un défaut de montage peut ne pas se voir dans le résultat qu'il
  menace.** Le montage de B-S27 a été faux deux fois : la fenêtre de mesure était contaminée par
  l'enroulement périodique, puis par la queue du train incident. **`R` valait 0,18 à 0,32 dans les
  trois montages**, faux comme juste. Seul un essai **construit pour être vide** — le même montage
  sans éponge, dont la fenêtre réfléchie doit ne rien contenir — a signalé les deux erreurs, à 5,1 %
  puis 3,4 % contre 4·10⁻¹⁷ pour le montage retenu. **Un résultat stable n'est pas un résultat
  valide**, et la stabilité est même ce qui endort : elle ressemble à de la robustesse. Le corollaire
  est opérationnel : *tout montage de mesure doit venir avec un essai dont le résultat attendu est
  zéro.*

  > **Note S42 — appliqué à C03 et C06, et il a payé sur le premier.** L'essai à zéro de C03 —
  > le même montage sans excitation — a montré que `C03-demi-vie` **déclarait le néant conforme**,
  > avec le meilleur score possible (**A170**). Celui de C06 — le même montage sans boost — rend
  > **exactement zéro** : le montage était sain, et on le sait maintenant.
  >
  > *Un essai à zéro qui réussit du premier coup n'est pas du travail perdu : c'est la seule façon
  > de distinguer un montage sain d'un montage jamais interrogé.* Restent **C08** et **C02**.

- **A173** *(sévérité 1, corrigé en S45)* — **Le refus peut retirer l'assertion ou être ignoré
  par une sélection.** C02 produisait zéro assertion lorsque sa mesure de période/longueur
  échouait ; C10 ignorait les NaN pendant la recherche d'une crête et validait quatre grandeurs
  sur une surface supposée nulle. Reproduit avec un champ hors référentiel ; C02 également
  sans excitation. Corrigé avec conservation des trois assertions C02 et propagation du refus
  aux quatre mesures C10. Témoins nominal et statique sans houle préservés.
  Voir AUDIT-REPLIS-S44 §7. Aucun succès indu supplémentaire trouvé sur les douze autres origines.

- **A174** *(sévérité 1, corrigé en S46 sur le rapport principal)* — **Filtrer les refus fabrique
  une famille qui peut sembler stable ; omettre les indéterminés fabrique un bilan incomplet.**
  asymptotique retirait les triplets non observables et pouvait rendre Some(true) sur la suite
  restante, reproduit par test. Deux branches de verdict C08 appliquaient ensuite des règles
  différentes et ne comptaient pas tous les sans-verdict. Désormais les trous sont conservés,
  la stabilité est exigée sur le cas régulier, chaque famille est comptée. Le cas régulier
  nominal p = 0,82 rejoint les quatre autres sans-verdict : 5/5 au lieu de 4/5. La paire
  héritée shallow reste à confronter au même contrat (S46-1). AUDIT-REPLIS-S44 §8.

- **A175** *(sévérité 1, corrigé S47)* — **Des nombres comparables peuvent porter des verdicts
  incompatibles.** Le C08 hérité validait p > 0,8 sur Ritter et trois grilles ; le contrat courant
  exige un cas régulier et un régime asymptotique établi. L'import conservait les chiffres et
  le mot « vert », sans confronter leurs conditions d'emploi. Diagnostic requalifié, contrôle
  de cohérence conservé et compté séparément ; notes ADR-040/043. S47-1 porte la mesure régulière.

- **A176** *(sévérité 2, corrigé S51)* — **Une formule dupliquée peut conserver un refus
  obsolète après correction de son autre copie.** Convergence::ordre refusait les non-finis
  depuis S46 ; ordre_grossier_estime rendait encore Some(NaN), reproduit par test.
  Les deux chemins partagent désormais le calcul de Richardson, en conservant leurs
  planchers et catégories. Aucun succès nominal indu démontré, mais le filtre pouvait
  recevoir NaN au lieu de sa valeur de secours. Voir AUDIT-MESURES-S51 et L172.

- **A177** *(sévérité 1, corrigé S53)* — **Un contrôle de convergence peut valider les nombres
  sans vérifier leurs grilles.** Une suite aux erreurs divisées par deux et aux tailles
  100,200,800,1600,3200 donnait une stabilité vraie : le calcul ignorait les tailles.
  Le montage C22 supprimait aussi une allocation refusée avant le contrôle. Doublements
  désormais vérifiés, refus conservés et filtre limité au préfixe ; grille nulle refusée
  avant modulo. Reproductions et témoins dans GRILLES-C22-S53. Rapport nominal inchangé.

- **A178** *(sévérité 2, corrigé S55)* — **Un plateau de quantification efface des extrema
  si le détecteur oublie le dernier sens non nul.** Seiche excitée nx=400, 60 s : cinq extrema
  détectés au lieu de treize, donc refus au seuil de six. La même faute biaisait la sélection
  des extrema sur les montages acceptés. Sens conservé au travers des plateaux ; seuils
  inchangés, tests analytiques et physiques. Voir EXTREMA-SEICHE-S55 et L174.

- **A179** *(sévérité 2, ouvert, S57)* — **Un filtre de contamination peut être piloté par
  la qualité de l'oracle dont on ne se sert pas.** Le filtre ×30 de C22 compare l'erreur
  d'une grille à l'**écart L1 entre les deux oracles**, quantité dominée par le biais du plus
  **grossier** des deux — l'auxiliaire, dont aucune erreur publiée ne dépend. Mesure : le
  déplacement des erreurs entre S56 et S57 est additif et constant (5,587e-11 dès nx=800) ;
  ajusté avec la colonne « variation » (1,310e-10) il donne `c(n) ∝ n^-1,879`, soit 4,9e-11
  pour l'oracle de mesure 153600 contre 1,80e-10 pour 76800. L'erreur de la grille 12800
  dépasse alors le biais qui la contamine d'un facteur **159**, quand l'indicateur employé
  n'en vaut que **5,5** fois — et elle est refusée à 0,9556 du seuil.
  **Aucun chiffre faux n'en découle** : le filtre est conservateur, il refuse et n'a jamais
  admis à tort. Le coût est ailleurs — trois campagnes « sans verdict » et vingt-six minutes
  de calcul pour un déficit final de 4,4 %. Loi ajustée sur deux différences, biais additif
  uniforme supposé et non établi : **rien n'est modifié**, le refus est maintenu.
  Voir MESURES-C22-S57 §3 et L175. Action S57-1.

- **A180** *(sévérité 2, ouvert par décision, S58)* — **Un cas peut être indépendant par sa
  méthode et aveugle à ses paramètres, et c'est la première propriété qu'on lui prête.** Les
  quatre assertions de C10 passent à écart **0,000 %** avec `ρ_eau = 1000` **comme** avec 1025,
  parce que leurs trois références sont construites avec la constante qu'elles sont censées
  contrôler. `C10-tirant` est pourtant décrit dans le harnais comme *« le seul vraiment
  indépendant »* — et il l'est, mais **du chemin de calcul** : la bissection sur la force contre
  une formule fermée. Cela ne lui donne aucune prise sur la **valeur** de la constante.
  Le corpus a présenté ce cas pendant **trente-sept sessions** comme celui qui arbitrait `ρ_eau`,
  en lui opposant une tolérance de ±1 % qui porte sur un écart structurellement nul.
  **Les seuls contrôles réels étaient deux assertions de test unitaire** comparant le tirant à un
  littéral — les seules à tomber au balayage. **Ouvert par décision** : ADR-048 D3 refuse de
  mettre un littéral dans la référence de C10, faute d'une source de tirant indépendante de la
  formule vérifiée. Inventer cette source serait fabriquer la mesure qui manque.
  Voir RHO-EAU-S58, ADR-048 et L176. Instance d'**A104**, jamais reliée à **A103**.

- **A181** *(sévérité 2, corrigé S59)* — **Un protocole peut prescrire un remède qui détruit ce
  qu'il doit rendre possible.** `REFERENCE-C22-S56` §5 demandait, au-delà du quart d'heure, « un
  découpage de calcul en tranches temporelles gardées en mémoire », pour rendre supportable une
  campagne longue. Or l'intégration est à **pas adaptatif tronqué** — `dt = dt_cfl.min(t_fin − t)`
  — donc chaque borne de tranche insère un pas absent du calcul continu : le champ change bit à
  bit, mesuré à nx=200 sur quatre tranches. Appliqué, le remède rendait la campagne incomparable
  à S48, S49, S56 et S57, c'est-à-dire supprimait la seule chose qu'on venait y chercher.
  La prescription a été écrite par la session qui n'avait pas encore le besoin, et n'a été
  éprouvée qu'au moment de l'appliquer — trois sessions plus tard. **Corrigé** : le découpage
  retenu est celui de l'**observation**, `avancer_jusqu_a_observe`, qui porte désormais la seule
  boucle d'intégration ; l'identité avec `avancer_jusqu_a` est structurelle. Le chemin fautif est
  conservé comme témoin dans `le_decoupage_temporel_n_est_pas_neutre`.
  Voir MESURES-C22-S59 et **L177**.

- **A182** *(sévérité 2, ouvert par décision, S60)* — **Un seul critère d'admission peut gouverner
  deux grandeurs de nature différente, et être juste pour l'une seulement.** C22 publie des
  **erreurs** et un **ordre**. Le filtre ×30 protège la fiabilité des erreurs, et il le fait bien :
  à oracle 3200/6400 il refuse une grille dont l'erreur est fausse de 5,2 %. Mais une grille
  refusée **coupe la famille**, donc supprime tous les triplets qui la contiennent — et l'ordre,
  estimé sur des différences successives, est insensible à la contamination additive qui motive
  le refus. Mesuré : le triplet écarté en S56 valait **1,997566515**, contre **1,997599436**
  publié par S59 après deux campagnes et **33 min 08 s** de calcul. Le filtre n'est pas mal
  calibré, il est **mal attribué** — voir ADR-049, qui requalifie **A179** et le conserve intact.
  **Ouvert par décision** : le régime où l'erreur d'une grille passe sous l'écart des oracles n'a
  jamais été observé, donc rien ne permet encore d'attribuer un critère propre à l'ordre.
  Action **S60-1**. Voir **L178**.

- **A183** *(sévérité 2, ouvert par décision, S61)* — **Le seuil d'admission d'une mesure d'ordre
  est une fonction de l'ordre qu'elle cherche.** Le filtre ×30 de C22 s'écrit, pour un schéma
  d'ordre `p` et un rapport `k = oracle/grille`, `k^p ≥ 30·(1 − 2^-p)` : il demande `k ≥ 4,7` à
  l'ordre deux, **`k ≥ 15` à l'ordre un**, `k ≥ 3,2` à l'ordre trois. Dimensionner une campagne
  suppose donc de connaître sa réponse — et se tromper d'hypothèse ne produit **aucune erreur
  visible** : cela produit un « sans verdict ». C'est l'histoire de C22 de S48 à S57, quatre
  campagnes et près d'une heure de calcul pour un point de bascule qui se lisait sur la suite
  `k = 2, 4, 6, 7`.
  Ce n'est pas un défaut de ce filtre mais une propriété de tout critère d'admission bâti sur une
  comparaison d'erreurs entre grilles. **Ouvert par décision** : rien n'est modifié, le critère
  reste celui d'ADR-049 D1 ; ce qui est ajouté est l'**annonce** de ce qu'une campagne pourra
  admettre, avant de la payer, avec l'hypothèse d'ordre écrite sur la même ligne.
  Voir ADR-050, GEOMETRIE-DU-FILTRE-S61 et **L179**.

- **A184** *(sévérité 2, corrigé S62)* — **La condition de validité d'une assertion peut vivre
  ailleurs que l'assertion, et le fichier qui l'affirme auto-suffisant ne s'en aperçoit pas.**
  `main.rs` appelait `hs_restitue(bg, t, sc.hs, 128, 3.0)` : la fenêtre d'échantillonnage — les
  deux nombres qui gouvernent la mesure la plus fragile du harnais — était **littérale**, invisible
  depuis le scénario. Or `C18-invariants.toml` déclare en en-tête que *« les assertions vivent ici
  et non dans le code du harnais : ce fichier est auto-suffisant »*, et SPEC-003 §3 l'exige.
  L'écart de `Hs` passe de **0,018 % à 15,58 %** selon cette fenêtre, et le lecteur du scénario
  n'avait aucun moyen de le savoir. Le défaut s'est révélé par un balayage **qui ne déplaçait
  rien** : `grille_cote` varié d'un facteur 16 laissait l'écart au chiffre près.
  **Corrigé** : `physics.fenetre_cote` et `physics.fenetre_pas_m`, défauts inchangés, mesures et
  hashs inchangés. Instance littérale d'**A104**. Voir AUDIT-REFERENCES-S62 §3.4 et **L180**.

- **A185** *(sévérité 2, corrigé S63)* — **Un état recopié se périme comme un décompte, et rien ne
  le signale.** `DOSSIER-B2` §8, *« ce qui doit être vrai avant de lancer »*, datait de S14 et
  annonçait cinq blocages : H1 non écrit, H3 non écrit, C01 et C02 non exécutés, ADR-020 proposé.
  **Quatre étaient levés depuis une quarantaine de sessions** — H1 et H3 en S20, C01 en S22 et S36,
  ADR-020 **actée en S19**. Une session lisant ce tableau pour décider du lancement du banc décisif
  de `λ_cut` en aurait conclu qu'il est hors d'atteinte, alors qu'il ne manque qu'une pièce.
  Le rituel de fin fait vérifier les **décomptes** recopiés depuis S07 et S10 ; il ne faisait rien
  pour les **états**, qui se périment de la même façon et pour la même raison.
  **Corrigé** : le tableau porte une colonne « constaté » par ligne et un encadré daté, et le
  rituel (`REPRISE.md` §6, point 5) porte la règle — *un état sans date se lit au présent, et il ne
  l'est plus*. Voir PRESCRIPTIONS-S63 §3 et **L181**.

- **A186** *(sévérité 2, ouvert, S64)* — **Il n'existe qu'une réalisation par état de mer, et un
  paramètre obligatoire du scénario laisse croire le contraire.** Le déphasage des composantes est
  dérivé de leur indice, sans PRNG — le commentaire l'assume : *reproductible, sans PRNG et sans
  horloge*. Mais `scenario.graine` est **lue, obligatoire, et utilisée nulle part** ; `SeaState`
  n'a pas de champ correspondant. Six graines rendent six fois le même chiffre à la sixième
  décimale. **Conséquence, et elle dépasse le paramètre mort** : aucune mesure statistique du
  corpus — `Hs`, homogénéité, toute grandeur d'ensemble — ne peut être répétée sur une autre
  réalisation. Leur écart n'a donc **aucune barre d'erreur mesurable**, et aucune tolérance ne se
  calibre (**A106**, I-14). Déterminisme et réalisation unique sont deux choses distinctes : une
  graine branchée les concilierait. Action **S64-1**, qui débloque **S64-2**.
  Voir ADR-051 §2 D2 et **L182**.

- **A187** *(sévérité 2, ouvert, S64)* — **À spectre dense, `Hs` s'écarte de 6,6 % pour une cause
  qui n'est ni la fenêtre ni le pas.** À 256 composantes et fenêtre 3072 m, l'écart vaut 6,612 % —
  contre 0,282 % à 32 composantes et 0,367 % à 64. Et il ne bouge pas quand on raffine
  l'échantillonnage : 6,612 / 6,614 / 6,615 % à pas 3,0 / 1,5 / 1,0 m, fenêtre égale. Ce n'est donc
  pas un défaut de mesure. Pistes écartées : la borne du spectre est `[Tp/2, 2Tp]` **quel que soit
  le nombre de composantes**, donc la plus grande longueur d'onde ne croît pas ; et l'amplitude par
  composante est choisie pour que `m0` vaille `Hs²/16` **exactement**, par construction. Reste la
  corrélation entre composantes — toutes dans un cône de 30 degrés, de plus en plus voisines quand
  leur nombre croît — qui invaliderait l'hypothèse d'indépendance sous-jacente. **Non vérifié.**
  Tant que la cause est inconnue, la tolérance de `Hs` ne peut pas descendre sous 7 % sans exclure
  une configuration que rien ne permet de déclarer invalide. Voir ADR-051 §2 D2.


**Suivi A186 — S65, 2026-09-08.** Paramètre mort corrigé : graine transmise aux phases,
réalisations distinctes et reproductibles vérifiées. GRAINES-S65. Barres d’erreur encore
à établir ; A187 reste ouvert. Six graines ne prouvent pas leur indépendance statistique.

- **A188** *(sévérité 2, S66 ; attribution corrigée, contrôle à instruire)* — **Un écart de
  variance entre deux fenêtres ne désigne pas sa cause.** Le contrôle prétend surveiller la
  précision spatiale ; son échec nominal à 39,75 % subsiste en f64, avec différence du ratio
  de 3,35e-7. Les covariances entre composantes sur 144 m expliquent la hausse. Aucune perte
  de précision causant ce refus établie. Commentaire corrigé, verdict et seuil conservés ;
  S66-1 porte le devenir du contrôle. HOMOGENEITE-S66, L183.

**Suivi A187 — S67, 2026-09-08 : cause expliquée.** Les phases historiques reproduisent
+6,612 % ; la contribution croisée des composantes explique le résidu, la somme des variances
individuelles rend Hs 1,200001971 m. À 6144 m, Hs vaut 1,218126498 : « ni la fenêtre ni le pas »
était trop fort, seul le défaut de pas avait été écarté. Les battements voisins atteignent
20,208 km à 256 composantes. S64-3 close ; calibration toujours à faire, aucune tolérance
resserrée. Voir SPECTRE-DENSE-S67 et L184. Le présent suivi supplante la cause inconnue ci-dessus.

**Suivi A188 — S68, 2026-09-08 : traité par ADR-052.** Le rapport sépare désormais le ratio
statistique sans verdict et la précision spatiale directement bornée, avec défaut injecté.
Refus statistique non fini conservé. Les mesures statistiques ne sont pas déclarées validées.
S66-1 close ; voir CONTROLES-S68.

- **A189** *(sévérité 2, S71 ; traité par ADR-054)* — **Un budget partagé devient un faux
  préalable de construction.** ADR-053 imposait B1 avant WaveEvent au motif du coût B+W,
  alors que B1 complet exige aussi LOD et évaluation perceptuelle. L’ordre aurait arrêté W
  derrière des composants absents. Coût partagé signifie réception commune du budget,
  pas dépendance des identités et unités d’événements. Première étape W1 désormais explicite.

- **A190** *(sévérité 1, S72 ; ouverte, W2)* — **La prédiction ne connaît pas le server_seq.**
  SPEC-006 §3.3 promet une réconciliation par id entre événement anticipé et serveur, sans
  définir leur correspondance. Deux compteurs ne constituent pas cette correspondance.
  Il faut une identité de cause corrélable et des espaces de noms, avant le journal W2.
  ADR-055 garde les id opaques et ne prétend pas résoudre cette association.

**Suivi A190 — S73 : contrat interne résolu par ADR-056.** La cause gameplay, distincte du
server_seq, relie prédiction et confirmation ; tests avec id différents. Enveloppe de transport
et raccordement hôte restent à réaliser, sans prétendre que le réseau est intégré.

- **A191** *(sévérité 1, S73 ; ouverte)* — **Un refus de capacité rend le rejeu incomplet.**
  Un pool qui refuse sans corrompre est correct localement mais peut exposer un sous-ensemble
  dépendant de l’ordre des arrivées. Full doit invalider la complétude et déclencher une reprise.
  Le premier journal le signale à l’appelant, sans encore mémoriser cet état. Suite S73-1.

**Suivi A191 — S74 : traité localement par ADR-057.** Full mémorise une perte connue,
persistée et restaurée. Aucun succès ultérieur ne l’efface implicitement. Absence de perte
connue ne signifie pas complétude réseau ; protocole de resynchronisation hôte encore ouvert.

- **A192** *(sévérité 1, S79 ; corrigée)* — **La vitesse verticale peut contredire la surface
  alors que la vitesse horizontale est juste.** B rendait w=-deta_dt. L’identité orbitale
  horizontale de S21 ne testait pas la condition cinématique verticale. Signe corrigé, contrôle
  par différence temporelle ajouté ; références de conformité renouvelées, ADR-062.

- **A193** *(sévérité 1, S83 ; corrigée dans WorldPos::to_local)* — **Une borne locale après
  soustraction ne protège pas la soustraction.** Deux positions i64 lointaines pouvaient faire
  déborder le calcul avant le contrôle des 4096 m. checked_sub refuse désormais les extrêmes,
  dans les deux sens, et le lot conserve sa sortie. ADR-065.

- **A194** *(sévérité 2, S118 ; ouverte)* — **Une publication réussie ne dit pas que le montage
  est échantillonnable.** Le contrôleur de pression valide la fenêtre de son contexte, qui ne
  sait rien de la validité des impacts. `update(6 s)` réussit alors que les impacts expirent à
  4 s ; la vue est finie, et c'est la requête mixte qui refuse ensuite. Rien dans le type
  n'avertit l'hôte que la date qu'il vient de publier est inexploitable pour le montage complet.
  Verrouillé par un test, non résolu : il faudrait que le contrôleur consulte la validité des
  impacts, ou que le montage publie son horizon effectif. Suite S118-1.

- **A195** *(sévérité 2, S118 ; ouverte)* — **La position d'un bloc dans la séquence de mesure
  fabrique un écart de coût.** Le premier `measure` d'un processus est surestimé jusqu'à 28 % ;
  le même code replacé en dernier rejoint son témoin à 1 % près, et l'effet disparaît dès la
  seconde recette. Les trois échauffements de `measure` ne mettent pas la machine en régime.
  Toutes les campagnes depuis S104 comparent des blocs successifs dans un même exemple, et
  publient donc au moins un chiffre biaisé — celui mesuré en premier, généralement la
  préparation. Les chiffres passés ne sont pas corrigeables a posteriori ; il faut un bloc de
  mise en régime avant la première mesure, ou mesurer chaque voie deux fois à des positions
  différentes. Voir L207 et CYCLE-MIXTE-S118.

**Suivi A194 — S119 : résolue par ADR-079.** `mixed::horizon` et `mixed::state` donnent, avant
toute publication, la fenêtre servable et ce que la requête fera d'un instant. L'équivalence
entre l'annonce et le comportement est vérifiée par balayage, dans les deux sens, sur trois
montages dont un d'horizon vide. La publication tardive reste possible et délibérée : la vue
produite est correcte pour la pression seule.

**Suivi A195 — S119 : corrigée et vérifiée.** Un bloc de mise en régime précède désormais la
première mesure de la campagne. `update`, le même appel mesuré en dernier, et la préparation
directe coïncident alors, là où S118 lisait 15 à 28 % d'écart ; vérifié sur trois exécutions.
Les campagnes antérieures à S119 gardent leur premier chiffre biaisé ; elles ne sont pas
corrigeables a posteriori.

- **A196** *(sévérité 3, S119 ; ouverte)* — **L'annonce couvre le montage, pas les points.**
  ADR-079 rend prévisible ce qui dépend du montage et de l'instant. Restent évalués point par
  point, au moment de la requête : le domaine de chaque position, la pente totale et la capacité
  des tampons. Ces refus sont tardifs au sens exact où A194 l'était — l'hôte a déjà payé une
  préparation quand il les découvre — mais ils dépendent des arguments de la requête, et non du
  montage, donc aucune annonce préalable ne peut les couvrir sans recevoir les mêmes points.
  Ce qui est annonçable est une **borne** : pente maximale atteignable sur un lot, emprise du
  domaine. Suite S119-1.

**Suivi A196 — S120 : traitée à moitié par ADR-080.** Ce qui dépend de la géométrie est
désormais décidable avant la requête, par des prédicats posés dans les couches qui les
appliquent (`admits`), et la part constante de l'enveloppe de pente est annonçable
(`slope_floor`). Filtrer coûte ~40 ns par point contre 35,6 ms pour le lot qu'il sauve.
Reste hors d'atteinte : la finitude des calculs, qui ne se prévoit pas sans évaluer.

- **A197** *(sévérité 3, S120 ; ouverte)* — **Un champ dégénéré se présente comme un mauvais
  point.** `RadialImpact::sample` rend `Error::Domain` aussi bien pour une position hors
  domaine que pour une sortie non finie. Les deux causes n'ont rien de commun : la première est
  imputable à l'appelant et se corrige en changeant le point, la seconde est un défaut du champ
  et se reproduira partout. Un appelant qui filtre ses points sur `Domain` — ce que ADR-080
  rend naturel — écarterait silencieusement des positions parfaitement valides autour d'un champ
  dégénéré, et masquerait le vrai défaut. Séparer les deux causes touche la couche, pas
  l'annonce. Suite S120-1.

**Suivi A197 — S121 : résolue, après requalification de sa cible.** ADR-081 sépare
`NotRepresentable` de `Steepness`. La cible annoncée était le mauvais étage : le bloc de
finitude de `RadialImpact::sample` n'a aucune entrée qui l'atteigne — 18 719 champs construits,
673 884 échantillons, aucun refus, et la construction refuse une borne non représentable avant
que `sample` puisse déborder. Le défaut réel et atteignable était dans les **constructeurs**, où
« pente trop raide » et « champ non représentable » partageaient un seul nom. L'invariant
« construit ⟹ sorties finies » est désormais un test, avec sa marge surveillée.

- **A198** *(sévérité 3, S121 ; ouverte)* — **Les bornes de construction se recouvrent sans
  ordre documenté.** Selon la famille de paramètres, un champ d'impact est refusé par `Medium`,
  `Domain`, `Steepness` ou `NotRepresentable`, et rien ne dit laquelle borne quoi : la sonde de
  S121 voit `ImpactField` passer de `Domain` à « construit » sans jamais franchir la borne de
  pente, tandis que `RadialImpact` bascule sur la représentabilité au même endroit. Un appelant
  qui veut savoir **quel paramètre réduire** ne dispose que du nom d'une borne, pas de sa cause.
  Cartographier ces bornes — ou nommer celle qui a mordu, avec la valeur en cause — est le
  prolongement naturel. Suite S121-1.

**Suivi A198 — S122 : résolue par ADR-082.** Neuf noms remplacent les deux fourre-tout de
`RadialImpact::new` ; chaque nom désigne le paramètre à revoir, et les trois bornes couplées
sont nommées comme telles. Le test exige que chaque nom soit atteignable — un nom qu'aucune
entrée ne produit serait une promesse vide — et c'est lui qui a révélé qu'`Energy` recouvrait
encore un refus de longueur d'onde. `ImpactField` garde ses noms : plus le chemin actif.

- **A199** *(sévérité 2, S122 ; ouverte)* — **L'enveloppe du candidat radial n'a jamais été
  confrontée aux impacts que le jeu produira.** La carte mesurée en S122 montre un couloir
  étroit : longueurs d'onde de l'ordre du mètre à la dizaine de mètres, rayon d'autant plus
  petit que l'onde est courte, et bordé de trois refus distincts. Rien ne dit si cet intervalle
  couvre les cas réels — une goutte, un projectile, une coque de vaisseau, un impact d'arme.
  Si un régime attendu tombe hors du couloir, ce n'est pas un défaut de nommage mais un manque
  de modèle, et il faudra soit un second candidat, soit un raccordement. La question est de
  conception et se tranche sans interlocuteur (ADR-028). Suite S122-1.

**Suivi A199 — S123 : traitée par ADR-083, et elle en a ouvert deux autres.** La confrontation
demandée a eu lieu : sur onze cas de jeu couvrant six ordres de grandeur en taille, **un seul se
construit à la portée voulue**, et ce verdict ne dépend pas de la calibration inconnue. Les
causes sont maintenant nommées et séparées — portée bornée par la table (A200), régime d'eau
profonde assumé comme limite du modèle, plafond d'énergie chiffré pour le générateur à venir.

- **A200** *(sévérité 1, S123 ; ouverte)* — **La longueur d'onde d'un impact n'est reliée à
  rien.** Tout le candidat radial découle de `wavelength_m` : son étendue, sa vitesse de
  propagation, les bornes qui l'acceptent. ADR-055 la valide comme « positive en mètres » et
  confie le reste à un générateur physique qui n'existe pas ; ADR-060 qualifie son λ de 4 m de
  « paramètre d'essai uniquement ». Aucun document ne dit comment un objet qui tombe produit une
  longueur d'onde. Tant que ce lien manque, **aucun verdict d'acceptation n'a de sens physique**,
  et deux contenus identiques peuvent donner des champs sans rapport. ADR-083 pose le contrat
  `λ = α·b` et laisse `α` à calibrer ; l'angle reste ouvert tant que `α` n'est pas mesuré.
  Suite : banc B2.

- **A201** *(sévérité 2, S123 ; ouverte)* — **La portée d'un champ d'impact est bornée par une
  table, pas par la physique.** `hi · radius ≤ 64` vient de la tabulation de Bessel, qu'ADR-060
  range parmi les « choix numériques testés, pas des paramètres gameplay ». La conséquence n'a
  jamais été examinée : la portée vaut **5,09 λ**, soit une dizaine de fois la taille de l'objet,
  et un plongeon humain n'est calculable que dans un rayon de trois mètres. Ce n'est pas une
  décision de conception, c'est un effet de bord d'outillage. Piste identifiée, non retenue avant
  mesure : développement asymptotique de `J0`/`J1` au-delà de la table. Suite S123-1.

**Suivi A201 — S124 : traitée par ADR-084, et le gain n'est pas celui qu'on croyait.** Le domaine
de Bessel passe de 64 à **2048**, borne mesurée et non choisie : au-delà, la précision de la
phase spatiale en `f32` fait sortir l'erreur de la tolérance. `Reach` recule d'un facteur 32,
mais **`Resolution` prend aussitôt le relais** et le gain effectif va de zéro à +82 %. Avec
`N = 256`, déjà permis par ADR-060, neuf cas de jeu sur onze atteignent leur portée — les deux
leviers étaient nécessaires, aucun ne suffisait seul.

- **A202** *(sévérité 3, S124 ; ouverte)* — **La borne active est désormais un paramètre de
  profil jamais dimensionné pour cet usage.** Depuis ADR-084, `Resolution` borne la portée de
  tous les cas mesurés, et elle dépend de `N` par `dk ∝ 1/N`. Or `N` est libre entre 64 et 256
  depuis ADR-060, où il est présenté comme un choix de quadrature — personne ne l'a jamais
  choisi en fonction de la portée voulue, qui est pourtant ce qu'il commande. À `N = 256` la
  portée est multipliée par quatre et neuf cas sur onze passent, pour quatre fois plus de modes
  par point d'évaluation. **Le coût n'est pas mesuré**, donc l'arbitrage n'est pas fait : c'est
  la suite S124-1. Tant qu'il ne l'est pas, la portée d'un champ dépend d'un paramètre choisi
  pour une autre raison.

**Suivi A202 — S125 : traitée par ADR-085.** Coûts de construction, évaluation et mémoire
mesurés. N64 conservé par défaut ; N128/N256 explicites selon le domaine commun du service,
pas selon la qualité graphique locale. Réception physique étendue suivie par A203.

- **A203** *(sévérité 2, S125 ; ouverte, limite déjà signalée en S124)* — **L'admission
  à grande portée n'a pas de réception physique correspondante.** Neuf cas sur onze sont
  géométriquement constructibles à N256 ; le contrôle de variation de phase et la précision
  de Bessel ne bornent pas l'erreur finale de quadrature. S125 constate seulement la finitude
  de 384 points-temps étendus. Avant adoption dans le cycle, recevoir les sept composantes
  contre une référence indépendante avec convergence de l'oracle, près des bornes de rayon
  et d'âge. Porteur : S125-1, prochaine session S126. Aucun défaut physique établi à ce stade.

**Suivi A203 — S126 : partielle.** RECEPTION-ETENDUE-S126 reçoit sept composantes sur
1350 points-temps N128/R64 et N256/R128, λ4/âge0–4 s, oracle indépendant raffiné.
Erreur normalisée<=4,44e-7 pour1e-4 annoncé. Réception limitée à ces fixtures ; autres
paramètres et précision relative des queues non reçus. Le transport effectif ouvre A204.

- **A204** *(sévérité 2, S126 ; ouverte)* — **Un domaine calculable et précis au loin ne
  prouve pas qu'il puisse y transporter l'onde.** Les montages S125/S126 ont un âge4 s ;
  le déplacement caractéristique du groupe le plus rapide n'est que7,07 m. Les points
  à64/128 m mesurent surtout des queues très faibles, pas un paquet arrivé. Les horizons
  numériques permis aux deux profils,12,07/24,15 s, restent inférieurs aux temps de groupe
  R/c_g,max=36,22/72,44 s. Une simple prolongation serait refusée. Dimensionner portée et
  horizon ensemble, puis mesurer le transport ; ne pas lire le succès spatial comme une
  réception de portée utile. Porteur : S126-1, prochaine session S127.

**Suivi A204 — S127 : traitée sur fixture.** TRANSPORT-ETENDU-S127 dimensionne ensemble
N256/R80/horizon48, λ4, puis reçoit99,985 % de l'énergie de référence entre32 et80 m à48 s,
contre1,30e-7 E0 au départ. Rayon moyen53,42 m. Les sept composantes du candidat concordent
avec l'oracle ; bilan total de référence et part potentielle du candidat reçus, pas son bilan
cinétique complet. Les anciennes demandes64/128 m aux temps de groupe restent refusées.
Cycle hôte transporté encore ouvert : S127-1. A203 demeure partielle pour les autres domaines.

**Suivi A204 — S128 : cycle hôte du montage reçu.** S127-1 réalisée dans
CYCLE-TRANSPORTE-S128, après renouvellement et restauration,1280 points-temps identiques
aux champs directs. Le bilan cinétique du candidat étendu demeure distinct et devient S128-1.
Aucun élargissement implicite de la réception physique d'A203.

**Suivi A203/A204 — S129 : bilan des nœuds du candidat reçu sur la même fixture.**
[BILAN-CANDIDAT-ETENDU-S129](../validation/BILAN-CANDIDAT-ETENDU-S129.md) ferme S128-1 :
cinétique complète et total contre la référence S127, écart total maximal9,045e-7 E0.
Interférences conservées et contre-épreuve diagonale seule rejetée ;99,985214 % de E0 dans
l'anneau32–80 à48 s. A203 reste partielle pour les autres paramètres ; aucune réception
en profondeur finie ou du bilan mixte. Aucun angle nouveau distinct ajouté.

**Suivi A200 — S136 : traitée par ADR-092.** La longueur d'onde se **dérive** : la forme
spatiale initiale du candidat est exactement homothétique en λ, son premier zéro vaut 0,2985 λ,
et faire coïncider cette étendue avec la demi-largeur mouillée de Wagner donne `α = 3,35`. Les
trois lectures raisonnables du rayon bornent α à [3,35 ; 6,11] — un facteur 1,8, contre un
paramètre libre auparavant. L'énergie, elle, ne se dérive pas : la fraction transférée reste à
calibrer, mais le modèle en donne la borne `η ≤ 2Kgα⁴bs²/v²`, mesurée et vérifiée contre le
candidat. Ce qui reste ouvert n'est plus « d'où vient ce nombre » mais « quelle mesure le
resserre » — c'est le banc B2, et c'est la suite S136-1.

- **A205** *(sévérité 2, S138 ; ouverte)* — **Aucun banc ne fixe la limite de pente, et elle est
  peut-être dérivable.** `max_slope` décide de l'admissibilité de tout champ — c'est le refus
  `Steepness` — et vaut 0,1 dans toutes les fixtures depuis S77, sans provenance. ADR-058 §21 et
  ADR-062 §50 le renvoient « à calibrer B2 », qui ne mesure pas cela ; B4 juge la décomposition
  additive, pas la linéarité d'une onde. Le renvoi désigne donc un banc qui ne répondra pas.
  **Et la question n'est peut-être pas une mesure du monde** : SPEC-001 §4 donne la cambrure
  limite de Stokes `H/λ ≈ 1/7`, d'où une pente de déferlement `πH/λ ≈ 0,449` — quatre fois et
  demie le seuil employé. Ce qui manque n'est donc pas la limite physique, mais **le rapport
  entre la borne L1 du modèle** — `Σ|a_k|·k`, majoration conservative — **et la pente réelle du
  champ**, qui se mesure dans le modèle comme `α` l'a été en S136. Suite S138-1.

**Suivi A205 — S139 : traitée par ADR-094, et la question n'était pas celle qu'on croyait.** La
limite physique n'avait pas à être mesurée : SPEC-001 §4 la donne, `πH/λ = 0,4488` à la cambrure
limite de Stokes. Ce qui manquait est le rapport entre la borne L1 et la pente réelle, et il vaut
**1,7950713** — constante du modèle, invariante en λ, en énergie, en `N` et en rayon, retrouvée à
sept chiffres par une quadrature f64 indépendante. `RadialImpact::slope_max()` publie désormais
la pente réelle. **Mais le seuil ne devient dérivable que si le budget d'ADR-080 cesse
d'additionner une pente exacte et deux bornes L1 de facteurs différents** — voir A206 et l'action
S139-1. Voir [PENTE-REELLE-S139](../validation/PENTE-REELLE-S139.md).

- **A206** *(sévérité 2, S139 ; ouverte)* — **Le facteur de conservatisme de la pression n'est pas
  mesuré, et il n'est probablement pas constant.** `slope_envelope` additionne
  `(|kx|+|ky|)·(|Re η|+|Im η|)` (`spectral_pressure.rs:346`) : chacun des deux facteurs majore la
  grandeur réelle de 1 à √2 selon la direction du vecteur d'onde et la phase, et la somme sur les
  cases perd en plus toute compensation entre elles. Contrairement à `ρ = 1,7950713`, qui est fixé
  par une forme spectrale figée, ce facteur dépend de ce que l'appelant publie. Tant qu'il est
  inconnu, le budget de pente reste hétérogène et aucun seuil unique n'y est physiquement juste.
  Mesurable sur les fixtures existantes de `bound_pressure`, et c'est un préalable à S139-1.

- **A207** *(sévérité 3, S139 ; ouverte par décision)* — **Le critère de Stokes est appliqué à un
  paquet transitoire, ce qu'il ne décrit pas.** La cambrure limite `H/λ ≈ 1/7` est établie pour
  une onde progressive monochromatique et permanente en eau profonde. Le champ d'impact est un
  paquet dispersif dont la crête vit une fraction de période, et rien ne dit qu'il déferle au même
  seuil — la littérature de déferlement transitoire donne des critères plus élevés pour des
  paquets focalisés. ADR-094 l'emploie comme **majorant géométrique** — au-delà, la surface cesse
  d'être une fonction de la position, ce qui est vrai pour toute forme — et non comme prédiction
  de déferlement. Ouverte par décision : la trancher demanderait une mesure du monde, hors de
  portée d'une session (`REPRISE.md` §5), et le majorant géométrique suffit à l'usage qui en est
  fait.

**Suivi A206 — S140 : traitée par ADR-095, et la réponse est « non ».** Le facteur de
conservatisme de `slope_envelope` **n'est pas une constante** : il vaut 3,027 sur un spectre
gaussien réaliste — stable en résolution, donc caractéristique du spectre publié — et croît sans
borne quand l'emprise se resserre sur un zéro du champ (16,7 mesuré à 0,01 λ). Il se décompose en
un facteur de **forme**, borné par 2 et éliminé exactement par `slope_envelope_tight()`, et un
facteur d'**alignement**, que rien ne borne. La voie que S139 recommandait pour S139-1 — chaque
couche divise sa borne par son facteur — n'existe donc pas. Voir
[ENVELOPPE-PRESSION-S140](../validation/ENVELOPPE-PRESSION-S140.md).

- **A208** *(sévérité 2, S140 ; ouverte)* — **L'hôte consomme le budget de pente par un choix
  qu'aucun refus ne lui désigne : l'emprise.** À champ identique, publier sur une emprise étroite
  posée loin de la source multiplie la part de budget consommée — mesuré ×10,9 sur une case, sans
  borne supérieure — parce que la borne ne dépend pas de l'emprise quand la pente réelle, elle,
  en dépend. Le refus rendu est `Slope` ou `Steepness` : il désigne la pente, c'est-à-dire la
  seule chose que l'hôte n'a pas à changer. ADR-082 exige qu'un nom de refus désigne ce qu'il
  faut revoir ; celui-ci désigne le contraire. Deux réparations possibles, aucune tranchée :
  nommer le cas (`Footprint`), ou publier le rapport des deux enveloppes pour que l'appelant voie
  sa propre marge. À instruire avec S139-1.

- **A209** *(sévérité 2, S141 ; ouverte, **introduite par la migration**)* — **`Medium::max_slope`
  ne signifie plus la même chose selon le champ qui le lit.** Depuis S141, `RadialImpact` lui
  compare la pente **réelle** (borne L1 divisée par `SLOPE_L1_RATIO`) ; `ImpactField::new` lui
  compare toujours **sa borne L1**, dont le rapport à la pente réelle n'a jamais été mesuré. Un
  même `Medium` passé aux deux produit donc deux frontières de sens différent, et rien dans le
  type ne le dit. La migration a préféré nommer ce défaut plutôt que remplacer un facteur inconnu
  par un autre. `ImpactField` n'étant plus construit que par `probe_degenerate`, trois issues :
  mesurer son rapport comme S139 l'a fait pour le candidat radial, le retirer, ou séparer les deux
  significations dans le type. À trancher, pas à laisser dormir — c'est exactement la forme de
  défaut que L219 décrit.

**Suivi A209 — S142 : traitée par ADR-096.** Le rapport du champ modal vaut **1,701591**, et c'est
une constante du modèle comme celle du candidat radial — invariante en λ et en énergie, maximum
toujours atteint parce que le champ est périodique et sans emprise restreinte. `ImpactField::new`
compare désormais la pente réelle ; `max_slope` a un seul sens dans le crate. Le champ n'est
**pas** retiré : ADR-059 le conserve délibérément comme support de comparaison, et « personne ne le
construit » n'établit pas qu'il est mort (S39). Voir
[PENTE-MODALE-S142](../validation/PENTE-MODALE-S142.md).

- **A210** *(sévérité 2, S142 ; ouverte)* — **Rien n'oblige un futur champ à mesurer son rapport
  avant de comparer quoi que ce soit à `max_slope`.** Le crate porte maintenant deux constantes
  homonymes — 1,795071 pour la quadrature de Hankel, 1,701591 pour les 40 modes cartésiens — et
  elles **ne se déduisent pas l'une de l'autre** : chacune est une propriété du spectre de son
  champ. Un troisième champ aurait la sienne, et le langage ne l'empêchera pas d'écrire
  `slope > medium.max_slope` comme les deux premiers l'ont fait pendant soixante sessions. Le
  défaut n'est pas dans un calcul, il est dans le **dispositif** : aucune trace du contrat « ce qui
  est comparé à `max_slope` est une pente réelle » ne vit ailleurs que dans deux commentaires et
  deux essais. Trois réparations possibles, aucune tranchée : un type qui porte la pente réelle
  plutôt qu'un `f32` nu, un essai générique que tout champ doit passer, ou une entrée d'invariant.
  À instruire avant qu'un troisième champ existe — après, ce sera un audit.

**Suivi A210 — S143 : traitée par ADR-097, et la pesée a inversé la préférence de départ.** Le
type porteur — `Medium::max_slope: RealSlope` — paraissait la garde la plus solide : elle tient à
la compilation, sans rien à inscrire. Elle ne tient pas, parce que **l'hôte doit pouvoir
construire un `RealSlope`** : le constructeur est public, et un troisième champ écrira
`RealSlope::new(slope)` pour faire compiler sa comparaison fausse. Une bosse, pas un mur, pour une
cinquantaine de sites et une API publique changée.
Retenu : deux gardes exécutables — l'une sur ce que les champs **calculent**, l'autre sur ce que le
crate **contient** — plus l'**invariant I-18** qui dit ce qu'elles protègent. Les deux ont été
**vues échouer** sur les fautes qu'elles gardent, dont celle de S141. Voir
[CONTRAT-PENTE-S143](../validation/CONTRAT-PENTE-S143.md).

**Suivi A208 — S144 : traitée par ADR-098, et aucune de ses deux réparations n'a été prise.**
`Footprint` attribuerait la cause à l'emprise quand le facteur d'alignement dépend aussi du
spectre : la bibliothèque ne peut pas trancher, et ADR-082 refuse un nom qui ment autant qu'un nom
vague. Publier le rapport des deux enveloppes ne dirait que le facteur de **forme**, que S141 avait
déjà retiré — la réparation était périmée par la session qui a suivi son écriture.
Ce qui est fait à la place tient à un constat : **les trois budgets ont la pente réelle au point
sous la main au moment du refus**. Trois causes décidables, trois noms — `MaxSlope`, `Slope`
resserré, `SlopeEnvelope`. Six essais ont changé d'attente ; cinq exerçaient le majorant en
croyant exercer la pente. Voir [REFUS-EMPRISE-S144](../validation/REFUS-EMPRISE-S144.md).

- **A211** *(sévérité 1, S145 ; ouverte)* — **Une recommandation globale n'a aucun porteur dans ce
  dépôt.** Le chaînage « suite Sxxx » n'a jamais rompu en 144 sessions, mais il est **local** : il
  propage ce que la dernière session a vu, pas ce qu'un bilan a conclu. [BILAN-S69](BILAN-S69.md)
  recommandait quatre choses ; **deux sont restées lettre morte pendant soixante-seize sessions**,
  dont « lancer B1 », qui ne demandait aucune couche manquante et tranchait une question de
  justesse ouverte depuis A187. Personne ne les a refusées — personne ne les a relues.
  La sévérité tient à ce que cela touche : non pas un calcul, mais **la direction du projet**. Un
  dépôt qui ne peut pas porter une intention au-delà d'une session choisit sa trajectoire par
  proximité, pas par importance.
  **Réparation appliquée en S145, à éprouver** : la ligne `Session suivante` du jeton porte la
  recommandation — c'est le seul mécanisme que toute session lit à l'amorce — et le rituel de fin
  (`REPRISE.md` §6) gagne un point qui demande de vérifier qu'elle y est, ou qu'elle a été écartée
  **par écrit**. Ce n'est pas un registre de plus : un registre est justement ce que personne ne
  relit. À juger dans quelques sessions : si B1 n'est toujours pas lancé en S150, la réparation
  aura échoué et il faudra autre chose.

**Suivi A211 — S190, 2026-09-12 : réparation étendue à une file plurielle.** L'utilisateur
signale que la ligne unique de suite a de nouveau laissé disparaître les autres axes
pendant la série B4. Pas de nouvel identifiant pour le même défaut de sévérité 1.
La [file active](QUESTIONS-OUVERTES.md#file-active) porte désormais
les chantiers, leurs limites et leurs déclencheurs ; REPRISE §6.7 exige sa relecture
en plus de la prochaine action. A211 reste à éprouver sur les sessions suivantes,
elle n'est pas fermée par la seule écriture du dispositif.

**Suivi A187 — S146 : requalifiée par B1, ce n'était pas un défaut.** Les +6,612 % mesurés à 256
composantes étaient **une réalisation à trois écarts-types sur une graine unique**. Douze graines
par densité montrent qu'il n'y a **aucun biais** — moyenne des écarts dans ±0,42 % à toutes les
densités — et que ce qui croît avec le nombre de composantes est la **dispersion** : écart-type
1,09 point à 32, 2,23 à 256. La cause identifiée en S67 explique exactement cela. **La tolérance de
`Hs` dépend donc de N** : ±3 % couvre 2,7 σ à 32 composantes et 1,3 σ à 256. Voir
[BANC-B1-S146](../validation/BANC-B1-S146.md) et ADR-099.

- **A212** *(sévérité 2, S146 ; partielle S147)* — **La forme du spectre de `B` est grossière, et le
  renvoi qui la couvrait était faux.** `Background::configure` répartit l'énergie **uniformément**
  dans la bande `[Tp/2, 2Tp]` ; le code renvoyait cette grossièreté à B1 — « c'est assumé : B1
  tranchera ». B1 a été exécuté en S146 : son protocole mesure **le nombre de composantes et le
  coût**, jamais la répartition de l'énergie. Aucun banc ne mesure la forme du spectre, et aucune
  décision ne la justifie : un spectre de mer réel suit JONSWAP ou Pierson-Moskowitz, pas une
  répartition uniforme. Le commentaire est corrigé ; la question reste entière. C'est le mécanisme
  de **L217** — un renvoi non vérifié ferme la question au lieu de la laisser ouverte —, cette
  fois dans le code plutôt que dans un ADR.
  *Ce que cela change, ou non : `Hs` est exact par construction quelle que soit la forme, donc
  rien de ce qui est mesuré aujourd'hui n'est faux. Ce qui dépend de la forme est le **contenu
  fréquentiel** — donc l'aspect, les périodes vues par un objet, et la réponse d'un corps flottant.*

**Suivi A212 — S147 : partielle, ADR-100.** Le spectre uniforme est uniforme en
log-fréquence ; Tp n'est pas un pic. Forme JONSWAP et bande explicite décidées, instrument
reçu contre intégrales fermées et raffinements (SPECTRE-FOND-S147). Normaliser Hs masque
la perte des moments dérivés : à gamma=3,3, 2fp retient 95,07 % de m0 mais 75,86 % de m2.
Le constructeur spectral et sa réception restent **S147-1**, prochaine production S148.
Aucun nouvel angle indépendant : cette conséquence relève de la forme déjà suivie par A212.
**Suivi A212 — S148 : partielle.** ADR-101 construit le candidat spectral explicite, cuisson
sans libm et gravité portée par B. Moments/pic, dérivée, pente et impact non nul reçus ; hashes
locaux debug/release identiques. Cycle hôte/transport de recette S148-1, statistiques et choix
des bandes/directions restent ouverts. Voir FOND-SPECTRAL-S148.
**Suivi A212 — S149 : partielle.** Transport WSPR et cycle hôte B+impact reçus,
comparaison directe et hashes debug/release identiques, coûts locaux mesurés
(CYCLE-SPECTRAL-S149). S148-1 close. Restent statistiques multigraines,
bandes/directions du jeu et pression/mixte. Aucun nouvel angle indépendant.

**Suivi A185 — S150 : état daté corrigé.** BILAN-S145 disait le sillage « jamais commencé » ;
ADR-069/070 avaient construit la pression mobile. Le raccordement objet/charge est ajouté
par ADR-103, la construction W4 reste partielle. Aucun nouvel angle indépendant.
**Suivi A212 — S150 : partielle.** B spectral+pression mobile reçu contre une référence
raffinée sur le trajet S150 ; restent statistiques, bandes/directions et montage mixte.

**Suivi S151 — aucun nouvel angle indépendant.** L'émission progressive conserve le
tronçon en attente et n'acquitte que le contenu admis. Les limites de rétention longue
durée (S72-2), de fenêtre16s et de sauvegarde du curseur hôte restent explicites dans
EMISSION-SILLAGE-S151. A212 inchangée et partielle ; pas de nouvelle réception du fond.

**Suivi S152 — B2 partiellement exécuté.** Couverture80m/60s exige N512 pour les sources2–4m.
Le garde de résolution ne certifie pas la précision ; l'oracle indépendant reçoit ensuite le champ.
Énergie globale60s, sillage long et sélection technologique restent ouverts dans BANC-B2-S152.
Aucun nouvel angle indépendant ; A212 inchangée, pas de nouvelle mesure du fond.

**Suivi S153 — énergie hors domaine.** À60s pour source4m, l'anneau80–120m contient
3,516 % de E0 ; le déficit du disque80m n'est pas une dissipation (ENERGIE-B2-S153).
Mécanisme déjà connu S127, aucun angle indépendant ajouté. A212 inchangée.

**Suivi S154 — transport sur la bande de fixtures.** ENERGIE-BANDE-B2-S154 retrouve
l'énergie des quatre autres impacts60s, collecteurs adaptés ; N2565/6 reste reçu
sur80m seulement. Aucun nouvel angle indépendant, A212 inchangée ; B2 reste partiel.

- **A213** *(sévérité 2, S155 ; ouverte)* — **La pulsation est stockée en f32, et l'erreur de
  phase qui en découle croît sans borne avec l'âge.** `ModalPressure` calcule
  `omega = (gravity * magnitude).sqrt()` en f32 ; son erreur relative, mesurée entre 6,6e-9 et
  5,7e-8 selon le mode, se traduit par une dérive de phase `|domega| * t` **linéaire en temps**.
  Mesuré S155 : l'écart au noyau f64 passe de 1,381e-7 m à 16 s à 5,931e-7 m à 60 s, et
  neutraliser la seule pulsation le divise par 58 pour k=(6,0). ADR-106 borne l'horizon à 64 s
  avec ce budget écrit ; **la borne est un budget, pas une correction**, et tout consommateur qui
  demandera mieux, ou plus long, rouvrira la question.
  Remède identifié et non appliqué : convertir la pulsation en Q32 depuis un calcul de précision
  supérieure au f32, ce qui coûte zéro à l'exécution — la conversion a lieu à la préparation — et
  ramènerait la dérive vers 1,5e-10 relatif. Ce n'est pas fait parce que cela change
  l'arithmétique du noyau, donc le condensat de réception `8ea15f4a3334830b` de S95, et que cela
  demande sa propre réception. Voir [[L231]].

- **A214** *(sévérité 2, S156 ; ouverte)* — **Le sillage n'annonce pas son domaine, et la loi
  manque pour qu'il le puisse.** ADR-107 établit que la validité d'un champ de sillage est un
  couple `(rayon, durée)` déduit de la recette : rayon proportionnel à `angular`, durée bornée par
  la périodicité `2π·radial/cutoff`. Mais la durée n'est **encadrée qu'en deux points** — radial
  128 décroche entre 15 et 20 s, radial 256 entre 45 et 50 s — et la formule de récurrence
  candidate se trompe d'un facteur 2,5 sur le second. Le dépôt annonce ailleurs l'admissibilité
  plutôt que de mentir en silence (ADR-091, ADR-107 §« ce qu'elle ne dit pas ») ; ici il ne le peut
  pas, faute de loi. Un garde bâti sur une loi non vérifiée refuserait des configurations valides
  ou en admettrait d'invalides, ce qui est pire que pas de garde du tout.
  Ce qui manque est peu cher : trois ou quatre points de plus par dichotomie sur l'instant de
  décrochage, et la dépendance à `sigma`. La sonde `wake_reach` existe. Suite S156-1.
  Voir [[L233]] et [[L234]].

**Suivi S157 — A214 change de nature, elle ne se ferme pas.** Une session entière de mesure a
établi qu'**il n'y a pas de loi à écrire** dans la fenêtre accessible : la dégradation est
graduelle, l'instant limite hérite de la tolérance choisie — 16 / 36 / 56 s à 10 % d'excès contre
32 / 46 / au-delà de 64 s à 100 % — et aucun groupement ne rassemble les mesures à mieux qu'un
facteur 2,5. Il ne manque donc plus une mesure mais une **spécification** de l'erreur acceptable,
et c'est le seul des trois manques qui ne coûte rien.
Les deux autres sont fermés par des décisions prises pour d'autres raisons : la fenêtre de 64 s
d'ADR-106 censure la source large, le plafond de 512 d'ADR-097 limite le bras de levier à deux
doublements. **Deux bornes décidées séparément se conjuguent ici pour fermer une question**, et
aucune des deux décisions ne pouvait le prévoir. Voir ADR-108, [[L235]], [[L236]].

**Suivi S158 — A214 change de dépendance et de gravité apparente.** L'inventaire des consommateurs
montre que le repliement est une **infidélité, pas une faute** : l'erreur est déterministe et
identique chez tous, donc ni désynchronisation, ni divergence de réplique, ni inégalité ; I-15
reste satisfait avec l'erreur dedans (ADR-109, [[L239]]). Et les consommateurs qui lisent une
**borne** — déclencheur d'écume par `slope_envelope`, budget de pente, admissibilité — y sont
insensibles : 6,2e-3 et 4,2e-4 d'écart quand le champ échantillonné se trompe d'un facteur 48
([[L238]], test `bornes_insensibles_au_repliement_s158`).
A214 ne réclame donc plus une mesure ni une spécification que nous pourrions écrire : elle attend
**B4**, seul juge de fidélité du corpus, lui-même bloqué par la référence substitutive intégrale.
Recours d'ici là : rester dans le domaine déduit de la recette (ADR-107), ou estimer l'erreur en
comparant `radial` et `radial+1` — fidèle à un facteur 2,5, et sous-estimant.

- **A215** *(sévérité 2, S159 ; ouverte)* — **Rien n'empêche une copie de travail de se recréer.**
  S159 a ramené six copies à trois et écrit la procédure de fermeture dans `AGENTS.md` (ADR-110),
  mais la copie `project-status-progress-d31d78` était apparue **pendant S158** sans annonce :
  l'outillage crée des worktrees de son propre chef, et une procédure ne contraint que l'agent qui
  lit l'amorce. Le jeton restant un fichier **versionné**, chaque copie nouvelle en portera un, et
  une copie en retard portera un jeton périmé qu'une session y trouvera `libre` — le mécanisme des
  trois forks (L137), intact.
  Ce qui manque n'est pas une décision de plus mais un **dispositif qui ne dépende pas de la bonne
  volonté** : un jeton non versionné cesserait de voyager avec l'histoire, ce qui est précisément
  sa vertu ; un contrôle à l'amorce qui refuse de travailler dans une copie en retard serait
  possible, et personne ne l'a spécifié. Voir ADR-110 §« ce qu'elle ne dit pas », [[L240]].

- **A216** *(sévérité 3, S161 ; ouverte)* — **Le coefficient d'additivité change de valeur à très
  faible amplitude de fond, et rien ne l'explique.** `écart ≈ k · max|δ|/h` avec `k = 0,24` pour
  `A_B/h ≥ 0,10`, mais **`k = 0,95`** à `A_B/h = 0,02` — quatre fois plus. La proportionnalité à
  `max|δ|` tient dans les deux régimes, vérifiée sur cinq décades, donc ce n'est ni un plancher
  d'intégration ni un artefact de pas de temps (contrôle à pas imposé : 0 à 2 %). Seule la
  constante change. Mesuré, pas compris. Peu coûteux à instruire — la sonde `additivite_b4.rs`
  existe et le balayage tient en une étape.

- **A217** *(sévérité 2, S161 ; **close S194 par ADR-123** — voir le suivi daté en fin de fichier)* — **En eau profonde, on ne sait pas quelle variable
  gouverne la validité de l'addition.** Le premier volet de B4 (ADR-111) l'établit en régime peu
  profond : c'est `max|δ|/h`, et non le rapport à `Hs` qu'ADR-001 proposait. Mais `B` est une houle
  **dispersive en eau profonde**, où la profondeur ne joue plus : la variable y serait
  vraisemblablement la cambrure, et rien ne le vérifie. Le blocage n'est pas budgétaire mais
  structurel — la référence disponible, `shallow.rs`, est non dispersive par construction, et le
  dépôt n'a **aucun** solveur à la fois non linéaire et dispersif. Or un solveur linéaire ne peut
  pas servir : la superposition y est vraie par construction. **Instruire d'abord si une telle
  référence est à portée**, avant de supposer qu'elle ne l'est pas — c'est exactement ce que S161 a
  gagné en ouvrant un blocage hérité (**L243**).

**Suivi A217 — S162, partielle :** référence analytique de Stokes d'ordre deux instrumentée,
terme croisé `kab`, dépendance à `ka` à rapport fixé et effet de phase reçus. Aucun solveur
évolutif non linéaire dispersif disponible. Voir ADDITIVITE-PROFONDE-S162 et ADR-112.

- **A218** *(sévérité 1, S162 ; ouverte)* — **La sonde d'additivité ne calcule pas le couplage
  dont elle a décidé le critère.** S161 compare `F(A)+F(B)` à `F(A+B)` ; SPEC-004 §6.1 prévoit
  une équation du résidu avec termes croisés et source du fond. L'absence de superposition des
  solutions autonomes ne reçoit ni ne réfute cette équation. ADR-112 corrige la portée d'ADR-111,
  sans effacer les mesures. Action S162-1 : intégrer réellement le résidu, puis comparer au
  total avec témoin de couplage supprimé. Gravité liée à la décision architecturale tirée du
  diagnostic ; aucun défaut de production nouveau observé.
**Suivi A218 — S163 : traitée sur véhicule 1D.** `residu_couple` intègre réellement le résidu
avec ses flux croisés et reçoit la reconstruction à chaque pas contre Shallow1D. Contre-épreuves
pression croisée, viscosité numérique et source du fond figé reçues. Ce n'est pas le couplage 3D
ni B4 complet. **A50 partiellement exercée** ; suite sous A219. Voir RESIDU-COUPLE-S163.

- **A219** *(sévérité 2, S163 ; ouverte)* — **Un fond analytique réévalué en temps n'est pas un
  fond avancé par le même intégrateur que le résidu.** S163 reçoit deux cas où cette différence
  ne se pose pas : fond évolué par RK2, et fond constant en temps. Avec `Q(t)` prescrit, employer
  `Q_t` continu dans la source n'assure pas que les deux étages reproduisent exactement
  `Q(t+dt)-Q(t)`. Le défaut temporel peut alors être attribué à tort au couplage spatial.
  S163-1 : mesurer et recevoir cette clôture, avec une onde analytique et une contre-épreuve.
  Limite nommée du véhicule, aucun défaut observé en production.
**Suivi A219 — S164 : traitée sur véhicule RK2.** Les incréments du fond aux deux étages
retrouvent la référence au pas donné ; la dérivée continue introduit un écart d'ordre deux,
alors que l'omission ne converge pas. Voir FOND-PRESCRIT-S164. A50 reste partielle.

- **A220** *(sévérité 2, S164 ; ouverte)* — **Le couplage reçu occupe le domaine entier.**
  S163/S164 intègrent le résidu sur tout le canal avec les mêmes murs que le total. Aucun
  échange à la frontière d'un domaine local n'est reçu, alors que δ est local par définition.
  Une source exacte dans l'intérieur n'empêche pas une condition de bord erronée de dégrader
  le champ. S164-1 : fenêtre interne, frontière fond seul confrontée à un témoin oracle total,
  avec traversée effective du bord par une perturbation. Ce témoin diagnostique n'est pas une
  alimentation de production. Limite de réception, aucun défaut runtime constaté.

**Suivi A220 — S165 : traitée sur véhicule local 1D.** Fenêtre [30,90] dans le canal de
120 m ; oracle aux étages RK2 reçu à <=1,12e-13 normalisé. Le bord fond seul échoue au
critère d'identité malgré un bilan de flux à l'arrondi ; retard d'étage convergent distingué.
La bosse traverse effectivement le bord. Voir FRONTIERE-LOCALE-S165 ; A50 reste partielle.

- **A221** *(sévérité 2, S165 ; ouverte)* — **La frontière reçue dépend d'une information
  extérieure indisponible au domaine local.** L'oracle S165 fournit le résidu extérieur
  aux deux étages RK2. Le remplacer par zéro donne peu d'erreur sur une bosse sortante
  en fond uniforme, mais injecte une anomalie quand le résidu extérieur compense le fond
  prescrit. Une fermeture sans oracle doit distinguer entrée et sortie et déclarer quelle
  information entrante elle suppose. S165-1 : extrapolation et fermeture caractéristique,
  cas sortant puis fond entrant ; aucun engagement à reconstruire un résidu extérieur
  arbitraire inconnu. Instrument 1D, aucun défaut de production constaté.

**Suivi A221 — S166 : traitée sur véhicule subcritique 1D, entrée connue.** Invariant
entrant fourni par Q, sortant par l'intérieur de chaque étage. Entrée/sortie dans les
deux directions reçues ; état supercritique refusé. Voir BORD-AUTONOME-S166. Aucune
promesse sur un extérieur résiduel inconnu ou sur le solveur 3D.

- **A222** *(sévérité 2, S166 ; ouverte)* — **Retrouver le solveur total transmet aussi
  ses défauts au fond analytique.** Sur l'onde simple exacte prescrite, d initial nul,
  le témoin analytique aux bords perd 12,8 % de hauteur de crête à N240 vers 12 s ; la
  fermeture caractéristique donne la même perte. L'erreur de frontière est secondaire.
  La source discrète reçue S164 impose l'identité au schéma total, pas la préservation de Q.
  S166-1 : distinguer défaut physique et résidu numérique du fond, recevoir un candidat
  préservant le fond exact puis une perturbation. Aucun changement de source runtime ici.

**Suivi A222 — S167 : traitée sur véhicule à source connue.** Flux résiduel seul et source
physique explicite : Q exact reste intact, d ajouté évolue et converge, Q figé exige sa
source. Le témoin S164 conserve sa réception d'identité. Voir FOND-PRESERVE-S167 ; pas
d'adoption runtime, A50 reste partielle.

- **A223** *(sévérité 2, S167 ; ouverte)* — **Un fond ponctuellement exact ne ferme pas
  le volume total discret.** Le budget du résidu ferme à l'arrondi, mais le flux corrigé
  delta numérique + fond physique laisse 5,20e-7 de défaut relatif au volume initial à
  N240. Le défaut décroît comme dx² avec dt proportionnel à dx ; les cellules portent
  des échantillons de Q, pas ses moyennes, et le flux temporel est une quadrature RK2.
  S167-1 : moyennes et flux intégrés cohérents, fond exact/perturbation et montage
  asymétrique. Ni perte de volume de production ni seuil perceptuel inféré.

**Suivi A223 — S168 : traitée sur véhicule à fond connu.** Moyennes spatiales et flux
physiques temporels intégrés indépendamment : bilan du volume <=2,14e-15 relatif.
Q exact intact, perturbation convergente, source moyenne du fond figé asymétrique
reçue avec flux net non nul. Voir VOLUME-MOYEN-S168 ; pas d'adoption runtime.

- **A224** *(sévérité 2, S168 ; ouverte)* — **Le bord autonome et le résidu équilibré
  conservatif n'ont été reçus que séparément.** S168 fournit toujours des fantômes
  issus de T exact ; S166 utilisait l'ancien couplage et des états ponctuels. Transférer
  un invariant total entre deux positions ne garantit pas la préservation de Q variable.
  S168-1 : dériver et recevoir l'assemblage, d nul puis entrée connue et sortie,
  bilan fondé sur le flux effectivement utilisé. Limite de réception, pas défaut observé
  en production ; aucune information extérieure résiduelle inconnue supposée disponible.

**Suivi A224 — S169 : traitée sur véhicule subcritique1D à fond exact connu.** Transfert
d'écart d'invariant sortant, réancré sur Q fantôme : Q intact, sortie reçue et volume
fermé sur les flux réels à<=1,90e-15. Voir ASSEMBLAGE-AUTONOME-S169. Ancien transfert
total produit un résidu sur Q variable ; A50 reste partielle.

**Suivi A50 — S169 : source grossière encore non reçue.** Après les contrôles S163–S169,
S169-1 priorise le réseau décimé de SPEC-004 §6.2 : source exacte, interpolée et omise,
fond figé asymétrique, résolutions du solveur et de la source variées séparément.

**Suivi A50 — S170 : décimation spatiale de S reçue sur véhicule, A50 reste partielle.**
SOURCE-DECIMEE-S170 :48 évolutions, source exacte/interpolée/omise, solveur et réseau
indépendants. Pas de seuil is_smooth_at ni réception de l'interpolation conjointe du fond.

- **A225** *(sévérité2, S170 ; ouverte)* — **Interpoler directement une source conservative
  peut introduire une injection nette artificielle.** Le défaut de volume égale l'intégrale
  temporelle de l'erreur de source, reçue à l'arrondi ; son signe change avec le décalage
  du réseau. Raffiner le solveur à réseau fixé n'y change rien. S170-1 : source par
  différence de flux reconstruit aux faces, sans correction globale, mesurer intégrale
  et précision locale séparément. Limite de véhicule1D, aucun défaut runtime observé.

**Suivi A225 — S171 : traitée sur véhicule1D à flux de bord connus.**
[SOURCE-FLUX-PARTAGES-S171](../validation/SOURCE-FLUX-PARTAGES-S171.md) : flux partagés
bruts ou ancrés aux bornes exactes,128 évolutions ; ancrés, volume fermé à<=2,32e-15
mais erreur de hauteur jusqu’à0,844 àN240/H16/phase0. La télescopie seule conserve
les erreurs aux bornes ; aucune correction uniforme de source. Pas de nouveau runtime.

**Suivi A50 — S171 : toujours partielle.** S170-1 réalisée ; S171-1 étend au fond Q
reconstruit avec source cohérente, même état total initial. Flux de bord exacts encore
supposés disponibles. A216/A217 inchangées.

**Suivi A50 — S172 : reconstruction conjointe reçue sur véhicule figé, reste partielle.**
[FOND-RECONSTRUIT-S172](../validation/FOND-RECONSTRUIT-S172.md) : même total initial,
Q reconstruit et source physique cohérente, défaut Lphys(Q)-Lnum(Q) mesuré en maximum
et norme intégrée. Le témoin S=Lnum(Q) retrouve le solveur total à<=2,23e-16 mais ne
prouve pas la préservation du fond mobile. S171-1 réalisée ; S172-1 reçoit cette étape.
A225 reste traitée à flux de bord connus ; A216/A217 inchangées. Aucun nouvel angle.

**Suivi A50 — S173 : fond mobile et source cohérente reçus sur véhicule connu.**
[FOND-MOBILE-S173](../validation/FOND-MOBILE-S173.md) : S172-1 réalisée,160 évolutions,
source intégrée préservant Q exact ; volume à<=2,42e-15. Le témoin discret retrouve
le solveur total mais diffuse Q ; omission sur Q exact seul est un témoin insuffisant.
A50 partielle : accès temporel analytique continu encore nécessaire, cadence grossière
suivie S173-1. A225 reste traitée, A216/A217 inchangées ; aucun nouvel angle.

**Suivi A50/A225 — S174 : cadence temporelle reçue sur véhicule à instantanés connus.**
[CADENCE-FOND-S174](../validation/CADENCE-FOND-S174.md) :192 évolutions, budget interpolé
fermé à<=1,21e-15, défaut analytique prédit et persistant au raffinement du solveur.
A225 décrit aussi cette erreur temporelle de flux aux bornes ; pas de nouvel angle.
S173-1 réalisée, A50 partielle ; S174-1 assemble la frontière autonome S169.
Instantané suivant et fantômes analytiques encore connus dans le véhicule ; aucune
anticipation d’événement inconnu reçue. A216/A217 inchangées.

**Suivi A50 — S175 : assemblage autonome avec fond décimé reçu sur véhicule.**
[FRONTIERE-FOND-DECIME-S175](../validation/FRONTIERE-FOND-DECIME-S175.md) : S174-1 réalisée,
288 évolutions, sortie de crête effective, budgets et identité discrète àbord identique.
A50 partielle : pas de3D, forces/perception ni coût runtime reçus. A225 reste qualifiée,
A216/A217 inchangées. Pas de nouvel angle ; S175-1 consolide la réception B4 avant
le prochain lot de construction.

**Suivi A50 — S176 : bilan consolidé, reste partielle.**
[BILAN-B4-S176](../validation/BILAN-B4-S176.md) sépare les contrôles1D de B4 complet.
Le fournisseur différentiel BackgroundSample manque encore dans la bibliothèque ;
S176-1 le construit pour B seul, sans prétendre fournir B+W ni choisir le solveur3D.
Pas de nouvel angle : c’est le maillon concret de l’angle existant.

**Suivi A50 — S177 : premier fournisseur B construit dans la bibliothèque.**
[FOURNISSEUR-B-S177](../validation/FOURNISSEUR-B-S177.md), ADR-113 : eta, grad_eta, u,
du_dt, grad_u et pression de vague àprofondeur connue, type distinct de WaterSample.
B seul, linéaire profond uniforme ; A50 reste partielle. S177-1 reçoit gradient de
pression et source continue avant extension W. Aucun nouvel angle, ni seuil B4 adopté.

**Suivi A50 — S178 : source volumique continue de B construite.**
[SOURCE-B-S178](../validation/SOURCE-B-S178.md), ADR-114 : gradient de pression,
Laplacien et contraction après sommation des modes ; S à soustraire. Six nouveaux
tests et contre-épreuve de l'advection omise. A50 partielle : source B+W, conditions
de surface et δ3D restent ouverts. S178-1 construit le différentiel RadialImpact.

**Suivi A50 — S179 : différentiel radial et composition B+un impact reçus.**
[DIFFERENTIEL-W-S179](../validation/DIFFERENTIEL-W-S179.md), ADR-115 : gradient
régulier au centre, pression profonde, contraction après composition et lot atomique.
Six nouveaux tests ; A50 partielle, pression forcée/multisource et δ3D non reçus.
S179-1 construit le fournisseur de pression forcée. Aucun nouvel angle.

**Suivi A50 — S180 : différentiel profond de pression forcée reçu.**
[DIFFERENTIEL-PRESSION-S180](../validation/DIFFERENTIEL-PRESSION-S180.md), ADR-116 :
pression imposée dans l'accélération et la pression profonde, commutations reçues.
Cinq nouveaux tests ; A50 reste partielle (gravité existante inchangée). Aucun nouvel
angle. S180-1 compose B+impacts+pressions ; monde, cycle vivant, coût et δ3D restent ouverts.

**Suivi A50 — S181 : composition différentielle mixte sur vues publiées reçue.**
[COMPOSITION-DIFFERENTIELLE-S181](../validation/COMPOSITION-DIFFERENTIELLE-S181.md),
ADR-117 : entrée WorldPos, contexte/instant partagés, source après somme et pression
imposée comptée une fois. Quatre nouveaux tests, termes croisés et refus atomiques.
A50 reste partielle, aucun nouvel angle ; S181-1 reçoit le cycle vivant du consommateur.

**Suivi A50 — S182 : cycle vivant du consommateur différentiel reçu.**
[CYCLE-DIFFERENTIEL-S182](../validation/CYCLE-DIFFERENTIEL-S182.md) : actualisation,
admission, saturation/reprise, renouvellement et restauration identiques àla préparation
directe, dérivées/source comprises. Trois nouveaux tests, aucun nouvel angle.
A50/B4 partiels ; S182-1 mesure coût et allocations avant budget de consommation.

**Suivi A50 — S183 : coût et allocations du consommateur différentiel mesurés, reste partielle.**
[COUT-DIFFERENTIEL-S183](../validation/COUT-DIFFERENTIEL-S183.md) : rapport différentiel/surface
**3,0 à 4,3** (médiane ~3,4), **identique couche par couche** (B 3,3 ; impact 3,6 ; pression 3,4),
donc porté par les 31 scalaires publiés contre 10 et non par la nature du calcul dérivé.
Préparation et actualisation inchangées entre les deux chemins ; zéro allocation d'hôte après
`seal()` sur six montages. A50 reste partielle : le solveur perturbatif qui consommerait
`momentum_residual` n'existe pas, et produire la source n'est pas s'en servir. S183-1 mesure
la consommation.

- **A226** *(sévérité 2, S183 ; ouverte)* — **Un refus porté par un point fait payer le lot
  entier, et le contrôle le moins cher est évalué en dernier.** Le lot est atomique (ADR-063) :
  sur refus, rien n'est publié. Mais le refus survient **pendant** la boucle, après que tous les
  points précédents ont été calculés. Mesuré : un lot de 64 dont le dernier point sort du domaine
  coûte 2232–2399 µs, soit **95 %** du même lot réussi, pour zéro sortie ; le même point placé
  en tête coûte 2,0 µs — rapport **1150**. Et ces 2,0 µs ne sont pas le plancher : un point hors
  du rayon d'un impact paie d'abord `differential_local` de B en entier (2,0 µs à 16 composantes,
  **7,6–8,4 µs à 64**), parce que le test géométrique du domaine d'impact vient après. Les refus
  indépendants des points, eux, sont gratuits (0,025–0,097 µs) et refusent avant tout calcul :
  c'est le contraste qui rend l'asymétrie visible. **Ce n'est pas un défaut de correction** —
  aucune valeur n'est fausse, aucune publication n'est altérée — mais un hôte qui interroge des
  points dont il n'a pas garanti l'appartenance au domaine paie le prix plein d'un lot pour rien,
  et le paie d'autant plus que le montage est gros. Deux voies, non tranchées : une passe
  géométrique préalable sur tous les points, ou la remontée des tests de domaine les moins chers
  avant l'évaluation de B. La seconde ne couvre pas tous les refus par point — `NonFinite` ne se
  prévoit pas — donc aucune des deux ne supprime le cas ; elles en réduisent la fréquence.
  Chiffrage et conditions dans COUT-DIFFERENTIEL-S183 §6.5.

**Suivi A50 — S184 : la consommation est mesurée, et elle déplace la question.**
[CONSOMMATION-S184](../validation/CONSOMMATION-S184.md) : la source coûte **~2100 fois** le pas
explicite qu'elle alimente (34–35 µs contre 14–17 ns par maille). La décimation spatiale achète
exactement le rapport des nœuds mais le contenu la plafonne à `r = 2` (λ_min = 1,081 m, coupure
de pression) ; la cadence divise exactement par `c` et le contenu temporel est lent. A50 n'attend
plus un chiffre mais une **décision** de cadence et de réseau ; l'erreur des deux axes n'est
mesurée qu'en 1D (S170, S174). S184-1 mesure l'erreur de cadence en 3D avec le fournisseur réel.

- **A227** *(sévérité 2, S184 ; ouverte)* — **Le fournisseur différentiel a la forme d'une
  requête, pas celle d'un champ, et sa forme ne se corrige pas par la traversée.** Sept sessions
  (S177–S183) ont construit un fournisseur **par point**, en supposant implicitement qu'un
  solveur perturbatif le consommerait maille par maille. Mesuré : il coûte 2100 fois le pas
  qu'il alimente. L'optimisation évidente — amortir les sommes modales sur un réseau régulier
  par récurrence de phase — a été chiffrée et **ne rend que 12 à 15 %** : la trigonométrie ne
  pèse que 15–18 % du différentiel de B, le reste étant l'arithmétique qui produit les 26
  scalaires de `BackgroundSample` par composante (140 ns par composante et par nœud, dont 21 de
  phase). **Le coût est le volume de sortie, pas la manière d'y arriver** ; changer le parcours
  ne changera pas l'ordre de grandeur. Ce qui reste disponible : la cadence temporelle (exacte
  en `1/c`, et le contenu temporel est lent), la décimation spatiale jusqu'à `r = 2` seulement
  (au-delà, le contenu de pression est replié), et la vectorisation — non mesurée, et le seul
  levier qui s'attaque aux 85 %. **Ce n'est pas un défaut de correction** : les valeurs sont
  justes, les contrats tenus, et le fournisseur reste bien dimensionné pour ce qu'il sert
  aujourd'hui — des consommateurs **épars** : flottabilité, véhicules, quelques sondes. C'est
  l'usage volumétrique qui n'est pas dans son enveloppe, et personne ne l'avait écrit.
  Sévérité 2 et non 1 : la décomposition B/W/δ/V (ADR-001) n'est pas en cause, c'est la manière
  dont δ consomme B+W qui l'est. Chiffres et conditions dans CONSOMMATION-S184 §6.2 et §6.4.

**Suivi A50 — S185 : l'erreur de cadence est mesurée en 3D, et les trois modes séparés.**
[CADENCE-3D-S185](../validation/CADENCE-3D-S185.md) : maintien `0,35·(τ/T)`, extrapolation
`0,35·(τ/T)²`, interpolation `0,05·(τ/T)²`, avec `T` la plus courte période du contenu
(0,5405 s ici). Une période de latence vaut `√7 ≈ 2,6` sur la cadence à erreur égale.
Combinée à `r = 2`, une cadence `τ ≈ 0,3·T` place la source à ~26 fois le pas pour 3,2 %
d'erreur en extrapolation, 0,46 % en interpolation. A50 n'attend plus d'ordre de grandeur :
il lui manque **un critère de justesse** et un solveur. S185-1 compose les deux erreurs.

- **A228** *(sévérité 2, S185 ; ouverte)* — **Réemployer une source en la maintenant
  constante est d'ordre un, et rien dans le corpus ne le disait.** Toutes les études de
  cadence du dépôt — S174 en tête — ont mesuré le régime **interpolé**, parce que le véhicule
  analytique connaissait l'instantané suivant. S174 l'a écrit honnêtement, et l'écart est
  resté ouvert onze sessions. Mesuré en 3D sur le fournisseur réel : le **maintien**, seul
  mode qui ne demande rien au runtime, est d'ordre **un** en `τ/T` quand les deux autres sont
  d'ordre **deux**. À `τ/T = 0,3` cela fait 10,4 % contre 3,2 % et 0,46 %. Un système qui
  réemploierait naïvement la dernière source publiée paierait donc un ordre entier, et le
  corpus ne contenait aucun chiffre pour l'en dissuader. **Le correctif est disponible et
  presque gratuit** : l'extrapolation causale à partir des deux dernières reconstructions
  coûte 1,9 ns par maille et un instantané de plus (32 ko par bloc de 2744 mailles), et gagne
  un facteur `T/τ`. Ce qui reste ouvert n'est donc pas quoi faire mais **ce qui l'exige** :
  aucun critère ne dit si 3 %, 1 % ou 0,1 % d'erreur de champ perturbatif est acceptable, et
  la latence que l'interpolation réclamerait n'a pas de prix connu ailleurs dans le système.
  Sévérité 2 : aucune valeur publiée n'est fausse et aucun contrat ne change ; c'est un choix
  d'intégration qui, pris par défaut, coûterait un ordre de grandeur de justesse.
  Chiffres et conditions dans CADENCE-3D-S185 §6.2 et §6.3.

**Suivi A50 — S186 : les deux erreurs sont composées, et la loi dépend du mode de réemploi.**
[COMPOSITION-ERREURS-S186](../validation/COMPOSITION-ERREURS-S186.md) : sur la grille
`r × mode × c`, la composition suit le **maximum** pour les deux modes causaux (maintien
0,826–1,155 ; extrapolation 0,860–1,034) et la **quadratique** pour l'interpolation
(0,991–1,209). L'additive n'est dépassée sur aucune des 84 cases : c'est une enveloppe sûre,
avec jusqu'à 1,9 fois de mou. **Un budget conjoint `r × c` est donc licite pour un
consommateur causal**, et la règle de dimensionnement est d'égaliser les erreurs des deux axes
pris seuls puis de s'arrêter. A50 n'attend toujours que son **critère de justesse** : la
composition est connue, le seuil ne l'est pas.

- **A229** *(sévérité 2, S186 ; ouverte)* — **Dégrader un axe peut réduire l'erreur totale, et
  un réglage à un axe à la fois trouve alors un optimum faux.** Sur un réseau décimé, réduire
  la cadence de reconstruction rend le champ **plus juste** : maintien `r = 4` passe de 13,60 %
  à `c = 1` à **11,23 %** à `c = 8`, soit −17,4 % ; maintien `r = 2` −13,2 % à `c = 2` ;
  extrapolation `r = 2` −14,1 % et `r = 4` −12,2 %. Les deux erreurs se compensent
  partiellement, et la compensation appartient aux **modes causaux** — elle disparaît avec
  l'interpolation (−0,04 %), dont l'erreur temporelle est sept fois plus petite. Elle est déjà
  visible dans l'erreur de **source** seule, donc ce n'est pas un artefact de l'évolution du
  champ. **Le piège :** une procédure de calibration qui balaie `c` en tenant `r` fixe verra
  l'erreur baisser, conclura que la cadence grossière est meilleure, et aura seulement trouvé
  l'endroit où deux défauts s'annulent le mieux. Cet endroit dépend du contenu, du mode et de
  la métrique ; il ne se transporte pas, et **il ne doit pas être dépensé comme une marge**.
  Ce qui reste ouvert : aucune procédure de calibration n'est écrite dans le corpus, et la
  première qui le sera aura ce piège devant elle. Sévérité 2 : aucune valeur publiée n'est
  fausse, mais un réglage pris par cette voie serait faux et paraîtrait bon.
  Chiffres et conditions dans COMPOSITION-ERREURS-S186 §8.6.

- **A230** *(sévérité 2, S186 ; ouverte)* — **« Points par longueur d'onde » n'est pas un
  critère valide pour une source 3D échantillonnée en profondeur : la longueur d'onde qui
  compte n'est pas celle de la recette, mais celle qui survit à la profondeur du
  consommateur.** S184 §5 bornait la décimation à `r = 2` en comptant 2,16 points par
  `λ_min = 1,081 m`, la coupure de la recette de pression. Mesuré sur la source réelle au
  bloc, le contenu présent vaut `k_eff` de 0,37 à 0,79 rad/m horizontalement et 0,50 à
  1,17 verticalement, soit `λ_eff` de **8 à 17 m** et de **5,4 à 12,7 m** : le contenu est
  **5 à 16 fois plus lisse** que la coupure, parce qu'un mode profond décroît en `exp(k z)` et
  que les modes courts sont morts avant d'atteindre `z = −0,80 m`. Le critère de S184 donnait
  la bonne réponse — `r = 2` reste le bon choix ici — **par le mauvais chemin**, et un chemin
  faux se trompe ailleurs : près de la surface il serait optimiste, et plus profond encore plus
  pessimiste. Corollaire inverse, et c'est lui qui coûte : la même mesure montre que **l'erreur
  globale est exactement celle de la tranche la plus haute** du bloc (2,54 / 13,60 / 32,96 %
  aux trois `r`, quand la tranche du fond ne vaut que 0,11 / 0,43 / 1,54 %). Un réseau isotrope
  surrésout treize tranches sur quatorze. Ce qui reste ouvert : le critère correct porte sur
  `k_eff(z)` du consommateur, il n'est écrit nulle part, et aucun réseau du dépôt ne sait
  graduer son pas. Sévérité 2 : la borne `r = 2` publiée n'est pas fausse, c'est sa
  justification qui l'est, et elle circule depuis S184.
  Chiffres et conditions dans COMPOSITION-ERREURS-S186 §8.2 et §8.3.

**Suivi A50 — S187 : le réseau d'échantillonnage est ancré et gradué, ADR-118.**
[RESEAU-GRADUE-S187](../validation/RESEAU-GRADUE-S187.md) : l'axe vertical domine
l'horizontal d'un facteur 1,6 à 3,4, mais le levier n'est pas celui qu'on cherchait.
**Ancrer le dernier nœud sur la frontière du domaine vaut jusqu'à un facteur 6 et ne coûte
aucun nœud** ; la graduation dérivée vaut 1,4 par-dessus, et bat les deux témoins naïfs.
Gains à erreur égale : −37,5 % de nœuds contre l'isotrope `r = 2` (et −29 % d'erreur en
même temps), −78,4 % contre `r = 4`, erreur divisée par 2,44 à nœuds identiques contre
`r = 8`. Raffiner un axe **sature** sur l'axe le plus grossier : six nœuds verticaux
suffisent à pas horizontal 2, quatre à 4, trois à 8. A50 attend toujours **un critère de
justesse** : ADR-118 dit où poser les nœuds, pas combien en payer.

**Suivi A229 — S187 : la compensation existe aussi entre les deux axes d'espace.** A229
avait été écrite pour la paire espace/temps. Mesurée entre `z` et `x,y` : l'erreur
isotrope est **sous** l'erreur de l'axe vertical seul aux trois ratios — 2,54 contre
2,66 %, 13,60 contre 15,41 %, 32,96 contre 45,51 %. Décimer **aussi** horizontalement
rend donc le champ plus juste que décimer verticalement seul. Le piège de réglage est le
même, mais il est ici **interne à une seule grandeur** : un balayage qui raffine un axe
d'espace en tenant l'autre peut voir l'erreur monter. Voir RESEAU-GRADUE-S187 §8.2.

- **A231** *(sévérité 2, S187 ; ouverte)* — **Le réseau d'échantillonnage du dépôt posait
  son dernier nœud hors du domaine mesuré, et cela coûtait jusqu'à un facteur six.**
  `nodes_per_axis` déborde par construction : à `r = 8` sur un bloc de côté 16, le dernier
  nœud tombe à l'indice 17 quand les mailles intérieures s'arrêtent à 14. La maille du
  haut — celle qui, d'après S186 §8.3, porte **intégralement** le maximum — était donc
  interpolée sur une portée de 2 m au lieu d'être échantillonnée. À nombre de nœuds
  verticaux égal : **41,2 % contre 6,8 %** à trois nœuds, **13,6 % contre 2,5 %** à cinq,
  **2,54 % contre 1,69 %** à huit. L'ancrage est **gratuit** — il ne change pas un nœud,
  seulement l'endroit où on le pose — et il vaut cinq fois plus que la graduation que
  cette session cherchait. Conséquence sur le corpus : **toutes les erreurs spatiales
  publiées par S186 le sont pour un réseau inutilement mauvais**, et les raisonnements qui
  s'appuyaient sur leur *magnitude* — la borne `r = 2`, la parité entre axe spatial et axe
  temporel — doivent être relus. Ce qui reste ouvert : la loi de composition de S186, le
  maximum pour les modes causaux, a été établie sur ce réseau débordant et sur une erreur
  **concentrée** sur une tranche ; la graduation la **répartit** (§8.5, erreur de tranche
  haute à zéro), et rien ne dit que la loi survit à cette redistribution. Sévérité 2 et
  non 1 : aucune valeur publiée n'est fausse — elles mesurent correctement ce réseau-là —
  aucun invariant ne tombe, et le code de bibliothèque n'est pas en cause. C'est
  l'interprétation de magnitudes de conception qui l'est. Corrigé par
  [ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md).
  Chiffres et conditions dans RESEAU-GRADUE-S187 §8.4.

**Suivi A50 — S188 : la loi de composition survit à l'ancrage, et l'on sait maintenant
pourquoi.** [COMPOSITION-ANCREE-S188](../validation/COMPOSITION-ANCREE-S188.md) rejoue la
grille de S186 sur un réseau ancré (ADR-118), à nombre de nœuds identique. Verdict
**inchangé mode par mode** — maximum pour les deux modes causaux, additive et quadratique
pour l'interpolation — et **mieux satisfait** : 0,895–1,060 contre 0,826–1,155 pour le
maintien. Les magnitudes, elles, changent jusqu'à **3,7 fois**, donc le point de parité
entre les deux axes passe de `c ≈ 20` à `c ≈ 6` à 125 nœuds : l'optimum va vers **plus** de
décimation spatiale et **moins** de réduction de cadence. Conversion mesurée : **27 nœuds
ancrés valent 125 nœuds débordants** à erreur égale. La limite déclarée par ADR-118 — la loi
non rejouée sur un réseau ancré — est **levée**. A50 attend toujours son seul manque : un
**critère de justesse**.

- **A232** *(sévérité 2, S188 ; ouverte)* — **La loi de composition en norme maximum n'est
  valide que tant que les maxima des deux erreurs coïncident, et rien dans le corpus ne le
  disait.** S186 avait conclu que l'erreur spatiale et l'erreur temporelle se composent
  selon le **maximum** pour les modes causaux ; S188 confirme le verdict et en trouve la
  raison : la tranche qui porte le maximum est la même — la plus haute du bloc — pour l'axe
  spatial seul, pour l'axe temporel seul et pour les 84 cases composées (**39 cases jugées
  sur 39**). Ce n'est pas une propriété de la composition, c'est une propriété du
  **contenu** : `|S|` culmine en haut parce qu'un mode profond décroît en `exp(k z)`, donc
  `|u'|` y culmine, donc tout écart relatif y culmine. Deux erreurs qui culminent au même
  endroit s'y rencontrent, et la plus grande gagne — d'où le maximum. **Ce qui n'est pas
  couvert** : un contenu dont la source culminerait au milieu du domaine, ou une
  configuration qui séparerait les deux maxima. Le dépôt en connaît déjà une, et elle n'est
  pas mesurée : le réseau **gradué** de S187, dont §8.5 relève une erreur de tranche haute
  **nulle** — le maximum spatial se déplace vers le milieu du bloc tandis que le maximum
  temporel reste accroché à celui du champ, en haut. Si la loi tombe là, la règle de
  dimensionnement d'ADR-118 devra dire **sur quel réseau** elle s'applique. Sévérité 2 :
  aucune valeur publiée n'est fausse, aucun contrat ne change, et la loi est vérifiée sur
  les deux réseaux mesurés ; c'est son domaine de validité qui était tacite.
  Chiffres et conditions dans COMPOSITION-ANCREE-S188 §7.3 et §7.4.

**Suivi A50 — S189 : la loi du maximum est réfutée sur le réseau recommandé ; ADR-119.**
[COMPOSITION-GRADUEE-S189](../validation/COMPOSITION-GRADUEE-S189.md) mesure la composition
sur le réseau **gradué** d'ADR-118, qui sépare les deux pics d'erreur de 6 à 10 mailles.
**Le maximum y est rejeté pour le maintien** (0,730–1,000) quand il tient sur le réseau
ancré (0,936–1,060) : même montage, même critère, la géométrie change le verdict. Seule
l'**additive** survit — rapport maximal 0,981 ici, 0,984 en S188, 0,988 en S186, jamais
dépassé sur trois géométries. D'où
[ADR-119](../adr/ADR-119-le-budget-conjoint-se-borne-par-la-somme.md), qui **remplace la
règle de dimensionnement** de S186 §8.5 : borner par la somme, ne pas estimer par le
maximum, abandonner « égaliser les deux axes puis s'arrêter ». Un budget conjoint reste
licite — c'est sa répartition qui change. A50 attend toujours son **critère de justesse**.

**Suivi A232 — S189 : confirmée, et le mécanisme est mesuré.** A232 disait que la loi du
maximum n'était valide que tant que les deux maxima coïncidaient. Deux corrections et une
confirmation. *Correction 1* : les deux pics ne coïncident **jamais** à la maille — 0 cas sur
78 — la « coïncidence » de S188 était un effet de granularité, il localisait à la tranche
(196 mailles). *Correction 2* : ce qui gouverne n'est pas la coïncidence des maxima mais
l'**additivité locale** des deux champs d'erreur, vérifiée à 10 % de l'erreur de chaque case
et exactement dans les cas dégénérés ; la norme maximum n'en est qu'une lecture.
*Confirmation* : séparer les pics **change bien le verdict**, et A232 avait donc raison sur
la conclusion. Elle reste ouverte sur un point, et c'est le plus lourd : l'additivité n'a été
mesurée que sur un véhicule **sans projection de pression**.

**Suivi A229 — S189 : le mécanisme de la compensation est trouvé.** A229 relevait que dégrader
un axe peut réduire l'erreur totale, sans savoir pourquoi. La cause est l'additivité locale à
**signes opposés** : les deux champs d'erreur s'ajoutent maille par maille, et là où ils
s'opposent le composé passe **sous** le maximum des deux — jusqu'à 0,730 fois sur le réseau
gradué. La compensation n'est donc ni un artefact ni une propriété de la physique : c'est une
superposition de signes, qui dépend du montage et ne se transporte pas. Voir
COMPOSITION-GRADUEE-S189 §7.4.

- **A233** *(sévérité 2, S189 ; ouverte)* — **La seule borne portable est lâche d'un facteur
  2,3, et aucune estimation ne tient sur toutes les géométries.** ADR-119 impose de
  dimensionner un budget conjoint par `eU(r,1) + eU(1,c)`, parce que c'est la seule forme
  jamais dépassée sur les trois géométries de réseau mesurées. Mais son rapport
  mesuré/prédit descend à **0,437** : le total réel peut valoir moins de la moitié de la
  borne. Dimensionner par elle coûte donc jusqu'à **2,3 fois** la résolution nécessaire — en
  nœuds, en cadence, ou dans le produit des deux, et S184 a établi que le coût suit
  exactement ces nombres. Les deux estimations plus serrées sont inutilisables telles quelles :
  le **maximum** est rejeté sur le réseau gradué et dépassé jusqu'à 1,71 sur l'ancré ; la
  **quadratique** tient par mode et par famille mais pas sur l'ensemble, et rien ne dit
  laquelle s'applique **avant** d'avoir mesuré. Ce qui reste ouvert : une estimation portable
  demanderait de prédire la position relative des deux pics, donc de connaître la géométrie du
  contenu et du réseau avant de mesurer — et c'est précisément ce qu'un consommateur ne sait
  pas. La voie praticable n'est peut-être pas une meilleure formule mais une **mesure du
  couple retenu**, ce qu'ADR-119 §3 prescrit déjà faute de mieux. Sévérité 2 : la borne est
  sûre, aucun contrat n'est faux, et le coût du surdimensionnement est un gaspillage, pas une
  erreur de justesse. Chiffres et conditions dans COMPOSITION-GRADUEE-S189 §7.5.

**Suivi A50 — S190, 2026-09-12 : attente du critère close, source reçue sur le montage.**
L'utilisateur fixe **2 %**, ADR-120. B4-TOLERANCE-S190 reçoit 12 couples sur 126 ;
profil choisi gradué 14×14×8, extrapolation 80 ms : budget spatial+temporel+réserve
1,800653 %, composé+réserve 1,161371 %. Source omise à 100 %, refusée. Les dérivées
et la consommation existantes ont désormais un critère de réception ; A50 reste
partielle pour projection, surface libre, frontières et candidat δ. Ne plus reporter
« quelle erreur est acceptable » ; S190-1 reprend S189-1 sous le seuil décidé.
**Suivi A50/A233 — S191, 2026-09-12.** Projecteur D/G reçu contre matrice dense ;
profil 14×14×8/c8 reçu projeté avec budget1,371947 %, 1,374540 % en incluant le
résidu d'additivité, sous les2 % inchangés. A50 reste partielle : surface libre et
bords physiques manquent. A233 conserve son problème d'enveloppe large ; ADR-121
corrige sa portée, la somme seule est mesurée et non universelle sans résidu.
S190-1/S189-1 reçues sur véhicule ; suite S191-1 surface libre2D. Aucun nouvel angle.

**Suivi A211 — S191.** File active S190 relue entière. Ligne A50/B4 actualisée ;
les neuf autres conservées avec leurs déclencheurs. La prochaine action construit
la surface libre, elle ne prolonge pas la seule campagne de réseaux. Le dispositif
reste à éprouver, aucun chantier latéral déclaré clos par le présent essai.
**Suivi A50/A217 — S192, 2026-09-12.** Tranche x-z à surface libre linéaire reçue
contre Airy, trois profondeurs constantes, vitesse fine≤1,732796 %, cinq périodes.
S191-1 réalisée ; le dépôt possède un véhicule2D dispersif, mais aucun véhicule
reçu à la fois non linéaire et dispersif. A217 reste ouverte ; S192-1 reçoit ensuite
la non-linéarité de surface contre Stokes. A50 reste partielle (source non branchée,
comparaison intégrale absente), A216 inexpliquée. Aucun nouvel angle.

**Suivi A211 — S192.** Toute la file S190 relue ; B4/B3/A217/profondeur constante
actualisés, fond variable et autres objets conservés. La file ne redemande pas le
seuil2 %, et la suite construit une physique absente. Aucun chantier latéral clos.

- **A234** *(sévérité 2, S193 ; ouverte)* — **La faible profondeur non linéaire n'a aucun
  oracle dans ce dépôt, et ce n'est pas un manque de candidat.** S193 reçoit une surface non
  linéaire dispersive contre Stokes, mais la théorie de Stokes est bornée par le nombre
  d'Ursell `U = aL²/h³ ≪ 1`, et cette borne est **mesurée comme une falaise** :
  à `U=0,05` le véhicule rend `b₂` à `6,4·10⁻⁵` près sur un coefficient de **101,6** ; à
  `U=65` il est faux d'un facteur 2 ; à `U=130` d'un facteur 225 avec un décalage de
  fréquence de `−51 %` ; à `U=261` l'état cesse d'être fini. Or `U ≪ 1` à `L=8 m` et
  `h=0,25 m` exige `a ≲ 2,4·10⁻⁵ m` — **quatre ordres de grandeur sous l'amplitude qu'un
  banc emploie**, et trois ordres sous celle que S192 employait pour son propre cas peu
  profond. Conséquence : aucune réception non linéaire n'est possible en eau peu profonde,
  donc aucune sur les zones côtières, les rivages et les hauts-fonds, qui sont précisément
  là où une vague est visible et où le joueur la regarde. Ce qui manque est une **famille de
  références** — cnoïdale, ou Boussinesq — et non un solveur. Sévérité 2 et non 1 : la borne
  est connue, déclarée avant mesure, et aucun chiffre publié n'est faux ; ce qui est bloqué
  est un pan de réception, pas un contrat. Voir
  [SURFACE-LIBRE-NL-S193](../validation/SURFACE-LIBRE-NL-S193.md) §2.3 et §7.4,
  [ADR-122](../adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md).

- **A235** *(sévérité 3, S193 ; ouverte)* — **La fréquence de Stokes d'ordre trois en
  profondeur finie dépend d'une convention de courant moyen, et un accord numérique ne
  fournit pas la convention manquante.** À l'ordre trois, la fréquence d'une onde de Stokes
  en profondeur finie diffère selon que l'on impose une vitesse eulérienne moyenne nulle
  sous le creux ou un flux de masse moyen nul ; les deux conventions s'écartent d'un terme
  **du même ordre que la correction mesurée**. Un candidat périodique à potentiel périodique
  ne peut porter aucun courant moyen : sa convention est imposée par sa représentation. S193
  a donc refusé d'adopter la formule en profondeur finie, et l'a publiée en regard — où elle
  se trouve **reproduite à 0,013 % près**. C'est justement le piège : l'accord invite à
  conclure, et il ne dit rien de la convention. Ce qui reste ouvert : quelle convention la
  formule usuelle suppose, et si un montage à courant moyen imposé peut la départager. Tant
  que ce n'est pas tranché, seule la profondeur infinie fournit un oracle de fréquence.

- **A236** *(sévérité 2, S193 ; ouverte)* — **Une condition initiale bâtie sur une grandeur
  du continu, posée sur un modèle semi-discret, biaise l'estimateur censé la mesurer — et le
  biais imite un effet physique.** S193 construisait la trace `ψ` de son onde initiale avec
  `ω₀ = √(gk tanh kh)` alors que le véhicule porte `ω_d = √(g G_h(k))`. L'écart, `6·10⁻⁴`
  relatif à `K=64`, suffisait à rendre le mode fondamental **elliptique** au lieu de
  circulaire ; combiné à une fenêtre non entière en cycles du modèle, il produisait un biais
  de fréquence de `−1,1125·10⁻⁷` — **0,14 % du décalage non linéaire à mesurer à la plus
  petite amplitude**, et de même nature que lui. Preuve de la cause : le biais valait
  `1,1125·10⁻⁷` à `h=8` et `6,928·10⁻⁹` à `h=2`, **rapport 16,1**, exactement le rapport des
  ellipticités. Corrigé, le résidu tombe à `−5,072982·10⁻¹⁰`, égal à l'erreur de phase de
  RK4 prédite analytiquement. Ce qui rend cet angle général et non anecdotique : **toute**
  réception de ce dépôt compare un candidat discret à un oracle continu, et rien ne garantit
  que ses conditions initiales, ses fenêtres et ses lignes de base soient celles du candidat
  plutôt que celles de l'oracle. Le cas trouvé ici l'a été par une contre-épreuve à signal
  nul ; les montages qui n'en déclarent pas ne l'auraient pas vu. Sévérité 2 : aucun chiffre
  publié n'est faux — le défaut a été trouvé et corrigé avant publication — mais le
  mécanisme n'a été audité sur aucun autre banc du dépôt.

- **A237** *(sévérité 3, S193 ; ouverte)* — **Un ordre de troncature n'a pas de taux de
  fidélité propre : la part qu'il capture dépend du régime.** À `M=2`, le décalage de
  fréquence vaut `0,4916 → 0,5004` fois celui de Stokes en profondeur infinie, et `0,663` à
  `kh=1,5708`. Ce n'est donc ni zéro — la conclusion naïve, prédite fausse avant mesure — ni
  une fraction transportable. Conséquence pour toute sélection : on ne peut pas caractériser
  un schéma tronqué par un coefficient d'ordre mesuré une fois, puis l'appliquer ailleurs.
  Ce qui reste ouvert : la loi qui donne cette fraction en fonction de `kh` n'est pas
  dérivée, et deux points ne font pas une loi. ADR-122 tranche en retenant l'ordre trois, ce
  qui rend la question non bloquante, mais elle redeviendra vive si le coût impose un jour de
  redescendre d'un ordre.

**Suivi A50/A217/A216 — S193, 2026-09-12.** Surface **non linéaire** dispersive reçue
contre Stokes, [SURFACE-LIBRE-NL-S193](../validation/SURFACE-LIBRE-NL-S193.md) et
[ADR-122](../adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md) : harmonique
liée à **0,4555 %** et décalage de fréquence à **1,6454 %**, sous les 2 % d'ADR-120 ;
ordres mesurés 2 en profondeur discrète et 4 en temps. S192-1 réalisée. **Le manque
structurel d'A217 tombe** — le dépôt possède un véhicule à la fois non linéaire et
dispersif — mais **A217 reste ouverte** : aucun couplage de deux trains n'est mesuré, et
ADR-112 garde toute sa portée. A50 reste partielle (source S191 non branchée, comparaison
intégrale absente, forces et perception non reçues) ; A216 reste inexpliquée et aucun seuil
de bascule ne se dérive d'ici. Quatre angles nouveaux — **A234** faute d'oracle en faible
profondeur, **A235** convention de courant moyen, **A236** ligne de base d'un modèle
semi-discret, **A237** fidélité d'un ordre non portable. Suite S193-1 : deux trains,
écart somme/évolution de la somme, contre-épreuve `M=1` exactement nulle.

**Suivi A211 — S193.** Toute la file active relue, et **renommée S193** : son titre portait
encore « S190 » alors que son contenu est daté ligne par ligne, ce qui est exactement le
défaut qu'A185 décrit — un état dont l'étiquette vieillit seule. Quatre ancres repointées
en conséquence (REPRISE, INDEX, ANGLES-MORTS, BILAN-B4). Lignes actualisées : S193-1/A50/B4
/A217, B3/δ, A216/A217, bathymétrie — cette dernière reçoit A234, qui change sa nature :
il y manque une **référence**, pas un solveur. Les six autres lignes sont conservées avec
leurs déclencheurs : forces/perception, A213, λ_cut/B2/coupure W–δ, A98 multiplateforme,
V/bancs restants, A94/A95 dossier de réunions. Aucun chantier latéral déclaré clos par le
présent essai. Le dispositif reste à éprouver.

- **A238** *(sévérité 2, S194 ; ouverte)* — **Un maximum sur une fenêtre ne converge pas
  quand la quantité mesurée est une différence de deux évolutions presque égales.** S194
  raffine la profondeur discrète sur son écart de superposition et obtient, sur la
  **moyenne quadratique**, un ordre de `1,9295` et un résidu de Richardson de `0,47 %` à
  `K=64` — de la convergence propre. Sur le **maximum** de la même quantité, aux mêmes
  points, les ordres valent `−0,7914`, `−0,3700`, `+0,6140`, et les valeurs ne sont même
  pas monotones (`1,668 / 1,709 / 1,781 · 10⁻²` pour `K=32/64/128`). La cause n'est pas un
  défaut de véhicule : un maximum est une **statistique d'ordre** sur un signal oscillant,
  et un changement de fréquence de `6·10⁻⁴` suffit à déplacer l'endroit où il tombe. Ce
  qui rend cet angle général : **S192 et S193 ont toutes deux reçu leur convergence
  spatiale sur des maxima**, avec des ordres 2 impeccables — parce que leur erreur était
  une fonction lisse et monotone du pas, et non un résidu de deux évolutions. Rien dans
  leurs protocoles ne disait pourquoi cela marchait, donc rien n'avertit qu'ailleurs cela
  ne marchera pas. Ce qui reste ouvert : aucune autre réception du dépôt n'a été relue sous
  cet angle, et plusieurs bancs mesurent des **écarts** entre deux montages. Sévérité 2 :
  aucun chiffre publié n'est faux — les maxima publiés restent les bons nombres pour juger
  un budget, et leur incertitude de discrétisation est celle de la moyenne quadratique —
  mais un *ordre de convergence* établi sur un maximum de résidu ne vaut rien.
  Voir [COUPLAGE-DEUX-TRAINS-S194](../validation/COUPLAGE-DEUX-TRAINS-S194.md) §7.9.

- **A239** *(sévérité 3, S194 ; ouverte)* — **Un contrôle de non-artefact qui juge la
  *taille* d'un déplacement ne distingue pas un artefact d'une convergence.** Le protocole
  de S194 exigeait que passer `K` de 32 à 64 ne déplace pas l'écart de plus de 2 % ; le
  déplacement mesuré vaut `5,37 %`, et le contrôle est donc **non tenu tel qu'il était
  écrit**. Or ce n'est pas un artefact : le rapport des déplacements successifs
  `|K32−K64|/|K64−K128|` vaut `3,81`, c'est-à-dire 4, c'est-à-dire exactement ce que produit
  un schéma d'ordre deux sur une grille grossière. **Un contrôle de ce genre échoue
  d'autant plus que le schéma converge mieux**, ce qui est l'inverse de son intention. La
  forme correcte juge l'**ordre** de la suite et le **résidu de Richardson** au pas retenu.
  Ce qui reste ouvert : le dépôt compte d'autres contrôles formulés en « déplacement sous
  x % » et aucun n'a été relu sous cet angle. Sévérité 3 : c'est une forme de contrôle à
  corriger, non un résultat faux ; la mesure de S194 a été refaite dans la bonne forme dans
  la même session.

- **A240** *(sévérité 2, S194 ; **close en S195**)* — **Deux trains ne sont pas `n` sources, et le
  nombre de paires croît comme `n²`.** ADR-123 chiffre le domaine de la superposition pour
  **deux** trains : sous 2 % en dessous d'une cambrure de `0,009` par train, fautif avant
  une période au-delà de `0,014`. Le chemin perturbatif du projet additionne en revanche
  autant de sources qu'il y a d'événements et de contributions — et **rien de ce qui est
  mesuré ne s'extrapole** : le forçage croisé compte un terme `2Q(a_i,a_j)` par paire, donc
  `n(n−1)/2` termes, et la part cumulative en compte davantage encore. Trois régimes sont
  concevables et aucun n'est mesuré : addition en racine si les phases sont décorrélées,
  addition linéaire si elles ne le sont pas, ou saturation si le domaine du véhicule est
  quitté avant. Ce qui rend l'angle lourd : c'est la **seule** limite d'ADR-123 qui touche
  l'architecture plutôt que le banc, et la frontière de `0,009` pourrait être bien plus
  basse pour un état de mer réel. Ce qui le borne : la mesure est à portée immédiate — trois
  puis quatre trains à cambrure **totale** fixée, sur le véhicule et le banc existants, avec
  la contre-épreuve `M=1` à écart nul déjà calibrée. **À instruire avant toute promesse sur
  un état de mer complet.**

**Suivi A217 — S194, 2026-09-12 : CLOSE par [ADR-123](../adr/ADR-123-le-domaine-de-validite-de-la-superposition.md).**
A217 demandait, depuis S161, quelle variable gouverne la validité de l'addition en eau
profonde, et supposait « vraisemblablement la cambrure, et rien ne le vérifie ». **C'est la
cambrure**, et la loi est mesurée : `écart/A ≈ α s + β s² N` avec `α = 1,302602`,
`β = 5,898728`, résidu à `1,82 %` de l'écart maximal. Le blocage qu'A217 nommait — « le
dépôt n'a aucun solveur à la fois non linéaire et dispersif », et « un solveur linéaire ne
peut pas servir, la superposition y étant vraie par construction » — est levé deux fois :
S193 a construit le véhicule, et S194 a **vérifié** cette dernière remarque au lieu de la
supposer, l'écart à `M=1` valant exactement zéro sur les modes.

**A217 ignorait deux variables, et elles ne sont pas secondaires.** La **durée** : à
cambrure `0,0125` la superposition tient `5,4` périodes et pas davantage, parce qu'une part
de l'écart est séculaire. Et le **désaccord de triade**, donc la profondeur : `α` vaut
`1,383` à `h=8 m` et `11,855` à `h=0,25 m`, facteur `8,6`, parce que la dispersion
s'affaiblissant, les triades approchent la résonance. Une frontière établie en eau profonde
ne se transporte donc pas vers le rivage.

Ce qui **n'est pas** clos et se poursuit ailleurs : `n` sources (**A240**), l'obliquité en
2D, et A216 qui reste inexpliquée et ne se dérive pas d'ici.

**Suivi A218 — S194 : le cas profond dispersif est traité.** A218 demandait d'intégrer
réellement le résidu de couplage et de le comparer au total avec témoin de couplage
supprimé. S163 l'avait fait sur véhicule 1D peu profond ; S194 le fait en eau profonde et
dispersive, avec `M=1` comme témoin exact à écart nul. Le résidu n'est pas intégré comme
équation propre — c'est la différence de deux évolutions qui est mesurée — donc A218 garde
son objet pour un montage à résidu explicite ; mais sa limite « aucune référence non
linéaire dispersive » ne tient plus.

**Suivi A50/B4 — S194.** Le chemin perturbatif a un domaine chiffré, et il est étroit : à
`s=0,0125` S193 recevait un train **unique** contre Stokes à `0,4555 %` quand **deux**
trains superposés franchissent 2 % en `5,4` périodes — facteur quarante à cambrure égale.
A50 reste partielle : `n` sources non mesurées, obliquité non mesurée, fournisseur S191 non
branché, forces et perception non reçues, frontières ouvertes absentes. Une **voie de
correction est chiffrée** : l'écart est la réponse à un forçage croisé explicite, dont la
part quadratique est bornée, non cumulative, et suffirait près de la frontière.

**Suivi A234 — S194 : la faible profondeur est mesurable sans oracle.** A234 constate
l'absence d'oracle de Stokes en faible profondeur. S194 y mesure pourtant le couplage,
parce que la comparaison est **candidat contre candidat** — somme des évolutions contre
évolution de la somme — et n'a besoin d'aucun oracle extérieur, seulement du domaine de
validité du véhicule (`U ≤ 5,2·10⁻²` ici). A234 reste ouverte pour ce qu'elle dit — aucune
réception **contre référence** n'y est possible — mais sa portée se restreint : elle ne
bloque pas les mesures différentielles.

**Suivi A236 — S194 : le mécanisme a été soupçonné, testé, et innocenté ici.** A236 dit
qu'une grandeur du continu posée sur un véhicule semi-discret biaise l'estimateur qui la
mesure. S194 a soupçonné ce mécanisme d'expliquer la sensibilité en `K` de son écart — la
condition initiale employant le `b₂` du continu — et l'a **testé en retirant le terme** :
résidu `0,4698 %` avec, `0,5026 %` sans, ordres `1,9295` et `1,8711`. Aucun changement :
l'hypothèse est fausse pour ce cas, et la sensibilité vient de la dynamique, `G_h` étant le
seul objet dépendant de `K`. A236 reste entière — elle a été vérifiée, pas confirmée — et
c'est la première fois qu'elle est mise à l'épreuve plutôt qu'invoquée.

**Suivi A211 — S194.** Toute la file active relue et **renommée S194**, comme en S193 et
pour le même motif (A185) ; quatre ancres repointées. Lignes actualisées : S194-1/A50/B4,
B3/δ, bathymétrie. Ligne **scindée** : A216/A217 devient A216 seule, A217 étant close.
**Ligne neuve** : `n` sources / A240, avec son déclencheur. Conservées avec leurs
déclencheurs : forces/perception, A213, λ_cut/B2/coupure W–δ, A98 multiplateforme, V/bancs
restants, A94/A95 dossier de réunions. Aucun chantier latéral déclaré clos par le présent
essai, et la suite est **à instruire** entre deux options — `n` sources ou la correction
croisée — plutôt qu'imposée par proximité.

**Clôture A240 — S195 : mesurée sur `n = 2..6`, et la crainte est levée dans son ordre.**
[SOURCES-MULTIPLES-S195](../validation/SOURCES-MULTIPLES-S195.md), empreinte
`0x5eb378f6ffe26c9f`. Des trois régimes qu'A240 disait concevables, **aucun** n'est celui qui
sort : à cambrure **par train** fixée l'écart croît en `n^0,75` — sous-linéaire, donc très loin
du `n²` du comptage de paires et sous le `n` de l'addition cohérente ; à cambrure **totale**
fixée il **décroît** en `1/√n`, si bien que répartir une même mer sur plus de composantes
*améliore* la superposition. A240 avait raison sur le comptage et tort sur la conséquence, et
la mesure va plus loin que la dérivation : même le régime cohérent était pessimiste.
**ADR-123 se transporte donc à `n` sources dans le sens favorable**, avec les limites de la
mesure : `n ≤ 6`, trains colinéaires, fond plat, eau profonde. Sept réceptions sur dix passent,
dont le cas nul exact, la continuité avec S194 à `10⁻⁶`, la convergence d'ordre 1,756 à résidu
de Richardson 0,892 %, et une bande neutre à `0,0000 %`. **A240 est close** ; ce qu'elle
laisse ouvert est repris en **A241**.

- **A241** *(sévérité 2, S195 ; ouverte)* — **Les harmoniques croisées retombent sur les modes
  des trains, et c'est cette part-là qui gouverne la loi à grand `n`.** Les deux régimes
  d'addition classiques — somme des amplitudes, racine de la somme des carrés — supposent
  tous deux que l'écart vit sur des nombres d'onde **propres au couplage**. Il n'en vit
  qu'une part. Mesuré en série A, phases alignées, de `n=2` à `n=6` : la part portée par les
  modes exclusivement croisés chute d'un facteur **6,8**, celle portée par les modes de train
  d'un facteur **1,4** seulement — un écart de **4,85** entre les deux vitesses. La seconde
  domine dès `n = 4`, et c'est elle qui maintient la loi mesurée (`n^-0,46` en série A,
  `n^0,75` en série B) **entre** les deux bornes dérivées, qu'aucune n'encadre. Ce qui rend
  l'angle sérieux : sur un spectre dense, *tous* les modes croisés retombent sur des modes
  existants, si bien que le régime mesuré ici sur six trains est le régime **naissant**, pas
  une exception. On ne sait donc pas si la loi tend vers une limite quand le spectre se
  peuple, ou si `n ≤ 6` en donne une image trompeuse — et c'est exactement ce qu'il faudrait
  savoir avant de promettre quoi que ce soit sur un état de mer complet. Ce qui le borne : le
  banc existe, le diagnostic modal est disponible (contrairement à ce que S195 §2.4 avait
  annoncé), et la mesure demande d'étendre `n` et de densifier la bande, pas de construire.
  Aucune décision de projet n'en dépend aujourd'hui ; aucune promesse de spectre non plus.

**Suivi A241 — S196 : requalifiée, ses deux moitiés n'ayant pas le même sort.**
[REPLI-CROISEES-S196](../validation/REPLI-CROISEES-S196.md), empreinte `0xbcf2911362458c13`.
Montage de **parité** : trois familles de modes à `n` et cambrure totale égaux, dont une —
les impairs `3,5,…,2n+1` — où somme et différence de deux trains sont paires et ne peuvent
donc **jamais** retomber sur un mode de train. Repli nul par arithmétique, vérifié par test.

- **Moitié « limite » : close.** L'exposant sature à **`−0,52`** dès `n ≈ 4` et n'y bouge plus
  jusqu'à `n = 16`. La loi tend bien vers quelque chose ; `n ≤ 6` la sous-estimait de `0,08`.
- **Moitié « cause » : réponse partielle, et la thèse de S195 était trop forte.** Éteindre
  tout le repli des paires déplace l'exposant de `−0,394` à `−0,525` : **0,131**, soit **32 %**
  du chemin jusqu'à la loi dispersée (`−0,805`). Le protocole exigeait `> 0,20` pour confirmer
  et `< 0,10` pour réfuter : **ni l'un ni l'autre**, et c'est ce qu'une prédiction déclarée
  d'avance rend impossible à maquiller. Le repli **déplace** la loi, il ne la **gouverne** pas ;
  deux tiers de l'écart restent sans cause identifiée. Signe supplémentaire : entre `n=8` et
  `n=16` la fraction de repli monte encore de `0,536` à `0,642` **pendant que l'exposant ne
  bouge plus du tout**.
- **Montage validé par son propre témoin** : dense contre paire — même repli, même bande
  relative, échelle doublée — s'accordent à `0,057`, sous le seuil de 0,10.

A241 reste **ouverte** sur sa moitié « cause ». Les deux suspects nommés et non séparés : le
confondant de bande relative (24 % entre les deux parités, borné mais pas isolé) et les termes
**triples**, que la parité ne neutralise pas.

- **A242** *(sévérité 2, S196 ; **close en S197**)* — **Le critère de conservation ne détecte pas la
  sous-résolution.** S194, S195 et S196 déclarent hors domaine toute configuration dont la
  dérive relative d'énergie dépasse `10⁻⁴`, et s'en servent comme **du** critère de validité.
  S196 en a trouvé un contre-exemple net : à `n = 16`, `K = 32`, la moyenne quadratique de
  l'écart vaut `1,3765e-2` quand `K = 64` donne `2,7174e-3` et `K = 128` `2,8352e-3` — **fausse
  d'un facteur cinq** — et la dérive d'énergie y vaut `1,54e-6`, soit **soixante-cinq fois sous
  le seuil**. Aucun pas n'a été refusé, aucune valeur n'est infinie, rien n'a divergé : la
  sous-résolution produit une réponse **lisse, conservative et fausse**, et un critère de
  conservation y est aveugle par construction — il mesure ce que le schéma préserve, pas ce
  qu'il résout. Conséquence immédiate dans ce banc : le triplet de Richardson y devient
  inutilisable (incréments `−1,105e-2` puis `+1,178e-4`, signes opposés) et la formule
  rendrait un « ordre 6,552 » sans broncher si on ne l'en empêchait pas. **Ce qui est en jeu
  n'est pas ce banc-ci** — il le détecte désormais et le dit — mais toute session qui a lu
  « énergie sous `10⁻⁴` » comme une attestation de justesse. Le correctif est connu et peu
  coûteux : apparier tout critère de conservation à un contrôle de **raffinement**, et refuser
  d'imprimer un ordre depuis un triplet non monotone. Voir **L277** et
  REPLI-CROISEES-S196 §8.5.

**Correction A241 — S197 : le verdict de S196 ne survit pas au raffinement, et le repli est
réfuté.** [AUDIT-RESOLUTION-S197](../validation/AUDIT-RESOLUTION-S197.md) §8.3. S196 avait
conclu que le repli des harmoniques croisées pèse **un tiers**, sur un écart pair/impair de
`0,131` mesuré à `K = 64`. À `K = 1024`, où l'erreur du symbole de dispersion tombe de
`120 %` à `0,75 %`, cet écart vaut **`0,005`** — les deux familles donnant `−0,432` et
`−0,426`. À `K = 64` l'audit reproduit exactement les chiffres de S196 : il est fidèle, et
c'est bien la conclusion qui tombe, pas la mesure qui diverge.

**C'est la prédiction 2 de S196**, celle qui réfute : « les deux familles s'accordent à mieux
que `0,10`, et l'explication de S195 tombe ». Elles s'accordent à `0,005`. **Le repli des
harmoniques croisées sur les modes de train n'explique rien de mesurable.** La moitié
« cause » d'A241 a donc sa réponse, et elle est négative ; la **totalité** de l'écart entre la
mesure et l'addition dispersée reste sans explication. La moitié « limite » survit, sa valeur
passant de `−0,52` à environ `−0,45`.

Pourquoi ce verdict est tombé et pas ceux de S194 et S195 : ceux-là comparaient des
configurations **à même bande**, où l'erreur de symbole est commune aux deux côtés et
s'annule ; S196 comparait deux familles de bandes différentes — 38 contre 40 — donc
d'exposition différente. Voir **L278**. A241 reste **ouverte** sur sa cause, avec un suspect
de moins et aucun de plus.

**Clôture A242 — S197 : traitée, et elle a servi dès le premier emploi.**
L'audit a couvert les trois cibles publiées. **ADR-123 tient** — sa table mesurée est
convergée dès `K = 256` et se déplace de 3,3 % au plus ; elle reçoit une note datée de
confirmation. **A240 tient** — exposants déplacés de 1 à 3 %. **Le verdict de S196 tombe**
(ci-dessus). Le remède est en place dans le support partagé et dans les trois bancs :
`dispersion_error(upto)`, qui donne l'écart entre le symbole discret employé et le symbole
continu **à la configuration réellement exécutée**, et `richardson()`, qui refuse de tirer un
ordre d'un triplet non monotone. Deux tests neufs les fixent. **A242 est close** ; ce qu'elle
laisse, c'est **L278**, et une réserve : le pas de temps `dt` n'a pas été audité, et S193 ne
l'a pas été non plus — sa bande peuplée est la moins exposée des quatre, mais ce n'est pas
une mesure.

- **A243** *(sévérité 2, S198 ; ouverte)* — **Un corpus produit du travail de corpus, en
  proportion de sa taille, et ce travail n'avance aucune couche.** Mesuré : **3,5 lignes de
  markdown par ligne de code d'exécution** (63 343 contre 18 144), **235 angles morts** dont
  la majorité trouvés dans nos propres écrits et non dans les documents sources, **23 notes
  correctives** datées. Une part croissante du temps de session sert à relire, corriger,
  propager et tenir à jour ce que les sessions précédentes ont écrit — l'entretien du rituel
  de fin, la file plurielle, les décomptes, les notes de propagation. Chacun de ces gestes
  est **justifié pris isolément** : ce sont eux qui ont permis à S197 de rattraper S196, et
  aucun ne doit être supprimé à la légère. Mais leur **somme** croît avec le corpus, quand ce
  que le projet doit encore construire, lui, ne décroît pas. Ce qui rend l'angle sérieux :
  il ne se manifeste jamais comme un problème — chaque session se termine en ayant bien
  travaillé — et il n'a pas de seuil d'alarme. Ce qui le borne : il est **mesurable en une
  commande**, `sh outils/velocite.sh`. **Déclencheur** : relancer l'outil tous les dix
  sessions ; si le ratio de prose monte ou si la part système reste sous 10 % sur deux ères
  consécutives, le dire au rituel de fin et arbitrer — élaguer le corpus est alors un
  travail légitime, au même titre qu'une mesure. Ne pas confondre cet angle avec un appel à
  écrire moins : les sessions qui ont le plus produit de code sont aussi celles qui avaient
  le mieux écrit leur protocole.

- **A244** *(sévérité 1, S199 ; ouverte)* — **Un noyau peut réussir ses tests de
  contrat tout en violant le contrat annoncé.** La reprise de delta_projection.rs
  trouve des Vec::clone dans le pas, y compris par itération de pression, alors que
  le reçu « zéro allocation » ne compte que les appels à l'allocateur d'hôte.
  Le plafond d'itérations n'est pas une durée garantie ; le refus de non-fini après
  mutation n'est pas atomique ; la pression f64 dans src n'est pas conforme à I-08
  par le seul fait que le calcul soit juste. Le scalaire g injecté ne reçoit pas un
  référentiel accéléré général malgré supports_frame_accel=true. Voir les preuves
  de lecture et restrictions dans CANDIDAT-DELTA-S199 §7. Gravité1 pour le contrat
  d'exécution, pas pour un accident observé en jeu : ce candidat n'est pas intégré.
  **S199-1** : corriger le noyau, recevoir les allocations globales et refus réels,
  borner les capacités et régler explicitement précision/budget. Le défaut spatial
  du fond reste un lot distinct **S199-2**, aucune famille éliminée.

**Note de comptage — S199, 2026-09-13.** Avant A244, le registre porte243 identifiants
A1–A243 sans trou ;236 était le nombre de puces reconnues par velocite.sh, qui omettait
les identifiants restés uniquement en tableau. L'outil compte désormais les identifiants
uniques. Le nombre235 cité dans A243 est historique et ne constitue pas un total exact.

**Suivi A244 — S200, 2026-09-13 : partiellement traitée.** Les clones du pas sont
supprimés, allocations globales nulles mesurées y compris au premier pas et en mode
dégradé. Refus après overflow : u/w/p conservés et récupération au pas suivant reçue.
Réservation de mémoire typée corrigée ; Caps retire référentiel général et plages
CFL/résolution non mesurées. Workspace342/cinq ignorés, trois tests globaux aussi
release. Reste **S200-1** : pression f64 expérimentale sous I-08 et budget itérations
sous I-05, explicitement non reçus pour production. S199-2 flux ouverts reste active.
Aucun nouvel angle ; voir CONTRATS-DELTA-S200.

**Suivi A228/A50 — S201.** B visible via IMAGE-B-S201, image CPU locale autorisée et
inspectée. Le critère numérique2 % est acquis depuis ADR-120 ; une image n'est pas
un protocole perceptuel reçu. A50 reste partielle, aucun seuil humain inventé.
ADR-124 change la priorité de construction (budget image puis effets δ bornés),
pas la validité physique des mesures. Aucun nouvel angle.

**Suivi A244 — S202.** Mesure du coût du pas reçue via horloge injectée et Caps,
avec absence d'allocation et invariance du résultat ; **ce n'est pas le respect du
budget temporel**. A244 demeure partielle pour I-05 et pression f64. Profil utilisateur
60Hz/eau2ms acté ADR-125 ; bloc64×32 convergé dépasse seul ce budget. Aucun autre
contrat reçu par le seul chronométrage.

- **A245** *(sévérité 1, S203 ; ouverte)* — **Le budget de pente de la composition refuse la mer
  de référence avant tout impact.** `compose` somme `steepness_B·π = Σ aᵢkᵢ`, borne L1 du fond,
  et la compare à π/7. ADR-094 et ADR-095 la tenaient pour la pente **exacte** de B, marge 1 :
  c'est faux dès que les directions s'écartent. Recette JONSWAP de S201 à Hs 1,5 m : L1 0,6082,
  majorant directionnel démontré 0,5733, pente échantillonnée 0,4215 (512 m, 120 s) — marge
  ≥ 1,061 et ≤ 1,443. **Chaque point de toute composition B+W est refusé**, et même le
  majorant directionnel refuse ; marge nulle à Hs ≈ 1,107 m pour cette recette. I-18
  (« ce qui est comparé à `max_slope` est une pente réelle ») n'est donc pas tenu pour le terme
  de B, et sa garde exécutable ne pouvait pas le voir : elle recense des **sites** de
  comparaison, pas les **termes** sommés. Personne ne l'avait vu parce que l'image S201
  n'évaluait que B. Gravité 1 : un impact ou un sillage W est impossible sur une mer modérée.
  Remèdes à instruire, aucun choisi : terme directionnel (5,7 %, insuffisant seul), pente
  réelle **au point** (connue depuis S144, mais le budget cesse d'être indépendant du point),
  ou borne statistique (ne garantit rien). Lot bibliothèque, change des bits et des frontières.
  Voir IMPACT-W-S203 §2, notes correctives ADR-062/094/095, L280.

- **A246** *(sévérité 2, S203 ; traitée par ADR-126)* — **L'emprise d'un impact était
  dimensionnée par son admission, jamais par ce qui se voit à sa frontière.** Les gardes de
  `RadialImpact` (résolution, portée, régime) admettent R 8 m / A 4 s ; hors emprise l'hôte rend
  B seul, et la couture temporelle vaut alors 78 mm à 2 s, encore 3,25 mm à 48 s, indépendamment
  de N. Au critère de 2 %, l'image demande R ≥ 15,5 λ, A ≥ 96·√(λ/g), N ≥ 256 — et place
  l'observateur S201 **dans** l'emprise. Ni ADR-060, ni ADR-066, ni ADR-085, ni ADR-105 ne
  posaient la question. ADR-126 fixe la réception par coutures et le profil ; restent ouverts le
  seuil perceptuel, le renouvellement d'horizon et plusieurs impacts. Voir IMPACT-W-S203 §4, L281.

- **A247** *(sévérité 2, S203 ; ouverte)* — **Un impact visible coûte à lui seul plus que le
  budget eau d'une image.** Mesuré sur ce CPU, un fil : B seul 1,6 µs par point (≈ 1 250 points
  dans 2 ms), B+W N256 14 µs par point (≈ 140 points). La table radiale η(r), η'(r) rend le point
  à 15 ns avec une erreur ≤ 0,006 mm, mais sa **construction** coûte 2,7 ms par image pour **un**
  impact — N×M échantillons, au-delà des 2 ms d'ADR-125 — et croît linéairement avec le nombre
  d'impacts. Aucun `paquets_W_max` n'a été confronté à ce budget. ADR-125 écartait déjà le
  renderer CPU comme chemin de production ; ce qui est neuf, c'est que le **champ** lui-même,
  évalué sur CPU, ne tient pas non plus. Leviers non mesurés : pas de table élargi, N réduit
  loin des coutures, parallélisme, évaluation GPU des phases repliées (I-08). Voir
  IMPACT-W-S203 §7.

- **A248** *(sévérité 1, S204 ; traitée par ADR-127)* — **Un ordre de construction a été inscrit
  comme une réduction du produit, sous le nom de l'utilisateur, et trois sessions l'ont
  portée.** L'utilisateur avait donné une direction d'ordre — image, puis budget, puis effets
  bornés. ADR-124 (S201) l'a écrite comme un périmètre : « δ est un effet borné par son cas
  d'usage », « V attend un besoin gameplay nommé », « revenir à un δ général demande une nouvelle
  décision ». Son statut — « direction explicite de l'utilisateur » — a prêté l'autorité de la
  demande à la glose. S202 l'a recopiée dans ADR-125 ; S203 — l'agent qui écrit cette entrée — dans la file
  active, REPRISE et IMPACT-W-S203, **sans qu'aucune la confronte aux ambitions initiales** : ni
  `docs/sources/`, ni ADR-001, ni ADR-053. Aucun travail supprimé, mais S199-2 et S200-1 étaient
  devenus conditionnels, V sans échéance, et la prochaine étape visait des effets et non le
  système. **L'utilisateur a dû le corriger lui-même.** Remède : ADR-127 (ambition complète,
  jalons J1–J5, V non repoussée), FEUILLE-DE-ROUTE comme porteur unique, et une ligne d'amorce
  dans AGENTS : une session ne réduit pas l'ambition, un ordre ou un budget ne retirent rien.
  Ce qui reste à éprouver : qu'aucune autre décision « de priorité » du corpus n'ait retiré du
  périmètre en silence — recherche non faite en S204. Voir L282.

**Clôture A245 — S205, 2026-09-13 : traitée par ADR-128.** La raideur de B sort du budget de
refus des quatre sites de composition ; elle reste publiée dans `steepness`, même ordre de somme,
et aucun bit publié ne change pour un lot que l'ancienne règle admettait (image S203 et hachages
C18/C02 reproduits). La mer S201 (Hs 1,5 m) se compose, impact compris : zéro refus, zéro pixel
hors emprise. I-18 est tenu pour le terme de B, qui n'est plus comparé. `slope_floor` devient
exact dans les deux sens. **Ce qui ne l'est pas** : la garantie « B+W sous π/7 », retirée par
décision ; la validité de la superposition sur mer raide (ADR-123, A207). Voir
COMPOSITION-MER-S205.

- **A249** *(sévérité 2, S205 ; ouverte)* — **Les bancs et essais de refus de la composition
  n'employaient que des mers jouets.** Hs 0,01 (`bench_water`, fond de pression d'amplitude
  0,01), Hs 0,1 (`tests_mixed_water`), une composante de 1 cm : sur aucun, la borne L1 de B
  n'approchait π/7. Le terme de B est entré dans le budget en S79 (ADR-062) ; il a fallu S203 et
  un rendu pour découvrir qu'il interdisait toute mer au-delà de Hs ≈ 1,1 m. Sept essais
  exerçaient même leurs verdicts **par la raideur de B** sans que personne ne se demande ce
  qu'elle vaut sur une mer de jeu. Le défaut général : un banc de refus dont les paramètres sont
  choisis pour isoler un mécanisme ne dit rien des frontières que le produit franchit. Remède
  partiel S205 : un essai à la recette S201. **À faire** : recenser les autres bancs de refus
  (pression, sillage, profils N/R/A, `max_slope` des fixtures à 0,1) et y ajouter au moins un point
  à paramètres de jeu. Voir L280.

- **A250** *(sévérité 1, S206 ; ouverte, arbitrage posé)* — **Le dépôt n'a jamais eu de chemin
  GPU, et le budget d'image a été fixé comme s'il n'en fallait pas.** ADR-003 et I-08 prévoient
  que « seules des phases repliées passent au GPU » ; ADR-012 déclarait `gpu_sim_ms = 2,5` à côté
  de `cpu_sim_ms = 2,0`. En 206 sessions, aucun hôte n'a exercé de GPU, et ADR-125 a fixé « eau
  2 ms par image » sans dire où B s'évalue. Mesuré en S206 : B par sommet sur CPU coûte 42 ms à la
  densité qui montre un impact, 36 ms sur seize fils — l'eau visible ne tient pas le profil sur
  CPU, et toutes les mesures de coût du dépôt étaient CPU par construction. Gravité 1 : bloque la
  sortie de J1. Remède : l'arbitrage « chemin de rendu et hôte » (FEUILLE-DE-ROUTE §4) ; la part
  W est déjà réduite d'un facteur 100 (ADR-129). Voir COUT-IMAGE-S206, L283.

**Suivi A250 — S207, 2026-09-13 : décision prise, pas encore traitée.** L'utilisateur retient le
rendu J1 sur GPU par un hôte séparé ([ADR-130](../adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md)).
A250 reste ouverte tant qu'aucun hôte n'exerce le GPU et que le budget GPU de l'eau n'est pas
confronté à une mesure.

**Suivi A247 — S208, 2026-09-13 : part W construite.** `RadialTable` (ADR-129) : l'impact coûte
0,04 ms de profil et ~43 ns par sommet ; sur 36 160 sommets, W passe de ~240 ms à ~0,95 ms par image,
image par la table à un niveau près de l'image directe. **A247 reste ouverte pour B** (41–42 ms sur
CPU à la même densité), dont le chemin est décidé par ADR-130 : GPU, hôte séparé.

**Suivi A250 — S208.** Pile recommandée pour l'hôte GPU (HOTE-GPU-S208) : wgpu 30.0.1, winit
0.30.13, pollster 1.0.1, espace de travail séparé. Rien téléchargé ; A250 reste ouverte jusqu'à un
hôte qui exerce le GPU et une mesure du budget GPU de l'eau.

**Clôture A250 — S211, 2026-09-13 : chemin GPU exercé et mesuré.** `viewer/` séparé,
mer B et impact W reçus sous DX12 ; passe eau médiane0,048576 ms à960×540 sur RTX5070 Laptop.
L'objet « aucun chemin GPU ni mesure » est traité. Cela ne clôt pas J1 : sillage absent,
cadence complète et budget total encore à recevoir. Voir [HOTE-GPU-S211](../validation/HOTE-GPU-S211.md).

**Suivi A247 — S211 : part B+impact traitée sur le GPU local.** Maximum de hauteur0,077657 mm
contre3 mm déclarés, passe eau max0,059008 ms à960×540. Le goulot CPU par sommet est retiré
de l'image ; reste le coût complet CPU/transfert/GPU/présentation et la scène avec sillage.
Ne pas lire une mesure de passe comme un reçu du profil complet60 images/s / eau2 ms.

**Suivi A247 — S212, 2026-09-13 : le sillage rouvre le coût, par deux autres lois.** Sillage du
cœur (4 096 nœuds) rendu exact à 0,089 mm, mais la préparation modale refaite par image coûte
10,6–10,9 ms de CPU et la somme par sommet porte la passe GPU de 0,049 à 4,10 ms à 960×540. GPU
∝ sommets × nœuds (7,6–7,9 ps), CPU ∝ nœuds × tronçons (317–331 ns). Leviers nommés, non
mesurés ; A247 reste partielle. Voir [HOTE-GPU-S212](../validation/HOTE-GPU-S212.md).

- **A251** *(sévérité 2, S212 ; ouverte)* — **Un sillage visible n'a ni durée honnête ni couture
  reçues, et rien n'empêche l'hôte de le montrer au-delà.** La fixture S212 déclare un contexte de
  40 s sur une recette 64×128 qui ne tient 2 % de la recette fine que pendant les 16 s de forçage
  (9,5 % à 24 s, 28 % à 39 s), et une emprise dont le bord porte 12,7 mm à 39 s — plus encore sur la
  recette fine, donc du contenu physique, pas seulement la récurrence d'ADR-107. L'impact a reçu
  ses coutures spatiale et temporelle par ADR-126 ; le sillage n'a rien d'équivalent, et A214 dit
  pourquoi aucun garde n'existe (loi en durée encadrée seulement). Le défaut général : une scène
  d'image hérite des horizons du service (64 s d'ADR-106) sans hériter des domaines de validité
  qu'ADR-107 attache à la recette. Gravité 2 : l'image ment sans refus, mais cosmétiquement (I-04).
  **À faire** : déduire l'emprise et la durée d'image d'un sillage de sa recette et de sa vitesse,
  les recevoir par coutures comme ADR-126, et les publier avec la fixture. Voir A214, L281.
  *S213 (ADR-131 D6) : travail nécessaire de J1, indépendant du coût — un LOD spectral réduirait
  encore la durée et le rayon honnêtes, ce qui rend A251 plus pressante, pas moins.*

- **A252** *(sévérité 2, S213 ; traitée par ADR-131)* — **Un dépassement de budget mesuré sur une
  implémentation presque dépourvue d'optimisation a été écrit comme un verdict sur la
  fonctionnalité, et sa suite comme un compte à rebours vers une réduction d'ambition.**
  HOTE-GPU-S212 : « coût refusé », « chemin refusé en coût », « si les deux leviers mesurés ne
  tiennent pas 2 ms, l'arbitrage revient à l'utilisateur » — alors que la mesure ne comportait ni
  LOD, ni visibilité, ni mutualisation, ni repli temporel. L'utilisateur a corrigé le jour même.
  C'est le mécanisme d'A248 sous une forme nouvelle : le vocabulaire de réception (« reçu /
  refusé ») appliqué au budget, et ADR-127 D7 (« toute incompatibilité donne lieu à un
  arbitrage ») lu comme un déclencheur après deux essais. Le corpus porte d'autres verdicts du
  même type, **non réécrits** et désormais lus comme portant sur l'implémentation de leur date :
  le bloc S202 de REPRISE et du README (« trop cher même isolé » pour un bloc δ de 64×32) ;
  COUT-IMAGE-S206, qui disait déjà mieux — « incompatible … sur cette machine, pour cette scène,
  sur CPU » — sans lister les techniques absentes. Remède : ADR-131 — techniques présentes, absentes et domaine avec chaque mesure ; espace
  d'optimisation nommé (J1-bis) ; 2 ms éprouvé sur la combinaison. **Reste à éprouver** : que les
  mesures de δ (B3, coût du pas) adoptent le même en-tête quand J2 les reprend. Voir L286, L282.

**Suivi A247 — S213, 2026-09-13 : loi CPU du sillage déplacée.** Repli temporel
(`pressure_timeline`) : 1,26 ms pendant le forçage, 0,36 ms après, contre 7,70 / 13,36 ms pour la
préparation par image (4 096 nœuds, un fil) ; hôte 1,7 ms. Loi GPU inchangée. Mesure publiée avec
techniques présentes, absentes et domaine (ADR-131) ; A247 reste partielle. Voir TEMPS-SILLAGE-S213.

- **A253** *(sévérité 2, S214 ; traitée dans l'hôte, ouverte pour l'interface)* — **Une même sonde
  avait deux positions : B au point du réseau monde, les perturbations au point `f32` brut.**
  `FrameData::references` — la référence contre laquelle `--verify` juge le GPU — convertissait le
  point en `WorldPos` pour B (quantification 1/2048 m, erreur ≤ 244 µm) et gardait le `f32` brut
  pour l'impact et le sillage. Écart mesuré contre la composition du cœur : **1,78e-5 m** de hauteur
  et 2,4e-5 de pente, quand la même somme faite **au point du cœur** est exacte au bit. Le cœur s'en
  protège par construction (`eval_local`, « conversion commune B/W ») ; l'hôte contournait cette
  protection en reconstruisant ses points. Portée : 0,6 % de la tolérance de 3 mm, jamais visible —
  mais c'est la **référence** qui mentait, pas le GPU, et l'écart croît avec la pente, donc avec un
  LOD spatial. **Corrigé dans l'hôte en S214** (une conversion, trois couches). **Reste ouvert** :
  `Background::eval` n'accepte qu'un `WorldPos` et `eval_local` est `pub(crate)`, donc un hôte qui
  veut évaluer B hors du réseau — ce que fait le GPU, à des décalages bruts depuis un œil quantifié
  — ne le peut pas par l'interface publique. Voir L287, COMPOSITION-J1-S214 §2.

- **A254** *(sévérité 1, S214 ; ouverte)* — **Le budget de pente est une somme sur les sources : la
  scène J1 en consomme 84 % avec deux, et la troisième refuserait toute l'image.** Mesuré sur la
  composition du cœur (`mixed_water`, ADR-128, ADR-119 règle 1) : budget conjoint 0,3477 à 0,3776
  contre `max_slope = π/7 = 0,4488`, soit **77,5 à 84,1 %**, pour **une** source de chaque type —
  impact 0,2126, sillage 0,1350 à 0,1650. Marge restante **0,0712** : un second sillage de la même
  recette ou un second impact refuse tout lot non vide. Et le refus serait **`SlopeEnvelope`** —
  vérifié, 4 477 points sur 4 477, zéro `Slope` — car la pente **réelle** des perturbations vaut
  0,0929 à 0,0352, soit **3,7 à 10,5 fois moins** que son majorant. La physique garde un facteur
  dix ; le contrat n'a plus rien. Gravité 1 : ce n'est pas un défaut cosmétique mais un refus, et il
  tombe sur la trajectoire — toute scène à plusieurs sources, donc la mutualisation de J1-bis, J3 et
  les inondations de J4. A208 nommait le mécanisme par le choix d'emprise, sur un champ seul ;
  ici c'est le **cardinal** qui consomme, et il n'avait jamais été mesuré composé. **À faire** :
  décider si le majorant se resserre (le rapport mesuré dit qu'il le peut), s'il se compose
  autrement que par la somme (ADR-119 règle 1 l'interdit sans mesure), ou si `max_slope` cesse
  d'être une constante de milieu. Voir L288, ADR-098, COMPOSITION-J1-S214 §3.
  *Précision S214, à ne pas lire trop fort* : la « pente réelle » citée est le **maximum
  échantillonné sur 4 477 sondes** (grille de 1,3 m plus les bords d'emprise), pas le maximum sur
  l'emprise — S140 a montré qu'il ne se calcule pas, il se cherche. Le rapport 3,7–10,5 est donc un
  majorant du pessimisme, pas sa valeur exacte ; pour calibrer, S203 mesurait un facteur 1,36 entre
  majorant directionnel et pente échantillonnée sur B, ce qui laisse l'essentiel de l'écart debout.
  **I-18 est tenu** : les deux termes sommés sont déjà convertis en pente réelle — `RadialImpact`
  divise sa borne L1 par son rapport mesuré (S141), `bound_pressure` retient l'enveloppe resserrée
  (S141). Le pessimisme est donc d'emprise et d'alignement (A208), pas d'unité.

**Suivi A251 — S214, 2026-09-13 : traitée par [ADR-132](../adr/ADR-132-domaine-d-image-d-un-sillage.md).**
Le domaine d'image d'un sillage se calcule depuis sa recette — `rayon = 2π·angular/(3·cutoff)`,
`durée = 4π/√(g·cutoff/radial)` —, il est publié avec la fixture (`WAKE_LOI`) et l'hôte annonce une
fois quand l'instant montré en sort (`WAKE_HORS_DOMAINE`). Fixture S212 : 89,36 m et 18,53 s contre
un coin d'emprise à 102,22 m et un contexte de 40 s, soit **2,2 fois** sa durée honnête ; conservée
telle quelle, parce qu'elle a servi à recevoir S211 à S214. Trois critères l'encadrent : 2 % franchi
entre 16 et 18 s, couture de 3 mm entre 18 et 20 s, rayon d'accord à 10 % entre 24 et 30 s — **une
durée honnête porte le critère qui l'a calibrée**. Voir COMPOSITION-J1-S214 §4.

**Suivi A214 — S214, 2026-09-13 : un troisième point de calibration, et toujours pas de garde.**
La récurrence radiale, refusée par S156 sous la forme qui donnait 13,1 s pour un encadrement de
15–20 s, est juste sous la forme `4π/√(g·dk)` : 18,53 s à radial 128/cutoff 6 (encadré 15–20 s),
26,2 s à radial 256 (conservateur contre 45–50 s), 18,53 s à radial 64/cutoff 3 (encadré 16–30 s
selon le critère, S214). **Reste dû** : la dépendance à `sigma`, aucun point entre radial 256 et
512, et la décision de bibliothèque — un garde à l'admission refuserait la fixture S212 elle-même.

**Suivi A247 — S214, 2026-09-13 : l'écart hôte/exemple de S213 n'avait pas de cause à chercher.**
Trois passages du même binaire donnent 1,2555 / 1,2458, puis 1,6477 / 1,7760, puis 1,5820 / 1,8398 ms
de sillage CPU, la valeur basse sur le passage à froid. L'écart « 1,7 contre 1,26 » que S213 laissait
non attribué se reproduit **entre deux passages du même programme**. Ce n'est pas une attribution
nommée — aucun compteur thermique n'a été lu. A247 reste partielle. Voir L289.

**Suivi A254 — S215, 2026-09-13 : le chiffre tenait, la cause était fausse, la moitié est traitée.**
Le pessimisme mesuré à échantillonnage fin (20 001 points de rayon ; le pic d'un impact radial est
en `r = 0,2062 λ` = 0,69 m, invisible à la grille de 1,3 m de S214) **n'est ni l'emprise d'A208 ni
un défaut d'unité — I-18 est tenu — mais la dispersion** : un majorant en somme de modules modaux
est invariant quand les modes ne font plus que tourner, tandis que le maximum spatial décroît. Il
vient presque entièrement de l'impact (facteur 1 à la naissance, 4,3 à 4 s, 30,4 à 56 s) et non du
sillage (1,4 à 1,9 pendant le forçage). Le refus a été **exercé** : deux impacts et un sillage,
budget 0,590210 contre 0,448799, `Err(SlopeEnvelope)`.
**Traitée par [ADR-133](../adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md)** : le majorant
d'un impact suit la dispersion (`slope_max_at`, table `RHO_DISPERSION` en âge adimensionné, sûre et
vérifiée hors famille génératrice), et le budget de composition le consomme. Occupation 84 % → 42 %,
et trois impacts avec un sillage passent désormais à 51 %. **Reste ouvert dans A254** : le budget
demeure une **somme**, donc le nombre de sources reste une ressource bornée — le resserrement recule
la limite, il ne la supprime pas. Voir L290, L291, BUDGET-PENTE-S215.

- **A255** *(sévérité 2, S215 ; ouverte)* — **Le majorant du sillage domine désormais le budget, et
  sa famille n'a aucune loi.** Après ADR-133, l'impact ne pèse plus que 0,0215 à 16 s quand le
  sillage prescrit en pèse 0,1650 — soit **88 %** du budget de la scène. Le mécanisme y est le même
  (L290) et le pessimisme est mesuré — 1,39 à 4 s pendant le forçage, 4,77 à 39 s une fois la source
  éteinte, son majorant restant figé à 0,157 — mais aucune similitude n'y a été établie : sa famille
  est paramétrée par σ, cutoff, radial, angular **et le découpage en tronçons**, et rien ne dit que
  le rapport s'effondre sur un âge adimensionné comme celui de l'impact. Sans cela, une scène à
  plusieurs sillages retrouve le refus qu'ADR-133 vient de lever pour les impacts. **À faire** : la
  même campagne que S215 sur la famille du sillage — faire varier chaque paramètre, chercher
  l'échelle de temps propre, et ne pas présumer qu'elle existe. Le forçage complique le cas :
  pendant qu'une source émet, son majorant **croît** au lieu de décroître. Voir ADR-133 §« ne fait
  pas », A254, L290.

**Suivi A208 — S215, 2026-09-13 : ce n'était pas le terme dominant, et A208 reste entière.**
Le pessimisme d'emprise et d'alignement qu'A208 décrit existe toujours ; S215 montre seulement
qu'ici il était masqué par un facteur trente venu du temps. Rien n'est clos : après ADR-133, le
majorant resserré d'un impact reste un majorant **sur toute son emprise**, et A208 s'applique à
lui comme avant.

**Suivi A255 — S216, 2026-09-13 : la part statique est traitée, et elle ne demandait aucune mesure.**
Le pessimisme du majorant du sillage se décompose en deux. **Directionnel** : `slope_envelope_tight`
sommait **scalairement** des contributions vectorielles de directions différentes — 1,2586 à 4 s,
1,1979 à 39 s, et **identique à quatre décimales** pour `angular` 64/128/256, `radial` 32/128 et
`cutoff` 2/4, donc propriété du champ et non de sa discrétisation. **Traité par
[ADR-134](../adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md)** : une inégalité de
Cauchy–Schwarz en `O(N)`, sans table, sans garde, sans domaine ; gain obtenu 1,155 à 1,200, soit
**79 à 87 %** du gain qu'un maximum exact rendrait. **De phase** : le reste, 1,10 à 4 s et **3,98 à
39 s**, gouverné par le temps écoulé **depuis l'extinction** (6,29 à 30 s pour deux tronçons, 1,75
pour seize encore en forçage) et insensible à la recette.
**Discriminant tranché** : agrandir l'emprise d'un facteur 4 en côté — **seize fois en aire** —, à
pas de grille constant, laisse le maximum réel identique à six décimales. Ce n'est donc **pas** une
limite d'emprise (A208) mais bien la décohérence de L290. **A255 reste ouverte, et ne contient plus
que cela.** Occupation de la scène J1 : 84,1 % (S214) → 41,6 % (ADR-133) → **36,3 %** (ADR-134).
Voir L293, L295, ENVELOPPE-SILLAGE-S216.

- **A256** *(sévérité 2, S216 ; ouverte)* — **Une annonce qui nomme le facteur qu'elle retire sans
  dire à quelle échelle il s'applique se lit comme complète.** La documentation de
  `slope_envelope_tight` disait, depuis S141 : « elle retire exactement deux facteurs indépendants
  valant chacun 1 à √2 : **la direction du vecteur d'onde** et la phase de la réponse ». C'est vrai
  — mais de la direction **au sein d'un mode** (`|kx|+|ky| → |k|`), pas de l'étalement des
  directions **entre** modes, qui valait encore 20 % et qu'une inégalité retire gratuitement. Un
  lecteur qui cherchait où gagner lisait « la direction est traitée » et passait. Le défaut n'est ni
  une erreur ni une omission : c'est une **portée non dite**, et elle a coûté le temps qui sépare
  S141 de S216 sur un gain qui ne demandait aucune mesure. Même famille qu'A248 (un ordre lu comme
  un périmètre) et A252 (un verdict lu plus large que sa mesure). **À faire** : quand une annonce
  dit retirer un facteur, lui faire dire **sur quel domaine de sommation** il porte — par mode, par
  source, par lot — et vérifier que les autres échelles sont nommées comme non traitées. Le corpus
  en porte probablement d'autres : `slope_bound`, `steepness`, les enveloppes de composition.
  Voir L293, ADR-095, ADR-134.

**Suivi A255 — S217, 2026-09-13 : similitude conditionnelle reçue, courbe unique réfutée.**
À tau=(t-D)/sqrt(sigma/g)=4, majorant ADR-134/maximum : base1,637275, vitesse divisée
par deux2,272143, durée doublée2,005464. Écarts38,8 % et 22,5 %, maxima convergés à
<0,002 % entre128×256 et256×512. Homothéties, amplitude et découpage reçus. Les
comparaisons à âge absolu égal de S216 ne séparaient pas âge après extinction et
histoire du forçage. **Reste ouverte** : borne plus serrée ; prochaine piste J1/W,
borne locale avec reste spatial démontré, sans table à un seul âge. Voir
[DECOHERENCE-SILLAGE-S217](../validation/DECOHERENCE-SILLAGE-S217.md).

- **A257** *(sévérité 2, S217 ; explication corrigée et contre-épreuve reçue)* —
  **L'invariance de l'énergie a été attribuée au module de hauteur.** S215/L290
  décrivaient le sillage éteint comme une somme de modules figés ; le code évolue
  eta et velocity ensemble. |eta| peut osciller, tandis que omega²|eta|²+|velocity|²
  reste constant. Le test mono-mode S217 vérifie les deux simultanément. Le majorant
  d'une source encore active peut aussi diminuer (fixture lente :0,059152→0,052456).
  Notes datées ADR-133, S215, S216 et L290 ; décisions d'impact et inégalité ADR-134
  inchangées. Toute future borne temporelle doit porter la réponse libre complète.

**Suivi A255 — S218, 2026-09-13.** Capacité locale construite (ADR-135), aucune
migration d'admission. Partition0,5m : gain multiplicatif1,026–1,224 dans les trois
cas recevables ; coût uniforme base27,5–28,2s. Suite J1/W : partition adaptative bornée
en travail ; somme A254, scènes à plusieurs sillages et mutualisation restent ouvertes.

- **A258** *(sévérité 2, S218 ; quantification traitée, certification f32 ouverte)* —
  **Une borne du champ continu ne couvre pas automatiquement sa représentation exécutée.**
  Le produit f32 `turns*x` suivi de Q32 peut sauter même sur un petit rectangle translaté.
  ADR-135 borne les produits arrondis aux extrémités et ajoute une réserve numérique ;
  centre trompeur et translation4000m éprouvés. La preuve de variation trigonométrique
  ne certifie pas tout l'arrondi de la norme et des sommes. Avant une migration des
  admissions, recevoir ou démontrer cette chaîne, sans faire passer le maximum d'une
  grille pour une preuve continue. Voir BORNE-LOCALE-S218 et L297.

**Suivi A255/A258 — S219.** Partition adaptative construite ; gain multiplicatif1,48–1,52
à65535 évaluations sur les trois cas recevables,35–36s. A255 partielle, aucune admission
migrée ; A258 (certification numérique) reste ouverte. Pas de reprise inter-appels.

- **A259** *(sévérité 2, S219 ; ouverte)* — **La borne qui écrête tout au même niveau
  prive l'adaptation de priorité spatiale.** À8191 évaluations, les quatre partitions
  retiennent encore la même borne globale qu'à la racine, pour4,4–4,5s. Le tas ne
  distingue les régions qu'après raffinement suffisant. Il améliore les résultats
  ensuite, mais changer l'ordre des égalités ne produit pas les annulations absentes
  de la borne. Suite J1/W : borne locale avec Hessienne signée et reste supérieur,
  quantification couverte, puis coût et gain reçus sur S219. Ce mécanisme ne préjuge
  ni d'un gain de cette piste ni du coût d'un chemin GPU. Voir PARTITION-S219, L298.

**Suivi A255/A258/A259 — S220, 2026-09-13.** ADR-136 : borne d'ordre deux à Hessienne signée.
Partition à 32767 évaluations : 1,006–1,012 × maximum de référence sur les quatre fixtures ;
**le pessimisme d'A255 n'est plus l'obstacle sur ces fixtures, le coût l'est** (≈30 s CPU un
fil par instant). A258 devient aussi le **plancher de précision** : sur les feuilles
millimétriques, la réserve `γ_N·C` (0,38–1,26 % du maximum) dépasse de 160 à 360 fois le
reste géométrique. A259 partielle : levée au-dessus de ≈16 000 feuilles, intacte à 8191
évaluations, mécanisme nommé A260. Voir ORDRE-DEUX-S220.

- **A260** *(sévérité 2, S220 ; ouverte)* — **Un mode non résolu payé isolément coûte deux
  fois sa masse.** ADR-135 et ADR-136 bornent la variation d'un mode à `D_k ≥ 2` par
  `2 c_k`, mode par mode, et ADR-136 paie `D_k²/2` presque autant aux modes inclus proches
  de `D = 2`. Sur une maille 2 × 1,5 m, 1608 modes sur 4096 sont exclus et le reste total pèse
  74–87 % de la borne globale — la répartition entre exclus et inclus n'est pas mesurée : la
  borne locale ne peut alors pas descendre sous la globale, quel que soit l'ordre. L'enveloppe directionnelle du seul sous-ensemble `U`
  (ADR-134) les paie au plus `C_U` ensemble, et `G(U) + |S_U(c)| ≤ 2 C_U` garantit de
  ne jamais perdre. Aucun gain chiffré avant mesure de `C_U` par taille de maille. Toute
  partition des modes est valide ; le tri éventuel vit dans la mémoire de l'appelant.
  Voir ORDRE-DEUX-S220 §Suite, L299.
**Suivi A259/A260 — S221, 2026-09-13.** ADR-137, coupure spectrale. **A259 levée à 4096
feuilles** : à 8191 évaluations, gain 1,115–1,230 sur la globale là où ADR-136 restait à 0,998 ;
intacte à 1024 feuilles (gain ≤ 1,024). **A260 traitée, mécanisme corrigé** : à 2 × 1,5 m, les
modes exclus (`D ≥ 2`) portent 1,1–2,2 % de la masse ; le reste venait de la classe `[1, 2)`
(44–67 % de la masse, reste résolu 0,0919 → 0,0138 quand on la retire, base). La coupure utile est
`D* = 1`. Voir COUPURE-SPECTRALE-S221, L301.

- **A261** *(sévérité 2, S221 ; ouverte)* — **Aucune enveloppe de modules ne voit la localisation
  spatiale d'un paquet.** ADR-135, ADR-136 et ADR-137 bornent chaque mode, ou chaque sous-ensemble,
  par des modules `|η_k|` indépendants du point. Quand une cellule est assez grande pour que la
  plupart de la masse y soit non résolue, sa borne vaut presque la globale **où qu'elle soit**,
  même loin du sillage : pire rectangle 2 × 1,5 m, `C_U` = 94 % de `C`, `G(U)` = 0,1097
  pour une globale de 0,1152 (base). La localisation vit dans la cohérence des phases entre modes
  voisins. Conséquence : en dessous d'environ 4096 feuilles sur 128 × 96 m, aucun ordonnancement ni
  aucune coupure de modules ne réduit le travail. Pistes non dérivées : champ lointain par sommation
  par parties sur la quadrature polaire, localisation par vitesse de groupe. Vérifier d'abord
  l'uniformité radiale de la quadrature et la phase `ω(k) t` d'un nœud au suivant.
  Voir COUPURE-SPECTRALE-S221, ADR-137 « Limite annoncée ».
**Suivi A254 — S222, 2026-09-13 : la part somme change de côté, et c'est le résultat.**
Mesurée sur une scène à une, deux et trois sources de sillage dans un **même journal**
([SOMME-SILLAGES-S222](../validation/SOMME-SILLAGES-S222.md)).
**Côté sillages, elle est absorbée** : les sources partagent les emplacements du demi-spectre
(témoin : `modes = 4096` pour une comme pour trois), leurs amplitudes s'additionnent en complexe,
et trois sources coûtent **1,67** fois une seule quand elles sont proches, **1,44** quand elles sont
éloignées — loin du facteur 3 des impacts. Ce qui reste de pessimisme y est **spatial** (A261) : le
maximum réel ne bouge pas avec la séparation (0,070316 / 0,070390 / 0,070463) pendant que
l'enveloppe croît de 44 %, d'où un pessimisme qui passe de 1,64 à **2,35**.
**Côté impacts, elle est littérale et intacte.** `slope_floor` somme `slope_max_at` par champ **sans
aucune conscience de la distance** ; ADR-133 a resserré chaque majorant dans le **temps**, jamais
leur somme dans l'**espace**. Voir **A262**.
**Traduction en admission** : trois sillages et **huit** impacts passent aujourd'hui (neuf si
éloignés), là où S214 voyait la deuxième source refuser l'image. La part somme n'est donc plus
contraignante **pour des impacts d'une dizaine de secondes** — mais sous deux secondes, un impact
neuf vaut **47,4 %** de π/7 à lui seul et un seul passe, ce qu'aucune borne ne change (à τ = 0 le
majorant est exactement atteint, S215). Voir L302, L303.

**Suivi A261 — S222, 2026-09-13 : chiffré sur une scène, et il plafonne tout.**
Loin de la source, sur un rectangle de 2 × 2 m, la meilleure borne disponible vaut **213 à 757 fois**
le maximum local. Le terme des modes non résolus `G(U)` (ADR-137) ne dépend ni du point ni des
phases : il est spatialement aveugle par construction. Conséquence mesurée : **au-delà d'un mètre de
demi-côté, la borne locale vaut exactement l'enveloppe globale** — la maille doit être
sous-ondulatoire (λ_min = 2,09 m) ou elle ne sert à rien, ce qui fixe la loi d'échelle du prix
(L302). A261 reste ouverte.

- **A262** *(sévérité 2, S222 ; ouverte)* — **La somme des majorants d'impact ignore la distance
  entre les champs.** `mixed_water::slope_floor` additionne `slope_max_at` pour chaque
  `RadialImpact` admis : deux impacts frais à **cent mètres** l'un de l'autre consomment exactement
  le même budget que deux impacts au même point, alors qu'aucun point du domaine ne voit plus de la
  moitié de cette pente. C'est le **goulot mesuré** depuis S222 : un impact neuf vaut 47,4 % de π/7,
  donc deux éclaboussures simultanées saturent, et la borne locale de pression n'y change rien —
  elle desserre un terme qui ne serre plus. Rien dans S215 à S222 ne l'a touchée.
  **Ce qui rend la ligne traitable, et la distingue d'A261** : un `RadialImpact` n'est pas une somme
  de modules à support infini. Il a un **support compact et déclaré** — un disque de rayon connu —
  et sa pente décroît avec la distance à son centre (ADR-094 : maximum en `r = 0,2062 λ`). Deux
  disques disjoints ne peuvent pas atteindre leur maximum au même point, et cela se démontre au lieu
  de se mesurer. **À faire** : une annonce conjointe qui tienne compte de la position relative des
  champs — pas une table, une inégalité, comme ADR-134 l'a été pour les directions. Voir A254, A261,
  SOMME-SILLAGES-S222 §5.

**Suivi A262 — S223, 2026-09-13 : traitée par [ADR-138](../adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md).**
Le budget de pente tient désormais compte de la **position relative** des champs d'impact.
`RadialImpact::slope_max_beyond(t, r)` majore la pente sur la **couronne** `r ≥ r₀` — décroissance
en `1/√r`, minimum avec `slope_max_at` — et `mixed::slope_floor_joint` en tire, par l'inégalité
triangulaire et un balayage à huit intervalles **sûr entre ses échantillons**, un plancher conjoint.
Gain **1,76** (deux impacts, 50 m, à la naissance) à **2,57** (trois, 90 m), **exactement 1,0000 à
séparation nulle** ; coût **13,3 µs**, soit 0,7 % du budget d'image. **Trois impacts frais séparés
de 50 m passaient de 142,1 % de π/7 — refusés — à 60,0 % — admis.** Zéro attente de test modifiée :
à un champ le chemin est celui d'avant au bit. Voir COURONNE-IMPACT-S223.
**Ce qui n'a pas été instruit** : l'**ancrage** du balayage (le plus grand majorant global est un
choix, pas un optimum) ; le cas de **plus de trois champs**, où la perte de l'inégalité triangulaire
pourrait croître ; l'usage de la borne hors admission.

- **A263** *(sévérité 3, S223 ; ouverte)* — **Une constante mesurée du dépôt rattrapait un facteur
  que personne n'avait nommé.** `SLOPE_L1_RATIO = 1,795071`, calibré en S141 pour convertir la borne
  L1 d'un champ radial en pente réelle, vaut `1/0,581865 × 1,045` : c'est le **pic de `J₁`**, que la
  mesure retrouvait sans le dire. Le code posait `|J₁| ≤ 1` avec, en commentaire, « borne
  conservative », et la calibration payait la différence derrière. Rien n'est faux — mais un facteur
  **calculable** a été mesuré pendant quatre-vingts sessions, et sa nature est restée invisible
  jusqu'à ce qu'on ait besoin de le faire **varier** avec le rayon. Gravité 3 : aucun défaut
  numérique, un coût d'occasion. **À faire** : passer en revue les constantes calibrées du dépôt et,
  pour chacune, se demander **quelle quantité analytique elle approche** — c'est la question qui a
  ouvert ADR-138. Candidats nommés : les deux `SLOPE_L1_RATIO` (`radial_impact` 1,795071 et
  `impact_field` 1,701591), la réserve trigonométrique `32·ε` d'ADR-135. Voir L304, L305, ADR-138.

- **A264** *(sévérité 2, S224 ; ouverte)* — **Le plancher de vidange d'un contenant croît avec sa
  surface, et rien ne l'annonce.** La hauteur d'eau est dérivée du volume et portée en micromètres
  entiers (ADR-010 §2, `shape_lut`). Un contenant cesse donc de se vider quand sa hauteur tombe sous
  l'unité de représentation, c'est-à-dire quand son volume tombe sous **sa surface × 1 µm** : un
  millilitre pour un réservoir d'un mètre carré — mesuré en S224 —, **cent millilitres pour une
  piscine de cent mètres carrés, dix litres pour un pont inondé d'un hectare**. Le reliquat n'est
  pas une erreur de calcul : il est exactement représenté, et il ne part jamais. Gravité 2 : sur les
  grandes surfaces que le jeu vise — cales, ponts, compartiments inondés —, une flaque résiduelle
  d'une dizaine de litres qu'aucune pompe ne peut vider est un symptôme de jeu, pas un détail
  numérique. **Ce qui a été fait** : l'interpolation arrondit au plus proche au lieu de tronquer, ce
  qui divise le plancher par deux — pas davantage. **À faire** : décider si la table de forme doit
  porter une unité plus fine pour les grandes surfaces (le nanomètre donnerait un facteur mille),
  si un seuil de « contenant vide » doit être déclaré par l'hôte, ou si le reliquat doit être
  absorbé par `absorb_rate`, qu'ADR-010 §2 prévoit et qui n'est pas construit. Voir L306,
  NOYAU-V-S224 §3.

**Suivi A17 — S224, 2026-09-13 : toujours ouverte, et désormais visible dans du code.**
`liquid_id` figure dans ADR-010 §2 et **pas** dans `HydroNode` : le premier module ne connaît qu'un
liquide. Ce n'est pas une régression — c'est la même question qu'A17 posait, maintenant attachée à
une structure qu'il faudra étendre plutôt qu'à un paragraphe.

- **A265** *(sévérité 3, S225 ; ouverte)* — **Le recouvrement CPU/GPU varie avec la charge, et la
  mesure ne l'explique pas.** À caméra fixe, `travail CPU + GPU` vaut 2,10 + 4,25 = 6,35 ms pour un
  intervalle de **5,05** : recouvrement partiel. À caméra balayée, où le GPU perd un tiers,
  2,08 + 2,93 = 5,01 ms pour un intervalle de **4,88** : **presque aucun recouvrement**. La cadence
  ne suit donc pas le GPU — elle passe de 198 à 205 Hz quand la passe d'eau tombe de 4,16 à 2,85 ms.
  Quelque chose borne la trame autour de 4,9 ms que ni le travail CPU ni la passe GPU n'expliquent :
  profondeur de la chaîne d'échange, cadencement du pilote, `AutoNoVsync` qui n'est pas
  nécessairement immédiat. Gravité 3 : aucune décision n'en dépend aujourd'hui, et le verdict de
  coût s'appuie sur le GPU, mesuré directement. **À faire** avant d'optimiser le CPU : savoir si la
  trame a un plancher indépendant du travail, sinon une seconde gagnée pourrait ne rien rendre.
  Voir L308, CADENCE-HOTE-S225 §4-bis.

**Suivi A247 — S225, 2026-09-13 : les exclusions du coût sont chiffrées, et elles ne cachaient rien.**
La réserve « sky, upload, readback, presentation excluded », écrite dans chaque ligne de coût depuis
S211, est mesurée : la passe d'eau vaut **97,95 à 98,06 %** de la trame GPU, tout le reste faisant
**0,087 ms**. Les verdicts de S211 à S224 portaient donc bien sur l'essentiel. En revanche la pose de
mesure était **proche du pire cas** sans que personne l'ait choisie : à caméra balayée le GPU d'eau
médian vaut 2,85 ms contre 4,16 à la pose héritée de S201 (L309). A247 reste partielle.

- **A266** *(sévérité 1, S226 ; ouverte)* — **ADR-010 §2 pose deux dispositions incompatibles, et
  leur désaccord est maximal là où chacune sert.** Le même paragraphe demande que `shape_lut` soit
  cuite « à partir du maillage (**coupes horizontales**) » et que le plan d'eau soit
  « **perpendiculaire à `g_eff`**, pas à Z ». Mesuré en S226 à 0,3 g latéral (16,7°, le montage de
  C16), par intégration numérique : pour un **prisme** à parois verticales la table reste **exacte**
  (0,0000 %) tant que le plan ne touche ni le fond ni le plafond, et dérape de 1 à 4 % aux extrêmes ;
  pour une **coque en V** elle se trompe de **9,89 % partout**, milieu compris. Or c'est
  précisément la cale que l'ADR invoque pour justifier la table — *« Un compartiment n'est pas un
  prisme »*. Sur la charge, 9,9 % d'erreur de hauteur donnent environ **5 %** sur le débit, qui va
  comme `√h`. **Gravité 1** : V est destinée aux cales, compartiments et dépressions de terrain,
  c'est-à-dire au cas faux, et l'erreur porte sur un volume — donc sur une conséquence de jeu
  (I-10). Ce n'est **pas** un défaut du module : c'est une incohérence de la conception, restée
  invisible vingt-cinq sessions parce que personne n'avait construit les deux ensemble (L311).
  **À trancher avant toute utilisation de V sur un contenant non prismatique.** Trois voies, aucune
  choisie : table à deux entrées (volume, inclinaison) ; correction analytique pour les sections
  convexes ; ou restriction déclarée de V aux prismes — ce qui retirerait à `shape_lut` sa raison
  d'être. Voir L310, L311, GRAVITE-DIRIGEE-S226 §4.

**Suivi I-07 — S226, 2026-09-13 : la violation est levée.** `hydro_network::step` reçoit désormais
`g_eff` en **vecteur**, et la charge à une ouverture est sa distance signée au plan perpendiculaire
à `g_eff`. Le hublot latéral ne fuit pas sous gravité verticale et fuit sous 0,3 g (1 829 ml en dix
pas) ; l'inclinaison de surface vaut 16,6990° contre 16,6992° attendus, à **0,0002°** du degré
qu'exige C16. Réduction au cas vertical **exacte** : tous les nombres de S224 sont identiques.
**Ce que le passage enseigne** : C12 passait à 0,08 % avec l'axe en dur — un cas canonique bien
choisi peut être muet sur un invariant (**L310**).

**Suivi A266 — S227, 2026-09-13 : domaine du défaut élargi par le pas réel.** Le diagnostic S226
imposait une cote verticale centrale, alors que le module utilise la hauteur tabulée comme une
distance normale au plan. Même le prisme à mi-remplissage est donc faux dans le module : à pente
0,3, sa cote centrale passe de 1 m à 1,04403065 m. Un hublot central à 1,01 m laisse partir **506 ml
en dix pas**, au lieu de zéro. Le test `tilted_prism_volume_regression_a266_s227` conserve cet
attendu correct, reste explicitement ignoré et a été exécuté en échec connu. La phrase S226
« 9,89 % partout » était aussi trop générale : elle ne vaut que dans le domaine non tronqué de
son montage ; sa dernière hauteur mesure 6,17 %. Avec la convention du module, l'écart de volume
de la cale non tronquée vaut 19,78 %. Dérivation et correction datée dans
[GRAVITE-DIRIGEE-S226](../validation/GRAVITE-DIRIGEE-S226.md). **Toujours ouverte, sévérité 1** :
prochain lot, relation volume/plan orienté construite et éprouvée sur les données réellement
consommées. Une restriction définitive aux prismes ne répondrait pas à ADR-127.

- **A267** *(sévérité 1, S227 ; corrigée en S227)* — **La capacité d'un receveur était réservée
  séparément par chaque arrivée.** Trois sources pouvaient transférer chacune 1 ml dans le même
  contenant de capacité 1 ml : le pas réussissait à 3 ml, puis le suivant refusait cet état.
  La masse totale restait exacte : sa conservation seule ne protège pas les capacités.
  Ajout d'une réduction collective des arrivées après la normalisation des sources, par arrondi
  cumulatif entier. Régressions sur cinq capacités, plusieurs pas, réseau avec sorties et rejet
  extérieur, deux ordres d'arêtes ; volumes bornés et bilan exact. Voir L312 et
  [BILAN-GLOBAL-S227](BILAN-GLOBAL-S227.md#réception-des-corrections-de-code-p4).

- **A268** *(sévérité 2, S227 ; corrigée en S227)* — **La différence de coordonnées débordait
  avant son élargissement.** Deux valeurs `i64` extrêmes pouvaient faire paniquer le pas, en
  debug comme en release, même sous gravité verticale où leur composante horizontale n'influe
  pas sur le débit. Soustraction faite désormais en `i128` avant projection. Les deux sens des
  extrêmes retrouvent les transferts et restes du témoin ordinaire. Aucun seuil ou repère changé.

**Suivi A211 / A243 — S227, 2026-09-13 : pilotage refondu, efficacité encore à constater.**
Les points d'entrée passent de 6 161 à 384 lignes ; la file remplace ses états périmés, l'indicateur
sépare activité et capacité, les maillons exigent un effet aval prouvé. La troisième session d'un
même fil oblige à comparer sa suite aux autres capacités. Le dépôt ne déclare pas ces angles
clos : les prochaines livraisons permettront d'évaluer la mesure, sans ajouter un audit périodique.

**Suivi A266 — S228, 2026-09-13 : corrigée par ADR-139 et consommée dans le pas réel.**
Le plan est inversé depuis le volume coupé d'une partition tétraédrique, dans le repère fixe du
contenant. Prisme, cale, forme non convexe, fonds/plafonds et directions/azimuts reçus ; A266
active passe à **0 ml** au lieu de 506. L'ancienne table est refusée hors +Z. Le hublot S226 passe
de 1 829 à 1 736 ml parce que son décalage faux a disparu ; sa pente reste reçue. La conception
n'est plus limitée aux prismes. Voir [VOLUME-ORIENTE-S228](../validation/VOLUME-ORIENTE-S228.md).
L'état restaurable est désormais le prochain lot exécutable ; pas de campagne géométrique préalable.

- **A269** *(sévérité 2, S228 ; ouverte, refus explicite construit)* — **Un état en millilitres
  entiers ne rend pas son inversion géométrique précise au millilitre à toute échelle.** Un cube
  de 3 km admet la géométrie locale, mais `2^53+1 ml` n'est plus distinct de ses voisins en f64.
  Le contrôle compare au véritable entier et refuse `Resolution`, sans arrondir la demande.
  Les petits contenants reçus tiennent le demi-millilitre contre leurs oracles ; ce résultat
  ne borne ni l'erreur directe de toute partition ni les tailles encore non éprouvées. **À faire
  quand un consommateur sort du domaine reçu** : qualifier forme/taille contre une référence
  indépendante, puis construire une représentation ou un calcul suffisamment précis si le
  refus bloque cet usage. Le coût du pas complet se reçoit avec cette extension. La grande
  échelle reste obligatoire ; cet angle ne bloque pas par principe la restauration de V.

**Suivi A211 / A243 et A266 — S229, 2026-09-14.** La suite bornée « V restaurable » de S228
est construite et consommée dans C19-V local, sans campagne géométrique supplémentaire.
La priorité revient maintenant à J2/A244 (budget δ), comparaison portée au journal et à la file.
A269 reste ouverte sur son domaine ; ni les angles de pilotage ni I-03 multiplateforme ne sont
clos par cette seule livraison.

**Suivi A244 — S230, 2026-09-14 : arrêt coopératif reçu, I-05 complet toujours ouvert.**
Le candidat reçoit une enveloppe en ms, contrôle ses phases par tranches et restitue u/w/p
par échange O(1) si elle expire. 204 points d'expiration testés, temps restant explicite,
reprise identique et zéro allocation. Les appels de réduction sont bornés à64 cellules.
Premier passage64×32/2ms : maximum15,8501ms ; second2,0732ms. Cause du premier pic non
identifiée ; sa non-répétition ne le réfute pas. Admission de blocs/charge viable, marges et
qualification de l'hôte restent dues avant I-05 ; pression f32 reste le prochain lot I-08.
La file donne les déclencheurs, [BUDGET-DELTA-S230](../validation/BUDGET-DELTA-S230.md) les preuves.

**Suivi A244 — S231, 2026-09-14 : pression f32 reçue sur le domaine comparé.**
La conversion seule produisait une fausse convergence : résidu récurrent petit mais b−Ap
jusqu'à4,45e-6. Contrôle réel et correction CG sous plafond global conservent le seuil1e-6 ;
oracle indépendant, continuation, refus et zéro allocation reçus. Stockage réduit d'environ28%,
aucun gain de vitesse. [PRESSION-F32-S231](../validation/PRESSION-F32-S231.md).
I-05 reste ouvert ; I-08 global aussi, puisque l'API temporelle transporte encore du f32.
Les domaines non éprouvés ne bénéficient pas automatiquement de cette réception.

**Suivi A244 / S199-2 — S232, 2026-09-14.** Le défaut spatial annoncé sur le débit venait
de l'absence de pondération par ouvertures dans le banc : contre-épreuve sans modifier le
solveur, ordre≈1,95 retrouvé. Une incohérence géométrique distincte est corrigée : SUB8
supprimait des triangles fluides tout en gardant leurs faces ouvertes. Intégrale du profil
linéaire et bilan du pas reçus sur triangle et miroir. La portée reste celle de
[FLUX-COUPES-S232](../validation/FLUX-COUPES-S232.md) : ordre local, cellules extrêmes et
discontinuités non reçus. I-05/I-08 temporel inchangés ; ne pas prolonger un faux diagnostic.

**Suivi A244 — S233, 2026-09-14.** La première surface linéarisée évolue réellement :
pression, flux et hauteur du pas suivant. Durée/budget entiers, coefficients selon ADR-141 ;
anciennes API temporelles f32 toujours à migrer. Une hausse isolée du résidu f32 ne reçoit
pas une stagnation : corrections sous plafond global conservé. Les déplacements sous l'ulp
de la hauteur sont compensés en f32 et consommés par la pression. Onde sur1s reçue,
213 expirations atomiques, aucun stockage persistant δ. Coût64×32 : médianes2,86 puis6,21ms,
maximum8,998ms au second passage ; I-05 complet non reçu. Géométrie mobile/non-linéaire
et domaine d'amplitude/horizon restent à construire/qualifier ;
[SURFACE-LINEARISEE-S233](../validation/SURFACE-LINEARISEE-S233.md).

- **A270** *(sévérité 2, S234 ; ouverte)* — **L'alimentation du portable change le coût GPU d'un
  facteur 1,65, et aucun en-tête de mesure ne la publiait.** Même binaire S233, même pose,
  960×540 : **7,0145 ms sur batterie**, **4,2448 sur secteur**. ADR-131 D3 exige « machine » dans
  le domaine ; l'état d'alimentation n'y figurait pas, de S211 à S225. Les valeurs de S225
  (4,16 ms) sont retrouvées sur secteur à 1–2 % : il est **probable, non vérifié**, qu'elles aient
  été prises ainsi. La bascule a eu lieu *pendant* une campagne S234 et a rendu deux bancs
  incomparables ; elle n'aurait pas été vue sans le témoin rejoué en alternance. Gravité 2 : un
  verdict de coût comparé à une mesure prise dans l'autre état serait faux d'un facteur 1,65,
  dans un sens ou dans l'autre. **À faire à chaque mesure de coût** : publier l'état
  (`Win32_Battery.BatteryStatus`) au début et à la fin ; comparer à un témoin mesuré dans le
  même état ; profil d'alimentation et pilote restent non contrôlés. Voir
  [LOD-SILLAGE-S234](../validation/LOD-SILLAGE-S234.md) §4.3.

**Suivi A265 — S234, 2026-09-14 : pas de plancher indépendant du travail.** Avec la grille du
sillage, la passe d'eau tombe à 0,43 ms et l'intervalle de 5,16 à **2,60 ms** (384 Hz) ;
l'acquisition d'image passe de 2,43 à 0,02 ms. La borne vers 4,9 ms était donc l'attente du GPU,
pas un plancher de la chaîne d'échange. La trame est désormais bornée par le CPU (sillage 1,3 ms,
présentation 0,5). Reste ouvert : pourquoi le recouvrement variait avec la charge à GPU chargé.

**Suivi A247 — S234, 2026-09-14 : la passe d'eau de la scène J1 passe sous 2 ms.** Grille locale
du sillage à pas borné et reconstruction bicubique : 0,426 ms au banc, 0,429/0,456 ms en
fenêtre fixe/balayée, contre 4,24/4,14/2,87 pour le témoin S233 ; recette 128×256 à 1,51 ms. Le
verdict porte sur cette implémentation et ce domaine — un sillage, un impact, une machine
sur secteur. A247 reste partielle : scène multi-sources, visibilité, CPU et seconde cible.

**Suivi A255 / A261 — S235, 2026-09-14 : le déclencheur est atteint, et par les majorants seuls.**
Scène déclarée avant mesure : trois sillages d'un journal commun, huit impacts nés toutes les 4 s.
Le budget de pente refuse **49 instants sur 161** (pire 1,251 π/7 à 28,25 s). La pente réelle des
perturbations, balayée au GPU sur l'union des domaines puis raffinée à 2 cm, reste **≤ 0,2154 —
48 % de π/7** aux 49 instants : **aucun** refus de pente réelle, plancher 2,1 à 8,2 fois la
réelle. À chaque naissance le maximum réel est celui de l'impact neuf (exact à la naissance) ; le
plancher y ajoute l'enveloppe des trois sillages (0,175–0,202, réelle ≈0,07 en S222) et les
majorants des impacts anciens que l'inégalité d'ADR-138 ne retire pas. **Sévérité portée à 1 pour
J1** : une scène représentative ne passe pas l'admission, sans qu'aucune pente réelle ne
l'explique. Correction de portée de S222 : « huit impacts passent » valait pour des impacts âgés,
pas pour une scène dont les naissances se renouvellent. Voir
[SCENE-MULTI-S235](../validation/SCENE-MULTI-S235.md) §2, L314.

**Suivi A265 — S235, 2026-09-14.** Scène multi-sources pendant le forçage : la trame est bornée par
le CPU (préparation du sillage 3,12 ms, 204 Hz) et hors forçage 415 Hz ; hors champ 727 Hz. Le
recouvrement à GPU chargé n'est toujours pas expliqué, et aucune décision n'en dépend.

- **A272** *(sévérité 2, S237 ; fermée S238 par ADR-143 sur le domaine mesuré, voir suivi)* — **Au-delà du domaine reçu par S231, la pression f32 de δ
  plafonne au-dessus de son propre seuil, et le pas refuse.** Onde stationnaire de 5 cm, grille
  128×144 (16 384 mailles fluides), quart de période, surface plate : résidu relatif figé à
  **1,0492·10⁻⁶** pour un seuil de 10⁻⁶, à 4 000, 16 000 et 64 000 itérations ; divergence après
  correction 1,45·10⁻⁷, physiquement excellente. S231 n'avait reçu que jusqu'à 8 192 mailles, avec des
  résidus déjà à 8,75·10⁻⁷, et l'écrivait. Le même pas refuse à `θ_min = 10⁻²` : la géométrie mobile n'est
  pas en cause ; le mode linéaire converge sur ce banc à 128 colonnes (nz = 128), mais aucune loi du
  plancher en fonction de la taille n'est établie. Gravité 2 : toute grille plus grande — la 3D d'abord —
  refusera des pas sporadiques, et **dix itérations ou dix mille ne changent rien**, puisque le solveur
  stagne ; le coût d'un pas refusé est celui du plafond entier (4 s ici). **À faire** : mesurer le
  plancher en fonction du nombre de mailles et de la structure du second membre ; choisir un remède reçu
  — critère d'arrêt fondé sur une grandeur physique (divergence, débit) plutôt que sur le résidu
  relatif seul, résidu compensé, détection de stagnation qui refuse tôt, ou solveur mieux conditionné —
  sans relâcher le seuil pour faire passer le banc. Voir
  [SURFACE-MOBILE-S237](../validation/SURFACE-MOBILE-S237.md) §6, PRESSION-F32-S231.

- **A271** *(sévérité 2, S236 ; corrigée par ADR-142 pour qui emploie le mode union)* — **La requête
  composée du cœur et l'image ne décrivaient pas la même eau.** Depuis ADR-126 (S203), l'image rend
  « hors emprise, B seul » ; la requête mixte d'ADR-080 refuse tout point qu'une seule emprise ne couvre
  pas. À un impact la différence était invisible ; sur la scène représentative S235, huit disques sur
  70 m, l'intersection est presque vide et la requête ne sert presque aucun point que l'image dessine.
  Trente-trois sessions l'ont laissée passer parce que les montages vérifiés n'avaient qu'un impact, ou
  des impacts confondus. Mode union ajouté (ADR-142) : **écart à la somme de référence de l'image
  < 1e-9 m** sur 6 988 sondes et 161 instants. Le mode intersection reste celui des consommateurs
  existants ; le choix du mode par un hôte autoritaire n'est pas tranché. Voir L315,
  [ADMISSION-UNION-S236](../validation/ADMISSION-UNION-S236.md).

**Suivi A255 / A261 — S236, 2026-09-15 : la scène représentative est admise, A261 reste entière.**
Sur l'union et sous plancher certifié (séparation 2D, pression locale ADR-137 sur les cellules
critiques), S235 est admise aux **161 instants** par le cœur, pire plancher 0,9995 π/7. Une somme
d'impacts exacte en position refusait encore 31 instants : c'est la pression **locale** qui suffit, et
seulement près des impacts forts. A261 — aucune enveloppe de modules ne voit la localisation d'un
paquet — reste vraie partout ailleurs ; une scène où la pression seule sature n'y gagnerait rien.
Sévérité d'A255 ramenée à 2 pour J1. **A258 entre dans l'admission** : ADR-137 est une réception
numérique, pas un certificat f32, et le plancher en hérite sur les cellules critiques.

**Suivi A272 — S238, 2026-09-15 : fermée sur le domaine mesuré, par ADR-143.** Au refus, l'erreur inverse
composante par composante valait **une unité d'arrondi** et l'état de pression parcourait un cycle exact
(période 420 dans un montage, plus de 16 000 itérations dans le banc) : aucun nombre d'itérations n'avait
d'issue. La pression s'arrête désormais quand son vrai résidu est indiscernable de l'arrondi de son propre
calcul (`ω ≤ γ₈`, dérivé de la ligne à quatre faces) ou quand l'état revient au bit, et le pas n'est reçu
que si la divergence tient la tolérance déclarée par S199 (10⁻⁵). Onde de 5 cm **reçue à 128 colonnes**
(profil 0,25 %, harmonique 0,71 %, pire pas 526 itérations) ; contre la solution f64 du même système,
vitesse à 5,5·10⁻⁸. Prix publié : les trajectoires S237 à 32 et 64 colonnes changent de bits (pas
arrêtés une ou deux itérations plus tôt), chiffres de réception inchangés. **Limites** : opérateur 2D
seulement (en 3D, `γ₁₀` à dériver avec l'opérateur) ; une forme de second membre au-delà de 16 384
mailles ; résidu relatif des pas reçus jusqu'à 3·10⁻⁶. Voir L317,
[PRESSION-PLANCHER-S238](../validation/PRESSION-PLANCHER-S238.md).

- **A273** *(sévérité 2, S238 ; ouverte)* — **Le critère d'arrêt premier de la pression δ ne garantit pas
  la tolérance physique de la projection.** Des pas **convergés** selon le résidu relatif 10⁻⁶ rendent une
  divergence mise à l'échelle de S199 de **1,02·10⁻⁵ à 128×64 sur la bosse** (`delta_precision`, dans le
  domaine reçu par S231) et **1,58·10⁻⁵ à 256×128** (famille S231), au-dessus des 10⁻⁵ déclarés avant
  construction. S231 n'appliquait ce critère qu'à 32×16 ; aucun banc ne l'a vérifié depuis aux tailles
  supérieures. La divergence croît avec la taille à résidu relatif constant : par l'identité
  `div u = r/scale`, c'est une norme maximale du résidu, qu'une norme euclidienne relative ne borne pas.
  Gravité 2 : toute grille plus grande, la 3D d'abord, s'éloignera de la tolérance sans que le rapport le
  dise. **À faire** : mesurer la loi divergence/taille à résidu relatif fixé, puis soit un arrêt qui tient
  la tolérance physique (en gardant le plancher d'ADR-143), soit une requalification datée de la tolérance
  avec provenance — jamais un seuil déplacé pour faire passer les cas. Voir PRESSION-PLANCHER-S238 §4.4.


**A282 — suivi S249, 2026-09-16 (sévérité 2, partielle).** La coupure lointaine est
construite et reçue pour B et le sillage, consommée par défaut par le rendu (ADR-148).
Les impacts tabulés restent non filtrés. Les normales de réflexion omettent la dérivée
spatiale du filtre de caméra, et les huit bandes atténuent parfois des modes résolubles.
Le mouvement et les reflets ne sont pas reçus perceptivement ; aucune absence globale
d’alias n’est revendiquée. Déclencheur : prochaine extension de J1 ou critère de réflexion.
Le coût GPU augmente de 0,76–0,85 ms en régime ; premier passage à 2,26 ms. Optimisation
de cuisson si ce poste devient prioritaire. [Preuve](../validation/COUPURE-S249.md).


**A283 — S250, 2026-09-16 (sévérité 2, ouverte).** Le premier raccordement B/W réel
au MAC refuse le démarrage à perturbation et hauteur imposée nulles, à seulement
16×8 et 32×16. La projection atteint son plancher mais D vaut 1,42·10⁻⁴ / 8,06·10⁻⁵,
au-dessus de 10⁻⁵. Ajouter une hauteur imposée de 1 cm permet vingt pas : ce n'est
pas une correction du défaut. Les champs publiés restent intacts au refus.
Source proche d'un gradient et normalisation par la petite vitesse projetée : suspects
à départager par une projection f64 indépendante. A275 résolvait une autre famille
de second membre ; sa fermeture ne couvre pas ce cas. Déclencheur S251, avant extension
du couplage. Ne pas relever un seuil ni effacer S. [Preuve](../validation/RACCORDEMENT-DELTA-S250.md).


**A283 — suivi S251, 2026-09-16 (sévérité 2, close).** Cause isolée par une projection
f64 indépendante : la projection retranche ~98 % de la vitesse prédite, et l'erreur
absolue de cette soustraction f32 domine la petite vitesse restante. ADR-150 ajoute
**un** affinage de divergence sur la vitesse corrigée, déclenché seulement au plancher
refusé. Reçu à 16×8 et 32×16 contre l'oracle (vitesse ≤2,15·10⁻⁶ relatif, D ≤5,8·10⁻⁸),
trajectoires de vingt pas, reprise au bit, zéro allocation ; tolérance et S inchangées.
Limites : deux tailles, pas de garantie qu'un seul affinage suffise à toute entrée,
surface mobile non couplée. [Preuve](../validation/DEMARRAGE-PLAT-S251.md).

**A284 — S251, 2026-09-16 (sévérité 2, ouverte).** Le démarrage plat reçu coûte **×40**
à taille égale : à 32×16, médiane 43–44 ms contre 1,09 ms sous hauteur imposée.
Dix-sept pas sur vingt font 581 à 672 itérations contre 69 à 73. Les pas 7–16, **non
affinés**, coûtent autant que les affinés : ce n'est pas le prix d'ADR-150. Les pas
17–19, toujours au plancher, reviennent à 82 itérations sans explication. Répartition
entre projection ordinaire et repli multigrille non mesurée ; toutes repartent de p=0.
Or un domaine perturbatif **naît** à δ=0 (I-12) : ce régime est celui de chaque création.
Déclencheur : avant tout banc de coût du raccordement ou extension de taille, attribuer
les itérations par projection ; puis éprouver les techniques absentes (ADR-131).
Ne pas relever la tolérance ni réduire le plafond pour abaisser le coût. Distinct
d'A276 (loi de taille) : ici la taille est fixe. [Mesure](../validation/DEMARRAGE-PLAT-S251.md).

**A284 — suivi S252, 2026-09-16 (sévérité 2, close).** Attribuée par une trace de test autour de
chaque projection du pas couplé. Le repli multigrille faisait **94 %** du pas plat 32×16 :
500 itérations et ~39 ms. La projection ordinaire (~81 itérations) et l'affinage (~90)
coûtaient chacun ~1 ms. La cause est un défaut, A285, et non le régime au plancher. Corrigé,
le pas plat coûte 2,53 ms de médiane contre 1,15 sous hauteur imposée ; l'écart restant
tient au repli et à l'affinage. [Mesures](../validation/MULTIGRILLE-BETA-S252.md).

**A285 — S252, 2026-09-16 (sévérité 2, close le jour même).** Le gradient conjugué préconditionné
par la multigrille formait `β = ‖r_{n+1}‖²/⟨r_n, z_n⟩` avant le cycle, depuis S245 P5
(`6dc0bfa`), alors que le commentaire du code donnait la bonne formule. Symptôme : premier vrai
résidu relatif 0,616 quand la récurrence s'arrête, 25 relances, 500 itérations. Le défaut est
resté invisible pendant sept sessions, pour trois raisons. Les portes d'acceptation recalculent
le vrai résidu, donc aucun résultat faux n'a été publié. Les relances faisaient progresser le
calcul malgré tout. Et S245 a interprété un compte « plat et haut » comme un défaut de
transfert. Portée : les comptes et coûts « avec » de S245/S246 et la prémisse de vitesse
d'ADR-147 étaient faux, et l'explication d'A275 aussi. Corrigé et reproduit par un essai qui
échouait avant. À 32 768 mailles, le pas corrigé serait redevenu refusé : ADR-151 le reçoit
par affinage. **Leçon** : un préconditionneur n'est éprouvé qu'avec le solveur qui l'emploie.
L'essai de symétrie du cycle passait ; aucun essai ne confrontait la récurrence au vrai résidu. L328.
[Preuve](../validation/MULTIGRILLE-BETA-S252.md).

**A286 — S253, 2026-09-16 (sévérité 2, ouverte).** Le pas couplé mobile (ADR-152) exige un fond
prolongé au-dessus du plan moyen **de façon incompressible**, avec un `S` calculé sur ses propres
champs. Le fournisseur B refuse `z > 0` (ADR-113). Le prolongement de Taylor d'ordre un n'est pas
incompressible (`div = z·U_xz`). Le prolongement analytique `e^{kz}`, employé par l'oracle S253 pour
un seul mode, amplifie les composantes courtes d'une mer large bande sous les crêtes des longues.
La réception S253 ne vaut donc que pour ce fond d'oracle : la vraie mer B/W n'est pas raccordée.
Déclencheur : S254, avant tout domaine couplé sous B réel. Choisir un prolongement borné et
incompressible (par exemple `U` constant et `W` fermé par continuité), et le recevoir contre
l'oracle S253. [Preuve](../validation/SURFACE-COUPLEE-S253.md).

*S254, 2026-09-16 — partielle.* **B prolongé** par la règle d'ADR-154 (vitesse horizontale
constante, `W` par continuité, `P` de Taylor d'ordre un), reçu contre l'oracle S253 : à 128
colonnes, 0,168 % / 0,58 % à 5 cm et 0,244 % / 0,66 % à 10 cm, décroissants. Le fournisseur de
production est identique au bit sous le plan moyen, et son intégration dans le pas couplé est
reçue. **Restent** : les couches W au-dessus du plan moyen, et la précision sous une mer large bande
réelle, qui attend les frontières du total et les bords ouverts.
[Preuve](../validation/PROLONGEMENT-FOND-S254.md).

**A287 — S256, 2026-09-16 (sévérité 2, ouverte).** **La rugosité de B est incomplète, et ses directions
sont liées à la fréquence.** Trouvé par la revue visuelle R1 (« la mer est trop lisse, on dirait un
lac »), confirmé par mesure : la `mss` de la recette J1 vaut 0,0075, contre 0,044 selon Cox–Munk au
vent minimal de `Hs` 1,5 m. La queue du même spectre en pentes par pixel (ADR-155) la porte à 0,0195.
Deux causes restent. (1) La forme JONSWAP en `f⁻⁵` plafonne à 52 % de la rugosité observée, même
jusqu'à la limite gravité-capillarité : le spectre des ondes courtes n'est pas modélisé. (2) La
cuisson donne à chaque composante une direction qui suit son rang, donc sa fréquence : la queue
dessine des stries parallèles (R2). Ce n'est pas une loi d'étalement. Déclencheur : le verdict de
l'utilisateur sur R2, ou le prochain travail sur B. Remède attendu : une loi d'étalement
directionnel dépendant de la fréquence, et un modèle de spectre court choisi et reçu contre
Cox–Munk. Pas un coefficient forcé. [Mesure](../validation/REVUE-VISUELLE.md) §7,
[réception](../validation/QUEUE-SPECTRALE-S256.md).

*S259, 2026-09-16 — partielle.* **Directions traitées** : la loi `cos^2s` de Mitsuyasu (s_max de
Goda) et un tirage de Weyl indépendant du rang (ADR-156). Sur la mer de vent, le Spearman rang/direction
passe de 1,000 à 0,012, et les stries disparaissent de R3. Pour la scène `--houle` seulement : la
scène par défaut garde la fixture V1 au bit. **Reste** : la rugosité, à 52 % de Cox–Munk au mieux.
[Réception](../validation/MER-MULTIMODALE-S259.md).

*S260, 2026-09-17 — rugosité atteinte sous `--vagues`.* La queue d'équilibre en `f⁻⁴` (ADR-157) porte
la `mss` à 0,0495, contre 0,0437 observé. Avec CWM, `c40`, `c22` et `c04` tombent dans les
incertitudes de Cox–Munk, sans ajustement. **Restent** : l'alignement des ondes courtes sur le vent
(`σu²/σc²` 0,96 contre 1,37, étalement gelé trop large), l'asymétrie des pentes (`c03` −0,22) et
l'asymétrie de l'élévation (`λ3`). [Mesure](../validation/REVUE-VISUELLE.md) §10.

**A288 — S260, 2026-09-17 (sévérité 2, ouverte).** **Sous `--vagues`, la surface rendue n'est pas celle
que le jeu interroge.** CWM déplace les sommets de `D_B` jusqu'à 1,75 m, et l'écart vertical avec
la requête eulérienne linéaire atteint 0,365 m aux sondes. Les couches W, elles, restent évaluées
au point de Lagrange. Un objet flottant posé par la requête serait visiblement décalé de la
vague. Déclencheur : avant tout consommateur de jeu sous `--vagues`, ou son adoption par défaut.
Remède attendu : une requête eulérienne CWM (inversion de `x = α + D(α)` par point fixe, bornée), et
W composé dans la même géométrie. [Réception](../validation/VAGUES-POINTUES-S260.md) §3.

*S261, 2026-09-17.* La rugosité est **ajustée à Cox–Munk** (ADR-158, `--modulation`) : coupure à 28 fp et
modulation par la bande `M` = 2. `mss` 0,0435 ; `c40`, `c22`, `c04` à 0,353, 0,129 et 0,351. **Constat
nouveau** : Cox–Munk borne la modulation, et une surface franchement lisse entre les pics (verdict
R4) n'est pas compatible avec l'observation à 8 m/s. Restent donc, en plus de l'anisotropie et des
asymétries : le **vent de la scène** comme paramètre, et la **transition des ondes non résolues
vers la BRDF** contre le grain proche (Bruneton et al., 2010).

*S262, 2026-09-17 — **A288 close**.* `Background::cwm_query` inverse le déplacement de Lagrange par
Newton (ADR-159). Contre l'image GPU, sur 5 592 sondes, la hauteur est à 0,30 mm de la surface
rendue, contre 0,365 m pour la requête linéaire ; aucun refus, au plus 3 itérations. Les couches W
se composent au point de Lagrange. [Preuve](../validation/DEFAUTS-S262.md) §3.

*S263, 2026-09-17.* **Le vent est un paramètre de scène** (ADR-160) : Pierson–Moskowitz pour la mer de vent,
coupure de la queue à la `mss` de Cox–Munk au même vent. La rugosité est donc conforme à l'observation à
3, 5 et 8,37 m/s. **Constat nouveau** : la pointe `c40` décroît avec le vent (0,12 à 3 m/s), sous la
borne de Cox–Munk à faible vent, parce que `M` = 2 est un ajustement à 8 m/s. Le vent de la scène
représentative attend le choix de l'utilisateur (R6). Suite 470 / 18 / 0.

*S265, 2026-09-17 — A287 partielle, sévérité 2 conservée.* Verdict R6 : zones entre pics moyens
et petits trop rugueuses, aucun vent choisi. ADR-161 construit une transition de la queue filtrée
vers une covariance de pentes dans l'éclairage ; 15 000 sondes GPU reçues, témoin R6 au bit.
R7 en attente. Fermeture gaussienne au premier ordre sous CWM, pas de masquage microfacette,
variance perdue de la bande géométrique non transférée. Quadrature 3×3/5×5 encore différente,
surtout vue haute (P99 RGB 25/255), budget eau dépassé à 2,49–2,71 ms en 3×3. Réception temporelle
absente. Déclencheur : verdict R7 ; si aspect accepté, intégrer/précalculer l'éclairage pour réduire
coût et erreur de quadrature avant adoption. [Preuve](../validation/REFLETS-S265.md).

*S266, 2026-09-18 — A287 toujours partielle.* R7 accepté pour la vue présentée. Sommes
suffixes et boucles 3×3 fixes (ADR-163) reçues : 10–17 % de coût GPU en moins, sept images
à un niveau RGB près, 15 000 sondes conservées. GPU eau 2,24–2,26 ms, dont cuisson sillage
1,06–1,08 ms, budget non tenu. Cache ciel ADR-162 rejeté (précision et gain), retiré du code.
La quadrature, la BRDF et la réception temporelle gardent leurs limites S265 ; coût du sillage
prioritaire avant approfondissement des reflets acceptés. [Preuve](../validation/CIEL-CACHE-S266.md).


*S267, 2026-09-18 — A282 partielle et A265 ouverte (sévérités conservées).* Cuisson
spectrale par huit accumulateurs explicites : mêmes grilles et sept images au bit,
36 cas S249 conservés, −46 % cuisson et −22 à −23 % eau totale. Aucun changement
d'I-09. Le rendu R7 accepté est conservé, sans fermer les limites physiques d'A287.
GPU médian ~1,74 ms, pointe 2,962 ms au premier passage ; second maximum 1,836 ms.
CPU médian ~4,1 ms, maxima ~26 ms. Pas d'attribution aux allocations, au pilote ou au
système sans mesure ; budget global et borne par image non reçus. Déclencheur :
prochain lot de coût après les bords ouverts J2, avec séparation des postes CPU/GPU.
[Preuve](../validation/CUISSON-SILLAGE-S267.md).


**Suivi A92/A50 — S268, 2026-09-18 (sévérités conservées).** Le pas mobile couplé
amortit désormais η' en plus du prédicteur de vitesse (ADR-164) ; étape locale et
transaction reçues, fond préservé. Cela ne reçoit ni réflexion ni largeur d'éponge :
la fermeture extérieure reste réfléchissante, aucune transduction vers W n'existe.
Un fond traversant et les frontières du total restent hors réception. Déclencheur
immédiat S269 : paquet sortant avec mesure intégrée à jauge et garde contre la
contamination des fenêtres selon ADR-046 ; ne pas hériter du 1 % du véhicule 1D.
[Preuve](../validation/RELAXATION-SURFACE-S268.md).


**Suivi A92/A50 — S269, 2026-09-18 (sévérités conservées).** Réception bornée du
bord absorbant : R_diff 0,00144442 / 0,00161565 à dx 0,25 / 0,125 ; doubles gardes
et identité incidente reçues. Signal brut contaminé par queue numérique et retour
gauche : il reste non recevable comme réflexion. Les témoins doivent partager le
bord gauche, sinon l'éponge symétrique fausse l'attribution au bord droit.
[Preuve](../validation/REFLEXION-PAQUET-S269.md). Déclencheur S270 : fond traversant
et flux de bande aux frontières ; pas de fermeture A92/B2 ou B4 global. Autres
spectres et horizons : nouvelle garde avant toute extrapolation de ce résultat.


**Suivi A92/A50 — S270, 2026-09-18 (sévérités conservées).** Le flux de bande était
fermé avec la perturbation : un courant/niveau constants créaient ±dt Ua/dx aux
extrémités. Défaut reproduit puis corrigé (ADR-165), deux mailles, deux signes,
bilan signé et 638 expirations/reprises sans allocation reçus. La fermeture de v
reste distincte : aucune réception de houle progressive entière ou de W(b)≠0.
Déclencheur S271 : onde progressive de profondeur finie, référence indépendante,
contrôle des perturbations induites et des fenêtres avant verdict. Spectres et
horizons hors S269 restent à recevoir avant extrapolation, sans campagne générique.
[Preuve](../validation/FOND-TRAVERSANT-S270.md).


**Suivi A92/A50 — S271, 2026-09-18 (sévérités conservées).** La houle linéaire
progressive induit un résidu cinématique non nul : le comparer à zéro serait
un faux oracle. Identité locale W(ζ)−W(0)−U(ζ)ζ_x reçue au démarrage, erreur
3,05 / 1,24 / 0,58 % aux trois mailles, contre témoin à flux de bord fermé.
Le pas réel tend vers ce transport à petit dt. Pas d'ordre deux spatial ni de
réception après propagation revendiqués. Déclencheur S272 : oracle temporel
indépendant du résidu d'ordre deux, puis fenêtre sans retour contaminant.
[Preuve](../validation/HOULE-PROGRESSIVE-S271.md).


**Suivi A92/A50 — S272, 2026-09-18 (sévérités conservées).** L'oracle modal
Neumann d'ordre deux reçoit ses contrôles indépendants, mais le résidu progressif
MAC manque 2 % : 8,68 % à dx0,03125 sur 2 s. Demi-dt 8,62 %, sensibilité 0,669 % ;
amplitude moitié 6,58 %, normalisation a² variant de 2,392 % : ne pas tout attribuer
au solveur quand l'ordre deux n'est pas qualifié. Erreur de quadrature verticale
isolée : bande partielle évaluée au centre de cellule, 1,60 % contre intégrale
exacte ; reconstruction linéaire 0,0162 % sur ce flux. Déclencheur S273 : intégrer
la reconstruction au transport réel (intérieur/bords), puis refaire le verdict,
sans extrapoler ce gain isolé à l'erreur temporelle. [Preuve](../validation/RESIDU-TEMPOREL-S272.md).


**Suivi A92/A50 — S273, 2026-09-18 (sévérités conservées).** La quadrature de
bande est d'ordre deux dans le produit (ADR-166) : flux 0,0162 % au lieu de 1,60 %
à la maille fine, fond affine exact. Résidu progressif sur 2 s : 9,11 / 6,18 /
5,88 %, encore au-dessus de 2 %. L'écart des champs divisés par a² vaut 2,74 /
2,85 / 2,89 % quelle que soit la maille : c'est l'ordre trois absent de l'oracle,
pas le pas. La part extrapolée `2N(a/2) − N(a)` converge vers 1,77 %, erreur
temporelle comprise. Déclencheur S274 : protocole de réception sur cette part,
écrit avant mesure, à dt 1 ms avec contrôle 0,5 ms ; sinon oracle d'ordre trois.
[Preuve](../validation/BANDE-LINEAIRE-S273.md).


**A289 — S274, 2026-09-18 (sévérité 2, ouverte).** **Un domaine δ fidèle dérive en phase de B.**
B est linéaire (ADR-113) ; la vraie vague non linéaire va plus vite de `ω₂ ≈ 0,5–0,6 (ak)² ω`
(Stokes, ordre trois). Le pas couplé porte cette dérive : 0,85 à 0,87 fois Stokes sur le banc
progressif, indépendant de la maille. `η'` croît donc comme `a·ω₂·t` : sous `ak` = 0,06 et
T = 7 s, environ 7 cm en une minute, par la formule seule. Trois conséquences : `|δ|` cesse d'être
petit devant B (ADR-001), couture de phase à la bordure, surface rendue différente de la surface
de jeu B+W (famille d'A288). Déclencheur : premier domaine appelé à vivre plus de dix périodes
sous `ak` ≥ 0,05, ou premier rendu de δ sous houle. Options : rappel lent de la grande longueur
d'onde de `η'` vers B, durée de vie bornée et recréation (I-12), ou dispersion d'amplitude dans B
(ADR nécessaire). [Mesure](../validation/HOULE-USAGE-S274.md) §3–§7.


**Suivi A276 et A92/A50 — S274, 2026-09-18 (sévérités conservées).** A276 : le pas couplé
mobile passe de 280 à 49 ms à 16 384 mailles par la multigrille du mode mobile (ADR-167), sans
changer ses réceptions ; il reste ≈ 24 fois le budget par pas, et la précision à un pas par image
n'est pas mesurée. A92/A50 : le banc de houle progressive est arrêté au niveau d'usage (écart brut
5,8 µm sur 1 cm, coefficient d'ordre deux 0,88–1,16 %) ; la réception au budget d'ADR-120 pour
les mers cambrées est différée avec déclencheur. [Usage](../validation/HOULE-USAGE-S274.md),
[coût](../validation/COUT-MOBILE-S274.md).


**Suivi A276 — S276, 2026-09-18 (sévérité conservée).** δ tourne en direct dans l'afficheur :
40 images/s à 6 656 mailles, 21,7 ms par image, zéro allocation. L'échantillonnage du fond, poste
dominant (33 ms), passe à 5,1 ms par grille, identique au bit ; le pas couplé passe de 24 à
17,5 ms en partant de la pression publiée (ADR-169). Reste ≈ 11 fois le budget ; absents : cadence
découplée, GPU, 3D. [Mesure](../validation/COUT-DIRECT-S276.md).

**Suivi A276 — S282, 2026-09-19 (sévérité conservée).** La médiane de coût de l'hôte ne reçoit
plus de mesures fictives aux appels sans pas réussi (naissance, pause, saut, refus). Deux
régressions échouent avant correction et passent après, dans le chemin `Layer::update`.
Aucun gain de vitesse revendiqué : la médiane ne reçoit pas le 99e centile, le coût des échecs
n'est pas qualifié, et l'oubli par appel refusé reste sensible à la cadence d'appel. Déclencheur :
avant cadence découplée ou multiplication des domaines ; I-05 reste non reçu pour cet hôte.
**A290 — S283, 2026-09-19 (sévérité 2, ouverte).** **Rétrécir un domaine perturbatif n'est
pas gratuit à un instant arbitraire.** Sur l'onde S277 à 1,024 s, transfert amorti 256→128 m :
saut de hauteur publiée 33,447 mm et écart central au témoin large de 21,255 mm dans les
2,048 s suivantes. Le garde intégré borne la hauteur instantanée à 3 mm et refuse ce cas
(borne 33,729 mm), sans publier ni perdre l'ancien domaine. Il n'assure ni les pentes/reflets
ni l'évolution après un transfert admis ; I-12 perceptif reste ouvert. Déclencheur : avant
réduction automatique/non focale, préparer l'amortissement dans le temps et recevoir perte,
reprise et rendu. Agrandissement et domaine mobile également non reçus.
[Contrat, contre-exemple et coût](../validation/RETRECISSEMENT-S283.md).

**Suivi A276 — S283, 2026-09-19 (sévérité conservée).** Réduction effective du calcul,
6 656→3 328 cellules : médiane 25,5464→12,3784 ms, p99 observé 32,9817→15,8659 ms ;
128 pas sous le même fond, sur secteur, diagnostic forcé de transition normalement refusée.
Transfert et garde 3,0283 ms (transfert seul 0,0276 ms au premier passage). Toujours hors
2 ms, aucun gain de mémoire : le volume large reste réservé. La baisse de coût ne reçoit
pas la qualité perdue (A290) ni la commande globale sous I-05.
**Suivi A290/A276 — S284, 2026-09-19 (sévérités conservées).** Préparation progressive
consommée : correction nodale 1,503 mm maximum (1,5 mm + arrondi f32), garde S283 inchangé,
demande à 1,024 s, réduction autorisée à 1,792 s. Zéro allocation mesuré sur 321 updates.
**Écart central maximal 66,994 mm** au large intact jusqu'à 5,120 s : ni fidélité temporelle ni
I-12 reçus. Ne pas comparer ce maximum à celui S283 sans égaliser les fenêtres. Coût complet
médian 12,3857 ms, p99 42,0321 ms et maximum 43,1006 ms ; I-05 non reçu. Déclencheur : avant
automatisation, séparer l'effet de la préparation de celui du domaine étroit avec un témoin
large préparé. La correction nodale ne borne pas tout l'interpolant entre colonnes.
[Mesure et limites](../validation/PREPARATION-RETRECISSEMENT-S284.md).

**Suivi A290 — S285, 2026-09-19 (sévérité 2, ouverte).** Trois trajectoires appariées :
la préparation seule produit déjà 8,118 mm au centre lors de la permutation, et jusqu'à
45,517 mm sur la fenêtre ; l'effet supplémentaire du domaine réduit atteint 25,391 mm.
Ces maxima ne sont pas additifs. Préparation arrêtée au même pas, champs identiques au bit
avant transfert. Le garde instantané ne reçoit pas la trajectoire préparatoire. Diagnostic
local terminé ; correction différée à la réduction automatique, après priorité coût A276.
[Attribution et limites](../validation/ATTRIBUTION-RETRECISSEMENT-S285.md).

**Suivi A276 — S286, 2026-09-19 (sévérité conservée).** L'oubli dépend désormais du temps
simulé non financé, pas du nombre d'appels ; défaut reproduit puis corrigé dans Layer.
La cadence 32/48 ms réduit la moyenne sur l'onde à 13,15/9,11 ms mais laisse des pas >32 ms,
et 4,445/8,916 mm de différence même aux instants calculés. Pas d'activation interactive.
Coût d'échecs représentatifs toujours non mesuré (aucun refus pendant ce banc) ; seuils
de pente et réception visuelle ouverts. Prochain levier : coût par pas, candidat GPU face
aux passes CPU ; cadence à reprendre avec intégration temporelle reçue ou solveur moins cher.
[Protocole et limites](../validation/CADENCE-DELTA-S286.md).

**Suivi A276 — S287, 2026-09-19 (sévérité conservée).** Inversion des boucles des passes
multigrilles et de l'opérateur fin essayée puis retirée. Identité des hauteurs/vitesses sur
six trajectoires, itérations et allocations inchangées, mais plages de coût moyen complet
qui se recouvrent après A/B/B/A. Pas de gain livré. Suite : contrat d'un candidat de pression
résidente GPU, opérateur/lissage contre CPU et coût incluant transferts/synchronisations,
avant cycle complet et intégration. Pas de preuve GPU acquise par la seule présence de wgpu.
[Mesures et limites](../validation/PASSES-PRESSION-S287.md).

**Suivi A276 — S288, 2026-09-19 (sévérité conservée).** Opérateur mobile et lissages GPU
reçus contre CPU, 30 cas, erreur normalisée ≤1,686·10⁻⁷ ; export natif au bit. 32 lissages
à 128×52 : coût complet 1,10–1,24 ms ; à 256×128 : 2,40–2,44 ms, maximum 4,22 ms sur ce lot.
Préparation comprise et sans allocation, mais pile/banc jusqu'à 105 allocations/appel.
Ni solveur complet, ni certificat de convergence, ni pas intégré, ni I-05/I-06 globaux reçus.
Déclencheur : construction du solveur résident puis intégration, avec réduction/cycle, refus
atomiques et coût réel ; ne pas extrapoler un gain de noyau au pas ou à toute l'eau.
[Réception et limites](../validation/PRESSION-GPU-S288.md).

**Suivi A211 / A243 — S293, 2026-09-19 (sévérité 1, ouverte).** La refonte S227 n'a pas tenu sur
ce point. S283 → S292 : **dix sessions sur dix** ont pris pour sujet la suite déclarée par la
précédente, à travers deux agents (Codex puis Claude) ; la règle des maillons ne l'a pas arrêté,
parce qu'une optimisation consommée par l'afficheur remet le compteur à zéro (S289, S290,
S291). Le chaînage a aussi produit un **verrou d'ordre sans dépendance** : « A276 avant la 3D »
(S249, déclencheur écrit en S276), alors que la porte C se reçoit sur la scène de la porte B et
que le livrable de J2 est tridimensionnel. Cinquième constat du mécanisme (S69, S145, S198,
S227, S293). Ce qui est nouveau : les portes de §3 bis donnent depuis S281 un critère « reçu si »
par système, auquel ancrer le choix du lot et la remise à zéro des maillons.
[BILAN-GLOBAL-S293](BILAN-GLOBAL-S293.md) §3.1 et §3.4.

**A295 — S293, 2026-09-19 (sévérité 1, ouverte).** **L'architecture d'exécution de δ ne peut pas
porter un domaine 3D dans le budget.** ADR-173 garde sur CPU, à chaque pas, le résidu
`b − A·p`, les itérations restantes et les portes d'ADR-143/144 ; advection et couplage y sont
aussi. Extrapolé des coûts par maille de S291 (0,23 µs pour les postes CPU du pas mobile,
≈ 0,9 µs pour le pas couplé) : 30 à 120 ms de CPU par pas pour 64×64×32 mailles, 200 à 800 ms
pour la bande actuelle étendue en 3D — **estimation, non mesure**. La porte C nomme « δ sur GPU
(décision d'architecture, ADR) », jamais prise. Découvert tard, cela coûterait la réécriture d'un
solveur 3D écrit dans le cœur CPU. Déclencheur : **avant d'écrire le premier domaine 3D** — ADR
d'architecture et classe de fidélité par couche, soumis à l'utilisateur (Q4 du bilan).

**A296 — S293, 2026-09-19 (sévérité 2, ouverte).** **Le profil de 2 ms ne donne aucune part à δ
ni de cible matérielle.** ADR-125 compte toute l'eau en somme CPU + GPU, sans répartition (note
S207) ; J1 seul en consomme l'essentiel (GPU 1,74 ms + CPU 4,1 ms, S267). Avec ADR-131, qui laisse
la liste des techniques ouverte, la porte C n'a **aucun critère atteignable** : le coût ne peut
être ni reçu ni déclaré incompatible, ce qui entretient le fil de coût. Mesures sur un portable
(A270, S292). Déclencheur : réponse de l'utilisateur à Q1 du bilan ; aucune part inventée d'ici là.

*S294, 2026-09-19 — **A296 close**.* Arbitrage de l'utilisateur, [ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) :
la machine de référence est nommée, le temps de l'eau sert l'objectif, et le profil de travail D3
donne à δ ≤ 2 ms GPU. La porte C a désormais un critère atteignable ; il reste à l'atteindre.

*S294, 2026-09-19 — **A295 décidée**, construction à venir.* [ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) :
la production de δ devient un pas résident sur GPU à travail borné, la référence CPU sort de la
boucle d'image et juge la production à la réception. En relisant les contrats, un second défaut du
même fil : le chemin S289–S291 attendait la carte à chaque pas, contre SPEC-004 §8.4. A295 reste
ouverte jusqu'à la porte B reçue.

*S296, 2026-09-19 — suivi A295, toujours ouverte.* Référence CPU 3D à surface mobile
reçue, après le mode linéaire S295 : [preuve](../validation/DELTA3D-MOBILE-S296.md).
Le couplage B/W, la production résidente et la scène restent à construire ; la réception CPU
ne ferme ni A295 ni la porte B. A274 conserve sa portée : la loi du plancher des lignes
fantômes en fonction de θ et de la taille n'est pas établie par ce lot.

*S297, 2026-09-19 — suivi A295, ouverte.* La référence 3D dispose du couplage B/W,
reçu contre HOS et un fond uniforme traversant ; un aperçu animé CPU est livré. Les frontières
construites restent partiellement reçues (absorption/progression S269–S274 à rejouer), les
fournisseurs réels et la production GPU manquent. A274 non fermée.
[Preuves et limites](../validation/DELTA3D-COUPLEE-S297.md).

**A297 — S301, 2026-09-19 (sévérité 2, ouverte).** **La hauteur de la référence δ 3D est
discontinue en la position de la surface par rapport aux centres de maille.** Le transport lit la
vitesse de la couche partiellement mouillée, pondérée par `(surface − k·dx)/dx` ; quand la surface
passe le centre de cette couche, sa face bascule de projetée (correction pondérée par 1/θ, θ petit)
à extrapolée depuis la couche du dessous. Mesuré : 0,25 m/s d'écart sur une face, ~0,5 mm de
hauteur par pas ; **deux cœurs partis à ±10⁻⁶ m l'un de l'autre divergent de 4,5 cm en 6 s** sur
le cas S298, premier millimètre à 1,3 s. Conséquences : (1) aucune implémentation non identique au
bit ne suit la référence au-delà de cet horizon — le critère 2 d'ADR-175 §4.2 se lit sur la durée
de prévisibilité mesurée ; (2) chaque trajectoire porte probablement des à-coups locaux d'origine
numérique, non mesurés en tant que tels. Déclencheur : avant la revue du critère 3, mesurer les
à-coups d'une trajectoire seule ; s'ils sont visibles, une vitesse de couche continue en θ est une
décision de schéma (nouvel ADR, réceptions 2D et 3D rejouées). [Preuve](../validation/DELTA3D-PAS-GPU-S301.md) §5.

*S301, 2026-09-19 — suivi A295, ouverte.* Le pas de production est complet sur la carte et reçu
étage par étage contre la référence ; la trajectoire la suit jusqu'à son propre horizon de
prévisibilité (A297). Diagnostics D3 relus en différé, dégradation déclarée. Manquent la scène et
la revue (critère 3), et la porte C sur cette scène. Pic d'amorçage observé à 8,4 ms au premier
pas d'un domaine de 131 072 mailles (famille A294), non attribué.
[Preuve](../validation/DELTA3D-PAS-GPU-S301.md).

**A298 — S305, 2026-09-20 (sévérité 1, ouverte).** **L'écart entre le pas de production et la
référence croît avec le temps, à pente mesurée.** Sur le cas de cuve — fond nul, murs, régime
linéaire, aucun déclenchement d'A297 — l'écart de hauteur passe de 2,5·10⁻⁷ à 7,8·10⁻⁷ m entre
t = 0,5 s et t = 5 s, soit **≈ 1,2·10⁻⁷ m par seconde**, à peu près linéairement. Dans le même
temps la moyenne de la surface dérive de 4 à 6·10⁻⁸ m sur la carte contre 1·10⁻¹⁰ m sur le cœur,
un facteur **400**. Ce n'est pas la divergence chaotique d'A297 : ce cas y échappe par
construction, les deux solveurs s'amortissent ensemble et aucune divergence de phase n'apparaît
sur 2,33 périodes. C'est un biais lent, probablement dans la somme compensée de la carte
(`exact_difference`, S301, L346) qui n'égale pas celle du cœur — **non démontré**. Conséquence :
un critère d'écart ponctuel se lit avec sa durée, y compris hors d'A297 ; les 3 mm d'ADR-175 §4.2
seraient franchis vers sept heures de temps simulé. Déclencheur : premier domaine δ appelé à vivre
plus de quelques minutes, ou premier lot de précision sur la production.
[Preuve](../validation/CUVE-GPU-S305.md) §6 et §7.

*S305, 2026-09-20 — suivi A295, ouverte.* Le critère 2 de la porte B est désormais mesuré sur
**deux** familles de cas et non une : le cas S298 (fond spectral, éponge) et les cas de cuve
(fond nul, murs). Le mode sans fond du pas de production est une donnée, pas une branche, et le
chaînon `step_perturbation_mobile` / `step_surface_mobile` est identique **au bit** à fond nul.
Restent non mesurés pour la production les cas 1 et 2 de §4.1 (HOS à `ny` = 1, invariance en `y`),
et le régime de forte cambrure. [Preuve](../validation/CUVE-GPU-S305.md).

**A299 — S306, 2026-09-20 (sévérité 1, ouverte).** **L'environnement lumineux de notre rendu n'a
jamais été comparé de façon contrôlée.** Le guide reçu place le ciel et l'exposition au **même
rang** que les normales fines dans les causes d'un rendu qui ne ressemble pas à une photo (§10.2
points 1–3, §10.3 rang A), et demande de fixer ciel, tone mapping et caméra **avant** toute
comparaison. Le dépôt possède un habillage « ciel clair » dont les couleurs ont été relevées sur
la photo de référence (S261, `p.eye.w`) et un ciel précalculé pour les reflets
([ADR-162](../adr/ADR-162-ciel-precalcule-des-reflets.md)), mais **aucune mesure** : ni
comparaison ciel par ciel à caméra et exposition fixées, ni vérification que le Fresnel employé
(0,02 à incidence normale, exposant 5) correspond à un diélectrique eau/air, ni contrôle de
l'exposition. Conséquence : une part inconnue de l'écart jugé en R11 peut venir de là, et aucun
travail sur la surface ne la corrigera. S306 a mesuré la part des **normales fines** (80–85 % de
l'énergie haute fréquence) ; la part de l'**environnement** reste non mesurée. Déclencheur : au
verdict R13, ou avant toute nouvelle revue de mer.
[Preuve partielle](../validation/STRIES-S306.md) §6.5.

*S306, 2026-09-20 — fermeture partielle du soupçon de repli du paramétrage.* L'instrument de S260
publie désormais `replis` : **zéro sur les treize modèles**, dont celui du rendu. Le jacobien du
déplacement horizontal ne s'annule ni ne s'inverse dans notre mer. L'indicateur que le guide met
au premier rang (§4.3) est mesuré, et il écarte cette hypothèse.

**A300 — S307, 2026-09-20 (sévérité 2, ouverte).** **Les constantes du nuanceur n'ont pas de
provenance, et personne ne les avait auditées.** La couleur du corps d'eau était 9 fois trop
verte et l'est restée à travers six revues visuelles, deux lots de correction de la mer et
plusieurs audits ([ADR-177](../adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md) la
corrige). Ce n'était pas un cas isolé : `water.wgsl` porte encore, **sans source**, les couleurs
d'horizon et de zénith du ciel (`CLEAR_HORIZON`, `CLEAR_ZENITH`), deux longueurs de brume
(**500 m** par défaut, 6 km en ciel clair, contre des dizaines de kilomètres en air marin
propre), le terme de Fresnel (`0,02 + 0,98·(1−cos)⁵`), la direction du soleil, et trois exposants
de miroitement (180, 512, 1024). **I-14 ne s'appliquait de fait qu'au cœur** : la règle « aucune
valeur physique sans provenance » n'avait jamais été portée sur le rendu, parce qu'on le tenait
pour cosmétique — or c'est lui que l'utilisateur juge. Déclencheur : au prochain lot de rendu,
auditer ces constantes comme on audite celles du cœur, et donner à chacune une source ou le
statut explicite de **choix déclaré**.
[Preuve](../validation/RENDU-ECART-S307.md) §4 et §5.

**A301 — S307, 2026-09-20 (sévérité 2, ouverte).** **Aucune mesure du dépôt ne regardait
l'image.** Jusqu'à S306, toutes les mesures du rendu portaient sur la **surface** (statistiques
de pente, `mss`, `Sk`, `c₀₃`) ou sur des **empreintes d'octets** — c'est-à-dire sur l'entrée et
sur l'identité, jamais sur ce qui est montré. Conséquence mesurée : trois revues envoyées avec
des options acceptées éteintes, une brume à 500 m et une couleur 9 fois fausse ont traversé six
revues sans être vues. Les deux instruments qui existent aujourd'hui sont récents et partiels —
`outils/spectre_image.py` (S306, énergie haute fréquence) et la mesure de couleur de S307 — et
**aucun n'est exécuté au rituel**. Déclencheur : faire entrer au moins une mesure d'image dans
les contrôles systématiques, au même titre que `etat_projet.py --check`.

**A302 — S308, 2026-09-20 (sévérité 2, ouverte).** **Aucun bilan de conservation n'a jamais été
mesuré à l'interface δ ↔ B/W.** Le couplage existe depuis S250 et tourne depuis S302 ; masse,
quantité de mouvement et énergie entrantes et sortantes n'ont **jamais** été comptées, et la
réflexion artificielle aux frontières n'a jamais été chiffrée. Ce qui existe à la place est un
**jugement visuel** : R11 a déclaré le raccord du domaine « invisible ». Même famille qu'A301 —
une propriété qu'on regarde au lieu de la mesurer — mais du côté physique cette fois, et c'est le
côté où le projet revendique sa rigueur. Aggravant : le couplage est **à sens unique** (l'éponge
absorbe vers B+W, rien n'écrit en retour dans W), ce qu'aucun document d'état ne disait avant
S308. Déclencheur : lot 1 d'[ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7,
sur la cuve de S305 et la scène de S302, qui existent toutes les deux.
[Confrontation](TROIS-SYSTEMES-S308.md) §2 et §3.

**A301 — note datée du 2026-09-20 (S308).** L'angle reste ouvert et son déclencheur inchangé.
Deux instruments d'image ont été construits depuis — `outils/cible_image.py` et
`outils/courbe_tonalite.py` — et **aucun des deux n'est encore exécuté au rituel**. S'y ajoute un
défaut d'instrument constaté **deux fois** : la détection automatique d'horizon se trompe quand le
ciel porte un fort gradient, en silence et avec un résultat plausible (`0,3652 / 15,11 / 0,4890`
au lieu de `0,1766 / 22,81 / 0,3082` sur le même rendu). Toute comparaison entre rendus passe
désormais par `--horizon=<y>` **forcé** ; publier la hauteur de chute, comme S308 P4 l'avait fait,
n'a pas suffi à l'empêcher.

**A303 — S309, 2026-09-20 (sévérité 1, fermée le jour même).** **Le battement du jeton n'était
relu par personne.** S308 a lu l'horloge une fois, à 13:12, puis a **extrapolé** cinq battements
successifs jusqu'à 15:14 — soit 1 h 30 d'avance sur l'heure réelle. L237 interdit exactement cela
depuis longtemps, et rien ne le vérifiait. La conséquence n'est pas cosmétique : AGENTS.md fait
d'un jeton `occupé` de moins de deux heures un **refus de reprise**, donc un horodatage avancé
**bloque la session suivante**, silencieusement. Remède **exécutable**, pas documentaire (L349) :
`outils/etat_projet.py --check` refuse désormais un battement dans le futur de plus de deux
minutes, et ses essais prennent pour contre-exemple l'erreur réelle de S308. Reste ouvert le
défaut de fond — rien n'oblige une session à lancer `--check` avant de committer son jeton.

**A302 — note datée du 2026-09-20 (S310). Partiellement fermée.** Le bilan de **masse** existe :
`Balance3`, tenu par le pas couplé et le pas non couplé, exact par télescopage et publié avec son
plancher ([BILAN-MASSE-S310](../validation/BILAN-MASSE-S310.md)). L'angle est **fermé pour la masse
sur la référence CPU**, et ce qu'il soupçonnait est confirmé et chiffré : l'éponge **efface** 10,2 %
du contenu perturbatif d'un domaine par seconde, et rien n'en revient dans W. **Reste ouvert**, à
la même sévérité 2 : le compteur n'existe pas sur la **carte** ; les bilans d'**énergie** et de
**quantité de mouvement** ne sont pas fermés — publiés comme états, avec les termes manquants
nommés (travail de la pression au bord, flux advectif) ; et la réflexion artificielle n'est
toujours chiffrée qu'en 2D (S269). Déclencheurs : lot 2 d'ADR-178 pour le retour vers W, et le
premier portage du compteur sur la production.

**A302 — note datée du 2026-09-20 (S311).** Ce que S310 appelait « un chemin manquant » est en
réalité **une paroi** : la vitesse normale aux faces extérieures d'un domaine δ vaut **0
exactement** (0,605 m/s à l'intérieur au même instant). Un domaine est une **boîte fermée**, et
l'éponge une région d'amortissement *intérieure*. Aucun document d'état ne le disait, et trois
sessions ont raisonné autour sans le nommer. La perturbation sortante se lit donc sur une
**surface de contrôle intérieure**, reçue contre les deux seuils de T3
([preuve](../validation/SORTIE-DELTA-S311.md)). **Reste ouvert** : le transfert lui-même, et le
fait que la composante de **volume net** n'a **aucun receveur** dans W — mesuré, l'impact de W
portant de l'énergie et pas de volume (−3,6·10⁻⁷ m³ pour 0,01 J).

**A302 — note datée du 2026-09-20 (S312). Sens δ → W : premier transfert construit.** Le chemin
existe : un événement de W émis au point de sortie, calibré sur la jauge, **T3 tenue en amplitude**
(1,24·10⁻⁵ pour 5 %) et **réflexion 2,84·10⁻⁷ en 3D** ([preuve](../validation/TRANSFERT-DELTA-W-S312.md)).
La composante de volume net est **registrée**, pas transférée, et ADR-180 D1 interdit d'appeler
cela une conservation. **Reste ouvert** à la même sévérité : le sens **W → δ**, le compteur sur la
**carte**, l'énergie et la quantité de mouvement, la cadence d'un transfert continu, et le
rebouclage du champ de W dans le domaine.

**A303 — S312, 2026-09-20 (sévérité 2, ouverte). Personne n'avait demandé à W ce qu'il sait
faire.** Cinquante sessions ont traité W comme la couche des perturbations propagatives, en
s'appuyant sur le contrat `WaveEvent` — qui porte une énergie, une longueur d'onde, une
**direction**, une **anisotropie** et un **volume déplacé**. Soumis aux champs **construits**, le
compte est tout autre :

| ce que le contrat annonce | ce que les champs construits en font |
|---|---|
| `displaced_l`, un volume | **inerte** — 0 et 1000 L donnent des `η` identiques **au bit** |
| `anisotropy`, une direction | **refusée** à toute valeur non nulle, `Error::Anisotropy` |
| `wavelength_m`, une longueur d'onde | le **centre d'une bande de deux octaves**, `k ∈ [k₀/2, 2k₀]` |
| une couche d'ondes de gravité | **eau profonde uniquement** : `h ≥ λ`, `h ≥ 2λ`, ou pas de profondeur du tout |

**Conséquences mesurées au premier couplage** : 100 % du volume net reste en attente, 50 % de
l'énergie transférée repart à contresens, 27 % d'écart de longueur d'onde et 12 % de vitesse. Rien
de tout cela n'était écrit nulle part, et rien ne l'aurait montré sans construire les champs et
les interroger (L355). **Déclencheur** : toute session qui prévoit d'envoyer quelque chose à W, ou
la décision de l'utilisateur sur la primitive orientée (ADR-180 D8, après le premier couplage —
donc maintenant possible).

**A304 — S312, 2026-09-20 (sévérité 1, ouverte). Un seuil dont le dénominateur peut s'annuler.**
T1 (ADR-179 D1) demande « résidu ≤ 10⁻⁶ de l'**échelle du pas** ». Sur un cas de **moyenne nulle**,
cette échelle vaut 1,15·10⁻⁷ m³ au maximum du banc et tombe au picolitre dès que le domaine se
calme, tandis que le résidu reste au plancher `f32` de ses sommes : le rapport vaut **1,95** sans
plancher d'activité et **encore 1,8·10⁻⁴ sur les 318 pas les plus actifs**. Normaliser par
`Balance3::volume` ne sauve rien — somme **signée**, nulle par construction pour un paquet
(2,78·10⁻⁴). Rapporté à une grandeur **absolue** : 8,3·10⁻¹⁰, trois ordres sous le seuil. Le
schéma n'est pas en cause. **Sévérité 1** parce que le défaut est dans l'énoncé du critère et non
dans le code, mais il produit des échecs qui ne disent rien et, ailleurs, des succès qui n'en
disent pas plus. **Déclencheur** : décision de l'utilisateur sur la normalisation de T1, que le
statut « provisoire, révisable » d'ADR-179 D1 prévoit. En attendant, aucun banc ne revendique T1
(L356).

**A304 — note datée du 2026-09-20 (S313). Remède mesuré, décision en attente.** Le défaut est
confirmé et sa cause précisée : ce n'est pas seulement que le dénominateur de T1 peut s'annuler,
c'est que **la loi du résidu n'était pas connue**. Elle l'est : `u₃₂ · activité / √N`, vérifiée à
un facteur 3 près sur quatre décades d'amplitude, un facteur 16 en `dt` et un facteur 16 en `N`
([preuve](../validation/PLANCHER-BILAN-S313.md) §2). L'échelle pertinente est le volume **absolu**
de perturbation, seul dénominateur dont le rapport soit stable. `Closure3` publie les quatre
grandeurs sans seuil, et trois tolérances sont **proposées** (C1, C2, C3). **Reste ouvert** jusqu'à
la décision de l'utilisateur : aucun banc n'applique de seuil, et ADR-179 D1 n'a toujours pas de
remplaçant acté.

**A305 — S313, 2026-09-20 (sévérité 2, ouverte). Le résidu du cas ouvert est d'un seul signe.**
La cuve fermée donne un résidu qui se compense d'un pas au suivant : forme du cumulé **0,06 à 2,93**
sur vingt-deux passages, une marche aléatoire. Le cas **ouvert** — bande B/W et éponge — donne
**13,67** pour `√200` = 14,14 : **tous les résidus ont le même signe**, donc le cumulé croît
**linéairement** avec le nombre de pas au lieu de croître en `√pas`.

En valeur c'est minuscule — 2,88·10⁻¹⁰ m³ sur 200 pas, soit 5·10⁻⁸ de la dérive physique du même
banc. Mais **un biais systématique n'est pas du bruit** : il ne se compense jamais, et sur une
scène qui vit des minutes il croît sans limite pendant que le bruit, lui, plafonne.

**Ce qui le rend un angle mort** : six sessions ont publié des bilans de masse sans jamais regarder
le **signe** de leurs résidus. S310 mesurait le pire pas, S312 le pire pas et le cumulé absolu ;
aucun ne distinguait une marche aléatoire d'une dérive. Il a fallu un indicateur écrit pour une
autre raison — éprouver une fuite volontaire — pour que le biais apparaisse.

**Suspects nommés, aucun démontré** : la **bande** B/W et l'**éponge**, les deux seuls termes qui
n'existent pas dans la cuve fermée. **Déclencheur** : l'ordre C du lot 2 (vérification conjointe),
ou tout banc de conservation sur une scène à fond réel — et il se teste en éteignant les deux
termes l'un après l'autre, comme L354 le prescrit.

**A306 — S314, 2026-09-20 (sévérité 2, ouverte). Aucun estimateur de fréquence du dépôt n'a été
vérifié.** Le comptage de passages par zéro sert de mesure de période dans quatre endroits :
`examples/transfert_paquet.rs` (S312), `examples/frame_cost.rs`, `water-harness/src/physics.rs` et
`physics_shallow.rs`. **Aucun n'a jamais été comparé à un second estimateur.**

S314 a montré qu'il **biaise dès que le signal porte des composantes courtes**, et que le biais
**croît avec la qualité du solveur** : 0,25 % à 25 cm, 0,38 % à 12,5 cm, **4,7 % à 6,25 cm**
([preuve](../validation/TRANSFERT-ORIENTE-S314.md) §7.1). Le maximum du périodogramme donne
−0,036 % au même point.

**Ce qui est déjà tranché** : à 12,5 cm l'écart vaut 0,38 %, donc les conclusions de S312 tiennent,
et son `λ_mesure` = 2,0183 m se lit **2,034 m** au périodogramme — sans effet sur ses verdicts.

**Ce qui reste ouvert** : les trois autres emplois n'ont pas été examinés. `physics.rs` et
`physics_shallow.rs` appartiennent au **harnais de validation** — un estimateur biaisé y fausserait
des réceptions, pas seulement un banc. **Déclencheur** : toute réception qui dépend d'une période
mesurée, et tout raffinement de maille sur un cas qui en emploie une. Le remède est connu et coûte
quelques lignes — deux estimateurs, publiés côte à côte, et leur écart lu comme une mesure (L360).

**A307 — S315, 2026-09-20 (sévérité 2, ouverte). Aucun banc du dépôt ne déroule une phase.**
Partout où une phase est comparée — S312, S314, S315 —, elle l'est **enroulée**, lue en un point
et modulo un tour. Tant que les deux signaux comparés arrivent ensemble, c'est licite. **Dès que
leurs vitesses de groupe diffèrent, ce ne l'est plus** : l'écart d'arrivée croît avec la distance,
dépasse une période, et plusieurs enroulements deviennent compatibles avec la mesure.

Mesuré en S315 ([preuve](../validation/ORACLE-ET-OBLIQUE-S315.md) §5) : à dix longueurs d'onde, le
train et δ arrivent à **1,66 s** l'un de l'autre, soit **1,46 tour** de porteuse ; le désaccord de
phase mesuré, 0,356 tour, est compatible avec au moins deux enroulements et **le banc ne tranche
pas**.

**Conséquence sur le passé** : la régression de phase de 0,9938 publiée en S314 est mesurée **sur
la ligne d'émission**, où les deux signaux sont au même endroit au même instant — elle est donc
valide, et elle ne dit rien de la propagation. Aucune réception du dépôt ne dépend d'une phase
comparée à distance, parce qu'aucune n'a été tentée avant S315.

**Déclencheur** : l'ordre C, qui doit évaluer la phase parmi les six propriétés. Le remède est
nommé et n'est pas construit : **suivre la porteuse en continu** le long du trajet — déroulement
de phase entre les deux lignes — plutôt que la lire aux extrémités. Interdit associé (ADR-183 D3,
L363) : décaler un signal pour superposer les deux **ne mesure rien**.

**A306 — close en S316, 2026-09-21.** Tous les emplois réels d'un estimateur par passages par zéro
sont examinés : `transfert_paquet.rs` (S314), `physics.rs`, `physics_shallow.rs` et
`physics_dispersif.rs` (S316, essais `a306_*`, écarts 0,03 à 0,24 % pour 1 % de marge). **Aucune
réception affectée.** `frame_cost.rs`, cité plus haut, n'en contient pas : la liste était fausse sur
ce point. Et sur ces signaux de mode, c'est le second estimateur qui se trompe — fuite de fenêtre,
−0,236 % à huit périodes ([preuve](../validation/ORDRE-C-S316.md) §11).

**A307 — close en S316, 2026-09-21.** L'instrument existe : projection à fréquence fixe, fenêtre
commune, déroulement ancré en `ω` puis en `x`, éprouvé sur un écart connu de −0,853 tour. La phase à
dix longueurs d'onde vaut −0,914 ; −0,237 ; −0,045 tour aux trois mailles
([preuve](../validation/ORDRE-C-S316.md) §2 et §5).

**A308 — S316, 2026-09-21 (sévérité 2, close dans le banc de l'ordre C). Les bancs du transfert
tiraient de l'onde posée ce qu'une scène ne leur donnerait pas.** Trois fuites : la largeur
temporelle devenait spatiale par la vitesse de groupe **posée** ; les fenêtres d'identification et
de retour étaient calées sur l'arrivée **théorique** ; et le train naissait sur la face de la ligne
quand la jauge lisait le centre de la colonne. À 25 cm, où δ va 25 % moins vite, la fenêtre de
retour lisait la traîne du passage direct comme une réflexion. Corrigé dans `ordre_c`, dont la
durée se décide sur la vitesse **mesurée** ; `essai` et `distance` restent tels quels, bancs de
leurs preuves, qui portent désormais une note datée.

**A309 — S316, 2026-09-21 (sévérité 2, ouverte). La direction que lit le raccord est biaisée, et le
biais ne converge pas.** Sur un paquet court (σ = 1 λ), le maximum du périodogramme en `k_y` lit
**+5,3 puis +5,2 %** à 20°, **+4,0 puis +4,3 %** à 40°, à 25 puis 12,5 cm : l'écart d'angle ne se
réduit que de sa part `ω`. Le train porte fidèlement une direction lue **+1 à +2°** trop ouverte
([preuve](../validation/ORDRE-C-S316.md) §10). **Déclencheur** : l'ordre E, ou tout train oblique
consommé loin de sa ligne. **Remède à éprouver comme l'instrument de phase** : un estimateur de
direction mesuré d'abord sur un paquet synthétique d'angle connu.

**A310 — S318, 2026-09-21 (sévérité 1, ouverte). Le SPH du banc du lot 5 garde une erreur que je n'ai
pas isolée.** Période du ballottement à −6,2 % et 27 à 29 % d'amortissement par période, **identiques
à deux mailles** ; ni la diffusion δ, ni une viscosité divisée par quatre, ni des parois glissantes ne
les font bouger ([preuve](../validation/COMPARAISON-LOT5-S318.md) §3). Suspects restants, non
éprouvés : les parois en particules dynamiques, l'intégrateur. **Conséquence** : la comparaison ne
retient contre SPH que la conservation et le coût du pas acoustique, qui n'en dépendent pas.
**Déclencheur** : si l'utilisateur penche pour SPH, ou pour comparer un SPH incompressible — alors
parois par particules fantômes et intégrateur symplectique, d'abord.

**A289 — note datée du 2026-09-21 (S319) : déclencheur atteint, sévérité relevée à 3.** Sous une houle
B d'une composante (λ = 4 m), δ sans perturbation **croît** — 0,28 cm puis 7,7 cm en 115 s sous 2,5 cm,
16 cm sous 5 cm —, un peu plus lentement à maille fine ; plus vite et plus loin que la formule linéaire
de S274 ([preuve](../validation/MER-S319.md) §4–5). **Il bloque l'ordre E** : la restitution de S317
reçoit des centaines de fois le volume d'un paquet. Voies à choisir : rappel lent vers zéro, durée de
vie bornée (I-12), dispersion d'amplitude dans B.

**A311 — S320, 2026-09-22 (sévérité 2, ouverte). L'air n'est pas modélisé : une bulle enfermée est à
pression nulle.** Dans APIC comme dans les deux autres candidats de S318, une poche d'air enfermée
garde la pression atmosphérique quelle que soit sa profondeur, et l'eau s'y engouffre sous la pression
hydrostatique. Une bulle réelle se comprimerait, rebondirait et remonterait. B10 reçoit donc la
**fermeture** de la cavité, pas **la vie de la bulle** après ([preuve](../validation/B10-APIC-S320.md)
§6). **Conséquence** : le jet qui suit le pincement, et tout ce qui dépend de l'air enfermé, sont hors
de portée. **Déclencheur** : C20 en production, ou une revue visuelle d'un impact. Remède à éprouver :
une pression de bulle par volume enfermé (loi adiabatique), avant une phase d'air complète.

**A311 — note datée du 2026-09-23 (S326). La bulle sans pression bloque le calcul fin.** À `D/dx` =
32, à la fermeture de la poche, la vitesse maximale passe de 9 à 290 m/s en 0,01 s simulée et le pas
tombe à 3·10⁻⁵ s : le calcul complet de B10 a tourné 13 h sans finir, pour 1 à 2 h attendues
([preuve](../validation/B10-APIC-S320.md) §5 bis). Cause probable, non prouvée — une instabilité à la
fermeture n'est pas exclue. **Conséquence nouvelle** : la vie de la bulle n'est pas seulement fausse,
elle **arrête le calcul** à maille fine ; le remède d'A311 précède toute maille fine de production.

**A312 — S320, 2026-09-22 (sévérité 2, ouverte). La couronne et le jet d'un impact sont des grandeurs
de la maille.** De `D/dx` = 8 à 16, ils changent de 40 à 60 %, dans un sens ou dans l'autre, alors que
le temps de pincement et la cavité maximale convergent à 10 % près
([preuve](../validation/B10-APIC-S320.md) §5). Sans tension de surface ni viscosité, rien n'arrête
l'amincissement d'une nappe. **Conséquence** : un δ de production, à maille de jeu, aura une gerbe
fixée par sa maille. **Déclencheur** : le choix de la maille de production de la seconde
représentation, ou le premier verdict de l'utilisateur sur une gerbe. Remède à chercher : un modèle
sous-maille de nappe (rupture en gouttes, embruns), jugé sur l'image et non sur la convergence.

**A312 — note datée du 2026-09-23 (S326).** Le contraste ci-dessus ne tient qu'à moitié : à trois
mailles, le temps de pincement ne converge pas non plus — 2,20 → 2,30 → 2,40 `√(D/g)` — et la cavité
maximale lentement, d'ordre ≈ 0,5 ([preuve](../validation/B10-APIC-S320.md) §5 bis).

**A313 — S320, 2026-09-22 (sévérité 1, ouverte). Le volume géométrique d'APIC n'a pas de mesure
propre en écoulement agité.** La masse est exacte, mais le volume dépend de la règle de séparation à
±10 % (0,4 contre 0,45 maille, P3). L'occupation est biaisée (−8 % sans cavité). L'écart entre niveau
géométrique et niveau de masse **borne** à 0,7 maille loin du corps, sans tenir la similitude
([preuve](../validation/B10-APIC-S320.md) §2 et §6). **Conséquence** : le jour où l'eau passera des
particules aux colonnes, c'est le **volume** qui doit se conserver au raccord, et aucun instrument ne
le mesure encore. **Déclencheur** : le raccord particules ↔ colonnes (ADR-186 §3) — avant lui, un
compteur de volume géométrique global (aire sous la surface reconstruite, moins le corps et l'air
enfermé), éprouvé sur le ballottement et le corps lent. **Hypothèse sur la cause du tassement**
([lecture de simufluid](LECTURE-SIMUFLUID-S320.md) §3) : la loi de conservation géométrique est
violée — des cellules entières basculent de fluide à solide sans que la pression voie le volume
balayé. Épreuve : divergence cible des cellules coupées prise des volumes balayés, corps lent **sans**
séparation.

**A289 — note datée du 2026-09-22 (S320).** simufluid, même architecture fond + résidu, a mesuré une
dérive du résidu sous houle **portée par le nombre de pas par période** : son taux change de signe
entre 256 et 4096 pas, à physique fixée, et se reproduit avec un fond nul
([lecture](LECTURE-SIMUFLUID-S320.md) §1). S319 n'a fait varier que la maille.
**Avant l'arbitrage** : le témoin E1 à maille fixe et à trois pas de temps. Si le taux en dépend, la
voie « dispersion d'amplitude dans B » ne soigne pas la cause.

**A289 — note datée du 2026-09-22 (S322).** L'essai demandé ci-dessus est fait : à maille fixe (25 cm),
le taux de croissance vaut 0,1015, 0,1012, 0,1009 et 0,1007 s⁻¹ à 20, 10, 5 et 2,5 ms — 0,8 % sur un
facteur huit ([preuve](../validation/MER-S319.md) §8). **Ce n'est pas un défaut d'intégration du pas
couplé.** Le mécanisme reste à nommer ; les trois voies restent ouvertes, au choix de l'utilisateur.

**A313 — note datée du 2026-09-22 (S323) : résolu pour la mesure.** Le compteur existe — aire sous la
surface reconstruite par carrés marchants, exacte sur un plan, d'ordre deux sur un disque — et il
mesure le tassement : −1,24 % au corps lent sans séparation, −12,2 % en B10, ±0,3 % avec séparation
([preuve](../validation/B10-APIC-S320.md) §10). Au repos, le volume géométrique est sous la masse de
0,146 maille par longueur de surface — un biais de reconstruction, vers lequel un écoulement se relaxe.
L'hypothèse de la loi de conservation géométrique n'est pas éprouvée : la séparation contient le
tassement, elle ne dit pas sa cause. Ce qui reste ouvert passe à A314.

**A314 — S323, 2026-09-22 (sévérité 2, ouverte). La surface d'APIC dépend de l'arrangement de ses
particules.** À masse égale, deux arrangements n'ont pas le même volume géométrique : le biais de
reconstruction change. Une conversion particules → colonnes → particules **à masse exacte** fait donc
sauter la surface de la différence des biais — **+0,07 à +0,17 maille** en moyenne, jusqu'à 0,35 par
colonne ([preuve](../validation/B10-APIC-S320.md) §10) ; à 25 cm de maille, 2 à 4 cm. **Conséquence** :
chaque passage d'une région d'une représentation à l'autre se verra. **Déclencheur** : le raccord
dynamique, ou toute conversion dans une scène rendue. Remède à chercher : une reconstruction dont le
biais ne dépende pas de l'arrangement — rayon calé sur l'espacement local, ou surface portée par un
ensemble de niveaux conservatif advecté avec les particules.

**A315 — S324, 2026-09-23 (sévérité 2, ouverte). Les petites cellules d'un fond coupé 3D font ramper le
gradient conjugué du mode linéaire.** Sur une bosse vraiment 3D à 128 mailles de long, un pas coûte
**16 029 itérations et 708 s**, contre 347 et 4 s pour un fond invariant en `y` de même taille ; la
divergence finale vaut 1,000·10⁻⁵, la tolérance d'ADR-144 à l'arrondi près
([preuve](../validation/FACES-COUPEES-3D-S324.md) §4). Fractions jusqu'à 6·10⁻⁹, ouvertures jusqu'à
1,5·10⁻⁶ — mais le témoin a aussi des coins à 7·10⁻⁶ et converge : la forme compte, pas la seule
petitesse. **Conséquence** : aucune trajectoire à maille fine, donc pas de frontière mobile. **Déclencheur** :
la prochaine session du lot 3. Remède à éprouver d'abord : **Jacobi** sur le chemin coupé, comme le
mode mobile 3D, l'identité 2D gardée à `ny` = 1 ; critère : itérations à 128 comparables au témoin.
Ensuite seulement, fusion des petites cellules ou multigrille.

**A316 — S325, 2026-09-23 (sévérité 2, ouverte). La frontière du raccord dynamique décale la surface et
dissipe.** Colonnes et particules côte à côte, échanges comptés : la masse tient à l'arrondi, mais la
surface saute de **1,8 à 2,8 mailles** à la frontière (APIC seul : 0,15), le ballottement perd **jusqu'à
16 % par période**, et des vitesses parasites de 0,5 m/s apparaissent ([preuve](../validation/B10-APIC-S320.md)
§11). L'amortissement ne suit pas la taille de la zone des colonnes : le lissage grille → réseau → grille
n'est pas la seule cause. **Conséquence** : le raccord ne se consomme pas encore. **Déclencheur** : la
prochaine session du lot 5. Suspects, un par un : insertion des particules sortantes, quantification
de l'ensemencement, colonnes sans vitesse propre — δ porte les siennes sur sa grille.

**A316 — note datée du 2026-09-23 (S327). Attribué en partie** ([preuve](../validation/B10-APIC-S320.md)
§12). L'échange asymétrique — sortie par le flux, entrée par les particules qui franchissent — fait le
saut ; la surface des colonnes arrondie au quart de maille fait la dissipation. Corrigés, commutables :
écart 0,24 maille à 5 cm, 0,59 à 2,5 cm ; amortissement 4,2 % et 0,71 %. **Reste ouvert** : une
dissipation propre aux colonnes à 5 cm — ni transport en amont, ni réespacement, ni aller-retour de
vitesse — et un bruit de frontière, sans biais, à 2,5 cm.

**A316 — note datée du 2026-09-25 (S354). Changé de nature** ([preuve](../validation/B10-APIC-S320.md) §13). La
« dissipation propre aux colonnes » de S327 était lue sur une jauge aveugle — particules réensemencées, quantifiées
par demi-maille — et sur 10 s, où la mesure d'amortissement se trompe d'un point : sur 30 s, les colonnes seules ne
dissipent pas plus qu'APIC seul. Ce que 30 s montrent : **la densité des particules n'est pas tenue à la frontière**
— tassée à 5 par cellule (paroi, solde), dilatée à 3,5 (eulérien) —, la masse migre vers les particules, +12 mm en
30 s à 5 cm, et la période s'allonge de 2,65 points. **Reste ouvert.** Déclencheur : la prochaine session du lot 5.
Remède à éprouver : la dernière colonne de cellules réensemencée depuis sa hauteur géométrique.

**A316 — note datée du 2026-09-26 (S394). La racine désignée : l'échange comprime** ([preuve](../validation/B10-APIC-S320.md)
§14). Le remède de S354 — la dernière colonne réensemencée depuis sa hauteur géométrique — vide les particules (il cède à
chaque pas le biais de reconstruction et l'arrondi des rangées) ; gardant ses particules, il les piège. Une correction de
densité **en position** (champ dont la divergence vaut `n/4 − 1`, deux colonnes) **tient la densité** (3,9 à 4,0) et, à 5 cm,
la masse (±0,0013 m²) — mais l'onde croît de 4 % par période : elle rend 102 J/m en 30 s, ce que l'échange ôte. **Reste
ouvert**, précisé : l'échange lui-même. Déclencheur : la prochaine session du raccord (C5a, deuxième part).

**A316 — note datée du 2026-09-26 (S395). Localisé : une circulation permanente à la frontière** ([preuve](../validation/B10-APIC-S320.md)
§15). La vitesse horizontale moyenne sur la face du raccord vaut +21 mm/s en bas et −54 mm/s en haut sur 30 s (APIC seul, même
face : moins de 2 mm/s) — l'eau entre dans les colonnes par le bas et en ressort par le haut ; même chose sans paroi, avec la
mémoire de vitesse, frontière déplacée : elle naît des colonnes. L'échange au sommet l'aggrave (le fond s'entasse, 7,8
particules par maille). **Reste ouvert.** Hypothèse à trancher : les colonnes, réensemencées à chaque pas, n'advectent pas la
quantité de mouvement.

**A315 — note datée du 2026-09-23 (S326) : résolu.** Un Jacobi sur le chemin coupé — celui du mode mobile
3D — ramène la bosse à 128 de 16 029 à **425 itérations** (5,7 s au lieu de 708), débits inchangés à
2,4·10⁻⁶ près ; à `ny` = 1, identité 2D gardée ([preuve](../validation/FACES-COUPEES-3D-S324.md) §6).

**A317 — S333, 2026-09-24 (sévérité 2, ouverte). Une coque qui perce le couvercle de δ rayonne selon la
position de sa paroi dans la maille.** Même scène de la porte D, grille décalée de 5,5 cm : parois à 8 % et
52 % d'eau dans leurs mailles de bord au lieu de 30 et 30 % — le rayonnement, symétrique à 2 s, devient
franchement dissymétrique à 8 s, fort du côté de la lamelle de 8 %, et la perturbation passe de 9,4 à
13,7 cm ([preuve](../validation/PORTE-D-S333.md) §4). La trajectoire de jeu est la même au bit et ne roule
pas : c'est δ. **Conséquence** : une coque qui se déplace dans la grille rayonnera selon sa position sous la
maille ; la scène est posée à 30/30. **Déclencheur** : une coque mobile dans δ, ou la production. Suspects :
la colonne en partie couverte, dont la surface s'élève sur toute sa section quand la pression ne voit que sa
part libre ; la vitesse de paroi prise au centre des faces (S332). Critère : même rayonnement, à quelques
pour cent, pour tout décalage sous la maille.

**A317 — note datée du 2026-09-24 (S334) : attribué — défaut de structure — ; remède construit, éteint.**
Reproduit hors du jeu, en pilonnement imposé : écart de 35 % en 3D à 25 cm. Sur une tranche, le couvercle de
S332 **ne converge pas** — 38,5 ; 44,2 ; 30,5 % à 25 ; 12,5 ; 6,25 cm — : la colonne en partie couverte porte
sa hauteur de remplissage comme si toute sa section était libre, surface `1/a` fois trop molle. Le
**couvercle partiel** (`set_partial_lid`) converge — 33,4 ; 10,9 ; 1,8 %, moyenne extrapolée 19,98 mm, 11 %
au-dessus de celle de S332 — et ramène la scène de la porte D de 3,42 à 1,22 entre flancs
([preuve](../validation/PORTE-D-S333.md) §6). **Reste ouvert** : il amplifie le résidu de rotation de S332
dans les colonnes en lamelle — 5,5 m/s dans la contre-épreuve de S333 —, d'où l'état éteint par défaut ;
**déclencheur** : la vitesse de paroi au centroïde de la part couverte, puis l'allumer. Et même corrigée, une
coque de 6,4 mailles rayonne à ± 33 % selon son placement : ~25 mailles pour ± 2 %.

**A317 — note datée du 2026-09-24 (S335) : corrigé par défaut ; reste la résolution.** Les pointes du couvercle
partiel ne venaient pas de la paroi lue au centre des faces — la lecture au centroïde, construite, ne les retire
pas et fait manquer à S332 son critère 1 : écartée — mais des colonnes ouvertes à moins de 10 %. Avec un
plancher d'ouverture de 10 %, la coque tenue sur la houle reste sous 0,56 m/s, la tranche converge encore (3,1 %
à 6,25 cm), et le couvercle partiel est **actif par défaut** ([preuve](../validation/PORTE-D-S333.md) §7).
**Reste ouvert** : la dépendance au placement à maille grossière — ± 43–49 % à 6,4 mailles de largeur de coque,
~25 mailles pour ± 3 % ; **déclencheur** : une coque dans la production, ou une maille locale autour d'elle.

**A318 — S345, 2026-09-24 (sévérité 2, ouverte). À 30 Hz, l'onde de δ sur une vraie mer garde jusqu'à 6 % de plus
d'amplitude qu'à 60 Hz.** La cadence d'ADR-012 §7 est la voie de la porte C : un pas de 3,7 ms étalé sur deux
images. Sur la cuve de S305, le pas de temps ne pèse rien (0,04 % de période, 0,013 % d'amplitude à 33,3 ms) ; sur
la scène de B, l'onde isolée diffère de +3,9 à +6,3 % entre 30 et 60 Hz, cinq fois ce que change un doublement des
cycles — ni la projection ni l'éponge. Candidat : la dissipation de l'advection, par pas. Bloque l'adoption de la
cadence, pas la porte C elle-même. Déclencheur : un paquet sur une mer au repos, aux deux cadences ; ou une revue.
[Preuve](../validation/COUT-DELTA3D-S341.md) §8.

**A318 — note datée du 2026-09-24 (S346) : attribué en partie.** Tant que l'onde est groupée, l'écart suit le pas —
40 Hz au niveau du témoin des cycles (≤ 0,73 %), 30 Hz jusqu'à 5,7 %, toujours plus d'amplitude — : un amortissement
numérique par pas, porté par l'onde elle-même (mer au repos : 1 à 3,9 %) et accru par la mer. Une fois l'onde
dispersée, les écarts n'ont plus d'ordre (horizon d'A297). Candidat : l'advection non linéaire du pas couplé, non
localisée. Suite : revue R17 aux deux cadences. [Preuve](../validation/COUT-DELTA3D-S341.md) §9.

**A318 — note datée du 2026-09-24 (S348) : accepté à l'œil.** Verdict R17 : *« Continue je valide »* — l'écart
d'amplitude entre 30 et 60 Hz ne se voit pas, ou ne gêne pas. La cadence de 30 Hz est adoptée ; l'amortissement par
pas reste une propriété mesurée du schéma, publiée, non corrigée. Reste ouvert comme connaissance, plus comme
obstacle.

**A319 — S351, 2026-09-24 (sévérité 2, ouverte). « Rétrécir ou détruire un domaine perturbatif est visuellement
gratuit » n'a jamais été mesuré.** C'est la prémisse d'I-12, d'ADR-005 §5 et des rangs 1 et 5 d'ADR-012 §4 : δ = 0 au
bord, donc rien à perdre. Le rang 1 l'a mesurée pour la première fois : sur la mer de la porte B (`Hs` 2,5 m), la
correction couplée que δ porte monte à 18,6 cm, et une descente de 120 × 112 à 69 × 65 mailles en **retire jusqu'à
11,3 cm** dans une bande que le rendu pondérait entièrement ; les descentes d'une maille coupent 0,3 à 10 cm, dans le
fondu. Rien ne dit si cela se voit. Déclencheur : avant qu'un point de la liste ne s'appuie sur cette gratuité (4.5,
9.9), une revue visuelle de la descente ; si elle se voit, un rétrécissement qui **amortit** avant de couper, comme
ADR-012 §4 le décrit pour le rang 1 sous pression. [Preuve](../validation/ARBITRAGE-3D-S344.md) §7.

**A289 — note datée du 2026-09-26 (S369) : résolue quant à sa cause, par ADR-198.** La croissance sous une houle B seule
est **forcée** par trois termes du pas couplé qui ne dépendent que de B — son résidu de quantité de mouvement (l'essentiel),
son transport jusqu'à sa propre surface, son erreur de pression à cette surface. Retirés (δ relatif à la dynamique de B),
δ nul reste nul au bit sous la houle, 40 s ([preuve](../validation/MER-S369.md) §1). Voie choisie par le projet sous la
délégation de l'utilisateur ([ADR-198](../adr/ADR-198-la-voie-d-a289.md)). Le blocage de l'ordre E passe à A320.

**A320 — S369, 2026-09-26 (sévérité 3, ouverte). Sous une houle raide, une perturbation de δ croît.** En mode relatif,
un germe de 1 mm comme un paquet de 2 cm croissent après 35 à 50 s : 0,052 à 0,060 s⁻¹ sous `ak` = 0,079, 0,115 sous
0,118, rien de visible sous 0,039 en 95 s — ≈ `a²`, ≈ 4,5 fois Benjamin-Feir. **Convective** (×e tous les ≈ 18 m, le long
de la houle), indépendante du pas de temps, **plus lente à maille fine** (0,033 à 12,5 cm). **Portée par le terme de
cisaillement `u'·∇U`** : lui seul retiré, plus rien ([preuve](../validation/MER-S369.md) §3). **Bloque l'ordre E**
(critère 3 manqué : 1 620 fois le paquet). Hypothèse : discrétisés séparément, `U·∇u'` et `u'·∇U` ne forment plus le
gradient `∇(U·u')` qu'ils sont pour deux écoulements irrotationnels. Remède à éprouver d'abord : cette forme de
Bernoulli. Déclencheur : la prochaine session du lot 2.

**A321 — S390, 2026-09-26 (sévérité 3, **corrigée S391**). À 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit le
solveur de pression.** Pas de 33,333 ms, `Config::review`, 1 800 pas : la surface publiée cesse d'être finie au pas 930 avec
Jacobi 512 et au pas 1 050 avec la multigrille à 24 cycles — deux projections **convergées** (résidu relatif ≈ 10⁻⁷) —, au
pas 1 200 avec Jacobi 32, la production reçue par la porte C (S348, R17). **À 60 Hz, la minute tient** (références,
Jacobi 16 à 128, multigrille 6 et 8) ([preuve](../validation/MULTIGRILLE-3D-S385.md) §5). La porte C a été reçue sur 1 000
pas horodatés **depuis l'état initial**, les revues sur quelques secondes : aucune mesure ne portait la durée d'usage
(L369). **Non attribué** ; la pression est hors de cause. Candidats : le transport et la hauteur explicites au double pas,
le couplage et l'éponge de B, la bascule de mouillure (A297). Déclencheur : **avant tout usage vivant de 30 Hz au-delà de
vingt secondes**, et au plus tard en C3b — témoin à 25 ms, termes éteints un à un (L136).

*Corrigée le 2026-09-26, S391* ([ADR-209](../adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md),
[preuve](../validation/A321-S391.md)). **Cause** : la prédiction avançait les vitesses de δ par Euler explicite et différences
centrées (FTCS), instable pour tout pas — explosion en ~`1/dt` (72 s à 60 Hz aussi), mode à l'échelle de la maille. **Remède** :
le terme de second ordre `+(dt²/2)·V_a·V_b·∂_a∂_b u` (Lax-Wendroff), actif par défaut dans la production, en option dans la
référence ; la scène tient deux minutes à 30, 25 et 16,7 ms. Reste la migration du défaut du cœur (ADR-209 D3).

