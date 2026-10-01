# APIC sur la carte — C7 — S416

2026-09-30. **C7** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
[conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5) : APIC **sur la carte**, dans sa **version étroite**
([ADR-211](../adr/ADR-211-les-trucages-retenus.md) D1, [ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) D5). Au poste,
RTX 5070 Laptop. La référence est le cœur (`apic3d.rs`, `apic3d_columns.rs`) ; la carte la reproduit (ADR-175 D1).

## Reproduire

- Commit `b9c35cf5` ou plus récent ; afficheur construit (`cargo build --release --offline` dans `viewer/`).
- Les étages : `ITERATIONS=150 viewer/target/release/water-viewer.exe --apic3d-carte-etages` — lignes `APIC_CARTE_S416`, 5 s
  (`DX=`, `CHAUFFE=` : 0,05 et 20 par défaut) : §3.
- Le ballottement : `DUREE=10 ITERATIONS=200 viewer/target/release/water-viewer.exe --apic3d-carte-ballottement` — lignes
  `APIC_CARTE_BALLOTTEMENT_S416`, une minute à 5 cm (`DX=0.025 DUREE=2 ITERATIONS=400` : §4.2) : §4.
- Le cœur : `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib _s416` (`step_upto`).

## En une phrase

Le pas d'APIC 3D **nu** tourne sur la carte — tri par maille, transferts en collecte, surface reconstruite, gradient conjugué,
extrapolation, advection, séparation — et reproduit la référence **étage par étage à l'arrondi** ; sur le ballottement (1, 0) de
S388, 10 s, sa surface reste à **0,46 mm** de la référence et sa période à **0,003 %** ; il coûte **2,05 ms au 99ᵉ centile** pour
12 800 particules (160 ns par particule), 4,77 ms pour 102 400 à 2,5 cm (**47 ns**), la projection d'abord — dont le coût est
celui des dispatchs enregistrés, non des calculs.

## 1. La conception de C7

### 1.1 Ce qu'il y a à porter

La bande étroite de la référence est `Apic3` **avec** sa zone de colonnes et son fond (S398–S415) : ≈ 3 100 lignes, dont la
moitié pour la zone, le fond, l'échange à masse exacte et la bascule. Le critère de C7 — *« B10 de la production à 3 mm de la
référence ; coût par particule publié ; δ ≤ 2 ms avec la bande »* — porte sur l'ensemble. Porté d'un bloc, un écart ne se
localiserait pas. **C7 se découpe**, chaque morceau reçu contre la référence avant le suivant :

