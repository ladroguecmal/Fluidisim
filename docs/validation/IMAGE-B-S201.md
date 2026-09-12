# S201 — B devient visible

2026-09-13. Direction utilisateur : image de B, puis budget d'image, puis périmètre
δ/V. L'utilisateur a explicitement autorisé une image locale après la question sur
les artefacts. Exemple Rust sans dépendance, CPU uniquement, aucune publication.

## Reproduction

Depuis `code/` :

```text
cargo run -p water-core --release --example render_background ../captures/b-s201.ppm 12 1.5
cargo run -p water-core --release --example render_background ../captures/b-s201-t13.ppm 13 1.5
cargo run -p water-core --release --example render_background ../captures/b-s201-flat.ppm 12 0
cargo test -p water-core --example render_background
```

Arguments : fichier PPM, temps en secondes, Hs en mètres. Temps converti en
microsecondes entières avant B. Limites de ce banc :0..3600 s et0..3 m ; ces bornes
protègent le montage, elles ne définissent pas le domaine de B. Images dans captures/
ignoré par git ; le code et les empreintes sont versionnés. PPM P6 RGB8,640×360,
deux échantillons diagonaux par pixel. Le répertoire de sortie est créé si nécessaire.

La preview PNG affichée dans Codex est une conversion sans retouche des pixels PPM
(Pillow installé sur la machine) ; Python n'est pas une dépendance du banc Rust.

## Ce qui produit l'image

Caméra perspective `(0,-18,7)m`, cible `(0,35,0)m`, verticale+z, champ vertical50°.
Recette JONSWAP V1 : Hs1,5 m, Tp6 s,32 composantes, graine201, direction0,12 tour,
éventail0,25 tour, gamma3,3, bande0,5..4 fois fp, g9,81.
Recette cuite **0x7e5cc32275ccce4e** ; construction par Background::from_spectrum.
Chaque interrogation de rayon appelle **Background::eval** ; hauteur et normale
viennent du champ du projet. Aucun maillage ou animation inventant une autre houle.

Intersection z=η(x,y,t), bornes globales issues des composantes : hauteurΣ|a|,
penteΣ|ak|. Le pas suit0,85·(z−η)/(|dz|+pente·|dxy|), arrêt au résidu vertical<3 mm.
C'est une marche conservatrice pour le champ continu ; conversion monde et arithmétique
du fournisseur sont discrètes, ce n'est pas une preuve d'intersection exacte.
Rayons montants et distances>600 m dessinent le ciel : domaine fini de diagnostic,
pas océan planétaire ni vue sous-marine. Caméra au-dessus de la borne de hauteur.

Éclairage de diagnostic : ciel analytique, reflet solaire, approximation Fresnel,
couleur d'eau et brume de distance, compression tonale puis gamma2,2. Paramètres
visuels choisis pour lire les normales ; **aucune calibration optique**, ombre portée,
réfraction volumique, écume, spray ou audio. La brume masque les détails lointains :
ne pas tirer un seuil perceptuel de cette image sans contrôler ce facteur.

## Réceptions exécutées

Cinq tests debug passent : deux nouveaux (caméra/intersection plane/PPM et onde
sinusoïdale avec contrôle de première rencontre échantillonné), trois du support hôte.
Bibliothèques inchangées ;342 tests/cinq ignorés restent le reçu S200, non rejoué.

| vue | rayons eau | non résolus | évaluations | rendu CPU local (ms) | FNV-1a RGB |
|---|---:|---:|---:|---:|---|
| t12, Hs1,5 | 289491 | 0 | 10021895 | 11727,894 | a52ff81902b150c3 |
| t13, Hs1,5 | 289482 | 0 | 10145445 | 11925,566 | 1df02ffb7c202b32 |
| plan Hs0 | 289225 | 0 | 289225 | 77,590 | dae2f2514cad0324 |

Total460800 rayons. Plan Hs0 est un témoin analytique retournant η0/normale verticale,
il ne mesure pas le coût de B ; le programme cuit une recette positive de réserve
mais ne l'évalue pas dans ce mode. Les deux instants et le plan ont des pixels
distincts (contrôle par différence), inspection visuelle des trois previews effectuée.
Résidu maximal des intersections eau<0,003 m. Rayons non résolus seraient magenta
et feraient échouer la commande ; ils ne sont jamais cachés comme ciel.

Premier jet limité256 itérations :997 et1021 refus aux deux instants, vus en magenta.
Plafond4096 : zéro refus, même tolérance3 mm. Coût t12≈11,5→11,7 s sur ces passages ;
mesures uniques locales, sans échauffement statistique ni budget temps réel reçu.
Premier export échoué faute de répertoire ; création de celui-ci ajoutée avant rendu.

## Portée et suite

**B a une image et une caméra de banc.** La distance observateur existe ; aucun LOD
spatial, bloc ou ordonnanceur n'est implémenté. L'image permet une discussion humaine,
elle ne fixe pas elle-même une tolérance perceptuelle et ne ferme pas B4/A50.
Les phrases historiques « critère manquant » sont périmées pour le seuil numérique :
**2 % acquis depuis ADR-120**, aucune redemande. Justesse physique, ADR-123 et A98
restent des réceptions distinctes. Les effets secondaires demandent encore leur code.

Suite choisie par l'utilisateur : **budget d'image et cost_per_block_ms**, sur une
cible et une charge nommées ; les≈12 s de cette référence CPU ne sont pas une promesse
de production. Puis cadrage détaillé de δ en effets bornés et V déclenchée par besoin
gameplay. S200-1 et S199-2 passent derrière cette direction, sans être déclarés résolus.
