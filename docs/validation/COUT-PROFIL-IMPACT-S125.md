# S125 — Coût et choix du profil radial

2026-09-09. S124-1 / A202. Mesure locale, aucun budget cible certifié.

## Protocole

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example impact_profile_cost`

AMD Ryzen AI 7 350, Windows x86_64 MSVC, rustc 1.97.0 / LLVM 22.1.6,
profil release du dépôt (contrôles de débordement actifs). Aucun changement de bibliothèque.
Deux exécutions indépendantes, une seconde de mise en régime complète chacune ; quinze blocs
alternant l'ordre des cinq montages. Chaque bloc mesure 2048 constructions puis 32 lots de
64 points. `black_box` protège entrées et résultats. Construction séparée de l'évaluation ;
points préparés hors chronométrage, dates entières variables après naissance.

Fixture ADR-060 : λ=4 m, E=0,01 J, g=9,81, ρ=1025, h=20 m, pente limite 0,1,
horizon 4 s. Domaine commun R16 ; extensions R64/N128 et R128/N256. Paramètres de banc,
pas paramètres gameplay. Rayons irréguliers et quadrants variés ; le centre est inclus.
La mesure est une boucle de `sample`, pas la requête transactionnelle B+W ou le cycle mixte.

## Mesures

Deuxième exécution, en microsecondes sauf dernière colonne. Min/max portent sur les moyennes
des quinze blocs, pas sur les latences extrêmes des appels individuels.

| N | rayon m | construction min/médiane/max µs | point min/médiane/max µs | 64 points médiane ms |
|---|---:|---:|---:|---:|
| 64 | 16 | 0,5631 / 0,5723 / 0,8338 | 1,4917 / 1,6258 / 1,9612 | 0,1041 |
| 128 | 16 | 1,0910 / 1,1623 / 2,5038 | 3,0836 / 3,2857 / 3,7411 | 0,2103 |
| 256 | 16 | 2,1252 / 2,3770 / 3,0711 | 6,1659 / 6,3903 / 7,9319 | 0,4090 |
| 128 | 64 | 1,0938 / 1,1757 / 1,5657 | 5,5277 / 5,8044 / 7,5252 | 0,3715 |
| 256 | 128 | 2,1684 / 2,5327 / 3,1229 | 13,8075 / 14,3896 / 15,5583 | 0,9209 |

Première exécution, médianes construction/point µs, même ordre :
0,6503/1,5547 ; 1,1467/3,3402 ; 2,2442/6,4454 ; 1,1369/5,6624 ; 2,2942/14,5824.
Le rapport N256/N64 sur les mêmes points vaut **3,93 à 4,15** en évaluation.
Mais N256/R128 contre N64/R16 vaut **8,85 à 9,38** : doubler la portée à N256
par rapport à R64 n'est pas mesuré ici ; le constat porte sur ces montages précis.
À N constant, l'extension modifie la distribution des arguments de Bessel ; davantage
de modes passent par l'asymptotique au-delà de 64. Le facteur quatre des modes ne suffit
donc pas à prédire le coût de l'usage étendu.

`size_of<RadialImpact<N>>` : **1632 / 3168 / 6240 octets** pour N64/128/256.
Ce sont les champs seuls ; les pools utilisent `Option<RadialImpact<N>>` et le service deux
pools, auxquels s'ajoutent journal et sorties. Ne pas annoncer ces tailles comme mémoire totale.

S118 mesurait ~49 ms de cycle mixte, dont ~35 ms de requête64. Ces valeurs historiques
restent un contexte, pas un budget disponible ni un témoin chronométré dans cette session.
Multiplier le coût isolé par le nombre d'impacts est une estimation, pas une réception du cycle.

## Contrôles et limite de la conclusion

256 points-temps communs, sept composantes : toutes finies. Sur 1792 composantes,
**1479** diffèrent en bits entre N64/N128, **1495** entre N64/N256, identiquement aux deux
exécutions. Maximum absolu par grandeur (comparaisons regroupées) :
η 4,66e-10 m ; dη/dt 1,52e-9 m/s ; potentiel 6,48e-10 m²/s ;
pentes 4,07e-10/4,66e-10 ; vitesses horizontales 6,43e-10/8,80e-10 m/s.
Ce sont des écarts entre candidats, **pas des erreurs contre une référence indépendante**.
384 points-temps étendus sont finis ; aucune réception physique à grande portée n'en découle.

Le profil change le résultat arrondi : il ne peut pas varier suivant la puissance du client
pour une même eau autoritaire (I-03/I-15). Le code de `live_snapshot.rs` porte déjà N dans
WLIV V1, octets 8–11, et refuse un contexte différent : cette protection existe, elle n'est
pas à reconstruire. Le pool `Prepared<N>` est homogène et son contexte partage rayon/horizon.
Choisir par impact demanderait une nouvelle organisation, absente aujourd'hui.

## Décision et suite

Voir [ADR-085](../adr/ADR-085-profils-radiaux-selon-le-domaine.md).
Le défaut N64 est conservé. Les montages étendus déclarent explicitement N128 ou N256
au niveau du service homogène, dimensionné au domaine commun, puis reçu physiquement.
S125-1 : réception indépendante du champ étendu, avant toute adoption dans le cycle mixte.
