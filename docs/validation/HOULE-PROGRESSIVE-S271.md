# Houle progressive — premier contrôle du résidu, S271

## Contrat avant mesure

Suite S270 : un courant uniforme ne reçoit pas une houle progressive. Avant une
campagne de durée, contrôler le résidu cinématique initial contre la dérivée
analytique indépendante d'ADR-152. Un fond linéaire n'est pas solution non linéaire :
η' ne doit pas rester nul. Le comparer à zéro masquerait la physique recherchée.

Fixture : h=1 m, longueur 4 m, k=π/2, a=0,125 m, g=9,81, ρ=1025, phase initiale
0,37 rad ; onde vers +x. θ=kx−ωt+phase, ω²=gk tanh(kh), q=agk/ω.
U=q cosh(k(z+h))/cosh(kh) cosθ ; W=q sinh(k(z+h))/cosh(kh) sinθ.
ζ=a cosθ, P=ρga cosh(k(z+h))/cosh(kh) cosθ. Dérivées exactes.

À v=η'=0, la cinématique perturbative continue exige
`η'_t = W(ζ) − W(0) − U(ζ) ζ_x`.
Cette expression locale sert d'oracle, sans réutiliser la quadrature des flux.
Contrôler d'abord incompressibilité, momentum linéaire, W(-h)=0 et ζ_t=W(0),
puis transport seul aux mailles 0,125, 0,0625, 0,03125 m. Norme L2 sur tout le
domaine, bords inclus, erreur relative <2 % à la maille fine et décroissante.
Bilan discret : télescopage vers les deux flux externes. Témoin supprimant ces
flux : doit manquer le seuil de 2 %, sinon la métrique ne distingue pas le défaut.

Pas réel : démarrage à v=η'=0, mêmes champs progressifs, dx=0,125, durées 1000 et
500 µs. Vérifier que le taux initial de hauteur tend vers le transport initial
lorsque dt baisse (la vitesse nouvellement projetée contribue à l'ordre dt).
Ce contrôle est une réception du démarrage, **pas une réception temporelle de la
houle traversante**. Arrêter au verdict de cet oracle avant de choisir la durée
et la référence de la campagne suivante. Aucun seuil modifié après mesure.


## Résultats

Trois essais Rust ciblés reçus, aucun échec. Formules du fond contrôlées sous et
au-dessus du niveau moyen, aux deux instants et jusqu'aux deux bords.

| dx (m) | erreur L2 initiale | témoin avec bande fermée |
|---|---:|---:|
| 0,125 | 3,0481 % | 168,8753 % |
| 0,0625 | 1,2370 % | 238,9358 % |
| 0,03125 | 0,5847 % | 337,7226 % |

Seuil final 2 % tenu, erreurs décroissantes. Ordre observé environ un sur les
dernières mailles : ne pas revendiquer l'ordre deux. La bande partiellement
mouillée et le raccord entre hauteur à la face externe et moyenne intérieure
restent des sources possibles ; aucune attribution fine nécessaire à ce reçu.
Le témoin ferme les seuls flux de bord dans le taux discret, les autres termes
restent identiques. Il n'est pas un second solveur indépendant.

Bilan global de taux : écarts de 1,14e-9, −5,68e-10 et −7,61e-10 m²/s au flux net
nul attendu sur une longueur d'onde. Ce montage périodique en valeurs est muet
sur un flux net non nul : le test asymétrique S270 porte cette contre-épreuve.

Le **pas réel** couplé avance aux deux durées : écart L2 non normalisé entre son
taux initial et le transport isolé 1,2025124e-3 à 1 ms, 6,0125284e-4 à 0,5 ms,
rapport 0,5000. Il tend donc bien vers le transport initial attendu ; ce résultat
n'est pas une borne d'erreur après plusieurs pas. Aucun code de production changé.

## Portée et suite

Reçus : fournisseur analytique de test, cinématique initiale spatiale et limite
de petit pas du chemin réel. **Houle progressive traversante sur durée utile non
reçue** : il manque une référence indépendante pour l'évolution non linéaire,
ainsi qu'un contrôle des retours aux bords sur cette durée. Prochain lot S272 :
référence temporelle de l'ordre deux et comparaison du résidu réellement évolué,
avec domaine/gardes et fenêtres déclarés avant campagne. Ne pas comparer η' à zéro
ni seulement vérifier la surface B imposée à chaque instant.

Les 478 tests reçus en S270 restent applicables au code produit inchangé ; ils
n'ont pas été rejoués. Les trois nouveaux tests s'exécutent explicitement ainsi :

```powershell
cargo test --release --offline --locked --manifest-path code/Cargo.toml -p water-core --lib s271 -- --nocapture
```

La liste du projet fini est actualisée sur S262–S271 : aucun périmètre final
supplémentaire n'est clos. Décompte recalculé : 119 points, 3 validés, 48 partiels,
68 absents. Ce n'est pas un pourcentage d'avancement.
