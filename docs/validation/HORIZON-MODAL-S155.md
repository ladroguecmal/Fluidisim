# Horizon du noyau modal — S155, 2026-09-10

Sonde : `cargo run -p water-core --release --example horizon_modal` depuis `code/`.
Test de réception : `cargo test -p water-core horizon_observation`, debug et release.
Décision : [ADR-106](../adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md).

S154 laissait deux branches pour B2 : quantifier le domaine d'un sillage prolongé, ou isoler par
mesure le blocage numérique. La première est inaccessible tant que le noyau refuse la durée à
laquelle B2 mesure — soixante secondes contre seize. C'est donc la seconde qui est prise.

## 1. Ce que la lecture du code prédisait, et qui était faux

Après extinction, `sample` calcule la rotation libre par `phase(frequency, age - active, 1e6)` :
arithmétique entière en i128, réduite modulo un tour. Aucun flottant ne porte le temps. La seule
accumulation f32 dépendant du temps est `scale_integer(sinc/1e6, us)`, dont l'argument est borné
par la **durée active**, pas par l'âge.

Prédiction écrite avant la mesure : l'erreur serait plate en âge et croissante en durée active.

**Les deux moitiés sont fausses.** L'erreur croît en âge ; et sa croissance en durée active est
pour moitié une croissance du signal, pas de l'erreur.

## 2. L'âge n'est pas gratuit

Trente couples par ligne — cinq vecteurs d'onde, six rapports Doppler, 10 Pa, origine
(0,7 ; -0,3) —, durée active 4 s, contre l'oracle f64 `PressureMode`. Convention de S95 : le même
f32 de g converti en f64, pour mesurer l'implémentation et non la quantification d'entrée.

| âge | écart élévation (m) | écart vitesse (m/s) | \|eta\| (m) | relatif | relatif / seconde |
|---:|---:|---:|---:|---:|---:|
| 1 s | 2,359e-9 | 2,212e-8 | 4,686e-3 | 5,03e-7 | 5,0e-7 |
| 4 s | 3,398e-8 | 3,178e-7 | 1,875e-2 | 1,81e-6 | 4,5e-7 |
| 16 s | 1,381e-7 | 1,295e-6 | 1,873e-2 | 7,37e-6 | 4,6e-7 |
| 32 s | 2,539e-7 | 2,384e-6 | 1,868e-2 | 1,36e-5 | 4,2e-7 |
| 60 s | 5,931e-7 | 5,539e-6 | 1,863e-2 | 3,18e-5 | 5,3e-7 |
| 64 s | 6,177e-7 | 5,767e-6 | 1,864e-2 | 3,31e-5 | 5,2e-7 |

L'erreur relative rapportée au temps écoulé est **constante à 25 % près sur deux décades** :
4,2e-7 à 5,3e-7 par seconde. C'est une dérive de phase linéaire, pas une dégradation.

**Aucune rupture à seize secondes.** Rien dans ces nombres ne distingue 16 s de 15 ou de 17. En
revanche, 16 000 000 µs vaut 2^24 à un pour cent près, et le commentaire de `scale_integer` parle
de « 24 bits ». La borne venait de la représentation, pas du phénomène.

## 3. D'où vient la dérive

Le désaccord de pulsation entre candidat et oracle est un **fait exact**, pas une mesure :
`omega = (gravity * magnitude).sqrt()` est calculé en f32.

| k | omega (rad/s) | domega/omega | 16 s | 32 s | 64 s | prédit \|domega\|·64 | même omega, 64 s |
|---|---:|---:|---:|---:|---:|---:|---:|
| (6 ; 0) | 7,6720 | -2,145e-8 | 1,554e-6 | 3,627e-6 | 8,794e-6 | 1,053e-5 | **1,505e-7** |
| (1 ; 0) | 3,1321 | -6,579e-9 | 8,285e-7 | 1,233e-6 | 2,132e-6 | 1,319e-6 | 1,912e-6 |
| (9 ; 0) | 9,3963 | -5,733e-8 | 2,739e-5 | 3,602e-5 | 2,283e-5 | 3,447e-5 | 4,402e-6 |
| (0,6 ; 0,8) | 3,1321 | -1,850e-8 | 1,028e-5 | 1,084e-5 | 1,250e-5 | 3,708e-6 | 5,377e-6 |
| (0,0234 ; 0) | 0,4795 | -2,145e-8 | 2,796e-7 | 7,657e-8 | 2,036e-7 | 6,582e-7 | 1,214e-7 |

