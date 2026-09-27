# Les trucages d'une eau de qualité cinéma, en temps réel — S411

2026-09-27. **Conception**, sans code, à la demande de l'utilisateur. Document de proposition : il ne décide rien ; ce que
l'utilisateur retiendra deviendra un ADR. La campagne qu'il prolonge : [CAMPAGNE-SOLVEUR-3D-S384](CAMPAGNE-SOLVEUR-3D-S384.md),
[ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md).

## 1. Les mots de l'utilisateur

Après la planche R34 ([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md) §39) :

> *« Il s'agit de 2D et de bille, encore loin du finale, qui est en 3D et une topology sans interstice visible dans l'eau sauf
> pour les jets. […] Point de réflexion est il intéréssant de simuler les billes en dessous en profondeur, car on ne les voit
> pas et ne sont pas en grand mouvement ou possibilités d'être arraché. Il faut réfléchir a comment pouvoir avoir une simulation
> digne des logiciels 3D comme HOUDINI ou autres spécialisé tout en étant en temps réel et dynamique. Je pense qu'il faut
> réfléchir a des astuces, trucages pour réussir. C'est comme pour le courant, peut être avoir un système de LOD pour le courant
> et avoir des courants plus généraux en profondeur et des courants détaillé en zone mouvementé, proche du joueur etc.... »*

Puis : *« Houdini était une référence pas forcément le choix adapté. »* — **la qualité de l'image d'un logiciel spécialisé est
la cible ; ses méthodes ne le sont pas.** Un logiciel de production calcule hors ligne, des minutes par image, sur des volumes
entièrement résolus ; nous avons **2 ms de carte par image** pour δ (ADR-174 D3). Ce qui suit ne cherche donc pas à reproduire
son calcul, mais **ce que l'œil en reçoit**, par des moyens faits pour le temps réel.

## 2. Le principe : on ne calcule que ce qui se voit ou ce qui agit

Toute l'architecture le dit déjà — B analytique partout, W par événements, δ seulement où il faut, V pour la masse
([ADR-001](../adr/ADR-001-decomposition-en-couches.md)). Les trucages ci-dessous l'appliquent **à l'intérieur** de δ et **à
l'image**. Chacun est jugé sur trois questions : que voit l'œil de ce qu'il remplace ? que coûte-t-il ? qu'existe-t-il dans le
dépôt ?

| | trucage | ce qu'il remplace | état dans le dépôt | gain |
|---|---|---|---|---|
| **T1** | **bande étroite en profondeur** | des particules jusqu'au fond | conçu (A1), **non construit** : la bande est pleine hauteur | particules ÷ 4 à 10 |
| **T2** | niveaux de détail de δ (25, 10, 5 cm ; épars ; qui suit) | une maille fine partout | partiel (C8, rang 4, S396–S404 ; C3b) | le budget, où il sert |
| **T3** | **surface continue pour l'image** | des particules montrées | absent pour la bande ; colonnes rendues | la qualité vue (R34) |
| **T4** | **particules diffuses** : embruns, écume, bulles | un solveur pour chaque goutte | écume de crête (S360, suspendue), pluie ; C9 prévu | des millions d'éléments sans pression |
| **T5** | **détail sous la maille par synthèse** | une maille de quelques mm | FFT fine dans Godot (ADR-195), rides de pluie | le millimètre sans le calculer |
| **T6** | le temps : 30 Hz et interpolation ; sous-pas seulement où il faut | 60 Hz partout | 30 Hz + interpolation (S353) ; A322 | la moitié du coût |
| **T7** | **courants à niveaux de détail** | un champ 3D partout | **conçu dès S01** (ADR-011), non construit (2.6) | un champ fin seulement près du joueur |
| **T8** | la prévision : le domaine prêt avant l'événement | un domaine qui naît en retard | reçue (ADR-013, 9.3, S405) | l'absence de « pop » |

## 3. T1 — la bande étroite en profondeur : la réponse à la question de l'utilisateur

**Non, il n'est pas utile de porter en particules l'eau profonde.** Sous quelques mailles, elle ne se voit pas, ne se retourne
pas, ne s'arrache pas : une grille la porte aussi bien, pour bien moins cher. C'est ce que la conception de la campagne voulait
(S384 §4.1, **A1** : « des particules APIC dans une bande **sous la surface** ») et que l'implémentation n'a pas fait — S398 à
S410 ont mis en particules des **colonnes entières**, du fond à la surface. La littérature l'a établi : **Narrow Band FLIP**
(Ferstl, Ando, Wojtan, Westermann et Thuerey, *Eurographics* 2016) garde les particules dans une bande sous la surface et
représente le reste du volume sur une grille régulière ; la méthode a été reprise en production (Houdini 16.5). Pour le temps
réel, Chentanez et Müller (2011) portaient déjà l'eau profonde par des **mailles hautes** — notre colonne graduée
([ADR-208](../adr/ADR-208-la-colonne-graduee.md)).

