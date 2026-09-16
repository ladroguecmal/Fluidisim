# Feuille de route du système d'eau

> **Ambition finale complète, construction progressive par versions de plus en plus capables.**
> — formulation de référence de l'utilisateur, 2026-09-13, [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md).

**Ce document est le seul qui porte la trajectoire.** REPRISE, l'index et la file active y
renvoient ; ils ne la recopient pas (L137). Il se met à jour au rituel de fin quand une session
change l'état d'un jalon. Chaque état est **daté** (A185). La décision vit dans ADR-127 ; ce
document en tient l'application.

## 1. Ce qui est visé, et ne se négocie pas en cours de route

B, W, **δ général** (solveur volumétrique à surface libre, domaines multiples, interactions
volumiques générales, frontières mobiles, régime substitutif), **V** (réseau hydraulique,
inondations complexes), leur articulation, et la **grande échelle**. Les rôles restent ceux
d'[ADR-001](adr/ADR-001-decomposition-en-couches.md) §2 : δ n'a jamais d'autorité gameplay
(I-04), les conséquences de jeu d'une inondation passent par V, déterministe et autoritaire.

Réduire ce périmètre n'appartient à aucune session : il faut une décision de l'utilisateur qui
nomme ce qui est retiré (ADR-127 §6). Un budget, un ordre ou une priorité ne retirent rien.

## 2. Les jalons

L'ordre suit les **dépendances**, pas une préférence. Une flèche dit « a besoin de ».

```
J1  B/W visible et interactif
 │
 ├──> J2  domaines δ bornés (cas du δ général)  ──> J3  phénomènes, interactions, frontières mobiles ──┐
 │                                                                                                    │
 └──> V-noyau (ouvert au plus tard avec J2) ──────────────────────────────> J4  inondations complexes, V↔δ
                                                                                                      │
                                                                                          J5  ambition complète, grande échelle
```

**V n'attend pas J3.** Son noyau ne dépend ni de δ ni des frontières mobiles (ADR-054 §1, C12
est un cas V seul). Ce que J4 attend de J3 est l'**articulation** — V expose une surface,
déclenche un domaine δ local, C21 compare la masse avec et sans δ.

### J1 — Version visible et interactive avec B et W

*Livre* : une scène représentative — mer de référence, impacts, sillages — **parcourue en temps
réel**, composée B+W sans refus, coût mesuré face au profil 60 images/s / eau 2 ms.

*État relu S227, 2026-09-13* : **hôte GPU B + impact + sillage présent** (S211–S213),
mer JONSWAP, composition par le cœur reçue S214. Budget de pente resserré S215/S216/S223 ;
**A254/A262 closes**, l'admission ne bloque plus par principe une scène à plusieurs sources.
Le domaine honnête du sillage est publié (ADR-132) ; l'hôte avertit lorsqu'il en sort.

*S234, 2026-09-14* : **la passe d'eau de la scène J1 passe sous 2 ms** — grille locale du sillage
à pas borné (3 mm) et reconstruction Hermite bicubique : **0,426 ms** cuisson comprise à 960×540
(témoin S233 4,24), 0,43/0,46 ms en fenêtre fixe/balayée, cadence 384/398 Hz ; recette 128×256 à
1,51 ms. Mesures sur secteur (A270). Voir [LOD-SILLAGE-S234](validation/LOD-SILLAGE-S234.md).

*S235, 2026-09-14* : **scène multi-sources dans l'hôte** — trois sillages d'un journal commun et
huit impacts nés toutes les 4 s — vérifiée contre le cœur (≤0,368 mm) ; passe GPU ≤0,47 ms, et
visibilité par emprise de la grille : hors champ 0,06 ms GPU, **retour dans le champ identique au
bit**. Voir [SCENE-MULTI-S235](validation/SCENE-MULTI-S235.md).

*S236, 2026-09-15* : **le cœur compose et admet la scène représentative** — mode union de la
requête mixte (ADR-142 : chaque perturbation contribue là où son emprise couvre le point), plancher
de pente certifié par séparation avec pression locale ADR-137 sur les cellules critiques. S235
admise aux **161 instants** (0 refus contre 49), requête à < 1e-9 m de la somme de référence de
l'image. Voir [ADMISSION-UNION-S236](validation/ADMISSION-UNION-S236.md).

