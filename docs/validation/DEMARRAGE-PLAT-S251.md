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

## Témoin de coût et attribution par pas — clôture S251

Reprise à chaud par Claude Opus 5 après interruption de P5, 2026-09-16, même poste,
secteur (état batterie 2, 99 %). Suite rejouée sur `ddf7606` : **449 / 16 / 0**.
Le témoin manquant du tableau précédent — 32×16 **sous hauteur imposée** — a été
mesuré, puis une trace temporaire par pas (retirée, banc livré inchangé) :

| cas | pas | itérations | affinage | plancher | pas (ms) |
|---|---|---|---|---|---|
| 32×16 imposé | 0–19 | 69–73 | 0 | non | 1,06–1,76 (médiane 1,09) |
| 32×16 plat | 0–6 | 667–672 | 1 | oui | 42,8–47,7 |
| 32×16 plat | 7–16 | 581–583 | 0 | oui | 41,0–46,0 |
| 32×16 plat | 17–19 | 82 | 0 | oui | 1,2–2,1 |
| 16×8 plat | 0–6 | 328–329 | 1 | non | 5,3–6,7 |
| 16×8 plat | 7–19 | 40 | 0 | oui | 0,18–0,36 |

Deux médianes indépendantes 32×16 plat : 44,1 et 42,7 ms, contre 1,09 imposé.
**Le surcoût ×40 n'est pas celui de l'affinage** : les pas 7 à 16, non affinés,
coûtent autant. Il accompagne l'arrêt au plancher d'un petit champ projeté, sans
s'y réduire : les pas 17–19, eux aussi au plancher, reviennent à 82 itérations
sans qu'on sache pourquoi. La répartition des ~580 itérations
entre projection ordinaire et repli multigrille n'est **pas** mesurée ; les pas 7–16
sont reçus avec D de 4,6 à 9,9·10⁻⁶, sous la tolérance mais sans marge large.
À 16×8 (un niveau grossier 8×4), le même régime ne coûte que 40 itérations par pas.

Techniques présentes : CG séquentiel, repli multigrille ADR-147, affinage unique
ADR-150. Absentes : tout départ non nul — chaque projection, repli et affinage
compris, repart de p=0 (`project`), sans la pression du pas précédent ni celle
de la projection ordinaire ; parallélisme (fermé pour cette boucle, S244).
Domaine : 2D x-z, deux tailles, B/W du banc S250. Suivi : A284.

*Note datée S252, 2026-09-16.* A284 est attribuée : 94 % du pas plat 32×16 venaient du repli
multigrille, dont le gradient conjugué avait un β fautif (A285). Corrigé, ce pas coûte
2,53 ms de médiane au lieu de 43,6. [Mesures](MULTIGRILLE-BETA-S252.md).