**Ce que la bande devient.** Non plus un ensemble de colonnes, mais un ensemble de **mailles** : celles à moins de `k` mailles
de la surface, **où qu'elle soit** — le long des parois d'une cavité, sous un jet, autour d'une poche —, dans les colonnes que
le critère de S408 désigne. Dessous, la vitesse est **eulérienne**, gardée et advectée sur la grille comme dans la zone des
colonnes (S398) ; une particule qui descend sous la bande y est **absorbée**, la grille en rend quand la bande descend — le
même échange à masse exacte que la frontière latérale du raccord (S399–S407), tourné à la verticale.

**Ce que cela rapporte**, sur nos propres bancs (estimé, *à mesurer*) :

| banc | eau sous la surface | bande de `k` = 4 à 6 mailles | particules |
|---|---|---|---|
| la vague de S410 | 20 mailles (1 m à 5 cm) | 4 | **÷ 4 à 5** |
| B10, la sphère (S408) | 64 mailles (3,2 m) | 6, la cavité comprise | **÷ 5 à 10** |
| la mer de la porte B à 5 cm | 70 mailles (3,5 m) | 6 | **÷ 10** |

**Ce qu'on y gagne aussi** : chaque conversion du sommet de la crête perturbait le déferlement (S410 §6.3) ; une bande qui suit
la surface en profondeur **n'a plus à convertir des colonnes entières** — seulement quelques mailles, près de l'eau qui bouge.
**Ce que cela coûte** : une frontière de plus (le bas de la bande), qui doit tenir la densité comme la frontière latérale l'a
demandé (A316, huit sessions) ; le risque est connu, l'outil aussi.

## 4. T3 — la surface continue : ce que l'œil doit recevoir

La planche R34 montrait des billes ; l'image finale est **une surface continue, sans interstice, dont seuls les jets se
détachent**. Rien de la bande ne doit se voir comme particules. Trois moyens, du moins cher au plus fidèle :

1. **La surface par la distance `φ`** que la bande reconstruit déjà (Zhu et Bridson 2005, `apic3d.rs`) : un maillage de son
   iso-zéro sur la bande, raccordé à la hauteur `η` des colonnes — une seule surface, deux sources.
2. **Des noyaux anisotropes** (Yu et Turk 2013) : la surface épouse l'étirement local des particules — les lames et les jets
   restent minces au lieu de gonfler en boules.
3. **En espace écran** (van der Laan, Green et Sainz 2009) : les particules projetées en profondeur, lissées par un flux de
   courbure, dans le nuanceur — aucune géométrie ; adapté à Godot (ADR-192).

Ce qui passe sous la maille — une lame plus fine qu'une maille, les gouttes d'un jet qui se brise — **quitte la surface** et
devient particule diffuse (T4). La frontière entre les deux est un seuil sur l'épaisseur locale, *à calibrer* sur image.

## 5. T4 et T5 — le détail qu'on ne simule pas

**T4, les particules diffuses** (Ihmsen, Akinci, Akinci et Teschner 2012 : embruns, écume, bulles unifiés). Émises par δ là où
l'air est piégé, où la crête se brise, où l'énergie cinétique est forte ; portées par la vitesse de δ ou balistiques, **sans
pression**, donc par millions sur la carte ; rendues en points, en écume sur la surface, en bulles dessous. C'est ce qui fait
l'essentiel de l'impression « cinéma » d'un déferlement. C9 de la campagne ; I-04 tenu (rendu seulement). L'écume est
**suspendue** par l'utilisateur jusqu'à des références photographiques (S368) : C9 commencera par les embruns et les bulles.

**T5, le détail sous la maille par synthèse.** δ donne le mouvement à 5 ou 10 cm ; les rides, la texture de la surface, la
turbulence fine se **synthétisent** par-dessus, modulées par ce que δ calcule (étirement, courbure, vitesse) — l'idée de la
turbulence au point le plus proche (Kim, Tessendorf et Thuerey 2013). Le dépôt en a les pièces : la FFT fine dans Godot
(ADR-195), les rides de pluie (R28). Il manque le lien : **l'amplitude du détail pilotée par δ**.

## 6. T7 — les courants : l'intuition de l'utilisateur est déjà la conception

