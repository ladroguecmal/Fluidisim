# Journal des sessions — S200 à S299 (archive)

*Archivé en S480, 2026-10-04, depuis [`notes/JOURNAL.md`](../JOURNAL.md) ; texte inchangé (les liens relatifs recalés d'un niveau), 100 entrées.* Le journal vivant
garde les entrées depuis S470 ; une archive ne se modifie plus.

---

## S200 — 2026-09-13 — Le pas δ cesse d'allouer et restaure ses refus

**Entrée.** Continue, S199-1/A244 ; quatre copies961a2e5 propres. Reprise/invariants
lus dans cette conversation ; aucun travail concurrent. Plan seul a23bf00.

**Code construit.** project ne clone plus us/ws, dir par itération ni u/w pour le
diagnostic. Trois tampons préalloués sauvegardent u/w/p ; toute sortie numérique
non finie restaure les champs publics avant Err. Réservation corrigée à la précision
réelle des tableaux (six f64/cellule avec sauvegarde), tailles checked avant allocation.
Les opérations de calcul nominales restent dans leur ordre S199.

**Réception qui manquait.** Nouveau test d'intégration delta_runtime avec allocateur
global System instrumenté par thread, JobSystem synchrone. Une vraie allocation témoin
est détectée ; pas nominaux/dégradés500/1/0/500 sans allocation, premier pas inclus.
Overflow pendant le calcul : u/w/p inchangés, zéro allocation, puis récupération
exacte face à un noyau neuf. Réservation exacte et débordement de dimensions contrôlés.
Trois tests debug/release. Test limité au thread exécutant ces réductions ; pas de
preuve sur un JobSystem tiers qui allouerait ou sur un panic de l'hôte.

**Validation.** Workspace debug342 réussis/cinq ignorés (246+3+93), aucun échec.
Huit tests noyau et trois intégration rejoués après correction de Caps. Avertissements
anciens dans exemples/harnais conservés. Filtre delta_filters release : empreinte
**0x0ad3f695685ca27a**, inchangée ; ordres plat1,947/lisse0,898/marche0,895. Aucun
gain de justesse spatiale ni mesure CPU. Relevés CONTRATS-DELTA-S200-MESURES.

**Contrats publiés.** supports_frame_accel devient false ; min_dx/max_dx/CFL deviennent
Option avec None (inconnus), au lieu de bornes sans réception. Aucune consommation de
ces champs hors test dans le dépôt, recherche effectuée. API de candidat modifiée,
pas interface de production figée. Pression f64 conservée expérimentalement et plafond
d'itérations nommé comme tel : I-08/I-05 restent non reçus. Pas d'ADR dérogeant aux
invariants par simple commodité, aucun solveur retenu. L'advection/diagnostic coûtent
même à zéro itération : ne pas assimiler ce plafond au budget en millisecondes.

**A244 partielle.** S199-1 a corrigé mémoire/refus et restreint les capacités ;
**S200-1**, prochaine session, reçoit précision de pression et budget temporel.
**S199-2** reste nommée : flux sur faces coupées et réception avant surface mobile.
B3 non admissible, B4/A50 partiels, seuil2 % inchangé. A217 reste close S194.

**Rituel.** Recommandation S199 exécutée partiellement et reliquat explicite, compteur0
car δ avance en bibliothèque. File entière relue, aucun autre chantier clos : A241,
A213, B2/coupure, bathymétrie, seconde cible, V, réunions restent portés. Aucun angle
ni leçon distincte : l'instrument doit mesurer le phénomène revendiqué, principe déjà
appliqué ; le reçu global complète celui de l'hôte.123 ADR/244 angles/279 leçons/
18 invariants/6 SPEC/23 cas vérifiés. B3/B4, registre, README/index/REPRISE mis à jour.
Copies propres à avancer après commit final, aucune suppression.

## S201 — 2026-09-13 — Première image du champ B, direction utilisateur

**Entrée.** Après S200, l'utilisateur choisit image→budget→δ bornée/V au besoin.
Autorisation d'image demandée conformément à son message, puis accordée (« oui »).
Quatre copies9a25798 propres, aucun travail concurrent. Plan seul74bc6a7.
S200-1 est reportée explicitement ; la suite ne vient pas du chaînage.

**Construction.** render_background.rs, exemple CPU Rust zéro dépendance : vraie
recette JONSWAP cuite N32, Background::eval pour chaque interrogation, hauteur et
normale existantes, caméra perspective/rayons. Pas de nouvelle houle ni maillage de
substitution. PPM P6 local640×360 à deux échantillons/pixel, sortie dans captures/
ignoré. Preview PNG transcodée avec Pillow pour affichage, pixels sans retouche.

**Réception.** Cinq tests debug passent (deux propres, trois hôte). Deux instants
12/13 s et témoin plat inspectés, différents visuellement et en pixels. Premier jet
256 étapes laissait997/1021 rayons rasants magenta ;4096 étapes donne zéro refus,
tolérance verticale3 mm conservée. Répertoire de sortie créé après un premier échec
explicite d'export. Reçu final : t12 hash a52ff81902b150c3,10021895 évaluations,
11727,894 ms ; t13 hash1df02ffb7c202b32,10145445 évaluations,11925,566 ms ; plan
hash dae2f2514cad0324,77,590 ms, sans évaluer B. Coûts locaux uniques, pas médianes
ni garanties temps réel. Limite600 m, éclairage/brume de diagnostic, aucune calibration
optique ni perception reçue. Images/paramètres dans IMAGE-B-S201.

**Décision durable : ADR-124 actée**, direction explicite utilisateur. Image locale
permise, budget d'image/coût par bloc ensuite, δ comme effets bornés, V au besoin
gameplay. Aucune autorité ni persistance transférée à δ ; ADR-027 non rouvert.
Les exigences B4 ne sont pas effacées, mais un δ général ne bloque plus les images
et réceptions propres de B/W. Seuil2 % déjà acquis depuis S190 ; les vieux passages
« critère manquant » sont historiques. Aucun effet mousse/spray/audio reçu par l'image.

**Suite S202 : S201-1**, budget image et coût par bloc sur cible/charge nommées,
puis détailler les effets δ bornés. S200-1 et S199-2 conservées et reportées ; ne pas
réparer automatiquement le noyau général avant ce choix. A50/B4 partiels, A98 intacte.

**Rituel.** File entière relue, V requalifiée par besoin gameplay, autres chantiers
conservés ; AGENTS porte l'autorisation d'image (un seul texte d'amorce). Aucun
angle/leçon distincts ;124 ADR/244 angles/279 leçons/18 invariants/6 SPEC/23 cas.
Bibliothèques inchangées,342 tests/cinq ignorés reste reçu S200 non rejoué.
Compteur0 : ADR-124 décide le périmètre δ/V, ce qui avance selon REPRISE §4.
Journal/index/README/REPRISE et B3/B4 actualisés, copies à avancer après clôture.

## S202 — 2026-09-13 — Le budget est choisi, le bloc publie son coût

**Entrée.** Continue, S201-1 sous ADR-124 ; quatre copiesf0fea77 propres. L'utilisateur
répond à la préférence de profil : **60 images/s, eau2 ms/image**. Plan seul194aa1d.
Le budget couvre toute l'eau, pas chaque couche ni chaque bloc ; ADR-125 actée.

**Code.** MonotonicClock injectée dans host, Volume::step_measured enveloppe le pas
complet ; Caps::cost_per_block_ms publie Option. Un bloc de banc=un domaine Volume,
dimensions/charge/plafond explicités ; aucun add/remove_blocks fictif. Pas d'horloge
système dans le cœur. Coût inconnu avant mesure, invalidé sur modification d'entrées,
pas non mesuré, refus ou horloge égale/reculant. Une mesure dégradée porte son report.
Coût observé, aucune garantie de délai ni conversion arbitraire itérations→ms.

**Vérification.** Horloge factice reçoit1,5ms et deux lectures ; u/w/p/report identiques
au pas non instrumenté, zéro allocation globale. Invalidations et cas dégradé reçus.
Quatre tests intégration debug/release, workspace343 réussis/cinq ignorés (246+4+93).
Premier import Allocator oublié dans exemple : erreur de compilation corrigée avant
mesure ; avertissements anciens exemples/harnais inchangés. Aucun seuil physique modifié.

**Coût.** AMD Ryzen AI7 350, CPU séquentiel, domaine8×4m fond0,4m, surface sinusoïdale
amplitude0,02m, pas1/60 ; trois chauffes/onze mesures avec vitesses initiales rétablies.
Neuf configurations16/32/64 colonnes × plafonds1/64/512.32×16 convergé médiane0,5865ms,
max0,6567ms (plafond512) ;64×32 médiane4,7886ms, max5,3201ms : dépasse2ms à lui seul.
Plafond64 sur64×32 :2,4051ms médiane ET dégradation ; aucune solution rapide recevable
par cela. Plafond1 résidu≈0,33 : faible coût n'est pas qualité. Même report entre
répétitions. Maximum observé, pas borne ni p99. Rendu S201≈11728ms reste hors ligne.
Relevés complets BUDGET-IMAGE-S202 et MESURES ; coût B/W/rendu absent du tableau δ.

**Décision et limites.** ADR-125 fixe le profil utilisateur, pas une cible matérielle
de livraison ni un moteur GPU. Coût seul sous2ms ne reçoit pas B3 ; A244 partielle,
précision et respect temporel restent ouverts. Le seuil physique2 % est distinct.
**Suite S203 : S202-1**, premier impact visible W, emprise et observateur explicites,
identifier la part nécessitant un effet δ borné ; V attend besoin gameplay. Pas de
retour automatique à la réparation du noyau général, S200-1/S199-2 restent reportées.

**Rituel.** File entière relue, budget clos par décision, autres points conservés.
A244 suivie sans clôture ; aucun angle/leçon distincts.125 ADR/244 angles/279 leçons/
18 invariants/6 SPEC/23 cas vérifiés. I-05 reste exigé, pas déclaré reçu ; horloge
coût séparée du temps de simulation, aucune autorité nouvelle. Compteur0 : code δ
avancé. Journal/index/README/REPRISE, B3/B4 et file actualisés ; copies à synchroniser
après clôture. Images S201 conservées, aucune image nouvelle nécessaire dans ce lot.

## S203 — 2026-09-13 — Un impact W devient visible, et la mer de référence n'était pas composable

**Entrée.** Claude Code (Opus 5), démarrage à froid, jeton libre une minute après S202 (Codex).
Trois copies isolées à e13d212 propres, aucun travail concurrent. S202-1 sous ADR-124/125 :
impact porté par W, emprise et observateur explicites. Plan seul 05b1f92. Compteur 0.

**Constat d'amorce, avant tout code.** Crate jetable hors dépôt : le plancher de pente L1
de B sur la recette S201 vaut 0,6082 à Hs 1,5 contre π/7 = 0,4488. `compose` refuse chaque
point de la mer S201 avant tout impact. Majorant directionnel démontré 0,5733 (refuse encore),
pente échantillonnée 0,4215. ADR-094/095 tenaient ce terme pour exact : notes correctives
datées, A245 gravité 1, I-18 non tenu pour B, garde aveugle aux termes. Scène passée à
Hs 0,5 m, dit comme choix de banc.

**Construction.** `render_impact.rs`, exemple seul, bibliothèque inchangée. Impact par
`impact_generator` (b 1 m, v 8 m/s, fraction 0,005 à calibrer B2 → λ 3,35 m, E 164 J), budget
de pente alloué π/7 − plancher. Coutures d'emprise mesurées (critère déclaré avant : 3 mm et
2 % du pic 0,15482 m) : temporelle 78 mm à 2 s → 2,80 mm à 56 s, indépendante de N ; spatiale
≈ 1/R, 3,06 mm à 50 m. Premier passant N256 R52 A56 ; accord N512 à 0,1 µm ; homothétie λ×2 à
0,019 point. Caméra/marche S201 extraites dans `support/ray_view.rs`, empreinte S201 reproduite
au bit. Rendu B+W par `Prepared::sample_world_batch` contre un témoin à marche identique.

**Réception.** +1/+3/+6 s : zéro non résolu, zéro refus, 6 780/8 640/15 737 px différents,
**zéro hors emprise** ; anneaux visibles, reflet déformé à +6 s. Observateur : λ 11 px le long
de la visée au point d'impact, sous 2 px dès 67 m ; l'observateur est dans l'emprise en plan
(boîte englobante fausse retirée avant publication). Coût : B 1,6 µs/pt, B+W 14 µs/pt
(≈ 140 pts dans 2 ms) ; table radiale Hermite ≤ 0,006 mm mais 2,7 ms de construction pour un
impact (A247). Part non portée par W nommée : 32,6 kJ hors ondes, Fr 1,81, balistique
3,26 m / 1,63 s, δ SPEC-001 §2.4 à 1,15 M cellules, aucun coût transposé. Onze tests exemple.

**Décision durable : ADR-126 actée.** Une emprise d'image se reçoit par ses deux coutures ;
R ≥ 15,5 λ, A ≥ 96·√(λ/g), N ≥ 256 (N512 recommandé) au critère 2 % ; budget de pente d'impact
= ce que B laisse ; réception au pixel contre témoin. Valeurs suffisantes, pas minima.

**Non fait.** Aucune migration du terme de B ; aucun δ ; renouvellement d'horizon non examiné ;
un impact, une énergie, une recette ; aucun seuil perceptuel. Workspace **rejoué** en fin de
session : 343 réussis / cinq ignorés (246+4+93), identique à S202 ; avertissement ancien du
support `nl_surface` inchangé.

**Suite S204 : A245**, rendre la composition B+W possible sur une mer de Hs > 1,1 m — lot
bibliothèque qui fait avancer la composition B+W (code dans `src`), puis A247 et le choix de
l'effet δ d'impact (cavité eulérienne ou gerbe particulaire). V attend un besoin gameplay.

**Rituel.** File active entière relue : S202-1 close, A245/A247 ajoutées, autres lignes
conservées. A246 traitée par ADR-126. L280, L281. 126 ADR, 247 angles, 281 leçons,
18 invariants, 6 SPEC, 23 cas. Compteur 0 : ADR-126 fixe un élément de W (§4). I-18 relu :
l'invariant est juste, le terme de B l'enfreint — rien à amender, un code à corriger.

## S204 — 2026-09-13 — Ambition finale complète, construction progressive : ADR-127 corrige ADR-124

**Entrée.** Clarification de l'utilisateur, reçue après S203 : les ambitions initiales restent
intégralement maintenues — interactions volumiques générales, inondations complexes, V, grande
échelle ; « image → budget → effets bornés » est un ordre de construction, pas une réduction ;
Codex a interprété trop largement ADR-124 et l'a reconnu en conversation sans l'appliquer.
Formulation de référence : *ambition finale complète, construction progressive par versions de
plus en plus capables*. État réel vérifié avant tout : master 5a4ba1e propre, trois copies
isolées à 5a4ba1e propres, jeton libre, aucun travail concurrent. Plan seul 9edc116.

**Constat.** Lecture restrictive dans 18 fichiers (58 occurrences), dont ADR-124, ADR-125
§35-36, la file active, REPRISE, l'index, PLAN-BENCHMARK, BILAN-B4-S176, IMAGE-B-S201,
BUDGET-IMAGE-S202 et **IMPACT-W-S203, écrit par cet agent en S203**. Aucun fichier
« roadmap » : la trajectoire était dispersée entre ADR-053 §3, ADR-054 §3, ADR-124 et
l'en-tête de REPRISE.

**Décision : ADR-127 actée.** D1 ambition complète obligatoire ; D2 jalons J1 (B/W visible et
interactif) → J2 (domaines bornés, cas du δ général) → J3 (phénomènes, interactions, frontières
mobiles) → J4 (V et inondations complexes, articulation volumétrique) → J5 (ambition complète) ;
D3 un domaine borné passe par les interfaces du système, S199-2/S200-1 sur le chemin ; D4
V-noyau ouvert au plus tard avec J2, il ne dépend pas de δ (ADR-054 §1, C12) ; D5 rôles,
autorité (I-04, I-10, I-11, I-15, I-17), conservation inchangés ; D6 bancs exécutés quand ils
tranchent ; D7 budget = cible confrontée aux scènes représentatives, incompatibilité ⇒
arbitrage explicite, à l'utilisateur si l'option retire de l'ambition. §6 : réduire l'ambition
n'appartient à aucune session.

**Propagation.** Note de renvoi datée en tête d'ADR-124, note corrective ADR-125. Nouvelle
`docs/FEUILLE-DE-ROUTE.md`, seul porteur de la trajectoire (état daté par jalon, bancs rattachés
aux décisions, arbitrages). REPRISE (en-tête, file active, §3, §4, marqueurs sur S201/S202),
file active (direction, A245 bloquant J1, A247 à arbitrer, hôte interactif, S200-1 et S199-2 sur
le chemin de J2, V-noyau obligatoire, bancs), AGENTS (« Ce que tu ne décides pas »),
PLAN-BENCHMARK B3, BILAN-B4-S176 état actif, index, README, notes datées S201–S203.
Rien supprimé, aucun ADR réécrit, aucun code.

**Arbitrage à obtenir de l'utilisateur : l'hôte interactif de J1.** Le workspace est sans
dépendance (ADR-020) ; une fenêtre temps réel demande soit des dépendances (téléchargement,
infrastructure), soit des appels système sans dépendance. Non tranché, remonté.

**Suite S205 : A245**, composition B+W sur mer Hs > 1,1 m — bloquant J1, lot bibliothèque.

**Rituel.** A248 (un ordre inscrit comme périmètre, propagé sous l'autorité de l'utilisateur),
L282. 127 ADR, 248 angles, 282 leçons, 18 invariants, 6 SPEC, 23 cas. Invariants cités relus
(I-03, I-04, I-05, I-10, I-11, I-15, I-17) : aucun ne devient faux ; I-05 (sous-résoudre pour
tenir le budget) est cohérent avec D7. File active entière relue. Compteur 0 : sujet imposé par
l'utilisateur, pas un reliquat ; ADR-127 fixe le périmètre de δ et V. Mémoire privée du compte
mise à jour (pointeur, le dépôt fait foi).

## S205 — 2026-09-13 — La mer de référence se compose : B sort du budget de refus

**Entrée.** « Enchaîne sur S205 ». État réel : master et trois copies à 990e6ae, propres ; jeton
libre (S204, même agent). Arbitrage de l'hôte interactif posé en S204, non répondu ; A245 n'en
dépend pas. Plan seul 60e1b58, avec la décision de conception et le critère de non-régression
écrits **avant** le code : tout lot déjà admis publie les mêmes bits.

**Décision.** Quatre sites mettaient `steepness_B·π` (borne L1 de B, 0,6082 à Hs 1,5) dans le
budget de refus. Remèdes pesés : majorant directionnel (refuse encore), refus sur pente réelle
au point (dispersé, causé par la mer, lot atomique perdu), borne statistique (ne garantit rien).
Retenu, **ADR-128** : le budget ne somme que les perturbations ; la raideur de B reste publiée ;
`Slope`/`SlopeEnvelope` jugés sur la pente des perturbations ; `slope_floor` exact dans les deux
sens ; budget d'impact = π/7 moins les autres perturbations (remplace ADR-126 règle 3). Aucune
SPEC ne consommait la garantie « B+W sous π/7 » ; `steepness` sert à l'écume et au déferlement.

**Construction.** `composition`, `mixed_water`, `mixed_differential`, `bound_pressure` :
accumulateur `budget` et pente `perturbation` séparés, publication inchangée, finitude de
`steepness` contrôlée en sortie de `compose`. `Background::differential_slope_envelope` retirée.

**Réception.** Premier passage 7 échecs sur 246, **tous** liés à B dans le budget ou le verdict :
garde I-18 (noms des sites), verdicts S144 reconstruits avec un impact, deux verdicts de pression
passés de `Slope` à `SlopeEnvelope` (la pente de B faisait la différence), plancher désormais
admettant tout au-dessus. Essai neuf `reference_sea_s201_composes_with_an_impact_s205`. Workspace
**344 réussis / cinq ignorés** ; release vert ; C18 `0x85c8bc610f551d11`, C02 `0x0a3a3bcc945db263`
inchangés ; **image S203 +3 s reproduite au bit**. Impact sur la mer S201 Hs 1,5 à +3 s et +6 s :
**zéro refus, zéro non résolu, zéro pixel hors emprise** (7 342 et 16 251 px différents),
anneaux visibles ; la marche coûte 1,67 fois plus, sa borne de pente passant de 0,58 à 0,99.

**Non fait.** Aucune image au-delà de Hs 1,5, ni de sillage ou de pression sur mer raide (essais
verts seulement) ; validité de la superposition sur mer raide non reçue (ADR-123, A207) ; A247 et
hôte interactif, les deux autres bloquants de J1, intacts.

**Suite S206 : A247**, coût d'un impact visible confronté au budget eau 2 ms sur une scène
représentative de J1 — mesurer les leviers (pas de table radiale, N hors coutures, parallélisme)
puis **arbitrage explicite** (ADR-127 D7). L'hôte interactif attend toujours l'utilisateur.

**Rituel.** A245 close (ADR-128) ; **A249** (bancs de refus à mers jouets) ; L280 reçoit un
corollaire plutôt qu'une leçon quasi dupliquée. 128 ADR, 249 angles, 282 leçons, 18 invariants,
6 SPEC, 23 cas. I-18 relu : désormais tenu pour B. velocite.sh : B et W avancent en S205, δ S202,
V jamais. Compteur 0 (code `src`). Feuille de route : J1 perd A245. File active entière relue.

## S206 — 2026-09-13 — Ce que l'eau coûte par image : W se divise par cent, B sur CPU ne tient pas

**Entrée.** « Enchaîne sur S206 ». État réel : master et trois copies à ef447fc, propres ; jeton
libre depuis 02:06. Arbitrage de l'hôte interactif toujours sans réponse. Plan seul c13d968, avec
la **scène représentative déclarée avant mesure** : observateur S201, mer S201 Hs 1,5, impact
S203 à +3 s, chemin hôte ; charge = grille de sommets projetée à 8, 4 et 2 px.

**Mesure.** Banc `frame_cost.rs`, qui inclut `render_impact.rs` comme module plutôt que de
recopier la scène. Un fil : 2 240 / 9 044 / 36 160 sommets, dont 86 % dans l'emprise ; image
17–18 / 72–74 / 280–293 ms ; **B seul dépasse 2 ms dès 8 px**. Densité exigée par l'observateur
au point d'impact : c ≤ 5,8 px pour B, ≤ 2,75 px pour les anneaux — la grille à 8 px perd
l'impact. Parallélisme (fils lancés dans l'image) : 3,6 / 10,0 / 36,2 ms à 16 fils, accélération
plafonnant vers ×8, bits identiques dans les quinze cas ; lancement seul 1,2–1,6 ms. Table de
Bessel précalculée (noyau N×M sur tableaux de la bonne taille) : **0,027 ms par impact et par
image**, 502 Ko, erreur 0,006 mm (λ/16) ; 0,016 ms, 254 Ko, 0,09 mm (λ/8). `paquets_W_max = 4096`
= 109 ms et 2 Go.

**Décision : ADR-129 actée.** Chemin d'image de W radial par table de Bessel précalculée, pas
≤ λ/8, cosmétique et jamais autoritaire (I-15), construit en S207 avec réception écrite ;
`paquets_W_max` retiré du profil, capacité calculée (I-16). Note datée ADR-012, qui prévoyait
`gpu_sim_ms = 2,5` jamais exercé.

**Verdict et arbitrage.** Incompatible avec 60 images/s / eau 2 ms sur CPU à toute densité qui
montre l'impact. L'écart est **B évalué par sommet sur CPU** (1,2–1,3 µs), plus W. Arbitrage
« chemin de rendu et hôte de J1 », fusionné avec A247, posé à l'utilisateur : (A) GPU par un hôte
séparé avec dépendances — recommandé, prévu par ADR-003/I-08/ADR-012 ; (B) CPU seul, B vectorisé
et 2 ms en temps mur multi-cœurs ; (C) profil changé ; (D) densité 8 px, qui retire l'impact
visible. Aucune option retenue sans lui ; aucune fonctionnalité retirée.

**Non fait.** Aucun GPU ni groupe de fils persistant ; B vectorisé, densité adaptative et nombre
de composantes par distance non mesurés ; noyau L2 sur valeurs synthétiques, pas encore dans la
bibliothèque ; un seul observateur, un impact, sans sillage ni pression.

**Suite S207 : construire ADR-129** dans `water-core` (W avance), réception écrite dans l'ADR ;
indépendante de la réponse à l'arbitrage. Si l'utilisateur tranche (A), l'hôte GPU devient le lot
suivant de J1.

**Rituel.** A247 mesurée et transformée (ADR-129 + arbitrage) ; **A250** ; **L283**. 129 ADR,
250 angles, 283 leçons, 18 invariants, 6 SPEC, 23 cas. Invariants relus : I-03, I-06, I-08, I-15,
I-16 — aucun ne devient faux ; I-16 est précisément ce qui retire `paquets_W_max`. Aucun code
`src` : compteur 0 par ADR-129 (décision qui fixe un élément de W), l'outil dirait 1. File active
entière relue. Feuille de route J1 à jour.

## S207 — 2026-09-13 — L'utilisateur tranche : l'eau de J1 se rend sur GPU, par un hôte séparé

**Entrée.** Réponse de l'utilisateur, par question structurée après le commit de S206, à
l'arbitrage « chemin de rendu et hôte de J1 » : **(A) GPU, hôte séparé**, l'option recommandée.
État : master et copies à 94dd705, propres. Session courte d'archivage, plan seul 9f9abfa.

**Décision : ADR-130 actée**, arbitrage de l'utilisateur. B et W s'évaluent sur GPU dans un hôte
d'affichage séparé ; (B) CPU multi-cœurs, (C) changement de profil et (D) densité réduite non
retenus ; ADR-125 inchangé. Conséquences techniques déléguées : `water-core` reste sans
dépendance et publie ce que le GPU consomme (recettes, phases repliées I-08, tables ADR-129) ;
l'hôte vit hors du workspace sans réseau ; chemin cosmétique (I-04, I-15). La réponse s'inscrit
dans ce que le corpus prévoyait : ADR-020 (`IGpuBackend` fourni par l'hôte), ADR-003, ADR-012
(`gpu_sim_ms = 2,5`).

**Ouvert.** Pile exacte ; budget GPU de l'eau, qu'aucune valeur ne fixe (première mesure de
l'hôte) ; **aucune dépendance téléchargée sans autorisation nommée**, demandée au début du lot.

**Suite S208 : construire ADR-129** (W, code `src`, indépendant de la pile) ; puis le lot de
l'hôte GPU, qui commence par proposer la pile et demander l'autorisation de téléchargement.

**Rituel.** Note datée ADR-125 ; feuille de route (§4 : arbitrage tranché, deux lignes nouvelles —
dépendances, budget GPU), file active, index, README, REPRISE. A250 reçoit sa décision, reste
ouverte jusqu'à l'hôte construit. Aucun angle, aucune leçon nouveaux. 130 ADR, 250 angles,
283 leçons, 18 invariants, 6 SPEC, 23 cas. Compteur 0 (décision de l'utilisateur, sujet imposé).

## S208 — 2026-09-13 — La table de Bessel construite ; wgpu et winit recommandés pour l'hôte GPU

**Entrée.** « Enchaîne sur S208, et pour la décision de l'hôte je pensais à l'option 1 mais à voir
via ta recommandation. » L'option 1 de S204 est celle qu'ADR-130 acte. État : master et copies à
79bfda6, propres. Plan seul c30515c avec conception et **cinq critères déclarés avant mesure**.

**Construction.** `radial_table.rs`, sous-module de `radial_impact` : `table_len`, `bake_table`
(stockage fourni par l'hôte), `profile` (mêmes opérations et même ordre que `sample`), `eval`
(Hermite et sa dérivée, refus exactement là où `admits` est faux), erreur `TableError`. Aucune
formule recopiée, aucune allocation.

**Réception.** (a) égalité **au bit** aux nœuds — reçue. (c) zéro allocation, témoin > 0 — reçue.
(d) refus nommés — reçus. (e) image S205 +3 s par la table : 27 pixels différents de l'image
directe, écart maximal 1 niveau, 0 hors emprise, 18,5 s contre 102 s — reçue. **(b) précision à
λ/8 ≤ 0,09 mm — ÉCHOUE** : 0,1820 mm à la naissance. Échec consigné et committé **avant** retouche
(b0aaa00) ; mesure ajoutée ensuite et dite comme telle : λ/16 = 0,0127 mm au pire, λ/8 sous
0,09 mm dès 2 s, rapport 14,3 ≈ h⁴. Le seuil n'a pas bougé ; le pas de réception devient λ/16
(note datée ADR-129). Coût par image à 2 px : profil 0,04 ms, évaluation 0,91 ms, image 43 ms dont
B 41,7 ms — contre 280 ms en direct. Workspace **348 réussis / cinq ignorés** ; C18/C02 inchangés.

**Recommandation (à la demande de l'utilisateur).** HOTE-GPU-S208 : **wgpu 30.0.1 + winit 0.30.13
+ pollster 1.0.1**, versions et licences lues sur crates.io sans téléchargement ; espace de travail
`viewer/` séparé, `code/` inchangé et hors réseau ; B évalué sur GPU depuis des phases repliées et
en coordonnées relatives à la caméra, W depuis les profils de table. Options écartées : D3D12 brut,
Vulkan brut, OpenGL, moteur. Arbre transitif **non chiffré** faute de résolution.

**Suite S209 : lot de l'hôte GPU**, qui commence par demander l'autorisation de **résolution**
(Cargo.lock de `viewer/`, index seulement) puis, liste et taille en main, celle des **sources** ;
question du vendoring à l'utilisateur.

**Rituel.** L284 ; suivis A247 (part W construite, B reste) et A250 (pile recommandée) ; aucun angle
nouveau. 130 ADR, 250 angles, 284 leçons, 18 invariants, 6 SPEC, 23 cas. velocite.sh : W avance
en S208 (24 modules). Compteur 0. Invariants relus : I-03, I-06, I-08, I-15 — tenus.

## S209 — 2026-09-13 — Préparation locale de la résolution GPU

**Entrée.** « Reprends le projet ». REPRISE et passation S208 lues ; état réel :
master et trois copies à 28bf6a5, propres, jeton libre ; cargo 1.97 disponible.
Plan seul 35de6b9 ; dossier committé en 1bce2a8.

**Sortie.** PREPARATION-HOTE-S209 : manifeste à versions exactes héritées de S208,
commande de résolution et inventaire nom/version/source/licence/taille/checksum attendu.
Le verrou ne porte ni licence ni taille : les métadonnées publiques seront nécessaires,
avec distinction arbre portable / sources Windows et archives / espace extrait.

**Vérification.** Métadonnées de code/ reçues hors réseau : deux membres, cœur sans
aucune dépendance externe, harnais avec chemin local seulement. Aucun code modifié,
aucun test numérique rejoué ; 348 réussis/cinq ignorés reste le reçu S208.

**Limite explicite.** ADR-130 D5 et HOTE-GPU-S208 §4 demandent un accord nommé :
la reprise générale ne le fournit pas. Aucun registre contacté, aucune archive téléchargée,
aucune application GPU construite. Demande finale : index et métadonnées crates.io pour
wgpu 30.0.1, winit 0.30.13, pollster 1.0.1 et leur arbre ; sources soumises ensuite.

**Suite S210.** Résoudre dès cet accord, établir liste/licences/tailles, puis obtenir
l'accord sur les sources et le choix de leur conservation. La construction J1 reste le
chantier actif ; sillages, J2 et V restent dus selon la feuille de route.

**Rituel.** File active plurielle relue, autres déclencheurs conservés ; aucun nouvel angle
ou leçon généralisable, aucun ADR changé. I-03/I-06/I-08/I-15 inchangés. Index, feuille
et REPRISE actualisés. 130 ADR,250 angles,284 leçons,18 invariants,6 SPEC,23 cas.
Compteur 1 : aucune couche avancée. Jeton libre, copies synchronisées en avance rapide.
## S210 — 2026-09-13 — Résolution autorisée et inventaire des dépendances GPU

**Entrée.** « Oui » à la demande S209 : index et métadonnées crates.io, sources distinctes.
master/copies propres à 3b5e34c, jeton libre. Rust 1.97.0, x86_64-pc-windows-msvc.

**Sortie.** viewer/ séparé, versions exactes wgpu 30.0.1, winit 0.30.13, pollster 1.0.1.
Cargo.lock : 256 paquets, dont 254 externes et deux locaux. DEPENDANCES-HOTE-S210 liste
les licences déclarées, tailles, sources et checksums. 254 réponses de métadonnées,
aucune manquante, aucun paquet retiré, checksums tous concordants. Archives portables :
49 174 790 octets ; pas la taille Windows seule, ni la taille extraite. Aucun source
externe téléchargé, aucune compilation ; main.rs vide sert seulement à la résolution.

**Vérification.** Régénération verrouillée en ligne réussie, verrou inchangé. Tentative
hors réseau verrouillée refusée, sans modification ; cause précise non diagnostiquée.
Métadonnées de code/ reçues hors réseau, fichiers code/ intacts. Tests numériques non
rejoués : 348/cinq ignorés reste le reçu S208. Aucun budget GPU nouveau.

**Manquements consignés.** La commande P1 initiale a échoué sans sortie et son statut
n'a pas été contrôlé avant la résolution ; plan réparé dès constat dans 378b11d.
Le message P2 dit 255 externes : il confond les dépendances annoncées par Cargo avec
les paquets du registre. Recompte réel 254, corrigé dans l'inventaire sans réécrire le commit.

**Suite S211.** File active J1, couches B/W : construire l'hôte GPU dès accord sur les
254 sources nommées. Proposer verrou versionné et cache Cargo local ; vendoring reste
au choix de l'utilisateur. Sillages dus pour J1, J2 et V maintenus selon ADR-127.

**Rituel.** File plurielle entière relue, autres déclencheurs conservés, A247/A250 ouvertes.
Aucun nouvel ADR, angle ou leçon ; I-03/I-06/I-08/I-15 inchangés. 130 ADR,250 angles,
284 leçons,18 invariants,6 SPEC,23 cas. Compteur 2, aucune couche avancée par un binaire
vide : la suite nomme explicitement J1 B/W. Jeton libre et copies avancées sur master.

## S211 — 2026-09-13 — Premier hôte GPU B + impact

**Entrée.** « oui » aux 254 sources S210, cache Cargo local et verrou versionné ; puis
« continue ». master et trois copies propres à de63a00. Plan seul aa1ecd3, P2 999dba6,
P3 e85a564, P4 fc9c0ba. Sources autorisées récupérées ; aucune extension de l'arbre.

**Sorties.** `Background::render_components` publie les phases repliées et vecteurs d'onde
sans allocation, avec refus atomiques. `viewer/` séparé affiche la mer S201 et l'impact
S203/S205, shader commun calcul/rendu, caméra, pause, relance, témoin, capture et mesures.
Mode d'emploi dans viewer/README ; protocole et résultats dans HOTE-GPU-S211.

**Réception.** 349 tests réussis/cinq ignorés, aucun échec ; compilation release hors réseau.
Sur RTX5070 Laptop/DX12 : hauteur max0,077657 mm contre3 mm déclarés ; pente max0,000123650.
640×360 : GPU eau médiane0,018304 ms ; 960×540 :0,048576 ms, max0,059008 ms.
CPU préparation/transfert/soumission médiane0,470200/0,460600 ms, maxima2,109700/1,953400 ms.
La passe d'eau exclut ciel, transferts et présentation ; **pas de réception 60 images/s / eau
complète2 ms**. Deux captures locales inspectées. Fenêtre ouverte, commandes exercées,
agrandissement observé et fermeture Échap reçue, rotation droite non testée.

**Défaut rencontré.** Instance multibackend : arrêt natif0xc0000005 ; DX12 seul réussit.
Windows sélectionne DX12 ; cause exacte non isolée, autres plateformes non reçues.
La mise à jour des cases P4 par Set-Content -NoNewline a échoué : corrigée immédiatement
et incluse dans le commit P4 amendé avant toute poursuite. Utiliser apply_patch ici.

**Limites et suite.** Sillage absent : **S212, file J1/W, intégrer le sillage du cœur au GPU**,
comparer au CPU et mesurer la scène complète. Angles rasants/caméra, cadence, allocations
de la pile graphique et seconde cible restent ouverts ; J2/δ général et V restent dus.
A250 close pour son objet (chemin GPU enfin exercé et mesuré) ; A247 partiellement traitée,
coût complet encore à recevoir. Aucun nouvel angle ou leçon indépendante des existants.

**Rituel.** File plurielle entière relue, autres déclencheurs conservés. I-03/I-15 : GPU
cosmétique sans autorité ; I-08 : phases repliées, domaine local ; I-06 : méthode cœur reçue,
pile graphique non reçue, aucune prétention de production. Aucun invariant ni ADR changé.
130 ADR,250 angles,284 leçons,18 invariants,6 SPEC,23 cas. Compteur0 : couche B avancée dans
code/water-core/src et B/W affichés. Jeton libéré ; copies à avancer après ce commit final.

## S212 — 2026-09-13 — Sillage du cœur dans l'hôte GPU : exact, trop cher

**Entrée.** « Reprends le projet ». Claude Code (Opus 5). master et trois copies propres à 0e204ad,
jeton libre (S211 close à 10:04). Maillons 0 ; suite S211 = ligne de file J1/W. Plan seul ba915ed,
P2 d398a90, P3 248814b, P4 8afa22f, P5 f060608. Aucune dépendance ajoutée.

**Sorties.** Cœur : `spectral_pressure::Field::render_components` et
`bound_pressure::Prepared::render_components` publient `[A, B, kx, ky]` par nœud, réponse pondérée
tournée de la phase repliée `k·origine` — ni temps ni coordonnée absolue au GPU (I-08), refus
atomiques, test contre `sample_batch` à 1e-5 et témoin vérifié. Hôte : `Wake` de huit tronçons
(3 m/s, 19 620 N, σ 2 m, 64×128) admis au journal, préparé par image, sommé par sommet dans son
emprise ; `--verify` étendu (coutures, témoin 128×256, admission, coûts deux recettes). Réception
[HOTE-GPU-S212](../../docs/validation/HOTE-GPU-S212.md), viewer/README.

**Chiffres qui orientent.** Hauteur GPU/cœur max 0,089 mm (tolérance 3 mm) avec un sillage de
15 cm. Témoin 64×128/128×256 : 0,37 % à 4 s, 1,6 % à 16 s, **9,5 % à 24 s, 28 % à 39 s**. Couture
au bord de l'emprise : 2,3 mm à 16 s, **12,7 mm à 39 s**, plus forte sur la recette fine — contenu
physique. Admission B+pression Ok (enveloppe ≤ 0,165). Coûts médians RTX5070 Laptop/DX12, 4 096
nœuds : **CPU sillage 10,6–10,9 ms**, GPU eau 1,90 ms (640×360) / **4,10 ms** (960×540), contre
0,018/0,049 sans sillage ; 16 384 nœuds : CPU 38–42 ms, GPU 7,8/16,9 ms. Deux lois : GPU
7,6–7,9 ps par sommet×nœud, CPU 317–331 ns par nœud×tronçon. 60 182 pixels différents à 8 s,
motif de Kelvin visible dans l'image de différence, discret à l'œil.

**Décision structurante.** Aucun ADR. Le chemin « préparer à chaque image, sommer par sommet » est
**reçu en exactitude, refusé en coût**. Incompatibilité mesurée **mais pas encore un arbitrage** :
deux leviers techniques, un par loi, aucun mesuré ni retirant de fonction — temps dans le cœur
(tronçons achevés réduits à un état tourné par nœud, tronçon actif préconstruit) ; espace dans
l'hôte (grille cartésienne et transformée ; une somme polaire par texel ne gagne rien). Un ADR
attend la mesure du levier positif, comme ADR-129 a attendu celle de la table.

**Ce qui n'a pas été fait.** Composition mixte impact + sillage (`mixed_water`, budget conjoint
ADR-119) non exercée ; interaction manuelle non rejouée (`--smoke` seul) ; aucune cadence complète ;
fixture de 40 s au-delà de la durée honnête de sa recette, sans garde (A214) — consigné A251.
L248 couvre déjà « l'accord GPU/cœur ne reçoit que la couche non partagée » : pas de doublon.

**Suite S213.** File J1/W : **levier temporel dans le cœur** — l'image ne refait plus la préparation
modale ; recevoir contre `from_journal`, mesurer par image. Puis levier spatial, composition mixte,
coutures (A251). J2/δ général et V-noyau restent dus (ADR-127).

**Rituel.** A251 ouverte ; suivi A247 ; L285 ; file plurielle relue (lignes A247 et J1 datées
S212, autres déclencheurs conservés) ; feuille de route J1 et §4 datées. I-03/I-04/I-15 : GPU
cosmétique ; I-08 tenu (amplitudes complexes, aucun temps f32) ; I-06 : méthode cœur sans
allocation, pile graphique non reçue. Aucun invariant ni ADR changé. Suite complète `code/` :
**350 réussis (252+4+1+93), 5 ignorés**, aucun échec. 130 ADR,251 angles,285 leçons,18 invariants,
6 SPEC,23 cas. Compteur 0 : W avancée dans code/water-core/src. Jeton libre, copies avancées.

## S213 — 2026-09-13 — Cadrage du coût corrigé (ADR-131) et levier temporel du sillage

**Entrée.** Réponse de l'utilisateur au compte rendu S212 : continuer la piste CPU, **mais corriger
le cadrage** — c'est l'implémentation qui dépasse le budget, pas le sillage ni l'objectif ; intégrer
LOD spatiaux, spectraux et temporels, visibilité et mutualisation ; ne pas attendre l'échec de deux
optimisations pour demander de réduire l'ambition ; chaque mesure précise techniques présentes,
absentes et domaine ; 2 ms à éprouver sur la combinaison ; conserver A251 et la composition
impact+sillage. Claude Code (Opus 5), master et copies à 1da11b6. Plan d7837ed, P2 f6d18d0, P3
358998a, P4 681fc5a, P5 0441412, P6 d6b547e, P7 f3d4053.

**Décision structurante.** **ADR-131 actée** sur clarification de l'utilisateur : D1 un dépassement
qualifie l'implémentation mesurée ; D2 espace d'optimisation nommé et ouvert (temps, espace, LOD
spatial, spectral, temporel, visibilité, mutualisation) ; D3 techniques présentes, absentes et
domaine avec chaque mesure ; D4 2 ms éprouvé sur la combinaison ; D5 aucune demande de réduction
fondée sur l'échec d'optimisations isolées ; D6 validité conservée — A251 et composition
impact+sillage restent travaux nécessaires de J1. Note datée ADR-127 D7. Feuille de route : J1-bis.
Cadrage S212 recadré en place (notes datées, verdicts barrés, texte d'origine lisible).

**Sorties code.** `pressure_timeline::Timeline` : tronçons achevés repliés par nœud sur `(η, v/ω)`
à l'instant de référence, rotation entière par image, modes préconstruits, tronçon en cours par
`ModalPressure::sample` (forme sinc gardée à la résonance) ; repli incrémental ou complet ; refus
atomiques. Hôte alimenté par ce levier ; référence `--verify` inchangée (`from_journal`). Exemple
`wake_timeline_cost`. Réception [TEMPS-SILLAGE-S213](../../docs/validation/TEMPS-SILLAGE-S213.md).

**Chiffres qui orientent.** Écart relatif au chemin préparé 6,07e-8 / 2,23e-8 (critère 1e-5) ;
repli incrémental = complet au bit ; témoin : 0,365. Levier seul, 64×128, un fil : **1,26 ms**
pendant forçage contre 7,70 (préparation), **0,36 ms** après contre 13,36 ; pic de borne 2,12 ms ;
retour arrière 3,10 ms ; construction 4,43 ms ; 3,1 Mo. 128×256 : 4,99 / 1,39 contre 30,9 / 52,9.
Hôte : CPU sillage 1,67–1,78 ms (S212 : 10,6–10,9), GPU inchangé 1,91 / 4,18 ms, GPU/cœur 0,089 mm.
**Prédiction contredite** (dixièmes / dizaines de µs) : publiée.

**Ce qui n'a pas été fait.** Composition impact+sillage et A251 : non traitées, maintenues en
travaux nécessaires. Aucune technique GPU. Écart hôte/exemple (1,7 contre 1,26 ms) non attribué ;
max 7,3 ms isolé non attribué. Captures toujours sous `captures/s212/`.

**Suite S214.** File J1, validité avant accélération (ADR-131 D6) : composition impact + sillage par
`mixed_water` sur la scène, puis A251 (emprise et durée d'image déduites de la recette, coutures).
Ensuite la loi GPU (espace, LOD, visibilité, mutualisation), chaque mesure avec son en-tête.

**Rituel.** A252 (sévérité 2, traitée), L286 ; A251 note S213 ; file plurielle entière relue (J1,
A247, suivi S213 ; autres déclencheurs conservés) ; feuille de route J1/J1-bis/§3/§4/§5. Invariants
I-05/I-06/I-08/I-09/I-12 relus : repli sans allocation dans le rendu, phases entières, aucun temps
f32 ; aucun amendé. Suite complète `code/` : **354 réussis (256+4+1+93), 5 ignorés**, aucun échec. 131 ADR,252 angles,286 leçons,18 invariants,
6 SPEC,23 cas. Compteur 0 : W avancée (code src) et ADR-131 actée. Jeton libre, copies avancées.

## S214 — 2026-09-13 — Composition par le cœur (exacte au bit), budget conjoint à 84 %, ADR-132

**Entrée.** Ligne `Session suivante` de S213, conforme à ADR-131 D6 et à la §Suite de
TEMPS-SILLAGE-S213 : validité avant accélération — composition impact + sillage par `mixed_water`
sur la scène, puis A251. Aucune entrée utilisateur nouvelle. Maillons 0 à l'amorce. Claude Code
(Opus 5) ; `master` et trois copies propres à 08c2011. Plan b82b325, P2 879774c, P3 9bd3175,
P4 8f7fc3b, P5 fba6d08, P6 0f97ead, P7 0c4b12b, P8 88b8a02.

**Décision structurante.** **ADR-132 actée** : le domaine d'image d'un sillage se calcule depuis sa
recette — `rayon = 2π·angular/(3·cutoff)`, `durée = 4π/√(g·cutoff/radial)` — et l'hôte l'**annonce**
sans refuser (ADR-091, chemin cosmétique ADR-129 §3). Une durée honnête porte le critère qui l'a
calibrée. La fixture S212 est déclarée hors domaine et **conservée**, parce qu'elle a servi à
recevoir S211 à S214. `water-core` intact ; aucun bit publié ne change. **A251 traitée.**

**Chiffres qui orientent.**
- **La composition du cœur est exacte au bit** contre la somme à la main de l'hôte, quand les deux
  évaluent au **même point** : écart η `0,000000000` aux cinq âges, pentes 1,5e-8 (un ulp).
- Avant correction, la même comparaison donnait **1,78e-5 m** — 18× le critère déclaré de 1e-6. La
  prédiction écrite pour être contredite l'a été. Cause : B au point monde quantifié (1/2048 m,
  ≤ 244 µm), impact et sillage au `f32` brut. **A253**, corrigé dans l'hôte.
- **Budget conjoint 0,3477 à 0,3776 contre π/7 = 0,4488** — 77,5 à **84,1 %** — pour *une* source de
  chaque type. Pente réelle des perturbations 0,0929 à 0,0352 : **majorant 3,7 à 10,5 fois** le réel.
  Marge 0,0712 : une troisième source refuse toute l'image, et par `SlopeEnvelope` (vérifié :
  4 477/4 477, zéro `Slope`). **A254, sévérité 1.** La pente réelle citée est un maximum
  **échantillonné**, pas le maximum sur l'emprise (S140) : le rapport majore le pessimisme sans le
  chiffrer exactement. **I-18 tenu** — les deux termes sommés sont déjà convertis (S141).
- **Le cœur ne compose que sur l'intersection** des domaines (ADR-077, ADR-080) : 4 477 sondes sur
  6 988. Le chemin mixte ne peut donc pas être le chemin de rendu — ce n'est pas un défaut, c'est la
  sémantique de service, et c'est pourquoi l'hôte sommait à la main.
- **Domaine du sillage** : 89,36 m et 18,53 s contre un coin d'emprise à 102,22 m et un contexte de
  40 s (**2,2×**). Trois critères : 2 % franchi entre 16 et 18 s, couture de 3 mm entre 18 et 20 s,
  rayon d'accord à 10 % entre 24 et 30 s. Le rayon ne mord jamais ici.
- **Coût variable de 20 % entre passages** du même binaire : 1,2555 / 1,2458 (froid), 1,6477 /
  1,7760, 1,5820 / 1,8398 ms. **L'écart hôte/exemple non attribué de S213 n'avait pas de cause à
  chercher** : c'était un rang de passage. **L289**, suivi A247.
- GPU eau 1,897 / 4,183 ms inchangé ; `VERIFY` max 8,03e-5 m, tolérance 3 mm tenue ; `--smoke` 120
  images code 0. Suite `code/` : **354 réussis (256+4+1+93), 5 ignorés**.

**Ce qui n'a pas été fait.** A254 ouverte le jour de sa découverte, non traitée — elle demande une
décision sur le majorant, sa composition ou `max_slope`, et elle passe avant toute scène à plusieurs
sources. A253 reste ouverte du côté de l'interface (`eval_local` est `pub(crate)` ; un hôte ne peut
pas évaluer B hors du réseau monde). A214 gagne un point de calibration, pas un garde. Aucune ligne
de `code/*/src` ajoutée : tout le travail est dans `viewer/` et dans la conception. Cadence
complète, interaction manuelle, poses de caméra, allocations de la pile (I-06), seconde cible (B7)
toujours dus. Captures toujours sous `captures/s212/`.

**Suite S215.** File J1, et **A254 d'abord** : le budget de pente ne passe pas à l'échelle en nombre
de sources. Mesurer une scène à deux impacts ou deux sillages, décider si le majorant se resserre
(le rapport 3,7–10,5 dit qu'il le peut), s'il se compose autrement que par la somme, ou si
`max_slope` cesse d'être une constante de milieu — puis reprendre la loi GPU (espace, LOD,
visibilité, mutualisation), chaque mesure avec son en-tête (ADR-131 D3). J2/δ général et V-noyau
restent dus (ADR-127).

**Rituel.** A253 (sévérité 2, traitée dans l'hôte), A254 (sévérité 1) ; suivis A251 (traitée par
ADR-132), A214, A247 ; L287, L288, L289 ; file plurielle entière relue ; feuille de route J1/J1-bis
datées S214 ; index, README, REPRISE §3/§4 et jeton. Invariants relus que la décision cite —
I-04 (autorité : l'hôte annonce, ne décide pas de la physique), I-06 (aucune allocation ajoutée au
pas ; pile graphique toujours non reçue), I-08 (une conversion monde → local, aucun temps `f32`),
I-14 (les deux lois citent leur provenance : ADR-107 et les mesures S156/S214), I-18 : aucun n'est
devenu faux, aucun amendé. Aucun ADR réécrit. Suite complète `code/` : **354 réussis, 5 ignorés**,
aucun échec. 132 ADR, 254 angles, 289 leçons, 18 invariants, 6 SPEC, 23 cas. **Compteur 0** :
ADR-132 fixe un élément de W, comme ADR-126 l'avait fait en S203. Jeton libre, copies avancées.

## S215 — 2026-09-13 — Le majorant de pente suit la dispersion (ADR-133) ; A254 traitée pour moitié

**Entrée.** Ligne `Session suivante` de S214 : A254, le budget de pente est une somme sur les
sources. Maillons 0 à l'amorce. Claude Code (Opus 5) ; `master` et trois copies propres à 70d2b36.
Plan 4bb9d9b, P2 0e6dad3, P3-bis a659ae6, P4 c4ed436, P5 a2756b7, P6 effceaf, P6-bis 5fa3567,
P7 fb3d5e4.

**Décision structurante.** **ADR-133 actée** : `RadialImpact` publie `slope_max_at(t)` — le majorant
de pente **à l'instant demandé** — et le budget de composition le consomme ; `slope_floor` reçoit
l'instant, sans quoi la garantie dans les deux sens d'ADR-128 cesserait de tenir. `slope_max()` et
le refus `Steepness` sont **inchangés** : ADR-094 a posé que les migrer est une décision distincte,
et elle le reste. Aucun bit publié ne change — `steepness` continue de lire `slope_max()`.

**Chiffres qui orientent.**
- **Le doute d'échantillonnage de S214 était fondé, et il a changé la cause.** À échantillonnage
  fin (20 001 points de rayon ; pic en `r = 0,2062 λ` = 0,69 m), le pessimisme vient **presque
  entièrement de l'impact** : 1,00 à la naissance, 4,30 à 4 s, 10,05 à 16 s, 20,20 à 39 s,
  **30,44 à 56 s**. Le sillage est **serré** : 1,39 à 4 s, 1,90 à 16 s, 4,77 à 39 s.
  **Prédiction écrite avant mesure — impact serré, sillage lâche — exactement inversée.**
- **Sûreté vérifiée sur 56 s** là où ADR-094 n'avait vu que 2 s : `max(réel/majorant)` = 0,999998
  (impact) et 0,718847 (sillage). Aucun majorant dépassé.
- **Le mécanisme est la dispersion**, et il est général à W : une somme de modules modaux est
  invariante quand les modes ne font plus que tourner, le maximum spatial ne l'est pas. **L290.**
- **La décroissance est universelle** : identique à trois décimales pour λ ∈ {0,5 ; 1 ; 3,35 ; 8} m
  et E ∈ {0,05 ; 0,5 ; 16,4 ; 164 ; 4 000} J, en âge adimensionné `τ = (t−birth)/√(λ/g)`.
  Indépendante de l'amplitude (deux E à λ égal), et **indépendante de la profondeur** de 20 à 4 m —
  en deçà `RadialImpact::new` refuse le champ lui-même. **L292.**
- **Table sûre** `RHO_DISPERSION` : minimum par intervalle (ρ n'est pas monotone : 8,713 à τ = 0,5
  puis 1,063 à τ = 1) sur 21 sous-échantillons et quatre λ génératrices, garde `1e-4` = 33 × le
  dépassement maximal mesuré (3,0e-6). Contrôle sur **quatre λ hors famille**, 961 τ chacune :
  `max(réel/resserré) = 0,999983`, **exactement** le pire cas du majorant d'origine.
- **Le refus a été exercé puis levé.** Deux impacts + un sillage : budget 0,590210 contre 0,448799,
  `Err(SlopeEnvelope)` — A254 confirmée dans ses termes. Après ADR-133 : 0,208086, **`Ok(())`**, et
  trois impacts passent à 51 %. Occupation de la scène J1 : **84 % → 42 %**.
- Hôte : `d_eta_m = 0,000000000` (aucun bit publié changé), `VERIFY` et GPU inchangés, `--smoke`
  code 0. Suite `code/` : **355 réussis (257+4+1+93), 5 ignorés** — un de plus, le test de sûreté.

**Ce qui n'a pas été fait.** **A255 ouverte le jour même** : le sillage pèse maintenant 88 % du
budget et sa famille n'a aucune loi — le forçage y complique le cas, puisqu'un majorant **croît**
pendant qu'une source émet. Le budget reste une **somme** : A254 n'est traitée que pour moitié, le
nombre de sources demeure une ressource bornée. A253 reste ouverte côté interface. A208 est
inchangée : elle n'était pas le terme dominant, elle n'est pas close. Aucune mesure de coût
d'image : ADR-133 est un contrat d'admission, pas une optimisation.

**Suite S216.** File J1 : **A255** — la même campagne que S215 sur la famille du sillage (varier σ,
cutoff, radial, angular et le découpage en tronçons ; chercher une échelle de temps propre ; ne pas
présumer qu'elle existe ; traiter la phase de forçage séparément). C'est elle qui décide de ce que
coûte une scène à plusieurs sillages, et donc de la mutualisation de J1-bis. Ensuite la loi GPU.
J2/δ général et V-noyau restent dus (ADR-127).

**Rituel.** A255 (sévérité 2) ; suivis A254 (traitée pour moitié, cause corrigée), A208 (non close,
non dominante) ; L290, L291, L292 ; file plurielle entière relue ; feuille de route J1/J1-bis ;
index, README, REPRISE §3/§4 et jeton. Invariants relus que la décision cite — **I-18** (ce qui est
comparé à `max_slope` est une pente réelle : ADR-133 le sert mieux qu'avant, le majorant étant plus
proche de la pente réelle à chaque instant), I-14 (la table cite son banc et sa garde son
dépassement mesuré), I-04, I-06 (aucune allocation : la table est une constante), I-08 : aucun n'est
devenu faux, aucun amendé. Aucun ADR réécrit. **Compteur 0** : W avancée dans `code/water-core/src`
(`radial_impact`, `composition`, `mixed_water`, `mixed_differential`) **et** ADR-133 actée.
133 ADR, 255 angles, 292 leçons, 18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S216 — 2026-09-13 — L'enveloppe de pente tient compte des directions (ADR-134) ; A255 pour moitié

**Entrée.** Ligne `Session suivante` de S215 : A255, la famille du sillage. Maillons 0 à l'amorce.
Claude Code (Opus 5) ; `master` et trois copies propres à bd4030b. Plan 7da10d2, P2 4d25e25,
P3 39d569a, P5 a7e8785, P6 5a6018e, P7 faf4129.

**Décision structurante.** **ADR-134 actée** : `Field::slope_envelope_directional()` — majorant de
pente qui tient compte de l'**étalement des directions entre modes**, par Cauchy–Schwarz, en `O(N)`
et sans aucune calibration ; `bound_pressure::Prepared` la retient aux trois sites de préparation.
`slope_envelope_tight()` reste publiée et sert de témoin. Aucun bit publié ne change.

**Ce qui a orienté la session, et ce n'était pas prévu.** La suite écrite en S215 demandait de
refaire sur le sillage la campagne **mesurée** de l'impact. La lecture du code l'a déplacée avant
toute mesure : `slope_envelope_tight` somme **scalairement** des contributions **vectorielles** de
directions différentes. Une partie du pessimisme ne demandait donc pas une table mais une
inégalité. **L293.**

**Chiffres qui orientent.**
- **Décomposition** (contrôle de lecture : somme reconstruite = publiée à 1e-7). Part statique
  1,5548 à 0,5 s, **1,2586 à 4 s**, 1,2154 à 16 s, **1,1979 à 39 s** ; résidu 1,1015 à 4 s,
  1,5616 à 16 s, **3,9782 à 39 s**. **Prédiction contredite pour moitié** : j'avais écrit « part
  statique ≈ 1,5 expliquant l'essentiel pendant le forçage » — elle retombe à 1,20 et explique
  moins de la moitié à 16 s. Le sillage de Kelvin concentre son énergie dans un cône ; un demi-disque
  uniforme aurait donné π/2 ≈ 1,571.
- **La part statique ne dépend pas de la quadrature** : 1,2586 à quatre décimales pour `angular`
  64/128/256, `radial` 32/128, `cutoff` 2/4 — donc elle se **calcule**, elle ne se tabule pas. Elle
  bouge avec le champ : 1,31 à σ = 1, 1,40–1,52 à 1,5 m/s, 1,44–1,48 à 6 m/s. σ = 4 refusée à la
  construction.
- **Le demi-spectre ne porte que `angular/2` directions distinctes** (32/64/128 mesurées) : un
  maximum exact coûterait `O(A²)`. Écarté au profit de Cauchy–Schwarz, qui ne suppose rien sur la
  disposition des emplacements — arbitrage écrit dans l'ADR.
- **Discriminant d'emprise, et il tranche** : à pas de grille **constant**, seize fois l'aire
  (512 × 416 m) laisse le maximum réel identique à six décimales. Le résidu n'est **pas** une limite
  d'emprise (A208) : c'est la décohérence de L290. **L295.**
- **Réception** : gain 1,2002 / 1,1671 / 1,1552 à 4 / 16 / 39 s, soit **79 à 87 %** du gain qu'un
  maximum exact rendrait. Budget de la scène J1 0,186540 → **0,162917** à 16 s ; occupation de π/7
  **84,1 % (S214) → 41,6 % (ADR-133) → 36,3 % (ADR-134)**.
- Hôte : `d_eta_m = 0,000000000`, `VERIFY` et GPU inchangés, `--smoke` code 0. Suite `code/` :
  **356 réussis (258+4+1+93), 5 ignorés** — un de plus, le test des deux bouts d'ADR-134.

**Ce qui n'a pas été fait.** **A255 reste ouverte sur sa part dynamique**, et ne contient plus que
cela : le résidu monte à 3,98 à 39 s, il est gouverné par le temps depuis l'extinction, et rien ne
dit qu'il admette une échelle propre — la campagne d'ADR-133 transposée reste à faire, avec la phase
de forçage traitée à part puisque le majorant y **croît**. Le budget reste une **somme** (A254).
**A256 ouverte** : une annonce qui nomme le facteur qu'elle retire sans dire à quelle échelle il
s'applique se lit comme complète — c'est ce qui a caché ce gain depuis S141. A253 inchangée. Aucune
mesure de coût d'image : ADR-134 est un contrat d'admission.

**Suite S217.** File J1 : **la part dynamique d'A255** — chercher si le résidu de décohérence admet
une échelle de temps propre depuis l'extinction, sur la famille du sillage, forçage traité à part ;
et ne pas présumer qu'elle existe, S216 ayant montré qu'une partie du pessimisme n'en demandait pas.
Ensuite la loi GPU (espace, LOD, visibilité, mutualisation). J2/δ général et V-noyau restent dus
(ADR-127).

**Rituel.** A256 (sévérité 2) ; suivi A255 (part statique traitée) ; L293, L294, L295 ; file
plurielle entière relue ; feuille de route J1/J1-bis ; index, README, REPRISE §3/§4 et jeton.
Invariants relus que la décision cite — **I-18** (ce qui est comparé à `max_slope` est une pente
réelle : ADR-134 le sert mieux, le majorant étant plus proche de la pente réelle) ; I-06 (aucune
allocation : deux accumulateurs scalaires, aucun tableau intermédiaire — c'est d'ailleurs pourquoi
le maximum exact a été écarté) ; I-14 (aucun nombre nouveau : la borne est une inégalité, elle n'a
rien à calibrer) ; I-04 et I-08 inchangés. Aucun devenu faux, aucun amendé, aucun ADR réécrit.
**Compteur 0** : W avancée dans `code/water-core/src` (`spectral_pressure`, `bound_pressure`) **et**
ADR-134 actée. 134 ADR, 256 angles, 295 leçons, 18 invariants, 6 SPEC, 23 cas. Jeton libre, copies
avancées.

## S217 — 2026-09-13 — Le temps depuis extinction ne suffit pas à décrire le sillage

**Entrée.** Reprise demandée par l'utilisateur ; file J1/A255 après S216. Codex,
fichiers/git/cargo disponibles. Master et trois copies propres à b1860c1 ; maillons0.
Plan d875a46, protocole e08fb8b, instrument/campagne dfaa24d, verdict8866fec.
Travail dans la copie principale, aucune dépendance ni nouvelle copie.

**Résultat structurant.** Similitude conditionnelle reçue sous sigma→a sigma,
x→a x, v→sqrt(a)v, durée→sqrt(a)durée, cutoff→cutoff/a, charge→a³charge.
Mais vitesse réduite et durée réduite restent indépendantes : **la courbe unique en
(t-D)/sqrt(sigma/g) est réfutée**, pas toute possibilité d'une famille paramétrée.
Voir [DECOHERENCE-SILLAGE-S217](../../docs/validation/DECOHERENCE-SILLAGE-S217.md).
Aucun ADR nouveau ni code d'exécution modifié ; A255 reste ouverte.

**Mesures.** 54 lignes initiales, neuf contrôles affinés. À tau = 4 et256×512,
majorant publié/maximum : base1,637275, vitesse divisée par deux2,272143 (+38,8 %),
durée doublée2,005464 (+22,5 %). Variation des maxima128→256 <0,002 % ; contrôle
spatial <0,025 %. Homothéties, charge et découpage reçus sous0,1 %. Reconstruction
f64/cœur <=2,383e-6 (seuil de banc1e-5). Les cas discriminants sont dans la durée
d'image annoncée ; les lignes plus tardives hors durée restent diagnostiques.
Aucun temps ni gain GPU mesuré. Deux tests d'instrument réussis debug et release.

**Correction non prévue.** La réponse libre de pression mélange hauteur et vitesse
complexes : |eta| n'est pas invariant ; l'énergie l'est. Le test mono-mode le
vérifie. « Majorant figé » de S215/L290 et « majorant croît pendant le forçage »
comme règle générale sont corrigés, avec notes datées ADR-133 et reçus S215/S216.
L'inégalité ADR-134 et la réception de l'impact ADR-133 ne sont pas invalidées.

**Non fait.** Pas de table sûre, pas de borne locale construite, pas de scène à
plusieurs sillages ni loi GPU. Cutoff et géométrie du trajet non balayés indépendamment ;
ils ne sont pas nécessaires à cette réfutation, mais restent requis pour une autre loi.
Le maximum échantillonné demeure un minorant, pas une preuve de sûreté continue.

**Suite S218.** File J1/W : construire et recevoir une borne locale de pente depuis
le champ préparé, avec reste spatial démontré (borne de Hessienne comme piste), puis
mesurer le gain de budget et le coût. Ne pas tabuler les maxima S217 comme bornes.
A254 reste la somme des sources ; GPU/espace/LOD/visibilité/mutualisation ensuite.
J2/δ général et V-noyau obligatoires conservés. Aucun arbitrage humain nouveau.

**Rituel.** A257 (sévérité2, explication corrigée) ; suivi A255 ; L296 et note L290.
File plurielle relue : A244/S200-1 et S199-2 à J2, V-noyau au plus tard J2 ; B4
forces/perception, A216, A241, A213, B2/coupure, bathymétrie, multiplateforme et
réunions gardent leurs déclencheurs. Invariants I-03/04/06/08/14/18 relus et inchangés :
l'instrument f64 reste un exemple hors exécution, aucune garantie de production élargie.
Maillons **1** : cette session éclaire W sans l'avancer au sens de REPRISE §6.8.
**Vérification finale.** Suite complète `cargo test --offline --release --workspace` :
356 réussis (258+4+1+93), cinq ignorés, zéro échec ; deux tests d'exemple supplémentaires
réussis en debug et release. La suite debug complète a été arrêtée pendant les anciens
balayages coûteux, sans verdict complet ; elle n'est pas comptée comme reçue. Les avertissements
préexistants du harnais/exemples sont conservés. 134 ADR,257 angles,296 leçons,
18 invariants,6 SPEC,23 cas ; compte des angles incluant les entrées de tableau héritées.

## S218 — 2026-09-13 — Une borne locale construite, un parcours uniforme trop coûteux

**Entrée.** Utilisateur « Continue », Codex, copie principale ; master et trois copies
propres à af1212b, jeton libre, maillons1. Plan f326d4f, ADR7351760,
construction e2a2eda, réception71cc850. Aucun téléchargement ni nouvelle copie.

**Construction W.** ADR-135 actée ; Field et Prepared publient une annonce locale
avec pente centrale, reste spatial et réserve numérique. Les phases réellement
arrondies aux bornes remplacent une simple variation continue k·dx. Refus domaine,
contexte, temps, débordement et norme sous-passant à zéro. Aucune allocation d'après
inspection du code, sans instrumentation. Échantillons et admissions historiques inchangés.
La preuve trigonométrique et la réception empirique ne certifient pas toute la chaîne f32.
Voir [BORNE-LOCALE-S218](../../docs/validation/BORNE-LOCALE-S218.md) et ses relevés bruts.

**Mesures déterminantes.** À0,5m, globale/partition vaut1,025625 (base),1,224415
(lente),1,156292 (longue) ; base tardive1,343845 hors durée recevable. Les partitions
2m/1m ne resserrent pas. Deux passages isolés de la base : 27,546 et28,209s pour49152
rectangles, préparation6,279/6,957ms séparée ; valeurs imprimées identiques.
Le premier passage complet partageait la machine avec compilation/tests : coût non nominal.
Verdict sur cette implémentation uniforme CPU, sans retirer de fonctionnalité ni conclure GPU.

**Vérification.** Workspace release360 réussis (262+4+1+93),5 ignorés ; après protection
finale de sous-passement,4 tests ciblés debug/release rejoués et réussis. Zéro, centre
manquant le pic, translation4000m, coins et rectangles multidirectionnels, refus et
identité des échantillons. Avertissements préexistants conservés.

**Non fait et suite S219.** File J1/W, construire une partition adaptative dans un pool
fourni par l'appelant, avec plafond de travail et couverture conservée par les bornes
des rectangles non raffinés ; recevoir sûreté de couverture, capacité, gain et coût.
Migration d'admission non décidée, certification f32 ouverte (A258). A255 partielle,
somme A254 et loi GPU toujours ouvertes. Aucun arbitrage humain nouveau.

**Rituel.** A258 sévérité2, L297 ; file active entière relue, autres déclencheurs
conservés : A244/S200-1 et S199-2 à J2, V-noyau au plus tard J2 ; B4 forces/perception,
A216, A241, A213, B2/coupure, bathymétrie, multiplateforme et réunions.
I-03/04/06/08/14/18 restent applicables ; aucune admission élargie, aucune promesse de
seconde cible, réserve numérique explicitement non certifiée. Maillons0 : W src et ADR
avancés. Erreur de procédure : battement P2 recopié14:59 après lecture14:57, corrigé par
battements mesurés ; diff P4 signalait deux lignes finales vides, nettoyées au rituel.
135 ADR,258 angles,297 leçons,18 invariants,6 SPEC,23 cas. Jeton libéré en fin de session,
avance rapide des copies après vérification de leur propreté.

## S219 — 2026-09-13 — Partition adaptative à couverture conservée

**Entrée.** Continue, Codex ; master et trois copies propres à4f9841f, jeton libre,
maillons0. Lectures de S218 conservées, état réel revérifié. Copie principale,
aucune dépendance ni nouvelle copie. Plan3a7cf59, construction e8eb0fb, réception8819b6b.

**Construction W.** `partition_slope_envelope` sur Field/Prepared, tas maximal dans
le pool fourni par l'appelant. Division du grand côté, priorité à la borne puis à
la largeur, minimum des bornes enfant/parent. Couverture complète et maximum non
croissant ; budget1+2 par division. Arrêt explicite capacité/évaluations/précision/zéro,
refus pool vide/budget nul/erreurs locales. Contexte et instant contrôlés.
Pas de reprise entre appels : repartir du champ courant évite un cache périmé ;
une future reprise doit lier le pool à la publication. Aucune admission migrée.
Implémentation d'ADR-135, aucun nouvel ADR ; A258 reste ouverte.

**Mesures.** [PARTITION-S219](../../docs/validation/PARTITION-S219.md), deux passages
isolés base et un passage quatre fixtures. À65535 évaluations : globale/borne1,488482
(base),1,520529 (lente),1,480557 (longue), soit32,5–34,2 % de réduction. Base35,637 et
35,754s, valeurs imprimées identiques ; pool640Kio, préparation6,491/6,454ms séparée.
À32767 évaluations : base0,098479681 en17,79–17,97s, contre0,112293623 en27,55–28,21s
uniforme S218 ; mais lente moins serrée à ce plafond que la grille0,5m. À8191 : aucun
gain sur les quatre cas, encore4,4–4,5s. Diagnostic tardif hors durée d'image conservé.
Aucun verdict GPU ni retrait de fonctionnalité ; coût de cette implémentation seulement.

**Obstacle.** A259 : la borne globale commune masque les différences spatiales aux
grandes mailles. Le tas ne peut hiérarchiser une information absente. **Suite S220,
file J1/W : construire une borne locale conservant les annulations** (pente et Hessienne
signée au centre, reste supérieur borné), avec traitement des phases quantifiées ;
recevoir gain/coût via S219. Garder ADR-135 comme repli, ne pas promettre de gain.
A258 avant migration d'admission ; somme A254 et loi GPU restent ouvertes.

**Vérification.** Deux nouveaux tests debug ; suite complète release362 réussis
(264+4+1+93),5 ignorés, zéro échec ; test Prepared existant étendu contexte/temps.
Couverture, aire, sondes, budgets pairs/impairs, capacité, point, zéro, refus et
identité bits/rectangles exercés. Allocation absente par inspection, sans instrumentation.
Aucun changement du calcul historique des échantillons.

**Rituel.** A259 sévérité2, L298 ; file active relue, A255 partielle, A258 ouverte.
A244/S200-1 et S199-2 à J2, V-noyau au plus tard J2 ; B4 forces/perception, A216,
A241, A213, B2/coupure, bathymétrie, multiplateforme, bancs et réunions gardent leurs
déclencheurs. I-03/04/06/08/14/18 restent applicables ; I-05 ne transforme pas ce plafond
d'évaluations en budget temporel, explicitement non reçu. Aucun arbitrage humain nouveau.
Maillons0 : W avancée dans src. 135 ADR,259 angles,298 leçons,18 invariants,6 SPEC,23 cas.
Jeton libéré, copies à avancer après le commit de clôture et vérification de propreté.

## S220 — 2026-09-13 — L'ordre deux rejoint le maximum ; la réserve devient le plancher

**Entrée.** « Reprends le projet », Claude Code (Opus 5), copie principale ; master et trois
copies propres à 8ef0c64, jeton libre, maillons 0. Plan 94df5ff, ADR 5df7a18, construction
3ae9150, tests 9d1b08d, campagne 0430776, réception d3a57a3. Aucun téléchargement ni copie.
Interruption utilisateur pendant P4 : niveau d'effort du modèle constaté `xhigh`, réglé par
l'utilisateur dans l'app (une session ne peut pas changer le sien), puis reprise.

**Construction W.** [ADR-136](../../docs/adr/ADR-136-borne-locale-d-ordre-deux-a-hessienne-signee.md)
actée : chaque mode développé à l'ordre deux autour de la phase exécutée au centre ; la dérivée
est `η_k(c)`, d'où une Hessienne **signée** `M = −Σ w_k ⊗ 2π t_k η_k(c)` dont le maximum
convexe est aux quatre coins. Écart phase exécutée/linéaire `E_k` (produits `t·x`, fraction,
Q32, coins) payé par mode ; mode dans `M` si `E + D²/2 < min(2, D)`. Une passe `O(N)` :
`Slot::accumulate` rend `(sin, cos)` sans changer une opération, la branche ADR-135 est
recalculée au bit, `bound = min(ordre un, ordre deux)`. `Field`/`Prepared` publient
`local_slope_envelope_second_order` et `partition_slope_envelope_order` ; l'appel S219
délègue en `First`. Aucune admission migrée ; réserve non certifiée (A258).

**Mesures déterminantes.** [ORDRE-DEUX-S220](../../docs/validation/ORDRE-DEUX-S220.md), cinq
processus isolés. Ordre un reproduit S218 et S219 au bit. Grille 0,5 m ordre deux : gain 1,41 à
1,61 sur l'ordre un ; à 1 m, déjà meilleure que l'ordre un à 0,5 m en 11,5–11,9 s contre
27,5–28,2 s. Partition : **à 32767 évaluations, ordre deux 1,006 à 1,012 × maximum de
référence**, mieux que l'ordre un à 65535 sur les quatre fixtures, en 29,7–33,8 s contre
34,7–37,2 s ; 65535 n'apporte presque rien (1,005–1,011). Gain sur la globale 1,63 à 2,58.
À 2047 et 8191 évaluations, aucun gain dans les deux ordres. Évaluation d'ordre deux 1,6 à
2,0 × ADR-135. Base passage 2 identique au bit.

**Deux régimes identifiés par la feuille maximale.** Feuilles millimétriques : plafonnées par
la **réserve numérique** — reste 160–360 × plus petit qu'elle sur les feuilles d'ordre deux,
réserve 0,38 % (ordre un, base) à 1,26 % (ordre deux, lente) de la référence. Mailles
2 × 1,5 m : le reste d'ordre deux vaut 74–87 % de la globale, fait des modes à grande largeur
de phase — exclus à `2 c_k`, inclus proches de `D = 2` ; répartition non mesurée (A260). Fausse alerte de sûreté examinée avant publication : un agrégat
laissait croire la borne sous « maximum + réserve d'ordre deux » ; la feuille était plafonnée
par l'ordre un, dont la réserve est deux fois plus petite (L299).

**Vérification.** Quatre tests S220 debug ; suite release 366 réussis (268+4+1+93), 5 ignorés,
zéro échec. Deux attentes fausses du test quadratique corrigées avant campagne, sans toucher
au code : borne globale exacte pour un mode unique (elle plafonnait les deux branches), et
excès d'ordre un `h` et non `2h` ; plancher `E ≈ 6·10⁻⁶` mesuré. Toutes les abscisses
f32 sondées près de 4000 m. Allocation absente par inspection, sans instrumentation.

**Non fait et suite S221, file J1/W.** Trois leviers publiés dans la réception, sans
prédiction chiffrée : **A260** enveloppe directionnelle du sous-ensemble non résolu
(`G(U) + |S_U(c)| ≤ 2 C_U` : jamais pire que le traitement actuel) pour lever A259 sous
16 000 feuilles ; **A258** borne d'erreur courante dans la passe, prérequis de migration et
plancher de précision ; coût (×1,7) et validité temporelle d'une borne d'instant. Recommandé :
A260 d'abord, parce qu'il réduit le travail nécessaire, ce que les deux autres ne font pas.
A254, loi GPU, migration d'admission restent ouvertes. Aucun arbitrage humain nouveau.

**Rituel.** A260 sévérité 2 ; suivis A255 (pessimisme résorbé sur ces fixtures, coût restant),
A258 (plancher), A259 (partielle) ; L299. File active entière relue : A244/S200-1 et S199-2
à J2, V-noyau au plus tard J2 ; B4 forces/perception, A216, A241, A213, B2/coupure,
bathymétrie, multiplateforme, bancs et réunions gardent leurs déclencheurs.
I-03/04/06/08/14/18 relus : la passe ne change aucun échantillon (I-03), les marges `8ε`,
`32ε`, `γ` sont dérivées et datées (I-14), aucune admission élargie (I-18). Maillons 0 :
W avancée dans src et ADR-136 actée. Procédure : un commit P3 d'abord créé sans sa case cochée
(chemin relatif .NET faux), complété par `--amend` avant tout autre commit ; journal de
campagne détachée bloqué par `tail -f`, surveillance restée ouverte 23 minutes après la fin.
136 ADR, 260 angles, 299 leçons, 18 invariants, 6 SPEC, 23 cas. Jeton libéré ; copies avancées
après le commit de clôture.
## S221 — 2026-09-13 — La coupure spectrale lève le plateau à 4096 feuilles ; la localisation manque

**Entrée.** « Continue » dans la même conversation, juste après S220 ; Claude Code (Opus 5),
copie principale, master et trois copies à 32e7afd, jeton libre, maillons 0. Plan 17b9494, ADR
7005f68, construction 9cd5636, tests dcb5a91, campagne 8422cdb, réception 2fca93a.

**Construction W.** [ADR-137](../../docs/adr/ADR-137-coupure-spectrale-de-la-borne-locale.md) actée :
`|S(p)| ≤ |S_R(p)| + |S_U(p)|`, `|S_U| ≤ G(U)` (ADR-134 sur le sous-ensemble), ADR-136 sur `R`.
Quatre classes de largeur de phase accumulées dans la passe d'ADR-136, rendue générique sur une
constante ; coupures `D* ∈ {2, 1, ½}` ; minimum avec ADR-135/136. **Bits de l'ordre deux figés
avant refactorisation** par un test. `Field`/`Prepared` publient
`local_slope_envelope_spectral` et `SlopeOrder::Spectral`. Aucune admission migrée.

**Mesures déterminantes.** [COUPURE-SPECTRALE-S221](../../docs/validation/COUPURE-SPECTRALE-S221.md).
Partition à 8191 évaluations : **0,1020 / 0,02654 / 0,1205 / 0,1031** contre le plafond global
d'ADR-136 — gain 1,115–1,230, **A259 levée à 4096 feuilles** ; 16383 : gain 1,00–1,18 sur
ADR-136 ; 2047 : 1,003–1,024 ; 32767 : identiques au plancher de réserve. **Hypothèse du plan
juste sur l'effet, fausse sur le mécanisme** : à 2 × 1,5 m les modes `D ≥ 2` portent 1,1–2,2 %
de la masse, la classe `[1, 2)` 44–67 % ; la coupure utile est `D* = 1`. Pire rectangle base :
`C_U` = 94 % de `C`, `G(U)` = 0,1097 pour une globale de 0,1152 — la limite spatiale écrite
dans ADR-137 avant mesure (A261). Coût : spectrale 26,1–26,3 s contre ordre deux 31,5–32,1 s à
32767 ; micro-mesure 709–758 µs contre 845–922 µs ; ordre deux passé par la passe `<true>` →
710 µs à bits identiques : **écart de code machine**, source rétabli en `<false>` (L300).

**Vérification.** Cinq tests S221 (dont bits figés) et contrôles Prepared ; release 370 réussis
(272+4+1+93), 5 ignorés, zéro échec. Une attente fausse (masse d'une classe arrondie, un ulp)
corrigée avant campagne. Base jouée deux fois, bornes identiques au bit.

**Bilan de ligne, A211.** S215–S221 : sept sessions chaînées sur le budget de pente du sillage,
chacune a fait avancer W (maillons 0), et le chaînage était donc permis. Où en est la ligne : pessimisme résorbé à
l'instant (1,006–1,012 × le maximum), plateau levé à 4096 feuilles. Restent le coût, A258,
A261 — **aucun de ces trois ne bloque la scène J1 actuelle**, qui passe avec une source de chaque type.
Ce qui la bloquera est **A254, part somme**, dès plusieurs sources. Vérifié dans
`mixed_water::slope_floor` : les impacts y sont **additionnés** champ par champ (`slope_max_at`),
tandis que toutes les sources de pression forment **un seul** `bound_pressure::Prepared`, dont
l'enveloppe directionnelle est déjà conjointe mode par mode. Mais cette enveloppe ne voit pas
que deux sillages sont éloignés l'un de l'autre (A261), alors que la partition sur le champ
conjoint le voit. Suite S222 choisie pour convertir la ligne en livrable J1, pas par proximité.

**Non fait et suite S222, file J1/W.** Scène à deux et trois sillages dans un même journal,
proches puis éloignés : enveloppe directionnelle conjointe (terme actuel du budget) contre borne
locale conjointe (ordre deux et spectrale) et maximum de référence, à budget d'évaluations égal.
Dire combien de π/7 la borne conjointe rendrait, et ce qu'il faudrait pour les impacts, qui restent
additionnés. Si la scène n'apporte rien, revenir à la file (cadence complète de l'hôte, V-noyau). A261,
coût de passe (table par champ, code machine) et A258 restent nommés, sans ordre imposé.

**Rituel.** Suivis A259 (levée à 4096 feuilles) et A260 (traitée, mécanisme corrigé) ; **A261**
sévérité 2 ; **L300** (code machine), **L301** (compter n'est pas peser). File active entière
relue : A244/S200-1 et S199-2 à J2, V-noyau au plus tard J2 ; B4 forces/perception, A216, A241,
A213, B2/coupure, bathymétrie, multiplateforme, bancs et réunions gardent leurs déclencheurs.
I-03 (échantillons inchangés), I-06 (aucune allocation, par inspection), I-14 (seuils `½, 1, 2`
nommés comme choix de famille, sans calibration), I-18 (aucune admission élargie) relus. Maillons
0 : W avancée dans src et ADR-137 actée. 137 ADR, 261 angles, 301 leçons, 18 invariants, 6 SPEC,
23 cas. Jeton libéré ; copies avancées après le commit de clôture.
## S222 — 2026-09-13 — La part somme d'A254 change de côté ; rien n'est migré

**Entrée.** « Reprends le projet », conversation neuve — **aucune mémoire de S217 à S221**, tout
relu depuis `AGENTS.md`, `REPRISE.md` §4 et les ADR 135/136/137. Ligne `Session suivante` de S221 :
A254 part somme. Maillons 0 à l'amorce. Claude Code, Opus 5 ; master et trois copies à c33953a.
Plan 86d4ddd, P2+P3 8d4747d, P4 60e7e4d, P5 8dda318, P6 df69a4a, P7 12e3db4.

**Décision structurante : aucune, et c'est la décision.** Aucun ADR, aucune migration d'admission.
La borne locale partitionnée rend exactement ce que l'enveloppe perd, et son prix est une **loi
d'échelle** qui interdit l'usage par image. Le terme de pression n'est plus le goulot. Le goulot a
changé de côté et il est nommé : la somme **spatiale** sur les impacts (**A262**).

**Chiffres qui orientent.**
- **Témoin de composition partagée** : `modes = 4096` pour une, deux et trois sources dans un même
  journal. Les amplitudes modales s'additionnent en complexe ; la préparation, elle, est linéaire
  (6,3 / 13,4 / 18,9 ms).
- **L'enveloppe est sous-additive** : trois sillages coûtent **1,67** fois un seul (proches),
  **1,44** (éloignés). La prédiction écrite avant mesure — « ≤ 1,6 à trois » — tient de justesse
  pour les éloignés et est dépassée de 4 % pour les proches.
- **Mais elle pénalise la séparation** : maximum réel **constant** à 0,0703 / 0,0704 / 0,0705 quand
  les sources s'éloignent, enveloppe +44 %, pessimisme de 1,64 à **2,35**. C'est A261 sur une scène.
- **La borne partitionnée rend tout** : borne/maximum **1,0053 à 1,0099** à 32 767 évaluations,
  gain jusqu'à **2,32** — mais **24 à 25 s**.
- **Et il n'y a pas de raccourci local** : au-delà d'un mètre de demi-côté la borne locale **vaut
  l'enveloppe globale** (0,9977), parce que tous les modes retombent alors dans la classe non
  résolue ; et un appel coûte **640–700 µs**, `O(N)` quelle que soit la taille. Hypothèse de départ
  contredite sur ses deux moitiés. **L303.**
- **Admission** : trois sillages et **huit** impacts passent (neuf si éloignés), contre une scène
  refusée en S214. La borne en rendrait deux à trois de plus, à un impact près du maximum réel.
- **Domaine de cette conclusion, et il la corrige** : un impact **neuf** vaut **47,4 %** de π/7, donc
  sous deux secondes **un seul** passe — et la borne n'y change rien, le majorant de naissance étant
  exactement atteint (S215). Le goulot n'est pas la pression.

**Ce qui n'a pas été fait.** Aucune ligne de `code/*/src` : la session mesure et décide de ne rien
changer. **A262 ouverte** et non traitée — c'est la suite. A261 reste ouverte, désormais chiffrée
sur une scène. A258, le coût de passe, la cadence complète de l'hôte, la loi GPU, J2/δ et V-noyau
conservés. Aucun usage hors image de la borne locale n'a été instruit, alors qu'il reste ouvert.

**Suite S223.** File J1/W : **A262**, la somme spatiale sur les impacts. La géométrie y est
favorable — support compact et déclaré, pente décroissante depuis `r = 0,2062 λ` (ADR-094) — donc
deux disques disjoints ne peuvent pas atteindre leur maximum au même point, et **cela se démontre
au lieu de se mesurer**. Chercher l'inégalité, comme ADR-134 l'a fait pour les directions, et non
une table. Si elle ne vient pas, revenir à la file (cadence complète de l'hôte, V-noyau).

**Rituel.** A262 (sévérité 2) ; suivis A254 (change de côté) et A261 (chiffré, plafonne tout) ;
L302, L303 ; file plurielle relue ; feuille de route J1/J1-bis ; index, README, REPRISE §4, file
active et jeton. Invariants relus que la mesure touche — **I-18** (ce qui est comparé à `max_slope`
est une pente réelle : rien n'a bougé, aucun terme n'a été substitué) ; I-06 (aucune allocation
ajoutée ; la partition emprunte le pool de l'appelant) ; I-14 (aucun nombre nouveau : rien n'a été
calibré). Aucun devenu faux, aucun amendé, aucun ADR réécrit ni acté.
**Maillons : 1** — ni code d'exécution dans `code/*/src`, ni décision actée. Le compteur monte
honnêtement ; la suite nomme **A262**, ligne née de la mesure mais rattachée à A254, qui est une
ligne de la file active. À `Maillons ≥ 2`, la suivante devra choisir dans la file.
370 tests release réussis, 5 ignorés — identique à S221. 137 ADR, 262 angles, 303 leçons,
18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S223 — 2026-09-13 — Le budget de pente tient compte de la position relative (ADR-138) ; A262 traitée

**Entrée.** « Continue avec S223 », même conversation que S222. Ligne `Session suivante` : A262,
chercher une **inégalité**, pas une table. Maillons **1** à l'amorce. Claude Code, Opus 5 ; master
et trois copies à 416daa3. Plan 900a6f7, P2 765ca33, P3 ae600a3, P4+P5 91145a2, P6 7a84e08,
P7 4067160.

**Décision structurante. ADR-138 actée et câblée** : `RadialImpact::slope_max_beyond(t, r)` majore
la pente sur la **couronne**, et `mixed::slope_floor_joint` en tire un plancher conscient de la
position relative, par inégalité triangulaire et balayage à huit intervalles. `slope_floor` aiguille
dès deux champs ; `sample_world_batch` calcule son budget **une fois** par `slope_floor`, sans quoi
l'annonce et le refus liraient deux quantités et la garantie d'ADR-128 tomberait. `slope_max()`,
`slope_max_at()`, `Steepness` et `steepness` publiée sont inchangés.

**Chiffres qui orientent.**
- **La prédiction est contredite dès P2, et pas là où je l'attendais** : j'annonçais un dépassement
  de la borne par l'approximation d'Hermite ; c'est **l'inégalité supposée qui est fausse**.
  `|J_ν(x)| ≤ √(2/πx)` vaut pour `ν = 1/2` (égalité) et **pas pour `ν = 1`** : dépassement de
  **3,4 %** en `x = 2,166`, plus 44 ppm dans le régime asymptotique. **L304.**
- Constante retenue : `sup |J₁(x)|·√x = 0,825031` sur `bessel` exécutée ; palier 0,5818650 dépassé
  de **0,36 ppm** ; garde `1e-4` = 275 fois ce dépassement.
- **Cela éclaire `SLOPE_L1_RATIO = 1,795071`** : c'est `1/0,5819 × 1,045`, **le pic de `J₁`** que la
  mesure de S141 retrouvait sans le nommer. **A263.**
- **Réception de la couronne** : quatre λ, six instants, 25 couronnes × 1 501 rayons — jamais
  dépassée ; égalité au bit à `radius = 0` ; resserrement > 3 à la naissance sur `r ≥ 15,5 λ`.
- **Gain conjoint** : **1,0000 exactement** à séparation nulle ; 1,76 (deux impacts, 50 m, âge 0) ;
  **2,37** (trois, 50 m) ; 2,57 (trois, 90 m). Maximal **à la naissance**, là où ADR-133 ne donne
  rien.
- **Admission** : trois impacts frais à 50 m passent de **142,1 % de π/7, refusés**, à **60,0 %,
  admis**. C'est A262 dans ses propres termes.
- **Coût** : huit intervalles rendent **96,5 %** du gain de soixante-quatre pour **11 %** du coût —
  13,3 µs, 0,7 % du budget d'image. À comparer aux **25 s** de la borne de pression en S222 pour un
  gain équivalent : six ordres de grandeur, parce qu'un champ radial décroît et qu'une somme de
  modules non. **L305.**
- **Zéro attente de test modifiée** ; 371 réussis (273+4+1+93), 5 ignorés. Hôte inchangé au bit
  (la scène J1 ne porte qu'un impact, l'aiguillage n'y joue pas).

**Ce qui n'a pas été fait.** L'**ancrage** du balayage n'est pas instruit : le champ au plus grand
majorant global est un choix, pas un optimum, et rien ne dit lequel resserre le plus. Le cas de
**plus de trois champs** n'est pas mesuré. L'usage de la borne hors admission n'est pas ouvert.
A261, A258, le coût de passe, la cadence complète de l'hôte, la loi GPU, J2/δ et V-noyau conservés.

**Suite S224.** Les deux termes du budget ont désormais des limites de **nature différente** :
l'impact est borné par une inégalité à 13 µs, la pression par une loi d'échelle à 25 s (A261). Le
budget de pente n'est donc plus le goulot d'une scène J1. **Revenir à la file** : cadence complète
de l'hôte, puis **V-noyau**, qu'ADR-127 demande au plus tard avec J2 et qui n'a **jamais** été
commencé — 0 module, 223 sessions. Si l'hôte passe avant, le dire et ne pas laisser V glisser.

**Rituel.** A263 (sévérité 3) ; suivi A262 (traitée) ; L304, L305 ; file plurielle relue ; feuille
de route J1/J1-bis ; index, README, REPRISE §4, file active et jeton. Invariants relus que la
décision cite — **I-18** (ce qui est comparé à `max_slope` est une pente réelle : la couronne majore
la pente réelle, pas une borne L1, et le minimum avec `slope_max_at` conserve la conversion) ;
**I-06** (aucune allocation : le balayage est une boucle sur deux accumulateurs scalaires) ;
**I-14** (deux constantes neuves, toutes deux avec leur banc) ; I-04 et I-08 inchangés. Aucun devenu
faux, aucun amendé, aucun ADR réécrit. **Compteur 0** : W avancée dans `code/water-core/src`
(`radial_impact`, `mixed_water`) **et** ADR-138 actée. 138 ADR, 263 angles, 305 leçons,
18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S224 — 2026-09-13 — La couche V existe : premier module, C12 reçu à 0,08 %

**Entrée.** « Continue avec S224 », même conversation. Ligne `Session suivante` de S223 : revenir à
la file, deux lignes possibles — cadence complète de l'hôte, ou V-noyau. Maillons 0. Claude Code,
Opus 5 ; master et trois copies à 71555d0. Plan 9109d54, P2 e3fc84d, P3+P4+P5 2f64748, P6 17fa8be,
P7 c2a7a77.

**Décision structurante : aucun ADR — ADR-010 est actée depuis S01 et complète. Cette session la
construit.** `code/water-core/src/hydro_network.rs` : nœuds en millilitres entiers, ouvertures à
deux lois (orifice de Torricelli, déversoir en `H^{3/2}`), pas fixe de 100 ms, quantification à
report de reste, normalisation par arrondi cumulatif, limiteur d'arrivée. **`outils/velocite.sh`
affiche désormais `V modules=1`** au lieu de `jamais`.

**Le choix de la ligne, et il n'était pas le plus confortable.** La cadence de l'hôte était
adjacente à tout ce que S220–S223 venaient de faire ; V avait **zéro module en 223 sessions** alors
qu'ADR-127 la rend obligatoire et que la feuille de route écrit « rien ne l'empêche de commencer ».
C'est exactement le mécanisme que BILAN-VELOCITE-S198 a mesuré — 33 sessions sur 38 prenant le
reliquat de la précédente — et j'avais écrit en S223 « ne pas laisser V glisser d'une session de
plus ». La cadence est **reportée explicitement**, pas oubliée, et la ligne de suite la reprend.

**Chiffres qui orientent.**
- **C12 : 727,4 s contre 728 s de référence analytique — écart 0,0824 %**, pour un cas qui exige
  ±3 %. Masse conservée exactement, volumes dans leurs bornes à chaque pas.
- **Deux défauts trouvés en construisant.** L'interpolation de hauteur **tronquait** : à un
  millilitre dans un réservoir d'un mètre carré la hauteur vaut un micromètre, et
  `999999/1000000 = 0` — charge nulle, vidange arrêtée. Et la **normalisation en nanolitres**,
  faite avant quantification, donnait à chaque arête une part sous le millilitre qui s'arrondissait
  à zéro : un nœud de 2 ml avec trois fuites les gardait indéfiniment. Normalisation passée après
  quantification, **en millilitres, par arrondi cumulatif**. **L306, L307.**
- **Report de reste : prédiction confirmée avec son chiffre.** Sans lui, la vidange **s'arrête à
  13 ml** — exactement le seuil dérivé (`Q·dt < 1 ml` quand `h < 13,3 µm`). La moitié de la
  prédiction que P2 avait déjà corrigée reste fausse : la masse est conservée par construction.
- **Déversoir séparé de l'orifice par son exposant** : `Q(2 m)/Q(1 m)` vaut **2,8284** contre
  `2^{3/2} = 2,8284`, et 1,4151 contre `√2` pour l'orifice.
- **Gauss-Seidel mesuré plutôt que supposé.** ADR-010 §4 annonce « 2 à 4 itérations suffisent » ; ce
  module n'en fait **aucune**. Chaîne de trois contenants, 60 s, 100 ms contre **1 ms** : écart
  maximal **0,0058 % de la capacité**. Un passage explicite suffit à 10 Hz *sur cette
  configuration* — pas une preuve générale.
- Neuf tests neufs ; suite complète **379 réussis, 5 ignorés**, aucun avertissement neuf.

**Ce qui n'a pas été fait.** **A264 ouverte** : le plancher de vidange croît avec la surface du
contenant — 1 ml pour 1 m², **10 L pour un hectare** —, et rien ne l'annonce. Non construits, et
tous présents dans ADR-010 : `liquid_id` (A17), `sky_exposure`, `absorb_rate`, vannes, pompes,
réseau fermé sous pression (reporté en v2 par l'ADR elle-même), et surtout la **surface libre en
référentiel accéléré** — le module prend `g_eff` en **module**, pas en direction, ce qui suffit à un
vaisseau immobile et pas à un vaisseau qui accélère. Aucun état répliqué ni restauré (ADR-022 §5.1),
donc la branche V de C19 reste hors d'atteinte. La cadence de l'hôte est reportée.

**Suite S225.** **Cadence complète de l'hôte** — travail nécessaire de J1 (ADR-131 D6), nommé depuis
S213, jamais mesuré, et reporté explicitement par S224 : la promesse se tient. **Et la brique
suivante de V est nommée dès maintenant pour qu'elle ne refroidisse pas** : direction de `g_eff`
(surface libre en référentiel accéléré, ADR-010 §2, spécifiée et absente) puis état répliqué et
restauré (ADR-022 §5.1), qui ouvre la branche V de C19 et C21. **Ne pas laisser passer plus d'une
session sans y revenir.**

**Rituel.** A264 (sévérité 2) ; suivi A17 (visible dans du code désormais) ; L306, L307 ; file
plurielle relue ; feuille de route V-noyau et J1 ; index, README, REPRISE §4, file active et jeton.
Invariants relus, et ils ont **dicté la forme** plutôt que d'être relus après : **I-03** (ordre du
tableau, arrondi cumulatif sans reste stocké, deux exécutions identiques au bit), **I-10** (entier,
10 Hz), **I-06** (aucune allocation, tout vient de l'appelant), **I-07** (`g_eff` injectée — en
module seulement, et c'est dit). Aucun devenu faux, aucun amendé, aucun ADR réécrit ni acté.
**Compteur 0** : V **et** W avancées dans `code/water-core/src`. 138 ADR, 264 angles, 307 leçons,
18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S225 — 2026-09-13 — La cadence complète de l'hôte, mesurée bout en bout

**Entrée.** « Continue avec S225 », même conversation. Ligne `Session suivante` de S224 : cadence
complète de l'hôte, reportée explicitement pour ouvrir V. Maillons 0. Claude Code, Opus 5 ; master
et trois copies à 59116c8. Plan 917f0dd, P2+P3+P4 421fb85, P5 eef0a99, P6 850053e, P6-bis 4efc922.

**Décision structurante : aucune.** La session mesure ; le verdict de coût qu'elle confirme est
celui d'ADR-125, et ADR-131 D1 lui garde son statut — il qualifie l'implémentation mesurée.

**Chiffres qui orientent.**
- **Cadence : 5,0450 ms d'intervalle, soit 198,2 Hz** à 960 × 540, fenêtre ouverte, `AutoNoVsync`.
  Quatre passages : écart **2,6 %** sur l'intervalle, **0,7 %** sur le GPU d'eau — bien plus stable
  que les 20 à 40 % de S213, parce que la charge est **soutenue** et non en rafales.
- **Les exclusions de S211–S224 sont chiffrées et ne cachaient rien** : la passe d'eau vaut
  **97,95 à 98,06 %** de la trame GPU ; ciel, cibles et résolutions font **0,087 ms**.
- **Fait neuf : le CPU d'une trame est à moitié de l'attente.** Acquisition d'image **2,22 ms sur
  4,32**, soit 51 % — contre-pression du GPU, pas travail. Le travail réel fait 2,10 ms (sillage
  1,25, transfert 0,21, reste 0,64). Aucun banc hors écran ne pouvait le voir. **L308.**
- **Et une conclusion corrigée dans la même session.** J'avais écrit « la trame est bornée par le
  GPU, une seconde gagnée sur le CPU serait invisible ». La caméra **balayée** le réfute : le GPU
  d'eau tombe à **2,85 ms** (max 4,23) et la cadence ne bouge presque pas — 205 Hz contre 198.
  En balayage `CPU + GPU` ≈ l'intervalle, donc **presque aucun recouvrement** ; à caméra fixe il y
  en a. **A265**, gravité 3.
- **La pose héritée de S201 est proche du pire cas.** 4,16 ms à la pose fixe contre 2,85 en médiane
  de balayage : la valeur publiée depuis S212 était à peu près le **maximum**. Le facteur au budget
  de 2 ms passe de **2,08 ×** à **1,42 ×** en médiane, 2,12 × au pire. **L309.**
- **ADR-125** : l'hôte tient 60 Hz trois fois, mais ce n'est pas la question — l'eau prend 4,16 ms
  d'une trame de 16,67 au lieu de 2, et la scène entière 5,05 : il resterait **11,6 ms** pour le
  reste au lieu de 14,67.
- Contrôles : `VERIFY` inchangé, `BENCH` inchangé, `--smoke` code 0 ; suite `code/` **379 réussis,
  5 ignorés** — la bibliothèque n'a pas bougé.

**Ce qui n'a pas été fait.** Aucune ligne de `code/*/src` : tout est dans `viewer/`. Un autre format
que 960 × 540, l'interaction manuelle, les angles rasants soutenus et les allocations de la pile
graphique (I-06) restent non mesurés. **A265** ouverte et non instruite. Les deux briques suivantes
de V — direction de `g_eff`, état répliqué — n'ont pas été touchées, et la consigne de S224 était de
ne pas laisser passer plus d'une session.

**Suite S226.** **V, sans faute** : la consigne de S224 arrive à échéance. Direction de `g_eff` —
surface libre en référentiel accéléré, ADR-010 §2, spécifiée et absente, le module ne prenant
aujourd'hui que le **module** de `g_eff` — puis état répliqué et restauré (ADR-022 §5.1), qui ouvre
la branche V de C19 et C21. La suite GPU de J1-bis (espace, LOD, visibilité, mutualisation) reste
nommée, et A265 est à instruire **avant** toute optimisation CPU.

**Rituel.** A265 (sévérité 3) ; suivi A247 (exclusions chiffrées, pose de mesure requalifiée) ;
L308, L309 ; file plurielle relue ; feuille de route J1 et J1-bis ; index, README, REPRISE §4, file
active et jeton. Invariants relus que la mesure touche — **I-06** (les allocations de la pile
graphique restent non reçues, et la cadence ne les mesure pas davantage), I-03 et I-14 sans objet
ici : aucune constante posée, aucun bit publié. Aucun devenu faux, aucun amendé, aucun ADR réécrit
ni acté. **Maillons : 1** — ni code d'exécution dans `code/*/src`, ni décision actée.
138 ADR, 265 angles, 309 leçons, 18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S226 — 2026-09-13 — La surface libre en référentiel accéléré, et une incohérence d'ADR-010

**Entrée.** « Continue avec S226 », même conversation. Ligne `Session suivante` de S225 : **V, sans
faute** — la consigne de S224 arrivait à échéance. Maillons **1** à l'amorce. Claude Code, Opus 5 ;
master et trois copies à e75f6aa. Plan 6aae62c, P2 355e10a, P3 f42cbdf, P4 7e5144a, P5 fe81735.

**Décision structurante : aucun ADR.** ADR-010 §2 écrivait déjà ce qu'il fallait ; la session le
construit — et met au jour que le paragraphe se contredit lui-même (**A266**).

**Chiffres qui orientent.**
- **Le module violait I-07**, et pas par inadvertance de lecture : il recevait `g_eff` en **module**,
  donc l'axe était en dur, ce que l'invariant appelle « un défaut bloquant ». **C12 passait quand
  même, à 0,08 %** — un réservoir posé à plat ne distingue pas les deux. **L310.**
- **Réduction exacte** : sous `g_eff = [0, 0, −9,81]`, tous les nombres de S224 sont **identiques**
  — C12 à 727,4 s, l'arrêt sans report à 13 ml, la chaîne à [532 351, 300 688, 166 961], les
  exposants 2,8284 et 1,4151. C'est le garde-fou qui sépare une généralisation d'une réécriture.
- **La phrase d'ADR-010 §2 est éprouvée telle qu'elle est écrite** : hublot latéral à 1,2 m,
  surface au repos à 1 m — **0 ml** sous gravité verticale, **1 829 ml** sous 0,3 g latéral.
- **C16, part V** : inclinaison **déduite du comportement** par dichotomie sur la cote de mise en
  débit, à deux abscisses — **16,6990°** contre 16,6992° attendus, soit **0,0002°** pour un degré
  exigé.
- **Erreur de ma part, dans le test et non dans le code** : le premier attendu posait `atan(−a/g)`
  et se trompait de signe. L'eau s'accumule du côté où le « bas » penche, donc la surface y monte ;
  la pente vaut `−g_x/g_z`. Corrigé, avec la raison écrite dans le test.
- **Prédiction contredite pour le prisme, confirmée pour la cale.** J'annonçais la table de forme
  fausse dès l'inclinaison. Mesuré à 0,3 g : **prisme exact à 0,0000 %** au milieu de sa course
  (1 à 4 % aux extrêmes), **coque en V fausse de 9,89 % partout**. Or la cale est le cas qui
  justifie la table. **A266, gravité 1**, et **L311**.

**Ce qui n'a pas été fait.** A266 n'est pas tranchée — trois voies nommées, aucune choisie, et c'est
une décision de conception. La brique suivante de V, l'**état répliqué et restauré** (ADR-022 §5.1),
n'est pas commencée : C19-V et C21 restent hors d'atteinte. `liquid_id` (A17), `sky_exposure`,
`absorb_rate`, vannes, pompes, réseau fermé sous pression et **A264** restent tous dus. Rien de
neuf sur A265, A261, A258, A263 ni sur la loi GPU.

**Suite S227.** **A266 d'abord** : elle est de gravité 1, elle porte sur un **volume** — donc sur une
conséquence de jeu (I-10) —, et elle bloque tout usage de V sur un contenant non prismatique,
c'est-à-dire sur le cas nominal. Trancher entre table à deux entrées, correction analytique pour les
sections convexes, et restriction déclarée aux prismes ; l'ADR qui en sortira remplacera la
disposition d'ADR-010 §2, sans la réécrire. **Puis** l'état répliqué. A265 reste à instruire avant
toute optimisation CPU.

**Rituel.** A266 (gravité 1) ; suivi I-07 (violation levée) ; L310, L311 ; file plurielle relue ;
feuille de route V-noyau ; index, README, REPRISE §4, file active et jeton. Invariants relus —
**I-07**, levé et vérifié par le comportement et non par la lecture ; **I-03** (projection en `f64`
depuis des différences entières, IEEE strict, réduction au bit sous gravité verticale) ; **I-10**
(l'état reste entier) ; **I-06** (aucune allocation ajoutée). Aucun devenu faux ; **I-07 cesse d'être
violé**, ce qui est un changement d'état et non un amendement. Aucun ADR réécrit ni acté.
**Compteur 0** : V avancée dans `code/water-core/src`. 138 ADR, 266 angles, 311 leçons,
18 invariants, 6 SPEC, 23 cas. Jeton libre, copies avancées.

## S227 — 2026-09-13 — Audit global, reprise allégée et intégrité du réseau V

**Entrée.** Audit demandé par l'utilisateur : intentions, dérives, gestion, documentation, zones
d'ombre et code. Codex, GPT-6 ; master et trois copies propres à dfd1507, lignée B archivée,
aucun distant. L'audit prend explicitement la place de la suite automatique A266 de S226.

**Diagnostic et changements.** L'ambition des sources reste entière ; la livraison privilégie
B/W et leurs validations, tandis que δ général, adaptation et inondations restent à construire.
[BILAN-GLOBAL-S227](../../docs/registres/BILAN-GLOBAL-S227.md) porte les preuves et la suite priorisée.
Les trois points d'entrée passent de **6 161 à 384 lignes**, sans supprimer ADR ni sources.
File et trajectoire remplacent leurs états périmés. La méthode exige un effet aval et un arrêt
du lot ; elle n'impose plus de trouver une impasse. L'indicateur Git distingue cœur, harnais,
exemples et afficheur, sans prétendre mesurer la productivité. L'histoire reste dans Git/journal.

**Capacité reçue.** Le pas réel de V peut recevoir plusieurs arrivées sans dépasser la capacité
commune : le cas **3 ml dans 1 ml**, vu échouer, est corrigé par limite collective. Les différences
de coordonnées extrêmes ne paniquent plus. Quatre régressions couvrent confluences, entrées/sorties,
rejet extérieur, coordonnées et refus tardif atomique. C'est une correction d'intégrité consommée
par `hydro_network::step`, pas une réception des inondations complètes. **Maillons : 0** selon le
critère révisé ; le nettoyage documentaire seul n'aurait pas remis le compteur à zéro.

**Preuves.** Suite complète `code/`, release hors réseau : **387 réussis, 6 ignorés**, contre
383/5 à l'entrée. Quatre tests de l'indicateur passent ; navigation active vérifiée, deux ancres
réparées. C12 et les témoins V antérieurs restent inchangés. **A266 reste en échec connu** : sous
gravité inclinée, un hublot central à 1,01 m fuit de **506 ml**, au lieu de zéro. Le nouveau test
ignoré conserve l'attendu correct ; la convention distance normale / cote centrale est explicitée
par notes datées à ADR-010, au reçu S226 et à L311. A267/A268 corrigées ; L312 ajoutée.

**Limites et suite.** Aucun nouvel ADR ni invariant modifié, aucune valeur physique abaissée.
I-03, I-06, I-07, I-08, I-10 et I-14 relus : état entier, ordre fixé, pas d'allocation ajoutée par
inspection ; le multiplateforme, le budget et la géométrie générale ne sont pas reçus par ces
tests. Pas de nouvelle réception GPU ou δ. **S228 : A266**, relation volume/plan orienté correcte
et première version construite ; puis état V restaurable. Budget/précision δ et J1-bis demeurent
des lots indépendants ; les raffinements de bornes n'en sont plus un préalable générique.
A211/A243 restent à éprouver par les livraisons suivantes. File plurielle relue, feuille de route,
index et passation actualisés. Jeton libéré ; après ce commit, les trois copies propres doivent
être avancées sur master selon l'amorce, sans suppression de copie dont l'inactivité est incertaine.

## S228 — 2026-09-13 — Le volume du contenant détermine sa surface inclinée

**Entrée.** « Continue », suite A266 de S227. Codex, GPT-6 ; master et trois copies propres à
a86bd43, lignée B archivée, maillons 0. Plan 0b868cb ; décision 2fbead9 ; géométrie ac39870 ;
consommation par V 6e64d99 ; réception 2ab4c65.

**Décision et capacité reçue.** ADR-139 remplace la table horizontale universelle d'ADR-010 par
une relation volume/plan sur partition tétraédrique. La surface inclinée d'un prisme, d'une cale
et d'une forme non convexe préserve désormais leur volume dans les cas reçus ; le pas réel
consomme ce plan pour source et receveur. **A266 active passe : 0 ml au lieu de 506** au hublot
central à 1,01 m. Les tables historiques refusent les directions hors +Z. Les origines sont
fixes dans la géométrie, pas déplacées avec la gravité. I-08 reçoit une exception explicite
pour les intermédiaires f64 de V ; état/échanges restent entiers, I-03 demeure obligatoire.

**Preuves et portée.** [VOLUME-ORIENTE-S228](../../docs/validation/VOLUME-ORIENTE-S228.md) publie les
oracles indépendants, domaines et coûts. **398 tests réussis, 5 ignorés**, aucun nouvel ignoré ;
compteur positif puis zéro allocation sur les plans/pas/refus. Refus tardifs sans mutation.
C12 à 727,4 s et chaîne S224 inchangés ; hublot latéral **1 736 ml** contre 1 829 avant correction,
pente C16 conservée. Anneau de 64 nœuds/64 arêtes, six tétraèdres partagés par forme : **812,6 µs
médiane**, 1 366 µs p95, 1 807,4 µs maximum observé ; aucune garantie de budget I-05.

**Non-fait.** État restaurable, cuisson d'assets réels/courbes, budget et seconde cible non reçus.
**A269** ouverte : à `2^53+1 ml` dans un cube de 3 km, la précision f64 ne suffit plus ; refus
nommé, demande entière préservée. La précision des formes/tailles supplémentaires se qualifie
lorsqu'un consommateur en dépend. Ni la grande échelle ni les formes générales ne sont retirées.
A264 reste sur le chemin tabulé ; aucune réception de sa vidange n'est déduite du nouveau plan.

**Suite S229 et comparaison des priorités.** La restauration V complète une capacité durable
attendue depuis S224 et ouvre C19-V ; elle est retenue pour un lot borné avant le budget δ de J2.
Même en regroupant les correctifs V de l'audit S227 avec S228, cette comparaison justifie la
suite : persister/restaurer le graphe et ses restes, avec identité/version des géométries, sans
nouvelle campagne de géométrie. Après réception, comparer la suite V à **J2 budget/précision**.
Les obligations J1-bis, B2, bathymétrie et multiplateforme gardent leur place dans la file.

**Rituel.** File plurielle relue, trajectoire et index actualisés ; A266 corrigée, A269 consignée,
aucune nouvelle leçon forcée. L'indicateur repère le nouveau fichier de géométrie V ; quatre
contrôles de l'outil passent. **Maillons 0** : plan correct consommé par le pas, avec défaut
reproduit puis levé. Jeton libéré ; après le commit du rituel, avance rapide des trois copies
propres, sans suppression des copies dont l'inactivité n'est pas prouvée.

*Clôture le 2026-09-14 après interruption en P6* : diff relu et complété ; aucun travail
concurrent constaté. Code Rust de P5 inchangé, reçu 398/5 conservé ; navigation active vérifiée.

## S229 — 2026-09-14 — Reprendre V sans perdre ses restes de débit

**Entrée.** « Continue », suite de S228. Codex, GPT-6 ; master propre et trois copies à
c2e9a56, jeton libre, maillons 0. Plan 36bccc8 ; contrat 3e90405 ; construction b172258 ;
réception 2db39a7. Aucune autre session ni branche avancée constatée.

**Capacité reçue.** Une instance neuve du noyau V reprend depuis une charge utile versionnée,
puis consomme le contexte restauré dans le pas réel. ADR-140 précise ADR-022 : écarts aux
volumes d'auteur **et restes des ouvertures**, base immuable empruntée, identité/révision et
empreinte des géométries. WVST V1 ne recopie ni données cuites ni état δ. Refus atomiques,
reconstruction depuis la base plutôt que conservation des anciennes valeurs des pools.

**Preuves.** [RESTAURATION-V-S229](../../docs/validation/RESTAURATION-V-S229.md) : 1 000 pas tabulés
avec témoin sans reste divergent ; 200 pas orientés sous cinq gravités, trois restaurations,
volumes/transferts/restes/plans/octets identiques, 3 492 ml rejetés, bilan entier exact. Capture
initiale de 148 octets. Compteur positif puis zéro allocation pour capture, restauration,
continuation et refus testés. Troncatures, corruptions et configurations modifiées refusées.
Suite finale release hors réseau : **404 réussis, 5 ignorés**, six nouveaux tests ; quatre
contrôles Python et navigation active reçus. Aucun avertissement neuf. L'appel standard trop
récent a été remplacé avant la dernière suite ; Rust 1.75 déclaré conservé, pas exécuté ici.

**Portée/non-fait.** C19-V local du noyau, pas C19 B/W/V complet ni réception multiplateforme.
Transport/autorité, stockage durable, migrations et nœuds dynamiques sont portés dans la file
avec leurs déclencheurs d'intégration. A17/A269 et budget V inchangés. FNV détecte les erreurs
accidentelles, pas une attaque ni toute collision ; aucune fausse garantie d'authentification.
Aucun nouvel angle ni leçon forcés : limites nommées et suivies dans les porteurs existants.

**Priorité suivante comparée.** Le lot borné de restauration attendu depuis S224 est reçu ;
prolonger le codec ou la géométrie ne débloquerait pas un consommateur actuellement construit.
**S230 : J2 / S200-1 / A244**, arrêt sous budget injecté, état réutilisable et dégradation
explicite, puis qualification de précision. δ n'a toujours pas ce contrat I-05 ; cette capacité
prime maintenant sur les extensions V et raffinements W. J1-bis, faces coupées, B2, bathymétrie,
forces/perception et seconde cible conservent leurs déclencheurs dans la file entière relue.
Aucun arbitrage utilisateur requis ni réduction d'ambition.

**Rituel.** ADR complétés par notes datées seulement, index et trajectoire actualisés en
remplacement, file relue. Maillons **0** : persistance consommée par une continuation réelle
avec témoin discriminant. Jeton libre ; après ce commit, avance rapide des trois copies propres
sur master, sans suppression de copie dont l'inactivité n'est pas prouvée.

## S230 — 2026-09-14 — δ abandonne un pas sans publier un état partiel

**Entrée.** « Continue », priorité J2 après V restaurable. Codex, GPT-6 ; quatre copies à
d6ff35d, master propre, jeton libre. Plan 9cd109f, contrat b1d7e00, construction d34d59d,
réception 4c9f3e3. Aucune autre branche avancée constatée ; pas de nouvelle copie.

**Capacité reçue.** Le consommateur de `Volume::step_budgeted` peut imposer une enveloppe de
calcul, recevoir zéro temps avancé/dt entier restant et relancer depuis les champs précédents.
Huit phases contrôlées, tranches et appels de réduction bornés ; anciens buffers préalloués
échangés pour abandon O(1), aucune copie de domaine après expiration. Aucune sérialisation δ.
Les plafonds d'itérations et chemins de banc restent disponibles ; pas de décision de famille.

**Preuves.** [BUDGET-DELTA-S230](../../docs/validation/BUDGET-DELTA-S230.md) : 204 points d'expiration
sur état non nul, invariance bit à bit puis continuation identique. Domaines8×4 et32×16,
réductions ≤64cellules, plafonds500/1/0/500 ; erreurs temporelles/numériques et zéro allocation
avec témoin positif. **407 tests réussis, 5 ignorés** ; filtre physique 0x0ad3f695685ca27a inchangé.
Après ajout du contrôle de colonne vide, sept tests runtime release et filtre reçus ; aucune
modification numérique. Navigation active valide, avertissements préexistants uniquement.

**Limite mesurée conservée.** Deux passages de coût :64×32 sous2ms, médiane2,0010ms et
maximum15,8501ms au premier ; médiane2,0008ms et maximum2,0732ms au second. Cause du pic
inconnue, aucune attribution OS gratuite. Contrôle coopératif reçu, **garantie murale I-05
non reçue** ; appel d'hôte, tranche et suspension doivent être qualifiés. La mesure externe
inclut le retour ; le rapport interne s'arrête au dernier contrôle. Aucun dépassement effacé,
aucun budget utilisateur changé. Un bloc qui abandonne toujours n'avance pas.

**Suite comparée.** **S231 : pression f32 du candidat**, autre obstacle d'intégration A244/I-08
exécutable sans approfondir la micro-mesure du temps. Construire puis comparer au témoin f64,
refus et budget conservés ; une exception éventuelle exige un ADR technique explicite.
L'admission de charge viable et les marges de l'hôte restent dans la file pour l'intégration,
avec le profilage du pic avant toute attribution. Faces coupées, surface mobile/3D, J1-bis,
B2, bathymétrie, V/J4 et seconde cible gardent leurs déclencheurs. Une correction spatiale ne
remplace pas le contrat de précision ; aucune réduction d'ambition.

**Rituel.** File entière relue, trajectoire/état remplacés et index complété ; A244 actualisée,
aucun ADR ni leçon artificiels. Maillons 0 : capacité d'abandon consommée dans le pas réel,
preuves d'intégrité et de reprise, distinctes d'I-05 complet. Jeton libre ; après commit,
synchroniser les trois copies propres en avance rapide, sans suppression incertaine.

## S231 — 2026-09-14 — Pression δ en f32, convergence vérifiée

**Entrée.** « Continue », quatre copies propres à88b98fe, jeton libre. Codex, GPT-6.
Plan61bfb95, critères/témoin82c3ae8, construction262969c, réception9c8a006.

**Capacité reçue.** Le pas réel calcule pression, opérateur, projection et réductions en f32.
La conversion naïve révélait une fausse convergence (vrai résidu jusqu'à4,45e-6) ; contrôle
de b−Ap et correction CG conservent le seuil1e-6 et le plafond global. Refus des normes
sous-débordantes non nulles. Transport exact par callback hôte f64, pas accumulation f64.

**Preuves.** [PRESSION-F32-S231](../../docs/validation/PRESSION-F32-S231.md) : dix comparaisons
contre f64, continuation100pas, oracle indépendant aux quatre résolutions ; vitesse différente
d'au plus6,16e-6 relatif normalisé. Verdicts spatiaux conservés, nouveaux bits documentés.
24octets/cellule économisés, environ28% du stockage ; **aucun gain de vitesse reçu**.
Suite release hors réseau : **409 réussis, 5 ignorés**, dont les sept tests runtime
(budget/refus/reprise/zéro allocation). Deux nouvelles régressions, avertissements préexistants.

**Limites et suite comparée.** I-08 pression reçu sur ces domaines seulement ; API temps f32
encore à remplacer à l'intégration. I-05 complet, flux coupés, surface mobile, 3D et B3 non reçus.
Avant une troisième session J2, comparaison à la file : **S232, flux des faces coupées**
débloque la précision physique du candidat, là où prolonger les micro-mesures de pression
ne livre plus de capacité utile. J1-bis apporterait du rendu moins coûteux, mais ne corrigerait
pas ce défaut physique mesuré ; extensions V différables après restauration reçue.
B2, bathymétrie, seconde cible et autres déclencheurs conservés. Aucun arbitrage utilisateur.

**Rituel.** File entière relue, états remplacés, A244 et index actualisés ; invariants I-05/06/08
relus, aucun ADR réécrit ni leçon forcée. Maillons0 : pression f32 consommée dans le pas et
fausse convergence reproduite puis corrigée. Jeton libre ; après commit, avance rapide des
trois copies propres, sans suppression incertaine. Le dépassement mural de P2 est consigné
dans EN-COURS, sans le masquer comme conformité de procédure.

## S232 — 2026-09-14 — Mesurer le débit ouvert, conserver les triangles fluides

**Entrée.** « continue », quatre copies propres à4a48c74, jeton libre. Codex, GPT-6.
Plan1cdb4a4, diagnostic458d379, correction4f3eacd, réception181ec00.

**Résultat.** L'ordre≈0,90 hérité de S199 venait d'une somme des vitesses sur faces entières.
En pondérant par l'ouverture, ordre≈1,95 sans changement du solveur : aucune justification
pour reconstruire son opérateur sur ce seul chiffre. Un défaut distinct est reproduit :
SUB8 supprime un triangle fluide d'aire2,38e-4 tout en laissant ses faces ouvertes.
L'intégrale du profil linéaire remplace l'échantillonnage ; la cellule participe maintenant
au pas de pression et ses flux projetés s'équilibrent. Même contrôle sur son miroir.

**Preuves.** [FLUX-COUPES-S232](../../docs/validation/FLUX-COUPES-S232.md) : régression rouge
avant correction, aire indépendante et bilan de quatre flux, partitions géométriques,
douze tests unitaires δ. Filtre final1,947/1,957/1,966, hash0xc5ab1eadb094d058 ; critère1,8
et rejeu désormais assertés. **411 tests réussis, 5 ignorés**, dont sept runtime δ : budget,
refus, reprise et zéro allocation. Navigation active reçue ; aucun changement de stockage.

**Limites.** Le triplet est une auto-convergence d'une fonctionnelle, pas une référence
manufacturée ; la « marche » tanh est lisse. Ordre local, stabilité des petites cellules,
fond discontinu, surface mobile, 3D et scénarios B3 non reçus. Les bits de certains champs
changent quand des cellules redeviennent fluides. Aucun gain de coût, I-05 ou I-08 global
revendiqué. Aucun ADR modifié ; rectification datée du reçu historique S199.

**Suite comparée.** Après trois sessions J2, **S233 : construire une première évolution de
surface consommant les flux**, avec domaine et oracle déclarés et traitement conforme du temps.
C'est une capacité physique absente, contrairement à une nouvelle campagne sur le débit
maintenant reçu. J1-bis reste utile pour le coût visible, mais ne débloque aucun phénomène
volumétrique ; extensions V différables après restauration reçue. δ général, 3D, B2,
bathymétrie, seconde cible et autres travaux gardent leurs déclencheurs dans la file relue.
Pas de réduction d'ambition ni arbitrage utilisateur nécessaire.

**Rituel.** Trajectoire/file mises à jour en remplacement, index et A244 actualisés. Pas de
nouvelle règle de méthode : distinguer instrument et modèle était déjà prescrit et a suffi.
Maillons0 : correction d'intégrité reproduite, consommée et testée dans le pas réel.
Jeton libre ; après commit, avance rapide des trois copies propres, sans suppression incertaine.

## S233 — 2026-09-14 — La surface linéarisée évolue dans le pas δ

**Entrée.** « continue », quatre copies propres à46851ce, jeton libre. Codex, GPT-6.
Plan02a3f05, contrat02188ba, construction64815ab, réception76f2e02.

**Capacité reçue.** `step_surface_linear` boucle pression, flux ouverts et hauteur ; η
modifiée alimente le pas suivant. Modèle linéaire sans advection quadratique, géométrie fixe.
ADR-141 précise I-08 : durée/budget en microsecondes entières, coefficients dimensionnés
construits en f64 puis arrondis en f32 ; pas de temps f32 ni accumulateur caché sur ce chemin.
Compensation f32 des déplacements consommée par la pression ; sauvegarde atomique incluse.

**Preuves.** [SURFACE-LINEARISEE-S233](../../docs/validation/SURFACE-LINEARISEE-S233.md) : onde
stationnaire analytique sur1s, g9,81/1,62, trois résolutions et deux pas. Erreurs fines
0,0659%/0,0173%, dérive moyenne≤4,48e-8m ; raffinement temporel utile, pas d'ordre spatial
déduit d'une compensation d'erreurs. Repos exact et retour de signe terrestre reçus.
213 expirations, reprise en bits et zéro allocation avec témoin. **413 tests réussis,
5 ignorés** ; deux tests S233 renforcés sur refus puis rejoués seuls, code inchangé.
Filtre S232 inchangé, navigation active valide, avertissements préexistants seulement.

**Défauts discriminés.** L'arrêt dès une hausse isolée du vrai résidu refusait le pas307
à64/1ms ; le seuil reste1e-6, les corrections continuent sous plafond global. Sans reste
de hauteur, le raffinement lunaire empirait l'erreur jusqu'à0,597% ; compensation construite
et testée. Trois tableaux nx f32 ajoutés, +12nx octets. Aucun état δ sérialisé.

**Coût/limites.** Pas64×32 : médiane2,8638/max3,8652ms au premier passage,6,2104/8,9980ms
au second. Variation conservée, pas de cause attribuée ni I-05 reçu. Pas de géométrie
mobile, cavité, non-linéaire, 3D ou couplage B/W. Les anciennes API dt f32 restent suivies.

**Suite comparée.** Le lot évolutif borné est reçu ; après quatre sessions J2, prolonger
l'onde ou sa micro-mesure retarderait une autre capacité absente. **S234 : J1-bis, premier
LOD spatial intégré à l'hôte**, qualité/coutures et coût complet. J1 dispose déjà d'une scène
dont le rendu dépasse sa cible ; ce consommateur immédiat prime sur une campagne supplémentaire
de surface linéarisée. Le prochain lot physique δ reste géométrie mobile puis3D/cavité,
avec oracle propre ; V, B2, bathymétrie et seconde cible gardent leurs déclencheurs.
Aucune ambition réduite ni arbitrage utilisateur nécessaire.

**Rituel.** File entière relue, trajectoire remplacée, A244/index/invariant actualisés ;
nouvel ADR141, aucun ADR réécrit. Maillons0 : surface évolutive consommée par pression
et reçue contre référence indépendante. Jeton libre ; après commit, synchroniser les trois
copies propres en avance rapide, sans suppression incertaine.

## S234 — 2026-09-14 — La densité du maillage venait de B, le coût venait du sillage

**Entrée.** « Reprends le projet », master propre à `98430a1`, trois copies propres au même
commit, jeton libre. Claude Code, Opus 5. Plan `430ed8c`, charge `a53eef8`, grille `225bf14`,
GPU `a25a7ad`, intérieurs `c029af0`, coût `401ae8f`.

**Ce que la charge exigeait, calculé avant de construire.** Sous le critère d'interpolation
linéaire `½·M·R²` et la tolérance de 3 mm de l'image S201, la hessienne de **B** (0,349,
32 composantes) dicte la densité du maillage ; alléger le maillage retire ≤35 % en pose S212 et
rien en vue haute. Nyquist par sommet : ≈2 % du travail. Le coût, lui, vient du sillage
(4 096 modes par sommet). D'où la construction : **découpler la densité d'évaluation du sillage
de celle du maillage** (L313).

**Capacité reçue.** Grille locale du sillage cuite en compute, pas borné par
`h⁴/384·(2Σ|a|k⁴ + h/4·Σ|a|k⁵)` ≤ 3 mm (1,1875–1,5625 m, ≤9 701 nœuds), reconstruction Hermite
bicubique par sommet ; chemin direct conservé (`--no-lod`). Consommateur : l'hôte J1, fenêtre et
banc. **La passe d'eau de la scène J1 passe sous 2 ms** sur ce domaine.

**Preuves.** [LOD-SILLAGE-S234](../../docs/validation/LOD-SILLAGE-S234.md). Référence CPU testée
(pire/borne 0,118 au pas de la borne). Centres et milieux d'arêtes, six âges : grille − direct
GPU 0,19 à 0,68 mm, 0,07 à 0,26 de la borne ; grille − cœur identique à 1 µm ; saut à travers les
arêtes ≤6 µm. `--verify` vert sur les deux chemins (0,400 mm grille, 0,089 direct). Sur
secteur, 960×540 : témoin S233 **4,24 ms**, grille **0,426 ms** cuisson comprise (×10,0) ; recette
128×256 17,6 → 1,51 ms. Cadence 194 → **384 Hz** fixe, 197 → **398 Hz** balayée ; la pose ne
compte plus (0,43/0,46 ms). Neuf tests de l'hôte ; `code/` non modifié.

**Défauts et limites.** Le ciel partage la mise en page : groupe de la grille à poser (panique
wgpu corrigée). Script de campagne à paramètre `$args` : témoin lancé en fenêtre interactive,
≈20 min perdues, garde ajoutée. **Alimentation** : témoin 7,01 ms sur batterie contre 4,24 sur
secteur ; aucun en-tête S211–S225 ne la publiait (A270). Erreur de pente jusqu'à 1,14e-3 aux
sondes irrégulières (maximum Hermite vers t≈0,21), publiée sans seuil. Cuisson insensible à
+21–33 % de nœuds, non expliquée. Un pic d'acquisition de 30 ms en balayage. Trame désormais
bornée par le CPU (sillage 1,3 ms) : A265 partielle. Un sillage, un impact, une machine.

**Non fait.** LOD de maillage par rangées (mesuré, non construit) ; visibilité et retour visible
de la grille ; plusieurs sources ; transformée ; angles rasants, interaction, I-06 de la pile
graphique, seconde cible.

**Suite comparée.** La priorité 4 du bilan S227 (J1-bis) est servie pour moitié : LOD et coutures
reçus, **scène représentative et retour visible** non. S222 a admis trois sillages et huit
impacts ; la cuisson suit les modes et croîtra avec les sources. **S235 : J1-bis, scène
multi-sources dans l'hôte** — cuisson par source ou mutualisée, visibilité et retour dans le
champ, coût GPU et CPU du sillage. J2 garde son prochain lot (géométrie mobile, oracle propre) ;
V, B2, bathymétrie et seconde cible gardent leurs déclencheurs. Aucune ambition réduite ni
arbitrage utilisateur nécessaire.

**Rituel.** A270 ouverte, suivis A265/A247 ; L313 ; feuille de route (LOD spatial présent,
J1 remplacé), file active, index et README. Aucun ADR. Maillons 0 : passe d'eau J1 sous 2 ms,
consommée par la fenêtre, reçue contre le cœur et le témoin S233.

## S235 — 2026-09-14 — La scène représentative se dessine, et le budget la refuse sans raison physique

**Entrée.** « Continue », master propre à `f0159ca`, trois copies au même commit, jeton libre,
secteur. Claude Code, Opus 5. Plan `d760c93`, admission `e28aba7`, rendu `c64a5b8`, pente réelle
`bfbf3c7`, visibilité `d459c22`, coût `8524cf5`.

**Scène déclarée avant mesure.** Trois sillages d'un journal commun (espacement « éloignés » de
S222) et huit impacts de l'entrée S203 nés toutes les 4 s ; variante dense à 1 s.

**Capacités reçues.** (1) **Rendu multi-sources** : une table radiale pour les huit impacts, profils
à l'âge de chacun, tampon GPU des centres ; mono identique au bit à S234. `--multi --verify` :
≤0,368 mm contre le cœur sur 23 âges, grille ≤0,16 de sa borne, sauts ≤7 µm. (2) **Visibilité** :
emprise de la grille sur l'eau (contour aux sommets de bord, marge par arête) ; hors champ ni
préparation, ni cuisson, ni profil ; **retour dans le champ identique au bit** sur 31 images après
150 images cachées, fin de tronçon comprise. Consommateur : la fenêtre J1.

**Ce que la mesure a trouvé.** Le budget de pente du cœur **refuse la scène sur 49 instants sur
161** (pire 1,251 π/7) — et **tous par majorant seul** : pente réelle des perturbations ≤0,2154,
soit 48 % de π/7, sur 447 161 points raffinés à 2 cm ; plancher 2,1 à 8,2 fois la réelle. À chaque
naissance, le maximum réel est celui de l'impact neuf ; le plancher y ajoute l'enveloppe des trois
sillages et les majorants des impacts anciens. S222 (« huit impacts passent ») valait pour des
impacts âgés (L314). Premier essai de prédiction faux : impacts futurs inscrits dès 0 s, que
`slope_max_at` compte à leur maximum de naissance (ADR-133, voulu).

**Coût** ([SCENE-MULTI-S235](../../docs/validation/SCENE-MULTI-S235.md), secteur relevé à chaque
passage). GPU d'eau ≤0,47 ms dans toutes les poses, +0,01–0,03 ms pour huit impacts et trois
sillages (nœuds mutualisés). Hors champ : GPU 0,448 → **0,062 ms**, CPU 1,59 → 0,67, 727 Hz ;
dans le champ la visibilité coûte ≈0,05 ms de CPU. **CPU de préparation 3,12 ms pendant le forçage
de trois sillages** (204 Hz), 0,48 ms après (415 Hz). ADR-125 ne fixe pas la répartition : somme
CPU+GPU 4,5 ms en forçage, 2,1–2,6 ms hors forçage — verdict sur l'implémentation mono-fil sans
LOD temporel. Non-régression : 86 lignes de vérification identiques avant et après la visibilité.

**Limites.** Maximum réel échantillonné (marge ×2,06). Mono 341 Hz contre 384 en S234, non attribué
au-delà de la visibilité. Occlusion, composantes de B, angles rasants, interaction, I-06 de la pile,
seconde cible non reçus.

**Suite comparée.** Deuxième session J1-bis. La suite ne relève pas d'un approfondissement : la
scène représentative est **refusée à l'admission** alors qu'aucune pente réelle ne le justifie, et
c'est le déclencheur écrit d'A255/A261. **S236 : resserrer les bornes d'admission de la scène S235**
— enveloppe de pression à sources séparées, majorants d'impacts anciens à distance du maximum —
sans substituer une mesure à une borne (I-18), garantie d'ADR-128 conservée. Travail dans
`code/water-core`. Le CPU de préparation attend l'admission ; J2 (géométrie mobile), V, B2,
bathymétrie et seconde cible gardent leurs déclencheurs. Aucune ambition réduite ni arbitrage
utilisateur nécessaire.

**Rituel.** Suivis A255/A261 (sévérité portée à 1 pour J1), A265, A270 appliquée ; L314 ; feuille de
route (visibilité et mutualisation présentes, J1 remplacé, ligne CPU ajoutée), file active (J1-bis,
A255/A261, CPU, A258/A263, A270), index et README. Aucun ADR. Maillons 0 : scène multi-sources et
visibilité au bit consommées par la fenêtre, reçues contre le cœur et un passage continu.

## S236 — 2026-09-15 — Le cœur admet la scène, parce qu'il sert enfin l'eau que l'image dessine

**Entrée.** « Continue », master propre à `5744c8d`, trois copies au même commit, jeton libre,
secteur ; cœur à 413 réussis, 5 ignorés. Claude Code, Opus 5. Plan `209f99f`, bornes `a46563f`,
ADR-142 `58109ad`, plancher `2ebd8f2`, requête `25f98e2`, réception `6c27787`.

**Ce que la lecture a changé avant le plan.** La requête mixte ne compose que l'**intersection** des
emprises : sur huit disques répartis sur 70 m, presque aucun point de la scène S235 n'était servable,
avant même la pente. L'image, elle, rend « hors emprise, B seul » depuis ADR-126 (A271). Et le balayage
d'ADR-138 ne couvre que le disque de l'ancre : sûr pour l'intersection, faux pour l'union.

**Calculé avant de construire** (`--bornes-union`). Sur l'union : plancher actuel 49 refus, ADR-138
étendu 40, séparation 2D 32, **valeur atteinte par la fonction bornée 31** — une somme d'impacts exacte
en position refuse encore : les termes sont trop larges. Avec la pression **locale** d'ADR-137 sur les
seules cellules critiques (≤ 1 m), **les 32 instants restants se certifient** (92–128 cellules,
3–13 ms). La thèse déclarée prévoyait la bascule vers la pression locale ; elle a suffi sans localisation
générale (A261 reste vraie ailleurs).

**Capacité reçue.** ADR-142 : mode union ajouté à la requête mixte, mode intersection intact au bit.
`slope_floor_union` certifie par séparation (pool hôte, pression locale sur cellules critiques, arrêt à
1 cm ou pool plein) ; annonce et refus au même `max_slope`. **Le cœur admet S235 aux 161 instants**,
pire plancher 0,9995 π/7, **0 requête refusée** sur 6 988 sondes, et **sa requête est à < 1e-9 m de la
somme de référence de l'image**. Consommateur : l'hôte J1 et tout consommateur de la requête mixte.

**Preuves** ([ADMISSION-UNION-S236](../../docs/validation/ADMISSION-UNION-S236.md)). Six tests du cœur :
**trou d'ADR-138 sur l'union reproduit** (réelle 0,4252 contre plancher 0,2837) ; échelle de seuils
(certifié ⟹ réelle ≤ plancher ≤ seuil, un certificat par séparation exigé) ; pool plein et vide ;
chemin rapide au bit ; couverture au bit contre une somme à la main ; deux sens d'ADR-128 sur 25 seuils,
pression locale sollicitée. Suite du cœur **419 réussis, 5 ignorés**.

**Défauts rencontrés.** Garde de contrat S143 échouée : la sélection des cellules critiques comparait à
`max_slope` sans déclaration — sites réécrits `budget > max_slope` et inscrits (11). Premier contre-exemple
mal construit (impacts d'une seconde, bornes lâches, pas de violation). Échelle de seuils qui ne
certifiait que par le chemin rapide — seuils ajoutés. Test de couverture en échec sur un point non
représentable au 1/2048 m (piège S214).

**Coût et limites.** Plancher médiane 0,001 ms (chemin rapide), max **12 ms** avec pression locale ;
requête ≈ **0,2 ms par point** (pression 4 096 modes sur CPU, un fil) — hors du chemin d'image. Pression
locale : réception numérique d'ADR-137, pas certificat f32 (A258). Aucun cache entre lots. Choix du mode
par un hôte autoritaire non tranché (A271).

**Suite comparée.** Troisième session J1-bis consécutive : la scène représentative est désormais rendue,
visible et admise par le cœur ; ce qui reste de J1 — CPU de préparation, angles rasants, interaction,
I-06, seconde cible — ne débloque plus l'usage visé par la file et relève d'approfondissements datés.
J2 n'a pas avancé depuis S233. **S237 : J2 — surface géométriquement mobile dans le candidat δ, contre
référence non linéaire** (lot annoncé par S233). V, B2, bathymétrie et seconde cible gardent leurs
déclencheurs. Aucune ambition réduite ni arbitrage utilisateur nécessaire.

**Rituel.** ADR-142 ; note corrective datée d'ADR-138 ; A271 ouverte et corrigée par le mode union ;
suivis A255/A261 et A258 ; L315 ; feuille de route (J1 remplacé, mutualisation, travaux nécessaires),
file active (admission, coût de la requête, A258, CPU, J2 → S237), index. Invariants relus : I-18
(seuls des majorants de pente réelle comparés à `max_slope`), I-06 (pool hôte), I-04 inchangé.
Maillons 0 : composition et admission de la scène par le cœur, consommées par la requête, reçues contre
la pente réelle et l'image.

## S237 — 2026-09-15 — La surface de δ se déplace, et l'oracle d'ordre trois la reconnaît

**Entrée.** « Continue », master propre à `f5f78bf`, trois copies au même commit, jeton libre, secteur ;
cœur 419/5. Claude Code, Opus 5. Plan `69e531e`, protocole `0820146`, oracle `d912529`, opérateur
`f472ccb`, pas `68e0f6e`, réception `9b05c17`.

**Capacité reçue.** Le candidat δ MAC x-z possède une **surface géométriquement mobile** :
fonction hauteur, condition `p = 0` imposée à la hauteur réelle par fluide fantôme — opérateur
**symétrique au bit**, CG préconditionné par la diagonale —, mailles qui entrent et sortent du fluide,
advection quadratique, débits intégrés à la hauteur mouillée, niveau de référence fourni (repos exact
au bit à tout niveau), pas atomique sous le contrat temporel d'ADR-141 (note datée). Consommateur :
la prochaine brique δ (3D, cavité) et tout banc B3/B4 non linéaire.

**Oracle, reçu avant le candidat.** Onde stationnaire d'amplitude finie, bassin 2×2 m (`kh = π`, choisi
pour tenir dans le domaine d'Ursell d'ADR-122 à des amplitudes visibles). Ordre deux depuis le repos
dérivé par sympy, écrit en forme fermée ; véhicule HOS d'ordre 3 de S193 : à `K = 64`, plancher 0,31 %
du **symbole discret** (A242/L277) ; à `K = 256`, 0,034 / 0,82 / 3,28 % à 1 / 5 / 10 cm, rapport
**3,99 = (ka)²** — l'ordre quatre absent de l'analytique. Précision propre 5·10⁻⁵ `a`.

**Réception** ([SURFACE-MOBILE-S237](../../docs/validation/SURFACE-MOBILE-S237.md)). Critère 4, mobile −
linéaire à 1 mm : **0,167 %** (ordre deux : 0,15 %). **10 cm : profil 1,71 → 0,59 → 0,23 %, harmonique
1,71 → 0,98 → 0,43 %** à 32/64/128 colonnes, sous 2 % et 20 %. 5 cm : 0,85/2,44 et 0,55/1,41 %, **refus
`Convergence` à 128 colonnes** au quart de période. Témoin linéaire : harmonique ≤ 1,7·10⁻⁵ de la
référence, profil faux de 7,7 % et 16,2 % — l'effet non linéaire lui-même. Volume ≤ 7,5·10⁻⁹ m.
Six tests du cœur et un d'exécution (580 expirations, zéro allocation) : suite **425 réussis, 5
ignorés** ; empreinte S232 inchangée ; chemins S199–S233 au bit.

**Ce qui a été trouvé.** (1) **Le protocole était faux avant mesure** : « petite amplitude » à 1 cm
porte déjà une harmonique de 1,5 % de `a` ; corrigé à 1 mm, daté (L316). (2) Le refus de 5 cm à 128
colonnes : résidu **1,0492·10⁻⁶** figé jusqu'à 64 000 itérations, divergence 1,45·10⁻⁷, surface plate —
plancher de la pression f32 **au-delà des 8 192 mailles reçues par S231**, écarté de `θ_min` par
contre-épreuve (**A272**). (3) Un champ à divergence nulle écrit pour éprouver la garde d'après pas n'a
produit qu'un second membre d'arrondi et un refus en amont ; garde reçue sur dynamique réelle. (4) `θ_min
= 10⁻²` dégrade la précision d'un tiers ; `10⁻³` conservé.

**Coût et limites.** 4,6 / 33 / 260 ms par pas à 32/64/128 colonnes, ≈3 fois le mode linéaire ; aucune
technique de coût. Surface graphe, fond sec, bords ouverts, air, 3D, cavité et couplage B/W non reçus ;
une période ; advection centrée ; oracle potentiel. **Impasse** : remplacement PowerShell sur une source
UTF-8 → accents corrompus, restauré par Git.

**Suite comparée.** Cinquième session J2 au total, deuxième consécutive après les trois J1-bis. Les quatre
priorités du bilan S227 sont servies et il ne demande pas de nouvel audit. A272 est révélé par mesure et
bloque toute grille plus grande, 3D comprise ; ce n'est pas un approfondissement différable. **S238 : A272
— pression δ au-delà de 8 192 mailles**, loi du plancher et remède reçu sans relâcher le seuil, puis
rejouer 5 cm à 128 colonnes. V, B2, bathymétrie, CPU de J1-bis et seconde cible gardent leurs
déclencheurs. Aucune ambition réduite ni arbitrage utilisateur nécessaire.

**Rituel.** A272 ; L316 ; note datée d'ADR-141 ; feuille de route J2, file active (J2, A272, précision δ),
index. Invariants relus : I-06 (tampon réservé à la configuration, zéro allocation reçue), I-08 (durée
entière, coefficients arrondis), I-03 (réductions ordonnées). Maillons 0 : surface mobile consommée par le
pas, reçue contre un oracle non linéaire indépendant.

## S238 — 2026-09-15 — La pression f32 s'arrête là où f32 ne peut plus rien dire

**Entrée.** « Continue », master propre à `17ca164`, trois copies au même commit, jeton libre, secteur ;
cœur 425/5. Claude Code, Opus 5. Plan `cf3297d`, protocole `3bf0c43`, loi du plancher `845b8a7`, cycle
`c779a9b`, réception `944bb65`.

**Capacité reçue.** La pression f32 de δ **accepte un pas au plancher de sa précision** au lieu de refuser
après son plafond (ADR-143) : critère premier 10⁻⁶ inchangé ; arrêt si l'erreur inverse composante par
composante est sous la borne d'arrondi de la ligne (`ω ≤ γ₈`, dérivée pour quatre faces) ou si l'état
revient au bit (Brent, mémoire constante) ; réception à la seule condition de la divergence déclarée par
S199 (10⁻⁵). Consommateur : la trajectoire de la surface mobile au-delà de 8 192 mailles, et toute grille
plus grande.

**Mesure avant règle** ([PRESSION-PLANCHER-S238](../../docs/validation/PRESSION-PLANCHER-S238.md)). Famille
S231 : converge jusqu'à 256×128. Pas refusé de S237 : **`ω` = 1 `u`**, résidu semblant alterner entre deux
valeurs. `ω` n'est pas un seuil d'acceptation (4,7·10⁻⁴ sur des pas convergés, 10⁻¹⁶ sur un système sans
solution) ; la non-décroissance stricte non plus (S233).

**Deux constructions écartées par la mesure.** (1) Mémoire de deux relances : l'état parcourait une
**période de 420 relances**. (2) Brent seul : certifie, mais **7 386 itérations** dans le banc et pas 404
refusé sous 16 000 — longueur de cycle non bornée, alors que `ω` restait à 0,9–1,3 `u`. D'où le certificat
par borne (L317).

**Réception.** 5 cm à 32/64/128 colonnes : profil 0,85/0,55/**0,25 %**, harmonique 2,44/1,41/**0,71 %**,
sous 2 % et 20 %, décroissants ; 12/39/38 pas au plancher, `ω` 2,2–7,8 `u`, divergence ≤ 1,5·10⁻⁷, pire pas
**526 itérations** (S237 : refus à 64 000). Contre f64 du même système : pression 3·10⁻⁵, **vitesse
5,5·10⁻⁸** (estimation d'avant mesure 1,3·10⁻⁴). Système sans solution : dégradé par les deux arrêts.
Identité `div u = r/scale` à 7·10⁻⁸. Empreinte S232 au bit, `delta_precision` sans arrêt au plancher,
suite **429 réussis, 9 ignorés**.

**Ce qui a été trouvé.** (1) **Promesse d'avant construction non tenue** : « cas S237 au bit » valait pour
un arrêt sur stagnation ; le certificat coupe une ou deux itérations plus tôt 12 et 39 pas à 32/64 colonnes
— chiffres de réception inchangés, volume 3,7·10⁻⁹ → 1,5·10⁻⁸ m, publié. (2) Le résidu relatif des pas reçus
monte à 3·10⁻⁶ : le pas garantit `ω ≤ γ₈` et la divergence, plus 10⁻⁶. (3) **A273** : des pas convergés au
critère premier dépassent la divergence de S199 — 1,02·10⁻⁵ à 128×64 sur la bosse, dans le domaine de
S231, 1,58·10⁻⁵ à 256×128.

**Coût et limites.** 256 ms par pas médian à 128 colonnes, comme S237 ; aucune technique de coût. Opérateur
2D seulement (`γ₁₀` en 3D à dériver) ; une forme de second membre au-delà de 16 384 mailles ; certificat
jamais mis en défaut par un cycle au-dessus de `γ₈`. **Impasse** : message de commit par `Out-File` →
marque d'ordre d'octets en tête, réécrit par heredoc.

**Suite comparée.** Deuxième session consécutive sur la pression δ, première sur A273. A273 est révélé par
mesure, porte sur une tolérance déclarée avant construction, et grandit avec la taille : la 3D et la cavité
(J2) le rencontreraient d'emblée. **S239 : A273 — tolérance physique de la pression δ**, loi
divergence/taille puis arrêt qui la tient ou requalification datée. Avant une troisième session sur la
pression : vérifier que la suite débloque encore la 3D. V, B2, bathymétrie, CPU de J1-bis et seconde cible
gardent leurs déclencheurs. Aucune ambition réduite ni arbitrage utilisateur nécessaire.

**Rituel.** A272 fermée sur le domaine mesuré, A273 ouverte ; L317 ; ADR-143 ; feuille de route J2, file
active (précision δ, J2, A273), index. Invariants relus : I-03 (arrêts déterministes, bits rejoués), I-06
(passes sans allocation, tests d'exécution verts), I-14 (`γ₈` dérivé, 10⁻⁵ de S199). Maillons 0 : pas au
plancher accepté, consommé par la trajectoire mobile à 16 384 mailles, reçu contre HOS et f64.

## S239 — 2026-09-15 — Le seuil qui garantissait quelque chose ne le garantissait que d'une norme

**Entrée.** « Reprends le projet », master propre à `e8cfc6f`, trois copies au même commit, jeton libre
(battement 09:31, horloge 18:19), secteur ; suite 429/9. Claude Code, Opus 5. Plan `4e7694f`, protocole
`733f47a`, loi `8a454a9`, règle `ccce741`, réception `f17f7a2`.

**Capacité reçue.** La tolérance physique de S199 est désormais une **condition d'acceptation** de la
projection (ADR-144), et non un diagnostic publié après coup : `accepté ⟺ (ρ ≤ 10⁻⁶ ou plancher
d'ADR-143) et D_franches ≤ 10⁻⁵`. Un consommateur peut lire `degraded = false` comme « la projection
tient la tolérance déclarée » — ce qui était **faux** jusqu'ici. Consommateur : `delta_precision`,
`delta_filters` et la trajectoire mobile de S237, tous rejoués ; preuve
[TOLERANCE-PRESSION-S239](../../docs/validation/TOLERANCE-PRESSION-S239.md).

**Décomposition avant campagne.** De l'identité `div u = r/scale` (S238), `D = ρ·θ·Λ` exactement :
`ρ` le résidu relatif que le critère premier borne, `θ = max|r|/‖r‖₂` la concentration, `Λ` la forme
du second membre. Le critère premier ne borne que `ρ`. La campagne n'avait donc que deux nombres à
mesurer, pas une curiosité à explorer.

**Loi mesurée** (§2, 128 à 32 768 mailles, deux fonds). **`Λ` double à chaque raffinement** — 17,8 /
33,8 / 69,3 / 154,7 / 362 : le second membre vit à l'échelle de la maille, pas de l'écoulement. `θ`
décroît plus lentement que `N^(−1/2)` — 0,314 → 0,0437. Leur produit croît d'environ 30 % par
raffinement : **la taille seule fait franchir la tolérance**.

**Ce que la mesure a corrigé dans le plan.** La règle du §1.4, appliquée à toutes les mailles
mouillées, a fait **refuser le premier pas** du cas de topologie mobile de S237. Diagnostic : `ρ`
descend de 8,1·10⁻⁷ à 8,5·10⁻⁹ pendant que `D` plafonne à 1,5·10⁻⁵, et les cinq pires lignes sont les
mailles de **surface**, diagonale ≈ 500 (`1/θ_surface`), résidus **exactement** 2⁻⁵, 2⁻⁶, 2⁻⁷ sur des
lignes de magnitude ≈ 2¹⁸ — un ulp de leur propre ligne. Les lignes **franches** du même pas passent
de 7,5·10⁻⁵ à 4,5·10⁻⁷. D'où l'amendement : l'acceptation porte sur les lignes franches. Deux
corroborations jamais rapprochées : S199 avait écrit « **pas borne universelle** » en face de son
`< 10⁻⁵`, et S237 avait desserré une de ses propres assertions à 10⁻⁴ sans dire pourquoi.

**Réception.** 128×64 bosse : 1,021·10⁻⁵ → **5,84·10⁻⁶ pour une itération de plus** (348 contre 347).
256×128 : 420 itérations, plancher d'ADR-143, `D` = 1,335·10⁻⁵, **déclaré dégradé** — trois itérations
de plus pour un refus honnête. `delta_filters` : le fond plat, déjà sous la tolérance, est **identique
au dernier chiffre publié** (219 itérations) ; les deux fonds coupés à 128 poursuivent, ordres
1,947/1,957/1,959 ≥ 1,8, empreinte `0xc5ab1eadb094d058` → **`0xfb12b2092df4ee6d`**, et ces deux cas en
sont la cause entière. Trajectoire mobile 5 cm conservée : 0,850/0,552/**0,252 %** et 2,44/1,41/**0,71 %**,
**258,4 ms médian à 128 colonnes contre 256** (+0,9 %), pire pas 592 itérations contre 526. Suite
**431 réussis, 11 ignorés**.

**Coût et limites.** Techniques présentes : Jacobi, f32, certificats d'ADR-143, une correction et une
divergence de plus par relance qui atteint le critère premier ; absentes : tout préconditionneur plus
fort, tout résidu en précision double, le GPU. Domaine : une machine sur secteur, 2D, un `dt`, une
période. **Impasse** : mesurer en abaissant le seuil *dès le départ* place le solveur dans un régime
où le résidu récurrent ne réclame jamais la convergence — aucune relance, donc le certificat
d'arrondi, qui n'y vit qu'aux relances, n'est jamais consulté ; un cas a consommé 20 000 itérations
sans verdict et la mesure a été abandonnée. La règle n'a pas ce défaut : elle atteint d'abord le
critère premier.

**Suite comparée.** Troisième session consécutive sur la pression de δ, et A273 est close. A275 — au-delà
de 32 768 mailles, f32 ne tient pas la tolérance — est le **prérequis du passage à la 3D**, pas un
blocage d'aujourd'hui ; A274 — le plancher des lignes à fantôme — attend une géométrie nouvelle. La
recommandation encore non consommée du dernier bilan (BILAN-GLOBAL-S227 §5, ordre 4) est **J1-bis :
espace, LOD et visibilité intégrés au rendu, coût complet confronté aux 2 ms**. **S240 : J1-bis**,
angles rasants et interaction représentative, avec I-06 de la pile graphique. V, B2, bathymétrie et
seconde cible gardent leurs déclencheurs. Aucune ambition réduite, aucun arbitrage utilisateur requis.

**Rituel.** A273 close, A274 et A275 ouvertes ; L318 ; ADR-144, note datée sur ADR-143 ; feuille de
route J2, file active (précision δ, J2, faces coupées, A274, A275), index. Invariants relus : I-03
(arrêt déterministe, rejeu local identique en bits dans `delta_filters`), I-06 (aucun tampon nouveau —
la correction et la divergence de la boucle réutilisent `u`/`w`/`tmp`, libres pendant `project`
puisque `run` met les champs publiés à l'abri dans `saved_*`), I-14 (aucun nombre nouveau : 10⁻⁶ de
S231, 10⁻⁵ de S199, `γ₈` d'ADR-143). Maillons 0 : l'acceptation garantit désormais la tolérance, elle
est consommée par les trois campagnes et la trajectoire mobile, et la preuve est publiée.

## S240 — 2026-09-15 — Ce qui n'était pas compté n'était pas tenu, et le suspect était innocent

**Entrée.** « Continue », master propre à `45d24a5`, une seule copie, jeton libre, secteur. Claude
Code, Opus 5. Plan `61c55db`, protocole `37c65c2`, instrument `f2967cc`, mesure `ddd3131`, correction
et verdict `db0b10c`.

**Choix du lot, et ce qui a été constaté avant de choisir.** La recommandation encore non consommée
du bilan S227 est J1-bis, ordre 4. Ses quatre reliquats sont les angles rasants, l'interaction
manuelle, la seconde cible et **I-06 de la pile graphique** ; les deux du milieu demandent une
personne et une machine. Avant de partir sur les angles rasants, j'ai lu le code : **la grille locale
du sillage a pour emprise le domaine du sillage, pas la caméra** (`WAKE_MIN/MAX`), donc l'hypothèse
« un angle rasant sature la capacité de la grille » est fausse. L'hôte a aussi été construit et lancé
ici (RTX 5070 Laptop, DX12, `--smoke` 120 images) avant de promettre une mesure.

**Capacité reçue.** **I-06 est tenue par le code du projet dans la boucle d'image de l'hôte** : zéro
allocation par image en régime, mesuré sur 590 images et deux poses. Consommateur : toute mesure de
coût de J1-bis, qui n'a plus à porter l'inconnue « et les allocations ? » ; preuve
[ALLOCATIONS-HOTE-S240](../../docs/validation/ALLOCATIONS-HOTE-S240.md), décision
[ADR-145](../../docs/adr/ADR-145-i-06-pour-l-hote-graphique.md), amendement daté sous I-06.

**L'instrument.** Un allocateur compteur global enveloppant `std::alloc::System`, **aucune dépendance
nouvelle**, relevé aux bornes de phase **déjà chiffrées en millisecondes par S225** — de sorte que
les allocations se lisent en face des durées. Attribution par phase et non par site d'appel : une
pile d'appels demanderait une dépendance et déplacerait la mesure ; limite déclarée avant de mesurer.
Les tampons de relevé du banc ont été réservés — un `Vec` qui grandit pendant un banc de cadence
alloue dans la région qu'il mesure.

**Mesure.** 134 allocations et 30 541 octets par image, **médiane = p95 = maximum**, à l'octet près,
sur 590 images et trois exécutions. Une seule est à nous : le contour de l'emprise de la visibilité,
`Vec::with_capacity(2·(481+271))` = **12 032 octets**, 39 % des octets de l'image, construit et jeté
à chaque image. winit n'alloue **rien** entre deux images. Hors champ, la visibilité de S235 retire
30 allocations sur 134 — mais pas la nôtre, puisque le contour est justement ce qui décide de la
visibilité.

**Le suspect est innocent.** Le protocole nommait l'allocation comme cause directe de la gigue : CPU
4,02 ms médian pour **16,3 ms au maximum**, GPU immobile. Le compte d'allocations d'une image lente
est **exactement** celui d'une image rapide. **A265 perd une hypothèse** sans être close ; la cause
est ailleurs — ordonnancement, défaut de page sur de la mémoire déjà demandée, ou pilote.

**Correction et contrôles.** `footprint_into` remplit un tampon gardé par `FrameData` : le premier
contour d'un format lui donne sa capacité, aucun suivant ne demande de mémoire. `update` passe à
**0 allocation**, l'image à **133 et 18 509 octets** — les 12 032 disparaissent exactement. `VERIFY`
rend **0,368476 mm** à 12 s avec quatre impacts, la valeur de S235 ; `--retour` rend **0 image
différente au bit** sur 31 comparées ; CPU médian 4,0367 ms, dans la dispersion d'avant ; tests de
`viewer/` **12 réussis**, dont la comparaison au bit du tampon gardé sur trois formats enchaînés.

**Coût et limites.** L'instrument coûte **au plus 0,16 ms de CPU médian**, du même ordre que la
dispersion entre deux exécutions instrumentées (0,12 ms) : la mesure ne l'en sépare pas, et le
document le dit ainsi au lieu de le chiffrer plus finement. Les comptes, eux, sont exacts — ce sont
des compteurs, pas des durées. Une seule machine, un seul format, une seule version de wgpu : **133
est une référence datée, pas une constante**. Site d'appel dans la pile non nommé, mémoire réellement
engagée par le système non suivie.

**Ce que la mesure oriente, contre une conclusion antérieure.** S225 concluait que « les quatre
techniques restantes de J1-bis sont du côté GPU ». Depuis le LOD de S234, ce n'est plus vrai : à
960×540 sur la scène S235, **le GPU eau vaut 0,44 ms et la seule préparation CPU du sillage 3,17 ms
de médiane pour 13,2 ms de maximum** — soit, à elle seule, **1,6 fois le budget eau de 2 ms**
(ADR-125), et l'essentiel de la gigue. Les deux techniques absentes qui la visent sont nommées dans
la feuille de route : **LOD temporel** de la préparation et **parallélisme CPU**.

**Suite.** **S241 : J1-bis — la préparation CPU du sillage**, là où le budget se perd désormais. Les
angles rasants soutenus gardent leur déclencheur ; l'interaction manuelle et la seconde cible restent
hors de portée d'une session (REPRISE §5). δ, V, B2 et bathymétrie gardent les leurs. Aucune ambition
réduite, aucun arbitrage utilisateur requis.

**Rituel.** L319 ; ADR-145 et amendement daté sous I-06 ; feuille de route J1-bis, file active
(J1-bis, A265), index. Invariants relus : **I-06** (l'objet du lot, amendé pour cette couche seule),
I-03 (`--retour` rend 0 image différente au bit, et le test du tampon compare au bit). Maillons 0 :
la boucle d'image de l'hôte n'alloue plus rien à nous, les bancs de J1-bis la consomment, et les
nombres sont publiés.

## S241 — 2026-09-15 — Le mur n'était pas celui que la file désignait

**Entrée.** « Continue, j'ai découvert un projet nommé Niagara Pyro, cela peut être intéressant à
étudier. » Master propre à `dc0a236`, une seule copie, jeton libre, secteur, **accès web disponible**.
Claude Code, Opus 5. Plan `5b40431`, étude `2428ea5`, confrontation `b8cd648`, file `59ea89f`.

**Ce que la demande a déplacé.** La suite désignée par S240 — la préparation CPU du sillage — repart
en file avec son déclencheur, lot prêt : la demande de l'utilisateur prime (REPRISE §6.7).

**La règle posée avant de lire quoi que ce soit.** Une documentation d'éditeur est une **affirmation
de fournisseur**, pas une mesure de ce projet. REPRISE §2, I-14 et ADR-028 s'appliquent : Epic n'est
pas un interlocuteur, c'est une publication. Chaque ligne de
[COMPARABLES-EXTERNES](../../docs/COMPARABLES-EXTERNES.md) porte son URL et son statut — **documenté,
déduit, non trouvé** —, et **aucun nombre lu ailleurs n'entre ici comme seuil**. Le fichier est un
porteur **durable** : un comparable par section datée, jamais un document par session (L137).

**Ce que Niagara Fluids fait, documenté.** Gabarits **2D pour les jeux, 3D pour les cinématiques** ;
la grille 3D coûte, et le temps réel d'un gaz 3D coûteux passe par la **cuisson** en texture de volume
épars ; la pression se règle par un **nombre d'itérations** et un **facteur de relaxation**, et
**aucun critère de convergence n'est exposé** ; l'eau des grandes surfaces est un **champ de hauteur**.
Non trouvé, donc inconnu et écrit tel quel : la méthode du solveur, les valeurs par défaut, la
précision de la grille, un coût par image chiffré, et **toute mesure d'erreur physique**.

**Trois confrontations, et une qui renverse un ordre.**

1. **L'architecture a un témoin indépendant.** Champ de hauteur bon marché partout, grille chère
   réservée à des volumes bornés : c'est le partage de ADR-001 entre B/W et δ, atteint sans lien.
   Corroboration, pas preuve ; ADR-001 était acté.
2. **La question qu'ils ne posent pas.** Deux de nos sessions viennent de construire une acceptation
   sur un résidu **et** une tolérance physique (ADR-143, ADR-144) ; eux comptent des itérations. Cela
   ne rend pas notre critère faux — δ est reçu contre un oracle HOS à **0,252 %**, et leur
   documentation ne confronte leur simulation à rien. Cela situe notre classe de fidélité comme un
   **choix payé en coût**, et nomme le levier bon marché qu'ADR-144 interdit aujourd'hui.
3. **L'ordre des obstacles était faux.** Nos propres chiffres, déjà publiés et jamais rapprochés :
   **5,5125 ms par pas à 2 048 mailles** (S230, un pas = une image) contre **2 ms par image pour toute
   l'eau** (ADR-125) — soit **2,8 fois le budget pour δ seul** — et **35,51 ms** à 8 192 mailles pour
   2 ms simulées, soit ≈ 296 ms par image. La loi est régulière : quadrupler les mailles multiplie le
   temps par ≈ 8, ce qu'on attend d'un gradient conjugué et que confirment les itérations mesurées.
   **À la taille où A275 mord, le coût est déjà deux à trois ordres de grandeur au-dessus du budget.**
   Un lot qui n'aurait corrigé que la précision n'aurait rien débloqué.

**Ce que ces chiffres ne disent pas.** δ n'a reçu **aucune** technique de coût : présentes — Jacobi
diagonal, f32 ; absentes — GPU, parallélisme, multigrille, factorisation incomplète, itérations fixes,
cuisson. ADR-131 : un dépassement qualifie cette implémentation, jamais la fonctionnalité, et il
s'éprouve sur la **combinaison**, dont aucune pièce n'a été tentée.

**Ce que le comparable n'autorise pas.** Aucune réduction d'ambition — « Epic réserve la 3D aux
cinématiques » n'est pas un argument, ADR-127 exige une décision de l'utilisateur. Aucun nombre
importé. Aucune conclusion sur leur fidélité, qu'ils ne mesurent pas.

**Non fait, et assumé.** Aucun code, aucune mesure nouvelle : la confrontation n'utilise que des
chiffres déjà publiés, comme le plan l'avait déclaré. Le coût à 32 768 mailles reste **non mesuré**.
Aucune source d'Epic n'a été lue au-delà de sa documentation publique — ni code, ni essai.

**Suite.** **Maillons passe à 1** : cette session n'a reçu **aucune capacité**. Elle a produit une
connaissance et une correction de priorité, ce que la règle compte exactement pour ce que c'est.
**S242 : J1-bis — la préparation CPU du sillage**, la désignation de S240 rendue à sa place : 3,17 ms
de médiane et 13,2 ms de maximum contre 0,44 ms de GPU eau, instrument de S240 déjà en place. **A276**
— le premier lot de coût de δ, sur la combinaison des techniques — garde son déclencheur et passe
devant A275, qui se remesurera après lui. V, B2, bathymétrie et angles rasants gardent les leurs.

**Rituel.** L320 ; aucun ADR — rien n'a été décidé ici ; file active (A275 réordonnée, **A276**
ouverte, J1-bis reporté), feuille de route, index, `COMPARABLES-EXTERNES` créé. Invariants relus :
**I-14** (aucune valeur externe n'est devenue une provenance) et le §5 de REPRISE (aucun fait externe
transformé en hypothèse acquise). Maillons 1.

## S242 — 2026-09-15 — Ce n'était pas 2 ms à gagner, c'était un terme qui grandissait

**Entrée.** « Continue », master propre à `39a2af8`, une seule copie, jeton libre, secteur,
**Maillons 1**. Claude Code, Opus 5. Plan `fe10656`, protocole et banc `0b89042`, mesure `e2a7774`,
construction et réception `a6e44f1`.

**Capacité reçue.** La préparation CPU du sillage **ne dépend plus du nombre de tronçons achevés**,
c'est-à-dire de l'histoire du journal. Consommateur : le chemin d'image de l'hôte, rejoué
(`--verify`, `--retour`, `--cadence`) ; preuve
[PREPARATION-SILLAGE-S242](../../docs/validation/PREPARATION-SILLAGE-S242.md), **six empreintes des
4 096 coefficients publiés identiques** avant et après, sur quatre exécutions.

**La thèse déclarée était fausse dans sa grandeur, et la mesure l'a dit avant toute construction.**
Le plan visait les 98 304 tests par image que coûtait la sélection des tronçons. Mesuré après le
forçage, où **aucun** tronçon n'est actif et où tout ce tri est donc perdu : **3,7 µs par tronçon**,
soit 0,0595 ms entre 8 et 24 segments — **2 %** des 3,17 ms. Le protocole §1.5 avait prévu ce cas.

**Ce que la même mesure a désigné.** Par différence entre deux fenêtres à segments égaux : coût fixe
par nœud **76 ns** ; coût d'un tronçon **actif** **0,87 ms** sur 4 096 nœuds, soit 211 ns par nœud.
**Le poste est `ModalPressure::sample` : 2,60 ms sur 2,97, soit 87 %** de la préparation en forçage —
cinq à sept `sin_cos` déterministes par nœud et par tronçon actif.

**Une impasse qu'il faut avoir payée une fois.** `Complex::phase(negative(p))` semble être le conjugué
de `Complex::phase(p)`, donc gratuit. Il ne l'est pas **au bit** : `sin_cos` reconstruit l'angle par
quadrant depuis l'entier `2³⁰ − W`, pas par soustraction flottante, et à l'origine le zéro signé
diffère. Même chose pour « pendant le forçage, la rotation libre est l'identité ». **Réduire les
`sin_cos` change des bits publiés** : c'est un lot avec son relevé (A277), pas une simplification.

**Ce qui a été construit, et un témoin qui est tombé du travail.** La sélection est hissée : part
repliée pour tous les nœuds, puis chaque tronçon **actif** — et lui seul — parcourt les nœuds ; mêmes
termes, même accumulateur, même ordre. Une **seconde** variante avait été écrite : trier sur la
rangée 0 en gardant l'imbrication. Elle ne gagne rien — la ligne est déjà contiguë, ce n'est pas la
lecture qui coûte mais la boucle — et c'est ce qui l'a rendue utile : **sémantiquement neutre, elle
mesure +2,6 à +7,7 %** en fenêtre de forçage. Le vrai changement y mesurait +0,5 à +2,4 % : **plus
petit que le témoin, donc non attribuable**, et le document ne l'attribue pas.

**Réception.** Après forçage, la médiane **croissait de 19 % entre 8 et 24 segments** (0,3114 →
0,3709 ms) ; elle tient désormais entre **0,293 et 0,308 ms sans aucune tendance** sur trois
exécutions — **−19 %** à 24 segments. `VERIFY` **0,368476 mm**, `RETOUR` **0 image différente** sur
31, suite `code/` **431 réussis, 0 échec, 12 ignorés**, allocations de l'hôte inchangées (`update` 0,
image 133 / 18 509 o). Hôte : CPU 3,9877 ms médian contre 4,0367 en S240 — dans le bruit, comme
attendu puisque la scène est en forçage.

**Ce qui est réellement gagné.** Pas une constante : **un terme de croissance**. Au tarif mesuré,
une partie qui aurait accumulé 200 tronçons payait **0,74 ms par image** — plus du tiers du budget de
toute l'eau — pour des termes qui ne contribuent à rien. Elle paie zéro.

**Non fait.** Le poste dominant reste entier. Un seul nombre de nœuds, une recette, une machine, un
`dt`. La fenêtre de forçage de ce banc est trop bruitée pour y trancher quelques pour cent.

**Suite.** La technique qui attaque `sample` est le **parallélisme CPU**, et ce projet n'a **aucun
fil d'exécution** : `JobSystem` n'expose qu'une réduction ordonnée, et la seule implémentation est
séquentielle. C'est un prérequis **partagé** — il sert la préparation du sillage *et* le coût de δ
(A276). **S243 : le parallélisme CPU**, interface SPEC-004, ordonnancement déterministe (la boucle
des nœuds n'a aucune réduction : chaque nœud écrit sa case), premier consommateur `render_components`.
**A277** — réduire les `sin_cos` de `sample`, qui change des bits — garde son déclencheur. δ, V, B2,
bathymétrie et angles rasants gardent les leurs.

**Rituel.** L321 ; aucun ADR — rien n'a été décidé, le changement est au bit ; file active (J1-bis,
**A277** ouverte, parallélisme nommé), feuille de route, index. Invariants relus : **I-03** (six
empreintes identiques, `--retour` à 0 différence) et **I-06** (`update` toujours à zéro allocation).
**Maillons 0** : la préparation cesse de croître avec l'histoire du journal, le chemin d'image la
consomme, la preuve est publiée.

## S243 — 2026-09-15 — La moitié qui manquait au contrat, et la porte qu'elle n'ouvre pas encore

**Entrée.** « continue », master propre à `bbc57bd`, une seule copie, jeton libre, secteur,
Maillons 0. Claude Code, Opus 5. Plan `c85116b`, protocole `cca55ed`, primitive `2f45a4b`,
consommateur `7065780`, réception `2965bbf`.

**Capacité reçue.** Le système a un **parallélisme déterministe** : `parallel_fill_f32`, la seconde
primitive que SPEC-004 §8.2 spécifiait depuis l'origine et que personne n'avait écrite. Consommateur :
`Timeline::render_components`, exercé par le banc `sillage_troncons` et par l'hôte ; preuve
[PARALLELISME-S243](../../docs/validation/PARALLELISME-S243.md), décision
[ADR-146](../../docs/adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md).
**×2,63 à huit fils** sur le poste dominant : 3,2719 ms deviennent 1,2432.

**Ce n'était pas un changement d'interface.** SPEC-004 §8.2 énonce deux primitives ; seule la
réduction existait. Et `SequentialJobs` portait depuis S20 la phrase à rendre vraie : *le jour où une
version parallèle existera, l'assertion « changer `worker_count` change la vitesse, jamais le
résultat » se vérifiera contre celle-ci.* Elle est désormais un test.

**La dissymétrie, qui est le fond de l'affaire.** Pour une **réduction**, ADR-029 §3 a dû inscrire
`grain` dans le contrat : l'addition flottante n'est pas associative, donc deux découpages donnent
deux sommes, et la garantie s'énonce *à `n` et `grain` égaux*. Pour une **écriture disjointe**, rien
ne s'accumule d'une tâche à l'autre — chaque élément est écrit une fois, depuis des lectures seules —
donc le résultat ne dépend **ni du grain, ni du nombre de fils, ni de l'ordre**. Garantie
**inconditionnelle**, donc plus forte. Elle se démontre ; la recopier de sa voisine aurait imposé une
contrainte inutile, ou promis faux.

**Ce que les contraintes du langage ont dicté.** `water-core` est en `#![forbid(unsafe_code)]` : le
cœur ne peut pas découper une tranche mutable entre fils. C'est donc **l'hôte** qui découpe
(`chunks_mut`) et exécute (`std::thread::scope`), sans dépendance et sans `unsafe` non plus. Le cœur
publie sa vue plate par `as_flattened_mut`, et l'accumulateur d'un nœud vit dans les deux premiers
`f32` de son quadruplet — les mêmes `f32` que l'ancien champ `current`, donc les mêmes bits.

**Deux défauts attrapés par les critères déclarés, et c'est l'enseignement du lot.** Le protocole
exigeait, avant tout code, que le chemin à un fil ne soit pas ralenti et qu'aucune allocation ne soit
ajoutée au chemin d'image. (1) L'hôte construisait un tampon de tranches par appel : `update` passait
de **0 à 50 allocations par image**, effaçant ce que S240 venait de recevoir. (2) Lancer des fils là
où il n'y avait que 0,30 ms de travail **faisait monter** ce travail à 0,76 ms. Le banc principal,
lui, allait très bien. Corrigés : portion contiguë par fil sans tampon ; et **le grain encode le
travail par élément, que seul l'appelant connaît** — 76 ns par nœud à vide, 211 ns de plus par
tronçon actif, contre ≈ 67 µs pour créer un fil.

**Réception.** Six empreintes identiques à **1, 2, 4, 8 et 16 fils** et identiques à celles de S242 ;
`VERIFY` **0,368476 mm** ; `RETOUR` **0 image différente** ; l'oracle de chemin indépendant
`timeline_matches_prepared_across_instants_and_jumps_s213` vert ; suite **433 réussis, 0 échec,
12 ignorés**. Après forçage, le coût reste plat à tout nombre de fils : la régression est éteinte.

**Ce qui n'est pas gagné, et il faut le dire net.** **Le chemin d'image reste à un fil.** Créer les
fils par appel coûte ≈ 67 µs pièce **et alloue**, ce qu'ADR-145 interdit à 60 Hz. Un vivier
persistant supprimerait les deux — mais partager une tranche `&mut` empruntée avec des fils qui
survivent à l'appel **n'existe pas en Rust sûr** : `std::thread::scope` est la seule voie sans
`unsafe`, et elle rejoint avant de rendre la main. Le parallélisme sert donc aujourd'hui **les bancs
hors ligne**. Sur l'hôte, CPU 4,0581 ms contre 3,9877 en S242 — sous le témoin de bruit de L321.

**Non fait.** Un seul consommateur. La réduction reste séquentielle. Une machine, un système, 1 à
16 fils. δ a `jobs` en main mais ses boucles chaudes interrogent un budget coopératif (I-05) à chaque
poll : parallèliser un compteur partagé est un autre problème.

**Suite.** **A278** — autoriser `unsafe` dans l'hôte pour un vivier persistant — est une **décision**,
pas un réglage, et elle n'est pas prise ici. Mais elle n'est pas le chemin le plus court : le coût de
δ (**A276**) se mesure sur des **bancs**, où le parallélisme fonctionne déjà, et δ dépasse le profil
de deux à trois ordres de grandeur dès 2 048 mailles. **S244 : A276, le premier lot de coût de δ**,
sur la combinaison des techniques (ADR-131), en commençant par ses passes à écritures disjointes.
A277, A278, angles rasants, V, B2 et bathymétrie gardent leurs déclencheurs.

**Rituel.** L322 ; ADR-146 ; file active (J1-bis, **A278** ouverte, A276 outillée), feuille de route,
index. Invariants relus : **I-03** (six empreintes identiques à cinq nombres de fils — c'est
l'invariant même du lot) et **I-06** (`update` toujours à zéro allocation, après l'avoir cassé et
réparé). Maillons 0 : la primitive existe, un consommateur réel la consomme, la preuve est publiée —
et ce qu'elle n'atteint pas encore est écrit plutôt que passé sous silence.

## S244 — 2026-09-16 — La carte du coût de δ, et la route qu'un chiffre a refermée

**Entrée.** « continue », master propre à `fa99085`, une seule copie, jeton libre, secteur,
Maillons 0. Claude Code, Opus 5. Plan `ece02f8`, protocole et instrument `67b8df1`, décomposition
`7e0a118`, prix de l'outil `ed97bbc`, résultat négatif `791233d`.

**Aucun facteur gagné, et c'est le résultat.** Cette session rend la **carte** du coût de δ : où va le
temps, ce que chaque technique restante achèterait, et laquelle est fermée. Preuve
[COUT-DELTA-S244](../../docs/validation/COUT-DELTA-S244.md).

**La décomposition, avec son contrôle de somme.** Chaque pass mesurée isolément, puis confrontée au
pas : le prédit rend **79 à 89 %** du pas mesuré, stablement, aux cinq tailles — le reste étant les
relances, l'advection, les diagnostics et les certificats. La décomposition est donc juste, et ce
qu'elle n'explique pas, elle le borne.

| mailles | itérations | pas | écritures disjointes | réductions |
|---:|---:|---:|---:|---:|
| 128 | 30 | 0,089 ms | 69 % | 13 % |
| 2 048 | 114 | 5,256 ms | **67 %** (dont `apply` 47 %) | 12 % |
| 32 768 | 425 | **286,2 ms** | **73 %** | 13 % |

**Le coût à 32 768 mailles était non mesuré** — S239 et la file le disaient. Il vaut **286 ms par
pas pour une image de 16,7 ms** : δ seul est **143 fois** les 2 ms qu'ADR-125 donne à toute l'eau.

**La thèse tenait, et un chiffre l'a refermée.** Les écritures disjointes dominent : c'est bien elles
qu'il fallait paralléliser, et S243 venait d'en fournir la primitive. **Avant d'écrire le refactor**,
dix minutes ont chiffré ce qu'un appel parallèle coûte **à vide** : ≈ **125 µs par fil**, presque
indépendamment du travail. Les passes valent 21,7 / 5,5 / 3,8 µs. Un appel à deux fils coûte
**quatorze fois** la pass qu'il découperait. La primitive de S243 ne peut pas servir cette boucle —
non qu'elle soit mauvaise, mais parce qu'elle crée ses fils à chaque appel, et que δ appelle 114 fois
par pas des passes de quelques microsecondes. Le modèle rend compte des deux lots : pour le sillage,
`3 270/8 + 950 ≈ 1 360 µs` contre 1 243 mesurés.

**Correction datée de S243.** Son « ≈ 67 µs par fil » était **inféré** d'une différence entre deux
fenêtres d'un banc chargé ; la mesure directe donne **125 µs**, et c'est elle qui fait foi. La
conclusion de S243 — le chemin d'image reste à un fil — en sort **renforcée**. Note portée dans
COUT-DELTA-S244 §3 ; PARALLELISME-S243 et ADR-146 gardent leur texte, l'un renvoyant à l'autre.

**Ce qui a été tenté et annulé.** `apply` parcourt ses mailles avec `i` à l'extérieur alors que
`c = k·nx + i` : la boucle interne saute une rangée à chaque maille. L'échange est **exact au bit** et
ne gagne **rien**, à aucune des cinq tailles — 0,34149 contre 0,34170 ms à 32 768. **Annulé.** Les
branches par face tiennent le processeur, pas la distance entre deux lectures ; c'est aussi pourquoi
la vectorisation automatique n'opère pas. Un changement gardé « parce qu'il devrait aider » serait
une dette : la session suivante le lirait comme une optimisation reçue.

**Ce que la carte désigne.** Ce qui grandit n'est pas le coût d'une itération — stable en structure à
toutes les tailles — mais **leur nombre** : 30, 61, 114, 220, 425, un doublement par raffinement.
**La seule technique dont le gain augmente avec la taille est la multigrille** (ou un préconditionneur
équivalent), et c'est donc elle qui commande le passage à la 3D. Le vivier persistant (A278)
achèterait au plus ×3 sur 70 % du pas, et sert désormais **deux** consommateurs — le chemin d'image
et δ — mais demande `unsafe` dans l'hôte.

**Non fait.** Aucune construction. Une machine, un `dt`, un fond plat, un plafond d'itérations. Les
pas aux grandes tailles sont bruités (36,6 et 41,7 ms au même point à 8 192 mailles, quand `apply` y
est stable à 0,3 % près) : ce sont les **passes** qui font foi, pas le pas.

**Suite.** **Maillons passe à 1** : aucune capacité reçue. **S245 : la multigrille pour la pression de
δ** — le seul levier dont le gain croît avec la taille, et le préalable mesuré du passage à la 3D. Sa
réception ne portera **pas** sur les bits, qui changeront, mais sur les critères d'acceptation
d'ADR-144 (résidu premier et tolérance physique) et sur le compte d'itérations. A278, A277, A275,
angles rasants, V, B2 et bathymétrie gardent leurs déclencheurs.

**Rituel.** L323 ; aucun ADR — rien n'a été décidé ni construit ; file active (A276 cartographiée,
A278 renforcée, multigrille nommée), feuille de route, index. Invariants relus : **I-05** (le budget
coopératif est un état partagé, et c'est pourquoi on ne le distribue pas) et **I-03** (l'échange de
boucles était exact au bit ; il a été annulé pour absence de gain, pas pour risque). Maillons 1.

## S245 — 2026-09-16 — La multigrille a raté sa cible et fermé une autre anomalie

**Entrée.** « Continue », master propre à `feb1dd9`, une seule copie, jeton libre, secteur,
Maillons 1. Claude Code, Opus 5. Plan `21cee50`, protocole `b93d173`, hiérarchie `6b81b87`, cycle et
symétrie `b97ee5f`, branchement et réception `6dc0bfa`.

**Capacité reçue.** **δ accepte 32 768 mailles**, ce qu'il refusait depuis S239. Consommateur : le
pas lui-même, qui rejoue avec la multigrille **s'il a été refusé** ; preuve
[MULTIGRILLE-S245](../../docs/validation/MULTIGRILLE-S245.md), décision
[ADR-147](../../docs/adr/ADR-147-la-multigrille-est-un-repli-de-precision.md). **A275 est fermée.**

**Ce que la lecture a corrigé au passage.** `jacobi = self.mobile` : le chemin à couvercle fixe —
celui que S230 et S244 mesurent — n'a **aucun** préconditionneur. L'en-tête ADR-131 de S244 annonçait
« Jacobi diagonal » ; note corrective datée portée dans ce document. Les mesures restent exactes.

**La cible visée, et manquée.** S244 avait désigné la multigrille comme le seul levier dont le gain
croît avec la taille. Construite, elle fait tomber les itérations de **425 à 134** à 32 768 mailles —
mais un cycle coûte **cinq produits fins par itération**, et le pas passe de 319 à 729 ms. Aux
petites tailles c'est pire encore : la hiérarchie n'a qu'un ou deux niveaux et les itérations
**triplent**. **En vitesse, la multigrille perd partout.**

**La contre-épreuve, faite avant de conclure.** Hiérarchie plus profonde, cycle allégé : **pire** —
282 / 260 / 292 / 284 / 300 itérations, plates mais hautes. Un compte plat est la signature d'une
multigrille qui fonctionne ; plat **et haut** dit que le taux par cycle est mauvais. Ce n'est donc ni
le niveau grossier ni le lisseur : c'est le **transfert**, la prolongation étant constante par
morceaux. Sans cette mesure, la suite aurait été cherchée au mauvais endroit.

**La cible atteinte, qui n'était pas la sienne.** Le banc relevait aussi la divergence et le drapeau
`degraded`. À 32 768 mailles : **1,339·10⁻⁵ refusé devient 8,512·10⁻⁶ reçu**. A275 disait que f32 ne
tenait pas la tolérance à cette taille ; la cause n'était pas la taille, c'était **l'arrondi accumulé
sur 425 itérations**. En en demandant 134, le solveur en accumule moins, et le **même** test sur le
**même** vrai résidu recalculé passe. **Aucune tolérance n'a été relâchée.**

**D'où le branchement : un repli, pas un remplacement.** Le pas ordinaire d'abord ; s'il est refusé,
et seulement alors, rejoué avec la multigrille. Aux tailles qui passaient déjà, **rien ne change, au
bit** — 30 / 61 / 114 / 220 itérations, mêmes résidus, mêmes divergences, empreinte `delta_filters`
**`0xfb12b2092df4ee6d` inchangée**, dix cas de `delta_precision` identiques. À 32 768 mailles, reçu
pour 1 006 ms au lieu de 319 refusées.

**Ce qui garde la construction.** L'opérateur d'un niveau grossier rend **les mêmes bits** que celui
du chemin fin sur la grille fine — l'opérateur est écrit deux fois et cet essai interdit aux deux
écritures de diverger (L137). Restriction et prolongation sont **adjointes**. Le cycle est
**symétrique et défini positif**, vérifié et non supposé : sans cela la récurrence du gradient
conjugué ne tient plus, et le symptôme se confondrait avec un mauvais réglage. Suite complète
**437 réussis, 0 échec, 15 ignorés**. L'essai de comptabilité d'allocation **recalcule** la
hiérarchie indépendamment du cœur — un essai de comptabilité qui appellerait la même fonction que le
code ne vérifierait rien.

**Ce qui n'est pas gagné.** **Le coût de δ reste entier** : 143 fois le budget d'ADR-125 à 32 768
mailles. Ce lot n'a pas gagné de vitesse et ne prétend pas le contraire. Le repli coûte un pas
perdu ; aucun critère ne prédit le refus avant de résoudre. Le mode mobile n'a pas la multigrille.
Une machine, un pas de temps, un fond plat au banc de coût.

**Suite.** **S246 : la prolongation bilinéaire** avec sa restriction transposée — c'est elle que la
contre-épreuve désigne, et les essais d'adjonction et de symétrie sont déjà écrits pour l'accueillir.
Si elle amène le taux par cycle où la théorie le place, la multigrille deviendra aussi un levier de
vitesse, et c'est le seul dont le gain croisse avec la taille. A277, A278, A274, angles rasants, V,
B2 et bathymétrie gardent leurs déclencheurs.

**Rituel.** L324 ; ADR-147 ; note corrective datée sur COUT-DELTA-S244 ; file active (**A275
fermée**, A276 mesurée, A280 ouverte), feuille de route, index. Invariants relus : **I-06** (la
hiérarchie est comptée avant d'être allouée, et l'essai le recalcule à part) et **I-03** (les tailles
qui passaient gardent leurs bits ; l'opérateur grossier et l'opérateur fin coïncident au bit).
Maillons 0 : 32 768 mailles est accepté, le pas le consomme, la preuve est publiée.

## S246 — 2026-09-16 — La thèse était fausse, et le coupable était un nombre que personne ne soupçonnait

**Entrée.** « Continue », master propre à `b5abb80`, une seule copie, jeton libre, secteur,
Maillons 0. Claude Code, Opus 5. Plan `a33da3d`, protocole et base `e4bfe4b`, construction, mesure et
annulation `39df8aa`.

**La thèse déclarée est réfutée.** A280 visait la **prolongation bilinéaire** : c'était le suspect
que la contre-épreuve de S245 désignait pour expliquer le mauvais taux par cycle. Construite comme le
protocole le demandait — poids (3/4, 1/4), report de bord, **source unique** pour que prolongation et
restriction restent transposées —, elle **ne gagne rien** : 0,648 / 0,642 / 0,652 contre 0,637 /
0,635 / 0,674 pour l'injection. Dans le bruit, et l'écart change de signe. **Annulée** (L323).

**Un cinquième suspect écarté au passage.** Le taux mesuré **sans aucune coupe du fond** est le même
qu'avec — 0,71 / 0,56 / 0,63 contre 0,65 / 0,64 / 0,67. La géométrie des mailles coupées n'y est pour
rien non plus.

**Ce que la recherche a trouvé à la place, et c'est l'apport du lot.** N'ayant plus où chercher, j'ai
relu le seul nombre que personne ne soupçonnait : l'amortissement du lisseur. Il portait `2/3` **avec
sa justification** — « il se dérive, il minimise le facteur de lissage du stencil à cinq points ».
La phrase avait la forme d'une provenance, citait le bon stencil, et **était fausse** : `2/3` est
l'optimum à **une** dimension. En deux dimensions, un balayage de Jacobi amorti multiplie le mode
`(θx, θz)` par `1 − (ω/2)(2 − cos θx − cos θz)` ; égaler les deux extrêmes de haute fréquence —
`1 − ω/2` et `1 − 2ω` — donne **ω = 4/5**, pour un facteur de lissage de **3/5** au lieu de `2/3`.

Je l'avais écrite en S245, dans ce dépôt, sous un invariant qui exige qu'aucune valeur n'entre sans
provenance (I-14). Elle a passé une relecture, un ADR et une réception. **Une provenance fausse est
plus dangereuse qu'un nombre nu** : le second attire la vérification, le premier l'écarte.

**Ce que la correction rend, mesuré.** Deuxième cycle à 32 768 mailles **0,187 → 0,113** ; taux
asymptotique **0,798 → 0,674** ; itérations forcées 99 / 252 / 220 / 167 / 134 → **94 / 177 / 158 /
120 / 106** ; et sur le chemin réel, à 32 768 mailles, **1 006 → 828 ms** avec une divergence de
**8,512·10⁻⁶ → 7,265·10⁻⁶**. **A275 reste fermée, avec plus de marge et pour moins cher.**

**Réception.** Suite **437 réussis, 0 échec, 16 ignorés** ; empreinte `delta_filters`
**`0xfb12b2092df4ee6d` inchangée** ; dix cas de `delta_precision` identiques ; symétrie et positivité
du cycle conservées ; opérateur grossier et opérateur fin toujours **au bit**. Aux tailles qui
passent sans repli, rien ne change : le repli ne s'y déclenche pas, donc l'amortissement n'entre dans
aucun bit publié.

**Ce qui reste, et ce que deux sessions ont éliminé.** Le taux asymptotique tient à **0,63–0,67**, là
où ce stencil devrait donner 0,1 à 0,3. Cinq suspects ont été écartés par la mesure — niveau le plus
grossier, quantité de lissage, ordre de la prolongation, géométrie des mailles coupées — et le
cinquième, l'amortissement, était **fautif et corrigé**. Reste le dernier debout, que ce lot **ne
nomme pas** : les **bords**, avec un opérateur grossier **re-discrétisé** et non construit par
Galerkin ; la différence entre les deux se voit surtout là où les coefficients varient.

**Non fait.** Le coût de δ reste entier. Une machine, un pas de temps, deux fonds. Le mode mobile n'a
toujours pas la multigrille.

**Suite.** Troisième session consécutive sur le solveur de δ, et **METHODE demande de comparer avant
la suivante**. Le plafond du taux (**A281**) est un approfondissement dont le gain est incertain et
dont le déclencheur naturel est le passage à la 3D ; la file porte, elle, une capacité mesurable et
en attente depuis S240 : **les angles rasants soutenus** de J1-bis, seul reliquat de cette ligne
qu'une session puisse traiter seule. **S247 : J1-bis, angles rasants soutenus.** A281, A278, A277,
A274, V, B2 et bathymétrie gardent leurs déclencheurs.

**Rituel.** L325 ; aucun ADR — ADR-147 n'est pas touché, seul un de ses paramètres est corrigé ;
correction datée dans `delta_multigrid.rs` et dans PROLONGATION-S246 ; file active (**A280 close**,
**A281 ouverte**), feuille de route, index. Invariants relus : **I-14** — c'est lui que la session a
vu manquer chez elle-même, et la leçon porte son nom — et **I-03** (empreintes inchangées, opérateurs
au bit). **Maillons 0**, et la justification est écrite pour qu'on puisse la contester : la thèse du
lot a échoué, mais une **correction d'intégrité** a été reproduite et testée, et elle a un effet aval
mesuré — le repli d'ADR-147 est 18 % moins cher et garde plus de marge.

## S247 — 2026-09-16 — Le coût tient, l'échantillonnage non, et pas comme on l'attendait

**Entrée.** « Continue », master propre à `e7a23e3`, une seule copie, jeton libre, secteur,
Maillons 0. Claude Code, Opus 5. Plan `cc88c7d`, protocole et pose `39f1502`, instrument étalonné
`7e308d6`, mesure et verdict `1f94e70`.

**Le reliquat traité.** Les **angles rasants soutenus** étaient inscrits depuis
[S225](../../docs/validation/CADENCE-HOTE-S225.md) parmi ce que les mesures de l'hôte ne couvraient pas,
et c'était le dernier reliquat de J1-bis qu'une session puisse traiter seule : l'interaction manuelle
demande une personne, la seconde cible une autre machine (REPRISE §5). Preuve
[RASANT-S247](../../docs/validation/RASANT-S247.md).

**L'étalon, passé avant la mesure.** Le protocole exigeait que l'instrument d'échantillonnage soit
d'abord passé à la pose de référence, où il devait retrouver les **7,5 %** publiés par S234. Il a
rendu **7,54 %**. Un instrument neuf qui annonce un chiffre neuf n'est pas une mesure ; l'ordre
— étalonner, puis employer — n'a rien coûté et rend défendable tout ce qui suit.

**Le coût tient, et la thèse est confirmée.** GPU eau **0,4397 ms, identique à la quatrième décimale**
entre la pose de référence et la pose rasante ; CPU médian 3,9021 contre 4,0563 ; intervalle 4,5826
contre 4,7729 ; allocations de `update` toujours nulles. Le nombre de sommets est fixé par la grille,
pas par la pose. La visibilité de S235 ne gagne rien à incidence rasante — 95,9 % de l'écran est de
l'eau — mais elle n'y perd rien non plus.

**L'échantillonnage ne tient pas, et ce n'est pas la part qui bouge, c'est la profondeur.** La
fraction de sommets sous Nyquist est celle de la pose de référence — **7,59 % contre 7,54 %** — mais
le **pire écart triple, de 2,589 à 8,243 m** : près de **quatre longueurs d'onde** sautées entre deux
échantillons, là où la référence en saute une demi. Un banc qui n'aurait relevé que la part aurait
conclu « rien ne change ». La pose haute, elle, est propre : 0,01 % et 1,050 m.

**Et le chiffre élimine le mauvais remède.** 8,2 m au loin, ce sont exactement les **deux pixels** de
la grille projetée : à cette distance, une onde de 2 m est **plus petite qu'un pixel**. Densifier le
maillage dépenserait des sommets pour dessiner ce que l'écran ne peut pas montrer. Le remède est
celui que la feuille de route nomme déjà — **couper les modes par la distance** — et cette mesure en
donne la **spécification chiffrée** : au plus loin de la pose rasante, les modes sous **≈ 16 m** ne
sont pas résolubles.

**Non fait, et assumé.** La coupure n'est pas construite : elle change le champ publié, donc les
bits, et sa réception demande de repenser ce que `VERIFY` compare — un champ coupé par la distance
**doit** s'écarter du cœur au loin, et l'écart mesuré aujourd'hui deviendrait le résultat attendu
plutôt que le défaut. La coupure du fond `B` n'est pas mesurée. Aucune image n'est jugée : la mesure
est géométrique. Un format, une machine, une scène.

**Suite.** **Maillons passe à 1** : cette session **qualifie** un chemin reçu, elle n'en reçoit pas
de nouveau. **S248 : A282 — la coupure des modes par la distance**, le seul lot de J1-bis qui ait
désormais un défaut mesuré et une spécification chiffrée derrière lui. A281, A278, A277, A274, V, B2
et bathymétrie gardent leurs déclencheurs.

**Rituel.** L326 ; aucun ADR — rien n'a été décidé ni construit, seule une mesure a été faite ; file
active (J1-bis : angles rasants faits, **A282 ouverte**), feuille de route, index. Invariants relus :
**I-06** (`update` reste à zéro allocation à la pose la plus chargée) et **I-09** — on interpole des
paramètres, jamais des réalisations, et c'est précisément ce que la coupure par la distance devra
respecter quand elle sera construite. Maillons 1.

## S248 — 2026-09-16 — La mer et son maillage, vus plutôt que décrits

**Entrée.** « Continue, avant réalise une mission intermédiaire qui met en avant la topologie de la
mer, le mesh du lod. » Master propre à `3338804`, une seule copie, jeton libre, secteur, Maillons 1.
Claude Code, Opus 5. **Demande explicite de l'utilisateur**, qui prime sur la suite automatique
(REPRISE §6.7) : A282 repart en file, lot prêt, déclencheur conservé. Plan `0ebab41`, ossature
`fd2652c`, mer `ae303eb`, réparation `ade7bcc`, maillage et réception `588a2b0`.

**Ce qui a été fait.** Six images locales de banc sous `captures/s248/`, ce qu'ADR-124 autorise
explicitement depuis S201 — **PPM local, aucune publication, aucune page**. Les images ne sont pas
versionnées : `.gitignore` exclut `captures/`, parce que le dépôt ne contient que de la connaissance.
**Seules leurs empreintes le sont**, et elles suffisent à refaire l'expérience. Preuve
[IMAGES-S248](../../docs/validation/IMAGES-S248.md).

**La mer, et pourquoi il a fallu deux images.** La première ne montre presque rien, et c'est le
résultat : le fond sature la rampe à **0,988644 m** et noie tout ; on ne voit que la houle longue en
bandes. La perturbation vaut **0,149809 m**, soit **15,15 %**. Séparées, les couches d'ADR-001 se
lisent d'un coup d'œil — trois sillages en V avec leur source, quatre anneaux d'impact. C'est
l'argument de la décomposition, rendu visible : `B` partout et lisse, `W` et `δ` locaux et structurés.
`background_only` rend `B` seul **par le même chemin** que `references`, pour que la soustraction
soit exacte et non approchée.

**L'étalon a mordu, et c'était son rôle.** Le protocole exigeait que les cartes retrouvent les
extrema de S247 **avant qu'on les regarde**. Le premier jet annonçait **567,9 m** et **1 228,8 m** là
où S247 publie 2,589 et 8,243. La carte ne mentait pas : elle mesurait **autre chose** — S247 compte
dans l'emprise du sillage, la carte comptait jusqu'à l'horizon. Les deux quantités séparées, l'étalon
passe **exactement** : 2,589 / 8,243 m et 7,54 % / 7,59 %. **Et le désaccord est devenu un résultat** :
sur toute l'eau visible, l'écart entre sommets monte à **567,9 m** à la pose de référence et
**1 228,8 m** à la rasante, ce que S247 n'avait pas relevé.

**Ce que les cartes montrent et que les nombres ne disaient pas.** La dégradation n'est **pas
répartie** : l'écran est vert partout sauf une **bande étroite à l'horizon**, qui vire au jaune puis
au rouge sur quelques pour cent de la hauteur d'image — les 7,59 % sont exactement cette bande. Un
histogramme donnait le même chiffre sans dire qu'il est concentré. Et la carte monde montre les
rangées de sommets **se séparer** en traits distincts avec du noir entre elles : des mètres d'eau
qu'aucun sommet n'échantillonne. À 150 m, **2,4 %** des pixels portent un sommet à incidence rasante,
contre 5,5 % à la pose de référence.

**Ce que cela instruit pour A282.** Une spécification **visible** : couper dans la bande d'horizon et
là seulement — le reste est sain, et une coupure uniforme y retirerait de l'onde pour rien ; suivre
une transition continue, puisque l'écart croît par rangées régulières ; et accepter que `VERIFY`
attende désormais un **écart** dans la bande rouge, pas son absence.

**Un outil, parce qu'il manquait.** `outils/apercu_ppm.py` écrit un PNG à côté du PPM — Python
standard, sans réseau ni dépendance. ADR-124 autorise « PPM et preview », et le dépôt n'avait pas de
quoi regarder ses propres images.

**Non fait.** Aucune capture de l'afficheur : ces cartes sont géométriques, pas des images rendues.
La coupure de `B` n'est pas mesurée. Un format, un âge, deux poses. Le maillage local du sillage
n'est pas représenté — il ne dépend pas de la caméra, et ces images portent sur ce qui en dépend.

**Une maladresse, consignée.** Deux notes de reprise ont été avalées par une substitution du shell
— des accents graves dans une chaîne passée à `bash` — et réparées au commit suivant. Écrire les
textes par fichier plutôt que par ligne de commande évite la classe entière.

**Suite.** **Maillons passe à 2** : cette session **instrumente et donne à voir**, elle ne reçoit
aucune capacité nouvelle. À deux maillons, REPRISE demande un lot qui fasse avancer une capacité, et
la file en porte un, prêt et désormais spécifié visuellement : **S249 : A282 — couper les modes par
la distance**. A281, A278, A277, A274, V, B2 et bathymétrie gardent leurs déclencheurs.

**Rituel.** L327 ; aucun ADR — aucune décision, aucune règle changée ; file active (J1-bis, A282
enrichie de sa spécification visible), feuille de route, index, `outils/apercu_ppm.py` référencé.
Invariants relus : **I-03** (six empreintes reproduites à l'identique sur deux exécutions) et le
§11.3 de SPEC-005 que `.gitignore` applique — rien de dérivé, rien de binaire dans le dépôt.
Maillons 2.


## S249 — 2026-09-16 — Couper les modes lointains dans l’image consommée

**Entrée.** « Reprends le projet », puis « Continue ». Master propre à `12bd90f`, une
seule copie ; lignée B archivée conservée. Codex GPT-6. Deux maillons d’instrumentation :
A282 choisie avant A281 (approfondissement incertain) et les extensions V encore absentes,
car son défaut, son consommateur et sa mesure étaient déjà identifiés. Plan `70d300e`,
décision `64cddf7`, construction `1e694a2`, réception `c589f54`.

**Capacité reçue.** L’afficheur retire désormais progressivement les modes de B et du
sillage que le maillage lointain ne résout pas. Chemin consommateur : rendu GPU par défaut,
bandes séparées à la cuisson, amplitudes pondérées selon les voisins projetés (ADR-148).
La recette autoritaire, ses phases et son domaine restent intacts. `--no-spectral` conserve
le témoin ; `--spectral-verify` sépare erreur numérique et quantité volontairement retirée.

**Preuves.** 36 cas, trois poses, deux formats, trois âges, direct/grille : hauteur
**≤0,340 mm pour 3 mm**, retour de caméra au bit. Huit modes isolés GPU : conservation
proche et zéro au-delà de Nyquist. Réception historique sans filtre passée (46 contrôles,
six intérieurs/coutures). Cœur/harnais **437 réussis / 16 ignorés**, hôte **16 / 1**.
Preuves, commandes et limites : [COUPURE-S249](../../docs/validation/COUPURE-S249.md).

**Prix réel.** En régime : GPU **1,22–1,33 ms** contre **0,46–0,49**, environ 0,8 ms
pour la qualité ; mémoire GPU +2 Mio. Premier passage jusqu’à **2,26 ms**, non effacé,
cause non attribuée. CPU toujours ≈4 ms : le budget global n’est pas reçu. Zéro allocation
dans `update`, pile constante **133 / 18 509 octets**. Premiers coûts relevés pendant la
suite du cœur écartés ; comparaison finale hors écran, âges identiques, secteur avant/après.

**Limites et non-fait.** Impacts non filtrés ; pentes cosmétiques sans dérivée spatiale
du filtre ; pas de réception perceptive en mouvement ni seconde cible. La transition
commence avant Nyquist et les diagonales sont prises en compte : elle dépasse la bande
rouge de S248. Aucun certificat global d’absence d’alias. A282 reste partielle, déclencheur
de ses reliquats : prochaine extension J1. Le filtrage ne réduit pas les nœuds de quadrature.

**Suite.** Maillons **0** : capacité intégrée et reçue, pas seulement un banc. **S250 :
raccorder B/W au candidat δ 2D, source d’écart et éponge**, prochain consommateur absent
des fournisseurs différentiels déjà construits (A50). Priorité à cette articulation plutôt
qu’à une quatrième session d’image ou à la seule vitesse des bandes ; A276/A281 restent
préalables mesurés de la 3D, A244 reste requis pour intégrer les domaines sous budget.
V, B2, bathymétrie et multiplateforme gardent leurs déclencheurs ; aucun arbitrage externe.

**Rituel.** ADR-148, preuve et index ; file active relue entièrement, états A280/A275 périmés
corrigés sur les lignes touchées, A282 actualisée avec sévérité 2 et limites. Trajectoire,
README, journal, plan et jeton synchronisés. I-03, I-06, I-08, I-09 et I-13 relus : cœur
inchangé, allocations mesurées, amplitudes du même champ, rendu sans autorité. Aucune
copie isolée ouverte ; aucun distant modifié.

## S250 — Premier raccordement volumique B/W→δ ; démarrage plat refusé

**2026-09-16, Codex GPT-6.** Entrée « Continue », master propre f2dd759, copie unique,
archive B conservée. Plan `7ed21ad`, décision `87cfd41`, pas `6d791b3`, réception
`79e32a0`. Suite choisie par S249 : coupler les fournisseurs existants au candidat MAC.

**Capacité reçue, maillons 0.** `Volume::step_perturbation` consomme les échantillons
différentiels B/W : source continue -S après somme, deux advections croisées, éponge
quadratique puis projection. API du candidat, pas simple composition finale dans un
banc. Durée/budget entiers, aucune allocation, publication atomique. Vingt pas avec
B et pression W réels reçus sous une hauteur perturbative imposée de 1 cm.
ADR-149 borne ce premier lot à la géométrie fixe et aux bords de perturbation.

**Preuves.** Six tests unitaires et un test d'allocations ajoutés ; suite complète
**444 réussis / 16 ignorés**, zéro échec. Empreinte historique **0xfb12b2092df4ee6d**
conservée. Banc réel : D maximal 5,787·10⁻⁶ ; retirer S change effectivement la vitesse.
Sommer les résidus séparément perd jusqu'à 1,520·10⁻³ m/s² de termes croisés.
Sur secteur, médianes préparation/pas 0,1833/0,1747 ms, maximum du pas 0,2743 ms,
20 pas, premier inclus. [Protocole, commandes et limites](../../docs/validation/RACCORDEMENT-DELTA-S250.md).

**Défaut trouvé, non escamoté.** À hauteur perturbative nulle et v=0, le pas refuse
la projection à 16×8 et 32×16, D=1,42·10⁻⁴ / 8,06·10⁻⁵ au plancher. A283, sévérité 2.
Cas `--flat` conservés, état intact vérifié. Une hauteur imposée qui permet de passer
ne corrige pas ce défaut. Source gradient, annulation et normalisation : suspects,
pas cause démontrée. Aucune tolérance changée. Attribution erronée du profil cubique
à ADR-046 dans ADR-149 : note factuelle datée ; le code emploie le profil quadratique.

**Non-fait et suite S251.** Recevoir le démarrage plat par comparaison à une projection
f64 indépendante avant d'étendre ce raccordement. Puis résidus de surface et frontières
du total avant couplage mobile. Ni B4 global, ni transduction vers W_local, ni réflexion
B2, ni intégration au rendu, ni budget I-05 global reçus. Les coupes radiales 3D sont
refusées explicitement. A276/A281, V, bathymétrie et multiplateforme restent à la file,
avec leurs déclencheurs ; pas d'arbitrage externe ni réduction d'ambition.

**Rituel.** File active relue entièrement ; trajectoire, index, A283, plan et jeton
actualisés. I-01/02/04/05/06/07/08/11/12/14 relus : échantillons transitoires d'un
instant, champs f32, coefficients ADR-141, éponge paramétrée de banc, aucune autorité
ou sérialisation ajoutée. Aucun worktree créé ni distant modifié ; archive conservée.

## S251 — Démarrage couplé plat reçu ; son coût ne l'est pas

**2026-09-16, Codex GPT-6 (P1–P4), clôture Claude Opus 5 (P5).** Entrée « continue »,
master propre 693ec55, copie unique, archive B conservée. Plan `ab8d198`, diagnostic
`fd2b57c`, correctif `9e3b01e`, réception `ddf7606`. Codex interrompu pendant P5, arbre
propre : **reprise à chaud, étape complétée**, rien à annuler. Fil de Codex transmis
par l'utilisateur.

**Capacité reçue, maillons 0.** Un domaine δ **né à zéro** (I-12) avance sous B/W réels :
`Volume::step_perturbation` accepte désormais le démarrage plat qu'il refusait (A283).
Cause isolée par une projection f64 indépendante : ~98 % de la vitesse prédite est
retranchée, l'erreur f32 de la soustraction domine le reste. ADR-150 : **un** affinage
de divergence sur la vitesse corrigée, seulement au plancher refusé ; tolérance S199,
source S et plafonds inchangés. Consommé par l'API publique et le banc `--flat`.

**Preuves.** Cinq tests (oracle, API, reprise au bit à cinq coupures, couvercle non
réappliqué, zéro allocation). Vitesse contre oracle 9,8·10⁻⁷ / 2,1·10⁻⁶ relatif,
D ≤5,8·10⁻⁸ ; vingt pas, D max 9,93·10⁻⁶. Suite **449 / 16 ignorés / 0**, rejouée à la
clôture ; empreinte 0xfb12b2092df4ee6d. [Preuve](../../docs/validation/DEMARRAGE-PLAT-S251.md).
+4·nx·nz octets par volume.

**Défaut trouvé à la clôture, A284.** Témoin 32×16 sous hauteur imposée, jamais mesuré :
1,09 ms, 69–73 itérations. Plat : 43–44 ms, 581–672 itérations sur 17 pas — **les pas
non affinés coûtent autant**, ce n'est pas ADR-150. Trace temporaire retirée. Toutes
les projections repartent de p=0 ; répartition ordinaire/repli non mesurée. Une
affirmation erronée (hiérarchie vide à 16×8, en fait un niveau) corrigée avant commit.

**Non-fait et suite.** Résidus de surface et frontières du total avant couplage mobile
(A50) ; A284 à attribuer d'abord, une étape, puisque le banc du lot suivant passe par ce
régime. Troisième session du raccordement : comparée à la file — c'est le chemin de J2
vers δ couplé mobile et B4, aucune autre capacité absente n'est débloquée plus tôt ;
V, bathymétrie, A282, A276/A281 et multiplateforme gardent leurs déclencheurs.
Recommandation BILAN-S227 portée (A266 close S228 ; J2 suivi selon §5). Aucun arbitrage.

**Rituel.** File active relue entière ; A283 close, A284 ouverte, feuille de route, index,
note datée S250, plan et jeton. I-05/06/08/12/14 relus : budget mural partagé et refus
atomique, zéro allocation testé, champs f32 et oracle f64 en test seulement, coût de création signalé (A284),
aucune valeur physique nouvelle. Aucun worktree ni distant touché.

## S252 — Le β du gradient conjugué multigrille ; la surface couplée reportée

**2026-09-16, Claude Opus 5.** Entrée « continue », master propre 97868aa, copie unique, archive
B conservée. Plan `e4b023a`, attribution `7c5c5a3`, plan amendé `ba7d11e`, correction `577ec29`,
re-mesure `f261dd5`, ADR-151 `002feb3`, preuve `cce58ba`.

**Plan amendé en cours de session, avant le travail qui en dépendait.** Déclaré : A284 puis la
surface mobile couplée. En attribuant A284, P2 a trouvé un défaut du cœur, A285, qui faussait
S245, S246 et la prémisse d'ADR-147. Correction prioritaire ; la surface couplée passe à S253.

**Capacité reçue, maillons 0.** Correction d'intégrité reproduite puis testée. Le gradient
conjugué multigrille formait `β = ‖r₊‖²/⟨r,z⟩` avant le cycle, depuis S245. Corrigé, il converge
en 6 à 8 itérations aux cinq tailles, au lieu de 94 à 177. Consommateurs : le repli de `run` et
celui du pas couplé. Le démarrage plat 32×16 passe de 43,6 à 2,53 ms de médiane (A284 close).
À 32 768 mailles, le gradient corrigé s'arrête au plancher avec D = 1,585·10⁻⁵ : A275 se
rouvrait. **ADR-151** étend l'affinage d'ADR-150 au pas à couvercle fixe refusé au plancher, et
fait compter au rapport les itérations de toutes les projections. Pas reçu : 441 itérations,
392 ms, D = 3,56·10⁻⁸ (880 ms avant). Forcée dès le départ, la multigrille reçoit ce pas en 137 ms.

**Preuves.** L'essai `multigrid_conjugate_gradient_keeps_its_recursion_s252` échouait avant
(premier vrai résidu 0,616 contre 1,07·10⁻⁵) et passe après. L'essai de réception à 32 768
mailles tourne en release, avec témoin refusé. Mesures avant/après sur la même machine, secteur
99 % : `delta_precision` identique hors temps, empreinte 0xfb12b2092df4ee6d, imposés couplés
identiques au bit. Suite **451 / 17 ignorés / 0**. Deux essais ajustés au compte cumulé, motif
écrit. [Preuve](../../docs/validation/MULTIGRILLE-BETA-S252.md) ; notes datées ADR-147, ADR-150,
S245, S246, S251 ; L328.

**Limites.** Les comptes et coûts multigrille de S245 et S246 sont invalides, pas leurs taux
stationnaires. L'ordre « ordinaire, puis repli » d'ADR-147 reste appliqué alors que sa prémisse
est réfutée : la multigrille gagne dès 512 mailles. Le mode mobile n'a ni multigrille ni affinage.

**Dérivation pour S253, surface mobile couplée, pas encore dans un fichier.** Garder `eta` =
repos + η' (perturbation, née à zéro) et une géométrie mouillée totale ζ = η' + ζ_fond, avec
ζ_fond lu dans `eta` de tout échantillon. Valeur fantôme : p'(Γ) = ρg(z_Γ − repos) − P_fond(Γ).
En vertical, prendre P_fond et ∂zP à la face w au-dessus de la maille ; en latéral, P_fond et
∂xP à la face u. Cinématique : pour un fond **linéaire** (ζ_t = W(0)) prolongé de façon
incompressible, R_s = ∂x∫₀^ζ U dz exactement, donc η'^{n+1} = η' − (dt/dx)Δ[Q_v(ζ) + bande] avec
bande = Σ ouverture·U_face·dx·(mouillé_k(ζ) − mouillé_k(repos)). Pas besoin de ζ_t ni
d'échantillon de colonne. Garder la somme S237 au bit et ajouter la bande à part, pour l'identité
à fond nul. B refuse z > 0 (ADR-113) : fournisseur de test prolongé par Taylor d'ordre un,
champ et dérivées cohérents. Oracle : onde stationnaire S237 (L = h = 2 m) depuis le repos,
fond = ordre un de profondeur finie, comparaison à HOS M=3 (profil, b₂). Témoin sans résidus.
Démarrage plat mobile : risque A283, ADR-150 ne couvrant pas les lignes à fantôme.

**Suite et arbitrages.** Recommandation BILAN-S227 portée (J2 suivi selon §5). Quatrième session
du raccordement B/W→δ en comptant S253 : la surface couplée reste le chemin de J2 et de B4, et
aucune autre capacité absente de la file n'est débloquée plus tôt. Le lot de coût δ (ordre
d'ADR-147) a son déclencheur : avant toute mesure de coût δ ≥ 2 048 mailles ou le passage à la
3D. Aucun arbitrage utilisateur, aucune réduction.

**Rituel.** File active relue ; A284 close, A285 ouverte et close, A276, A281, J2 et A50 mis à
jour ; feuille de route, index, L328, jeton. Invariants relus : I-05 (l'affinage consomme le même
budget mural, refus atomique), I-06 (même fonction d'affinage, zéro allocation testée en S251,
tampon préalloué), I-08 (f32), I-14 (aucune valeur nouvelle, seuils S199 inchangés). Aucun
worktree ni distant touché.

## S253 — La surface mobile couplée au fond B/W, reçue contre HOS

**2026-09-16, Claude Opus 5.** Entrée « continue », master propre 7e3e1d8, copie unique, archive B
conservée. Plan `57cfad3`, ADR-152 et protocole `0bbf607`, fond d'oracle `6f6a35a`, géométrie
`e7d7ca6`, pas `373efe8`, essais `f3091e5`, banc `2df8b71`, plan amendé `d1d0847`, ADR-153
`a9aa4b8`, réception `898b21f`, preuve `94787b1`.

**Capacité reçue, maillons 0.** Un domaine δ **né à zéro** sous un fond B/W linéaire suit une surface
géométriquement mobile : `Volume::step_perturbation_mobile` (ADR-152). `eta` porte `repos + η'` ;
la géométrie est totale (`ζ = η' + ζ_fond`) ; les fantômes reçoivent `ρg(z_Γ − repos) − P_fond(Γ)` ;
la cinématique ajoute la bande `∫₀^ζ U`, qui contient exactement le résidu cinématique d'un fond
linéaire. Contre HOS M=3 (onde stationnaire S237, fond = ordre un analytique), à 128 colonnes :
5 cm profil **0,162 %**, `b₂` **0,34 %** ; 10 cm **0,213 %**, **0,53 %**. Tout est décroissant de
32 à 128, et meilleur que le solveur total, sauf deux cases. Coût à 128 colonnes : 265–277 ms de
médiane, contre 260–262 ms pour le total.

**Deux défauts trouvés en cours de lot, publiés.** (1) Le premier **témoin** éteignait une partie
d'ordre un (`ρg·ζ_fond` des fantômes latéraux) : il divergeait, et « passait » donc le critère pour
une mauvaise raison. Corrigé par une note datée avant les critères 3 et 4 : 99,56 % contre 2,17 %
(L329). (2) **Refus au premier pas** à 64 et 128 colonnes : A283 en mode mobile, la petite vitesse
initiale butant sur le plancher f32. Plan amendé, puis ADR-153 : affinage d'ADR-150 à valeurs
fantômes homogènes. Premier pas à 128 colonnes reçu, `D = 6,7·10⁻⁸`.

**Preuves.** Neuf essais : fond incompressible et linéaire, fantômes contre la pression analytique,
identité au bit à fond nul avec S237, refus, expiration, allocation nulle, témoin, premier pas à
128 colonnes, diagnostic. Suite **459 / 18 ignorés / 0**. `delta_precision` identique hors temps et
octets alloués ; empreinte 0xfb12b2092df4ee6d ; banc S237 identique à 32 colonnes.
[Preuve](../../docs/validation/SURFACE-COUPLEE-S253.md).

**Limites.** Fond d'oracle analytique : B refuse `z > 0` (ADR-113), et le prolongement de Taylor
d'ordre un n'est pas incompressible (A286). Bassin à murs : pas de bords ouverts ni de relaxation
de `η'`. Frontières du total absentes (`W(fond) ≠ 0`, coques). Pas de multigrille en mode mobile
(A276), pas de 3D, pas d'I-05 mural. Le pas S237 total n'a pas l'affinage.

**Suite.** S254 : A286, prolongement incompressible de B/W au-dessus du plan moyen, pour que le pas
couplé consomme la vraie mer ; reçu contre l'oracle S253. Viendront ensuite les bords ouverts
(relaxation de `η'`, éponge) et les frontières du total. Cinquième session du raccordement : c'est
toujours le chemin de J2 (domaines pris dans le système) et de B4. Le lot de coût δ (ordre
d'ADR-147, multigrille en mode mobile) garde son déclencheur. Aucun arbitrage utilisateur.

**Rituel.** File active relue ; A286 ouverte ; J2, A50 et A276 actualisés ; feuille de route,
index, notes datées ADR-149 et ADR-152, L329, jeton. Invariants relus : I-05 (budget et refus
atomiques, affinage compris), I-06 (zéro allocation testée), I-08 (f32), I-12 (naissance à zéro,
démarrage reçu), I-14 (tolérances S237/S199 inchangées, oracle cité). Aucun worktree ni distant
touché.

## S254 — Revue visuelle ouverte ; le fond prolongé au-dessus du plan moyen

**2026-09-16, Claude Opus 5.** Entrée : l'utilisateur reprend le projet **comme superviseur des
rendus visuels**, et fournira à la demande des références réelles et son regard sur la perception
humaine ; « continue ». Master propre d9c6a35, copie unique, archive B conservée. Plan `b9ed890`,
protocole de revue `aa6333e`, R1 `a7ce249`, ADR-154 et protocole `ee84050`, oracle `a92cbce`,
fournisseur `da599e6`, réception `0b39fe6`.

**Revue visuelle ouverte.** Jusqu'ici, aucune réception perceptive n'était possible.
[REVUE-VISUELLE](../../docs/validation/REVUE-VISUELLE.md) fixe ce qu'une revue établit et ce qu'elle
n'établit pas, ce qui est envoyé et ce qui revient. Chaque verdict est classé : physique fausse,
physique juste mais perçue fausse, habillage, hors capacité, artefact. Un verdict déclenche une
mesure et ne la remplace pas. Hôte : `--multi --revue`, sept rendus 1280×720 à poses et âges
fixes, empreintes reproduites. **R1 envoyée, en attente** : questions d'échelle, de premier
défaut, de visibilité des sillages et d'horizon, et cinq références demandées (force 4–5 à 2–7 m
et à 25–30 m, drone sur un bateau à 3 m/s, objet tombant, vidéos). Constat personnel, non soumis
pour ne pas orienter : mer vitreuse (B n'a rien sous 3,5 m), sillages et impacts presque
invisibles une fois ombrés.

**Capacité reçue, maillons 0.** Un domaine δ couplé peut désormais consommer **B de production
sous les crêtes**. Ce qui le permet : `Background::differential_local_extended`, avec ses
variantes monde et lot. Le chemin : `BackgroundFaces` puis `step_perturbation_mobile`. La preuve :
banc HOS et essai d'intégration. Règle d'ADR-154 : vitesse horizontale constante au-dessus du plan
moyen, `W` fermée par continuité, `P` de Taylor d'ordre un, dérivées de ce champ. Elle est
incompressible, sans amplification, et son résidu est d'ordre deux. Oracle S253 rejoué avec ce
prolongement, à 128 colonnes : 5 cm **0,168 % / 0,58 %**, 10 cm **0,244 % / 0,66 %**, décroissants
de 32 à 128. Le fond analytique faisait 0,162 / 0,34 et 0,213 / 0,53 : la règle coûte un facteur
1,2 à 2 sur l'harmonique `2k`, non attribué par mesure. Sous le plan moyen, le fournisseur reste
**identique au bit** ; `differential_local` refuse toujours `z > 0`.

**Preuves.** Contrôles de l'oracle (continuité, divergence, différences finies, `S`, ordre deux),
essais du fournisseur (bit, refus, lot atomique, divergence, différences finies, règle f64) et
intégration : 50 pas reçus, 590 faces mouillées au-dessus du plan moyen consommées. Suite
**463 / 18 ignorés / 0** ; afficheur 16 / 1 / 0. [Preuve](../../docs/validation/PROLONGEMENT-FOND-S254.md).

**Deux incidents, publiés.** (1) Le premier essai du fournisseur contrôlait `du_dt` par différence
avant à `t = 0`, dont la troncature (0,025) dépasse la tolérance : défaut d'instrument. Il passe
désormais par une différence centrée, tolérance inchangée. (2) Sous Windows, la suite complète ne
lie pas `delta_mobile.exe` tant que le banc tourne (LNK1104). Il faut lancer la suite avant ou après
un banc, jamais pendant.

**Limites.** Couches W (impacts, pression) non prolongées. Précision sous B réel non reçue :
l'intégration tourne dans un bassin à murs, et `u'` y vaut 6,6·10⁻² m/s aux murs contre 6,1·10⁻³ à
l'intérieur. Mer large bande non éprouvée contre un oracle multimode. Coût non mesuré.

**Suite.** Dès que les références de l'utilisateur arrivent, la revue R1 passe en tête : c'est la
demande présente, et la seule voie vers une réception perceptive. Sinon, S255 prend les **bords
ouverts** du pas couplé (relaxation de `η'`, éponge) puis les frontières du total. C'est la sixième
session du raccordement, toujours sur le chemin de J2 et de B4 : sans bords ouverts, aucun domaine
n'est plongé dans une mer qui le traverse. Le prolongement des couches W suit le premier domaine
d'impact ou de sillage couplé. Aucun arbitrage utilisateur hors R1.

**Rituel.** File active relue ; A286 partielle ; lignes J2, A50, A282 et B4 perception
actualisées, ligne de revue ajoutée ; feuille de route J1 et J2 ; notes datées ADR-113 et ADR-152 ;
index ; jeton. Invariants relus : I-06 (fournisseur sans collection ; allocation non mesurée),
I-08 (f32, formules f64 seulement dans les essais), I-12 (naissance à zéro inchangée), I-14
(tolérances S237/S253 inchangées, règle citée). Aucune copie ni distant touché.

## S255 — La liste du projet fini

**2026-09-16, Claude Opus 5.** Entrée : demande de l'utilisateur, avant son premier retour visuel.
Il veut un .md en forme de liste de tâches, avec tout ce que le projet fini doit avoir et savoir
faire ; je valide ou non chaque point, et il demandera de temps en temps de la remplir. Master
propre 97129a4, copie unique. Plan `08d522f`, liste `b3c539d`, liens `a6e0c5a`.

**Fait.** [LISTE-PROJET-FINI](../../docs/LISTE-PROJET-FINI.md) : **119 points** en treize sections,
tirés de l'ambition complète et non de l'état construit. Sources : architecture globale, zones
ouvertes, couches d'ADR-001, invariants, SPEC-002 à 006, 23 cas, 11 bancs. États, recomptés par
script : **3 validés** (cœur sans dépendance, vidange de V par C12, aucun état δ sérialisé),
**46 partiels**, **70 absents**. « Validé » exige une preuve sur le périmètre final du point : un
banc isolé, un véhicule d'essai ou une scène unique ne suffisent pas. Sections sans aucun point
construit : phénomènes secondaires (écume, spray, bulles, air, glace, traversabilité, audio). La
couverture a été contrôlée contre les sources ; le changement de solveur manquait et a été ajouté.

**Rôle.** La liste coche et pointe. La trajectoire reste dans la feuille de route, et les preuves
dans les documents liés (L137). Elle est reliée à l'index, à METHODE et à la feuille de route.

**Limites.** Les états viennent de la feuille de route, de la file active et d'un sondage du code,
pas d'une réexécution. Pour C18, l'exécution sur le système reste à relire. Le décompte ne mesure
pas l'avancement.

**Maillons 1** : aucune capacité reçue, travail documentaire demandé. **Suite** inchangée : R1 dès
réception des références, sinon bords ouverts du pas couplé.

## S256 — « La mer est trop lisse, on dirait un lac » : mesuré, puis corrigé en physique

**2026-09-16, Claude Opus 5.** Entrée : premier verdict de l'utilisateur sur R1. Master propre
029cfd5, copie unique. Plan `fbd4324`, verdict et références `bcb2a25`, mesure `cb614b6`, ADR-155
`deec6da`, cœur `85bc3f9`, hôte `fd7dbd5`, réception et R2 `65c7d12`.

**Verdict confirmé par mesure, avant tout changement.** Critère écrit avant de mesurer. La pente
quadratique moyenne de B (recette J1, bande `[0,5 ; 4] fp`) vaut **0,0075** ; Cox & Munk (1954), au
vent minimal qui soutient `Hs` 1,5 m selon Pierson–Moskowitz, donnent **0,044**. La mer rendue est
5,8 fois moins rugueuse que la vraie. Classe : physique juste mais incomplète (spectre coupé à
3,5 m). Formules citées dans SPEC-001 §1 sexies. Inattendu : même prolongée jusqu'aux capillaires, la
queue JONSWAP ne donne que 52 % de l'observé.

**Capacité reçue, maillons 0.** Le rendu porte la **rugosité de la queue du même spectre** : c'est ce
qui devient possible. Le chemin : `background_spectrum::bake_tail`, `[4 ; 32] fp`, 64 composantes à
densité absolue, puis le fragment de l'hôte, en pentes par pixel filtrées par l'empreinte. La
preuve : ADR-155 et [QUEUE-SPECTRALE-S256](../../docs/validation/QUEUE-SPECTRALE-S256.md). `mss` 0,0195
(contre 0,01953 attendu), GPU contre CPU 1,2·10⁻⁴ (tolérance 2·10⁻⁴), coût +0,38 ms à 1280×720
(GPU eau 1,82 ms). La hauteur et les requêtes ne changent pas : `--spectral-verify` est identique à
S249, et R1 se rejoue au bit avec `--no-tail`. Suite **464 / 18 / 0**, afficheur 16 / 1 / 0.

**Trois incidents, publiés.** (1) Une compilation échouée a laissé lancer l'ancien binaire en mode
fenêtre, bloqué dix minutes, puis fermé. Désormais, l'exécutable n'est lancé que si la compilation
réussit. (2) Le critère 7 du protocole était faux (`π/k_max` au lieu de `π/k_min`) : note datée,
tolérance inchangée. (3) Sa première exécution lisait `k_max` avant la mise à jour des composantes :
elle ne contrôlait rien, c'est corrigé.

**Limites.** Rugosité à 52 % de l'observé au mieux. Stries parallèles, parce que la direction d'une
composante suit son rang de fréquence (fixture d'ADR-100) : A287. Pas de capillaires, pas de queue
pour les perturbations W, pas d'écume à `Hs` 1,5 m (ADR-014).

**Suite.** R2 est envoyée. Le verdict de l'utilisateur tranche entre poursuivre sur B (A287 :
étalement directionnel, spectre court) ou passer à un autre défaut perçu. Sans retour, S257 reprend
les bords ouverts du pas couplé. Liste du projet fini : 2.1 et 8.10 mis à jour, 8.9 passe de absent
à partiel.

**Rituel.** A287 ouverte ; file active (revue R1–R2, A287), feuille de route J1, liste du projet
fini, index (ADR-155, preuve), jeton. Invariants relus : I-13 (la queue ne pilote rien, rendu
seulement), I-14 (Cox–Munk et Pierson–Moskowitz cités), I-06 (tableau de queue fixe, `update` sans
allocation au banc), I-03 (cuisson reproductible, empreinte de la bande inchangée).

## S257 — « Un grand lac soumis au vent » : classé par construction ; la surface plane vue de dessus

**2026-09-16, Claude Opus 5.** Entrée : verdict R2 de l'utilisateur et une question. Le résultat se
raffine, mais le rendu paraît un grand lac sous un vent fort ; la haute mer est plus chaotique, et
la houle se forme vers les terres. Avait-on pensé à une surface plane à normales vue de dessus ?
Master propre d6a4253. Plan `726ff43`, classement et réponse `dc17d88`.

**Verdict classé sans campagne**, parce qu'il se calcule ([REVUE-VISUELLE](../../docs/validation/REVUE-VISUELLE.md)
§8). B est **une seule mer de vent**, pleinement développée à environ 8,4 m/s : `Tp` 6 s, `λp` 56 m,
cambrure 0,027, rien au-delà de 12 s. Il n'y a ni houle longue (225 m à 12 s), ni mer croisée, ni
déplacement horizontal des crêtes (absent du code), ni bathymétrie, ni écume. Ce que l'utilisateur
décrit est exactement une mer de vent locale : physique juste mais incomplète. L'ampleur du
« chaotique » n'est pas mesurée, faute de référence.

**Réponse à la question** (§9). Le dépôt l'avait envisagé : ADR-004 prévoyait tuiles de
déplacement et FFT haute fréquence, S234 avait mesuré qu'alléger le maillage ne gagne rien en vue
haute, et ADR-155 rend déjà les ondes courtes en normales seules. Critère chiffré : un déplacement
vertical `h` se voit avec un décalage de `h·sin2θ/(2Hα)` pixels, nul à la verticale exacte. Pour
cette mer, cela donne 1,2 à 3,4 px à 90 m d'altitude, 3 à 16 px aux vues de jeu, et moins de 0,1 px
pour la queue. Une surface plane à normales suffit donc de haut, pas aux vues de jeu. Deux réserves :
le déplacement horizontal doit passer dans les normales, et le décalage mouvant reste à juger à
l'œil. C'est un LOD dépendant de la vue, non prioritaire pour le coût aujourd'hui (le GPU va
surtout à la cuisson du sillage).

**Maillons 1** : aucune capacité reçue ; classement et réponse. **Suite** : S258, mer multimodale dans
B (houle longue + mer de vent, directions par système, ce qui traite aussi les stries d'A287),
réception par système, revue R3. Puis crêtes non linéaires, écume, levée bathymétrique (J5). Les
bords ouverts du pas couplé gardent leur déclencheur dans la file.

**Rituel.** File : revue R1–R2 actualisée ; mer multimodale, crêtes non linéaires et LOD
déplacement/normales ajoutés ; liste du projet fini 2.2 et 8.2 ; jeton. Invariants relus : I-13
(le LOD de vue reste un choix de rendu), I-14 (critère dérivé de la géométrie de projection, sans
constante nouvelle). Aucun ADR, aucun code.

## S258 — La tâche laissée ouverte : C18 vérifié sur le système

**2026-09-16, Claude Opus 5.** Entrée : « fais attention, il reste une tâche en cours ». Constat :
dépôt propre, aucune étape interrompue, aucun processus de la session. Seule tâche ouverte : la
liste du projet fini notait « C18 à relire » (13.2), et je l'avais annoncé à l'utilisateur. Plan
seul, puis vérification.

**Fait.** `water-harness check scenarios/C18-invariants.toml` : `OK`, empreinte `0x85c8bc610f551d11`
(identique à S178), allocation refusée après scellement comptée à 1, 13,8 ms. Lu contre
CAS-CANONIQUES §C18 : **4 lignes tenues sur 7**. Les lignes non exécutées sont le budget par
domaine, l'hôte serveur (vide par construction, S30), les forces avec et sans δ, `W_rep` entre
profils, et les capacités lues d'un profil. L'empreinte n'est comparée que sur une machine. Liste
13.2 corrigée : C18 partiel, lignes nommées.

**Maillons 2** : aucune capacité reçue. Justification : deux sessions de revue et une de
vérification, à la demande de l'utilisateur. La prochaine session construit : S259, mer
multimodale dans B, puis revue R3.

## S259 — Une mer à plusieurs systèmes : houle longue, mer de vent, étalement directionnel

**2026-09-16, Claude Opus 5.** Entrée : « continue ». Master propre fdc0f1c, maillons 2. Plan
`e6f1185`, références et ADR-156 `90d2698`, cœur `27368ba`, hôte `498e078`, réception et R3 `5809943`.

**Capacité reçue, maillons 0.** Ce qui devient possible : B peut réunir **plusieurs systèmes de
vagues**, chacun avec son spectre et sa **loi d'étalement directionnel**. Le chemin :
`bake_directional` et `assemble`, puis `Background::from_spectrum`, puis l'hôte, scène `--houle`
(mer de vent S201 et houle de 12 s, 225 m, à 43°). La preuve est
[MER-MULTIMODALE-S259](../../docs/validation/MER-MULTIMODALE-S259.md) :
- spectre de chaque système identique au bit ;
- inverse de `cos^2s` contre `s/(s+1)` à 10⁻⁵ près ;
- Spearman rang/direction 0,012, contre 1,000 pour la fixture ;
- `Hs` 2,5 m exact ;
- GPU contre cœur à 0,379 mm, pour +0,03 à 0,07 ms de GPU.

**Aucune migration silencieuse** : l'empreinte V1, les huit essais du spectre, R2 (sept empreintes)
et `--tail-verify` sont identiques. Suite **465 / 18 / 0**, afficheur 16 / 1 / 0.

**Incidents.** L'exponentielle sans libm du cœur (`decay`) n'accepte que 0..=32 : premier passage en
dépassement, densité désormais coupée au-delà de `e⁻³²`. Le registre de la revue avait la ligne R2
hors de son tableau : elle y est replacée, avec R3.

**Limites.** Houle et mer de vent sont une **fixture déclarée**, non calibrée. L'étalement est gelé
au-delà de la bande pour la queue, par convention non reçue. Rugosité toujours à 52 % de
l'observé. Les maxima GPU de `--houle` (2,15 et 2,63 ms) ne sont pas attribués. Crêtes linéaires, pas
d'écume, pas de levée sur la bathymétrie.

**Suite.** Verdict R3 de l'utilisateur. S'il juge la mer « haute mer », l'adoption par défaut devient
une décision explicite, avec des réceptions à rejouer. Sinon, correction de la fixture. Ensuite :
crêtes non linéaires, puis écume. Bords ouverts du pas couplé : déclencheur inchangé.

**Rituel.** A287 partielle ; file active (mer multimodale, A287, revue) ; feuille de route J1 ; liste
du projet fini 2.1 et 2.2 (partiel) ; index ; jeton. Invariants relus : I-03 (cuissons
reproductibles, empreintes nouvelles seulement pour la variante), I-09 (systèmes paramétrés, aucune
réalisation interpolée), I-14 (Mitsuyasu, Goda et Longuet-Higgins cités dans SPEC-001 §1 septies).

## S260 — « Pas assez de mini pics » : pentes gaussiennes mesurées, queue d'équilibre et vagues pointues

**2026-09-17, Claude Opus 5.** Entrée : verdict R3 et **deux premières références photographiques** —
« trop lisse, trop de petites bosses, pas assez de mini pics ; pics moyens et petits, rien n'est
uniforme ». Master propre 1b16986. Plan `797ff14`, verdict et Cox–Munk `baa10fe`, mesure `2309b4c`,
ADR-157 `fe862ce`, cœur `d3eb5f7`, hôte `3caea81`, réception et R4 à la suite.

**Mesuré avant de corriger.** Verdict traduit en statistiques de Cox & Munk (Gram-Charlier) et en
asymétrie du second ordre, critères écrits avant mesure, 10⁶ points. Le rendu a des **pentes
gaussiennes** (`c40` −0,026 contre 0,40) et des crêtes symétriques (`λ3` 0,000 contre ≈ 0,16).
Hypothèses confirmées. **Le remède a été choisi par le calcul** parmi sept candidats, sans en
construire aucun :
- CWM seul : `c40` 0,06 ;
- harmoniques par composante : effet nul ;
- queue en `f⁻⁴` seule : `mss` 0,048 ;
- **`f⁻⁴` + CWM : `mss` 0,0495, et `c40`, `c22`, `c04` à 0,21, 0,07 et 0,21, dans les incertitudes,
  sans ajustement** ;
- modulation `M` ajustée : écartée ; au-delà de 10, replis.

**Capacité reçue, maillons 0.** Le rendu porte une mer **à pentes non gaussiennes et crêtes
resserrées** : queue d'équilibre du cœur (`bake_tail_equilibrium`), puis hôte `--vagues` (sommets
déplacés de `D_B`, normales par `J⁻ᵀ`). Preuve :
[VAGUES-POINTUES-S260](../../docs/validation/VAGUES-POINTUES-S260.md) — queue contre f64 à 10⁻⁶, GPU contre
CPU 1,6 µm et 4,15·10⁻⁴, aucun repli, R2 et R3 identiques au bit. Suite **466 / 18 / 0**, afficheur
16 / 1 / 0.

**Limites, publiées.** (1) **Écart au jeu de 0,365 m** sous `--vagues` : l'ADR annonçait un ordre `k·a²`,
l'amplitude vient de la houle (note datée, **A288**). (2) **GPU eau 2,15–2,18 ms** à 1280×720 :
dépassement de l'implémentation, la bande étant parcourue deux fois par sommet. (3) Pas
d'anisotropie des ondes courtes, pas d'asymétrie des pentes ni de l'élévation. (4) Le ciel et la
couleur (habillage) pèsent probablement dans l'écart avec la référence A.

**Suite.** Verdict R4. Selon lui : habillage (ciel et couleur, étiquetés), fusion des boucles
(coût), anisotropie de la queue, requête eulérienne CWM (A288) avant tout usage de jeu. Les bords
ouverts du pas couplé gardent leur déclencheur.

**Rituel.** A287 actualisée, A288 ouverte ; file active (revue, A287, crêtes/A288) ; feuille de
route J1 ; liste 2.1 ; index (ADR-157, preuve) ; jeton. Invariants relus : I-13 (CWM rendu seulement,
requêtes inchangées), I-14 (Toba, Phillips, Donelan et al., Nouguier et al. et Cox–Munk cités ;
aucun coefficient ajusté), I-03 (cuissons reproductibles, scènes antérieures au bit).

## S261 — Ciel de la référence ; rugosité ajustée à Cox–Munk, et ce que Cox–Munk interdit

**2026-09-17, Claude Opus 5.** Entrée : « change le ciel et la couleur comme sur ma photo ; la mer a
l'air trop rugueuse, la surface entre les pics est plutôt lisse, mais il y a beaucoup de petites
vaguelettes ». Master propre a69259f. Plan `83e52ac`, verdict `4bd7923`, habillage `c44f66b`, critère
`5459d66`, mesure et ADR-158 `f8b49a8`, construction `1a8f849`, réception et R5 à la suite.

**Habillage.** « Ciel clair », d'après la référence A : ciel bleu profond dégradé, nuages blancs
procéduraux, eau bleu profond, air clair. Sélectionnable (`--ciel-clair`), étiqueté habillage, sans
physique ; la brume reste le défaut, et R2 à R4 se rejouent au bit. Premier jet corrigé : colonnes
de nuages à l'horizon, ciel trop pâle. Aperçu envoyé avant la suite. Coût : jusqu'à +0,11 ms (nuages
dans les reflets).

**Rugosité, mesurée avant d'agir.** Critère écrit avant mesure, 18 candidats. Retenu : coupure de la
queue à 28 fp et modulation des ondes courtes par la bande, `M` = 2 — **ajustements déclarés** contre
Cox–Munk. Résultat : `mss` 0,0435 (contre 0,0437) ; `c40`, `c22`, `c04` à 0,353, 0,129 et 0,351 ; score
de 1,31 à 0,15. **Constat qui compte** : Cox–Munk borne la modulation. À `M` = 2, 0,3 % de la surface
seulement devient « lisse », et une surface franchement lisse entre les pics dépasserait la pointe
observée à 8 m/s. Le verdict s'expliquerait sans contredire l'observation par un **vent plus faible**
(Cox–Munk à 4 m/s : `mss` 0,023) ou par le **grain** des ondes proches de la résolution (transition
vers la BRDF). Les deux sont nommés, aucun n'est construit.

**Capacité reçue, maillons 0.** Le rendu porte une mer dont la rugosité et la pointe des pentes sont
**calées sur les valeurs centrales de Cox–Munk**, sous l'habillage demandé. Le chemin : `--vagues
--modulation --ciel-clair`. La preuve : [RUGOSITE-S261](../../docs/validation/RUGOSITE-S261.md) — GPU contre
CPU 2,82·10⁻⁴, aucun repli, 60 lignes de queue comptées analytiquement, scènes antérieures au bit.
Suite **466 / 18 / 0**, afficheur 16 / 1 / 0.

**Limites.** GPU eau 2,24–2,26 ms, au-dessus du budget (dépassement de l'implémentation). Grain proche.
Ligne d'horizon à la fin de la grille, révélée par l'air clair. A288 ouverte (écart au jeu 0,365 m).

**Suite.** Verdict R5. Selon lui : vent de la scène comme paramètre (`Hs`, `Tp`, rugosité liés à `W`),
transition vers la BRDF, fin de grille à l'horizon, fusion des boucles (coût), A288 avant tout usage
de jeu.

**Rituel.** A287 actualisée ; file active (revue, A287) ; feuille de route J1 ; liste 2.1 ; index
(ADR-158, preuve) ; jeton. Invariants relus : I-13 (habillage et modulation au rendu seulement), I-14
(ajustements déclarés contre Cox–Munk, critère écrit avant mesure ; couleurs d'habillage relevées
sur la référence, hors physique).

## S262 — Défauts réparés avant le visuel : horizon, coût GPU, requête de jeu sous CWM

**2026-09-17, Claude Opus 5.** Entrée : « répare d'abord les défauts restants avant de peaufiner le
visuel ». Master propre 1e89da7. Plan `648d74e`, horizon `2734a11`, coût `4c31ddc`, ADR-159 `45ff37e`,
cœur `7b19e4c`, réception A288 à la suite. Verdict R5 toujours attendu.

**Horizon.** La grille projetée s'arrêtait à 1 500 m, et l'air clair le montrait. La distance
lointaine est devenue un paramètre : 1 500 m sous la brume, au bit (R2 à R4 et `--spectral-verify`
identiques) ; horizon géométrique `√(2·R·h)` sous le ciel clair (9,4 km à 7 m), avec raccord à la
couleur d'horizon.

**Coût.** Décomposé avant d'agir : la cuisson de la grille du sillage pesait 1,27 ms sur 2,28, la
queue par pixel 0,49. Réparations sans changement d'image au-delà de l'arrondi :
- bande de chaque mode de sillage et invariants de la queue précalculés par l'hôte ;
- une seule boucle de bande par sommet CWM ;
- total de la grille pris comme somme de ses bandes ;
- nuages des reflets à deux octaves (habillage).

Résultat : **1,54–1,60 ms en 960×540, 1,98–2,01 ms en 1280×720** (avant 2,28), cuisson 1,06–1,10 ms.
Les images changent de 0 à 27 octets sur 2,76 millions, au plus 8 niveaux sur des reflets. Les
vérifications CPU/GPU sont identiques, ou changent à la 7e décimale sur le chemin par grille ; en
multi-sources, hauteurs à 0,368 mm comme en S235.

**Capacité reçue, maillons 0.** Un consommateur de jeu peut désormais interroger **la surface
affichée sous CWM** : `Background::cwm_query` inverse le déplacement par Newton (ADR-159), et W se
compose au point de Lagrange. Preuve : [DEFAUTS-S262](../../docs/validation/DEFAUTS-S262.md) — mer de la
scène `η` à 3,8·10⁻⁵ m, au plus 3 itérations, replis refusés. **Contre l'image GPU : 0,30 mm, contre
0,365 m** pour la requête linéaire. A288 close. Suite **469 / 18 / 0**, afficheur 16 / 1 / 0.

**Incident.** Un rendu de contrôle a réécrit les images locales de R5 (non versionnées, reproductibles
au commit 1e89da7). Chaque revue a désormais son dossier.

**Limites.** 1280×720 reste à la limite des 2 ms, et le LOD temporel de la cuisson (1,07 ms) est la
technique absente la plus rentable. La requête CWM n'est branchée à aucun consommateur réel. Au loin,
l'écart image/requête est celui du filtre d'ADR-148.

**Suite.** Verdict R5 de l'utilisateur, puis les finitions visuelles demandées : vent de la scène
comme paramètre, transition vers la BRDF contre le grain proche.

**Rituel.** A288 close ; file active (crêtes et coût, revue) ; feuille de route J1 ; liste 8.8, 9.11
et 10.9 ; index (ADR-159, preuve) ; jeton. Invariants relus : I-06 (requête sans collection, boucle
d'image inchangée), I-13 (requête du cœur, image sans autorité), I-03 (phases entières de B dans la
requête ; images changées d'un arrondi, empreintes nouvelles expliquées).

## S263 — « Trop rugueuse, vent inconnu » : le vent devient un paramètre de scène

**2026-09-17, Claude Opus 5.** Entrée : verdict R5, « trop rugueuse, trop de petits pics, je ne connais
pas le niveau de vent ». Master propre 7a88546. Plan `caacf56`, ADR-160 `1a18310`, construction
`211aef6`, calibration R6 à la suite.

**Classement.** La rugosité rendue était celle de Cox–Munk **à 8,4 m/s**, le vent de la mer de la
scène (S261). Le verdict ne contredit pas la physique : la mer attendue est moins ventée. Dans une mer
pleinement développée, `Hs`, `Tp` et `mss` découlent du vent : rien ne se règle à part.

**Capacité reçue, maillons 0.** La scène se construit **pour un vent donné** : mer de vent de
Pierson–Moskowitz, queue d'équilibre coupée à la `mss` de Cox–Munk au même vent et à la limite
capillaire, houle, modulation et CWM inchangés. Le chemin : `--vent=U` ; fonctions du cœur partagées
par l'hôte et l'instrument. La preuve : [VENT-S263](../../docs/validation/VENT-S263.md) — `mss` rendue
0,0184, 0,0295 et 0,0457 contre 0,0184, 0,0286 et 0,0459 à 3, 5 et 8,37 m/s ; aucun repli ; requête
de jeu à 0,29 mm de l'image sous 5 m/s ; sans `--vent`, rendus identiques au bit à S262.

**Constats.** `c40` décroît avec le vent (0,12 à 3 m/s, sous la borne de Cox–Munk) : `M` = 2 est un
ajustement à 8 m/s. À 8,37 m/s, la limite de cuisson (32 fp) arrête la queue à 2,8 % sous Cox–Munk.

**Calibration perceptive R6** envoyée : même scène à 3, 5 et 8,37 m/s, trois poses. L'utilisateur, qui
ne connaît pas le vent de sa référence, choisit à l'œil. Le vent choisi deviendra celui de la scène
représentative, et c'est un fait de revue.

**Suite.** Choix du vent (R6). Puis, selon le verdict : transition des ondes non résolues vers la
BRDF (grain, « petits pics » restants), dépendance de `M` au vent, anisotropie des ondes courtes.

**Rituel.** A287 actualisée ; file active (revue, A287) ; feuille de route J1 ; liste 2.1 ; index
(ADR-160, preuve) ; jeton. Invariants relus : I-14 (Pierson–Moskowitz et Cox–Munk cités ; vent choisi
par revue, déclaré), I-03 (scènes sans vent au bit).

## S264 — Reprise vérifiée, calibration R6 disponible

**2026-09-17, Codex GPT-6.** Entrée : « reprends le projet ». Master propre afcf927,
une seule copie de travail, branche historique B conservée, jeton libre. Plan 1d8538b,
vérifications et protocole R6 7c6dcf7.

**Vérifié :** les 21 empreintes des PPM S263 correspondent au journal local, les neuf aperçus PNG
ont exactement les mêmes pixels. Test ciblé `cargo test -p water-core s263` : **1 réussi, 0 échec**.
Commandes, pose, couches et limites des images de référence consignées dans REVUE-VISUELLE §13.
Pas de changement du code, pas de nouvelle exécution GPU ni de réception multiplateforme.
Les suites complètes restent celles consignées en S263, sans être revendiquées comme rejouées.

**Choix demandé :** vent de 3, 5 ou 8,37 m/s ; aucun verdict nouveau reçu pendant la reprise.
La poursuite du visuel dépend de ce choix selon ADR-160 ; ni BRDF ni modulation supplémentaire
construites. Face à ce fil visuel prolongé, les bords ouverts du couplage B/W→δ (J2) restent un
lot de capacité distinct et utile ; aucune campagne visuelle supplémentaire engagée en attente.

**Rituel :** toute la file active relue, revue R6 actualisée, invariants I-03/I-13/I-14 relus ;
aucune capacité ni angle mort nouveau, feuille de route inchangée. Maillons **1**.
Prochaine session : recueillir le verdict R6 puis choisir le lot de construction qu'il justifie ;
δ, V, B2, bathymétrie et multiplateforme conservent leurs déclencheurs dans la file active.
Jeton libéré ; aucune copie isolée à fermer.

## S265 — Lisser le détail des reflets entre les pics

**2026-09-17, Codex GPT-6.** Verdict R6 : surface trop rugueuse entre pics moyens et petits ;
aucun choix de vent. Master propre 34dcf7d, copie unique. Plan bf87e5e, décision 648ac05,
construction 61c917f, réception aa2b1a6.

**Capacité reçue, maillons 0 :** l'hôte peut filtrer les reflets des petites ondes sans supprimer
leur variance de pente du modèle d'éclairage. Chemin consommateur : fenêtre et captures sous
`--vagues --reflets-filtres`, fermeture gaussienne ADR-161, spectre et géométrie inchangés.
Preuve : [REFLETS-S265](../../docs/validation/REFLETS-S265.md), covariance contre phases intégrées et
échantillons transformés, 15 000 sondes GPU aux trois vents (pente ≤9,32e-5, covariance ≤7,43e-7,
aucun repli), sept témoins R6 à 5 m/s identiques au bit. Requête CWM 0,288 mm, zéro refus.
Tests hôte **18 réussis / 1 ignoré / 0 échec**. Cœur inchangé, pas de suite complète cœur rejouée.

**R7** : avant/après à 5 m/s, vent témoin uniquement. Le contraste fin (RMS laplacien d'un
rectangle d'eau fixé) baisse de 61 % en référence et 66 % en rasante ; ce n'est pas une mesure
de rugosité physique ni un verdict utilisateur. Images R7 disponibles dans `viewer/captures/s265`.

**Limites :** fermeture au premier ordre sous CWM, sans masquage microfacette ni réception
radiométrique ; variance de la bande géométrique non transférée. Quadrature 3×3/5×5 différente,
notamment en vue haute. GPU eau **2,49–2,71 ms** contre 1,80–1,83, RTX 5070 Laptop DX12, secteur,
budget 2 ms non tenu ; `update` sans allocation. Variante optionnelle, aucun défaut historique
réglé silencieusement. Le visuel reste à juger, le mouvement/scintillement non reçu.

**Suite :** verdict R7 ; s'il convient, intégration analytique/précalculée pour diminuer coût et
erreur de quadrature ; sinon mesurer le détail résolu avec une référence réelle. Aucun
approfondissement automatique de M. Les bords ouverts J2 restent le lot indépendant comparé
au fil visuel ; celui-ci était explicitement demandé dans cette session.

**Rituel :** A287 actualisée, file active entière relue, J1-bis, SPEC-001 §1 nonies, index et
registre de revue actualisés. I-03 : témoin inchangé localement, pas de réception multiplateforme ;
I-06 : buffers réutilisés et `update` mesuré ; I-13 : éclairage sans autorité ; I-14 : fermeture,
quadrature et largeur à calibrer déclarées. δ, V, B2, bathymétrie et seconde cible gardent leurs
déclencheurs. Jeton libre, aucune copie isolée à fermer.

## S266 — R7 accepté, même aspect pour 10 à 17 % de GPU en moins

**2026-09-17–18, Codex GPT-6.** Entrée « Très bien continue », puis « continue ». Aspect R7
accepté dans la vue montrée, pas choix d'un vent ni réception de l'animation. Master propre
c4f9bb9, copie unique. Plan 902dc05, protocole ab231cd, cache e60b8fa, rejet/replanification
d95b46e, optimisation retenue 594dfac.

**Cache rejeté.** ADR-162 : cubemap 512² puis 1024². Critère maximum RGB16 manqué (22 puis19),
gain1024 seulement4–5 % au lieu des10 % requis, pour48 Mio. Code retiré ; expérience et
mesures conservées. Premier coût512 écarté car capture GPU encore concurrente.

**Capacité reçue, maillons0.** Le chemin `--reflets-filtres` conserve l'aspect accepté avec
moins de travail par pixel : covariances des modes entièrement filtrés regroupées, quadrature
3×3 spécialisée (ADR-163). Consommateurs : fenêtre et captures de l'hôte. Preuve :
[CIEL-CACHE-S266](../../docs/validation/CIEL-CACHE-S266.md) — **2,259 / 2,239 ms** contre
2,737 / 2,494, référence/rasante, soit17,48/10,20 %, secteur RTX5070 Laptop DX12 1280×720.
Sept images :2–29 canaux changés sur2 764 800, au plus1/255 ; sept témoins R7 au bit.
15 000 sondes GPU aux trois vents reçues. Tests hôte **19 réussis,1 ignoré,0 échec**.
Update sans allocation ; buffer GPU +1 Kio, temporaire CPU de pile1 Kio. Aucun cache de ciel.

**Limites :** budget2 ms encore dépassé ; gain rasante près du critère, pas de garantie
inter-machine. Aucun gain CPU revendiqué ; quadrature3/5 et approximation CWM inchangées.
Pas de réception d'animation/multiplateforme ni de modification du cœur, dont la suite complète
n'est pas rejouée.

**Suite choisie :** cuisson GPU du sillage, encore1,06–1,08 ms : prochain levier de coût utile
pour approcher2 ms, avec réception de l'erreur et d'I-09 avant cadence réduite. Les reflets ont
leur aspect accepté ; prolonger automatiquement leur calibration n'est plus prioritaire.
Comparé au lot J2 bords ouverts : celui-ci conserve son déclencheur, la demande de poursuivre
le rendu a motivé S266. La prochaine session doit garder cette comparaison explicite.

**Rituel :** file active entière et invariants I-03/I-06/I-13/I-14 relus ; feuille de route,
A287, registre de revue et index actualisés. Ordre de somme modifié documenté, cœur inchangé,
aucune valeur physique recalibrée. δ,V,B2,bathymétrie et seconde cible restent dans la file.
Jeton libéré, aucune copie isolée à fermer.


## S267 — Sillage identique, cuisson GPU réduite de 46 %

**2026-09-18, Codex GPT-6.** Entrée « continue », réitérée pendant la réception.
Master propre c476631, copie unique, plan ac7b0c5, contrat f3c1372 avant code,
construction 8ec6de0, réception bb09783. Comparaison à J2 déclarée avant construction.

**Capacité reçue, maillons 0 :** la fenêtre, les captures et sondes consomment par défaut
une cuisson des huit bandes à accumulateurs explicites, **sans modifier le champ**.
3 514 240 flottants GPU identiques, retours temporels exacts, sept PPM identiques à S266 ;
36 contrôles spectraux conservés (hauteur ≤0,340 mm). Tests hôte 19 réussis, 1 ignoré.
[Preuve et limites](../../docs/validation/CUISSON-SILLAGE-S267.md). Aucun nouvel ADR :
implémentation équivalente d'ADR-148, témoin conservé par option.

**Coût reçu :** cuisson 0,574–0,585 ms contre 1,064–1,086, −46 % ; eau totale ~1,74 ms,
−22 à −23 %, référence/rasante 1280×720, RTX 5070 Laptop DX12, secteur. Deux passages,
critères tenus aux deux. Première pointe 2,962 ms conservée ; second max 1,836 ms.
CPU ~4,1 ms médian, maxima ~26 ms : ni budget global 2 ms ni borne par image reçus.
Update zéro allocation, buffers inchangés, pipeline témoin supplémentaire au démarrage.
Pas de réception multiplateforme, d'animation perceptive, ni de modification du cœur.

**Suite choisie : S268, bords ouverts du pas couplé J2**, relaxation de hauteur et éponge,
puis frontières du total. Le rendu accepté est préservé et son poste GPU dominant traité :
un nouvel affinage cosmétique serait différable devant cette capacité physique absente.
Coût CPU et pointes restent ouverts A265/A278 avec déclencheur après ce lot J2 ; pas
promesse de les attribuer par raisonnement. Aucun arbitrage utilisateur nouveau requis.

**Rituel :** file active entière relue ; J1-bis, A282/A265, index et README actualisés.
I-03 cœur inchangé, I-06 update mesuré, I-09 aucune interpolation temporelle, I-13 rendu
cosmétique, I-14 seuils de rentabilité déclarés. δ, V, B2, bathymétrie et seconde cible
gardent leurs travaux. Jeton libéré, aucune copie isolée à fermer.


## S268 — Relaxation de la hauteur perturbative dans le pas mobile

**2026-09-18, Codex GPT-6.** Entrée « continue », master propre c9fd0d3, copie unique.
Plan 9f17665, contrat de78c26, construction 1f04add, réception 324863f.
Absence constatée : l'éponge S250/S253 amortissait les vitesses mais pas η'.
Lot borné avant code à cette brique, sans déclarer les frontières ouvertes reçues.

**Capacité reçue, maillons 0 :** `step_perturbation_mobile` applique après transport
la relaxation exponentielle de η' vers zéro (ADR-164), fond intact, reste compensé
amorti lui aussi, intérieur et taux nul au bit. Consommateur : pas mobile transactionnel
existant. Preuve : [RELAXATION-SURFACE-S268](../../docs/validation/RELAXATION-SURFACE-S268.md),
référence exponentielle indépendante, 20 pas avec fond non nul, témoin vitesse seule,
607 expirations/reprises exactes et zéro allocation. Suite complète cœur/harnais
**473 réussis, 18 ignorés, 0 échec** ; reçus sans éponge conservés. Rendu inchangé.

**Limites :** dissipation locale, pas absorption globale reçue ; fermeture extérieure
réfléchissante, volume perturbatif non conservé, pas de transduction W. Ni réflexion
<1 %, ni fond traversant, ni frontières du total, ni B4 global acquis. Aucune mesure de
coût ; durée entière, calcul de coefficient ADR-141, I-05 mural reste non reçu.

**Suite S269 :** mesurer la réflexion d'un paquet sortant avec témoin et contrôle de
contamination des fenêtres incident/réfléchi (ADR-046), puis fond traversant. Cette
preuve commande l'usage ouvert du domaine ; la seule décroissance locale ne suffit pas.

**Rituel :** file active entière relue ; J2, A92/A50, index actualisés. Invariants I-04
(non autoritaire), I-05 (transaction testée, pas budget mural), I-06 (zéro allocation),
I-08 (coefficients ADR-141) et I-14 (paramètres d'essai non calibrés) relus. δ général,
V, B2, bathymétrie et multiplateforme restent dans la file. Jeton libre, aucune copie
isolée à fermer ; aucun arbitrage utilisateur nouveau requis.


## S269 — Absorption du paquet sous double garde différentielle

**2026-09-18, Codex GPT-6.** Entrées « continue », master propre 24a4c03, copie unique.
Plan ff8711e, protocole 218e0f3, banc 2e85720, diagnostic 9ea320a, réception 611f90d.
Le banc consomme `step_perturbation_mobile` réel, fond nul, sans changer la bibliothèque.

**Preuve reçue :** [REFLEXION-PAQUET-S269](../../docs/validation/REFLEXION-PAQUET-S269.md).
Douze traces de 7 200 pas ; R_diff éponge 0,00144442 / 0,00161565, écart 0,000171226,
seuils 0,01 / 0,002 tenus ; doubles gardes <0,001, incidents <0,001, murs >0,5.
La première garde brute refuse : queue numérique et retour gauche. L'instrument
corrigé, déclaré avant verdict, soustrait un domaine long au même bord gauche,
puis contrôle un second domaine long. La réception brute reste refusée.
Tests de l'exemple : 6 réussis, 1 ignoré ; analyse Python : 3 réussis. Suite complète
S268 non rejouée, cœur inchangé ; aucun rendu modifié, aucun nouvel ADR.

**Limites et maillons : 1.** Cette session reçoit l'absorption d'un cas du chemin
existant, mais n'ajoute aucune capacité d'exécution : le banc seul ne remet pas
le compteur à zéro. Ni propagation exacte, ni horizon infini, ni autre spectre,
ni fond traversant, ni performance reçus. Les durées de calcul concurrentes ne
valent pas un coût produit. Aucun arbitrage utilisateur nouveau requis.

**Suite S270 : fond traversant et flux de bande aux frontières.** Avant cette
troisième session sur les bords, comparaison à la file : le passage réel de B
reste nécessaire à l'usage perturbatif J2 ; prolonger les seuls tests spectraux
serait différable. Construire/recevoir ce passage prime donc les reflets acceptés
et leur coût CPU. δ général, V, B2, bathymétrie et seconde cible gardent leurs
déclencheurs ; aucune ambition supprimée.

**Rituel :** file active entière relue ; J2, A92/A50, index actualisés. Invariants
I-04, I-05, I-06, I-08 et I-14 relus : banc hors temps réel, constantes de fixture,
aucune modification du contrat produit. Jeton libéré ; aucune copie isolée à fermer.


## S270 — Flux de bande prescrit aux frontières latérales

**2026-09-18, Codex GPT-6.** Entrée « continue », master propre 8e624a1, copie unique.
Plan 385f23e, contrat 54e76be, construction 453b117, réception 17f0d3f.
Le passage du fond J2 reste prioritaire devant les reflets acceptés et leur coût :
la troisième session du fil corrige une condition nécessaire, pas un affinage spectral.

**Capacité reçue, maillons 0 :** le courant analytique uniforme sous niveau uniforme
traverse le domaine sans créer de hauteur parasite. Consommateur : transport réel de
`step_perturbation_mobile`, flux de bande prescrit aux deux faces externes (ADR-165),
condition fermée de v conservée. Déficit ancien reproduit : erreur 0,0002501011 m au
premier pas contre dt Ua/dx=0,00025 m. Après correction : erreur de hauteur nulle,
huit cas, vingt pas, deux mailles. Intégrales signées et bilan global indépendants.
[Preuve](../../docs/validation/FOND-TRAVERSANT-S270.md).

**Validation :** 478 tests réussis, 18 ignorés, aucun échec (366 cœur, 17 intégrations,
95 harnais). 638 expirations avec flux non nul : zéro allocation, restauration et
reprise au bit. Élévation des faces extérieures incohérente ou hors domaine refusée ;
gardes avant/après transport. Identité S253 au fond nul et harmonicité stationnaire
conservées par leurs tests. Aucun champ GPU, dépendance ou état persistant ajouté.

**Limites :** reçu stationnaire et bilan, pas une houle progressive complète. v_n
reste nul au bord ; frontières du total, W(b)≠0, coques, B4 global et budget mural
restent ouverts. Suite S271 : onde progressive traversante de profondeur finie,
référence indépendante et contrôle de contamination avant mesure. Le test ne doit
pas traiter une reconstruction analytique imposée comme une propagation reçue.
Aucun arbitrage utilisateur nouveau requis.

**Rituel :** file active entière relue ; J2, A92/A50 et index actualisés. I-04/I-13
inchangés, I-05 transaction reçue sans promesse murale, I-06 mesuré, I-08 arithmétique
f32 et durées entières, I-14 identités exactes et fixtures explicites. δ général, V,
B2, bathymétrie et seconde cible gardent leurs déclencheurs. Jeton libre, aucune
copie isolée à fermer.


## S271 — Démarrage de houle progressive et liste utilisateur actualisée

**2026-09-18, Codex GPT-6.** Entrée « continue et mes a jour la to do list », master
propre 9155256, copie unique. Plan 6c83b1e, contrat/liste 6c0eb0d, tests cdf1487.
La vérification de fond variable J2 reste nécessaire devant l'affinage différable
des reflets acceptés ; le courant constant S270 ne suffit pas à la recevoir.

**Preuve bornée :** [HOULE-PROGRESSIVE-S271](../../docs/validation/HOULE-PROGRESSIVE-S271.md).
Fond progressif analytique de profondeur finie, source cinématique initiale
W(ζ)−W(0)−U(ζ)ζ_x contrôlée indépendamment de la quadrature. Erreur 3,048 / 1,237 /
0,585 %, décroissante ; témoin fermé 169–338 %. Le pas couplé réel avance et
l'écart au taux initial est divisé par deux en divisant dt par deux. Trois tests
ciblés reçus, aucun échec ; bibliothèque de production inchangée, suite S270 non
rejouée. Aucune évolution temporelle entière ni coût reçus. Aucun nouvel ADR.
**Maillons 1** : instruments et preuve locale, aucune capacité d'exécution ajoutée.

**Liste demandée :** états actualisés S262–S271 : requête CWM/A288 déjà close,
flux de bande, relaxation de hauteur, paquet absorbé, démarrage progressif,
filtre spectral B/sillage, R7 accepté, coût GPU/CPU récent et transaction. Tous
ces points restent partiels sur leur périmètre final. Décompte recalculé depuis
les entrées : 119 points = 3 validés + 48 partiels + 68 absents ; ne mesure pas
le pourcentage d'avancement. Les changements conservent l'ambition complète.

**Suite S272 :** référence temporelle indépendante du résidu progressif d'ordre
deux, puis comparaison sur durée utile avec contrôle des bords. Le fond linéaire
imposé ne constitue pas une réception de propagation du total ; η'=0 n'est pas
l'oracle. Cette étape garde le critère d'arrêt avant toute campagne.

**Rituel :** file active entière relue ; J2, A92/A50, index, liste mis à jour.
I-04/I-05/I-06/I-08 inchangés dans le produit, I-14 oracle dérivé et fixtures
explicites. δ général, V, B2, bathymétrie et seconde cible restent dans la file.
Jeton libre ; aucune copie isolée à fermer, aucun arbitrage utilisateur requis.


## S272 — Résidu temporel refusé, quadrature de bande isolée

**2026-09-18, Codex GPT-6.** Entrée « continue », master propre b92ba0b, copie unique.
Plan 345986b, contrat 6629ce1, instrument 77bbd8c, mesures 11cf3de. Pas de nouvel ADR.
Le reçu temporel J2 manque encore, priorité comparée à V et au coût ; aucun nouveau
lot cosmétique. [Preuve](../../docs/validation/RESIDU-TEMPOREL-S272.md).

**Résultat : refus explicite.** Oracle modal indépendant avec Neumann pour la
perturbation, conditions de surface à l'ordre deux dérivées ; trois tests Python
(projections, RK4, limites des conditions exactes) passent. Les cinq simulations
consomment le pas réel et terminent 6 000 pas. Erreur de η' seul sur 2 s : 22,52 /
13,04 / 8,68 % aux trois mailles. Demi-dt 8,62 %, sensibilité 0,669 % >0,5 % ;
amplitude moitié 6,58 %, champs/a² différant de 2,392 % >1 %. Aucun seuil relevé.
Ne pas attribuer tout l'écart au solveur : l'approximation d'ordre deux n'est pas
qualifiée à ces amplitudes. L'oracle 128/256 modes varie de <0,053 %.

**Diagnostic et limite :** la quadrature de bande utilise U au centre de cellule,
alors que la bande peut être partielle. Intégrale indépendante : erreur 1,60 %,
reconstruction linéaire avec ∂zU déjà fourni 0,0162 % sur le flux fin. C'est un
correctif concret à construire, pas une preuve que toute l'erreur temporelle vient
de là. Aucun code produit changé ; suite complète S270 non rejouée. Aucun rendu.

**Maillons 2. Suite S273 : construction de la reconstruction linéaire dans le
transport réel**, faces intérieures/extérieures, bandes signées et coupées,
identité du fond nul et transaction. Ce lot lève un défaut mesuré de J2 et prime
V ou le coût dans cette suite locale ; un troisième lot d'instruments seuls serait
injustifié. A276 reste prioritaire avant toute étude de coût. Rejouer ensuite le
résidu et qualifier dt/amplitude ; pas de campagne infinie de raffinements.

**Rituel :** file active entière relue, J2, A92/A50, index et liste utilisateur
actualisés (décompte inchangé 119=3+48+68). I-04/I-05/I-06/I-08 produit inchangés,
I-14 oracle dérivé et critères conservés. δ général, V, B2, bathymétrie et seconde
cible gardent leurs déclencheurs. Jeton libre, aucune copie isolée à fermer.


## S273 — Bande du fond d'ordre deux, troncature de l'oracle isolée

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée « reprends le projet », master propre
2506015, copie unique, jeton libre. Plan de892c7, ADR et critères e751947, construction b090418,
suite 38d6c0b, campagne cec9fcb. Suite prescrite par S272 : un correctif produit, pas un instrument.

**Capacité reçue, maillons 0.** Le transport réel de `step_perturbation_mobile` intègre la bande
du fond en quadrature linéaire par couche (ADR-166, fonction unique aux faces intérieures et
extérieures, `∂zU` déjà fourni). Défaut reproduit par un témoin dans le test (1,60 % à la maille
fine), corrigé : flux produit 0,4190 / 0,0892 / 0,0162 %, ordre deux ; fond affine exact à
3,9·10⁻⁸. Consommateurs : tout pas couplé mobile, dont le B de production S254.
[Preuve](../../docs/validation/BANDE-LINEAIRE-S273.md).

**Validation :** 480 tests réussis, 0 échec, 21 ignorés (368 cœur, 17 intégrations, 95 harnais) ;
S253 release vert, `b₂` couplé 2,17 → 2,14 % ; campagne S253 à 128 colonnes rejouée, profil
0,162 → 0,148 % (5 cm) et 0,213 → 0,178 % (10 cm), `b₂` inchangé à 0,01 point près. Fond nul au bit, fond uniforme, témoin sans résidus,
refus et expirations inchangés. S271 initial 0,585 → 0,137 %. Oracle Python : 4 tests.

**Résidu temporel, refus maintenu** aux critères S272 inchangés : 9,11 / 6,18 / 5,88 % (S272 :
22,52 / 13,04 / 8,68 %) ; demi-pas 0,668 % ; a² 2,89 %. Cause resserrée : l'écart normalisé en a²
ne dépend pas de la maille (2,74 / 2,85 / 2,89 %), donc c'est la troncature de l'oracle ; la part
extrapolée en amplitude, déclarée en P2 comme diagnostic, converge 7,74 / 2,92 / 1,77 %. Deux
passages a/2 aux mailles grossière et moyenne ajoutés après la mesure fine, dits comme tels.

**Troisième session du fil (A211).** La suite débloque encore l'usage J2 (entrée des vagues de B
dans le domaine, liste 4.6) et tient en une session de mesure. Comparée à A276 (coût δ,
multigrille du mode mobile) et aux frontières du total : ceux-ci gardent leur déclencheur.
**Suite S274 :** réception sur la part extrapolée, protocole écrit avant mesure (trois mailles,
dt 1 ms, contrôle 0,5 ms, l'exemple doit accepter 500 µs) ; en cas d'échec, oracle d'ordre trois.
Une quatrième session d'instrument sans reçu ne se justifierait pas : passer alors à A276.

**Rituel :** file active entière relue ; J2, A50, liste 4.6, A92/A50, index (ADR-166, preuve)
actualisés ; L330. ADR-152/165 : notes datées. I-12 et I-14 relus, inchangés. δ général, V, B2,
bathymétrie et seconde cible gardent leurs déclencheurs. Aucun arbitrage utilisateur requis.


## S274 — Précision rapportée à l'usage, multigrille du mode mobile

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée : consigne de l'utilisateur —
vérifier que la précision recherchée sert le résultat final avant tout raffinement, puis passer
au blocage suivant. master 43385bd. Plan 7522372, amendé 30a179f ; étapes b485f75 à 4e5e5fe.

**Houle progressive, banc arrêté.** L'écart de 5,88 % vaut 5,8 µm rms sur une vague de 1 cm
(0,058 % de a), pente 1,9·10⁻⁵, déphasage 1,35·10⁻³ rad à 2 s ; il suit à 0,85–0,87 la
dispersion d'amplitude de Stokes, que l'oracle d'ordre deux n'a pas (ADR-122 le prévoyait).
Campagne unique, troisième amplitude et pas de temps : coefficient d'ordre deux 1,16 % (1 ms),
0,88 % (0,5 ms) ; extrapolation qualifiée aux mailles grossières, **pas à la fine** (1,94 %) :
la crête de 2 cm y franchit le centre des mailles. Budget d'ADR-120 non démontré (2,26–2,43 %),
aucun seuil relevé. Usage : 2 % de la correction ≈ 3 mm d'image pour Hs 4 m et ak 0,1 ; la mer
J1 n'en demande que ≈ 13 %. Réception pour mers cambrées différée avec déclencheur. Aucune
conclusion visuelle : `viewer/` ne rend pas δ (liste 8.7). [Usage](../../docs/validation/HOULE-USAGE-S274.md).

**Besoin découvert, A289 (liste 4.21, 120 points = 3 + 48 + 69).** B linéaire, δ fidèle : `η'`
dérive comme `a·ω₂·t`, ≈ 7 cm/min sous ak 0,06 (formule). Options inscrites, rien choisi.

**Capacité reçue, maillons 0.** Le pas couplé mobile a sa multigrille (ADR-167, remplace
ADR-147 point 5) : niveaux grossiers recalculés par pas depuis les mailles mouillées, Dirichlet
vers l'air. **280 → 49 ms** de médiane à 16 384 mailles, 1 145 → 18 itérations, maximum
660 → 77 ms ; premier pas reçu sans affinage. Consommateur : tout pas couplé mobile (J2).
Preuves : symétrie 1,2·10⁻⁹, 20 pas contre le témoin de Jacobi (6·10⁻⁸ m/s), réceptions S253
à 128 colonnes et houle fine identiques. [Coût](../../docs/validation/COUT-MOBILE-S274.md).

**Validation :** debug 370 cœur + 17 intégrations + 95 harnais, 0 échec ; release 373. Deux
essais adaptés et expliqués : comptabilité mémoire (+832 flottants à 32 × 16), et l'essai
d'affinage S253 qui s'éprouve désormais sous le témoin de Jacobi. Limites : ≈ 24 fois le budget
par pas ; précision à un pas par image non mesurée ; 2D seulement.

**Suite S275 : δ visible.** Un domaine couplé sous houle à crêtes longues, rejoué dans
`viewer/` (B seul, B+δ à 1 ms, B+δ au pas d'image), hors budget déclaré, pour la revue visuelle
de l'utilisateur ; l'écart au pas d'image y est mesuré. Puis le lot de coût suivant (A276).

**Rituel :** file active relue ; J2, A50, A276, A289, liste (4.6, 4.19, 4.21, 8.7), index,
ADR-147 (note) actualisés ; L331. I-03 (réductions ordonnées), I-06 (mémoire comptée, zéro
allocation), I-14 relus. δ général, V, B2, bathymétrie et seconde cible gardent leurs
déclencheurs. Aucun arbitrage utilisateur requis ; les 2 % d'ADR-120 restent inchangés.


## S275 — δ visible dans l'afficheur, revue R10 demandée

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Suite de S274 dans la même conversation,
sur consigne de continuer sans attendre. master d0ab484. Plan 4e2a68c ; étapes f29cf11 à efe3932.

**Capacité reçue, maillons 0.** Pour la première fois, δ se voit : scène `--delta` de `viewer/`
(ADR-168), houle JONSWAP à crêtes longues (Hs 2 m, Tp 8 s), domaine couplé 256 × 104 m, dx 2 m,
rejeu précalculé hors budget et mis en cache, bande extrudée le long des crêtes avec fondus de
rendu ; touche D : B seul, B+δ 4 ms, B+δ au pas d'image. Consommateur : le chemin d'image.
Preuves : 7 500 et 1 875 pas couplés tous reçus ; couche GPU à 7,4·10⁻⁸ m de sa lecture CPU sur
19 630 sondes ; sept images S254 identiques au bit, `--verify` reçu, essais 21/21.
[Protocole et résultats](../../docs/validation/DELTA-VISIBLE-S275.md).

**Mesures.** `η'` 15,9 mm rms, max ≈ 10 cm (5 % de Hs), groupé sous les grosses crêtes. Pas
d'image contre 4 ms, hors éponge : 0,25 mm rms, 1,33 mm max ; 11 mm dans l'éponge amont, masqué
par le fondu : le bord amont est une limite (4.7). Dans l'image : δ change 10 à 24 % des pixels de
plus de 4 niveaux à hauteur d'œil et en rasant, rien vu d'en haut ; le pas d'image ne se distingue
en aucun pixel. **Un pas par image suffit donc à cette échelle** (mesure, avant verdict).

**Limites :** précalcul (258 s), 2D extrudé, frontière du total au fond, fondus de rendu au lieu
d'une frontière physique, dérive A289 non corrigée. Contrainte GPU : huit tampons de stockage
par étage ; la bande suit les impacts dans le même tampon.

**Incidents :** `--multi --revue` sans suffixe a réécrit les PPM R1 de `captures/s254` (PNG envoyés
intacts, `captures/` non versionné) ; un `sed` global avait touché d'autres lignes de `main.rs`,
restauré depuis Git puis réédité. Rien de faux n'a été committé.

**Suite S276 :** consigner le verdict R10 ; puis lot de coût vers δ en direct (A276) — coût par
image mesuré (pas couplé et échantillonnage de B, aujourd'hui dominant), départ depuis la
pression précédente, échantillonnage décimé du fond (SPEC-004 §6.2). Arbitrage : aucun.

**Rituel :** file active relue ; liste (1.1, 8.7 : 120 = 3 + 49 + 68), feuille de route J2, file
(rendu de δ), registre des revues (R10), index (ADR-168) actualisés. I-04 (δ cosmétique) et
I-06 (aucune allocation ajoutée au pas du cœur) relus ; l'afficheur précalcule hors image.


## S276 — δ en direct dans l'afficheur

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée « Continue », sans verdict R10 (la
revue reste en attente, rien consigné à sa place). master 79bcb50. Plan 9eab501 ; étapes 1f0ca8c,
ea3f1e9, 9e36897, 54de4f3.

**Capacité reçue, maillons 0.** δ tourne en direct : `water-viewer --delta --delta-direct`, un pas
de 16 ms par image dans la boucle de l'afficheur, **40,5 images/s** à 6 656 mailles, CPU 23,8 ms
par image dont δ 21,7 ms, **zéro allocation** dans la phase de mise à jour ; identique au bit au
rejeu de 16 ms sur 200 images. Consommateur : le chemin d'image.
[Mesure](../../docs/validation/COUT-DIRECT-S276.md).

**Techniques, par la carte du coût :** échantillonnage du fond 33 ms, pas 24 ms à l'ouverture.
(1) `Background::differential_grid_extended` : phase par colonne, facteur vertical par couche,
**identique au bit** au ponctuel — 33 → 5,1 ms ; (2) ADR-169, départ depuis la pression publiée
dans les deux pas mobiles — itérations 266 → 134 sur vingt pas, pas 24 → 17,5 ms, vitesses à
8·10⁻⁸ m/s du départ nul. `set_free_surface` remet la pression à zéro (renaissance d'un domaine).

**Validation :** suite 372 cœur + 17 intégrations + 95 harnais, 0 échec ; release 375 après la
remise à zéro de la pression ; identité fond nul / S237 conservée ; rejeux recalculés (cache
versionné), statistiques S275 inchangées aux décimales publiées.

**Limites :** ≈ 11 fois le budget d'eau ; 2D extrudé ; temps simulé fixe par image (ralenti si
l'image dure plus) ; un fil. Absents : cadence découplée, GPU, 3D.

**Suite S277 :** consigner R10 s'il arrive ; sinon δ sous budget déclaré — cadence de δ découplée
de l'image (pas de 32 à 48 ms, précision contre 16 ms), ordonnancement et dégradation (I-05) ;
puis la décision d'un δ sur GPU (ADR). Arbitrage utilisateur : aucun.

**Rituel :** file active relue ; liste (4.19, 8.7), feuille de route J2, file (A276, rendu de δ),
A276, index (ADR-169, COUT-DIRECT-S276), ADR-168 (note) ; L332. I-03 (ordre des réductions
inchangé), I-06 (aucune allocation dans la boucle), I-12 (naissance au repos) relus.

---

## S277 — la fenêtre qui ne s'ouvrait pas, le verdict R10, et l'onde injectée

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée : « je dois réaliser la revue 10 mais
la fenêtre ne s'ouvre pas ». master 6c22b90 (plan de S277 déjà committé, session coupée après P1).
Étapes a563b76, 7e5bf8b, 5b4fb4a, 95f003b, 23f2bb1, 45ba82b ; découpages 2b2b370, 861e82c, e5ee593.

**Capacité reçue, maillons 0.** Deux, et la seconde était imprévue.

1. **δ se lance depuis n'importe quel dossier** : le cache des rejeux était cherché relativement au
   dossier courant, donc manqué depuis la racine, et le viewer repartait pour trois minutes de
   précalcul muet — vu du dehors, une fenêtre qui ne s'ouvre pas. `captures!` ancre au crate tous
   les chemins de sortie du viewer ; `--delta-direct` ne précalcule plus aucun rejeu puisqu'il n'en
   lit aucun ; un précalcul nécessaire s'annonce et rend un dixième par ligne. **Fenêtre en 1,1 s**
   (`--delta`) et 1,4 s (`--delta --delta-direct`) depuis la racine. Consommateur : la revue de
   l'utilisateur, qui a pu être faite.
2. **Une onde s'injecte dans δ et l'effet de la houle sur elle se mesure** : `--onde[=<m>]` pose une
   bosse gaussienne dans le profil initial, `--onde-mesure`, `--onde-interaction` et `--onde-regime`
   la chiffrent, `--delta-hs=` / `--delta-tp=` règlent la houle.
   [Mesure](../../docs/validation/ONDE-INJECTEE-S277.md).

**Verdict R10, reçu et consigné** ([REVUE-VISUELLE](../../docs/validation/REVUE-VISUELLE.md#verdict-r10--reçu-s277-2026-09-18)).
Deux retours, **tous deux sur la scène, aucun sur la couche** : le motif de surface est trop
répétitif, et les vagues doivent interagir avec l'onde. L'utilisateur a aussi dit ne pas savoir quel
retour on attendait de lui — défaut de la demande, pas de la réponse.

Mesuré avant d'écrire le verdict (`code/water-core/examples/bandes_s277.rs`), houle de `--delta`
contre mer S201 : λ **40,1–198,7 m** contre 3,7–210,7 m ; étalement **0,00°** contre 87,19° ;
écart-type de η le long des crêtes **0,0000 m** contre 0,2872 m. Le constat est exact : aucune vague
sous 40 m, et le même profil de 128 colonnes répété sur 200 m. Cause structurelle et non
cosmétique — le domaine δ est une tranche 2D sans dimension `y`, l'étalement nul en est la
condition, et les vaguelettes visibles sont la queue spectrale S256, habillage de pentes hors du
domaine simulé. **Aucune conclusion d'invisibilité de δ n'est tirée** : la scène ne permettait pas
de la poser, et la question « le pas de 16 ms suffit-il ? » reste sans réponse visuelle.

**L'onde injectée, et ce que la houle lui fait.** Bosse de 0,6 m, σ = 8 m, vitesse nulle : elle se
sépare en deux fronts (Cauchy) qui vont de 13 m à 2 s à ≈ 105 m à 24 s, soit 4,2 m/s. Trois domaines
identiques au pas près — l'onde sous la houle, la houle seule, l'onde sur mer plate (`flat_background`,
`Hs` = 0) — isolent l'effet par différence : **4 à 8 % de l'onde** dans la houle de S275, cohérent
avec `u_orbital/c ≈ 15 %`. Contrôle interne : sur mer plate l'énergie des deux côtés reste égale à
**1,000 exactement**. C'est donc invisible : 5 % d'une onde de 10 cm font 5 mm.

**Le balayage des régimes** (`--onde-regime`) : écart/onde **8,8 % → 28,2 % → 73,3 %** quand `ak`
va de 0,063 à 0,224 (`Hs`/`Tp` de 2/8 à 4/8 puis 4/6). À 4/6 l'onde est méconnaissable et la
symétrie gauche/droite se brise (0,812), mais la correction couplée de la houle seule (223 mm)
écrase l'onde (87 mm) : **le régime lisible est 4 m / 8 s**. L'autre levier — une onde plus courte —
est **fermé par la résolution** : `DX` = 2 m impose `λ ≥ 16 m`, et `σ` = 8 m y est déjà.

**Limites et non-fait.** L'utilisateur n'a pas rendu de verdict sur l'onde (il regardait quand la
session a tourné vers l'orchestrateur). À `Hs` = 6 m / `Tp` = 6 s (`ak` = 0,335) le pas **refuse**
(`Domain`, garde de géométrie de SURFACE-MOBILE-S237) : **cause non diagnostiquée**, peut-être la
hauteur libre du domaine (`REST` = 96 m sous un sommet à 102 m) — ne pas en conclure une limite
physique. Aucune image de l'onde produite : seule la commande interactive existe. Rien de S275/S276
n'est touché ; essais 21/21.

**Suite S278 : établir l'orchestrateur des régimes**, demandé explicitement par l'utilisateur après
qu'il eut demandé si le système qui décide entre simulation volumétrique, haute mer analytique et
zone de transition existait. Réponse trouvée dans le dépôt : **entièrement conçu, pas écrit** —
`WaterManager` seul décideur et sac à dos sous budget (ADR-006, ADR-012 §2 : `P = gameplay ×
perception × urgence`), score `s` à hystérésis 0,60 / 0,40, durées de vie et pool (ADR-013 §5),
bascule perturbatif → substitutif à `max|δ| > 0,35·Hs_local` **proposée, jamais calibrée**
(ADR-001 §3.3, banc B4). Liste du projet fini 1.4, 1.5 et 1.6 : *absent, conçu*. Aucune trace de
`WaterSystem` ni de `WaterManager` dans `code/`. La bande δ que l'utilisateur regarde est une
transition **câblée en dur** : rien ne la décide, ne la déplace ni ne l'éteint.

**Arbitrage utilisateur :** aucun en attente ; la demande d'établir l'orchestrateur prime sur la
suite automatique de S276 (cadence découplée, δ GPU), qui garde son déclencheur dans la file.

**Rituel :** journal, L333, file active relue (rendu de δ actualisé, orchestrateur ajouté),
feuille de route J3, index (ONDE-INJECTEE-S277), REVUE-VISUELLE (verdict R10). Invariants relus :
I-12 (naissance au repos — l'onde injectée est une naissance, pas une exception), I-03, I-06.

---

## S278 — l'ordonnanceur : ce qui décide qu'une zone est simulée

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée : « fais le rituel, puis l'objectif est
d'établir ce système » — après que S277 eut répondu que le système de décision des régimes était
conçu depuis S01 et jamais écrit. master 4265d22. Plan 3703bbb ; étapes 8091470, 6b42c2e, 5cedb67,
3c7298f, f526a68.

**Maillons : 1, et c'est volontaire.** La règle demande trois choses ; il en manque une. *Ce qui
devient possible* : décider quels domaines vivent, sous budget, sans battement et de façon
reproductible. *La preuve* : le banc, quatre propriétés tenues
([mesure](../../docs/validation/ORDONNANCEUR-S278.md)). *Le chemin qui le consomme* : **aucun** — rien
n'est branché sur l'ordonnanceur, et la bande δ de l'afficheur reste câblée en dur. C'est
exactement le cas que la règle des maillons veut attraper, et le compter zéro serait se payer de
mots. S279 doit brancher.

**Construit** : `code/water-core/src/scheduler.rs`, sans dépendance, sans allocation à l'exécution.
Le sac à dos d'ADR-012 §1 — candidats `(P, C)`, tri par `P/C` décroissant **comparé en croix**
(pas de division, pas de cas particulier à coût nul, égalités départagées par identité), allocation
jusqu'au budget, `budget_ms` distribué. **Le budget donné est le coût annoncé, pas une part du
reliquat** : une borne, pas une enveloppe — la somme des bornes ne dépasse donc jamais le profil, et
c'est ce qui ferme le chemin vers un pic d'image. Hystérésis 0,60 / 0,40, durée de vie 750 ms,
délai d'extinction 1,0 s de séjour **continu** sous le seuil, horloge qui recule refusée. Seize
essais.

**[ADR-170](../../docs/adr/ADR-170-les-trois-poids-sont-bornes.md) — les trois poids sont bornés.**
ADR-012 §2 ne borne pas les poids et définit `W_urgence = 1/temps`, qui diverge ; ADR-013 §5 veut
un score dans `[0,1]` avec des seuils 0,60 / 0,40. **Les deux ne se raccordaient pas depuis S01**,
à travers six audits et deux revues croisées. Décision : les trois poids sont des fractions par
contrat, `s = P` sans facteur d'échelle, et la normalisation de `W_urgence` passe à l'hôte avec un
horizon déclaré — un horizon mal choisi se voit alors dans les décisions au lieu de se cacher dans
une échelle implicite.

**Le banc** (`ordonnanceur_s278.rs`) : un bateau longe cinq îlots serrés à 15 m/s, 1 200 pas à
30 Hz. **Cinq domaines vivants simultanément demandent 4,0 ms ; 1,600 ms sont distribués** pour 2,0
déclarés. Transitions : **2 par îlot**, une naissance et une mort, aucun battement. Plus courte vie
10 267 ms pour 750 minimum. Empreinte `6aebff024c734fc9`, identique à deux exécutions. Suite
complète 500 essais, 0 échec.

**Le banc refuse de passer si le budget n'a pas été disputé**, et c'est ce garde qui a sauvé la
mesure : sa première version espaçait les îlots de 150 m, un seul domaine vivait à la fois, tout
passait et rien n'était prouvé (L334).

**Limites, toutes écrites dans la mesure.** Seuils **non calibrés** — valeurs de départ d'ADR-013
§5, banc B8 inexistant. Poids déclarés à la main : aucun hôte réel ne les produit, `W_perception`
n'a jamais été calculé depuis une caméra. Coût constant dans le banc là où ADR-012 §3 le veut
mesuré en continu. Famine possible d'un gros candidat (glouton). Un vivant non financé reste
vivant : la dégradation d'ADR-012 §4 n'est pas écrite. **Rien de la forme des domaines** — grille,
blocs épars, fusion/séparation, substitutif et son critère `0,35·Hs_local` jamais calibré.

**Suite S279 : brancher.** Faire décider la bande δ de l'afficheur par l'ordonnanceur plutôt que
par le code — c'est le chemin qui consomme la capacité, et ce qui remettra les maillons à zéro.
Demandent d'abord un `W_perception` calculé depuis la caméra de la scène et un coût réinjecté
depuis `delta_budget`.

**Arbitrage utilisateur :** aucun. Reste en attente depuis S277 : le verdict visuel sur l'onde
injectée, que l'utilisateur regardait quand la session a tourné vers l'ordonnanceur.

**Rituel :** journal, L334 et L335, file active relue (orchestrateur actualisé), feuille de route
J3, index (ADR-170, ORDONNANCEUR-S278), liste 1.4 passée à *partiel*. Invariants relus : I-03
(décision reproductible, empreinte à deux exécutions), I-06 (capacité fixée à la construction,
mémoire demandée à l'hôte).

---

## S279 — la bande δ décidée par l'ordonnanceur

**2026-09-18, Claude Opus 5 (Claude Code desktop).** Entrée : « branche l'ordonnanceur sur la bande
δ ». master 57fb182, maillons 1. Plan 65e72e2 ; étapes 6030bfb, 13a4dbe, 16aab70, be9eefd ;
découpage 803a407.

**Capacité reçue, maillons remis à 0.** *Ce qui devient possible* : une zone de simulation vit ou
meurt parce que quelque chose l'a décidé, et non parce que le code le disait. *Le chemin qui le
consomme* : la bande δ de l'afficheur, à chaque image. *La preuve* : elle s'éteint et se rallume
toute seule, aux bons instants ([mesure](../../docs/validation/ORDONNANCEUR-S279.md)).

**Le relevé dynamique** (`--delta --delta-arbitrage`, 937 images au pas de δ, trois phases) :
allumée à 0 s (part de cadre 0,5456), **éteinte à 6,016 s** après le passage à la pose haute
(0,3185) — soit **1,016 s** plus tard, le délai d'ADR-013 §5 plus un pas — **rallumée sans délai à
10,0 s**. Trois transitions, aucune de plus. C'est la première fois que le système éteint quelque
chose de lui-même, et il l'éteint là où S275 avait mesuré que δ ne change **aucun** pixel.

**Rien ne change là où rien ne devait changer** : les douze empreintes de la revue R10 sont
identiques à celles de S275, pose haute comprise. Explication, et elle vaut d'être retenue :
`--revue-delta` travaille à **temps figé**, et ni la durée de vie minimale ni le délai d'extinction
ne s'écoulent quand le temps ne bouge pas. **Une décision demande du temps ; une image isolée n'en
donne aucun.** L'identité prouve que le branchement ne casse rien, pas qu'il décide — d'où le
relevé.

**`W_perception` se calcule** (`Projection::screen_fraction`) : emprise projetée, coupée au plan
proche **avant** la division par la profondeur — sans quoi une emprise dont on tourne le dos
paraîtrait immense — puis aux quatre bords du cadre. Deux attentes fausses corrigées par la mesure :
dans la pose de R10 la caméra est **à l'intérieur** de l'emprise (256 m sur 200), et la part
**n'est pas monotone** en lacet (0,0308 de face, 0,0323 à 0,3 rad). Ce dont l'hystérésis a besoin
n'est pas la monotonie mais l'absence de saut : moins de 0,01 par pas de 0,05 rad sur un demi-tour.
`W_gameplay` et `W_urgence` restent **déclarés** au maximum : un afficheur n'a ni acteur ni objectif.

**[ADR-171](../../docs/adr/ADR-171-les-seuils-d-activation-appartiennent-au-profil.md) — les seuils
appartiennent au profil.** Mesuré avant d'écrire le branchement : la part de cadre vaut 0,5774 /
0,5456 / 0,5571 / 0,3185 / 0,5089 dans les cinq poses. **Aucune n'atteint 0,60**, et comme `P` est
un produit de trois fractions il est toujours inférieur à la plus petite : brancher tel quel aurait
éteint la bande partout. Remonter les autres poids aurait été régler les poids sur le résultat
voulu (L334). Calibration sur un critère indépendant — S275 : δ change 10 à 24 % des pixels à
hauteur d'œil, **0 % vue d'en haut** — donc `on = 0,45`, `off = 0,35`, frontière entre la pose
haute et la rasante. Un intervalle inversé ou nul est refusé.

**Défaut trouvé par le relevé, et non corrigé : l'exclusion par le coût est absorbante** (L336).
Avec un budget d'image à 30 Hz, la bande mourait à 0,352 s sans jamais revenir, part de cadre
inchangée. Le pire pas vaut 45,8 ms (S276) ; un seul dépassement exclut, et **un domaine exclu
n'exécute plus de pas, donc ne produit plus de mesure, donc reste exclu**. Budget de l'afficheur
porté à 50 ms : le cas observé est refermé, **le défaut de fond reste entier** — avec les 2 ms d'un
jeu il revient aussitôt.

**Limites.** Un seul candidat : ni tri par `P/C`, ni contention, ni famine ne sont exercés par ce
branchement (ils l'étaient au banc S278). Les poids déclarés restent déclarés. **Rien ne déplace la
bande** : emprise, résolution et position restent écrites en dur ; grille et blocs épars (liste 1.5
et 1.6) ne sont pas commencés. Une extinction n'est pas gratuite pour l'onde injectée de S277 : δ
renaît au repos (I-12), donc elle repart de zéro. Essais 27 viewer + 18 ordonnanceur ; fenêtre
ouverte en 1,1 s.

**Suite S280 :** au choix de l'utilisateur. Ce que la file porte : sortir de l'exclusion absorbante
(coût décroissant ou dégradation d'ADR-012 §4 rang 1), plusieurs candidats réels dans l'afficheur,
ou la forme des domaines. Reste en attente depuis S277 : le verdict visuel sur l'onde injectée.

**Rituel :** journal, L336, file active relue, feuille de route J3, index (ADR-171,
ORDONNANCEUR-S279), liste 1.4. Invariants relus : I-06 (l'ordonnanceur alloue à la construction,
rien par image), I-12 (renaissance au repos d'un domaine rallumé), I-03 (décision reproductible).

---

## S280 — un pic ne condamne plus un domaine

**2026-09-19, Claude Opus 5 (Claude Code desktop).** Entrée : « Continue », jeton libre, master
5a3500e — la file portait en tête le défaut ouvert de S279. Plan 06ababb ; étapes 366f530, 3c215e8.

**Capacité reçue, maillons 0.** *Ce qui devient possible* : un domaine survit à un pic de coût au
lieu d'être condamné par lui. *Le chemin qui le consomme* : la bande δ de l'afficheur, au budget
même qui la tuait. *La preuve* : à 33 ms, 689 pas payés sur 689 vivants, là où S279 la voyait mourir
à 0,352 s ([mesure](../../docs/validation/COUT-ROBUSTE-S280.md)).

**Trois remèdes écartés par l'analyse, avant d'écrire une ligne.** Distribuer le reliquat au lieu
d'exclure — juste en général (ADR-012 §1 point 5), **inapplicable à δ**, dont le pas couplé est tout
ou rien (ADR-152). Admettre d'office un affamé — viole la seule propriété qu'ADR-012 §1 demande de
défendre avant les autres. Faire décroître l'estimation dans l'ordonnanceur — ce n'est pas son
travail, et il ne doit pas inventer un chiffre que personne n'a mesuré.

**Le vrai défaut était écrit dans ADR-012 §3 depuis S01** : « la cible de mesure est le 99ᵉ centile
de la contribution par frame, jamais la moyenne ». L'hôte estimait par **le dernier pas**. Une
valeur isolée n'est pas une estimation.

**Correction, côté hôte :** médiane des **huit derniers pas payés**, prise du côté prudent ; le
compte d'échantillons **décroît d'une unité par pas non payé**, et vidé le domaine retombe sur son
nominal et retente. Un domaine qui ne tourne plus ne sait plus ce qu'il coûte, et le dire est plus
honnête que de garder son pire chiffre. Le cœur ne change que d'un `set_profile`, qui permet à un
banc d'éprouver un budget serré sans rien réallouer.

| budget | pas vivants | pas payés | estimation maximale |
|---|---|---|---|
| 20 ms | 689 / 937 | **0** | 22,0 ms |
| 33 ms | 689 / 937 | **689** | 29,7 ms |
| 50 ms | 689 / 937 | **689** | 28,4 ms |

**Trois lectures.** Le budget de 33 ms ne tue plus : des pas individuels le dépassent, l'estimation
jamais. **La décision ne dépend pas du budget** — 689 pas vivants dans les trois cas, aux mêmes
instants : le score dit qui a le droit de vivre, le budget seulement qui tourne, et c'est la
séparation qu'ADR-012 §1 demande. À 20 ms le domaine est **affamé, pas absorbé** : vivant,
estimation au nominal, servable dès que le budget le permettrait.

**Inchangé** : douze empreintes de R10, relevé dynamique de S279 (allumée 0 s, éteinte 6,016 s,
rallumée 10,0 s, trois transitions), banc S278 (empreinte `6aebff024c734fc9`), onde injectée de
S277. Suite complète **502 essais, 0 échec** (390 cœur, 14 + 2 + 1 intégrations, 95 harnais) ;
28 essais viewer.

**Limites.** La **famine reste** — un domaine trop cher ne tourne pas, et aucune dégradation ne
vient le rétrécir ; les sept rangs d'ADR-012 §4 ne sont pas écrits, et le rang 1 demande la forme
des domaines, qui n'existe pas. **Huit échantillons est un choix, pas une mesure** : aucun banc ne
l'a éprouvé, B8 n'existe toujours pas. Le régime oscillant — médiane durablement au-dessus du
budget — n'a pas été observé : le cas à 20 ms ne l'atteint pas.

**Suite S281 :** au choix de l'utilisateur. La file porte la dégradation (ADR-012 §4 rang 1,
rétrécir au lieu d'exclure), plusieurs candidats réels dans l'afficheur, et la forme des domaines
(liste 1.5/1.6). **En attente depuis S277** : le verdict visuel sur l'onde injectée.

**Rituel :** journal, L336 complétée d'une levée datée, file active relue, index
(COUT-ROBUSTE-S280). Invariants relus : I-06 (l'anneau des coûts est un tableau fixe, rien par
image), I-03.

---

## S281 — la trajectoire en portes, et une v1 proposée

**2026-09-19, Claude Opus 5 (Claude Code desktop).** Entrée : l'utilisateur demande la trajectoire
sous une forme qu'elle n'avait pas — « création / test / validation d'un système puis d'un autre »
— et **la déclaration d'une v1**. master 0f2650d. Plan 27414b9 ; étape 27dd86b.

**Maillons : 1.** Session documentaire, aucune capacité nouvelle. Elle n'invente rien : elle
regroupe §2 en portes et nomme, pour chacune, ce qui vaut réception.

**Écrit dans FEUILLE-DE-ROUTE §3 bis, et nulle part ailleurs.** La tentation était un second
document ; deux lectures parallèles de la même trajectoire divergeront, et c'est le mécanisme
exact des trois forks (L137). La section le dit d'elle-même : si elle contredit §2 un jour, c'est
elle qui est fausse.

**Six portes** : A ce qui décide (en cours), B δ sur les deux dimensions horizontales, C δ sous
budget, D solides et flottabilité, E V articulé avec δ, F grande échelle. Chacune porte ce qu'on
crée, le banc ou le cas qui l'éprouve, et la condition de réception — **une porte se franchit
quand sa colonne « reçu si » est vraie, pas quand le code existe**.

**V1 proposée après la porte D** : mer parcourue en temps réel, perturbations locales décidées par
le système et non câblées, δ qui tient sur une vraie mer et dans le budget, objets qui flottent.
Sans les inondations complexes, la grande échelle, les phénomènes secondaires — **reportés, jamais
retirés** (ADR-127 §1). **Ce n'est pas une décision** : fixer ce qu'une version contient appartient
à l'utilisateur (ADR-127 §6), et la section l'écrit pour qu'aucune session ne parle d'une « v1 »
comme d'un périmètre acquis.

**Ce que l'état réel dit, sans l'embellir** : 3 points validés sur 120, 49 partiels, 68 absents. Le
chiffre ne mesure pas l'avancement — beaucoup de partiels portent l'essentiel de leur difficulté —
il mesure que **presque rien n'est allé jusqu'à la réception**. Les quatre manques qui commandent
l'ordre sont tous mesurés : δ est une tranche 2D (verdict R10), son coût vaut ≈ 11 fois le budget,
l'ordonnanceur ne décide ni où ni de quelle forme, V n'a aucune articulation avec δ.

**Suite S282 :** au choix de l'utilisateur, et cette fois une décision l'attend — **tranche-t-il la
v1 proposée ?** Sinon la file porte la porte A (dégradation, plusieurs candidats, forme des
domaines). **En attente depuis S277** : le verdict visuel sur l'onde injectée.

**Rituel :** journal, feuille de route §3 bis, file active relue. Aucun ADR : rien n'a été décidé,
une proposition a été écrite.

---

## S282 — le coût vient des pas réellement exécutés

**2026-09-19, Codex GPT-6, application desktop.** Entrée : reprendre vers une V1 proche des
intentions initiales, réaliste et performante. Amorce : master 22605fd propre, une seule copie,
branche historique archivée. Plan 88ddee7 ; correction 7e17803.

**Capacité reçue, maillons 0 :** le pilotage du coût de la bande en direct ne fabrique plus de
mesures pendant une pause, une naissance, une renaissance ou un refus. Consommateur réel :
`Layer::update` ; `Live::advance` distingue un pas réussi d'une simple publication de surface.
Les diagnostics de durée/itérations sont remis à zéro à chaque appel. Aucun calcul physique,
seuil ni budget modifié ; aucun nouvel espace mémoire dans la boucle.

**Preuve :** deux tests dans `viewer/src/delta.rs`, exécutés d'abord sur le code antérieur :
échecs attendus, une mesure au lieu de zéro à la naissance et deux au lieu d'une après refus.
Après correction : naissance, douze images en pause, saut, retour, reprise et refus vérifiés.
`cargo test --release` dans viewer : **30 réussis, 1 ignoré** ;
`cargo test --workspace --release` dans code : **505 réussis, 18 ignorés** (393 cœur,
14 + 2 + 1 intégrations, 95 harnais), aucun échec. Avertissements existants conservés.

**Choix du lot :** défaut d'intégrité constaté dans le consommateur pendant la lecture de la
porte A ; le corriger avant d'étendre les domaines évite de multiplier les mesures fictives.
Cela ne remplace pas le chantier spatial, prioritaire ensuite. Pas de nouvelle campagne de
coût ni revue visuelle : aucune accélération ou amélioration perceptive revendiquée.

**Limites et non-fait :** famine, budget I-05, domaine 3D et forme mobile inchangés. La médiane
S280 n'est pas le 99e centile ; coût des tentatives échouées et vieillissement par cadence non
reçus. L'oubli reste déclenché par appel d'arbitrage refusé, à qualifier avant cadence découplée.
Le périmètre V1 proposé en S281 n'est pas transformé en décision par cette reprise.

**Suite :** porte A, forme/redimensionnement consommés par δ et dégradation de rang 1 ; coût
A276 à traiter avant un deuxième domaine simultané. δ général, V, B2, bathymétrie et seconde
plateforme conservent leurs déclencheurs ; verdict visuel de l'onde S277 toujours attendu.

**Rituel :** file active relue entière, feuille de route actualisée ; suivi A276 dans le registre
des angles morts. Invariants I-05/I-06/I-12/I-13 relus, aucun amendé. Pas de nouvel ADR ni de
document à indexer. Copie principale conservée, aucune copie isolée à fermer.
---

## S283 — un domaine réellement plus étroit, avec refus avant rupture de hauteur

**2026-09-19, Codex GPT-6, application desktop.** Entrée : continuer la porte A ; master dcaa97f,
une seule copie, propre. Plan 0800c7d ; cœur 954fc7f ; consommation/réception 62072ce.

**Capacité reçue, maillons 0 :** réduire manuellement un domaine perturbatif vivant vers une
réserve plus étroite, sans changer sa maille ni son horloge. Le cœur transfère et amortit sans
allocation ; `Live` poursuit le vrai pas couplé, `Layer` publie la nouvelle emprise à l'image et
à l'ordonnanceur. Commande de banc N. Le centre est conservé au bit au transfert. Réception
bornée au fond plat et à une réduction 256→128 m dans l'hôte, pas au système spatial complet.

**La contre-épreuve a changé l'intégration :** rétrécir à 1,024 s change la hauteur de 33,447 mm,
contre 3 mm déclarés avant mesure. Garde ajouté avant permutation, sur l'interpolant et ses
fondus, avec borne de dérivée entre points ; le cas est refusé (borne 33,729 mm), l'ancien état
reste consommable. Le cas précoce à 0,016 s est admis. Pas de prétention à I-12 visuel : pentes
et suite temporelle restent ouvertes, A290. Le diagnostic forcé dérive de 21,255 mm au centre.

**Coût et preuves :** [RETRECISSEMENT-S283](../../docs/validation/RETRECISSEMENT-S283.md) porte
le contrat et le protocole. 6 656→3 328 cellules, médiane 25,55→12,38 ms, ×2,06 sur le pas ;
transfert + garde 3,03 ms, secteur avant/après. Pas de restitution de mémoire (réserve gardée).
Tests release : **506** cœur/harnais, **33** afficheur, aucun échec, 19 ignorés au total.
Refus atomiques, continuation couplée et zéro allocation mesurés ; coordonnées mondiales,
horloge et publication réduite vérifiées. Aucun gain de qualité ou verdict visuel inventé.

**Non-fait :** dégradation automatique/non focale, agrandissement, déplacement, domaines épars,
substitutif et 3D ; I-05 toujours non reçu. Le garde vérifie la hauteur CPU instantanée seulement.
**Suite :** préparer progressivement le rétrécissement, recevoir perte et évolution avant
permutation, puis décision non focale sous budget. Ce lot lève un défaut d'usage de la porte A ;
il prime un autre approfondissement de la médiane S282. Coût avant deuxième domaine/3D ;
V, B2, bathymétrie, multiplateforme et verdict visuel S277 gardent leurs déclencheurs.

**Rituel :** file active relue entière et états périmés remplacés, feuille de route et index
actualisés, A290 ouvert et A276 suivi. I-04/I-05/I-06/I-08/I-12/I-13/I-17 relus : aucun amendé,
aucune sérialisation de δ, aucun changement d'autorité ou de modèle B/W. Aucun ADR réécrit.
Une seule copie principale, aucune copie à fermer. V1 S281 reste proposée, ambition complète
maintenue. Journal, plan cochés et jeton libéré dans le commit de clôture.
---

## S284 — préparer la réduction, et mesurer ce qui reste perdu

**2026-09-19, Codex GPT-6, application desktop.** L'utilisateur demande de continuer pendant
la clôture S283. Base propre 48d2ab7, copie principale unique. Plan e8ade0b, noyau 22963d4,
consommation a3fac09.

**Capacité bornée reçue, maillons 0 :** une demande tardive qui échouait au garde S283 prépare
désormais le champ après chaque vrai pas et finit par permuter sans relâcher le garde. Chemin
`Volume::prepare_shrink` → `Live::advance` → `Layer::update`, commande M. Fenêtre/fond/maillage
identiques à S283. Pas de politique non focale automatique, pas de réception du résultat visuel.

**Preuves :** demande à 1,024 s, permutation à 1,792 s. Correction nodale visée 1,5 mm,
maximum 1,503 mm avec arrondi à hauteur 96 m ; garde 3 mm inchangé. Amortissement au centre
identité, refus atomiques et coefficient neutre reçus. Pause sans travail ni échantillon de
coût. **Zéro allocation dans 321 updates**, tentatives incluses. 507 tests cœur/harnais et
34 viewer réussis, aucun échec, 19 ignorés. Coût réinjecté élargi à la préparation et au garde ;
coût du grand domaine invalidé à la permutation.

**Échec de fidélité conservé :** l'écart central au témoin large atteint **66,994 mm** jusqu'à
5,120 s. La fenêtre diffère de S283, aucune conclusion de détérioration relative. Le garde
instantané et la correction nodale ne reçoivent pas les pentes, l'interpolant complet ni la
suite temporelle. A290 reste ouvert. Coût complet sur secteur : médiane 12,39 ms, p99 42,03 ms,
maximum 43,10 ms ; I-05 non reçu. [Preuve](../../docs/validation/PREPARATION-RETRECISSEMENT-S284.md).

**Suite recommandée :** témoin large préparé, comparé au large intact et au réduit préparé sur
une même fenêtre, pour attribuer A290 avant une correction. Ce serait une troisième session
spatiale : comparer explicitement ce lot au blocage A276/3D avant de le choisir. L'automatisation
reste suspendue à la réception de perte/évolution/coût, pas à une autorisation manquante.
V, B2, bathymétrie, seconde plateforme et verdict S277 restent dans la file ; V1 non redéfinie.

**Rituel :** file active relue (lignes S283 encore courantes hors remplacement spatial), feuille
de route/index actualisés, A290/A276 suivis, invariants I-04/I-05/I-06/I-12/I-13/I-17 conservés.
Aucun ADR modifié, aucune donnée persistée. Plan coché, jeton libre ; une seule copie, rien à fermer.
## S285 — 2026-09-19 — attribution du rétrécissement

**Entrée :** continuer. Troisième lot spatial explicitement comparé à A276/3D : attribution
bornée utile avant correction, mais prochain gain de capacité attendu du coût, pas d'un
quatrième approfondissement. Banc `--delta-attribution` consommant deux Live et un Layer.

**Résultat :** à la permutation (1,792 s), préparation seule : 8,118 mm au centre, transfert :
zéro. Sur 5,120 s, maxima préparation/intact 45,517 mm, réduit/préparé 25,391 mm ; total
66,994 mm. Maxima non additifs. 48 pas préparés identiques au bit ; arrêt commun de préparation,
963 appels sans allocation. 34 tests viewer réussis, un ignoré ; cœur inchangé, reçu S284
conservé. [Protocole et limites](../../docs/validation/ATTRIBUTION-RETRECISSEMENT-S285.md).

**Non-fait :** ni correction physique spéculative, ni rendu, ni gain de coût revendiqué.
A290 reste ouverte : le garde de permutation ne reçoit pas l'évolution préparatoire. Ce banc
n'est pas une capacité de jeu, maillons = 1. Pas d'ADR modifié ni d'arbitrage nouveau.

**Suite :** A276, cadence découplée avec précision et budget mesurés, coût des refus et
vieillissement de l'estimation ; avant 3D et deuxième domaine. A290 revient avant réduction
automatique ; V, B2, bathymétrie, multiplateforme, verdict S277 et proposition V1 restent dans
la file avec leurs déclencheurs. Recommandation du bilan S227 appliquée : quitter le fil local
après diagnostic borné et viser une capacité consommée.

**Rituel :** file active entièrement relue, état spatial remplacé, index et feuille de route
actualisés, suivi A290, invariants I-04/I-05/I-06/I-12 conservés. Dépôt principal seul, aucune
copie à fermer. Jeton libre et plan terminé.

## S286 — 2026-09-19 — cadence δ et intégrité du coût

**Entrée :** continuer, priorité A276 après attribution spatiale. **Capacité reçue :** la
pause et les appels répétés ne détruisent plus les mesures de coût ; `Layer::arbitrate` vieillit
selon le temps simulé non financé. Régression rouge (8→7 entrées sans temps écoulé), puis verte,
avec retour d'horloge et reprise. Correction consommée dans le chemin image : maillons = 0.

**Cadence construite au banc** : Live 16/32/48 ms, maintien du profil entre pas, vrais temps et
comptes contrôlés. Sur onde 0,6 m, 32/48 ms coûtent 13,15/9,11 ms moyens par image, mais les
pics calculés dépassent 32 ms et les erreurs synchronisées atteignent 4,445/8,916 mm.
**Refus de généralisation**, afficheur à 16 ms. Deux scénarios, zéro allocation, secteur aux
bornes. 36 tests viewer réussis, un ignoré ; cœur inchangé, reçu S284 conservé.
[Mesures et protocole](../../docs/validation/CADENCE-DELTA-S286.md).

**Limites :** référence 16 ms, hauteur/pente échantillonnées, aucun rendu jugé, pas de borne
matérielle de coût ; refus de solveur représentatif absent donc coût des échecs non qualifié.
Le banc expose sa durée si un refus survient, sans la confondre avec un pas réussi. Ni I-05
ni fidélité des cadences lentes reçus. Aucun ADR modifié, aucun périmètre réduit.

**Suite recommandée :** coût par pas, candidat GPU confronté aux passes CPU, décision
architecturale puis construction reçue avant 3D/deuxième domaine. Le budget ne se résout pas
par des images vides entre des pics. Cadence lente à reprendre quand son intégration temporelle
ou son coût par pas est reçu. A290 revient avant réduction automatique ; V, B2, bathymétrie,
multiplateforme, proposition V1 et verdict S277 conservent leurs déclencheurs.

**Rituel :** file active entièrement relue et états périmés remplacés, feuille de route/index,
A276 et invariants I-04/I-05/I-06/I-12 vérifiés. Une seule copie principale, rien à fermer.
Plan terminé et jeton libre. Aucun gain de fidélité ou de cadence interactive revendiqué.

## S287 — 2026-09-19 — parcours mémoire de la pression

**Entrée :** continuer, A276 coût par pas. Essai CPU borné avant port GPU : boucles
indépendantes parcourues dans l'ordre mémoire, opérateur fin mobile et multigrille.
Instrument ajouté au banc S286 : empreintes hauteur/u/w à toutes les images et itérations.

**Résultat : variante retirée.** A1/B1/B2/A2 sur secteur, six trajectoires strictement
identiques par empreinte, mêmes itérations et zéro allocation. Sur onde16, plages de moyennes
A 24,8542–25,1081 ms et B 24,8082–25,0584 ms se recouvrent ; petit gain de médiane insuffisant
pour recevoir un gain complet robuste. Le cœur est restauré exactement, aucun gain livré.
[Mesures et limites](../../docs/validation/PASSES-PRESSION-S287.md). 36 tests viewer réussis,
un ignoré ; reçu cœur S284 conservé puisque toute modification du cœur a été retirée.

**Choix de suite :** passer à la construction du candidat de pression résidente GPU, contrat
puis opérateur/lissage confrontés au CPU, coûts de transferts/synchronisations inclus ;
réductions/cycle complet et refus avant intégration. Matériel GPU constaté, compute existant,
aucune performance δ GPU acquise. Pas d'ADR acté prématurément. Cadence lente et A290 gardées
avec déclencheurs, V/B2/bathymétrie/multiplateforme et verdict S277 non effacés ; V1 ouverte.
Ce serait le troisième lot de coût : comparer explicitement à la 3D et aux solides absents,
viser une construction consommable plutôt qu'une nouvelle micro-optimisation locale.

**Rituel :** file active entièrement relue, prochain lot remplacé, feuille de route/index
actualisés, A276 suivi ; I-04/I-05/I-06/I-12/I-17 conservés. Maillons = 1, instrument seul ne
valant pas capacité de jeu. Une copie principale, rien à fermer ; plan terminé et jeton libre.

## S288 — 2026-09-19 — première pression sur GPU

**Entrée :** continuer. Troisième lot coût comparé à 3D/solides : construire le chemin moins
cher avant de multiplier les mailles. ADR-172 acte un candidat dans l'hôte wgpu existant,
sans nouvelle dépendance ; cœur toujours sans dépendance, δ toujours cosmétique.

**Construit et reçu :** export préalloué de l'opérateur mobile réel (au bit contre natif),
consommé par opérateur et Jacobi GPU ; jusqu'à32 lissages sans retour CPU intermédiaire.
30 cas, trois tailles, plans/coupés/ondulés, repos exact, erreur normalisée max1,686e-7.
Le test cœur inclut surface couplée distincte de η et refus atomiques. [Preuve](../../docs/validation/PRESSION-GPU-S288.md).

**Coût de la brique :**32 lissages128×52 coûtent1,1020–1,2410 ms, export/empaquetage,
transferts/attentes compris, contre2,5213–2,8444 ms CPU scalaire des mêmes lignes. GPU seul
0,09–0,096 ms. À256×128, complet2,40–2,44 ms ; petite grille défavorable. Secteur aux bornes,
RTX5070/DX12. Préparation sans allocation, pile+banc jusqu'à105 allocations/appel, pics
jusqu'à4,731 ms en régime et5,518 au premier appel. Pas de réception I-05/I-06 image.

**Vérification :**508 tests cœur/harnais +36 viewer réussis,19 ignorés,0 échec ; banc GPU
exécuté explicitement. Deux noms réservés WGSL corrigés pendant construction. Le premier
chronométrage omettait la préparation : élargi avant les chiffres finaux, pas masqué.

**Non-fait et suite :** ni convergence, ni cycle/réductions complets, ni correction des vitesses,
ni trajectoire ou gain interactif. Maillons=2 : le banc consommateur ne vaut pas jeu. Prochaine
capacité : projection GPU intégrable, solveur résident et portes d'acceptation/refus contre CPU,
puis consommation par le pas réel ; ne pas ouvrir une nouvelle micro-optimisation isolée.
Comparaison de priorité :3D/solides restent obligatoires, A276 les précède pour éviter une
augmentation de coût déjà incompatible. A290/cadence/V/B2/bathymétrie/multiplateforme,
proposition V1 et verdict S277 gardent leurs déclencheurs. Un troisième maillon nécessiterait
justification explicite : ne pas déclarer cette projection intégrée avant preuve.

**Rituel :** file active entièrement relue, état courant remplacé, feuille/index et A276
actualisés, ADR nouveau indexé ; I-04/I-05/I-06/I-13/I-17 préservés (aucun état δ persisté).
Plan terminé, jeton libre, copie principale unique et aucune fermeture nécessaire.

## S289 — 2026-09-19 — le pas réel consomme une pression calculée sur GPU

**Entrée :** reprendre le projet ; suite A276 déclarée par S288 — solveur résident, réductions
et cycle, acceptation/refus CPU, puis consommation par le pas réel.

**Ce qui devient possible.** Le coût de la projection de δ ne croît plus avec le nombre
d'itérations que le CPU doit payer. **Le chemin qui le consomme** est le pas mobile réel,
`step_surface_mobile_with` — le même que pilote la bande δ de l'afficheur. **La preuve** :
[PRESSION-RESIDENTE-S289](../../docs/validation/PRESSION-RESIDENTE-S289.md). À 6 656 mailles,
médiane du pas 9,2236 → 6,6220 ms et itérations du cœur 427 → 19 sur 60 pas ; à 32 768,
44,8697 → 30,2744 et 353 → 61. Gain ×1,39 à ×1,50.

**Décision structurante — [ADR-173](../../docs/adr/ADR-173-le-candidat-de-pression-ne-fournit-qu-un-depart.md).**
Un candidat externe ne fournit **qu'un départ**. Le cœur recalcule `b − A·p` avec son opérateur
et garde ADR-143/144 inchangés ; un candidat non fini, mal formé ou déclinant est refusé
atomiquement, départ restauré au bit. Pourquoi ce découpage et pas une délégation : les trois
portes du cœur demandent chacune un rapatriement par test, soit exactement ce qu'un solveur
résident existe pour supprimer — et un candidat faux devient alors inutile, jamais dangereux.
C'est ce qui permet d'activer un solveur GPU sans réception physique du GPU.

**Construit.** Cœur : `PressureCandidate`/`PressureProblem`/`ExternalPressure`,
`project_with`, `step_surface_mobile_with` ; `None` reproduit le pas historique au bit. Hôte :
`pressure_cg.wgsl` et `pressure_solver.rs` — gradient conjugué préconditionné dont opérateur,
réductions d'arbre, `α` et `β` vivent sur la carte, aucun retour CPU entre itérations.

**Reçu.** Réductions contre le CPU ≤ 2e-7 en relatif (critère 1e-5) ; convergence identique au
miroir CPU du même cycle, rapport des vrais résidus (arbitrés en f64) 1,0000 à 8 et 32
itérations, 0,964 à 1,094 partout ; repos exact ; 60/60 propositions retenues, 0 refus, 0 pas
dégradé, dérive de surface 0 à 7,63e-6 m — huit cas sur douze au bit. 508 essais cœur/harnais
et 36 viewer réussis, 21 et 1 ignorés, 0 échec ; deux bancs GPU exécutés explicitement.

**Chiffres qui ont orienté la suite.** L'appel du candidat coûte 4,1989 ms à 6 656 mailles dont
**0,2606 d'empaquetage et 3,9333 d'encodage-soumission-attente**, pour 1,150 ms de calcul réel :
27 % de temps utile. Le poste dominant est **l'enregistrement des commandes** — 7 dispatchs par
itération, 896 à 128 — et c'est la même cause que les allocations (82 à 0 itération, ≈ 7 par
itération, 990 à 128), toutes dans l'encodage de la pile graphique.

**Non-fait, limites.** Budget eau de 2 ms non reçu (6,62 ms, ×3,3). I-06 du chemin d'image non
reçu : le cycle alloue, ADR-145 ne l'admet pas — le pas le consomme, la boucle d'image non.
Le pire pas ne suit pas la médiane : il baisse à 32 768 mailles, il monte légèrement à 6 656,
dans la variance. La longueur du cycle est un réglage non calibré et non automatique : 32 et
256 itérations perdent tous deux à 6 656 mailles, seul 128 gagne. Préconditionneur Jacobi
seulement : S244 tient toujours que la multigrille est le seul levier dont le gain croît avec
la taille. Une seule carte, un seul backend. Ni 3D, ni solides, ni A290, ni famine.

**Impasse mesurée, à ne pas refaire.** Retirer le second aller-retour de cartographie
(horodatage GPU) ne donne aucun gain mesurable : 4,31 → 4,18 ms à 6 656 mailles mais
7,42 → 8,09 à 32 768, dans la variance.

**Découpage corrigé en cours de route.** L'étape « exporter le second membre et l'inverse de la
diagonale » annoncée au plan n'existe pas : le crochet passe `rhs` au candidat, et l'inverse de
la diagonale se déduit exactement des lignes déjà exportées par S288. Une étape de moins au
plan, aucune de moins faite. Une mesure d'allocations fautive — le compteur ne suivait que
`propose`, pas `solve` — a été corrigée **avant** publication, pas après.

**Prochaine capacité visée.** Faire tenir le cycle dans peu de dispatchs, ou l'enregistrer une
fois : c'est le même geste qui lève les allocations d'ADR-145 et les 73 % de temps perdu. Le
plafond de ce levier est ≈ ×3 sur l'appel. Ensuite seulement, la comparaison avec la 3D et les
solides se rejoue sur un coût de projection à jour.

**Maillons : remis à zéro.** Une capacité est reçue, son chemin la consomme, sa preuve existe.

**Rituel :** file active entièrement relue, états périmés remplacés, feuille de route et index
actualisés, ADR-173 indexé, ADR-172 laissé intact (sa suite est ADR-173, pas une correction).
I-04/I-05/I-06/I-13/I-17 : aucun état δ persisté, aucune porte déplacée, I-06 explicitement
non revendiquée. Plan terminé, jeton libre, copie principale unique, aucune fermeture requise.

## S290 — 2026-09-19 — l'enregistrement des commandes, poste dominant d'un appel GPU

**Entrée :** continuer ; A292, déclarée par S289 — le coût d'appel du cycle résident.

**Ce qui devient possible.** Le pas de δ coûte **4,6315 ms au lieu de 9,9248** à 6 656 mailles,
et son **pire** pas tombe de 26,3234 à 11,0477 ms quand le cycle est long. **Le chemin qui le
consomme** est le pas mobile réel, par les défauts du solveur — tout consommateur en bénéficie.
**La preuve** : [ENCODAGE-CYCLE-S290](../../docs/validation/ENCODAGE-CYCLE-S290.md).

**La mesure qui a décidé du lot.** S289 publiait « 3,9333 ms d'encodage-soumission-attente pour
1,150 de carte » sans les séparer. Décomposés : empaquetage 0,2768, **enregistrement 2,2163
(52 %)**, soumission 0,1925, attente 1,3663, recopie 0,0058. L'attente *est* le calcul ;
l'enregistrement est du temps CPU pendant lequel la carte ne fait rien. Une sonde qui enregistre
des dispatchs puis **jette** le tampon a nommé le coupable : `dispatch_workgroups` à **1,86 µs et
une allocation par appel**, `set_pipeline` n'ajoutant que 0,38 à 0,44 µs — et les deux sont
indépendants de la taille de grille.

**Construit.** Trois encodages du **même** calcul : 7 dispatchs par itération (S289), 5 (chaque
réduction repliée dans le noyau qui produit ses valeurs), **3** (les deux dispatchs à un seul
groupe disparaissent — chaque groupe refait la somme lui-même, sur des valeurs écrites par le
dispatch précédent, donc visibles par simple frontière de dispatch). Plus la soumission par
tranches, qui laisse l'encodage d'une tranche recouvrir l'exécution de la précédente. Défauts :
3 dispatchs, tranches de 32.

**Reçu.** 72 combinaisons comparées au chemin de S289 : **zéro bit d'écart** sur la pression
**et** sur les deux diagnostics. Appel : 3,9748 → 1,8228 ms à 6 656/128 (×2,18), 10,6282 → 5,8590
à 32 768/256 (×1,81). Pas réel, deux encodages contre le **même** témoin dans la **même**
exécution : 9,9248 → 7,0597 (S289) → **4,6315** (S290) à 6 656/128 ; 45,0679 → 30,2715 →
**26,1125** à 32 768/256. 24 combinaisons, 60/60 propositions retenues, 0 refus, 0 pas dégradé,
dérive ≤ 7,63·10⁻⁶ m. Vérification : 394 essais de la bibliothèque du cœur et 36 essais viewer
réussis, 0 échec, plus les trois bancs GPU exécutés explicitement. Les essais d'intégration du
harnais n'ont **pas** été rejoués : S290 ne touche que `viewer/`, et leur dernier passage est celui
de S289 (508 au total, 0 échec). Dit ainsi pour ne pas reporter un décompte non remesuré.

**Deux voies abandonnées sans être construites** : réduction finale par le dernier groupe via
compteur atomique (parie sur une visibilité inter-groupes que WGSL n'énonce pas nettement) et
récurrence `q = A·z + β·q` (Chronopoulos/Gear, moins stable en f32). La troisième voie, apparue
en construisant, était à la fois plus rapide et sans risque. **Refaire un petit calcul
redondamment coûte souvent moins que de le synchroniser** — L340.

**Une erreur de fait de S289, corrigée par note datée.** S289 donnait deux raisons à ce lot, dont
« les allocations qu'ADR-145 interdit à la boucle d'image ». ADR-145 §1 lit I-06 sur **le code du
projet** et §2 décide que les allocations des dépendances verrouillées sont **comptées et
publiées, non interdites**. L'obstacle était le temps, pas une règle violée. Notes correctives
datées dans ADR-173 et dans la preuve de S289. Le plafond « ≈ ×3 » de S289 était également faux —
il traitait tout le non-calcul comme récupérable ; il valait ≈ ×2,1, avant que les tranches ne
déplacent la borne.

**Deux mesures fautives, corrigées avant publication et non après.** (1) L'horodatage GPU n'était
payé que par les variantes à un seul tampon : le gain des tranches était surestimé d'environ
0,5 ms, et deux configurations identiques par construction affichaient 2,2454 contre 1,6761 ms.
(2) La fenêtre d'I-06 sur notre empaquetage englobait les `write_buffer` de wgpu et comptait
19 allocations attribuées à notre code ; resserrée, elle rend zéro, et le banc refuse sinon.
Enfin, dix passages laissaient des aberrations peser sur les médianes : trente désormais.

**Non-fait, limites.** Budget 2 ms non reçu (×2,3, contre ×3,3 en S289). Boucle d'image non
activée — il faudra y remesurer la référence « 133 allocations par image » d'ADR-145 et décider
du recouvrement. Mode et longueur de cycle non calibrés, non automatiques. **Croisement mesuré** :
le recalcul redondant coûte `groups²` lectures, si bien qu'à 512 groupes le mode à 3 dispatchs
perd sur la carte contre le mode à 5 (1,6009 contre 1,3354 ms) — il reste retenu parce que
l'encodage baisse plus, mais à grille plus grande il faudra choisir le mode par la taille, ou une
somme à deux niveaux. Une seule carte, un seul backend, une version de wgpu : 1,86 µs par dispatch
est une référence **datée**, comme les 133 d'ADR-145. Toujours Jacobi, donc S244 tient.

**Pas de nouvel ADR, et pourquoi.** Rien n'est décidé ici qu'ADR-172 et ADR-173 ne portaient déjà :
le calcul est au bit identique, les portes sont intactes, aucune dépendance n'est ajoutée. Les
faits nouveaux — croisement, réglages non calibrés — sont des points de file, pas des décisions.

**Prochaine capacité visée, et l'avertissement du troisième maillon.** S289 et S290 portent le
même sujet ; **une troisième session de micro-optimisation GPU serait un approfondissement
différable**, parce que la pression n'est plus la majorité du pas : 4,6315 ms de pas pour 2,1271
d'appel, donc **≈ 2,5 ms que personne n'a cartographiés** — S244 n'avait mesuré que la boucle de
pression. La suite utile est donc de cartographier ce que le pas dépense **hors** pression, puis
de rejouer là-dessus la comparaison de priorité avec la 3D et les solides, dont A292 était
justement le déclencheur.

**Maillons : 0.** Capacité reçue, chemin qui la consomme, preuve.

**Rituel :** file active entièrement relue, états périmés remplacés, feuille/index actualisés,
nouveau point A293 ouvert avec déclencheur, notes correctives datées posées, leçons L339 et L340
écrites. I-04/I-05/I-06/I-13/I-17 : aucun état δ persisté, aucune porte déplacée, I-06 vérifiée
sur notre code et refusée au banc si elle cède. Plan terminé, jeton libre, copie unique.

## S291 — 2026-09-19 — le pas décomposé, et un cycle multigrille qui partait à la poubelle

**Entrée :** demande de l'utilisateur — décomposer le temps par étape de calcul, trouver où il
part, chercher les erreurs et les calculs redondants, expérimenter, proposer ; garder un rendu
visuellement valide et un temps de réponse court. Cela recouvre A293.

**Ce qui devient possible.** Sur le chemin que la bande δ de l'afficheur emprunte réellement, à
sa grille de production (128 × 52, dx 2 m), le pas coûte **8,27 ms au lieu de 17,93** et son pire
pas 16,11 au lieu de 28,22. **Le chemin qui le consomme** est `step_perturbation_mobile_with`,
construit ici. **La preuve** : [PAS-DECOMPOSE-S291](../../docs/validation/PAS-DECOMPOSE-S291.md).

**Le rendu n'a pas changé d'un bit.** Les six empreintes de trajectoire publiées par S287 sont
rendues à l'identique, sommes d'itérations comprises — et ce banc emprunte le chemin couplé.

**Ce que la mesure a trouvé et qu'aucune relecture n'avait vu.** Le préconditionneur était
appliqué **avant** la boucle du gradient conjugué. Or depuis S289 le candidat GPU converge à
presque tous les pas, la boucle ne tourne pas, et ce **cycle multigrille complet partait à la
poubelle** : 0,8291 ms sur 5,7937, le plus gros poste hors GPU. Différé à la première itération
qui l'emploie, il coûte 0,0121 ms. Trois sessions avaient lu ce code sans le voir ; une mesure
par étape l'a vu au premier passage.

**Construit.** Instrument : `Control` attribue chaque segment à huit `Phase` publiques **et** à
dix-huit `Stage` internes, les deux sommes valant `elapsed()` — paire vérifiée à chaque exécution,
refus si elle diverge. `Phase` n'a pas gagné de variante, pour ne pas casser son exhaustivité de
S230. Quatre suppressions au bit : amorçage différé, `‖b‖²` réduit une fois au lieu de deux,
correction et divergence non refaites quand la porte d'ADR-144 vient de les produire, double
sondage retiré de l'advection. Validation aplatie : mêmes onze champs, même ordre, même grain,
forme vectorisable — 0,4189 → 0,1031 ms. Erreur inverse du rapport rendue optionnelle (`NaN`
quand coupée), 0,3379 ms, sans toucher au certificat d'arrêt d'ADR-143.

**Refusé explicitement.** Retirer du contrôle de finitude les six tampons non publiés : `dir`
n'est pas réécrit sur les mailles solides, l'argument demandait un raisonnement par tampon, et
une erreur y rendrait un `NaN` silencieux. Le gain était déjà pris sans ce risque.

**Une imprécision de S289, corrigée.** S289 écrivait « le même que pilote la bande δ de
l'afficheur ». La bande passe par `step_perturbation_mobile` (couplé, S253), pas par
`step_surface_mobile` : même fonction du cœur, pas même chemin de l'hôte, et le crochet n'était
donc pas atteignable par le rendu. Corrigé en le portant sur le chemin couplé, et par note datée.

**Ce que S289 et S290 cachaient.** Leurs mesures employaient une horloge **figée**. Avec une
horloge réelle, le sondage coûte **+8,0 à +11,0 %** selon le cas : leurs chiffres sous-estiment
la production d'environ 8 %. Le grain n'a pas été élargi — chaque sondage est un point
d'expiration, et en retirer relève d'un arbitrage sur ADR-007.

**Le pic et la médiane ne désignent pas le même réglage.** À 6 656 mailles, cycle 128 donne la
meilleure médiane (3,7983 ms) pour un maximum de 25,7902 ; cycle 256 donne 4,8067 et **8,0980** —
trois fois moins. À 384 le cœur ne fait plus aucune itération. Pour un rendu, c'est le pire pas
qui décide, et rien n'arbitre encore entre les deux.

**Le défaut qui compte plus que le gain.** Un pas à **467 ms**, puis 132 à la reproduction,
toujours au premier régime GPU venant après sept secondes sans GPU. Un envoi de préchauffage
**ne le supprime pas** ; ne pas laisser la carte refroidir le supprime (maximum retombé à
26,12 ms). C'est un chemin GPU **refroidi**, pas neuf. Pour un rendu c'est un gel visible, et il
suffit que la bande ait été éteinte un instant. Non corrigé : garder la carte tiède est un
arbitrage d'ordonnancement, pas un réglage. C'est le point le plus important laissé ouvert (A294).

**Non-fait, limites.** Budget 2 ms non reçu (×4,1 sur le chemin de l'afficheur). Longueur de
cycle non calibrée et non automatique. Tenue du pire pas non garantie. Une seule carte, un seul
backend. Ni famine, ni A290, ni topologie violente. Ni 3D ni solides.

**Deux corrections d'instrument, avant publication et non après.** Deux colonnes de détail du
balayage étaient indexées sur les mauvaises étapes. Et la fenêtre d'I-06 sur notre empaquetage
englobait les `write_buffer` de wgpu : resserrée, elle rend zéro, et le banc refuse sinon.

**Vérification.** 508 essais cœur/harnais réussis (394 + 16 + 2 + 1 + 95), 21 ignorés, 0 échec ;
36 essais viewer réussis, 1 ignoré, 0 échec ; six empreintes de S287 identiques.

**Pas de nouvel ADR, et pourquoi.** Rien de décidé ici qu'ADR-173 ne portait déjà : le calcul est
au bit identique, les portes sont intactes, et le crochet posé sur le pas couplé est le même
mécanisme appliqué au chemin que ce mécanisme visait. Le diagnostic optionnel est un réglage
d'hôte, pas une décision de conception. Les faits nouveaux sont des points de file.

**Prochaine capacité visée.** La comparaison de priorité avec la **3D et les solides** se rejoue
maintenant : c'était ce qu'A293 devait débloquer, et le pas a désormais tous ses postes connus.
Avant cela, un seul point est plus urgent que tout réglage : **le gel de la carte refroidie**
(A294), parce qu'un pic de 467 ms annule tout gain de médiane pour un rendu.

**Maillons : 0.** Capacité reçue, chemin qui la consomme, preuve.

**Rituel :** file active entièrement relue, A293 close, A294 ouverte avec déclencheur, note
corrective datée posée sur la preuve de S289, feuille/index/REPRISE actualisés, leçons L341 et
L342 écrites. I-04/I-05/I-06/I-13/I-17 : aucun état δ persisté, aucune porte déplacée, et les
huit points d'expiration de S230 restent atteignables — vérifié par son essai.


## S292 — 2026-09-19 — le gel de la carte refroidie ne se reproduit pas sur l'appel isolé (interrompue)

**Claude Opus 5 (Claude Code desktop).** Entrée : régler le gel de la carte refroidie (A294),
demande de l'utilisateur. Plan 935919b ; étape c629ebb. **Interrompue par l'utilisateur après
P2/P3**, puis close par S293, qui ouvre l'audit global demandé : P4–P7 non faits, rituel compris.

**Établi.** Banc `--pas-froid` — huit appels dos à dos, pause, puis l'appel froid décomposé en
ses cinq postes, trois répétitions par durée ; 6 656 mailles, cycle 128, horodatage éteint. Le
refroidissement existe mais vaut **×1,4 à ×2, pas ×200** : appel froid 2,29 → 4,84 ms en pause
endormie, 3,06 → 4,35 ms à processeur saturé, de 0 à 8 s de pause ; il apparaît vers 2 s et
sature ensuite. Il tombe dans l'**attente** (0,32 → 1,77 ms processeur occupé, 0,32 → 0,78
endormi), donc côté carte ou pilote, pas dans notre encodage ni notre empaquetage. Cohérent avec
un état d'alimentation ; non tranché, P4 existait pour cela.

**Conséquence.** Le pic de 467 puis 132 ms de S291 n'est pas un appel refroidi du solveur : il a
été vu dans le **pas complet** de `--pas-couple`, qui diffère du banc isolé par un `Volume` neuf
par régime (réserve de 64 Mo, défauts de page au premier pas), une pause remplie de 400 pas
couplés, et un pas qui contient bien plus que l'appel. L'hypothèse de S291 est fausse ou
incomplète : **aucun entretien de la carte ne doit être bâti dessus.**

**Écrit, compilé, jamais exécuté.** `--pas-couple` garde la carte du pire pas (`PIC_S292` :
étapes du cœur et cinq postes de l'appel), ordre des régimes remis à celui de S291. Commande :
`cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --pas-couple`.

**Maillons : 1** — mesure sans capacité reçue. Rituel porté par S293 : A294 actualisée dans la
file avec ce geste pour suite.

## S293 — 2026-09-19 — où l'avancement bloque : audit global

**Claude Opus 5 (Claude Code desktop).** Entrée : « analyser le projet et voir où cela bloque
dans l'avancement, regarder l'intégralité de ce projet », à 14:19, S292 venant d'être
interrompue par l'utilisateur. master c629ebb, copie unique, aucun distant. Plan 441eecc ; étapes
c9bd2c2 (S292 close), 7401568, 9cf5e5a, 49a4dfa, 49c2f94, 48f0d2c.

**Livrable** : [BILAN-GLOBAL-S293](../../docs/registres/BILAN-GLOBAL-S293.md). **Réponse : le projet
ne bloque ni sur son code ni sur une impossibilité technique, mais sur l'ordre de ses travaux.**
Suites vertes (511 cœur/harnais + 36 afficheur, 0 échec) ; 3 points sur 120 validés ; portes B
et D non commencées. Cinq blocages techniques : la porte B retenue par un déclencheur interne
sans dépendance (« A276 avant la 3D », S249/S276) alors que la porte C se reçoit sur sa scène ; la
famille de δ non choisie (B3), le seul candidat n'exécutant aucun de ses scénarios ; une
architecture d'exécution de δ qui garde du travail `O(N)` sur CPU à chaque pas (A295 — 30 à
120 ms extrapolés pour 64×64×32, estimation) ; un budget de 2 ms sans part pour δ ni cible
(A296) ; aucun objet pilotable ni corps rigide. Trois de pilotage : dix suites chaînées sur dix
depuis S283, à travers deux agents (cinquième constat du mécanisme) ; une classe de fidélité
uniforme appliquée à une couche cosmétique ; des documents d'état regonflés depuis S227 (file ×6
en mots, feuille de route ×3,3).

**Faux blocages** : A278 (l'hôte contient déjà du `unsafe`, et un vivier sûr existe — hypothèse à
éprouver) ; A294 ne concerne que δ GPU dans la boucle d'image.

**Non-fait, volontairement** : aucun code, aucune porte ni seuil déplacé, aucun ADR — A295 et le
lot 1 attendent la lecture de l'utilisateur ; la liste du projet fini n'est pas recomptée (elle se
remplit à sa demande ; décompte périmé d'environ trois points signalé). Estimations 3D non
mesurées. 141 fichiers de la copie de travail ont des fins de ligne CRLF ou mixtes, normalisées
par Git au commit : sans effet sur le dépôt, non traité.

**Arbitrages demandés à l'utilisateur** (bilan §6) : Q1 cible matérielle et part de δ dans les
2 ms ; Q2 v1 (en attente depuis S281) ; Q3 dépôt distant ; Q4 porte B avant la suite du coût en 2D,
et pas de δ résident sur GPU à travail borné ; Q5 onde de S277.

**Maillons : 2** — S292 mesure, S293 audit. La suivante doit viser une capacité : porte B ou
scène-témoin de la v1, après le lot 1 du bilan.

**Rituel** : A211/A243 suivi, A295 et A296 ouverts ; L343 écrite, note datée sur L342 ; file
active (décisions en attente regroupées, A276 contesté, A278, A211, A295, A296), feuille de route
§3 bis, REPRISE §4, index (ADR-170 à 173 manquants depuis S278, ajoutés : navigation à 0 erreur), deux
cellules périmées de REVUE-VISUELLE §5. Invariants relus : I-04,
I-05, I-13 ; aucun amendé. Copie principale seule, rien à fermer.

## S294 — 2026-09-19 — les arbitrages consignés, δ 3D décidé, le pilotage attaché aux portes

**Claude Opus 5 (Claude Code desktop).** Entrée : réponses de l'utilisateur aux questions de
BILAN-GLOBAL-S293 §6 (citées dans ADR-174 §1). master aa48867. Plan 48499f1 ; étapes 0a8d9a0,
7572c95, a4e13e6, b73def0, c71720d, b276d69.

**Décisions de l'utilisateur, consignées** ([ADR-174](../../docs/adr/ADR-174-arbitrages-du-2026-09-19.md)) :
machine de référence = ce poste (AERO X16, Ryzen AI 7 350, RTX 5070 Laptop) ; le temps de l'eau sert
l'objectif — « de l'eau d'un jeu en temps réel, dynamique à son environnement et aux joueurs » — et
devient un profil de travail : eau ≤ 4 ms GPU et ≤ 2 ms CPU par image, dont **δ ≤ 2 ms GPU**, à
calibrer par B7 ; v1 = porte D franchie ; aucun dépôt distant ; porte B avant la suite du coût en
2D ; onde de S277 sans verdict attendu. A296 close : la porte C a un critère atteignable.

**Décision technique** ([ADR-175](../../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md), A295) :
δ en 3D avec deux implémentations — la référence CPU dans le cœur, hors de la boucle d'image, et une
production résidente sur GPU à travail borné, diagnostics relus en différé, dégradation déclarée ;
fidélité par couche ; MAC x-y-z à surface fonction-hauteur, le non graphe réservé à une seconde
représentation. Relire les contrats a montré que c'était le dessin de S01/S04 (ADR-007 §4.1,
SPEC-004 §4 et §8.4, ADR-012 §7), et que le chemin S289–S291 attendait la carte à chaque pas, contre
SPEC-004 §8.4 — note datée sur ADR-173. Critères de la porte B posés avant construction (§4).

**Pilotage** : `Session suivante` se prend dans la porte en cours que désigne §3 bis — B, avec D en
parallèle, dépendances écrites ; pas de troisième session consécutive sur un point sans critère de
porte avancé ; une capacité compte si elle avance un critère « reçu si » ou un point de la liste.
Déclencheur « A276 avant la 3D » remplacé (L343). Une première rédaction disait « porte ouverte de
plus petit rang », ce qui renvoyait à A : corrigée dans la même session.

**Documents d'état** : plafonds de 90 mots par ligne de file et 450 par section de jalon, contrôlés
par `outils/etat_projet.py --check` (fonction `oversized`, un essai ajouté, 5 essais verts). File
active réécrite par porte : 5 806 → 3 030 mots ; feuille de route 41,9 → 20,5 ko ; REPRISE §4
ramené à l'état présent. Navigation 0 erreur, plafonds 0 dépassement.

**Non-fait** : aucun code de solveur ; A294 non exécuté ; la liste du projet fini n'est pas
recomptée (à la demande de l'utilisateur). Deux battements de jeton écrits avant la lecture de
l'horloge, corrigés au commit suivant (L237).

**Maillons : 0** — par la clause « décision qui lève un blocage et nomme le lot exécutable » : la
porte B n'avait ni architecture ni droit de commencer, elle a les deux, et son lot 1 est nommé.
**S295 doit recevoir un critère de porte**, sinon la clause aura servi à rien.

**Rituel** : A295 décidée, A296 close ; file, feuille de route §2 et §3 bis, REPRISE §4 à §8,
METHODE, index (ADR-174, 175), REVUE-VISUELLE (onde), note sur BILAN-GLOBAL-S293. Invariants
relus : I-03, I-04, I-05, I-06, I-08, I-12, I-13, I-14, I-17 ; aucun amendé. Copie unique.

## S295 — 2026-09-19 — porte B, lot 1 : la référence δ tridimensionnelle existe

**Claude Opus 5 (Claude Code desktop).** Entrée : suite désignée par S294, porte en cours B.
master f030f40. Plan 217840f ; étapes 3138ddf, 7f82956, e970e64, c3473ab.

**Ce qui devient possible.** δ a une **référence en trois dimensions** : `code/water-core/src/delta3d.rs`,
grille MAC x-y-z, surface linéarisée (ADR-141 en 3D), la 2D intacte. **Le chemin qui la consomme** :
les réceptions de la porte B (ADR-175 §4.1), et demain la production GPU qui sera jugée contre
elle. **La preuve** : [DELTA3D-LINEAIRE-S295](../../docs/validation/DELTA3D-LINEAIRE-S295.md).

**Reçu, contre des critères posés avant le code.** (1) À `ny = 1`, la trajectoire est **identique
au bit** à celle de la 2D de S233 — hauteurs et itérations, 500 et 1 000 pas — et l'opérateur l'est
sur un champ quelconque ; (2) une hauteur indépendante de `y` le reste **exactement** ; (3) l'onde
stationnaire oblique (1, 1) d'une cuve 8 × 4 × 4 m tient sa dispersion : 2,32 → 0,54 → **0,176 %**
de `A` en raffinant, ordre deux en espace, et **0,005 %** contre la fréquence du schéma calculée hors
du solveur ; (4) repos exact, zéro allocation arène scellée, refus atomique rendant six champs au
bit. `γ₁₀` dérivé pour six faces (ADR-143). Suite complète : 523 essais cœur/harnais (408 + 17 + 2 +
1 + 95), 36 afficheur, 0 échec, 19 ignorés.

**Une erreur d'oracle, pas du solveur** : sans le demi-pas d'Euler symplectique, l'oracle « schéma »
laissait 0,29 % ; corrigé dans l'oracle, dérivé de la récurrence (L344).

**Non-fait, limites.** Surface mobile, couplage B/W, production GPU, scène et revue, coût. Pas de
préconditionneur — un Jacobi briserait l'invariance en `y` à l'arrondi ; la multigrille viendra
avec la production. 16,5 ms par pas à 27 648 mailles sur un fil : c'est une référence, hors de la
boucle d'image. La liste du projet fini (4.1) n'est pas recochée : elle se remplit à la demande de
l'utilisateur.

**Maillons : 0.** Capacité reçue, chemin qui la consomme, preuve — et un critère de la porte B avance.

**Rituel** : L344 ; file (porte B), feuille de route (J2, porte B), REPRISE §4, index. Invariants
relus : I-04, I-06, I-07 (`g_eff` fourni), I-08 (f32, durée entière, coefficients seuls arrondis),
I-17 ; aucun amendé. Copie unique.

## S296 — 2026-09-19 — porte B, lot 2 : surface mobile de la référence 3D reçue

**Entrée et reprise.** Claude Opus 5 a posé le plan et les critères (`7c5180b`), puis passé le
jeton à Codex à la demande de l'utilisateur. Codex reprend sur « reprend le projet » : copie
unique, branche parallèle archivée, diff vide ; aucune étape à annuler. P2–P6 terminés.

**Capacité reçue.** Une surface graphe se déplace non linéairement dans la référence MAC x-y-z :
fantômes verticaux et latéraux, projection Jacobi à départ chaud, advection, extrapolation et
débits mouillés conservatifs. **Consommateur** : référence de réception de la porte B,
ADR-175 §4.1, qui prépare le couplage B/W puis juge la production GPU.
**Preuve** : [DELTA3D-MOBILE-S296](../../docs/validation/DELTA3D-MOBILE-S296.md).

**Résultats.** À `ny=1`, hauteurs, pressions, u/w et itérations **identiques au bit** à la 2D
sur une période, 32 colonnes, amplitudes 5 et 10 cm. Contre HOS à 128 colonnes : profil
**0,253 % / 0,224 %**, harmonique **0,704 % / 0,427 %**, décroissants en raffinant.
Onde oblique fine : **0,253 %** contre la solution linéaire analytique, phase **0,065°**,
écart au mode linéaire discret <0,10 %. Transposition x↔y : un ulp. Repos non aligné exact,
zéro allocation, refus atomiques avant/après transport et sur non-convergence. Suite
cœur/harnais : **530 réussis, 18 ignorés**, puis test matriciel supplémentaire reçu (531 au
total), aucun échec. Afficheur inchangé, non retesté. Aucune tolérance déplacée.

**Limites, suite.** Fond plat, surface graphe, une période ; pas de couplage B/W, production
GPU, rendu ni coût reçu. A274 (plancher des lignes fantômes) demeure ouverte ; aucune loi
universelle déduite des cas reçus. **S297 : lot 3 de la porte B, couplage B/W et frontières**,
réceptions S253/S268–S274 à `ny=1` et invariance transverse. Recommandation S293 maintenue :
avancer la 3D avant le coût, pas de retour au fil d'optimisation 2D. V, B2, bathymétrie et
multiplateforme restent dans la file, sans réduction d'ambition ni nouvel arbitrage.

**Maillons : 0.** La référence de la porte B gagne une capacité reçue. Invariants relus :
I-03 à I-08, I-12, I-13, I-17 ; aucun amendé. Pas de nouvel ADR ni de leçon forcée.
Rituel : file active entière relue, état J2/porte B remplacé, index et passation actualisés,
copie unique conservée ; navigation et plafonds vérifiés.

## S297 — 2026-09-19 — référence 3D couplée et premier aperçu animé

**Entrée.** « Continue, et j'aimerais pouvoir voir après », puis deux « continue » pendant le
travail. Copie unique, master propre ; plan committé avant code. Agent Codex GPT-6 desktop.

**Capacité reçue.** La référence MAC 3D résout maintenant la perturbation du fond B/W :
source et advection croisée, surface totale et fantômes, affinage au plancher, bandes aux quatre
bords, relaxation de hauteur. **Consommateurs** : banc HOS et aperçu du véritable pas 3D,
puis référence qui jugera le pas GPU de la porte B (ADR-175 §4.1).
**Preuve** : [DELTA3D-COUPLEE-S297](../../docs/validation/DELTA3D-COUPLEE-S297.md).

**Mesures.** HOS à 128 colonnes : profil 0,148 % / 0,178 % pour 5 / 10 cm, harmonique
0,346 % / 0,532 %, décroissants avec le raffinement. L'affinage est réellement exercé.
Fond nul identique au bit au mode mobile ; invariance transverse et rotation à un ulp ;
courant uniforme traversant sans perturbation créée ; refus atomiques et zéro allocation.
Suite complète : **537 réussis, 18 ignorés**, aucun échec ; afficheur inchangé.

**Visible.** GIF local de 121 images, 6 s physiques : impulsion dans deux ondes analytiques
croisées ; surface totale à échelle réelle et différence au témoin amplifiée ×4. Inspection
de plusieurs PNG et ouverture dans Codex ; habillage de banc explicite. Aucun état δ sérialisé.
Ce n'est ni la mer spectrale de production ni un verdict perceptif sur la porte B.

**Partiel et suite.** Bandes/éponge construites, réception absorption et houle progressive
S269–S274 encore due. Fournisseurs réels B/W, production GPU résidente, rendu dans la mer
étalée et revue à faire. La suite reste dans la porte B : compléter ces réceptions en préparant
la production GPU et sa surface publiée, pour passer de l'aperçu demandé à la scène interactive.
Pas de retour au coût 2D : recommandation S293 maintenue. V, B2, bathymétrie, multiplateforme
restent dans la file ; aucun retrait d'ambition ni arbitrage nouveau.

**Rituel.** Maillons **0** : un critère de référence de la porte B avance, avec consommateur
et preuve. A295 actualisée, A274 ouverte ; pas de leçon forcée. File active entière relue,
feuille de route et index remplacés/complétés. I-04/I-06/I-07/I-08/I-12/I-13/I-17 inchangés.
Navigation/plafonds vérifiés, copie unique, jeton libre. Deux battements intermédiaires ont été
reportés une à deux minutes trop tard par erreur de saisie (P1/P3) ; battement final relu
séparément et exact, sans conséquence de concurrence constatée.

## S298 — 2026-09-19 — porte B : frontières reçues en 3D, fond réel branché, et la maille qui manque

**Entrée.** « Continue » à Codex GPT-6, puis « Reprends le projet » à Claude Opus 5 après coupure.
**Reprise à chaud** : jeton `occupé`, battement de six minutes, mais interruption signalée par
l'utilisateur. Diff de P4 jugé cohérent avec la thèse déclarée — **complété, non annulé**. Copie
unique, master propre. P1–P3 étaient de Codex ; P4 à P6 de Claude.

**Capacité reçue.** La référence 3D reproduit les réceptions de frontières de S269–S274 — houle
progressive, paquet, mur et éponge, sur chaque axe — et consomme désormais le **fournisseur
spectral B du cœur** au lieu d'ondes analytiques choisies : `BackgroundGrid3` échantillonne B sur
les trois familles MAC par `differential_grid_extended`, scratch réservé avant scellement, zéro
allocation, publication atomique. **Ce qui devient possible** : la référence CPU peut juger le pas
GPU d'ADR-175 §4.2 sur les entrées réelles de la production, pas sur une fixture de banc.
**Consommateurs** : `delta3d_boundaries`, l'aperçu 3D, puis la production GPU.
**Preuve** : [DELTA3D-FOND-REEL-S298](../../docs/validation/DELTA3D-FOND-REEL-S298.md).

**Mesures.** Frontières : 139 200 pas acceptés, aucun refus, écart de hauteur maximal
**1,192093·10⁻⁷ m** tous cas et axes confondus ; erreur énergétique de jauge ≤ 1,59·10⁻⁵.
Fond réel : 64 composantes de deux systèmes JONSWAP directionnels, atténuation au fond
2,259·10⁻⁵, vitesse verticale résiduelle **1,194·10⁻⁶ m/s**, échantillons identiques au bit au
ponctuel prolongé aux trois familles et à trois instants. Suite complète : **539 réussis**,
18 ignorés, aucun échec ; afficheur inchangé.

**Visible, et ce que le regarder a trouvé.** Deux GIF locaux de 121 images. Le premier montre δ
sous une mer réellement étalée — 79° entre les deux systèmes — pour la première fois. En le
vérifiant : le champ de différence dégénérait en damier à l'échelle de la maille et **remontait**
après 3,3 s (0,047 → 0,091 m). Cause mesurée : la fixture portait sa composante la plus courte sur
**3,92 mailles**. Rejouée à **8,30 mailles**, tout le reste identique, elle redonne un anneau qui
se disperse, pic 0,027 m décroissant et divergence franche divisée par 1,9. Confusion assumée : à
`Hs` constant, allonger les périodes baisse aussi la cambrure — la résolution seule n'est pas
isolée, et aucun seuil de mailles par longueur d'onde n'est reçu.

**Blocage mesuré, pas supposé.** À `dx` 0,25 m dans 8 × 6 m, la bande à la fois résolue et contenue
tient dans λ ∈ [2,1 ; 7,3] m : **moins de deux longueurs d'onde en travers de la boîte**. Une mer
étalée *et* résolue n'entre pas dans ce banc CPU. Le critère 3 d'ADR-175 §4 — « une onde traverse
une mer étalée et s'y déforme », jugé par l'utilisateur — ne peut donc pas être franchi ici : il
demande le domaine de production GPU. C'est l'apport principal de la session pour l'ordre des lots.

**Partiel et suite.** La seconde moitié du « reçu si » de la porte B est tenue ; la première ne
l'est pas et ne peut pas l'être sur ce banc. Restent : pas GPU résident (§4.2), surface publiée,
scène de mer étalée, revue utilisateur (§4.3). A286 (W au-dessus du plan moyen), A274 et A281
inchangées. Absorption oblique, aux coins et mer large bande à la frontière restent non reçues :
l'équivalence 3D↔2D ne les reçoit pas. Aucun retrait d'ambition, aucun arbitrage nouveau.

**Rituel.** Maillons **0** : un critère « reçu si » de la porte B a avancé, avec consommateur et
preuve. Quatrième session consécutive sur la porte B, autorisée par A211 puisqu'un critère avance
à chaque fois ; ADR-174 D6 la désigne comme porte en cours. L84 reçoit une **récurrence datée** —
un livrable visuel doit publier sa résolution comme un banc, et l'aperçu l'imprime maintenant ;
pas de leçon nouvelle forcée, pas d'angle mort nouveau. File active entière relue, deux lignes
remplacées ; feuille de route et index actualisés. I-02/I-04/I-12/I-14/I-17 inchangés.
Navigation et plafonds vérifiés à 0. Copie unique, jeton libre.

## S299 — 2026-09-19 — porte B : le premier étage de δ passe sur la carte

**Entrée.** « Continue », après S298 close et jeton libre. Copie unique, master propre, plan
committé avant tout code. Agent Claude Opus 5, application desktop. Carte constatée avant de
planifier : NVIDIA GeForce RTX 5070 Laptop GPU, backend Dx12 — la machine de référence d'ADR-174 D1.

**Capacité reçue.** Le domaine 3D de δ vit sur la carte. L'opérateur de pression et son
préconditionneur y sont **assemblés** à partir de la seule géométrie, pas lus du cœur : en 3D le
cœur n'exporte rien, `apply_mobile3` est sans matrice, et la production porte donc la règle de
`mobile_row`. La projection y tourne à **travail borné** — `5 + 5·cycles + 2` dispatchs, fonction
des seuls cycles demandés, jamais de la donnée. **Ce qui devient possible** : un pas de production
dont le coût se connaît avant de le lancer, et qu'une référence CPU peut juger.
**Consommateurs** : les quatre bancs de ce lot, puis le pas complet d'ADR-175 D1.
**Preuve** : [DELTA3D-GPU-S299](../../docs/validation/DELTA3D-GPU-S299.md).

**Mesures.** Opérateur contre le cœur, trois géométries : surface plate **identique au bit**
(1287/1287), ondulée 9,554·10⁻⁸, au ras d'un centre 9,300·10⁻⁸ — sous l'ulp `f32` relatif, et
seulement sur les mailles à fantôme. Problème assemblé : rhs et prec **au bit** sur surface
plate, 7,748·10⁻⁸ et 4,470·10⁻⁸ au pire. Projection jugée **par le cœur** avec son propre
opérateur, 7 680 mailles, départ froid : 9,67·10⁻³ à 4 cycles, 8,94·10⁻⁴ à 32, 2,72·10⁻⁷ à 128 ;
la carte annonce les mêmes à quatre à six chiffres. Coût sur la machine de référence :
32³ en 0,314 ms à 32 cycles, 64×64×32 en 0,816 ms médiane, 0,849 ms au maximum.
Suite complète du cœur : **539 réussis**, 18 ignorés, aucun échec.

**Ce que ça dit du 32³.** ADR-144 le désignait comme le point dur du passage à la 3D, « qui y
arrivera d'emblée ». Il tient en un tiers de milliseconde. L'estimation d'ADR-175 §1 — 30 à 120 ms
de CPU par pas pour un 64×64×32 sur le chemin d'ADR-173 — se compare à 0,8 ms de carte pour la
projection seule. L'ordre de grandeur que la décision visait est atteint sur ce poste.

**Erreur commise et corrigée dans la session.** Le banc de coût publiait un « résidu relatif » de
10⁵ : le numérateur portait `‖b − A·x‖` avec un `b` assemblé sur la carte, le dénominateur la
divergence d'entrée. Deux grandeurs justes, calculées à deux endroits, dont le rapport ne veut
rien dire. `‖b‖` est désormais réduite sur la carte, ce qui ajoute deux dispatchs : la borne
passe de `3+5c+2` à `5+5c+2`, et la ligne correspondante du commit P5 est périmée sur ce point.
Les résidus de la réception P5 étaient déjà normalisés par le `‖b‖` du cœur : ils n'ont pas bougé.

**Partiel et suite.** Seule la projection est sur la carte. Restent le second membre **couplé**
(les fantômes de fond de S297 ne sont pas portés), l'advection, les bandes, la surface publiée
(D7), les diagnostics D3 — dérive de masse, âge publié, `PressureItersCut` au-dessus de 10⁻⁵ —
puis la scène de mer étalée et la revue. Aucun coût de porte C n'est reçu : ce banc ne couvre pas
le pas entier. A98 entière : ces chiffres valent pour cette carte et ce pilote. A270 due sur ce
banc — l'état d'alimentation n'a pas été relevé. Aucun retrait d'ambition, aucun arbitrage nouveau.

**Rituel.** Maillons **0** : un critère « reçu si » de la porte B avance — la production que le
critère 3 exige a son premier étage, reçu contre la référence — avec consommateur et preuve.
Cinquième session consécutive sur la porte B, autorisée par A211 puisqu'un critère avance à
chaque fois ; ADR-174 D6 la désigne comme porte en cours. L156 reçoit une **récurrence datée**
sur la variante « dénominateur venu d'ailleurs » ; pas de leçon nouvelle forcée, pas d'angle mort
nouveau. File active et feuille de route remplacées ; la section J2 dépassait son plafond, son
histoire est rendue aux preuves. Index complété. I-04/I-05/I-06/I-13/I-15/I-17 inchangés.
Navigation et plafonds vérifiés à 0. Copie unique, jeton libre.

*Tenue du plan* : le commit P1 porte sa case encore `[>]` — la bascule a été faite au début de P2
au lieu de la dernière action avant le commit. Sans conséquence ici, le commit P1 ne contenant que
le plan ; signalé pour que l'historique ne soit pas lu comme un `[x]` manquant.
