# ADR-169 — Les pas mobiles partent de la pression publiée

Actée S276, 2026-09-18, autonomie S71. Technique de coût (ADR-131, « départ non nul »,
liste S252–S274) ; ne change aucune porte d'acceptation (ADR-143/144).

## Constat

Avec la multigrille mobile (ADR-167) et l'échantillonnage par grille (S276), le pas couplé est
le poste dominant d'un pas en direct : 24 ms à 6 656 mailles. Le gradient conjugué part de `p = 0`
alors que la pression varie peu d'un pas à l'autre.

## Décision

La **projection principale** de `step_perturbation_mobile` et de `step_surface_mobile` part de la
pression publiée : mise à zéro hors des mailles mouillées, résidu vrai `b − A·p`. Second membre
nul : départ nul, comme avant. L'affinage (ADR-151/153) et les pas à couvercle partent de zéro.
Les deux pas mobiles partent de la même façon, ce qui garde l'identité au bit du pas couplé à fond
nul avec le pas S237. Un témoin à départ nul reste disponible en essai.

## Conséquences

Itérations divisées par deux sur vingt pas couplés (134 contre 266), pas couplé 24 → 17,5 ms en
direct, vitesses à 8·10⁻⁸ m/s du départ nul. Les bits des pas mobiles changent ; les rejeux S275
sont recalculés (cache versionné). Le premier pas depuis le repos est inchangé (pression nulle).

[Mesures](../validation/COUT-DIRECT-S276.md).
