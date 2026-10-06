# ADR-246 — Vingtième revue de méthode (S576–S580)

- **Statut : actée**, S581, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-245](ADR-245-dix-neuvieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| Un signe faux dans une formule écrite au plan (S578 : `+ Im H·sin ωt` pour `H = A·e^(−ig)`) ; la forme voulue (`A·cos(ωt − g)`) était écrite à côté, et c'est elle que l'essai a mesurée | aucun | **rien à changer** : l'essai jugeait la forme physique, pas la ligne fautive |
| Une coupure de session (S579 : « Réessayer ») au milieu de la lecture du code de B | aucune perte : le plan n'était pas commencé, l'état propre | la reprise à chaud a tenu ; **rien à changer** |
| Un critère qui aurait été sous le quantum (S579 : la vitesse du niveau par différences finies, rapport 8 à l'ulp) — vu au plan, remplacé par la dérivée analytique | — | ADR-236 D1 a servi ; **rien à changer** |
| Les nombres du plan écrits par le script (S576–S580, ADR-243 D1) ; l'état exact gardé à part (S575 → ADR-245 D1) ; les références par des formes fermées (le sinc de la corde, la dérive de l'arrondi prédite et retrouvée, S577–S580) | — | **rien à changer** |

## 2. Décision

**Aucune protection nouvelle.** Cinq sessions sans erreur qui coûte : la méthode n'ajoute rien quand rien ne s'est répété (ADR-222 :
une leçon ne s'écrit que si elle crée ou change une ligne de la table).

## 3. La prochaine revue

S586.
