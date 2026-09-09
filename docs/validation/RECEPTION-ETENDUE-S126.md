# S126 — Champ radial étendu : précision reçue, transport encore à recevoir

2026-09-09. S125-1 / A203. Applique ADR-060, ADR-084 et ADR-085 ; bibliothèque inchangée.

## Domaine et critère annoncés avant mesure

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example receive_extended_impact`

Deux fixtures : **N128/R64 m** et **N256/R128 m**, λ4 m, énergie0,01 J, g9,81 m/s²,
ρ1025 kg/m³, profondeur20 m, pente maximale0,1, âge0–4 s. Paramètres de banc, à calibrer B2.
Entrées f32 converties exactement en f64 dans la référence, constantes mathématiques f64.
Neuf instants : 0, 1, 137119, 500003, 1333331, 2718281, 3999000, 3999999, 4000000 µs.
74/76 points : centre, 64 rayons irréguliers répartis dans quatre directions, voisinages
où la bande spectrale traverse le raccord Bessel, 99 % du rayon, dernier f32 intérieur
et bord exact sur deux axes. **666 + 684 = 1350 points-temps**, sept composantes chacun.

Critère fixé dans le commit de plan : erreur absolue divisée par l'intégrale des poids
positifs de la composante **≤1e-4**. Le seuil normalisé d'élévation vient d'ADR-060 ; son
application aux six autres composantes est un **critère de banc à calibrer B2**, pas une
tolérance gameplay. L'échelle est indépendante de la valeur locale et ne diverge pas aux zéros.

| composante | échelle naturelle | valeur |
|---|---|---:|
| η | ∫ A k dk | 1,01250726e-3 m |
| dη/dt et u horizontal | ∫ A k ω dk | 4,53010631e-3 m/s |
| potentiel | ∫ A ω dk | 2,24802449e-3 m²/s |
| pente | ∫ A k² dk | 2,09029610e-3 |

## Oracle et contrôles indépendants

Spectre polynomial et normalisation physique d'ADR-060 reconstruits depuis les entrées.
L'oracle ne lit aucun nœud du candidat ; pas de table, pas d'asymptotique Bessel, pas de
PhaseQ32 : J0/J1 par leurs intégrales angulaires, sinus/cosinus f64, temps f64 hors runtime.
Même modèle physique : cette indépendance teste l'implémentation, pas la validité du modèle
profond ou du spectre de source face à une collision réelle.

Raffinements **séparés** : 512/1024/2048 nœuds spectraux à 1024 directions ; puis
1024/2048 directions à 2048 nœuds. Seuil d'écart normalisé oracle annoncé : **1e-6**,
soit 1 % du seuil du candidat. Maxima sur les sept composantes :

| comparaison | N128/R64 | N256/R128 |
|---|---:|---:|
| spectral512/1024 | 6,07e-11 | 9,80e-11 |
| spectral1024/2048 | 3,77e-12 | 5,99e-12 |
| angulaire1024/2048 | 4,29e-16 | 4,05e-16 |

L'écart spectral recule d'environ16 au doublement ; le résidu angulaire est au niveau des
arrondis f64. Convergence observée sur cette grille, pas preuve uniforme de l'erreur d'oracle.

Au centre initial, ∫x²(1−x)²dx=1/30 et la symétrie donnent
`η(0,0)=C·width·(lo+width/2)/30`. Accord relatif **4,64e-14** avec l'oracle2048.
Vitesses et potentiel initiaux exactement nuls. Les deux champs refusent le premier f32
au-delà du rayon et la première microseconde au-delà de4 s ; témoins intérieurs reçus.
Inverser volontairement les vitesses horizontales fait échouer le critère de **0,7473**
et **0,6774** : le banc distingue ce défaut de signe des faibles écarts nominaux.

## Résultats du candidat

| composante | erreur normalisée N128/R64 | N256/R128 | erreur absolue maximale des deux |
|---|---:|---:|---:|
| η | 2,24e-7 | 4,44e-7 | 4,50e-10 m |
| dη/dt | 2,34e-7 | 2,61e-7 | 1,18e-9 m/s |
| potentiel | 1,35e-7 | 1,78e-7 | 3,99e-10 m²/s |
| pente x | 5,25e-8 | 8,76e-8 | 1,83e-10 |
| pente y | 1,35e-7 | 1,09e-7 | 2,82e-10 |
| vitesse x | 6,58e-8 | 6,36e-8 | 2,98e-10 m/s |
| vitesse y | 7,39e-8 | 1,14e-7 | 5,16e-10 m/s |

**Les deux fixtures passent le critère annoncé**, avec une marge minimale d'environ225.
Deux exécutions release donnent les mêmes valeurs ; 25,50/25,85 s pour oracle et campagne
sur le Ryzen AI 7 350, rustc1.97 Windows MSVC. Ce temps n'est pas celui du runtime d'eau.
La deuxième exécution ajoute les diagnostics locaux ci-dessous sans déplacer le verdict.

## Ce que le succès ne valide pas

Dans l'anneau extérieur `r≥0,9R`, le maximum d'erreur divisé par le pic de référence
**de ce même anneau** vaut **1,56–4,01 %** à R64 et **11,32–14,84 %** à R128 selon la
composante. Il ne s'agit pas d'une erreur relative point par point, qui divergerait aux zéros.
Les pics d'élévation y sont seulement **2,82e-9 m** et **2,17e-10 m** : les grandes erreurs
relatives portent sur des queues extrêmement faibles. Aucun seuil local physique n'est
fixé rétroactivement, et aucun intérêt gameplay de ces queues n'est revendiqué.

Le point décisif est temporel : pour ce spectre profond, `c_g=0,5√(g/k)` et
`c_g,max≈1,767 m/s`. En4 s, le déplacement caractéristique du groupe le plus rapide est
**7,07 m**, très inférieur à64 ou128 m. Une onde à spectre borné n'a pas de front compact :
ces distances ne signifient pas que le champ est nul au-delà. Elles montrent en revanche
que les points lointains testent les queues, pas un paquet arrivé à la portée annoncée.

La borne d'ADR-060 impose `T≤(π/(2dk)−R)/c_g,max`, soit environ **12,07 s** à N128/R64
et **24,15 s** à N256/R128. Les temps caractéristiques `R/c_g,max` sont **36,22/72,44 s** :
on ne peut donc pas simplement attendre plus longtemps dans ces deux montages. Cette
estimation dérivée n'est pas une mesure de transport ; elle en définit le problème suivant.

## Statut et suite

**S125-1 réalisée sur les deux fixtures**, A203 partielle : sept composantes reçues contre
oracle indépendant selon le critère annoncé. Pas de bilan d'énergie étendu, de réception
continue, d'autres λ/énergies/profondeurs, ni d'intégration du cycle mixte dans cette session.
Défaut N64 et décision ADR-085 inchangés. Aucune correction de bibliothèque justifiée.

**S126-1 / A204, prochaine session S127 :** dimensionner ensemble portée et temps de
transport, puis mesurer un paquet effectivement propagé au loin. Identifier un domaine
admissible N≤256 ou constater la limite ; ne pas réduire silencieusement le rayon demandé.
Le critère spatial reçu ici ne remplace pas ce test de transport.

85 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas. Suite256/cinq ignorés vérifiée
en S125, non relancée : bibliothèque et tests existants inchangés. La nouvelle campagne
release exécute ses propres assertions ; compilation sans nouvel avertissement.

> **Suivi S127 — 2026-09-09 :** S126-1 réalisée sur fixture dans
> [TRANSPORT-ETENDU-S127](TRANSPORT-ETENDU-S127.md). N256/R80/horizon48 admis, transport
> hors32 m reçu ; anciens couples64/128 aux temps de groupe toujours refusés. A204 traitée
> dans ce périmètre ; cycle hôte du montage ouvert sous S127-1.
