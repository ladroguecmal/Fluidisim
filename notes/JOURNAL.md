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
