# ADR-129 — Le chemin d'image de W radial passe par une table de Bessel précalculée

- **Statut : actée**, S206, 2026-09-13, autonomie technique déléguée (S71).
- **Traite** la part technique d'A247 ; la part qui touche l'hôte, le profil et l'ambition est un
  arbitrage de l'utilisateur, posé dans la [feuille de route](../FEUILLE-DE-ROUTE.md) §4.
- **Retire** `paquets_W_max = 4096` d'ADR-012 §3 comme valeur de profil (I-16). ADR-012 n'est pas
  réécrit ; il reçoit une note datée.
- Mesures : [COUT-IMAGE-S206](../validation/COUT-IMAGE-S206.md). Construction : S207.

## Constat

Sur la scène représentative de J1 (mer S201, impact S203, grille projetée), l'impact coûte
**~9 µs par sommet** composé : chaque sommet réévalue N = 256 fonctions de Bessel. 86 % des
sommets tombent dans l'emprise. Or `J0(k_n r)` et `J1(k_n r)` ne dépendent pas du temps. Une
matrice précalculée par impact ramène le travail par image à N phases et N×M produits :

| pas | noyau par image | mémoire | erreur contre `sample` | par sommet (Hermite) |
|---:|---:|---:|---:|---:|
| λ/16 | 0,027 ms | 502 Ko | 0,0061 mm | 8,1 ns |
| λ/8 | 0,016 ms | 254 Ko | 0,090 mm | 8,8 ns |

Facteur **100** sur ce que coûtait la table de S203, sans erreur visible devant la tolérance de
3 mm. Avec ce chemin, W n'est plus le goulot de l'image ; B l'est.

## Décision

1. **Le chemin d'image de W radial est une table par impact** : matrice `(J0, J1)(k_n r_i)`
   calculée à la construction du champ, puis, par image, η(r_i) et η'(r_i) par N×M produits, et
   Hermite cubique par sommet. Construite dans la bibliothèque en S207, à côté de `sample`, sans
   le remplacer.
2. **Pas de table ≤ λ/8**, choisi par l'hôte selon sa tolérance ; λ/16 si la mémoire le permet.
   La taille de la matrice est une **allocation** (N×M×8 octets par impact) : un profil peut la
   déclarer (I-16), le pool la dimensionne au démarrage (I-06), rien n'alloue à l'image.
3. **Chemin cosmétique, jamais autoritaire.** La table ne reproduit pas `sample` au bit — ordre de
   sommation, interpolation — et n'est donc calculable à l'identique par personne (I-15). Toute
   grandeur de jeu reste servie par `sample` et la composition existante, déterministes (I-03).
4. **`paquets_W_max = 4096` cesse d'être une valeur de profil.** Mesuré : 109 ms et 2,0 Go par
   image à λ/16 pour 4 096 impacts, contre 2 ms et 384 Mo. Conformément à I-16, la capacité se
   **calcule** à l'initialisation à partir du coût par impact mesuré (16–27 µs par image) et de sa
   mémoire (254–502 Ko), dans le budget eau que le profil fixe — et elle se partage avec B.

## Réception attendue en S207

Valeurs aux nœuds égales à `sample` à l'ordre de sommation près (écart relatif borné et publié) ;
table contre `sample` ≤ 0,09 mm à λ/8 sur le champ S203, contrôlé sur toute l'emprise et
l'horizon ; coût par image conforme à la mesure du noyau ; **zéro allocation après construction** ;
refus explicites hors emprise et hors horizon, comme `sample` ; image S205 rendue par ce chemin
contre le chemin direct, écart de pixels borné et publié.

## Ce que cette décision ne fait pas

Elle **ne rend pas la scène compatible** avec 2 ms : B sur CPU coûte 42 ms à la densité qui montre
l'impact. Elle ne choisit ni l'hôte, ni le GPU, ni le sens du budget sur plusieurs cœurs, ni un
changement de profil : ce sont les arbitrages de l'utilisateur. Elle ne touche ni aux coutures
(ADR-126), ni au budget de pente (ADR-128), ni à la pression et aux sillages, qui auront chacun
leur mesure. Aucune fonctionnalité n'est retirée.
