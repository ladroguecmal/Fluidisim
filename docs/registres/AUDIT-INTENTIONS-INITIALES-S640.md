# Audit des intentions initiales — S640, 2026-10-07

*Sur la demande de l'utilisateur (« lit tous les documents écrits au début … intentions initiales non prises en compte, changées, ou qui ne portent plus d'intérêt »). Relecture menée par un agent en lecture seule ; trois constats vérifiés à la main (le seuil 0,35·Hs de S609 contre ADR-111/112 ; `saint_venant_2d.rs` en `f64` contre I-08 ; I-14). Rien n'est tranché ici : ce qui relève de l'utilisateur lui est soumis.*

Lus en entier : les deux documents sources, `01_INVARIANTS.md`, ADR-001 à ADR-021, SPEC-001, 002, 004, 005, 006, SPEC-003, CAS-CANONIQUES, PLAN-BENCHMARK, le début des registres ANGLES-MORTS et QUESTIONS-OUVERTES (dont la traçabilité §2–§32). Comparés à : BOUSSOLE, LISTE-PROJET-FINI (121 points), FEUILLE-DE-ROUTE, ADR-027, 028, 197, 218, 219, 247. Chaque absence affirmée ci-dessous a été vérifiée par recherche dans la liste et dans les ADR 022 à 258. *(Le `guide_topologie_ocean_haute_mer_plage.md` de `docs/sources/` date de S306 : ce n'est pas un document fondateur.)*

## 1. Intentions initiales non prises en compte

| # | Intention | Source | Trace actuelle |
|---|---|---|---|
| 1.1 | **L'aération modifie la flottabilité, de façon autoritaire** : un nageur ne flotte plus dans l'eau blanche, un navire perd de la portance (`A_rep`, `WaterSample.aeration`) | source §13.1 ; ADR-014 §5.2 ; ADR-021 §5 ; SPEC-004 §2 ; SPEC-006 §4.5 | **Aucune** dans la liste : 7.3 est purement visuel, et 6.2 ne cite que « la turbulence ». ADR-062, 077 et 178 la mentionnent sans la construire |
| 1.2 | **Le cycle des flaques** : un nœud naît au-delà de `V_min` = 2 L (accumulateur de terrain) et meurt avec hystérésis ; le mouillage de surface sert de puits universel ; TTL de 30 min pour les nœuds sans propriétaire | source §2.2 ; zones §17 (« seuil de création d'une nouvelle flaque ») ; ADR-010 §5, §7 ; ADR-027 §5 ; ADR-022 §4.3 | 5.5 parle de « la flaque » (infiltration) et de rétention de surface. **Ni la création, ni la destruction, ni le TTL** |
| 1.3 | **Transferts de V vers la rivière, la mer ou un caniveau** | source §2.2 ; zones §17 | 5.3 s'arrête à « l'hôte lit le déversé ». 5.7 ne couvre que les liquides autres que l'eau. **Aucun transfert de masse de V vers B, W ou δ** |
| 1.4 | **Hs borné par le fetch** dans les lacs et les baies : « le système doit l'imposer » | ADR-011 §5 ; SPEC-001 §4 ; ADR-004 §2.2 (champ `fetch`) ; SPEC-005 §2 | Le mot « fetch » n'apparaît pas dans la liste. 2.3 dit « ses vagues de vent », sans borne |
| 1.5 | **Sites turbulents permanents des rochers** : un terme stationnaire dérivé, publié en `BreakerVertex`, liste tirée de `h < 1,28·H`, allumée et éteinte par la marée | source §11.3 ; zones §26 ; ADR-013 §7.4 ; ADR-023 §4 | **Nommé nulle part.** 3.5 couvre les côtes et les îles, 4.15 les rochers en général (« turbulence »). SPEC-001 §3 (note S306) le dit « non construit » |
| 1.6 | **Retour d'une gerbe vers W** : une masse fragmentée de plus de 5 L émet un événement W, en dessous elle ne fait qu'un dépôt d'écume | source §13.3 ; ADR-014 §4 | **Aucune** (7.2 ne parle que de l'émission de gouttes) |
| 1.7 | **La glace cède** : la cellule devient des floes, un `WaveEvent` de rupture est émis, la masse est rendue à V, la surface navigable s'effondre | ADR-017 §4 ; ADR-018 §5 | 7.6 et 7.7 portent la croissance, la portance et `ice_capacity` ; **pas la rupture**. La neige sur la glace et le brise-glace (ADR-017 §7.3, §7.5) n'ont aucune trace |
| 1.8 | **δ anime la pose de rendu des corps** : un ressort borné à 8 cm et 3° entre pose physique et pose affichée | ADR-008 §1 ; SPEC-004 §7.2 | **Aucune** (4.13 et 6.1 ne disent que l'absence d'effet de δ sur la trajectoire) |
| 1.9 | **Horloge client** (filtre de Cristian, modulation du taux à ±0,1 %, resynchronisation après une coupure de plus de 5 s) et **hash de conformité de B à 1 Hz** entre client et serveur | ADR-003 §2.1, §2.4 | Non nommés. Ni 1.7 ni 10.1 à 10.3 ne les citent |
| 1.10 | **Rejeu de sessions réelles** (« à instrumenter dès les premières versions jouables »), **batterie de saturation** (la « falaise »), série temporelle de dérive des coûts, bisection automatique | SPEC-003 §7.1, §7.2, §8, §9.2 | Seulement sous le parapluie de 13.1 (« H1, H3 »). La note S321 de SPEC-003 les dit « intention, pas état ». Le rejeu devient directement utile pour 13.4 (DyingStar) |
| 1.11 | **Verser un liquide d'un référentiel à un autre** (depuis un vaisseau en vol), par V ou par des particules balistiques | ADR-002 §3 | **Aucune** — alors que c'est un cas naturel dans un jeu spatial |
| 1.12 | **L'air respirable d'une poche** (volume × O₂, consommation par occupant) | ADR-015 §3, §6.4 ; DOSSIER-REUNIONS fiche 13 | **Aucune.** ADR-028 §5 l'avait renvoyé à « quand il y aura un jeu » ; il y en a un |
| 1.13 | **Domaine fin perturbatif imbriqué** `δ_grossier + δ_fin` (les résolutions mixtes) | ADR-006 §3.3 ; PLAN-BENCHMARK B5 | Ni retenu ni écarté. 4.3 cite B5 pour les blocs épars, et ADR-210 a choisi le transfert d'état pour changer de niveau |
| 1.14 | **Le détail de l'audio** : trois lits et un bus, délai de propagation, mur acoustique, occlusion par l'aération (`ListenerAggregate`, 16 secteurs), résonance de remplissage, trois champs de `WaveEvent`, lit de la pluie | ADR-016 ; SPEC-006 §4.2–4.3 | Une seule ligne (7.8, « absent, à la fin, Wwise »). Le périmètre final n'est pas détaillé, et le point risque d'être validé sur une partie seulement |
| 1.15 | Mineurs : transition d'exposition à l'entrée dans l'eau ; visibilité sous l'eau publiée pour l'IA ; LOD propre de la transparence, de la réfraction, des caustiques et des particules sous-marines ; effondrement massif | ADR-019 §7.2 ; ADR-018 §7.4 et SPEC-006 §9.5 ; source §16 ; zones §20 | Absents : 8.4 ne donne un LOD qu'à l'écume, au spray, aux gouttes et aux bulles ; 11.3 ne cite que tsunami, crash et très grand navire |

## 2. Intentions qui ont changé

### 2.1 Changements implicites (dérives non décidées) — à traiter en priorité

1. **Le critère de bascule 0,35·Hs a été réintroduit.** ADR-001 (note S161), ADR-111 et ADR-112 l'ont retiré (« aucun seuil n'est rétabli ou gelé ») : le paramètre qui gouverne est `max|δ|/h`. Or S609 l'a implémenté et reçu comme critère (`validation/SUBSTITUTIF-S609.md`, l. 4 et 25), et le point **4.11** l'affiche. C'est une régression.
2. **I-14 n'est plus respecté.** L'invariant exige que tout nombre vienne d'une formule de SPEC-001 ou SPEC-002, ou soit marqué « à calibrer ». Depuis la v2, des dizaines de lois portent des résultats sans y figurer : Green–Ampt, Brooks–Corey, Mein–Larson, Tomiyama, Schiller–Naumann, Willis et Rayleigh, Marshall–Palmer, Synolakis, Keller, Thacker, Davies–Taylor, Minnaert (vérifié : zéro occurrence dans SPEC-001 et SPEC-002). L'invariant n'a jamais été amendé ; ADR-207 et ADR-208 écrivent « relus, aucun amendé ».
3. **I-08 (calcul en `f32` local) n'est plus respecté.** Plusieurs modules de la v2 calculent en `f64` : `saint_venant_2d.rs` (45 `f64`, 0 `f32`), `glace.rs`, `refraction.rs`, `geoide.rs`, `hydro_charge.rs`. ADR-141 interdit pourtant « tout champ ou réduction physique en f64 ». Les seules exceptions écrites sont V (ADR-139) et les coefficients temporels de δ (ADR-141). Aucun ADR ne range ces modules parmi les « références » ; B7 (S618) mesure même Saint-Venant 2D comme un module de jeu.
4. **Le terrain et l'eau ont échangé leur sens.** SPEC-005 §3 et ADR-011 §3.1 posaient : la rivière est la source de vérité, le terrain suit, il faut le graver. ADR-011 ajoutait « il faut choisir ». Depuis ADR-219 D4, *l'eau lit les hauteurs du terrain de DyingStar* : le choix a basculé sans être nommé. Pourtant **12.4** s'intitule toujours « eau en amont du terrain » et **12.2** prévoit toujours la gravure. Cela touche aussi 2.4 et 13.4 (« sans régression du jeu »).
5. **Le pavage des régions n'est pas tranché.** ADR-002 §2.4 et SPEC-005 §2 prévoyaient des régions hydrographiques sur un cube-sphère ; DyingStar pave ses planètes en tuiles HEALPix (ADR-219 D4). Aucun ADR ne choisit. 11.1 et 11.2 sont muets.
6. **La bibliothèque côtière était dimensionnée pour des niveaux faits à la main.** ADR-013 §4, ADR-005 §4.1 (« contrainte de design de niveau »), SPEC-005 §6 et ADR-027 §4.2 comptaient « 50 plages, quelques-unes par carte ». Une planète procédurale de 6 356 km n'a plus d'auteur pour borner ce nombre. **12.3** attend « les plages réelles » sans dire lesquelles, ni combien.
7. **I-03 et la reformulation « un seul PC ».** ADR-219 D2 reformule 1.7 et 10.3 en déterminisme « entre les chemins d'exécution de ce PC ». C'est explicite, mais c'est une décision sur la **preuve**. I-03 n'est pas amendé, et pour un MMO joué sur d'autres machines l'**intention** reste le déterminisme entre plateformes. Rien n'écrit cette distinction : la preuve allégée pourrait devenir l'exigence.
8. **Le solveur δ n'a pas été choisi par banc.** La source (zones §18) et B3 demandaient un choix par mesure : quatre scénarios, iso-qualité, couple coût et latence, double aveugle. En fait, la grille MAC à fonction hauteur vient d'ADR-175 (« B3 préliminaire », décision de projet), et APIC de l'utilisateur (ADR-186), après une comparaison 2D « sans validation » (S318). B3 n'a jamais tourné, et **13.3** l'attend encore comme verdict.
9. **Les zones de déferlement larges ne sont pas décidées.** La source (§11.1) veut le rouleau en 3D et refuse une 2D qui donnerait un rouleau ou une mousse faux. ADR-005 §4.1 dit qu'au-delà de 250 m, c'est du déferlement 2D dans W avec un habillage de rouleau procédural, le 3D restant à quelques mètres du joueur. La liste (4.14, rouleau 3D en APIC ; 3.5) ne dit pas comment rendre une zone de surf de plusieurs centaines de mètres hors du rayon 3D, et l'habillage procédural n'apparaît nulle part.

