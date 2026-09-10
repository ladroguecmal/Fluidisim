# S162 — A217 : superposition indépendante et résidu couplé

2026-09-10. Instruction de la suite S161. Travail sur master, sans nouvelle copie.

## 1. Blocage vérifié

Inventaire des modules déclarés dans `water-core/src/lib.rs`, puis lecture des équations :

| Instrument | Ce qu'il calcule | Limite pour A217 |
|---|---|---|
| `shallow.rs`, `delta.rs` | Saint-Venant non linéaire | non dispersif |
| `dispersif.rs` | oscillateurs à `ω²=gk tanh(kh)` | linéaire, aucune interaction |
| `background.rs` | composantes analytiques | fond à comparer, pas référence indépendante |
| impacts et pressions | modes propagés, superposition des sources | pas d'évolution non linéaire du champ total |

Le dépôt ne possède pas de référence évolutive non linéaire dispersive. Ajouter une dispersion
à Saint-Venant ne se réduit pas à changer sa célérité : cela change les équations et leur réception.
La FFT de `dispersif.rs` est privée et ne constitue pas à elle seule un solveur non linéaire.

Une voie analytique existe néanmoins : la famille de Stokes au second ordre en profondeur
infinie. Elle peut établir un terme d'interaction et sa dépendance à la longueur d'onde.
Elle ne simule pas deux états initiaux arbitraires et ne remplace donc pas la référence B4.

## 2. La comparaison doit porter sur le bon objet

ADR-001 §2 définit δ comme **l'écart au fond** ; SPEC-004 §6.1 décrit un terme source issu du
résidu du fond. Or `additivite_b4.rs` compare trois simulations autonomes, sans couplage :
`F(A+B) - F(A) - F(B)`. Cela mesure le défaut de superposition des évolutions indépendantes.
Ce n'est pas l'erreur d'un solveur du résidu `δ = total - fond`.

Pour une équation abstraite `U_t=N(U)`, poser `U=Q+d` donne exactement
`d_t=N(Q+d)-Q_t`. Même si `N` est non linéaire, la reconstruction `Q+d` n'exige aucune
superposition des solutions. L'approximation se situe dans l'équation effectivement retenue
pour `d`, dans ses sources, ses frontières et sa discrétisation. Cette identité n'est pas une
réception du couplage : elle localise ce qui doit être testé.

**Conséquence de lecture :** les chiffres S161 restent des mesures du montage autonome.
Ils ne suffisent pas à choisir une variable de bascule du solveur perturbatif prévu.
Le contrôle A50 demandé par B4 est précisément pertinent ici.

## 3. Expérience déclarée avant calcul

Comparer la somme de deux profils de Stokes au profil construit depuis leur fondamentale
combinée, à même nombre d'onde et même direction. Balayer amplitude, longueur d'onde et phase.
Vérifier séparément les conditions cinématique et dynamique à la surface réelle : le résidu
du profil d'ordre deux doit décroître d'un ordre supplémentaire par rapport au profil privé
de son harmonique deux. Ne pas appeler cette vérification une évolution à conditions initiales égales.

