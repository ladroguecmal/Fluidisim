# S147 — Forme du spectre, troncature et moments

2026-09-10. Instrument : `code/water-core/src/spectrum_reference_s147.rs`, compilé
uniquement pour les tests. Décision : [ADR-100](../adr/ADR-100-spectre-de-fond-et-bande-explicite.md).
Formules et sources : SPEC-001 §1 bis. Aucun calcul de production changé.

## Mesure du fond réellement construit

`Background::configure`, Hs=2 m, Tp=6 s, N=32, graine=42. Les composantes privées sont
lues par le test, sans nouvel accès public. Amplitudes identiques, m0 reçu à 1e-7 relatif ;
le rapport m2/m0 est vérifié indépendamment par la somme géométrique des fréquences.
Toutes les valeurs ci-dessous utilisent les fréquences normalisées x=f/fp.

| Spectre | m2/m0 | Tz/Tp |
|---|---:|---:|
| Fond actuel N32 | 1,377539699 | 0,852016368 |
| JONSWAP γ=3,3, même bande [0,5;2] | 1,320315989 | 0,870284119 |

L'écart m2 est **+4,334 %** à Hs identique. Ce résultat n'est pas une mesure de mouvement
de coque : une telle réponse demanderait sa fonction de transfert. Il montre seulement
qu'un Hs reçu laisse les vitesses orbitales différentes.

## Troncature avant renormalisation

Fractions du spectre continu conservées, borne basse 0,5fp :

| γ | borne haute/fp | m0 conservé | m2 conservé | Tz/Tp après normalisation |
|---:|---:|---:|---:|---:|
| 1 | 2 | 92,4849 % | 69,2634 % | 0,820860 |
| 1 | 4 | 99,5129 % | 92,1281 % | 0,738293 |
| 3,3 | 2 | 95,0719 % | 75,8610 % | 0,870284 |
| 3,3 | 4 | 99,6806 % | 93,8178 % | 0,801322 |
| 7 | 2 | 96,6227 % | 81,2109 % | 0,903699 |
| 7 | 4 | 99,7811 % | 95,1880 % | 0,848251 |

La référence « totale » est intégrée sur [0,125;1024] ; queue haute m0 ≤1/(4·1024⁴),
m2 ≤1/(2·1024²), queue basse négligeable à la précision publiée. Pour γ=1, les totaux
analytiques contrôlent cette approximation. **Il n'y a pas de m4 total fini** pour la
queue idéale f^-5 ; les pentes se comparent donc sur une bande déclarée.

## Discrétisation dans la bande

32, 64, 128 et 256 cellules logarithmiques ; énergie intégrée par cellule, fréquence au
centre géométrique. Référence Simpson en log-fréquence avec Jacobien, 16 384 puis 32 768
intervalles, **coupure de l'intégration au pic** où σ change. Accord relatif <1e-8 sur
m0/m1/m2/m4. Intégrales de cellule reçues à 64/128 subdivisions, même seuil.

À N32, erreur maximale sur les six fixtures : m1 **0,0170 %**, m2 **0,0639 %**,
m4 **0,1853 %**. Les sorties par N sont reproductibles par la commande ci-dessous.
La garde utilise une borne dérivée, non ces valeurs observées :
`erreur_relative ≤ exp(p ln(b/a)/(2N))-1`, puisque la fréquence d'une cellule reste
dans cet intervalle autour de son centre et ses poids sont positifs.

**Contre-épreuve :** supprimer le Jacobien revient à intégrer m-1 au lieu de m0 ;
l'écart dépasse 1 % sur γ=3,3, [0,5;2]. Le test refuse cette confusion.

Deux erreurs initiales de l'instrument sont conservées ici : une assertion anticipait
un écart >5 % entre les spectres, alors que la mesure donne 4,334 %. Elle est remplacée
par la séparation d'avec le budget arithmétique, pas par une tolérance physique ajustée.
Simpson traversant le changement de σ échouait le raffinement à 1e-8 sur une cellule ;
la coupure à x=1 a corrigé l'intégration, sans relâcher ce seuil.

## Rejouer et limites

`cargo test --release --manifest-path code/Cargo.toml -p water-core s147 -- --nocapture`

Deux tests reçus. Instrument f64/libm, sans dépendance externe, **hors runtime**.
Pas encore de champ JONSWAP évaluable, ni de réception de ses phases, de ses directions,
de ses refus, du budget B+W ou du déterminisme croisé. Aucun hash historique renouvelé.
Le premier lot de construction est S147-1 ; les seuils de cet instrument sont des
contrôles numériques et ne calibrent aucune mer du jeu.
