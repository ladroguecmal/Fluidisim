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
exactement nulle ([S317](validation/RESTITUTION-S317.md)). **Ordre E bloqué** : la dérive de S319
retirée à sa source en S369 ([ADR-198](adr/ADR-198-la-voie-d-a289.md)) ; reste A320, une perturbation qui croît
sous houle raide ([S369](validation/MER-S369.md)).

*Manque* : cuve sur la production (§4.1) ; mouillure (A297) ; I-05 (A244) ; cavité dans δ (B10 reçu sur le banc 2D d'APIC, [S320](validation/B10-APIC-S320.md)) ; phase
δ/B (A289, tranchée S369 ; A320) ; A274 ; A286 ; **compteur carte, énergie, quantité de mouvement, sens W → δ** (A302) ;
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
[ATTRIBUTION-RETRECISSEMENT-S285](validation/ATTRIBUTION-RETRECISSEMENT-S285.md). **En 3D** : deux
domaines réels arbitrés (S344) ; un domaine qui se déplace (S349) et se redimensionne (S350), au bit,
son coût proportionnel à sa surface ([preuve](validation/ARBITRAGE-3D-S344.md) §5–6).

*Manque* : dégradation automatique (ADR-012 §4) au-delà du rang 1, reçu au banc en S351 — rangs 2 à 7, régulateur PI
de §5, bande morte de l'échelle, prix visuel jugé —,
régime substitutif et son critère `0,35·Hs_local` jamais calibré ; seuils sans banc B8 ; W derrière
la requête du corps, la coque qui bouge dans la production de δ — le pas linéaire y est depuis S358, solide fixe
([preuve](validation/LINEAIRE-GPU-S358.md), ADR-193).

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
**S340 (2026-09-24) : B reçue.** **S348 : C reçue sur le banc** — 30 Hz validé à l'œil (R17). **S351 : A reçue
sur le banc** — le rang 1 ([preuve](validation/ARBITRAGE-3D-S344.md) §7). **Les quatre portes de la v1 sont reçues**,
chacune sur son banc ou sa référence (§ « La v1 » ci-dessous). **Depuis la v1** (ADR-190, décision de l'utilisateur) :
la [liste du projet fini](LISTE-PROJET-FINI.md) entière ; la session qui suit range ici ses points restants par
dépendance.
Dépendances qui fondent cet ordre (ADR-127 §6, L343) : la porte C se reçoit sur la scène de B ;
les critères restants de A — plusieurs candidats, domaine qui se déplace et se redimensionne —
portent sur les domaines 3D que B définit ([ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
D6) ; D ne dépend que de B+W. Ordre accepté par l'utilisateur ([ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D6).

| porte | ce qu'on crée | ce qui l'éprouve | reçu si |
|---|---|---|---|
| **A — ce qui décide** *(**reçue S351 sur le banc** — **S344** : deux domaines δ 3D se disputent un budget ; **S349–S350** : un domaine se déplace et se redimensionne, au bit ; **S351** : le rang 1, aucune image affamée contre 612, q99 mesuré 4,983 ms pour 5 — [preuve](validation/ARBITRAGE-3D-S344.md) §5–7)* | l'ordonnanceur : quels domaines vivent, où, de quelle forme, et ce qu'on dégrade quand le budget manque | banc **B8** (seuils, inexistant) ; la bande δ de l'afficheur comme premier consommateur | plusieurs candidats réels se disputent un budget ; un domaine **se déplace et se redimensionne** au lieu d'être seulement allumé ou éteint ; la dégradation d'ADR-012 §4 rang 1 existe, donc la famine a une issue |
| **B — δ sur les deux dimensions horizontales** *(**reçue S340** — référence S297–S298, production S299–S305 et S340, scène S302 et S339, verdict R16 ; le coût à la porte C)* | le solveur volumique qui n'est plus une tranche : domaine 3D, mer étalée, interaction avec des vagues réelles ; représentation et critères posés par [ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) D5 et §4 | banc **B3** (technique δ), cas **C10**, **C11** ; revue visuelle de l'utilisateur | une onde traverse une mer **étalée** et s'y déforme, jugée convaincante par l'utilisateur ; les réceptions 2D (S253, S268–S274) tiennent encore à trois dimensions. **S305** : la production suit la référence à 3·10⁻⁷ m pour 3 mm exigés sur les cas de cuve, phase décroissante ([preuve](validation/CUVE-GPU-S305.md)) — le cas 3 ; **S340** : les cas 1 et 2 aussi, sous 10⁻⁴ m ([§9](validation/CUVE-GPU-S305.md)) ; **R16** : une onde née d'un point traverse la mer de R14 et s'y déforme, jugée convaincante ([S339](validation/SCENE-DELTA3D-S302.md) §8) — **reçue** |
| **C — δ sous budget** *(**reçue S348 sur le banc** — 30 Hz, un pas pour deux images : 1,92 ms au 99ᵉ centile par image ; fond factorisé et compact, au bit ; **S353** : interpolé au rendu, 1,49 ms par image en direct ; [preuve](validation/COUT-DELTA3D-S341.md) §6–12)* | cadence découplée de l'image (I-05), δ sur GPU — **décidé S294** : pas résident à travail borné ([ADR-175](adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)) —, multigrille hors repli | **B7** sur la machine de référence ([ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D1), cartes de coût selon ADR-131 | le pas tient **δ ≤ 2 ms GPU** (ADR-174 D3) sur la scène de la porte B, au 99ᵉ centile, **techniques présentes et absentes publiées** |
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
reçue sur le banc** — fond factorisé, dix champs par face, cadence de 30 Hz en deux parts ([§11](validation/COUT-DELTA3D-S341.md)) ;
**S390** : à 30 Hz, la scène explosait en 24 à 40 s quel que soit le solveur de pression (**A321**) ; **S391** : cause —
l'advection explicite centrée —, corrigée par un terme de second ordre ([ADR-209](adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md)) : deux minutes tenues à 30 et 60 Hz.
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
date.

**Atteinte le 2026-09-24 (S351), au sens de D4** : les portes A, B, C et D sont reçues. **Chacune l'est sur son
banc ou sa référence** — D sur la référence CPU (S338), B sur la production (S340), C et A au banc (S348, S351) — et
elles ne sont **pas encore réunies en une scène vivante**. Ce qui sépare ces réceptions de la phrase ci-dessus —
*quelqu'un navigue, l'eau réagit, et rien ne saccade* — est connu et appartient à la liste (ADR-190) : la coque dans
la production de δ (6.4), l'ordonnanceur et le rang 1 dans l'afficheur, l'interpolation du rendu à 30 Hz (8.7), W
derrière la requête du corps (6.1), le prix visuel du rang 1 (4.5, 9.9). Profil de travail de la porte C : [ADR-174](adr/ADR-174-arbitrages-du-2026-09-19.md) D3 —
δ ≤ 2 ms GPU sur la machine de référence.

### Ce que l'état réel dit de la distance

[La liste du projet fini](LISTE-PROJET-FINI.md) compte **10 points validés sur 120**, 69 partiels,
41 absents (S513 ; 5.3 en S489, 6.5 en S493, 6.3 en S497, 6.1 en S502, 6.4 en S509, 9.5 en S510, 6.8 en S512) — **recalculé point par point en S309 puis en S350**, 4.8, 4.12, 6.2, 6.4, 4.13, puis 4.2 et 9.9 (S351), 8.5 (S359), 2.7 (S362), 8.6 (S365), 7.1 (S367), 4.21 (S369), 5.4 (S372), 5.10 (S375), 5.5 (S378), 8.4 (S380), 4.3 (S386), 4.16 (S393), 4.9 (S396), 9.2 (S401), 9.3 (S405), 4.10 (S408) passés à partiel ;
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

## 3 ter. Après la v1 — la liste entière, par fronts

**Depuis S475 ([ADR-218](adr/ADR-218-le-systeme-de-l-eau-complet.md), décision de l'utilisateur) : la liste validée à 100 % est la
condition de fin du système de l'eau. L'ordre est celui du [plan de complétion](registres/PLAN-COMPLETION-S475.md)** — treize
campagnes (K1 à K13), les faits que seul l'utilisateur peut fournir (F1 à F6, répondus en S476 : [ADR-219](adr/ADR-219-reponses-du-2026-10-04.md) — le jeu est
DyingStar) ; ce qui suit garde l'histoire de l'ordre par fronts. **S471–S477** : le banc visuel (ADR-216), le type d'eau (ADR-217 ; K1 en
cours), la liste à 100 % (ADR-218). **S478–S480** : K2 conçue (ADR-220), K2-1 — l'air enfermé dans la référence APIC (A311 : le remède
éprouvé) ; la structure du projet (ADR-221 : la boussole, le [tableau de bord](registres/TABLEAU-DE-BORD.md), le rituel outillé).
**S481–S483** : K2-2, l'air enfermé sur la carte, puis son coût (les poches dans la multigrille ; `--v1` à 1,4 % du témoin ; A311 close) ;
ADR-222, la méthode se révise elle-même — les calculs hors de la session, la référence APIC 3D sur les cœurs, le banc de non-régression.
**S484–S486** : K2-3, la remontée d'une grosse bulle — l'air conservé, cuve entière à 0,90 de Davies et Taylor (A325 : le quart de cuve
n'est pas une symétrie ; A326 : la carte bornée à 8 M particules) ; la première revue de méthode (ADR-223).
**S487–S491** : A326 levée pour les particules ; K2-4, les gouttes (A312 non levée : la couronne suit la vitesse d'éjection) ; **5.3 validée**
(le débordement vers l'extérieur) ; 6.5 avancée (A327 : le mur aligné sur la grille) ; la deuxième revue (ADR-224) ; `eveil.py`.
**S492–S494** : A327 réattribuée (la tolérance de divergence au point mort d'une seiche, ADR-225) ; **6.5 validée** (le décor qui perce
la surface, sur la carte) ; 6.2 avancée (les impacts de W derrière la requête du corps).
**S495–S497** : 6.2 avancée (le sillage pousse les corps : W entier derrière la requête du corps) ; la troisième revue (ADR-226 : localiser
avant de remédier) ; **6.3 validée** (le corps en marche émet son sillage).
**S498–S500** : 6.1 avancée — C11 (les régimes de flottabilité d'ADR-008 §3, le petit objet contraint à la surface), B6 (le modèle
d'erreur du proxy), la poussée au centre de la part immergée (ADR-227 : une couche suffit, 70 points pour un navire de 60 m).
**S501–S503** : la quatrième revue (ADR-228 : un corps d'essai loin de ses limites) ; **6.1 validée** (l'amortissement des six degrés de
liberté, mesuré par δ) ; 6.4 avancée (un solide immergé qui bouge sur la carte).
**S504–S506** : 6.4 avancée — la coque qui perce la surface en mouvement sur la carte (dépôt, transfert de S334), C23 sur le système (la paroi
dans la vitesse gouvernante, ADR-229) ; A328 ouverte (la convergence en `dt` près d'une coque mobile) ; la cinquième revue (ADR-230).
**S507–S510** : A328 réattribuée (l'ordre 1 en temps) ; **6.4 validée** (le recoupage dans la boîte du solide, la carte ne recevant que ce
qui change : 0,82 ms par pas) ; **9.5 validée** (le consommateur des impacts prédits, confirmés ou rejetés).
**S511–S513** : la sixième revue (ADR-231) ; **6.8 validée** (l'impulsion d'entrée dans l'eau, C20) ; 2.6 partielle et 6.2 avancée (le
courant C0 et C2 derrière la requête de l'eau).
**S514–S516** : 6.7 partielle (l'acteur poussé ou emporté, le nageur commandé) ; 5.4 avancée (la vanne selon sa courbe, la pompe sur sa
conduite et son énergie : ne manque que le réseau fermé) ; la septième revue (ADR-232).
**S517–S518** : 4.13 avancée (la coque en marche et sa vague d'étrave sur la carte ; le sillage stable, son angle non mesuré à 2° par trois
instruments) ; A329 ouverte puis levée (le recoupage d'une coque qui bouge à 2,2 ms sur 786 000 mailles, au bit ; le pas de la carte 9,2 ms).
**S519–S521** : C07 en eau profonde passe (W à 0,33 % de la théorie linéaire, 19,98° ; 3.2, 13.2) ; le sillage de la coque dans δ sans
référence éprouvée (A330 ; la carte au-delà de 65 535 groupes) ; la huitième revue (ADR-233).
**S522–S523** : W en profondeur uniforme (2.7 ; le sillage à `Fr_h` = 0,9 à 2,0 % de la théorie) ; C07 peu profond passe à `Fr_h` = 1,43
(44,00° pour 44,46°) ; A331 (le domaine honnête de W non vérifié).
**S524–S527** : A331 levée (le domaine honnête de W lu sur la distance du chemin aux points, la garde de l'hôte) ; la résonance de C07 (W à
0,03 % de la théorie, l'assertion corrigée) ; l'angle à `Fr_h` = 2,14 — **C07 passe entier** ; la neuvième revue (ADR-234).
**S528–S529** : les anneaux d'impact de W en eau peu profonde (3.1 ; 0,44 % d'une propagation FFT exacte) ; A330 localisée (l'amplitude du
sillage de δ stable en maille, l'écart revient au modèle de référence ; le pic d'étrave divergent au coin vif).
**S530–S533** : 5.5 avance deux fois — l'absorption par le sol (Green–Ampt dans V, intégré exactement) et la pluie hors contenant
(rétention, infiltration, ruissellement) ; la dixième revue (ADR-235 : les limites matérielles calculées) ; la carte refuse ses limites
avec un nom.
**S534–S536** : C1, le champ de courant 2D régional (2.6 ; la pente cyclostrophique porte le corps) ; l'assèchement du sol (5.5 :
drainage de Brooks–Corey, évaporation ; le cycle de l'eau du sol à la masse exacte) ; la onzième revue (ADR-236).
**S537–S539** : 9.3, le contact d'un corps quelconque (son enveloppe convexe) ; **C17 passe** — l'inondation limitée par l'air (5.9
partiel) ; la coque retournée et sa poche comprimée, le point de non-retour (7.5 partiel).
**S540–S541** : **C13 passe** (la remontée des petites bulles, Tomiyama) ; la douzième revue (ADR-237 : un nombre recalculé à chaque
changement de paramètre).
**S542–S545** : **C16 passe** (la pesanteur effective inclinée et la rotation dans δ ; 4.17 partiel) ; **C21 passe** (la masse d'un
compartiment avec et sans δ, fixe et accéléré) ; un plan sauté et une limite affirmée sans calcul, relevés.
**S546–S547** : la treizième revue (ADR-238 : le rituel refuse sans plan committé) ; l'évent à débit limité (5.9, `Q_eau ≤ Q_air`).
**S548–S551** : 6.6 partiel puis avancé trois fois — la barge envahie (la flottabilité perdue), la carène libre, l'angle de bande ; la
quatorzième revue (ADR-239 : une formule d'analyse éprouvée avant la mesure).
**S552–S556** : 6.6 avancé trois fois — la gîte par une brèche latérale, l'envahissement progressif par une cloison percée, la poche
porteuse d'un compartiment scellé ; C09 exécuté (la masse tient, l'énergie naturelle manque le critère, A332) ; la quinzième revue (ADR-240 :
un script en fichier, la constante de temps d'un équilibre au plan).
**S557–S560** : 4.18 avance deux fois — l'énergie que le pas linéaire de δ conserve (l'invariant mixte, A332 levée, C09 passé), puis
celle du chemin coupé ; **5.7 ouvert** (ADR-241 : les liquides de V, non miscibles, en couches) — la pression d'un nœud stratifié, le
débit par couches (le manomètre en U, la vidange stratifiée).
**S561–S562** : la seizième revue (ADR-242 : une grandeur intégrale aux poids du schéma ; le rituel refuse un lot dû) ; 5.7 avance —
l'écrémeur, l'instantané de la composition (`WVLQ`).
**S552–S553** : 6.6 — un navire gîte par sa brèche (une citerne latérale sous la pesanteur inclinée) ; l'envahissement progressif par une
cloison percée.

*Écrit en S352, 2026-09-24* ([ADR-190](adr/ADR-190-apres-la-v1-la-liste-entiere.md) D3). Ce que chaque point attend et
débloque est dans [DEPENDANCES-LISTE](registres/DEPENDANCES-LISTE.md), calculé par `outils/dependances_liste.py` et
tenu par `etat_projet.py --check`. Ici, l'ordre. Les jalons du §2 et les portes E et F restent vrais : ils disent les
mêmes dépendances, en plus gros.

**Depuis S355 ([ADR-191](adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md), décision de l'utilisateur), une session de rendu alterne avec une session de physique** ; **depuis S356 ([ADR-192](adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)),
le rendu de l'eau se fait dans Godot 4** — la mer de B rendue dans Godot (S357 ; R19 : *« pas du tout crédible »*) ; **S359** :
la lumière de l'eau portée de l'afficheur et la colonne d'eau, 8.5 partiel (ADR-194) ; **S360** : la surface fine par FFT,
l'écume au déferlement (ADR-195) ; **S361** : les caustiques ; **S363** : le ciel de la photographie, une courbe calée sur elle en option ; **S365** : sous la surface, 8.6
partiel — fenêtre de Snell, milieu ; **S366** : R24 reçu, la lumière de l'eau calée sur Tyler ; **S367–S368** : l'écume, champ
d'ADR-014 et son rendu — suspendue par l'utilisateur sauf références photographiques ; **S371** : la caméra à demi immergée, le milieu par pixel à l'objectif ; **S379** : les rides de la pluie, factices (8.9, R28 reçue) ; **S380** : la pluie complète ([ADR-205](adr/ADR-205-la-pluie-complete.md)), d'abord dans l'air — gouttes et extinction (R29 reçue) ; **S381** : le ciel de pluie, couvert de la CIE, sans soleil (R30 reçue) ; **S382** : l'occultation du ciel et les ombres portées (ADR-206, R31 reçue) ; **S383** : les gerbes (R32 posée) ; **S392** : les surfaces mouillées, pièce 5a (R33 reçue pour l'instant, S393). L'afficheur reste le banc. La physique garde l'ordre ci-dessous.

**La campagne du solveur volumique 3D temps réel** (décision de l'utilisateur, S379 : *« une session du plus dur et complexe […] un solveur […] qui va s'occuper des simulations 3D volumétriques ultra réalistes et performantes en temps réel dynamiquement »* ; R30 : *« le plus important »*). **Conçue en S384** ([conception](registres/CAMPAGNE-SOLVEUR-3D-S384.md), [ADR-207](adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)) : le domaine de la porte B consomme seul les 2 ms de δ ; colonnes hautes, pression par multigrille, APIC en bande, puis les scènes — onze sessions **C1 à C11**, dont cinq sans carte graphique. **S385 : C1 faite** — la multigrille 3D de la référence, 9 à 11 itérations quelle que soit la maille ([preuve](validation/MULTIGRILLE-3D-S385.md)). **S386 : C2 au pas linéaire** — la colonne haute unique mesurée insuffisante (÷1,47), la **colonne graduée** retenue ([ADR-208](adr/ADR-208-la-colonne-graduee.md), ÷2,55 sur une colonne de 7 m d'eau, [preuve](validation/COLONNES-HAUTES-S386.md)) ; 4.3 partiel. **S387** : au pas mobile, elle suit sa dispersion ; mais la surface de la porte B balaie 4,8 m — **la colonne graduée sert l'eau calme des contenants, pas la haute mer**, dont le levier de coût le plus probable est la multigrille sur la carte (C3 : la projection pèse 2,06 ms des 3,68 du pas). **S388–S389 : C4a** — APIC 3D dans le cœur, masse exacte ; la surface lue par un noyau de deux mailles, parois reflétées : ballottements à +0,39 % et +1,01 % à 2,5 cm ([preuve](validation/APIC3D-S388.md)). **S393 : C4b** — B10 en 3D, une sphère cinématique : pincement à 2,08 √(R/g), dans la plage publiée, convergé à 0,6 % ; 4.16 partiel ([preuve](validation/B10-APIC3D-S393.md)). **S394–S397 : C5a** — A316 en 2D : la circulation de la frontière du raccord était un défaut du banc, dont les colonnes n'advectaient pas ; reste une migration lente à 2,5 cm ([§14–16](validation/B10-APIC-S320.md)). **S396 : C8a** — fusion et séparation de domaines en référence, par ensembles de blocs ; 4.9 partiel ([preuve](validation/FUSION-S396.md)). **S398–S400 : C5b** — la zone des colonnes dans APIC 3D, à 0,03 point de δ ; puis la bande et l'échange (S399) ; la zone qui lit comme la bande (S400) : repos et migration tenus aux deux mailles ; **S406 : C5c** — le courant de surface attribué à la face de frontière, vue d'un seul côté, et levé (≤ 0,6 et ≤ 1,1 mm/s) ; **S407 : C5d** — la densité : une pose trop loin de la face ; posée à la face, **tout le critère du raccord tenu aux deux mailles**, frontière droite et fixe ([preuve](validation/RACCORD-3D-S398.md) §5–8). **S390 : C3a** — la multigrille sur la carte, éteinte par défaut, option `--multigrille` : à la porte B, le résidu de Jacobi-32 en 6 cycles, projection 1,08 ms contre 2,08 ; trois cas de cuve au plancher ([preuve](validation/MULTIGRILLE-3D-S385.md) §5). **S401 : C8b** — le domaine épars en référence : un ensemble de blocs dans une fenêtre, le pas mobile sur l'ensemble (un rectangle à un ulp du dense), qui suit sa perturbation et prévoit l'objet (ADR-013 §2), à 0,26–0,66 mm du domaine entier ; 9.2 partiel ([preuve](validation/DOMAINE-EPARS-S401.md)). **S402 : C8c** — [ADR-005](adr/ADR-005-zone-de-transition.md) restauré, son corps manquait depuis S35 ; le changement de niveau par transfert d'état ([ADR-210](adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md)) : sauts de 0,1 à 2 mm, une bosse gardée à 1,8 mm pendant le rang 4, quand le cycle de vie d'ADR-005 §5 la perd ([preuve](validation/NIVEAUX-S402.md)). **S403 : C8d** — le rang 4 dans l'ordonnanceur : le non-focal qui perd le moins descend (1,4 mm contre 14 mm au témoin), l'issue de la famine déclarée (rang 5, à l'hôte) ; S351 au bit sans déclaration ([preuve](validation/FAMINE-S403.md)). **S404 : C8e** — l'épars et les niveaux sous le **pas couplé**, en mer : le bord de l'ensemble absorbe comme celui de la boîte (un rectangle à un ulp de son dense sous la houle), le transfert porte l'ensemble (au bit), l'épars qui suit à 0,14 mm du domaine entier, et à 0,14 mm de lui à travers un changement de niveau ; il ne se vide pas en 10 s ([preuve](validation/MER-EPARS-S404.md)). **S405 : 9.3** — la prédiction balistique dans le cœur (`ballistic`), consommée par le domaine épars en mer : la région d'impact prête 1,11 s avant l'impact quelle que soit la cadence de revue ([preuve](validation/IMPACT-PREVU-S405.md)). **S408 : C6a** — la frontière qui bouge : la bascule colonnes ↔ particules à masse exacte et un critère avec hystérésis (`ColumnsSwitch`) ; sur B10, le pincement un pas plus tôt qu'APIC seul avec 22 % des colonnes en particules, 19 s contre 36 ; deux pertes de masse du pas de S399 trouvées et corrigées (10⁻⁹ → 10⁻¹²) ; 4.10 partiel ([preuve](validation/BASCULE-S408.md)). **S409 : C3b** — la multigrille sur la carte à **10 cm** : nécessaire (8 cycles, projection ÷ 2,8 au moins à résidu égal) ; sous 2 ms par image, 8 m × 8 m à 30 Hz — qui y explose en 62 s, quel que soit le solveur (**A322**) — et **5,6 m × 5,6 m à 60 Hz**, stable ; A298 refermée sur ce pas (27 µm en deux minutes), la cuve fermée qui gagne de l'énergie à pas long (**A323**) ([preuve](validation/MULTIGRILLE-3D-S385.md) §6). **S410 : C6b** — la vague qui déferle (Chen et al. 1999, `ka` = 0,55) : APIC 3D se retourne à 0,706 √(λ/g) (Chen : 0,72) ; la bande le prévoit sept pas avant, à masse exacte, mais traîne (maintien) ou hésite (pente), et chaque conversion au sommet de la crête perturbe le déferlement ; à la demande de l'utilisateur, jugé à l'image — **R34 posée** ([preuve](validation/BASCULE-S408.md) §6). **S411–S412** : R34 reçu (maintien 0,3 s) ; les trucages d'une eau de qualité cinéma ([conception](registres/TRUCAGES-TEMPS-REEL-S411.md)) — décisions de l'utilisateur ([ADR-211](adr/ADR-211-les-trucages-retenus.md)) : **C6c = la bande étroite en profondeur** ([ADR-212](adr/ADR-212-la-bande-etroite-en-profondeur.md)), avant C7 ; la surface continue avant C10 ; les courants après la campagne ; au loin, le calcul d'avance, à réfléchir. **S413 : C6c-1** — la bande étroite en profondeur : l'eau profonde sur la grille sous un fond fixe, les particules au-dessus, la masse au bit ; sur 30 s de ballottement, à 0,2 point d'APIC seul aux deux mailles, 4,7 à 7,7 fois moins de particules ([preuve](validation/BANDE-ETROITE-S413.md)). **S414 : C6c-2** — le fond placé par le critère : sur B10, la cavité d'APIC seul au chiffre près avec 7 fois moins de particules que la bande pleine ; la prédiction du corps (idée de l'utilisateur) au pas même d'APIC seul à horizon court ; sur la vague de Chen, ÷ 6 et deux fois plus vite — R35 posée ([§5](validation/BANDE-ETROITE-S413.md)). **S415 : C6c-3** — R35 reçue ; le fond qui suit l'écoulement (idée de l'utilisateur) : sur un tourbillon enfoui, la vitesse rend l'énergie d'APIC seul avec 2,4 à 3 fois moins de particules ; sous une houle, vorticité absolue et vitesse prennent tout — la voie : la vitesse propre de δ, relative à B ([§6](validation/BANDE-ETROITE-S413.md)). **S416 : C7a** — C7 découpé en cinq (nu, corps, zone et fond, relatif à B, budget) ; le pas d'APIC nu sur la carte, étage par étage à l'arrondi de la référence ; le ballottement à 0,46 mm sur 10 s ; 2,05 ms pour 12 800 particules, 4,77 ms pour 102 400 (47 ns par particule), la projection d'abord ([preuve](validation/APIC-CARTE-S416.md)). **S417 : C7b et C7c-1** — B10 nu sur la carte, pincement au pas de la référence au chiffre près (l'écart de `φ` au col, 9,2 mm, sous celui de la référence contre elle-même, 22,6 mm) ; la zone des colonnes sans échange, à 0,003 mm, **le volume en entiers, exact** ; C7c découpé en quatre ([§7–9](validation/APIC-CARTE-S416.md)). **S418 : C7c-2** — l'échange à la frontière, fidèle à l'ordre de la référence (tableaux indice pour indice, gestes sur un fil) : le raccord sur 30 s, volume exact à 0 quantum, surface à 4,4 mm sur la courbe des témoins (3,4 et 4,0) ([§10](validation/APIC-CARTE-S416.md)). **S419 : C7c-3** — le fond de la bande sur la carte : la bande étroite à 1,45 mm de la référence sur 30 s, volume exact ([§11](validation/APIC-CARTE-S416.md)). **S420 : C7c-4** — la bascule et le fond placé sur la carte : **B10 en bande étroite, pincement identique au chiffre près**, volume exact, `φ` au col dans l'enveloppe de trois témoins ; 28,7 ms ([§12](validation/APIC-CARTE-S416.md)). **S421 : C7e, premier temps** — le pas de B10 en bande étroite de 29,0 à 4,9 ms à sémantique exacte (l'échange en groupe, la liste des faces-mailles actives, le plafond adaptatif) ([§13](validation/APIC-CARTE-S416.md)). **S422 : C7e, la multigrille** — la projection d'APIC préconditionnée par un cycle en V sur la carte (symétrique, 11 à 14 itérations au lieu de 207) : 3,9 ms par pas, toutes les issues tenues ([§14](validation/APIC-CARTE-S416.md)). **S423 : C7e, la bascule et la surface** — bascule 1,9 → 0,48 ms (retrait à forme close, ensemencement par graine), surface 0,73 → 0,11 (32 fils par maille), multigrille par défaut : pas + bascule 3,74 ms au p99 ([§15](validation/APIC-CARTE-S416.md)). **S424 : C7e, la projection** — 1,35 → 0,885 ms au p99 (profil par noyau ; niveaux grossiers en mémoire de groupe ; restrictions et prolongation hors du groupe), issues identiques au chiffre près ; création des pipelines 80 → 27 s ([§16](validation/APIC-CARTE-S416.md)). **S425 : C7e, la fin du pas** — le profil désigne quatre noyaux d'un seul fil (préfixes, tri, séparation, solde vertical) : pas + bascule 3,25 → 2,45 ms au p99, issues identiques ([§17](validation/APIC-CARTE-S416.md)). **S426 : C7e, l'absorption face par face** — un fil par face touchée, à l'ordre de la référence : pas + bascule 2,31 ms au p99 ; la bande sur 30 s change par l'arrondi de FXC (isolé au bit) ([§18](validation/APIC-CARTE-S416.md)). **S427 : C7e, les gestes de l'échange groupés** — poses et retraits d'un solde d'un coup, au bit ; une course isolée et corrigée (`workgroupUniformLoad` sur un élément de tableau) : pas + bascule 2,16 ms au p99 ([§19](validation/APIC-CARTE-S416.md)). **S428 : C7e, la projection resserrée** — niveaux grossiers à 512 fils et sans barrières vides, restrictions en coopération, au bit : pas + bascule 2,07 ms au p99 ([§20](validation/APIC-CARTE-S416.md)). **S429 : C7e reçu** (l'utilisateur : le surplus de 0,07 ms accepté) ; **C7d conçue** (C7d-1 le critère relatif en référence, C7d-2 sur la carte, C7d-3 la bande dans la production couplée, après le mode relatif sur la carte et A320) ; **C7d-1 en partie** — la vitesse propre de δ : rien sous une houle calme, 0,94 de la crête raide ([§21](validation/APIC-CARTE-S416.md)). **S430** : la déformation propre écartée ; le critère (a) de C7d-1 réécrit ([§21.2](validation/APIC-CARTE-S416.md)). **S431 : C7d-1 reçu** — la vitesse propre de δ avec relâche, entrée 0,3 / relâche 0,15 m/s ([§21.3](validation/APIC-CARTE-S416.md)). **S432 : C7d-2 reçu** — le seuil de vitesse propre, sa relâche et le fond B dans la décision de la carte, identiques à la référence ([§21.4](validation/APIC-CARTE-S416.md)). **S433 : la conception de C7d-3** — la bande entre dans le pas couplé, ses particules portent la vitesse propre de δ ; C7d-3a A320 (la forme `∇(U·u′) − U×ω′`), C7d-3b le mode relatif sur la carte, C7d-3c la bande relative en référence, C7d-3d sur la carte ; le témoin d'A320 rejoué au chiffre près ([§22](validation/APIC-CARTE-S416.md)). **S434 : C7d-3a non reçu** — la forme de Bernoulli éprouvée ne freine A320 que de 15 à 20 % ; le terme d'ADR-209 n'y fait rien ; G seule et R seule stables, leur somme non ([MER-S369](validation/MER-S369.md) §6). **S435** : A320 à la longueur d'onde de la houle, 0,056 s⁻¹ à maille nulle (deux fois Benjamin-Feir) — pas de grille ; **A324** ouverte ([MER-S369](validation/MER-S369.md) §7). **S436 : A324 corrigée** (le fantôme latéral du mode relatif, [MER-S369](validation/MER-S369.md) §8). **S437 : C7d-3a non reçu** — l'excès d'A320 dépend de la place du repos dans la maille et ne se montre qu'à 16 mailles par longueur d'onde ([MER-S369](validation/MER-S369.md) §9). **S438** : la cause d'A320 non trouvée (l'échelle en mailles par longueur d'onde indécise, [MER-S369](validation/MER-S369.md) §10). **S439** : le mode relatif sur la carte, porté ; un critère manqué de 0,01 % ([§22.10](validation/APIC-CARTE-S416.md)). **S440 : C7d-3b reçu** (l'écart accepté) ; A322 sous le mode relatif : tient 120 s, des bouffées de maille ([MULTIGRILLE-3D-S385](validation/MULTIGRILLE-3D-S385.md) §7). **S441** : les bouffées, la bande relative FTCS ; Lax-Wendroff les lève ([MULTIGRILLE-3D-S385](validation/MULTIGRILLE-3D-S385.md) §8). **S442 : A322 levée en mode relatif** (l'écart accepté), la bande sous Lax-Wendroff par défaut ; A320 inchangée ([MER-S369](validation/MER-S369.md) §11). **S443 : C7d-3b reçu en entier** — la bascule : δ naît relatif à B, référence et carte ; coût 3,67 ms contre 3,73 ([§22.11](validation/APIC-CARTE-S416.md)) ; la méthode accélérée ([ADR-213](adr/ADR-213-accelerer-tolerance-plafond-rituel-bancs.md)). **S444** : [ADR-214](adr/ADR-214-b-entre-dans-la-bande.md), décision de l'utilisateur — B entre dans la bande (c1 à c4) ; c1 non reçu, δ croît dans `Apic3` relatif sous une houle stationnaire ([§23.1](validation/APIC-CARTE-S416.md)). **S445** : c1 plafonné — la bande simule l'eau totale, B à sa frontière (note d'ADR-214, [§23.2](validation/APIC-CARTE-S416.md)). **S446–S447 : c2**, le raccord bande ↔ mer — `Apic3` aux bords ouverts, le raccord conservatif : δ hors de la bande 9,4 mm, la masse au raccord à 0,01 % ; plafonné, retenu ([§23.3–23.4](validation/APIC-CARTE-S416.md)). **S448–S449 : c3 plafonné** — la bande déferle dans la mer par le raccord (`BandInSea`), la masse se tient, mais le raccord n'est pas robuste à son bord ([§23.5–23.6](validation/APIC-CARTE-S416.md)). **S450–S453 : la surface continue** (ADR-211 D2) — l'isosurface du champ unique `φ`, étanche, fondue au raccord ; **R37 reçu** (S452) ; en direct sur la carte seule, fenêtre `--surface-direct`, 1,3 ms par image ([SURFACE-CONTINUE-S450](validation/SURFACE-CONTINUE-S450.md)). **S454–S456 : C10-1 et la v1 solide** ([ADR-215](adr/ADR-215-autonomie-jusqu-a-une-v1-solide.md)) — le saut sur une scène de 4 m, la carte seule, stable (le plafond relevé), à 0,98 du temps réel, sous une houle qui entre par les bords ouverts, masse exacte ([C10-SCENES-S454](validation/C10-SCENES-S454.md)). **S457–S463** : la lumière reçue, la scène `--v1` (R38 reçu : *« Correct pour une V1 »*) ; C10-2 plafonné (le raccord à son bord) ; **C11** — la scène dans Godot, rejouée, caustiques et surface fine ([C10-SCENES-S454](validation/C10-SCENES-S454.md) §7–§12). **S464–S466** : le direct dans Godot (0,94 du temps réel), la pluie, le joueur debout — une capsule, 60 s, masse exacte ; **S467–S469** : le joueur éclairé comme l'eau, son ombre, le direct remesuré (0,995 du temps réel) ([C10-SCENES-S454](validation/C10-SCENES-S454.md) §13–18). **Suivantes** : la caméra qui suit le joueur ; le plafond du domaine, qu'une nappe ne doit plus faire diverger ; c4 après la stabilité du raccord ; le pool de blocs de C8 sur la carte, au poste. **S384, l'utilisateur** : *« Solveur 3D ici »* — la campagne dans la session cloud ; la pluie (pièce 5, R32) au poste.

**Front 0 — 36 points qu'une session peut faire avancer sans rien attendre.** Proposé, dans cet ordre :

1. **La v1 en une scène vivante** — réunir ce que les portes ont reçu séparément : la coque dans la production de δ
   (6.4 — **S358** : le pas linéaire sur la carte, solide fixe ; reste la coque qui bouge, ADR-193), l'ordonnanceur et le rang 1 dans l'afficheur (4.2), l'interpolation du rendu à 30 Hz (4.19 — **faite en
   S353**, R18 reçu en S369) ; puis la revue du prix du rang 1 (A319), que 4.5 et 9.9 attendent.
2. **Le lot 5, une session sur deux** (ADR-184 D1, ADR-190 D4) : 4.16, par A316 — 22 points en aval.
3. **La bathymétrie** (2.7) — 20 points en aval ; **S362** : la référence ; **S364** : l'entrée dans B, isobathes droites
   (ADR-196) ; restent les chemins de B jusqu'à Godot, la 2D, la marée —, puis W
   au-dessus du plan moyen (3.9) et les courants (2.6).
4. **Le reste du front 0**, par système : couplage (4.7, 4.18 ; 4.8 et 4.21 par A320), volumique (4.15, 6.5), V (5.2, 5.4 — **S372** : vannes et pompes, [ADR-199](adr/ADR-199-vannes-et-pompes-dans-v.md) ; **S374** : la piscine rejouée dans Godot, et la décision de l'utilisateur, la dynamique des contenants en 3D volumétrique, [ADR-200](adr/ADR-200-la-dynamique-des-contenants-en-3d-volumetrique.md) : porte E, 5.10 ; **S375** : le bassin en δ 3D, invisible à 20 cm ; **S376** : le niveau de détail des contenants, [ADR-202](adr/ADR-202-niveau-de-detail-des-contenants.md) ; **S377** : ses zones d'ombre, [ADR-203](adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) ; **S378** : la pluie dans V, bâches comprises, [ADR-204](adr/ADR-204-la-pluie-arete-de-v.md) —, 5.6, 5.7),
   solides (6.1, 6.3), rendu (8.1 — le cœur dans Godot —, 8.2, 8.3, 8.5, 8.8, 8.9), budget (9.2, 9.3, 9.7, 9.8, 9.13), et 1.8, 7.1, 7.7, 10.8,
   11.2, 12.1, 13.1.

**Fronts 1 à 5 — 43 points**, qui s'ouvrent à mesure ; le registre dit lesquels. **E — 38 points** attendent un fait
ou une action de l'utilisateur, demandé au moment où le point bloque (ADR-190 D5) : le réseau (10.1, *« pas encore »*,
S370) en commande 14 à lui seul, puis la météo — à la fin —, le verdict du rang 1 ([ADR-197](adr/ADR-197-reponses-du-2026-09-26.md) : Godot moteur
du jeu entier, 8.1 au front 0 ; 5.11 hors du périmètre ; la voie d'A289 tranchée, ADR-198 : 4.8 et 4.21 au front 0).

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