Référence de formules : [Principia, DeepLines, théorie potentielle et Stokes d'ordre deux](https://www.principia-support.com/Software/Deeplines/Theory/Hydrodynamics/Hydrodynamics_of_offshore_floating_structures/#stokes-2nd-order-waves),
consultée le 2026-09-10. Seules les équations aux limites et leur limite en profondeur infinie
sont utilisées ; aucune réception ni borne d'erreur n'est empruntée à cette documentation.

## 4. Calcul et mesure

Eau inviscide, irrotationnelle, profondeur **infinie**, gravité sans capillarité. À petite
cambrure `ka`, poser `θ=kx-ωt`, `ω²=gk`, une amplitude complexe `C`, puis
`η(C)=Re(C exp(iθ)) + (k/2) Re(C² exp(2iθ))`.
Le potentiel à cet ordre est `φ=(ω/k) exp(kz) Im(C exp(iθ))`.
Ces expressions sont asymptotiques ; leur troncature n'est pas une solution exacte à amplitude finie.
Formules également portées dans SPEC-001 §1 ter pour I-14.

Notre calcul : si `A=a` et `D=b exp(iϕ)`, la fondamentale combinée est `C=A+D` et

```
η(A+D) - η(A) - η(D) = k a b cos(2θ+ϕ)
RMS(différence) = k a b / sqrt(2)
E_total = k a b / [|A+D| sqrt(1 + k²|A+D|²/4)]
```

La dernière égalité vaut si `|A+D|>0`. Le dénominateur vient de l'orthogonalité des deux
harmoniques sur une période complète. En phase, à rapport `r=b/a` fixé,
`E_total = (ka) r/(1+r) / sqrt(1+(ka)²(1+r)²/4)`.
**La cambrure intervient déjà à l'ordre deux**, sans profondeur dans la formule.
Mais le rapport d'amplitudes et la phase ne disparaissent pas.

Sonde : `code/water-core/examples/stokes_additivite.rs`, exécution release du 2026-09-10.
Quadrature uniforme décalée d'un quart de cellule ; RMS contrôlée contre l'expression fermée
sur 27 montages et 32/64/128 points, erreur relative <1e-11. C'est une réception algébrique,
pas 81 expériences physiques indépendantes.

| k (m⁻¹) | a (m) | b/a | phase | E_total |
|---:|---:|---:|---:|---:|
| 0,25 | 0,02 | 0,35 | 0 | 0,129629 % |
| 1 | 0,02 | 0,35 | 0 | 0,518471 % |
| 4 | 0,02 | 0,35 | 0 | 2,071057 % |
| 4 | 0,005 | 0,35 | 0 | 0,518471 % |
| 1 | 0,02 | 1 | 0 | 0,999800 % |
| 1 | 0,02 | 1 | π/2 | 1,414072 % |
| 1 | 0,02 | 1 | π | **indéfini** |

Les trois premières lignes gardent `b/a` et changent la cambrure : l'écart varie d'un facteur
proche de seize. Les lignes deux et quatre gardent `ka` : même écart relatif, dimensions différentes.
Les trois dernières gardent les amplitudes : la RMS absolue reste **2,82842712e-4 m** alors que
le rapport au champ total change, puis n'existe plus quand les fondamentales s'annulent.
La sonde publie aussi `RMS/(a+b)`, fini dans ce cas. Aucun zéro de dénominateur n'est remplacé
par une valeur supposée valide ; le seuil 1e-12 sert seulement à identifier l'annulation en f64.

### Vérification physique distincte

Évaluer **sans troncature**, à `z=η`, les résidus des deux conditions aux limites :
`K=η_t+φ_x η_x-φ_z`, `D=φ_t+(φ_x²+φ_z²)/2+gη`.
La sonde dérive explicitement le profil et le potentiel, puis normalise par `aω` et `ga`.
À `k=1`, `g=9,81`, 2048 points :

| ka | maximum K normalisé | maximum D normalisé | K sans harmonique 2 | D sans harmonique 2 |
|---:|---:|---:|---:|---:|
| 0,04 | 2,39096e-3 | 9,37679e-4 | 4,08616e-2 | 2,07482e-2 |
| 0,02 | 5,89069e-4 | 2,32623e-4 | 2,02138e-2 | 1,01934e-2 |
| 0,01 | 1,46204e-4 | 5,79420e-5 | 1,00532e-2 | 5,04917e-3 |
| 0,005 | 3,64195e-5 | 1,44594e-5 | 5,01328e-3 | 2,51239e-3 |

Diviser l'amplitude par deux divise les résidus normalisés par environ quatre ; retirer
l'harmonique ne les divise plus que par deux. Le test exige ces deux comportements et la
séparation entre témoin et contre-épreuve. Le résidu dimensionnel est donc d'ordre trois
pour le profil d'ordre deux. **Un petit résidu aux limites ne borne pas l'erreur d'évolution.**

## 5. Verdict et prochain travail

**A217 partielle** : une référence analytique faiblement non linéaire est à portée et instrumentée ;
elle établit le terme croisé et la dépendance à `ka` dans cette famille. La référence évolutive
générale reste absente. Aucun seuil de bascule, aucune loi universelle en cambrure, aucune
réception perceptuelle ne sortent de cette sonde.

Un profil combiné de Stokes contient déjà le terme croisé à l'instant initial : comparer ces
profils ne remplace pas une comparaison de trajectoires partant du même état. Les deux
constituants ont ici la même longueur d'onde ; ils ne représentent pas la séparation d'échelles
B/W/δ. Le résultat est un instrument analytique et un contrôle de raisonnement.

La priorité suivante est **S162-1 : recevoir un résidu effectivement couplé**, d'abord dans
Saint-Venant où la référence existe. Définir l'état conservatif total, le fond et les termes
croisés ; comparer leur évolution à état initial et pas identiques, puis retirer volontairement
un terme couplé. Ne pas construire `d` par soustraction a posteriori pour prétendre avoir testé
son intégration. Ce lot exercera la distinction révélée ici avant d'écrire un nouveau solveur
dispersif. A216 reste ouverte ; sa constante n'est pas un seuil du couplage.


Réception finale : 299 tests workspace réussis, cinq ignorés ; un test d'exemple reçu
séparément en debug et assertions de la sonde reçues en release. Aucun calcul de production modifié.
