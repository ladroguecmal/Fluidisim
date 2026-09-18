# S283 — premier rétrécissement perturbatif consommé

## Contrat et réception

`Volume::shrink_perturbation_from` transfère un état **perturbatif**, à fond plat, vers un volume
plus étroit **préalloué**. Même maille, profondeur, milieu et origine verticale ; l'hôte translate
l'origine horizontale d'un nombre entier de colonnes. Pas de changement de résolution.

L'intérieur conserve hauteur, vitesses et compensation de hauteur ; les champs perturbatifs
dans la bande fournie sont multipliés par `s²(3−2s)`, `s = min(distance au bord / largeur, 1)`.
Cette interpolation cubique vaut 0/1 avec dérivée nulle aux deux extrémités. Les vitesses normales
aux nouveaux bords sont nulles. La pression de démarrage est invalidée : le prochain pas la
résout sous ses nouvelles conditions aux limites. Perte d'énergie assumée, sans transduction
(ADR-012 §4). Ce n'est ni une fusion ni un domaine épars d'ADR-006.

**Consommateur :** `Live` puis `Layer::update` dans l'afficheur ; largeur 128→64 colonnes,
256→128 m, dx=2 m, nz=52. Horloge et coordonnées mondiales du fond conservées. Les tampons de
publication et de fond gardent leur capacité ; seule leur partie utile est calculée. Le shader
lit déjà l'origine et le nombre de colonnes publiés. Le score de perception voit la nouvelle
emprise. Une réserve de volume étroit est allouée au démarrage de chaque `Live` ; après permutation,
l'ancien volume reste réservé. **Gain de calcul, pas restitution de mémoire.**

**Commande N :** réduction manuelle, une fois, avec garde avant publication. Ni focalité ni
dégradation automatique sous budget reçues. La nouvelle mesure de coût ne reprend pas l'historique
du domaine large ; l'a priori reste le nominal large, sans inventer un coût petit calibré.

## Le premier essai refuse la continuité

Protocole déclaré avant mesure : onde gaussienne de 0,6 m de S277, houle S275, transfert après
64 pas de 16 ms, comparaison au domaine large au même instant et sur 128 pas ultérieurs.
La tolérance de hauteur est **3 mm**, celle de l'image S201, pas un seuil choisi après résultat.

Le transfert brut change la hauteur publiée de **33,447 mm** au pire des centres de colonnes.
Au centre `|x| < 16 m`, l'écart maximal au témoin large atteint **21,255 mm** pendant les 2,048 s
suivantes. Le gain de coût ne reçoit donc pas une réduction visuellement gratuite.

**Correction intégrée :** préparer dans la réserve, borner le saut du profil publié, puis
permuter seulement si la borne vaut au plus 3 mm. Sinon, domaine, horloge, image et historique
de coût restent inchangés. Le cas à 1,024 s est refusé ; le cas à 0,016 s est admis par le test.

Le garde évalue les deux interpolants Hermite avec leurs fondus, par pas dx/256. Ce n'est pas
un simple maximum échantillonné : à celui-ci est ajoutée la demi-distance d'échantillonnage,
majorée de l'arrondi des positions f32, multipliée par la somme des bornes de dérivée. Les
contrôles de Bézier de chaque cubique bornent sa hauteur ; leurs différences bornent sa dérivée.
Le fondu en cosinus ajoute `max|h|·π/(2·largeur)`. Le fondu transversal est commun et ≤1.
À 1,024 s : **borne 33,729 mm**, refus. dx/64 donnait une borne trop lâche dans le cas précoce ;
le quadrillage du garde a été affiné, **pas la tolérance**.

Portée : borne du **profil CPU** à l'instant de la commande, pas preuve sur pixels GPU,
normales/reflets ou évolution ultérieure. I-12 perceptif reste ouvert (A290).

## Coût du chemin consommé

Commande depuis la racine :

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta-retrecissement
```

Le banc **force seulement dans son diagnostic** le transfert normalement refusé, et le dit en
sortie. Deux domaines réels, mêmes instants/fond/onde ; coûts d'échantillonnage + pas couplé,
128 pas après réduction, exécution séquentielle, GPU absent. Secteur avant/après (BatteryStatus=2,
98 %), Windows, AMD Ryzen AI 7 350 (8 cœurs / 16 processeurs logiques), compilation release.
Techniques présentes : grille du fond, multigrille mobile,
pression précédente (invalidée au transfert) ; absentes : GPU δ, cadence découplée, 3D,
pool épars, transition temporelle, restauration.

| domaine | cellules calculées | médiane du pas | p99 observé |
|---|---:|---:|---:|
| large | 6 656 | 25,5464 ms | 32,9817 ms |
| étroit | 3 328 | 12,3784 ms | 15,8659 ms |

**×2,064**, mais toujours hors 2 ms. Transfert et garde : **3,0283 ms**, hors coûts de pas
ci-dessus. Premier passage avant garde : transfert seul 0,0276 ms ; ×2,109 sur les pas.
Le coût du contrôle domine donc celui du transfert. 128 observations ne reçoivent pas B7.

## Vérifications et limites

- Cœur : transfert puis vrai pas couplé, zéro allocation au compteur global ; intérieur au bit,
  source intacte, pression invalidée, bords normaux nuls, refus de fenêtre/bande atomiques.
- Afficheur : même horloge, centre au bit à la réduction, fond échantillonné à −64 m plutôt
  qu'à l'ancien bord −128 m, en-tête réduit, zéro hors emprise, reprise après saut temporel.
- Garde : le cas destructif est refusé sans publication ; un profil à valeurs nodales nulles
  et tangentes non nulles vérifie que le relief entre nœuds est également couvert.

Suites release : **506 tests réussis** dans `code/` (393 cœur, 15 + 2 + 1 intégrations,
95 harnais ; 18 ignorés), **33 réussis** dans `viewer/` (1 ignoré), aucun échec.

Non reçus : généralisation hors fond plat, agrandissement, déplacement, plusieurs domaines,
choix non focal, rétrécissement automatique, budget mural I-05, coût global d'une commande
refusée (diagnostic texte), continuité des pentes et de l'évolution. Le retour temporel reprend
la condition initiale dans la fenêtre étroite, il ne restaure pas l'ancien domaine large.

**Prochain lot utile :** préparer la réduction progressivement dans le domaine vivant,
avec contrôle de la perte et de la suite temporelle avant permutation ; puis l'intégrer au
choix non focal sous budget. Le refus de S283 protège l'image instantanée, mais laisse la famine.