Écarts rapportés à l'amplitude invariante `sqrt(|eta|^2 + |v|^2/omega^2)`, que la propagation
libre conserve. La colonne « même omega » compare à un oracle dont la gravité est ajustée pour
porter **exactement** le omega f32 du candidat : `sample` n'emploie `gravity` qu'à travers omega,
donc c'est la seule différence neutralisée.

Pour k=(6 ; 0), l'écart croît d'un facteur 5,7 de 16 à 64 s, colle à la prédiction `|domega|·t` à
20 % près, et **tombe d'un facteur 58** quand l'oracle porte le même omega, devenant plat. La
cause est établie. Pour k=(9 ; 0) et (0,6 ; 0,8), neutraliser omega laisse 4,4e-6 et 5,4e-6,
**constants en temps** : un second terme, venant de la phase spatiale et de l'amplitude en f32.

Deux termes, donc, sans rapport l'un avec l'autre :

| terme | origine | dépendance au temps |
|---|---|---|
| phase spatiale, amplitude | `from_distance` par axe, `magnitude` f32 | aucune, présent dès t=0 |
| dérive de phase | **omega en f32** | linéaire, `\|domega\|·t` |

Ce n'est ni l'horizon, ni les 24 bits du doublement, ni la réduction Q32 : **c'est le type de
omega**. Enregistré en A213, avec un remède identifié et non appliqué.

## 4. Deux fois, un artefact de sonde a ressemblé à un résultat

Une ligne du premier tableau affichait `0,000000e0`. L'horizon demandé dépassait ce que le
constructeur acceptait, toutes les combinaisons étaient sautées, et **un écart nul sans
comparaison est indiscernable d'un résultat parfait**. La sonde compte désormais ses couples et
annonce d'abord l'horizon en vigueur.

Le premier dénominateur choisi, `|eta|` instantané, faisait exploser l'écart d'un facteur cent au
voisinage des nœuds de l'oscillation — 7,75e-5 pour k=(0,6 ; 0,8), qui n'était pas une perte de
précision mais un dénominateur proche de zéro. D'où l'amplitude invariante. **L232.**

## 5. Ce qui a été décidé et appliqué

ADR-106 sépare l'**horizon d'observation** de la **durée de forçage**, que « 16 s » confondait, et
porte le premier à 64 s. La durée active reste bornée à 16 s : ADR-104 découpe déjà le mouvement
de l'hôte en tronçons, chacun source distincte, donc un sillage de soixante secondes est fait de
tronçons courts qu'il faut pouvoir **observer** soixante secondes plus tard. B2 manquait
d'horizon, jamais de durée.

Trois constantes, pas deux : `modal_pressure`, `bound_pressure::Context::new`, et
`pressure_source::Source::new` qui en hérite par `Context::from_recipe`. **Celle-là, c'est un test
existant qui l'a trouvée**, en cessant de refuser ce qu'il refusait ; note datée ajoutée à ADR-106
le jour même.

Test témoin vérifié dans les deux sens : horizon ramené à 16 s, le test échoue à la construction
et non au seuil ; horizon rétabli, il passe. Le condensat de réception S95 `8ea15f4a3334830b` est
**inchangé** — seules des bornes de domaine ont bougé, aucune arithmétique.

## 6. Portée et suite

Ce document ne certifie pas un sillage à soixante secondes : il rend l'observation possible et en
annonce le prix. À 64 s, l'erreur relative reste sous 4e-5 sur les modes éprouvés, soit environ
7e-5 en énergie — **sous le seuil de 1e-4 E0 que B2 emploie depuis S153, sans marge confortable**,
et il faut le savoir avant de lire un bilan de sillage.

Les cinq vecteurs d'onde et les six rapports Doppler éprouvés ne sont pas une couverture de la
bande. La durée active longue est mesurée (2,8e-5 relatif à 64 s de forçage) mais **non reçue** :
aucun test ne la garde, et ADR-074 comme le format WPRS la bornent ailleurs.

**S155-1, prochaine S156 : la première branche de S154**, désormais accessible — bilan énergétique
d'un sillage prolongé et domaine de collecte requis, à comparer à ce que S153 et S154 ont établi
pour les impacts. Restent ouverts : la correction de la dérive (A213), un seuil de régression
relatif plutôt qu'absolu, la coupure W/δ, lambda_cut et la conformité multiplateforme.

297 tests réussis, cinq ignorés. 106 ADR, 213 angles, 232 leçons, 18 invariants, 6 SPEC, 23 cas.