**J1 reste partiel.** (1) **CPU** : l'implémentation mono-fil prépare le sillage en 3,1 ms pendant le
forçage de trois sillages ; somme CPU+GPU 4,5 ms alors, 2,1–2,6 ms hors forçage (ADR-125 ne fixe pas
la répartition). (2) **Requête** : ≈0,2 ms par point (pression 4 096 modes sur CPU) et plancher
jusqu'à 12 ms quand la pression locale intervient — hors du chemin d'image, borne de ce qu'un
consommateur gameplay peut demander. Restent l'interaction représentative et la seconde cible ;
angles rasants mesurés S247, allocations reçues S240 et filtrage B/sillage reçu S249 ci-dessous.

A261 reste vraie hors des cellules critiques ; A258 porte désormais sur une borne employée à
l'admission ; A263 suit la constante Bessel. Les déclencheurs de reprise vivent uniquement dans la
file active. L'ordre de livraison n'interdit pas de construire les briques indépendantes de J2 ou V
pendant que J1-bis progresse.

#### J1-bis — Espace d'optimisation du rendu et travaux nécessaires (ADR-131, S213)

Un dépassement mesuré qualifie l'implémentation, pas la fonctionnalité. **2 ms (ADR-125) est un
objectif éprouvé sur la combinaison** des techniques ci-dessous, sur des scènes représentatives,
sans présumer qu'elle réussira ou échouera. Aucune demande de réduction d'ambition ne se fonde sur
l'échec d'optimisations prises isolément. La liste est ouverte.

| technique | état (mises à jour datées) | publie avec elle |
|---|---|---|
| phases repliées de B au GPU | **présente** (S211) | écart GPU/cœur |
| table de Bessel d'un impact (ADR-129) | **présente** (S208, S211) | écart au champ direct, pas λ/16 |
| **temps** — sillage : tronçons achevés repliés, modes préconstruits | **présente** (S213) : 1,26 ms forçage / 0,36 ms après à 4 096 nœuds, un fil ; 6e-8 du chemin préparé | écart au chemin préparé, pic aux bornes (2,12 ms), retour arrière (3,10 ms) |
| **espace** — grille locale et transformée | **grille locale présente** (S234, évaluation directe des modes aux nœuds) ; **transformée absente** | quadrature d'image, coutures et période |
| **LOD spatial** — densité, emprise selon distance et écran | **présent pour le sillage** (S234) : densité d'évaluation selon sa borne bicubique, indépendante du maillage ; 0,426 ms contre 4,24. **LOD du maillage absent** (≤35 % mesurés, rien en vue haute : B dicte la densité) | borne et erreur aux intérieurs, coutures entre mailles (≤6 µm), rapport au témoin ; coupure des amplitudes selon Nyquist ajoutée S249, ligne distincte ci-dessous |
| **LOD spectral** — nœuds par source selon distance et visibilité | absente | écart à la recette pleine, durée et rayon honnêtes (ADR-107, **ADR-132** : les deux lois se recalculent depuis la recette réduite) |
| **Filtrage spectral de l'image** — amplitudes selon le pas projeté | **B et sillage présents S249** (ADR-148), huit bandes du même champ, aucune décimation de quadrature. Impacts absents. GPU 1,22–1,33 ms en régime, contre 0,46–0,49 ; premier passage à 2,26 ms | erreur numérique ≤0,340 mm, écart volontaire au champ complet séparé ; rayon/durée ADR-132 inchangés ; normales sans dérivée du filtre, perception non reçue ; [preuve](validation/COUPURE-S249.md) |
| **LOD temporel** — cadence de mise à jour selon distance, vitesse, régime | absente | erreur de phase, I-09 |
| **visibilité** — frustum, occlusion, hors écran | **présente pour sillage et impacts** (S235) : emprise de la grille sur l'eau, marge par arête ; hors champ 0,448 → 0,062 ms GPU, 1,59 → 0,67 ms CPU ; +0,05 ms CPU dans le champ. Occlusion et composantes de B absentes | **retour au bit** vérifié (31 images, fin de tronçon incluse) |
| **mutualisation** — nœuds partagés par sources de même recette, passe/grille communes B/W | **présente pour les sillages d'un même journal** (cœur, S222 ; hôte S235 : trois sillages, 4 096 modes, +0,01–0,03 ms GPU) ; table de Bessel partagée par les impacts de même entrée (S235) ; passe commune B/W absente | superposition dans son domaine (ADR-123) ; budget de pente : **admis sur l'union sous plancher certifié** (S236, ADR-142) — refusé par majorants en S235 |
| **parallélisme CPU / LOD temporel de la préparation** | **S243 : le parallélisme déterministe existe** (ADR-146) — `parallel_fill_f32`, écriture disjointe, garantie inconditionnelle ; **×2,63 à huit fils** sur `ModalPressure::sample`, 3,27 → 1,24 ms, empreintes identiques à 1/2/4/8/16 fils. **Le chemin d'image reste à un fil** : créer les fils par appel coûte ≈ 67 µs et alloue (A278). LOD temporel toujours absent | écart au chemin séquentiel **au bit**, vérifié ; pour le chemin d'image, un vivier persistant — donc `unsafe` dans l'hôte, une décision (A278). Le coût de δ (A276) se mesure sur bancs et n'attend pas |