| | contenu | reçu si |
|---|---|---|
| **C7a** | le pas **nu** : tri par maille, particules → grille, surface reconstruite, gravité, parois, projection à fluide fantôme, extrapolation, grille → particules, advection RK2, séparation | chaque étage à l'arrondi de la référence ; le ballottement (1, 0) de S388 à 3 mm sur 10 s ; coût par particule publié |
| **C7b** | le **corps** cinématique (S393) | B10 **nu** (APIC seul) : pincement au pas de la référence, surface à 3 mm avant le pincement |
| **C7c** | la **zone des colonnes et le fond** (S398–S414) : étiquettes, virtuelles, advection et transport de la part eulérienne, soldes, échange à la face, bascule | B10 en bande étroite à 3 mm de la référence ; masse au bit de la référence (ou l'écart publié) ; densité au bas de la bande |
| **C7d** | **relative à B** : la bande sur la production couplée (ADR-198), le fond qui suit la vitesse propre de δ (BANDE-ETROITE-S413 §6.4) | la vague de Chen sous B : particules seulement où δ se déforme ; rien sous une houle calme |
| **C7e** | **le budget** : la pression par la multigrille de la carte (C3), le tri par bloc si le tri par maille coûte | δ ≤ 2 ms au 99ᵉ centile avec la bande, sur la scène de la porte B |

L'ordre protège une dépendance chaque fois (L343) : la zone (C7c) s'appuie sur le pas nu ; le relatif à B (C7d) sur la zone ;
le budget (C7e) se mesure sur ce qui est complet, et **la multigrille ne se branche qu'une fois le pas reçu au gradient conjugué
simple**, pour qu'un écart ne mêle pas deux causes.

### 1.2 Les choix de C7a

- **Même disposition que la référence** : mailles `(k·ny + j)·nx + i`, trois grilles décalées de dimensions `(nx+1, ny, nz)`,
  `(nx, ny+1, nz)`, `(nx, ny, nz+1)` ; les relectures de banc se comparent indice à indice.
- **Le tri par maille** : compte (addition atomique entière), préfixe exclusif, rangement ; puis **chaque maille trie sa tranche
  par indice de particule** — l'ordre de la référence exactement, et un pas **déterministe** d'une exécution à l'autre, malgré
  l'ordre arbitraire des atomiques.
- **Particules → grille par collecte** : chaque face lit les particules des mailles qui peuvent l'atteindre — deux mailles le
  long de son axe, trois dans les deux autres — et recalcule leurs poids trilinéaires bornés (`weights` de la référence). **Aucun
  atomique flottant** (WGSL n'en a pas), aucun point fixe ; la somme se fait dans un autre ordre que la référence, donc à
  l'arrondi près. C'est l'inverse de Gao *et al.* (2018 ; dispersion par bloc en mémoire partagée, CUDA et atomiques flottants) :
  **non relu dans la session** — connu en résumé seulement ; le tri par bloc reste l'option de C7e si la collecte coûte.
- **La projection** : gradient conjugué préconditionné par la diagonale, comme la référence, scalaires gardés sur la carte (aucune
  relecture par itération), **arrêt au même critère relatif** (`‖r‖² ≤ tol²·‖b‖²`) par un drapeau que les noyaux lisent, sous un
  plafond d'itérations — travail borné (ADR-175 D2). Les produits scalaires se réduisent en `f32` par groupe puis en `f32`
  (la référence les somme en `f64`) : l'écart se lit sur le résultat, il ne se suppose pas.
- **La séparation** en collecte : chaque particule somme les poussées de ses voisines ; même résultat que la référence par paires,
  à l'ordre de sommation près.
- **Le pas `dt`** est donné par l'hôte (au banc : celui de la référence, pas à pas) ; la vitesse maximale pour le choisir sur la
  carte relève de C7e (ADR-175 D3 : diagnostics différés).

## 2. Critères — écrits avant (S416, C7a)

1. **Chaque étage**, sur le même état d'entrée, à l'arrondi de la référence : tri — comptes par maille identiques ; vitesses de
   grille à 10⁻⁵ m/s ; `φ` à 10⁻⁵ m, étiquettes identiques ; vitesses corrigées à 10⁻⁴ m/s (les deux solveurs à résidu relatif
   ≤ 10⁻⁵).
2. **Le ballottement (1, 0) de S388**, 5 cm, 10 s : la surface de la carte à **3 mm** de la référence ; période à 0,5 %.
3. **Le coût par particule**, par étage, publié.
4. Zéro avertissement ; la suite du cœur inchangée.

**Arrêt** : un étage qui diverge au-delà de l'arrondi se publie, et le suivant attend.

## 3. La construction, étage par étage

**Le cœur** : `Apic3::step_upto(d, étage)` exécute le pas jusqu'à un étage (`ApicStage`) et s'arrête ; `step` en est le cas
`Full`, **au bit** (essai `the_step_stops_at_each_stage_s416`). Accesseurs de banc : `affine`, `face_weights`, `pressure`, `bins`,
`settings`, `physics` ; les constantes du pas sont publiques. Rien d'autre ne change dans le cœur.

**La carte** : `viewer/src/apic3d_carte.rs` et `apic3d_carte.wgsl` — quinze tampons réservés à la configuration, vingt-sept
noyaux, un passage de calcul horodaté par étage. Le banc charge sur la carte l'état d'une référence chauffée (ballottement (1, 0),
5 cm, 20 pas, 12 800 particules), exécute les deux jusqu'au même étage et compare :

| étage | écart carte / référence | critère | carte, ms |
|---|---|---|---|
| tri par maille | débuts et ordre **identiques** | identiques | (dans le transfert) |
| particules → grille | vitesses **1,6·10⁻⁷ m/s** (max 0,13) ; poids 2,3·10⁻⁵ (sommes ≤ 8) ; mêmes faces alimentées | 10⁻⁵ m/s | 0,10 |
| surface reconstruite | `φ` **1,6·10⁻⁶ m** ; étiquettes identiques (1 600 mailles d'eau) | 10⁻⁵ m | 0,39 |
| projection | **94 itérations des deux côtés** ; résidu 7,05·10⁻⁷ contre 7,04·10⁻⁷ ; pression 5,4·10⁻³ Pa sur 4 694 ; vitesses **2,7·10⁻⁶ m/s** | 10⁻⁴ m/s | 1,0 à 3,0 |
| extrapolation | vitesses 2,7·10⁻⁶ m/s ; 5 668 faces non nulles des deux côtés | 10⁻⁴ m/s | 0,05 |
| grille → particules | vitesses 2,6·10⁻⁶ m/s ; `C` 1,1·10⁻⁴ s⁻¹ (le gradient amplifie de 1/dx) | 10⁻⁴ m/s | 0,02 |
| advection | positions **1,2·10⁻⁷ m** | 10⁻⁵ m | 0,01 |
| séparation | positions 1,2·10⁻⁷ m | 10⁻⁵ m | 0,19 |

**Critère 1 tenu**, à chaque étage. Les produits scalaires du gradient conjugué, réduits en `f32` sur la carte, s'arrêtent à la
même itération que ceux de la référence, en `f64`.

**Deux pièges de la cible** (L345) : FXC, le compilateur de DX12, refuse l'écriture indexée dynamiquement dans un vecteur
(`v[a] = …` dans une boucle) — les poids trilinéaires s'écrivent en opérations vectorielles et `select` ; et le drapeau de fin du
gradient conjugué se lit par `workgroupUniformLoad`, sans quoi la sortie anticipée d'un groupe n'est pas uniforme devant ses
barrières.

## 4. Le ballottement sur la carte

### 4.1 À 5 cm, 10 s — critères 2 et 3

La carte et la référence partent du même ensemencement et avancent du **même pas** (celui que choisit la référence). À chaque
pas, la surface de chaque colonne est lue sur `φ` des deux côtés — l'iso-zéro au-dessus de la plus haute maille d'eau, interpolée
—, et le moment de la masse donne la période (S388).

| grandeur | carte | référence |
|---|---|---|
| écart de surface, pire colonne, pire pas | **0,456 mm** (à t = 7,14 s) | — |
| période | 1,9965 s | 1,9964 s (**+0,003 %**) |
| passages par zéro | 10 | 10 |
| itérations moyennes | 93,5 | 93,5 |
| projections non convergées au plafond (200) | 0 | — |
| une particule, en fin de calcul | 1,72 mm de sa jumelle | — |

**Critère 2 tenu** : la surface à 0,46 mm, six fois sous les 3 mm de la production ; la période à 0,003 %. L'écart reste sous
0,03 mm pendant 6 s, puis saute à 0,46 mm et n'augmente plus : une colonne dont la surface passe d'une maille à l'autre un pas plus
tôt d'un côté. Les trajectoires individuelles divergent (1,72 mm), la surface non — ce qui se juge est la surface.

### 4.2 À 2,5 cm, 2 s — le coût quand les particules sont huit fois plus nombreuses

80 × 8 × 40 mailles, **102 400 particules**, 101 pas, plafond de 400 itérations : surface à **0,77 mm** de la référence (à
t = 1,96 s), **193 itérations des deux côtés**, aucune non convergée ; une particule à 11 mm de sa jumelle en fin — les
trajectoires divergent plus vite à maille fine, la surface non. Deux passages seulement en 2 s : la période ne se lit pas ici (la
durée est choisie pour le coût, la référence prenant une minute par seconde simulée).

| étage, ms au 99ᵉ centile | 5 cm (12 800) | 2,5 cm (102 400) | rapport |
|---|---|---|---|
| transfert | 0,102 | 0,481 | 4,7 |
| surface | 0,410 | 0,623 | 1,5 |
| projection (plafond enregistré) | 1,389 (200) | 3,315 (400) | 2,4 |
| séparation | 0,150 | 0,523 | 3,5 |
| le reste | 0,021 | 0,053 | 2,5 |
| **total** | **2,05** | **4,77** | **2,3** |
| **par particule** | 160 ns | **47 ns** | ÷ 3,4 |

Huit fois plus de particules coûtent 2,3 fois plus : la carte est encore loin d'être pleine à 12 800 particules — le coût par
particule n'a de sens qu'à charge donnée, et **47 ns** est le chiffre à la charge d'une scène.

## 5. Le coût — critère 3

Au 99ᵉ centile par pas, horodatages de la carte, 5 cm, 12 800 particules, 500 pas :

| étage | ms | part |
|---|---|---|
| transfert (tri compris) | 0,102 | 5 % |
| surface reconstruite | 0,410 | 20 % |
| **projection** (plafond de 200 itérations enregistrées) | **1,389** | 68 % |
| extrapolation | 0,013 | 1 % |
| grille → particules | 0,005 | — |
| advection | 0,003 | — |
| séparation (deux passes, tri compris) | 0,150 | 7 % |
| **total** | **2,05** | **160 ns par particule** |

**Ce qui coûte n'est pas ce qu'on attendait** (ADR-131 : un dépassement qualifie l'implémentation). Les transferts par particule —
ce que Gao *et al.* optimisent — pèsent 6 %. **La projection** pèse deux tiers, et pas par ses calculs : chaque itération du
gradient conjugué est cinq dispatchs, **enregistrés même quand le drapeau de fin les rend vides** ; 94 itérations utiles sur 200
enregistrées. Au banc des étages, le même pas passe de 1,0 ms (plafond 120) à 4,1 ms (plafond 400). **La reconstruction** vient
ensuite : chaque maille lit les 125 mailles de son noyau et leurs images. Les deux leviers sont nommés pour C7e : la multigrille de
la carte (C3 : 6 à 8 cycles là où le gradient conjugué en demande 94), et une reconstruction qui ne lit que près de la surface.

## 6. Ce que ce document ne dit pas

- **Le budget** : la bande étroite entière est sur la carte (§9–12, S417–S420), mais 28,7 ms sur B10 — C7e ; la vitesse relative à B et
  les critères d'écoulement de S415 — C7d.
- **Le pas `dt`** est celui de la référence : la carte ne le choisit pas elle-même (C7e, diagnostics différés).
- **Le budget** : 2,05 ms pour une cuve de 2 × 0,2 m est le budget entier de δ ; aucune conclusion de budget n'en sort avant la
  multigrille et la bande étroite, qui retire les particules profondes (÷ 4,7 à 7,7, S413).
- **Une seule cible** : DX12 sur ce poste.

## 7. C7b — le corps sur la carte (S417)

**Reproduire** : commit de S417 P3 ou plus récent ; `CAS=b10 CHAUFFE=15 ITERATIONS=400 … --apic3d-carte-etages` (2 min) ;
`… --apic3d-carte-b10` (une minute ; `FR=`, `ND=`, `ITERATIONS=` : 2, 8, 600) ; le témoin : `TEMOIN=1e-6 … --apic3d-carte-b10`
(deux minutes, la carte n'est pas calculée).

**La construction.** Le corps cinématique de S393 : `Params` porte le centre au début du pas, la vitesse et le centre avancé ;
`reconstruct` ajoute l'image radiale du corps et marque solides les mailles dont le centre est dans la sphère ; `impose_body`
(après la gravité, après l'extrapolation) donne sa vitesse à toute face qui touche une maille solide, une face de bord restant
une paroi ; `move_body` (fin du pas) repousse les particules atteintes. Le banc reproduit B10 de `examples/apic3d_b10.rs` (Fr = 2,
D/dx = 8, quart de domaine, 16 × 16 × 84 mailles, 131 072 particules), le corps reposé à chaque pas des deux côtés.

**Critère 1 — les étages avec le corps**, sur B10 chauffé de 15 pas (corps à −3,96 m/s, 67 mailles solides) : transfert 4,8·10⁻⁶
m/s (vitesse max 5,9) ; `φ` 4,7·10⁻⁶ m, étiquettes identiques ; **211 itérations des deux côtés**, vitesses 5,2·10⁻⁶ m/s ; faces
non nulles après extrapolation 49 248 contre 49 246 (deux faces nulles d'un seul côté, dans l'écart de vitesse) ; retour 7,8·10⁻⁶
m/s ; positions **2,4·10⁻⁷ m** après le corps. **Tenu.** Le ballottement de C7a est inchangé au caractère près.

**Critère 2 — B10 nu, 71 pas.**

| | carte | référence |
|---|---|---|
| pincement | **pas 54**, t = 2,1308 √(R/g) | **pas 54**, 2,1308 |
| profondeur du pincement | 1,312 D | 1,312 D |
| air enfermé au pincement | 0,0781 D³ | 0,0781 D³ |
| base du corps | 3,013 D | 3,013 D |
| cavité maximale | 1,937 D | 1,937 D |
| couronne | 0,205 D | 0,205 D |
| itérations moyennes | 207,5 | 207,4 |

**Le pincement est celui de la référence au chiffre près** — et celui de S414 (1,937 D, 0,078 D³). **L'écart de `φ` dans la bande
de l'interface** (|φ| < dx d'un côté), lui, **manque les 3 mm écrits** : ≤ 0,32 mm jusqu'au pas 44, puis des pointes d'un pas —
1,08 (pas 48), 1,70 (49), **9,20 (52)**, 2,58 (53), 0,66 (54).

**Le témoin** (METHODE : l'incertitude vraie est la sensibilité à une perturbation minime, L371) : une **seconde référence**, dont
les vitesses initiales sont perturbées de ±ε (pseudo-aléatoire par position), contre la première, par le même instrument.

| ε | écart de `φ` max | pas 48 | pas 49 | pas 52 | pincement |
|---|---|---|---|---|---|
| **carte** (ε = 0, l'arrondi de la carte) | **9,20 mm** | 1,08 | 1,70 | 9,20 | pas 54, identique |
| 10⁻⁶ m/s | **22,59 mm** | 0,50 | 22,59 | 8,77 | pas 54, identique |
| 10⁻⁴ m/s | **21,22 mm** | 0,68 | 21,22 | 7,87 | pas 54, identique |

**Ce que cela dit** (fait mesuré, puis explication, séparés — L177). *Fait* : la référence s'écarte d'elle-même de 21 à 23 mm sous
une perturbation de 10⁻⁶ m/s, aux mêmes pas que la carte, et la carte en reste plus près (9,2 mm) ; hors de ces pas, carte et
témoins sont au même niveau (0,2 à 0,5 mm au pas 40). *Explication* : `φ = |q − x̄| − r` est **discontinu** là où le noyau ne voit
presque plus de particule — x̄ porté par une ou deux particules lointaines, ou `φ = dx` sans voisine — ce qu'est le col de la
cavité juste avant qu'il se ferme. **Le critère 2, tel qu'écrit, est manqué ; il ne peut être tenu par aucune implémentation de
cette référence**, qui ne le tient pas contre elle-même. Il n'est pas relevé : il est remplacé, pour les réceptions suivantes
(C7c), par un critère écrit avant elles — **l'écart de la carte ne dépasse pas celui du témoin à 10⁻⁶ m/s, pas à pas**, et le
pincement tombe au même pas. Sur B10 nu, la carte le tient à chaque pas publié sauf au pas 48 (1,08 contre 0,50 mm), sous le
millimètre dans les deux cas.

**Critère 3 — le coût**, au 99ᵉ centile, 131 072 particules : **6,42 ms, 49 ns par particule** — transfert 0,68, surface 0,82,
**projection 4,18** (plafond de 600 itérations enregistrées pour 207 utiles), séparation et corps 0,80, le reste 0,09. Le même
constat qu'en C7a : la projection d'abord, par ses dispatchs enregistrés.

## 8. La conception de C7c — la zone des colonnes et le fond sur la carte (S417)

### 8.1 Ce que la référence fait, et ce qui ne se transporte pas tel quel

`apic3d_columns.rs` (≈ 1 800 lignes) porte quatre choses : **la zone** (masque, `η` par colonne, vitesses eulériennes advectées au
pied de la caractéristique, `φ = z − η`, particules virtuelles dans la reconstruction, `η` transporté par les débits mouillés —
S398) ; **l'échange** à la frontière latérale (absorption, soldes par face-maille, retrait de la particule la plus proche, pose
à la face — S399–S407) ; **le fond** de la bande (S413–S414 : contenant plein, solde vertical, déplacement) ; **la bascule** et
son critère (`ColumnsSwitch`, S408–S415). Trois de ses gestes n'ont pas d'équivalent direct sur la carte :

- **La masse en `f64`.** Soldes, débits et volumes sont en `f64` dans la référence, pour que la masse se compte au bit. **wgpu 30
  n'offre `SHADER_F64` que sous Vulkan** (lu dans `wgpu-hal`), et le poste tourne en DX12 (S211). **Décision** : sur la carte, les
  volumes de l'échange se comptent **en entiers** — un quantum fixe, fraction exacte du volume d'une particule (`dx³/8`), sur deux
  mots de 32 bits ; un débit s'arrondit **une fois**, au quantum, et le même entier s'ajoute d'un côté et se retranche de l'autre.
  La conservation est alors **exacte par construction**, ce que le `f64` n'assure qu'à l'arrondi ; l'écart à la référence est
  celui de l'arrondi d'un débit au quantum, qui se publie.
- **Le nombre de particules change pendant le pas** (absorption, retrait, pose, bascule). Sur la carte, `n` devient **résident** :
  les noyaux par particule se lancent en **dispatch indirect**, leur taille calculée sur la carte ; une particule retirée se
  marque, puis un **compactage stable** (préfixe sur les vivantes) la supprime ; les poses s'allouent par **préfixe** sur les
  faces-mailles qui posent — aucune allocation par atomique, donc un ordre déterministe, comme le tri de C7a.
- **Les choix séquentiels** (« la particule la plus proche de la face », « le sous-réseau le plus libre ») sont **locaux à une
  face-maille** : un fil par face-maille, sur les particules triées par maille, les rend en parallèle sans changer la règle.

### 8.2 Découpage

| | contenu | reçu si |
|---|---|---|
| **C7c-1** | la zone **sans échange** : tampons, `columns_begin`, `columns_advect`, `columns_label` (lecture tabulée de S400 comprise), particules virtuelles dans la reconstruction, `columns_transport` en quanta entiers | étages à l'arrondi sur une cuve mixte ; **la cuve tout en colonnes** (`APIC3D_COLONNES`, S398) : ballottement à 3 mm de la référence sur 10 s, volume conservé exactement |
| **C7c-2** | **l'échange** latéral : absorption, soldes, retrait, pose, réserve ; `n` résident, compactage, dispatch indirect | le **raccord** (`apic3d_raccord`, S399–S407) : ballottement à 3 mm sur 30 s ; volume exact ; densité au raccord dans la tolérance de S407 |
| **C7c-3** | **le fond** : étiquettes sous le fond, solde vertical, absorption sous le fond, déplacement (`move_band_floor`) | le ballottement en bande étroite (`APIC3D_FOND=4`, S413) à 3 mm ; volume exact |
| **C7c-4** | **la bascule et son critère** (`ColumnsSwitch` : pente, corps, dilatation, maintien, fond placé) sur la carte | **B10 en bande étroite** contre la référence : pincement au même pas, écart de `φ` à l'interface **pas plus grand que celui du témoin à 10⁻⁶ m/s, pas à pas** (§7 : le critère de 3 mm n'est pas tenable au col de la cavité) — **le critère de C7** |

L'ordre protège une dépendance chaque fois : l'échange suppose la zone ; le fond réemploie l'échange tourné à la verticale
(ADR-212 D3) ; la bascule manipule les trois. La décision de la bascule reste **sur la carte** : la production ne relit rien dans
le pas (SPEC-004 §8.4).

## 9. C7c-1 — la zone des colonnes sur la carte, sans échange (S417)

**Reproduire** : `CAS=raccord ITERATIONS=200 … --apic3d-carte-etages` (la cuve mixte : la moitié `x ≥ Lx/2` en colonnes) et
`CAS=colonnes` (tout en colonnes) ; `CAS=colonnes DUREE=10 ITERATIONS=200 … --apic3d-carte-ballottement` (12 s). Cœur :
`Apic3::columns_state` (masque, reste de `η`, table de lecture de S400, bande) ; rien d'autre ne change.

**La construction.** `load` charge aussi les vitesses de la grille du pas précédent et la zone. `columns_begin` garde la vitesse
du début du pas ; `columns_advect` la porte au pied de la caractéristique sur les faces de la zone (une face `u` ou `v` l'est si
l'une de ses colonnes l'est — S406 ; une face `w`, si sa colonne l'est) ; `reconstruct` donne à une maille de colonne
`φ = z − (η + e(η))` (la lecture de S400) et ajoute, pour les autres, les **particules virtuelles** des colonnes voisines ;
`columns_flux` et `columns_update` transportent `η` par les débits mouillés.

**La masse en entiers, dès C7c-1.** La référence avance `η` en `f64` avec un reste en `f32`. Sur la carte, ni `f64` (§8.1), ni
double flottant sûr : une transformation sans erreur exige un `mad` **fusionné**, que FXC ne garantit pas (L345 : le compilateur
est dans la boucle). **Le volume de chaque colonne est donc un entier** de quanta `q = dx³/8 · 2⁻²⁴` (une particule vaut 2²⁴ quanta ;
`q/dx²` = 3,7·10⁻¹⁰ m à 5 cm), sur deux mots de 32 bits en complément à deux ; chaque débit de rangée s'arrondit une fois au
quantum, et le même entier sort d'une colonne et entre dans l'autre. **La conservation est exacte par construction** ; `η` se relit
en flottant pour la surface. Une piège de plus : WGSL arrondit `round` au pair, la référence (`f32::round`) loin de zéro — les
rangées virtuelles s'écrivent `floor(x + 0,5)`.

**Critères** (écrits avant, EN-COURS S417) :

| étage, cuve mixte (moitié colonnes) | écart | critère |
|---|---|---|
| surface (étiquettes des colonnes, virtuelles) | `φ` **5,9·10⁻⁷ m**, étiquettes identiques | 10⁻⁵ m |
| advection de la zone | **7,8·10⁻⁸ m/s** | 10⁻⁵ m/s |
| projection | 94 / 94 itérations, vitesses 2,5·10⁻⁶ m/s | 10⁻⁴ m/s |
| transport de `η` | **2,4·10⁻⁷ m** (tout en colonnes : 2,7·10⁻⁷) | 10⁻⁶ m |
| advection des particules | positions 1,2·10⁻⁷ m | 10⁻⁵ m |
| fin du pas (séparation, échange) | non comparée : l'échange et la séparation tenue côté bande sont C7c-2 | — |

**La cuve tout en colonnes, 10 s, 500 pas** : surface **à 0,003 mm** de la référence, période 1,9768 s des deux côtés, 10
passages, 89,1 itérations des deux côtés ; **volume de la carte constant, exactement** (la référence : 3·10⁻¹⁶ m³). Coût p99 1,46 ms,
dont 1,39 de projection (le plafond d'itérations enregistrées, toujours). **C7c-1 reçu** ; le ballottement en particules et B10
sont inchangés au caractère près.

**Ce qui reste de C7c** (§8.2) : l'échange (C7c-2, le raccord, `n` résident), le fond (C7c-3), la bascule (C7c-4, B10 en bande
étroite contre le témoin).

## 10. C7c-2 — l'échange à la frontière sur la carte (S418)

**Reproduire** : `CAS=raccord CHAUFFE=<2…90> ITERATIONS=200 … --apic3d-carte-etages` (un pas entier, ligne `etage=echange`) ;
`CAS=raccord DUREE=30 ITERATIONS=200 … --apic3d-carte-ballottement` (80 s) ; les témoins : `TEMOIN=1e-6` ou `1e-4` (deux minutes,
la carte n'est pas calculée). Cœur : `columns_soldes`, `particle_capacity`.

**La construction.** `n` est **résident** (lu par les noyaux, lancés sur la capacité ; le dispatch indirect est C7e). Les soldes
sont des **quanta entiers**, chargés par le transport au même entier que le débit. L'échange de la référence est séquentiel et
**dépend de l'ordre** : le mélange `f += w·(v − f)/8` de deux absorbées dans l'ordre inverse diffère de `a·b·(v₂ − v₁)`, jusqu'à 1/64
de l'écart des vitesses — pas un arrondi. La carte **garde donc ses tableaux indice pour indice avec la référence** : les
absorbées sont listées en parallèle, triées, et un fil les traite dans **l'ordre de visite de la référence**, reconstruit à deux
pointeurs (elle monte, et remplace une absorbée par la dernière, examinée aussitôt) ; un second fil règle les soldes face-maille
par face-maille dans son ordre (retrait de la plus proche de la face, pose à `dx/16` au sous-réseau le plus libre), puis retire
les marquées par échange avec la dernière, du plus grand indice au plus petit. Les gestes sont rares (une ligne de faces-mailles) ;
les fils coûtent ≈ 0,7 ms, à paralléliser par coloriage en C7e. Un **compactage stable** (P3, exact) reste pour la bascule.

**Un pas entier, cuve mixte** (critère 2), après 2 à 90 pas de chauffe : absorbées, retirées, posées et `n` **identiques** à chaque
fois (retraits 32 et 12, poses 8 et 4, absorptions 16, 20, 4) ; **positions à 6·10⁻⁸ m indice pour indice** (3·10⁻⁷ au pire) ;
vitesses ≤ 1,4·10⁻⁵ m/s ; soldes ≤ 1,1·10⁻¹⁰ m³ ; `η` ≤ 3·10⁻⁷ m. **Tenu.** Les soldes après le seul transport sont à 58 quanta
(5,4·10⁻¹¹ m³) : le critère « au quantum près » était mal posé — un solde est `u·dx²·dt`, et `u` porte déjà l'écart admis de la
projection (2,5·10⁻⁶ m/s, soit 1,3·10⁻¹⁰ m³ au plus).

**Le raccord, 30 s** (critère 3) :

| grandeur | carte | témoin ε = 10⁻⁶ | témoin ε = 10⁻⁴ |
|---|---|---|---|
| écart de surface, maximum courant, 4 s | 0,34 mm | 0,39 | 2,01 |
| 10 s | 2,22 mm | 2,23 | 2,89 |
| 22 s | 3,23 mm | 3,11 | 3,98 |
| **30 s** | **4,41 mm** | **3,40** | **4,03** |
| écart de période | −0,036 % | −0,043 % | −0,063 % |
| volume | **constant, 0 quantum** | (référence : 2·10⁻¹⁶ m³) | |
| gestes cumulés, carte / référence | absorbées 5 204 / 5 256, retirées 109 / 108, posées 5 303 / 5 356, `n` 6 574 / 6 576 | | |

**Ce que cela dit.** *Faits* : les tableaux cessent d'être identiques indice pour indice **au pas 12**, le premier pas de retraits,
et l'ensemble diffère alors d'une particule à 25 mm ; la surface, elle, s'écarte lentement, **sur la même courbe que les
témoins** ; la carte finit à 4,41 mm, 10 % au-dessus du plus grand des deux témoins ; le volume de la carte ne varie pas d'un
quantum en 1 500 pas. *Explication* : le retrait prend « la plus proche de la face », et sur le réseau d'ensemencement plusieurs
particules en sont à **égale distance** — l'arrondi tranche l'égalité autrement, puis l'écoulement amplifie la différence comme il
amplifie une perturbation de 10⁻⁶ m/s. **Le critère 3 (3 mm) est manqué, et la référence ne le tient pas contre elle-même
(3,40 et 4,03 mm)** ; la comparaison « au plus le témoin » est manquée de peu sur deux échantillons, dans la même dispersion. La
masse, elle, est exacte, ce que la référence n'est qu'à l'arrondi du `f64`. **C7c-2 reçu à l'échelle du témoin** ; un troisième
témoin, ou une moyenne sur plusieurs, trancherait l'écart de 10 % — à faire avant de s'appuyer sur un écart plus fin.

**Coût** (raccord, 6 584 particules, p99) : 2,84 ms — projection 1,40, séparation et échange 0,88, surface 0,52.

## 11. C7c-3 — le fond de la bande sur la carte (S419)

**Reproduire** : `CAS=fond CHAUFFE=<5…70> ITERATIONS=200 … --apic3d-carte-etages` ; `CAS=fond DUREE=30 ITERATIONS=200 …
--apic3d-carte-ballottement` (70 s) ; témoins `TEMOIN=1e-6`, `1e-4`. `band_state` reproduit la bande étroite de S413 (toute la cuve
en bande, fond à quatre mailles sous le creux). Cœur : `columns_solde_w`.

**La construction.** Le fond par colonne (`cols[2C + 32 + col]`, paramètre `floors`) ; sous le fond, `φ = z − fond` et l'eau ; les
virtuelles jusqu'au fond ; les faces à la grille advectées (une face `w` au-dessus d'une maille sous le fond comprise) ; le
transport charge les **soldes verticaux** — chaque face de colonnes garde ses deux contributions (côté bas, côté haut), qu'un
noyau par colonne rassemble, sans atomique ; l'échange absorbe sous le fond (mélange aux faces à la grille, `floor_face`), lit la
frontière latérale maille par maille, et règle le solde vertical (retrait de la plus basse, pose à `dx/16` au-dessus du fond). Le
volume compte l'eau sous le fond en mailles entières. **Le déplacement du fond** (`move_band_floor`) part avec la bascule (C7c-4).

**Étages et pas entier** (critère 1) : `φ` 8,4·10⁻⁷ m, étiquettes identiques, projection 94/94, soldes verticaux à 8,2·10⁻¹⁰ m³ (sur
1,6·10⁻⁵ ; la borne de l'écart de vitesse admis ≈ 3·10⁻⁹) ; un pas entier après 5, 40, 70 pas : gestes et `n` identiques, positions
à 1,2·10⁻⁷ m indice pour indice. **Après 20 pas, 3 poses sur 32 tombent à l'emplacement miroir en `y`** (même maille, même hauteur,
sous-réseau 0,25 ↔ 0,75) : ce cas est **invariant en `y`** — les emplacements miroirs y sont à des distances quasi égales, et les
entrées diffèrent déjà à l'arrondi de l'advection. Une tolérance « le premier gagne » aggrave (7 sur 32 : la référence les ordonne
vraiment) ; les carrés évalués sans contraction en `mad` (`square_sum`) ne changent rien ici et sont gardés, plus fidèles — ils
déplacent le raccord de §10 à l'intérieur de sa dispersion (4,41 → 4,48 mm à 30 s).

**Le ballottement en bande étroite, 30 s** (critère 2) :

| grandeur | carte | témoin ±10⁻⁶ m/s | témoin ±10⁻⁴ m/s |
|---|---|---|---|
| écart de surface, maximum courant, 8 s | 0,57 mm | 0,43 | 0,56 |
| 20 s | 1,45 mm | 0,44 | 0,77 |
| **30 s** | **1,45 mm** | **0,92** | **1,18** |
| écart de période | −0,015 % | −0,005 % | 0,000 % |
| volume | **constant, 0 quantum** | | |
| gestes cumulés, carte / référence | absorbées 5 617 / 5 541, retirées 360 / 352, posées 5 950 / 5 862, `n` 5 093 / 5 089 | | |

**Critère 2 tenu** : la surface reste à 1,45 mm de la référence, sous les 3 mm, la période à 0,015 %, la masse exacte. La carte
finit au-dessus des deux témoins (0,92 et 1,18 mm) : l'arrondi de la carte pèse ici un peu plus qu'une perturbation de 10⁻⁴ m/s,
sans approcher la tolérance. **C7c-3 reçu.**

## 12. C7c-4 — la bascule et le fond placé sur la carte ; B10 en bande étroite (S420)

**Reproduire** : `--apic3d-carte-decision` (`CHAUFFES=0,10,…` ; `INITIAL=1` la bascule initiale ; `PENTE=0.02`, `MAINTIEN=0` des
bascules forcées ; `SANS_FOND=1`) ; `BANDE=1 … --apic3d-carte-b10` (20 s) ; témoins `BANDE=1 TEMOIN=1e-6|1e-5|1e-4` (30 s). Le réglage
de B10 en bande étroite est celui de R35 (maintien 0,3 s, fond 4). Cœur : `ColumnsSwitch::switch_state`, `Apic3::columns_reserve`.

**La construction.** **La décision** par colonne, en parallèle (`switch_need` : hauteur convertible et corps ; `switch_slope` ;
`switch_spread` et `switch_request` : dilatation de Chebyshev séparable, maintien sur un instant de 32 bits — 71 minutes, à rendre
relatif en C7e) sur la surface rafraîchie. **La bascule** sur un fil, dans l'ordre de la référence (`switch_apply`) : les particules des
converties retirées par la visite de la référence, l'eau sous le fond et le solde vertical comptés, **la voie mixte en quanta** (le
décalage borné à 2²⁵ quanta, un quart de maille ; le reste à la réserve, exactement), l'ensemencement nominal des colonnes qui
repassent aux particules, les soldes des faces qui cessent d'être frontière à la réserve ; **la réserve réglée** au début de l'échange ;
le drapeau « une bande existe » résident. **Le fond** placé par colonne (`floor_place`) et déplacé sur un fil (`floor_move`). **La
liste des retirées** se construit **triée, en parallèle** (préfixe par blocs) : le tri par insertion sur un fil, tenable pour quelques
absorbées, faisait tomber la carte (délai de garde du pilote) à la bascule initiale — 120 000 particules. Trois mots réservés de WGSL
rencontrés : `pass`, `target`, `from`.

**Une décision, une bascule** (critères 1 et 2), B10 en bande étroite :

| cas | colonnes basculées | écart |
|---|---|---|
| 21 instants ordinaires (0 à 70 pas) | aucune (la bande de B10 est stable) | masque demandé, `n`, positions, fonds **identiques** |
| ensemencements forcés (pente 0,02) | 186 et 166 vers les particules, 98 537 et 89 590 particules | positions **identiques indice pour indice**, réserves égales |
| conversions forcées (maintien nul) | 3 vers les colonnes | positions identiques, `η` 7,2·10⁻⁷ m, réserve égale |
| la bascule initiale, avec le fond | 200 vers les colonnes, 56 fonds posés | `n` 1 784, positions et fonds identiques, **`η` 1,9·10⁻⁶ m** |

Le volume de la carte est constant **à 0 quantum** dans tous les cas. Le critère « `η` à 10⁻⁶ m » est manqué à la bascule initiale
(1,9·10⁻⁶) : `η` s'y lit sur `φ`, admis à 10⁻⁵ — le critère était plus serré que sa source.

**B10 en bande étroite** (critère 3) :

| | carte | référence | témoin ±10⁻⁶ | ±10⁻⁵ | ±10⁻⁴ |
|---|---|---|---|---|---|
| pincement | **pas 55**, 2,1676 √(R/g) | pas 55, 2,1676 | pas 55 | pas 55 | pas 55 |
| profondeur / air / cavité / couronne | 1,438 D / 0,0781 D³ / 1,937 D / 0,199 D | identiques | identiques | air 0,0859 | air 0,0859 |
| `φ` à l'interface, max avant le pincement | **50,0 mm** (pas 51) | — | 2,46 mm | 50,0 mm | **197 mm** |
| volume | **0 quantum** | | | | |

Pas à pas (mm) — carte : 0,17 (28), 0,56 (36), 1,25 (44), 0,58 (48), 3,71 (50), 50,0 (51), 1,46 (52), 0,70 (53), 36,3 (54), 5,1 (55) ;
l'enveloppe des trois témoins : 1,22, 197, 4,59, 10,9, 50,0, 50,0, 42,9, 20,7, 36,3, 13,7. **La carte est dans l'enveloppe à chaque
pas publié** — égale au plus grand témoin aux pas 51 et 54 —, et son pincement est celui de la référence au chiffre près, quand deux
témoins changent l'air enfermé. Contre le seul témoin à ±10⁻⁶ que nommait le critère, c'est **manqué** : un témoin unique ne mesure pas
une dispersion (S418 l'annonçait ; trois la bornent). **Le premier écart** est au pas 35 : une pose de plus sur la carte (un solde au
seuil d'une particule, à quelques quanta ; les soldes portent l'écart de vitesse admis), masques et fonds identiques.

**Le critère de C7, pour ce qui touche à la précision** (*« B10 de la production à 3 mm de la référence »*, S384) : **tenu là où il a un
sens** — pincement, cavité, couronne, masse ; **sans objet au col**, où la référence ne se tient pas à 3 mm d'elle-même (2,5 à 197 mm sous
des perturbations de 10⁻⁶ à 10⁻⁴ m/s). **Le coût, lui, ne l'est pas** : 28,7 ms au 99ᵉ centile pour 4 000 particules et 21 504 mailles,
dont 23,6 ms pour la fin du pas (séparation, corps, échange sur un fil) et ≈ 5,5 ms de bascule — **C7e** : l'échange par coloriage, la
bascule en parallèle, la multigrille, le dispatch indirect. *« δ ≤ 2 ms avec la bande »* reste à faire.

## 13. C7e, premier temps — le coût de la bande étroite (S421)

**Reproduire** : `ADAPTATIF=1 BANDE=1 … --apic3d-carte-b10` (20 s) — la ligne `cout_p99_ms` donne chaque sous-étage ; `DEBUG_SOLDES=1`
ajoute la réserve, les faces-mailles mouillées et actives par pas.

**La méthode.** Mesurer d'abord, par sous-étage (horodatages : la fin du pas en séparation, absorption et échange, chacun séparé de
son fil ; la bascule en décision, application, fond), puis réduire **à sémantique exacte** — chaque réduction garde le pincement, les
gestes, `n`, le volume et les pas entiers comparés à la référence (raccord après 11, 17, 60 pas ; bande après 5, 40, 70 pas).

| B10 en bande étroite, p99 (ms) | S420 | après | ce qui a changé |
|---|---|---|---|
| échange | 21,44 | **0,20 + 0,59** | la réserve réglée en parallèle ; une **liste ordonnée des faces-mailles actives** (frontière, solde au-delà d'une particule) au lieu des ≈ 45 000 ; la pose en un parcours pour quatre emplacements (elle relisait quatre fois toutes les particules posées dans l'échange) ; puis le fil en **groupe de 64** : retrait et pose par minimums de groupe, exacts quel que soit l'ordre (clé du retrait : distance, côté, rang dans la maille — le départage de la référence) |
| absorption | 2,14 | **0,12 + 0,41** | les 24 faces qu'une absorbée met à jour sont distinctes : 24 fils, la visite de la référence gardée par le fil 0 |
| projection | 5,93 | **2,5 à 3,0** | **plafond d'itérations adaptatif** — 1,25 fois le plus grand des huit derniers pas, plus huit ; retour au plafond fixe après un pas non convergé ; aucun ne l'a été. Le dispatch indirect nul après convergence est refusé par wgpu (tampon d'arguments lié en écriture au même dispatch) |
| surface, séparation, transfert, le reste | 1,47 | 1,47 | — |
| **le pas** | **29,0** | **4,9** | ÷ 5,9 |
| bascule (décision, application, fond) | 1,93 | 1,91 | — (décision 0,86, dont la surface rafraîchie ; application 0,63 et fond 0,42 sur un fil) |

**Les bits, eux, bougent** — et ce n'est pas la sémantique. La réserve réglée en parallèle donne exactement les entiers du règlement
séquentiel à chaque pas (vérifié : −4 855 quanta au pas 37, puis 0), mais la série de `φ` passait de 1,25 à 1,24 mm au pas 44 ; **un
appel séquentiel sans effet remis dans le noyau de l'échange rendait les bits de S420**. FXC compile le flottant d'un noyau autrement
quand son code change (L345 : *le compilateur est dans la boucle*) : « au bit près » n'est pas un instrument de non-régression tenable
sur cette cible ; la non-régression se juge sur les issues discrètes et contre la référence. (Après la pose en un parcours, la série
est revenue à 1,25 — FXC encore.)

**Ce qui reste pour δ ≤ 2 ms au 99ᵉ centile** sur B10 en bande étroite (4 000 particules, 21 504 mailles) : **la projection** (2,5 à
3,0 ms : 207 itérations du gradient conjugué, cinq dispatchs chacune — la multigrille de C3, portée à l'opérateur à fluide fantôme
d'APIC, est le levier : 6 à 8 cycles là où le gradient conjugué en demande 207) ; **la bascule** (1,9 ms : application et fond sur un fil
— même traitement que l'échange ; la surface rafraîchie, 0,7 ms, que la référence recalcule aussi) ; **la surface** (0,73 ms : chaque
maille lit 125 mailles et leurs images ; ne reconstruire que près de l'interface). Somme visée de ces trois : ≈ 1 ms ; le reste ≈ 1,5 ms.
