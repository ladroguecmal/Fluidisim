# S113 — Réception indépendante du champ multisource

2026-09-09. Prolonge [MULTISOURCE-S112](MULTISOURCE-S112.md), sans modifier la bibliothèque.

## Montage et référence

Deux sources gaussiennes, sigma1 m, coupure6 rad/m, g=9,81f32, rho1025 :
10 Pa depuis(0,0), vitesse(2,0), active[0,2 s) ; 7 Pa depuis(1,1), vitesse(0,2),
active de0,5 s inclus à2,5 s exclu. Chaque source a un segment ; contexte frame7/cell9,
rectangle[-8,12]² m, horizon0–8 s. Le journal ordonne les sources par id.

`receive_multisource` construit un oracle plein en f64, avec rayons et angles aux
milieux, transformée gaussienne par exp et phases par sin/cos de la bibliothèque
mathématique. Chaque mode appelle la réponse analytique f64 `PressureMode` (S89),
indépendante du candidat Q32. Les réponses et pressions des deux sources sont
additionnées **avant** les bilans : E=rho/2 Σpoids(g|q|²+|qdot|²/k),
P=-Σpoids Re(Ptotal conj(qdot)). Les sept champs sont reconstruits directement.
Aucun noeud, phase, poids ni bilan candidat n'est réutilisé dans cet oracle.
Il partage le modèle physique analytique : cette indépendance numérique ne vaut
pas validation physique externe ni oracle exact du continuum.

La campagne compare deux quadratures complètes256² et512². Le mode normal examine
neuf recettes sur121 points de pas2 m ; `--dense` examine112×80,128²,192×128,224×128 et256×128
sur441 points de pas1 m. Dans les deux cas :23 instants, pas0,5 s entre0 et8 s,
plus ±1 µs autour de0,5/2/2,5 s. Soit10143 points-temps par candidat dense.

Critères absolus de fixture S103 : eta1e-6 m, w1e-5 m/s, potentiel1e-5 m²/s,
chaque pente1e-6, chaque vitesse horizontale1e-5 m/s, énergie1e-5 J.
S113 étend explicitement le seuil de puissance1e-7 W de S104 à la comparaison
entre résolutions. **À calibrer pour un usage jeu** par une campagne multiprofils ;
ce seuil de banc ne devient pas une contrainte universelle du système.
Tous les critères doivent passer contre les deux références ; leur propre écart
doit aussi rester sous les seuils. Cela ne borne pas leur erreur commune.

## Pourquoi raffiner aussi la référence

Le premier balayage128²/256² échoue partout sur la puissance : les deux références
diffèrent déjà de5,8441e-7 W, au-delà du seuil1e-7 W. Aucune réponse ne peut être à
moins de1e-7 W des deux simultanément. Comparer exclusivement à128² aurait au
contraire reçu128² sur ce critère avec8,186e-9 W, en masquant son erreur de quadrature.
Les références sont donc raffinées, sans relâcher le critère. L'essai initial est
un diagnostic, pas une réception échouée du seul noyau candidat.

## Résultats et coût

Écarts maxima entre les références256²/512² : eta3,3504e-8 m, w1,2859e-8 m/s,
potentiel5,7139e-7 m²/s, pente5,1065e-11, vitesse horizontale5,9269e-10 m/s,
énergie2,8262e-8 J, puissance3,6446e-8 W. Tous sont sous les seuils de fixture.

| Recette | Diagnostic contre512² |
|---|---|
|16×24|Refus : erreurs spatiales de plusieurs ordres de grandeur au-dessus des seuils ; le bilan discret S112 ne recevait pas ce champ.|
|64×128 et96×128|Refus radial : potentiel supérieur à1e-5 m²/s.|
|128×64|Refus angulaire : pente maximale supérieure à3,4e-6.|
|112×80|Refus : potentiel1,12809e-5 m²/s et puissance1,06389e-6 W.|
|128×128|Champs et énergie passent ; puissance6,16853e-7 W, refus global.|
|160×128|Puissance2,48050e-7 W, refus global.|
|192×128|Puissance1,17664e-7 W, refus global malgré8,12186e-8 W contre256².|
|224×128|Reçu : neuf critères passent contre les deux références, en grille dense.|
|256×128|Reçu : neuf critères passent contre les deux références, en grille dense.|

