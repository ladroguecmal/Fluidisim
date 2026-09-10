# ADR-102 — Transport de la recette spectrale

Statut : acté, S149, 2026-09-10. Complète ADR-100 et ADR-101.

Le rejeu mémoire S148 ne suffit pas pour restaurer un hôte. WSPR V1 transporte
les paramètres de B, puis les recuit avant de rendre un objet utilisable.
Aucun état temporel de B n'est sauvegardé (I-02/I-09).

## Format

Exactement 64 octets, little-endian, sans représentation native de structure.

| Octets | Contenu |
|---|---|
| 0..4 | ASCII WSPR |
| 4..8 | version numérique de cuisson u32, 1 |
| 8..12 | nombre de composantes u32 |
| 12..16 | réservés, zéro obligatoire |
| 16..24 | graine u64 |
| 24..56 | huit f32 : Hs, Tp, direction, gravité, gamma, min_ratio, max_ratio, spread_turns |
| 56..64 | hash de conformité de cuisson ADR-101 |

Seul Cooked expose encode ; decode vérifie longueur exacte, magie, réservés,
version, domaine numérique par bake, puis hash. Aucun remplacement partiel :
il retourne un nouveau Cooked ou une erreur. Taille, tableaux et boucles bornés,
aucune allocation du codec (I-06). Les bits f32, dont zéro signé, sont conservés.
Le hash détecte une divergence numérique ; il n'authentifie pas un expéditeur.
Une version inconnue est refusée, sans migration implicite du fond historique.

## Responsabilité de l'hôte

WSPR accompagne WLIV ; aucun des deux ne contient l'ancre de B. L'hôte conserve
ensemble recette, ancre exacte, référentiel/cellule, milieu et temps de simulation.
Il reconstruit dans des objets de réserve, vérifie les deux charges, puis publie
l'ensemble. Restaurer WLIV seul et garder un autre B produirait un autre champ.
L'exemple S149 reçoit ce montage en mémoire ; il ne définit pas le format disque
ou réseau global du jeu. Background alloue uniquement lors de cette initialisation.

I-03 reste à recevoir sur plusieurs plateformes : un hash identique localement
ne constitue pas cette preuve. I-07 conserve la gravité injectée et sa comparaison
B+W. Aucun invariant modifié. Suite : W4/sillage, puis B2 ; A212 reste partielle.
