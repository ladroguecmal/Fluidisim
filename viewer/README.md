# Afficheur GPU du système d'eau — J1

Application locale séparée du cœur. Mer JONSWAP S201, impact S203/S205, sillage prescrit S212,
caméra interactive. Les résultats GPU servent uniquement à l'image ; les requêtes de jeu restent
dans `water-core`. **J1 reste partiel** : le sillage est exact, mais l'implémentation actuelle de
son chemin d'image dépasse le budget eau ([HOTE-GPU-S212](../docs/validation/HOTE-GPU-S212.md)) ;
l'espace d'optimisation est nommé dans ADR-131 et la feuille de route (J1-bis).

## Lancer depuis la racine du dépôt

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml
```

Sources autorisées et récupérées en S211 dans le cache Cargo local. Le verrou est versionné ;
sur une machine neuve, récupérer les mêmes sources avant la construction hors réseau.
Aucune dépendance externe ajoutée au workspace `code/`.

Windows utilise DirectX 12 : la découverte multibackend a provoqué un arrêt natif sur la
machine S211. Les autres systèmes utilisent la sélection wgpu par défaut, encore non reçue.

## Commandes

| Commande | Effet |
|---|---|
| Clic droit maintenu + souris | Orienter la caméra |
| Flèches | Déplacer la caméra (répétition du clavier) |
| Page précédente / suivante | Monter / descendre |
| Espace | Pause / reprise |
| R | Relancer impact et sillage à leur position de référence |
| B | Afficher / masquer impact et sillage pour comparer au fond |
| Début (Home) | Caméra S201 et instant +3 s |
| Échap | Fermer |

Caméra bornée à ±1000 m horizontalement, hauteur 2–150 m : domaine d'observation de ce banc,
pas une limite du projet. L'impact s'arrête après l'horizon reçu de 56 s, le sillage après son
contexte de 40 s, sans boucle cachée.

## Vérifier et mesurer

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --verify
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --smoke
```

`--verify` compare le shader avec le cœur sur plusieurs âges, l'emprise de l'impact, les coutures
de l'emprise du sillage et un temps long ; mesure la fidélité de la recette du sillage contre une
recette quatre fois plus fine, sa couture et son admission par le cœur ; écrit deux PPM dans
`captures/s212/` ; puis mesure 120 images après 10 de mise en régime, en 640×360 et 960×540,
avec la recette 64×128 puis 128×256.
`--smoke` ouvre une fenêtre et la ferme après 120 images.

Le temps GPU annoncé couvre **la passe d'eau seulement**, hors ciel, transferts, lecture de
mesure et présentation. Le temps CPU annoncé couvre préparation, transfert et soumission ; la
part du sillage (préparation modale et publication) est aussi donnée seule.
La lecture GPU du banc attend le résultat ; ce mode ne mesure pas la cadence interactive.
Si le GPU ne fournit pas d'horodatage, la valeur est annoncée indisponible.

Les buffers de données et le profil sont réutilisés ; les ressources de maillage et profondeur
sont recréées au redimensionnement. L'absence d'allocation de la pile graphique complète
n'est pas reçue : cet afficheur est un hôte de validation, pas le moteur final.

## Données et portée

`Background::render_components` publie amplitude, vecteur d'onde et phase repliée à l'origine
caméra. Le GPU ajoute la phase spatiale relative ; aucun temps absolu f32 ne traverse l'interface.
W reçoit le profil `RadialTable` au pas λ/16 ; Hermite et sa dérivée partagent la même fonction
dans la passe de rendu et le contrôle compute. Les références CPU utilisent le champ radial
direct, sans interpolation. Soleil, ciel, couleurs et brouillard sont un habillage de banc.

Le sillage est une source `Wake` (huit tronçons de 2 s à 3 m/s, 19 620 N, σ 2 m) admise au journal
de pression. Depuis S213, l'image n'est plus préparée à chaque image : `pressure_timeline::Timeline`
replie les tronçons achevés et ne fait qu'une rotation par nœud plus le tronçon en cours
([TEMPS-SILLAGE-S213](../docs/validation/TEMPS-SILLAGE-S213.md)). Il publie `[A, B, kx, ky]`
rebasés à la caméra (4 096 nœuds), sommés par sommet dans l'emprise [−64, −48]–[64, 56] m. La
référence de `--verify` reste la préparation `bound_pressure::Prepared::from_journal`, chemin
indépendant. La recette 64×128 tient 2 % de la recette fine pendant les
16 s de forçage, pas au-delà ; la couture au bord de l'emprise dépasse 3 mm après 24 s.

La grille projetée a un pas nominal de deux pixels et un surbalayage de 18 % ; elle s'arrête à
1500 m. La précision de hauteur aux sondes ne valide pas à elle seule toute cette géométrie,
ni les angles rasants, ni les coûts sur le matériel cible de livraison.