Les recettes refusées restent dans le banc comme diagnostics. Les témoins224×128/256×128
reçu et112×80 refusé sont vérifiés par assertion, ainsi que la convergence des références.
La résolution224×128 est instruite directement en mode dense après le balayage.

Écarts maxima densifiés contre512² (les neuf composantes sont contrôlées séparément
dans le programme ; seules les deux pentes et les deux vitesses sont regroupées ici) :

| Grandeur |224×128|256×128|
|---|---:|---:|
|eta m|5,8912e-8|3,5782e-8|
|w m/s|3,9599e-8|6,6183e-8|
|potentiel m²/s|8,2507e-7|5,8436e-7|
|pente|1,1652e-8|1,5515e-8|
|vitesse horizontale m/s|3,5027e-8|3,7465e-8|
|énergie J|4,8779e-8|2,5992e-8|
|puissance W|6,1786e-8|3,5708e-8|

Le critère le plus proche de sa limite en224×128 est la puissance, marge mesurée38,2 %.
Cette marge est relative aux références échantillonnées, pas à une erreur continue certifiée.

| Recette |Modes réduits|Minimum ms|Médiane ms|Maximum ms|
|---|---:|---:|---:|---:|
|224×128|14336|47,1782|48,2895|51,2997|
|256×128|16384|52,9101|56,1288|62,7986|

Chronométrie release à1,5 s (deux sources actives), trois échauffements et21 mesures
successives par recette. Le lot est toujours le préfixe64 de la grille11² de pas2 m,
même en mode dense. Chaque mesure comprend `Prepared::from_journal` puis `sample_batch`
transactionnel ; les sorties et les bilans sont consommés par `black_box`.
Ce sont des latences locales en mémoire chaude, pas des quantiles de production.
Windows, AMD Ryzen AI 7 350, rustc1.97.0 (2d8144b78) ; séries successives, sans
autre campagne de calcul lancée par la session pendant la mesure. La première série
dense utilisait un lot géométriquement différent ; seule la série finale ci-dessus
est retenue. Le balayage normal a aussi terminé avec ses assertions.

Pour256×128 :16384 modes,655360 octets par pool de champ ; deux publications
demanderaient1310720 octets de pools. Les noeuds pleins prennent524288 octets,
les réduits262144 ; le lot64 prend512 octets de positions et3584 de sortie+brouillon.
Métadonnées, journal, allocations de l'instrument et pile ne sont pas inclus dans ces sommes.

Bibliothèque inchangée : la suite240 réussis/cinq ignorés reste celle exécutée en S112.
La validation S113 est la campagne release avec assertions ; aucun nouveau test unitaire.

Depuis la racine du dépôt :

```text
cargo run --release --manifest-path code/Cargo.toml -p water-core --example receive_multisource
cargo run --release --manifest-path code/Cargo.toml -p water-core --example receive_multisource -- --dense
```

## Limites et suite

Réception échantillonnée de ce seul montage ; pas de borne entre points/instants,
pas de preuve de résolution minimale, de conformité interplateforme, de coût cible,
ni changement de recette par défaut. La cuisson et les codecs sont hors chronométrie.
Le coût couvre la préparation des deux sources et la requête locale de pression ;
il exclut B, conversions monde, sauvegarde/restauration et admission du journal.
Les pools sont préalloués ; aucun compteur global d'allocations n'est revendiqué.

**S113-1, S114 :** recevoir la reprise WPJR d'un journal à deux sources, attente et
retry compris, jusqu'à B+pression aux résolutions reçues ici. Comparer les dix sorties
et bilans en bits à la construction directe ; mesurer le cycle complet et les refus.
Le raccordement aux impacts, le milieu variable et la durabilité restent ouverts.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.

**Mise à jour S114, 2026-09-09 :** S113-1 réalisée sur fixture, voir
[REPRISE-MULTISOURCE-S114](REPRISE-MULTISOURCE-S114.md). Reprise de deux sources jusqu’à
B+pression,2944 points-temps identiques en bits et refus reçus ; cycle depuis WPRS
47,95/55,55 ms médians locaux, cuisson exclue. Suite S114-1 : requête mixte impacts/pressions.
