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

- **A217** *(sévérité 2, S161 ; ouverte)* — **En eau profonde, on ne sait pas quelle variable
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