# Journal des sessions

Une entrée par session. Sert à reprendre le travail dans une conversation neuve sans relire
l'intégralité des documents.

---

## S01 — 2026-09-05 — Fondations architecturales

**Entrées.** `systeme_eau_architecture_globale.md`, `systeme_eau_zones_ouvertes_et_decisions_a_valider.md`
(copies de référence dans `docs/sources/`).

**Sorties.** Mise en place de la base de connaissances ; ADR-001 à ADR-013 ; SPEC-001 ;
registres des angles morts et de traçabilité ; plan de benchmark ; `METHODE.md` ; `LECONS.md`.
Page de suivi publiée pour l'équipe : https://claude.ai/code/artifact/a3343ad0-63f5-47e2-9bcb-ab40fc3de7da

**Décision structurante.** Décomposition de l'eau en quatre couches B / W / δ / V (ADR-001). La
couche W — perturbations propagatives à dispersion correcte, déterministes, dérivées d'événements
— est l'élément absent des documents sources ; son introduction dissout six questions ouvertes et
rend possible la séparation d'autorité gameplay.

**Chiffres qui ont orienté la conception.**
`λ = 2πv²/g` (64 m à 10 m/s) · `c_g = c/2` · coût `∝ dx⁻⁴` · ulp `f32` = 12 mm à 100 km ·
ulp temporel `f32` = 62 ms à 10⁶ s · perte de 29 % de Hs en mélangeant deux champs ·
flèche de 50 cm pour un bassin de 20 m dans un anneau de 100 m · `H/h = 0,78` ·
`arcsin(1/Fr_h)` en eau peu profonde · pente de rivière 1,6 ‰.

**Statut du document source après S01.** 30 sections traitées : 17 résolues, 6 dissoutes,
5 partielles, 2 ouvertes par décision. Détail dans `docs/registres/QUESTIONS-OUVERTES.md`.

**Ce que je n'ai pas fait, et qui attend.**
- ADR mousse / spray / bulles (§24) et simulation de l'air (§25).
- ADR audio (angle mort A12) — dépend d'un échange avec l'équipe audio.
- ADR glace et phases (A19), navigation IA (A20), vue sous-marine (A27).
- Format de `CondensedState` (ADR-007 §5).
- Réseaux V fermés sous pression.

**Prochaine session recommandée.** S02 : mousse/spray/bulles + air, puis les trois angles morts
non traités. Ce sont les derniers blocs conceptuels avant que le projet dépende entièrement des
bancs d'essai.

**Décisions qui demandent un arbitrage humain** (je les signale, je ne les prends pas) :
1. Le temps du monde peut-il être mis à l'échelle par joueur (voyage rapide) ? — ADR-003 §4.1.
   Si oui, la cohérence multijoueur de la houle est perdue et il faut basculer l'océan concerné
   en couche locale.
2. `liquid_id` porte-t-il une phase (glace/vapeur) ? — angle mort A19.
3. Le trait de côte mobile (marée) engage les équipes terrain, IA et audio — ADR-011 §6.

**Erreur de méthode commise.** Voir `LECONS.md` L09.

---

## S02 — 2026-09-05 — Phénomènes secondaires et interfaces inter-équipes

**Consigne reçue.** Livrables en Markdown uniquement ; pas de page HTML publiée. Le reste de la
production de S01 convient tel quel.

**Sorties.** ADR-014 à ADR-019 ; SPEC-002 ; registre des angles morts porté de 28 à 40 entrées et
entièrement soldé ; bancs B9 à B11 ; index et traçabilité mis à jour.

**Décisions structurantes.**
1. *Mousse* — représentation primaire par un **champ** advecté à deux canaux de décroissance, pas
   par des particules. Les particules sont une garniture proche-caméra. Même logique que
   ADR-001 : ce qui est déterministe est re-dérivé, ce qui est transitoire est mis en cascade.
2. *Air* — quatre niveaux dont le quatrième (solveur multiphasique) est refusé. Le niveau
   « poche discrète » couvre tous les cas qui ont une conséquence, pour le prix d'une EDO.
3. *Autorité* — première exception contrôlée à l'invariant I-04 : le champ d'aération `A` modifie
   la densité effective donc la flottabilité. La part issue de W et du vent est autoritaire ; la
   part issue de δ est plafonnée.

**Chiffres qui ont orienté la conception.**
Couverture de moutons `3,84·10⁻⁶·U10^3,41` (4 % à force 7, pas 40 %) · `We > 12` → toute nappe
au-delà de 1 m/s se fragmente · remontée d'une bulle de 1 mm : 0,12–0,25 m/s · Boyle : une poche
d'air perd la moitié de son volume à 10 m · vide : 14 % s'évapore, 86 % gèle · Stefan
`h ≈ 0,035·√FDD` · portance de la glace en `h²` · `HR = d·(v+0,5)` : 0,5 m à 2 m/s emporte un
adulte · transmission acoustique à l'interface `−29,5 dB` · fenêtre de Snell 97,2° · rouge éteint
à 13 m.

