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
