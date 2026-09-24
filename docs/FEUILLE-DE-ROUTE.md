# Feuille de route du système d'eau

> **Ambition finale complète, construction progressive par versions de plus en plus capables.**
> — formulation de référence de l'utilisateur, 2026-09-13, [ADR-127](adr/ADR-127-ambition-complete-construction-progressive.md).

**Ce document est le seul qui porte la trajectoire.** REPRISE, l'index et la file active y
renvoient ; ils ne la recopient pas (L137). Il se met à jour au rituel de fin quand une session
change l'état d'un jalon. Chaque état est **daté** (A185). La décision vit dans ADR-127 ; ce
document en tient l'application. La [liste du projet fini](LISTE-PROJET-FINI.md) énumère ce que
l'ambition complète contient, point par point, et coche ce qui est validé ; elle se remplit à la
demande de l'utilisateur.

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
réel**, composée B+W sans refus, coût mesuré face au profil (ADR-125, [ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D3).

*État au 2026-09-19* — **partiel**. Reçu : hôte GPU séparé B + impacts + sillages (S211–S213,
ADR-130) ; scène multi-sources composée et admise par le cœur (S214, S236, ADR-142) ; passe d'eau
réduite par grille locale du sillage (S234), visibilité à retour au bit (S235), filtrage spectral
de B et du sillage (S249, ADR-148) et cuisson optimisée (S267) : **GPU eau médian 1,74 ms** à
1280×720 ; boucle d'image sans allocation pour notre code (S240, ADR-145). La mer a été jugée par
l'utilisateur de R1 à R7 (S254–S266), **R7 accepté** : queue spectrale (ADR-155), mer multimodale
(ADR-156), vagues pointues et rugosité de Cox–Munk (ADR-157, 158), vent de scène (ADR-160),
reflets filtrés (ADR-161, 163). Preuves : [COUPURE-S249](validation/COUPURE-S249.md),
[CUISSON-SILLAGE-S267](validation/CUISSON-SILLAGE-S267.md), [REVUE-VISUELLE](validation/REVUE-VISUELLE.md).

*Manque* : le **CPU** — préparation du sillage 4,1 ms médian sur un fil, contre ≤ 2 ms (ADR-174
D3), levier A278 ; l'**interaction représentative** — aucun objet pilotable, objet de la
scène-témoin de la porte D ; les pointes au premier passage (A265) ; l'échantillonnage du lointain
aux angles rasants (S247, S248) ; la seconde cible (A98). Requête de jeu : ≈ 0,2 ms par point,
plancher jusqu'à 12 ms (A261) ; CWM cohérente avec l'image à 0,30 mm (ADR-159).

*Bancs* : B1, B2 partiel, B7 sur la machine de référence. *Cas* : C02, C07, C18, branche W de C19.
Composition de la scène par le cœur faite (S236) ; le choix du mode par un hôte autoritaire reste
à trancher avec lui (A271).

### J1-bis — Espace d'optimisation du rendu (ADR-131, S213)

Un dépassement qualifie l'implémentation, pas la fonctionnalité ; le budget s'éprouve sur la
**combinaison** des techniques, sur des scènes représentatives ; la liste est ouverte. Chaque
mesure publie techniques présentes, absentes et domaine de validité (ADR-131 D3).

| technique | état au 2026-09-19 | publie avec elle |
|---|---|---|
| phases repliées de B au GPU | présente (S211) | écart GPU/cœur |
| table de Bessel d'un impact (ADR-129) | présente (S208, S211) | écart au champ direct |
| **temps** — tronçons repliés, modes préconstruits | présente (S213) | écart au chemin préparé, pics aux bornes |
| **espace** — grille locale, transformée | grille présente (S234) ; transformée absente | quadrature, coutures, période |
| **LOD spatial** | sillage présent (S234) ; maillage absent | erreur aux intérieurs, coutures |
| **filtrage spectral de l'image** | B et sillage (S249, ADR-148) ; impacts absents. **S306 : la coupure est réglable** (`--coupure=<f>`), `f` = 1 étant ADR-148 au bit | erreur ≤ 0,340 mm, écart volontaire séparé ; **et depuis S306 l'énergie haute fréquence de l'image** ([STRIES-S306](validation/STRIES-S306.md)) |
| **LOD spectral** par source | absent | écart à la recette pleine ; durée et rayon (ADR-132) |
| **LOD temporel** | absent | erreur de phase, I-09 |
| **visibilité** | sillage et impacts (S235) ; occlusion absente | retour au bit |
| **mutualisation** | sillages d'un journal, table de Bessel partagée (S222, S235) ; passe B/W commune absente | superposition (ADR-123) |
| **parallélisme CPU** | construit (S243, ADR-146), hors chemin d'image (A278) | écart au bit |

**L'optique du rendu, S304 à S308 — lot clos**
([ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md) D2,
[confrontation](registres/TROIS-SYSTEMES-S308.md)). Asymétries de la surface, `Sk` 0,003 → 0,066
pour +0,25 % ([ADR-176](adr/ADR-176-asymetries-de-la-surface-rendue.md)) ; queue spectrale = 80–85 %
de l'énergie haute fréquence de l'image, `--coupure` réglable ([S306](validation/STRIES-S306.md)) ;
couleur du corps d'eau **9 fois trop verte**, désormais dérivée de ses sources
([ADR-177](adr/ADR-177-couleur-du-corps-d-eau-derivee-de-ses-sources.md),
[S307](validation/RENDU-ECART-S307.md)) ; trois revues envoyées options acceptées **éteintes**,
remède `--meilleur` (**L349**).

**S308 : la photographie de R14 devient une cible chiffrée** (`outils/cible_image.py`), le ciel
est calé sur elle (`--ciel-mesure`), la courbe de tonalité exposée (`--tonalite`), le miroitement
échelonnable (`--miroitement`). Deux hypothèses **réfutées** — la coupure spectrale
*ajoute* des pixels clairs, éteindre le miroitement n'en retire aucun. Puis
`outils/courbe_tonalite.py`, 6 300 réglages : **les huit meilleurs donnent le même contraste
local, 0,311–0,318 pour 0,455 mesurés**. Une courbe est point à point, le contraste local est
spatial (**L351**) : ce qui manque est **dans la mer**. Écume, diffusion aux crêtes, ECKV et
structure fine restent dus, **avec leur cible chiffrée**.

### J2 — Domaines volumiques bornés, comme cas de construction du δ général

*Livre* : un ou plusieurs domaines δ **pris dans le système** — interfaces `Volume`/`Caps`
(ADR-007), ordonnanceur et dégradation (ADR-012, I-05), création et destruction gratuites (I-12),
éponge vers B+W (ADR-005) — sur des cas bornés : cavité et gerbe d'impact, proche-coque. Un
domaine borné est une **étape** du δ général (ADR-127 D3).

*État au 2026-09-21* — **reçu en 2D ; la 3D est la porte B** ([ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)). **Référence CPU 3D reçue** : HOS à 0,148 % / 0,178 %, couplage B/W ([S297](validation/DELTA3D-COUPLEE-S297.md)) ; cas limites S269–S274 à 1,19·10⁻⁷ m ([S298](validation/DELTA3D-FOND-REEL-S298.md)).

La mer étalée **ne se juge pas sur un banc CPU** (S298 §5) : elle demande la production GPU.
**S301 l'a complète**, chaque étage reçu contre le cœur ; elle suit la référence à 2·10⁻⁵ m jusqu'à
l'**horizon de la référence** (1,1–1,3 s, A297), 0,84 ms à 64 cycles
([S299](validation/DELTA3D-GPU-S299.md),
[S300](validation/DELTA3D-FOND-GPU-S300.md), [S301](validation/DELTA3D-PAS-GPU-S301.md)).
**S302 : la scène tourne** — 30 × 28 m à 25 cm, rendu en direct à 197 Hz
([S302](validation/SCENE-DELTA3D-S302.md)). **R11 reçu** : raccord invisible — mais S310 montre
que cette invisibilité n'était **pas** de la conservation.
Tranche MAC x-z (2D), reçue : surface mobile couplée contre HOS à 0,162 % / 0,34 %, faces coupées,
multigrille, frontières (ADR-143 à 169) — [S253](validation/SURFACE-COUPLEE-S253.md),
[S274](validation/HOULE-USAGE-S274.md).

**S310 à S316, le lot 2 d'ADR-178 D7** (ADR [179](adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md)
à [183](adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md)) : bilan de masse exact, A302 chiffrée
([S310](validation/BILAN-MASSE-S310.md)) ; sortie lue sur une ligne de contrôle intérieure
([S311](validation/SORTIE-DELTA-S311.md)) ; premier transfert ([S312](validation/TRANSFERT-DELTA-W-S312.md)) ;
**ordre A**, loi du résidu et T2 tenue sur 10 s ([S313](validation/PLANCHER-BILAN-S313.md)) ; **ordre B**,
`WaveTrain` orienté ([S314](validation/TRANSFERT-ORIENTE-S314.md)) ; oblique sans composante transverse
([S315](validation/ORACLE-ET-OBLIQUE-S315.md)) ; **ordre C**, six propriétés attribuées — primitive exacte à
10⁻⁴, au raccord un degré de phase et 2 à 4 % de spectre, le reste à δ sauf une part non linéaire que
W ne porte pas ([S316](validation/ORDRE-C-S316.md)) ; **ordre D**, un receveur local, attente
exactement nulle ([S317](validation/RESTITUTION-S317.md)). **Ordre E bloqué** : sous une vraie mer, δ
croît jusqu'à trois fois la houle (A289, [S319](validation/MER-S319.md)), indépendamment du pas de
temps (S322).