**Statut du document source après S02.** Les 30 sections sont traitées : 19 résolues, 6 dissoutes,
4 partielles, 1 ouverte par décision (le choix du solveur, §18, qui relève du banc B3).

**Bascule d'état du projet.** Il ne dépend plus d'une décision de conception. Il dépend désormais
de trois choses : le harnais de validation, les bancs, et quatre accords inter-équipes listés dans
`docs/00_INDEX.md`.

**Prochaine session recommandée.** S03 — deux options, à trancher par l'équipe :
- *(a)* écrire la spécification du harnais de validation, qui est sur le chemin critique et que
  personne d'autre ne bloque ;
- *(b)* descendre d'un cran sur ADR-007 et produire les signatures détaillées de `IWaveSolver` et
  `IFluidSolver`, préalable à B2 et B3.
Recommandation : *(a)*, parce que le harnais conditionne la valeur de tous les bancs et qu'il est
utile même si les interfaces changent.

**Ce qui reste sans propriétaire.** Les quatre interfaces inter-équipes. Elles ne peuvent pas être
résolues depuis ce poste — elles demandent une réunion, pas un document.

---

## S03 — 2026-09-05 — Harnais de validation

**Sorties.** ADR-020 ; `validation/SPEC-003-harnais-de-validation.md` ;
`validation/CAS-CANONIQUES.md` (18 montages) ; registre porté à 49 angles morts ; correction
d'ADR-010 ; index et plan de benchmark mis à jour.

**Décision structurante — obtenue à l'envers.** Le harnais exige des centaines d'exécutions
rapides ; il ne peut donc pas démarrer le moteur ; donc **le système d'eau doit être une
bibliothèque autonome derrière sept interfaces d'hôte** (ADR-020). Le raisonnement part de l'outil
de mesure et remonte jusqu'à la structure du code. C'est une contrainte bloquante, à acter avant
la première ligne : elle ne se rétroporte pas.

Effet secondaire notable : l'**hôte serveur**, qui n'exécute ni δ ni rendu, transforme l'invariant
I-04 en propriété vérifiée par le build plutôt qu'en règle de revue de code.

**Trois régimes de déterminisme**, dont l'intermédiaire est celui qu'on oublie : D1 exact
inter-plateforme (B, W, V), **D2 reproductible sur la même machine** (δ avec graine imposée), D3
statistique. Sans D2, un bug qui survient une fois sur cinquante n'est jamais reproduit.

**Pièges méthodologiques identifiés** — ce sont les vraies trouvailles de la session, et ils sont
tous hors de la physique :
- comparer des solveurs à `dx` égal désigne le mauvais candidat, avec des chiffres à l'appui ;
- sans oracle indépendant (un solveur délibérément lent), « différent » et « faux » se confondent ;
- une dérive de 1 %/semaine est invisible par commit et vaut +68 % sur un an ;
- le chemin de dégradation est le moins testé et le plus exécuté ;
- sans paire nulle, un jury perceptuel produit du bruit présenté comme une donnée.

**Erreur trouvée dans nos propres écrits.** ADR-010 donnait une vidange de citerne en 6 min :
valeur à charge constante. La charge décroît ; le temps réel est de **12 min**. Découverte en
écrivant le cas de test C12 qui devait la vérifier — argument suffisant pour écrire les tests avant
le code. Corrigée dans ADR-010 avec une note visible ; enregistrée en A49.

**Chemin critique redessiné.** `ADR-020 acté → H1 → (C01, C02 → λ_cut → B2) et (H4 → B3) → B4`.
H1 précède la première ligne du solveur.

**Prochaine session recommandée.** S04 : descendre d'un cran sur ADR-007 et produire les signatures
détaillées de `IWaveSolver` et `IFluidSolver`, plus les sept interfaces d'hôte d'ADR-020. C'est
désormais le seul document qui manque avant que du code puisse être écrit.

**Décisions qui demandent un arbitrage humain** — inchangées (trois de design, quatre interfaces
inter-équipes), plus une nouvelle : **qui possède le harnais ?** Ni l'équipe eau seule (juge et
partie), ni une équipe d'outillage détachée du domaine. Proposition : propriété eau, revue par
l'assurance qualité technique. → SPEC-003 §11.4

---

## S04 — 2026-09-05 — Signatures des interfaces

**Sorties.** `specs/SPEC-004-interfaces.md` ; registre porté à 55 angles morts ; protocoles de B2
et B4 complétés ; notes de renvoi dans ADR-007 et ADR-020 ; index mis à jour.

