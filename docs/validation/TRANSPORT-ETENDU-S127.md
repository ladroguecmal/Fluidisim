# S127 — Un paquet transporté au loin dans un domaine admissible

2026-09-09. S126-1 / A204. Applique ADR-060, ADR-061, ADR-066 et ADR-085.
Bibliothèque inchangée ; nouveau banc `transport_extended_impact`.

## Dimensionnement avant la mesure

Le contrôle existant donne `dk·(R+c_g,max·T)≤π/2`. La bande d'ADR-060 ayant
`width=3π/λ`, la relation devient **`R+c_g,max·T≤Nλ/6`**. Rayon et temps de
propagation consomment la même capacité numérique ; ils ne se choisissent pas séparément.

À λ4 m, le groupe le plus rapide vaut1,76709 m/s. Demander les temps caractéristiques
`R/c_g,max` aux anciens couples N128/R64 et N256/R128 donne36,217661/72,435322 s.
**Les deux constructeurs refusent effectivement `Resolution`** : calcul et refus concordent.
Ce temps caractéristique n'est pas un temps d'arrivée exact d'un front ; le spectre ne
possède pas de support spatial compact.

Le montage déclaré avant mesure est **N256, rayon80 m, horizon48 s** :
`80+c_g,max·48=164,820497 m`, pour170,666667 m disponibles. Il se construit avec
la source S126 inchangée : λ4 m, énergie0,01 J, g9,81, ρ1025, profondeur20 m et pente0,1.
**TTL source4 s conservé** ; aucun redémarrage des phases. ADR-066 sépare déjà TTL et horizon.
Ce montage vise un transport hors32 m, pas une affirmation d'arrivée jusqu'au bord80 m.

## Méthode et critères annoncés