**Chaque mesure de coût publie** techniques présentes, techniques absentes et domaine de validité
(scène, recette, sources, formats, instants, machine, grandeur mesurée) — ADR-131 D3.

**Travaux nécessaires de J1, indépendants du coût** (ADR-131 D6) : A251 et composition faites
S214, A254 close S223, cadence complète faite S225. **Allocations de la pile graphique faites S240**
(ADR-145) : boucle d'image à **zéro allocation** pour le code du projet, pile verrouillée à **133
allocations et 18 509 octets par image, constantes** ; [preuve](validation/ALLOCATIONS-HOTE-S240.md).
**Angles rasants soutenus faits S247** : le coût tient à toutes les poses mesurées — GPU eau
identique à la quatrième décimale —, mais l'échantillonnage du champ lointain non : même part sous
Nyquist, **pire écart 8,243 m contre 2,589** ; [mesure](validation/RASANT-S247.md). **Mis en
images S248** : la dégradation n'est pas répartie mais concentrée dans une **bande étroite à
l'horizon**, et sur toute l'eau visible le pire écart monte à **1 228,8 m** à la pose rasante ;
[images](validation/IMAGES-S248.md). Restent l'interaction manuelle représentative et la seconde
cible (B7).
Composition de la scène représentative par le cœur : faite S236 (mode union, ADR-142). Le choix du
mode par un hôte autoritaire reste à trancher avec lui (A271).

*Bancs qui tranchent à ce jalon* : **B1** (nombre de composantes et coût de B, dès qu'un LOD
existe dans l'hôte) ; **B2** partiel (représentation de W, dès que le coût B+W par image est
mesuré sur la scène) ; **B7** partiel (budget sur la machine locale, pas encore la cible).
*Cas* : C02, C07, C18, branche W de C19.

### J2 — Domaines volumiques bornés, comme cas de construction du δ général

*Livre* : un ou plusieurs domaines δ **pris dans le système** — interfaces `Volume`/`Caps`
(ADR-007), ordonnanceur et dégradation (ADR-012, I-05), création et destruction gratuites
(I-12), éponge vers B+W (ADR-005) — sur des cas bornés : cavité et gerbe d'impact, proche-coque.
Un domaine borné est une **étape** du δ général, jamais un produit à part (ADR-127 D3).