**Contenu.** `IFluidSolver`, `IWaveSolver`, `IBackgroundField`, `ISolidSource`, les six services
d'hôte, les types fondamentaux, le contrat de fils d'exécution, le modèle d'erreur et le
versionnement.

**La trouvaille de la session — sévérité 1.** En régime perturbatif, l'équation de la perturbation
contient un terme source `S = ∂U/∂t + (U·∇)U + ∇P/ρ − ν∇²U`, non nul parce que le fond résout les
équations d'ondes *linéaires* et non les équations discrètes du solveur. Un `IBackgroundField` qui
ne renverrait que hauteur et vitesse rendrait ce terme incalculable, et le symptôme — dérive lente
du domaine, frontière redevenue visible — serait **attribué à tort à une faillite d'ADR-001**.
Quatre sessions de conception ne l'avaient pas fait remonter ; il est apparu au moment de décider
quels champs la fonction devait renvoyer. Enregistré en A50, intégré au protocole de B4.

Corollaire pratique : le fond étant lisse à l'échelle de `dx` (ses longueurs d'onde valent au
minimum `λ_cut`), il s'échantillonne sur un réseau grossier et s'interpole — facteur 64, ce qui
fait passer le terme source de rédhibitoire à négligeable.

**Second effet de bord notable — il tranche partiellement B2.** `advance(t)` doit être une fonction
pure du journal d'événements. Les paquets lagrangiens le sont ; un champ de hauteur 2D intégré ne
l'est pas et impose des points de reprise à stocker, répliquer et transmettre à toute arrivée en
cours de partie — exactement le coût réseau qu'ADR-009 avait supprimé. Ajouté au protocole de B2 :
mesurer la taille et la fréquence des points de reprise pour chaque candidat.

**Principe de conception retenu et généralisé.** Rendre l'interdit inexprimable plutôt que de
l'interdire : pas de lecture GPU synchrone dans l'API, aucun type n'exprime une coordonnée monde,
`seal()` fait échouer l'allocation générale, `accumulate_force` ne mène qu'à la pose de rendu — ce
qui transforme l'invariant I-04 en propriété mécanique. Une section entière de SPEC-004 (§9) liste
les demandes prévisibles et leur motif de refus.

**Correction apportée à ADR-020.** La surface d'hébergement compte **six interfaces et un
paramètre**, non sept interfaces : l'horloge est poussée, pas lue. C'est plus strict — un système
incapable de lire l'heure ne peut pas en dépendre par accident.

**État du projet.** Il n'y a plus de document bloquant. Conception, chiffrage, validation et
interfaces sont posés ; ce qui reste est du code, des mesures et des réunions.

**Prochaine session, si elle a lieu.** Plus rien n'est sur le chemin critique côté conception.
Trois candidats utiles, par ordre décroissant de valeur :
1. la spécification de l'outillage auteur (rivière source de vérité, gravure du terrain, précalcul
   côtier) — c'est la dépendance inter-équipes la plus lourde et la moins avancée ;
2. `CondensedState` et la persistance hors caméra, seul point de SPEC-004 volontairement reporté ;
3. une relecture critique de l'ensemble à froid, en cherchant les contradictions entre ADR — vingt
   ADR écrits en quatre sessions n'ont jamais été confrontés les uns aux autres de façon
   systématique.

Recommandation : **(3) puis (1)**. Une relecture croisée coûte peu et trouve ce qu'aucune session
d'écriture ne peut voir.

---

## S05 — 2026-09-05 — Revue croisée des vingt ADR

**Sorties.** `registres/REVUE-CROISEE-S05.md` (12 écarts) ; ADR-021 ; invariants I-15 et I-16 ;
notes correctives dans ADR-005, ADR-006, ADR-008, ADR-009, ADR-012, ADR-014 ; corrections dans
SPEC-004, PLAN-BENCHMARK et CAS-CANONIQUES ; registre porté à 58 angles morts.

**Résultat brut.** Douze écarts entre documents, dont **deux de gravité 1**. Aucun n'était visible
en relisant le document qui le contenait ; il fallait un second document pour les révéler.

**R03 — deux origines pour une même onde répliquée** *(gravité 1)*. ADR-005 faisait de la
transduction δ→W la voie d'influence de δ sur le monde répliqué, avec validation serveur ; mais
I-10 interdit au serveur d'exécuter δ. Le mécanisme de validation contenait sa propre réfutation :
*si le serveur connaît la cause, il peut émettre l'événement lui-même*. Résolution ADR-021 : les
ondes répliquées sont émises par le serveur depuis les causes, la transduction client ne produit
que du `W_local`. **Le chemin d'énergie client → serveur disparaît, et avec lui l'angle mort A16 en
totalité** — il n'est plus atténué, il n'existe plus. Une résolution qui retire du système.