### 2.2 Changements explicites (arbitrages ou ADR)

| Intention d'origine | Devenue | Par | Nature |
|---|---|---|---|
| Trois régimes, avec une zone de transition symétrique | Quatre couches B/W/δ/V ; transition asymétrique (éponge, puis transduction δ→W) | ADR-001, ADR-005 | Explicite, mais ces ADR sont toujours « proposée » : seul ADR-020 est acté (ADR-028 §6.3). La frontière imperceptible reste à atteindre (8.7) |
| « Conservation de phase : sans objet » | Démentie par la mesure (S274, A289) → δ relatif à B, point 4.21 | ADR-198, sur délégation d'ADR-197 D6 | Explicite |
| Subdivision anisotrope sur niveaux prédéfinis | Blocs épars sans anisotropie, puis colonne graduée | ADR-006 §3.1, ADR-208 | Explicite (technique) |
| Simulation hors caméra réduite | Destruction du domaine et relais par W ; puis le « calcul d'avance au loin », à réfléchir | ADR-013 §6 ; ADR-211 D4 (utilisateur) | Explicite |
| Pas de transfert d'état entre solveurs | Transfert d'état adopté pour changer de niveau | ADR-007 §3 → ADR-210 | Explicite (technique, ADR-215 et 222) |
| Flottabilité « à benchmarker » | Calculée sur B+W ou V seulement ; poussée au centre de la part immergée | ADR-008, ADR-227 | Explicite |
| Avance plus rapide que le temps réel | « Presque jamais », sauf pour un domaine substitutif | ADR-013 §1, §4 | Explicite |
| Profils et adaptation au matériel | Ce PC seul ; le matériel faible simulé par bridage (WARP) | ADR-174, ADR-219 D2 | Explicite |
| Multijoueur | « Pas encore », puis « le nôtre, prêt pour Horizon » | ADR-197 D1 (note S370), ADR-219 D1 | Explicite |
| Rendu | Moteur maison, puis Godot 4, puis moteur du jeu entier, puis DyingStar sur Godot 4.7 | ADR-191, 192, 197 D2, 219 | Explicite |
| Audio agnostique, « à confirmer avec l'équipe audio » | Audio de Godot, puis Wwise ; à la fin | ADR-197 D5, ADR-219 | Explicite |
| Budget (ADR-012 §3 : 2 ms CPU, 2,5 ms GPU) | 60 images/s et 2 ms pour l'eau ; profil de travail ; budget non opposable pendant la construction ; la physique d'abord | ADR-125, 174 D3, 178 D4, 247 | Explicite. **Effet cumulé à montrer à l'utilisateur** : le critère principal de la source — le rendu perçu en temps réel — est reporté à la fin de la v2 |
| Glace | Lacs et baies seulement, ni banquise ni icebergs | ADR-027 §3 | Explicite, mais **prise par délégation quand « il n'y a pas de jeu »** : la prémisse a changé |
| Positions monde | `int64` à 1/2048 m | ADR-028 §3 | Explicite, décidé « sans interlocuteur ». Compatible avec la double précision de DyingStar par conversion à la frontière, à relire à l'intégration |
| Écume | Suspendue, puis reprise d'après les vidéos V2 et V3 | ADR-197 D9, ADR-219 D5 | Explicite |
| Eaux souterraines | Hors du périmètre | ADR-197 D4 | Explicite |
| Météo et son | À la fin ; l'atmosphère après l'eau | ADR-197 D5, 217 D3, 218 D4 | Explicite |
| Courant macroscopique inscriptible ? | Non, en lecture seule | ADR-011 §2 | Explicite |
| Water Manager global avec affinage local | Ordonnanceur « sac à dos » sous budget | ADR-012 §1 | Explicite |

