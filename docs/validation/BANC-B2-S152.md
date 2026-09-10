# Banc B2 — premier volet exécuté S152, 2026-09-10

**Verdict partiel :** profils du candidat radial CPU reçus sur un domaine80m/60s.
ADR-105 ajoute N512 explicitement. Pas de sélection de technologie ni de lambda_cut.
Exécution depuis code/ : `cargo run -p water-core --release --example banc_b2`,
puis `cargo run -p water-core --release --example banc_b2 -- --cost-only`.
`--admission-only` permet de lire les refus sans lancer les oracles.

## Domaine et qualité annoncés avant mesure

Impact0,01J, g9,81/rho1025, profondeur20m, source2/3/4/5/6m ; disque80m,
17 rayons (centre,15 irréguliers,frontière), temps0/1/10/30/45/60s.
510 points-temps de référence ; sept composantes de surface, normalisées par leurs
échelles spectrales S126. Seuil commun1e-4. Oracle indépendant en f64/libm,
intégrale angulaire de Bessel, radial512/1024 et angulaire1024/2048 séparément.
Variation maximale radiale1,365063e-9, angulaire3,899869e-16 ; seuil1e-6.
Aucune valeur assouplie après mesure. Réception échantillonnée, pas borne continue.

| Longueur source m | N64/N128 | N256 | erreur N256 | erreur N512 | hash N512 |
|---:|---|---|---:|---:|---|
| 2 | refus résolution | refus | — | 1,341601e-6 | aa9b18494b130ddd |
| 3 | refus résolution | refus | — | 9,059439e-7 | 85b6c72126062487 |
| 4 | refus résolution | refus | — | 9,896844e-7 | 5b049763198377fd |
| 5 | refus résolution | reçu | 5,384762e-7 | 5,625456e-7 | b7938dee882460d1 |
| 6 | refus résolution | reçu | 4,109890e-7 | 9,062238e-7 | ee13437cd1232bee |

N512 reçu partout. N256 hashes5/6m : de0d39fd5196ca34,7ae7113b1c1ba371.
La convergence du champ complet empêche la dispersion analytique exacte de masquer
une sous-résolution de l'intégrale. Ce n'est pas le volet iso-célérité complet du dossier.

## Coût local séparé de l'oracle

Windows x86_64, cargo1.97.0 release ; aucun test lourd simultané. Échauffement20 lots,
cinq répétitions de101 lots de64 points à30s ; p50/p99 par répétition, indices50/99.
Table : médiane des cinq p50 et cinq p99 ; plus mauvais p99 conservé pour montrer
le bruit de queue. Microsecondes, W seul ; pas de GPU ni coût réseau réel.

| Source m / N | médiane p50 lot64 | médiane p99 | pire p99 | écart-type des p50 |
|---|---:|---:|---:|---:|
| 2 /512 | 2026,1 | 2751,6 | 8073,9 | 77,145 |
| 3 /512 | 1790,6 | 2383,4 | 2896,2 | 11,216 |
| 4 /512 | 1642,6 | 2325,7 | 6075,1 | 82,234 |
| 5 /256 | 711,4 | 1005,8 | 1149,4 | 13,704 |
| 5 /512 | 1499,9 | 2091,3 | 2591,0 | 37,922 |
| 6 /256 | 648,8 | 905,2 | 1168,0 | 11,869 |
| 6 /512 | 1338,6 | 1857,3 | 2146,2 | 31,145 |

Sur5/6m, écart des p50 bien supérieur à trois écarts-types combinés : N512
est plus coûteux pour une qualité déjà reçue à N256. La queue observée interdit
de transformer ces mesures en garantie temps réel. Mémoire champ :6240/12384 octets.
Aucune extrapolation à4096 sources, qui nécessiterait un coût de préparation,
d'accumulation, des pools et de la rétention mesuré au nombre de sources.

## Arrivée à30s — sous-volet B2-05

WLIV289 octets pour un impact, indépendant de N ; T_sim30s est fourni par l'hôte.
Snapshot restauré identique ;17 WaterSample comparés bit à bit à la source,
avec fond plat de contrôle. Les paramètres B/ancre/contextes hôte ne sont pas comptés
comme des octets de WLIV ni prétendus absents de la sauvegarde globale.
505 restaurations après20 échauffements, indices252/499 :

| Source m / N | restauration p50 µs | p99 µs |
|---|---:|---:|
| 2 /512 | 4,9 | 8,7 |
| 3 /512 | 4,9 | 9,7 |
| 4 /512 | 7,5 | 9,7 |
| 5 /256 | 3,3 | 26,5 |
| 5 /512 | 6,9 | 8,4 |
| 6 /256 | 3,5 | 4,6 |
| 6 /512 | 4,9 | 10,7 |

Ces distributions ne permettent pas un classement fin de la restauration. Aucun
transport réseau ni crash disque testé. Le coût d'allocation initiale des pools est exclu.

## Décision, couverture et suite

N512 pour source2/3/4m, N256 pour5/6m, uniquement sur le domaine testé ; défaut64
inchangé. Les modes257..511 et>512 restent refusés, ainsi qu'une portée300m
à2m/60s malgré N512. La pente réelle512 est contrôlée au pic à0,2062lambda.

B2-03 : couverture et reconstruction de surface reçues, **énergie globale60s non reçue**.
B2-05 : restauration locale d'un impact à30s reçue, concurrence technologique absente.
B2-01/02/04 restent non reçus : sillage limité16s, eau profonde uniforme, pas de plage.
La fenêtre historique lambda_cut2,5–3m du dossier est une hypothèse antérieure à
la rétractation S39, pas un résultat que cette campagne confirmerait.

**S152-1, prochaine S153 : énergie et transport à60s** sur ce même candidat/domaine ;
mesurer le flux ou l'énergie sortie du disque au lieu d'appeler sa perte une dissipation.
B2 reste la priorité du dernier bilan. Aucun choix de lambda_cut ni de technologie finale.

> **Actualisation S153 — 2026-09-10.** Le bilan énergétique60s de la source4m
> est reçu dans [ENERGIE-B2-S153](ENERGIE-B2-S153.md), collecte étendue à120m :
> l'énergie sortie de80m est retrouvée. Les quatre autres longueurs restent S153-1.
