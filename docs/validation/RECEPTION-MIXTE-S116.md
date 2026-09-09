# S116 — Réception échantillonnée et coût du montage mixte

2026-09-09. Prolonge [MIXTE-S115](MIXTE-S115.md), bibliothèque inchangée.

## Montage et portée de la référence

`receive_mixed` reprend B16 de S114/S115 : Hs0,1 m, Tp6 s, direction0,125 tour,
graine42, ancre monde(10⁹,-10⁹,0) m. Un impact0,01 J, longueur d'onde4 m,
naissance0, TTL4 s,64 modes radiaux, rayon16 m, horizon4 s, profondeur déclarée20 m.
Deux pressions du montage S112/S113 :10 Pa active0–2 s depuis(0,0) à(2,0) m/s ;
7 Pa active0,5–2,5 s depuis(1,1) à(0,2) m/s. Sigma1 m, coupure6 rad/m,
g9,81f32, rho1025, frame7/cell9. Recettes224×128 et256×128.

Grille17² de pas1 m, rectangle[-8,8]², entièrement dans le rayon de l'impact et
l'emprise de pression. Quinze instants :0–4 s par pas0,5 s et ±1 µs autour de
0,5/2/2,5 s.4335 points-temps par recette,8670 au total. Les64 points chronométrés
restent le préfixe de la grille11² de pas2 m de S113/S114, dans le domaine commun.

L'oracle de pression reprend explicitement l'instrument f64 plein de S113, aux
résolutions256² et512² : phases sin/cos, transformée exp, réponses analytiques S89,
aucun noeud ou coefficient candidat réutilisé. Une copie locale à cet exemple
conserve les paramètres de cette fixture ; aucune nouvelle API de production.
L'écart des deux références est contrôlé sur les sept composantes avec les seuils S103.

B et l'impact sont évalués séparément par leurs implémentations existantes, puis
composés avec la pression de référence **en f64** : hauteur, vitesses et pentes,
normalisation finale indépendante. L'enveloppe utilise les bornes candidates, car
elle est une donnée du contrat, pas la pente réelle. L'aération doit être identique.
Ce montage reçoit la pression et l'assemblage ; il **ne reçoit pas indépendamment
le modèle de B ni celui de l'impact**. Leurs réceptions antérieures restent nécessaires.

Seuils absolus de fixture : hauteur1e-6 m, dérivée/vitesses1e-5 m/s, composantes de
normale1e-6, enveloppe publiée1e-7, aération strictement identique. Les seuils de
champ prolongent S103, les contrôles d'assemblage S115 ; à calibrer pour le jeu.
Aucun bilan énergétique mixte n'est calculé par addition des bilans séparés.

## Correction physique de S115

L'affirmation « impacts à dispersion de profondeur finie » dans ADR-077 est fausse.
`RadialImpact::new` construit omega=sqrt(g*k). Le paramètre depth intervient dans
le refus depth<=pi/k_min ; il ne modifie pas omega. ADR-060 le décrit déjà comme
un candidat profond. La pression est elle aussi profonde : le prétendu conflit
entre deux lois de dispersion n'existe pas dans ces implémentations.

Cela ne certifie pas l'eau de profondeur20 m pour tout le spectre gaussien de pression :
ses petits k demanderaient une comparaison avec une référence de profondeur finie.
Le présent banc reçoit leur **modèle profond commun**, pas cette approximation sur
tous les milieux réels. Une note datée corrige ADR-077 sans réécrire sa décision.

## Résultats et coût

Campagne finale release reçue avec assertions. Écarts des références256²/512² :
hauteur2,833e-9 m, vitesse verticale2,764e-9 m/s, potentiel5,706e-7 m²/s,
pente4,000e-12, vitesse horizontale3,278e-10 m/s ; sous les seuils S103.

Écarts maximaux du mélange contre la référence de pression512² et composition f64 :

| Grandeur |224×128|256×128|
|---|---:|---:|
|Hauteur m|1,0051e-8|1,1844e-8|
|Dérivée/vitesse verticale m/s|3,9657e-8|6,5717e-8|
|Vitesse horizontale m/s|3,5027e-8|3,7232e-8|
|Composante de normale|1,1580e-7|1,1580e-7|
|Enveloppe publiée|4,195e-10|4,475e-10|
|Aération|0|0|

Toutes les dix composantes passent aussi contre256². Hash des dix sorties des4335
points-temps :957dc8b9608790cf et20f9a748a6978775. Les empreintes sont des diagnostics,
les assertions portent sur toutes les valeurs finies et leurs écarts.

| Étape, médiane locale |224×128|256×128|
|---|---:|---:|
|Préparation des pressions|12,7406 ms|14,1212 ms|
|Requête mixte64|38,4573 ms|42,4006 ms|
|Préparation impact+pressions et requête64|50,0497 ms|57,9712 ms|

Cycle complet min/max :47,4981/52,3537 ms et54,4666/60,5831 ms. Machine Windows
AMD Ryzen AI 7 350, rustc1.97.0, comme S113. Seule la campagne finale est retenue.
Bibliothèque inchangée : suite243 réussis/cinq ignorés exécutée en S115, non relancée.
Formatage et diff vérifiés. Aucune borne continue ni validation de tous les milieux.

Les refus sont exécutés avec une première position valide suivie de(13,0), hors
pression mais dans le rayon, puis(12,12), dans le rectangle de pression mais hors
rayon. La sortie entière reste intacte. À4 s+1 µs la pression est préparée avec succès,
mais la requête mixte refuse l'impact expiré même sur un lot vide.

Trois échauffements et21 mesures release par étape à1,5 s : préparation pression,
requête mixte64 sur champs préparés, puis préparation de l'impact et des deux
pressions suivie de la requête64. La sortie finale du cycle complet est comparée
à celle de la requête seule. Configuration de B, admission, cuisson et allocations
des grands pools exclues ; préparation radiale et préparation pression incluses
dans le cycle complet. Aucun codec/disque/réseau. Séries successives, pas de budget garanti.

```text
cargo run --release --manifest-path code/Cargo.toml -p water-core --example receive_mixed
```

## Suite

**S116-1, S117 :** construire un contrôleur de publication du champ de pression sur
deux pools, avec instant exact publié, préparation candidate puis bascule seulement
sur succès, et état explicitement indisponible pour une requête à un autre instant.
Recevoir l'avancement temporel et les refus sans perdre la dernière publication,
puis exercer la requête mixte sur les vues fournies par ce contrôleur. La vue S105
permet ce mécanisme mais l'hôte en écrit encore lui-même le cycle.
Profondeur finie de pression, bilan mixte et durabilité restent ouverts.
77 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.
