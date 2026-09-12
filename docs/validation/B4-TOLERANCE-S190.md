# B4 — Réception au seuil de 2 %, S190

2026-09-12. Arbitrage utilisateur acté par [ADR-120](../adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).
**Protocole déclaré avant exécution.** Le résultat sera ajouté après le rejeu.

## Conditions et décision

Véhicule et fournisseur partagés de S185–S189 : bloc 16³, 2744 cellules intérieures,
dx = 0,25 m, z de −4,05 à −0,80 m ; B16, impact et pressions du montage existant.
Repos initial, 100 pas de 10 ms, T0 = 1,5 s, T final = 2,5 s ; advection/viscosité
et soustraction de S, sans projection ni surface libre. Aucun changement du support.

Référence R : source pleine à chaque pas, chargée directement. Référence R2 : même
montage, 200 pas de 5 ms. Norme maximum composante par composante, normalisation
commune M = max|R(T)|. Réserve mesurée q = max|R−R2|/M ; ce n'est pas une borne
rigoureuse de l'erreur au continu. Le seuil de 2 % ne reçoit ici que ce montage.

Réseaux : plein ; ancrés isotropes 8³ et 5³ (témoins S188) ; gradués verticaux de
S189 à horizontale pleine, Nz = 3, 4, 5, 8 ; deux candidats mixtes ancrés horizontalement
à 8 nœuds par axe et gradués verticalement à Nz = 6, 8. Graduation issue du profil
de courbure au premier instant, comme S189. Cadences 1, 2, 4, 8, 16, 32, 64 ;
maintien et extrapolation causale, démarrage en maintien inclus. Pas d'instantané futur.

Pour chaque couple, mesurer s (spatial seul), t (temporel seul), e (composé), e2
(contre R2), tous divisés par M. **Reçu si s+t+q ≤ 0,02 ET e+q ≤ 0,02 ET e2 ≤ 0,02**,
avec toutes les valeurs finies et M > 0. Publier séparément ces nombres : un refus
par budget peut coexister avec un e inférieur à 2 % ; c'est le refus de financer une
compensation, pas une mesure cachée. La comparaison e2 ≤ e+q contrôle la triangulaire.

Choisir parmi les couples reçus celui qui minimise **le nombre exact d'évaluations
ponctuelles de source** `nœuds × ceil(100/c)` ; égalité départagée par e+q puis ordre
du tableau. C'est un décompte sur cette fenêtre, pas une mesure de temps CPU ni un
optimum sur tous les réseaux. Afficher aussi le rapport au plein (274400 évaluations).

## Contrôles requis

1. Plein/cadence 1 identique en bits à R dans les deux modes.
2. Réserve q reproduisant le contrôle S185 (environ 0,386 %) ; s des témoins ancrés
   reproduisant 1,7160 % et 3,6805 % à l'arrondi publié.
3. Au moins un couple dégradé reçu et un refusé ; omission totale de S refusée
   (erreur 100 % puisque le véhicule démarre au repos).
4. Refus des non-finis et du dénominateur nul, seuil inclusif exercé à 2 % et juste
   au-dessus. Toutes les cellules et sources contrôlées avant les maxima.
5. Exécutions répétées, sorties numériques identiques ; comparaison debug/release.
   Les résultats sont ceux de ce programme, aucun historique remplacé.

## Limites maintenues

Ce test ne mesure ni forces, ni perception, ni hauteur de surface. Le contrôle temporel
ne qualifie pas l'erreur spatiale du solveur et ne fournit pas une référence Navier–Stokes.
Un réseau/cadence reçu ici est un **profil de banc**, pas un défaut universel du runtime.
Les 2 % sont décidés indépendamment de l'issue de cette réception.

## Résultats du 2026-09-12

Commande : `cargo run --release --manifest-path code/Cargo.toml -p water-core --example b4_acceptance`.
Instrument : `code/water-core/examples/b4_acceptance.rs`. Les instantanés sont ceux du
fournisseur partagé ; les nœuds étant des cellules intérieures, le banc extrait leurs
valeurs du cache plein. L'identité des positions est vérifiée avant extraction. Le
décompte ci-dessous représente les évaluations nécessaires au profil, pas le travail
effectivement économisé par ce banc qui construit les références.

