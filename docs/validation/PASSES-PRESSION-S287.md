# S287 — ordre mémoire des passes de pression

## Hypothèse et critère

Après [S286](CADENCE-DELTA-S286.md), accélérer chaque pas plutôt que cacher son coût par des
images sans calcul. Les champs sont rangés `k*nx+i`, mais les boucles de la multigrille et
`apply_mobile` parcourent i puis k. Essai : inverser les boucles **indépendantes**, sans modifier
l'ordre des quatre contributions de chaque cellule ni les réductions du gradient conjugué.

Critères déclarés : empreintes identiques de toutes les images, mêmes itérations, zéro
allocation et gain du **chemin complet**. Retirer le changement si le gain n'est pas reçu.
L'essai porte sur l'opérateur fin mobile et sur les huit boucles des opérateurs grossiers,
diagonales, restrictions, prolongations et construction des géométries. Aucune nouvelle donnée,
aucun cache, aucun changement de solveur, de seuil ou d'arrêt budgétaire.

## Instrument conservé

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta-cadence
```

Le banc S286 imprime désormais `PASSES_S287` : FNV-1a des bits de la hauteur perturbative
publiée et des vitesses MAC u/w, **à chaque image**, ordre logique constant. La pression et
la compensation de hauteur ne sont pas incluses dans cette empreinte. Le total d'itérations
est publié séparément. Calcul des empreintes hors chronométrage, même instrumentation pour
tous les essais. Le champ δ n'est ni sauvegardé ni exporté ; seule une empreinte est imprimée.

Six trajectoires : houle S275 initialement sans perturbation et onde gaussienne 0,6 m, aux pas
16/32/48 ms. 192 images, durée 3,072 s, 6 656 mailles. Les cadences lentes restent des témoins
refusés pour l'afficheur (S286). CPU Ryzen AI 7 350, Windows release, secteur (statut 2, 98 %).
Ordre de la campagne : référence A1, variante B1, variante B2, retour à la référence A2.
Pas de tests ou compilation concurrente aux mesures.

## Identité

| scénario | pas | empreinte de trajectoire | somme des itérations |
|---|---:|---|---:|
| houle | 16 ms | `92d65e868ec29942` | 2648 |
| houle | 32 ms | `aee9db45c45129a9` | 1562 |
| houle | 48 ms | `c9c79e0075ee0ffc` | 1116 |
| onde | 16 ms | `1e5c6d5f5fed86bd` | 2895 |
| onde | 32 ms | `b0414fc315f5f3c6` | 1551 |
| onde | 48 ms | `6c037edac4f55f05` | 1105 |

A1/B1/B2/A2 : mêmes empreintes, mêmes itérations, zéro allocation et mêmes erreurs S286.
Cette identité mesurée complète la lecture : l'ordre intercellules change, aucune opération
locale ni réduction ne change. Elle n'est pas une nouvelle réception physique ou visuelle.

## Coût et décision

| scénario | pas | médiane A1 | médiane B1 | médiane B2 | médiane A2 |
|---|---:|---:|---:|---:|---:|
| houle | 16 ms | 23,3345 ms | 22,7746 ms | — | 23,2573 ms |
| houle | 32 ms | 25,6885 ms | 24,8079 ms | — | 25,4413 ms |
| houle | 48 ms | 26,7559 ms | 25,8874 ms | 26,1603 ms | 27,1293 ms |
| onde | 16 ms | 24,8565 ms | 24,6408 ms | 24,6883 ms | 25,1848 ms |
| onde | 32 ms | 25,7251 ms | 25,1215 ms | 25,8417 ms | 25,7479 ms |
| onde | 48 ms | 26,6230 ms | 26,0793 ms | 27,0496 ms | 27,1061 ms |

Deux sorties de B2 ont été tronquées par l'outil de lecture ; elles ne sont pas reconstituées.
Sur l'onde à 16 ms, moyenne par image A1/B1/B2 : 24,8542 / 24,8082 / 25,0584 ms.
Maximum A1/B1/B2 : 33,1619 / 34,9117 / 34,6653 ms. Le premier petit gain de médiane
ne suffit donc pas à établir un gain utile sur le pas complet.

Au retour A2, moyenne onde 16 ms : 25,1081 ms ; maximum 34,5219 ms. Les plages de moyennes
A et B se recouvrent. **Variante retirée** conformément au plan : pas de gain robuste reçu
sur le coût complet représentatif, malgré un petit gain de médiane. Ce n'est pas la preuve
qu'un parcours contigu est inutile sur une plus grande grille ; c'est l'arrêt de ce levier
sur cette charge. Le cœur retrouve exactement sa révision initiale ; seul l'instrument de
comparaison reste. Aucun nouveau test de physique requis pour un changement non conservé.
Suite viewer : 36 réussis, un ignoré, aucun échec. Reçu cœur S284 conservé, non rejoué
après restauration exacte ; zéro modification finale dans `code/`.

## Confrontation au GPU

Matériel constaté : RTX 5070 Laptop et GPU AMD intégrés. Le code de `viewer/src/gpu.rs`
contient déjà la création d'un appareil wgpu, des passes compute et des mesures temporelles
GPU optionnelles. **Aucun solveur δ GPU n'est présent**. Les mesures ci-dessus restent CPU.

La pression alterne opérateurs, réductions et cycles multigrilles plusieurs fois par pas.
Déplacer seulement l'opérateur, en rapatriant son résultat après chaque appel, introduirait
des échanges et synchronisations à chacune de ces dépendances. C'est un constat de structure,
pas une mesure de leur coût sur cette carte. Il ne prouve aucun gain du GPU.

Suite concrète : décider un **candidat de pression résidente sur GPU dans l'hôte**, conserver
le CPU comme référence, puis construire opérateur et lissage avec ressources préallouées,
mesurer calcul + transferts + synchronisations et comparer les résultats aux mêmes géométries.
Réductions, cycle complet, refus numériques et publication restent des portes avant activation
dans le vrai pas. Le cœur sans dépendance et l'absence d'autorité gameplay de δ restent acquis.
L'architecture GPU n'est pas actée par cette étude, et aucun budget GPU n'est encore reçu.