*État au 2026-09-15* : candidat MAC x-z en bibliothèque (S199), sans allocation dans le pas et
à refus atomiques (S200), coût du pas mesuré (S202). **S230 : arrêt coopératif atomique et
temps restant explicite**, reprise reçue ; [budget δ](validation/BUDGET-DELTA-S230.md).
I-05 complet non reçu : retards réels observés, admission/marges encore absentes.
Non admissible B3. **Sur le chemin** : A244 /
S200-1 (anciennes API temps f32, respect temporel I-05). **Pression f32 reçue S231** sur les
domaines éprouvés, résidu réel contrôlé, stockage réduit sans gain de vitesse reçu ;
[preuve](validation/PRESSION-F32-S231.md). **S232 : débit ouvert reçu** sur trois fonds lisses,
ordres1,947/1,957/1,966 ; triangles fluides perdus corrigés. L'ancien≈0,90 mesurait une somme
sans ouvertures ; [preuve et limites](validation/FLUX-COUPES-S232.md).
**S233 : hauteur évolutive linéarisée**, consommée par pression/flux du pas suivant ; durée
entière et compensation f32, onde stationnaire reçue, abandon atomique hauteur comprise.
[Preuve](validation/SURFACE-LINEARISEE-S233.md), ADR-141.
**S237 (2026-09-15) : surface géométriquement mobile** — fonction hauteur, Dirichlet par fluide
fantôme (opérateur symétrique au bit), mailles qui entrent et sortent du fluide, advection
quadratique, pas atomique. Reçue contre l'**onde stationnaire d'amplitude finie** du véhicule HOS
d'ordre 3 : à 10 cm (`ka = 0,16`), profil **0,23 %** et harmonique `2k` à **0,43 %** à 128 colonnes,
décroissants en raffinant ; le mode linéaire n'en produit que 10⁻⁵.
[Preuve](validation/SURFACE-MOBILE-S237.md). **S238 : la pression f32 s'arrête à sa précision
représentable** (ADR-143) — erreur inverse sous la borne d'arrondi de la ligne ou état revenu au bit, pas
reçu seulement à la divergence de S199 ; 5 cm **reçu à 128 colonnes** (0,25 % / 0,71 %), vitesse à
5,5·10⁻⁸ de la solution f64 ; [preuve](validation/PRESSION-PLANCHER-S238.md).
**S239 : la tolérance physique de S199 devient une condition d'acceptation** (ADR-144), sur les lignes
**franches** — sans fantôme de surface. Tenue jusqu'à 8 192 mailles pour **une itération de plus**
(348 contre 347, divergence 1,02·10⁻⁵ → 5,84·10⁻⁶) ; **refusée explicitement à 32 768 mailles**, où le
plancher d'ADR-143 arrête le solveur à 1,34·10⁻⁵ et le pas est déclaré dégradé. Réception S237/S238
conservée pour +0,9 % de coût médian ; [preuve](validation/TOLERANCE-PRESSION-S239.md). Restent :
surface non graphe (déferlement), mouillage du fond, bords ouverts, plancher des lignes à fantôme
(A274), grandes tailles en f32 (**A275 fermée S245** : la multigrille en repli fait passer 32 768 mailles de refusé à reçu, ADR-147), cavité et couplage de
l'écart à B+W. **S241 : l'ordre des obstacles du passage à la 3D est renversé** — à 2 048 mailles
δ seul coûte 5,5125 ms par image contre les 2 ms qu'ADR-125 donne à toute l'eau, et ≈ 296 ms par
image à 8 192 mailles ; le **coût** passe donc devant la précision (A276 avant A275), et aucune
technique de coût n'a encore été tentée sur δ (ADR-131).
[Confrontation](COMPARABLES-EXTERNES.md) §3. **S244 : la carte du coût est faite** — écritures
disjointes 67 à 73 % du pas, réductions 12-13 %, itérations doublant par raffinement ; **286,2 ms
par pas à 32 768 mailles**. Le parallélisme est fermé pour cette boucle (125 µs par fil contre
21,7 de pass) : **la multigrille est le seul levier dont le gain croît avec la taille** ;
[carte](validation/COUT-DELTA-S244.md). **S245 : elle est construite** — elle ne gagne pas de
vitesse, un cycle coûtant cinq produits fins par itération, mais elle **ferme A275** en précision et
est branchée en repli ; [mesure](validation/MULTIGRILLE-S245.md). Le coût reste entier. **S246 : la
prolongation bilinéaire n'apporte rien et est annulée ; l'amortissement du lisseur était faux** — `2/3`
donné comme dérivé est l'optimum à une dimension, `4/5` est celui à deux — et le repli passe de 1 006
à 828 ms ; [mesure](validation/PROLONGATION-S246.md). Le plafond du taux reste ouvert (A281). Part d'un impact que W ne porte pas nommée en S203 (énergie hors ondes, cavité, gerbe).

