# S192 — Tranche x-z à surface libre linéarisée

2026-09-12. S191-1. **Protocole et dérivation avant code et mesure**.
Véhicule de réception, pas choix technologique B3 ni réception non linéaire B4.

## 1. Problème physique et référence indépendante

Liquide incompressible, irrotationnel, inviscide ; potentiel φ en m²/s, vitesse ∇φ.
Domaine périodique x∈[0,L], z∈[−h,0], fond plat imperméable. Surface décrite par
η(x,t), conditions linéarisées au plan moyen :

```
Δφ=0 ; φ_z(−h)=0 ; ψ=φ(0)
η_t=φ_z(0) ; ψ_t=−gη           (pression atmosphérique)
```

La pression perturbative à z=0 vaut ρgη : elle compense la pression hydrostatique
−ρgη à la surface déplacée au premier ordre. Imposer φ=0 ou pression perturbative
nulle au plan moyen supprimerait le rappel de gravité.

Référence Airy : SPEC-001 §1/§6 et
[MIT, Free-Surface Waves](https://ocw.mit.edu/courses/2-016-hydrodynamics-13-012-fall-2005/3133000c14ec08086951d724ff277b13_free_surf_wave.pdf).
Onde stationnaire d'amplitude a, k=2π/L, ω²=gk tanh(kh) :
η=a cos(kx)cos(ωt), φ=−ag/ω cos(kx)sin(ωt) cosh(k(z+h))/cosh(kh).
Les vitesses de référence sont ses dérivées analytiques, évaluées aux positions
des vitesses discrètes. Elles ne partagent ni maillage ni solveur avec le candidat.
Le régime reste linéaire : aucune mousse, air, vorticité, rouleau ou coque reçus.

## 2. Tranche discrète : relèvement elliptique et surface évolutive

N points périodiques en x, K intervalles verticaux, dx=L/N, dz=h/K ; φ aux nœuds
z_j=−h+j dz, j=0..K. Laplacien centré horizontal/vertical. Pour un mode horizontal
q, `κ²=4sin²(πq/N)/dx²`, μ=κ²dz². Le relèvement de ψ se **résout** par système
tridiagonal : `φ_K=ψ`, `(1+μ/2)φ_0−φ_1=0`,
`−φ_(j−1)+(2+μ)φ_j−φ_(j+1)=0` à l'intérieur. Le bord inférieur représente
φ_−1=φ_1 (Neumann nul), pas φ_0=0.

La dérivée de surface issue du demi-volume supérieur est
`G_h ψ=(φ_K−φ_(K−1))/dz + dz κ² ψ/2`.
Le demi-terme horizontal est nécessaire : Taylor annule ainsi l'erreur d'ordre un.
G_h est positif, nul sur le mode constant. Le zéro est traité exactement.
Diagonalisation horizontale par DFT ; le candidat utilise le **symbole du Laplacien
discret**, pas k tanh(kh). La profondeur est résolue avec K inconnues ; aucun profil
cosh continu ne remplit le domaine candidat.

Pour vérification algébrique seulement, la récurrence admet un symbole indépendant :
γ=acosh(1+μ/2), φ_j/ψ=cosh(jγ)/cosh(Kγ),
`G_h = sinh(γ) tanh(Kγ)/dz`. Le programme de réception peut le comparer au
relèvement tridiagonal ; ce symbole ne remplace pas l'oracle continu Airy.

Évolution autonome des deux états η et ψ par Verlet : demi-pas de ψ avec −gη,
pas entier de η avec G_hψ, demi-pas final de ψ. Pas prescrit validé avec
`dt sqrt(g max G_h) < 2`, condition de stabilité dérivée de l'oscillateur discret.
Gravité injectée ; entrées/états non finis refusés. Publication seulement après
calcul fini ; aucun état changé lors d'un refus. f64/libm et allocations de banc,
pas prétention I-03/I-06/I-08 sur le runtime. Bibliothèque historique inchangée.

Vitesses : u aux faces horizontales `(φ_(i+1,j)−φ_(i,j))/dx` ; w aux faces verticales
`(φ_(i,j+1)−φ_(i,j))/dz`. Surface η aux nœuds x ; pression perturbative ρg relèvement(η).
La condition cinématique utilise le flux demi-volume G_h, pas la vitesse de la
dernière face située dz/2 sous la surface.

Énergie discrète par unité de densité/largeur :
`E=dx/2 Σ_i (gη_i² + ψ_i (G_h ψ)_i)` ; volume perturbatif dxΣη.
Vérifier aussi l'équivalence de ψG_hψ à la somme des carrés des gradients dans le
volume, avec poids verticaux 1/2 aux deux bords. L'énergie de Verlet oscille à O(dt²),
elle n'est pas supposée constante au bit.

## 3. Réceptions et campagne déclarées

Tests propres : relèvement contre récurrence et symbole, résidu du Laplacien,
bords/fond imperméable, identité énergétique surfacique/volumique ; onde au repos,
mode constant, deux modes distincts, non-finis/dimensions/g/dt/plafond CFL refusés
sans modifier l'état. Une gravité différente doit modifier l'évolution comme √g.

Campagne : L=8 m, a=0,001 m, g=9,81 m/s² injecté ; h=0,25/2/8 m
(peu profond/intermédiaire/profond). N/K=16/8,32/16,64/32,128/64.
Cinq périodes analytiques ; pas temporels T/1600 (ajustement entier exact de la
fenêtre), observations 80 par période, incluant début/fin et quadratures.
Mesurer sur toute la fenêtre : erreur max de η/a ; erreur max de u,w divisée par
`agk/ω` (borne analytique de vitesse sur le domaine) ; erreur de fréquence discrète
au continu ; dérive maximale d'énergie/E0 et de volume/(La).

**Réception numérique de ce montage** : à grille fine, η et vitesse ≤2 % ; c'est
une cible de validation du véhicule sous la précision demandée, pas un seuil de
bascule ou une réception de toute surface B4. Les erreurs spatiales doivent décroître
aux deux derniers raffinements, ordre >1,5 (ordre2 attendu, marge de régime déclarée).
Énergie relative <1e-4, volume normalisé <1e-10. Relever aussi une erreur de champ
contre la dynamique **semi-discrète** pour séparer l'espace et le temps.

Temps isolé sur N32/K16/h2 : T/50,100,200,400, même horizon, même oracle semi-discret ;
ordre >1,8 aux deux derniers doublements. Ce contrôle ne reçoit pas l'espace.
Les instantanés sont pris après des nombres entiers de pas, aucun pas rajouté aux
observations. Pour cette série, observations à chaque pas.

Contre-épreuves : surface figée (condition cinématique omise) et gravité de signe
inversé doivent dépasser2 % ; ne pas accepter un lac immobile comme onde reçue.
L'oracle peu profond ω=√(gh)k ne convient pas au cas profond : vérifier que la
différence avec Airy est bien observable, pas une convergence vers Saint-Venant.
Deux campagnes release identiques ; tests propres debug/release. Les chiffres
et empreintes ne seront ajoutés qu'après les exécutions.

## 4. Limite de ce qui sera fermé

Une onde2D en profondeur avec surface évolutive sera reçue dans le régime linéaire.
La comparaison additive de deux solutions linéaires est vraie par construction :
elle ne reçoit pas A217 ni B4 non linéaire (ADR-112). Le fournisseur B+W et le profil
de source80ms de S191 ne sont pas encore branchés à ce nouveau modèle.
Les autres chantiers de la file S190 restent portés. La suite devra nommer les
termes physiques manquants avant de prétendre comparer perturbatif et total.

## 5. Résultats exécutés — 2026-09-12

**S191-1 réalisée : onde de gravité reçue sur la tranche linéaire x-z.**
Les quatre grilles ont été exécutées aux trois profondeurs, sans modifier le protocole.
À N128/K64, erreurs maximales sur cinq périodes :

| h (m) | hauteur / a | vitesse / (agk/ω) | erreur de fréquence | ordre final hauteur / vitesse |
|---|---:|---:|---:|---:|
| 0,25 | 0,292748 % | 0,308062 % | 0,009809 % | 2,0069 / 2,0056 |
| 2 | 0,086520 % | 0,091040 % | 0,002899 % | 2,0256 / 2,0240 |
| 8 | 1,647737 % | 1,732796 % | 0,055212 % | 1,9923 / 1,9883 |

Le profond échoue nettement sur grille grossière (vitesse93,19 % à N16/K8),
puis converge ; une seule grille aurait confondu modèle dispersif et précision.
Les deux derniers ordres spatiaux sont tous >1,95. En temps isolé, erreurs de
hauteur semi-discrète1,965157/0,490840/0,122681/0,030672 % ; ordres finaux
2,000345 et1,999920. Au pas T/1600, le défaut temporel semi-discret de vitesse
reste au plus2,03423e-5 relatif sur l'ensemble des grilles.

Dérive maximale d'énergie de toute la campagne : **4,111837e-6 relatif**
(0,000411184 %), volume normalisé ≤1,456897e-16. Énergie contrôlée à chaque pas,
champs toutes les20 étapes ; les maxima de champ sont ceux de ces observations,
pas une borne analytique entre deux observations. Surface figée : erreur200 % ;
rappel inversé au quart de période :250,9176 %. Fréquence Saint-Venant au profond :
écart150,6637 % ; ce véhicule ne converge donc pas vers la dispersion peu profonde.

Trois tests propres **debug et release réussis**, dont récurrence indépendante,
identité énergétique volumique, deux fréquences simultanées, pression et refus
atomiques. Deux campagnes release ont des sorties identiques, empreinte des
indicateurs **0x4fc690d4ac035bf7**. Cible x86_64-pc-windows-msvc, rustc1.97.0
(2d8144b78), LLVM22.1.6. Pas de seconde cible, pas de mesure CPU.
Les331 tests workspace/cinq ignorés restent le reçu S190, non rejoués ici ;
seuls de nouveaux fichiers d'exemple/support ont été ajoutés.

Code : `code/water-core/examples/free_surface_2d.rs` et `support/free_surface.rs`.
[Sorties intégrales](SURFACE-LIBRE-2D-S192-MESURES.md).

## 6. Portée et prochaine construction

Le dépôt possède désormais une tranche2D **dispersive linéaire** avec fond physique
et surface évolutive ; l'affirmation « aucun domaine2D » expire pour les véhicules
de banc. Aucun solveur δ de production n'est choisi. A217 conserve exactement son
manque : aucun véhicule **à la fois non linéaire et dispersif** reçu. A216 reste
inexpliquée, forces et perception restent non reçues, A50 partielle.

La source S190/S191 n'est pas injectée ici. Ces erreurs Airy, normalisées par une
amplitude analytique, **ne s'ajoutent pas** au budget projeté1,374540 % : modèles,
champs de référence et conditions aux limites différents. Aucun montage couplé
complet n'a été déclaré sous2 %.

**S192-1 : construire les conditions de surface non linéaires dispersives et les
recevoir contre une référence de Stokes avec raffinement**, avant de brancher
B+W et de comparer perturbatif/total sous le critère ADR-120. Expliciter l'ordre
en amplitude retenu et son domaine de validité ; une superposition linéaire ne
fermera pas B4 (ADR-112). Il s'agit d'une prochaine tâche de construction, pas
d'une nouvelle campagne du réseau source. Aucun nouvel arbitrage demandé.
