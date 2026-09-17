# ADR-163 — Regrouper les ondes filtrées sans changer le reflet

Actée S266, 2026-09-18. Remplace la rétention conditionnelle du cache d'ADR-162, rejeté
selon ses critères de précision et de gain. Conserve la fermeture et la quadrature d'ADR-161.

## Décision

Le ciel reste calculé directement. L'hôte précalcule pour chaque rang de la queue la somme
des covariances linéaires de ce rang à la dernière composante active :
`C_suffixe[i] = somme(j=i..n-1) a_j² k_j k_jᵀ / 2`.
Les composantes étant triées par nombre d'onde, dès que le filtre Nyquist est nul à un rang,
tous les rangs suivants sont nuls. Le fragment ajoute alors `C_suffixe[i]` et arrête sa boucle.
La modulation et le transport eulérien s'appliquent ensuite comme avant.

Le produit tensoriel d'ordre 3 reçoit des boucles à bornes constantes (3), conservant
l'ordre des neuf termes, poids et nœuds. L'ordre 5 reste le contrôle existant.

`--reflets-filtres` emploie ce chemin ; `--reflets-somme-directe` conserve le témoin S265.
Le buffer GPU de queue gagne 1 Kio, la construction CPU emploie un tableau de pile de 1 Kio,
sans allocation par image. Aucun cache de ciel n'est alloué. Spectre, vent, hauteurs et
requêtes de jeu restent inchangés.

La somme suffixe change l'ordre d'arrondi, donc l'image n'est pas promise au bit ; la réception
garde les seuils déclarés avant construction dans [CIEL-CACHE-S266](../validation/CIEL-CACHE-S266.md).
Le gain de cette implémentation ne reçoit ni le budget 2 ms ni la convergence de la quadrature.