Reproduction :

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example transport_extended_impact`

L'oracle S126 est extrait dans `examples/support/radial_reference.rs` et partagé ; mêmes
formules, coefficients indépendants et intégrales angulaires f64. La campagne S126 est
rejouée après extraction : résultats conservés et assertions reçues.

La densité de référence reprend l'observable positive de S78 :

`e(r,t)=ρ/2 · [gη²+∫(u_r²+w²)dz]`.

Chaque vitesse modale contient `exp(kz)` ; intégrer le produit de deux modes donne
`1/(k_i+k_j)`. La double somme symétrique est évaluée en f64, depuis les seuls nœuds
de référence. Aucun proxy signé `ψ·∂tη` n'est employé comme densité locale.
Énergie et premier moment sont intégrés en rayon par Simpson :320 puis640 intervalles
sur0–80 m, avec32 m exactement sur une frontière dans les deux grilles.

Référence spectrale256/512 aux641 positions axiales, trois temps0/24/48 s ; Bessel à1024
directions. Contrôle supplémentaire512/1024 modes et1024/2048 directions sur33 positions
obliques, huit instants jusqu'à48 s. Soit **2187 points-temps** pour les sept composantes.
Le raffinement angulaire de S127 porte cette grille oblique, pas toute la double somme d'énergie.

Critères du commit de plan, paramètres de banc **à calibrer B2** :

- Sept composantes : erreur normalisée≤1e-4 ; écarts d'oracle≤1e-6, comme S126.
- Énergie totale fine : écart relatif≤0,003 ; raffinement radial<0,002 et spectral<1e-4,
  comme S78. Densité≥−1e-12 J/m² pour les arrondis, comme S78.
- Transport : énergie hors32 m initialement<0,003 E0, puis **>0,5 E0 à48 s**.
  C'est la majorité transportée demandée au banc, pas une portée gameplay calibrée.

## Énergie et déplacement mesurés

E0 désigne l'énergie de source0,01 J, sans renormalisation des résultats.

| temps | E(R80)/E0 | E(32≤r≤80)/E0 | rayon moyen dans R80 |
|---:|---:|---:|---:|
| 0 s | 1,0000606206 | 0,0000001300 | 1,16032681 m |
| 24 s | 0,9999999914 | 0,0433702968 | 26,70982379 m |
| 48 s | 0,9998653113 | **0,9998522288** | **53,41570380 m** |

**99,985 % de l'énergie prescrite est entre32 et80 m à48 s.** La valeur initiale
ne satisferait pas le critère de transport : un champ figé à la naissance serait refusé.
Le rayon moyen augmente aux trois instants ; il ne désigne pas le rayon maximal du paquet.

Écart total dû au raffinement radial320→640 :1,019e-3 E0 àt0,1,602e-9 à24 s,
2,269e-9 à48 s. Raffinement spectral256→512 : maximum6,937e-11 E0.
La densité minimale observée est **1,143e-20 J/m²**, positive.
Le biais initial de6,062e-5 E0 est compatible avec la quadrature spatiale ; il ne justifie
aucune compensation d'amplitude. À48 s le disque omet0,01347 % de E0 : aucune conservation
globale exacte ni explication exclusive de ce déficit n'est déduite d'un disque fini.

## Ce qui est reçu pour le candidat

Maximum d'erreur normalisée sur les sept composantes : **7,128e-7**, pour1e-4 demandé.
Par ordre η, dη/dt, potentiel, pentes x/y, vitesses x/y :
`4,701e-7 ; 6,886e-7 ; 7,128e-7 ; 3,103e-7 ; 1,429e-7 ; 4,137e-7 ; 2,158e-7`.
Oracle spectral : maximum8,683e-9 ; angulaire :2,598e-16.
Le candidat refuse la première microseconde après48 s, avec un témoin accepté à48 s.

La **part potentielle** intégrée du candidat est comparée à celle de la référence :

| temps | potentiel référence/E0 | potentiel candidat/E0 | écart/E0 |
|---:|---:|---:|---:|
| 0 s | 1,0000606206 | 1,0000607056 | 8,491e-8 |
| 24 s | 0,5000003092 | 0,5000008713 | 5,621e-7 |
| 48 s | 0,4999280208 | 0,4999275906 | 4,301e-7 |

**Le bilan total est celui de la référence**, et la surface ainsi que le potentiel intégré
du candidat sont reçus. Le candidat public ne livre pas ses vitesses en profondeur : aucune
mesure de son énergie cinétique totale n'est revendiquée ici. S78 l'avait calculée depuis
ses nœuds internes sur une autre fixture ; son résultat ne s'étend pas par simple citation.

## Portée de la réception et suite

**S126-1 réalisée sur fixture, A204 traitée dans ce périmètre.** Le rayon80 et l'horizon48
ont été choisis conjointement avant les résultats ; aucune ancienne demande à128 m n'a été
silencieusement réduite. Ces demandes aux temps de groupe restent explicitement refusées.
Défaut N64, modèles, seuils, TTL et ordre de sommation de production inchangés ; aucun ADR nouveau.

**S127-1, prochaine session S128 :** exercer le montage transporté N256/R80/horizon48 dans
le service B+W `LiveWater` : renouvellement au-delà du TTL4, requêtes à24/48 s, sauvegarde
et restauration dans le même N, identité avec la construction directe et coût du cycle.
Le transport ponctuel reçu ne reçoit pas ce chemin hôte complet. Le mélange avec pression
garde ses propres fenêtres et n'est pas inclus implicitement.

Restent ouverts : bilan cinétique du candidat étendu, autres paramètres, calibration des
sources physiques, profondeur finie, admission dynamique mixte, bilan mixte et disque.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas. L215 appliquée, aucune nouvelle
leçon distincte ajoutée. Suite256/cinq ignorés vérifiée S125, non relancée ; production inchangée.

## Vérification finale

Deux exécutions release du nouveau banc :15,94 puis15,22 s, valeurs identiques. La seconde
ajoute les raffinements sur l'anneau32–80 : maximum radial3,192e-6 E0, spectral2,599e-10 E0.
À48 s, leurs écarts valent1,861e-8 et1,053e-10 E0 ; rayon moyen modifié de6,543e-8 m au
raffinement radial. Les assertions du transport passent également sur ces mesures locales.
La campagne S126 repasse après extraction de l'oracle, valeurs conservées,27,77 s.
Ces durées incluent les références f64 sur Ryzen AI 7 350, rustc1.97 Windows MSVC ;
aucun chronométrage du runtime B+W dans cette session. Aucune modification de bibliothèque.