## 3. Intentions qui ne portent plus d'intérêt

1. **Les « équipes » destinataires.** On les trouve dans le DOSSIER-REUNIONS, dans les statuts d'ADR-016 à 018 (« à confirmer avec l'équipe… »), dans ADR-013 §7.2 (équipe véhicules), dans SPEC-005 (« engage les équipes terrain »), dans ADR-019 §7. ADR-028 les a requalifiées, mais les statuts sont restés. **A59** est toujours marqué ⏳ « attend l'équipe terrain », alors que le terrain de DyingStar est déjà sculpté (données QGIS et reliefs procéduraux). L'urgence d'agir « avant qu'un mètre carré de côte ne soit sculpté » est caduque ; la vraie question est devenue : le terrain de DyingStar applique-t-il le géoïde ?
2. **Le jury perceptuel** (SPEC-003 §5.3 : 8 personnes dont 3 hors de l'équipe, paire nulle ; double aveugle dans B1, B3 et B4) est impossible avec un seul observateur. Il est remplacé de fait par les verdicts de l'utilisateur et par le banc visuel (ADR-216), mais il reste dans le périmètre final de 13.1 et 13.3.
3. **Les seuils confiés à une assurance qualité** (ADR-027 §6, SPEC-003 §11.4) n'ont plus d'objet ; c'est déjà révisé par ADR-028 §4.
4. **L'infrastructure multi-machines** est sans objet sur un seul PC sans dépôt distant : intégration continue hebdomadaire, hashes multi-plateformes (SPEC-003 §7), producteur de cuisson désigné, postes d'artistes, magasin d'artefacts, promotion (SPEC-005 §7.2, §11.3).
5. **Le budget par joueur en écran partagé ou sur serveur d'écoute** (ADR-012 §8.2) est sans objet pour un MMO à serveur dédié (Horizon).
6. **`ABI_VERSION`** et le solveur tiers compilé séparément (SPEC-004 §1.4) n'ont plus de tiers à servir. Impact faible.
7. **La « contrainte de design de niveau »** et les plages « quelques-unes par carte » (ADR-005 §4.1, ADR-013 §4) ne valent plus telles quelles (voir 2.1, point 6).
8. **B3 comme banc de choix du solveur** : la décision est prise (ADR-175, ADR-186). Il faut soit le requalifier en validation du solveur retenu sur ses quatre scénarios, soit le retirer de 13.3.
9. **L'éditeur de rivières d'auteur** (12.2 : interface, spline, dessin ; règles de SPEC-005 §5) perd son utilisateur si les rivières viennent du terrain de DyingStar.
10. **Le statut « proposée » d'ADR-001 à 019 et 021** ne dit plus rien : la revue promise (ADR-028 §6.3) n'a jamais eu lieu.

## Recommandations

**Ajouter à la liste** (ADR-218 D2 permet d'ajouter sans l'utilisateur) :
- l'aération autoritaire et son effet sur la flottabilité (1.1) ;
- le cycle des flaques et les transferts de V vers la rivière ou la mer (1.2, 1.3) ;
- la borne de Hs par le fetch (1.4) ;
- les sites turbulents dérivés (1.5) ;
- le retour d'une gerbe vers W (1.6) ;
- la rupture de la glace (1.7) ;
- la pose de rendu animée par δ (1.8) ;
- l'horloge client et le hash de conformité, sous 10.x (1.9) ;
- le rejeu de sessions et la batterie de saturation, nommés dans 13.1 (1.10) ;
- le transfert entre référentiels (1.11) ;
- le détail de 7.8 selon ADR-016 et SPEC-006 §4 (1.14).

**Corriger, ce sont des décisions techniques** :
- 4.11 : retirer le critère 0,35·Hs de S609, ou écrire l'ADR qui le rétablit contre ADR-111 et ADR-112 ;
- I-14 et I-08 : amender par ADR (compléter SPEC-001 et SPEC-002 avec les lois employées ; déclarer quels modules sont des références en `f64`), ou mettre les modules en conformité ;
- 13.2 : retirer C11 de « non exécutés », puisque 6.1 le dit passé depuis S498.

**Soumettre à l'utilisateur** :
- le sens terrain–eau : graver le terrain de DyingStar ou en déduire les rivières (12.2, 12.4) ;
- cube-sphère ou HEALPix pour les régions ;
- quelles plages, et combien, sur une planète procédurale (12.3) ;
- la glace bornée aux lacs et aux baies : à garder pour DyingStar ? ;
- l'air respirable ;
- le traitement des zones de surf larges ;
- B3 : requalifier ou retirer ;
- le jury perceptuel, l'intégration continue multi-plateformes et l'écran partagé : retirer ou remplacer ;
- confirmer qu'I-03 reste, en intention, un déterminisme entre plateformes ;
- acter formellement ADR-001 comme remplacement des trois régimes de sa source.
