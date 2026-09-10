# Domaine d'un sillage prolongé — S156, 2026-09-10

Sondes, depuis `code/` : `cargo run -p water-core --release --example wake_long` (bilan et prix),
`cargo run -p water-core --release --example wake_reach` (portée et attribution).
Test de réception : `cargo test -p water-core recurrence_radiale`, debug et release.
Décision : [ADR-107](../adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md).

S155 a porté l'horizon d'observation à 64 s pour ouvrir la branche que S154 proposait en premier :
le bilan d'un sillage prolongé. Elle est ouverte. Ce qu'on y trouve n'est pas ce qu'on y cherchait.

Fixture : forçage de 16 s en huit tronçons de 2 s à 2 m/s, 100 N, sigma 1 m, cutoff 6, observation
jusqu'à 60 s.

## 1. Le bilan énergétique se conserve, et c'est un résultat vide

Puissance exactement nulle dès l'extinction. Énergie **identique au bit près** à 16, 20, 30, 45 et
60 s : 2,882884145e-1 J en 128×128, 2,884232700e-1 J en 256×256 — les deux quadratures ne
diffèrent que de 4,7e-4 relatif sur le total.

Il ne faut pas en tirer de fierté. **Après extinction chaque mode tourne, et la rotation laisse
`g|eta|² + |v|²/k` invariant** : le bilan spectral ne pouvait pas ne pas se conserver. La mesure
confirme l'implémentation et **ne dit rien de la validité spatiale du champ**. Publier cette
conservation comme un certificat de sillage prolongé aurait été une mesure vide.

La validité spatiale se mesure autrement, et elle est mauvaise.

## 2. Deux bornes indépendantes, deux lois différentes

Élévation efficace sur des cercles de rayon croissant, 32 directions, à résolution variable. Le
critère d'accord rapporte l'écart au **maximum du profil de référence** à cet instant, jamais à la
valeur locale : un dénominateur qui s'annule fait exploser la mesure sans qu'aucune précision ne
soit perdue (L232).

### Borne angulaire — le rayon

À 8 s, radial fixé à 512 :

| angulaire | rayon honnête à 10 % | attendu si proportionnel |
|---:|---:|---:|
| 64 | 20 m | 22 m |
| 128 | 45 m | 45 m |
| 256 | au-delà de 200 m | 90 m |

Le rayon honnête est **proportionnel à `angular`**, comme l'exige l'échantillonnage de
`exp(i k·x)` sur le cercle.

### Borne radiale — la durée

Le pas radial `dk = cutoff/radial` rend le champ **périodique**, de période `2π·radial/cutoff` :
67, 134, 268, 536 m pour radial 64, 128, 256, 512. À 4 s, deux résolutions voisines cessent de
s'accorder à 45 m (64 contre 128) et 100 m (128 contre 256) — **les deux tiers de la période de la
plus grossière** dans les deux cas. Le paquet ne part pas : il revient par l'autre bord.

Angulaire fixé à 512, rayon d'accord à 10 % :

| instant | 64 contre 128 | 128 contre 256 | 256 contre 512 |
|---:|---:|---:|---:|
| 4 s | 45 m | 100 m | > 200 m |
| 8 s | 45 m | > 200 m | > 200 m |
| 15 s | > 200 m | > 200 m | > 200 m |
| 20 s | aucun | aucun | > 200 m |
| 40 s | aucun | aucun | > 200 m |
| 45 s | aucun | 2 m | 160 m |
| 50 s | aucun | aucun | aucun |
| 60 s | aucun | aucun | aucun |

**Radial 128 décroche entre 15 et 20 s ; radial 256 entre 45 et 50 s.**

La récurrence prédite `2π/(c_g,max·dk)` donne 13,1 s et 18,5 s : juste à 30 % pour 128, trop
pessimiste d'un facteur 2,5 pour 256. La formule prend la vitesse de groupe au plus petit nœud du
maillage, où presque aucune énergie ne vit. **La loi n'est donc pas vérifiée, et ce document ne la
publie pas** — seuls les deux encadrements sont mesurés.

### Laquelle mord dépend de l'instant

C'est ce qui n'était pas anticipé. Les deux bornes sont indépendantes, et l'ordre s'inverse.

| instant | 512 radial, 128 angulaire | 128 radial, 512 angulaire |
|---:|---|---|
| 8 s | honnête à **45 m** | honnête **au-delà de 200 m** |
| 60 s | reproduit 512×512 au bit près en champ proche | se trompe d'un **facteur 75** à 2 m |

