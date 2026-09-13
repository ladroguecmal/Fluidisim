# Afficheur GPU du système d'eau — J1

Application locale séparée du cœur. Mer JONSWAP S201, impact S203/S205, caméra interactive.
Les résultats GPU servent uniquement à l'image ; les requêtes de jeu restent dans `water-core`.
Le sillage n'est pas encore affiché : **J1 reste partiel**.

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
| R | Relancer l'impact à sa position de référence |
| B | Afficher / masquer l'impact pour comparer au fond |
| Début (Home) | Caméra S201 et instant +3 s |
| Échap | Fermer |

Caméra bornée à ±1000 m horizontalement, hauteur 2–150 m : domaine d'observation de ce banc,
pas une limite du projet. L'impact s'arrête après l'horizon reçu de 56 s, sans boucle cachée.

## Vérifier et mesurer

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --verify
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --smoke
```

`--verify` compare le shader avec le cœur sur plusieurs âges, l'emprise et un temps long ;
écrit deux PPM dans `captures/s211/` puis mesure, en 640×360 et 960×540, 120 images après
10 images de mise en régime.
`--smoke` ouvre une fenêtre et la ferme après 120 images.

Le temps GPU annoncé couvre **la passe d'eau seulement**, hors ciel, transferts, lecture de
mesure et présentation. Le temps CPU annoncé couvre préparation, transfert et soumission.
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

La grille projetée a un pas nominal de deux pixels et un surbalayage de 18 % ; elle s'arrête à
1500 m. La précision de hauteur aux sondes ne valide pas à elle seule toute cette géométrie,
ni les angles rasants, ni les coûts sur le matériel cible de livraison.
