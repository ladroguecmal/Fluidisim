# Le corps qui flotte contre Archimède — S762 (listes 4.1, 6.7 ; ADR-289 D3.2)

*S762, 2026-10-10.* Le banc de la 3D corrigée (ADR-294). **La question** : une sphère libre se pose-t-elle à son tirant d'eau exact, et
l'écart diminue-t-il avec la maille ?

## Ce qui est fait

`flottaison_s762` : une cuve fermée de 1,2 × 0,4 m, 0,4 m d'eau ; une sphère de 0,1 m de rayon au centre, posée 2 cm au-dessus de son
équilibre, libre (`set_body_mass`, la masse ajoutée implicite de S655), 5 s. **La référence exacte** (Archimède) : la calotte immergée `h`
telle que `(h/R)²·(3 − h/R) = 4s`. **Le tirant mesuré** : le niveau de l'eau loin de la sphère (lu par φ, `|x − x_c| > 0,3 m`) moins le bas
de la sphère, moyenné sur la dernière seconde.

## Reproduire

`python outils/essai.py a_floating_sphere_against_archimedes_s762 --ignore` (≈ 1 h ; l'essai D seul ≈ 40 min).

## Mesuré

| essai | maille | s | 3D | le tirant | l'exact | **l'écart** | la vitesse verticale max (dernière seconde) |
|---|---|---|---|---|---|---|---|
| A | 2,5 cm | 0,25 | ADR-294 | 0,0655 m | 0,0653 m | **+0,3 mm** | 1,16 cm/s |
| A | 2,5 cm | 0,5 | ADR-294 | 0,0957 m | 0,1000 m | **−4,3 mm** | 4,11 cm/s |
| A | 2,5 cm | 0,75 | ADR-294 | 0,1327 m | 0,1347 m | **−2,0 mm** | 4,58 cm/s |
| B | 2,5 cm | 0,5 | sans projection | 0,0958 m | 0,1000 m | −4,2 mm | 2,47 cm/s |
| C | 5 cm | 0,5 | ADR-294 | 0,1115 m | 0,1000 m | +11,5 mm | 2,69 cm/s |
| D | 1,25 cm | 0,5 | ADR-294 | **non mesuré** : arrêté avant la fin, la session close à la demande de l'utilisateur | | | |

Dans tous les essais, la masse de l'eau est exacte (le nombre de particules constant) et la sphère ne dérive pas (0,000 m).

**Les critères, écrits avant :**
- **(1) Tenu.** Le tirant est à 1 cm de l'exact aux trois densités, à 2,5 cm : de 0,3 à 4,3 mm.
- **(2) En partie.** De 5 cm à 2,5 cm, l'écart passe de +11,5 à −4,3 mm. Le pas à 1,25 cm (D) reste à mesurer.
- **(3) Manqué tel qu'écrit** : la sphère oscille encore, de 1,2 à 4,6 cm/s, soit une amplitude de 2 à 5 mm à sa période propre (≈ 0,63 s
  pour s = 0,5). **Le critère était mal posé** :
  - la cuve est fermée et petite ; les vagues que la sphère rayonne (≈ 0,6 m de longueur d'onde) reviennent sur elle ;
  - la 3D d'ADR-294 conserve l'énergie (S759) : rien n'amortit cet échange ;
  - (B) sans projection, qui dissipe, bouge deux fois moins.

  Le repos d'un corps se juge dans une cuve dont les bords absorbent, ou par la décroissance de l'oscillation contre une référence.
- **(4) Tenu.**

## Ce que cela dit

- **La 3D corrigée porte un corps à son tirant d'Archimède**, à quelques millimètres près pour une sphère de 20 cm, dès 4 mailles par
  rayon.
- **La projection de densité ne fausse pas la flottaison** : (A) et (B) donnent le même tirant (−4,3 et −4,2 mm). Le piège annoncé (les
  mailles voisines du corps comptées creuses) ne se voit pas dans le tirant.

## La suite (S763)

- L'essai D (1,25 cm), seul, pour la convergence.
- Le repos du corps jugé dans une cuve aux bords absorbants, ou par la décroissance de son oscillation.
- Puis la rupture de barrage, qui attend les mesures de Martin et Moyce (un téléchargement à accorder).
