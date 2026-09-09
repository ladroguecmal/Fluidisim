# ADR-088 — Admission incrémentale, exacte ou pas du tout

- **Statut : actée**, S132, 2026-09-10, autonomie technique S71.
- **Prolonge :** préparation spectrale ADR-063, admission dynamique ADR-086, sortie de
  saturation ADR-087.
- **Résout :** S131-1.

## Ce que la mesure a montré

**Le coût de la préparation est linéaire en nombre de segments.** Mesuré à 224×128 : ×1,98 à
deux segments, ×3,91 à quatre, ×7,12 à huit, pour 4,85 ms par segment. Une admission recalcule
donc tout ce qui était déjà publié, et le gaspillage croît avec la charge.

**L'ajout après coup est exact — sous une condition.** `prepare_segments` accumule par nœud,
segment après segment, en `f32`. Ajouter la contribution d'une source à un champ déjà préparé
donne donc le **même champ au bit près** si cette source vient en dernier dans l'ordre
canonique ; si elle s'insère au milieu, les huit points de contrôle diffèrent tous. La sonde le
vérifie dans les deux sens — et il lui a fallu **trois** segments pour le voir : avec deux
termes l'addition `f32` est commutative, et une sonde à deux sources aurait conclu à tort que
l'ordre n'importe pas.

## Décision

**L'admission emprunte le chemin incrémental si et seulement si la source s'insère en dernier ;
sinon elle recalcule tout.** Dans les deux cas le résultat est celui de la voie directe, au bit
près. C'est la seule forme acceptable : un champ qui dépendrait de l'ordre historique des
admissions donnerait deux résultats pour un même journal, et le déterminisme bit à bit d'I-03
ne survivrait pas à deux hôtes ayant admis les mêmes sources dans un ordre différent.

**L'optimisation est donc invisible.** Aucune API ne change, aucun appelant n'a à savoir quel
chemin a été pris, et rien n'est annoncé : il n'y a rien à annoncer, puisque le résultat est le
même. C'est l'inverse de la discipline d'ADR-079 et ADR-080 — ici, ne pas exposer est ce qui
protège la propriété.

**Le `Slot` stocke désormais la pression modale cumulée**, deux `f32` de plus. L'énergie se
recalcule depuis la réponse cumulée, que le `Slot` portait déjà ; la puissance a besoin de la
pression, qu'il ne portait pas. La reconstituer aurait exigé de refaire une réponse modale par
segment — c'est-à-dire l'essentiel du coût qu'on cherche à éviter. Le prix est de +18 % sur les
pools de coefficients ; aucun format sérialisé n'est touché, `Slot` étant interne.

**La transaction d'ADR-086 est préservée.** Le chemin incrémental travaille sur le pool de
réserve, après recopie du pool actif : un échec laisse la publication intacte, comme avant. La
recopie coûte un `memcpy`, sans commune mesure avec la réponse modale qu'elle évite.

## Ce que cette décision ne fait pas

Elle n'accélère pas la reconstruction après élargissement (ADR-087) : le nouveau contrôleur part
d'un pool vide, et rien ici ne lui transmet les coefficients de l'ancien. La fenêtre sans champ
de S131 reste ce qu'elle est.

Elle ne change aucun résultat : les hachages de campagne doivent rester identiques, et c'est la
vérification qui accompagne la construction. Elle n'introduit aucun mode dégradé, aucune option,
aucun réglage.

Elle ne traite pas le retrait d'une source, qui n'existe toujours pas, ni la réadmission d'une
source modifiée — un conflit, refusé par ADR-075.

## Réception

[INCREMENTAL-S132](../validation/INCREMENTAL-S132.md). L'identité en bits est vérifiée dans les
deux configurations — insertion finale et insertion au milieu — contre la voie directe, et le
gain est mesuré aux recettes de la campagne.