[ADR-011](../adr/ADR-011-courants-et-ecoulements-diriges.md) (S01) pose exactement la hiérarchie proposée : **C0** un vecteur
(haute mer), **C1** un champ régional 2D précalculé (embouchures, détroits, côtes), **C2** un profil vertical analytique
(« des courants plus généraux en profondeur »), **C3** le champ 3D local d'un domaine δ (« détaillé en zone mouvementée,
proche du joueur ») — choisis par besoin, non par distance ; C0 à C2 en lecture seule pour les solveurs. **Rien n'en est
construit** (liste, 2.6). Ce qui s'ajoute depuis : δ est relatif à la dynamique de B ([ADR-198](../adr/ADR-198-la-voie-d-a289.md)) ;
le même geste vaut pour C0 à C2 — δ reçoit le courant comme il reçoit la houle, et ne porte que l'écart.

## 7. Ce que cela change à la campagne — proposé

| | proposition | à la place de |
|---|---|---|
| **C6c** | **la bande étroite en profondeur** (T1), et le critère qui suit la crête — le maintien de 0,3 s retenu par R34 | le critère seul |
| **C7** | APIC sur la carte, **dans sa version étroite** : ce qu'on porte sur la carte est ce qu'on garde | la bande pleine hauteur |
| **C9** | particules diffuses (T4), embruns et bulles d'abord ; puis la **surface continue** (T3) dans l'afficheur | inchangé, T3 ajouté |
| **C10** | les scènes, jugées sur une surface continue, non sur des billes | inchangé |
| **C11** | δ dans Godot, avec T3 et T5 | inchangé |
| hors campagne | T7 — les courants C0 à C2 (2.6), au front 0 de la liste | — |

L'ordre protège une dépendance chaque fois : T1 avant C7, parce que la carte reproduit la référence ; T3 avant C10, parce que
l'utilisateur juge l'image finale et non l'instrument (R34).

## 8. Ce qui demande l'utilisateur

1. **T1 comme C6c**, avant C7 ? *Proposé : oui* — c'est la réponse à sa question, et elle réduit C7.
2. **La surface continue (T3) avant les scènes** (C10) ? *Proposé : oui* ; laquelle des trois voies d'abord — *proposé : la
   voie `φ` dans l'afficheur, l'espace écran pour Godot*.
3. **Les courants (T7)** : quand ? *Proposé : après la campagne, au front 0*, sauf si une scène les demande plus tôt.
4. **Un événement scénarisé** (une cinématique) pourrait-il rejouer un calcul fait d'avance ? Ce n'est pas notre voie — tout est
   dynamique —, mais c'est le trucage le plus ancien du métier ; *à savoir seulement*, rien n'en dépend.

## 9. Sources

- F. Ferstl, R. Ando, C. Wojtan, R. Westermann, N. Thuerey, « Narrow Band FLIP for Liquid Simulations », *Computer Graphics
  Forum* 35(2), *Eurographics* 2016 — [TUM](https://www.cs.cit.tum.de/cg/research/publications/2016/narrow-band-flip-for-liquid-simulations/) ;
  repris dans Houdini 16.5 ([80.lv](https://80.lv/articles/houdini-16-5-narrow-band-flip)). Lus en session (résumé).
- G. Chen, C. Kharif, S. Zaleski, J. Li, « Two-dimensional Navier–Stokes simulation of breaking waves », *Phys. Fluids* 11,
  121 (1999) — le cas de S410 ([arXiv](https://arxiv.org/html/comp-gas/9605002)).
- *Cités de mémoire, non relus en session — à vérifier avant d'en tirer un chiffre* : N. Chentanez, M. Müller, « Real-time
  Eulerian water simulation using a restricted tall cell grid », SIGGRAPH 2011 ; N. Chentanez, M. Müller, T.-Y. Kim, « Coupling
  3D Eulerian, heightfield and particle methods for interactive simulation of large scale liquid phenomena », SCA 2014 ;
  M. Ihmsen, N. Akinci, G. Akinci, M. Teschner, « Unified spray, foam and air bubbles for particle-based fluids », *The Visual
  Computer* 28 (2012) ; J. Yu, G. Turk, « Reconstructing surfaces of particle-based fluids using anisotropic kernels », *ACM
  TOG* 32(1) (2013) ; W. J. van der Laan, S. Green, M. Sainz, « Screen space fluid rendering with curvature flow », I3D 2009 ;
  T. Kim, J. Tessendorf, N. Thuerey, « Closest point turbulence for liquid surfaces », *ACM TOG* 32(2) (2013) ; Y. Zhu,
  R. Bridson, « Animating sand as a fluid », SIGGRAPH 2005.

*Note datée du 2026-09-27 (S412)* : **réponses de l'utilisateur** — 1 et 2 validés, 3 « Ok », 4 *« cela dépends une simulation
d'un joueur de 15m peut être calculé a l'avance et plus le joueur se rapproche de la simulation et peux intérargir et simule en
temps réel, a réfléchir »*. Décisions et première analyse du point 4 : [ADR-211](../adr/ADR-211-les-trucages-retenus.md).
