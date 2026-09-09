# S129 — Bilan énergétique du candidat transporté

2026-09-10. S128-1, suivi A203/A204. Applique ADR-060 et ADR-085, méthode profonde S78.
Nouveau test privé `energy_tests::transported_candidate_energy_s129` ; aucun calcul
de production modifié, aucun accès public aux nœuds ajouté.

## Périmètre et méthode

S127 recevait le bilan total de la référence, la surface et la part potentielle du candidat.
S129 mesure aussi la **cinétique des nœuds effectivement construits par le candidat**.
Même fixture : N256, rayon80 m, horizon48 s, λ4 m, énergie0,01 J, g9,81, ρ1025,
profondeur déclarée20 m, pente maximale0,1, source née à0 et TTL4 s conservé.
Les paramètres sont ceux des entrées f32 réelles, convertis en f64 pour la mesure.

Le test reprend les coefficients, nombres d'onde, fréquences et phases PhaseQ32 internes,
ainsi que Bessel de production. Il assemble les vitesses modales en f64 hors runtime.
À chaque rayon, il contrôle leur somme à la surface contre `sample` : vitesse radiale et
vitesse verticale `deta_dt`. La potentielle emploie directement l'élévation publiée.

Observable : `e(r,t)=ρ/2 · [gη²+∫(u_r²+w²)dz]`, pour `−∞<z≤0`.
L'intégrale du produit de deux modes vaut `1/(k_i+k_j)` : **double somme entière**,
interférences incluses. Le zéro cinétique à la naissance est calculé par cette même somme,
jamais imposé par un raccourci. L'assemblage f64 mesure le modèle des nœuds ; aucune
fonction de vitesse profonde ni d'énergie intégrée n'est ajoutée au service public.

Simpson320 puis640 intervalles sur0–80 m, anneau32–80 avec frontière commune aux deux
grilles. Trois instants0/24/48 s, soit1923 rayons-temps. Même grille exacte que S127.
Les références figées viennent des tableaux de [TRANSPORT-ETENDU-S127](TRANSPORT-ETENDU-S127.md) :
512 modes indépendants f64, Bessel angulaire1024 directions et640 intervalles radiaux.
Leur convergence spectrale et angulaire est documentée là-bas ; elle n'est pas réévaluée
par le test du candidat. Arrondi de copie≤5e-11 E0 par valeur ; cinétique de référence
obtenue par total moins potentielle, incertitude de copie≤1e-10 E0.

## Critères déclarés avant mesure

- Total, potentielle, cinétique et énergie de l'anneau contre S127 : écart≤1e-4 E0.
- Raffinement radial de chaque part et de l'anneau :≤0,002 E0 ; total fin proche de E0
  à0,003 près. Critères S78/S127, paramètres de banc à calibrer B2.
- Densités≥−1e-12 J/m² pour les arrondis ; cinétique initiale exactement nulle.
- Énergie hors32 m initialement<0,003 E0, puis>0,5 E0 à48 s, comme S127.
- Sommes de vitesses à la surface : écart≤1e-6 normalisé à la somme des poids positifs
  coefficient×pulsation. Plus strict que S126 car seules les opérations d'assemblage diffèrent.
- Contre-épreuves : omettre la cinétique ou ses termes croisés doit faire échouer le
  critère énergétique aux instants non nuls. Aucun seuil ajusté après résultat.

## Résultats

E0 désigne l'énergie prescrite f32 de la source, sans renormalisation.

| temps | total/E0 | potentielle/E0 | cinétique/E0 | anneau32–80/E0 | rayon moyen dans R80 |
|---:|---:|---:|---:|---:|---:|
| 0 s | 1,0000607056 | 1,0000607056 | 0,0000000000 | 0,0000001300 | 1,16032675 m |
| 24 s | 1,0000008958 | 0,5000008713 | 0,5000000245 | 0,0433703138 | 26,70982468 m |
| 48 s | 0,9998652235 | 0,4999275906 | 0,4999376328 | 0,9998521408 | 53,41570410 m |

| temps | écart total/référence, /E0 | écart cinétique/référence, /E0 | raffinement potentielle, /E0 | raffinement cinétique, /E0 | raffinement anneau, /E0 |
|---:|---:|---:|---:|---:|---:|
| 0 s | 8,496e-8 | 0 | 1,019e-3 | 0 | 7,111e-11 |
| 24 s | 9,045e-7 | 3,424e-7 | 1,867e-8 | 2,887e-8 | 3,213e-6 |
| 48 s | 8,783e-8 | 3,424e-7 | 4,565e-7 | 8,533e-8 | 5,256e-7 |

Écart maximal potentielle/référence5,622e-7 E0 ; anneau/référence8,800e-8 E0.
L'écart de rayon moyen reste inférieur à8,95e-7 m, diagnostic sans seuil propre.
Contrôle de surface normalisé maximal **2,036e-8**, pour1e-6 admis.
Densité totale minimale **8,447e-22 J/m²** ; contrôles locaux de finitude et de positivité reçus.

À48 s, **99,985214 % de E0 est mesuré entre32 et80 m**. Le disque entier omet0,013478 %
de E0 ; comme S127, ce chiffre n'est ni une conservation globale exacte ni une preuve de
cause exclusive du déficit. Le raffinement initial est le plus contraignant et reste reçu.

## Contre-épreuves

Sans cinétique, le bilan vaut0,5000008713 E0 à24 s et0,4999275906 E0 à48 s : rejet.
Avec seulement les termes diagonaux de cinétique, le bilan vaut **0,6171877186 E0** puis
**0,6171145664 E0** : rejet. Une somme d'énergies modales isolées perd ici les interférences
qui localisent le paquet. Ces calculs volontairement incomplets ne changent pas le candidat.

## Vérification et portée

Reproduction ciblée :

`cargo test --release --manifest-path code/Cargo.toml -p water-core transported_candidate_energy_s129 -- --nocapture`

Version finale reçue en release :1 test,0 échec,0,32 s hors compilation.
Suite complète debug `cargo test --manifest-path code/Cargo.toml --quiet` :
**257 réussis,5 ignorés**,0 échec ; water-core164/2 en7,78 s, harnais93/3 en53,10 s.
Quatre avertissements préexistants du harnais, aucun nouveau.
Ces durées décrivent les tests locaux, pas le coût du service.

**S128-1 réalisée sur fixture.** Surface S127, chemin hôte S128 et bilan des nœuds S129
se complètent. A203 reste partielle pour les autres paramètres. Même modèle profond
linéaire : aucune réception physique en profondeur finie, aucun bilan mixte B+impacts+pression,
aucune calibration de source ou conservation globale déduite. Aucun défaut de production
établi ; modèle, seuils et défaut N64 conservés. Aucun ADR nouveau.

**S129-1, prochaine session S130 :** reprendre la construction par l'admission dynamique
des sources de pression. Le contrôleur ADR-078 emprunte encore un journal figé ; changer
les sources oblige à le détruire. Construire une publication cohérente journal/champ sur
pools hôte, y compris lorsqu'une source arrive à l'instant déjà publié : aucun `Unchanged`
sur un journal différent, dernière publication conservée au refus et attente explicite.
Recevoir ce contrat sur le chemin pression avant de revendiquer une transaction mixte.

85 ADR,204 angles,17 invariants,6 spécifications,23 cas. Aucune nouvelle leçon distincte :
interférences déjà enseignées par S78, portée de la preuve limitée conformément à L215.
