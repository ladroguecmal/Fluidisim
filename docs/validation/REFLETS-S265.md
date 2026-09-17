# Reflets des petites ondes — S265

Contrat : [ADR-161](../adr/ADR-161-reflets-de-la-queue-non-resolue.md).

## Protocole avant construction

- Conservation des seconds moments linéaires : variance résolue + transférée = variance
  initiale ; oracle par intégration de phases d'une onde oblique, covariance positive,
  limites `h=0` et `h` sous-résolu. Gauss–Hermite : moyenne nulle, covariance identité,
  moment d'ordre quatre égal à trois, poids normalisés (ordres 3 et 5).
- GPU contre référence CPU f64 : pente filtrée à 5e-4, covariance à 1e-5 absolu,
  aux vents 3/5/8,37, empreintes 0/0,02/0,1/0,5 m ; aucun repli sous le domaine sondé.
  Les valeurs et déterminants doivent rester finis ; hauteur CWM historique reçue séparément.
- Témoin sans option : les sept PPM à 5 m/s identiques aux images S263 ; revue dans un
  nouveau dossier `viewer/captures/s265`, sans écraser R6.
- R7 : mêmes poses à 5 m/s, comparaison ordre 3/5 à la pose de référence et rasante.
  Publier écart des pixels (ce n'est pas une tolérance perceptive), examen visuel et coût
  eau avec/sans option sur secteur, mêmes scènes. L'écart 3/5 ne doit pas être caché.
- Exécuter les tests de l'hôte et les réceptions GPU touchées. Pas de répétition du harnais
  cœur complet si ses sources restent inchangées.
- Arrêt : variante intégrée, preuves numériques et images fournies ; verdict utilisateur
  distinct. Si 3×3 expose des motifs de quadrature, conserver le témoin et qualifier la limite.

## Résultats

Code construit au commit `61c917f`. Windows, RTX 5070 Laptop GPU, DX12. Tests hôte release :
**18 réussis, 1 ignoré, 0 échec**. Cœur inchangé, suite complète cœur non rejouée.

### Numérique

5 000 sondes par vent, âges 3 et 12 s, quatre empreintes du protocole :

| vent | erreur pente | erreur covariance | déterminant minimal |
|---|---:|---:|---:|
| 3 m/s | 2,400e-5 | 8,538e-8 | 0,513524 |
| 5 m/s | 5,307e-5 | 2,154e-7 | 0,530496 |
| 8,37 m/s | 9,319e-5 | 7,427e-7 | 0,325611 |

Critères tenus ; valeurs finies, covariance positive, aucun repli sur ces sondes.
Moments Gauss–Hermite GPU 3/5 : poids, moyenne, variance et moment d'ordre quatre reçus à 2e-6.
Tests CPU : variance contre intégration de 4 096 phases obliques, covariance transportée contre
échantillons transformés. Ces essais reçoivent la fermeture déclarée, pas sa justesse physique générale.

`--cwm-query-verify` sous option à 5 m/s : **0,288 mm**, 5 592 sondes, aucun refus, au plus trois
itérations. La géométrie n'a pas changé. Les **sept images témoins sans option sont identiques au bit
aux sept PPM R6 à 5 m/s** (comparaison intégrale des fichiers, pas seulement des empreintes).

### Image et limites de quadrature

R7 : 21 images, sept poses × historique/3×3/5×5, dans `viewer/captures/s265/`.
Même géométrie, même vent de 5 m/s (témoin, pas choix utilisateur), même ciel de banc.
La queue seule est concernée par cette fermeture ; ni variance perdue de la bande géométrique,
ni impacts, ni sillage ne reçoivent ce traitement.

Diagnostic d'image dans le rectangle fixe `x=[80,1200), y=[360,680)` des images 1280×720 :
conversion sRGB vers linéaire, luminance Rec.709, RMS du laplacien à quatre voisins intérieurs.
Ce diagnostic mesure les contrastes fins, **pas une rugosité physique ou une acceptation visuelle**.

| pose | RMS historique | RMS 3×3 | RMS 5×5 | rapport 3×3 / historique |
|---|---:|---:|---:|---:|
| référence | 0,11071 | 0,04364 | 0,03870 | 0,394 |
| rasante | 0,18039 | 0,06113 | 0,05285 | 0,339 |
| haute | 0,13940 | 0,02728 | 0,01970 | 0,196 |

La quadrature **n'est pas convergée partout** : écart absolu des canaux sRGB 3×3/5×5 dans ce
rectangle, en niveaux sur 255 : moyenne / percentile 99 / maximum = **1,58 / 9 / 34** en référence,
**1,75 / 9 / 24** en rasante, **2,54 / 25 / 48** en haute. Les reflets fins des nuages et du soleil
exposent la faible quadrature ; ni 5×5 ni 3×3 ne sont une référence radiométrique indépendante.
L'inspection trouve moins de grain sur les deux vues proches, sans réception perceptive implicite.

### Coût

Secteur, batterie 99 % au début et à la fin. 1280×720, 120 images après dix de chauffe, âge initial
12 s, trois sillages (4 096 modes), impacts, ciel clair, CWM, M=2, vent 5 m/s, filtre spectral,
visibilité et grille locale actifs. CPU à un fil ; LOD temporel et éclairage précalculé absents.
Mesure GPU eau : cuisson de grille incluse, ciel/transfert/lecture/présentation exclus.

| pose | historique | 3×3 | 5×5 |
|---|---:|---:|---:|
| référence, médiane ms | 1,804 | 2,711 | 3,288 |
| rasante, médiane ms | 1,834 | 2,493 | 3,088 |
| référence, p95 ms | 1,862 | 2,853 | 3,392 |
| rasante, p95 ms | 1,863 | 2,516 | 3,127 |

Cuisson médiane 1,06–1,11 ms ; maximum eau isolé 5,02 ms à l'ordre 3 non attribué.
CPU préparation/transfert/soumission médian 4,17–4,39 ms, dont sillage 3,13–3,21 ms.
`allocations_update_max=0` aux six passages ; pas de nouvelle mesure globale de la pile.
**Budget 2 ms non tenu par cette variante**. Elle reste optionnelle ; optimiser une intégration
analytique ou précalculée est une suite conditionnée par le verdict, pas un reçu de ce lot.

### Reproduction et revue

Depuis `viewer/`, lancer `target/release/water-viewer.exe` avec les arguments communs
`--multi --vagues --modulation --ciel-clair --vent=5`, puis :

- témoin : `--revue=r7_avant` ;
- variante : `--reflets-filtres --revue=r7_filtre3` ;
- convergence : `--reflets-filtres --reflets-ordre=5 --revue=r7_filtre5` ;
- mesure : `--reflets-bench` ;
- oracle : `--reflets-filtres --reflets-verify` (répéter aux vents 3 et 8,37).

Empreintes des images de référence envoyées : historique `0x0f178a2e5a4fcb7d`,
3×3 `0xec4a559bf5ca0287`. Rasante 3×3 : `0xe68252f14891aca9`.
Contrôle 5×5 référence : `0x0a6f23b3fdddfa21`, rasante : `0x8f80fdc46ae57dbf`.
Métadonnées complètes dans les trois journaux locaux `avant.log`, `filtre3.log`, `filtre5.log`.
La pose à 29 s reste hors de l'horizon honnête du sillage (limite historique 18,53 s), annoncée.

**Réception :** capacité de filtrage des reflets intégrée et numériquement contrôlée dans son
approximation. R7 soumise à l'utilisateur ; ni acceptation visuelle, ni BRDF complète, ni budget
reçus. Aucun vent représentatif adopté.
