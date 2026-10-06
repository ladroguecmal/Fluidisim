# ADR-235 — Dixième revue de méthode (S526–S530)

- **Statut : actée**, S531, 2026-10-06 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 (toutes les cinq sessions) ; la précédente,
  [ADR-234](ADR-234-neuvieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **Une limite matérielle de la carte trouvée en lançant le premier grand domaine** : 65 535 groupes de dispatch (S520, 1,49 M mailles), puis une liaison de 128 Mo (S529, 3,4 M mailles) ; et la limite des noyaux de mailles (4,19 M) a fixé la taille du domaine de S529 | deux calculs relancés ; un domaine réduit | **protection nouvelle** (D1) — l'erreur s'est répétée, et aucune protection ne la nommait |
| Un critère de convergence point par point manqué alors que l'amplitude converge, la maille et le pas variant ensemble (S529) | un critère manqué, une cause « probable, non éprouvée » | une fois ; ADR-230 D1 (une référence convergée sur trois points) la couvre en esprit ; **rien à ajouter**, la suite d'A330 la reprendra |
| La garde de résolution qui refuse un montage hors du domaine de la somme (S528), la référence bruitée (S527), l'ordre de grandeur qui change la construction (S530 : l'Euler explicite à +212 %) | — | ADR-234 D1, D2, ADR-232 D2 appliquées ; **rien à changer** |

## 2. Décision

**D1 — Avant d'agrandir un domaine d'un ordre de grandeur, ses limites matérielles se calculent** : groupes de dispatch par dimension,
taille d'une liaison et d'un tampon, mémoire de la carte — une ligne de script au plan, comme un ordre de grandeur physique (ADR-232 D2) ;
et une limite atteinte se refuse avec un nom, jamais par un arrêt du programme.

## 3. La prochaine revue

S536.
