# ADR-242 — Seizième revue de méthode (S556–S560)

- **Statut : actée**, S561, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-240](ADR-240-quinzieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Un instrument d'énergie aux mauvais poids** (S555, relevé en S557) : la vitesse du couvercle comptée pour une maille pleine — l'énergie « 8,4 % au-dessus de E₀ » était l'instrument ; S558 montre la même erreur possible avec les ouvertures (1,8 %). L375 (« une grandeur de diagnostic s'éprouve à sa naissance ») n'a pas été chargée : l'instrument était dans l'essai, pas dans le cœur | une conclusion fausse publiée (S555), une note corrective, A332 mal posée | **protection élargie** (D1) |
| **Le rituel à une anomalie suivi d'un commit** (S559) : la commande enchaînait le rituel et le commit par `;` ; le décompte de la liste, faux, est parti, corrigé au commit suivant. Le rituel rendait pourtant le bon code de sortie | un commit de correction | **protection élargie et un outil** (D2) |
| Le lot dû rappelé en fin de rituel (S560) | — | lu et fait ; mais un rappel s'ignore — **outil** (D2) |
| Les dérivations depuis le code avant la mesure (S557–S558) ; la constante de temps au plan (S560, ADR-240 D2) ; les scripts en fichiers (S556–S560, ADR-240 D1) | — | appliqués, justes ; **rien à changer** |

## 2. Décisions

**D1 — Une grandeur intégrale d'un schéma (énergie, masse, norme) se mesure avec les poids de son produit scalaire** — ouvertures, demi-maille
du couvercle, volumes de contrôle —, lus dans le code ; et l'instrument s'éprouve d'abord sur un invariant connu du même schéma (S557 :
`Q = E₀` au départ) avant de juger (élargit L375).

**D2 — Le commit de clôture se garde par le code de sortie du rituel** (`&&` ou un test du code, jamais `;` après lui), et **le rituel sort
en erreur (code 3) quand le lot des registres est dû et que `--lot` manque**, comme il le fait sur une anomalie : un rappel qu'on peut
ignorer devient un refus (élargit L386 ; L349, « une liste à tenir se confie à un outil »).

## 3. La prochaine revue

S566.