**R04 — mémoire et temps se contredisaient** *(gravité 1)*. ADR-012 déclarait `memoire_blocs`,
`gpu_sim_ms` **et** `domaines_max = 24`. La cohérence mémoire avait été vérifiée en S01 ; la
cohérence temporelle jamais. 24 domaines dans 2,5 ms font 0,10 ms par domaine, soit de l'ordre de
4·10⁹ mises à jour de cellule par seconde de GPU — optimiste d'au moins un ordre de grandeur. La
mémoire autorisait six fois plus de domaines que le temps. `domaines_max` n'est pas une ressource
mais un résultat : retiré du profil, désormais calculé à l'initialisation. D'où l'invariant I-16.

**Généralisation retenue — I-15.** Les quatre écarts R02, R03, R05, R08 posaient la même question
sous quatre formes. Une règle unique les couvre : *une grandeur dérivée est autoritaire si et
seulement si tous les participants peuvent la calculer à l'identique à partir de données
répliquées*. I-04 en devient un corollaire, et l'argument d'« exception contrôlée » cesse d'être
recevable.

**Ce qui a été vérifié et tient** — la revue rapporte aussi ses contrôles passés : chaîne des
budgets temporels, tables numériques dupliquées, cohérence mémoire, plafond de la pose de rendu,
chaîne de déterminisme, traçabilité des 30 sections sources, et treize des quatorze invariants.

**Prochaine session recommandée.** S06 : la spécification de l'outillage auteur — rivière source de
vérité et gravure du terrain (ADR-011 §3.1), précalcul côtier (ADR-013 §4), polyligne de
déferlement pour l'audio (ADR-016 §2), `sky_exposure` et `shape_lut` (ADR-010 §2). C'est la
dépendance inter-équipes la plus lourde et la moins avancée, et elle produit des données dont
plusieurs ADR dépendent déjà.

Deuxième candidat, moins urgent : recroiser les **quatre spécifications** entre elles, ce que S05
n'a pas fait — la revue a confronté les ADR, pas SPEC-001 à SPEC-004.

---

## S06 — 2026-09-05 — Outillage auteur, et dispositif de passation

**Consigne reçue.** Deux demandes : la spécification de l'outillage auteur, et un dispositif
permettant à un autre compte Claude de reprendre le projet à la fin de chaque requête, dans les
deux sens.

**Sorties.** `specs/SPEC-005-outillage-auteur.md` ; `REPRISE.md` à la racine ; README et index mis à
jour ; registre porté à 64 angles morts ; trois leçons.

### Outillage auteur

**Trouvaille de gravité 1 — le géoïde absent de l'outil de terrain.** ADR-002 pose que le niveau
moyen est un géoïde. Un outil de terrain travaillant en plan tangent donne une mer plate ; l'écart
vaut `R(1−cos θ)`, soit **70,7 m à 30 km** de l'ancre de région, 7,9 m à 10 km, et déjà 71 cm à
3 km — plus que le marnage. Un artiste plaçant une plage à 30 km la place 70 m hors de l'eau. Le
« zéro » d'une scène n'est pas une altitude mais une distance au centre de la planète. À porter à
l'équipe terrain **avant qu'un mètre carré de côte ne soit sculpté** ; c'est désormais la plus
urgente des quatre dépendances inter-équipes.

**Le cycle eau ↔ terrain.** La rivière impose sa pente au terrain (1,6 ‰), le trait de côte impose
le fetch, la bathymétrie impose la zone de déferlement. Le graphe contient un cycle qui, non brisé,
bloque le pipeline. Décision : **l'eau est en amont du terrain**, ordre de résolution imposé en
quatre étapes, deux ou trois itérations. C'est l'inverse de la pratique courante et ce sera la
décision la plus coûteuse à faire accepter ; le motif tient en une ligne — l'eau obéit à une
contrainte physique, le terrain non.

**Correction d'une hypothèse implicite d'ADR-013 §4.** La bibliothèque côtière ne peut pas stocker
des volumes 3D : seize états à 12 Mo font 197 Mo **par plage**. Elle stocke des conditions
initiales 2D — 77 Ko par état, 1,2 Mo par plage — et le volume se ré-établit en 2 à 3 s au lieu
de 40. C'est ce qui rend l'activation d'une plage compatible avec une fenêtre de prédiction réelle.

**Partitionnement de cuisson.** Une houle ne sent le fond qu'en deçà de `h = λ/2` : retoucher un
haut-fond invalide la réfraction jusqu'à l'isobathe 50 m, soit 10 km au large sur un plateau à
1:200. Les partitions suivent donc les **isobathes**, pas une grille carrée.

**Deux validations gratuites** : un `shape_lut` non monotone révèle un maillage non étanche ; une
gravure impossible révèle un tracé incompatible avec le terrain, pendant l'édition.

### Dispositif de passation

