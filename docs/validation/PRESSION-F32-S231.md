# Pression f32 du candidat δ — S231, 2026-09-14

## Critères déclarés avant modification

I-08 demande f32 dans δ ; l'exception d'ADR-139 concerne V seulement. Construire stockage,
opérateur, projection et réductions en f32. L'interface d'hôte f64 peut transporter exactement
les scalaires f32 entre callbacks, mais ne doit pas effectuer les additions en double précision.
Les rapports peuvent élargir un diagnostic déjà calculé ; cela ne modifie pas la simulation.

Comparer au candidat f64 de `88b98fe` avec le même exemple `delta_precision` : domaine8×4m,
rho1025, g9,81, dt0,002s, surface4+0,01sin(2π(i+0,5)/nx), fonds plat0,5m et bosse S199,
nx16/32/64/128, nz=nx/2 ; un pas, plus100 pas à32×16. Champs complets, rapport et stockage
publiés hors fenêtre de mesure. Les sorties brutes sont des produits locaux dans `code/target/`,
pas des snapshots δ de jeu ; ce banc ne construit aucune persistance runtime.

Conserver repos exact et les critères existants (projection32×16 : divergence<1e-5, filtre
spatial S199). Quantifier l'écart au témoin f64, sans exiger les mêmes bits. Pour la sonde de
débit S199, vérifier que l'écart de précision reste inférieur à son erreur spatiale déjà mesurée
(Richardson f64 :0,025% plat et0,963% lisse au niveau128). Aucun seuil physique nouveau.
Le résidu récurrent de CG ne suffit pas à attester `b−Ap` : vérifier aussi le résidu recalculé,
et ne jamais publier une convergence que le vrai résidu ne permet pas d'affirmer.

Comparer le coût à charge/qualité identiques, stockage exact et mesures répétées hors I/O.
Conserver l'arrêt coopératif, les refus atomiques et le compteur du tas. Ne pas réduire les
seuils de qualité pour faire passer f32 ; signaler explicitement ses limites si elles apparaissent.
Ce lot ne reçoit ni I-05 complet, ni B3, ni surface mobile/3D/faces coupées.
