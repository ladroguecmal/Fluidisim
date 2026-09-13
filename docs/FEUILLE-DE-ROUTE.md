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

*État au 2026-09-13 (S204)* : B construit (JONSWAP cuit, ADR-100/101) ; W construit — événement,
journal rejouable, impact radial, sources de pression et sillages, composition B+W, préparation
et restauration du service. Images **hors ligne** seulement : B (S201), impact W dans son emprise
(S203, ADR-126). Profil ADR-125 acquis, seuil 2 % acquis. **Aucun hôte interactif.**
*S205* : la composition B+W admet toute mer — le budget de pente ne borne plus que les
perturbations (ADR-128) ; impact rendu sur la mer S201.
*S206* : coût d'image mesuré sur la scène J1 — **incompatible sur CPU** avec 2 ms à toute
densité qui montre l'impact ; chemin d'image de W par table de Bessel décidé (ADR-129) ;
`paquets_W_max` retiré du profil (I-16).
*S207* : **rendu J1 sur GPU par un hôte séparé**, arbitrage de l'utilisateur (ADR-130) ;
`water-core` reste sans dépendance et publie ce que le GPU consomme.
*S208* : **table de Bessel construite** (`RadialTable`, ADR-129) — W à ~0,95 ms par image sur
36 160 sommets, image à un niveau près du chemin direct, pas de réception λ/16 ; pile de l'hôte
recommandée : wgpu 30.0.1 + winit 0.30.13 ([HOTE-GPU-S208](validation/HOTE-GPU-S208.md)).

*Bloquants nommés* :

*S209* : [lot de résolution prêt](validation/PREPARATION-HOTE-S209.md), sans accès au registre ;
accord index/métadonnées attendu, puis accord sur les sources exactes. Aucun hôte construit.

*S210* : accord de résolution reçu, `viewer/Cargo.lock` construit ;
[254 archives inventoriées](validation/DEPENDANCES-HOTE-S210.md), 49 174 790 octets portables.
Accord des sources et choix cache/vendoring attendus ; cible vide, aucun hôte construit.

*S211* : sources autorisées récupérées, **hôte B+impact construit et exercé sur DX12** ;
voir [réception GPU](validation/HOTE-GPU-S211.md). Erreur max0,077657 mm ; passe eau960×540
médiane0,048576 ms. **A250 close pour le chemin GPU**. Sillage absent et cadence complète
non mesurée : J1 reste partiel. Suite : intégrer le sillage issu du cœur, puis recevoir la scène.

*S212* : **sillage du cœur intégré à l'hôte**, exact (0,089 mm). **L'implémentation S212** —
préparation modale à chaque image, somme par sommet, aucune autre technique de §J1-bis —
**dépasse le budget** : CPU 10,9 ms, GPU eau 4,10 ms à 960×540 pour 4 096 nœuds, un sillage, sur
la machine locale ([HOTE-GPU-S212](validation/HOTE-GPU-S212.md)). Recette honnête ~16 s, couture
12,7 mm à 39 s (A251) ; composition mixte impact+sillage non exercée.
*Cadrage corrigé en S213 par l'utilisateur ([ADR-131](adr/ADR-131-un-depassement-qualifie-une-implementation.md)) :
S212 écrivait « refusé en coût » et bornait la suite à deux leviers avant arbitrage — c'était
l'implémentation qui dépassait, pas le sillage ni l'objectif.*
*S213* : cadrage et espace d'optimisation ci-dessous (ADR-131). **Levier temporel construit**
dans le cœur ([TEMPS-SILLAGE-S213](validation/TEMPS-SILLAGE-S213.md)) : CPU sillage 1,26 ms pendant
le forçage et 0,36 ms après (préparation 7,70 / 13,36), hôte 1,7 ms, exact à 6e-8 ; GPU inchangé.
Coordonnée de l'espace, pas verdict. Suite : composition impact+sillage, puis A251.
*S221* : [coupure spectrale](validation/COUPURE-SPECTRALE-S221.md), ADR-137. A259 levée à 4096
feuilles, intacte à 1024 : aucune enveloppe de modules ne voit la localisation d'un paquet (A261).
**La part dynamique d'A255 est résorbée en précision à l'instant, pas en coût** (≈26 s CPU un fil par
instant). Ce qui bloquera la scène J1 à plusieurs sources est la **part somme d'A254**. Les
sources de pression forment déjà un seul champ préparé, dont la borne locale est conjointe.
*S223* : [inégalité de position relative](validation/COURONNE-IMPACT-S223.md),
[ADR-138](adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md). **A262 traitée** :
le budget tient compte de la distance entre champs d'impact — gain 1,76 à 2,57 selon la séparation,
**exactement 1,0000** à séparation nulle, pour **13,3 µs**. Trois impacts frais à 50 m passaient de
142 % de π/7 — refusés — à 60 %, admis. **Le budget de pente cesse d'être le goulot de J1.**
**Suite S224 : revenir à la file** — cadence complète de l'hôte, puis **V-noyau**.

