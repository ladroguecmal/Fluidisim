# S288 — premier opérateur et lissage de pression sur GPU

Application d'[ADR-172](../adr/ADR-172-candidat-pression-residente-gpu.md). Le candidat fonctionne
dans le banc de l'hôte ; le solveur de l'afficheur reste CPU. Aucun nouveau téléchargement.

## Ce qui est construit

`Volume::write_mobile_pressure_rows` écrit sans allocation les ouvertures et facteurs de
fantômes de l'opérateur mobile natif, ordre gauche/droite/bas/haut. La géométrie est figée,
le facteur 1/dx² reste explicite. Zéros pour l'air et le solide. Les valeurs inhomogènes des
fantômes ne sont pas incluses : elles appartiennent au second membre, encore à intégrer.
Export refusé sans écriture si forme ou géométrie incorrectes.

`viewer/src/pressure_gpu.rs` réserve coefficients, second membre, deux champs de pression,
lecture et horodatages ; `pressure.wgsl` applique l'opérateur ou Jacobi amorti 4/5. Les champs
sont alternés entre dispatchs : **aucun retour CPU entre les lissages**. Le cas impair 31 et
les cas pairs 2/32 reçoivent le bon tampon final. Chaque essai réinitialise la pression.

## Critères et protocole

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --pression-gpu
cargo test --workspace --release --offline --manifest-path code/Cargo.toml
cargo test --release --offline --locked --manifest-path viewer/Cargo.toml
```

Critères écrits au plan : export au bit contre application native ; différence GPU/CPU
maximale normalisée par max|CPU| ≤ 10⁻⁵, repos nul exact. Il s'agit d'une tolérance de port
f32 pour détecter une erreur de stencil, **pas d'une tolérance physique ni de hauteur**.
Ni le seuil 3 mm S201 ni les portes ADR-143/144 ne sont remplacés.

Test cœur : grilles 16×12 et 31×19, dx 0,5/2 m, fonds plans/coupés, surface ondulée et
géométrie couplée distincte de η ; opérateur reconstruit identique au natif, refus atomiques.
Banc GPU : 31×19 à dx 0,5, 128×52 à dx 2, 256×128 à dx 0,5 ; géométrie plane puis fond
coupé et surface ondulée. Chaque cas exécute l'opérateur seul (lissages=0), puis 1/2/31/32
lissages, avec champs initiaux et seconds membres non uniformes. **30 cas**, dix répétitions
chacun, premier appel séparé et neuf mesures en régime. Repos après données non nulles sur
les six géométries. La grille impaire éprouve les invocations GPU hors tableau.

CPU témoin : application scalaire des lignes exportées et même lissage, sans GPU. L'export
est lui-même confronté à l'opérateur natif ; ce témoin n'est pas le pas CPU complet de S286.
Le banc compare des noyaux équivalents, pas la convergence de deux solveurs.

## Résultats, 2026-09-19

Windows release, Ryzen AI 7 350, **RTX 5070 Laptop / DX12**, secteur constaté avant/après
(BatteryStatus=2, 98 %). Aucune autre campagne pendant la mesure. Horodatage GPU disponible.
Les 30 cas passent ; erreur normalisée maximale **1,686·10⁻⁷**. Repos exact. Export et
empaquetage **zéro allocation**, contrôlés pendant chaque appel mesuré.

Coût de **32 lissages** (ms). « Complet » inclut réexport des coefficients, empaquetage de
toutes les entrées, envoi, encodage, exécution, lecture, attentes, décodage et lecture des
horodatages. Initialisation appareil/pipelines/tampons et construction de la géométrie exclues.
« GPU » contient uniquement les dispatchs ; ces mesures ne sont pas des cadences d'image.

| grille / géométrie | CPU médian | préparation médiane | GPU médian | complet médian | complet max observé |
|---|---:|---:|---:|---:|---:|
| 31×19 plane | 0,2007 | 0,0224 | 0,0675 | 0,7025 | 0,8134 |
| 31×19 coupée/ondulée | 0,2038 | 0,0228 | 0,0759 | 0,6472 | 0,9214 |
| 128×52 plane | 2,8444 | 0,2224 | 0,0900 | **1,1020** | 1,9753 |
| 128×52 coupée/ondulée | 2,5213 | 0,2243 | 0,0958 | **1,2410** | 1,3985 |
| 256×128 plane | 13,5170 | 1,1733 | 0,1144 | **2,4008** | 4,2209 |
| 256×128 coupée/ondulée | 12,2376 | 1,1617 | 0,1108 | **2,4446** | 3,4547 |

À 128×52, ce lot de 32 lissages gagne ×2,0–2,6 en coût complet contre le témoin scalaire ;
à 256×128, ×5,0–5,6. La petite grille perd : le travail n'amortit pas les échanges. L'opérateur
isolé à 128×52 plane coûte 0,7301 ms complet contre 0,0689 ms CPU, malgré 0,00237 ms GPU.
Cela confirme l'intérêt de garder les itérations sur la carte, pas de déporter chaque appel.

Un passage préliminaire excluait l'export/empaquetage de la zone chronométrée (environ 0,8 ms
à 128×52 pour 32 lissages) ; il n'est **pas** utilisé comme coût complet ici. L'élargissement
du protocole conserve les mêmes écarts numériques. La mesure finale reçoit ce périmètre précis.

**Allocations non nulles de la pile et du banc** : maximum 65 à 105 par appel selon le nombre
de dispatchs, 105 pour 32 lissages. Encodeurs et canaux de lecture créés par appel, plus wgpu ;
pas d'attribution par origine faite ici, pas de réception I-06 du chemin image. Les réserves
GPU et les vecteurs de données sont réutilisés. Le premier appel monte à **5,518 ms** ;
maximum en régime de toute la campagne **4,731 ms**. Aucun plafond matériel ni budget 2 ms reçu.

## Limites et prochaine construction

Pas de solveur de pression complet : ni réductions, ni cycle multigrille, ni critère de
convergence, ni certificat ADR-143/144, ni correction des vitesses. Pas de trajectoire de
surface GPU, de 3D, de seconde cible, de publication au rendu ou de gain interactif.
L'export est un instantané de géométrie ; le futur couplage doit le renouveler pendant la
projection au bon état de surface totale. Une liste de coefficients correcte après pose de
surface ne prouve pas l'intégration de cette étape dans un pas couplé.

Suite : assembler un solveur de pression résident (réductions et cycle), comparer sa solution
et ses refus au cœur, puis l'intégrer au pas réel avec coût complet et publication atomique.
Prioritaire devant un nouveau raffinement local : la 3D et les solides restent absents et
obligatoires, mais multiplier les mailles du chemin actuel aggrave A276. Le coût GPU de ces
noyaux ne doit pas être extrapolé au solveur ni à toute l'eau.

Suites release finales : **508 tests cœur/harnais réussis** (394 + 16 + 2 + 1 + 95),
**36 viewer réussis**, 19 ignorés au total, aucun échec. Le banc GPU est lancé explicitement ;
il n'est pas implicitement exécuté par `cargo test`. Les avertissements antérieurs subsistent.
