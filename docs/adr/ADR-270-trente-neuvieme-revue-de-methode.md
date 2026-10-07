# ADR-270 — Trente-neuvième revue de méthode (S671–S675)

- **Statut : actée**, S676, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-269](ADR-269-trente-huitieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| session | ce qui s'est passé | coût | suite |
|---|---|---|---|
| S671 | la revue (ADR-269) ; la voie écartée de S668 écrite | — | — |
| S672 | trois solutions analytiques ; chaque écart mesuré retrouve celui du calcul du plan (0,10 % ; 0,73 % ; 0,99 %) | — | la pratique d'ADR-268 D1 (la borne au plancher calculé) |
| S673 | **la cuisson à 8,3 s** au premier essai : le courant cherché par bissection à chaque rangée fine. Le plan comptait 0,8·10⁹ opérations ; la bissection en demandait le double. Illinois, et les seules rangées des tables : 5,3 s | quelques minutes | — |
| S674 | **l'essai de S670 a échoué** après le point fixe : il comparait la côte à une référence sans rétroaction. Le critère (1) du plan l'avait prévu (« S670 et S673 inchangés ») ; le remède (une marche pour ces essais, par `cuire_interne`) est venu après l'échec | une relance | — |
| S675 | la séance visuelle ; trois défauts de mise en page vus avant l'envoi et corrigés | — | la pratique de S663 |

## 2. Décisions

**Aucune règle nouvelle.** Les deux frictions n'ont coûté qu'une relance chacune :

- un coût sous-estimé d'un facteur deux ;
- un essai ancien qui encodait l'ancien comportement par défaut.

Les règles en place les ont rattrapées : le coût est mesuré et rapporté, le témoin au bit est écrit au plan.

## 3. La prochaine revue

S681.
