# ADR-167 — La multigrille préconditionne le mode à surface mobile

Actée S274, 2026-09-18, autonomie S71. Remplace le **point 5** d'[ADR-147](ADR-147-la-multigrille-est-un-repli-de-precision.md)
(« le mode à surface mobile ne l'a pas ») ; les autres points d'ADR-147 et les portes d'acceptation
d'ADR-143/144 sont inchangés. Répond au premier blocage de coût de J2 (A276).

## Constat

Le pas couplé mobile (ADR-152), chemin de tout domaine δ sous B+W, résout la pression par un
gradient conjugué à préconditionneur de Jacobi : **1 145 itérations** au pire et **280 ms** de
médiane par pas à 16 384 mailles (S274, 200 pas, secteur). Corrigée en S252, la multigrille
converge en 6 à 8 itérations sur le chemin à couvercle, et elle est plus rapide dès 512 mailles.
Le mode mobile ne l'a jamais eue, faute de niveaux grossiers pour sa surface fantôme.

## Décision

1. En mode mobile, quand la grille se divise, le gradient conjugué est préconditionné par un
   **cycle en V** au lieu de Jacobi. Ce n'est pas un repli : c'est le chemin ordinaire du mode.
2. **Niveau fin** : l'opérateur mobile exact (fantômes `a/θ`), lissage de Jacobi amorti sur sa
   diagonale, résultat mis à zéro hors des mailles mouillées.
3. **Niveaux grossiers, recalculés à chaque projection** depuis les mailles mouillées : une maille
   grossière est active si l'une de ses filles l'est (fraction moyenne des filles mouillées) ;
   inactive, elle est **air** si l'une de ses filles l'est, sinon solide. Vers l'air, Dirichlet à
   demi-maille (coefficient 2, `θ = ½`) ; vers le solide, Neumann. Ouvertures solides du chemin
   ordinaire, inchangées. C'est une approximation licite : elle ne pèse que sur la vitesse.
4. **Symétrie définie positive** assurée comme dans ADR-147 (lisseur diagonal, restriction
   transposée de la prolongation, autant de lissages avant qu'après, masque symétrique) et
   **vérifiée numériquement**.
5. Mémoire comptée à la configuration : un tampon fin et deux tableaux par niveau grossier.
   Aucune allocation dans le pas. Le chemin à couvercle reste identique au bit.

## Conséquences

Les bits des pas mobiles changent ; pas leur verdict. Les réceptions S237/S253/S254 se rejouent
contre leurs critères, anciennes valeurs dans Git. Le témoin de Jacobi reste disponible en essai.
Coût attendu en baisse, pas sous 2 ms : ni 3D, ni GPU, ni parallélisme ici.

[Critères et mesures](../validation/COUT-MOBILE-S274.md).