*S222* : [scène à plusieurs sillages](validation/SOMME-SILLAGES-S222.md). **La part somme d'A254
change de côté.** Côté sillages elle est **absorbée** — trois sources dans un même journal coûtent
1,44 à 1,67 fois une seule, contre un facteur 3 chez les impacts — et ce qui y reste est spatial
(A261 : pessimisme 1,64 → **2,35** quand les sources s'éloignent, alors que le maximum réel ne bouge
pas). La borne locale partitionnée le rend en entier (**1,005–1,010** du maximum, gain jusqu'à 2,32)
mais **à 25 s**, et il n'y a pas de raccourci local : au-delà d'un mètre de demi-côté elle vaut
l'enveloppe globale, et un appel coûte 640–700 µs quelle que soit la taille. **Aucune migration
d'admission** — le prix est une loi d'échelle (L302, L303). **Côté impacts elle est littérale** :
un impact neuf vaut 47,4 % de π/7 et deux éclaboussures simultanées saturent, quelle que soit la
borne de pression. **Suite S223 J1/W : A262**, la somme spatiale sur les impacts — chercher une
**inégalité**, pas une table.
*S220* : [borne locale d'ordre deux](validation/ORDRE-DEUX-S220.md), ADR-136. Hessienne
signée, jamais pire qu'ADR-135 ; partition à 32767 évaluations à 1,006–1,012 × le maximum,
mieux que l'ordre un à 65535 en moins de temps. Le pessimisme n'est plus l'obstacle sur ces
fixtures : **le coût l'est** (≈30 s CPU un fil par instant, aucune technique de J1-bis). Plancher
= réserve numérique (A258) ; grosses mailles plafonnées par les modes non résolus (A260).
**Suite S221 J1/W : A260 puis A258** ; A254, migration d'admission et loi GPU restent ouvertes.
*S219* : [partition adaptative construite](validation/PARTITION-S219.md), pool et
plafond d'évaluations, couverture conservée. Gain1,48–1,52 à65535 évaluations mais
35–36s ; à8191, bornes identiques au plafond global (A259). **Suite S220 J1/W : borne
locale avec Hessienne signée et reste supérieur**, phases quantifiées couvertes,
gain/coût via S219. A255 partielle, A258 ouverte ; aucune admission migrée. A254 et GPU
restent ouverts ; δ/V conservent les jalons obligatoires.
*S218* : [borne locale construite](validation/BORNE-LOCALE-S218.md), ADR-135.
Reste spatial et réserve numérique publiés, aucune admission migrée ; certification
f32 ouverte. La partition uniforme resserre de facteur1,026–1,224 dans les cas
recevables mais coûte27,5–28,2s sur la base. **Suite J1/W S219 : partition adaptative,
pool fourni par l'appelant, plafond de travail et couverture conservée** ; gain/coût
à recevoir. A255 partielle ; somme A254, mutualisation et loi GPU restent ouvertes.
*S217* : [part dynamique d'A255 instruite](validation/DECOHERENCE-SILLAGE-S217.md).
La similitude tient à vitesse et durée réduites constantes ; une courbe unique en âge
depuis extinction est réfutée (38,8 % et 22,5 % d'écart après raffinement). A255 reste
ouverte, bibliothèque inchangée. Prochain lot W : borne locale dépendant du champ avec
reste spatial démontré, puis mesurer resserrement/coût avant mutualisation. La loi GPU
reste à construire ; aucun gain d'image déduit de cette étude.

*S216* : **part statique d'A255 traitée sans mesure**
([ENVELOPPE-SILLAGE-S216](validation/ENVELOPPE-SILLAGE-S216.md),
[ADR-134](adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md)) : occupation de π/7
**41,6 % → 36,3 %** sur la scène J1. Le résidu de décohérence reste, et il n'est pas d'emprise.
*S215* : **A254 traitée pour moitié** ([BUDGET-PENTE-S215](validation/BUDGET-PENTE-S215.md),
[ADR-133](adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md)). Le majorant de pente d'un
impact suit la dispersion ; occupation 84 % → 42 %, le refus à deux sources est levé, trois impacts
et un sillage passent à 51 %. Ouvre **A255** : le sillage domine désormais le budget.
*S214* : **les deux travaux de validité sont faits**
([COMPOSITION-J1-S214](validation/COMPOSITION-J1-S214.md)). La composition du cœur est exacte au
bit une fois le point d'évaluation commun rétabli (**A253**) ; le cœur ne compose toutefois que sur
l'**intersection** des domaines (4 477 sondes sur 6 988), donc le chemin mixte n'est pas le chemin
de rendu. Domaine d'image du sillage calculé, annoncé, publié avec sa fixture
([ADR-132](adr/ADR-132-domaine-d-image-d-un-sillage.md)) : **A251 traitée**. Et une ligne neuve,
plus lourde : **A254** — le budget de pente est une somme sur les sources, 84 % de π/7 pour deux,
marge 0,0712. Elle passe avant toute scène à plusieurs sources.

- ~~**A245**~~ — **levé en S205** (ADR-128) : la mer S201 (Hs 1,5 m) se compose, impact compris,
  zéro refus ; [COMPOSITION-MER-S205](validation/COMPOSITION-MER-S205.md) ;
- **hôte interactif** — **tranché S207 par l'utilisateur : GPU, hôte séparé** (ADR-130) ; **construit B+impact en S211**, dépendances autorisées ; reste le sillage et la réception complète ;
- **A247** — **mesuré en S206** ([COUT-IMAGE-S206](validation/COUT-IMAGE-S206.md)) : à la
  densité qui montre l'impact, l'image coûte 280 ms sur un fil et 36 ms sur seize. W n'est plus
  le goulot avec la table de Bessel précalculée (ADR-129, facteur 100) ; **B évalué par sommet sur
  CPU l'est** (1,2–1,3 µs). **S211 : retiré du chemin d'image CPU**, passe GPU B+impact mesurée ;
  A247 reste partielle tant que le coût complet et le sillage ne sont pas reçus ;
  **S212 : le sillage rouvre le coût** — dans l'implémentation S212, préparation par image (CPU ∝
  nœuds × tronçons) et somme par sommet (GPU ∝ sommets × nœuds) dépassent 5× et 2× le budget ;
  espace d'optimisation ci-dessous (ADR-131) ;
- ~~**A251**~~ *(S212)* — **traitée en S214** par [ADR-132](adr/ADR-132-domaine-d-image-d-un-sillage.md) :
  `rayon = 2π·angular/(3·cutoff)`, `durée = 4π/√(g·cutoff/radial)`, annoncées et publiées avec la
  fixture ; 89,36 m et 18,53 s contre 102,22 m et 40 s déclarés. Une durée honnête porte le critère
  qui l'a calibrée ;
- **A254** *(S214, sévérité 1)* — **traitée pour moitié en S215** par
  [ADR-133](adr/ADR-133-le-majorant-de-pente-suit-la-dispersion.md). Le pessimisme était de la
  **dispersion** et venait presque entièrement de l'impact ; son majorant suit désormais le temps
  (`slope_max_at`), et l'occupation tombe de **84 % à 42 %** — deux impacts et un sillage, qui
  refusaient (`SlopeEnvelope`), passent. **S222 : la part somme est mesurée et elle change de
  côté** — absorbée chez les sillages (1,44 à 1,67 pour trois sources partageant un journal),
  **littérale chez les impacts**, où `slope_floor` somme sans conscience de la distance. Ce qui
  reste ouvert d'A254 est donc **A262** — **traitée en S223** par
  [ADR-138](adr/ADR-138-le-budget-de-pente-tient-compte-de-la-position-relative.md), qui rend le
  budget conscient de la position relative. **A254 est close** : ses deux termes ont désormais leur
  limite connue, l'impact par une inégalité à 13 µs, la pression par une loi d'échelle à 25 s
  (A261) ;
- **A255** *(S215, sévérité 2)* — **part statique traitée en S216** par
  [ADR-134](adr/ADR-134-l-enveloppe-de-pente-tient-compte-des-directions.md) : l'enveloppe sommait
  scalairement des contributions vectorielles, et une inégalité en `O(N)` en retire 20 %, sans
  table ni garde. **Reste sa part dynamique** — la décohérence, 1,10 à 4 s et **3,98 à 39 s**,
  liée au temps depuis l'extinction et **non** à l'emprise dans le montage S216 (vérifié : seize fois l'aire,
  maximum identique à six décimales). **Avant toute scène à plusieurs sillages, donc avant la
  mutualisation ci-dessous** ; **S217** : courbe à un seul âge réfutée après contrôle de vitesse/durée ; suite de construction : borne locale avec reste spatial démontré ;

#### J1-bis — Espace d'optimisation du rendu et travaux nécessaires (ADR-131, S213)

Un dépassement mesuré qualifie l'implémentation, pas la fonctionnalité. **2 ms (ADR-125) est un
objectif éprouvé sur la combinaison** des techniques ci-dessous, sur des scènes représentatives,
sans présumer qu'elle réussira ou échouera. Aucune demande de réduction d'ambition ne se fonde sur
l'échec d'optimisations prises isolément. La liste est ouverte.

| technique | état au 2026-09-13 | publie avec elle |
|---|---|---|
| phases repliées de B au GPU | **présente** (S211) | écart GPU/cœur |
| table de Bessel d'un impact (ADR-129) | **présente** (S208, S211) | écart au champ direct, pas λ/16 |
| **temps** — sillage : tronçons achevés repliés, modes préconstruits | **présente** (S213) : 1,26 ms forçage / 0,36 ms après à 4 096 nœuds, un fil ; 6e-8 du chemin préparé | écart au chemin préparé, pic aux bornes (2,12 ms), retour arrière (3,10 ms) |
| **espace** — grille locale et transformée | absente, nommée S212 | quadrature d'image, coutures et période |
| **LOD spatial** — densité, emprise selon distance et écran | absente | Nyquist par distance (L283), coutures entre niveaux |
| **LOD spectral** — nœuds par source selon distance et visibilité | absente | écart à la recette pleine, durée et rayon honnêtes (ADR-107, **ADR-132** : les deux lois se recalculent depuis la recette réduite) |
| **LOD temporel** — cadence de mise à jour selon distance, vitesse, régime | absente | erreur de phase, I-09 |
| **visibilité** — frustum, occlusion, hors écran | absente | exactitude au retour dans le champ |
| **mutualisation** — nœuds partagés par sources de même recette, passe/grille communes B/W | absente ; **plus conditionnée par le budget de pente** (S222 : trois sillages et huit impacts passent), désormais par le coût de passe seul | superposition dans son domaine (ADR-123), et part du budget de pente consommée |

**Chaque mesure de coût publie** techniques présentes, techniques absentes et domaine de validité
(scène, recette, sources, formats, instants, machine, grandeur mesurée) — ADR-131 D3.

**Travaux nécessaires de J1, indépendants du coût** (ADR-131 D6) — accélérer ne les remplace pas.
*Faits en S214* : ~~A251~~ (ADR-132) et ~~composition impact + sillage par le cœur~~ — exacte au
bit, budget conjoint exercé, et **A254** ouverte par cette mesure. *Fait en S215* : ~~la moitié impact d'A254~~ (ADR-133), qui ouvre **A255**. *Fait en S216* : ~~la
part statique d'A255~~ (ADR-134). *Restent* : **la part dynamique d'A255** (la décohérence du
sillage — elle conditionne la mutualisation) ; la part somme d'A254 ;
~~cadence complète mesurée~~ — **faite en S225** ([CADENCE-HOTE-S225](validation/CADENCE-HOTE-S225.md)) : 198 Hz, exclusions chiffrées à 2 %, et la pose de mesure héritée de S201 requalifiée en pire cas (GPU d'eau 2,85 ms en médiane de balayage contre 4,16 fixe, soit 1,42 × le budget au lieu de 2,08) ; interaction manuelle, angles rasants et poses de caméra ; allocations de
la pile graphique (I-06) ; seconde cible (B7).

*Bancs qui tranchent à ce jalon* : **B1** (nombre de composantes et coût de B, dès qu'un LOD
existe dans l'hôte) ; **B2** partiel (représentation de W, dès que le coût B+W par image est
mesuré sur la scène) ; **B7** partiel (budget sur la machine locale, pas encore la cible).
*Cas* : C02, C07, C18, branche W de C19.

### J2 — Domaines volumiques bornés, comme cas de construction du δ général

*Livre* : un ou plusieurs domaines δ **pris dans le système** — interfaces `Volume`/`Caps`
(ADR-007), ordonnanceur et dégradation (ADR-012, I-05), création et destruction gratuites
(I-12), éponge vers B+W (ADR-005) — sur des cas bornés : cavité et gerbe d'impact, proche-coque.
Un domaine borné est une **étape** du δ général, jamais un produit à part (ADR-127 D3).

*État au 2026-09-13* : candidat MAC x-z en bibliothèque (S199), sans allocation dans le pas et
à refus atomiques (S200), coût du pas mesuré (S202). Non admissible B3. **Sur le chemin** : A244 /
S200-1 (précision f64, respect temporel I-05), S199-2 (flux des faces coupées), surface libre
mobile (couvercle imposé aujourd'hui), passage à la 3D, couplage de l'écart à B+W. Part d'un
impact que W ne porte pas nommée en S203 (énergie hors ondes, cavité, gerbe).

*Bancs* : **B3** (famille de δ) quand un candidat atteint ses critères ou qu'un second existe ;
**B4** (régime perturbatif, volets restants) sur les cas livrés ; **B5** (blocs épars) quand
plusieurs blocs existent ; **B10** (cavité d'entrée) avec le premier domaine d'impact.
*Cas* : C01, C03, C04, C05, C06, C08, C09, C20, C22, C23.

### V-noyau — ouvert au plus tard avec J2

*Livre* : graphe de contenants et d'arêtes (fuites, vannes, débordements), arithmétique entière,
pas serveur à basse fréquence, état répliqué et restauré (ADR-010, ADR-022, I-03, I-10).

*État au 2026-09-13* : **ouverte en S224** — un module, `hydro_network`, reçu par **C12**
(vidange en 727,4 s contre 728 s analytiques, 0,0824 %) ;
[NOYAU-V-S224](validation/NOYAU-V-S224.md). Nœuds en millilitres entiers, orifice et déversoir, pas
de 100 ms, report de reste, normalisation par arrondi cumulatif, refus atomiques, déterminisme
vérifié. *S226* : **direction de `g_eff` construite** — I-07 cesse d'être violé ; **A266** ouverte. *Restent* : `liquid_id` (A17),
`sky_exposure` et `absorb_rate`, vannes et pompes, **réseau fermé sous pression** (reporté en v2 par
l'ADR), et l'**état répliqué et restauré** d'ADR-022 §5.1, sans lequel la branche V de C19 et C21
restent hors d'atteinte. **A264** : le plancher de vidange croît avec la surface du contenant.

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

## 4. Arbitrages explicites ouverts

| arbitrage | pourquoi il est explicite | qui tranche |
|---|---|---|
| ~~Chemin de rendu et hôte de J1~~ — **tranché S207 : (A) GPU, hôte séparé** ([ADR-130](adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md)) *(fusionnait « hôte interactif » et A247, S206)* | Mesuré : B sur CPU coûte 42 ms par image à la densité qui montre l'impact (2 px), 36 ms sur 16 fils ; W est ramené à 27 µs par impact (ADR-129). Options : **(A)** hôte séparé qui évalue B et les tables W sur **GPU** — prévu par ADR-003 et I-08 (« seules des phases repliées passent au GPU ») et par `gpu_sim_ms = 2,5` d'ADR-012 ; `water-core` reste sans dépendance, l'hôte en a (téléchargement) ; **(B)** CPU seul sans dépendance — exige B vectorisé (non mesuré), un groupe de fils persistant, et que les 2 ms se comptent en **temps mur sur tous les cœurs**, ce qu'ADR-125 ne dit pas ; **(C)** changer le profil ADR-125 (fréquence ou temps eau) ; **(D)** grille à 8 px — **perd les anneaux**, donc retire l'impact visible | **l'utilisateur** — a retenu (A) |
| ~~Dépendances de l'hôte GPU~~ **autorisées S210/S211**, sources récupérées, cache local/verrou versionné *(état initial S207/S208)* | ADR-130 : aucune bibliothèque téléchargée sans autorisation nommée. S208 recommande wgpu 30.0.1, winit 0.30.13, pollster 1.0.1 ; demande en deux temps — résolution de l'arbre (index), puis sources ; vendoring ou non | **l'utilisateur** — accords reçus |
| **Budget GPU de l'eau** *(S207)* | ADR-125 ne dit pas où l'eau s'évalue ; ADR-012 déclarait `gpu_sim_ms = 2,5` ; aucune valeur inventée | **S211 : passe GPU locale mesurée**, budget complet encore à recevoir ; arbitrage si incompatible. **S212 : l'implémentation S212 du sillage dépasse** (CPU 10,9 ms, GPU 4,10 ms, un sillage, sans LOD, visibilité ni mutualisation). **S213, ADR-131** : pas un arbitrage ; 2 ms s'éprouve sur la combinaison de J1-bis ; aucune demande de réduction fondée sur l'échec d'optimisations isolées. *S212 écrivait « il revient à l'utilisateur si les leviers mesurés ne tiennent pas 2 ms » : cadrage retiré.* |
| ~~A247 — coût d'un impact visible~~ | **mesuré S206** ; part technique tranchée par ADR-129, part d'arbitrage fusionnée ci-dessus | — |
| ~~A245 — mer composable~~ | **tranché S205, ADR-128** : B hors du budget de refus, bits publiés inchangés | — |

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
