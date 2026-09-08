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

---

## S10 — 2026-09-05 — La persistance de l'eau

**Consigne reçue.** « Enchaîne S10. » Objectif recommandé par S09 : `CondensedState`, la persistance
hors caméra, et sa confrontation avec `CoastalState` — une configuration L22 signalée mais non
examinée.

**Sorties.** [`adr/ADR-022-persistance-de-l-eau.md`](../docs/adr/ADR-022-persistance-de-l-eau.md) ;
invariant **I-17** créé et **I-03** amendé ; cas canonique **C19** ; notes correctives dans ADR-007,
ADR-012, SPEC-004 et SPEC-005 ; registre porté à 77 angles morts ; leçons L40 à L42.

### La question était mal posée, et depuis neuf sessions

`condense`/`restore` figurent dans ADR-007 §3 sous le commentaire
`// persistance hors caméra (architecture_globale §9)`. ADR-013 §6, **écrit la même session**, dit
de cette même question `§9` : « il n'existe pas de simulation ralentie hors caméra […] cela
supprime toute la question ». Le mécanisme a été dissous ; **la signature écrite pour lui est
restée**, et ADR-007 §5.3 a même créé une tâche — « format exact de `CondensedState` → ADR à
écrire » — pour servir un besoin qui n'existait déjà plus.

C'est **L35 une seconde fois**, une session après avoir été écrite pour un cas identique
(`drain_outgoing_events`, S09). Une décision se propage vers la prose qui l'explique, pas vers les
signatures, qui n'ont l'air de rien affirmer.

**La démonstration, couche par couche.** Rien de δ ne mérite d'être conservé : B est recalculé
(I-02), W se dérive de son journal d'événements (ADR-003 §3), un domaine perturbatif renaît à δ = 0
(ADR-013 §3), un domaine substitutif couplé à V rend un **entier** (ADR-010 §6), un déferlement
s'amorce depuis une donnée **cuite** (ADR-013 §4), les cascades d'écume sont transitoires par
construction (ADR-014 §2.3). Quatre confirmations indépendantes, dont la plus forte vient d'un
document écrit pour autre chose : **le harnais rejoue une session entière à partir de
`(T_sim, descripteurs, journal)`** (SPEC-003 §8). Si une session se rejoue sans état δ, l'état δ ne
fait pas partie de l'état du monde. C'était déjà écrit.

D'où l'invariant **I-17 — aucun état de δ n'est jamais sérialisé**. L'invariant vaut mieux que la
décision seule : sans lui, la demande reviendrait « juste pour ce cas-là », sous une forme qui aura
l'air raisonnable.

### `SeedState` : les deux moitiés d'un même mécanisme

`CondensedState` (S04) et `CoastalState` (S06) répondaient à la même question — *amener un domaine
dans un état non trivial sans le simuler depuis zéro* — sous deux noms, à deux sessions
d'intervalle. SPEC-005 §6 avait déjà la bonne forme, une **condition initiale 2D** et non un volume
figé, trouvée pour la bonne raison : 197 Mo contre 1,2 Mo par plage. Ce qui manquait était de voir
que cette forme n'a rien de côtier.

- **`condense` est une opération d'outil de cuisson**, `restore` une opération d'exécution. Suivant
  L19, `condense` quitte `IFluidSolver` pour un `ISeedProducer` que seul un hôte de cuisson obtient :
  un hôte de jeu ne peut pas condenser, faute d'avoir le type.
- **Cela referme l'écart E07 de S08.** Une graine est produite par δ, qui n'est jamais D1 — sans
  importance, puisqu'elle n'est pas un calcul reproductible mais un **actif identifié par
  l'empreinte de son contenu**. Les deux résolutions se rejoignent sans avoir été conçues ensemble.
- **Le document disait déjà ce qu'il était** : la contrainte inscrite en SPEC-004 §10.2 — « il doit
  se relire sur une machine différente, donc pas de disposition mémoire brute » — est celle d'un
  actif cuit, et dépourvue de sens pour une condensation en mémoire.

### Ce que l'eau met dans une sauvegarde

Personne n'avait posé la question ; elle l'aurait été tard, par l'équipe du format de sauvegarde,
avec une échéance. Trois choses : `T_sim`, les événements W vivants, les volumes entiers des nœuds V
**modifiés par rapport à leur valeur d'auteur**. Au pire ≈180 Ko d'événements et 2 Mo pour cent
mille nœuds — une sauvegarde d'eau est un petit objet, et il fallait le chiffrer avant que quelqu'un
ne conçoive un découpage dont personne n'a besoin.

**Le fichier de sauvegarde et la charge utile d'une arrivée en cours de partie sont le même
objet.** Conséquence d'ADR-003 : quand l'état du monde se réduit à un temps et à un journal, le
destinataire — disque, réseau, harnais — n'y change rien. Un seul format à écrire et à versionner,
et le harnais le teste déjà (SPEC-003 §8) sans qu'aucun test de sauvegarde ne soit écrit. D'où le
cas canonique **C19**, binaire parce qu'en régime D1 — ce qu'il n'aurait pas pu être si un état de δ
figurait dans la sauvegarde.

Deux points que le corpus impliquait sans les dire : **le TTL d'ADR-010 §7 est la borne supérieure
de la persistance de l'eau**, pas un nettoyage cosmétique ; et **sauvegarder force un règlement
δ→V**, sans quoi la masse d'un compartiment en cours d'inondation vit dans un champ qu'I-17 interdit
d'écrire.

### Deux écarts trouvés en chemin

- **ADR-012 rang 5 contre les domaines substitutifs.** La dégradation détruit les domaines non
  focaux et engage sa décision pour « au moins 30 frames », soit 1 s. Un domaine substitutif se
  rétablit en 4,4 à 8 s depuis une graine, 40 s depuis rien. Il serait donc détruit puis redemandé
  **quatre à huit fois plus vite qu'il ne se rétablit**. Le rang 5 ne porte désormais que sur les
  domaines perturbatifs.
- **I-03 était incomplet.** Il énonçait « B et W répliqué sont déterministes » ; SPEC-003 §2 place
  **V** dans le régime D1 depuis S03, et ADR-010 §4 avait pris toutes les dispositions pour cela.
  Sans déterminisme inter-plateforme de V, un serveur et un client divergeraient sur le volume d'un
  compartiment — sur une issue de jeu. Amendé. Gravité 3 en conséquences, mais il portait sur le
  document qu'on cite pour refuser une proposition.

Au passage, une closure que le corpus impliquait : **V est la seule couche que le serveur exécute**
(I-10 ne lui interdit que W et δ), et sa forme — entiers, report de reste, 10 Hz — n'était pas une
commodité d'implémentation mais la forme qu'une couche doit avoir pour être autoritaire au sens
d'I-15. Corollaire à annoncer tôt : **le serveur charge des données cuites** (les `shape_lut`), il
n'est pas « sans assets ».

### Ce qui n'a pas été fait

ADR-022 n'a été croisé contre rien, non plus que SPEC-006 en S09. Deux documents structurants
écrits coup sur coup et non audités.

**Prochaine session recommandée.** S11 — **auditer les listes « ce qui reste ouvert »**, de tous les
documents. C'est l'axe d'audit que S10 vient de montrer productif et que personne n'a jamais
parcouru : un audit vérifie ce qui est *affirmé*, et un point reporté se lit comme une lacune connue
plutôt que comme une contradiction possible. Le corpus en compte de l'ordre de cent, répartis sur
22 ADR et 6 SPEC, et l'un d'eux cachait la dissolution complète de son propre objet depuis neuf
sessions. Pour chacun : a-t-il encore un objet, sa formulation tient-elle encore, et quelqu'un
attend-il quelque chose dessus ? L'exercice couvre au passage SPEC-006 et ADR-022.

**Arbitrages en attente — rappel.** Les trois arbitrages de design restent ouverts, et un quatrième
point demande une décision qui n'appartient pas à l'équipe eau : la durée de vie d'un nœud V
rattaché à un objet d'un joueur absent depuis des mois (ADR-022 §7.2). Les quatre interfaces
inter-équipes attendent toujours une réunion, et `WaveEvent` reste l'urgence de format.

---

## S11 — 2026-09-05 — Audit des points ouverts

**Consigne reçue.** « Enchaîne S11. » Objectif recommandé par S10 : auditer les listes « ce qui
reste ouvert » de tous les documents — l'axe que la leçon L40 venait de désigner comme aveugle.

**Sorties.** [`registres/AUDIT-POINTS-OUVERTS-S11.md`](../docs/registres/AUDIT-POINTS-OUVERTS-S11.md) ;
34 marques appliquées dans 15 documents ; `00_INDEX.md` élargi ; registre porté à 80 angles morts ;
`METHODE.md` reçoit une **phase 7** ; leçons L43 à L45.

### Le chiffre

**110 points ouverts, 26 documents. Un sur trois n'était pas dans l'état où son document le
présentait.**

| Verdict | Nombre |
|---|---|
| **E** — valide | 69 (63 %) |
| **C** — formulation périmée | 15 |
| **D** — dupliqué | 13 |
| **B** — clos ailleurs, jamais marqué | 8 |
| **A** — dissous | 3 |
| **F** — pas une question | 2 |

Les 63 % de E sont eux aussi un résultat : la dette est réelle mais bornée, et les 41 points non-E
représentent, pour la plupart, du travail que quelqu'un aurait refait.

### La cause, et elle n'est pas celle qu'on croyait

Quatre points réclamaient un mécanisme qui existait déjà : ADR-007 §5.3 (un format pour une
persistance dissoute la même session), ADR-006 §7.3 (un nombre maximal de blocs dans un profil, ce
qu'I-16 interdit depuis S05), ADR-017 §7.2 (une subdivision « dédiée » alors qu'ADR-006 §2 la porte
depuis S05), SPEC-004 §10.3 (mesurer « un point sur quatre » alors que S08 a montré que le paramètre
est `N`).

L'hypothèse de la distance entre documents ne tient pas : **dans le dernier cas, la correction et le
point périmé sont dans le même document, à quatre sections d'écart.**

> **Une correction s'applique là où vit l'affirmation qu'elle corrige. Un point ouvert n'affirme
> rien : il déclare une absence.** Personne ne relit une liste d'absences en se demandant si l'une
> d'elles a été comblée.

D'où la règle ajoutée au rituel de fin : une session qui décide ou corrige quelque chose parcourt
les points ouverts qui le citaient. C'est une recherche de texte, pas une relecture.

### Le livrable que l'audit produit en plus

La troisième question — *qui attend, et quoi ?* — n'avait jamais été posée, et elle produit un
tableau que rien d'autre ne produisait.

- **Onze destinataires extérieurs**, là où `00_INDEX.md` en listait quatre. Les sept nouveaux —
  véhicules, personnage, gameplay spatial, gameplay survie, réseau/physique solide, gameplay,
  assurance qualité technique — étaient chacun cités **dans un point ouvert**, c'est-à-dire là où
  personne ne cherche une dépendance. Ils sont plus légers que les quatre interfaces, et ils ne se
  rattrapent pas mieux : un modèle de nageur décidé après que l'équipe personnage a figé sa machine
  à états coûte un recâblage, exactement comme un format audio.
- **B2 débloque quatre points ouverts**, plus que tout autre banc, et il porte désormais deux
  critères de recevabilité de `λ_cut` et non un (l'autorité des ondes répliquées **et** la validité
  du signal de navigation, SPEC-006 §5.6). C'est une confirmation indépendante du chemin critique,
  établi jusqu'ici sur les dépendances et non sur un décompte.
- **Un cinquième arbitrage humain** : qui possède le harnais de validation (SPEC-003 §11.4). Il
  était rangé parmi des questions de format de fichier, alors que SPEC-003 §1 pose que la qualité
  de toutes les décisions à venir est plafonnée par celle du harnais.
- **Quatre points disent « à spécifier » et aucune session ne l'a jamais pris en charge** : le terme
  d'impact de flottabilité (*slamming*), le modèle du nageur en surface, le comportement des rochers
  turbulents permanents, la coalescence de deux poches d'air T2.

### Deux symptômes à retenir

- **« Comme partout, dépend du langage »** — ADR-022 §7.4, à propos de la représentation binaire,
  posée par trois documents qui se justifiaient les uns par les autres. Un point ouvert qui se
  justifie par le fait que d'autres le posent aussi ne devrait pas exister.
- **SPEC-006 est la plus chargée du corpus et la plus propre** : six de ses huit points nomment leur
  porteur dans leur énoncé même. Le coût est d'une ligne, à l'écriture.

### Ce qui a changé dans les documents

Huit clôtures, quinze notes correctives, dix renvois avec porteur désigné, une requalification. Les
plus utiles : la bibliothèque côtière avait son format et ses volumes **depuis S06** et attendait
encore un banc ; l'anticipation locale était traitée par deux documents qui ne s'étaient pas
concertés ; le TTL de la couche V a cessé d'être une « calibration gameplay » pour devenir la borne
supérieure de la persistance de l'eau ; et l'arbitrage sur la glace porte enfin son chiffre — la
glace en plaque est bornée par le fetch, 3,4 km à 5 m/s de vent.

Une précision d'invariant, aussi : **le critère d'I-16 n'était pas opérationnel**. « Ressource »
contre « capacité dérivée » ne tranche pas le cas d'une taille de pool, qui est les deux. Le critère
qui fonctionne : *une valeur peut figurer dans un profil si elle est allouée directement ; pas si
elle doit être cohérente avec deux autres valeurs déjà déclarées.* C'est ce qui condamnait
`domaines_max`, contradictoire à la fois avec la mémoire et avec le budget de temps.

### Ce qui n'a pas été fait

L'audit n'a porté que sur les listes de points ouverts. Les **registres** — angles morts, questions
sources — n'ont pas été passés au même filtre : un angle mort comblé y figure-t-il encore comme
ouvert ? La question se pose et n'a pas été traitée.

**Prochaine session recommandée.** S12 — **les quatre points « à spécifier »** que l'audit a fait
remonter (§7.4) : terme d'impact de flottabilité `∝ ρv²A`, modèle du nageur en surface, rochers
turbulents permanents, coalescence des poches T2. Quatre sujets courts, indépendants, et sans
dépendance à un banc — c'est du travail de conception pur, le premier depuis S10, et il est
disponible immédiatement.

**Arbitrages en attente — rappel.** Ils sont maintenant **cinq**, et non trois : l'échelle du temps,
la glace, le trait de côte mobile, la durée de vie d'un nœud V d'un joueur absent, et la propriété
du harnais. Les quatre interfaces attendent une réunion ; sept autres équipes doivent fournir une
donnée ou un cadrage. `WaveEvent` reste l'urgence de format.

---

## S12 — 2026-09-05 — Les quatre mécanismes restés à spécifier

**Consigne reçue.** « Enchaîne S12. » Objectif recommandé par S11 : les quatre points qui disaient
« à spécifier » sans qu'aucune session ne l'ait jamais pris en charge.

**Sorties.** [`adr/ADR-023-mecanismes-restes-a-specifier.md`](../docs/adr/ADR-023-mecanismes-restes-a-specifier.md) ;
cas canonique **C20** ; quatre clôtures dans ADR-008, ADR-013 et ADR-015 ; registre porté à
83 angles morts ; leçons L46 à L48.

### Le résultat d'ensemble, et il dit quelque chose du corpus

**Quatre mécanismes spécifiés, zéro interface nouvelle.** Deux se sont résolus en **élargissant un
mécanisme existant**, un troisième en réutilisant un canal déjà spécifié, un seul a demandé un
mécanisme neuf — et celui-là réutilise une grandeur déjà imposée par l'ADR qu'il complète.

C'est la mesure la plus honnête de la maturité d'une conception : non pas ce qu'elle contient, mais
la proportion d'exigences nouvelles qu'elle absorbe sans mécanisme nouveau. Ces quatre points
auraient coûté plus cher il y a huit sessions.

Une précision de méthode : les quatre ont une **origine** commune — l'audit — mais leur **cause** ne
l'est qu'à moitié. Le terme d'impact et le nageur sont les deux frontières du domaine de validité
d'ADR-008, l'une dans le temps, l'autre dans la nature du corps. Les deux autres n'ont de rapport ni
avec eux ni entre eux, et il valait mieux l'écrire que de forcer une unification inexistante.

### Terme d'impact : publier ce qu'on sait, pas ce qu'on veut

`t_impact = 2b·tan β/(πv)` donne **73 ms** pour une étrave de vedette, **17 ms pour un corps humain
tombant de trois mètres** — contre 33 ms de tick. L'impact est plus bref que le tick : une force
échantillonnée le rate ou le double selon la phase, et la dispersion qui en résulte sur des entrées
identiques est intermittente, donc introuvable.

Le réflexe serait de publier la pression de pic, `C_p = 1 + (π/2tan β)²`. Deux enseignements
contradictoires en sortent : **c'est l'angle qui domine** — 30° → 10° multiplie la pression par dix,
doubler la vitesse ne la multiplie que par quatre — mais **la formule diverge quand `β → 0`**. On ne
connaît donc pas la pression de pic, et un modèle qui la publie publie son incertitude.

> **Décision : l'impulsion de masse ajoutée**, `J = Δ(½πρc²)·v_rel`. C'est un bilan de quantité de
> mouvement, qui ne peut pas être faux ; c'est ce qu'un intégrateur de corps rigide applique
> exactement ; et c'est le tenseur de masse ajoutée qu'ADR-008 §2 imposait déjà, exploité en régime
> transitoire au lieu du régime établi.

Contrôle croisé fait : 1,1 MN moyens sur 73 ms concordent avec 105 kPa de pic sur la surface
mouillée. Et le terme est **autoritaire** (I-15) : ses entrées sont l'état du solide et B + W, jamais
δ. C'est ce qui permet qu'un claquement de coque casse quelque chose.

### Le nageur : il n'y avait pas de modèle à écrire

Le mode cinématique contraint existe depuis S01, ADR-008 §3. Il lui manquait une **seconde condition
d'entrée**. Le critère actuel mesure la **stabilité numérique**, et un nageur le passe largement —
`A ≈ 0,25 m²`, `k ≈ 2 450 N/m`, `m + m_a ≈ 145 kg`, `ω·dt ≈ 0,14` — alors que c'est ce mode qu'il lui
faut, pour le contrôle et la caméra. Le critère n'était pas faux : il était seul.

Deux seuils **dérivés**, que le design n'aura pas à choisir :

- `πH/T = 0,7 m/s` → **par mer de 1 à 2 m, un nageur ne va plus où il veut** ;
- un nageur ne décolle de la surface que sous un **rouleau plongeant** — l'accélération descendante
  de crête de SPEC-002 §1 vaut `g` en plongeant et `0,45 g` en glissant, ce qui est exactement le
  critère de sortie, écrit trois sessions plus tôt pour tout autre chose.

Contrôle fait avant d'accepter : les cinq effets qui comptent — emportement, eau blanche,
déferlante, hypothermie, seuils de progression — passent tous par d'autres chemins déjà spécifiés.
Le mode contraint ne coûte rien au gameplay.

### Sites turbulents : le mot « émetteur » ne survit pas au chiffrage

ADR-013 §7.4 proposait des « émetteurs W stationnaires ». Deux cents sites émettant un événement par
seconde font **9 000 o/s par joueur intéressé — dix fois une bataille navale, en permanence, pour du
décor** (ADR-009 §2 : 900 o/s à 20 événements/s). Et un phénomène stationnaire déterministe n'a
aucune raison d'être répliqué.

Un site est un **terme stationnaire dérivé**, re-calculé à la demande, publié comme `BreakerVertex`
sur le canal existant de SPEC-006 §6 — une polyligne de déferlement dégénérée en un point. C'est
exactement le mécanisme qu'ADR-014 §2.3 avait retenu pour l'écume permanente : ADR-013 en proposait
un second sans le savoir.

Sa liste se dérive de `h < 1,28·H_local` (McCowan, SPEC-001 §3). **Bénéfice non demandé** : avec 4 m
de marnage, un rocher à 3 m sous le niveau moyen brise à basse mer et pas à haute mer par mer de 2 m.
Un récif qui gronde deux fois par jour à heure prévisible, sorti d'une inégalité.

### Coalescence : une addition, parce que le bon état avait été stocké

`n_fusion = n_a + n_b`. La règle tient en une ligne **parce qu'ADR-015 §3 a stocké `n_moles`** et pas
seulement pression et volume — le champ n'avait pas été introduit pour cela et rend l'opération
triviale huit sessions plus tard (L06 en petit). Le volume résultant sort d'une dichotomie sur le
`shape_lut`, monotone par construction : six itérations pour 64 entrées, et la même monotonie sert
de test d'étanchéité en SPEC-005 §9.

Deux ajouts que le point ouvert ne demandait pas : la **scission** — moles au prorata des volumes,
toute autre répartition faisant sauter la poussée d'une coque retournée quand une cloison émerge — et
l'**hystérésis** d'ouverture, `ε = 5 cm`, même parade qu'ADR-010 §5 pour les flaques.

### Ce qui n'a pas été fait

`ADR-023` n'a été croisé contre rien, et il rejoint `SPEC-006` et `ADR-022` dans le même cas : **trois
documents structurants écrits en quatre sessions, aucun audité**. Les registres non plus n'ont pas
été passés au filtre de S11 — un angle mort comblé y figure-t-il encore comme ouvert ?

**Prochaine session recommandée.** S13 — **confronter SPEC-006, ADR-022 et ADR-023 au corpus**. La
revue S05 a porté sur les vingt premiers ADR, la revue S08 sur les cinq premières SPEC ; ces trois
documents-là n'ont jamais rencontré personne. S08 avait trouvé dix écarts dont deux de gravité 1 sur
un corpus plus mûr et mieux relu que ces trois-là.

Second candidat, moins urgent : l'audit des **registres**, que S11 avait explicitement laissé de côté.

**Arbitrages en attente — rappel.** Cinq arbitrages humains, quatre interfaces à confirmer, sept
autres destinataires extérieurs. L'entrée **personnage** est désormais exécutable — il y a un
document à soumettre — et `WaveEvent` reste l'urgence de format.

---

## S13 — 2026-09-05 — Revue croisée des documents récents

**Consigne reçue.** « Reprends le projet. » Jeton `libre`, S12 close ; objectif recommandé :
confronter SPEC-006, ADR-022 et ADR-023 au corpus — trois documents structurants écrits en quatre
sessions et jamais audités.

**Sorties.** [`registres/REVUE-CROISEE-S13.md`](../docs/registres/REVUE-CROISEE-S13.md) ;
[`adr/ADR-024`](../docs/adr/ADR-024-amendement-des-invariants-I11-I12.md) ; **I-11 et I-12 amendés** ;
dix notes correctives dans six documents ; registre porté à 86 angles morts ; leçons L49 à L51.

**Douze écarts, un de gravité 1, six de gravité 2.** Et un résultat que je n'attendais pas :
**aucun des trois documents audités ne viole un invariant. Ce sont deux invariants qui ont vieilli.**

### La règle de conduite, et pourquoi elle a payé

J'ai écrit les trois documents audités. Ce que je croyais y avoir mis n'est pas ce qui y est, et un
audit mené de mémoire n'aurait trouvé que ce que j'attendais. Chaque contrôle est donc parti du
**texte du corpus**. Le rendement le confirme : les deux écarts les plus coûteux — E01 sur un
invariant, E11 sur la sauvegarde — sont exactement ceux qui n'étaient pas dans mes notes de plan.

### E01 — un invariant exige un mécanisme aboli *(gravité 1)*

I-11 énonçait : « Toute demande d'événement issue d'un client est **plafonnée** par une cause connue
du serveur. » ADR-021 §3.1, en S05 : « Le chemin d'énergie client → serveur **disparaît** […] le
mécanisme de plafonnement devient **sans objet**. »

Un invariant est le seul document qu'on cite **pour refuser** une proposition. Un invariant faux
produit donc deux erreurs en sens contraires : on implémente le plafonnement parce qu'un invariant
l'exige ; ou, constatant qu'il n'existe pas, on « rétablit » le chemin client → serveur pour pouvoir
le plafonner — c'est-à-dire qu'on **rouvre l'angle mort A16**, en obéissant à l'invariant.

Le nouvel énoncé est strictement plus fort : non plus une porte gardée, mais l'absence de porte.

### E07 — un invariant vrai pour une moitié des cas

I-12 posait que « créer et détruire un domaine est visuellement gratuit », en citant ADR-013 — dont
le §4 dit qu'un domaine **substitutif** met 40 s à s'établir. L'invariant faisait donc refuser le
document qu'il citait en source, et ADR-022 §2.6 a dû redémontrer de son côté ce qu'un énoncé correct
lui aurait donné. C'est le coût habituel d'un invariant trop large : il n'est pas seulement faux, **il
empêche de voir ce qu'il masque**, puisqu'on ne cherche pas d'exception à ce qui est posé comme
universel.

### Ce que ces deux écarts disent, et qui déborde le cas

Un audit a jusqu'ici deux prises : les **affirmations** qu'on confronte entre elles (S05, S08), et
les **absences** qu'on relit pour vérifier qu'elles ont encore un objet (S11). Un invariant n'est ni
l'un ni l'autre — il se présente comme le socle contre lequel on vérifie le reste, et la flèche
`→ ADR-xxx` qu'il porte se lit comme une provenance, pas comme une dépendance à surveiller.

Deux des dix-sept se sont révélés faux à la première tentative. D'où la règle ajoutée au rituel :
**une session qui écrit un ADR relit les invariants que cet ADR cite.** Trois minutes ; cela aurait
attrapé I-11 en S05 et I-12 en S01.

### E11 — la sauvegarde casse le banc qui devait la prouver

ADR-022 §4.2 fait figurer dans la sauvegarde un `span<const WaveEvent> events_alive` **sans filtre**,
alors que SPEC-006 §3.1 — écrite une session plus tôt — a doté `WaveEvent` d'un `EventOrigin` dont
deux valeurs sur trois sont locales et non répliquées.

Conséquence la plus gênante : **le cas C19 en devient faux.** ADR-022 §6.1 exige un hash identique
après aller-retour de persistance, en régime D1 donc binaire ; un événement `TransductionLocale`
vient d'un solveur δ, qui n'est jamais D1. Le banc conçu comme l'argument phare d'I-17 aurait échoué
par intermittence, pour une cause étrangère à I-17 — et un test juste qui échoue pour une cause
fausse discrédite le test, pas la cause.

Le document se contredisait lui-même : son §1 dit que l'état persistant tient en « `T_sim`, le
journal des événements W encore vivants, et les volumes entiers », où « W » désigne depuis ADR-021 §3
le seul `W_rep`. La correction **applique** le §1, elle n'ajoute rien.

### E04 — la première erreur arithmétique du corpus

Trois structures sur trois sont mal comptées. `WaveEvent` d'origine : 40 octets annoncés, **45** de
champs. `WaveEvent` étendu : 45 annoncés, **50**. `ListenerAggregate` : 45 annoncés, **50**. SPEC-006
avait ajouté cinq octets à une base fausse et retrouvé par coïncidence la taille réelle de l'original.

Propagation : six documents, neuf endroits. Aucune conclusion ne change — 1 000 o/s au lieu de 900,
≈205 Ko au lieu de 180, tout reste négligeable. **Ce qui compte est la classe de contrôle absente** :
S05 a vérifié les tables dupliquées, S08 a recalculé quarante valeurs depuis leurs formules, personne
n'a jamais additionné les champs d'un `struct`.

### Les autres

`ring_slots` avait deux règles de dérivation contradictoires **dans le même document, à une section
d'écart** (E03) — tranché par `max()` et un renvoi à `validate_config`. Une cadence « par tick de
rendu » contredisait la règle du diviseur énoncée une ligne plus haut, et aurait fait cinq fois le
travail sur une machine à 144 Hz (E06). Deux échelles de rangs de dégradation portent les mêmes
numéros dans deux documents qui se citent, et une règle de S12 s'y lit avec un contresens complet
(E02). Le terme d'impact, autoritaire, n'avait pas de ligne dans la table d'autorité — **répétition
exacte de l'écart R08 de S05**, dont la correction avait pourtant créé le précédent (E08). Un site
turbulent publié en un seul sommet ne pouvait pas exprimer son étendue de crête (E09). Et l'équation
de coalescence employait une **température que la structure ne porte pas** (E10) — corrigée en forme
`P·V`, ce qui retire une grandeur au lieu d'en ajouter une.

### Ce qui n'a pas été fait

**Le contrôle inverse n'a jamais été fait** : pour chaque invariant, l'ADR qu'il cite dit-il encore
ce que l'invariant résume ? S13 a vérifié que trois documents récents respectent les dix-sept
invariants ; c'est un autre exercice. Le contrôle inverse vient de rapporter deux écarts sur deux
tentatives, et il porte sur quinze invariants non examinés.

**Prochaine session recommandée.** S14 — **auditer les quinze autres invariants contre leurs ADR
sources**. C'est l'axe que S13 vient d'ouvrir sans le parcourir, il est court — deux pages
d'invariants, une flèche `→ ADR-xxx` par ligne — et son rendement observé est de deux écarts sur deux,
dont un de gravité 1.

Second candidat, inchangé depuis S12 : l'audit des **registres**, que S11 avait laissé de côté.

**Arbitrages en attente — rappel.** Cinq arbitrages humains, quatre interfaces à confirmer, sept
autres destinataires extérieurs. `WaveEvent` reste l'urgence de format — et sa taille réelle est
désormais **50 octets**.

---

## S14 — 2026-09-05 — Audit inverse des invariants

**Consigne reçue.** « Enchaîne S14. » Objectif recommandé par S13 : le contrôle inverse — pour
chacun des quinze invariants restants, l'ADR qu'il cite dit-il encore ce que l'invariant résume ?

**Sorties.** [`registres/AUDIT-INVARIANTS-S14.md`](../docs/registres/AUDIT-INVARIANTS-S14.md) ;
[`ADR-025`](../docs/adr/ADR-025-propriete-de-la-masse-entre-V-et-delta.md) et
[`ADR-026`](../docs/adr/ADR-026-amendement-de-six-invariants.md) ; **six invariants amendés** ;
`SPEC-001 §5 bis` ; registre porté à 89 angles morts ; leçons L52 à L54.

### Le chiffre

**Sept invariants tiennent, huit sont en défaut.** Avec les deux de S13 : **dix sur dix-sept ne
disaient plus ce que leur source dit.** Le corpus qu'on cite pour refuser une proposition était le
moins vérifié de tous.

### Le motif, qui vaut mieux que le chiffre

> Les invariants qui tiennent sont ceux qu'une **signature rend mécaniques** — I-07, où `g_eff` est
> injecté dans `configure` ; I-08, où aucun type n'exprime une coordonnée monde — ou ceux qui servent
> à **décider** plutôt qu'à refuser : I-09 et I-15, invoqués pour trancher *comment* faire.
>
> Ceux qui ont vieilli **nomment un mécanisme**. I-01 nommait une fonction, I-11 nommait un
> plafonnement, I-16 nomme une catégorie de valeur. **Un invariant qui nomme un mécanisme vieillit
> avec lui ; un invariant qui énonce une propriété ne vieillit pas.**

C'est une règle d'écriture, et les six amendements d'ADR-026 la suivent : chacun remplace la
désignation d'un mécanisme par l'énoncé de la propriété qu'il servait.

### L'écart de gravité 1 — et il ne vient pas d'un invariant

I-04 tient partout où on l'a confronté : ADR-008, ADR-014 §5.2, ADR-021 §5, ADR-023 §2.4. **C'est
ADR-010 §6 qui le contredit**, depuis S01 :

```
V → δ : le nœud est gelé, sa masse M est remise au domaine
δ → V : le domaine rend M' ; l'écart M' − M est reporté comme perte contrôlée
```

Trois conséquences. **δ détermine la masse finale d'un compartiment** — donc un chavirement, une
ligne de flottaison — alors que δ n'est jamais D1. **Le serveur n'a pas d'histoire** : il exécute V
(ADR-022 §5.1) et jamais δ (I-10), il ne peut ni geler le nœud ni recevoir `M'` ; pendant l'épisode,
serveur et client tiennent deux valeurs du même volume. Et **le même écart sert deux fois**, comme
diagnostic de fuite *et* comme perte réelle — une grandeur ne peut pas être l'erreur qu'on mesure et
l'effet qu'on applique.

Le défaut a survécu treize sessions parce qu'**ADR-010 ne cite pas I-04 et I-04 ne cite pas
ADR-010** : les revues croisées confrontent des documents qui se citent.

**ADR-025** retire le transfert. La masse appartient au nœud en permanence ; δ est amorcé depuis
`shape_lut(volume)` puis **forcé** vers le nœud par une relaxation lente (`τ ≈ 1 s`) ; `M' − M`
redevient un pur diagnostic. Aucun message n'est échangé — V étant D1, chaque participant intègre la
même valeur, exactement le bénéfice qu'ADR-021 §3 avait obtenu par le même raisonnement.

Et la correction **rapporte un critère que le banc n'avait pas** : l'écart de niveau résiduel vaut
`dérive_par_s · τ · h`, soit 1 cm pour 1 %/s sur une tranche d'un mètre. D'où un seuil d'admission en
régime substitutif — 1 %/s, à calibrer à B3 — là où ADR-010 §6 se contentait de remarquer qu'un
solveur perdant 3 %/s « est inutilisable ».

### Les sept autres défauts

**I-01** citait `EvalWater` comme passage obligé de tout consommateur — faux depuis S09 **par
décision**, ADR-018 §1 l'interdisant nommément à la navigation. **I-02** était contredit par son
propre ADR source, la grille `HydroSample` étant une représentation par nœud, sur disque, et écrite
dans la sauvegarde ; la distinction manquante — état contre paramètres — était déjà dans I-09.
**I-10** ne disait que ce que le serveur ne fait pas, d'où l'angle mort A77 : un lecteur en conclut
« aucune eau sur le serveur » et dimensionne un serveur sans assets. **I-14** ne connaît que SPEC-001
et SPEC-002 comme sources de formules, et ADR-023 en avait introduit trois ailleurs — résolu en
**migrant les formules de Wagner vers SPEC-001 §5 bis** plutôt qu'en élargissant l'invariant : son
prix est qu'il n'existe qu'un seul endroit où chercher un nombre. **I-16** attendait depuis S11 une
précision décidée, inscrite dans une table « Suite », et **jamais appliquée**. **I-03** couvrait la
couche V sans citer sa source. **I-13** renvoyait à un `WaterManager` qui n'existe nulle part.

### Et une quatrième structure mal dimensionnée

`HydroSample` (ADR-004 §2.2) est annoncée à 40 octets et en somme **34**. Après les trois de S13, le
taux d'erreur de cette classe de contrôle — jamais passée avant la revue précédente — est de
**quatre sur quatre**.

### Une erreur d'outil, vue parce que vérifiée

Les deux blocs de formules migrés vers SPEC-001 ont d'abord été **mangés par bash** : les accents
graves d'un bloc de code, dans un `python -c` passé à l'interpréteur, sont interprétés comme une
substitution de commande. Le script a signalé une erreur de syntaxe et a **poursuivi** ; le contenu
écrit était amputé sans que rien ne le dise. Corrigé par un commit séparé. C'est L09 étendue, et le
seul motif pour lequel le défaut a été vu est qu'on relit ce qu'on vient d'écrire.

### Ce qui n'a pas été fait

Les **registres** n'ont toujours pas été audités — un angle mort comblé y figure-t-il encore comme
ouvert ? C'est le candidat laissé de côté depuis S11.

**Prochaine session recommandée.** S15 — **auditer les registres**. C'est le dernier corpus jamais
passé au filtre : 89 angles morts, la traçabilité des 30 sections sources, et les tables « Suite » de
cinq registres d'audit — dont S14 vient de montrer (angle mort A89) qu'elles ne sont pas plus
exécutées qu'un point ouvert n'est relu. Le rendement attendu est élevé pour la même raison qu'en S11
et S14 : c'est une classe de contrôle qui n'a jamais été passée, et le corpus a montré quatre fois
que ces classes-là sont fausses à un taux voisin de 100 %.

**Arbitrages en attente — rappel.** Cinq arbitrages humains, quatre interfaces à confirmer, sept
autres destinataires extérieurs. `WaveEvent` reste l'urgence de format, à 50 octets.

---

## S15 — 2026-09-05 — Audit des registres

**Consigne reçue.** « Enchaîne S15. » Objectif recommandé par S14 : auditer les registres, le dernier
corpus jamais passé au filtre.

**Sorties.** [`registres/AUDIT-REGISTRES-S15.md`](../docs/registres/AUDIT-REGISTRES-S15.md) ;
trois actions de validation retrouvées et exécutées — dont le cas canonique **C21** ; table d'actions
rétrospective pour le registre S11 ; A89 reformulé ; registre porté à 91 angles morts ; leçons L55 et
L56.

**Huit écarts, un de gravité 1.** Aucun ne porte sur une décision de conception : ce sont tous des
défauts de **tenue**. C'est le propre d'un registre — il ne se trompe pas, il vieillit.

### Le résultat n'est pas un écart, c'est une cause

Sept actions du corpus annonçaient un ajout à un banc, au harnais ou à un cas canonique. **Trois
n'avaient pas eu lieu.** La première explication — « les documents de validation n'appartiennent à
personne » — ne résiste pas : quatre actions visant ces mêmes documents ont bien été faites.

> Les quatre exécutées l'ont été **par la session qui les décidait, dans une étape inscrite à son
> plan**. Les trois perdues avaient été **annoncées dans le corps d'un document**. Et cela vaut même
> à l'intérieur d'une seule session : le cas canonique annoncé par ADR-025 §4 en S14 P6a n'a pas été
> posé, parce que le plan de S14 ne prévoyait aucun cas canonique.

**Le plan de `notes/EN-COURS.md` est la seule liste que quelqu'un relit.** Une annonce faite en prose
est une intention, pas une tâche. C'est aussi ce que confirme le décompte inverse : les quatre
registres qui portent une table « Suite » affichent **40 actions sur 40 exécutées**, et le seul qui
n'en portait pas est celui dont une action s'est perdue neuf sessions.

### Ce qui manquait, et qui a été posé

- **Le second fondement de la recevabilité de `λ_cut`** au protocole B2 : un relèvement remettrait en
  cause l'autorité des ondes répliquées **et** la validité du signal de navigation (SPEC-006 §5.6).
  Un banc qui ne vérifierait que la première laisserait passer des PNJ qui traversent un gué chez un
  joueur et se noient chez un autre.
- **Trois assertions au banc `starve`** : aucun instantané au contenu incomplet, aucun événement
  `Serveur` absent, `sequence` strictement croissant. Sans elles, la dégradation du chemin poussé
  n'était vérifiée par rien — et c'est le chemin que SPEC-003 §9.1 dit « le moins testé et le plus
  exécuté ».
- **Le cas canonique C21** — la masse d'un compartiment est identique avec et sans domaine δ actif.
  Sans lui, la décision la plus lourde de S14 n'avait aucun moyen d'être contrôlée. Il est binaire,
  V étant entier et D1 : ce qui n'aurait pas été possible si la masse transitait par δ.

### Deux registres qui mentaient dans les deux sens

**Un statut périmé fait refaire un travail fait.** La table « Suite » de S05 annonçait « à ajouter au
protocole » pour un critère que `PLAN-BENCHMARK` B1 porte depuis la même session. Faux depuis dix
sessions, et dans le sens qui coûte. C'est l'inverse exact d'A89 — les deux erreurs cohabitent, ce
qui interdit de croire un statut sans le vérifier.

**Un registre dit où un point est discuté, jamais s'il est refermé.** La colonne « Traité dans »
d'`ANGLES-MORTS` confondait *supprimé*, *comblé* et *en attente d'un tiers* : **A59**, sévérité 1 —
le géoïde et ses 70 mètres — affichait la même chose qu'un point réglé.

**Et une question dissoute est revenue.** « Quelle erreur de précalcul est acceptable ? » avait été
dissoute à juste titre en S01 ; ADR-022 §3.5 a introduit en S10 une seconde forme de précalcul — la
graine — qui, elle, porte un seuil de tolérance réel, « le seul du système ». Le registre annonçait
toujours qu'il n'y avait rien à calibrer.

### Deux erreurs de ma part, corrigées

**Une erreur factuelle de S14**, propagée en trois endroits : l'action perdue sur I-16 n'était pas
« inscrite dans une table Suite » — le registre S11 n'en a pas. A89 est reformulé, et le constat en
sort renforcé plutôt qu'affaibli.

**Une erreur de vérification, en séance.** Mon relevé annonçait quatre actions de validation perdues ;
il y en avait trois. La commande de contrôle enchaînait plusieurs recherches par `&&`, l'une n'a rien
trouvé, et la chaîne s'est interrompue avant la suivante : le résultat affiché — rien — était
indiscernable d'une absence réelle. **C'est L54 une seconde fois, une session après l'avoir écrite.**

### Ce qui n'a pas été fait

La colonne `Statut` d'`ANGLES-MORTS` n'est renseignée que pour le point de sévérité 1 en attente
(A59) et posée en règle pour la suite. Les 89 autres lignes se renseigneront au fil des sessions qui
les touchent.

**Prochaine session recommandée.** S16 — **le corpus n'a plus de classe de contrôle non passée.**
Les ADR ont été confrontés entre eux (S05), les SPEC entre elles (S08), les points ouverts audités
(S11), les documents récents confrontés (S13), les invariants audités dans les deux sens (S14), les
registres audités (S15). Ce qui reste est ce que `REPRISE.md` §4 annonce depuis S06 : **du code, des
mesures et des réunions**.

Le travail de conception disponible sans mesure ni réunion est donc épuisé. Deux emplois utiles du
temps, dans l'ordre :
1. **préparer l'exécution de B2** — c'est le banc qui débloque le plus de points ouverts (S11 §7.1),
   son protocole est complet depuis aujourd'hui, et `λ_cut` est « à décider en premier » depuis S01 ;
2. **rédiger le dossier de réunion** des onze destinataires extérieurs, qui n'existe nulle part sous
   forme présentable — l'index en donne la liste, pas le contenu.

**Arbitrages en attente — rappel.** Cinq arbitrages humains, quatre interfaces à confirmer, sept
autres destinataires. `WaveEvent` reste l'urgence de format, à 50 octets.

---

## S16 — 2026-09-05 — Dossier d'exécution du banc B2

**Consigne reçue.** « Continue. » S15 avait constaté que le corpus n'a plus de classe de contrôle non
passée et proposé deux emplois du temps ; le premier était de préparer B2, sur le chemin critique
depuis S01.

**Sorties.** [`validation/DOSSIER-B2.md`](../docs/validation/DOSSIER-B2.md), dix sections ; notes
correctives dans ADR-005 §5, ADR-021 §7.3 et `PLAN-BENCHMARK` B2 ; registre porté à 93 angles morts ;
leçons L57 et L58.

### Ce que le corpus savait déjà de `λ_cut`, sans aucune mesure

C'est la trouvaille de la session, et elle sort de deux paragraphes que personne n'avait rapprochés.

**Borne haute — l'éponge.** ADR-005 §5 pose `L_s = λ_cut/2` par face et conclut que `λ_cut` « fixe le
coût minimal d'un domaine ». Il ne calcule jamais ce que cela laisse. L'intérieur utile d'un domaine
de largeur `W` vaut `W − λ_cut`, et le domaine d'impact de référence fait **6 × 6 m**
(SPEC-001 §2.4) :

| `λ_cut` | 3 m | **4 m** | 6 m |
|---|---|---|---|
| Intérieur utile, linéaire | 50 % | **33 %** | **0 %** |
| Intérieur utile, en surface | 25 % | **11 %** | 0 % |

À la valeur proposée depuis S01, **un domaine d'impact est à 89 % d'éponge**. À 6 m il n'a plus
d'intérieur du tout. **C'est le plus petit domaine qui borne `λ_cut`**, et la borne est `≤ 3 m`.

**Borne basse — l'échantillonnage du fond.** L'écart E08 de S08 fait dépendre la validité de la
décimation du rapport `λ_cut/dx`. Au `dx` le plus grossier — 0,25 m, le déferlement — `λ_cut = 4 m`
donne quatre points par longueur d'onde après décimation ×4, et il en faudrait dix, soit
`λ_cut ≈ 10 m`.

**Les deux bornes ne se croisent pas**, à un facteur trois près. Mais elles ne sont pas de même
nature : l'éponge est une contrainte **dure** — à 6 m le domaine n'existe plus — tandis que la
décimation est une contrainte **de coût** : quatre points ne donnent pas un résultat faux, ils
donnent une économie qui s'effondre, ce qu'E08 disait déjà.

> **D'où la résolution portée au protocole : `λ_cut` reste global, c'est le taux de décimation qui
> s'adapte.** E08 avait écrit la contrainte sous la bonne forme — `dx ≤ λ_cut/N` — en laissant `N`
> libre. `N` est le paramètre, pas `λ_cut`. Le facteur d'économie passe de 64 à **8** dans une zone
> de déferlement, et B2 doit mesurer le coût du terme source **à décimation réduite** : sans cela le
> coût de δ en régime substitutif est sous-estimé d'un facteur voisin de huit, sur le domaine qui
> compte 384 000 cellules.

Fenêtre restante : **2,5 à 3 m**, contre 4 m proposés depuis S01. Ce n'est pas une réfutation, c'est
une hypothèse chiffrée — et le banc doit donc **balayer 2 à 6 m**, pas confirmer 4.

### Trois apports au protocole du banc

- **Un cinquième scénario, B2-05** — arrivée en cours de partie à `t` = 30 s. L'ajout S04 de
  `PLAN-BENCHMARK` demandait de mesurer « la taille d'un point de reprise, sa fréquence, et le volume
  à transmettre » ; aucun des quatre scénarios existants ne l'exerçait.
- **La métrique d'iso-qualité, nommée.** SPEC-003 §5.2 impose de régler chaque candidat jusqu'à une
  qualité cible commune, ce qui suppose une métrique d'erreur **unique** — B2 n'en avait pas. Retenue :
  l'**erreur de célérité relative** sur le cas C02, intégrée sur `[λ_cut, 4·λ_cut]`. Motif : référence
  analytique fermée, et c'est la grandeur qui gouverne le défaut visible d'un candidat de W — un train
  de vagues qui se désynchronise du fond en quelques dizaines de secondes.
- **B2 produit un couple, pas un nombre** : `λ_cut` **et** le tableau des décimations admissibles par
  classe de domaine. Un protocole qui n'en publie qu'un laisse l'autre se choisir plus tard, à l'œil,
  par quelqu'un qui n'aura pas les mesures.

### Deux vérifications faites plutôt que supposées

**Le critère de fermeture n'est pas ce qui borne `λ_cut`.** ADR-021 §3.2 exige qu'aucun phénomène
gameplay ne puisse naître exclusivement dans δ. En énumérant les phénomènes ondulatoires du corpus,
le plus court est le **sillage à 5 m/s, `λ = 2πv²/g` = 16 m** : le critère laisse de la marge jusqu'à
`λ_cut ≈ 6 m`. Ce n'est donc pas lui qui mord — c'est l'éponge.

**Le « ≈27 % » d'ADR-005 §5 n'est pas reproductible.** Pour un domaine de 20 m avec 2 m d'éponge, la
fraction de volume vaut 20 % (deux faces), 36 % (quatre faces) ou 49 % (six faces) ; le paragraphe ne
dit pas lesquelles. La grandeur est le coût d'entrée de tout domaine, et elle fonde la borne haute.
Note corrective posée ; le choix des faces reste à trancher avant B2.

### Ce qui n'a pas été fait

Le **dossier de réunion des onze destinataires extérieurs** — le second emploi que S15 proposait —
n'existe toujours nulle part sous forme présentable. L'index en donne la liste, pas le contenu.

**Prochaine session recommandée.** S17 — **le dossier de réunion**. C'est désormais le seul travail
de conception disponible qui ne demande ni mesure ni décision humaine préalable : il consiste à
rassembler, pour chacun des onze destinataires, ce qu'on lui demande, ce qu'on lui fournit, ce qu'il
risque s'il répond tard, et le chiffre qui rend la demande concrète. Tout existe dans le corpus,
dispersé sur vingt-six ADR et sept spécifications ; rien n'est présentable.

Second candidat : le même exercice que S16 pour **B3**, le second banc du chemin critique.

**Arbitrages en attente — rappel.** Cinq arbitrages humains, quatre interfaces à confirmer, sept
autres destinataires. `WaveEvent` reste l'urgence de format, à 50 octets.

---

## S17 — 2026-09-05 — Le dossier de réunion

**Consigne reçue.** « Enchaîne S17. » Dernier travail disponible ne demandant ni mesure ni décision
humaine préalable : rassembler ce que le système d'eau attend des autres équipes. Tout existait,
dispersé sur 26 ADR et 7 spécifications ; rien n'était présentable.

**Sorties.** [`docs/DOSSIER-REUNIONS.md`](../docs/DOSSIER-REUNIONS.md), seize fiches ; renvoi depuis
`00_INDEX.md` ; registre porté à 95 angles morts ; leçons L59 et L60.

### Le classement change, et c'est le résultat de la session

L'index classait par **gravité de conséquence**. Le critère utile est l'**irréversibilité** : que
débloque la réponse, et qu'est-ce qui devient irrattrapable si elle tarde ? Quatre rangs — ce qui
bloque la première ligne de code, ce qui bloque le format d'une autre équipe, ce qui bloque un banc,
ce qui est un cadrage.

**Deux demandes remontent en tête, qu'aucun document ne présentait comme urgentes :**

- **acter ADR-020** — la bibliothèque sans dépendance moteur. Signalé comme bloquant depuis S03, mais
  absent de la liste des destinataires : personne n'était nommé pour l'acter ;
- **désigner le propriétaire du harnais** — rangé depuis S03 parmi des questions de format de
  fichier, alors que SPEC-003 §1 pose que « la qualité des décisions qui suivent est plafonnée par
  celle du harnais » et que H1 doit précéder la première ligne du solveur.

Et **les cinq arbitrages ne sont pas au même rang**. Celui sur l'échelle du temps est le plus lourd
du projet : si la réponse est « oui », ce n'est pas un paramètre qui change, c'est le modèle de
réplication qui tombe pour l'océan concerné — la cohérence de la houle, le modèle d'événements,
l'écume identique entre joueurs, l'autorité du signal de navigation. Une cascade sur quatre
documents, présentée jusqu'ici comme le premier item d'une liste de cinq.

### Quatorze demandes, pas onze

Le décompte de S11 comptait les *destinataires*. En écrivant les fiches, deux demandes distinctes
sont apparues chez des destinataires que la liste ne nommait pas — acter ADR-020, et **ne pas figer
`WaveEvent` avant que l'audio ait répondu**. Cette dernière est le seul endroit du dossier où
**l'ordre entre deux équipes** compte : audio d'abord, réseau ensuite, une seule échéance pour deux.

### Ce que chaque fiche porte

Une règle d'écriture propre à ce document : **chaque fiche porte un chiffre**. Une demande sans
chiffre se discute, une demande avec un chiffre se traite — L14 appliquée à une réunion.

70,7 m d'écart à 30 km pour le terrain. 50 octets et 1 000 o/s par joueur pour `WaveEvent`.
130 évaluations/s contre 2 000 pour l'IA. 0,63 m/s d'oscillation parasite si le danger est calculé sur
la mauvaise vitesse. 1,78 m de mer au-delà de laquelle un nageur ne fait plus route. 1,4 s d'horizon
de prédiction pour un avion de chasse. 14 % évaporé et 86 % gelé pour une brèche vers le vide. 3,4 km
de fetch maximal pour la glace. 1,2 Mo contre 77 Ko par plage selon la réponse sur le trait de côte.

### Une section qui manquait : ce que nous ne demandons pas

Sept lignes, et elles évitent des malentendus coûteux — que le système d'eau veuille modifier un
maillage de navigation, réclamer un budget de frame, faire simuler l'eau au serveur, ou qu'il faille
« attendre que l'eau soit finie pour commencer », ce que seules les quatre premières fiches
justifieraient.

### Ce qui bloque le dossier lui-même

**Les fiches 1 et 2 n'ont pas de destinataire nommé.** « Direction technique » et « assurance qualité
technique » sont des rôles, pas des personnes. Personne n'est identifié pour acter ADR-020 ni pour
arbitrer la propriété du harnais — et c'est la condition préalable à la tenue des réunions. Un
dossier de demandes sans destinataire identifié est un dossier qui ne part pas.

C'est le seul point de ce dossier que je ne peux pas résoudre : il demande de connaître
l'organisation, ce que le dépôt ne contient pas.

### Ce qui n'a pas été fait

Aucune date. Le dossier classe par irréversibilité, faute de calendrier — celui-ci n'existe pas
encore. Et le format des réunions n'est pas décidé : seize fiches ne se traitent pas en une séance.

**Prochaine session recommandée.** S18 — **le dossier d'exécution du banc B3**, même exercice qu'en
S16 pour le second banc du chemin critique. C'est ce qui reste de plus utile : B3 tranche le solveur
volumétrique, ferme quatre points ouverts, et S16 a montré ce que ce type de dossier rapporte — un
encadrement calculé sans mesure, une métrique d'iso-qualité qui manquait, et un scénario absent.

Second candidat : le même exercice pour **B4**, juge d'ADR-001, dont le protocole porte déjà deux
paramètres partagés avec B2.

**Arbitrages en attente — rappel.** Cinq arbitrages humains et quatorze demandes extérieures, toutes
désormais présentables. `WaveEvent` reste l'urgence de format, à 50 octets — et sa fiche dit
maintenant à qui parler, dans quel ordre.

---

## S18 — 2026-09-05 — Les cinq arbitrages, tranchés

**Consigne reçue.** « Prends les décisions. » L'utilisateur lève explicitement la règle qu'il avait
posée en S06 : les arbitrages de design ne sont plus hors de ma portée.

**Sorties.** [`adr/ADR-027`](../docs/adr/ADR-027-les-cinq-arbitrages-tranches.md), sept sections ;
sept notes correctives ; `CLAUDE.md` et `REPRISE.md` §5 réécrits ; quatre fiches du dossier de réunion
requalifiées ; registre porté à 96 angles morts ; leçons L61 et L62.

### Conduite adoptée

Une décision prise **par délégation** doit être plus argumentée qu'une décision ordinaire, pas moins,
et **bon marché à défaire**. Chaque section porte son motif, son chiffre, et **ce qu'il faudrait
changer si la réponse était l'inverse** — pour que revenir dessus coûte une lecture et non une
enquête.

### Trois des cinq n'étaient pas des arbitrages

C'est le résultat de la session, et il était invisible tant que les questions restaient classées
« pas à moi ».

- **Le trait de côte mobile se dissout.** « Qui le porte ? » suppose qu'il soit stocké. Il est dérivé
  de la marée analytique, donc **personne ne le porte** — même famille que l'écume permanente
  (ADR-014 §2.3) et les sites turbulents (ADR-023 §4). Ce qui est stocké est la bibliothèque à seize
  états : **45 Mo pour cinquante plages**, tout le prix de la décision. Et une côte fixe supprimerait
  la prédictibilité de la marée — « ce gué se ferme dans quarante minutes » — pour économiser cela.
- **La durée de vie d'un nœud V se dissout.** La question supposait que l'eau ait besoin d'une
  politique de rétention propre. Elle pèse **20 octets par nœud, 4 Ko pour la flotte entière d'un
  joueur** : négligeable devant l'état du navire lui-même. L'eau suit la politique de l'objet. Ce qui
  reste nôtre est le TTL des nœuds **sans propriétaire**, qui borne la persistance de l'eau à
  l'échelle du monde — et c'est celui-là qu'il faut défendre.
- **La propriété du harnais se dissout à moitié.** La question cherchait **un** propriétaire là où il
  en faut deux : le code et les scénarios à l'équipe eau, **les seuils d'acceptation à la qualité
  technique** — c'est la seule barre qu'on est tenté de déplacer quand on ne la passe pas. Règle
  mécanique plutôt que consigne (L19) : les seuils vivent dans un fichier séparé dont la modification
  exige une approbation ; ajouter un scénario ne passe par personne.

### Les deux vrais choix

**L'échelle du temps : non par joueur, oui globalement.** La question mêlait trois besoins de design
— voyage rapide, pause, mode photo — dont **aucun n'exige de mettre le temps à l'échelle par
joueur** une fois traités séparément : le voyage rapide est un déplacement dans l'espace, la pause en
solo arrête l'horloge pour tout le monde, le mode photo fige le rendu. Et l'échelle **globale** reste
disponible — un serveur peut accélérer son cycle jour/nuit sans que rien ne casse. C'est un degré de
liberté que personne n'avait relevé, et il coûte zéro.

C'est la seule des cinq dont l'inversion tardive détruirait du travail fait : quatre propriétés
tomberaient pour l'océan concerné, dont la cohérence de la houle entre joueurs. Le chiffre : une
composante de période 8 s déphasée de **4 secondes** place une crête chez l'un là où l'autre voit un
creux. Il n'y a pas de tolérance intermédiaire.

**La glace : oui, bornée par le fetch.** `F_max = g·(0,15/(0,0016·U10))²` — **3,4 km à 5 m/s**. La
question se posait comme si « oui » engageait un océan gelé ; il n'engage que des plans d'eau de
quelques kilomètres. La glace s'active **par plan d'eau**, jamais comme état météorologique global,
ce qui **borne** au passage le travail de l'IA au lieu de l'étendre.

### Le point le plus important pour les sessions suivantes

`CLAUDE.md` et `REPRISE.md` §5 disaient depuis S06 « cinq arbitrages attendent, les rappeler, ne pas
les trancher ». Sans réécriture, la session suivante les aurait rappelés comme si rien n'avait été
décidé. Le nouveau texte distingue ce qui est **tranché** de ce qui reste **hors de portée d'une
session**, et cette seconde liste n'est pas de la conception :

1. **nommer des personnes** — qui acte ADR-020, qui occupe les deux rôles du harnais ;
2. **constater l'état réel du projet** — terrain sculpté ? format réseau figé ? code écrit ? Tout le
   classement d'urgence du dossier de réunion suppose que non ;
3. **agir sur l'infrastructure** — le dépôt distant, signalé depuis S07.

Aucune quantité de raisonnement ne produit ces trois-là.

### Ce qui n'a pas été fait

Le dossier d'exécution de B3, recommandé par S17, est reporté.

**Prochaine session recommandée.** S19 — **dossier d'exécution du banc B3**, même exercice qu'en S16.
B3 tranche le solveur volumétrique, ferme quatre points ouverts, et son protocole porte quatre
scénarios dont un — le compartiment inondé en référentiel accéléré — vient de recevoir un cas
canonique en S15 (C21). S16 a montré ce que ce type de dossier rapporte : un encadrement calculé sans
mesure, une métrique d'iso-qualité qui manquait, un scénario absent.

**Ce qui attend encore une réponse humaine.** Quatorze demandes extérieures, en fiches présentables ;
et les trois choses hors de portée ci-dessus. `WaveEvent` reste l'urgence de format, à 50 octets.

---

## S19 — 2026-09-05 — ADR-020 acté, et il n'y a pas d'autres équipes

**Consigne reçue.** Trois informations : « J'acte ADR-020 » · « Tu es le seul à travailler sur le
projet, les développeurs observent » · « Je ne sais pas », sur les positions monde et sur l'état réel
du projet.

**Sorties.** [`adr/ADR-028`](../docs/adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md), six sections ;
statut d'ADR-020 changé ; six répercussions dont `REPRISE.md` §1 et `CLAUDE.md` ; registre porté à
97 angles morts ; leçons L63 et L64.

### ADR-020 est acté

Premier ADR du corpus à quitter le statut « proposée ». Conséquence unique et immédiate : **H1 est
écrivable**. C'était le seul verrou du chemin critique, posé en S03 et tenu dix-sept sessions.

### Il n'y a pas d'autres équipes — et c'est le vrai sujet

Les onze destinataires extérieurs recensés en S11, mis en fiches en S17, **n'existent pas comme
interlocuteurs**.

**Ce que cela ne change pas** : aucune contrainte technique. Le géoïde décale toujours une plage de
70,7 m à 30 km, que l'équipe terrain existe ou non.

**Ce que cela change** : quatorze « demandes extérieures » ne sont pas des demandes. Ce sont **des
décisions différées à personne**.

C'est L61 — écrite la veille — à l'échelle du corpus, et amplifiée par un mécanisme de plus.
L'étiquette « attend une autre équipe » ne dispensait pas seulement de répondre : **elle désignait un
responsable**. Un report nominatif est plus confortable qu'un report simple, et surtout il ne se
relit jamais — on ne vérifie pas qu'un tiers a répondu si l'on n'attend rien de précis de lui. C'est
pourquoi l'audit des points ouverts de S11, qui cherchait exactement ce type de dette, est passé à
côté : il vérifiait si un point avait encore un objet, pas s'il avait encore un destinataire.

**Règle retenue** : on ne classe plus rien en « attend une autre équipe », mais en **« à trancher,
sans interlocuteur »**. La première formulation sort la question du champ de travail, la seconde l'y
laisse. Toute la différence est là.

### Les positions monde, tranchées faute d'interlocuteur

`int64` en virgule fixe, résolution **1/2048 m**, portée ±4,5·10¹⁵ m.

**La résolution se dérive** : c'est exactement l'ulp d'un `f32` au rayon de référentiel
(`4096 · 2⁻²³ = 2⁻¹¹ m`). Plus fine, elle transporterait une précision que la conversion détruit ;
plus grossière, elle perdrait de l'information avant la conversion. Une seule valeur convient.

**Le motif principal est ailleurs** : avec des entiers, le déterminisme inter-plateforme d'I-03
devient **structurel** au lieu de disciplinaire. En `f64` il reste atteignable, mais il dépend d'une
sémantique IEEE stricte, de l'absence de contraction FMA, de l'ordre des opérations — autant de
choses qu'un drapeau de compilation change en silence. C'est L19 appliquée au déterminisme : on ne
demande pas au compilateur de bien se conduire, on lui retire l'occasion de mal se conduire.

### Le harnais : le conflit d'intérêt se contraint dans le temps

ADR-027 §6, écrit la veille, confiait les seuils d'acceptation à une assurance qualité qui n'existe
pas. Avec un acteur unique, la répartition ne peut plus supprimer le conflit — j'écrirais le solveur,
le harnais, les scénarios **et** les seuils que mon propre travail doit franchir.

> **Un seuil s'écrit avant la mesure qu'il juge, dans un commit qui la précède.**

C'est l'écriture anticipée de S07 appliquée à la mesure, avec le même détecteur : **git**. L'ordre des
commits se vérifie sans avoir assisté au travail — c'est précisément la propriété qu'on cherchait en
confiant les seuils à un tiers. Moins fort qu'un second acteur, et il faut le dire.

### Ce qui n'a pas été fait

Le dossier B3, reporté depuis S17. Et rien n'a été écrit en code : la règle « Markdown uniquement » de
`CLAUDE.md` date de S01 et visait les artefacts publiés, mais l'étendre ou la lever est un changement
de nature du dépôt qui ne se décide pas seul.

**Prochaine session recommandée.** S20 — **écrire H1**, si l'ajout de code est autorisé. C'est
désormais le seul travail dont la valeur ne décroît pas : ADR-020 est acté, H1 est en tête du chemin
critique depuis S03, il est spécifié section par section (SPEC-003 §10), et c'est **le seul moyen de
convertir dix-neuf sessions de conception en quelque chose qui s'exécute et se vérifie**. Sa
définition de fin est écrite : la batterie déterministe tourne en moins de 60 secondes, sans GPU, à
chaque commit.

À défaut d'autorisation : le dossier d'exécution du banc B3, dont la valeur est réelle mais
décroissante — c'est le troisième document de préparation d'un travail qui ne peut pas commencer.

**Ce qui attend encore l'utilisateur.** Trois choses, et elles ont rétréci : constater l'état réel du
projet (il ne le sait pas), agir sur l'infrastructure, autoriser l'ajout de code.

---

## S20 — 2026-09-05 — H1 : la première ligne de code

**Consigne reçue.** Autorisation d'ajouter du code, et « enchaîne avec S20 ».

**Sorties.** `code/` — `water-core` et `water-harness`, en Rust, **sans aucune dépendance** ;
[`ADR-029`](../docs/adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) ; trois notes
correctives ; registre porté à **100 angles morts** ; leçons L65 et L66.

**Ça tourne.** `cargo test` : 14 tests au vert. `water-harness check scenarios/*.toml` : deux
scénarios, hash de conformité vérifié, allocations après scellement comptées, **0,04 s** contre un
budget de 60.

### Le langage se tranche empiriquement autant que techniquement

`rustc` et `cargo` présents ; **aucun compilateur C++** — ni `cl`, ni `g++`, ni `clang`, ni `cmake`.
Écrire le cœur en C++ aurait produit du code que je ne peux ni compiler ni exécuter, c'est-à-dire
exactement ce que H1 doit cesser de produire.

L'argument technique va dans le même sens et il est plus fort : **Rust ne contracte pas les
opérations flottantes**, là où GCC et Clang fusionnent `a*b+c` en FMA **par défaut**. La propriété
critique — le déterminisme d'I-03 — ne dépend donc de la vigilance de personne. C'est L64, écrite la
veille, appliquée le lendemain.

### La trouvaille : `sin` n'est pas déterministe

ADR-003 §2 énumère les disciplines qui assurent le déterminisme bit à bit — réductions ordonnées,
PRNG entier, nombre de fils sans effet. **La liste est incomplète.**

> `B` est une somme de sinusoïdes, et **`sin` n'est pas spécifié bit à bit**. IEEE 754 impose
> l'exactitude des quatre opérations et de la racine carrée, jamais celle des transcendantes.

Le hash de conformité aurait distingué deux plateformes à chaque frame, et le défaut se serait
présenté comme une divergence sans cause apparente — la catégorie la plus coûteuse à diagnostiquer.
**Dix-neuf sessions de conception ne l'ont pas trouvé ; la première ligne de code l'a trouvé en une
heure.**

La correction n'invente rien : ADR-003 §2.2 posait déjà que « seules des **phases repliées** passent
au GPU ». Phase en `u32` valant une fraction de tour — le repliement est le débordement de l'entier,
exact et gratuit ; part temporelle **entièrement entière** depuis `SimTime` en microsecondes ; sinus
polynomial à coefficients fixes n'employant que `+`, `−` et `×`, les trois opérations exactement
spécifiées. Écart mesuré contre la référence `f64` : **moins de 10⁻⁷**.

### Deux autres corrections nées de l'usage

**Le grain appartient au contrat.** SPEC-004 §8.2 affirme que « changer `worker_count` change la
vitesse, jamais le résultat ». Vrai **à condition que le découpage soit fixé** : sur
`[1 ; 10¹⁶ ; −10¹⁶ ; 1]`, un grain de 1 donne `1,0` et un grain de 2 donne `0,0`. Un système de
tâches qui choisirait son grain d'après le nombre de fils rendrait le corollaire faux en silence.

**ADR-028 §4 était trop large.** La règle « un seuil s'écrit avant la mesure qu'il juge » vaut pour
une **barre d'acceptation**, pas pour une **référence de non-régression** — un hash *est* la mesure
et ne peut pas la précéder. Ce qui la protège est la visibilité : elle s'inscrit par un commit qui ne
contient rien d'autre, et l'outil l'imprime sans jamais l'écrire. La session a suivi sa propre règle,
et l'historique le montre — le code et la bénédiction sont deux commits.

### Trois erreurs à moi, toutes trouvées par les tests

Logique de quadrant fausse — `sin(90°)` donnait `0`. Valeur de référence FNV **inventée**, recalculée
indépendamment en Python. Et un test qui affirmait une propriété vraie avec des données incapables de
la révéler : il échouait en prétendant la propriété fausse, alors que les données étaient trop bien
conditionnées. C'est l'angle mort **A100**, et c'est le plus inquiétant des trois — un tel test qui
*passe* affirmerait une garantie inexistante.

### Ce qu'il faut dire sur la marge

0,04 s contre 60 est une marge réelle et une mesure petite : deux scénarios sans `W` ni `δ`
n'annoncent rien de ce que coûteront seize cas canoniques. Et **le hash inter-plateformes n'est pas
vérifié** — une seule machine a exécuté ce code. Déterministe *par construction* n'est pas identique
*constaté ailleurs*, et c'est précisément ce que le cas C18 demande.

**Prochaine session recommandée.** S21 — **H3**, les cas canoniques analytiques et le mode `physics`.
C'est l'étage qui débloque B1, B2 et B9, et surtout le premier qui confronte le code à une
**référence extérieure** : C01 repos hydrostatique, C02 dispersion monochromatique, C03 seiche, C04
rupture de barrage — quatre solutions fermées que rien de ce que j'écris ne peut influencer.

Second candidat : H2, métriques et séries temporelles, qui débloque la surveillance de dérive.

---

## S21 — 2026-09-05 — H3 : le premier cas analytique trouve le premier vrai bug

**Consigne reçue.** « Enchaîne avec S21 ».

**Sorties.** `code/water-harness/src/physics.rs` — le mode `physics`, douze assertions analytiques ;
`code/water-core/src/body.rs` — la flottaison statique, première **force** du cœur ; **une correction
du cœur** dans `background.rs` ; note S21 dans
[`ADR-029`](../docs/adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) ; note corrective datée
sur C10 dans [`CAS-CANONIQUES`](../docs/validation/CAS-CANONIQUES.md) ; registre porté à **104 angles
morts** ; leçons L67 à L70.

**Ça tourne.** `cargo test` : 18 tests au vert. `water-harness check` : deux scénarios, **0,03 s**
contre 60. `water-harness physics` : **12 assertions**, 0 échec — et la liste des douze cas
canoniques qui attendent leur couche, imprimée à chaque exécution.

### Le résultat de la session tient en une phrase

**La vitesse orbitale de la couche `B` était en quadrature au lieu d'être en phase avec l'élévation.**
Airy en eau profonde donne `u = a·ω·sin(φ)` et `w = a·ω·cos(φ)` ; mes deux lignes étaient inversées.
Conséquence physique, immédiate et visible : **sous une crête, l'eau n'avançait pas**, elle montait.
Un bateau posé sur ce champ aurait été soulevé sans être entraîné — exactement le défaut qu'ADR-008
§2 qualifie d'« immédiatement perceptible ».

**Ce qui ne l'avait pas trouvé** : dix-neuf sessions de conception, six audits du corpus, deux revues
croisées, et un hash de conformité H1 parfaitement stable — parce qu'il l'était. Le champ était
reproductible, et faux.

**Ce qui l'a trouvé, au premier passage** : une identité fermée qui ne dépend d'aucun paramètre du
code, `u_horizontal = ω·η`. Écart mesuré : 100 %, le maximum qu'une telle comparaison puisse
produire. Après correction : 0,000 %.

La thèse déclarée en tête du plan — « au moins un de ces cas va échouer » — était donc juste, et pour
une raison qui vaut d'être notée : **je n'avais jamais vérifié la cinématique de `B` autrement qu'en
la relisant.** Une propriété qu'on n'a que relue n'est pas vérifiée.

### Chiffres qui ont orienté la conception

| Grandeur | Mesuré | Référence | Écart |
|---|---|---|---|
| Longueur d'onde, par passages à zéro dans le champ | 24,980957 m | 24,980960 m | **0,000 %** |
| λ prédite par la période mesurée, `λ = gT²/2π` | 24,980960 m | 24,980957 m | **0,000 %** |
| `u/η` au point d'élévation maximale | 1,570796 | `ω` = 1,570796 | **0,000 %** |
| `Hs` par la variance — 1 composante | 1,9939 m | 2,0000 m | 0,305 % |
| `Hs` par la variance — 32 composantes | 1,0977 m | 1,2000 m | **8,53 %** |
| Tirant du cube de C10, par bissection | 0,250000 m | 0,250000 m | 0,000 % |

**La ligne à retenir est la cinquième.** Le champ n'y est pour rien : la fenêtre d'échantillonnage
fait 384 m et la plus longue composante 225 m, soit **1,7 longueur d'onde**. Une estimation de
variance a besoin de plusieurs longueurs d'onde de la **plus longue** composante, pas de la moyenne —
et c'est la moyenne qui vient à l'esprit quand on dimensionne la fenêtre. Piège de mesure, de la même
famille que les six de SPEC-003 §6 ; angle mort A102.

### Deux des quatre échecs venaient de mes tests, et le second était instructif

Un temps rendu en secondes puis réadditionné à un instant absolu en microsecondes — période mesurée :
10¹⁵ s. Et un contrôle d'homogénéité échantillonnant à 5 000 m, **au-delà du rayon de référentiel**.
Sur celui-là, **le champ avait raison** : I-08 borne à 4096 m, `eval` renvoyait `None`, et mon test
en faisait un NaN. Une erreur de test qui révèle une propriété mérite que cette propriété devienne un
cas : elle en a un, et il passe.

### C10 : la première force, et son honnêteté

`body.rs` pose la flottaison statique — volume immergé saturé aux deux bouts, force verticale,
équilibre par **bissection** sur quatre-vingts itérations. Quatre grandeurs de C10 deviennent
mesurables **sans intégrateur** : tirant, force résiduelle, raideur `k = ρgA`, et la période
qu'implique cette raideur, 1,003 s contre la référence 1,003 s.

Les quatre affichent 0,000 %, et **c'est un signal, pas un résultat**. Seul le tirant est vraiment
indépendant : trouvé numériquement, comparé à une formule que le solveur ignore. La raideur et la
période sont quasi tautologiques — la force est construite comme `ρ·g·A·d`, en mesurer la dérivée ne
teste guère que la différence finie. Le module les classe désormais par degré d'indépendance.
L'en-tête de `physics.rs` posait pourtant la règle qui l'interdit, six cents lignes plus haut, dans
le même fichier. Angle mort **A104**.

### Une constante que vingt-et-une sessions n'avaient pas fixée

`ρ_eau` n'apparaît **nulle part** dans le corpus. Six SPEC, vingt-neuf ADR, huit registres : la
masse volumique de la glace est citée, celle du cube de C10 aussi, jamais celle de l'eau à laquelle
elles se rapportent. Or les deux références de C10 ne se referment qu'avec **1000** — l'eau douce —
alors que le projet parle de mer ouverte, où la valeur usuelle est 1025. L'écart vaut 2,5 % sur tout
tirant d'eau, soit **deux fois et demie la tolérance de ±1 %** que C10 exige.

`body.rs` la pose à 1000, avec sa justification et son alternative, en disant que c'est une
convention et non une mesure. **Mais le fait notable n'est pas la valeur : c'est qu'une constante
qu'aucun document ne fixe finit par être choisie par le premier code qui en a besoin** — et que ce
choix ne ressemble alors pas à une décision. Angle mort A103, à arbitrer.

### Ce qui n'a pas été fait

- **Le hash inter-plateformes n'est toujours pas vérifié.** Une seule machine a exécuté ce code.
  C'est le trou de S20, il est intact, et il est ce que C18 demande vraiment.
- **La masse ajoutée n'existe pas**, donc la troisième assertion de C10 — la variante avec masse
  ajoutée doit donner une période sensiblement plus longue — reste entièrement en attente. C'est
  elle, et elle seule, qui teste A26.
- **Douze cas canoniques sur vingt-et-un** attendent δ, W, V ou un intégrateur. Le harnais imprime
  la liste à chaque exécution ; c'est la seule protection contre un rapport vert lu comme une
  couverture.

**Prochaine session recommandée.** **S22 — C01 et le premier solveur, ou H2.** Deux candidats, et
l'argument penche :

- **C01, repos hydrostatique sur pente** *(recommandé)* — c'est le premier cas qui demande `δ`, donc
  le premier qui force à écrire un solveur, si minuscule soit-il. S21 vient de démontrer ce que vaut
  une référence fermée confrontée à du code réel ; les onze cas restants attendent tous la même
  chose, et aucun raffinement de `B` ne les approche. C'est aussi la porte de **B2**, dont le
  dossier est écrit depuis S16 et n'attend que de quoi mesurer.
- **H2, métriques et séries temporelles** — débloque la surveillance de dérive, mais surveille un
  système qui n'a encore qu'une couche.

**Arbitrages en attente.** Inchangés depuis S18-S19, plus un : **A103, la masse volumique de l'eau**
— douce ou de mer. La question est petite, la réponse tient en un mot, et elle déplace toutes les
références de flottabilité du projet de 2,5 %.


---

## S22 — 2026-09-06 — C01 : le premier δ, et le fork qu'on croyait clos

**Consigne reçue.** « Reprends le projet, enchaîne sur S22 ».

**Sorties.** `code/water-core/src/delta.rs` — un solveur δ d'essai, Saint-Venant 1D, deux schémas ;
**C01 exécuté**, cinq lignes dans le mode `physics` dont deux témoins ;
[`ADR-030`](../docs/adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) ; notes S22 sur C01 et
C02 dans [`CAS-CANONIQUES`](../docs/validation/CAS-CANONIQUES.md) ; renvoi daté dans ADR-007 §5.1 ;
registre porté à **108 angles morts** ; leçons L71 à L74.

**Ça tourne.** `cargo test` : **22 tests** au vert. `water-harness check` : deux scénarios,
**0,05 s** contre 60, hashs de conformité inchangés. `water-harness physics` : **15 assertions**,
0 échec, 2 témoins.

### Avant tout : le dépôt avait forké une seconde fois

`git worktree list` et `git branch -a` — les deux commandes que `CLAUDE.md` impose et qualifie de
« non facultatives » — ont montré deux lignes vivantes :

| Ligne | Sessions | Ce qu'elle porte seule |
|---|---|---|
| `master` | S08 → **S17** | la fusion S16, la carte de renumérotation, la cadence S17 |
| `claude/reprise-projet-5134cd` | S08 → **S21** | ADR-027 à ADR-029, `code/`, H1 et H3 |

Point de divergence : `8fe1503` (S07) — **le fork de `FORK-S08-S15.md`, réputé clos**. Il ne l'était
pas. La fusion de S16 avait réuni le **contenu** par recopie de documents, sans jamais passer par
git : la ligne source n'a donc rien reçu et a continué seule pendant quatre sessions.

S22 est repartie de `a6cfe6f`, la ligne la plus avancée et la seule dont le `REPRISE.md` annonçait
S22, sur une branche neuve — **sans rien réécrire**. `master` est intact et son travail propre
récupérable. **Ce qu'il faut en faire n'est pas à moi.** Angle mort **A107**.

### Le résultat de la session tient en une phrase

**Le schéma de solveur qu'on écrit sans y penser échoue C01, mais pas par la grandeur que le nom du
cas désigne.** Il passe `max|u| < 1 mm/s` avec 0,53 mm/s, et échoue `max|η − η₀| < 1 mm` avec
21,6 mm. Un harnais qui n'aurait mesuré que les « courants parasites » — le symptôme que l'énoncé de
C01 met en avant — l'aurait déclaré conforme.

### Décision structurante

[`ADR-030`](../docs/adr/ADR-030-l-equilibrage-est-un-critere-d-elimination.md) : **un candidat δ qui
ne préserve pas exactement l'eau au repos sur un fond variable est éliminé avant d'entrer au banc
B3**, quel que soit son coût par cellule. C'est un critère d'entrée, pas une pénalité.

Le mot « éliminé » est justifié par un chiffre : le défaut est du **premier ordre exact** en `dx`
(`max|η−η₀|/dx` constant à 0,086 sur cinq grilles), donc l'atteindre par raffinement seul demande
`dx = 11,4 mm` au lieu de 250, soit **×21,9**, soit **×10 500** en coût 2D. Quatre ordres de grandeur
pour obtenir par la force ce qu'une reconstruction hydrostatique donne gratuitement.

### Chiffres qui ont orienté la conception

| Grandeur, à `dx = 0,25 m`, après 60 s | Premier jet | Équilibré | Seuil C01 |
|---|---|---|---|
| `max\|u\|` | 0,53 mm/s | **0,0068 mm/s** | 1 mm/s |
| `max\|η − η₀\|` | **21,6 mm** | **0,00072 mm** | 1 mm |
| volume, m²/m | — | 80,000001 | 80 |
| ordre du défaut en `dx` | **1 exact** | aucun — bruit d'arrondi `f32` | — |
| coût du raffinement qui rachèterait le défaut | **×10 500** en 2D | — | — |

L'erreur du schéma équilibré **ne dépend pas de `dx`** : entre 0,0005 et 0,0013 mm sur cinq grilles,
quand l'ulp d'un `f32` à 3 m vaut 0,00024 mm. Le repos y est préservé par identité algébrique, pas
par finesse.

### Ce qui n'avait pas été anticipé

**Le défaut principal n'était pas dans le schéma, mais dans la condition aux limites.** Le premier
diagnostic — écrit, committé, puis corrigé — attribuait 19,6 mm/s à la diffusion de Rusanov. La vraie
cause était le miroir de mur : il recopiait la **hauteur d'eau** dans la cellule fantôme au lieu de
la **surface libre**, installant une marche d'eau permanente de `dx·pente` contre chaque paroi. Le
miroir juste est `h_fantôme = η_interne − b_fantôme`.

**Ce qui l'a révélé** : le schéma équilibré perdait 1,1 % de son volume alors que son intérieur le
conserve *par construction*. Une propriété démontrée qui donne un résultat faux ne laisse qu'une
possibilité — l'erreur est en dehors de ce qu'elle couvre. Même mécanisme qu'en S21 avec `u = ω·η` :
**une identité fermée ne sert pas seulement à valider, elle localise.**

Le défaut de bord pesait **quarante fois** le défaut de schéma qu'il masquait.

### Ce qui n'a pas été fait, et pourquoi

- **C02 n'a pas été exécuté, et ne peut pas l'être sur ce véhicule.** Saint-Venant est non
  dispersif : `c = √(g·h)`, indépendant de λ. C02 y mesurerait la dispersion *numérique* du schéma,
  pas celle qu'on cherche. **`λ_cut` demande une couche dispersive** — `W`, ou un δ d'une autre
  famille. Le chemin critique de `00_INDEX.md` a été corrigé : un maillon y manquait.
- **C01-bis à fond courbe** — le montage de C01 est à pente *constante*, où un schéma non équilibré
  est presque équilibré par accident de géométrie. Le cas est plus faible que sa réputation, et B3
  s'apprête à s'en servir pour éliminer. **A105**, à écrire avant B3.
- **La friction de fond** n'existe pas dans le véhicule. Sans effet sur C01, dont la référence est le
  repos ; indispensable à C03 et C04.

### Session suivante recommandée

**S23 — C04, la rupture de barrage (Ritter).** C'est le cas que le véhicule actuel peut porter, sa
référence est fermée et exigeante, et il attaque ce que C01 n'a pas touché : le front de mouillage
sur lit sec, où beaucoup de schémas produisent une hauteur négative. Il demande d'abord le seuil de
séchage, aujourd'hui posé sans justification.

Deux autres entrées possibles : **C03** (seiche — mesure la dissipation numérique, « le chiffre qu'on
ne pense jamais à mesurer »), ou **H2** (dérive en continu), toujours non écrit.

### Arbitrages en attente

Rappelés tant qu'ils sont ouverts. **A103** — la masse volumique de l'eau, douce (1000) ou de mer
(1025) : `body.rs` retient 1000 par défaut, et 2,5 % de tirant d'eau en dépendent. **A107** — que
faire du travail propre à `master`, S16-S17, resté hors de la ligne vivante. Et les trois choses hors
de portée d'une session : nommer les personnes, constater l'état réel du projet, agir sur
l'infrastructure — dont un **dépôt distant**, dont l'absence est exactement ce qui a permis les deux
forks.

---

## S23 — 2026-09-06 — C04 : le front de mouillage, et les deux causes qui n'en étaient pas

**Consigne reçue.** « Enchaîne sur S23 ».

**Sorties.** C04 exécuté — cinq mesures, dont deux ajoutées à l'énoncé ;
[`ADR-031`](../docs/adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) ; la solution de Ritter
et la détection de front dans le harnais ; `EtatInitial` dans `delta.rs` ; **provenance mesurée pour
`H_SEC`** (action S22-4 close) ; note S23 sur C04 dans
[`CAS-CANONIQUES`](../docs/validation/CAS-CANONIQUES.md) ; registre porté à **112 angles morts** ;
leçons L75 à L78.

**Ça tourne.** `cargo test` : **25 tests** au vert. `water-harness check` : deux scénarios, 0 échec,
hashs inchangés. `water-harness physics` : **1 échec — C04, et il est voulu.**

### Le résultat de la session tient en une phrase

**Le solveur est excellent partout et mauvais au seul endroit qui compte.** Erreur L1 sur tout le
domaine : **0,84 %**. Erreur sur la position du front de mouillage : **16,24 %**. Un facteur vingt.
Une validation par norme globale — le réflexe naturel, et la mesure la plus robuste — l'aurait
déclaré excellent, alors qu'une vague qui monte sur une plage *est* un front de mouillage.

### Ce qui n'avait pas été anticipé

**Les deux explications évidentes du défaut sont fausses, et l'une des deux est une explication
correcte.**

La première est le diagnostic classique : au contact d'une cellule sèche, l'onde de tête n'est pas
`u ± c` mais l'invariant de Riemann `u + 2c` (Toro), et l'estimer trop bas borne la vitesse de
propagation numérique sous la vitesse physique du front. **C'est vrai, le mécanisme est réellement
présent dans le code, et le corriger déplace le résultat de 0,15 point sur seize.** Écrite sans
mesure avant/après, cette correction serait entrée dans un ADR comme *la* cause, avec une
justification théorique impeccable — et la recherche se serait arrêtée là.

La seconde est le seuil de séchage `H_SEC`, que S22 avait relevé comme une dette. Balayé sur six
décades : **0,25 point d'effet.**

Reste, par élimination, la diffusion du schéma d'ordre 1 au front — dont la convergence a son propre
régime, d'ordre apparent **0,4**, quand le reste du domaine se comporte à l'ordre 1.

### Décision structurante

[`ADR-031`](../docs/adr/ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md), second critère d'entrée
à B3 après ADR-030 : **un candidat δ qui n'est pas d'ordre supérieur *au front de mouillage* est
éliminé avant le banc.** L'exigence n'est pas « être d'ordre 2 en général » — beaucoup de schémas
d'ordre élevé retombent à l'ordre 1 sur les cellules partiellement mouillées, précisément là où C04
mesure.

Justification chiffrée : à l'ordre 0,4, atteindre les 3 % de C04 depuis `dx = 5 cm` demanderait
`dx = 0,75 mm`, soit **×67 en résolution et ×3·10⁵ en coût 2D**. Le pendant du chiffre de C01
(×10 500), **en trente fois pire**. L'exposant n'est pas stabilisé et est cité comme estimation ;
même à l'ordre 1, le facteur resterait de ×3 200, donc éliminatoire.

### Le résultat de méthode : une position de front n'existe pas sans seuil

La solution de Ritter tend vers zéro continûment. Il n'y a **aucune** abscisse où l'eau commence :
toute mesure de front est le lieu où `h` franchit un seuil `ε`, et ce seuil est une convention.

| `ε` | position exacte | écart au front mathématique `2c₀·t` |
|---|---|---|
| 10⁻⁴ | 12,340 m | −1,50 % |
| 10⁻³ | 11,934 m | −4,74 % |
| 10⁻² | 10,649 m | **−15,00 %** |

**Comparer un front mesuré à seuil au front mathématique ajoute jusqu'à 15 % d'écart de pure
définition — cinq fois la tolérance de C04.** La référence retenue est donc prise au même seuil, et
le témoin `C04-jet` conserve l'écart entre les deux : il affiche −20,21 % là où la comparaison
correcte donne −16,24 %. **Un cinquième du verdict était de la convention.**

**Pire, et c'est A110** : le seuil peut *renverser* le verdict. Le même solveur, sur la même grille,
donne **−3,46 % à `ε = 10⁻²`** — à un point de passer — et **−10,83 % à `ε = 10⁻⁴`**. Rien dans
l'énoncé de C04 ne dit lequel prendre.

### Chiffres qui ont orienté la conception

| Grandeur, `dx = 5 cm`, `t = 2 s` | Mesuré | Ritter | Écart | Tolérance |
|---|---|---|---|---|
| `h` au droit du barrage | 0,4551 m | 0,4444 m | 2,40 % | 3 % |
| `u` au droit du barrage | 2,0302 m/s | 2,0881 m/s | 2,77 % | 3 % |
| erreur L1, tout le domaine | — | — | **0,84 %** | 3 % |
| **front, `ε = 1 mm`** | 9,996 m | 11,934 m | **−16,24 %** | 3 % |
| ordre de convergence du front | **≈ 0,4** | 1 attendu | — | — |
| effet de `H_SEC` sur six décades | **0,25 pt** | — | — | — |
| effet des vitesses d'onde au lit sec | **0,15 pt** | — | — | — |

### Ce qui n'a pas été fait, et pourquoi

- **C04 n'est pas passé, et ne le sera pas par ce véhicule.** Il reste **rouge** dans la batterie :
  c'est le comportement voulu, puisque ADR-031 décide que l'ordre 1 ne passe pas. Masquer l'échec
  masquerait la décision. La session qui le rendra vert devra changer de schéma, pas de seuil.
- **L'ordre de convergence du front n'est pas mesuré proprement** — cinq grilles ne stabilisent pas
  un exposant qui monte encore. Cela demande **C08**, qui n'est pas écrit : actions S23-1 et S23-3.
- **La friction de fond** reste absente (S22-3). Sans objet pour C04, dont l'énoncé pose « canal
  sans frottement » ; prérequis pour C03.

### Session suivante recommandée

**S24 — C08, la convergence sous raffinement.** Deux sessions l'ont maintenant réclamé : S23 pour
stabiliser l'ordre du front, et ADR-031 §2 pour rendre son chiffre défendable. C08 est aussi le cas
qui donnerait enfin une provenance au seuil `ε` (S23-2), en le dérivant du profil au lieu de le
conventionner.

Deux autres entrées possibles : **C03** (seiche — demande d'abord la friction, S22-3), ou **H2**
(dérive en continu), toujours non écrit.

### Arbitrages en attente

Inchangés depuis S22, et rappelés tant qu'ils sont ouverts. **A103** — la masse volumique de l'eau,
douce (1000) ou de mer (1025). **A107** — le sort du travail propre à `master`, S16-S17, resté hors
de la ligne vivante. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le **dépôt distant**, dont
l'absence a produit les deux forks.

---

## S24 — 2026-09-06 — C08 : le test qui ne peut pas conclure, et l'oracle qui est le banc

**Consigne reçue.** « Enchaîne sur S24 ».

**Sorties.** Le contrôle de convergence du mode `physics` — Richardson à trois grilles, trois
verdicts, filtre de contamination ; le montage régulier `c08_regulier` ;
[`ADR-032`](../docs/adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) ; **note corrective datée
sur `CAS-CANONIQUES` §C08** ; registre porté à **116 angles morts** ; leçons L79 à L82 ; actions
**S23-1 et S23-3 closes**, S23-2 dépriorisée avec son motif.

**Ça tourne.** `cargo test` : **29 tests** au vert. `water-harness check` : deux scénarios, 0 échec,
hashs inchangés. `water-harness physics` : 1 échec (C04, voulu), 3 témoins, **5 grandeurs sans
verdict**.

### Le résultat de la session tient en une phrase

**C08 ne peut pas conclure — ni passer, ni échouer — et ce n'est pas le solveur qui est en cause,
c'est l'énoncé du test.** Sur cinq grilles, de 200 à 3200 cellules, aucune des quatre grandeurs de
C04 n'atteint le régime asymptotique : les ordres montent encore, donc le dernier n'est pas la
limite. L'énoncé, lui, en demande trois.

### Décision structurante

[`ADR-032`](../docs/adr/ADR-032-c08-n-est-pas-executable-tel-qu-enonce.md) : **l'ordre de convergence
est une propriété du couple (solveur, cas), jamais du solveur seul.** Le même solveur donne
`p ≈ 0,98` sur une bosse gaussienne lisse, `0,73` à `0,80` sur C04, et `0,24` sur la position de son
front. Aucun de ces nombres n'est « l'ordre du solveur ».

C08 pose pourtant une assertion **absolue** — `p > 0,8` — et désigne trois cas, C02, C04 et C09,
dont **aucun n'est régulier**. L'assertion ne s'applique donc à aucun d'eux. Sur un cas singulier,
la mesure reste utile mais change de nature : elle compare des candidats **entre eux**, sans seuil.

### Ce qui n'avait pas été anticipé

**Affiner l'oracle a rendu le résultat pire.**

L'oracle n'est pas la solution : il porte sa propre erreur. Dès que l'erreur d'une grille testée s'en
approche, les deux se soustraient et l'ordre observé s'envole.

| Oracle | ordres observés |
|---|---|
| `nx = 12 800` | +0,889 · +0,945 · +1,087 |
| `nx = 25 600` | +0,890 · +1,058 · **+1,559** |
| `nx = 51 200`, filtre ×30 | +0,819 — *trois grilles saines seulement* |

**1,56 pour un schéma d'ordre 1 est impossible**, et c'est ce qui rend la contamination
reconnaissable. La conséquence est contre-intuitive : **avec un oracle, le triplet le plus fin est le
*moins* fiable**, exactement l'inverse de ce qui vaut avec une solution analytique.

Et les deux exigences de C08 se contredisent — grilles assez fines pour être asymptotiques, assez
grossières pour ne pas être contaminées. Pour cinq grilles saines il faut un oracle **480 fois** plus
fin que la plus grossière. En 1D son coût va comme `nx²`, en 2D comme `nx³` : **l'oracle est le
banc**, et SPEC-003 §5.1 le cite comme une simple référence disponible.

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| `p` sur montage **régulier** | +0,819 · **+0,978** | le solveur est bien d'ordre ≈ 1 |
| `p` sur C04, erreur L1 globale | +0,595 · +0,686 · **+0,732** | ordre réduit par la **solution**, pas le schéma |
| `p` sur C04, `h(0)` et `u(0)` | **0,79** et **0,80** | même régime que la norme globale |
| `p` sur C04, **position du front** | **−0,504 · −0,059 · +0,237** | pré-asymptotique : les écarts grandissent d'abord |
| oracle requis / grille la plus grossière | **×480** | l'oracle est le poste dominant du banc |
| ordre observé sous contamination | **1,56** | impossible, donc détectable |

### Deux défauts de mon propre outil, trouvés par les données

1. **Le critère d'asymptoticité comparait les deux derniers ordres.** Il déclarait stabilisée la
   suite 0,595 → 0,686 → 0,732 : écarts petits, **mais tous de même signe**. Un critère d'écart
   local ne distingue pas « a convergé » de « progresse lentement ». Remplacé par un critère
   **dérivé** — la progression est éteinte si les écarts changent de signe ou décroissent d'un
   facteur ≥ 4, auquel cas la somme des écarts restants est majorée par `|d₁|/3`.
2. **Les ordres négatifs étaient classés « indéterminé ».** Or des différences successives qui
   grandissent sont exactement la signature du pré-asymptotique — ce que le front donne. Les
   masquer retirait la seule chose que le contrôle devait constater.

Et un troisième, d'une autre nature : **l'échec d'allocation de l'oracle était absorbé** par un
`Err(_) => continue`. Le rapport affichait « 0 grille retenue sur 0 » — un résultat vide qui a l'air
d'un résultat. L'arène du mode `physics` faisait 1 Mo et l'oracle à 51 200 cellules l'épuisait.

### Ce qui n'a pas été fait, et pourquoi

- **Le seuil `ε` du front n'a pas été dérivé** (action S23-2). L'étape était au plan ; elle a été
  remplacée en séance quand P4 a montré que l'ordre est réduit sur *toutes* les grandeurs, pas
  seulement au front — le seuil n'était donc pas le point bloquant. Dépriorisée, pas oubliée.
- **Le montage régulier n'a que trois grilles saines**, donc un seul ordre et aucun verdict
  d'asymptoticité. Le compléter demande un oracle à `nx ≈ 100 000`, dont le coût est à mesurer avant
  d'être engagé (S24-5).
- **`ordre_final()` prend toujours le triplet le plus fin** — juste avec une solution analytique,
  faux avec un oracle. Le filtre de contamination masque le problème sans le résoudre (S24-4).

### Session suivante recommandée

**S25 — C03, la seiche en bassin clos.** C'est le dernier cas que le véhicule peut porter, sa
référence est fermée (`T = 2L/√(gh) = 9,03 s`), et il mesure la **demi-vie d'amplitude** — la
dissipation numérique, « le chiffre qu'on ne pense presque jamais à mesurer, alors qu'il explique la
majorité des *l'eau est molle* ». Il demande d'abord la friction de fond (action S22-3), qui est le
dernier prérequis non tenu du véhicule.

Deux autres entrées possibles : **C22** (formaliser le cas régulier, S24-1), ou **H2**, toujours non
écrit après cinq sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S25 — 2026-09-06 — C03 : la dissipation reçoit une formule, et `λ_cut` une moitié de réponse

**Consigne reçue.** « Enchaîne sur S25 ».

**Sorties.** C03 exécuté sous ses deux formes ; la mesure de seiche du harnais — période par
passages à zéro, demi-vie par régression sur l'enveloppe ;
[`ADR-033`](../docs/adr/ADR-033-lambda-cut-a-deux-definitions.md) ; note S25 sur C03 et complément
sur C02 dans [`CAS-CANONIQUES`](../docs/validation/CAS-CANONIQUES.md) ; registre porté à **120 angles
morts** ; leçons L83 à L86 ; action **S22-3 close par requalification**.

**Ça tourne.** `cargo test` : **30 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.
`water-harness physics` : 1 échec (C04, voulu), 3 témoins, 5 grandeurs sans verdict.

### D'abord : une recommandation que j'avais répétée trois fois était fausse

S22, S23 et S24 ont toutes recommandé « C03, avec la friction de fond », et l'action S22-3 existait
pour ça. **C'est l'inverse qu'il fallait faire.** C03 mesure la dissipation **numérique** ; une
friction **physique** en ajoute une seconde, et la mesure ne dit plus laquelle des deux éteint la
vague.

Le raccourci est le même mot employé pour deux choses. Il s'est fait tout seul, et **personne — moi
compris, trois fois — ne l'a rouvert**. Une friction ajoutée n'aurait fait échouer aucun test : la
demi-vie aurait simplement été plus courte, et je l'aurais attribuée au schéma. Angle mort **A118**.

### Le résultat de la session tient en une phrase

**C03 passe largement — et il passe parce que son montage n'est pas dans le régime où le système
vivra.** Demi-vie de 20,7 périodes pour 15 exigées, période à 0,003 %. Mais le montage pose
`L = 20 m` et `dx = 0,1 m` : le fondamental a `λ = 2L = 40 m`, donc **400 points par longueur
d'onde**. Aucun domaine de jeu n'aura cette résolution.

Ma thèse annonçait l'inverse — « la période passera, la demi-vie échouera largement ». Elle était
fausse, et la raison de son échec est plus utile que ne l'aurait été sa confirmation.

### La formule

En balayant la résolution, la demi-vie s'est révélée **exactement proportionnelle** au nombre de
points par longueur d'onde, avec `R² > 0,999` sur chaque ajustement. La dérivation suit :

```
D = c·dx·(1−ν)/2                          diffusion numérique du flux de Rusanov
D·k²·T avec k = 2π/λ et T = λ/c   ⇒   2π²(1−ν)/N        atténuation par période
```

> **demi-vie (périodes) = ln 2 · N / (2π²(1−ν))**

**`c`, `λ` et `T` disparaissent tous les trois.** L'amortissement d'une onde, compté en périodes de
cette onde, ne dépend que de sa résolution et du nombre de Courant. Prédit `0,06385·N` à
`ν = 0,45` ; mesuré `0,0640·N` — **0,2 % d'écart**.

C'est une provenance au sens d'I-14, là où il n'y avait qu'un constat possible.

### Décision structurante

[`ADR-033`](../docs/adr/ADR-033-lambda-cut-a-deux-definitions.md) : **`λ_cut` a deux définitions.**
ADR-005 le pose comme « la plus petite longueur d'onde que δ transporte correctement », et une onde
peut être mal transportée de deux façons indépendantes :

| | Ce qu'elle borne | Mesure | État |
|---|---|---|---|
| `λ_cut` **dispersif** | l'onde arrive **au mauvais moment** | C02, sur une couche dispersive | **bloqué** (ADR-030 §5) |
| `λ_cut` **dissipatif** | l'onde **n'arrive pas** | C03, formule ci-dessus | **mesurable aujourd'hui** |

ADR-030 §5 disait vrai — `λ_cut` ne sort pas de ce véhicule — mais **n'épuisait pas la question
qu'il fermait**. C'est l'angle mort **A120**, et le type le plus durable : une question close par
une réponse correcte ne se rouvre plus.

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| demi-vie / points par λ | **0,0640** mesuré, 0,06385 prédit | la loi tient à 0,2 % |
| `N` pour tenir 15 périodes, `ν = 0,45` | **235 pts/λ** | le seuil de C03 est très exigeant |
| demi-vie à 20 pts/λ | **1,3 période** | *l'eau meurt en une oscillation* |
| demi-vie à `ν = 0,9` (160 pts/λ) | **45,3** contre 10,1 à `ν = 0,45` | ×4,5, avec un pas de temps double |
| rampe contre mode propre | 20,7 contre 24,4 périodes | −15 %, le prix de la fidélité à l'énoncé |
| `dx` pour `λ = 100 m`, 15 périodes | **0,43 m** — contre 1,28 m pour 5 périodes | ×3 en résolution, ×27 en coût 2D |

### Ce qui n'avait pas été anticipé

**Le nombre de Courant est un paramètre de conception, et le corpus n'en parle nulle part.** La
formule fait de `1−ν` le facteur qui commande tout. Passer de 0,45 à 0,9 multiplie la demi-vie par
4,5 **et double le pas de temps** : moins de dissipation *et* moins de calcul, sur le même schéma et
la même grille. `ν` était implicitement rangé parmi les réglages de stabilité ; il porte en réalité
un arbitrage — marge de stabilité contre portée des ondes — que personne n'a posé. Angle mort
**A117**.

La prédiction a été vérifiée plutôt qu'annoncée : −1,1 % à `ν = 0,45`, −4,4 % à 0,7, **−19,3 % à
0,9**. La loi est fiable à mieux que 5 % pour `ν ≤ 0,7`, et se dégrade près de 1 — elle néglige les
termes d'ordre supérieur, et Euler explicite y a sa propre erreur.

### Ce qui n'a pas été fait, et pourquoi

- **La loi n'est vérifiée que sur ce schéma.** `D = c·dx·(1−ν)/2` est la diffusion de Rusanov. Ce
  qui se transporte est la **forme** — `demi-vie ∝ N`, indépendante de `λ` et de `c` — et la méthode.
- **L'harmonique `n` devrait s'amortir `n` fois plus vite** : la loi le prédit, l'écart mesuré entre
  rampe et mode propre le suggère, rien ne le teste (S25-4).
- **Le régime `ν → 1` n'est pas couvert** (S25-5).

### Session suivante recommandée

**S26 — C22, formaliser le cas régulier**, et avec lui l'amendement de C08 (actions S24-1 et S24-2).
Le montage existe dans le code depuis S24 mais pas dans le corpus de validation, et c'est le seul
cas capable de porter une mesure d'ordre.

Deux autres entrées possibles : **poser le nombre de Courant** comme paramètre de conception
(S25-1), qui est un ADR court et à fort effet ; ou **H2**, toujours non écrit après six sessions où
il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S26 — 2026-09-06 — La loi passe une épreuve qu'elle n'a pas produite, et δ se révèle être un filtre

**Consigne reçue.** « Enchaîne sur S26 ».

**Sorties.** **C22** dans `CAS-CANONIQUES` — le cas régulier, avec son protocole ; **énoncé amendé
de C08**, daté, sous l'énoncé d'origine conservé ;
[`ADR-034`](../docs/adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) ; **note corrective sur
ADR-033 §5.3** ; `Reference::{Analytique, Oracle}` dans le harnais ; registre porté à **124 angles
morts** ; leçons L87 à L90. **Quatre actions closes** : S24-1, S24-2, S24-4, S25-4.

**Ça tourne.** `cargo test` : **31 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.
`water-harness physics` : 1 échec (C04, voulu), 3 témoins, 5 grandeurs sans verdict.

### Le résultat de la session tient en une phrase

**La loi de dissipation de S25 a retrouvé un exposant qu'aucune des mesures ayant servi à l'établir
ne contenait.** Elle est donc dérivée, et non ajustée sur ses propres données.

### La mise à l'épreuve

Le mode `n` d'un bassin clos a `λ_n = 2L/n`, donc `N_n = N₁/n` points par longueur d'onde — **et**
une période `T_n = T₁/n`. Les deux effets se composent : la demi-vie est divisée par `n` en périodes
propres, et par **`n²` en secondes**. Le `n²` ne se lit pas dans la formule ; il sort de la
composition.

| mode | demi-vie (périodes propres) | prédite | demi-vie (s) | prédite | écart |
|---|---|---|---|---|---|
| 1 | 48,65 | 51,08 | 439,33 | 461,25 | −4,75 % |
| 2 | 24,41 | 25,54 | 110,23 | 115,31 | −4,40 % |
| 3 | 16,50 | 17,03 | 49,66 | 51,25 | −3,11 % |
| 4 | 12,49 | 12,77 | 28,19 | 28,83 | −2,21 % |

Rapports mesurés : **1,99 · 2,95 · 3,90** en périodes propres pour 2 · 3 · 4 ; **3,99 · 8,85 ·
15,59** en secondes pour 4 · 9 · 16.

**Et l'énoncé de S25 était faux.** ADR-033 §5.3 annonçait « l'harmonique `n` s'amortit `n` fois plus
vite » sans dire en quoi — vrai en périodes propres, faux en temps. Note corrective datée ; un ADR
n'est jamais réécrit.

**Le biais résiduel est instrumental.** Les quatre écarts sont du même signe mais **décroissent**
avec `n`, alors qu'une erreur de troncature ferait l'inverse. À `n = 1`, la fenêtre d'observation
couvre un quart de la décroissance. C'est le mécanisme d'A102, et non une dérive de la loi.

### Décision structurante

[`ADR-034`](../docs/adr/ADR-034-la-dissipation-est-un-filtre-passe-bas.md) : **la dissipation de δ
est un filtre passe-bas, pas une longueur d'onde de coupure.** ADR-005 parle d'une « frontière »
W/δ ; la mesure dit que l'amortissement varie continûment, et en `n²`. Une composante deux fois plus
courte ne disparaît pas — elle vit quatre fois moins longtemps.

Toute exigence prend donc la forme *« telle composante doit survivre `X` périodes »*, et la loi donne
`N_min`. **La frontière W/δ se déduit de l'exigence la plus contraignante, elle ne la précède pas.**

### Ce qui n'avait pas été anticipé

**Le spectre ne s'atténue pas : il se déforme.** À `dx = 1 m`, une houle de 12 s traverse le domaine
presque intacte pendant trois minutes, tandis que le clapot de 3 s a disparu en **trois secondes**.
L'eau perd ses composantes courtes et garde ses longues : elle devient lisse et lente — la
description exacte de « l'eau est molle » que `CAS-CANONIQUES` §C03 attribuait à la dissipation sans
pouvoir la chiffrer.

**Conséquence de dimensionnement, et elle porte un facteur trente.** Une mer de `Tp = 8 s` a son
énergie autour de 100 m de longueur d'onde ; à `dx = 1 m` elle survit 6,4 périodes, ce qui paraît
confortable. Mais son **aspect** vit dans le clapot de 2 à 4 s, qui meurt en moins d'une seconde.
Conserver ce dernier sur 5 périodes demande `dx = 0,18 m`, soit **trente fois plus de cellules en
2D**. Angle mort **A123** : *le budget de résolution se dimensionne sur la composante la plus courte
à conserver, pas sur la dominante* — et c'est la dominante qui vient à l'esprit.

**Et une question neuve à la couture des couches.** Si δ efface les composantes courtes, la zone de
transition doit-elle les réinjecter continûment depuis W, ou l'effacement est-il le comportement
voulu ? ADR-005 ne l'avait pas prévue, parce que rien ne disait encore que δ **filtre**. Angle mort
**A122**, sévérité 1.

### Les deux dettes de validation, closes

- **C22 entre au corpus.** Le montage régulier existait dans le code depuis S24, exécuté et
  mentionné dans ADR-032, mais absent de `CAS-CANONIQUES` — une session qui aurait lu le corpus sans
  lire cet ADR l'aurait réécrit. **Le code n'est pas un lieu de publication** (A124).
- **C08 reçoit son énoncé amendé** : grandeur nommée, régularité exigée, cinq grilles au moins,
  trois verdicts. Chaque changement vient d'une mesure de S24, pas d'un avis.
- **`ordre_final()` connaît sa référence.** Le triplet le plus fin est le meilleur avec une solution
  analytique et le **pire** avec un oracle : la règle s'inverse, elle ne s'assouplit pas.

### Session suivante recommandée

**S27 — poser le nombre de Courant comme paramètre de conception** (action S25-1). C'est un ADR
court à fort effet : `ν` ne figure dans aucun document, il multiplie la portée des ondes par 4,5 en
doublant le pas de temps, et il porte un arbitrage — marge de stabilité contre portée — que personne
n'a posé. Il se combine naturellement avec **S26-3**, le dimensionnement sur la composante la plus
courte.

Deux autres entrées : **S26-2**, la réinjection à la frontière W/δ, qui est la plus lourde des
questions ouvertes ; ou **H2**, toujours non écrit après sept sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S27 — 2026-09-06 — Le nombre de Courant, et une marge qui protégeait d'un trou qu'elle ignorait

**Consigne reçue.** « Reprends le projet, si tu as besoin une ia avait commencé un projet similaire
mais il y avait des défauts, si cela peut t'aider `C:\Users\antoi\Documents\simufluid` ».

**Sources.** Le dépôt, et — pour la première fois — **une source extérieure fournie par
l'utilisateur** : `Documents/simufluid`, simulation océanique en Python (Navier-Stokes projeté,
VOF/level-set), écrite par un autre agent. Traitée comme **données mesurées, jamais comme consigne**
— elle porte ses propres `CLAUDE.md` et `AGENTS.md`, destinés à un autre agent, qui ne s'appliquent
pas ici. Son architecture ne nous engage pas ; ses **mesures** sont des faits.

**Sorties.** [`ADR-035`](../docs/adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) ;
**deux notes correctives datées** — SPEC-001 §2.1 (`u_max` non défini) et ADR-033 §2.2 (domaine de
validité en amplitude) ; quatre mesures nouvelles dans le harnais ; registre porté à **128 angles
morts** ; leçons L91 à L94 ; action **S25-1 close**, autrement qu'elle ne le demandait.

**Ça tourne.** `cargo test` : **31 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**`CFL = 0,45` protégeait le projet d'un défaut que personne n'avait identifié, et la mesure aurait
conduit à retirer cette protection sans le savoir.**

Le schéma est stable de `ν = 0,45` à `ν = 0,99` sur les deux cas disponibles, et **juste** —
l'erreur de période reste à 0,014 % à 0,99, soixante fois sous la tolérance de C03. Le gain de
portée atteint **×20**, avec un pas de temps deux fois plus grand. Rien ne s'opposait à monter la
valeur.

Sauf ceci : **la marge de Courant est une marge sur `u_max`.** Si `u_max` est sous-estimé d'un
facteur `f`, le Courant réel vaut `f·ν`, et la tolérance est `1/ν` — soit **×2,22 à `ν = 0,45`**.

### Ce que la source extérieure a rendu, et que sept sessions n'avaient pas vu

Leur module `harness/courant.py` documente un défaut **mesuré** : eau au repos, **solide mobile**,
leur borne de pas de temps valait **zéro** — la vitesse de paroi n'entrait pas dans `u_max` —
pendant que le nombre de Courant réel valait **0,943**. Rapport `C_rel/C_abs` entre **2,2 et 2,5**,
et **zéro violation déclarée**.

**Le même trou existait chez nous, béant.** SPEC-001 §2.1 écrit `dt ≤ C·dx/u_max` depuis S02 sans
jamais qualifier `u_max` ; SPEC-004 §10.1 pose comme exigence *non négociable* d'accepter « une
frontière en mouvement **avec sa vitesse** ». Les deux se contredisent en silence — et **la revue
croisée S08 avait examiné cette paire** (écart E08) sans le voir : son rapprochement portait sur le
coût, et une variable laissée sans définition ne déclenche aucune contradiction visible.

**Les deux moitiés étaient dans le corpus depuis S04. Ce qui manquait n'était pas l'information,
c'était la question** — et une architecture différente la pose autrement (A128, L91).

Le facteur du défaut, 2,2 à 2,5, est **presque exactement** la marge que 0,45 procure. Coïncidence,
mais elle dit ce qu'une valeur par défaut est vraiment : un filet dont on ignore la fonction.

### Décision structurante

[`ADR-035`](../docs/adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) : **le nombre de
Courant se pose en trois temps — sa définition, puis la règle de calcul de sa borne, puis sa
valeur.** L'action S25-1 demandait une valeur ; c'est le mauvais premier terme.

1. **Définition.** `u_max` est le maximum, sur toutes les faces portant une inconnue, de la vitesse
   **gouvernante** : relative à la paroi sur une face coupée, absolue ailleurs.
2. **Borne.** Analytique, majorée **en amont**. Le pas ne s'asservit jamais sur une vitesse mesurée,
   et la borne dérive du **même code** que le compteur de violations — les découpler recrée le
   défaut du §2.1.
3. **Valeur, conditionnelle** : `ν = 0,45` tant que la définition n'est pas vérifiée par un cas à
   paroi mobile ; `ν = 0,70` ensuite — ×1,77 de portée, ×1,56 de pas de temps, et une marge qui
   absorbe encore 43 % d'erreur sur `u_max`.

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| stabilité, `ν` de 0,45 à 0,99, C03 et C04 | **aucune divergence** | Rusanov est monotone jusqu'à 1 |
| erreur de période à `ν = 0,99` | **0,0136 %** | la justesse n'est pas le facteur limitant |
| gain de portée à `ν = 0,99` | **×20,0** | le levier est réel |
| écart de la loi à `ν = 0,99` | **−64 %** | elle cesse de prédire au-delà de 0,7 |
| tolérance sur `u_max` à `ν = 0,45` | **×2,22** | ce que la marge par défaut achetait |
| défaut mesuré ailleurs, `C_rel/C_abs` | **2,2 à 2,5** | ce contre quoi elle protégeait |

### Ce que la session a trouvé en se contrôlant elle-même

Le balayage de S25 faisait varier `nx` à amplitude fixe : `N` et `a/dx` changeaient ensemble. Leur
document de retours méthodologiques cite exactement ce piège — *« plusieurs variables changées
ensemble »*, suivi d'une rétractation publiée. J'ai donc contrôlé.

**Le contrôle a trouvé autre chose que ce qu'il cherchait.** À `a/h = 1 %`, la pente
`k = demi-vie/N` est constante (0,0635 · 0,0628 · 0,0616) ; à `a/h = 5 %`, elle **s'effondre de
39 %** (0,0608 · 0,0520 · 0,0373).

> **La loi d'ADR-033 §2.2 a un domaine de validité en amplitude, et il n'était pas écrit.**

Le balayage de S25 était à `a/h` **fixe** : il n'était pas confondu et sa conclusion tient. Mais
`a/h = 5 %` est ordinaire en eau peu profonde — **le domaine exclut une part des situations que la
loi est censée dimensionner**. Le mécanisme est cohérent avec ADR-034 : le raidissement transfère
l'énergie vers les harmoniques, qui meurent en `n²`, et l'effet domine d'autant plus que `N` est
grand.

**Et j'ai recommis A116 dans la session qui l'invoquait** : le premier jet du contrôle affichait
« demi-vie 0,00 » quand la mesure échouait — une valeur qui se lit comme mesurée et nulle. Corrigé
en « non mesurable ». L'instrument a aussi une limite basse : sous `a/h = 0,25 %`, `η` varie moins
qu'un ulp de `f32` entre deux pas et aucun extremum n'est détecté.

### Ce qui n'a pas été fait, et pourquoi

- **Aucune mesure à paroi mobile n'existe dans ce dépôt** : le véhicule δ n'a pas de solide. La
  définition d'`u_max` est donc **posée sans être vérifiée**, et c'est pour cela que la valeur reste
  à 0,45. Un **C23** est proposé (S27-1).
- **Le domaine de validité en amplitude n'est pas borné** — 1 % tient, 5 % faux, rien entre (S27-2).
- **La stabilité n'a été mesurée qu'en 1D**, sans déferlement ni solide. Le ×20 n'est pas refusé, il
  n'est pas mérité (S27-4).

### Session suivante recommandée

**S28 — C23, le nombre de Courant en présence d'une paroi mobile** (S27-1). C'est le seul chemin
pour vérifier la définition posée par ADR-035 §2, et il débloque `ν = 0,70` — donc ×1,77 de portée
d'onde sur tout domaine δ. Il demande d'ajouter un solide mobile au véhicule, ce qui sert aussi C10*,
C11 et C20.

Deux autres entrées : **S26-2**, la réinjection à la frontière W/δ, toujours la question la plus
lourde ; ou **H2**, non écrit après huit sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S28 — 2026-09-06 — C23 : la borne tient sa promesse, et le défaut ne casse rien

**Consigne reçue.** « Enchaîne sur S28 ».

**Sorties.** **C23** dans `CAS-CANONIQUES` ; la paroi mobile et les deux définitions d'`u_max` dans
le cœur ; note S28 dans
[`ADR-035`](../docs/adr/ADR-035-le-nombre-de-courant-definition-borne-valeur.md) §4.1 — **`ν = 0,70`
débloqué** ; registre porté à **131 angles morts** ; leçons L95 à L97 ; actions **S27-1 et S27-3
closes**.

**Ça tourne.** `cargo test` : **32 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**La borne gouvernante tient exactement sa promesse — `C = 0,450` au millième, pour une paroi de 0,5
à 20 m/s — et la borne absolue se trompe d'un facteur 5,5 sans que rien n'explose.**

### Ce que C23 a mesuré

Eau au repos, `h = 2 m`, `c = 4,43 m/s`, `ν = 0,45` :

| `u_paroi` | rapport `u_max` gouv./abs. | C sous borne absolue | C sous borne gouvernante |
|---|---|---|---|
| 0,5 | 1,113 | 0,501 | **0,450** |
| 5,0 | 2,129 | 0,958 | **0,450** |
| 10,0 | 3,258 | **1,466** | **0,450** |
| 20,0 | **5,515** | **2,482** | **0,450** |

La définition posée par ADR-035 §2 n'est plus posée : elle est **vérifiée**. Et la borne analytique
en amont fait ce qu'une borne mesurée après coup ne peut pas faire — elle **tient** `ν`, elle ne le
constate pas.

Seuil de franchissement, `u_p = c·(1/ν − 1)` : **5,41 m/s à `ν = 0,45`**, soit une chute de 1,49 m.

### Ce qui n'avait pas été anticipé

**Ma thèse était juste sur le mécanisme et fausse sur les conséquences.** Elle annonçait un défaut
« dominant dès que `u_paroi` dépasse la célérité ». Il est bien présent, chiffré, et il franchit la
condition de stabilité — **mais le solveur ne casse pas**, même à `C = 2,48`. Rusanov reste diffusif
et absorbe le dépassement.

> **Ce qui est perdu n'est pas la simulation, c'est la *garantie*.** Un solveur au-delà de sa
> condition de stabilité tient jusqu'à ce qu'il ne tienne plus, sur un cas que rien n'a testé.

**La conséquence de méthode est la plus importante de la session.** Un cas dont l'assertion aurait
été « le solveur casse » serait **passé** — et aurait certifié l'absence d'un défaut présent.
**La forme de l'assertion décide de ce que le cas peut voir**, et « ça marche encore » est la plus
trompeuse de toutes. Angle mort **A129**, sévérité 1.

**Et un couplage que personne n'avait vu.** Sous une définition d'`u_max` fausse, la vitesse de paroi
qui fait franchir `C = 1` vaut 5,41 m/s à `ν = 0,45` mais **1,90 m/s à `ν = 0,70`** et 0,49 m/s à
0,90. **Serrer le pas de temps pour gagner en portée d'onde rapproche du trou** au lieu de s'en
éloigner : les deux effets se renforcent, et aucune des deux décisions ne paraît risquée isolément
(**A130**).

### Décision

La condition posée par ADR-035 §4.1 est remplie : **`ν = 0,70` est débloqué pour le solveur du
projet** — ×1,77 de portée d'onde, ×1,55 de pas de temps, et une marge qui absorbe encore 43 %
d'erreur sur `u_max`.

**La constante du véhicule d'essai reste à 0,45**, et ce n'est pas une hésitation : le véhicule sert
à mesurer, et changer son `ν` déplacerait toutes les références publiées — demi-vies de C03, front
de C04, ordres de C08 — sans qu'aucune mesure y gagne.

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| C sous borne gouvernante, 0,5 à 20 m/s | **0,450 partout** | la borne analytique tient au millième |
| sous-estimation d'`u_max` sous borne absolue | **jusqu'à ×5,5** | le défaut est réel et chiffré |
| C réalisé sous borne absolue à `u_p = 20` | **2,482** | la condition est franchie |
| divergence observée | **aucune** | ce qui est perdu est la garantie, pas la simulation |
| seuil de paroi à `ν = 0,70` | **1,90 m/s** | monter `ν` rapproche du trou |
| coût en pas de temps à `u_p = 10 m/s` | **×3,26** | un objet rapide triple le coût d'un domaine |

### Ce qui n'a pas été fait, et pourquoi

- **La paroi est un batteur au bord, pas une paroi intérieure à cellules coupées.** Dans un maillage
  fixe, la condition injecte du volume — c'est correct pour un batteur, et le code le dit. La
  grandeur mesurée, `|u_fluide − u_paroi|` sur la face au contact, est la même dans les deux
  montages, mais la géométrie que SPEC-004 §10.1 impose réellement reste à exercer (S28-2).
- **Aucune mesure en 2D ni avec déferlement** : `ν` au-delà de 0,70 reste fermé (S28-4).
- **Le budget d'un domaine δ ne tient pas compte de ce qui tombe dedans** — ×3,3 à 10 m/s, et C20
  décrit ce régime depuis S12 sans mentionner son coût (S28-1).

### Session suivante recommandée

**S29 — S28-3, réexaminer les assertions des cas existants.** A129 montre qu'une assertion de la
forme « le solveur casse » certifie l'absence d'un défaut présent ; il faut savoir combien de cas du
corpus sont écrits ainsi. C'est une revue courte, à fort rendement, et elle touche ce sur quoi B3
s'apprête à s'appuyer.

Deux autres entrées : **S26-2**, la réinjection à la frontière W/δ — toujours la question la plus
lourde ouverte ; ou **H2**, non écrit après neuf sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S29 — 2026-09-06 — L'audit des assertions, qui s'est trouvé lui-même deux fois

**Consigne reçue.** « Enchaîne S29 ».

**Sorties.** [`AUDIT-ASSERTIONS-S29`](../docs/registres/AUDIT-ASSERTIONS-S29.md) — les 23 cas
classés ; **notes correctives datées** sur C11 et C15 ; la mesure d'**amplification du mode de
maille** dans le harnais, et le retrait de `stabilite_par_courant` ; registre porté à **134 angles
morts** ; leçons L98 à L101 ; action **S28-3 close**.

**Ça tourne.** `cargo test` : **33 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**Le corpus tient mieux que ma thèse ne le craignait — dix-huit cas sur vingt-trois sont exempts —
et les deux fautes trouvées étaient les miennes.**

### Le critère, et la catégorie qu'il a fait apparaître

> **Une assertion est recevable s'il existe une grandeur continue dont elle est le seuil.**

Éprouvé sur trois cas connus avant d'être appliqué en série, il a révélé une troisième classe que le
plan ne prévoyait pas :

| Classe | Ce qui cloche | Remède |
|---|---|---|
| **A — recevable** | rien | — |
| **B — symptôme** | ne peut échouer que sur un accident | assertir sur la grandeur gouvernée |
| **C — vacuité** | satisfaite parce que le mécanisme testé est **absent** | un **témoin** qui doit la faire échouer |

**La classe C vient de C18**, « zéro allocation après initialisation ». Le réflexe est de la ranger
avec les assertions négatives : c'est faux, il existe un compteur, il est lu, elle est **recevable**.
Mais elle vaut aussi zéro **si rien ne tourne** — et c'est une seconde façon d'être verte sans rien
dire. Angle mort **A132**.

Le remède existait déjà ici **sans avoir été nommé** : `C01-jet` est un témoin, et le harnais signale
comme anomalie le jour où il cesserait d'échouer.

### Le corpus

Cinq cas portent au moins une assertion fautive : **C07** et **C10** (« nettement supérieure »,
« sensiblement plus longue » — aucun seuil), **C11** (« aucune divergence », « aucun tremblement
visible » — et « visible » n'a pas d'observateur défini), **C15** et **C18** (vacuité).

**Et C20 est exemplaire** : *« assertion sur la pente, pas sur la valeur absolue — une pente juste
avec un décalage constant révèle un défaut de détection de contact, une pente fausse révèle un défaut
de modèle »*. Il **distingue deux défauts par la forme de sa mesure**. Écrit en S12, sans que le
principe soit énoncé.

### La première faute était la mienne, et elle portait une conclusion

`stabilite_par_courant`, écrite en S27, classait une exécution en `Stable / Diverge / NonFini`. Elle
a répondu **« OK partout »** de `ν = 0,45` à `0,99`, et ADR-035 §4 en a tiré une ligne.

**Classe B, et la mesure de remplacement le démontre.** Le facteur d'amplification du mode de maille
(`λ = 2·dx`), la grandeur que von Neumann gouverne :

| `ν` | `\|G\|` par pas | amplitude finale/initiale |
|---|---|---|
| 0,45 | 0,9595 | 2,91e−4 |
| 0,90 | 0,9291 | 6,91e−4 |
| 0,99 | 0,9355 | 2,48e−3 |
| **1,05** | **1,0204** | **7,53e0** |
| 1,50 | 1,7903 | 5,50e20 |

> **La transition est exactement à `ν = 1`.** La mesure n'a pas servi à établir cette borne : elle la
> **retrouve** — et c'est ce qui la valide. Une mesure de stabilité incapable de retrouver la
> frontière connue ne dirait rien des frontières inconnues.

**À `ν = 1,05`, le schéma amplifie d'un facteur 7,5 en cent pas, et l'ancien critère aurait répondu
`Stable`** : amplitude finale 0,0075 m pour un seuil de divergence fixé à 0,06 m.

**Ce qui sauve ADR-035** est ailleurs : la mesure de justesse — erreur de période, continue — porte
réellement la conclusion. La ligne de stabilité était décorative. C'est **A134**, la configuration la
plus difficile à détecter : rien n'est faux, donc rien n'alerte.

### La seconde faute a été commise pendant l'audit

Le premier balayage d'amplification incluait `ν = 1,05` et a rendu **exactement le résultat de
`ν = 0,99`**. `avec_cfl` bornait silencieusement à `[0,05 ; 0,99]`.

**L'instrument était incapable de produire le résultat qu'il cherchait, et rien ne le disait.** Ce
n'est pas une assertion qui ne peut pas échouer, c'est un **réglage qui ne peut pas atteindre le
régime testé** — la même faute d'un cran plus haut, et invisible par la même mécanique. Une garde de
sécurité posée sur un instrument de mesure l'empêche de mesurer. Angle mort **A133**.

La borne haute passe à 2,0 : au-delà de 1 le schéma n'a plus de garantie, et c'est précisément ce
qu'on veut pouvoir observer.

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| cas exempts / total | **18 / 23** | le corpus tient |
| assertions fautives | **6**, réparties sur 5 cas | dont 4 de classe B, 2 de classe C |
| `\|G\|` à `ν = 0,99` puis 1,05 | **0,9355 → 1,0204** | la frontière théorique est retrouvée |
| amplification à `ν = 1,05` sur 100 pas | **×7,5** | et l'ancien critère disait `Stable` |
| mesures du harnais retirées | **1** | un faux positif en attente |

### Ce qui n'a pas été fait, et pourquoi

- **Les cinq assertions fautives ne sont pas réécrites.** Aucune n'est exécutable aujourd'hui — elles
  attendent des couches qui n'existent pas — et les réécrire demande de choisir des seuils, donc des
  provenances. C'est l'action **S29-1**, et le registre dit ce qu'il faut assertir à la place.
- **Le témoin n'est pas systématique** (S29-2), ni le contrôle d'atteignabilité (S29-3).
- **L'audit porte sur la *forme* des assertions, pas sur leur résultat.** La plupart n'ont jamais été
  exécutées. C'est délibéré : la forme est ce qui peut être corrigé avant que la couche existe.

### Session suivante recommandée

**S30 — S29-1, réécrire les cinq assertions fautives.** Le registre dit pour chacune ce qu'il faut
mesurer ; il reste à choisir les seuils et leur provenance, ce qui est le travail de conception que
S29 a délibérément laissé. C'est court, et cela ferme proprement ce que cet audit a ouvert.

Deux autres entrées : **S26-2**, la réinjection à la frontière W/δ — toujours la question la plus
lourde ouverte ; ou **H2**, non écrit après dix sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S30 — 2026-09-06 — Les cinq assertions réécrites, et aucun seuil inventé

**Consigne reçue.** « Enchaîne S30 ».

**Sorties.** Les **cinq assertions fautives réécrites**, avec notes correctives datées dans
`CAS-CANONIQUES` — C07, C10, C11, C15, C18 ; §5 bis ajouté au registre
[`AUDIT-ASSERTIONS-S29`](../docs/registres/AUDIT-ASSERTIONS-S29.md), **dont la correction de son
propre classement** ; registre porté à **137 angles morts** ; leçons L102 à L105 ; action **S29-1
close**.

**Ça tourne.** `cargo test` : **33 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.
Session sans code neuf : le travail était de conception.

### Le résultat de la session tient en une phrase

**Aucun seuil n'a eu à être inventé — ce que l'audit S29 avait pris pour un manque de seuils était
un manque de lecture.**

| Cas | Assertion de remplacement | Origine | Nombre neuf ? |
|---|---|---|---|
| **C07** | pente de `log A` contre `log\|1 − Fr_h²\|` = **−½ ± 0,15** | facteur de résonance, ADR-011 §4 | non |
| **C10** | `T_avec/T_sans = √2 ± 15 %` | `√(1 + m_a/m)` avec `m_a ≈ m` — A26 + Archimède | non |
| **C11** | `max\|z − η\| = 0` · `\|G\| ≤ 1` par période | ADR-008 §3, « exactement stable » · conservation | non |
| **C15** | témoin à `Hs = 0,05 m` : épaisseur **non nulle** | SPEC-002 §4 | non |
| **C18** | `scénarios_exécutés_par_l_hôte_serveur ≥ 1` | décompte ; vaut zéro aujourd'hui | non |

Les grandeurs étaient dans le corpus, **sous les assertions qui ne les nommaient pas**.

### Le plus joli des cinq : C10

`T = 2π·√((m + m_a)/(ρ_eau·g·A))` donne `T_avec/T_sans = √(1 + m_a/m)`. **A26** pose que la masse
ajoutée d'une coque vaut environ la masse déplacée ; et un corps qui flotte déplace, par Archimède,
**exactement sa propre masse**. Donc `m_a ≈ m`, et le rapport vaut **√2 ≈ 1,414** — sans dépendre de
la taille du cube, de sa densité ni de la profondeur : tout s'annule dans le quotient.

> **Il n'y avait pas de seuil à choisir. Il y avait une formule à retrouver.**

La tolérance de ±15 % n'est pas choisie non plus : elle encode le mot « environ » d'A26, un
coefficient `m_a/m ∈ [0,5 ; 1,5]` donnant `[1,225 ; 1,581]`. **Contrôle indépendant** : le disque
équivalent de même aire, `m_a = (8/3)ρR³`, donne **1,399** — les deux voies concordent à 1 % sans
rien partager.

### Ce qui n'avait pas été anticipé

**Ma thèse annonçait qu'un seuil au moins serait à calibrer. Zéro.** Et trois des cinq réécritures
sont **plus fortes** que l'énoncé d'origine.

**C11 en est l'exemple net.** « Aucun tremblement **visible** » tolère tout écart sous le seuil de
perception. Or ADR-008 §3 pose qu'en mode contraint l'objet est projeté sur la surface — `z = η` — et
qualifie le résultat d'« **exactement stable** ». La référence est **zéro**, pas un seuil.
**L'énoncé demandait donc moins que ce que la conception promet**, et un écart de `10⁻⁴ m` — invisible,
mais signalant que la projection n'est pas appliquée — l'aurait passé. Le flou ne rend pas seulement
imprécis : **il déplace l'exigence vers le bas**, et toujours dans ce sens (**A136**).

**Et C07 cumulait deux défauts, dont le second n'était visible qu'après correction du premier.** Par
5 m de fond, `√(g·h) = 7,00 m/s` ; les quatre vitesses de l'énoncé donnent
`Fr_h = 0,71 · 1,14 · 1,43 · 2,14`. **Le point critique est sauté** — l'assertion vise un régime que
le montage ne produit pas. C'est A133, trouvé en S29 dans le harnais, cette fois dans le corpus.

> **Une assertion sans grandeur masque un montage incapable**, et les deux défauts se protègent l'un
> l'autre : tant que l'assertion disait « nettement supérieure », rien n'obligeait à vérifier que le
> montage produisait le régime (**A137**).

### L'audit de S29 s'est trompé une fois, dans le sens le plus coûteux

C18 porte **deux** lignes vides, pas une. « L'hôte serveur compile et tourne » avait été recensée ;
« aucune capacité dérivée n'est lue depuis un profil de qualité » avait été **classée recevable**,
par ressemblance avec « zéro allocation ».

Les deux ont la même forme. Ce qui les sépare est l'existence d'un **instrument** : « zéro
allocation » a son compteur — `AllocStats::refused_after_seal`, lu par le harnais depuis S20 —
l'autre demanderait une analyse statique, qui n'existe pas.

> **L'instrument ne se lit pas dans l'énoncé.** Classer une assertion sur sa formulation seule est
> insuffisant : il faut, pour chacune, **nommer ce qui la mesure** et vérifier que cela existe. C'est
> un troisième contrôle, après la grandeur et le témoin (**A135**).

### Ce qui n'a pas été fait, et pourquoi

- **Le montage de C07 n'est pas corrigé**, seulement l'assertion : ajouter les quatre vitesses du
  balayage est une modification d'énoncé qui mérite sa propre étape (S30-1).
- **Les instruments ne sont pas nommés** pour les 47 assertions (S30-2), et « aucune capacité dérivée
  lue depuis un profil de qualité » reste sans le sien (S30-3).
- **Aucune des réécritures n'est exécutable aujourd'hui** — les cinq cas attendent leur couche. C'est
  le principe posé en S29 : la **forme** se corrige avant que la couche existe, et c'est ce qui rend
  la correction bon marché.

### Session suivante recommandée

**S31 — S26-2, la réinjection à la frontière W/δ.** C'est la question la plus lourde ouverte depuis
S26, et la seule qui touche la couture entre deux couches : si δ filtre les composantes courtes
(ADR-034), la transition doit-elle les réinjecter depuis W, ou l'effacement est-il voulu ? ADR-005 ne
l'a pas prévue.

Deux autres entrées : **S30-2**, nommer l'instrument de chaque assertion — court et mécanique ; ou
**H2**, non écrit après onze sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S31 — 2026-09-06 — La question la plus lourde se dissout, et ce qui la remplace est pire

**Consigne reçue.** « Enchaîne S31 ».

**Sorties.** [`ADR-036`](../docs/adr/ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) ; note
corrective datée dans ADR-034 §3.3 ; la mesure d'**étalement d'un paquet localisé** dans le harnais ;
registre porté à **140 angles morts** — dont le centième point ; leçons L106 à L109 ; action
**S26-2 close par dissolution**.

**Ça tourne.** `cargo test` : **33 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**A122 n'avait pas d'objet, et elle a été recommandée quatre fois avant d'être lue une fois.**

S26, S28, S29 et S30 l'ont toutes désignée comme « la question la plus lourde ouverte ». Elle s'est
dissoute en relisant ADR-001 §2 et ADR-005 §1 — **deux documents antérieurs à sa formulation**.

### La dissolution

ADR-001 §2 : `Surface_visible = B + W + δ`. ADR-005 §1 en tire déjà que la « conversion onde
analytique → état volumique » est *sans objet — B+W est un terme de forçage lu par le solveur, pas
une condition d'entrée à convertir*.

> **δ ne transporte pas la houle : il transporte l'*écart* à la houle.** Une composante courte de W
> traverse un domaine δ sans y être dissipée, puisqu'elle n'y est pas discrétisée. **Il n'y a rien à
> réinjecter.**

C'est la troisième dissolution du corpus, après les deux d'ADR-027 §1 — et comme elles, elle vient
d'une relecture de prémisse, pas d'une astuce.

### Ce qui la remplace, et qui est pire

**La loi de dissipation ne disparaît pas : elle change de sujet.** Elle gouverne ce que δ porte
réellement — les **perturbations locales** : sillage, impact, éclaboussure. C'est-à-dire exactement
ce pour quoi δ existe.

Un sillage de Kelvin a `λ = 2πv²/g`. À `dx = 1 m`, `ν = 0,45` :

| Vitesse | `λ` | demi-vie | **distance visible derrière le bateau** |
|---|---|---|---|
| 3 m/s | 5,8 m | 0,37 période | **2,1 m** |
| 5 m/s | 16,0 m | 1,02 période | **16,4 m** |
| 10 m/s | 64,0 m | 4,09 périodes | **262 m** |

**Une barque à 3 m/s laisse un sillage plus court qu'elle-même.** Et la loi d'échelle est brutale :
la durée de vie va comme `v³`, la distance visible comme **`v⁴/dx`** — vérifié, `(10/3)⁴ = 123`
contre 125 mesuré.

> **δ dissipe le plus vite précisément ce qu'il existe pour produire, et d'autant plus que l'objet
> est lent.** Barques, canoës et nageurs n'auront aucun sillage ; les navires rapides en auront un
> qui traverse le domaine. C'est probablement l'inverse de l'attendu.

### La mesure : le paquet s'étale, et l'étalement est un artefact

Bosse gaussienne, `dx = 0,5 m`, après 20 s :

| | `σ = 1 m` | `σ = 4 m` |
|---|---|---|
| pic / pic₀ | **0,102** | 0,315 |
| largeur / largeur₀ | **5,00** | 1,74 |

Le paquet fin **quintuple sa largeur** ; il se dégrade **en forme avant de se dégrader en
amplitude**, signature d'un filtre passe-bas appliqué à un spectre.

> **Et tout cet étalement est un artefact.** Saint-Venant est non dispersif : une perturbation s'y
> scinde en deux trains qui se propagent **sans déformation**. La forme est conservée exactement par
> l'équation que le solveur prétend résoudre, et **rien n'en mesurait la perte** (A138).

D'où **C24, « conservation de forme d'un paquet »** — référence `1` exactement, aucun seuil à
inventer.

### Le critère de traversée, et ce qu'il ajoute à ADR-005

Une perturbation survit à la traversée d'un domaine de largeur `D` si `λ² ≥ K·dx·D`, avec
`K = 2π²(1−ν)/ln2` = **15,66** à `ν = 0,45`.

| Domaine | `dx` | `λ_min` |
|---|---|---|
| 50 m | 0,25 m | 14,0 m |
| 200 m | 1,00 m | 56,0 m |
| 1 000 m | 2,00 m | 177,0 m |

> **`λ_min` dépend de la taille du domaine, en `√D`.** ADR-005 §2.1 ne rapporte `λ_cut` qu'à `dx` :
> **c'est insuffisant**. Doubler le domaine à résolution constante remonte `λ_min` de 41 %.

**Correction en séance** : la première rédaction portait `K = 28,5`, en omettant le facteur `(1−ν)`.
Les quatre `λ_min` en dépendaient. L'erreur a été prise **par le calcul, pas par la relecture** — le
tableau avait été écrit à la main à partir d'un facteur mémorisé de travers.

### Ce qui n'a pas été fait, et qui décide de la gravité du reste

**Le sillage est peut-être un objet de W et non de δ.** ADR-011 §4 place son générateur dans **W** —
*« le générateur de sillage de la couche W doit prendre `h` en entrée »*. S'il est dans W, il n'est
pas discrétisé, il ne se dissipe pas, et tout le §3 d'ADR-036 tombe.

**Aucun document ne tranche**, et les deux lectures se défendent : un sillage est une onde (donc W),
mais il est créé par un objet local en mouvement (donc δ). **C'est l'angle mort A139, sévérité 1**,
et il décide si le résultat le plus visible de cette session est un problème majeur ou sans objet.
Je ne le tranche pas : c'est une décision de conception que la session n'a pas les moyens d'appuyer
sur une mesure.

### Session suivante recommandée

**S32 — S31-1, trancher où vit le sillage.** C'est court, c'est de la conception pure, et cela
détermine la portée d'ADR-036 §3. Les deux ADR qui se contredisent sont identifiés ; il reste à
choisir, et à dire ce que le choix impose.

Deux autres entrées : **S31-2**, écrire C24 ; ou **S30-2**, nommer l'instrument de chaque assertion.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S32 — 2026-09-06 — La réponse était dans ADR-001, pour la deuxième fois de suite

**Consigne reçue.** « Enchaîne S32 ».

**Sorties.** [`ADR-037`](../docs/adr/ADR-037-la-dissipation-est-un-allie-pour-la-moitie-de-delta.md) ;
note corrective datée dans ADR-036 §3 — dont l'objet n'existe pas ; registre porté à **143 angles
morts** ; leçons L110 à L113 ; action **S31-1 close par dissolution**.

**Ça tourne.** `cargo test` : **33 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.
Session sans code neuf.

### Le résultat de la session tient en une phrase

**A139 était fausse, l'erreur était la mienne, et ADR-001 §2 y répondait depuis S01.**

| Couche | Contenu, verbatim |
|---|---|
| **W** | « **sillages**, anneaux d'impact, ondes d'explosion, tsunamis, déferlement, réfraction bathymétrique » |
| **δ** | « proche-coque, gerbe d'étrave, éclaboussure, cavité d'impact, poche d'air, remous sur rocher » |

En S31 j'avais écrit « aucun document ne tranche » après n'avoir lu qu'ADR-011 §4 — qui parle du
*générateur* de sillage dans W et ne contredit rien. **Tout ADR-036 §3, et son chiffre le plus
frappant — deux mètres de sillage derrière une barque — porte sur un objet que δ ne contient pas.**

> **Deux sessions de suite, le même défaut, sur le même document.** A122 dissoute en S31 en relisant
> ADR-001 ; A139 dissoute en S32 en relisant ADR-001. Le socle du corpus a répondu deux fois à une
> question dite ouverte, et la seconde fois c'est la session précédente qui l'avait posée sans le
> consulter. Angle mort **A143**, sévérité 1 : **un corpus qui grandit rend son propre socle moins
> consulté** — trente-sept ADR se lisent moins qu'un, et le premier est celui qu'on croit connaître.

### Ce qu'ADR-001 dit d'autre, et que S31 n'avait pas lu non plus

> *« δ tend vers 0 en s'éloignant de sa source. Ce n'est pas une contrainte imposée de l'extérieur :
> c'est la **définition de la couche**. »*

**La décroissance de δ est voulue** — mais **en espace**, alors que la dissipation numérique agit
**en temps**. Toute la question tient dans cet écart, et il partitionne le contenu de δ :

| | Phénomènes | Ce que la dissipation fait | Statut |
|---|---|---|---|
| **entretenus** | proche-coque, gerbe d'étrave, remous sur rocher | la source réalimente, la dissipation atténue avec le trajet → **équilibre spatial** | **mécanisme voulu** |
| **transitoires** | éclaboussure, cavité d'impact, poche d'air libérée | rien ne réalimente → **mort avant la fin physique** | **défaut dimensionnant** |

> **La dissipation réalise la définition de δ pour les entretenus et la trahit pour les
> transitoires.** C'est un renversement complet du cadrage de S31, qui la traitait comme un défaut
> uniforme.

**Et la longueur de décroissance tombe juste** : le proche-coque s'éteint naturellement à **25,6 m**,
quand ADR-001 donne au domaine « quelques dizaines de mètres ». Il n'est donc pas nécessaire
d'imposer cette décroissance par une éponge — elle est déjà là.

### Le critère de dimensionnement, et il est brutal

`t_num = K·L²/(dx·c)` contre `t_phys ≈ √(2L/g)`. La condition se résout, et **`g` disparaît** :

> ```
> dx ≤ K · L^1,5 / √(2h)
> ```

| `L` | `dx_max` à `ν = 0,45` | `dx_max` à `ν = 0,70` |
|---|---|---|
| 0,5 m | **1,1 cm** | 2,1 cm |
| 1 m | **3,2 cm** | 5,9 cm |
| 2 m | 9,0 cm | 16,6 cm |
| 5 m | 35,7 cm | 65,5 cm |

**Une éclaboussure d'un mètre demande `dx = 3,2 cm`, sur un solveur 3D.** À `dx = 0,25 m`, elle
s'éteint **huit fois trop tôt** ; une de cinquante centimètres, **vingt fois**. Le rapport croît
comme `L^1,5/dx` : ce sont les plus petits phénomènes qui sont détruits, et ce sont eux que δ existe
pour montrer.

### Ce que cela ajoute à ADR-035

Passer de `ν = 0,45` à `0,70` multiplie `dx_max` par **1,83**, donc divise le nombre de cellules 3D
par **6,1**. Le levier du nombre de Courant ne se mesure plus en portée d'onde — il se mesure en
**taille de maille pour une fidélité de transitoire donnée**, et c'est la contrainte dimensionnante
de δ.

### Ce qui n'a pas été fait, et pourquoi

- **La conclusion la plus rassurante est la moins étayée.** L'équilibre spatial du proche-coque à
  25,6 m est **dérivé**, jamais mesuré : il suppose qu'une source constante et une dissipation
  exponentielle donnent `exp(−x/L_d)`, vrai en linéaire, et le solveur ne l'est pas. Toutes les
  mesures de S25 à S32 portent sur des perturbations **relâchées**, aucune sur une source
  **entretenue** (A142, action S32-2).
- **`t_phys = √(2L/g)` est une estimation.** La durée *perçue* d'une éclaboussure inclut l'écume et
  le spray d'ADR-014, qui survivent plus longtemps que la déformation de surface. Le critère est
  conservateur ou optimiste selon ce qu'on décide de voir (A141).
- **La partition entretenus/transitoires n'est pas dans ADR-001**, qui liste six contenus sans les
  distinguer. Elle devrait y être portée (S32-1).

### Session suivante recommandée

**S33 — S32-2, mesurer un phénomène entretenu.** C'est la seule des quatre actions qui produise une
**mesure**, et elle porte sur la conclusion la moins étayée du corpus récent. Le véhicule sait déjà
imposer une paroi mobile (C23) : une source entretenue est à portée.

Deux autres entrées : **S31-2**, écrire C24 (conservation de forme d'un paquet), dont la référence
est `1` exactement ; ou **H2**, non écrit après douze sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S33 — 2026-09-06 — La conclusion la moins étayée devient la mieux mesurée

**Consigne reçue.** « Enchaîne S33 ».

**Sorties.** Le **batteur oscillant** et la mesure de décroissance spatiale dans le harnais ; **note
S33 datée** dans ADR-037 §2.1 ; registre porté à **145 angles morts** ; leçons L114 à L117 ; action
**S32-2 close**, angle mort **A142 levé**.

**Ça tourne.** `cargo test` : **34 tests** au vert. `water-harness check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**La conclusion la plus rassurante du corpus récent, écrite en S32 comme *dérivée et jamais mesurée*,
est devenue la mieux mesurée — `R²` = 1,0000.**

| `λ` | `L½` mesurée | `L½` prédite par `K·λ²/dx` | écart | `R²` |
|---|---|---|---|---|
| 10 m | 27,03 m | 25,54 m | +5,9 % | 0,9990 |
| 14 m | 51,40 m | 50,06 m | +2,7 % | 0,9990 |
| 20 m | 101,81 m | 102,15 m | **−0,3 %** | **0,9999** |
| 28 m | 195,34 m | 200,22 m | −2,4 % | **1,0000** |

La décroissance spatiale d'un train **entretenu** est exponentielle pure, et sa longueur de
demi-décroissance suit `L½ = K·λ²/dx` sur un facteur 8, de 25 à 200 m. **`c` disparaît** de la
formule — troisième annulation du corpus, après celle de `λ`, `c` et `T` dans la loi de dissipation
(S25) et celle de `g` dans le critère de transitoire (S32).

**Ma thèse annonçait « `L½` sera plus courte que prédit ». Elle est fausse** : l'écart **change de
signe** avec `λ`, ce qui est la signature des termes d'ordre supérieur en `k·dx`, non d'un biais.

**A142 est levé — dans le régime linéaire**, et la note le dit à l'endroit où elle rassure :
l'amplitude du batteur donne `a/h ≪ 1 %`, donc la mesure valide la dérivation **là où elle était
supposée valide** (A127). Elle ne dit rien du régime non linéaire.

### Ce qui a rendu la session possible

Rien de particulier — **sinon d'avoir écrit en S32 que la conclusion était dérivée et non mesurée**.
Sans cette phrase, personne ne serait allé la vérifier : elle rassurait, elle était cohérente, et
elle occupait la même place typographique qu'un résultat.

### Ce qui n'avait pas été anticipé

**Le garde-fou d'atteignabilité était incomplet, et j'y suis retombé.** Le premier passage donnait,
pour `λ = 20 m`, un `R²` de **0,487** là où les autres cas donnaient 0,999.

Le front était à **280 m dans un domaine de 200 m** : l'onde avait atteint le mur, s'était réfléchie,
et revenait polluer la fenêtre — que mon contrôle déclarait saine, puisqu'il vérifiait qu'on mesurait
*derrière le front* et non que le front *n'avait jamais atteint le mur*.

> **C'est A133 — un montage incapable — commis dans la fonction écrite pour l'éviter**, et dans la
> session qui l'invoquait au plan. L'idée du garde-fou était juste, sa condition ne l'était pas — et
> une condition fausse reste invisible tant qu'elle n'est pas franchie (**A144**).

**Ce qui l'a rattrapé n'est pas le garde-fou, c'est le `R²`** — une mesure de qualité d'ajustement,
qui n'était pas là pour détecter une réflexion mais pour dire si la décroissance est exponentielle.
Elle a signalé que la forme supposée était fausse **sans avoir à connaître la raison**, ce qu'aucun
garde-fou spécifique ne sait faire (**A145**).

### Chiffres qui ont orienté la conception

| Mesure | Valeur | Ce qu'elle dit |
|---|---|---|
| `R²` de la décroissance, `λ = 28 m` | **1,0000** | exponentielle pure |
| écart `L½` mesurée/prédite, `λ = 20 m` | **−0,3 %** | la loi tient |
| plage de `L½` couverte | **25 à 200 m** | un facteur 8, pas un point |
| `R²` du cas pollué par réflexion | **0,487** | la sonde générique a vu ce que le garde-fou n'a pas vu |

### Ce qui n'a pas été fait, et pourquoi

- **Le régime non linéaire n'est pas mesuré.** L'amplitude du batteur est faible à dessein : A127
  établit que la loi de dissipation est fausse à `a/h = 5 %`, et mesurer là demanderait de savoir ce
  qu'on compare — la loi n'y prédit rien (S33-1).
- **Les autres garde-fous du harnais n'ont pas été relus** dans le sens d'A144 (S33-2), et le `R²`
  n'accompagne pas encore toutes les régressions (S33-3).
- **La partition entretenus/transitoires n'est toujours pas dans ADR-001** (S32-1).

### Session suivante recommandée

**S34 — S33-2, relire les garde-fous du harnais.** A144 dit qu'un garde-fou peut porter sur la bonne
idée et la mauvaise condition, et qu'il est alors **plus dangereux qu'aucun garde-fou** — on lui fait
confiance. La session S29 a audité les *assertions* ; les *contrôles de validité de montage* n'ont
jamais été audités, et il en existe maintenant plusieurs.

Deux autres entrées : **S31-2**, écrire C24 (conservation de forme d'un paquet), dont la référence
est `1` exactement ; ou **H2**, non écrit après treize sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S34 — 2026-09-06 — Dix garde-fous mis à l'épreuve, et la non-testabilité prédit la défaillance

**Consigne reçue.** « Enchaîne S34 ».

**Sorties.** [`AUDIT-GARDE-FOUS-S34`](../docs/registres/AUDIT-GARDE-FOUS-S34.md) ; **onze tests de
déclenchement** dans le harnais ; le garde-fou G10 corrigé et **extrait en fonction pure** ; registre
porté à **148 angles morts** ; leçons L118 à L121 ; action **S33-2 close**.

**Ça tourne.** `cargo test` : **45 tests** au vert, contre 34 en début de session. `water-harness
check` : 0 échec, hashs inchangés.

### Le résultat de la session tient en une phrase

**Sur dix garde-fous, un seul masquait au lieu de refuser — et c'était le seul qui n'était pas
appelable isolément.**

### Le critère, et son corollaire

> **Un garde-fou qu'on n'a jamais vu déclencher n'a pas été testé.** Le test d'un garde-fou est
> **le cas qu'il doit refuser**, jamais le cas nominal — celui-ci passe de toute façon.

**Corollaire apparu en cours d'audit** : il faut aussi son **témoin**, le cas sain qu'il ne doit
*pas* refuser. Sans lui, un contrôle qui refuserait tout passerait son propre test. Trois des dix en
ont reçu un — G2, G6, G9.

### L'inventaire

| | Garde-fou | Cas qu'il doit refuser | Verdict |
|---|---|---|---|
| **G1** | pas de temps sur domaine sec | `vmax = 0` | refuse |
| **G2** | bornage de `ν` | que `ν = 1,5` soit ramené sous 1 | laisse passer, comme corrigé en S29 |
| **G3** | plancher d'arrondi | trois erreurs sous le plancher | refuse |
| **G4** | longueur de série | deux points pour Richardson | refuse |
| **G5** | amplitude de seiche | `a` sous l'ulp du `f32` | refuse |
| **G6** | réflexion | le montage à `R² = 0,487` de S33 | refuse, et laisse passer le domaine long |
| **G7** | seuil de front | un seuil qu'aucune cellule n'atteint | refuse |
| **G8** | référence nulle | la division par zéro de C01 | refuse |
| **G9** | définition d'`u_max` | confondre absolue et gouvernante | distingue, et coïncide sans paroi |
| **G10** | bornage de l'ordre grossier | un ordre **négatif** | **masquait** |

### G10, et pourquoi c'était lui

Le `clamp(0,3 ; 3,0)` sur l'ordre estimé aux grilles grossières corrigeait **en silence**. Or S24 a
mesuré des ordres **négatifs** — −0,504 puis −0,059 sur le front de C04 — signature du régime
pré-asymptotique.

> **Un ordre hors bornes n'est pas une valeur à corriger : c'est le signe que les grilles grossières
> ne sont pas asymptotiques**, donc que l'estimation d'erreur d'oracle qui en dépend n'a aucun
> fondement. Le borner revient à répondre à une question dont on vient d'apprendre qu'elle n'a pas
> de réponse.

**Trois gestes.** Le bornage **reste** — il faut un nombre pour filtrer, et il est conservateur. Il
est **signalé**, au `Sink` et dans le libellé. Et l'estimation est **extraite** en fonction pure
`ordre_grossier_estime → (brut, borné)`.

**Le troisième geste est le plus important**, et il porte la leçon de la session : l'estimation
vivait **en ligne** dans une fonction qui lance des simulations avec un oracle à 51 200 cellules. La
vérifier demandait d'en exécuter une.

> **Un garde-fou qu'on ne peut pas exercer isolément est un garde-fou qu'on n'exercera pas.**
>
> G10 était le **seul des dix sans test**, et le **seul défaillant**. Les neuf autres, appelables
> directement, avaient été éprouvés au fil des sessions **sans que ce soit délibéré**. La chaîne est
> mécanique : *emplacement en ligne → non testable isolément → jamais testé → jamais vu refuser →
> défaut invisible* (**A147**).

### Ce que l'audit dit de la méthode

**Le critère « l'a-t-on vu refuser ? » est plus discriminant que la relecture.** Les dix garde-fous
ont été relus plusieurs fois au fil des sessions, et G10 y a survécu parce qu'il **a l'air correct** :
borner une estimation entre deux valeurs raisonnables est un geste ordinaire, et rien dans sa
formulation ne dit qu'il masque.

**Ce qui l'a désigné n'est pas sa forme, c'est son absence de test.**

### Ce qui n'a pas été fait, et pourquoi

- **Les saturations de modèle n'ont pas été auditées.** `delta.rs` en contient une douzaine —
  `h.max(0.0)` après un pas, la reconstruction hydrostatique. Elles relèvent d'une autre famille :
  ce sont des rattrapages de **physique**, pas des contrôles de montage. **Aucune n'est comptée**, et
  une saturation rare est un filet quand une saturation à chaque pas est un solveur qu'on maquille —
  les deux sont indiscernables aujourd'hui (**A146**, action S34-1).
- **Aucun garde-fou ne compte ses déclenchements** : on sait qu'ils *peuvent* refuser, pas s'ils
  refusent en usage réel (**A148**).
- **Le signalement de G10 passe par le `Sink` et le libellé**, que personne ne lit
  automatiquement. Il devrait remonter dans le **verdict**, comme les témoins de C01 et C04 (S34-3).

### Session suivante recommandée

**S35 — S34-1, compter les saturations de modèle.** C'est la suite naturelle et elle produit une
**mesure** : combien de fois `h.max(0.0)` sauve un pas dans C04 ? Si c'est souvent, le lit sec est
maquillé plutôt que résolu, et le résultat d'ADR-031 change de nature. Le compteur coûte quelques
lignes et il est immédiatement exploitable.

Deux autres entrées : **S31-2**, écrire C24 (conservation de forme d'un paquet) ; ou **H2**, non
écrit après quatorze sessions où il est cité.

### Arbitrages en attente

Inchangés. **A103** — la masse volumique de l'eau, douce ou de mer. **A107** — le sort du travail
propre à `master`, S16-S17. Et les trois choses hors de portée d'une session : nommer les personnes,
constater l'état réel du projet, agir sur l'infrastructure — dont le dépôt distant.

---

## S35 — 2026-09-06 — Le dépôt avait forké une seconde fois, et les deux lignées avaient écrit le même solveur

**Consigne reçue.** « Reprends le projet », depuis un worktree positionné sur `master`.

**Entrées.** Les deux commandes d'amorce de `CLAUDE.md` — `git worktree list`, `git branch -a` — et
ce qu'elles ont montré : **trois lignées vivantes ou mortes, trois jetons tous `libre`**.

**Sorties.** [`FORK-S22-S26`](../docs/registres/FORK-S22-S26.md) ; cinq ADR importés et renumérotés
**038–042** ; [`ADR-043`](../docs/adr/ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) ; leçons
**L122–L136** reportées et **L137–L140** écrites ; angles morts **A149–A160** reportés et **A161**
ouvert ; notes correctives portées dans `ADR-005`, `ADR-007`, `DOSSIER-B2`, `CAS-CANONIQUES` ;
`shallow.rs` importé et compilé.

**Ça tourne.** `cargo test` : **55 tests** au vert, contre 45 en début de session — les dix de
`shallow.rs` passent sans retouche. `water-harness check` : 0 échec, **hashs inchangés**
(`0x3e2c06a7b00e73e3`, `0x1a8b0629a9f51b6e`).

### Ce qui a été trouvé au premier geste

Le dépôt a forké **deux fois**. Le premier fork (S07) était documenté — mais dans une lignée qui
n'est pas celle-ci : `FORK-S08-S15.md` vit sur `master`, qui n'est l'ancêtre d'aucune branche
vivante. **Cette lignée n'a jamais su qu'elle était une branche**, et elle a reforké huit sessions
plus tard, en S21.

| lignée | dernier | ADR | code | sort |
|---|---|---|---|---|
| `claude/s22-suite` | S34, 19h14 | 37 | `delta.rs` | **lignée d'accueil** |
| `claude/reprise-projet-5134cd` | S26, 17h12 | 34 | `shallow.rs` | fusionnée ici |
| `master` | S17, 01h51 | 27 | aucun | morte, non traitée |

Cinq identifiants d'ADR, quinze leçons et douze angles morts étaient en collision — deux documents
différents sous le même numéro, selon la lignée du lecteur.

### Le résultat de la session tient en une phrase

**Les deux lignées avaient écrit le même solveur, le même jour, sans se voir.**

`delta.rs` (1292 lignes) et `shallow.rs` (1070) résolvent les mêmes équations — Saint-Venant 1D,
volumes finis, flux de Rusanov, terme de fond centré au premier jet — et s'ouvrent sur la même
précaution et la même phrase de `CAS-CANONIQUES` à propos de C01. **Ce n'est pas une coïncidence :
c'est le corpus qui a dicté le solveur.** Deux lecteurs indépendants, partis de la même page, ont
écrit le même programme.

### Ce que cela vaut, et ce que cela ne vaut pas

**Cela ne vaut pas** une confirmation du modèle : les deux partagent une dimension, `c = √(g·h)` et
l'absence de dispersion — donc **exactement les mêmes angles morts**. Deux erreurs identiques ne se
corrigent pas en se répétant.

**Cela vaut** un oracle contre la **faute d'implémentation** — l'indice décalé, le signe inversé, la
condition de bord mal posée. Le projet n'avait aucun moyen de détecter cette classe de faute : un
seul code, ses propres assertions, et un corpus qui les a écrites. C'est le premier bénéfice net du
fork, et il disparaîtrait si l'on supprimait l'une des deux implémentations. **Les deux sont
conservées.**

### Les verdicts divergeaient sur deux cas, et l'écart a une cause unique

| cas | `delta.rs` — ordre un | `shallow.rs` — ordre deux |
|---|---|---|
| **C04** | **échoue** *(S23)* | **vert** *(B-S25)*, 0,74 % sur le front |
| **C08** | **sans verdict** *(S23-S24)* | rouge, puis **vert** *(B-S24)* : `p` = 1,003 |

`ADR-031` de cette lignée conclut que **le front de mouillage élimine l'ordre un**. La lignée B a
franchi ce pas — `ADR-040`, MUSCL + RK2 — et **C04 et C08 sont passés au vert ensemble**. Les deux
résultats se complètent exactement : l'un dit ce qui échoue, l'autre ce qui réussit, et c'est le
même seuil qui les sépare. **Ni l'une ni l'autre lignée ne pouvait l'établir seule.**

### L'éponge est trois choses, et une seule a été mesurée

Les deux lignées avaient attaqué la même contrainte dure du corpus — `λ_cut ≤ 3 m`, qui repose
entièrement sur `L_s = λ_cut/2` — par deux chemins sans rapport. B a **mesuré l'absorbeur** : la
largeur ne dépend pas de `λ` mais de la maille, huit fois plus étroit. L'accueil a **mesuré la
dissipation** : elle produit gratuitement la décroissance que le masque devait imposer.

Elles concordent. Mais la fusion révèle ce qu'aucune ne pouvait voir : **le corpus appelle
« éponge » trois fonctions distinctes** — absorber, faire décroître, transduire — qui occupent la
même bande de bord et se dimensionnent par des critères sans rapport. **Une seule des trois a été
mesurée** (A161). Sans cette distinction, « la dissipation rend l'éponge inutile », vrai du masque,
se lit comme « un domaine peut se passer d'absorbeur », qui est faux.

### Le code : ce qui a résisté n'est pas ce qu'on croyait

Le plan annonçait le code comme le morceau lourd. **Le solveur s'est importé en deux lignes** :
`shallow.rs` ne dépend que de `crate::host`, dont l'API n'avait pas divergé. 55 tests verts, hashs
inchangés, aucune retouche.

**C'est le harnais qui résiste** — `physics.rs` porte des montages de même nom des deux côtés
(`c03_seiche`, `ritter`, `c08_convergence`) avec des signatures différentes, parce que chacun est
écrit contre son propre solveur. Le découpage retenu est **un module séparé**, `physics_shallow.rs`,
et non une fusion : fusionner les montages détruirait exactement l'oracle qu'on cherche à garder.

### Chiffres qui ont orienté la session

- **195** commits de retard de `master` sur la lignée d'accueil — la branche depuis laquelle la
  session a été ouverte.
- **88 contre 35** commits depuis le fork de S21 : le critère du choix de lignée d'accueil, et rien
  d'autre.
- **6 ajouts, 14 modifications** : le diff documentaire de la lignée B. C'était peu, et c'est
  pourquoi la fusion des documents a tenu dans une session.
- **5 sévérité 1** sur les douze angles morts importés — une proportion élevée, et aucun n'a encore
  été relu par cette lignée.

### Ce qui n'a pas été fait

- **Le harnais n'est pas fusionné**, ni le journal de la lignée B (cinq entrées). Découpage en
  quatre sessions dans [`FORK-S22-S26`](../docs/registres/FORK-S22-S26.md) §7.
- **L'oracle croisé n'a jamais été exercé.** Deux solveurs coexistent dans le même binaire ; aucun
  cas ne les compare encore. Tant que ce n'est pas fait, ADR-043 §3 est une promesse.
- **Rien n'a été réexécuté.** Les verdicts de la lignée B sont cités depuis ses documents, pas
  reproduits ici.
- **La lignée S08–S17 n'est pas traitée.** Elle détient `FORK-S08-S15.md` et deux ADR — `ADR-026`,
  `ADR-027` — dont il reste à établir s'ils ont un équivalent ici. *Après la clôture de la session,
  l'utilisateur a fusionné S35 dans `s22-suite`, fait pointer `master` dessus et supprimé les
  branches mortes : cette lignée n'existe plus que sous l'étiquette `archive/lignee-S08-S17`.*
- **Les cinq angles morts de sévérité 1 importés n'ont pas été relus.**

### Session suivante recommandée

**S36 — `physics_shallow.rs`** : les six montages de la lignée B sur `Shallow1D`, sans toucher à
`physics.rs`. C'est le préalable à tout le reste, et notamment à l'exercice de l'oracle (S38).

*Solutions de rechange* : les saturations de modèle (S34-1, A146), reportées par cette session ; ou
la relecture des cinq angles morts de sévérité 1 importés.

### Arbitrages en attente

Inchangés — trois arbitrages de design et quatre interfaces inter-équipes, listés dans
`docs/00_INDEX.md`. **Cette session n'en a tranché aucun et n'en a ouvert aucun.**

Elle en ajoute cependant un de nature différente, qui appartient à l'utilisateur et non au projet :
**trois worktrees restent ouverts sur trois branches divergentes**, et le dispositif du jeton ne
protège d'aucun d'eux. Voir `FORK-S22-S26` §5.

---

## S36 — 2026-09-06 — L'oracle croisé a servi dès son premier usage, et pas comme prévu

**Consigne reçue.** « Lance S36 ».

**Entrées.** Action **S35-1**. S35 avait importé le *solveur* de la lignée B, `shallow.rs`, mais
laissé dehors ses *montages* : six cas canoniques dont les verdicts étaient cités dans le corpus
sans avoir jamais été exécutés ici. `CAS-CANONIQUES` le disait en toutes lettres — *un témoignage,
pas une mesure*.

**Sorties.** `code/water-harness/src/physics_shallow.rs` — six montages, treize tests ; le second
véhicule branché au mode `physics` ; `demi_vie_seiche` extraite en fonction pure ; le **double
format d'écart** absolu / relatif (**A149**, action S35-2) ; note corrective datée sur `ADR-040` ;
angle mort **A162** ; réserve n°1 de `CAS-CANONIQUES` **levée**.

**Ça tourne.** `cargo test` : **68 tests** au vert — 32 dans le cœur, 36 dans le harnais dont un
`ignore` — contre 55 en début de session. `water-harness check` : 0 échec, **hashs inchangés**.
Mode `physics` : **31 s** dont **12,5 s** pour le second véhicule, sur un budget de 60 s
(`SPEC-003 §1`).

### La thèse, et ce qu'elle a donné

*Déclarée avant de commencer : les chiffres publiés par la lignée B se reproduisent dans cet arbre,
au dernier chiffre significatif donné.*

| grandeur | document | publié | mesuré | verdict |
|---|---|---|---|---|
| C01, courant parasite du schéma naïf | `ADR-038` §2 | 19,5 mm/s | **19,5083** | reproduit |
| C04, écart sur le front | `ADR-041` | 0,74 % | **0,7365 %** | reproduit |
| C03, quatre demi-vies de seiche | `ADR-040` §5 | 6,01 · 44,36 · 43,12 · 161,14 | **identiques** | reproduit à **0,00 %** |
| C08, ordre de convergence | `ADR-040` §3 | `p` = 1,003 | **0,9997** | **périmé** |

**Trois sur quatre, et le quatrième est le résultat de la session.**

### Le résultat tient en une phrase

**`p = 1,003` a été périmé par une correction faite pour un autre cas, une session plus tard, et
rien ne l'a signalé — parce que le cas continuait de passer.**

En **B-S25**, la référence de l'erreur `L¹` de Ritter est passée de la valeur **au centre de
cellule** à la **moyenne sur la cellule**. La correction est juste : un schéma de volumes finis
porte des moyennes, et les confronter à une valeur ponctuelle ajoute une erreur d'ordre un qui n'est
pas celle du schéma. Elle a été faite pour C04. **Elle alimentait aussi le `p` de C08**, publié la
session d'avant.

La démonstration est un test qui rejoue les deux références côte à côte :

```
C08-p — publié 1,003 | référence ponctuelle (≤ B-S25) : 1,0030 | moyenne de cellule (≥ B-S25) : 0,9997
```

L'ancienne référence retrouve **1,0030 à la quatrième décimale**. Le chiffre était juste quand il a
été écrit ; il ne décrit plus le code depuis une session.

> **Ce qui rend cet écart invisible : rien ne casse.** L'assertion de C08 est un **minorant** —
> `p > 0,8` — et trois millièmes ne la font pas broncher. Un test vert ne dit pas qu'un chiffre
> publié est encore vrai ; il dit qu'il est encore **au-dessus du seuil**. Angle mort **A162**.

### Et ce que l'oracle croisé n'a pas trouvé

`ADR-043` §3 l'annonçait comme un détecteur de **fautes d'implémentation** : indice décalé, signe
inversé, condition de bord mal posée. **Il n'en a trouvé aucune.** Les deux implémentations
concordent partout où elles se recouvrent, ce qui est en soi le résultat le plus rassurant qu'ait
reçu ce code.

Ce qui a servi n'est donc pas la comparaison des deux codes, mais **le fait de rejouer des chiffres
publiés**. Ce n'est pas ce qui était prévu, et cela ne s'oppose pas à `ADR-043` : cela ajoute un
second usage à un instrument qui n'en annonçait qu'un.

### Une erreur de lecture, et ce qu'elle a appris

La première mesure de C03 a donné **161,1 périodes** contre 43,1 publiées, et la thèse a semblé
tomber. Elle ne tombait pas : **`ADR-040` §5 publie deux colonnes**, ordre un et ordre deux, et le
chiffre 43,1 est celui de l'ordre un — tandis que le montage courant est à l'ordre deux, qui donne
exactement 161,14.

Rien n'était faux dans le code. Ce qui a induit en erreur, c'est que **43,1 circule dans `ADR-039`
et dans `CAS-CANONIQUES` sans mention du schéma**, alors que le tableau qui le produit en distingue
deux. *Une demi-vie est un couple (schéma, maille), pas un nombre* — la même forme qu'**A153** pour
l'ordre d'un schéma.

Le test a été refait sur les **quatre** valeurs du tableau plutôt que sur une : elles se
reproduisent toutes à 0,00 %.

### Un résultat que la session ne cherchait pas

C05 — le seul des six que cette lignée n'avait jamais exécuté — a son montage de référence à 3 200
mailles × 90 s, une vingtaine d'essais : dix minutes en debug. Un montage **réduit** a donc été
écrit pour que le mécanisme soit exercé à chaque commit : quatre fois plus court, moitié moins de
longueur d'onde, maille double.

Il rend **7,0414 %** au réglage d'`ADR-005 §2`, contre **7,0 %** publié sur le montage de
référence — deux montages qui n'ont **aucune dimension en commun**, à 0,6 % l'un de l'autre.

C'est la **réserve n° 2 d'`ADR-042` §6 vérifiée sans qu'on la cherche** : *le groupe sans dimension
`σ_max·L_s/c` se transpose, la valeur de `σ_max` en s⁻¹ ne se transpose pas.* Elle était écrite
comme une supposition.

### Chiffres qui ont orienté la session

- **12,5 s sur 31 s** : le coût du second véhicule dans le mode `physics`, rapporté par le harnais
  lui-même. Un second jeu de montages est exactement le genre d'ajout qui grignote un budget sans
  qu'on le voie (**A158**).
- **×4,69** : le gain de l'ordre deux sur l'erreur `L¹` de C04, à maille égale.
- **6,01 contre 44,36** périodes à 100 mailles/λ : le même cas, la même maille, deux schémas, deux
  verdicts opposés autour du minorant de 15.

### Ce qui n'a pas été fait

- **L'oracle croisé n'est toujours pas *exercé* au sens strict** (action **S35-3**) : les deux
  solveurs tournent dans le même binaire, mais **aucun cas ne compare leurs deux sorties**. Ce qui
  a été fait est plus faible — vérifier que chacun retrouve ses propres chiffres publiés.
- **C05 n'est pas dans le mode `physics`**, seulement dans les tests. Son montage de référence
  dépasserait le budget de `SPEC-003 §1` à lui seul.
- **Les cinq angles morts de sévérité 1 importés en S35 n'ont toujours pas été relus** (S35-5).
- **Le journal de la lignée B n'est pas reporté** (S35-4), ni la lignée S08–S17 traitée (S35-6).

### Session suivante recommandée

**S37 — exercer l'oracle croisé** (S35-3) : un cas qui exécute `delta.rs` et `shallow.rs` sur le
même montage et compare leurs sorties, plutôt que leurs verdicts. C'est ce que `ADR-043` §3 promet
et que personne n'a encore fait. Le premier candidat est C01, dont les deux véhicules donnent
l'arrondi machine — un désaccord y serait sans ambiguïté.

*Solutions de rechange* : relire les cinq angles morts de sévérité 1 (S35-5) ; ou les saturations de
modèle (S34-1), reportées deux fois.

### Arbitrages en attente

Inchangés. Cette session n'en a tranché aucun et n'en a ouvert aucun.

---

## S37 — 2026-09-07 — L'oracle croisé exercé : il n'a pas trouvé de faute de calcul, il a trouvé un désaccord de vocabulaire

**Consigne reçue.** « Lance S37 ».

**Entrées.** Action **S35-3**, promise par `ADR-043` §3 et repoussée deux fois. S36 avait monté les
deux véhicules dans le même binaire, mais aucun code ne comparait leurs sorties.

**Sorties.** `code/water-harness/src/oracle.rs` — le comparateur, le plancher mesuré, six tests ;
[`ADR-044`](../docs/adr/ADR-044-ce-que-l-oracle-croise-peut-dire.md) ; angles morts **A163**
*(sévérité 1)* et **A164** ; note corrective datée sur `ADR-043` §7.2 ; leçons **L144–L146**.

**Ça tourne.** `cargo test` : **74 tests** au vert — 32 dans le cœur, 42 dans le harnais dont un
`ignore` — contre 68 en début de session. `water-harness check` : 0 échec, hashs inchangés.

### Ce que la session devait faire, et ce qu'elle a dû faire d'abord

Le plan annonçait deux volets. Le premier n'était pas prévu comme un résultat : **calibrer
l'instrument avant de le lire**.

`delta.rs` calcule en **`f32`**, `shallow.rs` en **`f64`**, et aucun document du corpus ne le disait.
Sur C01 au repos — solution exacte connue, `u ≡ 0` — chaque solveur a une erreur mesurable contre la
vérité :

| durée | `f32` | `f64` | rapport |
|---|---|---|---|
| 1 s | 1,88·10⁻⁶ | 9,48·10⁻¹⁶ | 2·10⁹ |
| 60 s | **4,40·10⁻⁶** | 2,72·10⁻¹⁵ | 1,6·10⁹ |

**Neuf ordres de grandeur**, et l'erreur `f32` **croît** avec le temps simulé.

> **Ce que cela dit à la conception.** « Bien équilibré » n'est pas binaire : en `f32`, la propriété
> vaut **4,4 µm/s après une minute**. Le seuil de C01 étant de 1 mm/s, la marge est de **×227** —
> confortable, et personne ne le savait. Un jeu de très grande échelle imposera vraisemblablement
> `f32` : **c'est ce chiffre-là le plancher réel de la couche δ**, pas celui que `shallow.rs`
> affiche. Angle mort **A164**.

### Le premier résultat : sur C01, l'oracle n'apprend rien

L'écart croisé vaut **exactement** le plancher, à chaque durée. C'est mécanique : `shallow.rs` est
si exact que `|u_delta − u_shallow|` se confond avec `|u_delta − 0|`, l'erreur que C01 mesurait déjà
seul.

> **Quand deux précisions diffèrent de neuf ordres de grandeur, la comparaison croisée dégénère en
> mesure d'erreur du moins précis.** Un oracle n'est symétrique que si les précisions le sont.

Ce n'est pas un échec — `ADR-043` situait sa valeur sur les cas **sans** référence analytique — mais
le §3 était trop optimiste, et il reçoit une note corrective datée.

*Un fait contraire à ce que le plan supposait* : les deux véhicules font **exactement le même nombre
de pas** sur C01 — 49, 482, 2891. Les `dt_cfl` calculés dans deux précisions ne divergent jamais
assez pour franchir une frontière de pas.

### Le deuxième résultat : sur C04, deux codes écrits sans se voir concordent à 0,065 %

C04 n'a pas de solution analytique **du schéma**. C'est le terrain de l'oracle — mais il a fallu
aligner d'abord : la lignée B monte C04 avec le flux **HLL**, `delta.rs` n'a que **Rusanov**.

| comparaison | écart `L∞` sur la hauteur | rapporté à `h₀` |
|---|---|---|
| **même flux** | 6,48·10⁻⁴ m | **0,065 %** |
| flux différents | 1,37·10⁻² m | 1,4 % — **×21** |

**Deux implémentations écrites indépendamment concordent à six pour dix mille sur un front de
rupture de barrage.** C'est le résultat le plus fort qu'ait reçu ce code, et rien d'autre que cet
oracle ne pouvait le donner.

### Le résultat de la session

**La vitesse, elle, diverge de 6,16 m/s sur une cellule — 98 % de la vitesse du front de Ritter.**
Son écart *moyen* reste à 7·10⁻² : le désaccord est concentré sur **trois cellules**.

La cause n'est pas un calcul, c'est un **mot** :

| | seuil de sec | où | justification |
|---|---|---|---|
| `delta.rs` | `10⁻⁶ m` | constante publique | `ADR-031` §5 |
| `shallow.rs` | `10⁻¹⁰ m` | **en dur**, à deux endroits | aucune |

Quatre ordres de grandeur. Une cellule entre les deux est sèche pour l'un, mouillée pour l'autre, et
`hu/h` sur un tel film rend n'importe quoi. **En écartant les cellules litigieuses, l'écart retombe
de 98 % à 1,8 % de `2c₀`** — la démonstration est un test, pas une affirmation.

Et sous ce désaccord, un second : à la cellule fautive, `delta.rs` porte **zéro exactement**,
`shallow.rs` un film de **1,05·10⁻¹⁰ m** — moins qu'un atome. Le premier porte sur la *lecture* de
la vitesse, celui-ci sur ce que le schéma *laisse derrière le front*.

> **Les deux lignées avaient identifié la question et y avaient répondu différemment sans le
> savoir.** `ADR-031` s'intitule *« une position de front n'existe pas sans seuil »* ; **A150**, de
> la lignée B, dit *« la position d'un front dépend du seuil qui la définit »*. Chacune avait
> raison, chacune était cohérente avec elle-même, et **aucun test des deux côtés ne pouvait le
> voir** : un test vérifie une cohérence interne, jamais une convention partagée. Angle mort
> **A163**, sévérité 1.

### Ce que la session n'a pas décidé, et pourquoi

**Le seuil de sec du projet n'est pas tranché.** Aligner les deux valeurs déplace la position du
front, donc le verdict de C04, donc le critère d'entrée au banc B3 d'`ADR-031`. Ce serait changer
une décision par une retouche de constante. La question va aux points ouverts.

### Chiffres qui ont orienté la session

- **×227** : la marge de `delta.rs` sur le seuil de C01, en `f32`.
- **×21** : ce que le seul changement de flux déplace — l'échelle qui donne son sens au 0,065 %.
- **3 cellules** sur 800 : l'étendue du désaccord de vitesse.
- **475 contre 454** pas sur C04, contre un nombre identique sur C01 : les précisions divergent là
  où le front sec fait varier `vmax`.

### Ce qui n'a pas été fait

- **C06 et C08 n'ont pas été confrontés.** Ce sont les deux autres cas sans référence analytique du
  schéma, et l'oracle y est utile par construction.
- **Le résidu de `10⁻¹⁰ m` derrière le front n'est pas expliqué**, seulement constaté. C'est très
  probablement une **saturation de modèle** non comptée — action **S34-1**, reportée trois fois.
- **L'écart résiduel de 1,8 % de `2c₀`** hors zone litigieuse n'est pas attribué.
- **Les cinq angles morts de sévérité 1 importés en S35 ne sont toujours pas relus** (S35-5), ni le
  journal de la lignée B reporté (S35-4), ni la lignée S08–S17 traitée (S35-6).

### Session suivante recommandée

**S38 — les saturations de modèle** (S34-1, **A146**), qui devient la suite naturelle : le résidu de
`10⁻¹⁰ m` trouvé ici en est très probablement une, et l'action attend depuis quatre sessions. Elle
répond en même temps au §8.3 d'`ADR-044`.

*Solutions de rechange* : confronter C06 et C08 sur l'oracle ; ou relire les cinq angles morts de
sévérité 1 (S35-5).

### Arbitrages en attente

Inchangés. Cette session n'en a tranché aucun, et **elle a refusé d'en trancher un par effet de
bord** — le seuil de sec, qui appartient à la conception et non à une session de code.

---

## S38 — 2026-09-07 — Les saturations ne maquillent rien, et c'est pour cela qu'elles sont dangereuses

**Consigne reçue.** « Enchaîne sur S38 ».

**Entrées.** Action **S34-1**, angle mort **A146**, **reportés quatre fois** ; action **S37-3**.

**Sorties.** Les compteurs de saturation des deux solveurs — sans allocation, sans changer un bit de
résultat ; [`AUDIT-SATURATIONS-S38`](../docs/registres/AUDIT-SATURATIONS-S38.md) ;
[`ADR-045`](../docs/adr/ADR-045-la-saturation-est-un-detecteur-pas-un-filet.md) ; angle mort
**A165** ; **A146 requalifié** par note datée ; les compteurs exposés au rapport du mode `physics`.

**Ça tourne.** `cargo test` : **77 tests** au vert — 32 dans le cœur, 45 dans le harnais dont un
`ignore` — contre 74 en début de session. `check` : 0 échec, **hashs inchangés**. Mode `physics` :
**33,7 s** sur 60 s, dont 1,1 s pour le bilan des saturations.

### Le résultat tient en une phrase

**La saturation d'état n'est pas un filet : c'est un détecteur de divergence, et il était muet.**

### Ce qui était craint, et ce qui a été mesuré

`A146` soupçonnait un solveur qui produirait des états impossibles à chaque pas derrière un
`h.max(0)` complaisant — le lit sec de C04 était nommément visé.

| cas | pas (`f32` / `f64`) | déclenchements |
|---|---|---|
| C01 — repos, 60 s | 2 891 / 2 891 | **0** |
| C03 — seiche, 20 périodes | 17 866 / 35 759 | **0** |
| **C04 — lit sec**, 2 s | 475 / 454 | **0** |

**Zéro, partout, y compris là où le soupçon portait.** Rusanov préserve la positivité sous sa
condition de Courant, et `0,45` est largement dedans.

### Mais une saturation jamais vue mordre n'a pas été testée

Deux lectures restaient indiscernables : soit le schéma ne produit jamais d'état impossible, soit la
condition est écrite de telle façon qu'elle ne peut pas être vraie. Le test d'une saturation est **le
cas qu'elle doit attraper** (L119) ; le levier est physique — au-delà de sa condition de Courant,
Rusanov cesse de préserver la positivité.

| `CFL` | déclenchements (`f32` / `f64`) | masse créée / volume | qdm |
|---|---|---|---|
| **0,95** | **0 / 0** | 0 | 0 |
| **1,20** | **267 / 11 252** | **1,4·10¹⁷** | `NaN` |

**Elle fonctionne, elle a son témoin, et la frontière tombe exactement sur la condition de Courant**
— retrouvée par un compteur qui ne la connaît pas.

### Ce qu'elle fait quand elle mord : rien de bon

Masse créée à **10¹⁷** fois le volume côté `f32`, **10¹⁵⁰** côté `f64` ; quantité de mouvement
débordant vers `NaN` ; nombre de pas multiplié par 69 pour la même demi-seconde, parce que `dt_cfl`
se recalcule sur des vitesses divergentes.

> **Elle ne convertit pas une instabilité en résultat acceptable.** Elle convertit une divergence
> franche — qui aurait produit des `NaN` visibles et arrêté tout le monde — en une suite de nombres
> finis qui ressemblent à un résultat. Le danger n'est donc pas une saturation *fréquente* en régime
> nominal : c'est **une seule** en régime dégradé, que rien ne signalait.

`A146` est requalifié, pas retiré : il avait raison de s'inquiéter et tort sur la raison. Le compteur
est désormais exposé, et **un déclenchement est un échec du cas**, pas un avertissement.

### Trois prédictions écrites avant la mesure, trois fausses

| prédiction | mesure |
|---|---|
| C04 déclenche, localisé au front | **jamais** |
| S5 (bord) déclenche une fois par pas sur C01 | **jamais** — `h[n+1] ≈ 0,99 m`, l'analyse était naïve |
| `delta.rs` n'a aucune cellule sous son propre seuil de sec | **trois** |

Les trois se trompent de la même façon : elles supposent qu'un test conditionnel gouverne plus qu'il
ne gouverne. Chacune a coûté une ligne de mesure et rapporté un fait que personne n'aurait cherché.

### Le résidu de S37 : la saturation est mise hors de cause

S37 avait trouvé un film de `1,05·10⁻¹⁰ m` derrière le front de `shallow.rs`, là où `delta.rs` porte
zéro. La saturation était le suspect naturel — c'est elle qui écrit des zéros. **Les compteurs sont à
zéro sur C04 : ce n'est pas elle.**

La cause est encore le seuil de sec (**A163**), et le chemin n'était pas celui qu'on croyait :

> **Un seuil de sec n'assèche pas une cellule : il l'empêche seulement de bouger.** Il coupe la
> **vitesse** — `u = 0` sous `h_sec` — et non le **flux de masse** : la diffusion de Rusanov,
> `α·(h_R − h_L)`, continue à déposer de la matière dans une cellule déclarée sèche, même quand les
> deux vitesses sont nulles.

`delta.rs` porte **3** cellules de film, `shallow.rs` **18** — le rapport qu'on attend de seuils
séparés par quatre ordres de grandeur. Angle mort **A165**.

### Ce que l'audit a trouvé par accident

**Le recensement écrit avait manqué un point de saturation** : l'étage intermédiaire de RK2 dans
`shallow.rs`. Il n'a été vu que parce que le compilateur a refusé l'appel restant après la
transformation de `saturer` en méthode.

*Un recensement à la lecture en manque.* Ce qui l'a rattrapé n'est pas une relecture plus attentive
mais un **changement de signature qui oblige chaque appel à se déclarer** — même conclusion que S34
sur G10.

### Chiffres qui ont orienté la session

- **0 / 0 / 0** : les déclenchements en régime nominal, sur trois cas et deux véhicules. C'est le
  chiffre qui a fait basculer la session de « mesurer une fréquence » à « chercher le cas qui
  déclenche ».
- **0,95 contre 1,20** : la frontière, et elle n'a pas été choisie — elle est tombée sur la condition
  de Courant.
- **6 890 pas au lieu de 100** pour la même demi-seconde à `CFL = 1,2`.
- **1,1 s sur 33,7 s** : ce que le bilan des saturations ajoute au mode `physics`, budget 60 s.

### Ce qui n'a pas été fait

- **S5, la saturation de bord, n'a pas de cas de déclenchement.** Sa condition est atteignable en
  principe — un bord presque sec sur fond montant — mais personne ne l'a exercée. C'est exactement
  la situation dont cette session vient de montrer qu'elle est indécidable sans test.
- **La conservation de la quantité de mouvement n'est mesurée par aucun cas canonique.** C01 mesure
  la dérive du volume ; rien ne surveille `hu`.
- **C04 n'a pas d'assertion de conservation**, contrairement à C01.
- **Le seuil de sec n'est toujours pas tranché** (A163, S37-1) — croisé trois fois cette session,
  touché zéro fois.
- **Les cinq angles morts de sévérité 1 importés en S35 ne sont toujours pas relus** (S35-5), ni le
  journal de la lignée B reporté (S35-4), ni la lignée S08–S17 traitée (S35-6).

### Session suivante recommandée

**S39 — trancher le seuil de sec** (**S37-1**, **A163**, sévérité 1). Trois sessions l'ont croisé
sans le toucher, et chacune a ajouté une raison de le décider : il déplace la position du front
(S37), il gouverne la longueur du film derrière lui (S38), et il sépare deux véhicules qui doivent
rester comparables. C'est un ADR, pas un patch.

*Solutions de rechange* : relire les cinq angles morts de sévérité 1 (S35-5) ; ou écrire le cas de
déclenchement de S5.

### Arbitrages en attente

Inchangés. Cette session n'en a tranché aucun.

---

## S39 — 2026-09-07 — Le fork a repris, et une conclusion importée a été mesurée fausse

**Consigne reçue.** « Reprends le projet ».

**Entrées.** L'amorce. `git worktree list` a montré un worktree neuf, `friendly-bhabha-6da427`, sur
`claude/reprise-projet-5134cd` — la branche de la lignée B, conservée en S35 pour son historique.
Une session y a travaillé : **B-S27, à 01h23**, contre ma S38 à **00h37**.

**Sorties.** [`ADR-046`](../docs/adr/ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md) importé ;
`dispersif.rs`, `eponge.rs`, `physics_dispersif.rs` ; leçons **L150–L152** reportées et
**L153–L155** écrites ; angles morts **A166–A168** ; la rétractation portée dans `ADR-042`,
`ADR-005`, `ADR-043` et `DOSSIER-B2` ; le **quatrième état du jeton**, `archivé` ; et le marqueur
écrit **des deux côtés**.

**Ça tourne.** `cargo test` : **84 tests** au vert — 38 dans le cœur, 46 dans le harnais dont deux
`ignore` — contre 77 en début de session. `check` : 0 échec, **hashs inchangés**.

### Le troisième fork, et ce qu'il apprend de neuf

Les deux premiers venaient d'une **ignorance** : personne ne savait que la branche voisine existait.
Celui-ci vient d'une **conservation délibérée**. En S35, la branche B avait été gardée exprès — elle
portait le seul historique de la lignée — et c'était le bon choix.

> **Une branche conservée pour son historique est un point de départ pour qui l'ouvre.** Aucune
> propriété de git ne sépare les deux ; seul un marqueur dans le contenu peut le faire, et il doit
> être lisible **avant** que le travail commence.

Les deux commandes d'amorce ont encore fonctionné : le fork a été vu au **premier geste**, pour la
troisième fois. C'est le seul dispositif du dépôt qui ait tenu trois fois.

### Ce que la session a dû faire avant tout le reste

**Mon corpus affirmait quelque chose qui venait d'être mesuré faux.** B-S27 s'intitule *l'éponge en
eau dispersive rétracte ADR-034* — et ADR-034 de la lignée B, c'est mon `ADR-042`, importé en S35.

C'est pourquoi le seuil de sec (S37-1), qui était la session recommandée, a été reporté d'un cran :
*un corpus qui affirme faux est plus urgent qu'un corpus qui laisse une question ouverte.*

### La réserve était fondée, et le document qui la portait s'en trouve grandi

`ADR-042` §6 posait sa propre réserve n° 1 en toutes lettres : *le solveur est non dispersif ; une
éponge d'eau profonde doit absorber une bande de célérités, et la règle `λ/2` protège peut-être
exactement de cela ; **c'est la première chose à mesurer**.*

Elle a été mesurée, dans un milieu à dispersion exacte écrit pour cela :

| `L_s/λ` | 0,125 | 0,25 | **0,5** | 1,0 | 2,0 |
|---|---|---|---|---|---|
| `R` | 0,669 | 0,515 | **0,227** | 0,0098 | 0,00144 |

**La règle `L_s ≥ λ/2` d'ADR-005 est du bon genre et de la mauvaise constante** : elle donne 23 %
pour un critère à 1 %. `ADR-046` retient **`L_s ≥ λ_δ`**, et **`L_s ≥ 2·λ_δ` dès que δ porte un
spectre** — le cas normal. La borne haute de `λ_cut` est **refermée, et deux à quatre fois plus
serrée qu'avant** : l'éponge coûte plus cher, pas moins.

Deux choses de plus, tranchées au passage : le réglage `σ_max = 10·c/L_s` d'`ADR-042` D1 est
**confirmé** — c'est un minimum franc en dispersif, pas un plancher — et le `c` ambigu d'ADR-005 est
la vitesse de **groupe**, ce qui vaut un facteur deux.

> **`ADR-042` n'est pas déshonoré par sa rétractation, il est validé comme instrument.** Il disait
> exactement quoi mesurer pour l'infirmer, et il a suffi de le lire. **L155**.

### Le résultat propre à cette session : `ADR-043` D1 avait une voie de trop

La rétractation force à relire ce qui s'appuyait dessus. `ADR-043` D1 rouvrait la borne de `λ_cut`
**par deux voies indépendantes** : l'absorbeur mesuré (`ADR-042` D4) et la dissipation qui rend le
masque inutile (`ADR-037`).

La voie 1 est rétractée. **Et la voie 2 était mal fondée depuis le début.** La borne de
`DOSSIER-B2 §3.1` vient de `L_s = λ_cut/2`, la largeur exigée par l'**absorbeur de bord** ;
`ADR-037` retire sa raison d'exister au **masque de décroissance**. Ce sont les deux objets que le
**§5 du même document** sépare, en mettant en garde : *tant qu'ils partagent le mot, un résultat sur
l'un se lit comme un résultat sur l'autre.*

**Le §6 a commis, trois paragraphes plus bas, l'erreur exacte contre laquelle son §5 mettait en
garde.** Angle mort **A168**. Il n'y avait qu'une voie, et elle est refermée.

### Le procédé, pour la troisième et dernière fois

**L137** — *un correctif de procédure écrit dans une seule branche ne protège que cette branche* — a
été écrite en S35. Le troisième fork s'est produit **pour exactement cette raison**, quatre sessions
plus tard : le `CLAUDE.md` corrigé vivait d'un seul côté.

L'action **S35-7** disait quoi faire. Elle était portée par « l'utilisateur », c'est-à-dire par
personne au moment où il fallait agir.

Cette fois le marqueur est écrit **des deux côtés, le jour même** :

| côté | ce qui a été écrit |
|---|---|
| lignée vivante | le quatrième état du jeton, **`archivé`**, dans `REPRISE.md` et `CLAUDE.md` |
| **lignée B** | jeton `archivé`, encadré en tête de son `REPRISE.md`, et **l'amorce qui lui manquait depuis S35** — commit `5d9bf2f` |

La branche B reste entière : rien n'est supprimé, son historique est son seul rôle. Mais **elle dit
maintenant ce qu'elle est** à qui l'ouvre.

### Les chiffres rejoués

L'instrument d'abord — une mesure faite avec un instrument non vérifié ne vaut rien :

| `λ` | `T` mesurée ici | `T = 2π/ω` | écart |
|---|---|---|---|
| 32 m | 4,52894 s | 4,52897 s | **0,0005 %** |
| 16 m | 3,20125 s | 3,20122 s | **0,0009 %** |
| 8 m | 2,26361 s | 2,26360 s | **0,0004 %** |

Puis les deux `R` : **0,2306** et **0,003885**, contre 0,227 et 0,00393 publiés — à 1,6 % et 1,2 %.

**Et le piège de L142 s'est représenté.** Le tableau `L_s/λ` de la note d'`ADR-005` est en bande
**étroite** ; ce montage mesure en bande **large**. Comparer 0,2306 à 0,227 revenait à confronter
deux conditions différentes. *Un chiffre cité hors du tableau qui le produit perd ce qui le rend
vrai* — leçon écrite en S36, piège retrouvé trois sessions plus tard sur un autre document.

### Ce qui n'a pas été fait

- **Le seuil de sec n'est toujours pas tranché** (S37-1, A163, sévérité 1). Quatre sessions l'ont
  maintenant croisé.
- **Les montages dispersifs ne sont pas dans le mode `physics`**, seulement dans les tests, dont un
  `ignore`. Le budget est à 33,7 s sur 60 ; côté B, la batterie **dépassait** le sien — 167 s pour
  120 (**A158**, **L152**). Le même arbitrage n'a pas été repris avec le code.
- **Deux spécifications ont divergé sans que personne le décide** : le budget de la batterie vaut
  60 s ici et 120 s côté B. Signalé, pas tranché.
- **Les angles morts de sévérité 1 importés ne sont pas relus** — ils sont maintenant **sept**
  (S35-5, plus A166 et A167).

### Session suivante recommandée

**S40 — trancher le seuil de sec** (S37-1, A163). Quatre sessions de suite l'ont croisé sans le
toucher, et chacune a ajouté une raison : il déplace la position du front, il gouverne la longueur
du film derrière lui, il sépare deux véhicules qui doivent rester comparables.

*Solutions de rechange* : relire les sept angles morts de sévérité 1 (S35-5) ; ou réconcilier les
deux budgets de batterie.

### Arbitrages en attente

Inchangés côté conception. **Un arbitrage d'infrastructure est en revanche résolu** : la branche B
est marquée archivée des deux côtés, et le mécanisme qui a produit trois forks porte enfin un
marqueur lisible avant le travail.

---

## S40 — 2026-09-07 — Le seuil de sec ne décide de rien de publiable, et la question était mal posée

**Consigne reçue.** « Enchaîne sur S40 ».

**Entrées.** Action **S37-1**, angle mort **A163** *(sévérité 1)*, **reportée quatre fois**. Deux
valeurs de seuil coexistaient : `10⁻⁶` dans `delta.rs`, `10⁻¹⁰` dans `shallow.rs`.

**Sorties.** Le seuil de `shallow.rs` rendu **réglable** — il était en dur à huit endroits ; le
balayage sur sept décades et deux véhicules ;
[`ADR-047`](../docs/adr/ADR-047-le-seuil-de-sec-ne-decide-de-rien-de-publiable.md) ; **A163
requalifié**, **A169** ouvert ; note corrective datée sur `ADR-044` §7 ; `oracle.rs` cesse de
recopier deux constantes.

**Ça tourne.** `cargo test` : **85 tests** au vert — 38 dans le cœur, 47 dans le harnais dont deux
`ignore` — contre 84 en début de session. `check` : 0 échec, hashs inchangés.

### Deux mesures du corpus semblaient se contredire

C'est ce qui a donné sa forme à la session, et c'était visible avant de mesurer quoi que ce soit :

| | ce qui était mesuré | résultat |
|---|---|---|
| `ADR-031` §4 *(S23)* | position du front, `H_SEC` sur six décades | **0,25 point sur seize** — *« la valeur est libre »* |
| `ADR-044` §5 *(S37)* | vitesse dans une cellule du film | **6,16 m/s**, soit **98 % de `2c₀`** |

Les deux sont justes. **Elles ne parlent pas de la même grandeur.** La question n'était donc pas
*quelle valeur choisir* — `ADR-031` avait déjà répondu — mais **de quoi ce seuil décide**.

### La mesure

Sept décades, `10⁻³` à `10⁻¹⁰`, sur les deux véhicules, C04 à `t = 2 s` :

| | `delta.rs` (`f32`) | `shallow.rs` (`f64`) |
|---|---|---|
| **front** | 9,5416 → 9,5274 m — **0,148 %** | 9,52500 → 9,52500 — **0,000 %** |
| **`h(0)`** | 0,45165 → 0,45166 | 0,45118 → 0,45119 |
| **volume** | 20,000000 partout | 20,000000 partout |
| **film** | 3 → **15** cellules | 7 → **18** cellules |
| **`max\|u\|`** | **5,18 à 11,06 m/s** | **5,10 à 12,86 m/s** |

**Toutes les grandeurs publiées de C04 sont insensibles** — 0,148 % au pire pour une tolérance de
3 %. La thèse déclarée avant la mesure tient.

**Et `max|u|` dépasse `2c₀ = 6,264 m/s`**, la vitesse du front de Ritter, la plus grande que cette
solution contienne. Elle varie d'un facteur 2,1 à 2,5 avec le seuil, **sans tendance monotone**.

> **Une quantité qui dépasse la borne physique de son propre montage et qui varie sans tendance avec
> un réglage arbitraire n'est pas une mesure.** C'est `hu/h` sur un film dont l'épaisseur est un
> paramètre.

### La décision, et pourquoi elle n'est pas un nombre

**Les deux valeurs ne sont pas alignées.** Aligner coûterait la reproductibilité de tous les chiffres
publiés par la lignée B, mesurés à `10⁻¹⁰`, et n'achèterait rien puisque l'écart ne déplace aucune
grandeur publiable. *Une différence sans conséquence se documente au lieu de se corriger.*

Ce qui est décidé à la place : **`u` dans une cellule sous le seuil n'est pas une grandeur
publiable** — aucune assertion, aucun rapport, aucun oracle ne doit la lire. La règle vaut au-delà de
ce seuil : *une grandeur définie par une division dont le dénominateur est un réglage n'est pas
mesurable.*

### Ce qui justifiait la sévérité 1, et qui demeure

**Ce n'était pas la différence, c'était l'ignorance.** Un des deux véhicules portait une constante
posée au jugé, en dur, à huit endroits, sans que rien ne dise ce qu'elle commandait.

> *Ce qui rendait ce seuil dangereux n'est pas qu'il valait `10⁻¹⁰` plutôt que `10⁻⁶`, c'est que
> personne ne pouvait dire ce qui changerait s'il valait autre chose.*

Le défaut est levé par la mesure, pas par un alignement. Le seuil est réglable, rapporté, et sa
provenance est ce balayage.

### Ce que la session a corrigé dans ce qu'elle a lu

`ADR-044` §7 refusait d'aligner les deux seuils, et donnait pour raison qu'un alignement
*« déplace la position du front, donc le verdict de C04, donc le critère d'entrée au banc B3 »*.
**La prudence était bonne ; sa justification était fausse** — le front bouge de 0,148 %.

Et cela a coûté : **l'action a été reportée quatre fois**, parce que le prix annoncé — rouvrir un
critère de banc — la faisait paraître plus lourde qu'elle n'était. Angle mort **A169** : *une raison
fausse donnée à l'appui d'une bonne décision la rend indéfendable au moment de l'exécuter, parce que
personne ne peut estimer ce qu'elle coûte.*

### Ce que la session a dû faire pour pouvoir mesurer

Le seuil de `shallow.rs` était **en dur à huit endroits**, dont cinq dans des fonctions statiques.
Le rendre réglable a demandé de propager un paramètre à travers `flux_physique`, `rusanov`, `hll`,
`flux_num`, les deux reconstructions et `residu`. **Le compilateur a énuméré chaque point** — c'est
**L149**, et c'est la deuxième fois en trois sessions qu'un changement de signature trouve ce qu'une
lecture aurait manqué.

Les 84 tests sont restés verts après la propagation : la valeur par défaut est inchangée, et le
comportement aussi.

### Chiffres qui ont orienté la session

- **0,148 %** contre une tolérance de **3 %** : le rapport qui rend la décision facile.
- **12,86 m/s** pour une borne physique de **6,26** : ce qui prouve que `max|u|` n'est pas une
  grandeur.
- **Huit endroits en dur**, dont cinq inaccessibles à `self` : le coût réel de la mesure.
- **Quatre reports** : ce qu'a coûté une bonne prudence mal justifiée.

### Ce qui n'a pas été fait

- **Le seuil de mesure du front** — `10⁻²·h₀` — n'est pas celui-ci et n'a pas été réexaminé.
  `ADR-031` §5.3 demandait qu'il soit *fixé par C04 lui-même* (**A154**). Toujours ouvert.
- **`hu` dans le film** n'a pas été regardé. Si `u` n'y est pas publiable, `hu` non plus.
- **`max|u|` sert de borne interne** au pas de temps (`ADR-035`). Le §2 dit qu'elle n'est pas
  publiable ; il ne dit pas qu'elle est inutilisable comme borne, et la différence n'est pas
  instruite.
- **Les sept angles morts de sévérité 1 importés ne sont pas relus** (S35-5, S39-3).
- **Les deux budgets de batterie divergent toujours** — 60 s ici, 120 s côté lignée B (S39-2).

### Session suivante recommandée

**S41 — relire les sept angles morts de sévérité 1 importés** (S35-5, S39-3). Ils sont entrés par
deux réconciliations, aucun n'a été examiné par cette lignée, et cette session vient de montrer sur
`A163` qu'un angle mort importé peut être **juste et mal formulé** : son énoncé désignait une
incompatibilité qui n'existe pas, alors que le défaut réel — l'absence de provenance — était à côté.

*Solutions de rechange* : le seuil de mesure du front (A154) ; ou réconcilier les deux budgets
(S39-2).

### Arbitrages en attente

Inchangés. Cette session a **tranché une question qui attendait depuis S37** sans avoir eu à choisir
un nombre, ce qui est le résultat le plus économique qu'elle pouvait produire.

---

## S41 — 2026-09-07 — Cinq angles morts importés sur sept désignaient un défaut présent ici

**Consigne reçue.** « Enchaîne sur S41 ».

**Entrées.** Actions **S35-5** et **S39-3**. Sept angles morts de sévérité 1 entrés par deux
réconciliations, aucun relu par cette lignée.

**Sorties.** [`AUDIT-ANGLES-IMPORTES-S41`](../docs/registres/AUDIT-ANGLES-IMPORTES-S41.md) ; **neuf
rubriques « Conditions de mesure »** reportées dans `CAS-CANONIQUES` ; deux tests de vérification
(`a157_*`, `adr_037_*`) ; notes correctives datées sur `ADR-037` et sur le tableau de S36 ;
`A157` complété ; leçons **L158–L160**.

**Ça tourne.** `cargo test` : **88 tests** au vert — 38 dans le cœur, 50 dans le harnais dont deux
`ignore` — contre 85 en début de session. `check` : 0 échec, hashs inchangés.

### La méthode, et pourquoi elle ne pouvait pas être une lecture

S40 avait montré le risque sur `A163` : un angle mort importé, de sévérité 1, dont l'énoncé désignait
une incompatibilité **qui n'existe pas**. Le fait était exact, le défaut nommé était à côté, et cela
a coûté quatre reports.

Chaque fiche a donc reçu quatre questions, dont **trois se vérifient** — la troisième étant *le
défaut existe-t-il ici, aujourd'hui, dans le code et le corpus d'accueil ?*

| | Q1 exact | Q2 bien nommé | **Q3 présent ici** | Q4 action |
|---|---|---|---|---|
| **A152** | oui | oui | **OUI — le remède manquait** | faite |
| **A155** | oui | oui | **latent** — désamorcé | faite |
| **A156** | oui | oui | non — déjà traité | — |
| **A157** | oui | **incomplet** | **OUI, et pire** | S41-1 |
| **A159** | oui | oui | **OUI** — deux arrondis | faite + S41-2 |
| **A166** | oui | oui | **OUI, structurel** | S41-3 |
| **A167** | oui | oui | **OUI** | S41-4 |

**Cinq sur sept.** La thèse déclarée avant la relecture en prévoyait deux ; elle sous-estimait.

### Le trou de la réconciliation elle-même

**A152 avait un remède, et il n'avait pas été importé.** La lignée B avait construit contre lui une
rubrique **Conditions de mesure** dans `CAS-CANONIQUES` — *tout paramètre dont dépend la valeur
mesurée et que le montage ne fixe pas*.

| | rubriques |
|---|---|
| lignée B | **neuf** |
| ici, avant S41 | **zéro** |

S35 et S39 ont importé le constat et laissé le dispositif. Le registre du fork listait les documents
modifiés ; il ne listait pas les **rubriques ajoutées à l'intérieur** d'un document déjà modifié des
deux côtés. Les neuf sont reportées.

### Le résultat de la session : une attribution fausse de moitié

`CAS-CANONIQUES` porte depuis S36 un tableau où C04 **échoue** sur un véhicule et vaut **0,74 %** sur
l'autre, avec cette explication : *la lignée B est passée à l'ordre deux, et C04 comme C08 sont
passés au vert ensemble.*

**Trois choses changent entre les deux colonnes, et une seule était nommée** : le schéma, mais aussi
le **seuil** de mesure du front (`10⁻³ m` contre `10⁻²·h₀`) et la **référence** (front ponctuel de
Ritter contre front moyenné sur la maille).

Mesuré à **ordre un des deux côtés**, donc à schéma égal :

| même solveur, ordre un | écart au front |
|---|---|
| mesure d'accueil | **20,4 %** |
| mesure de la lignée B | **10,2 %** |
| *(publié, ordre deux, mesure de B)* | *0,74 %* |

**La révision de la mesure retire dix points sur vingt ; l'ordre deux retire les neuf et demi qui
restent** — 52 % contre 48 %. Aucun des deux seul ne franchit la tolérance de 3 %.

Et à **seuil égal**, les deux véhicules donnent le même front **à la quatrième décimale** : 9,9750 m
à `10⁻³`, 9,5250 m à `10⁻²`. *Le désaccord n'était pas entre les solveurs.*

### A159 exercé : deux broutilles, et c'est un bon résultat

La fiche prescrivait de *refaire une fois les formules dont dépend une décision, avec leurs
constantes*. Personne ne l'avait fait ici. `ADR-037` §3, qui dimensionne δ pour les transitoires :

- **la dérivation est juste** — `dx ≤ K·L^1,5/√(2h)`, et `g` disparaît bien ;
- **sept valeurs sur neuf sont exactes** ;
- **deux sont mal arrondies** : `65,5` pour 65,43 et `÷6,1` pour 6,16.

Aucune ne change la conclusion. **C'est le sujet** : *une vérification qui ne trouve que des
broutilles est une vérification qui a réussi, et elle ne pouvait pas le dire avant d'avoir été
faite.* Le calcul est désormais un test.

### Ce que A166 coûte réellement, chiffré pour la première fois

La fiche énonce une limite de méthode : *une mesure ne peut pas dire de quel cadre elle dépend.* La
relecture ajoute la **liste de ce qui est exposé ici** — toute conclusion mesurée sur `delta.rs` ou
`shallow.rs` l'a été en **1D, non dispersif, Saint-Venant, sans friction** : `ADR-037`, `ADR-044`,
`ADR-045`, `ADR-047`. Aucune n'est fausse ; **aucune ne peut dire qu'elle vaut au-delà**.

### A167 : un seul essai à zéro, et c'est celui qui a été importé

*Tout montage doit venir avec un essai dont le résultat attendu est zéro.* Inventaire : `B-S27-garde`
est le seul. `C05-temoin` attend 1, `C01-jet` et `C04-jet` attendent un échec. C03, C06, C08 et C02
n'ont rien — et C03 est le plus exposé, sa demi-vie venant d'une régression sur une enveloppe.

### Deux erreurs commises en séance, et ce qu'elles ont appris

- **J'ai comparé les deux fronts avec un seul barrage**, alors que les origines diffèrent de 20 m —
  ce que l'en-tête d'`oracle.rs` dit explicitement. L'écart de 20 m m'a sauté aux yeux ; s'il avait
  valu 2 cm, il serait passé.
- **J'ai corrompu l'encodage d'`oracle.rs`** avec un `unicode_escape` mal placé, et j'ai dû restaurer
  depuis git. Deux minutes perdues, aucune conséquence — le fichier était committé.

### Ce qui n'a pas été fait

- **Les autres formules à constantes** n'ont pas été refaites — `ADR-033`, `ADR-036`, `ADR-046`
  (S41-2).
- **La réserve de cadre n'est pas portée** dans les quatre ADR exposés (S41-3).
- **Aucun essai à zéro n'a été écrit** pour C03, C06, C08 (S41-4).
- **Les deux budgets de batterie divergent toujours** — 60 s ici, 120 s côté lignée B (S39-2).
- **La distinction absorbeur / masque / transducteur** n'a pas été passée sur les conclusions déjà
  écrites (S39-1).

### Session suivante recommandée

**S42 — écrire l'essai à zéro de C03** (S41-4). C'est le montage le plus exposé au motif d'A167, la
mesure la plus indirecte du corpus, et l'action la plus courte des quatre ouvertes.

*Solutions de rechange* : refaire les formules restantes (S41-2) ; ou porter la réserve de cadre
(S41-3).

### Arbitrages en attente

Inchangés.

---

## S42 — 2026-09-07 — C03 déclarait le néant conforme, avec le meilleur score possible

**Consigne reçue.** « Enchaîne sur S42 ».

**Entrées.** Action **S41-4**, angle mort **A167** : *tout montage de mesure doit venir avec un essai
dont le résultat attendu est zéro.* L'inventaire de S41 en avait trouvé **un seul** dans tout le
harnais, et c'était celui que la lignée B avait apporté.

**Sorties.** L'essai à zéro de **C03** et celui de **C06** ; deux refus dans la mesure de demi-vie,
avec leur témoin ; **une seule implémentation** de la régression là où il y en avait deux ; angle
mort **A170** *(sévérité 1)* ; `A167` relu ; conditions de mesure de C03 et C06 complétées.

**Ça tourne.** `cargo test` : **93 tests** au vert — 38 dans le cœur, 55 dans le harnais dont deux
`ignore` — contre 88 en début de session. `check` : 0 échec, hashs inchangés. Le chiffre publié
**161,138607 périodes** est intact.

### La thèse était fausse, et le vrai défaut était plus bas

*Thèse déclarée : `demi_vie_seiche` rend un nombre fini et plausible sur un bassin au repos.*

Elle rend **`INFINITY`** — le filtre `pic > 0` ne laisse passer aucun point, la régression sur zéro
point rend `NaN`, et `NaN < 0` est faux. Un refus, mais **par accident**.

**Le défaut était juste après.** `c03_seiche` sature cette valeur pour l'affichage :

```rust
mesure: demi_vie_periodes.min(1e6)
```

**`f64::min` avale les `NaN`** : `NaN.min(1e6)` rend `1e6`. Le refus devenait donc **la plus grande
valeur du domaine**, face à un minorant de 15 périodes — c'est-à-dire **le meilleur score
possible**.

| C03 sur un bassin **sans seiche**, avant S42 | |
|---|---|
| `C03-T` | `NaN` — échoue |
| `C03-T-mur` | `NaN` — échoue |
| **`C03-demi-vie`** | **`10⁶` — PASSE** |

Le cas entier était rouge, les deux autres assertions y veillaient. Mais **l'assertion qui porte le
résultat publié déclarait le néant excellent**, et le rapport affichait `1000000` là où la mesure
valait `NaN`. Angle mort **A170**.

### Le refus, et une correction qui n'en était pas une

Deux refus ajoutés, **dérivés et non choisis** — le corpus a déjà payé un seuil posé au jugé
(**A157**) :

1. **moins de trois points** — une régression sur deux points passe exactement par eux ;
2. **une amplitude sous `ε·h₀`** — `η` ne peut pas varier moins qu'un ulp de la hauteur d'eau sans
   que la variation soit un artefact. C'est le pendant `f64` de **G5**, que le véhicule d'accueil
   porte depuis S27 et que celui-ci n'avait pas.

**Le refus est `NaN`, pas une valeur de repli** — une valeur de repli aurait été relue comme une
mesure, ce qui est précisément le défaut qu'on corrige.

> **Et il n'a d'abord rien corrigé.** Après l'avoir écrit, le cas continuait de déclarer le néant
> conforme. La régression était écrite **deux fois dans le même fichier** — une fois en ligne dans
> `c03_seiche`, une fois dans `demi_vie_seiche`, la fonction extraite en S36 précisément pour être
> testable. **J'avais corrigé celle qui alimente le tableau, pas celle qui alimente l'assertion.**
> Les deux sont désormais la même fonction.

### Le témoin

*Le test d'un garde-fou est le cas qu'il doit refuser, et il lui faut aussi son témoin* (**L119**).

| | attendu | obtenu |
|---|---|---|
| bassin sans seiche | refus | **`NaN`** aux deux ordres |
| bassin excité, ordre deux | pas de refus, valeur publiée | **161,14 périodes** |

Un refus mal placé aurait fait disparaître le chiffre du corpus sans que rien ne le dise.

### C06 passe son essai à zéro, exactement

Le même montage **sans boost** (`u₀ = 0`) : les deux simulations comparées sont alors **le même
calcul**, et l'écart doit être **nul**, pas petit. Mesuré : **`0,0`** sur les deux assertions.

C'est le meilleur genre d'essai à zéro — il exerce une **identité**, pas une tolérance.

> *Un essai à zéro qui réussit du premier coup n'est pas du travail perdu : c'est la seule façon de
> distinguer un montage sain d'un montage jamais interrogé.*

### Chiffres qui ont orienté la session

- **`NaN.min(1e6) = 1e6`** : une ligne de langage, et tout le défaut.
- **2 implémentations** de la même régression dans le même fichier ; **1** après.
- **161,14 périodes** avant et après : le refus ne déplace aucun chiffre publié.
- **0,0** exactement pour C06 : ni tolérance ni interprétation.

### Ce qui n'a pas été fait

- **C08 et C02 n'ont pas d'essai à zéro.** C08 est le moins évident des quatre : l'essai naturel —
  une solution représentée exactement, dont l'erreur serait nulle à toutes les grilles — rendrait un
  ordre **indéfini**, pas zéro. Il demande d'être pensé, pas seulement écrit.
- **Les autres formules à constantes** ne sont pas refaites (S41-2).
- **La réserve de cadre** n'est pas portée dans les quatre ADR exposés (S41-3).
- **Les deux mesures de front ne sont toujours pas comparables** (S41-1).
- **Les deux budgets de batterie divergent** — 60 s ici, 120 s côté lignée B (S39-2).

### Une erreur de conduite, à noter

**J'ai committé un message qui décrivait plus que ce que le commit contenait.** Un script a échoué à
mi-parcours ; `CAS-CANONIQUES` était écrit, les angles morts non, et j'ai committé sans le vérifier.
Le commit suivant porte `(correctif)`. Le défaut n'est pas dans le script — il est d'avoir fait
confiance à un enchaînement dont une étape avait affiché une exception.

### Session suivante recommandée

**S43 — l'essai à zéro de C08** (S41-4, reliquat). C'est le seul des quatre qui demande de
**concevoir** ce que « résultat attendu zéro » veut dire pour une mesure d'ordre de convergence.

*Solutions de rechange* : refaire les formules restantes (S41-2) ; ou rendre les deux mesures de
front comparables (S41-1).

### Arbitrages en attente

Inchangés.

---

## S43 — 2026-09-07 — Un solveur qui ne converge pas recevait l'ordre 1

**Consigne reçue.** « Enchaîne sur S43 ».

**Entrées.** Action **S42-1** — *concevoir l'essai à zéro de C08*, le seul des quatre qui demandait
de penser ce que « résultat attendu zéro » veut dire pour une mesure d'**ordre de convergence**.

**Sorties.** L'essai à zéro de C08 sous ses trois formes ; `ordre_grossier_estime` rend désormais un
refus **typé** ; les trois estimateurs de Richardson ramenés à un ; un troisième volet au test de
**G10** ; angle mort **A171** *(sévérité 1)* ; conditions de mesure de C08 complétées.

**Ça tourne.** `cargo test` : **95 tests** au vert — 38 dans le cœur, 57 dans le harnais dont deux
`ignore` — contre 93 en début de session. `check` : 0 échec, hashs inchangés. **Aucun chiffre publié
ne bouge** : `C08-p = 0,999745`, et les trois ordres d'`ADR-040` §3 — 0,654 / 0,621 / 1,000.

### La réponse à la question que S42 laissait ouverte

*Que veut dire « résultat attendu zéro » pour une mesure d'ordre ?* **L'essai ne porte pas sur le
solveur, il porte sur l'estimateur.** Un montage sans objet à mesurer, ici, c'est une suite d'erreurs
qui ne converge pas : trois grilles, la même erreur. Il n'y a pas d'ordre, et l'estimateur doit le
dire.

C'est ce qui rend l'essai **immédiat** : il se joue sur des suites synthétiques, sans lancer une
simulation. `ordre_grossier_estime` avait été extraite en S34 précisément pour cela.

### Le résultat

L'estimateur est **exact** là où il y a un ordre :

| suite | rendu |
|---|---|
| `e ∝ dx¹` | **1,0000** |
| `e ∝ dx²` | **2,0000** |
| `e ∝ dx^0,5` | **0,5000** |

Et il rendait **1,0000** dans les **trois** cas où il n'y a rien à mesurer :

| | rendu avant S43 |
|---|---|
| erreur **constante** — le solveur ne converge pas | **1,0000** |
| moins de trois grilles — triplet incomplet | **1,0000** |
| erreurs toutes nulles | **1,0000** |

### Pourquoi c'est le pire cas possible d'A170

`1,0` n'est pas une valeur neutre : c'est **l'ordre nominal du schéma d'essai**, exactement ce qu'on
espère lire. Et le garde-fou **G10**, qui signale tout ordre hors de `[0,3 ; 3,0]`, ne pouvait pas
broncher — `1,0` est dedans.

> **Le repli était silencieux par construction**, et non par oubli de signalement : la valeur choisie
> pour « ne rien dire » était celle qui dit *tout va bien*. Angle mort **A171**.

C03, en S42, saturait à `10⁶` périodes — une valeur qui finit par paraître suspecte. **Une valeur de
repli à l'intérieur du domaine nominal ne paraîtra jamais suspecte à personne.**

Le chemin du défaut allait jusqu'au filtre : `p_brut` → `p_grossier` → `e_oracle = e_max/ratio^p` →
`retain(|e| e >= 30·e_oracle)`. Un ordre inventé déplace le seuil qui écarte les grilles.

### Ce que l'audit de S34 n'avait pas vu

S34 a audité G10, l'a trouvé défaillant — *le seul des dix qui masquait au lieu de refuser* — et a
corrigé son **bornage**. Le repli est sur la **ligne juste au-dessus du `clamp`** :

```rust
let brut = if d1 > 0.0 { (d0 / d1).log2() } else { 1.0 };
(brut, brut.clamp(0.3, 3.0))
```

L'audit a regardé le `clamp` et pas le `if`. Ses deux tests donnaient à l'estimateur une série
pré-asymptotique et une série saine — **jamais une série sans ordre du tout**. *Un garde-fou testé
sur ce qu'on a pensé à lui donner n'est pas un garde-fou testé.*

### La correction, et ce qu'elle a forcé à déclarer

Le refus est passé **dans le type** : `(Option<f64>, f64)`. Le compilateur a énuméré les usages —
c'est **L149**, pour la troisième fois en six sessions — et il en a trouvé un que je n'attendais pas :
le harnais portait **trois** estimateurs de Richardson, dont un troisième dans `physics_shallow.rs`
sans aucun garde. Il délègue désormais au partagé.

Le second membre reste un nombre — il faut bien filtrer — et il vaut `1.0`, ce qui est conservateur :
un `p` bas surestime l'erreur d'oracle, donc écarte **plus** de grilles. La différence est que
l'appelant **sait** maintenant, et le signale.

### Chiffres qui ont orienté la session

- **1,0000 trois fois** : le même repli pour trois formes distinctes de « rien à mesurer ».
- **3 estimateurs** de Richardson dans le harnais ; **1** après, plus l'ordre direct qui est une
  autre grandeur.
- **0,999745** et **0,654 / 0,621 / 1,000** : inchangés, ce qui était la condition de la correction.

### Ce qui n'a pas été fait

- **C02 n'a pas d'essai à zéro** (S42-1, reliquat).
- **Les autres valeurs de repli** n'ont pas été inventoriées (S42-2) — cette session en a traité une,
  trouvée par le chemin de C08, pas par une recherche.
- **Les autres formules à constantes** ne sont pas refaites (S41-2).
- **La réserve de cadre** n'est pas portée dans les quatre ADR exposés (S41-3).
- **Les deux mesures de front ne sont pas comparables** (S41-1).

### Session suivante recommandée

**S44 — inventorier les valeurs de repli placées après une mesure** (S42-2). Deux sessions de suite
en ont trouvé une par hasard, chacune sévérité 1 : `NaN.min(10⁶)` en S42, `else { 1.0 }` en S43. La
troisième ne devrait pas être trouvée par hasard. La recherche est mécanique — `min`, `max`,
`unwrap_or`, `clamp`, `else` d'un `if` de validité — et elle a un critère : *que devient un refus
qui passe là-dedans ?*

*Solutions de rechange* : l'essai à zéro de C02 ; ou refaire les formules restantes (S41-2).

### Arbitrages en attente

Inchangés.

---

## S44 — 2026-09-07 — Trois formes, une cause : quand la grandeur est un écart, zéro est le succès parfait

**Consigne reçue.** « Rajoute la possibilité d'utiliser aussi ChatGPT dans le projet. Enchaîne
ensuite sur S44. »

**Entrées.** Action **S43-1**, qui reprend **S42-2** : *inventorier les valeurs de repli placées
après une mesure.* Deux sessions de suite en avaient trouvé une par hasard, chacune sévérité 1.

**Sorties.** [`AUDIT-REPLIS-S44`](../docs/registres/AUDIT-REPLIS-S44.md) ; la fonction `deficit()`
qui sépare le minorant tenu du refus ; angle mort **A172** *(sévérité 1)*.

**Ça tourne.** `cargo test` : **96 tests** au vert — 38 dans le cœur, 58 dans le harnais dont deux
`ignore` — contre 95 en début de session. `check` : 0 échec, hashs inchangés. Aucun chiffre publié ne
bouge : 20,69 et 24,40 périodes, `R²` 0,9920 et 0,9998, `C08-p` 0,999745.

### Avant la session : l'amorce ouverte aux autres agents

Demandé par l'utilisateur, fait hors session, commit `faeb56c`. **Le dispositif ne dépendait déjà
d'aucun fournisseur** — il repose sur des fichiers versionnés et sur `git` — mais le fichier
d'amorce portait un nom de fournisseur, et chaque outil lit automatiquement un nom différent.

La tentation évidente était un second fichier d'amorce à côté du premier. **C'est exactement ce que
`FORK-S22-S26` interdit** : trois forks, dont le troisième parce qu'un correctif vivait dans une
seule branche (**L137**). Le même mécanisme, transposé des branches aux fichiers.

| | |
|---|---|
| **`AGENTS.md`** | **le texte**, seul endroit où l'amorce existe |
| `CLAUDE.md` | un renvoi de quatre lignes : *ne recopie rien ici* |

Et une ligne **`Agent`** au jeton — non pour discriminer un fournisseur, mais pour dire **quels
outils étaient disponibles** : un agent sans `cargo` n'a pas vérifié les tests, un agent sans `git`
n'a pas committé ses étapes. Cette session est la première à la renseigner. Voir `FORK-S22-S26` §9.5.

### La thèse était fausse sur la cible et juste sur le fond

*Thèse déclarée : au moins un `unwrap_or(0.0)` se trouve sur le chemin d'une grandeur publiée et y
transforme un refus en résultat parfait.*

**Les huit `unwrap_or(0.0)` sont innocents.** Deux sont sains — l'absence de paroi mobile *est* une
vitesse nulle, ce n'est pas un refus. Les six autres portent sur des positions de front : un front
introuvable y devient `0`, c'est-à-dire *au barrage*, ce qui produit un **écart de 100 %** contre une
référence à 10,6 m. Le cas échoue. Bruyamment.

**Les deux fautifs étaient ailleurs**, et le fond de la thèse s'y applique mot pour mot.

### Ce qui a été trouvé

`physics.rs` exprime les deux minorants de C03 sous forme de **déficit** — une formulation
excellente, et le commentaire qui l'accompagne dit pourquoi : un minorant s'exprime mal avec une
tolérance relative.

```text
déficit de demi-vie = max(0, 15 − mesure) / 15      référence 0, tolérance 0
déficit de R²       = max(0, 0,9 − mesure)          référence 0, tolérance 0
```

**Le `max(0, …)` qui rend cette forme juste est exactement ce qui avale les refus.**

| entrée | ce que ça veut dire | avant | après |
|---|---|---|---|
| `+∞` | le schéma n'amortit pas | déficit **0** — passe | **0** — passe, et c'est juste |
| `NaN` | il n'y avait **rien à mesurer** | déficit **0** — **PASSE** | **`NaN`** — échoue |

### Le résultat de la session

Trois défauts en trois sessions, trois formes, **une seule cause** :

| | forme | valeur rendue | position dans le domaine |
|---|---|---|---|
| S42 | `NaN.min(10⁶)` | `10⁶` | **hors** du plausible |
| S43 | `else { 1.0 }` | `1,0` | **dans** le nominal |
| S44 | `(15 − NaN).max(0)` | `0` | **le meilleur point** |

> **La gravité croît et la visibilité décroît dans le même ordre.** Un `10⁶` finit par se faire
> remarquer ; un `1,0` au milieu des ordres attendus, jamais ; un `0` sur un déficit **est le
> résultat qu'on espère**.

Et le motif ne tient pas au repli mais à la **grandeur** : dès qu'une mesure est un écart, une erreur
ou un déficit, **son domaine contient zéro et zéro en est le meilleur point**. Toute opération capable
de produire zéro à partir d'un refus le transforme en succès parfait. Angle mort **A172**.

### Le corollaire, qui est ce que l'audit a coûté à trouver

**Un repli ne s'inspecte jamais seul.** Les treize `unwrap_or(NaN)` du harnais sont irréprochables —
`NaN` échoue toute comparaison — et **ils ont produit les trois défauts**, parce que `min`, `max` et
une soustraction suivie d'un `max` avalent tous le `NaN`.

*C'est l'aval qu'il faut suivre, jusqu'à l'assertion ou l'affichage.* Un inventaire des replis qui se
serait arrêté aux replis n'aurait rien trouvé.

### Chiffres qui ont orienté la session

- **25 `unwrap_or` et 24 `min`/`max`** : la taille de l'inventaire, et ce qui rendait la recherche
  faisable en une session.
- **8 suspects, 0 fautif** : la thèse, démentie sur sa cible.
- **2 fautifs**, tous deux sur des assertions publiées de C03.
- **0 chiffre déplacé** : la condition de toute correction de ce genre.

### Ce qui n'a pas été fait

- **`front_mouille` rend toujours zéro** quand le front est introuvable, plutôt que de refuser. Un
  front à zéro n'est pas un front au barrage (S44-1).
- **Les treize `unwrap_or(NaN)` n'ont pas tous été suivis** jusqu'à leur assertion — trois l'ont été,
  parce que trois défauts y menaient (S44-2).
- **L'essai à zéro de C02** reste à écrire (S43-3).
- **Le troisième cas des contrôles** — l'entrée vide de ce qu'ils examinent — n'est pas généralisé
  (S43-2).
- **Les autres formules à constantes** ne sont pas refaites (S41-2), la réserve de cadre n'est pas
  portée (S41-3), les deux mesures de front ne sont pas comparables (S41-1).

### Session suivante recommandée

**S45 — suivre les treize `unwrap_or(NaN)` jusqu'à leur assertion** (S44-2). C'est la seule des
actions ouvertes dont cette session ait montré qu'elle trouve quelque chose : trois sur treize ont
été suivis, et les trois menaient à un défaut de sévérité 1.

*Solutions de rechange* : faire refuser `front_mouille` (S44-1) ; ou l'essai à zéro de C02 (S43-3).

### Arbitrages en attente

Inchangés.

## S45 — 2026-09-07 — Un refus peut supprimer son assertion

**Agent : Codex**, git et cargo disponibles. Reprise demandée par l'utilisateur ; travail sur
master, après contrôle des trois copies. Aucun travail parallèle constaté.

**Entrées.** S44-2 et AUDIT-REPLIS-S44. Treize replis NaN recensés : onze dans le harnais,
deux dans les tests du cœur. Le suivi complet est au §7 du registre existant.

**Sorties.** Deux défauts sur la même origine eta() : C02 omettait ses trois assertions quand
aucun passage par zéro n'était trouvé ; C10 ignorait les points invalides dans son maximum et
pouvait valider une surface à zéro sans mesure. Les tests échouent avant correction, passent
après avec témoins. C02 garde trois assertions en échec ; C10 propage le refus aux quatre mesures.
A173, L167 ; S44-2 et S43-3 closes. Aucun ADR ni seuil physique changé ; I-08 reste vrai.

**Validation.** cargo test --offline : 38 + 60 = 98 succès, deux tests longs ignorés.
check : deux scénarios, zéro échec ; hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e.
physics : 267 lignes comparées avant/après, seules les durées et les coûts par pas changent.
Sortie 1 attendue : C04 ordre un reste en échec. Les quatre avertissements de compilation
préexistants ne sont pas traités. Aucun résultat nominal déplacé.

**Décision structurante.** La liste des assertions ne dépend pas de la réussite des mesures.
L'eau plate refuse une mesure de période mais convient à la statique : le témoin vide dépend
de la grandeur observée.

**Non fait.** C08 peut encore représenter un refus par Observe(NaN) ; il ne devient pas un
succès sur le chemin audité, mais son statut est trompeur. Action S45-1. S44-1 (front absent),
S43-2 (entrées vides des autres contrôles), S42-3 (mesures dupliquées) restent ouvertes.

**Suite recommandée : S46 — rendre explicites et compter les verdicts indéterminés de C08**
(S45-1), avec témoins finis et refus. Arbitrages inchangés : masse volumique A103, état réel
et infrastructure ; aucun dépôt distant créé. A107 est historiquement réconcilié en S35,
les branches conservées restent soumises au contrôle de reprise.

## S46 — 2026-09-07 — Un bilan doit compter ce qui ne conclut pas

**Agent : Codex**, git et cargo disponibles. Départ master 53ded2b propre ; copies parallèles
vérifiées, aucune plus avancée ou modifiée. Action S45-1.

**Sorties.** Convergence::ordre refuse NaN, infinis et résultats non finis ; les ordres négatifs
finis restent visibles. Le contrôle de stabilité ne retire plus les triplets inexploitables.
Le rapport principal utilise une seule classification, appliquant C08 amendé : cas régulier,
stabilité établie, puis seuil. Toutes les familles comptent dans succès/échec/sans-verdict.
AUDIT-REPLIS-S44 §8, note sur CAS-CANONIQUES C08, A174, L168. Aucun ADR ni seuil modifié.

**Constats.** Deux tests rouges avant correction : Observe(NaN), puis stabilité artificielle
obtenue en retirant des triplets refusés. Le rapport possédait deux règles différentes ; la
branche singulière pouvait réussir sans preuve de stabilité, la régulière pouvait échouer
sans cette preuve. Indéterminé et plancher manquaient dans le décompte.

**Validation.** 101 tests exécutés réussis (38 cœur + 63 harnais), deux longs ignorés.
Sept tests ciblés ; dix familles synthétiques classées jusque dans le texte du bilan.
check : deux scénarios, zéro échec, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e.
physics avant/après : 56 lignes de mesures/assertions/suites C08 identiques ; sortie 1
attendue, C04 ordre un reste rouge. Le bilan C08 compte désormais cinq sans-verdict au lieu
de quatre : p = 0,82 du cas régulier existait mais sa stabilité inconnue échappait au compteur.

**Décision structurante.** Un refus est conservé dans toute la famille de mesure ; le bilan
compte chaque famille une fois, même quand elle n'établit rien. Invariants inchangés.

**Non fait / suite recommandée.** S47 : confronter la paire héritée C08-p/C08-coherence de
physics_shallow au contrat amendé C08 (S46-1). Elle reste sur Ritter avec trois grilles et un
seuil absolu, séparée des cinq familles corrigées ici. Préserver les mesures historiques,
clarifier leur portée. S44-1, S43-2 et S42-3 restent ouverts.

**Arbitrages.** A103 (masse volumique), état réel et infrastructure inchangés ; aucun distant
créé. A107 réconcilié historiquement ; copies anciennes conservées et contrôlées au démarrage.

## S47 — 2026-09-07 — La mesure se conserve, sa portée se corrige

**Agent : Codex**, git et cargo disponibles. Master 7b0f1ec propre ; copies et branches
contrôlées, aucun travail parallèle constaté. Action S46-1.

**Sorties.** Le C08 hérité de shallow est DiagnosticRitter, séparé des assertions : p sans
seuil absolu, refus Option explicite, contrôle C08-coherence conservé à 0,25. Le contrôle
signale toujours une incohérence ou une entrée non finie et affecte le code de sortie.
Notes correctives datées dans ADR-040 et ADR-043 ; tableau des verdicts et résumés corrigés.
AUDIT-REPLIS-S44 §9, A175 et L169. S46-1 close.

**Constat.** Un p = 0,999745 sur trois grilles de Ritter ne valide pas C08 amendé : le support
est singulier et la stabilité n'est pas établie. Le chiffre reste utile pour comparer les
schémas ; le contrôle de cohérence a trouvé un défaut de fond en B-S24 et ne doit pas disparaître.
La mesure était reproductible ; c'est la portée du mot « vert » qui était fausse.

**Validation.** 102 tests réussis (38 cœur + 64 harnais), deux ignorés. Quatre tests ciblés C08 :
valeurs sous/au-dessus de l'ancien seuil, incohérence 0,83, refus NaN/infinis, mesure historique.
57 lignes de mesures/assertions/suites inchangées ; p = 0,999745, écart = 0,001459.
check : zéro échec, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e.
physics sortie 1 attendue, C04 ordre un toujours rouge. Second véhicule : 12 assertions,
un contrôle de cohérence et un diagnostic sans verdict de validation. Aucun solveur modifié.

**Décision structurante.** Appliquer le contrat existant n'exige ni nouvelle décision physique,
ni effacement des chiffres historiques. Le diagnostic et son contrôle ont des rôles distincts.
Invariants relus dans le socle de cette conversation : aucun n'est modifié ou invalidé.

**Suite recommandée.** S48 — construire C22 régulier sur shallow avec au moins cinq grilles,
mesurer l'ordre et sa stabilité, conserver coûts et refus (S47-1). S44-1, S43-2 et S42-3 restent
ouverts ; ce travail ne remplace pas le banc B3 ni un solveur de production.

**Arbitrages inchangés.** A103 (masse volumique), constat d'état réel et infrastructure ; aucun
nouveau distant. A107 historiquement réconcilié, anciennes copies conservées et contrôlées.

## S48 — 2026-09-07 — Affiner la référence ne stabilise pas les grilles mesurées

**Agent : Codex**, git et cargo disponibles. Master 5795f05 propre à la reprise, anciennes
copies contrôlées. Action S47-1. P3 achevée après la relance « reprends » de l'utilisateur.

**Sorties.** Nouveau mode c22-shallow [nx_oracle], montage gaussien C22 à cinq grilles,
HLL/MUSCL/RK2, deux oracles et projection conservative. MESURES-C22-S48 documente conditions,
chiffres, coûts et limites. S47-1 close. Aucun solveur existant modifié.

**Mesures.** Oracles 6400/12800 : quatre grilles retenues, stabilité inconnue, 5,416 s.
12800/25600 : cinq retenues, p = 1,637633 / 1,631728 / 1,849853, non asymptotique, 22,159 s.
25600/51200 : cinq retenues, p = 1,637646 / 1,631733 / 1,849841, même verdict, 102,612 s.
La dernière durée inclut une charge concurrente de tests : coût local, pas benchmark contrôlé.
L'écart entre oracles tombe à 1,717298105e-9 ; les ordres bougent de moins de 0,000013.

**Portée.** Affiner l'oracle seul ne résout pas la non-stabilisation de la famille 100–1600.
L'écart des deux oracles sert au filtre empirique ×30 de C22 ; il ne borne pas leur erreur
commune. Le résultat n'est ni une validation C08 ni un échec physique du solveur. La campagne
coûte assez pour rester dans un mode dédié, en dehors du rapport physics courant.

**Validation.** 104 tests réussis (38 cœur + 66 harnais), deux ignorés. Projection conservative,
champ sans excitation exactement constant, largeur gaussienne, refus de tailles/non-finis/norme
nulle. Les champs doivent atteindre le temps final et ne pas saturer. check : zéro échec,
hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e. Taille d'oracle invalide : sortie 1.
Les anciennes mesures sont protégées par leurs tests ; physics complet non relancé cette session.

**Enseignements / angles morts.** Aucun nouvel angle mort enregistré : coût d'oracle A114/A116
et dépendance au montage A153 déjà connus. La largeur de configure_bosse désigne exp(-d²),
celle de C22 exp(-d²/2) : conversion sqrt(2) testée, pas changement de cas. L170 ci-dessous.
Invariants inchangés, aucun ADR réécrit ni nouvelle décision d'architecture.

**Suite recommandée : S49.** Déplacer la fenêtre de grilles vers les plus fines, conserver au
moins cinq grilles non contaminées, réutiliser explicitement les oracles si possible, mesurer
stabilité et coût (S48-1). Ne pas assouplir le critère pour conclure. S44-1, S43-2, S42-3 restent
ouverts. A103, constat d'état réel et infrastructure inchangés ; aucun distant créé.

## S49 — 2026-09-07 — L'ordre approche deux, la stabilité reste à établir

**Agent : Codex**, git et cargo disponibles. Master 8395ca0 propre à la reprise,
copies anciennes contrôlées. Action S48-1 exécutée, sans changer schéma ni critères.

**Sorties.** Mode c22-shallow-fenetres : trois fenêtres fixées avant mesure, sept champs
et deux oracles réutilisés en mémoire (I-17 relu et respecté). MESURES-C22-S49 conserve
les nombres, le coût et les limites ; C22 canonique pointe vers ce complément.

**Mesures.** Oracles 51200/102400, écart 5,207593646e-10. Sept grilles passent le filtre
empirique ×30. Fenêtres 100–1600, 200–3200 et 400–6400 : toutes sans verdict.
La dernière donne p=1,849839 / 1,960632 / 2,011665 ; d1=0,051033 < 0,10, mais
4*d1=0,204132 > d0=0,110793. Le critère de ralentissement de la dérive refuse encore.
Coût local release : 382,716 s, sans suite de tests concurrente. La grille 6400 conserve
une marge de deux seulement au filtre ; affiner sans revoir la référence serait injustifié.

**Décision structurante.** Une proximité de l'ordre nominal ne remplace pas le test de
stabilité convenu. Aucun seuil assoupli. Les trois fenêtres partagent des points ; ce ne
sont pas trois confirmations indépendantes. S48-1 close, extension coûteuse S49-1 ouverte.

**Validation.** 105 tests réussis (38 cœur + 67 harnais), deux ignorés. Test du filtrage
avec trou, refus des tailles incompatibles, vérification CLI (sortie 1), mode historique
rejoué. Aucun solveur modifié ; mesures historiques protégées par leurs tests.
Les campagnes physics/check complètes ne sont pas répétées pour ce mode dédié.

**Enseignements.** L171 ; aucun nouvel angle mort (175). Pas de nouvel ADR, invariant,
SPEC ou cas. Ce travail ne valide ni B3 ni la physique 3D.

**Suite recommandée S50 : S44-1**, refus explicite de front_mouille sans cellule au seuil,
puis propagation aux six replis. S49-1 attend une stratégie et un budget adaptés à des
oracles plus fins ; S43-2 et S42-3 restent ouverts. A103, état réel du projet et infrastructure
inchangés ; aucun distant créé. A107 historiquement réconcilié, copies conservées.

## S50 — 2026-09-07 — Le front absent ne devient plus une position

**Agent : Codex**, git et cargo disponibles. Départ master 7a33d95 propre, anciennes
copies contrôlées. REPRISE et S49 relus ; socle déjà lu dans cette conversation.

**Sorties.** S44-1 close. Cinq sites corrigés dans physics_shallow : quatre positions
et un indice de profil. front_ritter conserve Option ; cas_front distingue absence et
position zéro. Le profil absent est annoncé, sans première cellule de remplacement.
AUDIT-REPLIS-S44 §10 suit les conversions terminales et leurs usages jusqu'au verdict.

**Correction du diagnostic initial.** Le détecteur front_mouille du cœur refusait déjà.
Ce sont ses appelants qui effaçaient le refus ; le compte de six replis annoncé par S44
était inexact. Aucun solveur ni seuil modifié. Pas de nouvel ADR ou invariant.

**Validation.** 107 tests réussis (38 cœur + 69 harnais), deux ignorés. Sept tests ciblés,
dont deux nouveaux : bassin sec/seuil non atteint et témoin humide ; absence face à une
référence nulle ou non nulle, position zéro valide, profil indisponible. Comparaison des
rapports physics avant/après : seules quatre lignes de durées diffèrent. Front shallow
10,562500 m contre 10,640868 m, écart 0,736 %, inchangé. physics sort à 1 pour C04 ordre un,
comme attendu. check : zéro échec, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e.

**Enseignements / angles morts.** Aucun nouveau : application du suivi de refus de L166
et de la règle du §5 de l'audit. Le risque déjà anticipé — zéro pourrait devenir favorable
si la référence change — est désormais couvert par un test. Total inchangé : 175 angles,
L171 dernière leçon. Aucun invariant invalidé par cette correction du harnais.

**Suite S51 : S42-3**, inventorier les mesures écrites plusieurs fois et vérifier leurs
appelants avant toute extraction. S43-2 (troisième cas des garde-fous) et S49-1 (budget de
raffinement C22) restent ouverts. A103, état réel du projet et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, copies conservées.

## S51 — 2026-09-07 — Deux formules identiques avaient des refus différents

**Agent : Codex**, git et cargo disponibles. Départ master 8fc8a65 propre, copies anciennes
contrôlées. P4 reprise après la relance utilisateur ; P1–P3 déjà committées, non refaites.

**Sorties.** AUDIT-MESURES-S51 inventorie dix familles et leurs appelants, en distinguant
les mesures différentes des copies algébriques. S42-3 close comme inventaire exécuté.
La formule de Richardson utilisée par Convergence::ordre et ordre_grossier_estime est
partagée ; catégories, planchers et valeur bornée de secours restent propres aux appelants.

**Défaut reproduit.** Le refus ajouté au rapport en S46 ne protégeait pas le filtre :
ordre_grossier_estime rendait encore (Some(NaN), NaN). Le test échoue avant correction,
passe ensuite pour NaN, infinis et débordement du rapport depuis des entrées finies.
Aucun succès nominal indu établi ; aucun seuil ni solveur modifié. Correctif de portée
ajouté à AUDIT-REPLIS-S44 §11, sans réécriture d'ADR. A176, sévérité 2, corrigé ; L172.

**Validation.** 108 tests réussis (38 cœur + 70 harnais), deux ignorés. Tests ciblés verts,
compilation release réussie. Rapport physics comparé au résultat S50 : trois lignes de
durées différentes, aucune mesure ou assertion déplacée. Sortie 1 attendue pour C04 ordre un.
check : zéro échec, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés.
C22 coûteux non rejoué ; son calcul de rapport est testé sans nouvelle simulation.

**Limites.** Inventaire manuel, pas preuve d'absence de toute duplication sémantique.
Régression centrée et projection conservative restent identifiées, non supprimées.
Les mesures de période, les références Ritter et les montages des deux véhicules ne sont
pas fusionnés ; leur indépendance et leurs différences de support sont utiles (ADR-043).
Aucun invariant invalidé ; 47 ADR, 17 invariants, 6 SPEC, 14 registres, 23 cas inchangés.

**Suite S52 : S51-1.** Partager la régression centrée en préservant fenêtres et unités,
tester refus/témoins ; examiner séparément le contrat de projection des deux C22 avant
une extraction. S43-2 et S49-1 restent ouvertes. A103, état réel et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, anciennes copies conservées.

## S52 — 2026-09-07 — Partager le calcul, préserver les conditions de mesure

**Agent : Codex**, git et cargo disponibles. Master 50e952d propre, copies anciennes
contrôlées. S51-1 exécutée : régression partagée et projections C22 examinées.

**Sorties.** regression::centree remplace les deux boucles pente/R² dans physics.rs.
Fenêtres, logarithmes ln/log2, minima de points et conversions temporelle/spatiale restent
chez les appelants. Option refuse les séries dégénérées, les non-finis et les débordements.
Les libellés de refus incluent la régression, sans attribuer tous les refus à une onde éteinte.
MESURES-PARTAGEES-S52 documente les contrats, limites et tests. Aucun solveur ni seuil changé.

**Distinction utile.** Une série horizontale a une pente mais pas de R² : le couple demandé
est refusé. Une covariance nulle avec variances positives rend bien pente=0 et R²=0.
Les anciennes boucles ne distinguaient pas ces cas. Le contrôle reste borné aux points
reçus : il ne prouve pas que la sélection amont a conservé toutes les données nécessaires.

**Projections.** Elles sont conservées après examen : norme signée/absolue, refus, évolution
et filtration diffèrent. Le montage delta peut retirer des grilles avant Convergence,
qui suppose ensuite un doublement sans vérifier les tailles. S52-1 exige un essai de
ce chemin amont avant extraction. Aucun verdict nominal faux démontré par cette lecture.

**Validation.** 111 tests réussis (38 cœur + 73 harnais), deux ignorés. Trois tests nouveaux :
droite/résidus connus, séries inexploitables, logarithmes et unités. Rapport physics identique
à S51 sauf quatre lignes de durées ; sortie 1 attendue pour C04 ordre un. check sans échec,
hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés. C22 coûteux non répété.

**Enseignements / angles morts.** Application de L172 et du risque de trous déjà identifié
par A174 ; aucun nouvel identifiant attribué sans essai supplémentaire. Total 176 angles,
L172 dernière leçon. Invariants inchangés, aucun ADR réécrit.

**Suite S53 : S52-1**, vérifier les tailles et conserver les grilles refusées avant le calcul
d'ordre C22 delta. S43-2 et S49-1 restent ouvertes. A103, état réel et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, anciennes copies conservées.

## S53 — 2026-09-07 — Une famille de valeurs n'est pas une famille de grilles

**Agent : Codex**, git et cargo disponibles. Master 348f5f5 propre, copies anciennes
contrôlées. S52-1 exécutée. Socle connu dans cette conversation, état et journal S52 relus.

**Défauts reproduits.** Une grille nulle provoque un modulo zéro. Une allocation refusée
transforme 4,8,16,32,64 en 4,16,32 après filtre. Une suite artificielle aux erreurs divisées
par deux mais aux tailles 100,200,800,1600,3200 est déclarée stable. Ce dernier test isole
le défaut du validateur ; il ne représente pas une nouvelle mesure physique.

**Sorties.** Admission des tailles avant allocation/modulo ; doublements vérifiés dans
Convergence::ordre et ordre_grossier_estime. Grilles refusées conservées avec NaN, filtre
suspendu sur famille incomplète. Filtration des familles exploitables limitée au préfixe.
GRILLES-C22-S53, correctif de portée AUDIT-REPLIS-S44 §12, A177 et L173.
Aucun seuil ni solveur modifié ; projections delta/shallow encore séparées.

**Validation.** Quatre tests ciblés : trois reproductions, témoins réguliers, tailles
répétées/décroissantes/débordantes, oracle indisponible, préfixe avec trou. 115 tests réussis
(38 cœur + 77 harnais), deux ignorés. Rapport physics identique à S52 sauf trois durées,
sortie 1 attendue pour C04 ordre un. check : zéro échec, hashs 0x3e2c06a7b00e73e3 et
0x1a8b0629a9f51b6e inchangés. Campagne coûteuse shallow non répétée.

**Portée.** Le contrat de Richardson suppose un doublement ; les tailles ne peuvent pas
rester de simples étiquettes. S46 protégeait les trous déjà représentés, pas les grilles
retirées en amont. Aucun changement d'invariant ou d'ADR ; pas de validation 3D.

**Suite S54 : S43-2**, confronter les garde-fous existants à une entrée vide de l'objet
mesuré, avec refus et témoin. S49-1 reste ouverte pour le raffinement C22 plus coûteux.
A103, état réel et infrastructure inchangés ; aucun distant créé. A107 historiquement
réconcilié, anciennes copies conservées.

## S54 — 2026-09-07 — Le vide ne demande pas toujours un refus

**Agent : Codex**, git et cargo disponibles. Départ master 91e26df propre, copies anciennes
contrôlées. P3/P4 achevées après relance utilisateur ; travaux précédents conservés.

**Sorties.** Six tests complémentaires couvrent G1–G10 avec absence et témoin. Aucun faux
succès supplémentaire observé sur ces entrées. C33 possède un montage interne à batteur
réglable pour injecter zéro ; API publique et amplitude nominale 0,05 inchangées.
GARDE-FOUS-VIDE-S54 et complément audit S34 §6 ; référence G9 erronée de S51 corrigée en G6.
S43-2 close pour les dix garde-fous inventoriés.

**Constat.** Absence physique valide (domaine sec), défaut de configuration optionnelle
(CFL non réglée) et absence de mesure demandent trois réponses différentes. G1 testait déjà
un domaine sec : la formulation S43 « aucun » était trop générale. La géométrie saine de G6
ne prouve pas la présence de la source : le batteur arrêté est refusé par le profil vide.

**Observation à suivre.** Le premier témoin G5, 400 cellules et 60 s, est refusé malgré une
excitation nominale. Le témoin publié, 200 cellules et 20 périodes, passe ; c'est celui retenu
pour le test à vide. S54-1 conserve le premier constat et demande sa cause, sans préjuger
qu'il s'agit d'un défaut ni assouplir les conditions de mesure.

**Validation.** 121 tests réussis (38 cœur + 83 harnais), deux ignorés. Compilation release
réussie. Rapport physics identique à S53 sauf trois durées, sortie 1 attendue pour C04 ordre un.
check : zéro échec ; hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés.
Aucun calcul de solveur ni seuil modifié. Paramètres invalides autres que le vide hors périmètre.

**Enseignements / angles morts.** Aucun nouvel identifiant : application de L164, L169 et L172.
177 angles, L173 dernière leçon. Aucun invariant invalidé, aucun ADR réécrit.

**Suite S55 : S54-1**, expliquer le refus de la seiche excitée à 400 cellules et 60 s.
S49-1 reste ouverte pour le raffinement C22. A103, état réel et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, anciennes copies conservées.

## S55 — 2026-09-07 — Un plateau ne supprime pas un extremum

**Agent : Codex**, git et cargo disponibles. Départ master e55582a propre, autres copies
contrôlées. Entrée : S54-1, seiche excitée refusée à nx=400 sur 60 s.

**Cause et sortie.** La pente nulle effaçait le sens précédent : huit extrema à nx=200,
cinq à nx=400, pour sept passages à zéro aux deux. Conserver la dernière pente non nulle
retrouve treize extrema aux deux. Aucun seuil diminué, aucun calcul de solveur modifié.
Instrumentation temporaire retirée ; deux tests ajoutés (signaux analytiques et deux témoins).
Rapport EXTREMA-SEICHE-S55 ; notes correctives datées ADR-033/034 et C03 canonique.

**Chiffres.** Sur 60 s : demi-vie nx=200 de 25,23785 à 25,08627 périodes ; nx=400 devient
mesurable à 50,34245, R² 0,99964488. Dans la campagne de vingt périodes, C03 rampe passe
de 20,69 à 21,20 ; mode propre de 24,40 à 24,45. Les tableaux de résolution, Courant,
amplitude et harmoniques sont corrigés dans le rapport. Les petites amplitudes deviennent
mesurables mais restent différentes : la cause de leur biais physique n'est pas établie.

**Validation.** 123 tests réussis (38 cœur + 85 harnais), deux ignorés ; release compilée.
Campagne nominale : verdicts inchangés, seule sortie 1 de C04 ordre un attendue ; changements
numériques limités aux mesures des extrema, hors durées machine. check : zéro échec, hashs
0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés. C22 coûteux non relancé.

**Portée.** A178 corrigé, L174 ; S54-1 close. Aucun invariant invalidé, aucune décision
physique modifiée. La position exacte de l'extremum dans un plateau reste non résolue par
l'échantillonnage ; pas de revendication nouvelle sur la précision des demi-vies.

**Suite S56 : S49-1**, stratégie de référence et budget pour une fenêtre C22 plus fine,
avec contrôle de contamination conservé. A103, état réel et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, anciennes copies conservées.

## S56 — 2026-09-07 — La référence limite la fenêtre suivante

**Agent : Codex**, git et cargo disponibles. Départ master d8e952a propre, anciennes copies
contrôlées propres. Entrée : S49-1, stratégie de référence et budget pour raffiner C22.

**Sorties.** Mode séparé c22-shallow-fin : huit grilles, quatre fenêtres jusqu’à 800–12800,
deux oracles réutilisés en mémoire, chronométrés séparément. Anciens modes et seuils conservés.
REFERENCE-C22-S56 décrit résultats, coût, extrapolations et protocole de la campagne suivante.

**Mesure.** Références 51200/102400, 371,116 s au total ; 72,727/292,354 s pour les oracles.
Les sept anciennes grilles reproduisent S49 aux chiffres imprimés. La grille 12800 a une
erreur 7,710700097e-9 contre 102400, sous le seuil empirique 1,562278094e-8. Elle est rejetée :
fenêtre fine 4/5, deux ordres seulement, sans verdict. Bilan : quatre familles sans verdict,
zéro succès et zéro échec physique. Aucun ordre reconstruit après la grille rejetée.

**Stratégie.** Prochain couple 76800/153600, budget estimé 827,467 s (13 min 47 s), sans
garantie de délai ni d’admission. L’exposant empirique de décroissance des écarts d’oracles
vaut 1,72145 ; utiliser deux à sa place changerait la prévision du filtre sur 12800.
Mesurer d’abord cette admission ; pas de doublement automatique si elle échoue. Un couple
102400/204800 coûterait environ 24 min 26 s : prévoir un découpage en mémoire avant extension.
Aucun champ sérialisé (I-17), aucune extrapolation du champ fondée sur l’ordre recherché.

**Validation.** 123 tests réussis, deux ignorés ; trois tests C22 ciblés avant mesure,
refus du nouveau mode ajoutés au test existant. Release compilée. check vert, hashs
0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés. Tests complets exécutés après
la mesure de coût. physics général non répété ; solveurs et scénarios inchangés.

**Enseignements / angles.** Pas de nouvel identifiant : contamination déjà documentée S48/S49,
application de L173 au support et des limites connues de l’oracle. 178 angles, dernière leçon L174.
Aucun invariant invalidé ni ADR réécrit ; les mesures historiques restent valables.

**Suite S57 : S56-1**, exécuter le couple 76800/153600 selon REFERENCE-C22-S56.
S49-1 close pour la stratégie et le budget. A103, état réel et infrastructure inchangés ;
aucun distant créé. A107 historiquement réconcilié, anciennes copies conservées.

## S57 — 2026-09-07 — Une extrapolation n'est incertaine qu'en proportion de sa portée

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ 13851c1, identique à
master, dans le worktree isolé claude/reprise-projet-29ef50 ; les trois autres copies sont
en retard ou archivées, aucune session concurrente. Entrée : S56-1, exécuter le couple
d'oracles 76800/153600 selon REFERENCE-C22-S56.

**Sorties.** MESURES-C22-S57 : mesure, admission, biais d'oracle estimé, budget révisé.
A179, L175, action S57-1. Aucun fichier de code modifié : la campagne est une exécution
du binaire construit sur 13851c1, sans changement de montage, de seuil ni de critère.

**Mesure.** 844,433 s contre 827,467 s estimés (+2,05 %) ; oracles 165,833 et 672,560 s,
reste 6,040 s comme en S56. Écart d'oracles 2,709078717e-10, seuil ×30 8,127236151e-9.
La grille 12800 rend 7,766762184e-9, soit **0,9556 fois le seuil : encore refusée**, contre
0,494 en S56. Fenêtre 800–12800 à 4/5, deux ordres ; quatre familles sans verdict, sortie 0.
Les sept anciennes grilles conservent leurs ordres aux chiffres imprimés depuis S48.

**Décision structurante.** Aucune. Le refus est maintenu, le filtre garde sa définition,
et aucun doublement n'a été lancé automatiquement — le protocole de S56 l'interdisait.

**Chiffres qui ont orienté.** Les deux extrapolations de S56 sous-estiment la contamination
de 14,6 % (n⁻²) et 4,6 % (empirique) ; l'exposant local tombe de 1,72145 à 1,61233. Mais le
déficit final n'est que de **4,44 %**, et les quatre exposants candidats s'accordent à 0,8 %
sur l'oracle requis — environ **79 000**. En S56 le modèle décidait du verdict ; ici il ne
décide plus de rien, parce que la portée extrapolée est passée d'un facteur 1,5 à 1,03 (L175).

Et le déplacement des erreurs entre S56 et S57, sur les mêmes grilles, est **additif et
constant** : 5,587e-11 dès nx=800. Ajusté avec la colonne « variation » (1,310e-10), il donne
un biais d'oracle en n^-1,879, soit **4,9e-11 pour 153600** contre **1,80e-10 pour 76800**.
L'erreur de 12800 dépasse alors le biais de l'oracle qui la mesure d'un facteur 159, quand
l'indicateur du filtre n'en vaut que 5,5 fois : **le filtre est piloté par le biais de
l'oracle auxiliaire, dont aucune erreur publiée ne dépend** (A179, sévérité 2).

**Ce qui n'a pas été fait.** Le filtre n'est pas modifié et la loi de biais n'est pas
validée : deux différences, un modèle additif uniforme non établi. La borne du mode n'est
pas relevée — c'est une modification de code, elle se déclare et se teste à part. Aucune
mesure à 89600.

**Validation.** 123 tests réussis (38 cœur + 85 harnais), deux ignorés, avant la mesure ;
release compilée ; check vert, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés.
Aucun test lancé pendant la campagne. Aucun champ sur disque (I-17).

**Suite S58 : S57-1**, relever la borne du mode à 89600, déclarer le découpage du calcul
exigé au-delà du quart d'heure, puis mesurer 89600/179200 — environ 19 min 07 s projetées,
admission prévue avec 20 à 30 % de marge, **sans promesse de verdict** : la fenêtre n'aurait
que trois ordres, et la stabilité reste à établir. A103, état réel et infrastructure
inchangés ; aucun distant créé. A107 historiquement réconcilié.

**Point de procédure à signaler.** Cette session a travaillé dans un worktree isolé sur
claude/reprise-projet-29ef50 : c'est exactement le mécanisme des trois forks (L137). La
branche doit être fusionnée dans master, et ce geste appartient à l'utilisateur.

## S58 — 2026-09-07 — Le cas qui devait arbitrer la constante ne pouvait pas la voir

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ bd9f087 ; le travail de S57
avait été fusionné dans master en avance rapide juste avant l'ouverture. Entrée : **délégation
explicite de l'utilisateur**, le 2026-09-07, pour réaliser les points laissés « pour lui ».

**Sorties.** [`ADR-048`](../docs/adr/ADR-048-la-masse-volumique-est-une-propriete-du-milieu.md),
[`RHO-EAU-S58`](../docs/validation/RHO-EAU-S58.md), le type `Milieu` dans `water-core`, A180,
L176, notes correctives datées dans `CAS-CANONIQUES` et `00_INDEX`. **A103 est close.**

**Décision structurante.** La masse volumique de l'eau du projet est **1025** — l'eau de mer —
et elle **cesse d'être une constante globale** : c'est une propriété du milieu, `Milieu::MER`
et `Milieu::EAU_DOUCE`, la mer par défaut. Motif : le monde contient des eaux intérieures, et
l'estuaire est l'endroit où un même corps change de tirant en avançant — une constante globale
rend ce phénomène inexprimable, et le rend inexprimable *silencieusement*. ADR-048 §4 dit ce
qu'il faudrait pour inverser chacune des trois décisions.

**Chiffres qui ont orienté.** Le balayage, et non le raisonnement. Constante portée à 1025 par
recompilation : tirant 0,250000 → **0,243902 m** (−2,439 %), raideur 2452,5 → **2513,8125 N/m**
(+2,500 %), période 1,003033 → **0,990726 s** (−1,227 %) — et **les quatre assertions de C10
restent vertes à écart 0,000 %**, aux deux valeurs. Les trois références sont construites *avec*
la constante ; la tolérance de ±1 % que le corpus opposait aux 2,5 % porte sur un écart
structurellement nul.

**Ce que cela retourne.** L'argument qui bloquait A103 depuis S21 — *« C10 exige 1000, la mer
demande 1025, et l'écart vaut deux fois et demie la tolérance »* — est faux dans sa conséquence,
et il a été recopié dans trois documents et rappelé en fin de **trente-sept** sessions. Les seules
choses du projet qui échouent quand la constante bouge sont **deux assertions de test unitaire**
de `body.rs`, qui comparent le tirant à un littéral. **Le seul contrôle réel était le petit ; le
cas canonique, qui porte la tolérance et le verdict, était aveugle** (**A180**, sévérité 2).

Et la faute avait un nom depuis S21 : **A104**, *une référence tirée des paramètres ne prouve
rien*, énoncée dans l'en-tête du module qui l'a commise. Personne n'avait relié A104 à A103.

**Ce qui n'a pas été fait.** Aucune mesure indépendante du tirant n'a été créée — il n'en existe
pas de source, et en inventer une aurait été fabriquer la mesure qui manque. C10 reste donc
aveugle à `ρ`, par décision et non par oubli (ADR-048 D3). La masse ajoutée (A26) reste en
attente, et S57-1 n'a pas été entamée.

**Validation.** **125 tests réussis** (40 cœur + 85 harnais), deux ignorés — deux de plus qu'en
S57, tous deux sur le milieu. Release compilée. Les deux hashs `check` sont **inchangés**,
`0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e` : la constante ne touche pas l'état scellé.
Campagne `physics` complète identique — seul C04 à l'ordre un échoue, comme voulu, et C08 reste
sans verdict. `RHO_EAU` n'apparaissait dans aucun solveur : la portée est bornée aux corps.

**Suite S59 : S57-1**, relever la borne du mode `c22-shallow-fin` à 89600, déclarer le découpage
du calcul exigé au-delà du quart d'heure, puis mesurer 89600/179200 (≈ 19 min projetées).
**A103 est close** et sort des rappels de fin de session. Restent : l'état réel du projet et
l'infrastructure — aucun distant créé. A107 historiquement réconcilié.

## S59 — 2026-09-08 — C22 conclut, et le remède prescrit aurait empêché de le voir

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ ae93fd0, master et worktree
confondus. Entrée : **S57-1** — découpage du calcul, borne du mode, mesure 89600/179200.

**Sorties.** MESURES-C22-S59 ; `avancer_jusqu_a_observe` dans `water-core` ; borne du mode
portée à 89600 ; deux tests de découpage ; A181, L177. **S57-1 close.**

**La réserve levée en premier, et elle était fondée.** REFERENCE-C22-S56 §5 prescrivait, au-delà
du quart d'heure, « un découpage de calcul en tranches temporelles gardées en mémoire ». Or
`avancer_jusqu_a` prend `dt = dt_cfl.min(t_fin − t)` : **chaque borne de tranche insère un pas
tronqué** absent du calcul continu. Mesuré avant d'implémenter : à nx=200, t=1 s, quatre tranches
donnent un champ différent bit à bit. Le remède aurait rendu la campagne incomparable à S48, S49,
S56 et S57 — c'est-à-dire aurait détruit ce qu'il devait rendre possible (**A181**, **L177**).

Retenu à la place : le découpage d'**observation**. `avancer_jusqu_a_observe` porte la seule
boucle, `avancer_jusqu_a` n'en est qu'un appel avec un observateur vide — l'identité est
structurelle et non retestée à chaque modification (L162, A176). Et le test d'identité a
d'abord échoué **en trouvant une faute de sa propre écriture** : cadence de 50 pour 35 pas,
observateur jamais appelé, champ pourtant identique. Un témoin muet passait pour neutre.

**Décision structurante.** Aucune. Le critère est resté intact — c'était la condition pour que le
résultat signifie quelque chose.

**Le résultat.** Campagne 1143,284 s contre 1147,2 projetés (**−0,34 %**). Écart d'oracles
**2,118278666e-10**, prédit 2,1129e-10 (**+0,26 %**). La grille **12800 est admise** à 1,2240 fois
le seuil, marge **+22,40 %** — refusée à 0,494 en S56, à 0,9556 en S57. **La fenêtre 800–12800
conclut : 5/5 grilles, ordres 1,960625 · 2,011671 · 1,997599, `p = 1,96` stabilisé.** Bilan C22 :
**un succès**, zéro échec, trois sans verdict — le premier après quatre campagnes vides.

**Ce que le succès ne dit pas**, et le rapport le porte : la référence est un oracle numérique
**du même schéma**. Ce qui est établi, c'est que le schéma converge à l'ordre 2 environ vers sa
propre limite de raffinement, pas qu'il résout Saint-Venant. A114 reste entier. Trois réserves
accompagnent le chiffre : filtre empirique, marge de 22 % seulement, ordres non monotones.

**Et un fait inattendu, pour A179.** Les ordres calculés contre l'oracle 89600 et contre l'oracle
179200 coïncident à **1e-5** sur les six triplets : l'estimateur de Richardson travaille sur des
différences successives, où une contamination additive uniforme s'annule. **La grandeur mesurée
est presque insensible à l'oracle, alors que le droit de la publier en dépend entièrement.** La
loi de biais ajustée en S57 se vérifie par ailleurs à −3,8 % sur ce troisième couple, qui n'a pas
servi à l'ajuster.

**Ce qui n'a pas été fait.** **S57-2 n'a pas été traitée, volontairement** : discuter le critère
pendant la mesure aurait rendu le succès inexploitable — on n'aurait pas su s'il venait de la
référence plus fine ou d'un critère assoupli. Aucun couple plus grand n'a été lancé : il ne
renforcerait qu'une marge, sans déplacer un ordre qui ne dépend pas de l'oracle.

**Validation.** **127 tests réussis** (40 cœur + 87 harnais), deux ignorés. Release compilée.
Hashs `check` inchangés, `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`. Refus du mode vérifiés,
dont 102400 — multiple de 12800 que seule la borne de campagne refuse. Aucun test pendant la
mesure.

**Suite S60 : S57-2**, éprouver le critère lui-même par ADR — comparer l'erreur au biais de
l'oracle de mesure plutôt qu'à l'écart des deux oracles. C'est désormais la seule question
ouverte du dossier C22, et S59 lui apporte un troisième point et un argument neuf. Restent
S58-2 puis S58-1 sur les cas aveugles à leurs paramètres. A103 close en S58 ; l'état réel et
l'infrastructure restent hors de portée, aucun distant créé.

## S60 — 2026-09-08 — Un seul critère d'admission pour deux grandeurs de nature différente

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ 283e82a. Entrée : **S57-2**,
décider si le filtre de contamination de C22 doit comparer l'erreur au biais de l'oracle de
mesure plutôt qu'à l'écart des deux oracles.

**Sorties.** [`ADR-049`](../docs/adr/ADR-049-le-filtre-de-contamination-n-est-pas-mal-calibre-il-est-mal-attribue.md),
la mesure d'invariance à l'oracle publiée comme diagnostic, l'essai de refus à oracle grossier,
la contre-épreuve rétrospective de 51200/102400, **A182**, **L178**, action **S60-1**.
**S57-2 close.**

**Décision structurante — et c'est une décision de ne pas changer.** Le filtre ×30 est conservé
sans aucune modification. Rouvrir un critère juste après le premier succès de C22 était le piège
de la session, et il était nommé dans le plan avant la première mesure.

**Les deux mesures vont en sens contraire, et c'est ce qui tranche.**

*L'invariance ne peut pas remplacer le filtre.* Essai de refus à oracle 3200/6400, contamination
flagrante — erreur de la grille 1600 fausse de **5,2 %**, deux grilles refusées : l'invariance ne
vaut que **5,1e-3**, soit 0,3 % d'un ordre de 1,85. Aucun seuil raisonnable ne l'aurait refusée.
Deux oracles emboîtés d'un facteur deux partagent leur erreur, donc leurs ordres sont également
faux et leur écart reste petit — **A114 appliqué au critère lui-même**. Le test synthétique cerne
le domaine : un biais uniforme mille fois l'erreur la plus fine ne déplace pas l'ordre ; un biais
hétérogène cent fois plus petit le déplace. *L'invariance ne mesure pas la contamination, elle
mesure son hétérogénéité.*

*Mais le filtre a bloqué trois sessions un verdict déjà atteignable.* Contre-épreuve : la campagne
51200/102400, celle que le filtre a fait refuser en S56, rejouée en **371,972 s**. Le triplet du
verdict rend **1,997566515**, contre **1,997599436** publié par S59 — **écart 3,29e-5**. Entre les
deux, **1987,7 s de calcul, 33 min 08 s, et deux sessions**, pour déplacer un ordre de trois
centièmes de millième.

**Ce que la décision retient.** A179 est **requalifié** : son constat est exact, sa conclusion
implicite ne l'est pas. Le filtre n'est pas mal calibré — il protège très bien ce qu'il protège,
les **erreurs** publiées, et à oracle 3200 il refuse à bon droit une erreur fausse de 5,2 %. Il
est **mal attribué** : C22 publie deux grandeurs de nature différente, des erreurs et un ordre,
sous **un seul** critère d'admission, et une grille refusée coupe la famille donc supprime des
triplets dont l'ordre était juste (**A182**, **L178**).

**Ce qui n'a pas été fait, et pourquoi.** Le critère n'est pas remplacé : aucune mesure de ce
dépôt n'a atteint le régime décisif, celui où l'erreur d'une grille passe **sous** l'écart des
oracles — même à oracle 3200, elle vaut encore 4,8 fois cet écart. Tant qu'on ne l'a pas observé,
rien ne dit que la contamination y reste uniforme, et changer le critère serait extrapoler hors
du domaine mesuré (**L175**). L'expérience manquante est nommée dans **S60-1**.

**Validation.** **128 tests réussis** (40 cœur + 88 harnais), deux ignorés — deux ajoutés.
Release compilée ; hashs `check` inchangés, `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`.
**Aucun verdict n'a bougé** : filtre, seuils et familles sont ceux de S59.

**Suite S61 : S60-1**, mesurer une grille dont l'erreur passe sous l'écart des oracles — 25600
contre 51200/102400, où l'erreur attendue vaut environ 3,7 fois l'écart. C'est le seul point qui
manque pour décider du critère, et il coûte moins de sept minutes. Restent S58-2, S59-1 et S58-1.
A103 close en S58 ; l'état réel et l'infrastructure restent hors de portée, aucun distant créé.

## S61 — 2026-09-08 — Quatre campagnes ont mesuré ce qu'un rapport d'entiers donnait

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ 9f63f2c. Entrée : **S60-1**,
mesurer une grille dont l'erreur passe sous l'écart des deux oracles.

**Sorties.** [`ADR-050`](../docs/adr/ADR-050-le-filtre-de-contamination-est-une-condition-geometrique.md),
[`GEOMETRIE-DU-FILTRE-S61`](../docs/validation/GEOMETRIE-DU-FILTRE-S61.md), l'annonce
d'admissibilité et le mode `--annonce`, **A183**, **L179**, action **S61-1**.
**S60-1 close par dissolution.**

**Le premier geste a été d'éprouver la prescription** — c'est L177, apprise en S59, et elle a
servi dès la session suivante. **Aucun solveur n'a été lancé pour ce résultat** : les treize
points viennent des campagnes déjà consignées.

**Décision structurante.** Le rapport entre l'erreur d'une grille et l'écart des deux oracles
vaut `k^p / (1 − 2^-p)`, où `k = oracle/grille` : les deux quantités ont la **même origine**,
l'erreur du schéma. Ajusté sur treize couples issus de cinq campagnes, oracles de 3200 à 89600 :
`ratio ≈ 2,011·k^1,902·o^-0,058`, écart maximal **23,5 %**. **La taille de l'oracle ne compte
presque pas.**

Trois conséquences. **Le filtre ×30 équivaut à `k ≥ 6`** — une condition géométrique, lisible sur
un rapport d'entiers avant tout calcul. L'historique s'y range sans exception : `k` valait 2 en
S48, 4 en S49 et S56, 6 en S57, **7 en S59**, et l'admission bascule exactement là. **Quatre
campagnes et près d'une heure de calcul ont mesuré ce que la suite 2, 4, 6, 7 donnait.**

**Et S60-1 est dissoute** : `ratio < 1` demande `k ≈ 1`, quand l'emboîtement impose `k ≥ 2`. Le
minimum atteignable est **4,75 mesuré**, et la dérive en `o^-0,058` ne l'efface jamais. *Le régime
n'est pas coûteux, il n'existe pas.* L'hypothèse d'uniformité qu'il devait éprouver est établie
autrement, et depuis longtemps : la colonne « variation » est plate à 2 % sur un facteur 16 en
grille et 256 en erreur.

**Une erreur de cette session, corrigée dans la session même.** Le premier chiffrage annonçait
1 355 000 cellules et **72,3 h** — donc « hors budget ». Il extrapolait l'exposant **local**
1,596, mesuré entre 76800 et 89600, alors que le calage sur toute la plage donne **1,9**. Le
chiffre était faux et la conclusion trop douce. **C'est L175 une seconde fois, commise par la
session qui venait de l'écrire.**

**Ce que cela apprend sur le critère.** Le filtre est décrit depuis S48 comme empirique. C'est
vrai de ce qu'il **borne**, faux de ce qu'il **exige** : `(o/n)^p ≥ 30·(1 − 2^-p)`. **Le seuil
d'admission d'une mesure d'ordre est une fonction de l'ordre** — `k ≥ 4,7` à l'ordre deux,
`k ≥ 15` à l'ordre un. Dimensionner la campagne suppose de connaître la réponse cherchée, et se
tromper d'hypothèse ne produit pas d'erreur visible mais un **« sans verdict »** : l'histoire de
C22 de S48 à S57 (**A183**, **L179**).

**Ce qui a été construit.** L'annonce d'admissibilité s'affiche avant chaque campagne, et
`--annonce` la donne sans rien calculer. Elle ne commande rien, dit qu'elle suppose l'ordre deux,
et **avoue son incertitude entre `k = 5` et `k = 7`** — à `k = 6` elle annoncerait 31,6 contre
28,7 mesurés, du mauvais côté du seuil. Un test la confronte à l'historique.

**Validation.** **129 tests réussis** (40 cœur + 89 harnais), deux ignorés. Release compilée ;
hashs `check` inchangés ; aucun verdict déplacé, aucun résultat rouvert.

**Suite S62 : S61-1** est close par construction — l'annonce existe. Restent **S58-2** puis
**S59-1**, deux recensements du même genre : des fautes identifiées dont on ignore le nombre
d'instances, et qui ne se périment pas. Puis **S58-1**. A103 close en S58 ; l'état réel et
l'infrastructure restent hors de portée, aucun distant créé.

## S62 — 2026-09-08 — Le cas qui mesurait la taille de sa fenêtre

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ 92042b4. Entrée : **S58-2**,
recenser les instances d'**A104** — les cas dont la référence est construite avec ce qu'ils
devraient contrôler.

**Sorties.** [`AUDIT-REFERENCES-S62`](../docs/registres/AUDIT-REFERENCES-S62.md), la fenêtre
statistique rendue déclarable, **A184**, **L180**, action **S62-1**. **S58-2 close.**

**Le recensement d'abord, et il rassure.** Quarante et une références, trois degrés — tautologie,
aller-retour, indépendance — et **un seul est une faute**. Vingt-quatre références valent `0` ou
`1` et ne peuvent rien tirer d'un paramètre ; `C04` et `C03` se comparent à des solutions fermées
que le solveur ignore ; `C02-c` confronte deux **mesures**, `λ` et `T` relevés dans le champ, à la
relation de dispersion. La faute de S58 sur C10 (**A180**) reste la seule tautologie du corpus.

**Mais le balayage de `Hs` a trouvé autre chose que ce qu'il cherchait.**

*Aveugle à ce qu'il nomme* : sur `hs` = 0,6 · 1,2 · 2,4 · 4,8, le rapport mesure/référence vaut
**0,914723 à chaque fois**. La mer est engendrée d'après `hs` et la mesure le reconstruit — le cas
teste une chaîne, jamais une valeur.

*Gouverné par ce qu'il ne nomme pas* : la fenêtre rapportée à la longueur d'onde de pic. C'est
**A102**, énoncé en S21 et **jamais mesuré**. Le voici — 8,53 % à 6,8 λ, 2,43 % à 27,3 λ,
**0,28 % à 54,7 λ**. *« Plusieurs fois la plus longue onde » est insuffisant : il en faut une
cinquantaine.* Par `tp`, à fenêtre nominale : 0,018 % à 4 s, 8,53 % à 6 s, **15,58 % à 9 s —
échec**. Et par nombre de composantes, rien ne converge : 1,42 · 10,24 · 8,53 · 12,83 · 6,53 ·
**26,30 %** de 8 à 256. **Raffiner la configuration fait échouer le cas.**

**Ce que cela change.** Le cas est décrit comme *« le contrôle de bout en bout de toute la chaîne
d'amplitude »* ; à sa fenêtre nominale il consomme **85 % de sa tolérance**, et ce qu'il mesure là
est la taille de sa fenêtre. *La chaîne, elle, est juste à 0,28 %* — et **sans le témoin à grande
fenêtre, les 8,53 % nominaux étaient indiscernables d'un défaut de sommation**. Ce témoin manquait.

**Et la fenêtre n'était pas dans le scénario.** `main.rs` passait `128` et `3.0` en dur : les deux
nombres qui gouvernent la mesure la plus fragile du harnais étaient invisibles depuis le fichier
qui affirme, dans son en-tête, être *auto-suffisant et lisible sans le code sous les yeux*.
**C'est A104 dans sa forme littérale** — une règle violée dans le fichier qui l'énonce — et c'est
**A184**. Corrigé : `physics.fenetre_cote` et `physics.fenetre_pas_m` sont déclarables, défauts
inchangés ; nommer une constante ne la déplace pas.

**Une erreur de méthode, corrigée en séance.** Le premier test figeait le rapport à 0,914723. Il a
échoué : dans le montage du test — autre instant, autre graine — il vaut **0,688**. *Le rapport
n'est pas une constante du système, c'est une propriété de l'échantillonnage d'une réalisation*
(**L180**). Le test est devenu structurel : invariance en `hs`, et division de l'écart par plus de
quatre quand la fenêtre s'élargit.

**Ce qui n'a pas été fait.** La fenêtre à retenir n'est pas tranchée : l'élargir rendrait `Hs`
juste à 0,28 % mais multiplierait son coût par 64, et à 12288 m la variance en une passe rend
`NaN` — le cas échoue alors correctement, sans faux succès. C'est **S62-1**.

**Validation.** **130 tests réussis** (40 cœur + 90 harnais), deux ignorés. Release compilée ;
hashs `check` inchangés ; **aucune valeur nominale déplacée** — les 8,528 % restent publiés sous
une tolérance inchangée. Ce qui change est ce qu'on en dit.

**Suite S63 : S59-1**, reprendre les prescriptions non éprouvées laissées par les rapports
antérieurs — A181 en a coûté une, personne ne sait combien il en reste. Puis **S62-1** et
**S58-1**. A103 close en S58 ; l'état réel et l'infrastructure restent hors de portée.

## S63 — 2026-09-08 — Le banc décisif était lisible comme hors d'atteinte

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ 5aac200. Entrée : **S59-1**,
recenser les prescriptions non éprouvées — les consignes écrites par une session qui ne subissait
pas encore le cas. Aucun calcul lancé : la session est documentaire.

**Sorties.** [`PRESCRIPTIONS-S63`](../docs/registres/PRESCRIPTIONS-S63.md), le tableau des
préalables de B2 corrigé et daté, une règle ajoutée au rituel de fin, **A185**, **L181**.
**S59-1 close.**

**Le premier apport est un tri, et il corrige l'énoncé de l'action.** Trois genres de texte se
ressemblent et ne se valent pas. Une **condition de réversibilité** — *« si la réponse était
l'inverse, il faudrait rouvrir X »*, cinq dans ADR-027, une dans ADR-048 — n'est pas une recette :
il n'y a rien à exécuter, et c'est un dispositif voulu (**L155**). Une **anticipation de
conception** ne se donne pas pour vérifiée. Seule la **recette procédurale** se vérifie, et
S59-1 les confondait.

**Le recensement rassure et inquiète à la fois.** Il existe **peu** de recettes, et aucune autre
que les trois connues n'annonce une action conditionnelle non éprouvée. Mais **les trois recettes
que le projet a effectivement mises à l'épreuve étaient fautives**, par trois mécanismes
différents : fausse dès l'écriture (A181, le découpage temporel), prescrite sans être chiffrée
(S60-1, le régime impossible), et — nouveau — **périmée en silence**.

**Le troisième mécanisme est le résultat de la séance.** `DOSSIER-B2` §8, *« ce qui doit être vrai
avant de lancer »*, datait de S14 et annonçait cinq blocages. **Quatre étaient levés depuis une
quarantaine de sessions** : H1 et H3 écrits en S20, C01 passé en S22 et S36, ADR-020 **actée en
S19**. Rien ne l'avait signalé. Une session qui aurait lu ce tableau pour décider s'il faut lancer
le banc décisif de `λ_cut` aurait conclu qu'il est hors d'atteinte — **alors qu'il ne manque
qu'une pièce** (**A185**).

**Et la cinquième ligne était mal qualifiée, ce qui compte davantage.** C02 n'est pas « non
exécuté » : il est **inexécutable** avec ce que le projet possède. Les deux `δ` d'essai sont
Saint-Venant, non dispersifs ; et le milieu à dispersion exacte de S39 ne convient pas non plus —
son en-tête pose qu'il n'est pas un solveur `δ`, et sa dispersion étant exacte, son erreur serait
nulle. *« Non exécuté » invite à exécuter ; « inexécutable » dit qu'il manque une pièce de
conception* (**L181**). Ce qui manque est une couche dispersive du projet.

**La règle, et elle est bon marché.** Le rituel fait vérifier les **décomptes** recopiés depuis
S07 et S10. Il ne faisait pas vérifier les **états** recopiés, qui se périment identiquement. Le
remède n'est pas de tout relire à chaque session — c'est de **dater** : *un état sans date se lit
au présent, et il ne l'est plus.* Le tableau de B2 porte désormais une colonne « constaté » par
ligne et un encadré de vérification daté.

**Ce qui n'a pas été fait.** `PLAN-BENCHMARK` n'est pas corrigé : ses prescriptions sont des
anticipations de conception, pas des recettes, et elles n'annoncent aucun état vérifié. Le journal
n'a pas été passé au peigne : il raconte au lieu de prescrire, et **L55** avait déjà établi qu'une
annonce en prose n'est pas une tâche.

**Validation.** Aucun code modifié, aucun calcul lancé. 130 tests réussis, deux ignorés, inchangés
depuis S62 ; hashs `check` inchangés.

**Suite S64 : S62-1**, trancher la fenêtre d'échantillonnage de `Hs` par ADR, après avoir corrigé
la sommation de variance. Puis **S58-1**. Et une question que ce recensement met en avant sans la
trancher : **le seul blocage réel de B2 est une couche dispersive**, qui n'a jamais été planifiée.
A103 close en S58 ; l'état réel et l'infrastructure restent hors de portée.

## S64 — 2026-09-08 — Une action, deux motifs techniques, tous deux faux

**Agent : Claude Code (Opus 5)**, git et cargo disponibles. Départ eacd9d9. Entrée : **S62-1**,
corriger la sommation de variance puis trancher la fenêtre d'échantillonnage de `Hs`.

**Sorties.** ADR-051, la fenêtre portée à 3072 m dans le scénario, le comptage des points hors de
portée de l'ancre, **A186**, **A187**, **L182**, actions **S64-1** et **S64-2**. **S62-1 close.**

**Le premier geste a été de refuser l'explication non mesurée**, et il a payé. S62 attribuait le
`NaN` à l'annulation catastrophique de la variance en une passe, et prescrivait donc Welford. **Le
`NaN` vient de `to_local`**, qui refuse tout point à plus de **4096 m de l'ancre** — mesuré à six
mètres près : demi-fenêtre 4092 m rend 0,505 %, demi-fenêtre 4098 m rend `NaN`. La correction
prescrite n'avait pas lieu d'être.

**Le second motif de l'action était faux aussi.** « 64 fois le coût » : mode `physics` complet,
**33,9 s** au nominal contre **35,9 s** avec la fenêtre 64 fois plus grande — **+6 %**.
L'échantillonnage de `Hs` est marginal devant les solveurs. *Une action, deux raisons techniques,
aucune éprouvée, toutes deux fausses* — le défaut que **S63** venait de recenser (**A181**,
**L177**), commis par la session immédiatement précédente, dans l'action même que celle-ci
exécutait.

**Décision structurante.** La fenêtre passe à **3072 m**, soit 54,7 longueurs d'onde de pic. Le
chiffre publié pour `Hs` passe de **8,528 %** à **0,282 %**, et surtout : à `tp = 9 s`, le cas
**échouait** à l'ancienne fenêtre — 15,58 % — et rend 0,184 % à la nouvelle. **La correction répare
un cas qui échouait ailleurs.**

**Et la conséquence gênante, écrite plutôt que découverte plus tard** : la tolérance de 10 %
restant inchangée, **le cas devient plus juste et moins sévère**. Avant, une erreur de chaîne de
2 % portait l'écart à 10,5 % et le faisait tomber ; désormais elle le porte à 2,3 % et il passe.
La marge réelle passe de 1,5 point à 9,7.

**Deux faits mesurés interdisent de resserrer la tolérance.** À **256 composantes**, l'écart vaut
**6,612 %** — et ce n'est ni la fenêtre ni le pas : 6,612 / 6,614 / 6,615 % à pas 3,0 / 1,5 / 1,0 m
à fenêtre égale. **Cause non identifiée** (**A187**). Et aucune barre d'erreur n'est mesurable :
**il n'existe qu'une réalisation par état de mer**. Le déphasage est dérivé de l'indice, sans PRNG,
et le paramètre `graine` du scénario, **obligatoire**, n'est utilisé **nulle part** — six graines
rendent six fois le même chiffre à la sixième décimale (**A186**). Une tolérance calibrée sur un
échantillon unique n'aurait pas de provenance (**A106**).

**Le refus, lui, est complété.** Les points hors de portée sont **comptés** et le motif est écrit
dans le libellé du cas. La mesure reste refusée : écarter les points invalides et mesurer sur le
reste fabriquerait une variance sur un domaine amputé — la faute corrigée en S45 sur C10
(**A173**, **L166**).

**Validation.** **131 tests réussis** (40 cœur + 91 harnais), deux ignorés. Release compilée ;
hashs `check` **inchangés**. Campagne `physics` inchangée par ailleurs : seul C04 à l'ordre un
échoue, comme voulu.

**Suite S65 : S64-1**, brancher la graine sur les phases — déterminisme conservé, réalisations
multiples possibles — ce qui débloque **S64-2**, la tolérance. Puis **A187**. Reste **S58-1**, et
surtout **S63-1** : la couche dispersive, seul blocage réel de B2. A103 close en S58 ; l'état réel
et l'infrastructure restent hors de portée.


## S65 — 2026-09-08 — La graine produit enfin une autre mer

Agent Codex, git/cargo disponibles. Départ 6a128e7, S64 achevée par Claude Code ; copie 29ef50
propre, avancée à chaque étape vers master. Horloge locale antérieure au battement S64 : jeton
libre et git terminé vérifiés. S64-1 réalisée : SeaState.graine, raccord scénario, phases
SplitMix64 indexées sans état partagé. Formule, source et mesures dans GRAINES-S65.

133 tests passent (41 cœur + 92 harnais), trois ignorés. Diagnostic explicitement exécuté :
douze réalisations finies. Hs nominal +1,38833 %, six graines : −2,435 à +1,388 % ; à 256
composantes : −7,270 à +5,844 %. A187 reste ouvert, aucune calibration de tolérance effectuée.
Nouvel échec homogénéité : ratio 1,397507 face à 15 %, conservé. C04 ordre un reste en échec.
Check vert après inscription des hashs dans 7723ab8 : C02 0x9babd7e12935c263 et C18
0xd57d81f47d9f8611. Reproductibilité locale vérifiée, conformité interplateforme non mesurée.

A186 corrigé pour le paramètre mort ; aucune preuve statistique tirée de six graines. Aucun
nouvel angle ni leçon : application de L180/L182 et ADR-003. Invariants préservés ; notes
correctives datées, aucun ADR réécrit. 51 ADR, six SPEC, seize registres, 187 angles, L182.
Suite S66 : S65-1, diagnostic homogénéité, puis S64-3 et S64-2 par ADR. S63-1, couche
dispersive, reste le seul blocage réel de B2. A103 et C22 clos ; aucun distant créé.

## S66 — 2026-09-08 — Le refus était une interférence, pas une perte de précision

Agent Codex, git/cargo disponibles. Départ master 25d3a9e propre ; copie 29ef50 identique,
avancée à chaque étape. Entrée S65-1 : homogénéité nominale en échec après graine effective.

Diagnostic de test sur les mêmes composantes : calcul spatial et sommation f64, variance totale
et somme des variances individuelles. Six graines, trois fenêtres 144/576/1536 m, deux positions
0/3000 m, dix-huit couples ; 9,46 s en release. Les fenêtres restent dans la portée 4096 m.
Au nominal : ratio 1,397506641 en production, 1,397506306 en f64. La contribution croisée passe
de 0,006459304 à 0,046176100 m² ; elle explique la hausse de variance. La somme des variances
individuelles baisse légèrement, ratio 0,979715316. La précision ne cause pas le refus.

Quatre graines sur six échouent à 144 m, trois à 576 m, aucune à 1536 m ; effet non monotone
par graine, aucune calibration déduite. Test ordinaire de causalité, témoin monochromatique à
covariance nulle. 134 tests réussis, quatre ignorés ; diagnostic ignoré exécuté séparément.
Check : hashs inchangés 0x9babd7e12935c263 / 0xd57d81f47d9f8611. Production inchangée :
C04 et homogénéité restent en échec, campagne générale non répétée. HOMOGENEITE-S66 consigne
montage, résultats et limites de la référence partageant la table spectrale.

A188 (sévérité 2), attribution causale corrigée ; L183, propriété d’ensemble contre réalisation
finie. S65-1 close ; S66-1 ouverte pour décider la séparation contrôle de précision / diagnostic
statistique. Aucun seuil ni invariant modifié, aucune API publique nouvelle, aucun ADR réécrit.
51 ADR, 17 invariants, six SPEC, seize registres, 23 cas, 188 angles ; dernière leçon L183.

Suite S67 : S64-3, expliquer A187 à 256 composantes avec la décomposition mesurée, sans
supposer que le résultat à 32 composantes suffit. Puis instruire S66-1 et S64-2 par ADR.
S63-1, couche dispersive, demeure le blocage B2. A103 et C22 clos ; aucun distant créé.

## S67 — 2026-09-08 — La plus longue onde ne donne pas le plus long battement

Agent Codex, git/cargo disponibles. Master 87d3186 et copie 29ef50 propres et identiques,
copie avancée à chaque étape. Entrée S64-3 : cause de Hs à 256 composantes, A187.

Ajout sous cfg(test) de moments exacts sur grille finie par sommes géométriques et produits
trigonométriques. Vérifiés contre sommation directe à 1/32/256 composantes, fréquence nulle
et alias exact. 28 montages mesurés : 32/256 composantes, phases historiques et six graines,
fenêtres 3072/6144 m ; deux vérifications directes sur 1024² points. Diagnostic 49,40 s.

A187 reproduit : Hs historique 1,279349902 en production, 1,279349889 par formule. Somme des
variances individuelles -> Hs 1,200001971 ; covariance croisée 0,012295713 m², dont 73,1 %
entre voisines. Battements voisins jusqu’à 20,208 km à 256 composantes contre 2,361 km à 32.
À 6144 m, Hs historique 1,218126498 : la fenêtre compte, contrairement à la formulation S64.
SPECTRE-DENSE-S67 conserve les chiffres et la portée ; ni normalisation ni précision ne causent
cet écart. Aucun seuil ni paramètre de production changé, pas de calibration sur six graines.

135 tests réussis (43 cœur + 92 harnais), cinq ignorés ; diagnostic exécuté séparément.
Check vert et hashs inchangés. Physics général non répété, chemins de production inchangés.
A187 expliqué, S64-3 close. L184 ajoutée, pas de nouvel angle. Notes correctives ADR-051,
registre et cas canoniques ; I-08 respecté, aucun invariant invalidé, aucun ADR réécrit.
51 ADR, 17 invariants, six SPEC, seize registres, 23 cas, 188 angles ; dernière leçon L184.

Suite S68 : S66-1, décider par ADR la séparation du contrôle de précision et du diagnostic
statistique ; puis S64-2, désormais à instruire avec une cause connue et des ensembles possibles.
S63-1 demeure le seul blocage B2 : couche dispersive à planifier. A103/C22 clos, aucun distant créé.

## S68 — 2026-09-08 — Deux contrats au lieu d’une attribution

Agent Codex, git/cargo disponibles. Départ master 3734fa4 propre, copie 29ef50 identique,
avancée à chaque étape. S66-1 : décider le devenir du contrôle après S66/S67.

ADR-052 sépare une assertion directe de précision spatiale et un diagnostic statistique
sans verdict. Le ratio reste affiché et compté, un NaN reste un refus compté en échec.
La primitive spatiale appelée par eval est partagée avec l’audit ; score erreur circulaire
sur borne gamma3 Σ|k·coord·direction| + u + 2^-32, seuil dérivé 1 avant mesure. 49 positions,
toutes les composantes ; témoin 256 composantes et phase dégradée à quatre bits éprouvés.
Le contrôle ne prouve ni le spectre partagé ni la phase temporelle ni toute la chaîne.

C02/C18 : scores 0,227536 / 0,650281, diagnostics 0,979548732 / 1,397506641 inchangés.
Le second ne devient pas une statistique validée. 137 tests passent (44 cœur + 93 harnais),
cinq ignorés ; release compilée. Check : hashs inchangés 0x9babd7e12935c263 /
0xd57d81f47d9f8611. Physics terminé avec sortie 1 pour C04 ordre un seulement ; nombre
d’assertions conservé par substitution et un diagnostic explicite ajouté par scénario.
CONTROLES-S68 contient portée et reproduction ; tests de refus et défaut injecté réussis.

S66-1 close, A188 traité. Aucun nouvel angle ni leçon : application de L183, contrat séparé.
52 ADR, 17 invariants, six SPEC, seize registres, 23 cas, 188 angles ; L184 dernière leçon.
I-02/I-03/I-06/I-08 relus, pas de changement de leurs exigences ; aucune allocation dans audit,
aucun état sérialisé. Hs, sa fenêtre et sa tolérance restent sous ADR-051.

Suite S69 : S64-2, instruire calibration statistique et décision de tolérance par ADR.
S63-1 demeure le blocage B2 : couche dispersive à planifier. A103/C22 clos, aucun distant créé.

## S69 — 2026-09-08 — Vingt-deux sessions sur l'instrument, zéro sur l'eau

**Agent : Claude Code (Opus 5)**. Départ e839c77 ; S65 à S68 conduites par Codex, fusionnées,
rien en cours. Entrée : **demande de l'utilisateur** — évaluer le taux de progression et les
étapes futures. S64-2 est décalée à S70.

**Sortie.** [`BILAN-S69`](../docs/registres/BILAN-S69.md). Aucun code modifié, aucun calcul lancé.

**Précaution de méthode.** Les colonnes « État » des tableaux d'actions antérieurs à S45 disent
« ouverte » pour des actions closes par une note en prose (S35-1, S35-2). **A185** interdisait de
s'y fier ; le bilan s'appuie sur les compteurs de fichiers, la sortie du harnais et le chemin
critique.

**Le chiffre dépend de ce qu'on appelle le projet.** Comme corpus de conception — ce que le dépôt
dit être — **~85 %**. Comme système utilisable dans un jeu — **~15 %**. Le harnais est solide, la
conception est mûre, et **rien de ce qui doit tourner dans le jeu n'est écrit** : `δ`, `W` et `V`
n'existent pas, et les deux véhicules 1D ne sont pas le système.

**Le fait central, et il est vérifiable dans le journal.** De **S47 à S68, aucune session n'a
produit de conception du système d'eau** : quinze sur le dossier C22, sept sur `Hs`, la graine et
le spectre dense. Ce travail n'est pas perdu — il a fait conclure C22, mesuré A102 après quarante
sessions, réconcilié quatre forks, et surtout construit la fiabilité de l'instrument. Mais son
rendement se dégrade, et le corpus le mesure lui-même : S61 a établi que quatre campagnes avaient
mesuré ce qu'un rapport d'entiers donnait, S64 que deux motifs de report étaient faux, S63 que les
préalables du banc décisif étaient périmés depuis quarante sessions.

**Le goulot.** *Le projet ne peut plus progresser par la mesure, parce que ce qu'il faudrait
mesurer n'existe pas.* Le harnais l'imprime à chaque exécution : **onze cas sur 23** et **onze
bancs sur onze** attendent une couche non écrite.

**Ce que le bilan trouve, et qui est actionnable tout de suite** : **B1 est le seul des onze bancs
exécutable en l'état** — `B` existe, le harnais mesure, et depuis S65 la graine produit des
réalisations indépendantes. Il n'a jamais été lancé. Et **A187** vient de le rendre urgent : le
nombre de composantes n'est plus seulement une question de coût mais de **justesse**.

**Ce qui n'est pas de ma portée**, et qui est remonté à l'humain : décider si le projet passe à la
construction ou reste un corpus de conception. *Ne pas choisir revient à choisir le second par
défaut* — c'est ce qui s'est produit pendant vingt-deux sessions.

**Suite S70 : S64-2**, la calibration statistique, comme prévu avant ce bilan. Mais la
recommandation du bilan est **B1** avant toute nouvelle session de raffinement, et **S63-1** pour
débloquer B2. A103 close en S58 ; l'état réel et l'infrastructure restent hors de portée.

## S70 — 2026-09-08 — La construction est décidée, et elle commence par W

**Agent : Claude Code (Opus 5)**. Départ 1afbb3a. Entrée : **arbitrage de l'utilisateur**, rendu
le jour même en réponse au point 4 de `BILAN-S69` §6 — *« Je ne savais pas, donc on passe à la
construction. »*

**Sortie.** [`ADR-053`](../docs/adr/ADR-053-le-projet-passe-a-la-construction.md), **ACTÉE** —
le deuxième ADR acté du dépôt après ADR-020. Aucun code écrit dans cette session : elle
enregistre une décision.

**Ce qui est tranché.** Le projet construit le système. La réserve que chaque module de `code/`
porte depuis S20 — *ceci n'est pas le système* — tombe pour les couches du projet : ce qui sera
écrit désormais est jugé par les cas canoniques, pas par sa ressemblance à un véhicule d'essai.

**Par quelle couche, et pourquoi ce n'était pas un choix ouvert.** `W` est désignée par **quatre
besoins sans rapport entre eux** : elle est la couche dispersive que **S63-1** cherche depuis S22
— donc elle débloque C02, `λ_cut` et **B2** ; elle débloque **C07** et **C19** ; elle est
**analytique**, des paquets dérivés d'événements horodatés, quand `δ` est un solveur 3D et la
pièce la plus lourde du projet ; et son format réseau **`WaveEvent`** est *la seule urgence de
format du corpus*, qu'écrire `W` force à trancher. C'est le seul point du graphe où quatre besoins
indépendants tombent sur la même pièce.

**Et B1 passe devant, pour une raison de dépendance.** `W` se superpose à `B`, dont le nombre de
composantes conditionne le coût d'évaluation que `W` paiera à chaque point — question devenue de
**justesse** et non de coût depuis **A187**. B1 reste le seul banc exécutable sans écrire une
ligne de couche, et il n'a jamais été lancé.

**Ce que la décision ne dit pas.** Aucune technologie pour `W` : paquets lagrangiens, champ 2D
GPU, ou les deux — ADR-001 §2 laisse les deux ouverts et **B2 est le banc qui tranche**. Écrire
`W` commence donc par ce que B2 exige de comparable, pas par un choix d'implémentation. Et aucun
calendrier : l'état réel du projet reste hors de portée d'une session.

**Validation.** Aucun code modifié, aucun calcul. 137 tests, cinq ignorés, inchangés depuis S68.

**Suite S71 : B1**, le banc du champ de fond — nombre de composantes et coût d'évaluation. Puis
arrêter `WaveEvent`, puis écrire `W`. A103 close en S58 ; l'état réel et l'infrastructure restent
hors de portée, aucun distant créé.


## S71 — 2026-09-08 — Construction validée, faux préalables retirés

**Entrée :** demande de validation autonome d’ADR-053 ; master et copie 29ef50 à 14b4b7a.
**Décision :** ADR-054 actée par délégation technique. Construction et priorité W confirmées.
B1 informe le budget sans bloquer WaveEvent. V dispose de C12 sans δ ; C19 entier exige aussi V.
A187 est expliqué depuis S67. Une loi dispersive exacte ne valide pas la coupure numérique de δ.
**Sortie :** cinq lots avec réception : événement, journal, propagation, intégration, sélection B2.
Premier candidat CPU analytique à milieu uniforme, sans prétendre avoir gagné B2.
**Chiffres :** B1 demande 32/64/128/256 composantes et des volets LOD/perceptuels absents ;
50 octets de champs WaveEvent ne fixent pas encore un protocole évolutif.
**Validation :** comparaison des décisions aux invariants et protocoles, liens locaux et diff.
Aucun code changé ; tests non relancés, dernier état rapporté S68 : 137 réussis, cinq ignorés.
**Apprentissage :** A189, L185. Aucun lot W implémenté, aucun banc supplémentaire validé.
**Suite S72 :** S70-2 / W1, contrat versionné et code de validation des événements.
S70-1 reste partielle à construire ; S70-3 et S63-1 restent ouvertes. Pas d’arbitrage technique
renvoyé à une équipe ; état réel moteur/protocole/matériel cible toujours non constaté.

## S72 — 2026-09-08 — Impact V1 devient du code

**Entrée :** Go utilisateur, suite W1 de S71 ; départ 399830b, copies actives identiques.
**Décision :** ADR-055 actée, première tranche Impact de WaveEvent. 76 octets explicites,
f32 locaux au lieu de half, durée entière ; provenance locale refusée sur decode_server.
**Produit :** constructeur validé, encodage/décodage sans allocation, quatre tests de contrat.
Vecteur d’octets indépendant, refus des enveloppes et scalaires invalides, limites temporelles,
canonicalisation et grands événements représentables. Aucune saturation silencieuse.
**Limites :** autres kinds refusés ; aucune propagation, journal, authentification ou preuve
interplateforme. W1 reste partielle. A190 expose la corrélation absente des identités prédites.
**Suite S73 :** S72-1, identité et retrait, puis journal Impact borné. S72-2 porte la rétention.
L186 enregistrée ; aucun arbitrage reporté à une équipe. Résultats de vérification ci-dessous.

**Vérification S72 :** cargo test complet : 48 core + 93 harnais = 141 réussis, cinq ignorés ; aucun échec. Quatre avertissements préexistants dans le harnais. Diff vérifié ; code formaté. Copies synchronisées après commit.

## S73 — 2026-09-08 — La cause et le journal deviennent exécutables

**Entrée :** Continue utilisateur ; départ 8ae26b4, copies actives identiques.
**Décision :** ADR-056, cause gameplay distincte du server_seq, époque serveur contrôlée.
**Produit :** journal sur mémoire prêtée, sans allocation ; déduplication, conflits atomiques,
confirmation remplaçant une prédiction à pool plein, rejet terminal et ordre autoritaire stable.
Cinq tests ciblés passent : replay encodé avec causes, désordre, rejets, saturation et provenance.
**Limites :** sans enveloppe persistée, bus concurrent, purge ni propagation ; W2 partielle.
A190 résolu au niveau interne ; A191 expose la complétude après saturation. L187 enregistrée.
**Suite S74 :** S73-1, enveloppe de restauration et état de complétude. S72-2 reste ouverte.
Les causes doivent être fournies par le gameplay authentifié, pas inventées depuis les arrivées.

**Vérification :** 53 tests core + 93 harnais = 146 réussis, cinq ignorés, aucun échec ; quatre avertissements préexistants du harnais. Diff vérifié. Copies synchronisées après commit.

## S74 — 2026-09-08 — Le journal se sauvegarde et se restaure

**Entrée :** Continue utilisateur ; départ 2483f8e, copies actives identiques.
**Décision :** ADR-057. Perte connue mémorisée au refus Full, persistée sans prétendre prouver
la complétude réseau. Format WJNL V1 : 24 octets + 97 par enregistrement, causes et rejets inclus.
**Produit :** save dans mémoire hôte ; restore via temporaire fourni, puis remplacement unique.
Erreur tardive : journal vivant inchangé. Quatre nouveaux tests, neuf ciblés réussis.
**Limites :** pas de fichiers ni transport, pas de purge ni propagation, pas de preuve globale
C19. Une restauration zéro perte requiert une référence hôte pertinente, pas seulement valide.
**Suite S75 :** S74-1, premier impact propagé ; S72-2 rétention et S74-2 intégration ouvertes.
A191 traité localement, L188 enregistrée. Aucun nouvel angle numéroté ni invariant amendé.

**Vérification finale :** 57 tests core + 93 harnais = 150 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Format et diff vérifiés ; copies synchronisées après commit.

## S75 — 2026-09-08 — Première expansion dispersive Impact

**Entrée :** Continue utilisateur ; départ b438b1e, copies actives identiques.
**Décision :** ADR-058, 40 modes profonds sur carré périodique ; premier support analytique
explicite, pas encore paquet régional. Gravité/densité injectées, énergie normalisée.
**Produit :** impact_field sans allocation, élévation, dérivée, potentiel et pente, temps entier.
Tests : énergie intégrée indépendante, volume nul, fréquence par projection spatiale, refus.
100 J refusés par la borne de pente du test ; scénario réduit à 1 J, contrôle conservé.
**Limites :** répétition périodique, isotropie et vitesse de groupe non reçues ; pas de
raccordement B+W ni journal, pas de réception B2/C19. S74-1 reste partielle.
**Suite S76 :** S75-1, transport radial et retours, puis support régional. L189 enregistrée.
Aucun nouvel angle numéroté, aucun invariant amendé. Résultats complets ci-dessous.

**Vérification finale :** 60 tests core + 93 harnais = 153 réussis, cinq ignorés, aucun échec ; quatre avertissements préexistants du harnais. Diff vérifié, copies synchronisées après commit.

## S76 — 2026-09-08 — La répétition existe avant tout retour

**Entrée :** Continue utilisateur ; départ 466ba65, copies actives identiques.
**Mesure :** densité physique positive intégrée en profondeur ; grille 32/64, sept instants.
Rayon moyen 2,217 m à naissance, 7,040 à 8 s, 6,293 à 12 s. Copie exacte à 16 m dès t=0.
Énergie 1,00000003 à 1,00000005 J. Rapport TRANSPORT-RADIAL-S76 reproductible.
**Décision :** ADR-059, support périodique refusé comme impact isolé. Prochain candidat radial
non périodique dans sa définition continue, quadrature bornée à recevoir ; pas de sélection B2.
**Correction :** identité énergétique intégrée distincte de densité locale, L190. Témoin
initial de déplacement à 2 s réfuté ; reporté explicitement à 4 s comme régression seulement.
**Suite S77 :** S76-1, construire Hankel et son domaine. S75-1 close ; W3 et rétention ouvertes.
Pas de nouvelle couche régionale cette session ; aucune prétention de vitesse de groupe validée.
Aucun nouvel angle numéroté ni invariant amendé.

**Vérification finale :** 61 tests core + 93 harnais = 154 réussis, cinq ignorés, aucun échec. Diagnostic release passé, quatre avertissements préexistants du harnais. Diff vérifié ; copies synchronisées après commit.

## S77 — 2026-09-08 — Candidat radial sans pavage

**Entrée :** Reprends utilisateur ; départ e70e67d, copies actives identiques.
**Produit :** radial_impact : quadrature de Hankel à N nœuds, Bessel par directions fixes,
source profonde isotrope et énergie normalisée par intégrale fermée ; aucune allocation.
**Résultats :** écart N64/128 au pic 1,3e-7 ; ancien emplacement de copie/pic 0,00003731.
Énergie initiale dans disque 16 m : 1,00052365 puis 1,00012767 de la prescription, 256/512 anneaux.
Volumes tronqués -3,7e-7 et -1,15e-6 m³, diagnostics seulement. ADR-060 contient protocole et limites.
**Décision :** S76-1 réalisée en tant que candidat, aucun domaine physique universel certifié.
Contrôle de résolution distinct de réception, L191. Pas de nouveau numéro d’angle ni invariant.
**Suite S78 :** S77-1, énergie dans le temps et transport radial, puis réception et coût.
B+W, journal→champ, autorité croisée et rétention restent ouverts. Quatre tests ciblés réussis.

**Vérification finale :** 65 tests core + 93 harnais = 158 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Tests ciblés release et diff vérifiés ; copies synchronisées après commit.

## S78 — 2026-09-08 — Le déficit sort du disque

**Entrée :** Continue utilisateur ; départ e872a3c, copies actives identiques.
**Mesure :** densité positive avec intégration analytique en profondeur, quatre instants,
trois rayons et deux raffinements. À 4 s, R8 contient 99,319 % ; R20 récupère 100,000146 %.
Rayon moyen : 1,160 à 4,457 m. Écart spectral <1e-7, biais des anneaux identifié.
**Décision :** ADR-061 : scénario reçu pour poursuivre l’intégration limitée ; pas de
compensation d’un déficit dû au transport. Pas de réception universelle ni sélection B2.
**Produit :** contrôle temporel reproductible et BILAN-RADIAL-S78. Aucune tolérance modifiée.
**Suite S79 :** S78-1, vitesse orbitale puis composition B+W ; S77-1 close sur le scénario.
Rétention, autres profils, budget et autorité croisée ouverts. L192, aucun angle numéroté ajouté.

**Vérification finale :** 66 tests core + 93 harnais = 159 réussis, cinq ignorés, aucun échec. Diagnostic release passé ; quatre avertissements préexistants du harnais. Diff vérifié, copies synchronisées après commit.

## S79 — 2026-09-08 — Vitesses et composition B+W

**Entrée :** Continue utilisateur ; départ 2734123, copies actives identiques.
**Défaut :** vitesse verticale B opposée à deta_dt ; corrigée après lecture de la convention
kx-omega t, contrôlée par différences temporelles. A192, L193, ADR-062.
**Produit :** vitesse horizontale radiale et support périodique ; composition ponctuelle avec
journal confirmé, ordre et contenu exacts, refus de perte/domaine, normale reconstruite.
Test avec Background réel ; aucune construction de champ pendant compose, aucune allocation.
**Références :** C02 0x0a3a3bcc945db263, C18 0x85c8bc610f551d11 ; anciens hashs invalidés par
la correction verticale, deux check réussis après inscription. C04 non modifié.
**Limites :** l’hôte garantit la correspondance B/W du point et du milieu ; service en lot,
préparation, index, réseau et rétention restent à construire. Pas de généralisation physique.
**Suite S80 :** S79-1, préparation bornée et lot ; S78-1 réalisée au niveau ponctuel.

**Vérification finale :** 71 tests core + 93 harnais = 164 réussis, cinq ignorés, aucun échec. C02/C18 check : deux succès. Quatre avertissements préexistants dans la compilation des tests du harnais. Diff vérifié ; copies synchronisées après commit.

## S80 — 2026-09-08 — Préparation bornée et interrogation par lots

**Entrée :** Continue utilisateur ; départ cc1915e, copies actives identiques.
**Produit :** prepared_water construit les confirmations une fois dans le pool hôte ; emprunt
immuable du journal interdisant sa mutation pendant la vie de la préparation. Lot évalué dans
un temporaire puis copié en sortie après succès complet. Aucun résultat partiel sur erreur.
**Contrôles :** quatre tests préparation/lot, refus tardifs, queue conservée et journal vide.
Compose refuse aussi un point NaN sans champ W. Pas de changement physique ni de hash B attendu.
**Décision :** ADR-063, S79-1 réalisée localement ; stockage de préparation distinct de publication.
L194 enregistrée ; aucun angle numéroté supplémentaire, aucun invariant amendé.
**Suite S81 :** S80-1, mesurer le chemin réel B+W et optimiser le coût dominant. Préconditions
B, bus, index et rétention ouverts. Aucun budget matériel cible proclamé.

**Vérification finale :** 75 tests core + 93 harnais = 168 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants des tests du harnais. Tests ciblés release et diff vérifiés ; copies synchronisées après commit.

## S81 — 2026-09-08 — Le coût vient de Bessel

**Entrée :** Continue puis Reprends en cours ; départ a5385e2, copies actives identiques.
**Produit :** exemple bench_water mesurant préparation, B réel et lot B+W, pools et hashs.
**Mesures :** 1×64 points 17,349→9,956 ms médian ; 16×64 277,877→159,158 ms.
Préparation 0,6 à 9,8 µs après, B seul environ 57 µs pour 64 points. Poste local seulement.
**Optimisation :** 128 directions constantes tabulées en bits, 512 octets ; aucun changement
physique. Quatre empreintes de lots inchangées. Pas de nouvel ADR car décision physique inchangée.
**Suite S82 :** S81-1, candidat Bessel accéléré avec erreur reçue ; ne pas consacrer une
session à optimiser la préparation négligeable. S80-1 close. Rapport COUT-BW-S81, L195.
Aucun nouvel angle numéroté ni invariant amendé. Charge encore impropre à une promesse AAA.

**Vérification finale :** 76 tests core + 93 harnais = 169 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants des tests du harnais. Diff vérifié ; copies synchronisées après commit.

## S82 — 2026-09-08 — Bessel passe à une interpolation reçue

**Entrée :** Continue utilisateur ; départ f736683, copies actives identiques.
**Produit :** table de 1025 nœuds J0/J1/J1’, générateur et interpolation Hermite, référence
angulaire conservée. Pas de libm au runtime, mêmes refus et domaines. ADR-064 actée.
**Résultats :** erreur max 5,85699e-8 sur 8193 arguments ; huit tests radiaux release passent.
B+W 1×64 : 9,956→0,1553 ms, 16×64 : 159,158→1,5978 ms. Résultats numériques changés,
quatre hashs de lots nouveaux documentés, références B seules inchangées.
**Limites :** borne Hermite idéale distincte des arrondis, L196. Pas de réception globale ni
interplateforme. Aucun seuil physique déplacé, aucune capacité AAA déduite du poste local.
**Suite S83 :** S82-1, contexte/temps/points attachés au lot B ; arrêter optimisation ici.
S81-1 close. Aucun angle numéroté ajouté ni invariant amendé. Vérification complète ci-dessous.

**Vérification finale :** 77 tests core + 93 harnais = 170 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants des tests du harnais. Diff vérifié ; copies synchronisées après commit.

## S83 — 2026-09-08 — Une requête commune pour B et W

**Entrée :** Continue utilisateur ; départ dc0dd39, copies actives identiques.
**Produit :** BoundBackground et sample_world_batch calculent B puis W aux mêmes points/instant.
Contexte et gravité contrôlés, output transactionnel conservé ; banc raccordé au chemin commun.
**Défaut trouvé :** to_local soustrayait les i64 avant de borner ; checked_sub et extrêmes testés.
A193 corrigée, L197. Ancre décalée de 1 million de mètres, deux points/deux instants testés.
**Mesure :** quatre hashs S82 conservés ; latences en environnement occupé, pas de verdict sur
un surcoût isolé. ADR-065 distingue déclaration hôte des axes et garanties du code.
**Suite S84 :** S83-1 / S72-2, rétention et validité prolongée. S82-1 réalisée sur chemin commun.
Ancien chemin brut conservé avec préconditions, pas de réception multi-référentiels universelle.

**Vérification finale :** 78 tests core + 93 harnais = 171 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants des tests du harnais. Banc et test ciblé release vérifiés ; copies synchronisées après commit.

## S84 — 2026-09-08 — Horizon numérique indépendant du TTL

**Entrée :** Continue utilisateur ; départ 419e217, copies actives identiques.
**Produit :** valid_until et renewal_deadline exposés ; horizon radial découplé du TTL,
débordement temporel refusé. Reconstruction à N constant depuis la naissance d'origine,
second pool testé sans invalidation du premier en cas de refus. ADR-066 actée.
**Vérification :** trois tests nouveaux debug et release. Période commune identique bit à bit,
frontières 4/16 s, comparaison 128/256 après TTL et date u64 maximale. Suite complète :
81 core + 93 harnais = 174 tests réussis, cinq ignorés, aucun échec ; quatre avertissements
préexistants du harnais. Formatage des deux fichiers modifiés, diff vérifié.
**Décision :** aucune purge TTL. Conservation intégrale dans la capacité existante, saturation
visible ; ni durée arbitraire ni mémoire de rejeu durable résolues. Les comparaisons à 16 s
ne remplacent pas une réception énergétique élargie. L198, aucun angle ni invariant ajouté.
**Suite S85 :** S84-1, contrôleur de renouvellement à deux pools ; bascule transactionnelle et
état après épuisement. S83-1 et S72-2 partielles. Copies synchronisées après commit de clôture.

## S85 — 2026-09-08 — Contrôleur de renouvellement construit

**Entrée :** Continue utilisateur ; départ c8fbba5, copies actives synchronisées, jeton libre.
**Produit :** RenewalController à deux pools hôte, état Ready/Due/Expired ou vide, marge explicite,
une tentative par appel et échange après validation complète. Milieu, résolution et journal fixes.
**Vérification :** quatre tests ciblés release ; suite debug 85 core + 93 harnais = 178 réussis,
cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Deux bascules et
comparaison directe, refus après premier champ, expiration, reprise, capacité et u64 extrêmes.
Formatage et diff vérifiés. P2 : 394d869. Documentation CONTROLEUR-S85, réalisation ADR-066.
**Limites :** appel synchrone sans budget d'ordonnancement reçu ; journal figé par emprunt.
Pas de purge, de résolution adaptative ni de publication multilecteur. Aucun nouvel ADR ni
angle numéroté, invariants inchangés, L199 enregistrée. README actualisé.
**Suite S86 :** S85-1, admission transactionnelle d'un journal actualisé avec ses champs.
S84-1 close ; S72-2 partielle. Rituel exécuté, copies synchronisées après commit de clôture.

## S86 — 2026-09-08 — Admission transactionnelle journal et champs

**Entrée :** Continue utilisateur ; départ 338a2f9, copies actives identiques et jeton libre.
**Produit :** LiveWater possède deux journaux et deux pools, applique les commandes dans la
réserve puis échange la paire complète après calcul. Prédictions, confirmations, rejets et
renouvellement explicite raccordés. Commande bloquée conservée, vue courante refusée jusqu'au succès.
ADR-067 actée ; copie interne du journal conserve époque, ordre et indicateur de perte.
**Vérification :** quatre tests ciblés release ; suite debug 89 core + 93 harnais = 182 réussis,
cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Refus sur second champ,
saturation journal/champs, reprise, erreurs d'époque et horizon, comparaison directe du lot.
Formatage et diff vérifiés. P2 : 48c7d99. L200 enregistrée, aucun nouvel angle numéroté ni invariant.
**Limites :** commande unique en attente, contre-pression hôte, pas de routage spatial ni
réception réseau complète. La sauvegarde WJNL seule ne couvre pas le service bloqué.
**Suite S87 :** S86-1, sauvegarde de la paire publiée et de la commande en attente, puis reprise
sur pools élargis. S85-1 réalisée ; rétention S72-2 encore partielle. Copies synchronisées à clôture.

## S87 — 2026-09-08 — Sauvegarde du service et de son attente

**Entrée :** Continue utilisateur ; départ 1fd8c60, copies actives identiques et jeton libre.
**Produit :** WLIV V1, en-tête 168 octets et WJNL inchangé ; contexte, N, horizon et attente
sauvegardés. Restauration sur réserves, reconstruction des champs, échange après succès seulement.
Reprise sur cible plus grande sans détruire la source ; attente conservée avant tentative explicite.
ADR-068 actée ; S86-1 réalisée au niveau bibliothèque.
**Vérification :** trois tests ciblés release ; suite debug 92 core + 93 harnais = 185 réussis,
cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Trois commandes bloquées,
reprise après saturation, identité des octets et des échantillons, troncatures exhaustives du fixture,
mutations ciblées, refus tardif physique et préservation du blocage. Formatage et diff vérifiés.
**Limites :** stockage hôte fiable requis, sans checksum/authentification ni gestion des crashs disque.
Contexte et époque identiques ; aucune durée arbitraire ni purge reçue. L201, aucun nouvel angle
numéroté ni invariant modifié. P2 : 455ff14.
**Suite S88 :** S87-1, scénario hôte complet avec requêtes monde B+W, admissions, sauvegarde,
redémarrage et reprise ; coût réel. Copies synchronisées après clôture ; S72-2 reste partielle.

## S88 — 2026-09-08 — Cycle hôte complet et coût réel

**Entrée :** Continue utilisateur ; départ e8d8c01, copies actives identiques, jeton libre.
**Produit :** bench_live_water, B32 et W N128, ancre à un million de mètres, 64 points monde.
Cinq admissions, deux impacts, saturation, WLIV, destruction source, cible plus grande,
restauration bloquée puis reprise à 12 s/16 s d'horizon. Référence directe des confirmations.
**Résultats :** empreinte 2518ba19f6e53c8d identique, refus tardif sans sortie partielle.
Médianes µs : admissions 6,6 ; requête 2×64 429,2 ; save 0,2 ; restore 2,8 ; reprise 3,7 ;
requête 3×64 639,6. Trois échauffements et 21 mesures release, comparaison hors chronométrage.
CYCLE-HOTE-S88 précise les exclusions et la mémoire. Aucune optimisation du noyau justifiée ici.
**Vérification :** suite 92 core + 93 harnais = 185 réussis, cinq ignorés ; cible exemple
explicitement testée : quatre réussis dont trois tests host_impl importés et un nouveau scénario.
Ces trois tests importés ne sont pas trois nouvelles propriétés. Quatre avertissements préexistants
harnais. Formatage/diff vérifiés. P2 c0d5320. Aucun nouvel ADR ni angle ; L202, invariants inchangés.
**Limites :** sauvegarde mémoire seulement, physique commune non revalidée par identité de code,
aucun budget cible ou coût de crash disque. Journal toujours sans purge.
**Suite S89 :** S88-1, source de sillage depuis trajectoire en milieu profond uniforme, relation
forçage/énergie puis candidat testable. S87-1 close ; copies synchronisées après clôture.

## S89 — 2026-09-08 — Réponse d'une pression mobile

**Entrée :** Continue utilisateur ; départ fe64cd7, copies actives identiques, jeton libre.
**Produit :** pressure_mode.rs, instrument f64 hors runtime répliqué. Segment de pression mobile,
réponse analytique finie à résonance, extinction puis propagation libre, énergie moyenne et puissance.
Sources primaires consultées et dérivation dans ADR-069 ; amplitude de pression explicitement fournie.
**Vérification :** cinq tests release puis suite 97 core + 93 harnais = 190 réussis, cinq ignorés,
aucun échec ; quatre avertissements préexistants du harnais. Quadrature temporelle indépendante,
pression stationnaire, résonance ±ω et voisins, travail intégré, énergie après extinction,
translation, segmentation et refus numériques. Écart travail/énergie maximal du fixture : 9,31e-11 J/m².
**Décision :** ne pas normaliser les segments comme des impacts indépendants : leurs champs
s'additionnent, leurs énergies interfèrent. L203 enregistrée. P2 78b7bf8, formatage et diff vérifiés.
**Limites :** pas de pression localisée ni champ de sillage complet, pas de codec ni de LiveWater,
libm/temps relatif f64 de référence, aucune conformité runtime I-03/I-08 revendiquée.
Aucun nouvel angle numéroté ni invariant amendé ; ADR-069 actée, 69 ADR.
**Suite S90 :** S89-1, pression localisée et superposition spectrale avec convergence et travail
total. S88-1 partielle ; copies synchronisées après clôture.

## S90 — 2026-09-08 — Champ d'une pression localisée

**Entrée :** Continue utilisateur ; départ 1dc6e38, copies actives identiques et jeton libre.
**Produit :** gaussian_pressure.rs, quadrature polaire d'une pression gaussienne mobile, champ
hauteur/vitesse et énergie/puissance globales. Instrument f64 allouant, hors runtime autoritaire.
ADR-070 actée, normalisation Fourier/Parseval explicitée ; aucune nouvelle décision de coque.
**Échec instructif :** 64²→128² diffère de 3,25e-5 J à 4 s, refus au seuil 1e-5 J.
Seuil conservé, 128²→256² reçu à 2,10e-6 J (~0,0024 %). Travail spatial/spectral à 2 s :
écart 1,76e-11 W ; travail temporel/énergie du même maillage : 1,08e-6 J. L204 enregistrée.
**Vérification :** trois tests release ; suite debug 100 core + 93 harnais = 193 réussis,
cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Normalisation,
raffinement, directions et coupure séparés, symétrie, énergie après extinction et refus testés.
P2 d3e3f87 ; formatage/diff vérifiés. Aucun nouvel angle numéroté ni invariant amendé.
**Limites :** énergie de la référence continue estimée par quadrature, pas intégrale infinie
de la somme finie ; emprise non reçue, Kelvin/coque/pression et runtime ouverts.
**Suite S91 :** S90-1, trajectoire multi-segments et travail contre vitesse totale, avec virage
et découpage invariant. S89-1 réalisée sur un segment gaussien ; copies synchronisées à clôture.

## S91 — 2026-09-08 — Trajectoires segmentées et interférences

**Entrée :** Continue puis Reprends utilisateur. Plan 906bb53 déjà committé, seule P2 marquée
active sans code au moment de la reprise ; étape complétée. Copies actives identiques.
**Produit :** trajectory dans GaussianPressure ; validation de continuité, somme complexe des
réponses puis énergie, pression active contre vitesse totale. field délègue au cas un segment.
**Mesures :** virage à 2 s, travail 0,1267090729839 J contre énergie 0,1267085948949 J,
écart 4,781e-7 J ; énergies isolées 0,2160453260870 J. Puissance spatiale/spectrale à 3 s :
écart ~2,13e-12 W. Découpage rectiligne invariant aux arrondis près. TRAJECTOIRES-S91 publié.
**Vérification :** trois tests release ; suite debug 103 core + 93 harnais = 196 réussis,
cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais. Refus de trous,
chevauchements, saut spatial et NaN futur. Formatage/diff vérifiés ; P2 741672b.
**Limites :** égalité exacte aux jonctions f64, virage prescrit instantané, référence allouante
hors runtime. Bilan fermé ne reçoit pas la résolution spectrale du virage. Aucun nouvel ADR,
angle numéroté ou invariant amendé ; L203/L204 appliquées sans nouvelle leçon distincte.
**Suite S92 :** S91-1, recevoir et borner l'emprise spatio-temporelle avant runtime. S90-1
réalisée ; copies synchronisées après clôture.

## S92 — 2026-09-08 — Emprise et raffinement du virage

**Entrée :** Continue utilisateur ; départ b8cede4, copies actives identiques, jeton libre.
**Produit :** QueryDomain, BoundedGaussian et BoundedField ; refus hors bornes déclarées,
validation de la trajectoire entière. Exemple receive_gaussian pour trois axes de raffinement.
**Mesures :** fixture du virage, [-8,12]² m, 0–8 s, 1089 échantillons par comparaison.
Maxima radiaux : 3,828e-7 m, 1,016e-7 m/s, 2,091e-6 J et 8,872e-7 W ; directions et coupure
également sous seuils. EMPRISE-S92 distingue contrat, échantillonnage et absence de borne continue.
**Vérification :** campagne release ; test de frontières puis suite debug 104 core + 93 harnais
= 197 réussis, cinq ignorés, aucun échec. Quatre avertissements préexistants du harnais.
P2 6c64acc. Formatage/diff vérifiés. Aucun nouvel ADR, angle ou invariant ; L205 enregistrée.
**Limites :** domaine déclaré non certifié par constructeur ; modèle f64/libm et allocations.
La validation initiale construit un champ temporaire. Pas de réception continue ni multiprofils.
**Suite S93 :** S92-1, préparation sur buffers hôte avec résultats identiques et refus atomiques.
S91-1 réalisée dans cette portée ; copies synchronisées après clôture.

## S93 — 2026-09-08 — Champ préparé sur mémoire hôte

**Entrée :** Continue utilisateur ; départ 01716ce, copies actives identiques, jeton libre.
**Produit :** PreparedMode opaque, prepare_into et BorrowedGaussian. Validation de trajectoire
extraite sans champ temporaire ; calcul et échantillonnage partagés avec la référence possédée.
Capacité/date refusées avant écriture, erreur numérique sans vue, queue du pool préservée.
**Vérification :** deux tests release d'identité/refus, sept tests précédents release repassés,
campagne S92 aux maxima inchangés. Suite debug 106 core + 93 harnais = 199 réussis, cinq ignorés,
aucun échec ; quatre avertissements préexistants du harnais. Formatage/diff vérifiés. P2 6aa7979.
**Limites :** chemin emprunté sans allocation par inspection, pas de compteur d'allocations.
Nœuds du modèle toujours alloués à l'initialisation, f64/libm hors runtime ; aucune mesure de coût.
MEMOIRE-GAUSSIENNE-S93 publié, aucun nouvel ADR/angle/invariant ni leçon distincte ; L194/L197 appliquées.
**Suite S94 :** S93-1, pente et vitesse horizontale avec vérification par potentiel, avant
composition. S92-1 réalisée dans la référence ; copies synchronisées après clôture.

## S94 — 2026-09-08 — Grandeurs de surface du sillage

**Entrée :** Continue utilisateur ; départ c68e2e8, copies actives propres et identiques, jeton libre.
**Produit :** potentiel, pente et vitesse horizontale dans Surface ; potentiel modal préparé,
contrôles non finis étendus, calcul partagé possédé/emprunté. SURFACE-S94 publié.
**Mesures :** dérivées spatiales à 4,49e-12 / 5,52e-12, cinématique à 2,50e-11,
dynamique à 1,50e-10 ; témoins non nuls. Raffinement radial : potentiel 4,617e-6 m²/s,
pente 1,703e-9, vitesse horizontale 3,586e-8 m/s. Trois axes sous seuils, maxima S92 inchangés.
**Vérification :** trois tests release S94, identité empruntée étendue, symétries et découpage,
refus des nouvelles composantes non finies ; campagne release et suite debug complète :
109 core + 93 harnais = 202 réussis, cinq ignorés, quatre avertissements préexistants.
Formatage et diff vérifiés. P2 b3dfae4 ; aucun nouveau modèle, ADR, angle ou invariant.
L204/L205 appliquées sans nouvelle leçon distincte.
**Limites :** référence f64/libm, vitesses linéarisées à z=0, pas de champ immergé ni composition.
Réception échantillonnée sur fixture, seuils hors fixture à calibrer ; coût non mesuré.
**Suite S95 :** S94-1, noyau modal f32 à phases déterministes, résonances et comparaison S89,
avant portage du champ gaussien. S93-1 réalisée ; copies actives synchronisées à la clôture.

## S95 — 2026-09-08 — Candidat modal déterministe

**Entrée :** Continue puis Reprends ; départ 2064105, copies actives propres et identiques.
Étape P2 reprise après interruption, suite déjà terminée récupérée dans s95-tests.log.
**Produit :** ModalPressure f32, phases Q32 signées, multiplication entière des durées,
résonance régulière, horizon inclusif borné à 16 s. ADR-071 acté ; référence S89 conservée.
**Mesures :** 550 réponses contre référence : hauteur 1,436e-7 m, vitesse 1,355e-6 m/s.
Hash debug/release 8ea15f4a3334830b ; translation entière proche de u64::MAX à bits identiques.
Travail/énergie : 9,173e-9 J/m². Premiers échecs corrigés sans assouplissement : petits angles
négatifs par parité ; conversion fréquence Q32 depuis mantisse/exposant et constante entière.
**Vérification :** quatre tests release, suite debug 113 core + 93 harnais = 206 réussis,
cinq ignorés ; quatre avertissements préexistants. P2 ebc2f03, formatage et diff vérifiés.
**Limites :** candidat reçu localement, pas de conformité interplateforme démontrée, aucun
raccordement autoritaire. Paramètres représentables plus larges que la fixture reçue.
Aucun nouvel angle numéroté ; leçon L206. 71 ADR, 193 angles, invariants inchangés.
**Suite S96 :** S95-1, découpage et superposition gaussienne sur pool avec le candidat,
comparaison des grandeurs complètes à la référence. S94-1 réalisée dans cette portée.

## S96 — 2026-09-08 — Superposition sur spectre fourni

**Entrée :** Continue ; départ 288dfdb, copies actives propres et identiques.
**Produit :** spectral_pressure, préparation du champ total sur pool hôte, vue empruntée
et bornes ; potentiel, pente, vitesses et énergie. SUPERPOSITION-S96 publié, P2 acf0eed.
**Mesures :** 1089 points du virage contre référence complète : maxima hauteur 8,079e-9 m,
vitesse verticale 3,941e-8 m/s, potentiel 6,100e-8 m²/s, pente 9,963e-9, vitesse horizontale
2,976e-8 m/s. Découpage reçu aux mêmes seuils. Énergie reçue après compensation f32,
somme naïve initialement refusée ; seuil 2e-6 J conservé.
**Vérification :** deux tests release ; suite complète consignée à la clôture ci-dessous.
**Limites :** spectre fourni par hôte ; cuisson des tests f64/libm, donc non déterministe
par contrat. Pas de puissance candidate, coût mesuré, codec ni intégration LiveWater.
Aucun nouvel ADR, angle numéroté ou invariant ; L204/L205 appliquées sans nouvelle leçon.
**Suite S97 :** S96-1, fabrication reproductible du spectre gaussien sur mémoire hôte,
contrat et provenance. S95-1 réalisée pour le chemin sur spectre fourni.

**Clôture :** 115 core + 93 harnais = 208 tests réussis, cinq ignorés, quatre avertissements préexistants. Copies synchronisées après commit final.

## S97 — 2026-09-08 — Cuisson gaussienne sans libm

**Entrée :** Continue ; départ d88f10d, copies actives propres et identiques.
**Produit :** gaussian_spectrum::bake, recette V1 et vue empruntée, hash de recette/contenu.
Directions Q32, exponentielle réduite polynomiale, limites explicites ; ADR-072 acté.
**Mesures :** exp relative 1,16047e-6 ; profil 9,17911e-5 ; 27 recettes comparées aux nœuds
f64, erreurs direction/transformée/poids 1,259e-7 / 2,110e-6 / 1,306e-7. Champ complet
1089 points sous 1e-7 pour chaque grandeur et 2e-6 J ; découpage reçu sans déplacer les seuils.
Hash nominal 20e64a392ae237a1, assertion debug/release locale.
**Vérification :** trois tests release ; suite complète 117 core + 93 harnais = 210 réussis,
cinq ignorés, puis test de limites ajouté et exécuté en debug séparément : total 211 réussis.
Quatre avertissements préexistants ; formatage/diff vérifiés. P2 4c45621.
**Limites :** conformité interplateforme non démontrée, aucun coût mesuré ni codec de recette.
Hash diagnostique, pas authentification. 72 ADR, 193 angles morts ; invariants inchangés.
Aucune nouvelle leçon distincte ; L205 appliquée aux limites de recette.
**Suite S98 :** S97-1, coût réel et mémoire de cuisson/préparation/requête avant optimisation.
S96-1 réalisée dans la bibliothèque ; copies actives synchronisées après clôture.

## S98 — 2026-09-08 — Coût réel du chemin gaussien

**Entrée :** Continue ; départ 15e2b4b, copies actives propres et identiques.
**Produit :** bench_gaussian et COUT-GAUSSIEN-S98 ; cuisson, préparation et requêtes séparées,
mémoire explicite. Aucun code bibliothèque modifié ni optimisation introduite.
**Mesures :** Ryzen AI 7 350, rustc 1.97.0, release ; 3 échauffements, 21 répétitions.
Médianes cuisson 532 µs, préparation de deux segments 12501 µs, requêtes 1/64/121 points
621,5 / 44637,8 / 87066,5 µs. Requête dominante, candidat trop coûteux pour cet usage par image.
Spectre 262144 octets, champ 589824 ; total buffers avec deux champs 1446228 octets.
**Vérification :** exemple exécuté avec assertions ; 121 sorties comparées à référence
f64, max 2,285177e-8 sous 1e-7 par composante. Hash121 47820ae52df0b569 ; recette inchangée.
Formatage/diff vérifiés, P2 0a66565. Suite 211 réussis/cinq ignorés inchangée, non relancée
pour cet ajout de banc ; aucun nouveau test unitaire. Pas de compteur global d'allocation.
**Limites :** mesure locale bruitée, aucun pire cas ni budget cible certifié. Coûts du journal,
réseau et composition exclus ; oracle hors chronométrage. 72 ADR, 193 angles morts inchangés.
Aucune nouvelle leçon distincte ; séparation mesure/coût/propriété appliquée.
**Suite S99 :** S98-1, candidat utilisant conjugaison k/-k ; vérifier les bits de géométrie,
recevoir erreur puis mesurer avant de changer de représentation. S97-1 réalisée.
