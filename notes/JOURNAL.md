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
