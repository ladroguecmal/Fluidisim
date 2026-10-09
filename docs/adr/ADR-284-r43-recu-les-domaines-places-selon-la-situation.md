# ADR-284 — R43 reçu ; les domaines placés selon la situation

- **Statut : actée**, S730, 2026-10-09. Décision de l'utilisateur, au verdict de R43.

## Contexte

R43 (S720) montrait la vague de bout en bout (SGN au large, la bande 3D, Saint-Venant au rivage) contre le tout-3D. Le verdict de
l'utilisateur, le 2026-10-09 :

- *« pour le reste de R43 le rendu est réaliste et cohérent »* ;
- sauf au raccord du rivage. Il a joint une capture du déferlement : le jet de la 3D retombe contre un mur d'eau de Saint-Venant, avec une
  poche d'air entre les deux. *« Cela ne fait pas réaliste. »*

Il demande de déplacer Saint-Venant plus loin pour cette vague, ou de s'en passer. Plus généralement : *« un système où le domaine d'action
de SGN, de la 3D et de Saint-Venant se déplace intelligemment en fonction de la situation, pour garder le plus de réalisme possible avec les
meilleures performances »*. Sa remarque de la veille, *« la 3D jusqu'à la plage si une vague éclate trop près du bord »*, venait du même
constat.

Le défaut est dans les deux films, le tout-3D compris : son raccord au rivage est fixé à 10,775 m (trois mailles de fond), là où le jet
retombe. Saint-Venant porte l'onde sans la faire déferler, et plus vite que la 3D. Son eau monte devant le jet.

## Décision

**D1 — R43 est reçu**, réaliste et cohérent, sauf le raccord au rivage pendant le déferlement. Ce défaut est une demande, traitée d'abord.

**D2 — Une frontière entre deux modèles ne se place jamais là où agit une physique que l'un des deux n'a pas.** Pour la 3D et Saint-Venant :
la frontière se place au-delà du point où le jet retombe, plus une marge. Si ce point dépasse le rivage, la 3D couvre la plage jusqu'à sa
mort (M1). Pour SGN et la 3D : avant le déferlement, avec une marge (déjà le cas, ADR-278).

**D3 — L'ordre** :
1. le raccord du rivage placé par la règle de D2, sur la vague de R43 (S730) ;
2. la règle tirée de la prédiction (le déclencheur D1 de LOD-ETAPE-2-S705 : le point et l'instant du déferlement prévus) ;
3. les frontières qui se déplacent pendant le calcul (le geste de B4b, S728) ;

puis la bande étroite (étape 4 d'ADR-275 D2).

**D4 — Le réalisme d'un raccord se mesure** : le saut du niveau entre ses deux côtés, lu par la surface de la 3D (ADR-280 D1) et par la
hauteur de Saint-Venant. Chaque montage à raccord le rapporte.

## Conséquences

- ADR-275 D2 garde ses étapes ; D3 s'y insère avant l'étape 4.
- Le juge tout-3D de la plage (S717) avait le même raccord : ses mesures de remontée sont à relire après S730.

*Note datée du 2026-10-09 (S731)* : l'utilisateur fait du sélecteur des domaines la pièce la plus peaufinée et la plus solide (*« c'est grâce à
lui que l'on aura le meilleur compromis entre réalisme et performance »*). ADR-285 D4 en fixe les juges : une batterie de scènes, le
réalisme, le coût, la solidité.