`REPRISE.md` à la racine : rôle, règles de travail, carte de la connaissance, état, arbitrages,
**rituel de fin de session obligatoire**, et un **jeton** pour qu'une seule session travaille à la
fois. Tout ce qui vivait dans une mémoire privée de compte y est désormais, parce qu'une mémoire
privée ne voyage pas.

Limites signalées et non contournées : le dépôt n'est pas sous gestion de version, et le partage
des fichiers entre comptes relève de l'infrastructure de l'utilisateur. Aucun document ne peut y
suppléer.

**Prochaine session recommandée.** S07 — recroiser les **cinq spécifications** entre elles, ce que
S05 n'a pas fait (la revue a confronté les ADR, pas les SPEC). SPEC-005 vient d'ajouter des
chiffres qui touchent SPEC-001 et ADR-013 ; le risque de contradiction est frais.

Second candidat : `CondensedState` et la persistance hors caméra, seul point de SPEC-004
volontairement reporté.

---

## S07 — 2026-09-05 — Gestion de version et reprise après interruption

**Consigne reçue.** Faire le `git init`, puis construire un dispositif permettant à un compte qui
reprend une requête interrompue par une limite d'usage de continuer proprement.

**Sorties.** Dépôt git initialisé (`c6886a7`, 41 fichiers), `.gitignore`, `.gitattributes` ;
`notes/EN-COURS.md` ; `REPRISE.md` §7 et jeton refondu ; README ; leçons L28 à L30.

### Le raisonnement

Une session coupée par une limite d'usage n'a **aucune occasion d'écrire qu'elle s'arrête**. Tout
dispositif reposant sur une action au moment de l'arrêt — résumé final, mise à jour d'état — est
inutile précisément dans le cas pour lequel on le conçoit. La seule information exploitable est
antérieure.

D'où le dispositif, en trois pièces :

1. **Écriture anticipée.** Le plan complet est déclaré dans `notes/EN-COURS.md` et committé *seul*,
   avant toute modification. Chaque étape porte sa **thèse** en une ligne — ce qu'elle doit
   démontrer — et pas seulement son intitulé : une session qui reprend peut alors *finir
   l'argument* au lieu d'en inventer un autre.
2. **Git comme détecteur d'achèvement.** Ce qui est committé est fait ; ce qui est modifié non
   committé appartient à l'étape marquée `[>]`, et à elle seule. Un commit par étape, aucune étape
   dépassant une quinzaine de minutes — c'est la seule prophylaxie réelle contre une coupure.
3. **Notes de reprise.** Un espace pour ce qui n'est encore dans aucun fichier : un chiffre
   calculé, une décision prise, **une impasse explorée**. Git conserve les fichiers, jamais le
   raisonnement ; ce qui disparaît d'abord dans une interruption n'est pas le travail produit mais
   le « j'ai essayé X, ça ne marche pas parce que Y ».

Le jeton passe à trois états — `libre`, `occupé` (battement < 2 h), `interrompu` — parce qu'avec
deux états, personne n'ose reprendre : un jeton `occupé` est indiscernable d'un jeton abandonné.

Le rituel de fin est désormais **une étape du plan** : interrompue, elle reste visiblement non
cochée.

### Le protocole appliqué à lui-même

S07 a été conduite sous son propre protocole. Deux défauts en deux étapes :

- l'étape qui *crée* le journal d'intention ne peut pas cocher sa propre case avant qu'il existe —
  la première étape d'un protocole d'écriture anticipée n'est jamais protégée par ce protocole ;
- la case doit être cochée **en dernière action avant le commit**, pas avant le travail, faute de
  quoi l'historique ment dans l'autre sens et fera refaire du travail déjà fait.

Aucun des deux n'était visible à la relecture. D'où la leçon L30 : un protocole qu'on n'a pas
exécuté est un protocole faux.

**Ce qui reste sans réponse.** Il n'y a pas de dépôt distant : la passation entre deux machines
repose sur un dossier partagé, et deux sessions écrivant en parallèle n'auraient aucun moyen de
fusionner. À proposer à l'utilisateur.

**Prochaine session recommandée.** S08 — recroiser les cinq spécifications entre elles, reporté
depuis S06. SPEC-005 a ajouté des chiffres qui touchent SPEC-001 et ADR-013 ; le risque de
contradiction est frais et la revue croisée des ADR a montré ce que ce type d'exercice rapporte.

---

## S08 — 2026-09-05 — Revue croisée des cinq spécifications

**Consigne reçue.** « Reprends le projet. » Jeton `libre`, démarrage à froid ; session recommandée
par S07 et reportée depuis S06 : recroiser les cinq SPEC entre elles, ce que la revue de S05 n'avait
pas fait — elle avait confronté les vingt ADR, et SPEC-005 n'existait pas encore.