*Manque* : cuve sur la production (§4.1) ; mouillure (A297) ; I-05 (A244) ; cavité dans δ (B10 reçu sur le banc 2D d'APIC, [S320](validation/B10-APIC-S320.md)) ; phase
δ/B (A289) ; A274 ; A286 ; **compteur carte, énergie, quantité de mouvement, sens W → δ** (A302) ;
**résidu biaisé en cas ouvert** (A305).

*Bancs* : **B3**, **B4**, **B5**, **B10**. *Cas* : C01, C03 à C06, C08, C09, C20, C22, C23.

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

*État au 2026-09-24* : parois et corps mobiles dans la référence 3D de δ (S330–S332), corps rigide du
jeu (S331) ; **porte D reçue** sur la référence CPU (S338, [preuve](validation/PORTE-D-S333.md) §9). **L'ordonnanceur est ouvert** (porte A) : `scheduler.rs` décide quels domaines
vivent et avec quel budget (S278, ADR-170), branché sur la bande δ (S279, ADR-171) ; coût estimé
par la médiane des pas payés et oublié selon le temps (S280, S282, S286) ; rétrécissement manuel
puis préparé (S283–S285), fidélité temporelle non reçue (A290). Cadences lentes mesurées, non
activées (S286). Preuves : [ORDONNANCEUR-S279](validation/ORDONNANCEUR-S279.md),
[ATTRIBUTION-RETRECISSEMENT-S285](validation/ATTRIBUTION-RETRECISSEMENT-S285.md).

*Manque* : plusieurs candidats réels, domaine qui se déplace et se redimensionne, dégradation
automatique (ADR-012 §4), régime substitutif et son critère `0,35·Hs_local` jamais calibré —
sur les domaines 3D de la porte B ; seuils sans banc B8 ; W derrière la requête du corps, la coque
dans la production de δ.

*Bancs* : **B6** (flottabilité), **B8** (seuils d'activation), **B9** (écume), **B11** (rendu
sous-marin), B4 forces et perception. *Cas* : C10, C11, C13, C14, C16, C23.

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

## 3 bis. Portes de version — un système à la fois, et où poser la v1

*Écrit en S281, 2026-09-19, à la demande de l'utilisateur : « création / test / validation d'un
système puis d'un autre, déclaration de la v1 ».*

Cette section **ne double pas §2** : elle le regroupe. Les jalons disent les dépendances ; les
portes disent dans quel ordre on amène un système jusqu'à sa réception, et laquelle vaut version.
Si les deux divergent un jour, **§2 fait foi** et cette section est fausse (L137).

**Une porte se franchit quand sa colonne « reçu si » est vraie, pas quand le code existe.** Un
système qu'on n'a pas éprouvé n'est pas construit : il est écrit.

**Porte en cours, S294 (2026-09-19) : B**, avec **D en parallèle** par la scène-témoin de la v1.
**S338 (2026-09-24) : D reçue** sur la référence CPU ; la v1 demande encore A, B et C (ADR-174 D4).
**S340 (2026-09-24) : B reçue.** **S348 : C reçue sur le banc** — 30 Hz validé à l'œil (R17). **Porte en cours : A**,
la dernière de la v1 : déplacement, redimensionnement, rang 1.
Dépendances qui fondent cet ordre (ADR-127 §6, L343) : la porte C se reçoit sur la scène de B ;
les critères restants de A — plusieurs candidats, domaine qui se déplace et se redimensionne —
portent sur les domaines 3D que B définit ([ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
D6) ; D ne dépend que de B+W. Ordre accepté par l'utilisateur ([ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D6).

| porte | ce qu'on crée | ce qui l'éprouve | reçu si |
|---|---|---|---|
| **A — ce qui décide** *(ouverte ; **S344** : deux domaines δ 3D se disputent un budget, premier critère tenu au banc — [preuve](validation/ARBITRAGE-3D-S344.md))* | l'ordonnanceur : quels domaines vivent, où, de quelle forme, et ce qu'on dégrade quand le budget manque | banc **B8** (seuils, inexistant) ; la bande δ de l'afficheur comme premier consommateur | plusieurs candidats réels se disputent un budget ; un domaine **se déplace et se redimensionne** au lieu d'être seulement allumé ou éteint ; la dégradation d'ADR-012 §4 rang 1 existe, donc la famine a une issue |
| **B — δ sur les deux dimensions horizontales** *(**reçue S340** — référence S297–S298, production S299–S305 et S340, scène S302 et S339, verdict R16 ; le coût à la porte C)* | le solveur volumique qui n'est plus une tranche : domaine 3D, mer étalée, interaction avec des vagues réelles ; représentation et critères posés par [ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D5 et §4 | banc **B3** (technique δ), cas **C10**, **C11** ; revue visuelle de l'utilisateur | une onde traverse une mer **étalée** et s'y déforme, jugée convaincante par l'utilisateur ; les réceptions 2D (S253, S268–S274) tiennent encore à trois dimensions. **S305** : la production suit la référence à 3·10⁻⁷ m pour 3 mm exigés sur les cas de cuve, phase décroissante ([preuve](validation/CUVE-GPU-S305.md)) — le cas 3 ; **S340** : les cas 1 et 2 aussi, sous 10⁻⁴ m ([§9](validation/CUVE-GPU-S305.md)) ; **R16** : une onde née d'un point traverse la mer de R14 et s'y déforme, jugée convaincante ([S339](validation/SCENE-DELTA3D-S302.md) §8) — **reçue** |
| **C — δ sous budget** *(**reçue S348 sur le banc** — 30 Hz, un pas pour deux images : 1,92 ms au 99ᵉ centile par image ; fond factorisé et compact, au bit ; [preuve](validation/COUT-DELTA3D-S341.md) §6–11)* | cadence découplée de l'image (I-05), δ sur GPU — **décidé S294** : pas résident à travail borné ([ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)) —, multigrille hors repli | **B7** sur la machine de référence ([ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D1), cartes de coût selon ADR-131 | le pas tient **δ ≤ 2 ms GPU** (ADR-174 D3) sur la scène de la porte B, au 99ᵉ centile, **techniques présentes et absentes publiées** |
| **D — solides et flottabilité** *(**reçue S338** sur la référence CPU : partie numérique S333–S337, verdict R15 ; [preuve](validation/PORTE-D-S333.md) §9)* | corps flottants pris dans B+W+δ, forces rendues au jeu | **B6** (flottabilité), **B4** (forces et perception), cas **C10**, **C11**, **C23** *(C13 et C14, écrits ici en S281, sont les bulles et l'écume de J3 : corrigé S338)* | un bateau flotte et perturbe l'eau qui le porte, sans autorité de δ sur le gameplay (I-04) |
| **E — V articulé avec δ** | inondations : V expose sa surface, déclenche un δ local, la masse reste celle de V | cas **C17**, **C21**, **C19** complet | la comptabilité de masse est identique **avec et sans** δ (C21) |
| **F — grande échelle** | référentiels multiples, bathymétrie, hauts-fonds, conformité multiplateforme | **B7** complet, **A98** | une scène lointaine et une scène proche coexistent sans rupture ni perte de précision |

**Réorganisation du 2026-09-20 (S308), décision de l'utilisateur**
([ADR-178](adr/ADR-178-strategie-en-trois-systemes-physiques.md)). Les travaux se regroupent
désormais en **trois systèmes** — A haute mer superficielle (B + W), B volumique 3D (δ), C
transition et couplage —, validés indépendamment puis assemblés puis optimisés. **Les portes
ci-dessus restent vraies** ; ce sont leurs priorités relatives qui changent, et l'ordre des lots
est celui de [TROIS-SYSTEMES-S308](registres/TROIS-SYSTEMES-S308.md) §5 : compteurs de
conservation, retour δ → W, faces coupées 3D, corps rigides *(ces deux-là franchissent la porte
D, donc la v1)*, seconde représentation de surface libre, requête de jeu à travers δ, puis
adaptation et budget. **A est déclaré suffisant pour servir B et C** : aucun lot de
perfectionnement visuel ne s'ouvre avant que la physique le demande. Le profil d'ADR-174 D3 reste
**mesuré et publié**, sans être opposable pendant la construction (ADR-178 D4). **Le 2026-09-21**
([ADR-184](adr/ADR-184-seconde-representation-en-parallele.md)), l'utilisateur fait avancer le lot 5
**en parallèle** du lot 2, par sessions alternées ; la v1 reste la porte D. **Le 2026-09-22**
([ADR-188](adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md)), le lot 3 prend la place du lot 2, bloqué
par A289, dans cette alternance. **S324** : le fond coupé entre dans la référence 3D — identique au bit
à la 2D sans `y`, ordre 1,956 sur une bosse ; ses petites cellules, qui faisaient ramper le solveur (A315),
sont préconditionnées en **S326** : 425 itérations à 128 au lieu de 16 029 ([preuve](validation/FACES-COUPEES-3D-S324.md) §6) ;
**S328** : le **mode mobile** porte la découpe, au bit de la 2D sans `y`, ordre 1,954 sur la bosse (§7) ;
**S329** : un **solide quelconque** immergé — Archimède exact au niveau discret, sphère d'ordre 1,966 (§8) ;
**S330** : il **bouge** — masse ajoutée d'une sphère `C_m` = 0,508 (§9) ; **S331** : le **corps rigide du
jeu**, sur B + W, tient C10 ([preuve](validation/CORPS-RIGIDE-S331.md)) — lot 4 ouvert ; **S332** : il pilote
sa coque dans δ, qui perce la surface, et δ ne le pilote jamais — trajectoire identique au bit (§4) ;
**S333** : la coque sur une houle de B, et δ qui porte sa perturbation **relative à l'eau qui la porte** —
anneaux de 9,4 cm, I-04 au bit ([preuve](validation/PORTE-D-S333.md)) ; verdict visuel attendu ; **S334–S335** :
A317 corrigé — le couvercle partiel, actif par défaut, converge (§6–7) ; **S336** : la coque reçoit la masse
ajoutée et l'amortissement que δ lui mesure, et s'arrête ([preuve](validation/RAYONNEMENT-COQUE-S336.md)) ;
**R15** : le bateau qui se pose est juste, la coupure au bord de δ se voyait — **S337** : éponge et fondu (§8) ;
**S338** : *« Plus de coupure »* — **porte D reçue** sur la référence CPU (§9) ; **S340** : **porte B reçue** —
R16, et la production sur les trois cas de cuve ([§9](validation/CUVE-GPU-S305.md)) ; **S341–S348** : **porte C
reçue sur le banc** — fond factorisé, dix champs par face, cadence de 30 Hz en deux parts ([§11](validation/COUT-DELTA3D-S341.md)).
**Le 2026-09-23** ([ADR-189](adr/ADR-189-la-v1-d-abord.md)), l'utilisateur demande la v1 d'abord : lots 3
et 4 jusqu'à la porte D, l'alternance avec le lot 5 suspendue jusque-là. **S318** : comparaison
chiffrée des trois représentations ([S318](validation/COMPARAISON-LOT5-S318.md)) ; **APIC retenue**
par l'utilisateur ([ADR-186](adr/ADR-186-apic-seconde-representation.md)) ; **S320** : B10 sur APIC,
une cavité se pince au même instant à toute échelle ([S320](validation/B10-APIC-S320.md)) — mais ce
temps ne converge pas encore à trois mailles, et la bulle sans pression emballe le calcul fin (S326,
§5 bis) ; **S323** :
une région passe des particules aux colonnes et retour à masse exacte, la surface sautant de la
différence des biais de reconstruction (A314, [§10](validation/B10-APIC-S320.md)).

### La v1 — tranchée par l'utilisateur le 2026-09-19 (ADR-174 D4)

**Une v1 après la porte D**, c'est-à-dire : mer crédible parcourue en temps réel (J1, tenu),
perturbations locales **décidées par le système** et non câblées (porte A), δ qui tient sur une
vraie mer (porte B) et dans le budget (porte C), objets qui flottent et remuent l'eau (porte D).

C'est ce qu'il faut pour qu'un jeu s'en serve : quelqu'un navigue, l'eau réagit, et rien ne saccade.

**Ce que cette v1 ne contiendrait pas** : les inondations complexes et V articulé (porte E), la
grande échelle (porte F), les phénomènes secondaires — écume, spray, bulles, glace — et le
multijoueur au-delà de ce qui est déjà déterministe. **Ce sont des reports, pas des retraits** :
l'ambition complète reste celle d'ADR-127 §1, et aucune session ne la réduit.

**Décision de l'utilisateur, 2026-09-19** ([ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md)
D4) : la v1 est la porte D franchie. Proposée en S281, elle n'était pas une décision avant cette
date. Profil de travail de la porte C : [ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D3 —
δ ≤ 2 ms GPU sur la machine de référence.

### Ce que l'état réel dit de la distance

[La liste du projet fini](LISTE-PROJET-FINI.md) compte **3 points validés sur 120**, 55 partiels,
62 absents — **recalculé point par point en S309**, 4.8 (S316), 4.12 (S320), 6.2 (S333) et 6.4 (S338) passés à partiel ;
le décompte est vérifié par l'outil depuis S321. Ce
chiffre ne mesure pas l'avancement : beaucoup de partiels portent l'essentiel de leur difficulté.
Il mesure autre chose, qu'il vaut mieux regarder en face : **presque rien n'est allé jusqu'à la
réception**, et entre S276 et S308 le dépôt a écrit un solveur 3D, l'a porté sur GPU et l'a rendu
en direct **sans amener un seul point jusqu'à son périmètre final**. Rangés par système
([TROIS-SYSTEMES-S308](registres/TROIS-SYSTEMES-S308.md) §8) : A 20, B 29, C 7, et **64 hors des
trois** — la stratégie d'ADR-178 couvre 56 points sur 120, le reste venant après par construction.

Les quatre manques qui commandent l'ordre ci-dessus, tous mesurés :

1. δ est une **tranche 2D** (`Domain { nx, nz, dx }`, aucune dimension `y`) sous une houle sans
   étalement — c'est ce que le [verdict R10](validation/REVUE-VISUELLE.md#verdict-r10--reçu-s277-2026-09-18)
   a désigné, et aucune session ne peut le contourner ;
2. son coût vaut **≈ 11 fois** le budget d'ADR-012 §3 ([COUT-DIRECT-S276](validation/COUT-DIRECT-S276.md)) ;
3. l'ordonnanceur décide **qu'un** domaine vit, pas où ni de quelle forme
   ([ORDONNANCEUR-S279](validation/ORDONNANCEUR-S279.md)) ;
4. **V a un noyau reçu et aucune articulation** avec δ.

*Audit S293, 2026-09-19* ([BILAN-GLOBAL-S293](registres/BILAN-GLOBAL-S293.md)) : la porte B n'a pas
commencé parce qu'un déclencheur interne — « A276 avant la 3D » — la place derrière la porte C,
dont la réception est pourtant définie sur la scène de la porte B (L343). Deux préalables à la 3D
sont nommés : l'architecture d'exécution de δ (A295) et la part de δ dans le budget (A296,
décision de l'utilisateur). La porte D n'a ni objet pilotable ni corps rigide. Ordre recommandé
et questions : bilan §5 et §6 ; rien n'est retiré de l'ambition.

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
ADR-174 (S294) : machine de référence, temps de l'eau au service de l'objectif, v1 = porte D
franchie, porte B avant la suite du coût en 2D — arbitrages de l'utilisateur. ADR-175 (S294) : δ
en 3D, production résidente sur GPU à travail borné, référence CPU pour la réception.
