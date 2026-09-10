
# ADR-093 — Où se calibre la source d'un impact

- **Statut : actée**, S137, 2026-09-10, autonomie technique S71.
- **Prolonge :** candidat radial ADR-060, portée ADR-083, générateur ADR-092.
- **Corrige** un renvoi erroné porté par ces trois décisions.

## Problème

ADR-060, ADR-083 et ADR-092 renvoient la calibration de la forme spectrale, de `α` et de `η`
« au banc B2 ». **Ce renvoi est faux, et il l'a toujours été.**

B2 est le banc de la **technologie de W** : il choisit entre paquets lagrangiens, équation d'onde
2D et Boussinesq, et fixe `λ_cut`. Ses métriques sont l'erreur d'angle de sillage, la
conservation d'énergie sur 60 s, le coût pour 4096 paquets et le déterminisme croisé. Aucune ne
mesure ce qu'un objet qui entre dans l'eau **émet**.

B10, « cavité d'entrée dans l'eau », est plus proche : sphères et corps allongés, nombre de
Froude d'entrée de 1 à 15. Mais ses métriques portent sur la **cavité** — profondeur de
pincement, hauteur du jet de Worthington, durée — c'est-à-dire le phénomène proche et visuel, pas
l'onde de gravité qui en part.

**Aucun banc ne mesure la source d'onde d'un impact.** Le renvoi à B2 a masqué ce trou depuis
S77, et trois décisions successives l'ont recopié sans le vérifier.

## Décision

**La calibration de la source relève de B10, dont le protocole fournit déjà les entrées.** Des
sphères et corps allongés entrant à Froude connu, c'est exactement le montage dont `α` et `η` ont
besoin ; créer un douzième banc dupliquerait un protocole existant, et deux protocoles voisins
divergent (**L137**).

**Deux métriques y sont ajoutées**, et elles sont observables sans instrumenter la source :

- **Longueur d'onde dominante.** Chronométrer l'arrivée du maximum d'amplitude à une distance `r`
  du point d'entrée donne `λ = 8πr²/(g·t²)`, par `c_g = ½√(gλ/2π)` (SPEC-001 §1). `α = λ/b` s'en
  déduit, où `b` est la demi-largeur du corps.
- **Énergie rayonnée.** L'énergie du train d'ondes, intégrée sur un anneau à distance `r`,
  rapportée à `½ρb³v²` donne `η` directement.

**La décision de B10 s'étend en conséquence** : à la table de coefficients par `Fr` et au seuil
de Froude s'ajoutent `α` et `η`, avec leur dépendance éventuelle au régime.

**Ce que la calibration doit resserrer est désormais borné**, ce qui donne au banc un critère de
réussite plutôt qu'une plage ouverte : `α ∈ [3,35 ; 6,11]` par dérivation (ADR-092), et
`η ≤ 2Kgα⁴bs²/v²` par la borne du modèle. Une mesure hors de ces bornes ne calibrerait pas le
modèle : elle le réfuterait, ce qui est un résultat en soi et doit être dit comme tel.

## Ce que cette décision ne fait pas

Elle ne mesure rien : elle dit où et comment mesurer. Aucun banc de ce plan n'a été exécuté, et
le bilan S69 le rappelle — onze bancs sur onze attendent une couche non écrite.

Elle ne modifie ni B2 ni son dossier, dont l'objet reste le choix technologique de W. Elle ne
touche pas aux métriques existantes de B10.

Elle ne prétend pas que `λ = 8πr²/(g·t²)` soit mesurable sans précaution : à distance trop
faible le paquet n'est pas encore dispersé, à distance trop grande l'amplitude passe sous le
bruit. Fixer ces deux distances fait partie du protocole, et pas de cette décision.

## Réception

Aucune, et c'est le point : un banc ne se reçoit qu'en l'exécutant. Ce qui est vérifiable ici
tient à la cohérence documentaire — les trois ADR portent une note corrective datée, et le plan
de benchmark porte les deux métriques. Voir [BANC-SOURCE-S137](../validation/BANC-SOURCE-S137.md).