**Sorties.** `registres/REVUE-CROISEE-S08.md` (dix écarts, deux de gravité 1, huit contrôles passés
détaillés) ; six notes correctives datées dans SPEC-001, SPEC-004 et SPEC-005 ; registre porté à 70
angles morts ; leçons L31 à L34.

### Vérification préalable, avant tout croisement

Une quarantaine de valeurs de SPEC-001 et SPEC-002 ont été **recalculées depuis leurs formules**
avant de chercher la moindre contradiction : dispersion, CFL, `dx⁻⁴`, énergie, fetch, Kelvin, ulp
du temps, Monahan, Weber, Stokes, Boyle, bilan du vide, Stefan, emportement, acoustique, optique.
**Aucune erreur.** Les deux fiches chiffrées sont saines. C'était la vérification la moins
intéressante à faire et la plus nécessaire : sans elle, tout écart trouvé plus loin aurait pu venir
d'un chiffre faux plutôt que d'un désaccord entre documents.

### Les deux écarts de gravité 1

**E04 — le chemin poussé n'existe dans aucun document.** SPEC-004 spécifie le chemin *tiré*
(`EvalWaterBatch`) et le chemin de *branchement de solveur*. Trois ADR exigent un troisième chemin,
où le système d'eau **publie** à basse fréquence ce que personne ne vient chercher : champ de
moussage (ADR-014 §2), `TraversabilitySample` par cellule à 5 Hz (ADR-018 §1), bus d'événements
audio (ADR-016 §2). Aucun n'a de signature, et **`WaveEvent` n'est défini nulle part dans SPEC-004**
alors qu'il traverse trois frontières et qu'ADR-016 demande de l'élargir « avant de figer le
format ».

Ce ne sont pas trois oublis : c'est un **mode de communication absent du modèle**. SPEC-004 a été
écrite depuis le point de vue du consommateur qui interroge, et tout ce qui se publie sans être
demandé est passé au travers — trois ADR ont alors chacun décrit sa propre publication dans son
propre vocabulaire (L22, à trois exemplaires).

La conséquence donne la gravité : **trois des quatre interfaces inter-équipes en attente d'accord
humain n'ont aucun document à soumettre.** On demandait à l'audio, à l'IA et au rendu de confirmer
une interface écrite nulle part sous forme de signature. Le risque annoncé dans l'index était donc
sous-estimé : il ne s'agit pas d'obtenir un accord, il s'agit d'abord d'avoir quelque chose à
présenter.

**E07 — une cuisson « bit à bit » exigée d'un solveur qui n'est jamais D1.** SPEC-005 §7.2 exige
qu'une cuisson soit reproductible à l'octet près entre machines. Or la bibliothèque côtière est
produite en faisant tourner **δ** (§7.1 — c'est même l'argument central : l'état cuit est
*exactement* ce que le jeu produirait), et SPEC-003 §2 pose qu'un solveur δ n'est **jamais D1**, au
mieux D2 : même binaire, même machine. L'exigence est inatteignable par construction.

Résolution par L24 — chercher l'hypothèse commune plutôt que départager. Les deux branches
supposaient que la cuisson devait être *reproductible* ; elle n'a pas à l'être. Le motif exige
seulement que **tous les participants chargent le même octet**, ce qui s'obtient par un **producteur
unique**, pas par un calcul reproductible partout. Une cuisson livrable est donc **autoritaire, pas
reproductible** ; le `bake_manifest` de §7.3 porte déjà tout ce qu'il faut. La correction retire une
exigence et n'ajoute aucun mécanisme. Ce qui reste exigible, et l'était déjà : le régime **D2** de
l'outil, sans lequel une cuisson n'est pas déboguable.

### Chiffres qui ont orienté la conception

- **4,4 à 8,0 s** — établissement de la structure verticale d'un train de houle (≈1 période,
  λ = 30 à 100 m). SPEC-005 §6 annonçait « 2 à 3 s » sans provenance, et faisait reposer dessus la
  faisabilité du précalcul côtier. Face aux **7,8 s** de fenêtre de préparation utile
  (`√(2R/a_max)`, ADR-013 §2), la conclusion tient mais la marge disparaît.
- **4 points par longueur d'onde** — ce que devient l'échantillonnage grossier du champ de fond
  (SPEC-004 §6.2, facteur ×64) dans une zone de déferlement à `dx = 0,25` avec `λ_cut = 4 m`, contre
  10 au scénario nominal à `dx = 0,10`. Deux fois Nyquist : l'économie s'effondre exactement dans le
  domaine le plus gros, et le symptôme est celui que §6.1 décrit comme indiagnostiquable.
- **0,63 m/s** — vitesse orbitale de crête à `Hs = 1 m`, `T = 5 s` (`πHs/T`). `WaterSample.u` mêle
  orbitale et courant ; le produit d'emportement de SPEC-002 §5 attend un **courant**. Calculé sur
  `u`, tout seuil de danger oscille à la période de la houle, à un ordre de grandeur qui vaut le
  double du seuil « faible / dangereux ».
