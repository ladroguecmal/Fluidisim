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