*Bancs* : **B3** (famille de δ) quand un candidat atteint ses critères ou qu'un second existe ;
**B4** (régime perturbatif, volets restants) sur les cas livrés ; **B5** (blocs épars) quand
plusieurs blocs existent ; **B10** (cavité d'entrée) avec le premier domaine d'impact.
*Cas* : C01, C03, C04, C05, C06, C08, C09, C20, C22, C23.

### V-noyau — ouvert au plus tard avec J2

*Livre* : graphe de contenants et d'arêtes (fuites, vannes, débordements), arithmétique entière,
pas serveur à basse fréquence, état répliqué et restauré (ADR-010, ADR-022, I-03, I-10).

*État au 2026-09-14* : **ouverte en S224** — un module, `hydro_network`, reçu par **C12**
(vidange en 727,4 s contre 728 s analytiques, 0,0824 %) ;
[NOYAU-V-S224](validation/NOYAU-V-S224.md). Nœuds en millilitres entiers, orifice et déversoir, pas
de 100 ms, report de reste, normalisation par arrondi cumulatif, refus atomiques, répétabilité
locale vérifiée (seconde cible non reçue). *S228* : **géométrie orientée consommée dans le pas**,
prisme, cale et forme non convexe reçus ; **A266 corrigée**, tables historiques limitées à +Z.
Arrivées collectives bornées (S227), gravité dirigée (S226). Voir
[VOLUME-ORIENTE-S228](validation/VOLUME-ORIENTE-S228.md) pour précision, refus et coût complet.
*S229* : **capture/restauration du noyau reçue**, écarts aux valeurs d’auteur et restes conservés,
base géométrique identifiée, continuation C19-V locale identique ;
[RESTAURATION-V-S229](validation/RESTAURATION-V-S229.md).
*Restent* : `liquid_id` (A17),
`sky_exposure` et `absorb_rate`, vannes et pompes, **réseau fermé sous pression** (reporté en v2 par
l'ADR), et l’**intégration réseau/stockage** du codec, l’assemblage B/W/V de C19
et le couplage V↔δ de C21. **A264** : plancher des anciennes tables ; **A269** : précision des
géométries et tailles supplémentaires. Cuisson réelle et budget encore à recevoir selon l'usage.

*Cas* : C12 ; branche V de C19.

### J3 — Phénomènes étendus, interactions entre domaines, frontières mobiles

*Livre* : parois et corps mobiles, flottaison, plusieurs domaines et leurs raccords, régime
substitutif et sa restauration depuis graine (ADR-001 §3.3, ADR-013, I-17), référentiel accéléré,
aération et bulles, écume, vue sous-marine.

*État au 2026-09-13* : véhicules d'essai Saint-Venant 1D à paroi mobile et corps flottant simple
(S21–S58), qui ne sont pas le système ; rien dans le candidat δ.

*Bancs* : **B6** (flottabilité), **B8** (seuils d'activation et de prédiction), **B9** (écume),
**B11** (rendu sous-marin), B4 forces et perception. *Cas* : C10, C11, C13, C14, C16, C23.

### J4 — V et inondations complexes, articulées avec la représentation volumétrique

*Livre* : compartiments en réseau, brèches, poches d'air (ADR-015), inondation visible — V expose
sa surface et déclenche δ local — avec une comptabilité de masse qui reste celle de V.

*Dépend de* : V-noyau et J3. *Cas* : C17, C21, C19 complet.

### J5 — Ambitions initiales complètes

*Livre* : grande échelle — référentiels multiples, coordonnées lointaines, bathymétrie et
hauts-fonds, conformité multiplateforme (A98), matériel cible (B7 complet), glace (C15) — puis
approfondissement.

## 3. Comment un banc entre dans la trajectoire

Un banc s'exécute **quand les composants construits permettent de trancher une décision
concrète**, et il la nomme avant de mesurer. On ne ferme pas tous les bancs avant une version
utilisable ; une version ne revendique aucune réception qu'un banc n'a pas rendue (ADR-127 D6).
Les protocoles restent ceux de [PLAN-BENCHMARK](validation/PLAN-BENCHMARK.md).

**Une mesure de coût**, à tout jalon, publie les techniques présentes, les techniques absentes et
son domaine de validité ; son verdict porte sur l'implémentation mesurée, jamais sur une
fonctionnalité ; le budget s'éprouve sur la combinaison des techniques (ADR-131).

## 4. Arbitrages explicites

**Aucun accord de dépendances du lot S210/S211 n'est encore attendu.** GPU séparé choisi en
S207 (ADR-130), sources autorisées et récupérées S211. Une dépendance nouvelle se traite selon
son autorisation propre. Le budget eau 2 ms est acquis : on ne le redemande pas comme « budget
GPU » ; une incompatibilité établie selon ADR-131 ferait l'objet d'un arbitrage explicite.
Les faits d'intégration ou le matériel externe inconnus restent à constater.

## 5. Historique de la trajectoire

ADR-053 (S70) : construire, en commençant par W. ADR-054 (S71) : ordre des lots W sans faux
préalable. ADR-124 (S201) : image, puis budget, puis effets bornés — **lu à tort comme une
réduction**, corrigé par ADR-127 (S204). ADR-125 (S202) : profil 60 images/s, eau 2 ms.
ADR-126 (S203) : emprise d'un impact visible. ADR-128 (S205) : le budget de pente borne les
perturbations, pas la mer — A245 levé. ADR-129 (S206) : chemin d'image de W par table de
Bessel ; coût d'image J1 incompatible sur CPU, arbitrage de rendu posé. ADR-130 (S207) : rendu J1
sur GPU par un hôte séparé, choix de l'utilisateur. ADR-131 (S213) : un dépassement qualifie une
implémentation ; espace d'optimisation nommé ; 2 ms éprouvé sur la combinaison — clarification de
l'utilisateur. *Les verdicts « incompatible » de S206 à S212 se lisent désormais comme portant sur
les implémentations mesurées à leur date.*