- **3,4 km** — fetch maximal permettant la glace en plaque à `U10 = 5 m/s`
  (`F_max = g·(0,15/(0,0016·U10))²`, croisement de SPEC-002 §4 et SPEC-001 §4). 0,86 km à 10 m/s.
  Ce n'est pas un défaut mais une **dérivation nouvelle** : la glace en plaque est un phénomène de
  lac et de baie, jamais de haute mer. Elle borne le coût de l'arbitrage n°2.

### Ce qui n'a pas été fait

**Les signatures du chemin poussé n'ont pas été écrites.** C'est un travail de session entière et
non une note corrective : SPEC-004 §10 reçoit un point ouvert n°6 qui nomme le chemin manquant, ses
trois consommateurs et les cinq contraintes déjà établies qu'il devra respecter. Écrire ce document
est l'objectif recommandé pour S09.

**Prochaine session recommandée.** S09 — écrire le chemin poussé (SPEC-006, ou un §11 de SPEC-004) :
publication par tick à basse fréquence, instantané immuable à N lecteurs, anneau sans allocation,
et migration de `WaveEvent` dans SPEC-004 avec ses trois champs audio **avant que le réseau ne fige
le format**. C'est ce qui débloque trois des quatre accords inter-équipes.

Second candidat, inchangé depuis S06 : `CondensedState` et la persistance hors caméra.

**Arbitrages en attente — rappel.** Les trois arbitrages de design et les quatre interfaces
inter-équipes restent ouverts. L'arbitrage n°2 (la glace) est désormais accompagné du fetch maximal
ci-dessus, qui en réduit nettement la portée.

---

## S09 — 2026-09-05 — Le chemin poussé

**Consigne reçue.** « Enchaîne S09, écris le chemin poussé. » C'était l'objectif recommandé par S08,
qui avait constaté (écart E04, gravité 1) que ce chemin n'existait dans aucun document.

**Sorties.** [`specs/SPEC-006-chemin-pousse.md`](../docs/specs/SPEC-006-chemin-pousse.md) — neuf
sections, quatre canaux spécifiés ; quatre corrections dans SPEC-004 ; registre porté à 74 angles
morts ; leçons L35 à L39 ; index et registre S08 mis à jour.

### Ce qui est spécifié

Quatre canaux, et non trois : le bus d'événements avec `WaveEvent` enfin défini dans une
spécification, les champs d'écume `F` et d'aération `A`, la traversabilité par tuiles, et — trouvée
en relisant ADR-016 §2 — la **polyligne de déferlement**, qu'aucun ADR ne porte et que trois
consommateurs attendent.

Le document énonce d'abord ce qui distingue un chemin poussé d'un chemin tiré, parce que c'est de
ne pas l'avoir écrit que sont nées trois publications divergentes. La ligne qui compte se chiffre :
publier la traversabilité d'une zone de 4 km coûte **130 évaluations/s** contre 2 000 pour
200 agents qui interrogeraient `EvalWater` à 10 Hz — quinze fois moins, mais surtout **un coût
indépendant du nombre d'agents**. C'est la propriété que le chemin tiré ne peut structurellement
pas offrir.

### La règle qui a le plus de portée

> **Le chemin poussé publie au CPU des réductions, jamais des champs.**

Une cascade d'écume de 1024² en RG16F pèse 4,2 Mo ; quatre cascades à 30 Hz feraient ≈500 Mo/s de
lecture arrière et une à trois frames de latence pour répondre à une question qui tient en
45 octets — « combien d'écume autour de l'auditeur ». La réduction se fait donc dans la passe GPU
qui produit déjà le champ, et seul l'agrégat traverse. Le rendu, lui, reçoit une poignée de texture
sans copie. **Un même champ, deux publications de formes différentes** : c'est la décision
structurante du document.

L'occlusion acoustique par l'aération réutilise les **16 secteurs azimutaux d'ADR-005 §3** plutôt
que d'inventer une seconde discrétisation — L22 évitée en la voyant venir, pour une fois, plutôt
qu'en la corrigeant après.

### Quatre défauts trouvés en écrivant les signatures

Aucun n'était visible en relecture ; tous sont apparus au moment de poser une structure — L20, à
quatre reprises dans une seule session.

- **`drain_outgoing_events()` était un résidu.** La fonction servait le chemin δ→serveur qu'ADR-021
  §3 a supprimé en S05. L'écart R03 avait retiré le chemin de données ; la signature qui le servait
  a survécu, dans un document dont le statut est « dernier avant l'écriture de code ». Elle aurait
  été implémentée. Supprimée, pas renommée — et par-dessus, un drain a un **consommateur unique**,
  ce qui est intenable dès que l'audio, le rendu et le gameplay lisent le même flux.