**12 couples reçus, 114 refusés.** Profil retenu parmi les 126 couples testés :
**14 × 14 × 8 nœuds, extrapolation causale, c = 8 (80 ms)**. Les deux axes horizontaux
sont pleins ; indices verticaux `[1, 4, 7, 9, 11, 12, 13, 14]`, ancrés et gradués.
Pas vertical maximal 0,75 m ; pas horizontal 0,25 m. Le profil n'est pas un ratio
isotrope `N`. Le démarrage sans deuxième instantané est en maintien, déjà compté.

`M = 7,993167674e-5 m/s` ; réserve `q = 0,385992 %`, contrôle S185 reproduit.
Tous les pourcentages du tableau ont ce même dénominateur.

| profil / cadence | spatial s % | temporel t % | s+t+q % | e+q % | contre R2 % | verdict |
|---|---:|---:|---:|---:|---:|---|
| plein, extrapolation 80 ms | 0 | 0,775379 | 1,161371 | 1,161371 | 1,161371 | reçu |
| **gradué 14×14×8, extrapolation 80 ms** | **0,639282** | **0,775379** | **1,800653** | **1,161371** | **1,161371** | **retenu** |
| gradué 14×14×8, extrapolation 160 ms | 0,639282 | 3,157033 | 4,182307 | 3,543025 | 3,543025 | refusé |
| ancré 8³, source chaque 10 ms | 1,715993 | 0 | 2,101984 | 2,101984 | 1,680683 | refusé par réserve |
| mixte 8×8×6, extrapolation 40 ms | 1,715990 | 0,168801 | 2,270783 | 2,047709 | 1,626408 | refusé |

Le profil retenu a `e = 0,775379 %` ; **marge au budget conservateur : 0,199347 point
de pourcentage**. Les 13 reconstructions de 1568 nœuds représentent **20384 évaluations**
contre 274400 au plein, soit **13,461538 fois moins**. Ce n'est ni une accélération
CPU mesurée, ni le facteur 64 historique de SPEC-004, ni une réception temps réel.

La borne choisie a une conséquence visible : le réseau 8³ serait sous 2 % contre
chacune des deux références, mais **sa marge ne couvre pas la réserve**. On publie
ce refus sans relever le seuil ni soustraire le défaut de référence favorablement.
Les candidats mixtes n'améliorent pas son erreur horizontale dominante sur ce montage.

Les contrôles du protocole passent : plein/cadence 1 identique en bits ; instants communs
des deux références identiques en bits malgré les actualisations intermédiaires ;
source omise à **100 %**, refusée ; non-finis/dénominateur nul refusés ; seuil inclusif
et dépassement éprouvés ; état contrôlé à chaque pas, triangulaire vérifiée.

**Empreinte release : `0x4b479c21a520cd7b`**, reproduite sur deux exécutions release et une debug, sorties intégralement identiques.
Le relevé intégral est dans [B4-TOLERANCE-S190-MESURES](B4-TOLERANCE-S190-MESURES.md).

## Verdict de réception et suite

**Attente du critère close. Volet d'échantillonnage de source reçu à 2 % sur ce montage,
avec le profil ci-dessus.** A50 reste partielle dans son périmètre physique général ;
B4 complet n'est pas déclaré reçu. Les mesures S185–S189 ont désormais un critère
et un résultat de dimensionnement, plus une contre-épreuve qui le refuse hors marge.

**S190-1 reprend S189-1** : éprouver la projection de pression et le profil reçu avec
le seuil inchangé, contre référence pleine/raffinée ; puis confronter le candidat δ
au substitutif intégral. La projection est un travail de construction/validation,
elle ne justifie plus de demander quelle erreur est acceptable.

Suite workspace debug : **331 tests réussis, cinq ignorés**, aucun échec. Aucun code
de bibliothèque ni support historique modifié ; pas de nouvelle réception multiplateforme.
