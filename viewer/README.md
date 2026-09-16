# Afficheur GPU du système d'eau — J1

Application locale séparée du cœur. Mer JONSWAP S201, impact S203/S205, sillage prescrit S212,
caméra interactive. Les résultats GPU servent uniquement à l'image ; les requêtes de jeu restent
dans `water-core`. **J1 reste partiel** : la préparation CPU dépasse encore le budget global.
Le rendu filtre désormais B et le sillage selon le pas projeté (S249, ADR-148) ; la
[feuille de route](../docs/FEUILLE-DE-ROUTE.md) porte les capacités et limites courantes.

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
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --lod-charge
cargo test --release --offline --locked --manifest-path viewer/Cargo.toml
```

`--no-lod` (avec la fenêtre ou `--cadence`) rend le chemin direct du sillage S212–S225, conservé
comme témoin. `--lod-charge` publie les bornes de courbure des trois couches et la charge de
maillage qu'elles exigent par pose (S234), sans GPU.

**Filtrage lointain (S249).** Activé par défaut pour B et le sillage. `--no-spectral`
retrouve le champ complet ; `--multi --spectral-verify` reçoit le champ filtré sur trois
poses et deux formats, avec écart volontaire au champ complet publié séparément.
`--multi --spectral-bench` mesure avec/sans filtre aux mêmes âges et deux poses.
`--verify` conserve explicitement le champ complet et ses anciens critères.
`--rasant --multi --cadence` mesure le rendu à incidence rasante ; ajouter `--no-spectral`
pour son témoin. Les impacts restent non filtrés. Les pentes de réflexion sont les
pentes modales filtrées, sans dérivée du filtre de caméra : voir
[ADR-148](../docs/adr/ADR-148-filtrage-spectral-image.md).

**Scène multi-sources (S235).** `--multi` remplace le sillage et l'impact uniques par la scène
déclarée dans `scene.rs` : trois sillages d'un journal commun et huit impacts nés toutes les 4 s.

| option | effet |
|---|---|
| `--multi --verify` | contrôles `VERIFY` sur 23 âges et deux chemins, intérieurs de grille, capture `captures/s235/`, bancs 3 s et 29 s |
| `--scene-admission` | plancher de pente du cœur tous les 0,25 s, scène et variante dense, sans GPU |
| `--multi --admission-reelle` | pente réelle des perturbations aux instants refusés, contre le plancher |
| `--multi --retour` | caméra détournée puis revenue : comparaison au bit avec un passage continu |
| `--start=N` | ouvre la fenêtre à N secondes de scène |
| `--away` | caméra hors champ (0, −300, 12), dos à la scène |
| `--no-cull` | fenêtre sans visibilité (toutes les sources préparées à chaque image) |

La fenêtre ne prépare que ce que la grille voit : emprise de la grille projetée sur l'eau, contour
exact aux sommets de bord, marge par arête. Les vérifications gardent la visibilité coupée, leurs
sondes étant hors de l'emprise de la caméra.

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
sont recréées au redimensionnement. La boucle du projet n'alloue rien en régime (ADR-145) ;
les allocations de la pile verrouillée sont comptées séparément par `--cadence`.

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
indépendant.

**Domaine d'image du sillage (S214, [ADR-132](../docs/adr/ADR-132-domaine-d-image-d-un-sillage.md)).**
Il se calcule depuis la recette : `rayon = 2π·angular/(3·cutoff)` et `durée = 4π/√(g·cutoff/radial)`,
soit **89,36 m** et **18,53 s** pour la recette 64×128 à cutoff 3 — contre un coin d'emprise à
102,22 m et un contexte déclaré de 40 s. `--verify` publie `WAKE_LOI` avec la fixture, et l'hôte
imprime `WAKE_HORS_DOMAINE` une fois au premier instant qui dépasse. **Il n'y a pas de refus** :
le chemin est cosmétique. Mesures : 2 % de la recette fine franchi entre 16 et 18 s, couture au
bord de 3 mm entre 18 et 20 s, rayon d'accord à 10 % effondré entre 24 et 30 s.

**Composition par le cœur (S214).** `--verify` publie aussi des lignes `MIXED` : la scène composée
par `mixed_water` — B, impact et sillage ensemble — son budget de pente conjoint et ses refus.
La référence CPU évalue désormais les trois couches **au même point local**, tiré d'une seule
conversion monde → local ; auparavant B était pris au point du réseau monde et les perturbations au
point `f32` brut, ce qui écartait la référence de la composition du cœur de 18 µm (A253). Le cœur
ne compose que sur l'**intersection** des domaines (4 477 sondes sur 6 988) : c'est pourquoi il ne
peut pas servir de chemin de rendu. Voir
[COMPOSITION-J1-S214](../docs/validation/COMPOSITION-J1-S214.md).

**Grille locale du sillage (S234, LOD spatial de couche).** Par défaut, le sillage n'est plus
sommé à chaque sommet : une passe compute évalue `(η, ηx, ηy, ηxy)` sur une grille de l'emprise
dont le pas vient de sa borne Hermite bicubique `h⁴/384·(2Σ|a|k⁴ + h/4·Σ|a|k⁵)` ≤ 3 mm
(tolérance S201), arrondi au 1/16 m inférieur ; les sommets reconstruisent par Hermite bicubique
(`lod::hermite` en est la référence CPU testée). Capacité 16 384 nœuds ; `LOD_CAPACITE` annonce un
pas imposé par la capacité. La cuisson est chronométrée et incluse dans `GPU_water_ms`. Voir
[LOD-SILLAGE-S234](../docs/validation/LOD-SILLAGE-S234.md).

La grille projetée a un pas nominal de deux pixels et un surbalayage de 18 % ; elle s'arrête à
1500 m. La précision de hauteur aux sondes ne valide pas à elle seule toute cette géométrie,
ni les angles rasants, ni les coûts sur le matériel cible de livraison.
