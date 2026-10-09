# ADR-283 — Quarante-neuvième revue de méthode (S721–S725)

- **Statut : actée**, S726, 2026-10-09 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-282](ADR-282-quarante-huitieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S721 | la revue (ADR-282) ; `fermer.py` a refusé son propre premier appel (un argument manquant au script de fin), sans rien commettre | aucun | l'outil a fait son office |
| S722 | le déclencheur au plus simple, choisi puis écarté avant tout code (il se déclenchait au départ sur les plages de référence) | aucun | — |
| S723 | B1 : n'échanger que la masse laissait l'interface réfléchir (8,3 %) ; le flux complet, 1,6 % | un essai | une leçon de conception, inscrite au registre ; reprise en B3 |
| S723, S725 | deux longs *heredocs* rejetés par bash, sans effet ; ADR-267 D2 le défend déjà | aucun | ADR-267 D2 le dit déjà |
| S724, S725 | le code écrit avant le plan commis, les critères écrits avant l'essai | aucun résultat touché | la règle existe ; rappelée en D3 |
| S724 | `fermer.py` a écrit des `%%` dans un titre, et `--lot` a été passé une session trop tôt | un amendement | aucune : cosmétique |
| S725 | **une seule source oubliée** : Saint-Venant partait de 0,4 m, la 3D lisait 0,39931 m. Le couplage s'est emballé (1,2 m/s au repos). ADR-273 D2 et S684 avaient déjà la leçon | un essai, un diagnostic | **D1** |
| S725 | **les constantes de deux solveurs mêlées dans un bilan** : le `dx` d'APIC (`f32`) comptait le volume cédé par Saint-Venant (`f64`) ; une fuite de 1,7·10⁻⁸ par échange, 2,4·10⁻¹¹ en tout | deux essais, un diagnostic pas à pas | **D2** |
| S725 | **un témoin mal choisi** : une bosse dispersive (`kd` ≈ 1,6) pour juger un raccord entre la 3D et Saint-Venant, qui ne la portent pas de la même façon ; la réflexion physique mêlée à celle du raccord | deux essais | **D1** |

## 2. Décisions

**D1 — Un raccord entre deux modèles différents se juge sur un état et un régime qu'ils partagent.**
- **L'état** : chaque solveur part du même état, tel que l'autre le *lit*. Pour un niveau, c'est celui que lit la 3D par sa surface, non
  celui qu'on a posé. ADR-273 D2 (une seule source) s'applique à chaque montage neuf, et son plan le nomme.
- **Le régime** : le critère se prend sur une onde que les deux portent également (une onde longue pour la 3D et Saint-Venant). Une onde
  que l'un porte mal (dispersive, déferlante) se rapporte, et ne fait pas le critère du raccord.
- C'est le complément d'ADR-273 D1, quand les deux copies du même solveur ne sont pas possibles.

**D2 — Un bilan entre deux solveurs compte chaque volume avec les constantes du solveur qui le cède ou le reçoit** : le pas d'espace, la
largeur des faces, la précision. Jamais une constante convertie depuis l'autre (un `f32` élargi n'est pas le `f64` de l'autre). Le
diagnostic pas à pas (la somme des parts) se fait dès le premier écart de masse au-delà de 10⁻¹³.

**D3 — Rappel : le plan précède le code.** Le code de S724 et S725 a été écrit avant le plan commis, avec des critères écrits avant les
essais. Aucun résultat n'en a souffert, mais la règle est la même pour une session longue : P1 commis, puis le code.

## 3. La prochaine revue

S731.