- **`displaced_ml` est impossible sous ce nom.** Un `half` en millilitres sature à 65 litres,
  dépassé par n'importe quelle claque de coque. Publié en litres : mêmes deux octets, plafond
  65 m³. **Le même piège s'est reproduit vingt pages plus loin** — un flux dissipé en W/m sature à
  65 kW/m, dépassé dès `Hs = 4 m` (`P = E·c_g`, `E ∝ Hs²`). Publié en kW/m.
- **L'anticipation locale ferait jouer deux fois le même impact.** ADR-009 §7.2 autorise un client à
  émettre par anticipation l'événement que le serveur émettra ; les deux arrivent séparés du temps
  d'aller-retour réseau, soit 100 à 300 ms. Chacun des deux mécanismes est correct seul ; c'est leur
  conjonction qui produit le défaut, et personne n'en est propriétaire. D'où un bit de
  **rétractation** sur le bus.
- **`t_next_cross` publiait une prédiction sans son hypothèse.** « Ce gué se ferme dans quarante
  minutes » n'est vrai que si seule la marée agit. Une vanne ouverte en amont laisse la valeur en
  place, fausse, jusqu'à trente secondes — et un PNJ maintient son plan. La prédiction porte donc sa
  cause (`CrossCause`), et une commande de la couche V invalide l'aval. **Publier « je ne sais
  plus » est un résultat**, et son absence est ce qui rend une prédiction dangereuse.

### Deux conséquences d'I-15 qui rapportent

- **La traversabilité est calculée depuis les seules couches répliquées, δ exclu.** Sans quoi deux
  clients ne prendraient pas la même décision de cheminement, et un PNJ traverserait un gué chez
  l'un en se noyant chez l'autre — défaut rare, non reproductible, attribué au réseau. La portée de
  l'omission est bornée par l'argument de fermeture d'ADR-021 §3.2 : δ ne contient que ce qui est
  plus court que `λ_cut`.
- **Les franchissements de seuil ne transitent pas par le réseau.** Toutes leurs entrées étant
  répliquées ou cuites, chaque participant — serveur compris — dérive le même franchissement au
  même instant. Le serveur peut donc rendre une décision autoritaire (« ce gué est fermé ») sans
  simuler d'eau, ce qu'I-10 lui interdit. Même raisonnement qu'ADR-021 §3, même bénéfice : on
  supprime un chemin de données au lieu d'ajouter un arbitre.

### Dégradation

Le chemin poussé **dégrade en cadence, jamais en contenu** : jamais d'instantané partiel, jamais de
tuile à moitié remplie. Cinq rangs, cohérents avec ADR-012 §4 ; le rang 5 est la transposition
exacte d'ADR-021 §4 — on élague les événements locaux et cosmétiques, **jamais un événement
`Serveur`**. Et une ligne sans rang : brèche, franchissement de seuil et inondation de compartiment
ne se dégradent sous aucun budget. Trois assertions correspondantes sont à ajouter au banc `starve`
de SPEC-003 §9.1.

### Effet sur l'état du projet

Les quatre interfaces inter-équipes ont désormais un document à soumettre ; le préalable trouvé en
S08 est levé. **Une urgence de format demeure** : `WaveEvent` est une structure **répliquée** et
porte trois champs demandés par l'audio. Elle doit être arrêtée avant que le réseau ne fige son
format — l'ajouter après coûtera une migration de protocole. Coût vérifié, pas supposé : 45 octets
au lieu de 40, soit 900 o/s par joueur intéressé dans une zone à 20 événements/s, toujours
négligeable devant le trafic d'entités.

**Ce qui n'a pas été fait.** SPEC-006 n'a été croisée contre rien : elle est écrite, pas auditée.
S08 a montré ce que ce type d'exercice rapporte sur un corpus qui vient de grossir.

**Prochaine session recommandée.** S10 — `CondensedState` et la persistance hors caméra, seul point
de SPEC-004 volontairement reporté depuis S04 (§10.2), **et sa confrontation avec `CoastalState`**
(SPEC-005 §6). Les deux résolvent le même problème — amener un domaine dans un état non trivial
sans le simuler depuis zéro — par deux mécanismes distincts, écrits à deux sessions d'intervalle.
C'est la configuration exacte de L22, et il vaut mieux l'examiner **avant** d'écrire le second que
de le découvrir à la revue croisée suivante.

**Arbitrages en attente — rappel.** Les trois arbitrages de design restent ouverts. L'arbitrage n°3
(qui porte le trait de côte mobile) conditionne directement le nombre d'états de la polyligne de
déferlement publiée en SPEC-006 §6 ; l'arbitrage n°2 (la glace) conditionne deux champs de
`TraversabilitySample`. Les quatre interfaces inter-équipes attendent désormais une réunion, non un
document.
