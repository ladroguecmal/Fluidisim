# Démarrage couplé plat — S251

## Protocole et diagnostic avant correctif

Reprendre exactement les cas A283 de S250 (16×8 et 32×16), couvercle homogène,
v=0, B/W réels et éponge. Assembler une projection indépendante f64 par arêtes
à partir du même prédicteur f32 ; CG à résidu relatif 10⁻¹³, vrai résidu vérifié
à 10⁻¹¹. Comparer la vitesse, pas seulement la pression. Puis éprouver une projection
du défaut de divergence directement sur la vitesse corrigée, en f32.

| nx | max vitesse prédite | max vitesse oracle | D avant | erreur vitesse relative avant | D après correction | erreur vitesse après |
|---|---|---|---|---|---|---|
| 16 | 1,4033·10⁻⁵ | 2,4695·10⁻⁷ | 1,4173·10⁻⁴ | 1,2819·10⁻⁴ | 2,8772·10⁻⁸ | 9,4477·10⁻⁷ |
| 32 | 1,4035·10⁻⁵ | 3,2145·10⁻⁷ | 8,0636·10⁻⁵ | 5,0539·10⁻⁵ | 4,4209·10⁻⁸ | 2,1468·10⁻⁶ |

Le solveur initial atteint le plancher de pression. La projection retranche environ
98 % de la vitesse prédite : l'erreur absolue de cette soustraction devient visible
rapportée à la petite vitesse restante. Une correction appliquée **à cette vitesse**
abaisse D de trois ordres sans f64 dans le cœur et sans toucher aux tolérances.
46/87 itérations supplémentaires dans ce montage ; pas de boucle de retentatives.
Ce diagnostic ne prouve pas qu'un unique affinage suffira à toute entrée.

Réception avant construction : D≤10⁻⁵ (S199), erreur vitesse contre oracle ≤10⁻⁴
(norme maximale relative, même exigence que le test diagnostic), cas sans correctif
refusé, cas avec correctif reçu ; trajectoire de vingt pas S250, rollback et allocations,
suite complète et empreinte historique. Surface mobile et B4 global hors lot.

Commande diagnostic :
`cargo test --release --offline --manifest-path code/Cargo.toml -p water-core diagnose_flat_projection_s251 -- --nocapture`.

## Réception du chemin consommé

ADR-150 branche l'affinage seulement après un refus au plancher des projections
ordinaires du pas couplé. Une seule tentative ; tous les champs restent f32.
`flat_coupled_step_received_against_oracle_s251` appelle l'API publique et mesure :

| nx | D final | erreur max vitesse / max oracle | erreur max pression / max oracle | itérations totales |
|---|---|---|---|---|
| 16 | 5,7545·10⁻⁸ | 9,8463·10⁻⁷ | 4,9965·10⁻⁸ | 329 |
| 32 | 4,4209·10⁻⁸ | 2,1468·10⁻⁶ | 5,3922·10⁻⁸ | 672 |

Les itérations comprennent le solveur initial, son repli multigrille et l'affinage.
Les légères différences avec le diagnostic proviennent de ce chemin initial complet,
alors que le diagnostic appelle directement le préconditionneur multigrille.
L'oracle teste **la projection du même prédicteur**, pas la précision physique B4
du couplage à une solution totale indépendante.

Cinq tests ajoutés : diagnostic indépendant, réception API, expiration/reprise au bit
à cinq points du calcul, couvercle non homogène qui ne doit pas être appliqué deux
fois, allocation globale nulle pendant un affinage effectivement déclenché.
Le test de mémoire compte le tampon ajouté : **4·nx·nz octets** (512/2 048 octets
aux deux tailles), y compris sur les volumes qui n'appellent pas le pas couplé.
Les diagnostics de résidu/erreur inverse du rapport portent sur le dernier système
incrémental, explicitement signalé par `refinements=1` ; la pression publique est p+q.

Le banc `delta_coupling --flat [--fine]` poursuit désormais vingt pas au lieu
d'exiger le refus du premier. L'ancien comportement reste une contre-épreuve dans
le test diagnostic, par appel direct à la projection sans affinage.

## Suite, coût et limites

2026-09-16, Windows x86-64, release hors suite en cours, CPU séquentiel, secteur
avant/après (état batterie 2, 99 %). 20 pas, départ 1 s, dt=1 ms ; première exécution
incluse, aucune chauffe écartée. Paramètres du banc inchangés depuis S250.

| cas | D maximal sur 20 pas | pas affinés | pas médian / max (ms) | préparation médiane (ms) |
|---|---|---|---|---|
| 16×8, couvercle imposé S250 | 5,7869·10⁻⁶ | 0 | 0,1896 / 0,2749 | 0,1858 |
| 16×8, plat | 9,9022·10⁻⁶ | 7 | 0,2717 / 8,5510 | 0,1893 |
| 32×16, plat | 9,9284·10⁻⁶ | 7 | 45,0700 / 56,8791 | 0,7966 |

La variante à 32 colonnes a d'abord été exécutée pendant les tests pour vérifier
qu'elle progressait ; ses chronométrages concurrents ne sont pas utilisés ici.
Pas de gain de vitesse revendiqué : le petit champ projeté impose une précision
relative difficile aux projections initiales et à leur repli. Le budget de 1 s
du banc permet de mesurer ce travail, **pas de recevoir les 2 ms du système**.
A276/A244 conservent le coût/admission ; ce lot reçoit le démarrage, pas l'usage temps réel.

Suite complète : **449 réussis, 16 ignorés, zéro échec**. Empreinte `delta_filters`
inchangée **0xfb12b2092df4ee6d**, ordres 1,947/1,957/1,959. Le nominal S250 ne déclenche
aucun affinage et conserve ses mesures numériques. A283 fermée sur les deux cas
et trajectoires reçus ; pas de garantie universelle au plancher ni de couplage mobile.
Le pas garde le droit de refuser après l'unique correction, sans temps avancé.
