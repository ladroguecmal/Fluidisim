# ADR-211 — Les trucages retenus : la bande étroite, la surface continue, les courants ensuite, le calcul d'avance au loin

- **Statut : actée**, S412, 2026-09-27 — **décisions de l'utilisateur** sur la proposition
  [TRUCAGES-TEMPS-REEL-S411](../registres/TRUCAGES-TEMPS-REEL-S411.md) §8 ; D4 est une **orientation à réfléchir**, pas une
  décision de construction.
- **Modifie l'ordre** de la campagne du solveur volumique 3D ([ADR-207](ADR-207-la-campagne-du-solveur-volumique-3d.md),
  [conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5) : C6c s'insère avant C7 ; la surface continue avant C10.
- **Conception de C6c** : [ADR-212](ADR-212-la-bande-etroite-en-profondeur.md).

## 1. Les mots de l'utilisateur

Verdict R34 et réflexion (S411) : *« Il s'agit de 2D et de bille, encore loin du finale, qui est en 3D et une topology sans
interstice visible dans l'eau sauf pour les jets. […] est il intéréssant de simuler les billes en dessous en profondeur, car on
ne les voit pas et ne sont pas en grand mouvement ou possibilités d'être arraché. Il faut réfléchir a comment pouvoir avoir une
simulation digne des logiciels 3D comme HOUDINI ou autres spécialisé tout en étant en temps réel et dynamique. […] C'est comme
pour le courant, peut être avoir un système de LOD pour le courant […] »* ; puis *« Houdini était une référence pas forcément le
choix adpaté »*.

Réponses au §8 (S412) : *« 1. Je valides ton choix 2. Je valides ton choix 3. Ok 4. cela dépends une simulation d'un joueur de
15m peut être calculé a l'avance et plus le joueur se rapproche de la simulation et peux intérargir et simule en temps réel, a
réfléchir »*.

## 2. Décisions

**D1 — La bande étroite en profondeur est C6c, avant C7.** Les particules ne portent l'eau qu'à moins de quelques mailles de la
surface, où qu'elle soit ; dessous, la grille (T1 ; Ferstl et al. 2016). C'était l'intention d'A1 (S384 §4.1) ; les colonnes
entières en particules (S398–S410) en étaient une étape. C7 porte sur la carte **la version étroite**. La masse reste exacte :
c'est une exigence de la campagne (§3.4), non une option. Structure : [ADR-212](ADR-212-la-bande-etroite-en-profondeur.md).

**D2 — La surface continue avant les scènes.** L'utilisateur juge C10 sur **une surface sans interstice**, dont seuls les jets se
détachent — jamais sur des particules (R34). Dans l'afficheur, la surface par la distance `φ` que la bande reconstruit, raccordée
à `η` des colonnes ; dans Godot (C11), en espace écran. Ce qui passe sous la maille devient particule diffuse (C9).

**D3 — Les courants après la campagne.** La hiérarchie d'[ADR-011](ADR-011-courants-et-ecoulements-diriges.md) (C0 à C3) est
celle que l'utilisateur décrit ; C0 à C2 (liste, 2.6) se construisent **après la campagne**, au front 0, sauf si une scène les
demande plus tôt. δ les recevra comme il reçoit la houle, relatif à eux ([ADR-198](ADR-198-la-voie-d-a289.md)).

**D4 — Au loin, le calcul d'avance ; de près, la simulation vivante — à réfléchir.** L'idée de l'utilisateur : un événement à
distance — *« un joueur de 15 m »* — rejoué d'un calcul fait d'avance, et, à mesure que l'observateur s'approche et **peut
interagir**, simulé en temps réel. **Première analyse** (S412) :

- **Où elle s'insère** : un niveau de plus entre le factice et δ vivant, dans l'échelle d'[ADR-202](ADR-202-niveau-de-detail-des-contenants.md)
  D1 (V seul, effets factices, δ) et le rang 4 de l'ordonnanceur (ADR-012 §4) — un niveau **cuit** : une bibliothèque
  d'événements génériques (saut, chute, sillage à une vitesse, explosion) calculés hors ligne par notre propre solveur, rejoués
  comme perturbation relative à B+W (ADR-198), donc posés sur n'importe quelle mer.
- **Le passage au vivant** : δ repart de l'état de l'événement cuit à cet instant — un transfert d'état
  ([ADR-210](ADR-210-changer-de-niveau-par-transfert-d-etat.md)), préparé par la prévision (ADR-013 ; 9.3). **I-17 tient** : une
  bibliothèque cuite est une donnée d'auteur, restaurée comme une graine (`SeedState`, [ADR-022](ADR-022-persistance-de-l-eau.md)
  §3), jamais une capture d'exécution. **I-04 tient** : rien de gameplay n'en sort.
- **Ce qui reste à trancher** : quels événements sont assez génériques ; la mémoire de la bibliothèque ; la continuité au passage
  (l'événement cuit l'a été sur une mer calme) ; la distance ou le critère du passage (la possibilité d'interagir, dit
  l'utilisateur, plutôt que la distance seule).
- **Déclencheur** : quand l'ordonnanceur doit servir plus d'événements que le budget (les scènes de C10, le rang 5), ou quand
  un premier événement récurrent le demande. Point de [file](../registres/QUESTIONS-OUVERTES.md#file-active).

## 3. Ce qui ne change pas

L'ambition (ADR-127), le critère d'arrêt de la campagne (§3.4 de sa conception), I-04, I-17 ; la dynamique en temps réel reste
la voie par défaut — D4 ne vise que ce qu'on ne peut ni voir de près ni toucher.