Une seule des deux résolutions suffit à ruiner le champ, mais pas la même selon la date. Une borne
unique, écrite une fois pour toutes, serait fausse dans un régime ou dans l'autre.

## 3. Un point de logique, et la limite de ce qu'on peut savoir

Quand radial 256 et 512 divergent à 50 s, cela accuse **256** : les deux s'accordent en dessous de
45 s, et 128 échoue plus tôt. Mais `radial` et `angular` plafonnent tous deux à **512** dans la
grammaire de recette (`validate_recipe`, ADR-097). **La durée honnête de 512 n'est donc pas
mesurable** : il n'existe aucune référence plus fine dans le corpus. L'extrapolation donnerait
plus de cent secondes ; ce serait une extrapolation, pas une mesure.

C'est aussi pourquoi un oracle ne sauverait rien ici. `GaussianPressure`, la référence f64 de
S150, porte **la même discrétisation** en plus fin : les deux replient. Seule la comparaison entre
deux résolutions dit quelque chose, et elle s'arrête au plafond.

## 4. Le prix de ce qu'il faudrait

Médianes sur cent répétitions, après un bloc de chauffe séparé (A195), en release.

| recette | nœuds du demi-spectre | préparation p50 | lot 64 points p50 |
|---|---:|---:|---:|
| 128×128 | 8 192 | 25,6 ms | 19,5 ms |
| 256×256 | 32 768 | 102,0 ms | 75,0 ms |
| 512×256 | 65 536 | 205,5 ms | 150,4 ms |
| 512×512 | 131 072 | 402,9 ms | 298,3 ms |

Coût linéaire en nombre de nœuds. Passer de la recette reçue à celle qu'exigeraient 60 s multiplie
le prix par huit, et le point de départ est déjà de 19,5 ms pour soixante-quatre points.

Machine de développement, chemin scalaire : ce n'est pas un budget matériel cible. L'écart au
raisonnable n'est cependant pas d'un facteur deux, et cela suffit à la décision.

## 5. Ce qui est décidé

ADR-107 : le domaine de validité d'un sillage est un couple `(rayon, durée)` **déduit de sa
recette**, publié avec elle, et non une constante. Le plafond de 512 n'est pas relevé — il n'y
aurait aucune référence pour vérifier, et le prix de ce qu'il faudrait est déjà hors de portée.

**Le volet sillage de B2 reçoit un verdict partiel et négatif à 60 s**, fondé sur une mesure et
non sur un manque de mesure. Aux durées où le candidat a été reçu — 8 s en S150 et S151 — il est
dans son domaine, et ces réceptions restent valides.

Un test épingle le fait, pas la loi : à 8 s radial 128 et 512 s'accordent à 2,4 % près du centre ;
à 60 s la grossière y montre 6,406e-5 m contre 1,322e-6, quarante-huit fois plus. Témoin vérifié —
les deux mesures prises à la même résolution, le test échoue.

## 6. Portée et suite

Une seule fixture, un seul sigma, un seul cutoff. Les lois d'échelle sont mesurées sur cette
fixture ; rien ne dit qu'un sigma de 4 m les laisse inchangées, et cela se mesure.

Aucun garde-fou n'est ajouté : refuser à l'exécution un échantillonnage hors domaine serait
conforme à l'habitude du dépôt (ADR-091), mais la loi en durée n'est **encadrée qu'en deux
points**, et un garde bâti sur une loi non vérifiée refuserait du valide ou admettrait de
l'invalide. **A214.**

Deux remèdes sont nommés et non mesurés : une quadrature non uniforme en k, plus dense près de
zéro où vivent les modes rapides, déplacerait la borne sans multiplier les nœuds ; une fenêtre
spatiale amortissant le retour par l'autre bord aussi. Les nommer n'est pas les recevoir.

**S156-1, prochaine S157 : établir la loi en durée** au lieu de l'encadrer — trois ou quatre
points de plus par dichotomie sur l'instant de décrochage, à radial 64, 128 et 256, et la
dépendance à `sigma`. C'est ce qui manque pour qu'A214 devienne un garde-fou plutôt qu'un doute,
et c'est peu cher : la sonde existe.

298 tests réussis, cinq ignorés. 107 ADR, 214 angles, 234 leçons, 18 invariants, 6 SPEC, 23 cas.
